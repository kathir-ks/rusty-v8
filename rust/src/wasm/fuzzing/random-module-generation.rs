// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This whole compilation unit should only be included in non-official builds to
// reduce binary size.  We normally disable V8_WASM_RANDOM_FUZZERS in official builds.
// #[cfg(not(feature = "V8_WASM_RANDOM_FUZZERS"))]
// compile_error!("Exclude this compilation unit in official builds.");

use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{
    cmp::min,
    convert::TryInto,
    marker::PhantomData,
    mem,
    num::Wrapping,
    ops::{Deref, DerefMut, Range},
    ptr, slice,
};

const K_MAX_ARRAYS: usize = 3;
const K_MAX_STRUCTS: usize = 4;
const K_MAX_STRUCT_FIELDS: usize = 4;
const K_MAX_GLOBALS: usize = 64;
const K_MAX_LOCALS: u32 = 32;
const K_MAX_PARAMETERS: usize = 15;
const K_MAX_RETURNS: usize = 15;
const K_MAX_EXCEPTIONS: usize = 4;
const K_MAX_TABLES: usize = 4;
const K_MAX_MEMORIES: usize = 4;
const K_MAX_ARRAY_SIZE: usize = 20;
const K_MAX_PASSIVE_DATA_SEGMENTS: usize = 2;
const K_MAX_RECURSION_DEPTH: u32 = 64;
const K_MAX_CATCH_CASES: usize = 6;
const K_SIMD128_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    I32,
    I64,
    F32,
    F64,
    S128,
    FuncRef,
    ExternRef,
    AnyRef,
    EqRef,
    StructRef,
    ArrayRef,
    I31Ref,
    ExnRef,
    NullRef,
    NullExternRef,
    NullFuncRef,
    I8,
    I16,
    Void,
    Top, // For type inference / local lookup
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmOpcode {
    Nop,
    I32Const,
    I64Const,
    F32Const,
    F64Const,
    LocalGet,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Ior,
    I32Xor,
    I32Shl,
    I32ShrU,
    I32ShrS,
    I32Rotl,
    I32Rotr,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Ge,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Ior,
    I64Xor,
    I64Shl,
    I64ShrU,
    I64ShrS,
    I64Rotl,
    I64Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    I32ConvertI64,
    I32SConvertF32,
    I32UConvertF32,
    I32SConvertF64,
    I32UConvertF64,
    I32ReinterpretF32,
    I64ConvertI32,
    I64SConvertF32,
    I64UConvertF32,
    I64SConvertF64,
    I64UConvertF64,
    I64ReinterpretF64,
    F32ConvertI32,
    F32ConvertI64,
    F32DemoteF64,
    F32ReinterpretI32,
    F64ConvertI32,
    F64ConvertI64,
    F64PromoteF32,
    F64ReinterpretI64,
    Return,
    CallFunction,
    CallIndirect,
    CallRef,
    ReturnCall,
    ReturnCallIndirect,
    ReturnCallRef,
    Block,
    Loop,
    If,
    Else,
    End,
    Br,
    BrIf,
    BrTable,
    Select,
    SelectWithType,
    MemorySize,
    MemoryGrow,
    Drop,
    RefNull,
    RefIsNull,
    RefFunc,
    TableGet,
    TableSet,
    TableSize,
    TableGrow,
    TableFill,
    TableCopy,
    RefAsNonNull,
    Try,
    Catch,
    CatchAll,
    Delegate,
    Throw,
    Rethrow,
    TryTable,
    I31GetS,
    I31GetU,
    S128LoadMem,
    S128StoreMem,
    S128Const,
    I8x16Shuffle,
    S128Load8Splat,
    S128Load16Splat,
    S128Load32Splat,
    S128Load64Splat,
    S128Load32Zero,
    S128Load64Zero,
    S128Store8Lane,
    S128Store16Lane,
    S128Store32Lane,
    S128Store64Lane,
    I32SConvertSatF32,
    I32UConvertSatF32,
    I32SConvertSatF64,
    I32UConvertSatF64,
    I64SConvertSatF32,
    I64UConvertSatF32,
    I64SConvertSatF64,
    I64UConvertSatF64,
    RefCast,
    RefCastNull,
    BrOnCast,
    BrOnCastFail,
    BrOnNull,
    BrOnNonNull,
    AnyConvertExtern,
    StructNew,
    StructNewDefault,
    StructGet,
    StructGetS,
    StructGetU,
    StructSet,
    ArrayNew,
    ArrayNewFixed,
    ArrayNewData,
    ArrayNewElem,
    ArrayNewDefault,
    ArrayLen,
    ArrayGet,
    ArrayGetS,
    ArrayGetU,
    ArraySet,
    ArrayCopy,
    ArrayFill,
    ArrayInitData,
    ArrayInitElem,
    RefEq,
    RefTest,
}

