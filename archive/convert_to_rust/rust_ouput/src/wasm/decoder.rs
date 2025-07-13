// Converted from V8 C++ source files:
// Header: decoder.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::fmt;
use std::io::Write;
use std::mem;
use std::ptr;
use std::result;
//use crate::base::compiler_specific::*;
//use crate::base::strings::*;
//use crate::base::vector::*;
//use crate::flags::flags::*;
//use crate::utils::utils::*;
use crate::wasm::wasm_opcodes::WasmOpcode;
use crate::wasm::wasm_result::VoidResult;
use std::convert::TryInto;

pub type DecodeResult = VoidResult;

pub struct WasmFunction {}

pub trait ITracer {
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
    fn bytes(&mut self, start: *const u8, count: u32);
    fn description_str(&mut self, desc: &str);
    fn description_sized_str(&mut self, desc: &str, length: usize);
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

#[derive(Debug, Clone)]
pub struct WasmError {
    offset: u32,
    message: String,
}

impl WasmError {
    pub fn new(offset: u32, message: String) -> WasmError {
        WasmError { offset, message }
    }

    pub fn has_error(&self) -> bool {
        !self.message.is_empty()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// Implement Display for WasmError for easy printing
impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at offset {}: {}", self.offset, self.message)
    }
}

pub struct NoValidationTag {
    pub validate: bool,
}

impl NoValidationTag {
    pub const validate: bool = false;
}

pub struct FullValidationTag {
    pub validate: bool,
}

impl FullValidationTag {
    pub const validate: bool = true;
}

pub struct NoName {
    name: Option<String>,
}

impl NoName {
    pub fn new(_s: &str) -> NoName {
        NoName { name: None }
    }
}

// Implement the `Deref` trait
impl std::ops::Deref for NoName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Provide a default string when `name` is `None`.
        // This avoids panicking when `UNREACHABLE` is called.
        "UNREACHABLE"
    }
}

impl From<&str> for NoName {
    fn from(_s: &str) -> Self {
        NoName { name: None }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TraceFlag {
    kTrace = 1,
    kNoTrace = 0,
}

//#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
    ExnRef,
    AnyRef,
    NullRef,
    EqRef,
    StructRef,
    ArrayRef,
    I8,
    I16,
}

//#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HeapType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
    ExnRef,
    AnyRef,
    NullRef,
    EqRef,
    StructRef,
    ArrayRef,
    I8,
    I16,
}

pub struct FunctionSig {}

pub struct WasmModule {}

