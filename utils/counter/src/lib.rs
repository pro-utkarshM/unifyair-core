use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

macro_rules! define_counter {
    ($counter_name:ident, $atomic_type:ident, $base_type:ty) => {
        #[derive(Debug)]
        pub struct $counter_name($atomic_type);

        impl $counter_name {
            pub const fn new() -> Self {
                $counter_name($atomic_type::new(1))
            }

            pub fn increment(&self) -> $base_type {
                self.0.fetch_add(1, Ordering::Relaxed) as $base_type
            }
        }
        impl Default for $counter_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

define_counter!(CounterU8, AtomicU8, u8);
define_counter!(CounterU16, AtomicU16, u16);
define_counter!(CounterU32, AtomicU32, u32);
define_counter!(CounterU64, AtomicU64, u64);
define_counter!(CounterUsize, AtomicUsize, usize);