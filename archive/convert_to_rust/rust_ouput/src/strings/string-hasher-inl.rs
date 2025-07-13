// Converted from V8 C++ source files:
// Header: string-hasher-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::mem;
use std::os::raw::c_char;

//use crate::objects::string::String;
//use crate::strings::string_hasher::StringHasher;
//use crate::strings::char_predicates_inl::IsInRange;
//use crate::utils::utils_inl::kMaxUInt64;

// Mock declarations for types and constants used in the original C++ code
pub struct String {
 k_max_hash_calc_length: usize,
 k_max_array_index_size: usize,
 k_max_integer_index_size: usize,
}

impl String {
 const kMaxLength: usize = 1024;
 const kMaxHashCalcLength: usize = 16;
 const kMaxArrayIndexSize: usize = 9;
 const kMaxIntegerIndexSize: usize = 12;
 const kArrayIndexValueBits: ArrayIndexValueBits = ArrayIndexValueBits {};
 const HashBits: HashBits = HashBits {};
 fn CreateHashFieldValue(hash: u32, field_type: HashFieldType) -> u32 {
  hash
 }
 fn IsIntegerIndex(value: u32) -> bool {
  value > 0
 }
}

struct ArrayIndexValueBits {}
impl ArrayIndexValueBits {
 const kShift: u32 = 0;
}
struct ArrayIndexLengthBits {}
impl ArrayIndexLengthBits {
 const kShift: u32 = 0;
}

struct HashBits {}
impl HashBits {
 const kMax: u64 = 0xFFFFFFFF;
}

struct Name {}
impl Name {
 fn ContainsCachedArrayIndex(hash: u32) -> bool {
  hash > 0
 }
}

#[derive(Debug, PartialEq, Eq)]
enum HashFieldType {
 kHash,
 kIntegerIndex,
}

struct StringHasher {}
impl StringHasher {
 const kZeroHash: u32 = 0;
}

const kMaxUInt64: u64 = u64::MAX;
const kMaxSafeIntegerUint64: u64 = 9007199254740991;

fn TenToThe(exponent: usize) -> u64 {
  let mut result: u64 = 1;
  for _ in 0..exponent {
   result = result.wrapping_mul(10);
  }
  result
}

struct V8_EXPORT_PRIVATE {}

mod detail {
 use super::*;
 extern "C" {
  pub fn rapidhash(data: *const u8, len: usize, seed: u64) -> u64;
 }

 pub fn HashConvertingTo8Bit(chars: *const u16, length: u32, seed: u64) -> u64 {
  let slice = unsafe { std::slice::from_raw_parts(chars, length as usize) };
  let mut bytes: Vec<u8> = Vec::with_capacity(length as usize);
  for &c in slice {
   bytes.push(c as u8);
  }
  let ptr = bytes.as_ptr();
  unsafe { rapidhash(ptr, length as usize, seed) }
 }

 pub fn ConvertRawHashToUsableHash(raw_hash: u64) -> u32 {
  let hash = (raw_hash & String::HashBits::kMax) as i32;
  if hash == 0 {
   StringHasher::kZeroHash
  } else {
   hash as u32
  }
 }

 pub fn IsOnly8Bit(chars: *const u16, len: u32) -> bool {
  let slice = unsafe { std::slice::from_raw_parts(chars, len as usize) };
  for &c in slice {
   if c > 255 {
    return false;
   }
  }
  true
 }

 pub fn GetRapidHash_u8(chars: *const u8, length: u32, seed: u64) -> u64 {
  unsafe { rapidhash(chars, length as usize, seed) }
 }

 pub fn GetRapidHash_u16(chars: *const u16, length: u32, seed: u64) -> u64 {
  if unsafe { IsOnly8Bit(chars, length) } {
   unsafe { HashConvertingTo8Bit(chars, length, seed) }
  } else {
   unsafe {
    let byte_ptr = chars as *const u8;
    rapidhash(byte_ptr, (2 * length) as usize, seed)
   }
  }
 }

