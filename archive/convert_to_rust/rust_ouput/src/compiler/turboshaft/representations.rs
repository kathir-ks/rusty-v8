// Converted from V8 C++ source files:
// Header: representations.h
// Implementation: representations.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod hashing {
        pub fn hash<T>(_value: &T, _seed: u32) -> usize {
            0
        }
    }
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK {
            ($condition:expr) => {
                if !$condition {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
        }
    }
}

pub mod codegen {
    pub enum MachineType {
        Int8,
        Uint8,
        Int16,
        Uint16,
        Int32,
        Uint32,
        Int64,
        Uint64,
        Float16,
        Float32,
        Float64,
        AnyTagged,
        TaggedPointer,
        TaggedSigned,
        ProtectedPointer,
        IndirectPointer,
        SandboxedPointer,
        Simd128,
        Simd256,
        None,
        Bit,
        CompressedPointer,
        Compressed,
        MapWord,
        Float16RawBits,
    }

    impl MachineType {
        pub fn representation(&self) -> super::compiler::turboshaft::MachineRepresentation {
            match self {
                MachineType::Int8 | MachineType::Uint8 => {
                    super::compiler::turboshaft::MachineRepresentation::kWord8
                }
                MachineType::Int16 | MachineType::Uint16 => {
                    super::compiler::turboshaft::MachineRepresentation::kWord16
                }
                MachineType::Int32 | MachineType::Uint32 => {
                    super::compiler::turboshaft::MachineRepresentation::kWord32
                }
                MachineType::Int64 | MachineType::Uint64 => {
                    super::compiler::turboshaft::MachineRepresentation::kWord64
                }
                MachineType::Float16 => {
                    super::compiler::turboshaft::MachineRepresentation::kFloat16
                }
                MachineType::Float32 => {
                    super::compiler::turboshaft::MachineRepresentation::kFloat32
                }
                MachineType::Float64 => {
                    super::compiler::turboshaft::MachineRepresentation::kFloat64
                }
                MachineType::AnyTagged => {
                    super::compiler::turboshaft::MachineRepresentation::kTagged
                }
                MachineType::TaggedPointer => {
                    super::compiler::turboshaft::MachineRepresentation::kTaggedPointer
                }
                MachineType::TaggedSigned => {
                    super::compiler::turboshaft::MachineRepresentation::kTaggedSigned
                }
                MachineType::ProtectedPointer => {
                    super::compiler::turboshaft::MachineRepresentation::kProtectedPointer
                }
                MachineType::IndirectPointer => {
                    super::compiler::turboshaft::MachineRepresentation::kIndirectPointer
                }
                MachineType::SandboxedPointer => {
                    super::compiler::turboshaft::MachineRepresentation::kSandboxedPointer
                }
                MachineType::Simd128 => {
                    super::compiler::turboshaft::MachineRepresentation::kSimd128
                }
                MachineType::Simd256 => {
                    super::compiler::turboshaft::MachineRepresentation::kSimd256
                }
                _ => {
                    super::compiler::turboshaft::MachineRepresentation::kNone
                }
            }
        }
        pub fn IsSigned(&self) -> bool {
            match self {
                MachineType::Int8 | MachineType::Int16 | MachineType::Int32 | MachineType::Int64 => true,
                _ => false,
            }
        }
    }
}

pub mod compiler {
    pub mod turboshaft {
        use std::{
            fmt,
            fmt::{Display, Formatter},
            marker::Copy,
        };

        use crate::codegen::MachineType;

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            kProtectedPointer,
            kIndirectPointer,
            kSandboxedPointer,
            kCompressedPointer,
            kCompressed,
            kMapWord,
            kFloat16RawBits,
        }

