// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod memory_span {
    use std::{
        array,
        cmp::Ordering,
        fmt::{self, Debug},
        iter::Iterator,
        marker::PhantomData,
        ops::{Deref, DerefMut, Index, IndexMut},
        ptr,
        slice,
    };

    /// Points to an unowned contiguous buffer holding a known number of elements.
    ///
    /// This is similar to std::span, but provides compatibility with older Rust versions.
    /// In the future, this may be replaced with or aliased to std::span.
    #[derive(Copy, Clone)]
    pub struct MemorySpan<'a, T> {
        data: *mut T,
        size: usize,
        _phantom: PhantomData<&'a mut T>,
    }

    impl<'a, T> MemorySpan<'a, T> {
        /// The default constructor creates an empty span.
        pub const fn new() -> Self {
            MemorySpan {
                data: ptr::null_mut(),
                size: 0,
                _phantom: PhantomData,
            }
        }

        /// Constructor from pointer and count.
        pub const fn from_raw_parts(data: *mut T, size: usize) -> Self {
            MemorySpan {
                data,
                size,
                _phantom: PhantomData,
            }
        }

        /// Constructor from a slice.
        pub fn from_slice(slice: &'a mut [T]) -> Self {
            MemorySpan {
                data: slice.as_mut_ptr(),
                size: slice.len(),
                _phantom: PhantomData,
            }
        }

        /// Returns a pointer to the beginning of the buffer.
        #[inline]
        pub fn data(&self) -> *mut T {
            self.data
        }

        /// Returns the number of elements that the buffer holds.
        #[inline]
        pub const fn size(&self) -> usize {
            self.size
        }

        /// Returns true if the buffer is empty.
        #[inline]
        pub const fn empty(&self) -> bool {
            self.size == 0
        }

        /// Returns a reference to the element at the given index.
        ///
        /// # Panics
        ///
        /// Panics if `i` is out of bounds.
        #[inline]
        pub fn get(&self, i: usize) -> Option<&T> {
            if i < self.size {
                unsafe { Some(&*self.data.add(i)) }
            } else {
                None
            }
        }

        /// Returns a mutable reference to the element at the given index.
        ///
        /// # Panics
        ///
        /// Panics if `i` is out of bounds.
        #[inline]
        pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
            if i < self.size {
                unsafe { Some(&mut *self.data.add(i)) }
            } else {
                None
            }
        }

        /// Returns an iterator over the elements in the span.
        #[inline]
        pub fn iter(&self) -> slice::Iter<'a, T> {
            unsafe { slice::from_raw_parts(self.data, self.size) }.iter()
        }

        /// Returns a mutable iterator over the elements in the span.
        #[inline]
        pub fn iter_mut(&mut self) -> slice::IterMut<'a, T> {
            unsafe { slice::from_raw_parts_mut(self.data, self.size) }.iter_mut()
        }

        /// Converts the `MemorySpan` to a slice.
        #[inline]
        pub fn as_slice(&self) -> &[T] {
            unsafe { slice::from_raw_parts(self.data, self.size) }
        }

        /// Converts the `MemorySpan` to a mutable slice.
        #[inline]
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            unsafe { slice::from_raw_parts_mut(self.data, self.size) }
        }
    }

    impl<'a, T> Index<usize> for MemorySpan<'a, T> {
        type Output = T;

        #[inline]
        fn index(&self, i: usize) -> &Self::Output {
            if i >= self.size {
                panic!("index out of bounds: the len is {} but the index is {}", self.size, i);
            }
            unsafe { &*self.data.add(i) }
        }
    }

    impl<'a, T> IndexMut<usize> for MemorySpan<'a, T> {
        #[inline]
        fn index_mut(&mut self, i: usize) -> &mut Self::Output {
            if i >= self.size {
                panic!("index out of bounds: the len is {} but the index is {}", self.size, i);
            }
            unsafe { &mut *self.data.add(i) }
        }
    }

    impl<'a, T> Debug for MemorySpan<'a, T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemorySpan")
                .field("data", &self.data)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<'a, T> From<&'a mut [T]> for MemorySpan<'a, T> {
        fn from(slice: &'a mut [T]) -> Self {
            MemorySpan::from_slice(slice)
        }
    }

    /// Helper function template to create an array of fixed length, initialized by
    /// the provided initializer list, without explicitly specifying the array size,
    /// e.g.
    ///
    ///   let arr = to_array([1, 2, 3]);
    pub fn to_array<T, const N: usize>(arr: [T; N]) -> [T; N] {
        arr
    }
}