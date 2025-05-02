// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: `Zone` is assumed to be a custom memory arena allocator,
// and its functionality is not directly replicable with standard Rust allocators.
// For simplicity and to avoid introducing external dependencies,
// this conversion uses the standard `Box` for memory allocation.

use std::cmp;
use std::ops::{Deref, DerefMut};

// TODO: Replace with actual zone allocator
struct Zone;

impl Zone {
    fn allocate_array<T>(&self, length: usize) -> Box<[T]>
    where
        T: Default + Copy,
    {
        vec![T::default(); length].into_boxed_slice()
    }
}

// Mimic V8_EXPORT_PRIVATE.  In a real project, this would control
// symbol visibility.
macro_rules! V8_EXPORT_PRIVATE {
    () => {};
}

// Mimic V8_INLINE, compiler should inline these anyway.
macro_rules! V8_INLINE {
    () => {};
}

// Mimic V8_ASSUME, skip checks.
macro_rules! V8_ASSUME {
    ($condition:expr) => {};
}

// Mimic V8_NOEXCEPT, no exceptions.
macro_rules! V8_NOEXCEPT {
    () => {};
}

// Mimic V8_UNLIKELY, skip branch prediction
macro_rules! V8_UNLIKELY {
    ($condition:expr) => {
        $condition
    };
}

// Mimic CHECK_GE, skip checks.
macro_rules! CHECK_GE {
    ($x:expr, $y:expr) => {};
}

mod base {
    pub mod bits {
        pub fn count_trailing_zeros(x: usize) -> u32 {
            x.trailing_zeros()
        }
        pub fn round_up_to_power_of_two32(x: u32) -> u32 {
            let mut x = x;
            x -= 1;
            x |= x >> 1;
            x |= x >> 2;
            x |= x >> 4;
            x |= x >> 8;
            x |= x >> 16;
            x + 1
        }
    }
}

mod internal {
    use super::{base, Zone};
    use std::cmp;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    pub struct BitVector {
        length: usize,
        data: DataStorage,
        data_begin: *mut usize,
        data_end: *mut usize,
    }

    #[derive(Debug)]
    union DataStorage {
        ptr: *mut usize,
        inline_: usize,
    }

    impl DataStorage {
        const fn new(value: usize) -> Self {
            DataStorage { inline_: value }
        }
    }

    impl BitVector {
        pub const K_DATA_BITS: usize = usize::BITS as usize;
        pub const K_DATA_BIT_SHIFT: usize = usize::BITS.ilog2() as usize;

        pub fn new() -> Self {
            Self {
                length: 0,
                data: DataStorage::new(0),
                data_begin: std::ptr::null_mut(),
                data_end: std::ptr::null_mut(),
            }
        }

        pub fn with_length(length: usize, zone: &Zone) -> Self {
            assert!(length >= 0);
            let data_length = (length + Self::K_DATA_BITS - 1) >> Self::K_DATA_BIT_SHIFT;

            if data_length > 1 {
                let mut data = zone.allocate_array::<usize>(data_length);
                data.fill(0);

                let data_begin = data.as_mut_ptr();
                let data_end = unsafe { data_begin.add(data_length) };
                Self {
                    length,
                    data: DataStorage {
                        ptr: data_begin,
                    },
                    data_begin,
                    data_end,
                }
            } else {
                let mut data = Box::new([0]);
                let data_begin = data.as_mut_ptr();
                let data_end = unsafe { data_begin.add(1) };
                Self {
                    length,
                    data: DataStorage::new(0),
                    data_begin,
                    data_end,
                }
            }
        }

        pub fn copy_from(other: &BitVector, zone: &Zone) -> Self {
            let mut new_bv = Self::with_length(other.length, zone);
            new_bv.copy_from_internal(other);
            new_bv
        }

        fn copy_from_internal(&mut self, other: &BitVector) {
            assert_eq!(other.length(), self.length());
            assert_eq!(self.is_inline(), other.is_inline());

            unsafe {
                std::ptr::copy_nonoverlapping(
                    other.data_begin,
                    self.data_begin,
                    self.data_length(),
                );
            }
        }

