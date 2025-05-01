// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
//#[cfg(not(feature = "webassembly"))]
//compile_error!("This header should only be included if WebAssembly is enabled.");

use std::{
    any::Any,
    fmt,
    fmt::Debug,
    io::Read,
    mem,
    ops::{Deref, DerefMut},
    result,
};

//use crate::base::strings::String;
//use crate::base::vector::Vector;
//use crate::flags::flags;
//use crate::utils::utils;
//use crate::wasm::wasm_opcodes::WasmOpcode;
//use crate::wasm::wasm_result::VoidResult;

macro_rules! TRACE {
    ($($arg:tt)*) => {
        if v8_flags.trace_wasm_decoder {
            println!($($arg)*);
        }
    };
}

macro_rules! TRACE_IF {
    ($cond:expr, $($arg:tt)*) => {
        if v8_flags.trace_wasm_decoder && ($cond) {
            println!($($arg)*);
        }
    };
}

// A {DecodeResult} only stores the failure / success status, but no data.
pub type DecodeResult = VoidResult;

pub struct WasmFunction {}

pub trait ITracer {
    // Hooks for extracting byte offsets of things.
    fn type_offset(&mut self, offset: u32);
    fn import_offset(&mut self, offset: u32);
    fn imports_done(&mut self, module: &WasmModule);
    fn table_offset(&mut self, offset: u32);
    fn memory_offset(&mut self, offset: u32);
    fn tag_offset(&mut self, offset: u32);
    fn global_offset(&mut self, offset: u32);
    fn start_offset(&mut self, offset: u32);
    fn element_offset(&mut self, offset: u32);
    fn data_offset(&mut self, offset: u32);
    fn string_offset(&mut self, offset: u32);
    fn rec_group_offset(&mut self, offset: u32, group_size: u32);

    // Hooks for annotated hex dumps.
    fn bytes(&mut self, start: *const u8, count: u32);

    fn description_str(&mut self, desc: &str);
    fn description_usize(&mut self, length: usize);
    fn description_u32(&mut self, number: u32);
    fn description_u64(&mut self, number: u64);
    fn description_value_type(&mut self, type_: ValueType);
    fn description_heap_type(&mut self, type_: HeapType);
    fn description_function_sig(&mut self, sig: &FunctionSig);

    fn next_line(&mut self);
    fn next_line_if_full(&mut self);
    fn next_line_if_non_empty(&mut self);

    fn initializer_expression(
        &mut self,
        start: *const u8,
        end: *const u8,
        expected_type: ValueType,
    );
    fn function_body(&mut self, func: &WasmFunction, start: *const u8);
    fn function_name(&mut self, func_index: u32);
    fn name_section(&mut self, start: *const u8, end: *const u8, offset: u32);
}