#[derive(Clone, Copy)]
pub struct ValueType {
    kind: ValueKind,
    nullable: bool,
    shared: bool,
    heap_type_code: i32,
    index: Option<ModuleTypeIndex>, // Option<usize>
}

impl ValueType {
    pub fn primitive(kind: ValueKind) -> Self {
        ValueType {
            kind,
            nullable: false,
            shared: false,
            heap_type_code: -1,
            index: None,
        }
    }
    pub fn ref_maybe_null(index: ModuleTypeIndex, nullable: bool, shared: bool, ref_type_kind: RefTypeKind) -> Self {
      ValueType {
          kind: ValueKind::AnyRef, // Default to AnyRef, other Ref Types should be set appropriately.
          nullable,
          shared,
          heap_type_code: index.0 as i32,
          index: Some(index),
      }
    }
    pub fn ref_null(index: ModuleTypeIndex, shared: bool, ref_type_kind: RefTypeKind) -> Self {
      ValueType {
        kind: ValueKind::AnyRef, // Default to AnyRef, other Ref Types should be set appropriately.
        nullable: true,
        shared,
        heap_type_code: index.0 as i32,
        index: Some(index),
      }
    }

    pub fn is_reference(&self) -> bool {
        matches!(self.kind,
                 ValueKind::FuncRef | ValueKind::ExternRef | ValueKind::AnyRef | ValueKind::EqRef |
                 ValueKind::StructRef | ValueKind::ArrayRef | ValueKind::I31Ref | ValueKind::ExnRef |
                 ValueKind::NullRef | ValueKind::NullExternRef | ValueKind::NullFuncRef)
    }
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
    pub fn is_non_nullable(&self) -> bool {
        !self.nullable
    }
    pub fn kind(&self) -> ValueKind {
        self.kind
    }
    pub fn value_type_code(&self) -> i32 {
        match self.kind {
            ValueKind::I32 => -1,
            ValueKind::I64 => -2,
            ValueKind::F32 => -3,
            ValueKind::F64 => -4,
            _ => 0, // Placeholder for other types
        }
    }

    pub fn has_index(&self) -> bool {
      self.index.is_some()
    }
    pub fn ref_index(&self) -> ModuleTypeIndex {
      self.index.unwrap()
    }

