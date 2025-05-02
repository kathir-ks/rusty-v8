// src/utils/locked_queue.rs

use std::sync::{Arc, Mutex, atomic::{AtomicPtr, AtomicUsize, Ordering}, MutexGuard};
use std::ptr;

/// A thread-safe queue that allows multiple producers and consumers.
pub struct LockedQueue<T> {
    head: Arc<AtomicPtr<Node<T>>>,
    tail: Arc<AtomicPtr<Node<T>>>,
    tail_mutex: Arc<Mutex<()>>,
    head_mutex: Arc<Mutex<()>>,
    size: AtomicUsize,
}

struct Node<T> {
    value: T,
    next: AtomicPtr<Node<T>>,
}

impl<T> LockedQueue<T> {
    /// Creates a new empty LockedQueue.
    pub fn new() -> Self {
        let head = Arc::new(AtomicPtr::new(Box::into_raw(Box::new(Node {
            value: unsafe { std::mem::zeroed() }, // Dummy value, will be overwritten.  Safe since head is never accessed directly for its value.
            next: AtomicPtr::new(ptr::null_mut()),
        }))));

        LockedQueue {
            head: head.clone(),
            tail: head.clone(),
            tail_mutex: Arc::new(Mutex::new(())),
            head_mutex: Arc::new(Mutex::new(())),
            size: AtomicUsize::new(0),
        }
    }

    /// Enqueues a new record into the queue.
    pub fn enqueue(&self, record: T) {
        let n = Box::new(Node {
            value: record,
            next: AtomicPtr::new(ptr::null_mut()),
        });
        let n_ptr = Box::into_raw(n);

        let _guard = self.tail_mutex.lock().unwrap();
        self.size.fetch_add(1, Ordering::SeqCst);
        unsafe {
            let tail_ptr = self.tail.load(Ordering::SeqCst);
            (*tail_ptr).next.store(n_ptr, Ordering::SeqCst);
            self.tail.store(n_ptr, Ordering::SeqCst);
        }
    }

    /// Dequeues a record from the queue. Returns `None` if the queue is empty.
    pub fn dequeue(&self) -> Option<T> {
        let _guard = self.head_mutex.lock().unwrap();
        let old_head_ptr = self.head.load(Ordering::SeqCst);
        unsafe {
            let next_node_ptr = (*old_head_ptr).next.load(Ordering::SeqCst);
            if next_node_ptr.is_null() {
                return None;
            }

            let next_node = Box::from_raw(next_node_ptr);
            let record = next_node.value;

            self.head.store(next_node_ptr, Ordering::SeqCst);

            self.size.fetch_sub(1, Ordering::SeqCst);
            drop(Box::from_raw(old_head_ptr));  //Free the old head.
            Some(record)
        }
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        let _guard = self.head_mutex.lock().unwrap();
        unsafe { (*self.head.load(Ordering::SeqCst)).next.load(Ordering::SeqCst).is_null() }
    }

    /// Peeks at the next record in the queue without removing it. Returns `None` if the queue is empty.
    pub fn peek(&self) -> Option<&T> {
        let _guard = self.head_mutex.lock().unwrap();
        unsafe {
            let next_node_ptr = (*self.head.load(Ordering::SeqCst)).next.load(Ordering::SeqCst);
            if next_node_ptr.is_null() {
                None
            } else {
                Some(&(*next_node_ptr).value)
            }
        }
    }

    /// Returns the number of elements in the queue.
    pub fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
}

impl<T> Drop for LockedQueue<T> {
    fn drop(&mut self) {
        // Destroy all remaining nodes. Note that we do not destroy the actual values.
        let mut cur_node_ptr = self.head.load(Ordering::SeqCst);
        unsafe {
            while !cur_node_ptr.is_null() {
                let cur_node = Box::from_raw(cur_node_ptr);
                cur_node_ptr = cur_node.next.load(Ordering::SeqCst);
            }
        }
    }
}

unsafe impl<T> Send for LockedQueue<T> {}
unsafe impl<T> Sync for LockedQueue<T> {}