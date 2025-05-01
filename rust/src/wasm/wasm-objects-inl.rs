#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// use std::mem;
// use std::ptr;
// use std::sync::atomic::{AtomicU32, Ordering};
// use std::sync::Arc;
// use std::ops::{Deref, DerefMut};

// Assuming these are defined elsewhere, similar to the C++ includes
// mod base;
// mod common;
// mod heap;
// mod objects;
// mod roots;
// mod wasm;
// mod third_party;

// use base::*;
// use common::*;
// use heap::*;
// use objects::*;
// use roots::*;
// use wasm::*;
// use third_party::*;

// Replicated macros and constants.  These need proper definitions based
// on the surrounding code.
const kObjectAlignmentBits: usize = 3;
const kTaggedSize: usize = 8;
const kHeaderSize: usize = 8; // Example value
const kIntSize: usize = 4;   // Example value
const kWasmTrustedInstanceDataIndirectPointerTag: usize = 1; //Example value
const kWasmDispatchTableIndirectPointerTag: usize = 1; //Example value
const kWasmStackMemoryTag: usize = 1; //Example value
const wasm_kMaxValueTypeSize: usize = 8;  // Example maximum value type size
const wasm_kV8MaxWasmStructFields: usize = 32;  // Example max struct fields

macro_rules! FIELD_ADDR {
    ($base:expr, $offset:expr) => {
        ($base as usize + $offset) as *mut u8
    };
}

macro_rules! OffsetOf {
    ($index:expr) => {
        kHeaderSize + $index * kTaggedSize // Example calculation
    };
}

macro_rules! SBXCHECK {
    ($condition:expr) => {
        if !$condition {
            println!("SBXCHECK failed!");
        }
        assert!($condition);
    };
}

// Placeholder traits/enums/structs.  These need to be defined based on
// the surrounding code.
// trait Object {}
// trait HeapObject: Object {}

// struct Smi { value: i32 }
// impl Smi {
//     fn FromInt(i: i32) -> Self { Smi { value: i } }
//     fn ToInt(&self) -> i32 { self.value }
// }

// struct HeapNumber { value: f64 }
// impl HeapNumber {
//     fn value(&self) -> f64 { self.value }
// }

// struct BigInt {}
// impl BigInt {
//   fn FromInt64(_isolate: &Isolate, value: i64) -> Self {
//         BigInt {}
//     }
//     fn AsUint64(&self) -> u64 {0}
// }

// struct Isolate {}
// impl Isolate {
//     fn Current() -> &'static Isolate {
//         static ISOLATE: Isolate = Isolate {};
//         &ISOLATE
//     }
//     fn factory(&self) -> Factory {
//         Factory {}
//     }
// }

// struct Factory {}
// impl Factory {
//     fn NewNumberFromInt(&self, value: i32) -> Number {
//         Number {}
//     }
//     fn NewNumber(&self, value: f64) -> Number {
//         Number {}
//     }
//     fn undefined_value(&self) -> Undefined {
//         Undefined {}
//     }
// }

// struct Undefined {}
// struct Number {}
// struct Context {}
// struct Tuple2 {}
// struct FixedArray {}
// struct FixedInt32Array {}
// struct FixedAddressArray {}
// struct FixedUInt32Array {}
// struct WeakArrayList {}
// struct JSArrayBuffer {}
// struct ObjectSlot(*mut Object);

// macro_rules! DEF_GETTER {
//   ($holder:ident, $name:ident, $type:ty) => {
//     impl $holder {
//       fn $name(&self) -> $type {
//         todo!()
//       }
//     }
//   };
// }

// macro_rules! ACCESSORS {
//   ($holder:ident, $name:ident, $type:ty, $offset:ident) => {
//     impl $holder {
//       fn $name(&self) -> $type {
//         todo!()
//       }
//       fn set_$name(&mut self, value: $type) {
//         todo!()
//       }
//     }
//   };
// }

// macro_rules! TRUSTED_POINTER_ACCESSORS {
//   ($holder:ident, $name:ident, $type:ty, $offset:ident, $tag:ident) => {
//     impl $holder {
//       fn $name(&self) -> $type {
//         todo!()
//       }
//       fn set_$name(&mut self, value: $type) {
//         todo!()
//       }
//     }
//   };
// }

