// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::convert::TryFrom;

// Placeholder for src/common/globals.h
// Assuming it defines common types and constants
mod common {
    pub type ExternalArrayType = u32; // Placeholder type
    pub type ElementsKind = u32;     // Placeholder type
    
    pub const kExternalFloat64Array: ExternalArrayType = 0;
    pub const kExternalInt32Array: ExternalArrayType = 1;

    pub const RAB_GSAB_FLOAT64_ELEMENTS: ElementsKind = 0;
    pub const RAB_GSAB_INT32_ELEMENTS: ElementsKind = 1;
    pub const HOLEY_SMI_ELEMENTS: ElementsKind = 2; // Example ElementsKind
}

// Placeholder for src/flags/flags.h
// Assuming it defines flags used in the V8 engine
mod flags {
    pub struct Flags {
        pub turbo_collect_feedback_in_generic_lowering: bool,
    }
    
    impl Flags {
        pub const fn new() -> Self {
            Flags {
                turbo_collect_feedback_in_generic_lowering: false,
            }
        }
    }
    
    pub static mut v8_flags: Flags = Flags::new(); // Use a static mutable Flags instance for simplicity.  In a real Rust V8 port, this might need to be thread-safe (using Mutex/RwLock) if multiple threads access flags.
}

// Placeholder for src/objects/js-objects.h
// Assuming it defines constants related to JS objects
mod js_objects {
    pub const kMaxInObjectProperties: i32 = 10; // Placeholder value
}

// Placeholder for src/runtime/runtime.h
// Assuming it defines runtime function IDs
mod runtime {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum FunctionId {
        kStackGuardWithGap,
        kHandleNoHeapWritesInterrupts,
        kStackGuard,
    }
}

/// Determines whether to collect feedback in generic lowering.
///
/// This flag is currently used to experiment with feedback collection in
/// optimized code produced by generic lowering.
///
/// Returns:
///   true if feedback collection is enabled; otherwise, false.
pub fn collect_feedback_in_generic_lowering() -> bool {
    unsafe { flags::v8_flags.turbo_collect_feedback_in_generic_lowering }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum StackCheckKind {
    JSFunctionEntry = 0,
    JSIterationBody,
    CodeStubAssembler,
    Wasm,
}

impl StackCheckKind {
    pub fn get_builtin_for_stack_check_kind(&self) -> runtime::FunctionId {
        match self {
            StackCheckKind::JSFunctionEntry => runtime::FunctionId::kStackGuardWithGap,
            StackCheckKind::JSIterationBody => runtime::FunctionId::kHandleNoHeapWritesInterrupts,
            _ => runtime::FunctionId::kStackGuard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum CanThrow {
    No,
    Yes,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum LazyDeoptOnThrow {
    No,
    Yes,
}

impl fmt::Display for LazyDeoptOnThrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LazyDeoptOnThrow::Yes => write!(f, "LazyDeoptOnThrow"),
            LazyDeoptOnThrow::No => write!(f, "DoNOTLazyDeoptOnThrow"),
        }
    }
}

impl fmt::Display for StackCheckKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackCheckKind::JSFunctionEntry => write!(f, "JSFunctionEntry"),
            StackCheckKind::JSIterationBody => write!(f, "JSIterationBody"),
            StackCheckKind::CodeStubAssembler => write!(f, "CodeStubAssembler"),
            StackCheckKind::Wasm => write!(f, "Wasm"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum CheckForMinusZeroMode {
    CheckForMinusZero,
    DontCheckForMinusZero,
}

impl fmt::Display for CheckForMinusZeroMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckForMinusZeroMode::CheckForMinusZero => write!(f, "check-for-minus-zero"),
            CheckForMinusZeroMode::DontCheckForMinusZero => write!(f, "dont-check-for-minus-zero"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CallFeedbackRelation {
    Receiver,
    Target,
    Unrelated,
}

impl fmt::Display for CallFeedbackRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallFeedbackRelation::Receiver => write!(f, "CallFeedbackRelation::kReceiver"),
            CallFeedbackRelation::Target => write!(f, "CallFeedbackRelation::kTarget"),
            CallFeedbackRelation::Unrelated => write!(f, "CallFeedbackRelation::kUnrelated"),
        }
    }
}

/// Maximum depth for literal graphs to be considered for fast deep-copying.
pub const MAX_FAST_LITERAL_DEPTH: i32 = 3;

/// Maximum number of elements and properties for literal graphs to be considered for fast deep-copying.
pub const MAX_FAST_LITERAL_PROPERTIES: i32 = js_objects::kMaxInObjectProperties;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum BaseTaggedness {
    UntaggedBase,
    TaggedBase,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum MemoryAccessKind {
    Normal,
    Unaligned,
    ProtectedByTrapHandler,
}

impl fmt::Display for MemoryAccessKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryAccessKind::Normal => write!(f, "Normal"),
            MemoryAccessKind::Unaligned => write!(f, "Unaligned"),
            MemoryAccessKind::ProtectedByTrapHandler => write!(f, "ProtectedByTrapHandler"),
        }
    }
}

impl Hash for MemoryAccessKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}

pub fn get_array_type_from_elements_kind(kind: common::ElementsKind) -> common::ExternalArrayType {
    match kind {
        common::RAB_GSAB_FLOAT64_ELEMENTS => common::kExternalFloat64Array,
        common::RAB_GSAB_INT32_ELEMENTS => common::kExternalInt32Array,
        _ => panic!("ElementsKind not supported."),
    }
}

pub fn external_array_element_size(element_type: common::ExternalArrayType) -> usize {
    match element_type {
        common::kExternalFloat64Array => {
            std::mem::size_of::<f64>()
        }
        common::kExternalInt32Array => {
            std::mem::size_of::<i32>()
        }
        _ => panic!("ExternalArrayType not supported."),
    }
}

/// The biggest double value that fits within the int64_t value range.
pub const MAX_DOUBLE_REPRESENTABLE_INT64: f64 = 9223372036854774784.0;

pub const MIN_DOUBLE_REPRESENTABLE_INT64: f64 = i64::MIN as f64;

pub const MAX_DOUBLE_REPRESENTABLE_UINT64: f64 = 18446744073709549568.0;

// Replicate the functionality of kMinusZeroBits without relying on undefined behavior
pub const MINUS_ZERO_BITS: i64 = i64::from_be_bytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);