// src/codegen/machine_type.rs

use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MachineRepresentation {
    kNone,
    kBit,
    kWord8,
    kWord16,
    kWord32,
    kWord64,
    kFloat16,
    kFloat16RawBits,
    kFloat32,
    kFloat64,
    kSimd128,
    kSimd256,
    kTaggedSigned,
    kTaggedPointer,
    kTagged,
    kCompressedPointer,
    kCompressed,
    kProtectedPointer,
    kIndirectPointer,
    kMapWord,
    kSandboxedPointer,
}

impl MachineRepresentation {
    pub fn to_string(&self) -> &'static str {
        match self {
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
}

impl fmt::Display for MachineRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl fmt::Display for MachineSemantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MachineType {
    representation: MachineRepresentation,
    semantic: MachineSemantic,
}

impl MachineType {
    pub fn new(representation: MachineRepresentation, semantic: MachineSemantic) -> Self {
        MachineType { representation, semantic }
    }

    pub fn representation(&self) -> MachineRepresentation {
        self.representation
    }

    pub fn semantic(&self) -> MachineSemantic {
        self.semantic
    }

    pub fn none() -> Self {
        MachineType {
            representation: MachineRepresentation::kNone,
            semantic: MachineSemantic::kNone,
        }
    }
}

impl fmt::Display for MachineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &MachineType::none() {
            return Ok(());
        } else if self.representation == MachineRepresentation::kNone {
            write!(f, "{}", self.semantic)
        } else if self.semantic == MachineSemantic::kNone {
            write!(f, "{}", self.representation)
        } else {
            write!(f, "{}|{}", self.representation, self.semantic)
        }
    }
}

pub fn is_subtype(rep1: MachineRepresentation, rep2: MachineRepresentation) -> bool {
    if rep1 == rep2 {
        return true;
    }
    match rep1 {
        MachineRepresentation::kTaggedSigned | MachineRepresentation::kTaggedPointer => {
            rep2 == MachineRepresentation::kTagged
        }
        MachineRepresentation::kCompressedPointer => rep2 == MachineRepresentation::kCompressed,
        _ => false,
    }
}