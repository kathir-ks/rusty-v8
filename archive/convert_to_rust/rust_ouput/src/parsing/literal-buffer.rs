// Converted from V8 C++ source files:
// Header: literal-buffer.h
// Implementation: literal-buffer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn begin_mut(&mut self) -> *mut T {
            self.data.as_mut_ptr()
        }

        pub fn Dispose(&mut self) {
            self.data.clear();
            self.data.shrink_to_fit();
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }
        pub fn New(capacity: usize) -> Self
            where T: Default + Copy,
        {
            Vector { data: vec![T::default(); capacity] }
        }
    }
}

pub mod unibrow {
    pub mod Latin1 {
        pub const kMaxChar: u32 = 255;
    }

    pub mod Utf16 {
        pub const kMaxNonSurrogateCharCode: u32 = 0xD7FF;

        pub fn LeadSurrogate(code_unit: u32) -> u16 {
            ((code_unit >> 10) + 0xD800) as u16
        }

        pub fn TrailSurrogate(code_unit: u32) -> u16 {
            ((code_unit & 0x3FF) + 0xDC00) as u16
        }
    }
}

pub mod execution {
    pub struct Isolate {}
    pub struct LocalIsolate {}
}

pub mod heap {
    pub struct Factory {}
}

pub mod utils {
    pub mod memcopy {
        pub fn MemCopy<T: Copy>(dst: *mut T, src: *const T, count: usize) {
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst, count);
            }
        }
    }
}

pub struct DirectHandle<T> {
    value: T,
}

pub struct String {}

impl DirectHandle<String> {
    pub fn new(value: String) -> Self {
        DirectHandle { value }
    }
}

pub mod parsing {
    use crate::base::Vector;
    use crate::unibrow;
    use std::mem;

    pub struct LiteralBuffer {
        backing_store_: Vector<u8>,
        position_: usize,
        is_one_byte_: bool,
    }

    impl LiteralBuffer {
        pub fn new() -> Self {
            LiteralBuffer {
                backing_store_: Vector::new(Vec::new()),
                position_: 0,
                is_one_byte_: true,
            }
        }

        pub fn AddChar(&mut self, code_unit: char) {
            assert!(self.IsValidAscii(code_unit));
            self.AddOneByteChar(code_unit as u8);
        }

        pub fn AddChar_uc32(&mut self, code_unit: u32) {
            if self.is_one_byte() {
                if code_unit <= unibrow::Latin1::kMaxChar {
                    self.AddOneByteChar(code_unit as u8);
                    return;
                }
                self.ConvertToTwoByte();
            }
            self.AddTwoByteChar(code_unit);
        }

        pub fn is_one_byte(&self) -> bool {
            self.is_one_byte_
        }

        pub fn Equals(&self, keyword: Vector<char>) -> bool {
            self.is_one_byte()
                && keyword.length() == self.position_
                && unsafe {
                    libc::memcmp(
                        keyword.begin() as *const std::ffi::c_void,
                        self.backing_store_.begin() as *const std::ffi::c_void,
                        self.position_,
                    ) == 0
                }
        }

        pub fn two_byte_literal(&self) -> Vector<u16> {
            self.literal::<u16>()
        }

        pub fn one_byte_literal(&self) -> Vector<u8> {
            self.literal::<u8>()
        }

        pub fn literal<Char: Copy>(&self) -> Vector<Char> {
            assert_eq!(self.is_one_byte_, mem::size_of::<Char>() == 1);
            assert_eq!(self.position_ & (mem::size_of::<Char>() - 1), 0);
            let ptr = self.backing_store_.begin() as *const Char;
            let len = self.position_ / mem::size_of::<Char>();
             unsafe {
                 let slice = std::slice::from_raw_parts(ptr, len);
                 Vector::new(slice.to_vec())
             }
        }

        pub fn length(&self) -> usize {
            if self.is_one_byte() {
                self.position_
            } else {
                self.position_ >> 1
            }
        }

        pub fn Start(&mut self) {
            self.position_ = 0;
            self.is_one_byte_ = true;
            self.backing_store_.Dispose();
            self.backing_store_ = Vector::new(Vec::new());
        }

        pub fn Internalize<IsolateT>(&self, isolate: &IsolateT) -> DirectHandle<String> {
             if self.is_one_byte() {
                  isolate.factory().InternalizeString(self.one_byte_literal())
              } else {
                  isolate.factory().InternalizeString(self.two_byte_literal())
             }
        }

