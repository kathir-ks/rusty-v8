#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

// use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

// use crate::base::bit_field::BitField;
// use crate::debug::interface_types::StackFrameId;
// use crate::heap::heap::Heap;
// use crate::objects::backing_store::BackingStore;
// use crate::objects::casting;
// use crate::objects::foreign::Foreign;
// use crate::objects::js_function::JSFunction;
// use crate::objects::js_objects::JSObject;
// use crate::objects::objects_body_descriptors::ObjectsBodyDescriptors;
// use crate::objects::objects::Object;
// use crate::objects::structs::Struct;
// use crate::objects::trusted_object::TrustedObject;
// use crate::wasm::module_instantiate;
// use crate::wasm::stacks::Stacks;
// use crate::wasm::struct_types::StructTypes;
// use crate::wasm::value_type::ValueType;
// use crate::wasm::wasm_code_manager::WasmCodeManager;
// use crate::wasm::wasm_module::WasmModule;

// placeholder definitions for types not defined in provided files
pub type Address = usize;
pub type WasmCodePointer = *const u8;
pub type ObjectSlot = *mut u8;
pub type MessageTemplate = u32;
pub type StackFrameId = u32;
pub type WriteBarrierMode = u32;
pub type Tagged<T> = *mut T;
pub type DirectHandle<T> = *mut T;
pub type MaybeDirectHandle<T> = Option<*mut T>;
pub type JSReceiver = u32;
pub type JSArrayBuffer = u32;
pub type FixedArray = u32;
pub type SeqOneByteString = u32;
pub type String = u32;
pub type HeapObject = u32;
pub type Map = u32;
pub type Script = u32;
pub type BreakPoint = u32;
pub type HeapNumber = u32;
pub type Code = u32;
pub type Tuple2 = u32;
pub type FixedInt32Array = u32;
pub type FixedAddressArray = u32;
pub type ProtectedFixedArray = u32;
pub type FixedUInt32Array = u32;
pub type Context = u32;
pub type WeakArrayList = u32;
pub type ProtectedWeakFixedArray = u32;
pub type AllocationType = u32;
const UPDATE_WRITE_BARRIER: u32 = 0;
const KB: usize = 1024;
const kSimd128Size: usize = 16;
const kMinimumOSPageSize: usize = 4096;
const kHeapObjectTag: usize = 1;