        const kSystemPointerSize: u16 = 8;
        const kTaggedSize: u16 = 8;
        const kInt64Size: u16 = 8;
        const COMPRESS_POINTERS_BOOL: bool = false;
        const V8_MAP_PACKING_BOOL: bool = false;
        const Is64Arch: bool = true;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct MaybeRegisterRepresentation {
            value_: MaybeRegisterRepresentationEnum,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MaybeRegisterRepresentationEnum {
            kWord32,
            kWord64,
            kFloat32,
            kFloat64,
            kTagged,
            kCompressed,
            kSimd128,
            kSimd256,
            kNone,
            kInvalid,
        }

        impl MaybeRegisterRepresentation {
            pub const fn new(value: MaybeRegisterRepresentationEnum) -> Self {
                Self { value_: value }
            }

            pub const fn invalid() -> Self {
                Self {
                    value_: MaybeRegisterRepresentationEnum::kInvalid,
                }
            }

            pub const fn is_valid(&self) -> bool {
                self.value_ != MaybeRegisterRepresentationEnum::kInvalid
            }

            pub const fn value(&self) -> MaybeRegisterRepresentationEnum {
                crate::base::logging::DCHECK!(self.is_valid());
                self.value_
            }

            pub const fn Word32() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kWord32)
            }

