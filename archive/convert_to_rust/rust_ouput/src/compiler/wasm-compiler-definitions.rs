// Converted from V8 C++ source files:
// Header: wasm-compiler-definitions.h
// Implementation: wasm-compiler-definitions.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fmt;
use std::hash::{Hash, Hasher};
use crate::wasm;
use crate::compiler;
use crate::compiler::Node;
use crate::execution::isolate::Isolate;
use crate::objects::tagged::Tagged;
use crate::wasm::Signature;
use crate::wasm::ValueType;
use crate::wasm::ModuleWireBytes;
use crate::compiler::LinkageLocation;
use crate::compiler::MachineType;
use crate::compiler::RegList;
use crate::compiler::DoubleRegList;
use crate::execution::isolate::DirectHandle;
use crate::objects::js_array::ArrayList;
use crate::objects::feedback_vector::FeedbackVector;
use crate::compiler::JSHeapBroker;
use crate::objects::heap_object::HeapObjectRef;
use crate::objects::map::MapRef;
use crate::execution::address::Address;
use crate::deoptimizer::deoptimizer::Debug;
use crate::execution::root_index::RootIndex;
use crate::objects::object::Object;
use crate::codegen::register::Register;
use crate::codegen::signature::LocationSignature;
use crate::codegen::signature::StackArgumentOrder;
use crate::base::flags::Flags;
use crate::base::flags::Flag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmTypeCheckConfig {
    pub from: wasm::ValueType,
    pub to: wasm::ValueType,
}

impl fmt::Display for WasmTypeCheckConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from.name(), self.to.name())
    }
}

impl Hash for WasmTypeCheckConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.raw_bit_field().hash(state);
        self.to.raw_bit_field().hash(state);
    }
}

impl WasmTypeCheckConfig {
    pub fn new(from: wasm::ValueType, to: wasm::ValueType) -> Self {
        WasmTypeCheckConfig { from, to }
    }
}

pub const K_CHAR_WIDTH_BAILOUT_SENTINEL: i32 = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NullCheckStrategy {
    kExplicit,
    kTrapHandler,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EnforceBoundsCheck {
    kNeedsBoundsCheck = 1,
    kCanOmitBoundsCheck = 0,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AlignmentCheck {
    kYes = 1,
    kNo = 0,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundsCheckResult {
    kDynamicallyChecked,
    kTrapHandler,
    kInBounds,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CheckForNull {
    kWithoutNullCheck,
    kWithNullCheck,
}

impl fmt::Display for CheckForNull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckForNull::kWithoutNullCheck => write!(f, "no null check"),
            CheckForNull::kWithNullCheck => write!(f, "null check"),
        }
    }
}

pub fn get_debug_name<'a>(
    zone: &'a Zone,
    module: *const wasm::WasmModule,
    wire_bytes: *const wasm::WireBytesStorage,
    index: i32,
) -> base::Vector<'a, i8> {
    unsafe {
        if !wire_bytes.is_null() && !module.is_null() {
            let module_bytes = (*wire_bytes).GetModuleBytes();
            if module_bytes.is_some() {
                let name = (*module).lazily_generated_names.LookupFunctionName(module_bytes.unwrap(), index);
                if !name.is_empty() {
                    let name_len = name.length();
                    let mut index_name = zone.allocate_array::<i8>(name_len as usize);
                    std::ptr::copy_nonoverlapping(
                        (*module_bytes.unwrap().start()) as *const i8,
                        index_name.as_mut_ptr(),
                        name_len as usize
                    );

                    return base::Vector::new(index_name.as_mut_ptr(), name_len);
                }
            }
        }
    }
    let k_buffer_length = 24;
    let mut name_vector: base::EmbeddedVector<i8, 24> = base::EmbeddedVector::new();
    let name_len = format!("wasm-function#{}", index).len();
    if name_len > k_buffer_length {
      panic!("name length exceeds buffer length");
    }
    let name_str = format!("wasm-function#{}", index);

    for (i, c) in name_str.chars().enumerate() {
      name_vector.data[i] = c as i8;
    }

    let mut index_name = zone.allocate_array::<i8>(name_len);
    unsafe {
      std::ptr::copy_nonoverlapping(
        name_vector.data.as_ptr(),
        index_name.as_mut_ptr(),
        name_len,
      );
    }
    
    return base::Vector::new(index_name.as_mut_ptr(), name_len);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmCallKind {
    kWasmFunction,
    kWasmIndirectFunction,
    kWasmImportWrapper,
    kWasmCapiFunction,
}

