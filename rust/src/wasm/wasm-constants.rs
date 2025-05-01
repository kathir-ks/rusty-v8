// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This code should only be included if WebAssembly is enabled.");

/// Binary encoding of the module header.
pub const KWASM_MAGIC: u32 = 0x6d736100;
pub const KWASM_VERSION: u32 = 0x01;

/// Binary encoding of value and heap types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueTypeCode {
  /// Current value types
  Void = 0x40,
  I32 = 0x7f, // -0x01
  I64 = 0x7e, // -0x02
  F32 = 0x7d, // -0x03
  F64 = 0x7c, // -0x04
  S128 = 0x7b, // -0x05
  I8 = 0x78,   // -0x08, packed type
  I16 = 0x77,  // -0x09, packed type
  F16 = 0x76,  // -0x0a, packed type
  NoExn = 0x74, // -0x0c
  NoFunc = 0x73, // -0x0d
  NoExtern = 0x72, // -0x0e
  None = 0x71, // -0x0f
  FuncRef = 0x70, // -0x10
  ExternRef = 0x6f, // -0x11
  AnyRef = 0x6e, // -0x12
  EqRef = 0x6d, // -0x13
  I31Ref = 0x6c, // -0x14
  StructRef = 0x6b, // -0x15
  ArrayRef = 0x6a, // -0x16
  Ref = 0x64, // -0x1c
  RefNull = 0x63, // -0x1d
  // Non-finalized proposals below.
  ExnRef = 0x69, // -0x17
  ContRef = 0x68, // -0x18
  NoCont = 0x75, // -0x0b
  StringRef = 0x67, // -0x19
  StringViewWtf8 = 0x66, // -0x1a
  StringViewWtf16 = 0x62, // -0x1e
  StringViewIter = 0x61, // -0x1f
}

impl ValueTypeCode {
  // For decoding, we build an array for all heap types with these bounds:
  pub const FIRST_HEAP_TYPE_CODE: ValueTypeCode = ValueTypeCode::StringViewIter; // Lowest assigned code.
  pub const LAST_HEAP_TYPE_CODE: ValueTypeCode = ValueTypeCode::NoCont; // Highest assigned code.
}

/// Binary encoding of type definitions.
pub const KSHARED_FLAG_CODE: u8 = 0x65;
pub const KWASM_FUNCTION_TYPE_CODE: u8 = 0x60;
pub const KWASM_STRUCT_TYPE_CODE: u8 = 0x5f;
pub const KWASM_ARRAY_TYPE_CODE: u8 = 0x5e;
pub const KWASM_CONT_TYPE_CODE: u8 = 0x5d;
pub const KWASM_SUBTYPE_CODE: u8 = 0x50;
pub const KWASM_SUBTYPE_FINAL_CODE: u8 = 0x4f;
pub const KWASM_RECURSIVE_TYPE_GROUP_CODE: u8 = 0x4e;

/// Binary encoding of import/export kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImportExportKindCode {
  Function = 0,
  Table = 1,
  Memory = 2,
  Global = 3,
  Tag = 4,
}

/// The limits structure: valid for both memory and table limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LimitsFlags {
  NoMaximum = 0x00,
  WithMaximum = 0x01,
  SharedNoMaximum = 0x02,
  SharedWithMaximum = 0x03,
  Memory64NoMaximum = 0x04,
  Memory64WithMaximum = 0x05,
  Memory64SharedNoMaximum = 0x06,
  Memory64SharedWithMaximum = 0x07,
}

/// Flags for data and element segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SegmentFlags {
  ActiveNoIndex = 0,    // Active segment with a memory/table index of zero.
  Passive = 1,          // Passive segment.
  ActiveWithIndex = 2,  // Active segment with a given memory/table index.
}

/// Binary encoding of sections identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum SectionCode {
  UnknownSectionCode = 0,     // code for unknown sections
  TypeSectionCode = 1,        // Function signature declarations
  ImportSectionCode = 2,      // Import declarations
  FunctionSectionCode = 3,    // Function declarations
  TableSectionCode = 4,       // Indirect function table and others
  MemorySectionCode = 5,      // Memory attributes
  GlobalSectionCode = 6,      // Global declarations
  ExportSectionCode = 7,      // Exports
  StartSectionCode = 8,       // Start function declaration
  ElementSectionCode = 9,     // Elements section
  CodeSectionCode = 10,       // Function code
  DataSectionCode = 11,       // Data segments
  DataCountSectionCode = 12,  // Number of data segments
  TagSectionCode = 13,        // Tag section
  StringRefSectionCode = 14,  // Stringref literal section

  // The following sections are custom sections, and are identified using a
  // string rather than an integer. Their enumeration values are not guaranteed
  // to be consistent.
  NameSectionCode = 15,               // Name section (encoded as a string)
  SourceMappingURLSectionCode = 16,   // Source Map URL section
  DebugInfoSectionCode = 17,          // DWARF section .debug_info
  ExternalDebugInfoSectionCode = 18,  // Section encoding the external symbol path
  BuildIdSectionCode = 19,            // Unique build id to match the symbol file
  InstTraceSectionCode = 20,          // Instruction trace section
  CompilationHintsSectionCode = 21,   // Compilation hints section
  BranchHintsSectionCode = 22,        // Branch hints section
}