// A helper utility to decode bytes, integers, fields, varints, etc, from
// a buffer of bytes.
pub struct Decoder<'a> {
    start_: *const u8,
    pc_: *const u8,
    end_: *const u8,
    // The offset of the current buffer in the module. Needed for streaming.
    buffer_offset_: u32,
    error_: WasmError,
    _marker: std::marker::PhantomData<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    // Don't run validation, assume valid input.
    pub const K_NO_VALIDATION: NoValidationTag = NoValidationTag {};
    // Run full validation with error message and location.
    pub const K_FULL_VALIDATION: FullValidationTag = FullValidationTag {};

    pub fn new(start: *const u8, end: *const u8, buffer_offset: u32) -> Self {
        Decoder {
            start_: start,
            pc_: start,
            end_: end,
            buffer_offset_: buffer_offset,
            error_: WasmError::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn from_bytes(bytes: &'a [u8], buffer_offset: u32) -> Self {
        let start = bytes.as_ptr();
        let end = unsafe { start.add(bytes.len()) };
        Decoder {
            start_: start,
            pc_: start,
            end_: end,
            buffer_offset_: buffer_offset,
            error_: WasmError::default(),
            _marker: std::marker::PhantomData,
        }
    }

    // Reads an 8-bit unsigned integer.
    pub fn read_u8<ValidationTag>(&self, pc: *const u8, msg: &str) -> u8 {
        self.read_little_endian::<u8, ValidationTag>(pc, msg)
    }

    // Reads a 16-bit unsigned integer (little endian).
    pub fn read_u16<ValidationTag>(&self, pc: *const u8, msg: &str) -> u16 {
        self.read_little_endian::<u16, ValidationTag>(pc, msg)
    }

    // Reads a 32-bit unsigned integer (little endian).
    pub fn read_u32<ValidationTag>(&self, pc: *const u8, msg: &str) -> u32 {
        self.read_little_endian::<u32, ValidationTag>(pc, msg)
    }

    // Reads a 64-bit unsigned integer (little endian).
    pub fn read_u64<ValidationTag>(&self, pc: *const u8, msg: &str) -> u64 {
        self.read_little_endian::<u64, ValidationTag>(pc, msg)
    }

    // Reads a variable-length unsigned integer (little endian). Returns the read
    // value and the number of bytes read.
    pub fn read_u32v<ValidationTag>(&self, pc: *const u8, name: &str) -> (u32, u32) {
        self.read_leb::<u32, ValidationTag, false>(pc, name)
    }

    // Reads a variable-length signed integer (little endian). Returns the read
    // value and the number of bytes read.
    pub fn read_i32v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i32, u32) {
        self.read_leb::<i32, ValidationTag, false>(pc, name)
    }

    // Reads a variable-length unsigned integer (little endian). Returns the read
    // value and the number of bytes read.
    pub fn read_u64v<ValidationTag>(&self, pc: *const u8, name: &str) -> (u64, u32) {
        self.read_leb::<u64, ValidationTag, false>(pc, name)
    }

    // Reads a variable-length signed integer (little endian). Returns the read
    // value and the number of bytes read.
    pub fn read_i64v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i64, u32) {
        self.read_leb::<i64, ValidationTag, false>(pc, name)
    }

    // Reads a variable-length 33-bit signed integer (little endian). Returns the
    // read value and the number of bytes read.
    pub fn read_i33v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i64, u32) {
        self.read_leb::<i64, ValidationTag, false, 33>(pc, name)
    }

    // Reads a prefixed-opcode, possibly with variable-length index.
    // Returns the read opcode and the number of bytes that make up this opcode,
    // *including* the prefix byte. For most opcodes, it will be 2.
    pub fn read_prefixed_opcode<ValidationTag>(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (WasmOpcode, u32) {
        // Prefixed opcodes all use LEB128 encoding.
        let (index, index_length) =
            self.read_u32v::<ValidationTag>(unsafe { pc.add(1) }, "prefixed opcode index");
        let length = index_length + 1; // 1 for prefix byte.
                                        // Only support opcodes that go up to 0xFFF (when decoded). Anything
                                        // bigger will need more than 2 bytes, and the '<< 12' below will be wrong.
        if ValidationTag::validate && index > 0xfff {
            self.errorf(pc, "Invalid prefixed opcode {}", index);
            // On validation failure we return "unreachable" (opcode 0).
            //static_assert!(kExprUnreachable == 0);
            return (WasmOpcode::Unreachable, 0);
        }

        if index > 0xff {
            let opcode_val = unsafe { (*pc as u32) << 12 | index };
            (WasmOpcode::from(opcode_val as usize), length)
        } else {
            let opcode_val = unsafe { (*pc as u32) << 8 | index };
            (WasmOpcode::from(opcode_val as usize), length)
        }
    }

    // Reads a 8-bit unsigned integer (byte) and advances {pc_}.
    pub fn consume_u8(&mut self, name: &str) -> u8 {
        self.consume_little_endian::<u8, true>(name)
    }

    pub fn consume_u8_with_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u8 {
        if tracer as *mut dyn ITracer as *mut dyn Any != std::ptr::null_mut() {
            tracer.bytes(self.pc_, mem::size_of::<u8>() as u32);
            tracer.description_str(name);
        }
        self.consume_little_endian::<u8, false>(name)
    }

    // Reads a 16-bit unsigned integer (little endian) and advances {pc_}.
    pub fn consume_u16(&mut self, name: &str) -> u16 {
        self.consume_little_endian::<u16, true>(name)
    }

    // Reads a single 32-bit unsigned integer (little endian) and advances {pc_}.
    pub fn consume_u32_with_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u32 {
        if tracer as *mut dyn ITracer as *mut dyn Any != std::ptr::null_mut() {
            tracer.bytes(self.pc_, mem::size_of::<u32>() as u32);
            tracer.description_str(name);
        }
        self.consume_little_endian::<u32, false>(name)
    }

    // Reads a LEB128 variable-length unsigned 32-bit integer and advances {pc_}.
    pub fn consume_u32v(&mut self, name: &str) -> u32 {
        let (result, length) = self.read_leb::<u32, FullValidationTag, true>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_u32v_with_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u32 {
        let (result, length) = self.read_leb::<u32, FullValidationTag, false>(self.pc_, name);
        if tracer as *mut dyn ITracer as *mut dyn Any != std::ptr::null_mut() {
            tracer.bytes(self.pc_, length);
            tracer.description_str(name);
        }
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    // Reads a LEB128 variable-length signed 32-bit integer and advances {pc_}.
    pub fn consume_i32v(&mut self, name: &str) -> i32 {
        let (result, length) = self.read_leb::<i32, FullValidationTag, true>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    // Reads a LEB128 variable-length unsigned 64-bit integer and advances {pc_}.
    pub fn consume_u64v_with_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u64 {
        let (result, length) = self.read_leb::<u64, FullValidationTag, false>(self.pc_, name);
        if tracer as *mut dyn ITracer as *mut dyn Any != std::ptr::null_mut() {
            tracer.bytes(self.pc_, length);
            tracer.description_str(name);
        }
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    // Reads a LEB128 variable-length signed 64-bit integer and advances {pc_}.
    pub fn consume_i64v(&mut self, name: &str) -> i64 {
        let (result, length) = self.read_leb::<i64, FullValidationTag, true>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    // Consume {size} bytes and send them to the bit bucket, advancing {pc_}.
    pub fn consume_bytes(&mut self, size: u32, name: Option<&str>) {
        // Only trace if the name is not null.
        if let Some(name) = name {
            TRACE_IF!(true, "  +{}  {:<20}: {} bytes\n", self.pc_offset(), name, size);
        }
        if self.check_available(size) {
            unsafe {
                self.pc_ = self.pc_.add(size as usize);
            }
        } else {
            self.pc_ = self.end_;
        }
    }

    pub fn consume_bytes_with_tracer(&mut self, size: u32, name: &str, tracer: &mut dyn ITracer) {
        if tracer as *mut dyn ITracer as *mut dyn Any != std::ptr::null_mut() {
            tracer.bytes(self.pc_, size);
            tracer.description_str(name);
        }
        self.consume_bytes(size, None);
    }

    pub fn available_bytes(&self) -> u32 {
        let pc = self.pc_ as usize;
        let end = self.end_ as usize;
        assert!(pc <= end);
        assert!(u32::MAX as usize >= end - pc);
        (end - pc) as u32
    }

    // Check that at least {size} bytes exist between {pc_} and {end_}.
    pub fn check_available(&self, size: u32) -> bool {
        if size > self.available_bytes() {
            self.errorf(self.pc_, "expected {} bytes, fell off end", size);
            return false;
        }
        true
    }

    // Do not inline error methods. This has measurable impact on validation time,
    // see https://crbug.com/910432.
    // Behavior triggered on first error, overridden in subclasses.
    pub fn on_first_error(&mut self) {}

    // Debugging helper to print a bytes range as hex bytes.
    pub fn trace_byte_range(&self, start: *const u8, end: *const u8) {
        assert!(start <= end);
        for p in unsafe { std::slice::from_raw_parts(start, (end as usize) - (start as usize)) }
        {
            TRACE!("{:02x} ", *p);
        }
    }

    // Debugging helper to print bytes up to the end.
    pub fn trace_off_end(&self) {
        self.trace_byte_range(self.pc_, self.end_);
        TRACE!("<end>\n");
    }

    // Converts the given value to a {Result}, copying the error if necessary.
    pub fn to_result<T, R>(&self, val: T) -> Result<R, WasmError>
    where
        T: Into<R>,
    {
        if self.failed() {
            TRACE!("Result error: {}\n", self.error_.message());
            Err(self.error_.clone())
        } else {
            Ok(val.into())
        }
    }

    // Resets the boundaries of this decoder.
    pub fn reset(&mut self, start: *const u8, end: *const u8, buffer_offset: u32) {
        assert!(start <= end);
        assert!((end as usize - start as usize) <= u32::MAX as usize);
        self.start_ = start;
        self.pc_ = start;
        self.end_ = end;
        self.buffer_offset_ = buffer_offset;
        self.error_ = WasmError::default();
    }

    pub fn reset_from_bytes(&mut self, bytes: &'a [u8], buffer_offset: u32) {
        let start = bytes.as_ptr();
        let end = unsafe { start.add(bytes.len()) };
        self.reset(start, end, buffer_offset);
    }

    pub fn ok(&self) -> bool {
        !self.failed()
    }
    pub fn failed(&self) -> bool {
        self.error_.has_error()
    }
    pub fn more(&self) -> bool {
        self.pc_ < self.end_
    }
    pub fn error(&self) -> &WasmError {
        &self.error_
    }

    pub fn start(&self) -> *const u8 {
        self.start_
    }
    pub fn pc(&self) -> *const u8 {
        self.pc_
    }
    #[inline]
    pub fn position(&self) -> u32 {
        let start = self.start_ as usize;
        let pc = self.pc_ as usize;
        (pc - start) as u32
    }
    // This needs to be inlined for performance (see https://crbug.com/910432).
    #[inline]
    pub fn pc_offset_ptr(&self, pc: *const u8) -> u32 {
        let start = self.start_ as usize;
        let pc_usize = pc as usize;
        assert!(start <= pc_usize);
        assert!(u32::MAX as usize - self.buffer_offset_ as usize >= pc_usize - start);
        ((pc_usize - start) as u32) + self.buffer_offset_
    }

    pub fn pc_offset(&self) -> u32 {
        self.pc_offset_ptr(self.pc_)
    }
    pub fn buffer_offset(&self) -> u32 {
        self.buffer_offset_
    }
    // Takes an offset relative to the module start and returns an offset relative
    // to the current buffer of the decoder.
    pub fn get_buffer_relative_offset(&self, offset: u32) -> u32 {
        assert!(self.buffer_offset_ <= offset);
        offset - self.buffer_offset_
    }
    pub fn end(&self) -> *const u8 {
        self.end_
    }
    pub fn set_end(&mut self, end: *const u8) {
        self.end_ = end;
    }

    // Check if the uint8_t at {offset} from the current pc equals {expected}.
    pub fn lookahead(&self, offset: isize, expected: u8) -> bool {
        assert!(self.pc_ <= self.end_);
        let end = self.end_ as usize;
        let pc = self.pc_ as usize;

        if end.saturating_sub(pc) > offset as usize {
            let target_ptr = unsafe { self.pc_.offset(offset) };
            unsafe { *target_ptr == expected }
        } else {
            false
        }
    }

    fn read_little_endian<IntType, ValidationTag>(&self, pc: *const u8, msg: &str) -> IntType
    where
        IntType: Sized + Copy + Debug,
    {
        if !ValidationTag::validate {
            let pc_usize = pc as usize;
            let end_usize = self.end_ as usize;
            assert!(pc_usize <= end_usize);
            assert!(std::mem::size_of::<IntType>() <= end_usize - pc_usize);
        } else if (std::mem::size_of::<IntType>() as isize) > (unsafe { self.end_.offset_from(pc) }) {
            self.error(pc, msg);
            return IntType::default();
        }
        unsafe { pc.cast::<IntType>().read_unaligned() }
    }

    fn consume_little_endian<IntType, const TRACE: bool>(&mut self, name: &str) -> IntType
    where
        IntType: Sized + Copy + Debug,
    {
        if TRACE {
            TRACE!("  +{}  {:<20}: ", self.pc_offset(), name);
        }
        if !self.check_available(std::mem::size_of::<IntType>() as u32) {
            self.trace_off_end();
            self.pc_ = self.end_;
            return IntType::default();
        }
        let val = self.read_little_endian::<IntType, NoValidationTag>(self.pc_, name);
        self.trace_byte_range(self.pc_, unsafe { self.pc_.add(std::mem::size_of::<IntType>()) });
        if TRACE {
            TRACE!("= {:?}\n", val);
        }
        unsafe {
            self.pc_ = self.pc_.add(std::mem::size_of::<IntType>());
        }
        val
    }

    // The implementation of LEB-decoding; returns the value and the number of
    // bytes read.
    #[inline]
    fn read_leb<IntType, ValidationTag, const TRACE: bool, const SIZE_IN_BITS: usize = 0>(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (IntType, u32)
    where
        IntType: Sized + Copy + Debug,
    {
        let size_in_bits = if SIZE_IN_BITS == 0 {
            8 * std::mem::size_of::<IntType>()
        } else {
            SIZE_IN_BITS
        };

        if TRACE {
            TRACE!("  +{}  {:<20}: ", self.pc_offset(), name);
        }
        // Fast path for single-byte integers.

        if !ValidationTag::validate || (pc < self.end_ && unsafe { !(*pc & 0x80 != 0) }) {
            let b = unsafe { *pc };
            if TRACE {
                TRACE!("{:02x} ", b);
            }
            let result: IntType = unsafe { *pc as IntType };
            if std::any::TypeId::of::<IntType>() != std::any::TypeId::of::<u32>() {
                // Perform sign extension.
                let sign_ext_shift: i32 = (8 * std::mem::size_of::<IntType>() as i32) - 7;
                let signed_result: IntType = result.wrapping_shl(sign_ext_shift as u32).wrapping_shr(sign_ext_shift as u32);
                if TRACE {
                    TRACE!("= {}\n", signed_result);
                }
                (signed_result, 1)
            } else {
                if TRACE {
                    TRACE!("= {}\n", result);
                }
                (result, 1)
            }
        } else {
            self.read_leb_slowpath::<IntType, ValidationTag, TRACE, SIZE_IN_BITS>(pc, name)
        }
    }

    #[inline(never)]
    #[cold]
    fn read_leb_slowpath<IntType, ValidationTag, const TRACE: bool, const SIZE_IN_BITS: usize>(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (IntType, u32)
    where
        IntType: Sized + Copy + Debug,
    {
        // Create an unrolled LEB decoding function per integer type.
        self.read_leb_tail::<IntType, ValidationTag, TRACE, SIZE_IN_BITS, 0>(pc, name, 0)
    }

    #[inline]
    fn read_leb_tail<
        IntType,
        ValidationTag,
        const TRACE: bool,
        const SIZE_IN_BITS: usize,
        const BYTE_INDEX: usize,
    >(
        &self,
        pc: *const u8,
        name: &str,
        intermediate_result: IntType,
    ) -> (IntType, u32)
    where
        IntType: Sized + Copy + Debug,
    {
        const IS_SIGNED: bool = std::any::TypeId::of::<IntType>() != std::any::TypeId::of::<u32>();
        const MAX_LENGTH: usize = (if SIZE_IN_BITS == 0 {
            8 * std::mem::size_of::<IntType>()
        } else {
            SIZE_IN_BITS
        } + 6)
            / 7;

        let at_end = ValidationTag::validate && pc >= self.end_;
        let mut b: u8 = 0;
        if !at_end {
            b = unsafe { *pc };
            if TRACE {
                TRACE!("{:02x} ", b);
            }

            let shift = BYTE_INDEX * 7;
            let unsigned_b = b & 0x7f;
            let signed_b = unsafe { std::mem::transmute::<u8, i8>(b) };
            let next_intermediate_result: IntType = intermediate_result
                .wrapping_add(((unsigned_b as u64) << shift) as IntType);

            if !((BYTE_INDEX + 1) == MAX_LENGTH) && (b & 0x80 != 0) {
                return self.read_leb_tail::<
                    IntType,
                    ValidationTag,
                    TRACE,
                    SIZE_IN_BITS,
                    { BYTE_INDEX + 1 },
                >(unsafe { pc.add(1) }, name, next_intermediate_result);
            }

            if ValidationTag::validate && (at_end || (b & 0x80 != 0)) {
                TRACE!("{}", if at_end { "<end> " } else { "<length overflow> " });
                self.errorf(
                    pc,
                    "{} while decoding {}",
                    if at_end { "reached end" } else { "length overflow" },
                    name,
                );
                return (0 as IntType, 0);
            }

            let mut intermediate_result = next_intermediate_result;

            if (BYTE_INDEX + 1) == MAX_LENGTH {
                // A signed-LEB128 must sign-extend the final byte, excluding its
                // most-significant bit; e.g. for a 32-bit LEB128:
                //   kExtraBits = 4  (== 32 - (5-1) * 7)
                // For unsigned values, the extra bits must be all zero.
                // For signed values, the extra bits *plus* the most significant bit must
                // either be 0, or all ones.
                let k_extra_bits = if SIZE_IN_BITS == 0 {
                    8 * std::mem::size_of::<IntType>()
                } else {
                    SIZE_IN_BITS
                } - ((MAX_LENGTH - 1) * 7);

                let k_sign_ext_bits = k_extra_bits - if IS_SIGNED { 1 } else { 0 };

                let checked_bits = b & (0xFFu8.wrapping_shl(k_sign_ext_bits as u32));
                let k_sign_extended_extra_bits =
                    0x7fu8 & (0xFFu8.wrapping_shl(k_sign_ext_bits as u32));
                let valid_extra_bits = checked_bits == 0
                    || (IS_SIGNED && checked_bits == k_sign_extended_extra_bits);
                if !ValidationTag::validate {
                    assert!(valid_extra_bits);
                } else if !valid_extra_bits {
                    self.error(pc, "extra bits in varint");
                    return (0 as IntType, 0);
                }
            }

            let shift = if IS_SIGNED {
                std::cmp::max(
                    0,
                    (8 * std::mem::size_of::<IntType>() as i32) - (BYTE_INDEX as i32 * 7) - 7,
                )
            } else {
                0
            };

            if std::any::TypeId::of::<IntType>() != std::any::TypeId::of::<u32>() {
                intermediate_result = unsafe {
                    std::mem::transmute::<i64, IntType>((intermediate_result as i64).wrapping_shl(shift as u32).wrapping_shr(shift as u32))
                };

                if TRACE {
                    TRACE!("= {}\n", intermediate_result);
                }
            } else {
                if TRACE {
                    TRACE!("= {}\n", intermediate_result);
                }
            }
            let length = (BYTE_INDEX + 1) as u32;
            return (intermediate_result, length);
        } else {
            TRACE!("{}", if at_end { "<end> " } else { "<length overflow> " });
            self.errorf(
                pc,
                "{} while decoding {}",
                if at_end { "reached end" } else { "length overflow" },
                name,
            );
            return (0 as IntType, 0);
        }
    }
}

#[derive(Default, Clone)]
pub struct WasmError {
    offset: u32,
    message: String,
}

impl WasmError {
    pub fn new(offset: u32, message: String) -> Self {
        WasmError { offset, message }
    }

    pub fn has_error(&self) -> bool {
        !self.message.is_empty()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WasmError {{ offset: {}, message: {} }}", self.offset, self.message)
    }
}

trait PrivateErrorExtension {
    fn errorf(&mut self, offset: u32, format: String);
    fn error(&mut self, pc: *const u8, msg: &str);
    fn error_offset(&mut self, offset: u32, msg: &str);
    fn error_pc(&mut self, pc: *const u8, msg: &str);
}

impl<'a> PrivateErrorExtension for Decoder<'a> {
    // Behavior triggered on first error, overridden in subclasses.
    fn errorf(&mut self, offset: u32, format: String) {
        // Only report the first error.
        if !self.ok() {
            return;
        }
        let k_max_error_msg = 256;
        if format.len() > k_max_error_msg {
            panic!("Error message exceeds maximum length");
        }
        self.error_ = WasmError::new(offset, format);
        self.on_first_error();
    }

    fn error(&mut self, pc: *const u8, msg: &str) {
        self.errorf(self.pc_offset_ptr(pc), msg.to_string());
    }

    fn error_offset(&mut self, offset: u32, msg: &str) {
        self.errorf(offset, msg.to_string());
    }

    fn error_pc(&mut self, pc: *const u8, msg: &str) {
        self.errorf(self.pc_offset_ptr(pc), msg.to_string());
    }
}

// Marker types for template specialisation
pub struct NoValidationTag {}
impl NoValidationTag {
    pub const validate: bool = false;
}
pub struct FullValidationTag {}
impl FullValidationTag {
    pub const validate: bool = true;
}

// Placeholder types, replace with actual implementation
pub type VoidResult = Result<(), WasmError>;
pub struct WasmModule {}
pub struct FunctionSig {}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {}
#[derive(