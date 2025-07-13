// Converted from V8 C++ source files:
// Header: small-vector.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub use std::cmp;
use std::mem::MaybeUninit;
use std::ptr;
use std::{alloc::{Allocator, GlobalAlloc, Layout}, marker::Copy, mem::size_of};
use std::alloc::Global;

#[derive(Debug)]
pub enum SmallVectorError {
    OutOfMemory,
    InvalidOperation,
}

// Minimal SmallVector implementation. Uses inline storage first, switches to
// dynamic storage when it overflows.
pub struct SmallVector<T, const kSize: usize, A: Allocator = Global> {
    allocator_: A,
    begin_: *mut T,
    end_: *mut T,
    end_of_storage_: *mut T,
    inline_storage_: [MaybeUninit<T>; kSize],
    phantom: std::marker::PhantomData<T>,
}

impl<T, const kSize: usize> SmallVector<T, kSize, Global> {
    pub const kInlineSize: usize = kSize;

    pub fn new() -> Self {
        Self::with_allocator(Global)
    }
}

impl<T: Copy, const kSize: usize> SmallVector<T, kSize, Global> {
    pub fn from_slice(slice: &[T]) -> Self {
        let mut vec = SmallVector::new();
        vec.extend_from_slice(slice);
        vec
    }
}

impl<T, const kSize: usize, A: Allocator> SmallVector<T, kSize, A> {
    pub const kInlineSize: usize = kSize;
    pub type value_type = T;

    pub fn with_allocator(allocator: A) -> Self {
        let mut inline_storage: [MaybeUninit<T>; kSize] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        let begin_ptr: *mut T = inline_storage.as_mut_ptr() as *mut T;

        SmallVector {
            allocator_: allocator,
            begin_: begin_ptr,
            end_: begin_ptr,
            end_of_storage_: unsafe { begin_ptr.add(kSize) },
            inline_storage_: inline_storage,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn with_capacity_and_allocator(size: usize, allocator: A) -> Self {
        let mut vec = SmallVector::with_allocator(allocator);
        vec.resize(size);
        vec
    }

    pub fn with_capacity_and_default_value_and_allocator(size: usize, initial_value: T, allocator: A) -> Self
    where T: Copy
    {
        let mut vec = SmallVector::with_allocator(allocator);
        vec.resize_with(size, || initial_value);
        vec
    }

    pub fn from_small_vector(other: &SmallVector<T, kSize, A>, allocator: A) -> Self
    where T: Copy
    {
        let mut new_vec = SmallVector::with_allocator(allocator);
        new_vec.extend_from_slice(other.as_slice());
        new_vec
    }

    pub fn from_small_vector_moved(other: SmallVector<T, kSize, A>, allocator: A) -> Self {
        let mut new_vec = SmallVector::with_allocator(allocator);
        if other.is_big() {
            new_vec.begin_ = other.begin_;
            new_vec.end_ = other.end_;
            new_vec.end_of_storage_ = other.end_of_storage_;
        } else {
            new_vec.extend_from_slice(other.as_slice());
        }
        std::mem::forget(other);
        new_vec
    }

    pub fn from_initializer_list(init: &[T], allocator: A) -> Self
        where T: Copy
    {
        let mut vec = SmallVector::with_allocator(allocator);
        vec.extend_from_slice(init);
        vec
    }

    pub fn from_vector(init: &[T], allocator: A) -> Self
        where T: Copy
    {
        let mut vec = SmallVector::with_allocator(allocator);
        vec.extend_from_slice(init);
        vec
    }

    fn free_storage(&mut self)
    {
        unsafe {
            let layout = Layout::array::<T>(self.size()).unwrap();
            if self.is_big() {
                let ptr = self.begin_ as *mut u8;
                self.allocator_.deallocate(ptr, layout);
            }
        }
    }

    fn reset_to_inline_storage(&mut self) {
        if self.is_big() {
           unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.begin_, self.size()));
            }
        }

