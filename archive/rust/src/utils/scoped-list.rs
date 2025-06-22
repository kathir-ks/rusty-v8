// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/utils/scoped-list.h

use std::ops::{Index, IndexMut};
use std::slice;

macro_rules! dcheck_le {
    ($left:expr, $right:expr) => {
        debug_assert!($left <= $right);
    };
}

macro_rules! dcheck_lt {
    ($left:expr, $right:expr) => {
        debug_assert!($left < $right);
    };
}

macro_rules! dcheck_eq {
    ($left:expr, $right:expr) => {
        debug_assert_eq!($left, $right);
    };
}

pub mod internal {
    use super::*;

    // TODO: Implement ZoneList in Rust if necessary.
    // pub struct ZoneList<T> { ... }

    /// ScopedList is a scope-lifetime list with a Vec backing that can be
    /// reused between ScopedLists. Note that a ScopedList in an outer scope cannot
    /// add any entries if there is a ScopedList with the same backing in an inner
    /// scope.
    pub struct ScopedList<'a, T, TBacking = T>
    where
        TBacking: 'a,
    {
        buffer: &'a mut Vec<TBacking>,
        start: usize,
        end: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<'a, T, TBacking> ScopedList<'a, T, TBacking>
    where
        TBacking: 'a,
        TBacking: Clone + 'a,
        T: 'a,
        T: From<TBacking> + Into<TBacking>,
    {
        pub fn new(buffer: &'a mut Vec<TBacking>) -> Self {
            let start = buffer.len();
            let end = buffer.len();
            ScopedList {
                buffer,
                start,
                end,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn rewind(&mut self) {
            dcheck_eq!(self.buffer.len(), self.end);
            self.buffer.truncate(self.start);
            self.end = self.start;
        }

        pub fn merge_into(&mut self, parent: &mut ScopedList<'a, T, TBacking>) {
            dcheck_eq!(parent.end, self.start);
            parent.end = self.end;
            self.start = self.end;
            dcheck_eq!(0, self.length());
        }

        pub fn length(&self) -> usize {
            self.end - self.start
        }

        pub fn add(&mut self, value: T)
        where
            TBacking: From<T>,
        {
            dcheck_eq!(self.buffer.len(), self.end);
            self.buffer.push(value.into());
            self.end += 1;
        }

        pub fn add_all(&mut self, list: &[T])
        where
            T: Clone,
            TBacking: From<T>,
        {
            dcheck_eq!(self.buffer.len(), self.end);
            self.buffer.reserve(self.buffer.len() + list.len());
            for i in 0..list.len() {
                self.buffer.push(list[i].clone().into());
            }
            self.end += list.len();
        }

        pub fn as_slice(&self) -> &[T]
        where
            TBacking: AsRef<T>,
        {
            let ptr = self.buffer.as_ptr();
            let slice = unsafe { slice::from_raw_parts(ptr.add(self.start) as *const T, self.length()) };
            slice
        }

        pub fn as_mut_slice(&mut self) -> &mut [T]
        where
            TBacking: AsRef<T> + AsMut<T>,
        {
            let ptr = self.buffer.as_mut_ptr();
            let slice = unsafe { slice::from_raw_parts_mut(ptr.add(self.start) as *mut T, self.length()) };
            slice
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T>
            where TBacking: AsRef<T> {
            self.as_slice().iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>
            where TBacking: AsRef<T> + AsMut<T> {
            self.as_mut_slice().iter_mut()
        }
    }

    impl<'a, T, TBacking> Drop for ScopedList<'a, T, TBacking>
    where
        TBacking: 'a,
    {
        fn drop(&mut self) {
            self.rewind();
        }
    }

    impl<'a, T, TBacking> Index<usize> for ScopedList<'a, T, TBacking>
    where
        TBacking: 'a,
        TBacking: AsRef<T>,
    {
        type Output = T;

        fn index(&self, i: usize) -> &Self::Output {
            let index = self.start + i;
            dcheck_le!(self.start, index);
            dcheck_lt!(index, self.buffer.len());
            self.buffer[index].as_ref()
        }
    }

    impl<'a, T, TBacking> IndexMut<usize> for ScopedList<'a, T, TBacking>
    where
        TBacking: 'a,
        TBacking: AsRef<T> + AsMut<T>,
    {
        fn index_mut(&mut self, i: usize) -> &mut Self::Output {
            let index = self.start + i;
            dcheck_le!(self.start, index);
            dcheck_lt!(index, self.buffer.len());
            self.buffer[index].as_mut()
        }
    }

    pub type ScopedPtrList<'a, T> = ScopedList<'a, *mut T, *mut T>;
} // namespace internal