#[macro_export]
macro_rules! DECL_GETTER {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> $type {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DECL_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn get_$name(&self) -> $type {
            todo!()
        }
        fn set_$name(&mut self, value: $type) {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DECL_TRUSTED_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn get_$name(&self) -> $type {
            todo!()
        }
        fn set_$name(&mut self, value: $type) {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DECL_PRIMITIVE_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn get_$name(&self) -> $type {
            todo!()
        }
        fn set_$name(&mut self, value: $type) {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DECL_EXTERNAL_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn get_$name(&self) -> $type {
            todo!()
        }
        fn set_$name(&mut self, value: $type) {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DECL_CODE_POINTER_ACCESSORS {
    ($name:ident) => {
        fn get_$name(&self) -> WasmCodePointer {
            todo!()
        }
        fn set_$name(&mut self, value: WasmCodePointer) {
            todo!()
        }
    };
}

#[macro_export]
macro_rules! DEFINE_FIELD_OFFSET_CONSTANTS {
    ($base:expr, $($V:ident, $field:ident, $offset:expr, $size:expr),*) => {
        $(
            const $field: usize = $base + $offset;
        )*
    };
}

#[macro_export]
macro_rules! TQ_OBJECT_CONSTRUCTORS {
    ($name:ident) => {
        impl $name {
            fn cast(obj: Tagged<Object>) -> Option<*mut Self> {
                todo!()
            }

            fn unchecked_cast(obj: Tagged<Object>) -> *mut Self {
                todo!()
            }
        }
    };
}

#[macro_export]
macro_rules! OBJECT_CONSTRUCTORS {
    ($name:ident, $parent:ident) => {
        impl $name {
            fn cast(obj: Tagged<Object>) -> Option<*mut Self> {
                todo!()
            }

            fn unchecked_cast(obj: Tagged<Object>) -> *mut Self {
                todo!()
            }
        }
    };
}

// Constants
const POINTER_SIZE: usize = std::mem::size_of::<usize>();
const K_INT32_SIZE: usize = 4;
const K_UINT8_SIZE: usize = 1;
const K_SIZET_SIZE: usize = std::mem::size_of::<usize>();
const K_TAGGED_SIZE: usize = std::mem::size_of::<*mut u8>(); // Assuming a tagged pointer
const K_INT32_SIZE_LOG2: usize = 2;
const K_UINT32_SIZE: usize = 4;
const K_INT64_SIZE: usize = 8;
const K_FLOAT32_SIZE: usize = 4;
const K_FLOAT64_SIZE: usize = 8;
const K_SYSTEM_POINTER_SIZE: usize = std::mem::size_of::<usize>();

const FALSE_VALUE: usize = 0; // Assuming Smi zero represents false
const TRUE_VALUE: usize = 1; // Assuming Smi one represents true

// Utility functions
fn is_aligned(offset: usize, size: usize) -> bool {
    offset % size == 0
}

fn pointer_size_padding(offset: usize) -> usize {
    let remainder = offset % POINTER_SIZE;
    if remainder == 0 {
        0
    } else {
        POINTER_SIZE - remainder
    }
}
// enums
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SharedFlag {
    False = 0,
    True = 1,
}

impl From<bool> for SharedFlag {
    fn from(b: bool) -> Self {
        if b {
            SharedFlag::True
        } else {
            SharedFlag::False
        }
    }
}
pub mod wasm {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OnResume {
        kContinue,
        kThrow,
    }

    pub struct NativeModule;
    pub struct WasmCode;
    pub struct WasmFunction;
    pub struct WasmGlobal;
    pub struct WasmModule;
    pub struct WasmTag;
    pub type WasmTagSig = FunctionSig;
    pub struct WasmValue;
    pub struct WireBytesRef;
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Suspend {
        kFalse,
        kTrue,
    }
    pub struct FunctionSig;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Promise {
        kFalse,
        kTrue,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ImportCallKind {
        kRegular,
        kTailCall,
        kAsync,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ValueKind {
        I32,
        I64,
        F32,
        F64,
        Ref,
        // Add more as needed
    }

    impl ValueKind {
        pub fn value_kind_size(&self) -> u32 {
            match self {
                ValueKind::I32 => 4,
                ValueKind::I64 => 8,
                ValueKind::F32 => 4,
                ValueKind::F64 => 8,
                ValueKind::Ref => 8, // Assuming 8 bytes for reference types
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AddressType {
        k32,
        k64,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CanonicalSig;

    pub type CanonicalTypeIndex = u32;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CanonicalValueType(pub u32);

    impl CanonicalValueType {
        pub const INVALID: CanonicalValueType = CanonicalValueType(u32::MAX);
    }
}

impl From<bool> for wasm::Suspend {
    fn from(b: bool) -> Self {
        if b {
            wasm::Suspend::kTrue
        } else {
            wasm::Suspend::kFalse
        }
    }
}

impl From<bool> for wasm::Promise {
    fn from(b: bool) -> Self {
        if b {
            wasm::Promise::kTrue
        } else {
            wasm::Promise::kFalse
        }
    }
}

pub struct FunctionTargetAndImplicitArg {
    implicit_arg_: DirectHandle<TrustedObject>,
    call_target_: WasmCodePointer,
    #[cfg(V8_ENABLE_DRUMBRAKE)]
    target_func_index_: i32,
}

impl FunctionTargetAndImplicitArg {
    pub fn new(
        isolate: *mut Isolate,
        target_instance_data: DirectHandle<WasmTrustedInstanceData>,
        target_func_index: i32,
    ) -> Self {
        // TODO: Implement the constructor logic
        FunctionTargetAndImplicitArg {
            implicit_arg_: unsafe { mem::zeroed() },
            call_target_: ptr::null(),
            #[cfg(V8_ENABLE_DRUMBRAKE)]
            target_func_index_: target_func_index,
        }
    }

    pub fn implicit_arg(&self) -> DirectHandle<TrustedObject> {
        self.implicit_arg_
    }

    pub fn call_target(&self) -> WasmCodePointer {
        self.call_target_
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn target_func_index(&self) -> i32 {
        self.target_func_index_
    }
}

pub struct ImportedFunctionEntry {
    instance_data_: DirectHandle<WasmTrustedInstanceData>,
    index_: i32,
}

impl ImportedFunctionEntry {
    pub fn new(
        instance_data: DirectHandle<WasmTrustedInstanceData>,
        index: i32,
    ) -> Self {
        ImportedFunctionEntry {
            instance_data_: instance_data,
            index_: index,
        }
    }

    pub fn set_generic_wasm_to_js(
        &mut self,
        isolate: *mut Isolate,
        callable: DirectHandle<JSReceiver>,
        suspend: wasm::Suspend,
        sig: *const wasm::CanonicalSig,
        sig_id: wasm::CanonicalTypeIndex,
    ) {
        todo!()
    }

    pub fn set_compiled_wasm_to_js(
        &mut self,
        isolate: *mut Isolate,
        callable: DirectHandle<JSReceiver>,
        wasm_to_js_wrapper: *mut wasm::WasmCode,
        suspend: wasm::Suspend,
        sig: *const wasm::CanonicalSig,
        sig_id: wasm::CanonicalTypeIndex,
    ) {
        todo!()
    }

    pub fn set_wasm_to_wasm(
        &mut self,
        target_instance_object: Tagged<WasmTrustedInstanceData>,
        call_target: WasmCodePointer,
        sig_id: wasm::CanonicalTypeIndex,
        #[cfg(V8_ENABLE_DRUMBRAKE)] exported_function_index: i32,
    ) {
        todo!()
    }

    pub fn callable(&self) -> Tagged<JSReceiver> {
        todo!()
    }

    pub fn maybe_callable(&self) -> Tagged<Object> {
        todo!()
    }

    pub fn implicit_arg(&self) -> Tagged<Object> {
        todo!()
    }

    pub fn target(&self) -> WasmCodePointer {
        todo!()
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn function_index_in_called_module(&self) -> i32 {
        todo!()
    }
}

#[derive(PartialEq)]
pub enum InternalizeString {
    kInternalize,
    kNoInternalize,
}

pub struct WasmModuleObject {
    // Assuming JSObject fields are defined elsewhere
    native_module_: *mut wasm::NativeModule, //Raw pointer
    shared_native_module_: Option<Arc<wasm::NativeModule>>,
    script_: DirectHandle<Script>,
}

impl WasmModuleObject {
    pub fn native_module(&self) -> *mut wasm::NativeModule {
        self.native_module_
    }

    pub fn shared_native_module(&self) -> &Option<Arc<wasm::NativeModule>> {
        &self.shared_native_module_
    }

    pub fn module(&self) -> *const wasm::WasmModule {
        todo!()
    }

    pub fn new(
        isolate: *mut Isolate,
        native_module: Arc<wasm::NativeModule>,
        script: DirectHandle<Script>,
    ) -> DirectHandle<WasmModuleObject> {
        todo!()
    }

    pub fn is_asm_js(&self) -> bool {
        todo!()
    }

    pub fn get_module_name_or_null(
        isolate: *mut Isolate,
        module_object: DirectHandle<WasmModuleObject>,
    ) -> MaybeDirectHandle<String> {
        todo!()
    }

    pub fn get_function_name_or_null(
        isolate: *mut Isolate,
        module_object: DirectHandle<WasmModuleObject>,
        func_index: u32,
    ) -> MaybeDirectHandle<String> {
        todo!()
    }

    pub fn get_raw_function_name(&self, func_index: i32) -> Vec<u8> {
        todo!()
    }

    pub fn extract_utf8_string_from_module_bytes(
        isolate: *mut Isolate,
        module_object: DirectHandle<WasmModuleObject>,
        wire_bytes_ref: wasm::WireBytesRef,
        internalize: InternalizeString,
    ) -> DirectHandle<String> {
        todo!()
    }

    pub fn extract_utf8_string_from_module_bytes_static(
        isolate: *mut Isolate,
        wire_byte: Vec<u8>,
        wire_bytes_ref: wasm::WireBytesRef,
        internalize: InternalizeString,
    ) -> DirectHandle<String> {
        todo!()
    }
}

#[cfg(any(V8_ENABLE_SANDBOX, debug_assertions))]
pub fn function_sig_matches_table(
    sig_id: wasm::CanonicalTypeIndex,
    table_type: wasm::CanonicalValueType,
) -> bool {
    todo!()
}

pub struct WasmTableObject {
    // Assuming JSObject fields are defined elsewhere
}

impl WasmTableObject {
    pub fn type_(&self, module: *const wasm::WasmModule) -> wasm::ValueType {
        todo!()
    }

    pub fn canonical_type(&self, module: *const wasm::WasmModule) -> wasm::CanonicalValueType {
        todo!()
    }

    pub fn unsafe_type(&self) -> wasm::ValueType {
        todo!()
    }

    pub fn get_trusted_data(&self) -> Tagged<WasmTrustedInstanceData> {
        todo!()
    }

    pub fn set_trusted_data(&mut self, value: Tagged<WasmTrustedInstanceData>) {
        todo!()
    }

    pub fn get_trusted_dispatch_table(&self) -> Tagged<WasmDispatchTable> {
        todo!()
    }

    pub fn set_trusted_dispatch_table(&mut self, value: Tagged<WasmDispatchTable>) {
        todo!()
    }

    pub fn grow(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        count: u32,
        init_value: DirectHandle<Object>,
    ) -> i32 {
        todo!()
    }

    pub fn new(
        isolate: *mut Isolate,
        trusted_data: DirectHandle<WasmTrustedInstanceData>,
        type_: wasm::ValueType,
        canonical_type: wasm::CanonicalValueType,
        initial: u32,
        has_maximum: bool,
        maximum: u64,
        initial_value: DirectHandle<Object>,
        address_type: wasm::AddressType,
        out_dispatch_table: *mut DirectHandle<WasmDispatchTable>,
    ) -> DirectHandle<WasmTableObject> {
        todo!()
    }

    pub fn is_in_bounds(&self, entry_index: u32) -> bool {
        todo!()
    }

    pub fn is_table64(&self) -> bool {
        todo!()
    }

    pub fn maximum_length_u64(&self) -> Option<u64> {
        todo!()
    }

    pub fn js_to_wasm_element(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry: DirectHandle<Object>,
        error_message: &*const char,
    ) -> MaybeDirectHandle<Object> {
        todo!()
    }

    pub fn set(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        index: u32,
        entry: DirectHandle<Object>,
    ) {
        todo!()
    }

    pub fn get(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        index: u32,
    ) -> DirectHandle<Object> {
        todo!()
    }

    pub fn fill(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        start: u32,
        entry: DirectHandle<Object>,
        count: u32,
    ) {
        todo!()
    }

    pub fn update_dispatch_table_wasm_function(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry_index: i32,
        func: *const wasm::WasmFunction,
        target_instance: DirectHandle<WasmTrustedInstanceData>,
        #[cfg(V8_ENABLE_DRUMBRAKE)] target_func_index: i32,
    ) {
        todo!()
    }

    pub fn update_dispatch_table_js_function(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry_index: i32,
        function: DirectHandle<WasmJSFunction>,
    ) {
        todo!()
    }

    pub fn update_dispatch_table_capi_function(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry_index: i32,
        capi_function: DirectHandle<WasmCapiFunction>,
    ) {
        todo!()
    }

    pub fn clear_dispatch_table(&mut self, index: i32) {
        todo!()
    }

    pub fn set_function_table_placeholder(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry_index: i32,
        trusted_instance_data: DirectHandle<WasmTrustedInstanceData>,
        func_index: i32,
    ) {
        todo!()
    }

    pub fn get_function_table_entry(
        isolate: *mut Isolate,
        table: DirectHandle<WasmTableObject>,
        entry_index: i32,
        is_valid: *mut bool,
        is_null: *mut bool,
        instance_data: *mut MaybeDirectHandle<WasmTrustedInstanceData>,
        function_index: *mut i32,
        maybe_js_function: *mut MaybeDirectHandle<WasmJSFunction>,
    ) {
        todo!()
    }
}

pub struct WasmMemoryMapDescriptor {}

impl WasmMemoryMapDescriptor {
    pub fn new_from_anonymous(
        isolate: *mut Isolate,
        length: usize,
    ) -> MaybeDirectHandle<WasmMemoryMapDescriptor> {
        todo!()
    }

    pub fn new_from_file_descriptor(
        isolate: *mut Isolate,
        file_descriptor: v8::WasmMemoryMapDescriptor::WasmFileDescriptor,
    ) -> DirectHandle<WasmMemoryMapDescriptor> {
        todo!()
    }

    pub fn map_descriptor(
        &self,
        memory: DirectHandle<WasmMemoryObject>,
        offset: usize,
    ) -> usize {
        todo!()
    }

    pub fn unmap_descriptor(&self) -> bool {
        todo!()
    }
}

pub mod v8 {
    pub mod WasmMemoryMapDescriptor {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum WasmFileDescriptor {
            Invalid,
        }
    }
}
pub struct WasmMemoryObject {
    // Assuming JSObject fields are defined elsewhere
}

impl WasmMemoryObject {
    pub fn get_instances(&self) -> Tagged<WeakArrayList> {
        todo!()
    }

    pub fn set_instances(&mut self, value: Tagged<WeakArrayList>) {
        todo!()
    }

    pub fn use_in_instance(
        isolate: *mut Isolate,
        memory: DirectHandle<WasmMemoryObject>,
        trusted_instance_data: DirectHandle<WasmTrustedInstanceData>,
        shared_trusted_instance_data: DirectHandle<WasmTrustedInstanceData>,
        memory_index_in_instance: i32,
    ) {
        todo!()
    }

    pub fn has_maximum_pages(&self) -> bool {
        todo!()
    }

    pub fn is_memory64(&self) -> bool {
        todo!()
    }

    pub fn new_from_buffer(
        isolate: *mut Isolate,
        buffer: DirectHandle<JSArrayBuffer>,
        maximum: i32,
        address_type: wasm::AddressType,
    ) -> DirectHandle<WasmMemoryObject> {
        todo!()
    }

    pub fn new_from_initial(
        isolate: *mut Isolate,
        initial: i32,
        maximum: i32,
        shared: SharedFlag,
        address_type: wasm::AddressType,
    ) -> MaybeDirectHandle<WasmMemoryObject> {
        todo!()
    }

    pub fn set_new_buffer(&mut self, new_buffer: Tagged<JSArrayBuffer>) {
        todo!()
    }

    pub fn grow(
        isolate: *mut Isolate,
        memory: DirectHandle<WasmMemoryObject>,
        pages: u32,
    ) -> i32 {
        todo!()
    }

    pub const K_NO_MAXIMUM: i32 = -1;
}

pub struct WasmGlobalObject {
    // Assuming JSObject fields are defined elsewhere
}

impl WasmGlobalObject {
    pub fn get_untagged_buffer(&self) -> Tagged<JSArrayBuffer> {
        todo!()
    }

    pub fn set_untagged_buffer(&mut self, value: Tagged<JSArrayBuffer>) {
        todo!()
    }

    pub fn get_tagged_buffer(&self) -> Tagged<FixedArray> {
        todo!()
    }

    pub fn set_tagged_buffer(&mut self, value: Tagged<FixedArray>) {
        todo!()
    }

    pub fn get_type(&self) -> wasm::ValueType {
        todo!()
    }

    pub fn set_type(&mut self, value: wasm::ValueType) {
        todo!()
    }

    pub fn get_trusted_data(&self) -> Tagged<WasmTrustedInstanceData> {
        todo!()
    }

    pub fn set_trusted_data(&mut self, value: Tagged<WasmTrustedInstanceData>) {
        todo!()
    }

    pub fn new(
        isolate: *mut Isolate,
        instance_object: DirectHandle<WasmTrustedInstanceData>,
        maybe_untagged_buffer: MaybeDirectHandle<JSArrayBuffer>,
        maybe_tagged_buffer: MaybeDirectHandle<FixedArray>,
        type_: wasm::ValueType,
        offset: i32,
        is_mutable: bool,
    ) -> MaybeDirectHandle<WasmGlobalObject> {
        todo!()
    }

    pub fn type_size(&self) -> i32 {
        todo!()
    }

    pub fn get_i32(&self) -> i32 {
        todo!()
    }

    pub fn get_i64(&self) -> i64 {
        todo!()
    }

    pub fn get_f32(&self) -> f32 {
        todo!()
    }

    pub fn get_f64(&self) -> f64 {
        todo!()
    }

    pub fn get_s128_raw_bytes(&self) -> *mut u8 {
        todo!()
    }

    pub fn get_ref(&self) -> DirectHandle<Object> {
        todo!()
    }

    pub fn set_i32(&mut self, value: i32) {
        todo!()
    }

    pub fn set_i64(&mut self, value: i64) {
        todo!()
    }

    pub fn set_f32(&mut self, value: f32) {
        todo!()
    }

    pub fn set_f64(&mut self, value: f64) {
        todo!()
    }

    pub fn set_ref(&mut self, value: DirectHandle<Object>) {
        todo!()
    }
}

pub struct ExposedTrustedObject;

#[derive(Debug)]
pub struct WasmTrustedInstanceData {
    instance_object_: Option<Tagged<WasmInstanceObject>>,
    native_context_: Tagged<Context>,
    memory_objects_: Tagged<FixedArray>,
    #[cfg(V8_ENABLE_DRUMBRAKE)]
    interpreter_object_: Option<Tagged<Tuple2>>,
    untagged_globals_buffer_: Option<Tagged<JSArrayBuffer>>,
    tagged_globals_buffer_: Option<Tagged<FixedArray>>,
    imported_mutable_globals_buffers_: Option<Tagged<FixedArray>>,
    tables_: Option<Tagged<FixedArray>>,
    dispatch_table_for_imports_: Mutex<WasmDispatchTable>,
    imported_mutable_globals_: Tagged<FixedAddressArray>,
    #[cfg(V8_ENABLE_DRUMBRAKE)]
    imported_function_indices_: Tagged<FixedInt32Array>,
    shared_part_: Mutex<WasmTrustedInstanceData>,
    dispatch_table0_: Mutex<WasmDispatchTable>,
    dispatch_tables_: Mutex<ProtectedFixedArray>,
    tags_table_: Option<Tagged<FixedArray>>,
    func_refs_: Tagged<FixedArray>,
    managed_object_maps_: Tagged<FixedArray>,
    feedback_vectors_: Tagged<FixedArray>,
    well_known_imports_: Tagged<FixedArray>,
    memory0_start_: *mut u8,
    memory0_size_: usize,
    globals_start_: *mut u8,
    jump_table_start_: Address,
    hook_on_function_call_address_: Address,
    tiering_budget_array_: *mut std::sync::atomic::AtomicU32,
    memory_bases_and_sizes_: Mutex<TrustedFixedAddressArray>,
    data_segment_starts_: Tagged<FixedAddressArray>,
    data_segment_sizes_: Tagged<FixedUInt32Array>,
    element_segments_: Tagged<FixedArray>,
    break_on_entry_: u8,
    stress_deopt_counter_address_: Address,
    managed_native_module_: Arc<wasm::NativeModule>,
    new_allocation_limit_address_: *mut Address,
    new_allocation_top_address_: *mut Address,
    old_allocation_limit_address_: *mut Address,
    old_allocation_top_address_: *mut Address,
}

impl WasmTrustedInstanceData {
    pub fn has_instance_object(&self) -> bool {
        todo!()
    }

    pub fn get_instance_object(&self) -> Tagged<WasmInstanceObject> {
        todo!()
    }

    pub fn set_instance_object(&mut self, value: Tagged<WasmInstanceObject>) {
        todo!()
    }

    pub fn get_native_context(&self) -> Tagged<Context> {
        self.native_context_
    }

    pub fn set_native_context(&mut self, value: Tagged<Context>) {
        self.native_context_ = value;
    }

    pub fn get_memory_objects(&self) -> Tagged<FixedArray> {
        self.memory_objects_
    }

    pub fn set_memory_objects(&mut self, value: Tagged<FixedArray>) {
        self.memory_objects_ = value;
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn has_interpreter_object(&self) -> bool {
        todo!()
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn get_interpreter_object(&self) -> Tagged<Tuple2> {
        todo!()
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn set_interpreter_object(&mut self, value: Tagged<Tuple2>) {
        todo!()
    }

    pub fn has_untagged_globals_buffer(&self) -> bool {
        todo!()
    }

    pub fn get_untagged_globals_buffer(&self) -> Tagged<JSArrayBuffer> {
        todo!()
    }

    pub fn set_untagged_globals_buffer(&mut self, value: Tagged<JSArrayBuffer>) {
        todo!()
    }

    pub fn has_tagged_globals_buffer(&self) -> bool {
        todo!()
    }

    pub fn get_tagged_globals_buffer(&self) -> Tagged<FixedArray> {
        todo!()
    }

    pub fn set_tagged_globals_buffer(&mut self, value: Tagged<FixedArray>) {
        todo!()
    }

    pub fn has_imported_mutable_globals_buffers(&self) -> bool {
        todo!()
    }

    pub fn get_imported_mutable_globals_buffers(&self) -> Tagged<FixedArray> {
        todo!()
    }

    pub fn set_imported_mutable_globals_buffers(&mut self, value: Tagged<FixedArray>) {
        todo!()
    }

    pub fn has_tables(&self) -> bool {
        todo!()
    }

    pub fn get_tables(&self) -> Tagged<FixedArray> {
        todo!()
    }

    pub fn set_tables(&mut self, value: Tagged<FixedArray>) {
        todo!()
    }

    pub fn get_dispatch_table_for_imports(&self) -> MutexGuard<'_, WasmDispatchTable> {
        self.dispatch_table_for_imports_.lock().unwrap()
    }

    pub fn set_dispatch_table_for_imports(&self, value: WasmDispatchTable) {
        let mut lock = self.dispatch_table_for_imports_.lock().unwrap();
        *lock = value;
    }

    pub fn get_imported_mutable_globals(&self) -> Tagged<FixedAddressArray> {
        self.imported_mutable_globals_
    }

    pub fn set_imported_mutable_globals(&mut self, value: Tagged<FixedAddressArray>) {
        self.imported_mutable_globals_ = value;
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn get_imported_function_indices(&self) -> Tagged<FixedInt32Array> {
        todo!()
    }

    #[cfg(V8_ENABLE_DRUMBRAKE)]
    pub fn set