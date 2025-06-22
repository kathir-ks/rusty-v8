pub mod base {
    /// Traits for a doubly-threaded list.
    pub trait DoublyThreadedListTraits<T> {
        /// Returns a mutable pointer to the "prev" field of the node.
        fn prev(t: &T) -> *mut *mut T;
        /// Returns an immutable pointer to the "next" field of the node.
        fn next(t: &T) -> *mut T;
        /// Returns true if the node is non-null.
        fn non_empty(t: *mut T) -> bool;
    }

    /// Default implementation of `DoublyThreadedListTraits`.
    pub struct DefaultDoublyThreadedListTraits {}

    impl<T> DoublyThreadedListTraits<T> for DefaultDoublyThreadedListTraits {
        fn prev(t: &T) -> *mut *mut T {
            unsafe { t.prev_ptr() }
        }
        fn next(t: &T) -> *mut T {
            unsafe { t.next_ptr() }
        }
        fn non_empty(t: *mut T) -> bool {
            !t.is_null()
        }
    }

    /// An intrusive doubly-linked list that threads through its nodes.
    pub struct DoublyThreadedList<T, DTLTraits = DefaultDoublyThreadedListTraits>
    where
        DTLTraits: DoublyThreadedListTraits<T>,
    {
        head: *mut T,
        _phantom: std::marker::PhantomData<DTLTraits>,
    }

    impl<T> DoublyThreadedList<T, DefaultDoublyThreadedListTraits> {
        pub fn new() -> Self {
            DoublyThreadedList {
                head: std::ptr::null_mut(),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, DTLTraits> DoublyThreadedList<T, DTLTraits>
    where
        DTLTraits: DoublyThreadedListTraits<T>,
    {
        /// Since C++17, it is possible to have a sentinel end-iterator that is not an
        /// iterator itself.
        pub struct EndIterator {}

        /// Iterator for `DoublyThreadedList`.
        pub struct Iterator<'a, T> {
            curr: *mut T,
            _phantom: std::marker::PhantomData<&'a T>,
        }

        impl<'a, T> Iterator<'a, T> {
            /// Creates a new iterator starting from the given head.
            pub fn new(head: *mut T) -> Self {
                Iterator {
                    curr: head,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<'a, T> std::iter::Iterator for Iterator<'a, T> {
            type Item = *mut T;

            fn next(&mut self) -> Option<Self::Item> {
                if DTLTraits::non_empty(self.curr) {
                    let current = self.curr;
                    unsafe {
                        self.curr = *(DTLTraits::next(&*self.curr));
                    }
                    Some(current)
                } else {
                    None
                }
            }
        }

        /// Removes `x` from the list. Iterators that are currently on `x` are
        /// invalidated. To remove while iterating, use `remove_at`.
        pub fn remove(x: &mut T) {
            unsafe {
                if (*DTLTraits::prev(x)).is_null() {
                    assert!((*(DTLTraits::next(x))).is_null());
                    // {x} already removed from the list.
                    return;
                }
                let prev: *mut *mut T = DTLTraits::prev(x);
                let next: *mut T = DTLTraits::next(x);

                **prev = *next;

                if DTLTraits::non_empty(*next) {
                    *DTLTraits::prev(&mut **next) = *prev;
                }

                *DTLTraits::prev(x) = std::ptr::null_mut();
                *DTLTraits::next(x) = std::ptr::null_mut();
            }
        }

        /// Add `x` at the beginning of the list. `x` will not be visible to any
        /// existing iterator. Does not invalidate any existing iterator.
        pub fn push_front(&mut self, x: &mut T) {
            unsafe {
                assert!((*(DTLTraits::next(x))).is_null());
                assert_eq!(*DTLTraits::prev(x), std::ptr::null_mut());
                *DTLTraits::next(x) = self.head;
                *DTLTraits::prev(x) = &mut self.head;

                if DTLTraits::non_empty(self.head) {
                    *DTLTraits::prev(&mut *self.head) = DTLTraits::next(x);
                }
                self.head = x;
            }
        }

        /// Returns the first element of the list.
        pub fn front(&self) -> Option<*mut T> {
            if self.is_empty() {
                None
            } else {
                Some(self.begin().next().unwrap()) // Corrected to use iterator and Option
            }
        }

        /// Removes the first element of the list.
        pub fn pop_front(&mut self) {
            if let Some(front) = self.front() {
                unsafe {
                    Self::remove(&mut *front);
                }
            }
        }

        /// Returns true if the list is empty.
        pub fn is_empty(&self) -> bool {
            !DTLTraits::non_empty(self.head)
        }

        /// Returns an iterator to the beginning of the list.
        pub fn begin(&self) -> Iterator<'_, T> {
            Iterator::new(self.head)
        }

        /// Returns an end iterator for the list.
        pub fn end(&self) -> EndIterator {
            EndIterator {}
        }

        /// Removes the element at `it`, and make `it` point to the next element.
        /// Iterators on the same element as `it` are invalidated. Other iterators are
        /// not affected.
        pub fn remove_at<'a>(&mut self, it: &mut Iterator<'a, T>) -> Iterator<'a, T> {
            assert!(DTLTraits::non_empty(it.curr));
            let curr = it.curr;
            unsafe {
                let next = *(DTLTraits::next(&*curr));
                Self::remove(&mut *curr);
                Iterator::new(next)
            }
        }

        // The `Contains` method cannot be directly translated because it relies on a
        // `in_use` function in the `DTLTraits` which depends on the internal state
        // of the `T` type and cannot be generally defined.

        pub fn contains_slow(&self, needle: &T) -> bool {
            for element in self.begin() {
                unsafe {
                    if *element == *needle {
                        return true;
                    }
                }
            }
            false
        }
    }

    impl<T, DTLTraits> Drop for DoublyThreadedList<T, DTLTraits>
    where
        DTLTraits: DoublyThreadedListTraits<T>,
    {
        fn drop(&mut self) {
            // This is intentionally left empty because the list holds raw pointers and
            // does not own the elements, so there is nothing to drop.
        }
    }

    // Added unsafe traits to the types that will be used in the tests

    pub trait DoublyLinkedListNode {
        unsafe fn prev_ptr(&self) -> *mut *mut Self;
        unsafe fn next_ptr(&self) -> *mut Self;
    }
}