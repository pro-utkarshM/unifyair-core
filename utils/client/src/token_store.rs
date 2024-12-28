use std::{
	fmt,
	future::Future,
	hash::Hash,
	sync::{
		Arc,
		atomic::{AtomicU32, Ordering},
	},
};

use scc::HashMap as SccReadHashMap;
use thiserror::Error;
use tokio::sync::Notify;

const HASH_MAP_CAPACITY: usize = 1024;

// A global atomic counter for generating unique IDs for TokenState instances.
static TOKEN_COUNTER: AtomicU32 = AtomicU32::new(1);

/// Increments the global atomic counter and returns the new value.
fn increment_counter() -> u32 {
	TOKEN_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Represents the internal state of a token.
#[derive(Clone)]
pub(crate) enum TokenStateInner<V> {
	Ready(V),              // Token is ready with a value.
	Updating(Arc<Notify>), // Token is being updated, and `Notify` is used to signal completion.
	Failed,                // Token update failed.
	Uninitialized,         // Token has not been initialized yet.
}

/// Represents the state of a token with a unique ID and an internal state.
pub(crate) struct TokenState<V> {
	id: u32,                   // Unique identifier for the token.
	state: TokenStateInner<V>, // Internal state of the token.
}

impl<V> TokenState<V> {
	/// Creates a new `TokenState` with a `Ready` value.
	pub(crate) fn new_ready(value: V) -> Self {
		let id = increment_counter();
		let state = TokenStateInner::Ready(value);
		Self { id, state }
	}

	/// Creates a new `TokenState` in the `Uninitialized` state.
	pub(crate) fn new_unitialized() -> Self {
		let id = increment_counter();
		Self {
			id,
			state: TokenStateInner::Uninitialized,
		}
	}

	/// Creates a new `TokenState` in the `Updating` state and returns it along
	/// with the `Notify` instance.
	pub(crate) fn new() -> (Self, Arc<Notify>) {
		let id = increment_counter();
		let notify = Arc::new(Notify::new());
		(
			Self {
				id,
				state: TokenStateInner::Updating(notify.clone()),
			},
			notify.clone(),
		)
	}

	/// Checks if the token is in the `Updating` state.
	pub(crate) fn is_updating(&self) -> bool {
		match &self.state {
			TokenStateInner::Updating(_) => true,
			_ => false,
		}
	}

	/// Checks if the token is in the `Ready` state.
	pub(crate) fn is_ready(&self) -> bool {
		match &self.state {
			TokenStateInner::Ready(_) => true,
			_ => false,
		}
	}
}

/// Errors that can occur when interacting with the `TokenStore`.
#[derive(Error, Debug)]
pub enum StoreError {
	#[error("ReadError: Unable to read the value for token_id: {0}")]
	ReadError(u32),

	#[error("InvalidStateTransitionError: Transitioning to invalid state: {0}")]
	InvalidStateTransitionError(u32),

	#[error("UpdateAlreadyInProgress: Update already in progress for token_id: {0}")]
	UpdateAlredyInProgress(u32),

	#[error(
		"TokenEntryCreationError: Token entry cannot be created because token state is invalid: \
		 {0}"
	)]
	TokenEntryCreationError(u32),

	#[error("MaximumReadIterations: Unable to read token after MAX iterations {0}")]
	MaximumReadIterations(u32),

	#[error("InnerFutureError: Error Occured in the passed future")]
	InnerFutureError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// A store that manages tokens using a concurrent hash map.
#[derive(Default)]
pub struct TokenStore<K, V>
where
	K: 'static,
	V: 'static,
{
	map: SccReadHashMap<K, Arc<TokenState<V>>>, // Concurrent hash map storing token states.
}

/// Represents an entry in the `TokenStore`.
pub struct TokenEntry<V>(Arc<TokenState<V>>);

