// Converted from V8 C++ source files:
// Header: intrusive-set.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::marker::PhantomData;
    use std::{
        iter::Iterator,
        limits::*,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        ptr::NonNull,
        sync::{Arc, Mutex, RwLock},
    };

    #[derive(Default, Debug)]
    pub struct IntrusiveSetIndex {
        value: usize,
    }

    impl IntrusiveSetIndex {
        const K_NOT_IN_SET: usize = usize::MAX;

        pub fn new() -> Self {
            IntrusiveSetIndex {
                value: Self::K_NOT_IN_SET,
            }
        }

        pub fn is_not_in_set(&self) -> bool {
            self.value == Self::K_NOT_IN_SET
        }

        pub fn set_in_set(&mut self, index: usize) {
            self.value = index;
        }

        pub fn set_not_in_set(&mut self) {
            self.value = Self::K_NOT_IN_SET;
        }

        pub fn get_index(&self) -> usize {
            self.value
        }
    }

    pub struct IntrusiveSet<T, GetIntrusiveSetIndex, Container>
    where
        GetIntrusiveSetIndex: Fn(&T) -> &mut IntrusiveSetIndex,
    {
        elements: Container,
        index_functor: GetIntrusiveSetIndex,
        _phantom: PhantomData<T>,
    }

    impl<T, GetIntrusiveSetIndex, Container> IntrusiveSet<T, GetIntrusiveSetIndex, Container>
    where
        T: Copy,
        GetIntrusiveSetIndex: Fn(&T) -> &mut IntrusiveSetIndex,
        Container: std::ops::DerefMut + std::ops::Deref,
        <Container as std::ops::Deref>::Target: std::vec::Vec<T>,
    {
        pub fn new(container: Container, index_functor: GetIntrusiveSetIndex) -> Self {
            Self {
                elements: container,
                index_functor,
                _phantom: PhantomData,
            }
        }

        fn index(&self, x: &T) -> &mut IntrusiveSetIndex {
            (self.index_functor)(x)
        }

        pub fn contains(&self, x: &T) -> bool {
            !self.index(x).is_not_in_set()
        }

        pub fn add(&mut self, x: T) {
            if self.contains(&x) {
                return;
            }
            let index = self.elements.len();
            self.index(&x).set_in_set(index);
            self.elements.push(x);
        }

        pub fn remove(&mut self, x: &T) {
            if !self.contains(x) {
                return;
            }

            let index_to_remove = self.index(x).get_index();

            if index_to_remove >= self.elements.len() {
                return;
            }

            let last_element = self.elements.last().copied();
            if let Some(last_element) = last_element {
                let last_element_index = self.elements.len() - 1;
                if index_to_remove != last_element_index {
                    self.elements[index_to_remove] = last_element;
                    self.index(&last_element).set_in_set(index_to_remove);
                }
            }

            self.elements.pop();
            self.index(x).set_not_in_set();
        }

        pub fn iter(&self) -> IntrusiveSetIterator<'_, T, GetIntrusiveSetIndex, Container> {
            IntrusiveSetIterator {
                set: self,
                index: 0,
                last_index_location: None,
                _phantom: PhantomData,
            }
        }
    }

    pub struct IntrusiveSetIterator<'a, T, GetIntrusiveSetIndex, Container>
    where
        GetIntrusiveSetIndex: Fn(&T) -> &mut IntrusiveSetIndex,
    {
        set: &'a IntrusiveSet<T, GetIntrusiveSetIndex, Container>,
        index: usize,
        // If the current element is removed, another element is swapped in to the
        // same position. We notice this by remembering the index memory location of
        // the last retrieved element.
        last_index_location: Option<*mut usize>,
        _phantom: PhantomData<T>,
    }

    impl<'a, T, GetIntrusiveSetIndex, Container> Iterator
        for IntrusiveSetIterator<'a, T, GetIntrusiveSetIndex, Container>
    where
        T: Copy,
        GetIntrusiveSetIndex: Fn(&T) -> &mut IntrusiveSetIndex,
        Container: std::ops::DerefMut + std::ops::Deref,
        <Container as std::ops::Deref>::Target: std::vec::Vec<T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.set.elements.len() {
                let result = self.set.elements[self.index];
                let index_ptr = &mut self.set.index(&result).value as *mut usize;
                self.last_index_location = Some(index_ptr);

                if let Some(last_index_location) = self.last_index_location {
                    if self.index < self.set.elements.len()
                        && last_index_location == &mut self.set.index(&result).value as *mut usize
                    {
                        self.index += 1;
                        return Some(result);
                    }
                }
                None
            } else {
                None
            }
        }
    }
}
