#![allow(non_camel_case_types)]
#![allow(dead_code)]
// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NOTE: This is a partial conversion and may contain placeholders.

use std::cmp;
use std::convert::TryInto;
use std::mem;
use std::ptr;

// use crate::base::bounds::Bounded; // Placeholder for base::bounds
// use crate::base::small_vector::SmallVector; // Placeholder for base::small-vector
// use crate::base::strings::string_view; // Placeholder for base::strings
// use crate::base::vector::Vector; // Placeholder for base::vector
// use crate::strings::unicode; // Placeholder for src/strings/unicode
// use crate::utils::bit_vector::BitVector; // Placeholder for src/utils/bit-vector
// use crate::wasm::decoder::Decoder; // Placeholder for src/wasm/decoder
// use crate::wasm::function_body_decoder::FunctionBodyDecoder; // Placeholder for src/wasm/function-body-decoder
// use crate::wasm::value_type::ValueType; // Placeholder for src/wasm/value-type
// use crate::wasm::wasm_features::WasmFeatures; // Placeholder for src/wasm/wasm-features
// use crate::wasm::wasm_limits::kWasmMaxTypes; // Placeholder for src/wasm/wasm-limits
// use crate::wasm::wasm_module::WasmModule; // Placeholder for src/wasm/wasm-module
// use crate::wasm::wasm_opcodes::WasmOpcode; // Placeholder for src/wasm/wasm-opcodes
// use crate::wasm::wasm_subtyping::IsSubtypeOf; // Placeholder for src/wasm/wasm-subtyping

//Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TrapReason {
    Unreachable,
    // Add more trap reasons as needed
}

macro_rules! trace {
    ($($arg:tt)*) => {
        if v8_flags::trace_wasm_decoder {
            println!($($arg)*);
        }
    };
}

macro_rules! validate {
    ($condition:expr) => {
        if ValidationTag::validate {
            if !$condition {
                // Placeholder for ValidateAssumeTrue or similar logic
                eprintln!("Validation failed: {}", stringify!($condition));
                false // Or some sentinel value
            } else {
                true
            }
        } else {
            true
        }
    };
}

