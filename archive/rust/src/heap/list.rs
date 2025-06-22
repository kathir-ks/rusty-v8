// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap {
    use std::ptr::null_mut;

    #[derive(Default)]
    pub struct List<T> {
        front: *mut T,
        back: *mut T,
        size: usize,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                front: null_mut(),
                back: null_mut(),
                size: 0,
            }
        }

        pub fn push_back(&mut self, element: *mut T) {
            assert!((unsafe { (*element).list_node().next() }).is_null());
            assert!((unsafe { (*element).list_node().prev() }).is_null());
            if !self.back.is_null() {
                assert!(!self.front.is_null());
                self.insert_after(element, self.back);
            } else {
                self.add_first_element(element);
            }
            self.size += 1;
        }

        pub fn push_front(&mut self, element: *mut T) {
            assert!((unsafe { (*element).list_node().next() }).is_null());
            assert!((unsafe { (*element).list_node().prev() }).is_null());
            if !self.front.is_null() {
                assert!(!self.back.is_null());
                self.insert_before(element, self.front);
            } else {
                self.add_first_element(element);
            }
            self.size += 1;
        }

        pub fn remove(&mut self, element: *mut T) {
            assert!(self.contains(element));
            if self.back == element {
                self.back = unsafe { (*element).list_node().prev() };
            }
            if self.front == element {
                self.front = unsafe { (*element).list_node().next() };
            }

            let next = unsafe { (*element).list_node().next() };
            let prev = unsafe { (*element).list_node().prev() };

            if !next.is_null() {
                unsafe { (*next).list_node_mut().set_prev(prev) };
            }
            if !prev.is_null() {
                unsafe { (*prev).list_node_mut().set_next(next) };
            }
            unsafe { (*element).list_node_mut().set_prev(null_mut()) };
            unsafe { (*element).list_node_mut().set_next(null_mut()) };

            self.size -= 1;
        }

        pub fn contains(&self, element: *const T) -> bool {
            let mut it = self.front;
            while !it.is_null() {
                if it as *const T == element {
                    return true;
                }
                it = unsafe { (*it).list_node().next() };
            }
            false
        }

        pub fn is_empty(&self) -> bool {
            assert_eq!(self.size == 0, self.front.is_null());
            assert_eq!(self.size == 0, self.back.is_null());
            self.size == 0
        }

        pub fn front(&self) -> *mut T {
            self.front
        }

        pub fn back(&self) -> *mut T {
            self.back
        }

        pub fn size(&self) -> usize {
            self.size
        }

        fn add_first_element(&mut self, element: *mut T) {
            assert!(self.back.is_null());
            assert!(self.front.is_null());
            assert!((unsafe { (*element).list_node().next() }).is_null());
            assert!((unsafe { (*element).list_node().prev() }).is_null());

            unsafe { (*element).list_node_mut().set_prev(null_mut()) };
            unsafe { (*element).list_node_mut().set_next(null_mut()) };

            self.front = element;
            self.back = element;
        }

        fn insert_after(&mut self, element: *mut T, other: *mut T) {
            let other_next = unsafe { (*other).list_node().next() };

            unsafe { (*element).list_node_mut().set_next(other_next) };
            unsafe { (*element).list_node_mut().set_prev(other) };
            unsafe { (*other).list_node_mut().set_next(element) };

            if !other_next.is_null() {
                unsafe { (*other_next).list_node_mut().set_prev(element) };
            } else {
                self.back = element;
            }
        }

        fn insert_before(&mut self, element: *mut T, other: *mut T) {
            let other_prev = unsafe { (*other).list_node().prev() };

            unsafe { (*element).list_node_mut().set_next(other) };
            unsafe { (*element).list_node_mut().set_prev(other_prev) };
            unsafe { (*other).list_node_mut().set_prev(element) };

            if !other_prev.is_null() {
                unsafe { (*other_prev).list_node_mut().set_next(element) };
            } else {
                self.front = element;
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            // This drop implementation only deallocates the List struct itself.
            // It DOES NOT deallocate any of the T elements stored in the list.
            // Deallocating the T elements is the responsibility of the code that owns the List.
            self.front = null_mut();
            self.back = null_mut();
            self.size = 0;
        }
    }

    pub struct ListNode<T> {
        next: *mut T,
        prev: *mut T,
    }

    impl<T> ListNode<T> {
        pub fn new() -> Self {
            ListNode {
                next: null_mut(),
                prev: null_mut(),
            }
        }

        pub fn next(&self) -> *mut T {
            self.next
        }

        pub fn prev(&self) -> *mut T {
            self.prev
        }
    }

    impl<T> ListNode<T> {
        fn set_next(&mut self, next: *mut T) {
            self.next = next;
        }
        fn set_prev(&mut self, prev: *mut T) {
            self.prev = prev;
        }
    }
}