        fn IsValidAscii(&self, code_unit: char) -> bool {
            code_unit.is_ascii_control() || code_unit.is_ascii_graphic()
        }

        fn AddOneByteChar(&mut self, one_byte_char: u8) {
            assert!(self.is_one_byte());
            if self.position_ >= self.backing_store_.length() {
                self.ExpandBuffer();
            }
            unsafe {
                let ptr = self.backing_store_.begin_mut().add(self.position_);
                *ptr = one_byte_char;
            }
            self.position_ += kOneByteSize;
        }

        fn AddTwoByteChar(&mut self, code_unit: u32) {
            assert!(!self.is_one_byte());
            if self.position_ >= self.backing_store_.length() {
                self.ExpandBuffer();
            }
            if code_unit <= unibrow::Utf16::kMaxNonSurrogateCharCode {
                let ptr = self.backing_store_.begin_mut() as *mut u8;
                unsafe {
                    *(ptr.add(self.position_) as *mut u16) = code_unit as u16;
                }
                self.position_ += kUC16Size;
            } else {
                let ptr = self.backing_store_.begin_mut() as *mut u8;
                unsafe {
                    *(ptr.add(self.position_) as *mut u16) = unibrow::Utf16::LeadSurrogate(code_unit);
                }
                self.position_ += kUC16Size;
                if self.position_ >= self.backing_store_.length() {
                    self.ExpandBuffer();
                }
                let ptr = self.backing_store_.begin_mut() as *mut u8;
                unsafe {
                    *(ptr.add(self.position_) as *mut u16) = unibrow::Utf16::TrailSurrogate(code_unit);
                }
                self.position_ += kUC16Size;
            }
        }

        fn NewCapacity(&self, min_capacity: usize) -> usize {
            if min_capacity < (kMaxGrowth / (kGrowthFactor - 1)) {
                min_capacity * kGrowthFactor
            } else {
                min_capacity + kMaxGrowth
            }
        }

        fn ExpandBuffer(&mut self) {
            let min_capacity = std::cmp::max(kInitialCapacity, self.backing_store_.length());
            let new_capacity = self.NewCapacity(min_capacity);
            let mut new_store: Vector<u8> = Vector::New(new_capacity);
            if self.position_ > 0 {
                 crate::utils::memcopy::MemCopy(
                    new_store.begin_mut(),
                    self.backing_store_.begin(),
                    self.position_,
                );
            }
            self.backing_store_.Dispose();
            self.backing_store_ = new_store;
        }

        fn ConvertToTwoByte(&mut self) {
            assert!(self.is_one_byte());
            let mut new_store: Vector<u8>;
            let new_content_size = self.position_ * kUC16Size;
            if new_content_size >= self.backing_store_.length() {
                new_store = Vector::New(self.NewCapacity(new_content_size));
            } else {
                new_store = self.backing_store_;
            }
            let src = self.backing_store_.begin() as *const u8;
            let dst = new_store.begin_mut() as *mut u8 as *mut u16;

            for i in 0..self.position_ {
                unsafe {
                    *dst.add(i) = *src.add(i) as u16;
                }
            }

            if new_store.begin() != self.backing_store_.begin() {
                self.backing_store_.Dispose();
                self.backing_store_ = new_store;
            }
            self.position_ = new_content_size;
            self.is_one_byte_ = false;
        }
    }

    impl Drop for LiteralBuffer {
        fn drop(&mut self) {
            self.backing_store_.Dispose();
        }
    }

    const kInitialCapacity: usize = 256;
    const kGrowthFactor: usize = 4;
    const kMaxGrowth: usize = 1 * 1024 * 1024;
    const kOneByteSize: usize = 1;
    const kUC16Size: usize = 2;
}

pub trait IsolateTrait {
    fn factory(&self) -> &Factory;
}

impl IsolateTrait for execution::Isolate {
    fn factory(&self) -> &Factory {
        todo!()
    }
}

impl IsolateTrait for execution::LocalIsolate {
    fn factory(&self) -> &Factory {
        todo!()
    }
}

impl heap::Factory {
    pub fn InternalizeString(&self, literal: base::Vector<u8>) -> DirectHandle<String> {
           DirectHandle::new(String{})
    }
    pub fn InternalizeString(&self, literal: base::Vector<u16>) -> DirectHandle<String> {
        DirectHandle::new(String{})
 }
}
