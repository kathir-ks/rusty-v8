// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};

/// Simple lock-based unbounded size queue (multi producer; multi consumer) based
/// on "Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue
/// Algorithms" by M. Scott and M. Michael.
/// See:
/// https://www.cs.rochester.edu/research/synchronization/pseudocode/queues.html
pub struct LockedQueue<T> {
    head_mutex: Mutex<()>,
    tail_mutex: Mutex<()>,
    head: Arc<Node<T>>,
    tail: Arc<Node<T>>,
    size: AtomicUsize,
}

struct Node<T> {
    data: Option<T>,
    next: Mutex<Option<Arc<Node<T>>>>,
}

impl<T> LockedQueue<T> {
    /// Creates a new, empty `LockedQueue`.
    pub fn new() -> Self {
        let dummy = Arc::new(Node {
            data: None,
            next: Mutex::new(None),
        });
        LockedQueue {
            head_mutex: Mutex::new(()),
            tail_mutex: Mutex::new(()),
            head: dummy.clone(),
            tail: dummy,
            size: AtomicUsize::new(0),
        }
    }

    /// Adds an element to the back of the queue.
    pub fn enqueue(&self, record: T) {
        let node = Arc::new(Node {
            data: Some(record),
            next: Mutex::new(None),
        });

        let _tail_lock = self.tail_mutex.lock().unwrap();
        let tail = Arc::clone(&self.tail);
        *tail.next.lock().unwrap() = Some(node.clone());
        self.tail = node;
        self.size.fetch_add(1, Ordering::SeqCst);
    }

    /// Attempts to remove an element from the front of the queue.
    ///
    /// Returns `true` if an element was successfully removed and placed into
    /// `record`. Returns `false` if the queue is empty.
    pub fn dequeue(&self, record: &mut T) -> bool {
        let _head_lock = self.head_mutex.lock().unwrap();
        let head = Arc::clone(&self.head);
        let next = head.next.lock().unwrap().take();

        match next {
            Some(node) => {
                if let Some(data) = Arc::try_unwrap(head).ok().and_then(|n| n.data) {
                    //drop(data); // Explicitly drop the data from the dummy node
                }
                self.head = node.clone();
                if let Some(val) = node.data {
                  *record = val;
                } else {
                  return false;
                }
                self.size.fetch_sub(1, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }

    /// Returns `true` if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.size.load(Ordering::SeqCst) == 0
    }

    /// Attempts to peek at the front element of the queue without removing it.
    ///
    /// Returns `true` if an element was successfully peeked and placed into
    /// `record`. Returns `false` if the queue is empty.
    pub fn peek(&self, record: &mut T) -> bool {
      let _head_lock = self.head_mutex.lock().unwrap();
        let head = Arc::clone(&self.head);
      let next_lock = head.next.lock().unwrap();
      match next_lock.as_ref() {
          Some(node) => {
            if let Some(val) = &node.data {
              *record = val.clone();
              true
            } else {
              false
            }
          }
          None => false,
      }
    }

    /// Returns the number of elements in the queue.
    pub fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
}

impl<T> Default for LockedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}