    pub fn as_nullable(&self) -> Self {
      ValueType {
          kind: self.kind,
          nullable: true,
          shared: self.shared,
          heap_type_code: self.heap_type_code,
          index: self.index,
      }
    }
    pub fn heap_type(&self) -> HeapType {
      match self.kind {
          ValueKind::FuncRef => HeapType::Func,
          ValueKind::ExternRef => HeapType::Extern,
          ValueKind::AnyRef => HeapType::Any,
          ValueKind::EqRef => HeapType::Eq,
          ValueKind::StructRef => HeapType::Struct,
          ValueKind::ArrayRef => HeapType::Array,
          ValueKind::I31Ref => HeapType::I31,
          ValueKind::ExnRef => HeapType::Exn,
          ValueKind::NullRef => HeapType::None,
          ValueKind::NullExternRef => HeapType::NoExtern,
          ValueKind::NullFuncRef => HeapType::NoFunc,
          _ => HeapType::Any
      }
    }
    pub fn unpacked(&self) -> Self {
      ValueType::primitive(self.kind)
    }
    pub fn is_defaultable(&self) -> bool {
      self.is_nullable() ||
          matches!(self.kind, ValueKind::I32 | ValueKind::I64 | ValueKind::F32 | ValueKind::F64)
    }
    pub fn is_packed(&self) -> bool {
      matches!(self.kind, ValueKind::I8 | ValueKind::I16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleTypeIndex(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nullability {
    Nullable,
    NonNullable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefTypeKind {
    kFunction,
    kStruct,
    kArray,
    kOther,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatchKind {
    Catch,
    CatchRef,
    CatchAllRef,
    CatchAllRef,
}

const K_LAST_CATCH_KIND: CatchKind = CatchKind::CatchAllRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericKind {
    Any,
    Eq,
    Struct,
    Array,
    I31,
    None,
    Extern,
    NoExtern,
    Exn,
    NoExn,
    Func,
    NoFunc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapType {
    Any,
    Eq,
    Struct,
    Array,
    I31,
    None,
    Extern,
    Exn,
    Func,
    NoExtern,
    NoExn,
    NoFunc,
    Index(ModuleTypeIndex, bool, RefTypeKind), //ModuleTypeIndex is type index
    Generic(GenericKind, bool) // Generic types
}

impl HeapType {
  pub fn code(&self) -> i32 {
    match self {
        HeapType::Any => K_ANY_REF_CODE,
        HeapType::Eq => K_EQ_REF_CODE,
        HeapType::Struct => K_STRUCT_REF_CODE,
        HeapType::Array => K_ARRAY_REF_CODE,
        HeapType::I31 => K_I31_REF_CODE,
        HeapType::None => K_NONE_CODE,
        HeapType::Extern => K_EXTERN_REF_CODE,
        HeapType::Exn => K_EXN_REF_CODE,
        HeapType::Func => K_FUNC_REF_CODE,
        HeapType::NoExtern => K_NO_EXTERN_REF_CODE,
        HeapType::NoExn => K_NO_EXN_REF_CODE,
        HeapType::NoFunc => K_NO_FUNC_REF_CODE,
        HeapType::Index(index, _, _) => index.0 as i32,
        HeapType::Generic(kind, _) => match kind {
            GenericKind::Any => K_ANY_REF_CODE,
            GenericKind::Eq => K_EQ_REF_CODE,
            GenericKind::Struct => K_STRUCT_REF_CODE,
            GenericKind::Array => K_ARRAY_REF_CODE,
            GenericKind::I31 => K_I31_REF_CODE,
            GenericKind::None => K_NONE_CODE,
            GenericKind::Extern => K_EXTERN_REF_CODE,
            GenericKind::Exn => K_EXN_REF_CODE,
            GenericKind::Func => K_FUNC_REF_CODE,
            GenericKind::NoExtern => K_NO_EXTERN_REF_CODE,
            GenericKind::NoExn => K_NO_EXN_REF_CODE,
            GenericKind::NoFunc => K_NO_FUNC_REF_CODE,
        }
    }
  }
  pub fn representation(&self) -> HeapType {
    match self {
        HeapType::Any => HeapType::Any,
        HeapType::Eq => HeapType::Eq,
        HeapType::Struct => HeapType::Struct,
        HeapType::Array => HeapType::Array,
        HeapType::I31 => HeapType::I31,
        HeapType::None => HeapType::None,
        HeapType::Extern => HeapType::Extern,
        HeapType::Exn => HeapType::Exn,
        HeapType::Func => HeapType::Func,
        HeapType::NoExtern => HeapType::NoExtern,
        HeapType::NoExn => HeapType::NoExn,
        HeapType::NoFunc => HeapType::NoFunc,
        HeapType::Index(_, _, _) => HeapType::Any,
        HeapType::Generic(kind, _) => match kind {
            GenericKind::Any => HeapType::Any,
            GenericKind::Eq => HeapType::Eq,
            GenericKind::Struct => HeapType::Struct,
            GenericKind::Array => HeapType::Array,
            GenericKind::I31 => HeapType::I31,
            GenericKind::None => HeapType::None,
            GenericKind::Extern => HeapType::Extern,
            GenericKind::Exn => HeapType::Exn,
            GenericKind::Func => HeapType::Func,
            GenericKind::NoExtern => HeapType::NoExtern,
            GenericKind::NoExn => HeapType::NoExn,
            GenericKind::NoFunc => HeapType::NoFunc,
        }
    }
  }
  pub fn ref_index(&self) -> ModuleTypeIndex {
    match self {
        HeapType::Index(index, _, _) => *index,
        _ => ModuleTypeIndex(0),
    }
  }
  pub fn ref_type_kind(&self) -> RefTypeKind {
    match self {
        HeapType::Index(_, _, ref_type_kind) => *ref_type_kind,
        _ => RefTypeKind::kOther,
    }
  }
  pub fn is_index(&self) -> bool {
    matches!(self, HeapType::Index(_,_,_))
  }
}

const K_ANY_REF_CODE: i32 = -16;
const K_EQ_REF_CODE: i32 = -17;
const K_STRUCT_REF_CODE: i32 = -18;
const K_ARRAY_REF_CODE: i32 = -19;
const K_I31_REF_CODE: i32 = -20;
const K_NONE_CODE: i32 = -21;
const K_EXTERN_REF_CODE: i32 = -22;
const K_EXN_REF_CODE: i32 = -23;
const K_FUNC_REF_CODE: i32 = -24;
const K_NO_EXTERN_REF_CODE: i32 = -30;
const K_NO_EXN_REF_CODE: i32 = -31;
const K_NO_FUNC_REF_CODE: i32 = -32;

const K_WASM_VOID: i32 = -64;

//use crate::wasm::module::FunctionSig;

// Mock FunctionSig
#[derive(Debug, Clone)]
pub struct FunctionSig {
    params: Vec<ValueType>,
    returns: Vec<ValueType>,
}

impl FunctionSig {
    pub fn new(params: Vec<ValueType>, returns: Vec<ValueType>) -> Self {
        FunctionSig { params, returns }
    }
    pub fn parameters(&self) -> &[ValueType] {
      &self.params
    }
    pub fn returns(&self) -> &[ValueType] {
      &self.returns
    }

    pub fn parameter_count(&self) -> usize {
        self.params.len()
    }

    pub fn return_count(&self) -> usize {
        self.returns.len()
    }
    pub fn get_param(&self, index: usize) -> ValueType {
        self.params[index]
    }

    pub fn get_return(&self, index: usize) -> ValueType {
        self.returns[index]
    }
}

// Mock StructType
#[derive(Debug, Clone)]
pub struct StructType {
    fields: Vec<ValueType>,
    mutable_fields: Vec<bool>,
}

impl StructType {
    pub fn new(fields: Vec<ValueType>, mutable_fields: Vec<bool>) -> Self {
        StructType { fields, mutable_fields }
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    pub fn field(&self, index: usize) -> ValueType {
        self.fields[index]
    }
    pub fn fields(&self) -> &Vec<ValueType> {
      &self.fields
    }
    pub fn mutability(&self, index: usize) -> bool {
        self.mutable_fields[index]
    }
}

// Mock ArrayType
#[derive(Debug, Clone)]
pub struct ArrayType {
    element_type: ValueType,
    mutability: bool,
}

impl ArrayType {
    pub fn new(element_type: ValueType, mutability: bool) -> Self {
        ArrayType { element_type, mutability }
    }
    pub fn element_type(&self) -> &ValueType {
        &self.element_type
    }
    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

// Mock WasmModuleBuilder
pub struct WasmModuleBuilder {
    types: Vec<FunctionSig>,
    structs: Vec<StructType>,
    arrays: Vec<ArrayType>,
    functions: Vec<Function>,
    data_segments: Vec<Vec<u8>>,
    element_segments: Vec<WasmElemSegment>,
    memories: Vec<bool>, // isMemory64
    tables: Vec<ValueType>, //Table type
    imported_functions: usize,
    tags: Vec<FunctionSig>
}

impl WasmModuleBuilder {
    pub fn new() -> Self {
        WasmModuleBuilder {
            types: Vec::new(),
            structs: Vec::new(),
            arrays: Vec::new(),
            functions: Vec::new(),
            data_segments: Vec::new(),
            element_segments: Vec::new(),
            memories: Vec::new(),
            tables: Vec::new(),
            imported_functions: 0,
            tags: Vec::new()
        }
    }
    pub fn add_signature(&mut self, sig: FunctionSig) -> ModuleTypeIndex {
      self.types.push(sig);
      ModuleTypeIndex((self.types.len() - 1) as u32)
    }
    pub fn num_types(&self) -> u32 {
      self.types.len() as u32
    }
    pub fn is_signature(&self, index: ModuleTypeIndex) -> bool {
      (index.0 as usize) < self.types.len()
    }

    pub fn add_struct_type(&mut self, struct_type: StructType) -> ModuleTypeIndex {
      self.structs.push(struct_type);
      ModuleTypeIndex((self.structs.len() - 1) as u32)
    }
    pub fn num_structs(&self) -> u32 {
      self.structs.len() as u32
    }
    pub fn is_struct_type(&self, index: ModuleTypeIndex) -> bool {
      (index.0 as usize) < self.structs.len()
    }
    pub fn get_struct_type(&self, index: ModuleTypeIndex) -> &StructType {
      &self.structs[index.0 as usize]
    }

    pub fn add_array_type(&mut self, array_type: ArrayType) -> ModuleTypeIndex {
      self.arrays.push(array_type);
      ModuleTypeIndex((self.arrays.len() - 1) as u32)
    }
    pub fn num_arrays(&self) -> u32 {
      self.arrays.len() as u32
    }
    pub fn is_array_type(&self, index: ModuleTypeIndex) -> bool {
      (index.0 as usize) < self.arrays.len()
    }
    pub fn get_array_type(&self, index: ModuleTypeIndex) -> &ArrayType {
      &self.arrays[index.0 as usize]
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
    pub fn num_declared_functions(&self) -> usize {
      self.functions.len()
    }
    pub fn get_function(&self, index: usize) -> &Function {
      &self.functions[index]
    }

    pub fn add_passive_data_segment(&mut self, data: &[u8]) {
        self.data_segments.push(data.to_vec());
    }
    pub fn num_data_segments(&self) -> usize {
        self.data_segments.len()
    }

    pub fn add_element_segment(&mut self, segment: WasmElemSegment) -> u32 {
      self.element_segments.push(segment);
      (self.element_segments.len() - 1) as u32
    }
    pub fn num_element_segments(&self) -> usize {
      self.element_segments.len()
    }

    pub fn add_memory(&mut self, is_memory64: bool) {
        self.memories.push(is_memory64);
    }
    pub fn num_memories(&self) -> usize {
        self.memories.len()
    }
    pub fn is_memory64(&self, index: usize) -> bool {
      self.memories[index]
    }

    pub fn add_table(&mut self, table_type: ValueType) {
      self.tables.push(table_type);
    }
    pub fn num_tables(&self) -> usize {
      self.tables.len()
    }
    pub fn is_table64(&self, index: usize) -> bool {
      false //Table64 not supported
    }
    pub fn get_table_type(&self, index: usize) -> ValueType {
      self.tables[index]
    }

    pub fn num_imported_functions(&self) -> usize {
      self.imported_functions
    }

    pub fn add_tag(&mut self, sig: FunctionSig) {
      self.tags.push(sig);
    }
    pub fn num_tags(&self) -> usize {
      self.tags.len()
    }
    pub fn get_tag_type(&self, index: usize) -> &FunctionSig {
      &self.tags[index]
    }

    pub fn get_super_type(&self, index: u32) -> Option<ModuleTypeIndex> {
      None
    }
}

// Mock WasmFunctionBuilder
pub struct WasmFunctionBuilder<'a> {
    module_builder: &'a mut WasmModuleBuilder,
    locals: Vec<ValueType>,
    signature: FunctionSig,
    body: Vec<u8>,
}

impl<'a> WasmFunctionBuilder<'a> {
    pub fn new(module_builder: &'a mut WasmModuleBuilder, signature: FunctionSig) -> Self {
        WasmFunctionBuilder {
            module_builder,
            locals: Vec::new(),
            signature,
            body: Vec::new(),
        }
    }

    pub fn add_local(&mut self, local_type: ValueType) -> u32 {
        self.locals.push(local_type);
        (self.locals.len() - 1) as u32
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.body.push(byte);
    }
    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.body.extend_from_slice(bytes);
    }

    pub fn signature(&self) -> &FunctionSig {
      &self.signature
    }
    pub fn body(&self) -> &[u8] {
      &self.body
    }

    pub fn builder(&mut self) -> &mut WasmModuleBuilder {
      self.module_builder
    }
}

// Mock Function
#[derive(Debug, Clone)]
pub struct Function {
    signature: FunctionSig,
    body: Vec<u8>,
}

impl Function {
    pub fn new(signature: FunctionSig, body: Vec<u8>) -> Self {
        Function { signature, body }
    }
    pub fn signature(&self) -> &FunctionSig {
      &self.signature
    }
    pub fn body(&self) -> &[u8] {
      &self.body
    }
}

#[derive(Debug, Clone)]
pub struct WasmElemSegment {
  element_type: ValueType,
  is_declarative: bool,
  init_expr: WasmInitExpr,
  entries: Vec<WasmElemSegmentEntry>
}

impl WasmElemSegment {
  pub fn new(element_type: ValueType, is_declarative: bool, init_expr: WasmInitExpr) -> Self {
    WasmElemSegment {
      element_type,
      is_declarative,
      init_expr,
      entries: Vec::new()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmElemSegmentEntryKind {
    kRefNullEntry
}

#[derive(Debug, Clone)]
pub struct WasmElemSegmentEntry {
  kind: WasmElemSegmentEntryKind,
  func_index: u32
}

impl WasmElemSegmentEntry {
    pub fn new(kind: WasmElemSegmentEntryKind, func_index: u32) -> Self {
      WasmElemSegmentEntry {
        kind,
        func_index
      }
    }
}

#[derive(Debug, Clone)]
pub enum WasmInitExpr {
  RefNullConst(HeapType)
}

#[derive(Default, Debug, Clone)]
pub struct WasmModuleGenerationOptions {
    generate_simd: bool,
    generate_wasm_gc: bool,
}

impl WasmModuleGenerationOptions {
    pub fn new() -> Self {
        WasmModuleGenerationOptions {
            generate_simd: false,
            generate_wasm_gc: false,
        }
    }

    pub fn generate_simd(&self) -> bool {
        self.generate_simd
    }

    pub fn generate_wasm_gc(&self) -> bool {
        self.generate_wasm_gc
    }

    pub fn to_integral(&self) -> u8 {
        (if self.generate_simd { 1 } else { 0 }) | (if self.generate_wasm_gc { 2 } else { 0 })
    }
}

// Mock flag struct for v8 flags
#[derive(Debug, Default, Clone)]
pub struct V8Flags {
  pub wasm_max_table_size: Wrapping<i32>,
  pub max_wasm_functions: Wrapping<i32>,
  pub experimental_wasm_exnref: bool
}

// Implements global singleton-like access to flags, like v8_flags in C++.
lazy_static::lazy_static! {
    pub static ref V8_FLAGS: V8Flags = {
        let mut flags = V8Flags::default();
        flags.wasm_max_table_size = Wrapping(32);
        flags.max_wasm_functions = Wrapping(4);
        flags.experimental_wasm_exnref = false;
        flags
    };
}

fn max_table_size() -> i32 {
    min(V8_FLAGS.wasm_max_table_size.0, 32)
}

fn max_num_of_functions() -> i32 {
    min(V8_FLAGS.max_wasm_functions.0, 4)
}

#[derive(Debug, Clone)]
struct StringImports {
    cast: u32,
    test: u32,
    from_char_code: u32,
    from_code_point: u32,
    char_code_at: u32,
    code_point_at: u32,
    length: u32,
    concat: u32,
    substring: u32,
    equals: u32,
    compare: u32,
    from_char_code_array: u32,
    into_char_code_array: u32,
    measure_string_as_utf8: u32,
    encode_string_into_utf8array: u32,
    encode_string_to_utf8array: u32,
    decode_string_from_utf8array: u32,
    array_i16: ModuleTypeIndex,
    array_i8: ModuleTypeIndex,
}

struct DataRange<'a> {
    data: &'a [u8],
    rng: SmallRng,
}

impl<'a> DataRange<'a> {
    fn new(data: &'a [u8], seed: Option<i64>) -> Self {
        let rng = match seed {
            Some(s) => SmallRng::seed_from_u64(s as u64),
            None => SmallRng::from_entropy(),
        };
        DataRange { data, rng }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn split(&mut self) -> DataRange<'a> {
        let random_choice: u16 = if self.data