        self.begin_ = self.inline_storage_begin() as *mut T;
        self.end_ = self.begin_;
        self.end_of_storage_ = unsafe { self.begin_.add(kSize) };
    }

    fn is_big(&self) -> bool {
        self.begin_ != self.inline_storage_begin() as *mut T
    }

    fn inline_storage_begin(&self) -> *mut MaybeUninit<T> {
        self.inline_storage_.as_mut_ptr()
    }

    pub fn allocator(&self) -> &A {
        &self.allocator_
    }

    pub fn data(&mut self) -> *mut T {
        self.begin_
    }

    pub fn data_const(&self) -> *const T {
        self.begin_
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.begin_
    }

    pub fn as_ptr(&self) -> *const T {
        self.begin_
    }

    pub fn begin(&mut self) -> *mut T {
        self.begin_
    }

    pub fn begin_const(&self) -> *const T {
        self.begin_
    }

    pub fn end(&mut self) -> *mut T {
        self.end_
    }

    pub fn end_const(&self) -> *const T {
        self.end_
    }

    pub fn rbegin(&mut self) -> std::slice::IterMut<'_, T> {
        unsafe { std::slice::from_raw_parts_mut(self.begin_, self.size()).iter_mut() }
    }

    pub fn rbegin_const(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.begin_, self.size()).iter() }
    }

    pub fn rend(&mut self) -> std::slice::IterMut<'_, T> {
        unsafe { std::slice::from_raw_parts_mut(self.begin_, self.size()).iter_mut() }
    }

    pub fn rend_const(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.begin_, self.size()).iter() }
    }

    pub fn size(&self) -> usize {
        unsafe { self.end_.offset_from(self.begin_) as usize }
    }

    pub fn capacity(&self) -> usize {
        unsafe { self.end_of_storage_.offset_from(self.begin_) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.begin_ == self.end_
    }

    pub fn front(&mut self) -> &mut T {
        if self.is_empty() {
            panic!("SmallVector::front() called on an empty vector");
        }
        unsafe { &mut *self.begin_ }
    }

    pub fn front_const(&self) -> &T {
        if self.is_empty() {
            panic!("SmallVector::front() called on an empty vector");
        }
        unsafe { &*self.begin_ }
    }

    pub fn back(&mut self) -> &mut T {
        if self.is_empty() {
            panic!("SmallVector::back() called on an empty vector");
        }
        unsafe { &mut *self.end_.sub(1) }
    }

    pub fn back_const(&self) -> &T {
        if self.is_empty() {
            panic!("SmallVector::back() called on an empty vector");
        }
        unsafe { &*self.end_.sub(1) }
    }

    pub fn at(&mut self, index: usize) -> &mut T {
        if index >= self.size() {
            panic!("SmallVector::at() index out of bounds");
        }
        unsafe { &mut *self.begin_.add(index) }
    }

    pub fn at_const(&self, index: usize) -> &T {
        if index >= self.size() {
            panic!("SmallVector::at() index out of bounds");
        }
        unsafe { &*self.begin_.add(index) }
    }

    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.size() {
            return None;
        }
        unsafe { Some(&mut *self.begin_.add(index)) }
    }

    pub fn get_const(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        unsafe { Some(&*self.begin_.add(index)) }
    }

    pub fn push(&mut self, value: T) -> Result<(), SmallVectorError> {
        if self.end_ == self.end_of_storage_ {
            self.grow(1)?;
        }
        unsafe {
            std::ptr::write(self.end_, value);
            self.end_ = self.end_.add(1);
        }
        Ok(())
    }

    pub fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), SmallVectorError>
        where T: Copy
    {
        let count = slice.len();
        if self.size() + count > self.capacity() {
            self.grow(count)?;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                self.end_ as *mut T,
                count,
            );
            self.end_ = self.end_.add(count);
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            self.end_ = self.end_.sub(1);
            let value = std::ptr::read(self.end_);
            Some(value)
        }
    }

    pub fn insert(&mut self, pos: *mut T, value: T) -> Result<*mut T, SmallVectorError>
        where T: Copy
    {
        self.insert_multiple(pos, 1, value)
    }

    pub fn insert_multiple(&mut self, pos: *mut T, count: usize, value: T) -> Result<*mut T, SmallVectorError>
        where T: Copy
    {
        if pos < self.begin_ || pos > self.end_ {
            return Err(SmallVectorError::InvalidOperation);
        }

        let offset = unsafe { pos.offset_from(self.begin_) as usize };
        let old_size = self.size();
        self.resize(old_size + count);

        let pos = unsafe { self.begin_.add(offset) };
        let old_end = unsafe { self.begin_.add(old_size) };

        unsafe {
            std::ptr::copy(
                pos,
                pos.add(count),
                old_end.offset_from(pos) as usize,
            );
            std::ptr::write_bytes(pos, 0, count);
            for i in 0..count {
                std::ptr::write(pos.add(i), value);
            }
        }
        Ok(pos)
    }

    pub fn insert_from_slice<It>(&mut self, pos: *mut T, begin: It, end: It) -> Result<*mut T, SmallVectorError>
        where
            It: Iterator<Item = T>,
            T: Copy,
    {
        if pos < self.begin_ || pos > self.end_ {
            return Err(SmallVectorError::InvalidOperation);
        }

        let offset = unsafe { pos.offset_from(self.begin_) as usize };
        let mut iterator = begin;
        let mut count: usize = 0;
        while iterator.next().is_some() {
            count = count + 1;
        }
        let old_size = self.size();
        self.resize(old_size + count);
        let pos = unsafe { self.begin_.add(offset) };
        let old_end = unsafe { self.begin_.add(old_size) };
        unsafe {
            std::ptr::copy(
                pos,
                pos.add(count),
                old_end.offset_from(pos) as usize,
            );
        }
        let mut iterator = begin;
        for i in 0..count {
            let value = iterator.next().unwrap();
            unsafe {
                std::ptr::write(pos.add(i), value);
            }
        }
        Ok(pos)
    }

    pub fn insert_from_initializer_list(&mut self, pos: *mut T, values: &[T]) -> Result<*mut T, SmallVectorError>
        where T: Copy
    {
        if pos < self.begin_ || pos > self.end_ {
            return Err(SmallVectorError::InvalidOperation);
        }
        self.insert_from_slice(pos, values.iter().copied(), values.iter().copied())
    }

    pub fn erase(&mut self, erase_start: *mut T) -> Result<(), SmallVectorError> {
        if erase_start < self.begin_ || erase_start > self.end_ {
            return Err(SmallVectorError::InvalidOperation);
        }

        let count = unsafe { self.end_.offset_from(erase_start) as usize };
        self.end_ = erase_start;

        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.end_, count));
        }

        Ok(())
    }

    pub fn resize(&mut self, new_size: usize)
    {
        if new_size > self.capacity() {
            self.grow(new_size).unwrap();
        }
        let new_end = unsafe { self.begin_.add(new_size) };
        if new_end > self.end_ {
            let diff = unsafe { new_end.offset_from(self.end_) as usize };
            for i in 0..diff {
                unsafe {
                    std::ptr::write(self.end_.add(i), std::mem::zeroed());
                }
            }
        } else {
            let diff = unsafe { self.end_.offset_from(new_end) as usize };
            unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(new_end, diff));
            }
        }
        self.end_ = new_end;
    }

    pub fn resize_with<F: FnMut() -> T>(&mut self, new_size: usize, initial_value: F) {
        if new_size > self.capacity() {
            self.grow(new_size).unwrap();
        }
        let new_end = unsafe { self.begin_.add(new_size) };
        if new_end > self.end_ {
            let diff = unsafe { new_end.offset_from(self.end_) as usize };
            for i in 0..diff {
                unsafe {
                    let value = initial_value();
                    std::ptr::write(self.end_.add(i), value);
                }
            }
        } else {
            let diff = unsafe { self.end_.offset_from(new_end) as usize };
            unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(new_end, diff));
            }
        }
        self.end_ = new_end;
    }

    pub fn reserve(&mut self, new_capacity: usize) -> Result<(), SmallVectorError> {
        if new_capacity > self.capacity() {
            self.grow(new_capacity)?;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.begin_, self.size()));
        }
        self.end_ = self.begin_;
    }

    fn grow(&mut self, min_capacity: usize) -> Result<(), SmallVectorError> {
        let in_use = self.size();
        let new_capacity = cmp::max(min_capacity, 2 * self.capacity());

        let layout = Layout::array::<T>(new_capacity).map_err(|_| SmallVectorError::OutOfMemory)?;

        let new_storage = unsafe {
            self.allocator_.allocate(layout.clone())
                .map_err(|_| SmallVectorError::OutOfMemory)?
        } as *mut T;

        if new_storage.is_null() {
            return Err(SmallVectorError::OutOfMemory);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(self.begin_, new_storage, in_use);
        }

        self.free_storage();

        self.begin_ = new_storage;
        self.end_ = unsafe { new_storage.add(in_use) };
        self.end_of_storage_ = unsafe { new_storage.add(new_capacity) };

        Ok(())
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.begin_, self.size()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.begin_, self.size()) }
    }
}

impl<T, const kSize: usize, A: Allocator> Drop for SmallVector<T, kSize, A> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.begin_, self.size()));
        }
        if self.is_big() {
            self.free_storage();
        }
    }
}
}