 pub fn GetUsableRapidHash_u8(chars: *const u8, length: u32, seed: u64) -> u32 {
  ConvertRawHashToUsableHash(GetRapidHash_u8(chars, length, seed))
 }

 pub fn GetUsableRapidHash_u16(chars: *const u16, length: u32, seed: u64) -> u32 {
  ConvertRawHashToUsableHash(GetRapidHash_u16(chars, length, seed))
 }

 #[derive(Debug, PartialEq, Eq)]
 pub enum IndexParseResult {
  kSuccess,
  kNonIndex,
  kOverflow,
 }

 #[cfg(target_arch = "x86_64")]
 type ArrayIndexT = u64;
 #[cfg(not(target_arch = "x86_64"))]
 type ArrayIndexT = u32;

 pub fn TryParseArrayIndex(
  chars: *const u16,
  length: u32,
  i: &mut u32,
  index: &mut ArrayIndexT,
 ) -> IndexParseResult {
  if length == 0 {
   return IndexParseResult::kNonIndex;
  }
  if length > String::kMaxIntegerIndexSize as u32 {
   return IndexParseResult::kOverflow;
  }

  let chars_slice = unsafe { std::slice::from_raw_parts(chars, length as usize) };

  *index = (chars_slice[0] as u64).wrapping_sub('0' as u64);
  *i = 1;

  if *index > 9 {
   return IndexParseResult::kNonIndex;
  }
  if *index == 0 {
   if length > 1 {
    return IndexParseResult::kNonIndex;
   }
   return IndexParseResult::kSuccess;
  }

  if length as usize > String::kMaxArrayIndexSize {
   return IndexParseResult::kOverflow;
  }

  for j in 1..length {
   let c = chars_slice[j as usize];
   let val: u64 = (c as u64).wrapping_sub('0' as u64);
   if val > 9 {
    return IndexParseResult::kNonIndex;
   }
   *index = (10 * *index).wrapping_add(val);
  }

  if mem::size_of::<ArrayIndexT>() == 8 {
   if *index > String::kMaxArrayIndex as u64 {
    return IndexParseResult::kOverflow;
   }
  } else {
   if length as usize == String::kMaxArrayIndexSize {
    const K_MIN_VALID_VALUE: u64 = 100000000;
    if *index + 1 < K_MIN_VALID_VALUE + 1 {
     return IndexParseResult::kNonIndex;
    }
   }
  }
  return IndexParseResult::kSuccess;
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
  if length > String::kMaxIntegerIndexSize as u32 {
   return IndexParseResult::kOverflow;
  }

  let chars_slice = unsafe { std::slice::from_raw_parts(chars, length as usize) };

  let mut mutable_index = index;
  let mut mutable_i = i;

  for j in mutable_i..length {
   let c = chars_slice[j as usize];
   let val: u64 = (c as u64).wrapping_sub('0' as u64);
   if val > 9 {
    return IndexParseResult::kNonIndex;
   }
   mutable_index = (10 * mutable_index).wrapping_add(val);
  }

  if mutable_index > kMaxSafeIntegerUint64 {
   return IndexParseResult::kOverflow;
  }

  return IndexParseResult::kSuccess;
 }
}

#[derive(Default)]
pub struct RunningStringHasher {
 running_hash_: u32,
}

impl RunningStringHasher {
 pub fn AddCharacter(&mut self, c: u16) {
  self.running_hash_ = self.running_hash_.wrapping_add(c as u32);
  self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 10);
  self.running_hash_ ^= self.running_hash_ >> 6;
 }

 pub fn Finalize(&mut self) -> u32 {
  self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 3);
  self.running_hash_ ^= self.running_hash_ >> 11;
  self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 15);
  detail::ConvertRawHashToUsableHash(self.running_hash_ as u64)
 }
}

pub struct StringHasher {}

