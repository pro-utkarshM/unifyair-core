#[cfg(not(feature = "loom-testing"))]
use std::sync::{
	atomic::{AtomicBool, AtomicPtr, Ordering},
	Arc,
};
use std::{
	fmt::Debug,
	mem::{self},
	ops::Deref,
};

#[cfg(feature = "loom-testing")]
use loom::sync::{
	atomic::{AtomicBool, AtomicPtr, Ordering},
	Arc,
};

pub struct AtomicReader<T: Debug> {
	status: AtomicBool,
	data: AtomicPtr<T>,
}

impl<T: Debug> Drop for AtomicReader<T> {
	fn drop(&mut self) {
		let status = self.status.load(Ordering::Acquire);
		if status {
			panic!("Cannot be dropped while writing")
		}
		let ptr: *mut T = self.data.load(Ordering::Relaxed);
		#[cfg(feature = "testing")]
		println!("ptr={:?}", ptr);
		let arc: *const Arc<T> = ptr as *const _;
		#[cfg(feature = "testing")]
		println!("arc={:?}", arc);
		let arc = unsafe { Arc::from_raw(ptr) };
		#[cfg(feature = "testing")]
		println!("strong-count = {:?}", Arc::strong_count(&arc));
		drop(arc);
	}
}

impl<T: Debug> AtomicReader<T> {
	pub fn new(t: T) -> Self {
		let arc = Arc::new(t);
		let arc_inner = Arc::into_raw(arc);
		AtomicReader {
			status: AtomicBool::new(false),
			data: AtomicPtr::new(arc_inner as *mut _),
		}
	}

	pub async fn read(&self) -> Arc<T> {
		let status = self.status.load(Ordering::Acquire);
		if status {
			// TODO: Use tokio Handle runtime to sleep. And wake it up later when write is
			// completed.
			panic!("Atomic Write Case not Implemented");
		}
		#[cfg(feature = "testing")]
		println!("borrow called");
		let ptr = self.data.load(Ordering::Relaxed);
		let arc: Arc<T> = unsafe { Arc::from_raw(ptr as *const _) };
		let arcx = arc.clone();
		// Prevent decreasing the counter for the arc stored in the Data Structure
		mem::forget(arc);
		#[cfg(feature = "testing")]
		println!("Arc Strong Count: {}", Arc::strong_count(&arcx));
		arcx
	}

	// TODO: Implement Asynchronorous version of the write here in order to udpate
	// the config Implement a Write Future Which internally manages a queue of all
	// the write operations. and then writes the lock.
	fn replace(
		&self,
		t: T,
	) {
		let status = self.status.swap(true, Ordering::Acquire);
		if status {
			panic!("Two Replace cannot be done together");
		}
		let mut arc = Arc::new(t);
		let arc_inner: *mut T = Arc::into_raw(arc) as *mut _;
		let old_ptr: *mut T = self.data.swap(arc_inner, Ordering::Relaxed);
		let old_arc = unsafe { Arc::from_raw(old_ptr) };
		#[cfg(feature = "testing")]
		println!(
			"old_arc={:?}, strong refs={}",
			old_arc,
			Arc::strong_count(&old_arc)
		);
		drop(old_arc);
		self.status.swap(false, Ordering::Release);
	}
}

#[cfg(all(test, feature = "tokio-testing"))]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_inner_arc_refcount() {
		let value = Arc::new(AtomicReader::new(42));

		// Initial read to check the reference count
		let initial_read = value.read().await;
		assert_eq!(Arc::strong_count(&initial_read), 2); // One for `initial_read` and one for `value`

		// Clone the Arc to simulate another thread holding a reference
		let cloned_read = initial_read.clone();
		assert_eq!(Arc::strong_count(&cloned_read), 3); // `value`, `initial_read`, and `cloned_read`

		drop(initial_read);
		assert_eq!(Arc::strong_count(&cloned_read), 2); // Dropped `initial_read`

		drop(cloned_read);
		let final_read = value.read().await;
		assert_eq!(Arc::strong_count(&final_read), 2); // One for `value` and one for `final_read`

		// Perform a replace operation
		value.replace(100);
		let new_read = value.read().await;
		assert_eq!(Arc::strong_count(&new_read), 2); // One for `value` and one for `new_read`

		// Ensure the old Arc is no longer referenced
		assert_eq!(Arc::strong_count(&new_read), 2);
	}

	#[test]
	fn bounds() {
		fn check_send<T: Send>() {}
		fn check_sync<T: Sync>() {}
		fn check_unpin<T: Unpin>() {}
		// This has to take a value, since the async fn's return type is unnameable.
		fn check_send_sync_val<T: Send + Sync>(_t: T) {}

		check_send::<AtomicReader<u32>>();
		check_sync::<AtomicReader<u32>>();
		check_unpin::<AtomicReader<u32>>();
	}

}

#[cfg(all(test, feature = "loom-testing"))]
mod tests {
	use super::*;
	use loom::thread;


	async fn read_value<T: Debug>(value: Arc<AtomicReader<T>>) -> Arc<T> {
		let value_clone = value.clone();
		let inner_arc = value.read().await;
		inner_arc
	}

	#[test]
	fn test_with_loom() {
		loom::model(|| {
			let x = Arc::new(1);
			println!("Mem Type: {}", std::any::type_name_of_val(&x));
			let value = Arc::new(AtomicReader::new(42));
			let value_clone = value.clone();
			let t1 = loom::thread::spawn(move || {
				loom::future::block_on(crate::tests::read_value(value_clone));
			});
			let value_clone = value.clone();
			let t2 = loom::thread::spawn(move || {
				loom::future::block_on(crate::tests::read_value(value_clone));
			});
			t1.join().unwrap();
			t2.join().unwrap();
			assert_eq!(Arc::strong_count(&value), 1);
		})
	}

	#[test]
	fn test_concurrent_read_and_write_with_loom() {
		loom::model(|| {
			let value = Arc::new(AtomicReader::new(42));

			let value_reader = Arc::clone(&value);
			let reader_thread = loom::thread::spawn(move || {
				let read_value = loom::future::block_on(crate::tests::read_value(value_reader));
				assert!(
					*read_value == 42 || *read_value == 100,
					"Reader must see consistent values"
				);
			});

			let value_writer = Arc::clone(&value);
			let writer_thread = loom::thread::spawn(move || {
				value_writer.replace(100);
			});

			reader_thread.join().unwrap();
			writer_thread.join().unwrap();

			// Ensure the final value is updated correctly
			let final_value = loom::future::block_on(crate::tests::read_value(Arc::clone(&value)));
			assert_eq!(*final_value, 100);
		});
	}

	#[test]
	fn test_multiple_concurrent_writes_with_loom() {
		loom::model(|| {
			let value = Arc::new(AtomicReader::new(42));

			let value_writer1 = Arc::clone(&value);
			let writer1 = loom::thread::spawn(move || {
				value_writer1.replace(100);
			});

			let value_writer2 = Arc::clone(&value);
			let writer2 = loom::thread::spawn(move || {
				// This should panic as concurrent writes are not allowed
				assert!(
					std::panic::catch_unwind(|| value_writer2.replace(200)).is_err(),
					"Concurrent writes must not be allowed"
				);
			});

			writer1.join().unwrap();
			writer2.join().unwrap();

			// Final value should reflect only one successful write
			let final_value = loom::future::block_on(crate::tests::read_value(Arc::clone(&value)));
			assert_eq!(*final_value, 100);
		});
	}
}
