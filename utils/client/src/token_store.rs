use std::{fmt, future::Future, hash::Hash, sync::Arc};

use counter::CounterU32;
pub use rustc_hash::FxBuildHasher;
use scc::HashMap as SccReadHashMap;
use thiserror::Error;
use tokio::sync::Notify;

const HASH_MAP_CAPACITY: usize = 1024;

// A global atomic counter for generating unique IDs for TokenState instances.
static TOKEN_COUNTER: CounterU32 = CounterU32::new();

/// Represents the internal state of a token.
#[derive(Clone)]
pub(crate) enum TokenStateInner<V> {
	Ready(V),              // Token is ready with a value.
	Updating(Arc<Notify>), // Token is being updated, and `Notify` is used to signal completion.
	Failed,                // Token update failed.
}

/// Represents the state of a token with a unique ID and an internal state.
pub(crate) struct TokenState<V> {
	id: u32,                   // Unique identifier for the token.
	state: TokenStateInner<V>, // Internal state of the token.
}

impl<V> TokenState<V> {
	/// Creates a new `TokenState` with a `Ready` value.
	pub(crate) fn new_ready(value: V) -> Self {
		let id = TOKEN_COUNTER.increment();
		let state = TokenStateInner::Ready(value);
		Self { id, state }
	}

	/// Creates a new `TokenState` in the `Updating` state and returns it along
	/// with the `Notify` instance.
	pub(crate) fn new(notify: Arc<Notify>) -> Self {
		let id = TOKEN_COUNTER.increment();
		// let notify = Arc::new(Notify::new());
		Self {
			id,
			state: TokenStateInner::Updating(notify.clone()),
		}
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
	map: SccReadHashMap<K, Arc<TokenState<V>>, FxBuildHasher>, /* Concurrent hash map storing
	                                                            * token states. */
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
			map: SccReadHashMap::with_capacity_and_hasher(HASH_MAP_CAPACITY, FxBuildHasher),
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
	where
		E: std::error::Error + Send + Sync + 'static,
	{
		let notify = Arc::new(Notify::new());
		let new_token_state = TokenState::<V>::new(notify.clone());
		let id = new_token_state.id;

		let entry = self.map.get_async(&key).await;
		match entry {
			Some(val) if val.as_ref().is_updating() => {
				return Err(StoreError::UpdateAlredyInProgress(val.as_ref().id));
			}
			_ => (),
		};
		drop(entry); // Drop the reference to avoid deadlocks.

		self.map
			.upsert_async(key.clone(), Arc::new(new_token_state))
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

	use super::*;

	#[tokio::test]
	async fn test_token_state_ready() {
		let value = 42;
		let token_state = TokenState::new_ready(value);
		assert!(token_state.is_ready());
		assert!(!token_state.is_updating());
	}

	#[tokio::test]
	async fn test_token_state_updating() {
		let notify = Arc::new(Notify::new());
		let token_state = TokenState::<i32>::new(notify.clone());
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
}