            pub const fn Word64() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kWord64)
            }

            pub const fn WordPtr() -> Self {
                if kSystemPointerSize == 4 {
                    Self::Word32()
                } else {
                    crate::base::logging::DCHECK!(kSystemPointerSize == 8);
                    Self::Word64()
                }
            }

            pub const fn Float32() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kFloat32)
            }

            pub const fn Float64() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kFloat64)
            }

            pub const fn Tagged() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kTagged)
            }

            pub const fn Compressed() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kCompressed)
            }

            pub const fn Simd128() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kSimd128)
            }

            pub const fn Simd256() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kSimd256)
            }

            pub const fn None() -> Self {
                Self::new(MaybeRegisterRepresentationEnum::kNone)
            }

            pub const fn IsWord(&self) -> bool {
                match self.value_ {
                    MaybeRegisterRepresentationEnum::kWord32 | MaybeRegisterRepresentationEnum::kWord64 => true,
                    MaybeRegisterRepresentationEnum::kFloat32
                    | MaybeRegisterRepresentationEnum::kFloat64
                    | MaybeRegisterRepresentationEnum::kTagged
                    | MaybeRegisterRepresentationEnum::kCompressed
                    | MaybeRegisterRepresentationEnum::kSimd128
                    | MaybeRegisterRepresentationEnum::kSimd256
                    | MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub const fn IsFloat(&self) -> bool {
                match self.value_ {
                    MaybeRegisterRepresentationEnum::kFloat32 | MaybeRegisterRepresentationEnum::kFloat64 => true,
                    MaybeRegisterRepresentationEnum::kWord32
                    | MaybeRegisterRepresentationEnum::kWord64
                    | MaybeRegisterRepresentationEnum::kTagged
                    | MaybeRegisterRepresentationEnum::kCompressed
                    | MaybeRegisterRepresentationEnum::kSimd128
                    | MaybeRegisterRepresentationEnum::kSimd256
                    | MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub const fn IsTaggedOrCompressed(&self) -> bool {
                match self.value_ {
                    MaybeRegisterRepresentationEnum::kTagged | MaybeRegisterRepresentationEnum::kCompressed => true,
                    MaybeRegisterRepresentationEnum::kWord32
                    | MaybeRegisterRepresentationEnum::kWord64
                    | MaybeRegisterRepresentationEnum::kFloat32
                    | MaybeRegisterRepresentationEnum::kFloat64
                    | MaybeRegisterRepresentationEnum::kSimd128
                    | MaybeRegisterRepresentationEnum::kSimd256
                    | MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub fn MaxUnsignedValue(&self) -> u64 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => u32::MAX as u64,
                    MaybeRegisterRepresentationEnum::kWord64 => u64::MAX,
                    MaybeRegisterRepresentationEnum::kFloat32
                    | MaybeRegisterRepresentationEnum::kFloat64
                    | MaybeRegisterRepresentationEnum::kTagged
                    | MaybeRegisterRepresentationEnum::kCompressed
                    | MaybeRegisterRepresentationEnum::kSimd128
                    | MaybeRegisterRepresentationEnum::kSimd256
                    | MaybeRegisterRepresentationEnum::kNone => panic!("UNREACHABLE"),
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("Invalid"),
                }
            }

            pub fn machine_representation(&self) -> MachineRepresentation {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => MachineRepresentation::kWord32,
                    MaybeRegisterRepresentationEnum::kWord64 => MachineRepresentation::kWord64,
                    MaybeRegisterRepresentationEnum::kFloat32 => MachineRepresentation::kFloat32,
                    MaybeRegisterRepresentationEnum::kFloat64 => MachineRepresentation::kFloat64,
                    MaybeRegisterRepresentationEnum::kTagged => MachineRepresentation::kTagged,
                    MaybeRegisterRepresentationEnum::kCompressed => MachineRepresentation::kCompressed,
                    MaybeRegisterRepresentationEnum::kSimd128 => MachineRepresentation::kSimd128,
                    MaybeRegisterRepresentationEnum::kSimd256 => MachineRepresentation::kSimd256,
                    MaybeRegisterRepresentationEnum::kNone => panic!("UNREACHABLE"),
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("Invalid"),
                }
            }

            pub const fn bit_width(&self) -> u16 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => 32,
                    MaybeRegisterRepresentationEnum::kWord64 => 64,
                    MaybeRegisterRepresentationEnum::kFloat32 => 32,
                    MaybeRegisterRepresentationEnum::kFloat64 => 64,
                    MaybeRegisterRepresentationEnum::kTagged => kSystemPointerSize,
                    MaybeRegisterRepresentationEnum::kCompressed => kSystemPointerSize,
                    MaybeRegisterRepresentationEnum::kSimd128 => 128,
                    MaybeRegisterRepresentationEnum::kSimd256 => 256,
                    MaybeRegisterRepresentationEnum::kNone => panic!("UNREACHABLE"),
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("Invalid"),
                }
            }
        }

        impl From<MaybeRegisterRepresentationEnum> for MaybeRegisterRepresentation {
            fn from(value: MaybeRegisterRepresentationEnum) -> Self {
                MaybeRegisterRepresentation::new(value)
            }
        }

        impl From<MaybeRegisterRepresentation> for MaybeRegisterRepresentationEnum {
            fn from(rep: MaybeRegisterRepresentation) -> Self {
                rep.value()
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct RegisterRepresentation {
            maybe_rep: MaybeRegisterRepresentation,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum RegisterRepresentationEnum {
            kWord32 = MaybeRegisterRepresentationEnum::kWord32 as isize,
            kWord64 = MaybeRegisterRepresentationEnum::kWord64 as isize,
            kFloat32 = MaybeRegisterRepresentationEnum::kFloat32 as isize,
            kFloat64 = MaybeRegisterRepresentationEnum::kFloat64 as isize,
            kTagged = MaybeRegisterRepresentationEnum::kTagged as isize,
            kCompressed = MaybeRegisterRepresentationEnum::kCompressed as isize,
            kSimd128 = MaybeRegisterRepresentationEnum::kSimd128 as isize,
            kSimd256 = MaybeRegisterRepresentationEnum::kSimd256 as isize,
        }

        impl RegisterRepresentation {
            pub const fn new(value: RegisterRepresentationEnum) -> Self {
                Self {
                    maybe_rep: MaybeRegisterRepresentation::new(unsafe {
                        std::mem::transmute(value as u8)
                    }),
                }
            }

            pub const fn default() -> Self {
                Self {
                    maybe_rep: MaybeRegisterRepresentation::invalid(),
                }
            }

            pub const fn from_maybe(rep: MaybeRegisterRepresentation) -> Self {
                Self { maybe_rep: rep }
            }

            pub const fn value(&self) -> RegisterRepresentationEnum {
                unsafe { std::mem::transmute(self.maybe_rep.value() as u8) }
            }

            pub const fn Word32() -> Self {
                Self::new(RegisterRepresentationEnum::kWord32)
            }

            pub const fn Word64() -> Self {
                Self::new(RegisterRepresentationEnum::kWord64)
            }

            pub const fn WordPtr() -> Self {
                Self::from_maybe(MaybeRegisterRepresentation::WordPtr())
            }

            pub const fn Float32() -> Self {
                Self::new(RegisterRepresentationEnum::kFloat32)
            }

            pub const fn Float64() -> Self {
                Self::new(RegisterRepresentationEnum::kFloat64)
            }

            pub const fn Tagged() -> Self {
                Self::new(RegisterRepresentationEnum::kTagged)
            }

            pub const fn Compressed() -> Self {
                Self::new(RegisterRepresentationEnum::kCompressed)
            }

            pub const fn Simd128() -> Self {
                Self::new(RegisterRepresentationEnum::kSimd128)
            }

            pub const fn Simd256() -> Self {
                Self::new(RegisterRepresentationEnum::kSimd256)
            }

            pub const fn from_machine_representation(rep: MachineRepresentation) -> Self {
                match rep {
                    MachineRepresentation::kBit
                    | MachineRepresentation::kWord8
                    | MachineRepresentation::kWord16
                    | MachineRepresentation::kWord32 => Self::Word32(),
                    MachineRepresentation::kWord64 => Self::Word64(),
                    MachineRepresentation::kTaggedSigned
                    | MachineRepresentation::kTaggedPointer
                    | MachineRepresentation::kTagged
                    | MachineRepresentation::kProtectedPointer => Self::Tagged(),
                    MachineRepresentation::kCompressedPointer | MachineRepresentation::kCompressed => {
                        Self::Compressed()
                    }
                    MachineRepresentation::kFloat16 | MachineRepresentation::kFloat32 => Self::Float32(),
                    MachineRepresentation::kFloat64 => Self::Float64(),
                    MachineRepresentation::kSimd128 => Self::Simd128(),
                    MachineRepresentation::kSimd256 => Self::Simd256(),
                    MachineRepresentation::kMapWord => {
                        crate::base::logging::DCHECK!(!V8_MAP_PACKING_BOOL);
                        Self::Tagged()
                    }
                    MachineRepresentation::kIndirectPointer | MachineRepresentation::kSandboxedPointer => {
                        Self::WordPtr()
                    }
                    MachineRepresentation::kNone | MachineRepresentation::kFloat16RawBits => panic!("UNREACHABLE"),
                }
            }

            pub const fn from_machine_type(type_: MachineType) -> Self {
                Self::from_machine_representation(type_.representation())
            }
            pub fn from_ctype_info(
                t: CTypeInfo,
                int64_repr: CFunctionInfoInt64Representation,
            ) -> Self {
                if t.GetType() == CTypeInfoType::kVoid || t.GetType() == CTypeInfoType::kPointer {
                    RegisterRepresentation::Tagged()
                } else if t.GetType() == CTypeInfoType::kInt64 || t.GetType() == CTypeInfoType::kUint64 {
                    if int64_repr == CFunctionInfoInt64Representation::kBigInt {
                        RegisterRepresentation::Word64()
                    } else {
                        crate::base::logging::DCHECK_EQ!(
                            int64_repr,
                            CFunctionInfoInt64Representation::kNumber
                        );
                        RegisterRepresentation::Float64()
                    }
                } else {
                    RegisterRepresentation::from_machine_type(MachineType::TypeForCType(t))
                }
            }

            pub const fn AllowImplicitRepresentationChangeTo(
                &self,
                dst_rep: RegisterRepresentation,
                graph_created_from_turbofan: bool,
            ) -> bool {
                if self == &dst_rep {
                    return true;
                }
                match dst_rep.value() {
                    RegisterRepresentationEnum::kWord32 => {
                        if self == &any_of(
                            RegisterRepresentation::Tagged(),
                            RegisterRepresentation::Compressed(),
                        ) {
                            return true;
                        }
                        if graph_created_from_turbofan && self == &RegisterRepresentation::Word64() {
                            return true;
                        }
                        false
                    }
                    RegisterRepresentationEnum::kWord64 => {
                        if kTaggedSize == kInt64Size && self == &RegisterRepresentation::Tagged() {
                            return true;
                        }
                        false
                    }
                    RegisterRepresentationEnum::kTagged => {
                        if self == &RegisterRepresentation::WordPtr() {
                            return true;
                        }
                        false
                    }
                    RegisterRepresentationEnum::kCompressed => {
                        if self
                            == &any_of(
                                RegisterRepresentation::Tagged(),
                                RegisterRepresentation::WordPtr(),
                                RegisterRepresentation::Word32(),
                            )
                        {
                            return true;
                        }
                        false
                    }
                    _ => false,
                }
            }

            pub const fn MapTaggedToWord(&self) -> Self {
                if self.value() == RegisterRepresentationEnum::kTagged {
                    if COMPRESS_POINTERS_BOOL {
                        RegisterRepresentation::Word32()
                    } else {
                        RegisterRepresentation::WordPtr()
                    }
                } else {
                    *self
                }
            }
        }

        const fn any_of(a: RegisterRepresentation, b: RegisterRepresentation) -> RegisterRepresentation {
            if a == RegisterRepresentation::Tagged() {
                return RegisterRepresentation::Tagged();
            } else if b == RegisterRepresentation::Tagged() {
                return RegisterRepresentation::Tagged();
            }
            RegisterRepresentation::Tagged()
        }

        impl From<RegisterRepresentationEnum> for RegisterRepresentation {
            fn from(value: RegisterRepresentationEnum) -> Self {
                RegisterRepresentation::new(value)
            }
        }

        impl From<RegisterRepresentation> for RegisterRepresentationEnum {
            fn from(rep: RegisterRepresentation) -> Self {
                rep.value()
            }
        }

        impl From<MaybeRegisterRepresentation> for RegisterRepresentation {
            fn from(rep: MaybeRegisterRepresentation) -> Self {
                RegisterRepresentation::from_maybe(rep)
            }
        }

        impl Display for MaybeRegisterRepresentation {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => write!(f, "Word32"),
                    MaybeRegisterRepresentationEnum::kWord64 => write!(f, "Word64"),
                    MaybeRegisterRepresentationEnum::kFloat32 => write!(f, "Float32"),
                    MaybeRegisterRepresentationEnum::kFloat64 => write!(f, "Float64"),
                    MaybeRegisterRepresentationEnum::kTagged => write!(f, "Tagged"),
                    MaybeRegisterRepresentationEnum::kCompressed => write!(f, "Compressed"),
                    MaybeRegisterRepresentationEnum::kSimd128 => write!(f, "Simd128"),
                    MaybeRegisterRepresentationEnum::kSimd256 => write!(f, "Simd256"),
                    MaybeRegisterRepresentationEnum::kNone => write!(f, "None"),
                    MaybeRegisterRepresentationEnum::kInvalid => write!(f, "Invalid"),
                }
            }
        }

        impl Display for RegisterRepresentation {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.maybe_rep)
            }
        }

        struct MultiSwitch<T, Enable = ()> {
            _phantom: std::marker::PhantomData<T>,
            _phantom_enable: std::marker::PhantomData<Enable>,
        }

        trait Enable {}

        impl Enable for () {}

        impl MultiSwitch<RegisterRepresentation> {
            const max_value: u64 = 8;

            fn encode(rep: RegisterRepresentation) -> u64 {
                let value = rep.value() as u64;
                crate::base::logging::DCHECK!(value < Self::max_value);
                value
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct WordRepresentation {
            register_rep: RegisterRepresentation,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum WordRepresentationEnum {
            kWord32 = RegisterRepresentationEnum::kWord32 as isize,
            kWord64 = RegisterRepresentationEnum::kWord64 as isize,
        }

        impl WordRepresentation {
            pub const fn new(value: WordRepresentationEnum) -> Self {
                Self {
                    register_rep: RegisterRepresentation::new(unsafe {
                        std::mem::transmute(value as u8)
                    }),
                }
            }

            pub const fn default() -> Self {
                Self {
                    register_rep: RegisterRepresentation::default(),
                }
            }

            pub const fn from_register(rep: RegisterRepresentation) -> Self {
                crate::base::logging::DCHECK!(rep.maybe_rep.IsWord());
                Self { register_rep: rep }
            }

            pub const fn Word32() -> Self {
                Self::new(WordRepresentationEnum::kWord32)
            }

            pub const fn Word64() -> Self {
                Self::new(WordRepresentationEnum::kWord64)
            }

            pub const fn WordPtr() -> Self {
                Self::from_register(RegisterRepresentation::WordPtr())
            }

            pub const fn value(&self) -> WordRepresentationEnum {
                unsafe { std::mem::transmute(self.register_rep.value() as u8) }
            }

            pub fn MaxUnsignedValue(&self) -> u64 {
                match self.value() {
                    WordRepresentationEnum::kWord32 => u32::MAX as u64,
                    WordRepresentationEnum::kWord64 => u64::MAX,
                }
            }

            pub fn MinSignedValue(&self) -> i64 {
                match self.value() {
                    WordRepresentationEnum::kWord32 => i32::MIN as i64,
                    WordRepresentationEnum::kWord64 => i64::MIN,
                }
            }

            pub fn MaxSignedValue(&self) -> i64 {
                match self.value() {
                    WordRepresentationEnum::kWord32 => i32::MAX as i64,
                    WordRepresentationEnum::kWord64 => i64::MAX,
                }
            }
        }

        impl From<WordRepresentationEnum> for WordRepresentation {
            fn from(value: WordRepresentationEnum) -> Self {
                WordRepresentation::new(value)
            }
        }

        impl From<WordRepresentation> for WordRepresentationEnum {
            fn from(rep: WordRepresentation) -> Self {
                rep.value()
            }
        }

        impl From<RegisterRepresentation> for WordRepresentation {
            fn from(rep: RegisterRepresentation) -> Self {
                WordRepresentation::from_register(rep)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct FloatRepresentation {
            register_rep: RegisterRepresentation,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum FloatRepresentationEnum {
            kFloat32 = RegisterRepresentationEnum::kFloat32 as isize,
            kFloat64 = RegisterRepresentationEnum::kFloat64 as isize,
        }

        impl FloatRepresentation {
            pub const fn new(value: FloatRepresentationEnum) -> Self {
                Self {
                    register_rep: RegisterRepresentation::new(unsafe {
                        std::mem::transmute(value as u8)
                    }),
                }
            }

            pub const fn from_register(rep: RegisterRepresentation) -> Self {
                crate::base::logging::DCHECK!(rep.maybe_rep.IsFloat());
                Self { register_rep: rep }
            }

            pub const fn Float32() -> Self {
                Self::new(FloatRepresentationEnum::kFloat32)
            }

            pub const fn Float64() -> Self {
                Self::new(FloatRepresentationEnum::kFloat64)
            }

            pub const fn value(&self) -> FloatRepresentationEnum {
                unsafe { std::mem::transmute(self.register_rep.value() as u8) }
            }
        }

        impl From<FloatRepresentationEnum> for FloatRepresentation {
            fn from(value: FloatRepresentationEnum) -> Self {
                FloatRepresentation::new(value)
            }
        }

        impl From<FloatRepresentation> for FloatRepresentationEnum {
            fn from(rep: FloatRepresentation) -> Self {
                rep.value()
            }
        }

        impl From<RegisterRepresentation> for FloatRepresentation {
            fn from(rep: RegisterRepresentation) -> Self {
                FloatRepresentation::from_register(rep)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct MemoryRepresentation {
            value_: MemoryRepresentationEnum,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MemoryRepresentationEnum {
            kInt8,
            kUint8,
            kInt16,
            kUint16,
            kInt32,
            kUint32,
            kInt64,
            kUint64,
            kFloat16,
            kFloat32,
            kFloat64,
            kAnyTagged,
            kTaggedPointer,
            kTaggedSigned,
            kAnyUncompressedTagged,
            kUncompressedTaggedPointer,
            kUncompressedTaggedSigned,
            kProtectedPointer,
            kIndirectPointer,
            kSandboxedPointer,
            kSimd128,
            kSimd256,
            kInvalid,
        }

        const kTaggedSizeLog2: u8 = 3;
        const kSystemPointerSizeLog2: u8 = 3;

        impl MemoryRepresentation {
            pub const fn new(value: MemoryRepresentationEnum) -> Self {
                Self { value_: value }
            }

            pub const fn invalid() -> Self {
                Self {
                    value_: MemoryRepresentationEnum::kInvalid,
                }
            }

            pub const fn is_valid(&self) -> bool {
                self.value_ != MemoryRepresentationEnum::kInvalid
            }

            pub const fn value(&self) -> MemoryRepresentationEnum {
                crate::base::logging::DCHECK!(self.is_valid());
                self.value_
            }

            pub const fn Int8() -> Self {
                Self::new(MemoryRepresentationEnum::kInt8)
            }
            pub const fn Uint8() -> Self {
                Self::new(MemoryRepresentationEnum::kUint8)
            }
            pub const fn Int16() -> Self {
                Self::new(MemoryRepresentationEnum::kInt16)
            }
            pub const fn Uint16() -> Self {
                Self::new(MemoryRepresentationEnum::kUint16)
            }
            pub const fn Int32() -> Self {
                Self::new(MemoryRepresentationEnum::kInt32)
            }
            pub const fn Uint32() -> Self {
                Self::new(MemoryRepresentationEnum::kUint32)
            }
            pub const fn Int64() -> Self {
                Self::new(MemoryRepresentationEnum::kInt64)
            }
            pub const fn Uint64() -> Self {
                Self::new(MemoryRepresentationEnum::kUint64)
            }
            pub const fn UintPtr() -> Self {
                if Is64Arch {
                    Self::Uint64()
                } else {
                    Self::Uint32()
                }
            }
            pub const fn Float16() -> Self {
                Self::new(MemoryRepresentationEnum::kFloat16)
            }
            pub const fn Float32() -> Self {
                Self::new(MemoryRepresentationEnum::kFloat32)
            }
            pub const fn Float64() -> Self {
                Self::new(MemoryRepresentationEnum::kFloat64)
            }
            pub const fn AnyTagged() -> Self {
                Self::new(MemoryRepresentationEnum::kAnyTagged)
            }
            pub const fn TaggedPointer() -> Self {
                Self::new(MemoryRepresentationEnum::kTaggedPointer)
            }
            pub const fn TaggedSigned() -> Self {
                Self::new(MemoryRepresentationEnum::kTaggedSigned)
            }
            pub const fn AnyUncompressedTagged() -> Self {
                Self::new(MemoryRepresentationEnum::kAnyUncompressedTagged)
            }
            pub const fn UncompressedTaggedPointer() -> Self {
                Self::new(MemoryRepresentationEnum::kUncompressedTaggedPointer)
            }
            pub const fn UncompressedTaggedSigned() -> Self {
                Self::new(MemoryRepresentationEnum::kUncompressedTaggedSigned)
            }
            pub const fn ProtectedPointer() -> Self {
                Self::new(MemoryRepresentationEnum::kProtectedPointer)
            }
            pub const fn IndirectPointer() -> Self {
                Self::new(MemoryRepresentationEnum::kIndirectPointer)
            }
            pub const fn SandboxedPointer() -> Self {
                Self::new(MemoryRepresentationEnum::kSandboxedPointer)
            }
            pub const fn Simd128() -> Self {
                Self::new(MemoryRepresentationEnum::kSimd128)
            }
            pub const fn Simd256() -> Self {
                Self::new(MemoryRepresentationEnum::kSimd256)
            }

            pub fn IsSigned(&self) -> bool {
                match self.value() {
                    MemoryRepresentationEnum::kInt8
                    | MemoryRepresentationEnum::kInt16
                    | MemoryRepresentationEnum::kInt32
                    | MemoryRepresentationEnum::kInt64 => true,
                    MemoryRepresentationEnum::kUint8
                    | MemoryRepresentationEnum::kUint16
                    | MemoryRepresentationEnum::kUint32
                    | MemoryRepresentationEnum::kUint64 => false,
                    MemoryRepresentationEnum::kFloat16
                    | MemoryRepresentationEnum::kFloat32
                    | MemoryRepresentationEnum::kFloat64
                    | MemoryRepresentationEnum::kAnyTagged
                    | MemoryRepresentationEnum::kTaggedPointer
                    | MemoryRepresentationEnum::kTaggedSigned
                    | MemoryRepresentationEnum::kAnyUncompressedTagged
                    | MemoryRepresentationEnum::kUncompressedTaggedPointer
                    | MemoryRepresentationEnum::kUncompressedTaggedSigned
                    | MemoryRepresentationEnum::kProtectedPointer
                    | MemoryRepresentationEnum::kIndirectPointer
                    | MemoryRepresentationEnum::kSandboxedPointer
                    | MemoryRepresentationEnum::kSimd128
                    | MemoryRepresentationEnum::kSimd256 => panic!("UNREACHABLE"),
                    MemoryRepresentationEnum::kInvalid => false,
                }
            }

            pub fn IsCompressibleTagged(&self) -> bool {
                match self.value() {
                    MemoryRepresentationEnum::kAnyTagged
                    | MemoryRepresentationEnum::kTaggedPointer
                    | MemoryRepresentationEnum::kTaggedSigned => true,
                    MemoryRepresentationEnum::kInt8
                    | MemoryRepresentationEnum::kInt16
                    | MemoryRepresentationEnum::kInt32
                    | MemoryRepresentationEnum::kInt64
                    | MemoryRepresentationEnum::kUint8
                    | MemoryRepresentationEnum::kUint16
                    | MemoryRepresentationEnum::kUint32
                    | MemoryRepresentationEnum::kUint64
                    | MemoryRepresentationEnum::kFloat16
                    | MemoryRepresentationEnum::kFloat32
                    | MemoryRepresentationEnum::kFloat64
                    | MemoryRepresentationEnum::kAnyUncompressedTagged
                    | MemoryRepresentationEnum::kUncompressedTaggedPointer
                    | MemoryRepresentationEnum::kUncompressedTaggedSigned
                    | MemoryRepresentationEnum::kIndirectPointer
                    | MemoryRepresentationEnum::kProtectedPointer
                    | MemoryRepresentationEnum::kSandboxedPointer
                    | MemoryRepresentationEnum::kSimd128
                    | MemoryRepresentationEnum::kSimd256 => false,
                    MemoryRepresentationEnum::kInvalid => false,
                }
            }

            pub fn ToRegisterRepresentation(&self) -> RegisterRepresentation {
                match self.value() {
                    MemoryRepresentationEnum::kInt8
                    | MemoryRepresentationEnum::kUint8
                    | MemoryRepresentationEnum::kInt16
                    | MemoryRepresentationEnum::kUint16
                    | MemoryRepresentationEnum::kInt32
                    | MemoryRepresentationEnum::kUint32 => RegisterRepresentation::Word32(),
                    MemoryRepresentationEnum::kInt64 | MemoryRepresentationEnum::kUint64 => {
                        RegisterRepresentation::Word64()
                    }
                    MemoryRepresentationEnum::kFloat16 | MemoryRepresentationEnum::kFloat32 => {
                        RegisterRepresentation::Float32()
                    }
                    MemoryRepresentationEnum::kFloat64 => RegisterRepresentation::Float64(),
                    MemoryRepresentationEnum::kAnyTagged
                    |