impl<V> TokenEntry<V> {
	/// Creates a new `TokenEntry` from a `TokenState` if it is in the `Ready`
	/// state.
	pub(crate) fn new(value: Arc<TokenState<V>>) -> Result<TokenEntry<V>, StoreError> {
		if !value.as_ref().is_ready() {
			return Err(StoreError::TokenEntryCreationError(value.as_ref().id));
		}
		Ok(Self(value))
	}

	/// Creates a new `TokenEntry` without checking the state.
	pub(crate) fn new_unchecked(value: Arc<TokenState<V>>) -> Self {
		Self(value)
	}

	/// Retrieves the value from the `TokenEntry`.
	/// Panics if the state is not `Ready` (should not happen if used
	/// correctly).
	pub fn get(&self) -> &V {
		match self.0.as_ref().state {
			TokenStateInner::Ready(ref v) => v,
			_ => unreachable!(),
		}
	}
}

impl<V: fmt::Debug> fmt::Debug for TokenEntry<V> {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.get().fmt(f)
	}
}

impl<K, V> TokenStore<K, V>
where
	K: Eq + Hash + Clone + 'static,
	V: Clone + 'static,
{
	/// Creates a new `TokenStore`.
	pub fn new() -> TokenStore<K, V> {
		TokenStore {
			map: SccReadHashMap::with_capacity(HASH_MAP_CAPACITY),
		}
	}

	/// Asynchronously retrieves a token entry from the store.
	/// Waits if the token is in the `Updating` state.
	pub async fn get(
		&self,
		key: &K,
	) -> Result<Option<TokenEntry<V>>, StoreError> {
		for _ in 1..10 {
			let value = self.map.read_async(key, |_, v| v.clone()).await;

			let notify = match value {
				None => return Ok(None),
				Some(ref val) => match &val.as_ref().state {
					TokenStateInner::Ready(ref _v) => {
						return Ok(Some(TokenEntry::new_unchecked(val.clone())));
					}
					TokenStateInner::Failed => return Err(StoreError::ReadError(val.id)),
					TokenStateInner::Updating(notify) => notify.clone(),
					TokenStateInner::Uninitialized => return Ok(None),
				},
			};

			let notify = notify.clone();
			drop(value); // Drop the reference to avoid deadlocks.
			notify.notified().await; // Wait for the update to complete.
		}
		Ok(None)
	}

	/// Asynchronously sets a token entry in the store, transitioning it to
	/// `Ready` or `Failed` based on the future's result. It early returns with
	/// error if another update is already in progress.
	pub async fn set<E>(
		&self,
		key: K,
		future: impl Future<Output = Result<V, E>>,
	) -> Result<TokenEntry<V>, StoreError> 
    where E: std::error::Error + Send + Sync + 'static
	{
		let (ongoing_update_state, notify) = TokenState::<V>::new();
		let id = ongoing_update_state.id;

		let entry = self.map.get_async(&key).await;
		match entry {
			Some(val) if val.as_ref().is_updating() => {
				return Err(StoreError::UpdateAlredyInProgress(val.as_ref().id));
			}
			_ => (),
		};
		drop(entry); // Drop the reference to avoid deadlocks.

		self.map
			.upsert_async(key.clone(), Arc::new(ongoing_update_state))
			.await;

		let res = future.await;
		let (val, res) = match res {
			Ok(value) => (TokenStateInner::Ready(value), Ok(())),
			Err(err) => (
				TokenStateInner::Failed,
				Err(StoreError::InnerFutureError(Box::new(err))),
			),
		};
		let token_state = Arc::new(TokenState { id, state: val });
		let state_copy = token_state.clone();
		self.map
			.update_async(&key, move |_, v| {
				*v = token_state;
			})
			.await;

		notify.notify_waiters(); // Notify waiters that the update is complete.

		// If the operation is successful, the token state is set to Ready with the new
		// value.
		res.map(|_| TokenEntry::new_unchecked(state_copy))
	}
}

#[cfg(test)]
mod tests {
	use std::{
		sync::atomic::{AtomicU32, Ordering},
		time::Instant,
	};

	use dashmap::DashMap;
	use tokio::{sync::RwLock, task};

