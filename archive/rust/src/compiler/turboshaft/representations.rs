// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod representations {
    use std::fmt;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaybeRegisterRepresentation {
        Word32,
        Word64,
        Float32,
        Float64,
        Tagged,
        Compressed,
        Simd128,
        Simd256,
        None,
    }

    impl fmt::Display for MaybeRegisterRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MaybeRegisterRepresentation::Word32 => write!(f, "Word32"),
                MaybeRegisterRepresentation::Word64 => write!(f, "Word64"),
                MaybeRegisterRepresentation::Float32 => write!(f, "Float32"),
                MaybeRegisterRepresentation::Float64 => write!(f, "Float64"),
                MaybeRegisterRepresentation::Tagged => write!(f, "Tagged"),
                MaybeRegisterRepresentation::Compressed => write!(f, "Compressed"),
                MaybeRegisterRepresentation::Simd128 => write!(f, "Simd128"),
                MaybeRegisterRepresentation::Simd256 => write!(f, "Simd256"),
                MaybeRegisterRepresentation::None => write!(f, "None"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MemoryRepresentation {
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
        AnyUncompressedTagged,
        UncompressedTaggedPointer,
        UncompressedTaggedSigned,
        ProtectedPointer,
        IndirectPointer,
        SandboxedPointer,
        Simd128,
        Simd256,
    }

    impl fmt::Display for MemoryRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MemoryRepresentation::Int8 => write!(f, "Int8"),
                MemoryRepresentation::Uint8 => write!(f, "Uint8"),
                MemoryRepresentation::Int16 => write!(f, "Int16"),
                MemoryRepresentation::Uint16 => write!(f, "Uint16"),
                MemoryRepresentation::Int32 => write!(f, "Int32"),
                MemoryRepresentation::Uint32 => write!(f, "Uint32"),
                MemoryRepresentation::Int64 => write!(f, "Int64"),
                MemoryRepresentation::Uint64 => write!(f, "Uint64"),
                MemoryRepresentation::Float16 => write!(f, "Float16"),
                MemoryRepresentation::Float32 => write!(f, "Float32"),
                MemoryRepresentation::Float64 => write!(f, "Float64"),
                MemoryRepresentation::AnyTagged => write!(f, "AnyTagged"),
                MemoryRepresentation::TaggedPointer => write!(f, "TaggedPointer"),
                MemoryRepresentation::TaggedSigned => write!(f, "TaggedSigned"),
                MemoryRepresentation::AnyUncompressedTagged => write!(f, "AnyUncompressedTagged"),
                MemoryRepresentation::UncompressedTaggedPointer => write!(f, "UncompressedTaggedPointer"),
                MemoryRepresentation::UncompressedTaggedSigned => write!(f, "UncompressedTaggedSigned"),
                MemoryRepresentation::ProtectedPointer => write!(f, "ProtectedPointer"),
                MemoryRepresentation::IndirectPointer => write!(f, "IndirectPointer"),
                MemoryRepresentation::SandboxedPointer => write!(f, "SandboxedPointer"),
                MemoryRepresentation::Simd128 => write!(f, "Simd128"),
                MemoryRepresentation::Simd256 => write!(f, "Simd256"),
            }
        }
    }
}