use std::{collections::HashMap, sync::Arc, time::Instant};

use client::token_store::{StoreError, TokenStore};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dashmap::DashMap;
use tokio::{
	runtime::{Builder, Runtime},
	sync::RwLock,
	task,
};

async fn tokenstore_write_operation(
	store: Arc<TokenStore<String, usize>>,
	key: String,
	value: usize,
) {
	store
		.set(key.clone(), async move { Ok::<usize, StoreError>(value) })
		.await
		.unwrap();
}

async fn tokenstore_read_operation(
	store: Arc<TokenStore<String, usize>>,
	key: &String,
	expected_value: usize,
) {
	let entry = store.get(key).await.unwrap();
	assert!(entry.is_some());
	assert_eq!(*entry.unwrap().get(), expected_value);
}

// DashMap operations
fn dashmap_write_operation(
	store: Arc<DashMap<String, usize>>,
	key: String,
	value: usize,
) {
	store.insert(key, value);
}

fn dashmap_read_operation(
	store: Arc<DashMap<String, usize>>,
	key: &String,
	expected_value: usize,
) {
	let entry = store.get(key).expect("Key not found");
	assert_eq!(*entry.value(), expected_value);
}

// RwLock<HashMap> operations
async fn rwlock_write_operation(
	store: Arc<RwLock<HashMap<String, usize>>>,
	key: String,
	value: usize,
) {
	let mut guard = store.write().await;
	guard.insert(key, value);
}

async fn rwlock_read_operation(
	store: Arc<RwLock<HashMap<String, usize>>>,
	key: &String,
	expected_value: usize,
) {
	let guard = store.read().await;
	let entry = guard.get(key).expect("Key not found");
	assert_eq!(*entry, expected_value);
}

fn build_runtime() -> Runtime {
	Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
}
// Macro for generic benchmark function
macro_rules! read_write_benchmark {
	($c:expr, $name:expr, $store:expr, $clone_fn:expr, $write_fn:expr, $read_fn:expr) => {
		$c.bench_function($name, |b| {
			b.iter_custom(|iters| {
				let store = $store.clone();
				let runtime = build_runtime();
				runtime.block_on(async {
					let mut tasks = Vec::new();

					for i in 0..iters {
						let store = $clone_fn(&store);
						let key = format!("key_{}", i);
						tasks.push(task::spawn(async move {
							$write_fn(store.clone(), key.clone(), i as usize).await;

							let read_start = Instant::now();
							black_box($read_fn(store, &key, i as usize).await);
							read_start.elapsed()
						}));
					}

					// Collect all elapsed times from tasks
					futures::future::join_all(tasks)
						.await
						.into_iter()
						.map(|r| r.unwrap())
						.sum()
				})
			});
		});
	};
}

macro_rules! read_benchmark {
	($c:expr, $name:expr, $store:expr, $clone_fn:expr, $write_fn:expr, $read_fn:expr) => {
		$c.bench_function($name, |b| {
			b.iter_custom(|iters| {
				let store = $store.clone();
				let runtime = build_runtime();
				runtime.block_on(async {
					// Ensure all keys are written before reading
					for i in 0..iters {
						let key = format!("key_{}", i);
						$write_fn($clone_fn(&store), key.clone(), i as usize).await;
					}

					let mut tasks = Vec::new();

					for i in 0..iters {
						let store = $clone_fn(&store);
						let key = format!("key_{}", i);
						tasks.push(task::spawn(async move {
							let start = Instant::now();
							black_box($read_fn(store, &key, i as usize).await);
							start.elapsed()
						}));
					}

					futures::future::join_all(tasks)
						.await
						.into_iter()
						.map(|r| r.unwrap())
						.sum()
				})
			});
		});
	};
}

