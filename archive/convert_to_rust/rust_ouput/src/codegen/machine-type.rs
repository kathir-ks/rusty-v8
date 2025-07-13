// Converted from V8 C++ source files:
// Header: machine-type.h
// Implementation: machine-type.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod machine_type {
    use std::{fmt, mem};
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum MachineRepresentation {
        kNone,
        kBit,
        kWord8,
        kWord16,
        kWord32,
        kWord64,
        kMapWord,
        kTaggedSigned,
        kTaggedPointer,
        kTagged,
        kCompressedPointer,
        kCompressed,
        kProtectedPointer,
        kIndirectPointer,
        kSandboxedPointer,
        kFloat16RawBits,
        kFloat16,
        kFloat32,
        kFloat64,
        kSimd128,
        kSimd256,
    }

    impl Default for MachineRepresentation {
        fn default() -> Self {
            MachineRepresentation::kNone
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum MachineSemantic {
        kNone,
        kBool,
        kInt32,
        kUint32,
        kInt64,
        kUint64,
        kSignedBigInt64,
        kUnsignedBigInt64,
        kNumber,
        kHoleyFloat64,
        kAny,
    }

    impl Default for MachineSemantic {
        fn default() -> Self {
            MachineSemantic::kNone
        }
    }

    const K_INT_SIZE: usize = 4;
    const K_BITS_PER_BYTE: usize = 8;
    const K_SYSTEM_POINTER_SIZE: usize = 8; // Assuming 64-bit system for now
    const K_TAGGED_SIZE_LOG2: usize = 2;  // Assuming tagged size log2 is 2 (4 bytes)
    const K_SYSTEM_POINTER_SIZE_LOG2: usize = if K_SYSTEM_POINTER_SIZE == 4 { 2 } else { 3 };

    pub fn is_subtype(rep1: MachineRepresentation, rep2: MachineRepresentation) -> bool {
        if rep1 == rep2 {
            return true;
        }
        match rep1 {
            MachineRepresentation::kTaggedSigned | MachineRepresentation::kTaggedPointer => {
                rep2 == MachineRepresentation::kTagged
            }
            MachineRepresentation::kCompressedPointer => {
                rep2 == MachineRepresentation::kCompressed
            }
            _ => false,
        }
    }

    pub fn machine_repr_to_string(rep: MachineRepresentation) -> &'static str {
        match rep {
            MachineRepresentation::kNone => "kMachNone",
            MachineRepresentation::kBit => "kRepBit",
            MachineRepresentation::kWord8 => "kRepWord8",
            MachineRepresentation::kWord16 => "kRepWord16",
            MachineRepresentation::kWord32 => "kRepWord32",
            MachineRepresentation::kWord64 => "kRepWord64",
            MachineRepresentation::kFloat16 => "kRepFloat16",
            MachineRepresentation::kFloat16RawBits => "kRepFloat16RawBits",
            MachineRepresentation::kFloat32 => "kRepFloat32",
            MachineRepresentation::kFloat64 => "kRepFloat64",
            MachineRepresentation::kSimd128 => "kRepSimd128",
            MachineRepresentation::kSimd256 => "kRepSimd256",
            MachineRepresentation::kTaggedSigned => "kRepTaggedSigned",
            MachineRepresentation::kTaggedPointer => "kRepTaggedPointer",
            MachineRepresentation::kTagged => "kRepTagged",
            MachineRepresentation::kCompressedPointer => "kRepCompressedPointer",
            MachineRepresentation::kCompressed => "kRepCompressed",
            MachineRepresentation::kProtectedPointer => "kRepProtectedPointer",
            MachineRepresentation::kIndirectPointer => "kRepIndirectPointer",
            MachineRepresentation::kMapWord => "kRepMapWord",
            MachineRepresentation::kSandboxedPointer => "kRepSandboxedPointer",
        }
    }

    impl Display for MachineRepresentation {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", machine_repr_to_string(*self))
        }
    }

    impl Display for MachineSemantic {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                MachineSemantic::kNone => write!(f, "kMachNone"),
                MachineSemantic::kBool => write!(f, "kTypeBool"),
                MachineSemantic::kInt32 => write!(f, "kTypeInt32"),
                MachineSemantic::kUint32 => write!(f, "kTypeUint32"),
                MachineSemantic::kInt64 => write!(f, "kTypeInt64"),
                MachineSemantic::kUint64 => write!(f, "kTypeUint64"),
                MachineSemantic::kSignedBigInt64 => write!(f, "kTypeSignedBigInt64"),
                MachineSemantic::kUnsignedBigInt64 => write!(f, "kTypeUnsignedBigInt64"),
                MachineSemantic::kNumber => write!(f, "kTypeNumber"),
                MachineSemantic::kHoleyFloat64 => write!(f, "kTypeHoleyFloat64"),
                MachineSemantic::kAny => write!(f, "kTypeAny"),
            }
        }
    }

    #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MachineType {
        representation_: MachineRepresentation,
        semantic_: MachineSemantic,
    }

    impl MachineType {
        pub const fn new(representation: MachineRepresentation, semantic: MachineSemantic) -> Self {
            MachineType {
                representation_: representation,
                semantic_: semantic,
            }
        }

        pub const fn representation(&self) -> MachineRepresentation {
            self.representation_
        }
        pub const fn semantic(&self) -> MachineSemantic {
            self.semantic_
        }

        pub const fn is_none(&self) -> bool {
            self.representation() == MachineRepresentation::kNone
        }

        pub const fn is_map_word(&self) -> bool {
            self.representation() == MachineRepresentation::kMapWord
        }

        pub const fn is_signed(&self) -> bool {
            self.semantic() == MachineSemantic::kInt32 ||
                self.semantic() == MachineSemantic::kInt64
        }
        pub const fn is_unsigned(&self) -> bool {
            self.semantic() == MachineSemantic::kUint32 ||
                self.semantic() == MachineSemantic::kUint64
        }
        pub const fn is_tagged(&self) -> bool {
            self.representation() == MachineRepresentation::kTaggedPointer ||
                self.representation() == MachineRepresentation::kTaggedSigned ||
                self.representation() == MachineRepresentation::kTagged
        }
        pub const fn is_tagged_signed(&self) -> bool {
            self.representation() == MachineRepresentation::kTaggedSigned
        }
        pub const fn is_tagged_pointer(&self) -> bool {
            self.representation() == MachineRepresentation::kTaggedPointer
        }
        pub const fn is_compressed(&self) -> bool {
            self.representation() == MachineRepresentation::kCompressedPointer ||
                self.representation() == MachineRepresentation::kCompressed
        }
        pub const fn is_compressed_pointer(&self) -> bool {
            self.representation() == MachineRepresentation::kCompressedPointer
        }
        pub const fn is_indirect_pointer(&self) -> bool {
            self.representation() == MachineRepresentation::kIndirectPointer
        }
        pub const fn pointer_representation() -> MachineRepresentation {
            if K_SYSTEM_POINTER_SIZE == 4 {
                MachineRepresentation::kWord32
            } else {
                MachineRepresentation::kWord64
            }
        }
        pub const fn uint_ptr() -> Self {
            if K_SYSTEM_POINTER_SIZE == 4 {
                Self::uint32()
            } else {
                Self::uint64()
            }
        }
        pub const fn int_ptr() -> Self {
            if K_SYSTEM_POINTER_SIZE == 4 {
                Self::int32()
            } else {
                Self::int64()
            }
        }
        pub const fn int8() -> Self {
            MachineType::new(MachineRepresentation::kWord8, MachineSemantic::kInt32)
        }
        pub const fn uint8() -> Self {
            MachineType::new(MachineRepresentation::kWord8, MachineSemantic::kUint32)
        }
        pub const fn int16() -> Self {
            MachineType::new(MachineRepresentation::kWord16, MachineSemantic::kInt32)
        }
        pub const fn uint16() -> Self {
            MachineType::new(MachineRepresentation::kWord16, MachineSemantic::kUint32)
        }
        pub const fn int32() -> Self {
            MachineType::new(MachineRepresentation::kWord32, MachineSemantic::kInt32)
        }
        pub const fn uint32() -> Self {
            MachineType::new(MachineRepresentation::kWord32, MachineSemantic::kUint32)
        }
        pub const fn int64() -> Self {
            MachineType::new(MachineRepresentation::kWord64, MachineSemantic::kInt64)
        }
        pub const fn uint64() -> Self {
            MachineType::new(MachineRepresentation::kWord64, MachineSemantic::kUint64)
        }
        pub const fn signed_big_int64() -> Self {
            MachineType::new(MachineRepresentation::kWord64, MachineSemantic::kSignedBigInt64)
        }
        pub const fn unsigned_big_int64() -> Self {
            MachineType::new(MachineRepresentation::kWord64, MachineSemantic::kUnsignedBigInt64)
        }
        pub const fn float16() -> Self {
            MachineType::new(MachineRepresentation::kFloat16, MachineSemantic::kNumber)
        }
        pub const fn float32() -> Self {
            MachineType::new(MachineRepresentation::kFloat32, MachineSemantic::kNumber)
        }
        pub const fn float64() -> Self {
            MachineType::new(MachineRepresentation::kFloat64, MachineSemantic::kNumber)
        }
        pub const fn holey_float64() -> Self {
            MachineType::new(MachineRepresentation::kFloat64, MachineSemantic::kHoleyFloat64)
        }
        pub const fn simd128() -> Self {
            MachineType::new(MachineRepresentation::kSimd128, MachineSemantic::kNone)
        }
        pub const fn simd256() -> Self {
            MachineType::new(MachineRepresentation::kSimd256, MachineSemantic::kNone)
        }
        pub const fn pointer() -> Self {
            MachineType::new(Self::pointer_representation(), MachineSemantic::kNone)
        }
        pub const fn tagged_pointer() -> Self {
            MachineType::new(MachineRepresentation::kTaggedPointer, MachineSemantic::kAny)
        }
        pub const fn map_in_header() -> Self {
            MachineType::new(MachineRepresentation::kMapWord, MachineSemantic::kAny)
        }
        pub const fn tagged_signed() -> Self {
            MachineType::new(MachineRepresentation::kTaggedSigned, MachineSemantic::kInt32)
        }
        pub const fn any_tagged() -> Self {
            MachineType::new(MachineRepresentation::kTagged, MachineSemantic::kAny)
        }
        pub const fn compressed_pointer() -> Self {
            MachineType::new(MachineRepresentation::kCompressedPointer, MachineSemantic::kAny)
        }
        pub const fn any_compressed() -> Self {
            MachineType::new(MachineRepresentation::kCompressed, MachineSemantic::kAny)
        }
        pub const fn sandboxed_pointer() -> Self {
            MachineType::new(MachineRepresentation::kSandboxedPointer, MachineSemantic::kInt64)
        }
        pub const fn protected_pointer() -> Self {
            MachineType::new(MachineRepresentation::kProtectedPointer, MachineSemantic::kAny)
        }
        pub const fn indirect_pointer() -> Self {
            MachineType::new(MachineRepresentation::kIndirectPointer, MachineSemantic::kInt32)
        }
        pub const fn bool() -> Self {
            MachineType::new(MachineRepresentation::kBit, MachineSemantic::kBool)
        }
        pub const fn none() -> Self {
            MachineType::new(MachineRepresentation::kNone, MachineSemantic::kNone)
        }

        pub fn type_for_representation(rep: &MachineRepresentation, is_signed: bool) -> Self {
            match rep {
                MachineRepresentation::kNone => MachineType::none(),
                MachineRepresentation::kBit => MachineType::bool(),
                MachineRepresentation::kWord8 => {
                    if is_signed {
                        MachineType::int8()
                    } else {
                        MachineType::uint8()
                    }
                }
                MachineRepresentation::kWord16 => {
                    if is_signed {
                        MachineType::int16()
                    } else {
                        MachineType::uint16()
                    }
                }
                MachineRepresentation::kWord32 => {
                    if is_signed {
                        MachineType::int32()
                    } else {
                        MachineType::uint32()
                    }
                }
                MachineRepresentation::kWord64 => {
                    if is_signed {
                        MachineType::int64()
                    } else {
                        MachineType::uint64()
                    }
                }
                MachineRepresentation::kFloat16 => MachineType::float16(),
                MachineRepresentation::kFloat32 => MachineType::float32(),
                MachineRepresentation::kFloat64 => MachineType::float64(),
                MachineRepresentation::kSimd128 => MachineType::simd128(),
                MachineRepresentation::kSimd256 => MachineType::simd256(),
                MachineRepresentation::kTagged => MachineType::any_tagged(),
                MachineRepresentation::kTaggedSigned => MachineType::tagged_signed(),
                MachineRepresentation::kTaggedPointer => MachineType::tagged_pointer(),
                MachineRepresentation::kCompressed => MachineType::any_compressed(),
                MachineRepresentation::kIndirectPointer => MachineType::indirect_pointer(),
                MachineRepresentation::kCompressedPointer => MachineType::compressed_pointer(),
                MachineRepresentation::kSandboxedPointer => MachineType::sandboxed_pointer(),
                _ => panic!("UNREACHABLE"),
            }
        }

       pub fn type_for_ctype(type: &CTypeInfo) -> Self {
            match type.get_type() {
                CType::kVoid => MachineType::any_tagged(),
                CType::kBool => MachineType::bool(),
                CType::kUint8 => MachineType::uint8(),
                CType::kInt32 => MachineType::int32(),
                CType::kUint32 => MachineType::uint32(),
                CType::kInt64 => MachineType::int64(),
                CType::kAny => {
                    assert_eq!(mem::size_of::<AnyCType>(), 8, "CTypeInfo::Type::kAny is assumed to be of size 64 bits.");
                    MachineType::int64()
                }
                CType::kUint64 => MachineType::uint64(),
                CType::kFloat32 => MachineType::float32(),
                CType::kFloat64 => MachineType::float64(),
                CType::kPointer => MachineType::pointer(),
                CType::kV8Value | CType::kSeqOneByteString | CType::kApiObject => MachineType::any_tagged(),
            }
        }

        pub const fn less_than_or_equal_pointer_size(&self) -> bool {
            element_size_log2_of(self.representation()) <= K_SYSTEM_POINTER_SIZE_LOG2
        }

        pub const fn mem_size(&self) -> u8 {
            1 << element_size_log2_of(self.representation())
        }
    }

    impl Display for MachineType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            if *self == MachineType::none() {
                return Ok(());
            } else if self.representation() == MachineRepresentation::kNone {
                return write!(f, "{}", self.semantic());
            } else if self.semantic() == MachineSemantic::kNone {
                return write!(f, "{}", self.representation());
            } else {
                return write!(f, "{}|{}", self.representation(), self.semantic());
            }
        }
    }

    pub const fn is_integral(rep: MachineRepresentation) -> bool {
        (rep as u8) >= (MachineRepresentation::kWord8 as u8) &&
            (rep as u8) <= (MachineRepresentation::kWord64 as u8)
    }

    pub const fn is_floating_point(rep: MachineRepresentation) -> bool {
        (rep as u8) >= (MachineRepresentation::kFloat16 as u8)
    }

    pub const fn is_simd128(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kSimd128
    }

    pub const fn can_be_tagged_pointer(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kTagged ||
            rep == MachineRepresentation::kTaggedPointer ||
            rep == MachineRepresentation::kMapWord
    }

    pub const fn can_be_tagged_signed(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kTagged ||
            rep == MachineRepresentation::kTaggedSigned
    }

    pub const fn is_any_tagged(rep: MachineRepresentation) -> bool {
        can_be_tagged_pointer(rep) || rep == MachineRepresentation::kTaggedSigned
    }

    pub const fn can_be_compressed_pointer(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kCompressed ||
            rep == MachineRepresentation::kCompressedPointer
    }

     pub const fn can_be_indirect_pointer(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kIndirectPointer
    }

    pub const fn can_be_tagged_or_compressed_pointer(rep: MachineRepresentation) -> bool {
        can_be_tagged_pointer(rep) || can_be_compressed_pointer(rep) ||
             rep == MachineRepresentation::kProtectedPointer
    }

    pub const fn can_be_tagged_or_compressed_or_indirect_pointer(rep: MachineRepresentation) -> bool {
        can_be_tagged_pointer(rep) || can_be_compressed_pointer(rep) || can_be_indirect_pointer(rep)
    }

    pub const fn is_any_compressed(rep: MachineRepresentation) -> bool {
        can_be_compressed_pointer(rep)
    }

    pub const fn element_size_log2_of(rep: MachineRepresentation) -> usize {
        match rep {
            MachineRepresentation::kBit | MachineRepresentation::kWord8 => 0,
            MachineRepresentation::kWord16 | MachineRepresentation::kFloat16 => 1,
            MachineRepresentation::kWord32 | MachineRepresentation::kFloat32 | MachineRepresentation::kIndirectPointer => 2,
            MachineRepresentation::kWord64 | MachineRepresentation::kFloat64 => 3,
            MachineRepresentation::kSimd128 => 4,
            MachineRepresentation::kSimd256 => 5,
            MachineRepresentation::kTaggedSigned |
            MachineRepresentation::kTaggedPointer |
            MachineRepresentation::kTagged |
            MachineRepresentation::kMapWord |
            MachineRepresentation::kCompressedPointer |
            MachineRepresentation::kCompressed |
            MachineRepresentation::kProtectedPointer => K_TAGGED_SIZE_LOG2,
            MachineRepresentation::kSandboxedPointer => K_SYSTEM_POINTER_SIZE_LOG2,
            _ => panic!("UNREACHABLE"),
        }
    }

    const K_MAXIMUM_REPR_SIZE_LOG2: usize = element_size_log2_of(MachineRepresentation::kSimd128);
    const K_MAXIMUM_REPR_SIZE_IN_BYTES: usize = 1 << K_MAXIMUM_REPR_SIZE_LOG2;

    pub const fn element_size_in_bytes(rep: MachineRepresentation) -> usize {
        1 << element_size_log2_of(rep)
    }

    pub const fn element_size_in_bits(rep: MachineRepresentation) -> usize {
        8 * element_size_in_bytes(rep)
    }

    pub const fn max_unsigned_value(rep: MachineRepresentation) -> u64 {
        match rep {
            MachineRepresentation::kWord8 => u8::MAX as u64,
            MachineRepresentation::kWord16 => u16::MAX as u64,
            MachineRepresentation::kWord32 => u32::MAX as u64,
            MachineRepresentation::kWord64 => u64::MAX,
            _ => panic!("UNREACHABLE"),
        }
    }

    pub const fn element_size_in_pointers(rep: MachineRepresentation) -> usize {
        (element_size_in_bytes(rep) + K_SYSTEM_POINTER_SIZE - 1) / K_SYSTEM_POINTER_SIZE
    }

    pub const fn representation_bit(rep: MachineRepresentation) -> i32 {
        1 << (rep as i32)
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CType {
        kVoid,
        kBool,
        kUint8,
        kInt32,
        kUint32,
        kInt64,
        kAny,
        kUint64,
        kFloat32,
        kFloat64,
        kPointer,
        kV8Value,
        kSeqOneByteString,
        kApiObject,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CTypeInfo {
        type_: CType,
    }

    impl CTypeInfo {
        pub fn new(type_: CType) -> Self {
            CTypeInfo { type_ }
        }

        pub fn get_type(&self) -> CType {
            self.type_
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct AnyCType {
        value: u64,
    }
}
