// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]

mod utils; // Assuming utils-inl.h and utils.h are in a 'utils' module
mod objects; // Assuming string-inl.h and name-inl.h are in an 'objects' module
mod strings {
    pub mod char_predicates;
    pub mod string_hasher;
}

use std::{
    mem,
    os::raw::c_char,
    ptr,
    slice,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::objects::{
    name::Name,
    string::String,
};
use crate::strings::string_hasher::StringHasher;
use crate::utils::*;

// rapidhash-v8 is assumed to be implemented here or linked externally.
// Replace with a suitable Rust hashing library if needed.
extern "C" {
    fn rapidhash(data: *const u8, length: u32, seed: u64) -> u64;
}

pub mod detail {
    use super::*;

    // Placeholder for V8_EXPORT_PRIVATE (assuming it's only for internal linkage)
    pub fn HashConvertingTo8Bit(chars: *const u16, length: u32, seed: u64) -> u64 {
        unsafe {
            let slice = slice::from_raw_parts(chars, length as usize);
            let mut hash: u64 = seed;
            for &c in slice.iter() {
                hash = hash.wrapping_add(c as u64);
                hash = hash.wrapping_add(hash << 10);
                hash ^= hash >> 6;
            }
            hash
        }
    }

    pub fn ConvertRawHashToUsableHash<T: Into<i64>>(raw_hash: T) -> u32 {
        let raw_hash_i64 = raw_hash.into();
        let hash = (raw_hash_i64 & String::HashBits::kMax as i64) as i32;
        if hash == 0 {
            StringHasher::kZeroHash
        } else {
            hash as u32
        }
    }

    pub fn IsOnly8Bit(chars: *const u16, len: u32) -> bool {
        unsafe {
            let slice = slice::from_raw_parts(chars, len as usize);
            for &c in slice.iter() {
                if c > 255 {
                    return false;
                }
            }
            true
        }
    }

    pub fn GetRapidHash_u8(chars: *const u8, length: u32, seed: u64) -> u64 {
        unsafe { rapidhash(chars, length, seed) }
    }

    pub fn GetRapidHash_u16(chars: *const u16, length: u32, seed: u64) -> u64 {
        unsafe {
            if IsOnly8Bit(chars, length) {
                return HashConvertingTo8Bit(chars, length, seed);
            }
            GetRapidHash_u8(
                chars as *const u8, // Cast *const u16 to *const u8
                2 * length,
                seed,
            )
        }
    }

    pub fn GetUsableRapidHash<T>(chars: *const T, length: u32, seed: u64) -> u32 {
        // This assumes T is u8 or u16; adjust as needed if supporting other types.
        if mem::size_of::<T>() == 1 {
            let chars_u8 = chars as *const u8;
            ConvertRawHashToUsableHash(GetRapidHash_u8(chars_u8, length, seed))
        } else if mem::size_of::<T>() == 2 {
            let chars_u16 = chars as *const u16;
            ConvertRawHashToUsableHash(GetRapidHash_u16(chars_u16, length, seed))
        } else {
            panic!("Unsupported char size."); //Or, return a Result
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum IndexParseResult {
        kSuccess,
        kNonIndex,
        kOverflow,
    }

    #[cfg(target_arch = "x86_64")]
    pub type ArrayIndexT = u64;
    #[cfg(not(target_arch = "x86_64"))]
    pub type ArrayIndexT = u32;

    pub fn TryParseArrayIndex(
        chars: *const u16,
        length: u32,
        i_out: &mut u32,
        index_out: &mut ArrayIndexT,
    ) -> IndexParseResult {
        if length == 0 {
            return IndexParseResult::kNonIndex;
        }
        if length > String::kMaxIntegerIndexSize {
            return IndexParseResult::kNonIndex;
        }

        unsafe {
            let chars_slice = slice::from_raw_parts(chars, length as usize);
            let mut index = (chars_slice[0] as u32).wrapping_sub('0' as u32) as ArrayIndexT;
            let mut i = 1;

            if index > 9 {
                return IndexParseResult::kNonIndex;
            }

            if index == 0 {
                if length > 1 {
                    return IndexParseResult::kNonIndex;
                }
                *i_out = i;
                *index_out = index;
                return IndexParseResult::kSuccess;
            }

            if length > String::kMaxArrayIndexSize {
                return IndexParseResult::kOverflow;
            }

            while i < length {
                let c = chars_slice[i as usize];
                let val = (c as u32).wrapping_sub('0' as u32);
                if val > 9 {
                    return IndexParseResult::kNonIndex;
                }
                index = (10 * index) + val as ArrayIndexT;
                i += 1;
            }

            #[cfg(target_arch = "x86_64")]
            {
                if index > String::kMaxArrayIndex as ArrayIndexT {
                    return IndexParseResult::kOverflow;
                }
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                if length == String::kMaxArrayIndexSize {
                    const K_MIN_VALID_VALUE: u32 = TenToThe(String::kMaxArrayIndexSize - 1) as u32;
                    if (index + 1) < (K_MIN_VALID_VALUE + 1) as ArrayIndexT {
                        return IndexParseResult::kNonIndex;
                    }
                }
            }
            *i_out = i;
            *index_out = index;
            return IndexParseResult::kSuccess;
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn TryParseIntegerIndex(
        chars: *const u16,
        length: u32,
        i: u32,
        index: ArrayIndexT,
    ) -> IndexParseResult {
        if length == 0 {
            return IndexParseResult::kNonIndex;
        }
        if length > String::kMaxIntegerIndexSize {
            return IndexParseResult::kNonIndex;
        }
        if i == 0 {
            return IndexParseResult::kNonIndex;
        }
        if index == 0 {
            return IndexParseResult::kNonIndex;
        }
        if index > kMaxSafeIntegerUint64 {
            return IndexParseResult::kNonIndex;
        }

        let mut mutable_index = index;
        let mut mutable_i = i;

        unsafe {
            let chars_slice = slice::from_raw_parts(chars, length as usize);
            while mutable_i < length {
                let c = chars_slice[mutable_i as usize];
                let val = (c as u32).wrapping_sub('0' as u32);
                if val > 9 {
                    return IndexParseResult::kNonIndex;
                }
                mutable_index = (10 * mutable_index) + val as ArrayIndexT;
                mutable_i += 1;
            }
            if mutable_index > kMaxSafeIntegerUint64 {
                return IndexParseResult::kOverflow;
            }

            return IndexParseResult::kSuccess;
        }
    }
} // namespace detail

pub struct RunningStringHasher {
    running_hash_: u32,
}

impl RunningStringHasher {
    pub fn new() -> Self {
        RunningStringHasher { running_hash_: 0 }
    }

    pub fn AddCharacter(&mut self, c: u16) {
        self.running_hash_ += c as u32;
        self.running_hash_ += self.running_hash_ << 10;
        self.running_hash_ ^= self.running_hash_ >> 6;
    }

    pub fn Finalize(&mut self) -> u32 {
        self.running_hash_ += self.running_hash_ << 3;
        self.running_hash_ ^= self.running_hash_ >> 11;
        self.running_hash_ += self.running_hash_ << 15;
        detail::ConvertRawHashToUsableHash(self.running_hash_)
    }
}

impl StringHasher {
    pub fn GetTrivialHash(length: u32) -> u32 {
        assert!(length > String::kMaxHashCalcLength);
        // The hash of a large string is simply computed from the length.
        // Ensure that the max length is small enough to be encoded without losing
        // information.
        let hash = length;
        String::CreateHashFieldValue(hash, String::HashFieldType::kHash)
    }

    pub fn MakeArrayIndexHash(value: u32, length: u32) -> u32 {
        // For array indexes mix the length into the hash as an array index could
        // be zero.
        assert!(length <= String::kMaxArrayIndexSize);
        assert!(TenToThe(String::kMaxCachedArrayIndexLength) < (1 << String::ArrayIndexValueBits::kShift));

        let mut value = value << String::ArrayIndexValueBits::kShift;
        value |= length << String::ArrayIndexLengthBits::kShift;

        assert!(String::IsIntegerIndex(value));
        assert_eq!(
            length <= String::kMaxCachedArrayIndexLength,
            Name::ContainsCachedArrayIndex(value)
        );
        value
    }
}

impl StringHasher {
    pub fn HashSequentialString<T>(chars_raw: *const T, length: u32, seed: u64) -> u32 {
        assert!(mem::size_of::<T>() <= 2);

        if length >= 1 {
            if length <= String::kMaxIntegerIndexSize {
                // Possible array or integer index; try to compute the array index hash.
                assert!(String::kMaxArrayIndexSize <= String::kMaxIntegerIndexSize);

                let mut index: detail::ArrayIndexT = 0;
                let mut i: u32 = 0;
                let chars_u16 = chars_raw as *const u16; // Convert to u16 pointer
                match detail::TryParseArrayIndex(chars_u16, length, &mut i, &mut index) {
                    detail::IndexParseResult::kSuccess => {
                        assert!(index <= String::kMaxArrayIndex as detail::ArrayIndexT);
                        return StringHasher::MakeArrayIndexHash(index as u32, length);
                    }
                    detail::IndexParseResult::kNonIndex => {
                        // A non-index result from TryParseArrayIndex means we don't need to
                        // check for integer indices.
                    }
                    detail::IndexParseResult::kOverflow => {
                        #[cfg(target_arch = "x86_64")]
                        {
                            // On 64-bit, we might have a valid integer index even if the value
                            // overflowed an array index.
                            assert!(String::kMaxArrayIndexSize < String::kMaxIntegerIndexSize);
                            let chars_u16 = chars_raw as *const u16; // Convert to u16 pointer
                            match detail::TryParseIntegerIndex(chars_u16, length, i, index) {
                                detail::IndexParseResult::kSuccess => {
                                    let mut hash = String::CreateHashFieldValue(
                                        detail::GetUsableRapidHash(chars_u16, length, seed),
                                        String::HashFieldType::kIntegerIndex,
                                    );
                                    if Name::ContainsCachedArrayIndex(hash) {
                                        // The hash accidentally looks like a cached index. Fix that by
                                        // setting a bit that looks like a longer-than-cacheable string
                                        // length.
                                        hash |= (String::kMaxCachedArrayIndexLength + 1)
                                            << String::ArrayIndexLengthBits::kShift;
                                    }
                                    assert!(!Name::ContainsCachedArrayIndex(hash));
                                    return hash;
                                }
                                detail::IndexParseResult::kNonIndex => {}
                                detail::IndexParseResult::kOverflow => {}
                            }
                        }
                        #[cfg(not(target_arch = "x86_64"))]
                        {
                            assert_eq!(String::kMaxArrayIndexSize, String::kMaxIntegerIndexSize);
                        }
                    }
                }
                // If the we failed to compute an index hash, this falls through into the
                // non-index hash case.
            } else if length > String::kMaxHashCalcLength {
                // We should never go down this path if we might have an index value.
                assert!(String::kMaxHashCalcLength > String::kMaxIntegerIndexSize);
                assert!(String::kMaxHashCalcLength > String::kMaxArrayIndexSize);
                return StringHasher::GetTrivialHash(length);
            }
        }

        // Non-index hash.
        if mem::size_of::<T>() == 1 {
            let chars_u8 = chars_raw as *const u8;
            return String::CreateHashFieldValue(
                detail::GetUsableRapidHash(chars_u8, length, seed),
                String::HashFieldType::kHash,
            );
        } else if mem::size_of::<T>() == 2 {
            let chars_u16 = chars_raw as *const u16;
            return String::CreateHashFieldValue(
                detail::GetUsableRapidHash(chars_u16, length, seed),
                String::HashFieldType::kHash,
            );
        } else {
            panic!("Unsupported char size.");
        }
    }
}

pub struct SeededStringHasher {
    hashseed_: u64,
}

impl SeededStringHasher {
    pub fn new(hashseed: u64) -> Self {
        SeededStringHasher { hashseed_: hashseed }
    }
}

impl SeededStringHasher {
    pub fn hash(&self, name: &str) -> usize {
        StringHasher::HashSequentialString(
            name.as_ptr() as *const i8,
            name.len() as u32,
            self.hashseed_,
        ) as usize
    }
}

//Implement the Fn trait to mimic the operator() from C++
impl FnOnce<(&str,)> for SeededStringHasher {
    type Output = usize;

    extern "rust-call" fn call_once(self, args: (&str,)) -> Self::Output {
        self.hash(args.0)
    }
}

impl FnMut<(&str,)> for SeededStringHasher {
    extern "rust-call" fn call_mut(&mut self, args: (&str,)) -> Self::Output {
        self.hash(args.0)
    }
}

impl Fn<(&str,)> for SeededStringHasher {
    extern "rust-call" fn call(&self, args: (&str,)) -> Self::Output {
        self.hash(args.0)
    }
}