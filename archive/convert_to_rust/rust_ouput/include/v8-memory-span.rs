// Converted from V8 C++ source files:
// Header: v8-memory-span.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::{
    array, cmp::Ordering, fmt, iter, marker::PhantomData, mem, ops, ptr, slice,
};

// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-function-callback.h
pub struct V8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-array-buffer.h
pub struct V8_EXPORT {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-array-buffer.h
pub struct V8_EXPORT {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Numeric {}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-function-callback.h
pub struct Local<'a, T> {
    _marker: PhantomData<&'a T>,
}
// From /home/kathirks_gc/v8_go/archive/codebase/include/v8-value.h
struct Object {}

impl<'a, T> Local<'a, T> {
    fn empty() -> Self {
        Local {
            _marker: PhantomData,
        }
    }
}

impl V8 {
    pub fn new() -> Self {
        V8 {}
    }
}

impl Default for V8 {
    fn default() -> Self {
        V8::new()
    }
}

impl<'a> Local<'a, Object> {
    // From /home/kathirks_gc/v8_go/archive/codebase/include/v8-function-callback.h
    pub fn this(&self) -> Local<Object> {
        Local::empty()
    }
}

#[derive(Debug)]
pub enum MemorySpanError {
    EmptySpan,
    OutOfBounds,
    IteratorError,
}

#[derive(Debug)]
struct StringView {}

pub trait StringBuffer {
    fn write(&mut self, string: &str);
    fn contents(&self) -> String;
}

pub fn create(string_view: StringView) -> Box<dyn StringBuffer> {
    struct SimpleStringBuffer {
        buffer: String,
    }

    impl StringBuffer for SimpleStringBuffer {
        fn write(&mut self, string: &str) {
            self.buffer.push_str(string);
        }

        fn contents(&self) -> String {
            self.buffer.clone()
        }
    }

    Box::new(SimpleStringBuffer {
        buffer: String::new(),
    })
}

mod detail {
    use std::array;

    pub fn to_array_lvalue_impl<T: Copy, const N: usize>(a: &[T; N]) -> [T; N] {
        let mut result: [T; N] = [a[0]; N];
        for i in 0..N {
            result[i] = a[i];
        }
        result
    }

    pub fn to_array_rvalue_impl<T: Copy, const N: usize>(a: [T; N]) -> [T; N] {
        a
    }
}

#[derive(Clone, Copy)]
pub struct MemorySpan<'a, T> {
    data: *mut T,
    size: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> MemorySpan<'a, T> {
    /// The default constructor creates an empty span.
    pub const fn new() -> Self {
        MemorySpan {
            data: ptr::null_mut(),
            size: 0,
            _marker: PhantomData,
        }
    }

    /// Constructor from nullptr and count, for backwards compatibility.
    /// This is not compatible with C++20 std::span.
    pub const fn from_null_ptr(_: *const (), size: usize) -> Self {
        MemorySpan {
            data: ptr::null_mut(),
            size,
            _marker: PhantomData,
        }
    }

    /// Constructor from "iterator" and count.
    pub fn from_iter<I>(first: I, count: usize) -> Self
    where
        I: Iterator<Item = T>,
        T: Copy,
    {
        let mut vec = Vec::new();
        for item in first.take(count) {
            vec.push(item);
        }
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
        let size = slice.len();
        mem::forget(slice); // Prevent deallocation

        MemorySpan {
            data,
            size,
            _marker: PhantomData,
        }
    }

    /// Constructor from two "iterators".
    pub fn from_iters<I>(first: I, last: I) -> Result<Self, MemorySpanError>
    where
        I: Iterator<Item = T> + ExactSizeIterator,
        T: Copy,
    {
        let mut first_vec = first.collect::<Vec<_>>();
        let mut last_vec = last.collect::<Vec<_>>();

        if first_vec.len() > last_vec.len() {
            return Err(MemorySpanError::IteratorError);
        }

        let size = last_vec.len() - first_vec.len();

        let mut slice = last_vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
        mem::forget(slice);

        Ok(MemorySpan {
            data,
            size,
            _marker: PhantomData,
        })
    }

    /// Implicit conversion from C-style array.
    pub const fn from_array<const N: usize>(a: &'a mut [T; N]) -> Self {
        MemorySpan {
            data: a.as_mut_ptr(),
            size: N,
            _marker: PhantomData,
        }
    }

