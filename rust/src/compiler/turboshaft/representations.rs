// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    limits::*,
    mem,
};

macro_rules! any_of {
    ($($x:expr),+ ) => {
        {
            #[allow(unused_comparisons)]
            $(
                $x
            )
        }
    };
}

mod base {
    pub mod hashing {
        pub fn hash<T: std::hash::Hash>(t: &T) -> u64 {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
        
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
    }
}

mod codegen {
    pub mod machine_type {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MachineRepresentation {
            kBit,
            kWord8,
            kWord16,
            kWord32,
            kWord64,
            kTaggedSigned,
            kTaggedPointer,
            kTagged,
            kProtectedPointer,
            kIndirectPointer,
            kSandboxedPointer,
            kCompressedPointer,
            kCompressed,
            kFloat16,
            kFloat32,
            kFloat64,
            kFloat16RawBits,
            kSimd128,
            kSimd256,
            kMapWord, // Added to match all C++ enum values
            kNone, //Added to match all C++ enum values
        }

        impl MachineRepresentation {
            pub fn is_signed(&self) -> bool {
                match self {
                    MachineRepresentation::kWord8 |
                    MachineRepresentation::kWord16 |
                    MachineRepresentation::kWord32 |
                    MachineRepresentation::kWord64 => true,
                    _ => false,
                }
            }
        }
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct MachineType {
            representation: MachineRepresentation,
            is_signed: bool,
        }

        impl MachineType {
            pub fn representation(&self) -> MachineRepresentation {
                self.representation
            }

            pub fn is_signed(&self) -> bool {
                self.is_signed
            }

            pub fn int8() -> Self {
                MachineType { representation: MachineRepresentation::kWord8, is_signed: true }
            }

            pub fn uint8() -> Self {
                MachineType { representation: MachineRepresentation::kWord8, is_signed: false }
            }

            pub fn int16() -> Self {
                MachineType { representation: MachineRepresentation::kWord16, is_signed: true }
            }

            pub fn uint16() -> Self {
                MachineType { representation: MachineRepresentation::kWord16, is_signed: false }
            }

            pub fn int32() -> Self {
                MachineType { representation: MachineRepresentation::kWord32, is_signed: true }
            }

            pub fn uint32() -> Self {
                MachineType { representation: MachineRepresentation::kWord32, is_signed: false }
            }

            pub fn int64() -> Self {
                MachineType { representation: MachineRepresentation::kWord64, is_signed: true }
            }

            pub fn uint64() -> Self {
                MachineType { representation: MachineRepresentation::kWord64, is_signed: false }
            }

            pub fn float16() -> Self {
                MachineType { representation: MachineRepresentation::kFloat16, is_signed: false }
            }

            pub fn float32() -> Self {
                MachineType { representation: MachineRepresentation::kFloat32, is_signed: false }
            }

            pub fn float64() -> Self {
                MachineType { representation: MachineRepresentation::kFloat64, is_signed: false }
            }

            pub fn any_tagged() -> Self {
                MachineType { representation: MachineRepresentation::kTagged, is_signed: false }
            }

            pub fn tagged_pointer() -> Self {
                MachineType { representation: MachineRepresentation::kTaggedPointer, is_signed: false }
            }

            pub fn tagged_signed() -> Self {
                MachineType { representation: MachineRepresentation::kTaggedSigned, is_signed: false }
            }

            pub fn protected_pointer() -> Self {
                MachineType { representation: MachineRepresentation::kProtectedPointer, is_signed: false }
            }

            pub fn indirect_pointer() -> Self {
                MachineType { representation: MachineRepresentation::kIndirectPointer, is_signed: false }
            }

            pub fn sandboxed_pointer() -> Self {
                MachineType { representation: MachineRepresentation::kSandboxedPointer, is_signed: false }
            }

            pub fn simd128() -> Self {
                MachineType { representation: MachineRepresentation::kSimd128, is_signed: false }
            }

            pub fn simd256() -> Self {
                MachineType { representation: MachineRepresentation::kSimd256, is_signed: false }
            }

            // Placeholder implementation
            pub fn type_for_ctype(_t: &CTypeInfo) -> Self {
                MachineType::any_tagged()
            }
        }
    }

}

mod compiler {
    pub mod turboshaft {
        use std::fmt;
        use crate::codegen::machine_type::*;
        use std::hash::{Hash};
        use std::marker::ConstParamTy;

