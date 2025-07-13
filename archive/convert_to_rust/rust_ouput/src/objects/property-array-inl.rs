// Converted from V8 C++ source files:
// Header: property-array-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::mem::size_of;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::v8::internal::{Isolate, Object};
use crate::v8::internal::Tagged;
use crate::v8::internal::code::Code;
use crate::v8::internal::v8;
use crate::v8::internal::DisallowGarbageCollection;
use crate::v8::internal::WriteBarrierMode;

mod property_array_tq_inl;

#[repr(C)]
pub struct PropertyArray {
    length_and_hash: AtomicI32,
}

impl PropertyArray {
    pub fn length_and_hash(&self) -> i32 {
        self.length_and_hash.load(Ordering::Relaxed)
    }

    pub fn set_length_and_hash(&self, value: i32) {
        self.length_and_hash.store(value, Ordering::Relaxed)
    }

    pub fn get(&self, index: usize) -> Tagged<Object> {
        let cage_base = 0;
        self.get_with_cage_base(cage_base, index)
    }

    pub fn get_with_cage_base(&self, _cage_base: usize, index: usize) -> Tagged<Object> {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }

        unsafe {
            let ptr = (self as *const Self as *const u8).add(Self::offset_of_element_at(index)) as *const Tagged<Object>;
            ptr.read_unaligned()
        }
    }

    pub fn get_with_tag(&self, index: usize, _tag: SeqCstAccessTag) -> Tagged<Object> {
        let cage_base = 0;
        self.get_with_cage_base_and_tag(cage_base, index, _tag)
    }

    pub fn get_with_cage_base_and_tag(&self, _cage_base: usize, index: usize, _tag: SeqCstAccessTag) -> Tagged<Object> {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }
        unsafe {
            let ptr = (self as *const Self as *const u8).add(Self::offset_of_element_at(index)) as *const Tagged<Object>;
            ptr.read_unaligned()
        }
    }

    pub fn set(&self, index: usize, value: Tagged<Object>) {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }
        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(Self::offset_of_element_at(index)) as *mut Tagged<Object>;
            ptr.write_unaligned(value);
        }
    }

    pub fn set_with_mode(&self, index: usize, value: Tagged<Object>, _mode: WriteBarrierMode) {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }
        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(Self::offset_of_element_at(index)) as *mut Tagged<Object>;
            ptr.write_unaligned(value);
        }
    }

    pub fn set_with_tag(&self, index: usize, value: Tagged<Object>, _tag: SeqCstAccessTag) {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }
        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(Self::offset_of_element_at(index)) as *mut Tagged<Object>;
            ptr.write_unaligned(value);
        }
    }

    pub fn swap(&self, index: usize, value: Tagged<Object>, _tag: SeqCstAccessTag) -> Tagged<Object> {
        let cage_base = 0;
        self.swap_with_cage_base(cage_base, index, value, _tag)
    }

    pub fn swap_with_cage_base(&self, _cage_base: usize, index: usize, value: Tagged<Object>, _tag: SeqCstAccessTag) -> Tagged<Object> {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }

        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(Self::offset_of_element_at(index)) as *mut Tagged<Object>;
            let old_value = ptr.replace(value);
            old_value
        }
    }

    pub fn compare_and_swap(&self, index: usize, expected: Tagged<Object>, value: Tagged<Object>, _tag: SeqCstAccessTag) -> Tagged<Object> {
        if index >= self.length() as usize {
            panic!("Index out of bounds");
        }

        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(Self::offset_of_element_at(index)) as *mut Tagged<Object>;
            //Note: Replace this with an actual atomic compare and swap when possible
            let old_value = ptr.read_unaligned();
            if old_value == expected {
                ptr.write_unaligned(value);
            }
            old_value
        }
    }

    pub fn data_start(&self) -> usize {
        Self::offset_of_element_at(0)
    }

    pub fn raw_field_of_element_at(&self, index: usize) -> usize {
        Self::offset_of_element_at(index)
    }

    pub fn length(&self) -> i32 {
        LengthField::decode(self.length_and_hash())
    }

    pub fn initialize_length(&self, len: i32) {
        if !LengthField::is_valid(len) {
            panic!("Invalid length");
        }
        self.set_length_and_hash(len);
    }

    pub fn length_acquire_load(&self) -> i32 {
        LengthField::decode(self.length_and_hash.load(Ordering::Acquire))
    }

    pub fn hash(&self) -> i32 {
        HashField::decode(self.length_and_hash())
    }

    pub fn set_hash(&self, hash: i32) {
        let value = self.length_and_hash();
        let value = HashField::update(value, hash);
        self.length_and_hash.store(value, Ordering::Release);
    }

    pub fn copy_elements(isolate: *mut Isolate, dst: &PropertyArray, dst_index: usize, src: &PropertyArray, src_index: usize, len: usize, _mode: WriteBarrierMode) {
        if len == 0 {
            return;
        }

        let _no_gc = DisallowGarbageCollection {};

        unsafe {
            let dst_ptr = (dst as *const PropertyArray as *mut u8).add(Self::offset_of_element_at(dst_index)) as *mut Tagged<Object>;
            let src_ptr = (src as *const PropertyArray as *const u8).add(Self::offset_of_element_at(src_index)) as *const Tagged<Object>;
            let size = len * size_of::<Tagged<Object>>();
            std::ptr::copy_nonoverlapping(src_ptr, dst_ptr as *mut Tagged<Object>, size);
        }
    }

    const fn offset_of_element_at(index: usize) -> usize {
        let element_size = size_of::<Tagged<Object>>();
        let offset = Self::kDataOffset + index * element_size;
        offset
    }
}

impl PropertyArray {
    const kLengthAndHashOffset: usize = 0;
    const kDataOffset: usize = 4;
}

struct LengthField {}

impl LengthField {
    const kLengthBits: i32 = 24;
    const kLengthMask: i32 = (1 << Self::kLengthBits) - 1;

    fn decode(value: i32) -> i32 {
        value & Self::kLengthMask
    }

    fn is_valid(len: i32) -> bool {
        len >= 0 && len <= Self::kLengthMask
    }
}

struct HashField {}

impl HashField {
    const kHashBits: i32 = 8;
    const kHashMask: i32 = ((1 << Self::kHashBits) - 1) << LengthField::kLengthBits;

    fn decode(value: i32) -> i32 {
        (value & Self::kHashMask) >> LengthField::kLengthBits
    }

    fn update(value: i32, hash: i32) -> i32 {
        let hash_masked = (hash << LengthField::kLengthBits) & Self::kHashMask;
        (value & !Self::kHashMask) | hash_masked
    }
}

pub enum SeqCstAccessTag {}
