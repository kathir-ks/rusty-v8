// Copyright 2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::MaybeUninit;

const PROCESSOR_CACHE_LINE_SIZE: usize = 64;

mod internal {
    use super::*;

    /// Lock-free cache-friendly sampling circular queue for large records.
    /// Intended for fast transfer of large records between a single producer
    /// and a single consumer. If the queue is full, `start_enqueue` will
    /// return `None`. The queue is designed with a goal in mind to evade
    /// cache lines thrashing by preventing simultaneous reads and writes to
    /// adjacent memory locations.
    pub struct SamplingCircularQueue<T, const LENGTH: usize> {
        buffer: [Entry<T>; LENGTH],
        enqueue_pos: CacheAligned<*mut Entry<T>>,
        dequeue_pos: CacheAligned<*mut Entry<T>>,
    }

    impl<T, const LENGTH: usize> SamplingCircularQueue<T, LENGTH> {
        /// Executed on the application thread.
        pub fn new() -> Self {
            let mut buffer: [MaybeUninit<Entry<T>>; LENGTH] = MaybeUninit::uninit_array();
            for elem in buffer.iter_mut() {
                elem.write(Entry::new());
            }

            // SAFETY: buffer is now fully initialized.
            let buffer: [Entry<T>; LENGTH] = unsafe { std::mem::transmute_copy(&buffer) };

            let enqueue_pos = CacheAligned::new(&buffer[0] as *const Entry<T> as *mut Entry<T>);
            let dequeue_pos = CacheAligned::new(&buffer[0] as *const Entry<T> as *mut Entry<T>);

            SamplingCircularQueue {
                buffer,
                enqueue_pos,
                dequeue_pos,
            }
        }

        /// Executed on the application thread.
        pub fn start_enqueue(&self) -> Option<&mut T> {
            let current_enqueue_pos = unsafe { &mut *self.enqueue_pos.value };
            if current_enqueue_pos.marker.load(Ordering::Acquire) == kFull {
                return None;
            }

            Some(&mut current_enqueue_pos.record)
        }

        /// Notifies the queue that the producer has complete writing data into the
        /// memory returned by `start_enqueue` and it can be passed to the consumer.
        pub fn finish_enqueue(&self) {
            let current_enqueue_pos = unsafe { &mut *self.enqueue_pos.value };
            current_enqueue_pos.marker.store(kFull, Ordering::Release);

            let next_enqueue_pos = self.next(current_enqueue_pos);
            self.enqueue_pos.value = next_enqueue_pos;
        }

        /// Executed on the consumer (analyzer) thread.
        /// Retrieves, but does not remove, the head of this queue, returning `None`
        /// if this queue is empty. After the record had been read by a consumer,
        /// `remove` must be called.
        pub fn peek(&self) -> Option<&T> {
            let current_dequeue_pos = unsafe { &mut *self.dequeue_pos.value };
            if current_dequeue_pos.marker.load(Ordering::Acquire) == kEmpty {
                return None;
            }

            Some(&current_dequeue_pos.record)
        }

        pub fn remove(&self) {
            let current_dequeue_pos = unsafe { &mut *self.dequeue_pos.value };
            current_dequeue_pos.marker.store(kEmpty, Ordering::Release);

            let next_dequeue_pos = self.next(current_dequeue_pos);
            self.dequeue_pos.value = next_dequeue_pos;
        }

        fn next(&self, entry: *mut Entry<T>) -> *mut Entry<T> {
            let entry_ptr = entry as *const Entry<T>;

            let entry_index = unsafe {
                self.buffer.as_ptr().sub_ptr(entry_ptr)
            };

            let next_index = (entry_index + 1) % LENGTH;
            &self.buffer[next_index] as *const Entry<T> as *mut Entry<T>
        }
    }

    const kEmpty: i32 = 0;
    const kFull: i32 = 1;

    #[repr(align(64))]
    struct Entry<T> {
        record: T,
        marker: AtomicI32,
    }

    impl<T> Entry<T> {
        fn new() -> Self {
            // Initialize record with a default value if possible.
            // Otherwise, caller must initialize before using.
            let record = unsafe { std::mem::zeroed() };
            Entry {
                record,
                marker: AtomicI32::new(kEmpty),
            }
        }
    }

    #[repr(align(64))]
    struct CacheAligned<T> {
        value: T,
    }

    impl<T> CacheAligned<T> {
        const fn new(value: T) -> Self {
            CacheAligned { value }
        }
    }

    impl<T, const LENGTH: usize> Drop for SamplingCircularQueue<T, LENGTH> {
        fn drop(&mut self) {
            // No need to drop elements individually as MaybeUninit doesn't own them.
            // If T needs to be dropped, it must be handled separately.
        }
    }

} // namespace internal
   // No direct equivalent for v8 namespace in Rust