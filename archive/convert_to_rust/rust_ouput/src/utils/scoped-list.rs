// Converted from V8 C++ source files:
// Header: scoped-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]

use std::vec::Vec;

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: `{}` != `{}`", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("DCHECK_LE failed: `{}` > `{}`", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_LT {
    ($left:expr, $right:expr) => {
        if $left >= $right {
            panic!("DCHECK_LT failed: `{}` >= `{}`", stringify!($left), stringify!($right));
        }
    };
}

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Vector { data }
    }
    pub fn at(&self, i: usize) -> &T {
        &self.data[i]
    }
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> Vector<&'a T> {
    pub fn to_vec(&self) -> Vec<&'a T> {
        self.data.clone()
    }
}

impl<'a, T> From<&'a [T]> for Vector<&'a T> {
    fn from(slice: &'a [T]) -> Self {
        Vector { data: slice.to_vec() }
    }
}

mod internal {
    use super::*;
    use std::marker::PhantomData;

    pub struct ZoneList<T> {
        _phantom: PhantomData<T>,
    }

    #[must_use]
    pub struct ScopedList<'a, T, TBacking = T> {
        buffer: &'a mut Vec<TBacking>,
        start: usize,
        end: usize,
        _phantom: PhantomData<T>,
    }

    impl<'a, T, TBacking> ScopedList<'a, T, TBacking>
    where
        TBacking: Clone,
    {
        pub fn new(buffer: &'a mut Vec<TBacking>) -> Self {
            let start = buffer.len();
            let end = buffer.len();
            ScopedList {
                buffer,
                start,
                end,
                _phantom: PhantomData,
            }
        }

        pub fn rewind(&mut self) {
            DCHECK_EQ!(self.buffer.len(), self.end);
            self.buffer.truncate(self.start);
            self.end = self.start;
        }
        pub fn merge_into(&mut self, parent: &mut ScopedList<'a, T, TBacking>) {
            DCHECK_EQ!(parent.end, self.start);
            parent.end = self.end;
            self.start = self.end;
            DCHECK_EQ!(0, self.length());
        }
        pub fn length(&self) -> i32 {
            (self.end - self.start) as i32
        }
    }

    impl<'a, T, TBacking> ScopedList<'a, T, TBacking>
    where
        TBacking: Clone,
        T: 'a,
    {
        pub fn at(&self, i: i32) -> &TBacking {
            let index = self.start + i as usize;
            DCHECK_LE!(self.start, index);
            DCHECK_LT!(index, self.buffer.len());
            &self.buffer[index]
        }

        pub fn at_mut(&mut self, i: i32) -> &mut TBacking {
            let index = self.start + i as usize;
            DCHECK_LE!(self.start, index);
            DCHECK_LT!(index, self.buffer.len());
            &mut self.buffer[index]
        }
        pub fn to_const_vector(&self) -> Vector<&TBacking> {
             let slice = &self.buffer[self.start..self.end];
            Vector::from(slice)
        }

        pub fn add(&mut self, value: TBacking)
        where
            TBacking: Clone,
        {
            DCHECK_EQ!(self.buffer.len(), self.end);
            self.buffer.push(value);
            self.end += 1;
        }
        pub fn add_all(&mut self, list: Vector<&TBacking>)
        where
            TBacking: Clone,
        {
            DCHECK_EQ!(self.buffer.len(), self.end);
            self.buffer.reserve(self.buffer.len() + list.length());
            for i in 0..list.length() {
                self.buffer.push(list.at(i).clone());
            }
            self.end += list.length();
        }
    }

    impl<'a, T, TBacking> Drop for ScopedList<'a, T, TBacking> {
        fn drop(&mut self) {
            self.rewind();
        }
    }

    impl<'a, T> ScopedList<'a, *mut T, *mut std::ffi::c_void> {
        pub fn new_ptr(buffer: &'a mut Vec<*mut std::ffi::c_void>) -> Self {
            let start = buffer.len();
            let end = buffer.len();
            ScopedList {
                buffer,
                start,
                end,
                _phantom: PhantomData,
            }
        }
        pub fn at_ptr(&self, i: i32) -> &*mut T {
            let index = self.start + i as usize;
            DCHECK_LE!(self.start, index);
            DCHECK_LT!(index, self.buffer.len());
            unsafe { &*(&self.buffer[index] as *const *mut std::ffi::c_void as *const *mut T) }
        }

        pub fn at_ptr_mut(&mut self, i: i32) -> &mut *mut T {
            let index = self.start + i as usize;
            DCHECK_LE!(self.start, index);
            DCHECK_LT!(index, self.buffer.len());
            unsafe { &mut *(&mut self.buffer[index] as *mut *mut std::ffi::c_void as *mut *mut T) }
        }

        pub fn add_ptr(&mut self, value: *mut T) {
            DCHECK_EQ!(self.buffer.len(), self.end);
            self.buffer.push(value as *mut std::ffi::c_void);
            self.end += 1;
        }
    }

    pub type ScopedPtrList<'a, T> = ScopedList<'a, *mut T, *mut std::ffi::c_void>;

    impl<'a, T, TBacking> ScopedList<'a, T, TBacking> {
        pub fn begin(&mut self) -> *mut TBacking {
            self.buffer.as_mut_ptr().add(self.start)
        }

        pub fn begin_const(&self) -> *const TBacking {
            self.buffer.as_ptr().add(self.start) as *const TBacking
        }

        pub fn end(&mut self) -> *mut TBacking {
            self.buffer.as_mut_ptr().add(self.end)
        }

        pub fn end_const(&self) -> *const TBacking {
            self.buffer.as_ptr().add(self.end) as *const TBacking
        }
    }
}
