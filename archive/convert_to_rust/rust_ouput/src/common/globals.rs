// Converted from V8 C++ source files:
// Header: globals.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
use std::{sync::{Mutex, RwLock}, cell::RefCell, rc::Rc, sync::Arc, os::raw::c_void};

use crate::inspector::string_util::V8;
pub mod base {
    pub struct Mutex {}
    pub struct RecursiveMutex {}
    pub mod atomic_utils{
        pub struct Atomic32 {}
        pub struct AtomicWord {}
    }
    pub mod flags{
        pub struct Flags<E, T> {}
    }
}
pub mod objects {
    pub struct JSFunction {}
    pub struct Object {}
    pub struct String {}
    pub struct Name {}
    pub struct ScopeInfo {}
}
pub mod handles {
    pub struct Handle<T> {}
    pub struct DirectHandle<T> {}
}
pub mod parsing {
    pub struct Script {}
}
pub mod compiler {
    pub struct AccessBuilder {}
}
pub mod builtins {
    pub struct Builtins {}
}
pub mod execution{
    pub struct Builtin {}
}
pub mod codegen{
    pub struct Register {}
    pub struct Label {}
    pub mod arm64{
        pub struct RegisterArray {}
    }
}

#[allow(dead_code)]
pub const V8_INFINITY: f64 = f64::INFINITY;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageMode {
    kSloppy,
    kStrict,
}

impl LanguageMode {
    fn to_string(&self) -> &'static str {
        match self {
            LanguageMode::kSloppy => "sloppy",
            LanguageMode::kStrict => "strict",
        }
    }

    fn is_sloppy(&self) -> bool {
        *self == LanguageMode::kSloppy
    }

    fn is_strict(&self) -> bool {
        *self == LanguageMode::kStrict
    }
}

