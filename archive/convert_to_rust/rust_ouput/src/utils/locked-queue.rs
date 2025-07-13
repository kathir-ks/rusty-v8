// Converted from V8 C++ source files:
// Header: locked-queue.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

mod base {
    pub mod platform {
        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Mutex {
                Mutex { inner: std::sync::Mutex::new(()) }
            }

            pub fn lock(&self) {
                let _guard = self.inner.lock().unwrap();
            }

            pub fn unlock(&self) {} // MutexGuard handles unlocking
        }
    }
}

mod v8 {
    pub mod internal {

        pub struct LockedQueue<Record> {
            head_mutex_: base::platform::Mutex,
            tail_mutex_: base::platform::Mutex,
            head_: Arc<Node<Record>>,
            tail_: Arc<Node<Record>>,
            size_: AtomicUsize,
        }

        struct Node<Record> {
            data: Option<Record>,
            next: Mutex<Option<Arc<Node<Record>>>>,
        }

        impl<Record> LockedQueue<Record> {
            pub fn new() -> LockedQueue<Record> {
                let dummy_node = Arc::new(Node {
                    data: None,
                    next: Mutex::new(None),
                });
                LockedQueue {
                    head_mutex_: base::platform::Mutex::new(),
                    tail_mutex_: base::platform::Mutex::new(),
                    head_: dummy_node.clone(),
                    tail_: dummy_node,
                    size_: AtomicUsize::new(0),
                }
            }

            pub fn enqueue(&self, record: Record) {
                let new_node = Arc::new(Node {
                    data: Some(record),
                    next: Mutex::new(None),
                });

                self.tail_mutex_.lock();
                let tail = self.tail_.clone();
                let mut tail_next = tail.next.lock().unwrap();
                *tail_next = Some(new_node.clone());
                self.tail_ = new_node;
                self.tail_mutex_.unlock();

                self.size_.fetch_add(1, Ordering::SeqCst);
            }

            pub fn dequeue(&self, record: &mut Record) -> bool {
                self.head_mutex_.lock();
                let head = self.head_.clone();
                let head_next_mutex = head.next.lock().unwrap();
                let head_next = match &*head_next_mutex {
                    Some(node) => node.clone(),
                    None => {
                        self.head_mutex_.unlock();
                        return false;
                    }
                };
                
                if let Some(data) = head_next.data.as_ref() {
                    // need to copy since we are about to replace the head
                    *record = unsafe { std::ptr::read(data as *const Record) };
                }
                else{
                    self.head_mutex_.unlock();
                    return false;
                }
                
                let new_head = head_next.clone();
                let mut head_guard = self.head_.next.lock().unwrap();
                *head_guard = {
                    let next_next = new_head.next.lock().unwrap();
                    next_next.clone()
                };
                
                self.head_ = new_head;
                self.head_mutex_.unlock();
                self.size_.fetch_sub(1, Ordering::SeqCst);
                return true;
            }

            pub fn is_empty(&self) -> bool {
                self.size_.load(Ordering::SeqCst) == 0
            }

            pub fn peek(&self, record: &mut Record) -> bool {
                let head_next_mutex = self.head_.next.lock().unwrap();
                let head_next = match &*head_next_mutex {
                    Some(node) => node.clone(),
                    None => {
                        return false;
                    }
                };
                if let Some(data) = head_next.data.as_ref() {
                    *record = unsafe { std::ptr::read(data as *const Record) };
                    return true;
                }
                return false;
            }

            pub fn size(&self) -> usize {
                self.size_.load(Ordering::SeqCst)
            }
        }
    }
}
