// src/execution/encoded_c_signature.rs

//use v8::fast_api_calls::*; // Include v8-fast-api-calls crate if needed
use std::num::NonZeroU32;

// Define necessary type enums and structs to support conversion
// CTypeInfo and CFunctionInfo need to be defined based on include/v8-fast-api-calls.h

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SequenceType {
    kScalar,
    // Other sequence types, if needed
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    kFloat64,
    kFloat32,
    // Other types, if needed
}

pub struct CTypeInfo {
    sequence_type: SequenceType,
    type_: Type,
}

impl CTypeInfo {
    pub fn GetSequenceType(&self) -> SequenceType {
        self.sequence_type
    }

    pub fn GetType(&self) -> Type {
        self.type_
    }
}

pub struct CFunctionInfo {
    argument_count: usize,
    argument_infos: Vec<CTypeInfo>,
    return_info: CTypeInfo,
    has_options: bool,
}

impl CFunctionInfo {
    pub fn ArgumentCount(&self) -> usize {
        self.argument_count
    }

    pub fn ArgumentInfo(&self, i: usize) -> &CTypeInfo {
        &self.argument_infos[i]
    }

    pub fn ReturnInfo(&self) -> &CTypeInfo {
        &self.return_info
    }

    pub fn HasOptions(&self) -> bool {
        self.has_options
    }
}

pub fn IsFloatingPointType(type_: Type) -> bool {
    type_ == Type::kFloat64 || type_ == Type::kFloat32
}


const K_RETURN_INDEX: usize = 31; // Assuming it's the last bit, adjust if needed.

/// Represents an encoded C signature, optimized for floating-point parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct EncodedCSignature {
    bitfield_: u32,
    parameter_count_: usize,
}

impl EncodedCSignature {
    /// Creates a new `EncodedCSignature`.
    pub fn new() -> Self {
        EncodedCSignature {
            bitfield_: 0,
            parameter_count_: 0,
        }
    }

    /// Checks if the signature is valid (non-zero).
    pub fn IsValid(&self) -> bool {
        self.bitfield_ != 0 || self.parameter_count_ != 0
    }

    /// Returns the number of floating-point parameters.
    pub fn FPParameterCount(&self) -> usize {
        assert!(self.IsValid());
        (self.bitfield_ & !(1 << K_RETURN_INDEX)).count_ones() as usize
    }

    /// Sets the bit for a floating-point parameter at the given index.
    pub fn SetFloat(&mut self, index: usize) {
        self.bitfield_ |= 1 << index;
    }

    /// Sets the bit for a float64 return type.
    pub fn SetReturnFloat64(&mut self) {
        self.bitfield_ |= 1 << K_RETURN_INDEX;
    }

    /// Sets the bit for a float32 return type.
    pub fn SetReturnFloat32(&mut self) {
        self.bitfield_ |= 1 << K_RETURN_INDEX;
    }

    /// Creates an `EncodedCSignature` from a `CFunctionInfo` struct.
    pub fn from_c_function_info(signature: &CFunctionInfo) -> Self {
        let mut encoded_signature = EncodedCSignature::new();
        encoded_signature.parameter_count_ = signature.ArgumentCount();

        for i in 0..encoded_signature.parameter_count_ {
            if signature.ArgumentInfo(i).GetSequenceType() == SequenceType::kScalar &&
                IsFloatingPointType(signature.ArgumentInfo(i).GetType()) {
                encoded_signature.SetFloat(i);
            }
        }

        if signature.HasOptions() {
            encoded_signature.parameter_count_ += 1;
        }

        if signature.ReturnInfo().GetSequenceType() == SequenceType::kScalar &&
            IsFloatingPointType(signature.ReturnInfo().GetType()) {
            if signature.ReturnInfo().GetType() == Type::kFloat64 {
                encoded_signature.SetReturnFloat64();
            } else {
                encoded_signature.SetReturnFloat32();
            }
        }

        encoded_signature
    }
}