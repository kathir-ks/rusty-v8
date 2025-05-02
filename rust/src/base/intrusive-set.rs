// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod intrusive_set {
    use std::{
        iter::Iterator,
        marker::PhantomData,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct IntrusiveSetIndex {
        value: usize,
    }

    impl IntrusiveSetIndex {
        const NOT_IN_SET: usize = usize::MAX;

        pub fn new() -> Self {
            IntrusiveSetIndex {
                value: Self::NOT_IN_SET,
            }
        }

        pub fn is_in_set(&self) -> bool {
            self.value != Self::NOT_IN_SET
        }
    }

    impl Default for IntrusiveSetIndex {
        fn default() -> Self {
            Self::new()
        }
    }

    /// A set of pointer-like values (`T`) that point to memory containing the
    /// position inside of the set (`IntrusiveSetIndex`), to allow for O(1) insertion
    /// and removal without using a hash table. This set is intrusive in the sense
    /// that elements need to know their position inside of the set by storing an
    /// `IntrusiveSetIndex` somewhere. In particular, all copies of a `T` value
    /// should point to the same `IntrusiveSetIndex` instance. `GetIntrusiveSetIndex`
    /// has to be a functor that produces `IntrusiveSetIndex&` given a `T`. The
    /// reference has to remain valid and refer to the same memory location while the
    /// element is in the set and until we finish iterating over the data structure
    /// if the element is removed during iteration.
    ///
    /// Add(T):     amortized O(1)
    /// Contain(T): O(1)
    /// Remove(T):  O(1)
    pub struct IntrusiveSet<T, F, C>
    where
        T: Copy,
        F: Fn(&T) -> &mut IntrusiveSetIndex,
        C: DerefMut + Deref<Target = Vec<T>>,
    {
        elements: C,
        index_functor: F,
        _phantom: PhantomData<T>,
    }

    impl<T, F, C> IntrusiveSet<T, F, C>
    where
        T: Copy,
        F: Fn(&T) -> &mut IntrusiveSetIndex,
        C: DerefMut + Deref<Target = Vec<T>>,
    {
        pub fn new(container: C, index_functor: F) -> Self {
            // This is not needed for soundness, but rather serves as a hint that `T`
            // should be a lightweight pointer-like value.
            //static_assert(std::is_trivially_copyable_v<T>);
            IntrusiveSet {
                elements: container,
                index_functor,
                _phantom: PhantomData,
            }
        }

        pub fn contains(&self, x: &T) -> bool {
            self.index(x).value != IntrusiveSetIndex::NOT_IN_SET
        }

        /// Adding elements while iterating is allowed.
        pub fn add(&mut self, x: T) {
            assert!(!self.contains(&x));
            self.index_mut(&x).value = self.elements.len();
            self.elements.push(x);
        }

        /// Removing while iterating is allowed under very specific circumstances. See
        /// comment on `IntrusiveSet::iterator`.
        pub fn remove(&mut self, x: &T) {
            assert!(self.contains(x));
            let index = self.index(x).value;
            assert_eq!(*x, self.elements[index]);
            self.index_mut(&self.elements.last().copied().unwrap()).value = index;
            self.elements.swap_remove(index);
            self.index_mut(x).value = IntrusiveSetIndex::NOT_IN_SET;
        }

        fn index(&self, x: &T) -> &IntrusiveSetIndex {
            let f: &F = &self.index_functor;
            let ptr = f(x) as *mut IntrusiveSetIndex;
            unsafe { &*ptr }
        }

        fn index_mut(&mut self, x: &T) -> &mut IntrusiveSetIndex {
            (self.index_functor)(x)
        }

        pub fn iter(&self) -> IntrusiveSetIterator<T, F, C> {
            IntrusiveSetIterator {
                set: self,
                index: 0,
                last_index_location: None,
                _phantom: PhantomData,
            }
        }
    }

    /// This iterator supports insertion (newly inserted elements will be visited
    /// as part of the iteration) and removal of the current element while
    /// iterating. Removing previously visited elements is undefined behavior.
    /// ATTENTION! The memory the removed element points to needs to remain alive
    /// until the end of the iteration.
    pub struct IntrusiveSetIterator<'a, T, F, C>
    where
        T: Copy,
        F: Fn(&T) -> &mut IntrusiveSetIndex,
        C: DerefMut + Deref<Target = Vec<T>>,
    {
        set: &'a IntrusiveSet<T, F, C>,
        index: usize,
        // If the current element is removed, another element is swapped in to the
        // same position. We notice this by remembering the index memory location of
        // the last retrieved element.
        last_index_location: Option<*const usize>,
        _phantom: PhantomData<T>,
    }

    impl<'a, T, F, C> Iterator for IntrusiveSetIterator<'a, T, F, C>
    where
        T: Copy,
        F: Fn(&T) -> &mut IntrusiveSetIndex,
        C: DerefMut + Deref<Target = Vec<T>>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.set.elements.len() {
                // This iterator requires `operator*` being used before `operator++`.

                if let Some(last_index_location) = self.last_index_location {
                    let current_element = self.set.elements[self.index];
                    let index_ptr = &self.set.index(&current_element).value as *const usize;

                    if index_ptr == last_index_location {
                        self.index += 1;
                        if self.index < self.set.elements.len() {
                          let next_element = self.set.elements[self.index];
                          self.last_index_location = Some(&self.set.index(&next_element).value as *const usize);
                          return Some(next_element);
                        }
                    }
                }
                if self.index < self.set.elements.len() {
                  let current_element = self.set.elements[self.index];
                  self.last_index_location = Some(&self.set.index(&current_element).value as *const usize);
                  return Some(current_element);
                }
                None
            } else {
                None
            }
        }
    }
}