    /// Implicit conversion from std::array.
    pub fn from_std_array<U, const N: usize>(a: &'a mut array::IntoIter<U, N>) -> Self
    where
        U: Copy + Into<T>,
        T: 'a,
    {
        let mut vec: Vec<T> = a.map(|x| x.into()).collect();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
        let size = slice.len();
        mem::forget(slice);

        MemorySpan {
            data,
            size,
            _marker: PhantomData,
        }
    }

    /// Implicit conversion from const std::array.
    pub fn from_const_std_array<U, const N: usize>(a: &'a array::IntoIter<U, N>) -> Self
    where
        U: Copy + Into<T>,
        T: 'a,
    {
        let mut vec: Vec<T> = a.map(|x| x.into()).collect();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
        let size = slice.len();
        mem::forget(slice);

        MemorySpan {
            data,
            size,
            _marker: PhantomData,
        }
    }

    /// Returns a pointer to the beginning of the buffer.
    pub fn data(&self) -> *mut T {
        self.data
    }

    /// Returns the number of elements that the buffer holds.
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, i: usize) -> Result<&T, MemorySpanError> {
        if i >= self.size {
            return Err(MemorySpanError::OutOfBounds);
        }
        unsafe { Ok(&*self.data.add(i)) }
    }

    pub fn get_mut(&mut self, i: usize) -> Result<&mut T, MemorySpanError> {
        if i >= self.size {
            return Err(MemorySpanError::OutOfBounds);
        }
        unsafe { Ok(&mut *self.data.add(i)) }
    }

    pub fn set(&mut self, i: usize, value: T) -> Result<(), MemorySpanError>
    where
        T: Copy,
    {
        if i >= self.size {
            return Err(MemorySpanError::OutOfBounds);
        }
        unsafe {
            *self.data.add(i) = value;
        }
        Ok(())
    }

    /// Returns true if the buffer is empty.
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    pub fn iter(&self) -> MemorySpanIterator<'a, T> {
        MemorySpanIterator {
            span: *self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> MemorySpanMutIterator<'a, T> {
        MemorySpanMutIterator {
            span: *self,
            index: 0,
        }
    }

    pub fn begin(&self) -> Iterator<'a, T> {
        Iterator {
            ptr_: self.data,
            _marker: PhantomData,
        }
    }

    pub fn end(&self) -> Iterator<'a, T> {
        unsafe {
            Iterator {
                ptr_: self.data.add(self.size),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T: Copy> MemorySpan<'a, T> {
    pub fn to_vec(&self) -> Vec<T> {
        if self.data.is_null() {
            return Vec::new();
        }
        unsafe { slice::from_raw_parts(self.data, self.size).to_vec() }
    }
}

impl<'a, T> ops::Index<usize> for MemorySpan<'a, T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        self.get(i).unwrap()
    }
}

pub struct MemorySpanIterator<'a, T> {
    span: MemorySpan<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for MemorySpanIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.span.size() {
            let result = unsafe { &*self.span.data.add(self.index) };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub struct MemorySpanMutIterator<'a, T> {
    span: MemorySpan<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for MemorySpanMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.span.size() {
            let result = unsafe { &mut *self.span.data.add(self.index) };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Iterator<'a, T> {
    ptr_: *mut T,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator<'a, T> {
    // Required to satisfy `std::semiregular<>`.
    pub const fn new() -> Self {
        Iterator {
            ptr_: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    pub fn offset(self, offset: isize) -> Self {
        unsafe {
            Iterator {
                ptr_: self.ptr_.offset(offset),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> PartialEq for Iterator<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr_ == other.ptr_
    }
}

impl<'a, T> Eq for Iterator<'a, T> {}

impl<'a, T> PartialOrd for Iterator<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T> Ord for Iterator<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.ptr_ as usize).cmp(&(other.ptr_ as usize))
    }
}

impl<'a, T> ops::Add<usize> for Iterator<'a, T> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        unsafe {
            Iterator {
                ptr_: self.ptr_.add(rhs),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> ops::Sub<usize> for Iterator<'a, T> {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self {
        unsafe {
            Iterator {
                ptr_: self.ptr_.sub(rhs),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> ops::Sub for Iterator<'a, T> {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { self.ptr_.offset_from(rhs.ptr_) }
    }
}

impl<'a, T> ops::AddAssign<usize> for Iterator<'a, T> {
    fn add_assign(&mut self, rhs: usize) {
        self.ptr_ = unsafe { self.ptr_.add(rhs) };
    }
}

impl<'a, T> ops::SubAssign<usize> for Iterator<'a, T> {
    fn sub_assign(&mut self, rhs: usize) {
        self.ptr_ = unsafe { self.ptr_.sub(rhs) };
    }
}

impl<'a, T> ops::Deref for Iterator<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr_ }
    }
}

impl<'a, T> ops::DerefMut for Iterator<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr_ }
    }
}

pub fn to_array<T: Copy, const N: usize>(a: &[T; N]) -> [T; N] {
    detail::to_array_lvalue_impl(a)
}

pub fn to_array_rvalue<T: Copy, const N: usize>(a: [T; N]) -> [T; N] {
    detail::to_array_rvalue_impl(a)
}
