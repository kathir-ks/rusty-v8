// Converted from V8 C++ source files:
// Header: wasm-constants.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
pub mod common {
    pub mod globals {}
}
pub mod strings {
    pub mod uri {}
}
pub mod ast {
    pub mod scopes {}
}
pub mod baseline {
    pub mod s390 {
        pub mod baseline_compiler_s390_inl {}
    }
}
pub mod regexp {
    pub mod experimental {
        pub mod experimental_interpreter {}
    }
}
pub mod sandbox {
    pub mod trusted_pointer_table {}
}
pub mod ic {
    pub mod call_optimization {}
}
pub mod execution {
    pub mod isolate {}
}
pub mod init {
    pub mod bootstrapper {}
}
pub mod snapshot {
    pub mod startup_deserializer {}
}
pub mod torque {
    pub mod types {}
    pub mod torque_parser {}
    pub mod type_visitor {}
    pub mod ls {
        pub mod message {}
    }
}
pub mod asmjs {
    pub mod asm_types {}
    pub mod asm_js {}
}
pub mod compiler {
    pub mod js_create_lowering {}
    pub mod persistent_map {}
    pub mod common_operator {}
    pub mod wasm_gc_lowering {}
    pub mod backend {
        pub mod riscv {
            pub mod instruction_selector_riscv64 {}
            pub mod instruction_selector_riscv {}
        }
    }
}
pub mod zone {
    pub mod zone_chunk_list {}
}
pub mod wasm {
    pub mod decoder {}
    pub mod struct_types {}
}
pub mod objects {}
pub mod base {
    pub mod flags {}
}
pub mod d8 {}
pub mod heap {
    pub mod gc_idle_time_handler {}
}
pub mod numbers {}
pub mod util {
    pub mod histograms {}
}
pub mod flags {}
pub mod objects_body_descriptors {}
pub mod strings {
    pub mod string_builder {}
    pub mod unicode_inl {}
}
pub mod external_pointer {}
pub mod threads {}
pub mod common {
    pub mod message_template {}
}
pub mod interpreter {
    pub mod interpreter_intrinsics {}
}
pub mod objects {
    pub mod js_array_buffer {}
}
pub mod v8_array_buffer {}
pub mod compiled_method {}
pub mod accessor_assembler {}
pub mod arm64 {
    pub mod baseline {
        pub mod arm64_baseline {}
    }
}
pub mod s390 {
    pub mod baseline {
        pub mod s390_baseline {}
    }
}
pub mod mips64 {
    pub mod baseline {
        pub mod mips64_baseline {}
    }
}
pub mod wasm {
    // Binary encoding of the module header.
    pub const KWASM_MAGIC: u32 = 0x6d736100;
    pub const KWASM_VERSION: u32 = 0x01;