macro_rules! write_benchmark {
	($c:expr, $name:expr, $store:expr, $clone_fn:expr, $write_fn:expr) => {
		$c.bench_function($name, |b| {
			b.iter_custom(|iters| {
				let store = $store.clone();
				let runtime = build_runtime();
				runtime.block_on(async {
					let mut tasks = Vec::new();

					for i in 0..iters {
						let store = $clone_fn(&store);
						let key = format!("key_{}", i);
						tasks.push(task::spawn(async move {
							let start = Instant::now();
							black_box($write_fn(store, key.clone(), i as usize).await);
							start.elapsed()
						}));
					}

					// Collect all elapsed times from tasks
					futures::future::join_all(tasks)
						.await
						.into_iter()
						.map(|r| r.unwrap())
						.sum()
				})
			});
		});
	};
}

/// Spawns multiple tasks onto runtime. In each task it writes to the store, and
/// then reads it. It only benchmarks the read operation.
fn bench_read_write_with_runtime(c: &mut Criterion) {
	// TokenStore benchmark
	let tokenstore = Arc::new(TokenStore::<String, usize>::new());
	read_write_benchmark!(
		c,
		"tokenstore_multi_spawned_read_write_operation",
		tokenstore,
		Arc::clone,
		tokenstore_write_operation,
		tokenstore_read_operation
	);

	// DashMap benchmark
	let dashmap = Arc::new(DashMap::new());
	read_write_benchmark!(
		c,
		"dashmap_multi_spawned_read_write_operation",
		dashmap,
		Arc::clone,
		|store, key, value| async move { dashmap_write_operation(store, key, value) },
		|store, key, value| async move { dashmap_read_operation(store, key, value) }
	);

	// RwLock<HashMap> benchmark
	let rwlock_map = Arc::new(RwLock::new(HashMap::new()));
	read_write_benchmark!(
		c,
		"rwlock_multi_spawned_read_write_operation",
		rwlock_map,
		Arc::clone,
		rwlock_write_operation,
		rwlock_read_operation
	);
}

/// Benchmarks only the read operations after pre-filling the store.
fn bench_read_with_runtime(c: &mut Criterion) {
	// TokenStore benchmark
	let tokenstore = Arc::new(TokenStore::<String, usize>::new());
	read_benchmark!(
		c,
		"tokenstore_read_operation_with_runtime",
		tokenstore,
		Arc::clone,
		tokenstore_write_operation,
		tokenstore_read_operation
	);

	// DashMap benchmark
	let dashmap = Arc::new(DashMap::new());
	read_benchmark!(
		c,
		"dashmap_read_operation_with_runtime",
		dashmap,
		Arc::clone,
		|store, key, value| async move { dashmap_write_operation(store, key, value) },
		|store, key, value| async move { dashmap_read_operation(store, key, value) }
	);

	// RwLock<HashMap> benchmark
	let rwlock_map = Arc::new(RwLock::new(HashMap::new()));
	read_benchmark!(
		c,
		"rwlock_read_operation_with_runtime",
		rwlock_map,
		Arc::clone,
		rwlock_write_operation,
		rwlock_read_operation
	);
}

/// Benchmarks only the write operations.
fn bench_write_with_runtime(c: &mut Criterion) {
	// TokenStore benchmark
	let tokenstore = Arc::new(TokenStore::<String, usize>::new());
	write_benchmark!(
		c,
		"tokenstore_write_operation_with_runtime",
		tokenstore,
		Arc::clone,
		tokenstore_write_operation
	);

	// DashMap benchmark
	let dashmap: Arc<DashMap<String, usize>> = Arc::new(DashMap::new());
	write_benchmark!(
		c,
		"dashmap_write_operation_with_runtime",
		dashmap,
		Arc::clone,
		|store, key, value| async move { dashmap_write_operation(store, key, value) }
	);

	// RwLock<HashMap> benchmark
	let rwlock_map: Arc<RwLock<HashMap<String, usize>>> = Arc::new(RwLock::new(HashMap::new()));
	write_benchmark!(
		c,
		"rwlock_write_operation_with_runtime",
		rwlock_map,
		Arc::clone,
		rwlock_write_operation
	);
}

fn criterion_benchmark(c: &mut Criterion) {
	bench_read_write_with_runtime(c);
	bench_read_with_runtime(c);
	bench_write_with_runtime(c);
}

criterion_group!(
	benches,
	criterion_benchmark,
);
criterion_main!(benches);