impl SectionCode {
  /// Helper values
  pub const FIRST_SECTION_IN_MODULE: SectionCode = SectionCode::TypeSectionCode;
  pub const LAST_KNOWN_MODULE_SECTION: SectionCode = SectionCode::StringRefSectionCode;
  //pub const FIRST_UNORDERED_SECTION: SectionCode = SectionCode::DataCountSectionCode; //TODO(you): Check if this needs to be defined elsewhere/differently
}

/// Binary encoding of compilation hints.
pub const KDEFAULT_COMPILATION_HINT: u8 = 0x0;
pub const KNO_COMPILATION_HINT: u8 = u8::MAX;

/// Binary encoding of name section kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NameSectionKindCode {
  Module = 0,
  Function = 1,
  Local = 2,
  // https://github.com/WebAssembly/extended-name-section/
  Label = 3,
  Type = 4,
  Table = 5,
  Memory = 6,
  Global = 7,
  ElementSegment = 8,
  DataSegment = 9,
  // https://github.com/WebAssembly/gc/issues/193
  Field = 10,
  // https://github.com/WebAssembly/exception-handling/pull/213
  Tag = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CatchKind {
  Catch = 0x0,
  CatchRef = 0x1,
  CatchAll = 0x2,
  CatchAllRef = 0x3,
  LastCatchKind = CatchKind::CatchAllRef,
}

pub const KWASM_PAGE_SIZE: usize = 0x10000;
pub const KWASM_PAGE_SIZE_LOG2: u32 = 16;

//static_assert!(KWASM_PAGE_SIZE == size_t{1} << KWASM_PAGE_SIZE_LOG2, "consistency");
//TODO(you): Figure out how to assert this kind of condition

/// TODO(wasm): Wrap WasmCodePosition in a struct.
pub type WasmCodePosition = i32;
pub const KNO_CODE_POSITION: WasmCodePosition = -1;

pub const KEXCEPTION_ATTRIBUTE: u32 = 0;

pub const KANONYMOUS_FUNC_INDEX: i32 = -1;

// This needs to survive round-tripping through a Smi without changing
// its value.
pub const KINVALID_CANONICAL_INDEX: u32 = u32::MAX;

//static_assert!(static_cast<uint32_t>(Internals::SmiValue(Internals::IntToSmi(
//                 static_cast<int>(KINVALID_CANONICAL_INDEX)))) ==
//              KINVALID_CANONICAL_INDEX);
//TODO(you): Figure out how to assert this kind of condition

/// The number of calls to an exported Wasm function that will be handled
/// by the generic wrapper. Once the budget is exhausted, a specific wrapper
/// is to be compiled for the function's signature.
/// The abstract goal of the tiering strategy for the js-to-wasm wrappers is to
/// use the generic wrapper as much as possible (less space, no need to compile),
/// but fall back to compiling a specific wrapper for any function (signature)
/// that is used often enough for the generic wrapper's small execution penalty
/// to start adding up.
/// So, when choosing a value for the initial budget, we are interested in a
/// value that skips on tiering up functions that are called only a few times and
/// the tier-up only wastes resources, but triggers compilation of specific
/// wrappers early on for those functions that have the potential to be called
/// often enough.
pub const KGENERIC_WRAPPER_BUDGET: u32 = 1000;

/// The minimum length of supertype arrays for wasm-gc types. Having a size > 0
/// gives up some module size for faster access to the supertypes.
pub const KMINIMUM_SUPERTYPE_ARRAY_SIZE: u32 = 3;

/// Maximum number of call targets tracked per call.
pub const KMAX_POLYMORPHISM: i32 = 4;

/// A struct field beyond this limit needs an explicit null check (trapping null
/// access not guaranteed to behave properly).
pub const KMAX_STRUCT_FIELD_INDEX_FOR_IMPLICIT_NULL_CHECK: i32 = 4000;

// #if V8_TARGET_ARCH_X64
// constexpr int32_t kOSRTargetOffset = 4 * kSystemPointerSize;
// #endif
//TODO(you): conditionally define this based on architecture?  Need to know what kSystemPointerSize is
#[cfg(target_arch = "x86_64")]
pub const KOSR_TARGET_OFFSET: i32 = 4 * 8; // Assuming kSystemPointerSize is 8 for x64