impl StringHasher {
 pub fn GetTrivialHash(length: u32) -> u32 {
  assert!(length > String::kMaxHashCalcLength as u32);
  let hash = length;
  String::CreateHashFieldValue(hash, HashFieldType::kHash)
 }

 pub fn MakeArrayIndexHash(value: u32, length: u32) -> u32 {
  assert!(length <= String::kMaxArrayIndexSize as u32);
  let mut value = value << String::ArrayIndexValueBits::kShift;
  value |= length << ArrayIndexLengthBits::kShift;
  assert!(String::IsIntegerIndex(value));
  value
 }

 #[allow(unused_variables)]
 pub fn HashSequentialString<T>(chars_raw: *const T, length: u32, seed: u64) -> u32 {
  use std::any::TypeId;
  use std::mem;
  assert!(mem::size_of::<T>() <= 2);
  if TypeId::of::<T>() == TypeId::of::<i8>() {
   let chars = chars_raw as *const i8;
   StringHasher::hash_sequential_string_impl(chars, length, seed)
  } else if TypeId::of::<T>() == TypeId::of::<u8>() {
   let chars = chars_raw as *const u8;
   StringHasher::hash_sequential_string_impl(chars, length, seed)
  } else if TypeId::of::<T>() == TypeId::of::<i16>() {
   let chars = chars_raw as *const i16;
   StringHasher::hash_sequential_string_impl(chars, length, seed)
  } else if TypeId::of::<T>() == TypeId::of::<u16>() {
   let chars = chars_raw as *const u16;
   StringHasher::hash_sequential_string_impl(chars, length, seed)
  } else {
   panic!("Unsupported char type");
  }
 }

 fn hash_sequential_string_impl<U>(chars_raw: *const U, length: u32, seed: u64) -> u32 {
  let chars = chars_raw as *const u16;
  if length >= 1 {
   if length <= String::kMaxIntegerIndexSize as u32 {
    let mut index: detail::ArrayIndexT = 0;
    let mut i: u32 = 0;
    match unsafe { detail::TryParseArrayIndex(chars, length, &mut i, &mut index) } {
     detail::IndexParseResult::kSuccess => {
      assert!(index <= String::kMaxArrayIndex as u64);
      return StringHasher::MakeArrayIndexHash(index as u32, length);
     }
     detail::IndexParseResult::kNonIndex => {}
     detail::IndexParseResult::kOverflow => {
      #[cfg(target_arch = "x86_64")]
      {
       match unsafe { detail::TryParseIntegerIndex(chars, length, i, index) } {
        detail::IndexParseResult::kSuccess => {
         let mut hash = String::CreateHashFieldValue(
          unsafe { detail::GetUsableRapidHash_u16(chars, length, seed) },
          HashFieldType::kIntegerIndex,
         );
         if Name::ContainsCachedArrayIndex(hash) {
          hash |= (String::kMaxHashCalcLength as u32 + 1) << ArrayIndexLengthBits::kShift;
         }
         assert!(!Name::ContainsCachedArrayIndex(hash));
         return hash;
        }
        detail::IndexParseResult::kNonIndex => {}
        detail::IndexParseResult::kOverflow => {}
       }
      }
     }
    }
   } else if length > String::kMaxHashCalcLength as u32 {
    return StringHasher::GetTrivialHash(length);
   }
  }

  String::CreateHashFieldValue(
   unsafe { detail::GetUsableRapidHash_u16(chars, length, seed) },
   HashFieldType::kHash,
  )
 }
}

pub struct SeededStringHasher {
 hashseed_: u64,
}

impl SeededStringHasher {
 pub fn new(hashseed: u64) -> Self {
  SeededStringHasher { hashseed_: hashseed }
 }

 pub fn call(&self, name: *const c_char) -> usize {
  let len = unsafe {
   let mut len = 0;
   while *name.offset(len) != 0 {
    len += 1;
   }
   len as u32
  };
  StringHasher::HashSequentialString(name as *const i8, len, self.hashseed_) as usize
 }
}