        // Placeholder constant, replace with actual value from v8-internal.h
        const kSystemPointerSize: u16 = 8;
        const kTaggedSize: u16 = 8;

        // Placeholder constant, replace with actual value from v8-internal.h
        const kInt64Size: u16 = 8;

        // Placeholder constant, replace with actual value depending on the build config.
        const COMPRESS_POINTERS_BOOL: bool = false;
        const V8_MAP_PACKING_BOOL: bool = false;

        const kTaggedSizeLog2: u8 = if kTaggedSize == 8 { 3 } else { 2 };
        const kSystemPointerSizeLog2: u8 = if kSystemPointerSize == 8 { 3 } else { 2 };

        const fn Is64() -> bool {
            kSystemPointerSize == 8
        }

        /// Optional register representation.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub struct MaybeRegisterRepresentation {
            value_: MaybeRegisterRepresentationEnum,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub enum MaybeRegisterRepresentationEnum {
            kWord32,
            kWord64,
            kFloat32,
            kFloat64,
            kTagged,
            kCompressed,
            kSimd128,
            kSimd256,
            kNone, // No register representation.
            kInvalid, // Added to mimic C++'s kInvalid
        }

        impl MaybeRegisterRepresentation {
            pub const fn new(value: MaybeRegisterRepresentationEnum) -> Self {
                MaybeRegisterRepresentation { value_: value }
            }

            pub const fn invalid() -> Self {
                MaybeRegisterRepresentation { value_: MaybeRegisterRepresentationEnum::kInvalid }
            }

            pub const fn is_valid(&self) -> bool {
                self.value_ != MaybeRegisterRepresentationEnum::kInvalid
            }

            pub const fn value(&self) -> MaybeRegisterRepresentationEnum {
                assert!(self.is_valid());
                self.value_
            }

            pub const fn word32() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kWord32)
            }

