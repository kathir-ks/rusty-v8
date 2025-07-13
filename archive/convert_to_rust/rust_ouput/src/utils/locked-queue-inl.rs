// Converted from V8 C++ source files:
// Header: locked-queue-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::mem::MaybeUninit;

//use crate::src::base::atomic_utils;
//use crate::src::utils::allocation::Malloced;
//use crate::src::utils::locked_queue::LockedQueue;

mod base {
    pub mod atomic_utils {
        // Placeholder for atomic utilities if needed
    }
}

mod utils {
    pub mod allocation {
        pub trait Malloced {}
    }
    pub mod locked_queue {
        // Definition of LockedQueue (skeleton)
        pub struct LockedQueue<T> {
            head_: *mut LockedQueueNode<T>,
            tail_: *mut LockedQueueNode<T>,
            size_: AtomicUsize,
            head_mutex_: Mutex<()>,
            tail_mutex_: Mutex<()>,
        }

        impl<T> LockedQueue<T> {
            pub fn new() -> Self {
                LockedQueue {
                    head_: std::ptr::null_mut(),
                    tail_: std::ptr::null_mut(),
                    size_: AtomicUsize::new(0),
                    head_mutex_: Mutex::new(()),
                    tail_mutex_: Mutex::new(()),
                }
            }
        }

        struct LockedQueueNode<T> {
            value: MaybeUninit<T>,
            next: AtomicPtr<LockedQueueNode<T>>,
        }
    }
}

use utils::allocation::Malloced;
use utils::locked_queue::LockedQueue;
use utils::locked_queue::LockedQueueNode;


struct Node<Record> {
    value: Record,
    next: AtomicPtr<Node<Record>>,
}

impl<Record> Node<Record> {
    fn new(value: Record) -> Box<Self> {
        Box::new(Node {
            value,
            next: AtomicPtr::new(std::ptr::null_mut()),
        })
    }
}

impl<Record> Malloced for Node<Record> {}

impl<Record> LockedQueue<Record> {
    pub fn new() -> Self {
        let head = Box::new(Node {
            value: unsafe { MaybeUninit::zeroed().assume_init() }, //  Initialize with a placeholder
            next: AtomicPtr::new(std::ptr::null_mut()),
        });

        let head_ptr = Box::into_raw(head);

        LockedQueue {
            head_: head_ptr,
            tail_: head_ptr,
            size_: AtomicUsize::new(0),
            head_mutex_: Mutex::new(()),
            tail_mutex_: Mutex::new(()),
        }
    }

    pub fn enqueue(&self, record: Record) {
        let new_node = Node::new(record);
        let new_node_ptr = Box::into_raw(new_node);

        let _guard = self.tail_mutex_.lock().unwrap();
        self.size_.fetch_add(1, Ordering::SeqCst);

        unsafe {
            (*self.tail_).next.store(new_node_ptr, Ordering::SeqCst);
            self.tail_ = new_node_ptr;
        }
    }

    pub fn dequeue(&self, record: &mut MaybeUninit<Record>) -> bool {
        let _guard = self.head_mutex_.lock().unwrap();

        let old_head = self.head_;
        unsafe {
            let next_node_ptr = (*old_head).next.load(Ordering::SeqCst);
            if next_node_ptr.is_null() {
                return false;
            }

            std::ptr::copy_nonoverlapping(
                &mut (*next_node_ptr).value as *mut MaybeUninit<Record>,
                record as *mut MaybeUninit<Record>,
                std::mem::size_of::<Record>(),
            );

            self.head_ = next_node_ptr;

            let old_size = self.size_.fetch_sub(1, Ordering::SeqCst);
            assert!(old_size > 0);

            drop(Box::from_raw(old_head)); // Clean up old head
        }

        return true;
    }

    pub fn is_empty(&self) -> bool {
        let _guard = self.head_mutex_.lock().unwrap();
        unsafe { (*self.head_).next.load(Ordering::SeqCst).is_null() }
    }

    pub fn peek(&self, record: &mut MaybeUninit<Record>) -> bool {
        let _guard = self.head_mutex_.lock().unwrap();
        unsafe {
            let next_node_ptr = (*self.head_).next.load(Ordering::SeqCst);
            if next_node_ptr.is_null() {
                return false;
            }

            std::ptr::copy_nonoverlapping(
                &mut (*next_node_ptr).value as *mut MaybeUninit<Record>,
                record as *mut MaybeUninit<Record>,
                std::mem::size_of::<Record>(),
            );

            return true;
        }
    }

    pub fn size(&self) -> usize {
        self.size_.load(Ordering::SeqCst)
    }
}

impl<Record> Drop for LockedQueue<Record> {
    fn drop(&mut self) {
        // Destroy all remaining nodes. Note that we do not destroy the actual values.
        unsafe {
            let mut cur_node = self.head_;
            while !cur_node.is_null() {
                let old_node = cur_node;
                cur_node = (*cur_node).next.load(Ordering::SeqCst);
                drop(Box::from_raw(old_node));
            }
        }
    }
}
