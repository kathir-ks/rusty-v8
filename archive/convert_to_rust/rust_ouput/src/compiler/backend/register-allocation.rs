// Converted from V8 C++ source files:
// Header: register-allocation.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_allocation {
    use crate::codegen::register_configuration::RegisterConfiguration;
    use crate::compiler::backend::code_generator::kDoubleSize;
    use crate::compiler::backend::code_generator::kSimd128Size;
    use crate::compiler::backend::code_generator::kSimd256Size;
    use crate::execution::isolate::kSystemPointerSize;
    use crate::objects::heap_number::UNREACHABLE;
    use std::fmt;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RegisterKind {
        kGeneral,
        kDouble,
        kSimd128,
    }

    impl fmt::Display for RegisterKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RegisterKind::kGeneral => write!(f, "kGeneral"),
                RegisterKind::kDouble => write!(f, "kDouble"),
                RegisterKind::kSimd128 => write!(f, "kSimd128"),
            }
        }
    }

    pub fn get_register_count(config: &RegisterConfiguration, kind: RegisterKind) -> i32 {
        match kind {
            RegisterKind::kGeneral => config.num_general_registers(),
            RegisterKind::kDouble => config.num_double_registers(),
            RegisterKind::kSimd128 => config.num_simd128_registers(),
        }
    }

    pub fn get_allocatable_register_count(
        config: &RegisterConfiguration,
        kind: RegisterKind,
    ) -> i32 {
        match kind {
            RegisterKind::kGeneral => config.num_allocatable_general_registers(),
            RegisterKind::kDouble => config.num_allocatable_double_registers(),
            RegisterKind::kSimd128 => config.num_allocatable_simd128_registers(),
        }
    }

    pub fn get_allocatable_register_codes(
        config: &RegisterConfiguration,
        kind: RegisterKind,
    ) -> &[i32] {
        match kind {
            RegisterKind::kGeneral => config.allocatable_general_codes(),
            RegisterKind::kDouble => config.allocatable_double_codes(),
            RegisterKind::kSimd128 => config.allocatable_simd128_codes(),
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MachineRepresentation {
        kNone,
        kBit,
        kWord8,
        kWord16,
        kWord32,
        kWord64,
        kFloat16,
        kFloat32,
        kFloat64,
        kSimd128,
        kSimd256,
        kTaggedSigned,
        kTaggedPointer,
        kTagged,
        kMapWord,
        kIndirectPointer,
        kCompressedPointer,
        kCompressed,
        kProtectedPointer,
        kFloat16RawBits,
        kSandboxedPointer,
    }

    impl fmt::Display for MachineRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MachineRepresentation::kNone => write!(f, "kNone"),
                MachineRepresentation::kBit => write!(f, "kBit"),
                MachineRepresentation::kWord8 => write!(f, "kWord8"),
                MachineRepresentation::kWord16 => write!(f, "kWord16"),
                MachineRepresentation::kWord32 => write!(f, "kWord32"),
                MachineRepresentation::kWord64 => write!(f, "kWord64"),
                MachineRepresentation::kFloat16 => write!(f, "kFloat16"),
                MachineRepresentation::kFloat32 => write!(f, "kFloat32"),
                MachineRepresentation::kFloat64 => write!(f, "kFloat64"),
                MachineRepresentation::kSimd128 => write!(f, "kSimd128"),
                MachineRepresentation::kSimd256 => write!(f, "kSimd256"),
                MachineRepresentation::kTaggedSigned => write!(f, "kTaggedSigned"),
                MachineRepresentation::kTaggedPointer => write!(f, "kTaggedPointer"),
                MachineRepresentation::kTagged => write!(f, "kTagged"),
                MachineRepresentation::kMapWord => write!(f, "kMapWord"),
                MachineRepresentation::kIndirectPointer => write!(f, "kIndirectPointer"),
                MachineRepresentation::kCompressedPointer => write!(f, "kCompressedPointer"),
                MachineRepresentation::kCompressed => write!(f, "kCompressed"),
                MachineRepresentation::kProtectedPointer => write!(f, "kProtectedPointer"),
                MachineRepresentation::kFloat16RawBits => write!(f, "kFloat16RawBits"),
                MachineRepresentation::kSandboxedPointer => write!(f, "kSandboxedPointer"),
            }
        }
    }

    pub fn byte_width_for_stack_slot(rep: MachineRepresentation) -> usize {
        match rep {
            MachineRepresentation::kBit
            | MachineRepresentation::kWord8
            | MachineRepresentation::kWord16
            | MachineRepresentation::kWord32
            | MachineRepresentation::kFloat16
            | MachineRepresentation::kFloat32
            | MachineRepresentation::kSandboxedPointer => kSystemPointerSize,
            MachineRepresentation::kTaggedSigned
            | MachineRepresentation::kTaggedPointer
            | MachineRepresentation::kTagged
            | MachineRepresentation::kCompressedPointer
            | MachineRepresentation::kCompressed
            | MachineRepresentation::kProtectedPointer => {
                kSystemPointerSize
            }
            MachineRepresentation::kWord64 | MachineRepresentation::kFloat64 => kDoubleSize,
            MachineRepresentation::kSimd128 => kSimd128Size,
            MachineRepresentation::kSimd256 => kSimd256Size,
            MachineRepresentation::kNone
            | MachineRepresentation::kMapWord
            | MachineRepresentation::kIndirectPointer
            | MachineRepresentation::kFloat16RawBits => {
                panic!("UNREACHABLE");
            }
        }
    }
}