            pub const fn word64() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kWord64)
            }

            pub const fn word_ptr() -> Self {
                if kSystemPointerSize == 4 {
                    MaybeRegisterRepresentation::word32()
                } else {
                    assert_eq!(kSystemPointerSize, 8);
                    MaybeRegisterRepresentation::word64()
                }
            }

            pub const fn float32() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kFloat32)
            }

            pub const fn float64() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kFloat64)
            }

            pub const fn tagged() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kTagged)
            }

            pub const fn compressed() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kCompressed)
            }

            pub const fn simd128() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kSimd128)
            }

            pub const fn simd256() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kSimd256)
            }

            pub const fn none() -> Self {
                MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kNone)
            }

            pub const fn is_word(&self) -> bool {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 | MaybeRegisterRepresentationEnum::kWord64 => true,
                    MaybeRegisterRepresentationEnum::kFloat32 |
                    MaybeRegisterRepresentationEnum::kFloat64 |
                    MaybeRegisterRepresentationEnum::kTagged |
                    MaybeRegisterRepresentationEnum::kCompressed |
                    MaybeRegisterRepresentationEnum::kSimd128 |
                    MaybeRegisterRepresentationEnum::kSimd256 |
                    MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub const fn is_float(&self) -> bool {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kFloat32 | MaybeRegisterRepresentationEnum::kFloat64 => true,
                    MaybeRegisterRepresentationEnum::kWord32 |
                    MaybeRegisterRepresentationEnum::kWord64 |
                    MaybeRegisterRepresentationEnum::kTagged |
                    MaybeRegisterRepresentationEnum::kCompressed |
                    MaybeRegisterRepresentationEnum::kSimd128 |
                    MaybeRegisterRepresentationEnum::kSimd256 |
                    MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub const fn is_tagged_or_compressed(&self) -> bool {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kTagged | MaybeRegisterRepresentationEnum::kCompressed => true,
                    MaybeRegisterRepresentationEnum::kWord32 |
                    MaybeRegisterRepresentationEnum::kWord64 |
                    MaybeRegisterRepresentationEnum::kFloat32 |
                    MaybeRegisterRepresentationEnum::kFloat64 |
                    MaybeRegisterRepresentationEnum::kSimd128 |
                    MaybeRegisterRepresentationEnum::kSimd256 |
                    MaybeRegisterRepresentationEnum::kNone => false,
                    MaybeRegisterRepresentationEnum::kInvalid => false,
                }
            }

            pub fn max_unsigned_value(&self) -> u64 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => u32::MAX as u64,
                    MaybeRegisterRepresentationEnum::kWord64 => u64::MAX,
                    MaybeRegisterRepresentationEnum::kFloat32 |
                    MaybeRegisterRepresentationEnum::kFloat64 |
                    MaybeRegisterRepresentationEnum::kTagged |
                    MaybeRegisterRepresentationEnum::kCompressed |
                    MaybeRegisterRepresentationEnum::kSimd128 |
                    MaybeRegisterRepresentationEnum::kSimd256 |
                    MaybeRegisterRepresentationEnum::kNone => panic!("UNREACHABLE"),
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("UNREACHABLE"),
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
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("UNREACHABLE"),
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
                    MaybeRegisterRepresentationEnum::kInvalid => panic!("UNREACHABLE"),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub struct RegisterRepresentation {
            rep: MaybeRegisterRepresentation
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub enum RegisterRepresentationEnum {
            kWord32,
            kWord64,
            kFloat32,
            kFloat64,
            kTagged,
            kCompressed,
            kSimd128,
            kSimd256,
        }

        impl RegisterRepresentation {
            pub const fn new(value: RegisterRepresentationEnum) -> Self {
                RegisterRepresentation { rep: match value {
                    RegisterRepresentationEnum::kWord32 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kWord32),
                    RegisterRepresentationEnum::kWord64 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kWord64),
                    RegisterRepresentationEnum::kFloat32 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kFloat32),
                    RegisterRepresentationEnum::kFloat64 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kFloat64),
                    RegisterRepresentationEnum::kTagged => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kTagged),
                    RegisterRepresentationEnum::kCompressed => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kCompressed),
                    RegisterRepresentationEnum::kSimd128 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kSimd128),
                    RegisterRepresentationEnum::kSimd256 => MaybeRegisterRepresentation::new(MaybeRegisterRepresentationEnum::kSimd256),
                } }
            }

            pub const fn default() -> Self {
                RegisterRepresentation { rep: MaybeRegisterRepresentation::invalid() }
            }

            pub const fn from_maybe(rep: MaybeRegisterRepresentation) -> Self {
                 RegisterRepresentation { rep: rep }
            }

            pub const fn value(&self) -> MaybeRegisterRepresentationEnum {
                self.rep.value()
            }

            pub const fn word32() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kWord32)
            }
            pub const fn word64() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kWord64)
            }
            // The equivalent of intptr_t/uintptr_t: An integral type with the same size
            // as machine pointers.
            pub const fn word_ptr() -> Self {
                RegisterRepresentation::from_maybe(MaybeRegisterRepresentation::word_ptr())
            }
            pub const fn float32() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kFloat32)
            }
            pub const fn float64() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kFloat64)
            }
            // A tagged pointer stored in a register, in the case of pointer compression
            // it is an uncompressed pointer or a Smi.
            pub const fn tagged() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kTagged)
            }
            // A compressed tagged pointer stored in a register, the upper 32bit are
            // unspecified.
            pub const fn compressed() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kCompressed)
            }
            pub const fn simd128() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kSimd128)
            }
            pub const fn simd256() -> Self {
                RegisterRepresentation::new(RegisterRepresentationEnum::kSimd256)
            }

            pub fn from_machine_representation(rep: MachineRepresentation) -> Self {
                match rep {
                    MachineRepresentation::kBit => RegisterRepresentation::word32(),
                    MachineRepresentation::kWord8 => RegisterRepresentation::word32(),
                    MachineRepresentation::kWord16 => RegisterRepresentation::word32(),
                    MachineRepresentation::kWord32 => RegisterRepresentation::word32(),
                    MachineRepresentation::kWord64 => RegisterRepresentation::word64(),
                    MachineRepresentation::kTaggedSigned => RegisterRepresentation::tagged(),
                    MachineRepresentation::kTaggedPointer => RegisterRepresentation::tagged(),
                    MachineRepresentation::kTagged => RegisterRepresentation::tagged(),
                    MachineRepresentation::kProtectedPointer => RegisterRepresentation::tagged(),
                    MachineRepresentation::kCompressedPointer => RegisterRepresentation::compressed(),
                    MachineRepresentation::kCompressed => RegisterRepresentation::compressed(),
                    MachineRepresentation::kFloat16 => RegisterRepresentation::float32(),
                    MachineRepresentation::kFloat32 => RegisterRepresentation::float32(),
                    MachineRepresentation::kFloat64 => RegisterRepresentation::float64(),
                    MachineRepresentation::kSimd128 => RegisterRepresentation::simd128(),
                    MachineRepresentation::kSimd256 => RegisterRepresentation::simd256(),
                    MachineRepresentation::kMapWord => {
                        assert!(!V8_MAP_PACKING_BOOL);
                        RegisterRepresentation::tagged()
                    }
                    MachineRepresentation::kIndirectPointer => RegisterRepresentation::word_ptr(),
                    MachineRepresentation::kSandboxedPointer => RegisterRepresentation::word_ptr(),
                    MachineRepresentation::kNone | MachineRepresentation::kFloat16RawBits => panic!("UNREACHABLE"),
                }
            }

            pub fn from_machine_type(type_: MachineType) -> Self {
                RegisterRepresentation::from_machine_representation(type_.representation())
            }

            // Placeholder for CTypeInfo and CFunctionInfo
            pub fn from_ctype_info(_t: CTypeInfo, int64_repr: CFunctionInfoInt64Representation) -> Self {
                if _t.get_type() == CTypeInfoType::kVoid || _t.get_type() == CTypeInfoType::kPointer {
                  RegisterRepresentation::tagged()
                } else if _t.get_type() == CTypeInfoType::kInt64 || _t.get_type() == CTypeInfoType::kUint64 {
                  if int64_repr == CFunctionInfoInt64Representation::kBigInt {
                    RegisterRepresentation::word64()
                  } else {
                    assert_eq!(int64_repr, CFunctionInfoInt64Representation::kNumber);
                    RegisterRepresentation::float64()
                  }
                } else {
                  RegisterRepresentation::from_machine_type(MachineType::type_for_ctype(&_t))
                }
            }

            pub const fn allow_implicit_representation_change_to(&self, dst_rep: RegisterRepresentation, graph_created_from_turbofan: bool) -> bool {
                if self == &dst_rep {
                    return true;
                }
                match dst_rep.rep.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => {
                        // We allow implicit tagged -> untagged conversions.
                        // Even without pointer compression, we use `Word32And` for Smi-checks on
                        // tagged values.
                        if *self == any_of!(RegisterRepresentation::tagged(),
                                            RegisterRepresentation::compressed()) {
                            return true;
                        }
                        if graph_created_from_turbofan &&
                            *self == RegisterRepresentation::word64() {
                            // TODO(12783): Remove this once Turboshaft graphs are not constructed
                            // via Turbofan any more. Unfortunately Turbofan has many implicit
                            // truncations which are hard to fix. Still, for wasm it is required
                            // that truncations in Turboshaft are explicit.
                            return true;
                        }
                        false
                    }
                    MaybeRegisterRepresentationEnum::kWord64 => {
                        // We allow implicit tagged -> untagged conversions.
                        if kTaggedSize == kInt64Size &&
                            *self == RegisterRepresentation::tagged() {
                            return true;
                        }
                        false
                    }
                    MaybeRegisterRepresentationEnum::kTagged => {
                        // We allow implicit untagged -> tagged conversions. This is only safe for
                        // Smi values.
                        if *self == RegisterRepresentation::word_ptr() {
                            return true;
                        }
                        false
                    }
                    MaybeRegisterRepresentationEnum::kCompressed => {
                        // Compression is a no-op.
                        if *self == any_of!(RegisterRepresentation::tagged(),
                                            RegisterRepresentation::word_ptr(),
                                            RegisterRepresentation::word32()) {
                            return true;
                        }
                        false
                    }
                    _ => false
                }
            }

            pub const fn map_tagged_to_word(&self) -> Self {
                if self.rep.value() == MaybeRegisterRepresentationEnum::kTagged {
                    if COMPRESS_POINTERS_BOOL {
                        RegisterRepresentation::word32()
                    } else {
                        RegisterRepresentation::word_ptr()
                    }
                } else {
                    *self
                }
            }
        }

        impl From<MaybeRegisterRepresentation> for RegisterRepresentation {
            fn from(rep: MaybeRegisterRepresentation) -> Self {
                RegisterRepresentation::from_maybe(rep)
            }
        }

        impl std::ops::Deref for RegisterRepresentation {
            type Target = MaybeRegisterRepresentation;

            fn deref(&self) -> &Self::Target {
                &self.rep
            }
        }

        // Placeholder implementation for MaybeRegisterRepresentation printing
        impl fmt::Display for MaybeRegisterRepresentation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.value())
            }
        }

        // Placeholder implementation for RegisterRepresentation printing
        impl fmt::Display for RegisterRepresentation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.rep.value())
            }
        }

        impl fmt::Debug for WordRepresentation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "WordRepresentation::{:?}", self.value())
            }
        }
          
        impl fmt::Debug for FloatRepresentation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "FloatRepresentation::{:?}", self.value())
            }
        }
          
        impl fmt::Debug for MemoryRepresentation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "MemoryRepresentation::{:?}", self.value())
            }
        }
        
        
        struct MultiSwitchHelper {}

        impl MultiSwitchHelper {
            const MAX_VALUE: u64 = 8;
        }
        
        // C++ Template struct MultiSwitch equivalent
        trait MultiSwitch<T> {
            const MAX_VALUE: u64;
            fn encode(rep: T) -> u64;
        }
        
        impl MultiSwitch<MaybeRegisterRepresentation> for MultiSwitchHelper {
            const MAX_VALUE: u64 = 8;
            fn encode(rep: MaybeRegisterRepresentation) -> u64 {
                let value = rep.value() as u64;
                assert!(value < Self::MAX_VALUE);
                value
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub struct WordRepresentation {
            rep: RegisterRepresentation
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub enum WordRepresentationEnum {
            kWord32,
            kWord64
        }

        impl WordRepresentation {
            pub const fn new(value: WordRepresentationEnum) -> Self {
                WordRepresentation { rep: match value {
                    WordRepresentationEnum::kWord32 => RegisterRepresentation::new(RegisterRepresentationEnum::kWord32),
                    WordRepresentationEnum::kWord64 => RegisterRepresentation::new(RegisterRepresentationEnum::kWord64),
                } }
            }

            pub const fn default() -> Self {
                WordRepresentation { rep: RegisterRepresentation::default() }
            }
            
            pub const fn from_register_representation(rep: RegisterRepresentation) -> Self {
                assert!(rep.rep.is_word());
                WordRepresentation { rep: rep }
            }

            pub const fn word32() -> Self {
                WordRepresentation::new(WordRepresentationEnum::kWord32)
            }
            pub const fn word64() -> Self {
                WordRepresentation::new(WordRepresentationEnum::kWord64)
            }

            pub const fn word_ptr() -> Self {
                WordRepresentation::from_register_representation(RegisterRepresentation::word_ptr())
            }

            pub const fn value(&self) -> MaybeRegisterRepresentationEnum {
                self.rep.value()
            }

            pub fn max_unsigned_value(&self) -> u64 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => u32::MAX as u64,
                    MaybeRegisterRepresentationEnum::kWord64 => u64::MAX,
                    _ => panic!("UNREACHABLE")
                }
            }
            pub fn min_signed_value(&self) -> i64 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => i32::MIN as i64,
                    MaybeRegisterRepresentationEnum::kWord64 => i64::MIN,
                    _ => panic!("UNREACHABLE")
                }
            }
            pub fn max_signed_value(&self) -> i64 {
                match self.value() {
                    MaybeRegisterRepresentationEnum::kWord32 => i32::MAX as i64,
                    MaybeRegisterRepresentationEnum::kWord64 => i64::MAX,
                    _ => panic!("UNREACHABLE")
                }
            }
        }
                
        impl From<RegisterRepresentation> for WordRepresentation {
            fn from(rep: RegisterRepresentation) -> Self {
              WordRepresentation::from_register_representation(rep)
            }
        }
        
        impl std::ops::Deref for WordRepresentation {
            type Target = RegisterRepresentation;
        
            fn deref(&self) -> &Self::Target {
                &self.rep
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub struct FloatRepresentation {
            rep: RegisterRepresentation
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub enum FloatRepresentationEnum {
            kFloat32,
            kFloat64
        }

        impl FloatRepresentation {
            pub const fn new(value: FloatRepresentationEnum) -> Self {
                FloatRepresentation { rep: match value {
                    FloatRepresentationEnum::kFloat32 => RegisterRepresentation::new(RegisterRepresentationEnum::kFloat32),
                    FloatRepresentationEnum::kFloat64 => RegisterRepresentation::new(RegisterRepresentationEnum::kFloat64),
                } }
            }
            
            pub const fn from_register_representation(rep: RegisterRepresentation) -> Self {
                assert!(rep.rep.is_float());
                FloatRepresentation { rep: rep }
            }

            pub const fn float32() -> Self {
                FloatRepresentation::new(FloatRepresentationEnum::kFloat32)
            }
            pub const fn float64() -> Self {
                FloatRepresentation::new(FloatRepresentationEnum::kFloat64)
            }

            pub const fn value(&self) -> MaybeRegisterRepresentationEnum {
                self.rep.value()
            }
            
            pub const fn default() -> Self {
                FloatRepresentation { rep: RegisterRepresentation::default() }
            }
        }
        
        impl From<RegisterRepresentation> for FloatRepresentation {
            fn from(rep: RegisterRepresentation) -> Self {
              FloatRepresentation::from_register_representation(rep)
            }
        }
        
        impl std::ops::Deref for FloatRepresentation {
            type Target = RegisterRepresentation;
        
            fn deref(&self) -> &Self::Target {
                &self.rep
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
        pub struct MemoryRepresentation {
            value_: MemoryRepresentationEnum,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
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
            kInvalid
        }

        impl MemoryRepresentation {
            pub const fn new(value: MemoryRepresentationEnum) -> Self {
                MemoryRepresentation { value_: value }
            }

            pub const fn default() -> Self {
                MemoryRepresentation { value_: MemoryRepresentationEnum::kInvalid }
            }

            pub const fn is_valid(&self) -> bool {
                self.value_ != MemoryRepresentationEnum::kInvalid
            }

            pub const fn value(&self) -> MemoryRepresentationEnum {
                assert!(self.is_valid());
                self.value_
            }

            pub const fn int8() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kInt8)
            }
            pub const fn uint8() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUint8)
            }
            pub const fn int16() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kInt16)
            }
            pub const fn uint16() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUint16)
            }
            pub const fn int32() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kInt32)
            }
            pub const fn uint32() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUint32)
            }
            pub const fn int64() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kInt64)
            }
            pub const fn uint64() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUint64)
            }
            pub const fn uint_ptr() -> Self {
                if Is64() {
                    MemoryRepresentation::uint64()
                } else {
                    MemoryRepresentation::uint32()
                }
            }
            pub const fn float16() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kFloat16)
            }
            pub const fn float32() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kFloat32)
            }
            pub const fn float64() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kFloat64)
            }
            pub const fn any_tagged() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kAnyTagged)
            }
            pub const fn tagged_pointer() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kTaggedPointer)
            }
            pub const fn tagged_signed() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kTaggedSigned)
            }
            pub const fn any_uncompressed_tagged() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kAnyUncompressedTagged)
            }
            pub const fn uncompressed_tagged_pointer() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUncompressedTaggedPointer)
            }
            pub const fn uncompressed_tagged_signed() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kUncompressedTaggedSigned)
            }
            pub const fn protected_pointer() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kProtectedPointer)
            }
            pub const fn indirect_pointer() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kIndirectPointer)
            }
            pub const fn sandboxed_pointer() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kSandboxedPointer)
            }
            pub const fn simd128() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kSimd128)
            }
            pub const fn simd256() -> Self {
                MemoryRepresentation::new(MemoryRepresentationEnum::kSimd256)
            }

            pub fn is_signed(&self) -> bool {
                match self.value() {
                    MemoryRepresentationEnum::kInt8 |
                    MemoryRepresentationEnum::kInt16 |
                    MemoryRepresentationEnum::kInt32 |
                    MemoryRepresentationEnum::kInt64 => true,
                    MemoryRepresentationEnum::kUint8 |
                    MemoryRepresentationEnum::kUint16 |
                    MemoryRepresentationEnum::kUint32 |
                    MemoryRepresentationEnum::kUint64 => false,
                    MemoryRepresentationEnum::kFloat16 |
                    MemoryRepresentation