	use super::*;

	#[tokio::test]
	async fn test_increment_counter() {
		let initial = TOKEN_COUNTER.load(Ordering::Relaxed);
		increment_counter();
		let after = TOKEN_COUNTER.load(Ordering::Relaxed);
		assert_eq!(after, initial + 1);
	}

	#[tokio::test]
	async fn test_token_state_ready() {
		let value = 42;
		let token_state = TokenState::new_ready(value);
		assert!(token_state.is_ready());
		assert!(!token_state.is_updating());
	}

	#[tokio::test]
	async fn test_token_state_updating() {
		let (token_state, _notify) = TokenState::<i32>::new();
		assert!(token_state.is_updating());
		assert!(!token_state.is_ready());
	}

	#[tokio::test]
	async fn test_token_store_get_and_set() {
		let store = TokenStore::<String, i32>::new();

		// Set a key-value pair asynchronously.
		let key = "test_key".to_string();
		store
			.set(key.clone(), async { Ok::<i32, StoreError>(123) })
			.await
			.unwrap();

		// Get the value back.
		let entry = store.get(&key).await.unwrap();
		assert!(entry.is_some());
		let value = entry.unwrap();
		assert_eq!(*value.get(), 123);
	}

	#[tokio::test]
	async fn test_token_store_updating() {
		let store = TokenStore::<String, i32>::new();

		let key = "test_key".to_string();
		let future = store.set(key.clone(), async {
			tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
			Ok::<i32, StoreError>(456)
		});

		let get_future = store.get(&key);

		let (set_result, get_result) = tokio::join!(future, get_future);
		set_result.unwrap();
		assert_eq!(get_result.unwrap().unwrap().get(), &456);
	}