impl std::fmt::Display for LanguageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreOrigin {
    kMaybeKeyed,
    kNamed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeofMode {
    kInside,
    kNotInside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextKind {
    kDefault,
    kScriptContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveFPRegsMode {
    kIgnore,
    kSave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndirectPointerMode {
    kStrong,
    kCustom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgvMode {
    kStack,
    kRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallApiCallbackMode {
    kGeneric,
    kOptimizedNoProfiling,
    kOptimized,
}

pub const K_NO_SOURCE_POSITION: i32 = -1;

pub const K_FUNCTION_ENTRY_BYTECODE_OFFSET: i32 = -1;

pub const K_FUNCTION_EXIT_BYTECODE_OFFSET: i32 = -1;

pub const K_NO_DEOPTIMIZATION_ID: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeoptimizeKind {
    kEager,
    kLazy,
}

impl std::fmt::Display for DeoptimizeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeoptimizeKind::kEager => write!(f, "Eager"),
            DeoptimizeKind::kLazy => write!(f, "Lazy"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LookupHoistingMode {
    kNormal,
    kLegacySloppy,
}

impl std::fmt::Display for LookupHoistingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookupHoistingMode::kNormal => write!(f, "normal hoisting"),
            LookupHoistingMode::kLegacySloppy => write!(f, "legacy sloppy hoisting"),
        }
    }
}

pub const K_SMI_TAG_MASK: usize = 0b1;
pub const K_SMI_TAG: usize = 0b0;
pub const K_SMI_SHIFT_SIZE: usize = 0;
pub const K_HEAP_OBJECT_TAG_MASK: usize = 0b111;
pub const K_HEAP_OBJECT_TAG: usize = 0b000;
pub const K_WEAK_HEAP_OBJECT_TAG: usize = 0b001;
pub const K_API_TAGGED_SIZE: usize = 4;
pub const K_SMI_VALUE_SIZE: usize = 31;
pub const K_POINTER_COMPRESSION_BOOL : bool = false;

#[derive(Clone, Copy)]
pub struct Smi {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchHint {
    kNone,
    kTrue,
    kFalse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GotoHint {
    kNone,
    kLabel,
    kFallthrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvertReceiverMode {
    kNullOrUndefined,
    kNotNullOrUndefined,
    kAny,
}

impl std::fmt::Display for ConvertReceiverMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertReceiverMode::kNullOrUndefined => write!(f, "NULL_OR_UNDEFINED"),
            ConvertReceiverMode::kNotNullOrUndefined => write!(f, "NOT_NULL_OR_UNDEFINED"),
            ConvertReceiverMode::kAny => write!(f, "ANY"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrdinaryToPrimitiveHint {
  kNumber,
  kString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToPrimitiveHint {
  kDefault,
  kNumber,
  kString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreateArgumentsType {
    kMappedArguments,
    kUnmappedArguments,
    kRestParameter,
}

impl std::fmt::Display for CreateArgumentsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateArgumentsType::kMappedArguments => write!(f, "MAPPED_ARGUMENTS"),
            CreateArgumentsType::kUnmappedArguments => write!(f, "UNMAPPED_ARGUMENTS"),
            CreateArgumentsType::kRestParameter => write!(f, "REST_PARAMETER"),
        }
    }
}

pub const K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE: i32 = 75;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeType {
    SCRIPT_SCOPE,
    REPL_MODE_SCOPE,
    CLASS_SCOPE,
    EVAL_SCOPE,
    FUNCTION_SCOPE,
    MODULE_SCOPE,
    CATCH_SCOPE,
    BLOCK_SCOPE,
    WITH_SCOPE,
    SHADOW_REALM_SCOPE,
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeType::SCRIPT_SCOPE => write!(f, "SCRIPT_SCOPE"),
            ScopeType::REPL_MODE_SCOPE => write!(f, "REPL_MODE_SCOPE"),
            ScopeType::CLASS_SCOPE => write!(f, "CLASS_SCOPE"),
            ScopeType::EVAL_SCOPE => write!(f, "EVAL_SCOPE"),
            ScopeType::FUNCTION_SCOPE => write!(f, "FUNCTION_SCOPE"),
            ScopeType::MODULE_SCOPE => write!(f, "MODULE_SCOPE"),
            ScopeType::CATCH_SCOPE => write!(f, "CATCH_SCOPE"),
            ScopeType::BLOCK_SCOPE => write!(f, "BLOCK_SCOPE"),
            ScopeType::WITH_SCOPE => write!(f, "WITH_SCOPE"),
            ScopeType::SHADOW_REALM_SCOPE => write!(f, "SHADOW_REALM_SCOPE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationSiteMode {
    DONT_TRACK_ALLOCATION_SITE,
    TRACK_ALLOCATION_SITE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationSiteUpdateMode {
    kUpdate,
    kCheckOnly,
}

// Little-Endian architecture, as opposed to Big-Endian
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeType {
    DOUBLE(f64),
    INTEGER(i32),
}

pub fn is_little_endian() -> bool {
    let n: u32 = 0x12345678;
    let b: [u8; 4] = unsafe { std::mem::transmute(n) };
    b[0] == 0x78
}

pub const K_HOLE_NAN_UPPER32: u32 = 0xFFF7FFFF;
pub const K_HOLE_NAN_LOWER32: u32 = 0xFFF7FFFF;

// Tagged<T> and Local<T>
struct Tagged<T> {}
struct Local<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeFlushMode {
  kFlushBytecode,
  kFlushBaselineCode,
  kForceFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Executability { NOT_EXECUTABLE, EXECUTABLE };

impl std::fmt::Display for Executability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Executability::NOT_EXECUTABLE => write!(f, "NOT_EXECUTABLE"),
            Executability::EXECUTABLE => write!(f, "EXECUTABLE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptEventType {
    kReserveId,
    kCreate,
    kDeserialize,
    kBackgroundCompile,
    kStreamingCompileBackground,
    kStreamingCompileForeground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Executability { NOT_EXECUTABLE, EXECUTABLE };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NewJSObjectType {
    kNoAPIWrapper,
    kAPIWrapper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeType {
    DOUBLE(f64),
    INTEGER(i32),
}

pub const PROCESSOR_CACHE_LINE_SIZE: i32 = 64;