pub struct Decoder<'a> {
    start_: *const u8,
    pc_: *const u8,
    end_: *const u8,
    buffer_offset_: u32,
    error_: WasmError,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Decoder<'a> {
    pub fn new(start: *const u8, end: *const u8, buffer_offset: u32) -> Decoder<'a> {
        assert!(start <= end);
        Decoder {
            start_: start,
            pc_: start,
            end_: end,
            buffer_offset_: buffer_offset,
            error_: WasmError {
                offset: 0,
                message: String::new(),
            },
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_vec(bytes: &'a Vec<u8>, buffer_offset: u32) -> Decoder<'a> {
        let start = bytes.as_ptr();
        let end = unsafe { start.add(bytes.len()) };
        Decoder::new(start, end, buffer_offset)
    }

    pub fn read_u8<ValidationTag>(&self, pc: *const u8, msg: &str) -> u8 {
        self.read_little_endian::<u8, ValidationTag>(pc, msg)
    }

    pub fn read_u16<ValidationTag>(&self, pc: *const u8, msg: &str) -> u16 {
        self.read_little_endian::<u16, ValidationTag>(pc, msg)
    }

    pub fn read_u32<ValidationTag>(&self, pc: *const u8, msg: &str) -> u32 {
        self.read_little_endian::<u32, ValidationTag>(pc, msg)
    }

    pub fn read_u64<ValidationTag>(&self, pc: *const u8, msg: &str) -> u64 {
        self.read_little_endian::<u64, ValidationTag>(pc, msg)
    }

    pub fn read_u32v<ValidationTag>(&self, pc: *const u8, name: &str) -> (u32, u32) {
        self.read_leb::<u32, ValidationTag, TraceFlag::kNoTrace>(pc, name)
    }

    pub fn read_i32v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i32, u32) {
        self.read_leb::<i32, ValidationTag, TraceFlag::kNoTrace>(pc, name)
    }

    pub fn read_u64v<ValidationTag>(&self, pc: *const u8, name: &str) -> (u64, u32) {
        self.read_leb::<u64, ValidationTag, TraceFlag::kNoTrace>(pc, name)
    }

    pub fn read_i64v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i64, u32) {
        self.read_leb::<i64, ValidationTag, TraceFlag::kNoTrace>(pc, name)
    }

    pub fn read_i33v<ValidationTag>(&self, pc: *const u8, name: &str) -> (i64, u32) {
        self.read_leb::<i64, ValidationTag, TraceFlag::kNoTrace, 33>(pc, name)
    }

    pub fn read_prefixed_opcode<ValidationTag>(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (WasmOpcode, u32) {
        let (index, index_length) =
            self.read_u32v::<ValidationTag>(unsafe { pc.add(1) }, "prefixed opcode index");
        let length = index_length + 1;

        if ValidationTag::validate && index > 0xfff {
            self.errorf(pc, "Invalid prefixed opcode {}", index);
            return (WasmOpcode::kExprUnreachable, 0);
        }

        if index > 0xff {
            let opcode = unsafe { (*pc as u32) << 12 | index };
            (unsafe { std::mem::transmute(opcode) }, length)
        } else {
            let opcode = unsafe { (*pc as u32) << 8 | index };
            (unsafe { std::mem::transmute(opcode) }, length)
        }
    }

    pub fn consume_u8(&mut self, name: &str) -> u8 {
        self.consume_little_endian::<u8, TraceFlag::kTrace>(name)
    }

    pub fn consume_u8_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u8 {
        if tracer != ptr::null_mut() as *mut dyn ITracer {
            let pc = self.pc_;
            unsafe {
                tracer.bytes(pc, mem::size_of::<u8>() as u32);
                tracer.description_str(name);
            }
        }
        self.consume_little_endian::<u8, TraceFlag::kNoTrace>(name)
    }

    pub fn consume_u16(&mut self, name: &str) -> u16 {
        self.consume_little_endian::<u16, TraceFlag::kTrace>(name)
    }

    pub fn consume_u32_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u32 {
        if tracer != ptr::null_mut() as *mut dyn ITracer {
            let pc = self.pc_;
            unsafe {
                tracer.bytes(pc, mem::size_of::<u32>() as u32);
                tracer.description_str(name);
            }
        }
        self.consume_little_endian::<u32, TraceFlag::kNoTrace>(name)
    }

    pub fn consume_u32v(&mut self, name: &str) -> u32 {
        let (result, length) =
            self.read_leb::<u32, FullValidationTag, TraceFlag::kTrace>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_u32v_tracer(&mut self, name: &str, tracer: &mut dyn ITracer) -> u32 {
        let (result, length) =
            self.read_leb::<u32, FullValidationTag, TraceFlag::kNoTrace>(self.pc_, name);
        if tracer != ptr::null_mut() as *mut dyn ITracer {
            unsafe {
                tracer.bytes(self.pc_, length);
                tracer.description_str(name);
            }
        }
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_i32v(&mut self, name: &str) -> i32 {
        let (result, length) =
            self.read_leb::<i32, FullValidationTag, TraceFlag::kTrace>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_u64v(&mut self, name: &str, tracer: &mut dyn ITracer) -> u64 {
        let (result, length) =
            self.read_leb::<u64, FullValidationTag, TraceFlag::kNoTrace>(self.pc_, name);
        if tracer != ptr::null_mut() as *mut dyn ITracer {
            unsafe {
                tracer.bytes(self.pc_, length);
                tracer.description_str(name);
            }
        }
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_i64v(&mut self, name: &str) -> i64 {
        let (result, length) =
            self.read_leb::<i64, FullValidationTag, TraceFlag::kTrace>(self.pc_, name);
        unsafe {
            self.pc_ = self.pc_.add(length as usize);
        }
        result
    }

    pub fn consume_bytes(&mut self, size: u32, name: &str) {
        if name != "" {
            println!("  +{}  {:<20}: {} bytes", self.pc_offset(), name, size);
        }
        if self.check_available(size) {
            unsafe {
                self.pc_ = self.pc_.add(size as usize);
            }
        } else {
            self.pc_ = self.end_;
        }
    }

    pub fn consume_bytes_tracer(&mut self, size: u32, name: &str, tracer: &mut dyn ITracer) {
        if tracer != ptr::null_mut() as *mut dyn ITracer {
            unsafe {
                tracer.bytes(self.pc_, size);
                tracer.description_str(name);
            }
        }
        self.consume_bytes(size, "");
    }

    pub fn available_bytes(&self) -> u32 {
        let pc = self.pc_ as usize;
        let end = self.end_ as usize;
        if pc <= end {
            let result: usize = end - pc;
            return result.try_into().unwrap();
        } else {
            return 0;
        }
    }

    pub fn check_available(&self, size: u32) -> bool {
        if size > self.available_bytes() {
            self.errorf(self.pc_, "expected {} bytes, fell off end", size);
            return false;
        }
        true
    }

    pub fn error(&mut self, msg: &str) {
        self.errorf(self.pc_, "{}", msg);
    }

    pub fn error_pc(&mut self, pc: *const u8, msg: &str) {
        self.errorf(pc, "{}", msg);
    }

    pub fn error_offset(&mut self, offset: u32, msg: &str) {
        self.errorf_offset(offset, "{}", msg);
    }

    pub fn errorf(&mut self, format: *const u8, args: &str) {
        self.errorf(self.pc_, format, args);
    }

    pub fn errorf_pc(&mut self, pc: *const u8, format: *const u8, args: &str) {
        self.errorf(self.pc_offset(pc), "{}", args);
    }

    pub fn errorf_offset(&mut self, offset: u32, format: &str, args: &str) {
        self.verrorf(offset, format, args);
    }

    pub fn on_first_error(&mut self) {}

    pub fn trace_byte_range(&self, start: *const u8, end: *const u8) {
        assert!(start <= end);
        let mut current = start;
        while current < end {
            print!("{:02x} ", unsafe { *current });
            unsafe {
                current = current.add(1);
            }
        }
    }

    pub fn trace_off_end(&self) {
        self.trace_byte_range(self.pc_, self.end_);
        println!("<end>\n");
    }

    pub fn to_result<T, R>(&mut self, val: T) -> Result<R, WasmError>
    where
        T: Into<R>,
    {
        if self.failed() {
            println!("Result error: {}", self.error_.message());
            Err(self.error_.clone())
        } else {
            Ok(val.into())
        }
    }

    pub fn reset(&mut self, start: *const u8, end: *const u8, buffer_offset: u32) {
        assert!(start <= end);
        let diff: usize = unsafe { end.offset_from(start) }.try_into().unwrap();
        assert_eq!(diff as u32, end as u32 - start as u32);
        self.start_ = start;
        self.pc_ = start;
        self.end_ = end;
        self.buffer_offset_ = buffer_offset;
        self.error_ = WasmError {
            offset: 0,
            message: String::new(),
        };
    }

    pub fn reset_vec(&mut self, bytes: &'a Vec<u8>, buffer_offset: u32) {
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

    pub fn error_details(&self) -> &WasmError {
        &self.error_
    }

    pub fn start(&self) -> *const u8 {
        self.start_
    }

    pub fn pc(&self) -> *const u8 {
        self.pc_
    }

    pub fn position(&self) -> u32 {
        let pc = self.pc_ as usize;
        let start = self.start_ as usize;
        let result: usize = pc - start;
        return result.try_into().unwrap();
    }

    pub fn pc_offset(&self, pc: *const u8) -> u32 {
        assert!(self.start_ <= pc);
        let diff: usize = unsafe { pc.offset_from(self.start_) }.try_into().unwrap();
        assert!(u32::MAX - self.buffer_offset_ >= diff as u32);
        (diff as u32) + self.buffer_offset_
    }

    pub fn pc_offset_pc(&self) -> u32 {
        self.pc_offset(self.pc_)
    }

    pub fn buffer_offset(&self) -> u32 {
        self.buffer_offset_
    }

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

    pub fn lookahead(&self, offset: i32, expected: u8) -> bool {
        assert!(self.pc_ <= self.end_);
        let diff: usize = unsafe { self.end_.offset_from(self.pc_) }.try_into().unwrap();
        if diff > offset as usize && unsafe { *self.pc_.add(offset as usize) } == expected {
            return true;
        }
        false
    }

    fn verrorf(&mut self, offset: u32, format: &str, args: &str) {
        if !self.ok() {
            return;
        }
        let message = format!("{}", args);
        self.error_ = WasmError {
            offset: offset,
            message: message.clone(),
        };
        self.on_first_error();
    }

    fn read_little_endian<IntType, ValidationTag>(
        &self,
        pc: *const u8,
        msg: &str,
    ) -> IntType {
        if !ValidationTag::validate {
            assert!(self.start_ <= pc);
            assert!(pc <= self.end_);
            assert!(mem::size_of::<IntType>() <= unsafe {
                self.end_.offset_from(pc)
            } as usize);
        } else if unsafe { mem::size_of::<IntType>() as isize > self.end_.offset_from(pc) {
            self.error_pc(pc, msg);
            return 0 as IntType;
        }

        unsafe {
            let result = (0..mem::size_of::<IntType>())
                .map(|i| *pc.add(i) as IntType)
                .enumerate()
                .fold(0 as IntType, |acc, (i, byte)| acc | (byte << (i * 8)));
            return result;
        }
    }

    fn consume_little_endian<IntType, const TRACE: TraceFlag>(&mut self, name: &str) -> IntType {
        if TRACE as u8 == TraceFlag::kTrace as u8 {
            print!("  +{}  {:<20}: ", self.pc_offset_pc(), name);
        }
        if !self.check_available(mem::size_of::<IntType>() as u32) {
            self.trace_off_end();
            self.pc_ = self.end_;
            return 0 as IntType;
        }
        let val: IntType =
            self.read_little_endian::<IntType, NoValidationTag>(self.pc_, name);
        self.trace_byte_range(self.pc_, unsafe { self.pc_.add(mem::size_of::<IntType>()) });
        if TRACE as u8 == TraceFlag::kTrace as u8 {
            println!("= {}", val);
        }
        unsafe {
            self.pc_ = self.pc_.add(mem::size_of::<IntType>());
        }
        return val;
    }

    fn read_leb<
        IntType,
        ValidationTag,
        const TRACE: TraceFlag,
        const SIZE_IN_BITS: usize = 8 * mem::size_of::<IntType>(),
    >(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (IntType, u32) {
        if TRACE as u8 == TraceFlag::kTrace as u8 {
            print!("  +{}  {:<20}: ", self.pc_offset(pc), name);
        }
        if !ValidationTag::validate || unsafe { pc < self.end_ } && unsafe { !(*pc & 0x80 != 0) } {
            if TRACE as u8 == TraceFlag::kTrace as u8 {
                print!("{:02x} ", unsafe { *pc });
            }
            let result: IntType = unsafe { *pc as IntType };
            if std::mem::size_of::<IntType>() > 1 && std::is_signed::<IntType>() {
                let sign_ext_shift: i32 = (8 * std::mem::size_of::<IntType>() as i32) - 7;
                let result: IntType = result << sign_ext_shift >> sign_ext_shift;
                if TRACE as u8 == TraceFlag::kTrace as u8 {
                    println!("= {}", result);
                }
            } else {
                if TRACE as u8 == TraceFlag::kTrace as u8 {
                    println!("= {}", result);
                }
            }
            return (result, 1);
        }
        let (result, length) = self.read_leb_slowpath::<
            IntType,
            ValidationTag,
            TRACE,
            SIZE_IN_BITS,
        >(pc, name);
        return (result, length);
    }

    #[inline(never)]
    #[no_mangle]
    fn read_leb_slowpath<
        IntType,
        ValidationTag,
        const TRACE: TraceFlag,
        const SIZE_IN_BITS: usize = 8 * mem::size_of::<IntType>(),
    >(
        &self,
        pc: *const u8,
        name: &str,
    ) -> (IntType, u32) {
        return self.read_leb_tail::<IntType, ValidationTag, TRACE, SIZE_IN_BITS, 0>(
            pc, name, 0 as IntType,
        );
    }

    #[inline]
    fn read_leb_tail<
        IntType,
        ValidationTag,
        const TRACE: TraceFlag,
        const SIZE_IN_BITS: usize,
        const BYTE_INDEX: usize,
    >(
        &self,
        pc: *const u8,
        name: &str,
        intermediate_result: IntType,
    ) -> (IntType, u32) {
        const IS_SIGNED: bool = std::is_signed::<IntType>();
        const MAX_LENGTH: usize = (SIZE_IN_BITS + 6) / 7;
        const SHIFT: usize = BYTE_INDEX * 7;
        const IS_LAST_BYTE: bool = BYTE_INDEX == MAX_LENGTH - 1;

        let at_end: bool = ValidationTag::validate && unsafe { pc >= self.end_ };
        let mut b: u8 = 0;
        if !at_end {
            assert!(unsafe { pc < self.end_ });
            b = unsafe { *pc };
            if TRACE as u8 == TraceFlag::kTrace as u8 {
                print!("{:02x} ", b);
            }
            let signed_b = b as IntType;
            let unsigned_intermediate_result: IntType = intermediate_result
                | (signed_b & 0x7f) << SHIFT;
            //intermediate_result | (((b as i8) & 0x7f) << SHIFT) as IntType;
        }
        if !IS_LAST_BYTE && (b & 0x80 != 0) {
            let (next_intermediate_result, next_length) = self.read_leb_tail::<
                IntType,
                ValidationTag,
                TRACE,
                SIZE_IN_BITS,
                { BYTE_INDEX + 1 },
            >(unsafe { pc.add(1) }, name, intermediate_result);
            return (next_intermediate_result, next_length);
        }
        if ValidationTag::validate && (at_end || (b & 0x80 != 0)) {
            let error_message: String = if at_end {
                "<end> ".to_string()
            } else {
                "<length overflow> ".to_string()
            };
            self.errorf(pc, "{} while decoding {}", error_message, name);
            return (0 as IntType, 0);
        }
        if IS_LAST_BYTE {
            let extra_bits: usize = SIZE_IN_BITS - ((MAX_LENGTH - 1) * 7);
            let sign_ext_bits: usize = extra_bits - (if IS_SIGNED { 1 } else { 0 });
            let checked_bits: u8 = b & (0xff << sign_ext_bits) as u8;
            let sign_extended_extra_bits: u8 = 0x7f & (0xff << sign_ext_bits) as u8;
            let valid_extra_bits: bool = checked_bits == 0
                || (IS_SIGNED && checked_bits == sign_extended_extra_bits);
            if !ValidationTag::validate {
                assert!(valid_extra_bits);
            } else if !valid_extra_bits {
                self.error(pc, "extra bits in varint");
                return (0 as IntType, 0);
            }
        }
        let sign_ext_shift = if IS_SIGNED {
            std::cmp::max(0, (8 * std::mem::size_of::<IntType>() as i32) - SHIFT as i32 - 7)
                as usize
        } else {
            0
        };
        let intermediate_result: IntType = intermediate_result >> 0;
        if TRACE as u8 == TraceFlag::kTrace as u8 && IS_SIGNED {
            println!("= {}", intermediate_result);
        } else if TRACE as u8 == TraceFlag::kTrace as u8 {
            println!("= {}", intermediate_result);
        }
        let length: u32 = (BYTE_INDEX + 1) as u32;
        return (intermediate_result, length);
    }
}
