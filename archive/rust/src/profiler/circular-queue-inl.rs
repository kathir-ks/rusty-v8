// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod circular_queue {
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::marker::PhantomData;

    const EMPTY: u8 = 0;
    const FULL: u8 = 1;

    /// A circular queue for sampling data.
    pub struct SamplingCircularQueue<T, const L: usize> {
        buffer: [Entry<T>; L],
        enqueue_pos: *mut Entry<T>,
        dequeue_pos: *mut Entry<T>,
        _phantom: PhantomData<T>,
    }

    #[repr(C)]
    struct Entry<T> {
        marker: AtomicU8,
        record: T,
    }

    impl<T, const L: usize> SamplingCircularQueue<T, L> {
        /// Creates a new SamplingCircularQueue.
        pub fn new() -> Self {
            let mut queue = Self {
                buffer: unsafe {
                    // Initialize the buffer with uninitialized Entry values.
                    // This is safe because we immediately overwrite the marker
                    // field in the Entry struct.
                    let mut arr: [std::mem::MaybeUninit<Entry<T>>; L] = std::mem::MaybeUninit::uninit().assume_init();
                    for elem in arr.iter_mut() {
                        std::ptr::write(elem.as_mut_ptr(), std::mem::MaybeUninit::zeroed().assume_init());
                    }

                    // Convert the array of MaybeUninit<Entry<T>> to [Entry<T>; L]
                    unsafe { std::mem::transmute_copy::<_, [Entry<T>; L]>(&arr) }
                },
                enqueue_pos: std::ptr::null_mut(),
                dequeue_pos: std::ptr::null_mut(),
                _phantom: PhantomData,
            };

            queue.enqueue_pos = &mut queue.buffer[0] as *mut Entry<T>;
            queue.dequeue_pos = &mut queue.buffer[0] as *mut Entry<T>;

            queue
        }

        pub fn initialize(&mut self) {
            self.enqueue_pos = &mut self.buffer[0] as *mut Entry<T>;
            self.dequeue_pos = &mut self.buffer[0] as *mut Entry<T>;
        }

        /// Returns a mutable reference to the next available element in the queue, or `None` if the queue is full.
        pub fn peek(&self) -> Option<&mut T> {
            std::sync::atomic::fence(Ordering::SeqCst);
            unsafe {
                if (*self.dequeue_pos).marker.load(Ordering::Acquire) == FULL {
                    Some(&mut (*self.dequeue_pos).record)
                } else {
                    None
                }
            }
        }

        /// Removes the element at the head of the queue.
        pub fn remove(&mut self) {
            unsafe {
                (*self.dequeue_pos).marker.store(EMPTY, Ordering::Release);
                self.dequeue_pos = self.next(self.dequeue_pos);
            }
        }

        /// Returns a mutable reference to the next available element for enqueuing, or `None` if the queue is full.
        pub fn start_enqueue(&self) -> Option<&mut T> {
            std::sync::atomic::fence(Ordering::SeqCst);
            unsafe {
                if (*self.enqueue_pos).marker.load(Ordering::Acquire) == EMPTY {
                    Some(&mut (*self.enqueue_pos).record)
                } else {
                    None
                }
            }
        }

        /// Marks the enqueued element as full and advances the enqueue position.
        pub fn finish_enqueue(&mut self) {
            unsafe {
                (*self.enqueue_pos).marker.store(FULL, Ordering::Release);
                self.enqueue_pos = self.next(self.enqueue_pos);
            }
        }

        /// Advances the given entry pointer to the next entry in the queue.
        unsafe fn next(&self, entry: *mut Entry<T>) -> *mut Entry<T> {
            let next = entry.add(1);
            if next == &self.buffer[L] as *const Entry<T> as *mut Entry<T> {
                &mut self.buffer[0] as *mut Entry<T>
            } else {
                next
            }
        }
    }
}