	#[tokio::test]
	async fn test_token_store_failed() {
		let store = TokenStore::<String, i32>::new();

		let key = "test_key".to_string();
		store
			.set(key.clone(), async { Err(StoreError::ReadError(1)) })
			.await
			.unwrap_err();

		let result = store.get(&key).await;
		assert!(matches!(result, Err(StoreError::ReadError(_))));
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 12)]
	async fn test_thread_contention_with_avg_times() {
		let store = Arc::new(TokenStore::<String, i32>::new());
		let write_time_total = Arc::new(AtomicU32::new(0));
		let read_time_total = Arc::new(AtomicU32::new(0));
		let mut handles = vec![];

		// Writer threads: Insert 100 entries.
		for i in 0..1000 {
			let store = store.clone();
			let write_time_total = write_time_total.clone();
			handles.push(task::spawn(async move {
				let key = format!("key_{}", i);
				let start = Instant::now();
				store
					.set(key.clone(), async move { Ok::<i32, StoreError>(i * 10) })
					.await
					.unwrap();
				let elapsed = start.elapsed().as_nanos() as u32;
				write_time_total.fetch_add(elapsed, Ordering::Relaxed);
			}));
		}

		// Allow writers to complete.
		for handle in handles.drain(..) {
			handle.await.unwrap();
		}

		println!(
			"Average write time: {} ns",
			write_time_total.load(Ordering::Relaxed) / 1000
		);

		// Reader threads: Read those entries.
		for read_iter in 0..5 {
			for i in 0..1000 {
				let store = store.clone();
				let read_time_total = read_time_total.clone();
				handles.push(task::spawn(async move {
					let key = format!("key_{}", i);
					let start = Instant::now();
					let entry = store.get(&key).await.unwrap();
					let elapsed = start.elapsed().as_nanos() as u32;
					read_time_total.fetch_add(elapsed, Ordering::Relaxed);
					if let Some(entry) = entry {
						assert_eq!(*entry.get(), i * 10);
					} else {
						panic!("Key not found: {}", key);
					}
				}));
			}

			for handle in handles.drain(..) {
				handle.await.unwrap();
			}

			println!(
				"Iteration {}: Average read time: {} ns",
				read_iter,
				read_time_total.load(Ordering::Relaxed) / 1000
			);

			read_time_total.store(0, Ordering::Relaxed);
		}
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 12)]
	async fn test_tokio_rwlock_contention_with_avg_times() {
		let store = Arc::new(RwLock::new(std::collections::HashMap::<String, i32>::new()));
		let write_time_total = Arc::new(AtomicU32::new(0));
		let read_time_total = Arc::new(AtomicU32::new(0));
		let mut handles = vec![];

		// Writer threads: Insert 1000 entries.
		for i in 0..1000 {
			let store = store.clone();
			let write_time_total = write_time_total.clone();
			handles.push(tokio::spawn(async move {
				let key = format!("key_{}", i);
				let start = Instant::now();
				let mut map = store.write().await;
				map.insert(key, i * 10);
				let elapsed = start.elapsed().as_nanos() as u32;
				write_time_total.fetch_add(elapsed, Ordering::Relaxed);
			}));
		}

		// Allow writers to complete.
		for handle in handles {
			handle.await.unwrap();
		}

		println!(
			"Average write time: {} ns",
			write_time_total.load(Ordering::Relaxed) / 1000
		);

		handles = vec![];

		// Reader threads: Read those entries multiple times.
		for read_iter in 0..5 {
			for i in 0..1000 {
				let store = store.clone();
				let read_time_total = read_time_total.clone();
				handles.push(tokio::spawn(async move {
					let key = format!("key_{}", i);
					let start = Instant::now();
					let map = store.read().await;
					if let Some(value) = map.get(&key) {
						assert_eq!(*value, i * 10);
					}
					let elapsed = start.elapsed().as_nanos() as u32;
					read_time_total.fetch_add(elapsed, Ordering::Relaxed);
				}));
			}

			// Wait for all read operations to complete.
			for handle in handles.drain(..) {
				handle.await.unwrap();
			}

			println!(
				"Iteration {}: Average read time: {} ns",
				read_iter,
				read_time_total.load(Ordering::Relaxed) / 1000
			);

			// Reset the read time total for the next iteration.
			read_time_total.store(0, Ordering::Relaxed);
		}
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 12)]
	async fn test_dashmap_contention_with_avg_times() {
		let store = Arc::new(DashMap::<String, i32>::new());
		let write_time_total = Arc::new(AtomicU32::new(0));
		let read_time_total = Arc::new(AtomicU32::new(0));
		let mut handles = vec![];

		// Writer threads: Insert 1000 entries.
		for i in 0..1000 {
			let store = store.clone();
			let write_time_total = write_time_total.clone();
			handles.push(task::spawn(async move {
				let key = format!("key_{}", i);
				let start = Instant::now();
				store.insert(key, i * 10);
				let elapsed = start.elapsed().as_nanos() as u32;
				write_time_total.fetch_add(elapsed, Ordering::Relaxed);
			}));
		}

		// Wait for all writes to complete.
		for handle in handles {
			handle.await.unwrap();
		}

		println!(
			"Average write time: {} ns",
			write_time_total.load(Ordering::Relaxed) / 1000
		);

		// Reset handles for reading.
		handles = vec![];

		// Reader threads: Read those entries multiple times.
		for read_iter in 0..5 {
			for i in 0..1000 {
				let store = store.clone();
				let read_time_total = read_time_total.clone();
				handles.push(task::spawn(async move {
					let key = format!("key_{}", i);
					let start = Instant::now();
					if let Some(value) = store.get(&key) {
						assert_eq!(*value, i * 10);
					}
					let elapsed = start.elapsed().as_nanos() as u32;
					read_time_total.fetch_add(elapsed, Ordering::Relaxed);
				}));
			}

			// Wait for all read operations to complete.
			for handle in handles.drain(..) {
				handle.await.unwrap();
			}

			println!(
				"Iteration {}: Average read time: {} ns",
				read_iter,
				read_time_total.load(Ordering::Relaxed) / 1000
			);

			// Reset the read time total for the next iteration.
			read_time_total.store(0, Ordering::Relaxed);
		}
	}
}