    // Binary encoding of value and heap types.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum ValueTypeCode {
        // Current value types
        VoidCode = 0x40,
        I32Code = 0x7f,
        I64Code = 0x7e,
        F32Code = 0x7d,
        F64Code = 0x7c,
        S128Code = 0x7b,
        I8Code = 0x78,
        I16Code = 0x77,
        F16Code = 0x76,
        NoExnCode = 0x74,
        NoFuncCode = 0x73,
        NoExternCode = 0x72,
        NoneCode = 0x71,
        FuncRefCode = 0x70,
        ExternRefCode = 0x6f,
        AnyRefCode = 0x6e,
        EqRefCode = 0x6d,
        I31RefCode = 0x6c,
        StructRefCode = 0x6b,
        ArrayRefCode = 0x6a,
        RefCode = 0x64,
        RefNullCode = 0x63,
        ExnRefCode = 0x69,
        ContRefCode = 0x68,
        NoContCode = 0x75,
        StringRefCode = 0x67,
        StringViewWtf8Code = 0x66,
        StringViewWtf16Code = 0x62,
        StringViewIterCode = 0x61,
        FirstHeapTypeCode = StringViewIterCode as u8,
        LastHeapTypeCode = NoContCode as u8,
    }

    // Binary encoding of type definitions.
    pub const KSHARED_FLAG_CODE: u8 = 0x65;
    pub const KWASM_FUNCTION_TYPE_CODE: u8 = 0x60;
    pub const KWASM_STRUCT_TYPE_CODE: u8 = 0x5f;
    pub const KWASM_ARRAY_TYPE_CODE: u8 = 0x5e;
    pub const KWASM_CONT_TYPE_CODE: u8 = 0x5d;
    pub const KWASM_SUBTYPE_CODE: u8 = 0x50;
    pub const KWASM_SUBTYPE_FINAL_CODE: u8 = 0x4f;
    pub const KWASM_RECURSIVE_TYPE_GROUP_CODE: u8 = 0x4e;

    // Binary encoding of import/export kinds.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum ImportExportKindCode {
        ExternalFunction = 0,
        ExternalTable = 1,
        ExternalMemory = 2,
        ExternalGlobal = 3,
        ExternalTag = 4,
    }

    // The limits structure: valid for both memory and table limits.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    // Flags for data and element segments.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum SegmentFlags {
        ActiveNoIndex = 0,
        Passive = 1,
        ActiveWithIndex = 2,
    }

    // Binary encoding of sections identifiers.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(i8)]
    pub enum SectionCode {
        kUnknownSectionCode = 0,
        kTypeSectionCode = 1,
        kImportSectionCode = 2,
        kFunctionSectionCode = 3,
        kTableSectionCode = 4,
        kMemorySectionCode = 5,
        kGlobalSectionCode = 6,
        kExportSectionCode = 7,
        kStartSectionCode = 8,
        kElementSectionCode = 9,
        kCodeSectionCode = 10,
        kDataSectionCode = 11,
        kDataCountSectionCode = 12,
        kTagSectionCode = 13,
        kStringRefSectionCode = 14,

        kNameSectionCode = 100,
        kSourceMappingURLSectionCode = 101,
        kDebugInfoSectionCode = 102,
        kExternalDebugInfoSectionCode = 103,
        kBuildIdSectionCode = 104,
        kInstTraceSectionCode = 105,
        kCompilationHintsSectionCode = 106,
        kBranchHintsSectionCode = 107,
    }

    impl SectionCode {
        pub const KFirstSectionInModule: SectionCode = SectionCode::kTypeSectionCode;
        pub const KLastKnownModuleSection: SectionCode = SectionCode::kStringRefSectionCode;
        pub const KFirstUnorderedSection: SectionCode = SectionCode::kDataCountSectionCode;
    }

    // Binary encoding of compilation hints.
    pub const KDEFAULT_COMPILATION_HINT: u8 = 0x0;
    pub const KNO_COMPILATION_HINT: u8 = u8::MAX;

    // Binary encoding of name section kinds.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum NameSectionKindCode {
        kModuleCode = 0,
        kFunctionCode = 1,
        kLocalCode = 2,
        kLabelCode = 3,
        kTypeCode = 4,
        kTableCode = 5,
        kMemoryCode = 6,
        kGlobalCode = 7,
        kElementSegmentCode = 8,
        kDataSegmentCode = 9,
        kFieldCode = 10,
        kTagCode = 11,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum CatchKind {
        kCatch = 0x0,
        kCatchRef = 0x1,
        kCatchAll = 0x2,
        kCatchAllRef = 0x3,
    }

    impl CatchKind {
        pub const kLastCatchKind: CatchKind = CatchKind::kCatchAllRef;
    }

    pub const KWASM_PAGE_SIZE: usize = 0x10000;
    pub const KWASM_PAGE_SIZE_LOG2: u32 = 16;

    pub type WasmCodePosition = i32;
    pub const KNO_CODE_POSITION: WasmCodePosition = -1;

    pub const KEXCEPTION_ATTRIBUTE: u32 = 0;

    pub const KANONYMOUS_FUNC_INDEX: i32 = -1;

    pub const KINVALID_CANONICAL_INDEX: u32 = u32::MAX;

    pub const KGENERIC_WRAPPER_BUDGET: u32 = 1000;

    pub const KMINIMUM_SUPERTYPE_ARRAY_SIZE: u32 = 3;

    pub const KMAX_POLYMORPHISM: i32 = 4;

    pub const KMAX_STRUCT_FIELD_INDEX_FOR_IMPLICIT_NULL_CHECK: i32 = 4000;
}