// macro_rules! PRIMITIVE_ACCESSORS {
//   ($holder:ident, $name:ident, $type:ty, $offset:ident) => {
//     impl $holder {
//       fn $name(&self) -> $type {
//         todo!()
//       }
//       fn set_$name(&mut self, value: $type) {
//         todo!()
//       }
//     }
//   };
// }

// macro_rules! CODE_POINTER_ACCESSORS {
//     ($holder:ident, $name:ident, $offset:ident) => {
//       impl $holder {
//         fn $name(&self) -> WasmCodePointer {
//             todo!()
//         }
//         fn set_$name(&mut self, value: WasmCodePointer) {
//             todo!()
//         }
//       }
//     };
//   }

//   macro_rules! PROTECTED_POINTER_ACCESSORS {
//     ($holder:ident, $name:ident, $type:ty, $offset:ident) => {
//       impl $holder {
//         fn $name(&self) -> $type {
//           todo!()
//         }
//       }
//     };
//   }

//   macro_rules! EXTERNAL_POINTER_ACCESSORS {
//     ($holder:ident, $name:ident, $type:ty, $offset:ident, $tag:ident) => {
//       impl $holder {
//         fn $name(&self) -> $type {
//           todo!()
//         }
//       }
//     };
//   }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmCodePointer {
    value: u32,
}

impl WasmCodePointer {
    pub fn value(&self) -> u32 {
        self.value
    }
}

mod wasm {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ValueKind {
        I8,
        I16,
        I32,
        I64,
        F16,
        F32,
        F64,
        S128,
        Ref,
        RefNull,
        Void,
        Top,
        Bottom,
    }