macro_rules! check_prototype_opcode {
    ($feat:ident) => {
        debug_assert_eq!(module_.origin, kWasmOrigin);
        if !validate!(enabled_.has_$feat()) {
            decode_error!(
                "Invalid opcode 0x{:02x} (enable with --experimental-wasm-{})",
                opcode,
                stringify!($feat)
            );
            return 0;
        }
        detected_.add_$feat();
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LoadType {
    kI32Load,
    kI64Load,
    kF32Load,
    kF64Load,
    kI32Load8S,
    kI32Load8U,
    kI32Load16S,
    kI32Load16U,
    kI64Load8S,
    kI64Load8U,
    kI64Load16S,
    kI64Load16U,
    kI64Load32S,
    kI64Load32U,
}

pub fn get_load_type(opcode: WasmOpcode) -> LoadType {
    // Placeholder
    match opcode {
        WasmOpcode::I32LoadMem => LoadType::kI32Load,
        WasmOpcode::I64LoadMem => LoadType::kI64Load,
        WasmOpcode::F32LoadMem => LoadType::kF32Load,
        WasmOpcode::F64LoadMem => LoadType::kF64Load,
        WasmOpcode::I32Load8S => LoadType::kI32Load8S,
        WasmOpcode::I32Load8U => LoadType::kI32Load8U,
        WasmOpcode::I32Load16S => LoadType::kI32Load16S,
        WasmOpcode::I32Load16U => LoadType::kI32Load16U,
        WasmOpcode::I64Load8S => LoadType::kI64Load8S,
        WasmOpcode::I64Load8U => LoadType::kI64Load8U,
        WasmOpcode::I64Load16S => LoadType::kI64Load16S,
        WasmOpcode::I64Load16U => LoadType::kI64Load16U,
        WasmOpcode::I64Load32S => LoadType::kI64Load32S,
        WasmOpcode::I64Load32U => LoadType::kI64Load32U,
        _ => panic!("Unexpected opcode"),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StoreType {
    kI32Store,
    kI64Store,
    kF32Store,
    kF64Store,
    kI32Store8,
    kI32Store16,
    kI64Store8,
    kI64Store16,
    kI64Store32,
}

pub fn get_store_type(opcode: WasmOpcode) -> StoreType {
    // Placeholder
    match opcode {
        WasmOpcode::I32StoreMem => StoreType::kI32Store,
        WasmOpcode::I64StoreMem => StoreType::kI64Store,
        WasmOpcode::F32StoreMem => StoreType::kF32Store,
        WasmOpcode::F64StoreMem => StoreType::kF64Store,
        WasmOpcode::I32Store8 => StoreType::kI32Store8,
        WasmOpcode::I32Store16 => StoreType::kI32Store16,
        WasmOpcode::I64Store8 => StoreType::kI64Store8,
        WasmOpcode::I64Store16 => StoreType::kI64Store16,
        WasmOpcode::I64Store32 => StoreType::kI64Store32,
        _ => panic!("Unexpected opcode"),
    }
}

macro_rules! atomic_op_list {
    ($V:ident) => {
        $V!(AtomicNotify, u32);
        $V!(I32AtomicWait, u32);
        $V!(I64AtomicWait, u64);
        $V!(I32AtomicLoad, u32);
        $V!(I64AtomicLoad, u64);
        $V!(I32AtomicLoad8U, u8);
        $V!(I32AtomicLoad16U, u16);
        $V!(I64AtomicLoad8U, u8);
        $V!(I64AtomicLoad16U, u16);
        $V!(I64AtomicLoad32U, u32);
        $V!(I32AtomicAdd, u32);
        $V!(I32AtomicAdd8U, u8);
        $V!(I32AtomicAdd16U, u16);
        $V!(I64AtomicAdd, u64);
        $V!(I64AtomicAdd8U, u8);
        $V!(I64AtomicAdd16U, u16);
        $V!(I64AtomicAdd32U, u32);
        $V!(I32AtomicSub, u32);
        $V!(I64AtomicSub, u64);
        $V!(I32AtomicSub8U, u8);
        $V!(I32AtomicSub16U, u16);
        $V!(I64AtomicSub8U, u8);
        $V!(I64AtomicSub16U, u16);
        $V!(I64AtomicSub32U, u32);
        $V!(I32AtomicAnd, u32);
        $V!(I64AtomicAnd, u64);
        $V!(I32AtomicAnd8U, u8);
        $V!(I32AtomicAnd16U, u16);
        $V!(I64AtomicAnd8U, u8);
        $V!(I64AtomicAnd16U, u16);
        $V!(I64AtomicAnd32U, u32);
        $V!(I32AtomicOr, u32);
        $V!(I64AtomicOr, u64);
        $V!(I32AtomicOr8U, u8);
        $V!(I32AtomicOr16U, u16);
        $V!(I64AtomicOr8U, u8);
        $V!(I64AtomicOr16U, u16);
        $V!(I64AtomicOr32U, u32);
        $V!(I32AtomicXor, u32);
        $V!(I64AtomicXor, u64);
        $V!(I32AtomicXor8U, u8);
        $V!(I32AtomicXor16U, u16);
        $V!(I64AtomicXor8U, u8);
        $V!(I64AtomicXor16U, u16);
        $V!(I64AtomicXor32U, u32);
        $V!(I32AtomicExchange, u32);
        $V!(I64AtomicExchange, u64);
        $V!(I32AtomicExchange8U, u8);
        $V!(I32AtomicExchange16U, u16);
        $V!(I64AtomicExchange8U, u8);
        $V!(I64AtomicExchange16U, u16);
        $V!(I64AtomicExchange32U, u32);
        $V!(I32AtomicCompareExchange, u32);
        $V!(I64AtomicCompareExchange, u64);
        $V!(I32AtomicCompareExchange8U, u8);
        $V!(I32AtomicCompareExchange16U, u16);
        $V!(I64AtomicCompareExchange8U, u8);
        $V!(I64AtomicCompareExchange16U, u16);
        $V!(I64AtomicCompareExchange32U, u32);
    };
}

macro_rules! atomic_store_op_list {
    ($V:ident) => {
        $V!(I32AtomicStore, u32);
        $V!(I64AtomicStore, u64);
        $V!(I32AtomicStore8U, u8);
        $V!(I32AtomicStore16U, u16);
        $V!(I64AtomicStore8U, u8);
        $V!(I64AtomicStore16U, u16);
        $V!(I64AtomicStore32U, u32);
    };
}

// Placeholder Decoder struct, needs adaptation based on C++ Decoder class
pub struct Decoder<'a> {
    start: *const u8,
    end: *const u8,
    pc: *const u8,
    buffer_offset: u32,
    error_message: Option<String>,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> Decoder<'a> {
    pub fn new(start: *const u8, end: *const u8, buffer_offset: u32) -> Self {
        Decoder {
            start,
            end,
            pc: start,
            buffer_offset,
            error_message: None,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn ok(&self) -> bool {
        self.error_message.is_none()
    }
    pub fn error(&mut self, pc: *const u8, str: &str) {
        self.error_message = Some(str.to_string());
        self.pc = self.end; // Ensure further reads fail
    }
    pub fn errorf(&mut self, pc: *const u8, str: &str, args: Vec<String>) {
        self.error_message = Some(format!("{} {:?}", str, args));
        self.pc = self.end; // Ensure further reads fail
    }

    fn read_u8<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> u8 {
        if T::validate {
            if pc >= self.end {
                self.error(pc, "Unexpected end of data");
                return 0;
            }
        }
        unsafe { *pc }
    }

    fn read_u32<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> u32 {
        if T::validate {
            if (self.end as usize).wrapping_sub(pc as usize) < 4 {
                self.error(pc, "Unexpected end of data");
                return 0;
            }
        }
        unsafe {
            (pc as *const u32).read_unaligned()
        }
    }

    fn read_u64<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> u64 {
        if T::validate {
            if (self.end as usize).wrapping_sub(pc as usize) < 8 {
                self.error(pc, "Unexpected end of data");
                return 0;
            }
        }
        unsafe {
            (pc as *const u64).read_unaligned()
        }
    }

    fn end(&self) -> *const u8 {
        self.end
    }

    fn more(&self) -> bool {
        self.pc < self.end
    }

    fn available_bytes(&self) -> usize {
        unsafe {
            self.end.offset_from(self.pc) as usize
        }
    }

    fn read_i32v<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> (i32, u32) {
        //Placeholder for LEB128 decoding
        (0, 0)
    }

    fn read_i64v<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> (i64, u32) {
        //Placeholder for LEB128 decoding
        (0, 0)
    }

    fn read_u32v<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> (u32, u32) {
        //Placeholder for LEB128 decoding
        (0, 0)
    }

    fn read_i33v<T: ValidationTag>(&mut self, pc: *const u8, name: &str) -> (i64, u32) {
        //Placeholder for LEB128 decoding
        (0, 0)
    }

    fn read_prefixed_opcode<T: ValidationTag>(&mut self, pc: *const u8) -> (WasmOpcode, u32) {
        //Placeholder for prefixed opcode decoding
        (WasmOpcode::Nop, 0)
    }

    fn checkAvailable(&self, table_count: u32) -> bool {
        true
    }
}

// Placeholder for WasmGlobal struct
pub struct WasmGlobal {}
// Placeholder for WasmTag struct
pub struct WasmTag {}
// Placeholder for WasmMemory struct
pub struct WasmMemory {
    is_memory64: bool,
}
// Placeholder for WasmTable struct
pub struct WasmTable {
    type_: ValueType
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HeapType {
    Bottom,
    Index(ModuleTypeIndex, bool, RefTypeKind),
    Any,
    Extern,
    Func,
    Exn,
    String,
    I31,
    Struct,
    Array,
    None,
    NoExtern,
    NoFunc,
    NoExn,
    StringViewWtf8,
    StringViewWtf16,
    StringViewIter,
    Cont,
    NoCont,
}

impl HeapType {
    fn from_code(code: u8, is_shared: bool) -> Self {
        match code {
            0x40 => HeapType::Any,
            0x6f => HeapType::Extern,
            0x70 => HeapType::Func,
            0x71 => HeapType::Exn,
            0x72 => HeapType::String,
            0x73 => HeapType::I31,
            0x74 => HeapType::Struct,
            0x75 => HeapType::Array,
            0x76 => HeapType::None,
            0x78 => HeapType::NoExtern,
            0x79 => HeapType::NoFunc,
            0x7a => HeapType::NoExn,
            0x7b => HeapType::StringViewWtf8,
            0x7c => HeapType::StringViewWtf16,
            0x7d => HeapType::StringViewIter,
            0x7e => HeapType::Cont,
            0x7f => HeapType::NoCont,
            _ => HeapType::Bottom
        }
    }

    fn is_index(&self) -> bool {
        match self {
            HeapType::Index(_, _, _) => true,
            _ => false,
        }
    }

    fn ref_index(&self) -> ModuleTypeIndex {
        match self {
            HeapType::Index(index, _, _) => *index,
            _ => ModuleTypeIndex {index: 0}
        }
    }

    fn is_bottom(&self) -> bool {
        self == &HeapType::Bottom
    }

    fn is_string_view(&self) -> bool {
        match self {
            HeapType::StringViewWtf8 | HeapType::StringViewWtf16 | HeapType::StringViewIter => true,
            _ => false,
        }
    }

    fn populate(&mut self, is_shared: bool, kind: RefTypeKind) {
        // Placeholder - implement population logic based on module info
        if let HeapType::Index(index, _, _) = self {
            *self = HeapType::Index(*index, is_shared, kind);
        }
    }

    fn has_index(&self) -> bool {
        match self {
            HeapType::Index(_, _, _) => true,
            _ => false
        }
    }

    fn name(&self) -> String {
        match self {
            HeapType::Bottom => "bottom".to_string(),
            HeapType::Index(_, _, _) => "index".to_string(), // Replace with actual index string
            HeapType::Any => "any".to_string(),
            HeapType::Extern => "extern".to_string(),
            HeapType::Func => "func".to_string(),
            HeapType::Exn => "exn".to_string(),
            HeapType::String => "string".to_string(),
            HeapType::I31 => "i31".to_string(),
            HeapType::Struct => "struct".to_string(),
            HeapType::Array => "array".to_string(),
            HeapType::None => "none".to_string(),
            HeapType::NoExtern => "noextern".to_string(),
            HeapType::NoFunc => "nofunc".to_string(),
            HeapType::NoExn => "noexn".to_string(),
            HeapType::StringViewWtf8 => "stringviewwtf8".to_string(),
            HeapType::StringViewWtf16 => "stringviewwtf16".to_string(),
            HeapType::StringViewIter => "stringviewiter".to_string(),
            HeapType::Cont => "cont".to_string(),
            HeapType::NoCont => "nocont".to_string(),
        }
    }

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RefTypeKind {
    kOther,
    kFunction,
    kStruct,
    kArray
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModuleTypeIndex {
    index: u32,
}

impl ModuleTypeIndex {
    const fn Invalid() -> Self {
        ModuleTypeIndex{index:u32::MAX}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueTypeCode {
    kI32Code = 0x7f,
    kI64Code = 0x7e,
    kF32Code = 0x7d,
    kF64Code = 0x7c,
    kRefCode = 0x6b,
    kRefNullCode = 0x68,
    kEqRefCode = 0x67,
    kI31RefCode = 0x66,
    kStructRefCode = 0x65,
    kArrayRefCode = 0x64,
    kAnyRefCode = 0x69,
    kNoneCode = 0x6a,
    kNoExternCode = 0x77,
    kNoFuncCode = 0x76,
    kExternRefCode = 0x6f,
    kFuncRefCode = 0x70,
    kNoExnCode = 0x7a,
    kExnRefCode = 0x71,
    kStringRefCode = 0x72,
    kStringViewWtf8Code = 0x7b,
    kStringViewWtf16Code = 0x7c,
    kStringViewIterCode = 0x7d,
    kNoContCode = 0x7e,
    kContRefCode = 0x7f,
    kS128Code = 0x7b,
    kVoidCode = 0x40,
    kI8Code = 0x78,
    kI16Code = 0x79,
    kF16Code = 0x7a,
    kSharedFlagCode = 0x10
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Nullability {
    kNullable,
    kNonNullable,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    Ref(HeapType),
    RefNull(HeapType),
    ExnRef,
    NullExnRef,
    S128,
    StringRef,
    ContRef,
    NullContRef,
    Void,
    Bottom
}

impl ValueType {
    pub const I32: Self = ValueType::I32;
    pub const I64: Self = ValueType::I64;
    pub const F32: Self = ValueType::F32;
    pub const F64: Self = ValueType::F64;
    pub const ExnRef: Self = ValueType::ExnRef;
    pub const NullExnRef: Self = ValueType::NullExnRef;
    pub const S128: Self = ValueType::S128;
    pub const StringRef: Self = ValueType::StringRef;
    pub const ContRef: Self = ValueType::ContRef;
    pub const NullContRef: Self = ValueType::NullContRef;
    pub const Void: Self = ValueType::Void;
    pub const Bottom: Self = ValueType::Bottom;
    pub fn RefMaybeNull(heap_type: HeapType, nullability: Nullability) -> Self {
        match nullability {
            Nullability::kNullable => ValueType::RefNull(heap_type),
            Nullability::kNonNullable => ValueType::Ref(heap_type),
        }
    }

    fn is_object_reference(&self) -> bool {
        match self {
            ValueType::Ref(_) | ValueType::RefNull(_) | ValueType::ExnRef | ValueType::NullExnRef | ValueType::StringRef | ValueType::ContRef | ValueType::NullContRef => true,
            _ => false,
        }
    }

    fn heap_type(&self) -> HeapType {
        match self {
            ValueType::Ref(heap_type) | ValueType::RefNull(heap_type) => *heap_type,
            _ => HeapType::Bottom,
        }
    }

    fn is_shared(&self) -> bool {
        // Placeholder - depends on how shared types are tracked
        true
    }

    fn has_index(&self) -> bool {
        match self {
            ValueType::Ref(heap_type) | ValueType::RefNull(heap_type) => heap_type.has_index(),
            _ => false,
        }
    }

    fn ref_index(&self) -> ModuleTypeIndex {
        match self {
            ValueType::Ref(heap_type) | ValueType::RefNull(heap_type) => heap_type.ref_index(),
            _ => ModuleTypeIndex {index: 0}
        }
    }

    fn populate(&mut self, is_shared: bool, kind: RefTypeKind) {
        // Placeholder - implement population logic based on module info
        match self {
            ValueType::Ref(heap_type) | ValueType::RefNull(heap_type) => {
                let mut mutable_heap_type = *heap_type;
                mutable_heap_type.populate(is_shared, kind);
                *self = match self {
                    ValueType::Ref(_) => ValueType::Ref(mutable_heap_type),
                    ValueType::RefNull(_) => ValueType::RefNull(mutable_heap_type),
                    _ => *self,
                };
            }
            _ => {}
        }
    }

    fn ref_type_kind(&self) -> RefTypeKind {
        match self {
            ValueType::Ref(heap_type) | ValueType::RefNull(heap_type) => {
                match heap_type {
                    HeapType::Index(_, _, kind) => *kind,
                    _ => RefTypeKind::kOther,
                }
            }
            _ => RefTypeKind::kOther,
        }
    }

    fn name(&self) -> String {
        match self {
            ValueType::I32 => "i32".to_string(),
            ValueType::I64 => "i64".to_string(),
            ValueType::F32 => "f32".to_string(),
            ValueType::F64 => "f64".to_string(),
            ValueType::Ref(heap_type) => format!("ref {}", heap_type.name()),
            ValueType::RefNull(heap_type) => format!("ref null {}", heap_type.name()),
            ValueType::ExnRef => "exnref".to_string(),
            ValueType::NullExnRef => "nullexnref".to_string(),
            ValueType::S128 => "s128".to_string(),
            ValueType::StringRef => "stringref".to_string(),
            ValueType::ContRef => "contref".to_string(),
            ValueType::NullContRef => "nullcontref".to_string(),
            ValueType::Void => "void".to_string(),
            ValueType::Bottom => "bottom".to_string(),
        }
    }
}

// Placeholder for WasmEnabledFeatures struct
#[derive(Debug, Default, Clone)]
pub struct WasmEnabledFeatures {
    simd: bool,
    threads: bool,
    exceptions: bool,
    bulk_memory: bool,
    mutable_global: bool,
    truncated_floats: bool,
    sat_float_to_int: bool,
    sign_extension: bool,
    multi_value: bool,
    reftypes: bool,
    tail_call: bool,
    gc: bool,
    stringref: bool,
    shared: bool,
    exnref: bool,
    wasmfx: bool,
}

impl WasmEnabledFeatures {
    pub fn has_simd(&self) -> bool {
        self.simd
    }

    pub fn has_threads(&self) -> bool {
        self.threads
    }
    pub fn has_exnref(&self) -> bool {
        self.exnref
    }

    pub fn has_shared(&self) -> bool {
        self.shared
    }

    pub fn has_stringref(&self) -> bool {
        self.stringref
    }

    pub fn has_wasmfx(&self) -> bool {
        self.wasmfx
    }

    pub fn all() -> Self {
        WasmEnabledFeatures {
            simd: true,
            threads: true,
            exceptions: true,
            bulk_memory: true,
            mutable_global: true,
            truncated_floats: true,
            sat_float_to_int: true,
            sign_extension: true,
            multi_value: true,
            reftypes: true,
            tail_call: true,
            gc: true,
            stringref: true,
            shared: true,
            exnref: true,
            wasmfx: true,
        }
    }
}

// Placeholder for WasmDetectedFeatures struct
pub struct WasmDetectedFeatures {
    multi_memory: bool,
    reftypes: bool,
    bulk_memory: bool,
}

impl WasmDetectedFeatures {
    fn add_multi_memory(&mut self) {
        self.multi_memory = true;
    }

    fn add_reftypes(&mut self) {
        self.reftypes = true;
    }

    fn add_bulk_memory(&mut self) {
        self.bulk_memory = true;
    }
}

// Placeholder for FunctionSig struct
#[derive(Debug, Copy, Clone)]
pub struct FunctionSig {
    parameter_count: u32,
    return_count: u32,
    all: *const ValueType,
}

impl FunctionSig {
    pub fn parameters(&self) -> &[ValueType] {
        unsafe { std::slice::from_raw_parts(self.all, self.parameter_count as usize) }
    }
    pub fn returns(&self) -> &[ValueType] {
        unsafe { std::slice::from_raw_parts(self.all.add(self.parameter_count as usize), self.return_count as usize) }
    }
    pub fn GetParam(&self, index: u32) -> ValueType {
        unsafe {*self.all.add(index as usize)}
    }
    pub fn GetReturn(&self, index: u32) -> ValueType {
        unsafe {*self.all.add(self.parameter_count as usize + index as usize)}
    }
    pub fn parameter_count(&self) -> i32 {
        self.parameter_count as i32
    }
    pub fn return_count(&self) -> i32 {
        self.return_count as i32
    }
    pub fn all(&self) -> SignatureAll {
        SignatureAll{begin: self.all}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SignatureAll {
    begin: *const ValueType
}

impl SignatureAll {
    pub fn begin(&self) -> *const ValueType {
        self.begin
    }
}

mod value_type_reader {
    use super::*;

    pub fn read_heap_type<T: ValidationTag>(
        decoder: &mut Decoder,
        pc: *const u8,
        enabled: WasmEnabledFeatures,
    ) -> (HeapType, u32) {
        let (heap_index, length) = decoder.read_i33v::<T>(pc, "heap type");
        if heap_index < 0 {
            let min_1_byte_leb128 = -64;
            if !validate!(heap_index >= min_1_byte_leb128) {
                decode_error!(decoder, pc, "Unknown heap type {}", heap_index);
                return (HeapType::Bottom, length);
            }
            let uint_7_mask = 0x7F;
            let code = (heap_index as u8 & uint_7_mask) as u8;
            let mut is_shared = false;
            if code == ValueTypeCode::kSharedFlagCode as u8 {
                if !validate!(enabled.has_shared()) {
                    decode_error!(
                        decoder,
                        pc,
                        "invalid heap type 0x{:02x}, enable with --experimental-wasm-shared",
                        ValueTypeCode::kSharedFlagCode as u8
                    );
                    return (HeapType::Bottom, length);
                }
                let code = decoder.read_u8::<T>(unsafe { pc.add(length as usize) }, "heap type");
                return (HeapType::Bottom, length);
                // length += 1;
                // is_shared = true;
            }
            match code {
                0x67 => (HeapType::Any, length),//ValueTypeCode::kEqRefCode as u8 => (HeapType::Any, length),
                0x66 => (HeapType::I31, length),//ValueTypeCode::kI31RefCode as u8 => (HeapType::I31, length),
                0x65 => (HeapType::Struct, length),//ValueTypeCode::kStructRefCode as u8 => (HeapType::Struct, length),
                0x64 => (HeapType::Array,