pub fn get_wasm_call_descriptor<'a, T>(
    zone: &'a Zone,
    fsig: *const Signature<T>,
    call_kind: WasmCallKind,
    need_frame_state: bool,
) -> *mut compiler::CallDescriptor {
    let extra_callable_param =
        call_kind == WasmCallKind::kWasmImportWrapper || call_kind == WasmCallKind::kWasmCapiFunction;

    let mut parameter_slots: i32 = 0;
    let mut return_slots: i32 = 0;
    let location_sig = unsafe { build_locations(zone, fsig, extra_callable_param, &mut parameter_slots, &mut return_slots) };

    let k_callee_save_registers = RegList{};
    let k_callee_save_fp_registers = DoubleRegList{};

    let target_type = MachineType::Pointer();
    let target_loc = LinkageLocation::ForAnyRegister(target_type);

    let descriptor_kind;
    let mut signature_hash: u64 = k_invalid_wasm_signature_hash;

    match call_kind {
        WasmCallKind::kWasmFunction => {
            descriptor_kind = compiler::CallDescriptor::kCallWasmFunction;
        }
        WasmCallKind::kWasmIndirectFunction => {
            descriptor_kind = compiler::CallDescriptor::kCallWasmFunctionIndirect;
            signature_hash = unsafe { wasm::SignatureHasher::Hash(fsig) };
        }
        WasmCallKind::kWasmImportWrapper => {
            descriptor_kind = compiler::CallDescriptor::kCallWasmImportWrapper;
        }
        WasmCallKind::kWasmCapiFunction => {
            descriptor_kind = compiler::CallDescriptor::kCallWasmCapiFunction;
        }
    }

    let flags = if need_frame_state {
        compiler::CallDescriptor::kNeedsFrameState
    } else {
        compiler::CallDescriptor::kNoFlags
    };

    unsafe {
        zone.new(compiler::CallDescriptor {
            kind: descriptor_kind,
            tag: KWASM_ENTRYPOINT_TAG,
            target_machine_type: target_type,
            target_location: target_loc,
            location_signature: location_sig,
            parameter_slot_count: parameter_slots,
            properties: compiler::Operator::kNoProperties,
            callee_saved_registers: k_callee_save_registers,
            callee_saved_fp_registers: k_callee_save_fp_registers,
            flags,
            debug_name: "wasm-call",
            stack_argument_order: StackArgumentOrder::kDefault,
            allocatable_registers: RegList{},
            return_slot_count: return_slots,
            signature_hash,
        })
    }
}

pub fn build_locations<'a, T>(
    zone: &'a Zone,
    sig: *const Signature<T>,
    extra_callable_param: bool,
    parameter_slots: &mut i32,
    return_slots: &mut i32,
) -> *mut LocationSignature {
    let extra_params = if extra_callable_param { 2 } else { 1 };
    let mut locations = LocationSignature::Builder::new(zone, unsafe { (*sig).return_count() }, unsafe { (*sig).parameter_count() } + extra_params);

    let mut untagged_parameter_slots: i32 = 0;
    let mut untagged_return_slots: i32 = 0;

    unsafe {
        wasm::IterateSignatureImpl(
            sig,
            extra_callable_param,
            &mut locations,
            &mut untagged_parameter_slots,
            parameter_slots,
            &mut untagged_return_slots,
            return_slots,
        );
    }
    locations.Get()
}

pub struct Zone {

}

impl Zone {
    pub fn allocate_array<T>(&self, count: usize) -> Vec<T> {
        vec![unsafe { std::mem::zeroed() }; count]
    }
    pub fn new<T>(&self, value: T) -> *mut T {
        let boxed = Box::new(value);
        Box::into_raw(boxed)
    }
}

mod base {
  pub mod flags {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Flags<T> {
      _phantom: std::marker::PhantomData<T>,
    }
    impl<T> Flags<T> {
        pub fn new() -> Self {
            Flags { _phantom: std::marker::PhantomData }
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Flag {
    }
  }
  pub struct Vector<'a, T> {
      ptr: *mut T,
      length: usize,
      _phantom: std::marker::PhantomData<&'a T>,
  }

  impl<'a, T> Vector<'a, T> {
      pub fn new(ptr: *mut T, length: usize) -> Self {
          Vector { ptr, length, _phantom: std::marker::PhantomData }
      }
  }

    pub struct EmbeddedVector<T, const SIZE: usize> {
        pub data: [T; SIZE],
    }

    impl<T, const SIZE: usize> EmbeddedVector<T, const SIZE> {
        pub fn new() -> Self {
            EmbeddedVector { data: [unsafe { std::mem::zeroed() }; SIZE] }
        }
        pub fn length(&self) -> usize {
            SIZE
        }
        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }
    }
}

const KWASM_ENTRYPOINT_TAG: i32 = 123;
const k_invalid_wasm_signature_hash: u64 = 0;