    impl ValueKind {
        pub fn name(&self) -> &'static str {
            match self {
                ValueKind::I8 => "i8",
                ValueKind::I16 => "i16",
                ValueKind::I32 => "i32",
                ValueKind::I64 => "i64",
                ValueKind::F16 => "f16",
                ValueKind::F32 => "f32",
                ValueKind::F64 => "f64",
                ValueKind::S128 => "s128",
                ValueKind::Ref => "ref",
                ValueKind::RefNull => "refnull",
                ValueKind::Void => "void",
                ValueKind::Top => "top",
                ValueKind::Bottom => "bottom",
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ValueType {
        raw_bits: u8, // Using u8 for simplicity, adjust as needed
    }

    impl ValueType {
        pub fn FromRawBitField(raw_bits: i32) -> Self {
            ValueType {
                raw_bits: raw_bits as u8,
            }
        }

        pub fn raw_bit_field(&self) -> i32 {
            self.raw_bits as i32
        }

        pub fn kind(&self) -> ValueKind {
            match self.raw_bits {
                1 => ValueKind::I8,    // Example mapping
                2 => ValueKind::I16,   // Example mapping
                3 => ValueKind::I32,   // Example mapping
                4 => ValueKind::I64,   // Example mapping
                5 => ValueKind::F32,   // Example mapping
                6 => ValueKind::F64,   // Example mapping
                7 => ValueKind::Ref,   // Example mapping
                8 => ValueKind::RefNull, // Example mapping
                _ => ValueKind::Void,   // Default mapping
            }
        }

        pub fn is_valid(&self) -> bool {
          true
        }

        pub fn is_reference(&self) -> bool {
          self.kind() == ValueKind::Ref || self.kind() == ValueKind::RefNull
        }

        pub fn is_object_reference(&self) -> bool {
          self.is_reference()
        }

        pub fn value_kind_size(&self) -> i32 {
          match self.kind() {
            ValueKind::I32 => 4,
            ValueKind::I64 => 8,
            ValueKind::F32 => 4,
            ValueKind::F64 => 8,
            _ => 4
          }
        }

        pub fn has_index(&self) -> bool {
            false
        }

        pub fn ref_index(&self) -> CanonicalTypeIndex {
            CanonicalTypeIndex { index: 0 }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CanonicalValueType {
        raw_bits: u32,
    }

    impl CanonicalValueType {
        pub fn FromRawBitField(raw_bits: u32) -> Self {
            CanonicalValueType { raw_bits }
        }

        pub fn IsFunctionType(&self) -> bool {
            true
        }

        pub fn ref_index(&self) -> CanonicalTypeIndex {
            CanonicalTypeIndex { index: 0 }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CanonicalTypeIndex {
        index: u32,
    }

    pub struct NativeModule {
      module_: WasmModule
    }

    impl NativeModule {
      pub fn module(&self) -> &WasmModule {
        &self.module_
      }
    }

    pub struct WasmModule {
      pub num_imported_functions: i32,
    }

    impl WasmModule {
        pub fn has_type(&self, index: CanonicalTypeIndex) -> bool {
            true
        }

        pub fn canonical_type(&self, unsafe_: ValueType) -> CanonicalValueType {
            CanonicalValueType { raw_bits: 0 }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AddressType {
        kI32,
        kI64,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Suspend {
        kNoSuspend,
        kSuspend,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Promise {
        kNoPromise,
        kPromise,
    }

    pub const kInvalidWasmCodePointer: WasmCodePointer = WasmCodePointer { value: 0 };
}

struct AsmWasmData {}
impl AsmWasmData {}
struct WasmArray {}
impl WasmArray {}
struct WasmCapiFunctionData {}
impl WasmCapiFunctionData {}
struct WasmContinuationObject {}
impl WasmContinuationObject {}
struct WasmExceptionTag {}
impl WasmExceptionTag {}
struct WasmExportedFunctionData {}
impl WasmExportedFunctionData {}
struct WasmFunctionData {}
impl WasmFunctionData {}
struct WasmFuncRef {}
impl WasmFuncRef {}
struct WasmGlobalObject {}
impl WasmGlobalObject {}
struct WasmImportData {}
impl WasmImportData {}
struct WasmInstanceObject {}
impl WasmInstanceObject {}
struct WasmInternalFunction {}
impl WasmInternalFunction {}
struct WasmJSFunctionData {}
impl WasmJSFunctionData {}
struct WasmMemoryObject {}
impl WasmMemoryMapDescriptor {}
impl WasmModuleObject {}
struct WasmNull {}
impl WasmObject {}
impl WasmResumeData {}
struct WasmStruct {}
impl WasmStruct {}
struct WasmSuspenderObject {}
impl WasmSuspendingObject {}
struct WasmTableObject {}
impl WasmTagObject {}
struct WasmTypeInfo {}
impl WasmTypeInfo {}
struct WasmDispatchTable {}
impl WasmTrustedInstanceData {}
struct WasmExceptionPackage {}
impl WasmExceptionPackage {}
struct WasmExportedFunction {}
impl WasmExportedFunction {}
struct WasmJSFunction {}
impl WasmCapiFunction {}
struct WasmExternalFunction {}

impl WasmModuleObject {
    fn native_module(&self) -> *mut wasm::NativeModule {
        todo!()
    }
    fn shared_native_module(&self) -> SharedNativeModule {
      todo!()
    }
    fn module(&self) -> &wasm::WasmModule {
        unsafe { (*self.native_module()).module() }
    }
    fn is_asm_js(&self) -> bool {
      todo!()
    }

}

impl WasmMemoryObject {
    // ACCESSORS(WasmMemoryObject, instances, Tagged<WeakArrayList>, kInstancesOffset)
    fn instances(&self) -> WeakArrayList {
      todo!()
    }
    fn set_instances(&mut self, value: WeakArrayList) {
      todo!()
    }
}

impl WasmGlobalObject {
    // ACCESSORS(WasmGlobalObject, untagged_buffer, Tagged<JSArrayBuffer>, kUntaggedBufferOffset)
    fn untagged_buffer(&self) -> JSArrayBuffer {
      todo!()
    }
    fn set_untagged_buffer(&mut self, value: JSArrayBuffer) {
      todo!()
    }

    // ACCESSORS(WasmGlobalObject, tagged_buffer, Tagged<FixedArray>, kTaggedBufferOffset)
    fn tagged_buffer(&self) -> FixedArray {
      todo!()
    }
    fn set_tagged_buffer(&mut self, value: FixedArray) {
      todo!()
    }

    // TRUSTED_POINTER_ACCESSORS(WasmGlobalObject, trusted_data, WasmTrustedInstanceData, kTrustedDataOffset, kWasmTrustedInstanceDataIndirectPointerTag)
    fn trusted_data(&self) -> WasmTrustedInstanceData {
      todo!()
    }
    fn set_trusted_data(&mut self, value: WasmTrustedInstanceData) {
      todo!()
    }

    fn type_(&self) -> wasm::ValueType {
        // Various consumers of ValueKind (e.g. ValueKind::name()) use the raw enum
        // value as index into a global array. As such, if the index is corrupted
        // (which must be assumed, as it comes from within the sandbox), this can
        // lead to out-of-bounds reads outside the sandbox. While these are not
        // technically sandbox violations, we should still try to avoid them to keep
        // fuzzers happy. This SBXCHECK accomplishes that.
        let type_ = wasm::ValueType::FromRawBitField(self.raw_type());
        SBXCHECK!(type_.is_valid());
        return type_;
    }

    fn set_type(&mut self, value: wasm::ValueType) {
        self.set_raw_type(value.raw_bit_field());
    }

    fn type_size(&self) -> i32 {
        self.type_().value_kind_size()
    }

    fn address(&self) -> *mut u8 {
        assert!(!self.type_().is_reference());
        assert!(self.offset() + self.type_size() <= self.untagged_buffer().byte_length());
        self.untagged_buffer().backing_store() + self.offset() as usize
    }

    fn GetI32(&self) -> i32 {
        unsafe { (self.address() as *const i32).read_unaligned() }
    }

    fn GetI64(&self) -> i64 {
        unsafe { (self.address() as *const i64).read_unaligned() }
    }

    fn GetF32(&self) -> f32 {
        unsafe { (self.address() as *const f32).read_unaligned() }
    }

    fn GetF64(&self) -> f64 {
        unsafe { (self.address() as *const f64).read_unaligned() }
    }

    fn GetS128RawBytes(&self) -> *mut u8 {
        self.address()
    }

    fn GetRef(&self) -> Object {
        assert!(self.type_().is_reference());
        self.tagged_buffer().get(self.offset())
    }

    fn SetI32(&mut self, value: i32) {
        unsafe { (self.address() as *mut i32).write_unaligned(value) }
    }

    fn SetI64(&mut self, value: i64) {
        unsafe { (self.address() as *mut i64).write_unaligned(value) }
    }

    fn SetF32(&mut self, value: f32) {
        unsafe { (self.address() as *mut f32).write_unaligned(value) }
    }

    fn SetF64(&mut self, value: f64) {
        unsafe { (self.address() as *mut f64).write_unaligned(value) }
    }

    fn SetRef(&mut self, value: Object) {
        assert!(self.type_().is_object_reference());
        self.tagged_buffer().set(self.offset(), value);
    }

    // Fields
    fn raw_type(&self) -> i32 {
        todo!()
    }
    fn set_raw_type(&mut self, value: i32) {
      todo!()
    }
    fn offset(&self) -> i32 {
      todo!()
    }
}

impl WasmTrustedInstanceData {
  fn clear_padding(&mut self) {
    // constexpr int kPaddingBytes = FIELD_SIZE(kOptionalPaddingOffset);
    // static_assert(kPaddingBytes == 0 || kPaddingBytes == kIntSize);
    // if constexpr (kPaddingBytes != 0) {
    //   WriteField<int>(kOptionalPaddingOffset, 0);
    // }
  }

  fn memory_object(&self, memory_index: i32) -> WasmMemoryObject {
    todo!()
  }

  fn memory_base(&self, memory_index: i32) -> *mut u8 {
    todo!()
  }

  fn memory_size(&self, memory_index: i32) -> usize {
    todo!()
  }

  fn dispatch_table(&self, table_index: u32) -> WasmDispatchTable {
    todo!()
  }

  fn has_dispatch_table(&self, table_index: u32) -> bool {
    todo!()
  }

  fn native_module(&self) -> *mut wasm::NativeModule {
    todo!()
  }

  fn module_object(&self) -> WasmModuleObject {
    todo!()
  }

  fn module(&self) -> &wasm::WasmModule {
    unsafe { (*self.native_module()).module() }
  }
}

impl WasmInstanceObject {
  fn trusted_data(&self) -> WasmTrustedInstanceData {
    todo!()
  }
  fn set_trusted_data(&mut self, value: WasmTrustedInstanceData) {
    todo!()
  }
  fn module(&self) -> &wasm::WasmModule {
    self.module_object().module()
  }

  fn module_object(&self) -> WasmModuleObject {
    todo!()
  }
}

struct SharedNativeModule {}

struct Object {}

struct DirectHandle<T> {
  value: T,
}

impl<T> DirectHandle<T> {
  fn value(&self) -> &T {
    &self.value
  }
}

fn direct_handle<T>(value: T, _isolate: &Isolate) -> DirectHandle<T> {
  DirectHandle { value }
}

struct ImportedFunctionEntry {
    instance_data_: DirectHandle<WasmTrustedInstanceData>,
    index_: i32,
}

impl ImportedFunctionEntry {
    fn new(instance_data: DirectHandle<WasmTrustedInstanceData>, index: i32) -> Self {
        assert!(index >= 0);
        // assert!(index < instance_data.module().num_imported_functions);
        ImportedFunctionEntry {
            instance_data_: instance_data,
            index_: index,
        }
    }
}

impl WasmDispatchTable {
  fn protected_offheap_data(&self) -> TrustedManaged<WasmDispatchTableData> {
    todo!()
  }

  fn offheap_data(&self) -> *mut WasmDispatchTableData {
    todo!()
  }

  fn protected_uses(&self) -> ProtectedWeakFixedArray {
    todo!()
  }

  fn table_type(&self) -> wasm::CanonicalValueType {
    todo!()
  }
  fn set_table_type(&mut self, value: wasm::CanonicalValueType) {
    todo!()
  }

  fn clear_entry_padding(&mut self, _index: i32) {
    // static_assert(kEntryPaddingBytes == 0 || kEntryPaddingBytes == kIntSize);
    // if constexpr (kEntryPaddingBytes != 0) {
    //   WriteField<int>(OffsetOf(index) + kEntryPaddingOffset, 0);
    // }
  }

  fn length_acquire(&self) -> i32 {
    todo!()
  }
  fn length(&self) -> i32 {
    todo!()
  }

  fn capacity(&self) -> i32 {
    todo!()
  }

  fn implicit_arg(&self, _index: i32) -> Object {
    todo!()
  }

  fn target(&self, _index: i32) -> WasmCodePointer {
    todo!()
  }

  fn sig(&self, _index: i32) -> wasm::CanonicalTypeIndex {
    todo!()
  }

  fn function_index(&self, _index: i32) -> u32 {
    todo!()
  }

}

struct TrustedManaged<T> {}

struct WasmDispatchTableData {}

struct ProtectedWeakFixedArray {}

impl WasmImportData {
  fn protected_instance_data(&self) -> WasmTrustedInstanceData {
    todo!()
  }

  fn protected_call_origin(&self) -> TrustedObject {
    todo!()
  }

  fn suspend(&self) -> wasm::Suspend {
    todo!()
  }
  fn set_suspend(&mut self, value: wasm::Suspend) {
    todo!()
  }

  fn table_slot(&self) -> u32 {
    todo!()
  }
  fn set_table_slot(&mut self, value: u32) {
    todo!()
  }

  fn bit_field(&self) -> i32 {
    todo!()
  }

  fn set_bit_field(&mut self, value: i32) {
    todo!()
  }
}

struct TrustedObject {}

impl WasmInternalFunction {
  fn protected_implicit_arg(&self) -> TrustedObject {
    todo!()
  }

  fn call_target(&self) -> WasmCodePointer {
    WasmCodePointer { value: self.raw_call_target() }
  }
  fn set_call_target(&mut self, code_pointer: WasmCodePointer) {
    self.set_raw_call_target(code_pointer.value());
  }

  fn raw_call_target(&self) -> u32 {
    todo!()
  }
  fn set_raw_call_target(&mut self, value: u32) {
    todo!()
  }
}

impl WasmFunctionData {
  fn wrapper_code(&self) -> WasmCodePointer {
    todo!()
  }
  fn set_wrapper_code(&mut self, value: WasmCodePointer) {
    todo!()
  }

  fn protected_internal(&self) -> WasmInternalFunction {
    todo!()
  }
}

impl WasmExportedFunctionData {
  fn protected_instance_data(&self) -> WasmTrustedInstanceData {
    todo!()
  }

  fn c_wrapper_code(&self) -> WasmCodePointer {
    todo!()
  }
  fn set_c_wrapper_code(&mut self, value: WasmCodePointer) {
    todo!()
  }

  fn sig(&self) -> *const wasm::CanonicalSig {
    todo!()
  }

  fn sig_index(&self) -> wasm::CanonicalTypeIndex {
    wasm::CanonicalTypeIndex {
      index: self.canonical_type_index() as u32,
    }
  }

  fn is_promising(&self) -> bool {
    todo!()
  }

  fn js_promise_flags(&self) -> i32 {
    todo!()
  }

  fn canonical_type_index(&self) -> i32 {
    todo!()
  }
}

impl WasmJSFunctionData {
  fn sig_index(&self) -> wasm::CanonicalTypeIndex {
    wasm::CanonicalTypeIndex {
      index: self.canonical_sig_index() as u32,
    }
  }

  fn protected_offheap_data(&self) -> TrustedManaged<WasmJSFunctionDataOffheapData> {
    todo!()
  }

  fn offheap_data(&self) -> *mut WasmJSFunctionDataOffheapData {
    todo!()
  }

  fn canonical_sig_index(&self) -> i32 {
    todo!()
  }
}

struct WasmJSFunctionDataOffheapData {}

impl WasmCapiFunctionData {
  fn sig_index(&self) -> wasm::CanonicalTypeIndex {
    wasm::CanonicalTypeIndex {
      index: self.canonical_sig_index() as u32,
    }
  }

  fn canonical_sig_index(&self) -> i32 {
    todo!()
  }
}

impl WasmExternalFunction {
  fn func_ref(&self) -> WasmFuncRef {
    self.shared().wasm_function_data().func_ref()
  }
}

impl WasmTypeInfo {
  fn type_(&self) -> wasm::CanonicalValueType {
    wasm::CanonicalValueType::FromRawBitField(self.canonical_type())
  }

  fn type_index(&self) -> wasm::CanonicalTypeIndex {
    self.type_().ref_index()
  }

  fn element_type(&self) -> wasm::CanonicalValueType {
    wasm::CanonicalValueType::FromRawBitField(self.canonical_element_type())
  }

  fn canonical_type(&self) -> u32 {
    todo!()
  }

  fn canonical_element_type(&self) -> u32 {
    todo!()
  }
}

impl WasmTableObject {
    fn trusted_data(&self) -> WasmTrustedInstanceData {
      todo!()
    }
    fn set_trusted_data(&mut self, value: WasmTrustedInstanceData) {
      todo!()
    }

    fn trusted_dispatch_table(&self) -> WasmDispatchTable {
      todo!()
    }
    fn set_trusted_dispatch_table(&mut self, value: WasmDispatchTable) {
      todo!()
    }

    fn type_(&self, module: &wasm::WasmModule) -> wasm::ValueType {
        let type_ = self.unsafe_type();
        SBXCHECK!(!type_.has_index() || module.has_type(type_.ref_index()));
        return type_;
    }

    fn canonical_type(&self, module: &wasm::WasmModule) -> wasm::CanonicalValueType {
        let unsafe_ = self.unsafe_type();
        if !unsafe_.has_index() {
            return wasm::CanonicalValueType { raw_bits: unsafe_.raw_bit_field() as u32 };
        }
        SBXCHECK!(module.has_type(unsafe_.ref_index()));
        return module.canonical_type(unsafe_);
    }

    fn unsafe_type(&self) -> wasm::ValueType {
        // Various consumers of ValueKind (e.g. ValueKind::name()) use the raw enum
        // value as index into a global array. As such, if the index is corrupted
        // (which must be assumed, as it comes from within the sandbox), this can
        // lead to out-of-bounds reads outside the sandbox. While these are not
        // technically sandbox violations, we should still try to avoid them to keep
        // fuzzers happy. This SBXCHECK accomplishes that.
        let type_ = wasm::ValueType::FromRawBitField(self.raw_type());
        SBXCHECK!(type_.is_valid());
        return type_;
    }

    fn is_in_bounds(&self, entry_index: u32) -> bool {
        entry_index < self.current_length() as u32
    }

    fn is_table64(&self) -> bool {
        self.address_type() == wasm::AddressType::kI64
    }

    fn maximum_length_u64(&self) -> Option<u64> {
      todo!()
    }

    // Accessors

    fn raw_type(&self) -> i32 {
      todo!()
    }

    fn current_length(&self) -> i32 {
      todo!()
    }

    fn address_type(&self) -> wasm::AddressType {
      todo!()
    }

    fn maximum_length(&self) -> Object {
      todo!()
    }
}

impl WasmMemoryObject {
  fn has_maximum_pages(&self) -> bool {
    self.maximum_pages() >= 0
  }

  fn is_memory64(&self) -> bool {
    self.address_type() == wasm::AddressType::kI64
  }

  fn maximum_pages(&self) -> i32 {
    todo!()
  }

  fn address_type(&self) -> wasm::AddressType {
    todo!()
  }
}

impl WasmObject {
    // static
    fn ReadValueAt(_isolate: &Isolate, obj: DirectHandle<impl Object>, type_: wasm::CanonicalValueType, offset: u32) -> DirectHandle<Object> {
      todo!()
    }
}

// Conversions from Numeric objects.
impl WasmObject {
    // static
    fn FromNumber<ElementType>(value: Object) -> ElementType {
        todo!()
    }
}

impl WasmStruct {
  // static
  fn EncodeInstanceSizeInMap(_instance_size: i32, _map: Map) {}

  // static
  fn DecodeInstanceSizeFromMap(_map: Map) -> i32 {
    0
  }

  fn GcSafeSize(_map: Map) -> i32 {
    WasmStruct::DecodeInstanceSizeFromMap(_map)
  }

  fn RawFieldAddress(&self, _raw_offset: i32) -> *mut u8 {
    todo!()
  }

  fn RawField(&self, _raw_offset: i32) -> ObjectSlot {
    todo!()
  }
}

const WASM_ARRAY_TYPE: i32 = 1;

impl WasmArray {
  fn type_index(map: Map) -> wasm::CanonicalTypeIndex {
    assert_eq!(WASM_ARRAY_TYPE, map.instance_type());
    let type_info = map.wasm_type_info();
    type_info.type_().ref_index()
  }

  const fn GcSafeElementType(_map: Map) -> wasm::CanonicalValueType {
    wasm::CanonicalValueType { raw_bits: 0 }
  }

  fn SizeFor(_map: Map, _length: i32) -> i32 {
    0
  }

  fn element_offset(&self, _index: u32) -> u32 {
    0
  }

  fn ElementAddress(&self, _index: u32) -> *mut u8 {
    todo!()
  }

  fn ElementSlot(&self, _index: u32) -> ObjectSlot {
    todo!()
  }

  // static
  fn GetElement(_isolate: &Isolate, _array: DirectHandle<WasmArray>, _index: u32) -> DirectHandle<Object> {
    todo!()
  }

  // static
  fn EncodeElementSizeInMap(_element_size: i32, _map: Map) {}

  // static
  fn DecodeElementSizeFromMap(_map: Map) -> i32 {
    0
  }

  fn map(&self) -> Map {
    todo!()
  }

  fn length(&self) -> i32 {
    todo!()
  }

  fn ptr(&self) -> u64 {
    todo!()
  }
}

impl WasmTagObject {
  fn trusted_data(&self) -> WasmTrustedInstanceData {
    todo!()
  }
  fn set_trusted_data(&mut self, value: W