        pub fn resize(&mut self, new_length: usize, zone: &Zone) {
            assert!(new_length > self.length());
            let old_data_length = self.data_length();
            assert!(1 <= old_data_length);
            let new_data_length = (new_length + Self::K_DATA_BITS - 1) >> Self::K_DATA_BIT_SHIFT;

            if new_data_length > old_data_length {
                let mut new_data = zone.allocate_array::<usize>(new_data_length);

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        self.data_begin,
                        new_data.as_mut_ptr(),
                        old_data_length,
                    );
                }
                new_data[old_data_length..].fill(0);

                self.data_begin = new_data.as_mut_ptr();
                self.data_end = unsafe { self.data_begin.add(new_data_length) };
                self.data = DataStorage {
                    ptr: self.data_begin,
                };
            }

            self.length = new_length;
        }

        pub fn contains(&self, i: usize) -> bool {
            assert!(i >= 0 && i < self.length());
            unsafe { (*self.data_begin.add(Self::word(i)) & Self::bit(i)) != 0 }
        }

        pub fn add(&mut self, i: usize) {
            assert!(i >= 0 && i < self.length());
            unsafe { *self.data_begin.add(Self::word(i)) |= Self::bit(i) };
        }

        pub fn add_all(&mut self) {
            let data_length = self.data_length();
            let slice = unsafe { std::slice::from_raw_parts_mut(self.data_begin, data_length) };
            slice.fill(!0);
        }

        pub fn remove(&mut self, i: usize) {
            assert!(i >= 0 && i < self.length());
            unsafe { *self.data_begin.add(Self::word(i)) &= !Self::bit(i) };
        }

        pub fn union(&mut self, other: &BitVector) {
            assert_eq!(other.length(), self.length());
            for i in 0..self.data_length() {
                unsafe { *self.data_begin.add(i) |= *other.data_begin.add(i) };
            }
        }

        pub fn union_is_changed(&mut self, other: &BitVector) -> bool {
            assert_eq!(other.length(), self.length());
            let mut changed = false;
            for i in 0..self.data_length() {
                let old_data = unsafe { *self.data_begin.add(i) };
                unsafe { *self.data_begin.add(i) |= *other.data_begin.add(i) };
                if unsafe { *self.data_begin.add(i) != old_data } {
                    changed = true;
                }
            }
            return changed;
        }

        pub fn intersect(&mut self, other: &BitVector) {
            assert_eq!(other.length(), self.length());
            for i in 0..self.data_length() {
                unsafe { *self.data_begin.add(i) &= *other.data_begin.add(i) };
            }
        }

        pub fn intersect_is_changed(&mut self, other: &BitVector) -> bool {
            assert_eq!(other.length(), self.length());
            let mut changed = false;
            for i in 0..self.data_length() {
                let old_data = unsafe { *self.data_begin.add(i) };
                unsafe { *self.data_begin.add(i) &= *other.data_begin.add(i) };
                if unsafe { *self.data_begin.add(i) != old_data } {
                    changed = true;
                }
            }
            return changed;
        }

        pub fn subtract(&mut self, other: &BitVector) {
            assert_eq!(other.length(), self.length());
            for i in 0..self.data_length() {
                unsafe { *self.data_begin.add(i) &= !*other.data_begin.add(i) };
            }
        }

        pub fn clear(&mut self) {
            let data_length = self.data_length();
            let slice = unsafe { std::slice::from_raw_parts_mut(self.data_begin, data_length) };
            slice.fill(0);
        }

        pub fn is_empty(&self) -> bool {
            let slice = unsafe { std::slice::from_raw_parts(self.data_begin, self.data_length()) };
            slice.iter().all(|&x| x == 0)
        }

        pub fn equals(&self, other: &BitVector) -> bool {
            if self.data_length() != other.data_length() {
                return false;
            }
            let self_slice =
                unsafe { std::slice::from_raw_parts(self.data_begin, self.data_length()) };
            let other_slice =
                unsafe { std::slice::from_raw_parts(other.data_begin, other.data_length()) };

            self_slice == other_slice
        }

        // TODO: Implement Count()
        pub fn count(&self) -> i32 {
            0
        }

        pub fn length(&self) -> usize {
            self.length
        }

        pub fn begin(&self) -> Iterator {
            Iterator::new(self, StartTag)
        }

        pub fn end(&self) -> Iterator {
            Iterator::new(self, EndTag)
        }

        fn is_inline(&self) -> bool {
            unsafe { self.data_begin == &self.data.inline_ as *const usize as *mut usize }
        }

        fn data_length(&self) -> usize {
            unsafe { self.data_end.offset_from(self.data_begin) as usize }
        }

        #[inline]
        fn word(index: usize) -> usize {
            //V8_ASSUME!(index >= 0); // index is usize, it is >= 0
            index >> Self::K_DATA_BIT_SHIFT
        }

        #[inline]
        fn bit(index: usize) -> usize {
            1 << (index & (Self::K_DATA_BITS - 1))
        }

        //#[cfg(debug_assertions)]
        //pub fn print(&self) {
        //    println!("{:?}", self);
        //}
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Iterator<'a> {
        target: &'a BitVector,
        ptr: *mut usize,
        end: *mut usize,
        current_index: usize,
    }

    impl<'a> Iterator<'a> {
        fn new(target: &'a BitVector, tag: Tag) -> Self {
            match tag {
                Tag::Start(StartTag) => {
                    let mut iter = Iterator {
                        target,
                        ptr: target.data_begin,
                        end: target.data_end,
                        current_index: 0,
                    };
                    //assert!(iter.ptr < iter.end); // ptr is checked as we iterate
                    while unsafe { *iter.ptr } == 0 {
                        unsafe { iter.ptr = iter.ptr.add(1) };
                        iter.current_index += BitVector::K_DATA_BITS;
                        if iter.ptr == iter.end {
                            return iter;
                        }
                    }
                    iter.current_index += base::bits::count_trailing_zeros(unsafe { *iter.ptr }) as usize;
                    iter
                }
                Tag::End(EndTag) => Iterator {
                    target,
                    ptr: target.data_end,
                    end: target.data_end,
                    current_index: target.data_length() * BitVector::K_DATA_BITS,
                },
            }
        }
    }

    impl<'a> Iterator<'a> {
        pub fn next(&mut self) -> Option<usize> {
            if self.ptr == self.end {
                return None;
            }

            let result = self.current_index;

            let bit_in_word = self.current_index & (BitVector::K_DATA_BITS - 1);
            if bit_in_word < BitVector::K_DATA_BITS - 1 {
                let remaining_bits = unsafe { *self.ptr } >> (bit_in_word + 1);
                if remaining_bits != 0 {
                    let next_bit_in_word =
                        base::bits::count_trailing_zeros(remaining_bits as usize) as usize;
                    self.current_index += next_bit_in_word + 1;
                    return Some(result);
                }
            }

            // Move {current_index_} down to the beginning of the current word, before
            // starting to search for the next non-empty word.
            self.current_index = self.current_index - (self.current_index & (BitVector::K_DATA_BITS - 1));
            loop {
                unsafe { self.ptr = self.ptr.add(1) };
                self.current_index += BitVector::K_DATA_BITS;
                if self.ptr == self.end {
                    return Some(result);
                }
                if unsafe { *self.ptr != 0 } {
                    break;
                }
            }
            let trailing_zeros = base::bits::count_trailing_zeros(unsafe { *self.ptr }) as usize;
            self.current_index += trailing_zeros;

            Some(result)
        }
    }

    impl<'a> std::iter::Iterator for Iterator<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr == self.end {
                return None;
            }

            //DCHECK_NE(end_, ptr_);
            //DCHECK(target_->Contains(current_index_));
            let current_index = self.current_index;

            let bit_in_word = self.current_index & (BitVector::K_DATA_BITS - 1);
            if bit_in_word < BitVector::K_DATA_BITS - 1 {
                let remaining_bits = unsafe { *self.ptr } >> (bit_in_word + 1);
                if remaining_bits != 0 {
                    let next_bit_in_word =
                        base::bits::count_trailing_zeros(remaining_bits as usize) as usize;
                    self.current_index += next_bit_in_word + 1;
                    return Some(current_index);
                }
            }

            // Move {current_index_} down to the beginning of the current word, before
            // starting to search for the next non-empty word.
            self.current_index = self.current_index - (self.current_index & (BitVector::K_DATA_BITS - 1));
            loop {
                unsafe { self.ptr = self.ptr.add(1) };
                self.current_index += BitVector::K_DATA_BITS;
                if self.ptr == self.end {
                    return None;
                }
                if unsafe { *self.ptr != 0 } {
                    break;
                }
            }
            let trailing_zeros = base::bits::count_trailing_zeros(unsafe { *self.ptr }) as usize;
            self.current_index += trailing_zeros;

            Some(current_index)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct StartTag;
    #[derive(Debug, PartialEq, Eq)]
    struct EndTag;

    #[derive(Debug, PartialEq, Eq)]
    enum Tag {
        Start(StartTag),
        End(EndTag),
    }

    impl<'a> IntoIterator for &'a BitVector {
        type Item = usize;
        type IntoIter = Iterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            self.begin()
        }
    }

    impl Drop for BitVector {
        fn drop(&mut self) {
            if !self.is_inline() {
                // Safety: `data_begin` and `data_end` are valid pointers
                // within the allocated `Box`.
                let data_length = self.data_length();
                if data_length > 0 {
                    unsafe {
                        let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.data_begin, data_length));
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct GrowableBitVector {
        bits: BitVector,
    }

    impl GrowableBitVector {
        pub fn new() -> Self {
            GrowableBitVector {
                bits: BitVector::new(),
            }
        }

        pub fn with_length(length: usize, zone: &Zone) -> Self {
            GrowableBitVector {
                bits: BitVector::with_length(length, zone),
            }
        }

        pub fn contains(&self, value: usize) -> bool {
            if !self.in_bits_range(value) {
                return false;
            }
            self.bits.contains(value)
        }

        pub fn add(&mut self, value: usize, zone: &Zone) {
            if V8_UNLIKELY!(!self.in_bits_range(value)) {
                self.grow(value, zone);
            }
            self.bits.add(value);
        }

        pub fn is_empty(&self) -> bool {
            self.bits.is_empty()
        }

        pub fn clear(&mut self) {
            self.bits.clear();
        }

        pub fn length(&self) -> usize {
            self.bits.length()
        }

        pub fn equals(&self, other: &GrowableBitVector) -> bool {
            self.length() == other.length() && self.bits.equals(&other.bits)
        }

        pub fn begin(&self) -> Iterator {
            self.bits.begin()
        }

        pub fn end(&self) -> Iterator {
            self.bits.end()
        }

        const K_INITIAL_LENGTH: usize = 1024;
        const K_MAX_SUPPORTED_VALUE: usize = (1 << 30) - 1;

        fn in_bits_range(&self, value: usize) -> bool {
            self.bits.length() > value
        }

        #[inline(never)]
        fn grow(&mut self, needed_value: usize, zone: &Zone) {
            assert!(!self.in_bits_range(needed_value));
            // Ensure that {RoundUpToPowerOfTwo32} does not overflow {int} range.
            CHECK_GE!(Self::K_MAX_SUPPORTED_VALUE, needed_value);

            let new_length = cmp::max(
                Self::K_INITIAL_LENGTH,
                base::bits::round_up_to_power_of_two32((needed_value + 1) as u32) as usize,
            );
            self.bits.resize(new_length, zone);
        }
    }
} // namespace internal