// Converted from V8 C++ source files:
// Header: vlq.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
  use std::{convert::TryInto, fmt, io::Read, marker::PhantomData};

  const K_CONTINUE_SHIFT: u32 = 7;
  const K_CONTINUE_BIT: u32 = 1 << K_CONTINUE_SHIFT;
  const K_DATA_MASK: u32 = K_CONTINUE_BIT - 1;

  // Encodes an unsigned value using variable-length encoding and stores it using
  // the passed process_byte function.
  pub fn vlq_encode_unsigned<F>(mut process_byte: F, value: u32)
  where
    F: FnMut(u8),
  {
    // Write as many bytes as necessary to encode the value, with 7 bits of data
    // per byte (leaving space for one continuation bit).
    const K_DATA_BITS_PER_BYTE: u32 = K_CONTINUE_SHIFT;
    if value < 1 << (K_DATA_BITS_PER_BYTE) {
      return process_byte(value as u8);
    }
    if value < 1 << (2 * K_DATA_BITS_PER_BYTE) {
      process_byte((value | K_CONTINUE_BIT) as u8);
      return process_byte((value >> K_CONTINUE_SHIFT) as u8);
    }
    if value < 1 << (3 * K_DATA_BITS_PER_BYTE) {
      process_byte((value | K_CONTINUE_BIT) as u8);
      process_byte(((value >> K_CONTINUE_SHIFT) | K_CONTINUE_BIT) as u8);
      return process_byte((value >> (2 * K_CONTINUE_SHIFT)) as u8);
    }
    if value < 1 << (4 * K_DATA_BITS_PER_BYTE) {
      process_byte((value | K_CONTINUE_BIT) as u8);
      process_byte(((value >> K_CONTINUE_SHIFT) | K_CONTINUE_BIT) as u8);
      process_byte(((value >> (2 * K_CONTINUE_SHIFT)) | K_CONTINUE_BIT) as u8);
      return process_byte((value >> (3 * K_CONTINUE_SHIFT)) as u8);
    }
    process_byte((value | K_CONTINUE_BIT) as u8);
    process_byte(((value >> K_CONTINUE_SHIFT) | K_CONTINUE_BIT) as u8);
    process_byte(((value >> (2 * K_CONTINUE_SHIFT)) | K_CONTINUE_BIT) as u8);
    process_byte(((value >> (3 * K_CONTINUE_SHIFT)) | K_CONTINUE_BIT) as u8);
    process_byte((value >> (4 * K_CONTINUE_SHIFT)) as u8);
  }

  pub fn vlq_convert_to_unsigned(value: i32) -> u32 {
    let is_negative = value < 0;
    // Encode sign in least significant bit.
    let abs_value = value.abs() as u32;
    (abs_value << 1) | (is_negative as u32)
  }

  // Encodes value using variable-length encoding and stores it using the passed
  // process_byte function.
  pub fn vlq_encode<F>(mut process_byte: F, value: i32)
  where
    F: FnMut(u8),
  {
    let bits = vlq_convert_to_unsigned(value);
    vlq_encode_unsigned(process_byte, bits);
  }

  // Wrapper of VLQEncode for std::vector backed storage containers.
  pub fn vlq_encode_vec<A>(data: &mut Vec<u8, A>, value: i32)
  where
    A: std::alloc::Allocator,
  {
    vlq_encode(|byte| data.push(byte), value);
  }

  // Wrapper of VLQEncodeUnsigned for std::vector backed storage containers.
  pub fn vlq_encode_unsigned_vec<A>(data: &mut Vec<u8, A>, value: u32)
  where
    A: std::alloc::Allocator,
  {
    vlq_encode_unsigned(|byte| data.push(byte), value);
  }

  // Decodes a variable-length encoded unsigned value from bytes returned by
  // successive calls to the given function.
  pub fn vlq_decode_unsigned<F>(mut get_next: F) -> u32
  where
    F: FnMut() -> u8,
  {
    let cur_byte = get_next();
    // Single byte fast path; no need to mask.
    if cur_byte <= K_DATA_MASK as u8 {
      return cur_byte as u32;
    }
    let mut bits = (cur_byte & K_DATA_MASK as u8) as u32;
    let mut shift = K_CONTINUE_SHIFT;
    loop {
      let cur_byte = get_next();
      bits |= ((cur_byte & K_DATA_MASK as u8) as u32) << shift;
      if cur_byte <= K_DATA_MASK as u8 {
        break;
      }
      shift += K_CONTINUE_SHIFT;
      if shift > 32 {
        // Handle the case of malformed VLQ
        break;
      }
    }
    bits
  }

  // Decodes a variable-length encoded unsigned value stored in contiguous memory
  // starting at data_start + index, updating index to where the next encoded
  // value starts.
  pub fn vlq_decode_unsigned_from_slice(data_start: &[u8], index: &mut usize) -> u32 {
    let mut local_index = *index;
    let result = vlq_decode_unsigned(|| {
      let byte = data_start[local_index];
      local_index += 1;
      byte
    });
    *index = local_index;
    result
  }

  // Decodes a variable-length encoded value stored in contiguous memory starting
  // at data_start + index, updating index to where the next encoded value starts.
  pub fn vlq_decode_from_slice(data_start: &[u8], index: &mut usize) -> i32 {
    let bits = vlq_decode_unsigned_from_slice(data_start, index);
    let is_negative = (bits & 1) == 1;
    let result = (bits >> 1) as i32;
    if is_negative {
      -result
    } else {
      result
    }
  }
} // namespace base
} // namespace v8
