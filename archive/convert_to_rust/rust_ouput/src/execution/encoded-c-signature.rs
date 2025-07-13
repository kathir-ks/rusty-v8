// Converted from V8 C++ source files:
// Header: encoded-c-signature.h
// Implementation: encoded-c-signature.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/execution/encoded-c-signature.h
pub mod encoded_c_signature {
    use std::marker::PhantomData;

    pub struct EncodedCSignature {
        bitfield_: u32,
        return_type_is_float64_: bool,
        parameter_count_: i32,
    }

    impl Default for EncodedCSignature {
        fn default() -> Self {
            Self {
                bitfield_: 0,
                return_type_is_float64_: false,
                parameter_count_: Self::kInvalidParamCount,
            }
        }
    }

    impl EncodedCSignature {
        pub fn new(bitfield: u32, parameter_count: i32) -> Self {
            Self {
                bitfield_: bitfield,
                return_type_is_float64_: false,
                parameter_count_: parameter_count,
            }
        }

        pub fn from_parameter_count(parameter_count: i32) -> Self {
            Self {
                bitfield_: 0,
                return_type_is_float64_: false,
                parameter_count_: parameter_count,
            }
        }

        //Assume CFunctionInfo defined elsewhere, use PhantomData
        pub fn from_c_function_info(_signature: &CFunctionInfo) -> Self {
            // Placeholder implementation
            EncodedCSignature::default()
        }
        

        pub fn is_float(&self, index: i32) -> bool {
            (self.bitfield_ & (1u32 << index)) != 0
        }

        pub fn is_return_float(&self) -> bool {
            self.is_float(Self::kReturnIndex)
        }

        #[cfg(target_arch = "riscv64")]
        pub fn is_return_float64(&self) -> bool {
            self.is_float(Self::kReturnIndex) && self.return_type_is_float64_
        }

        pub fn set_float(&mut self, index: i32) {
            self.bitfield_ |= (1u32 << index);
        }

        pub fn set_return_float64(&mut self) {
            self.set_float(Self::kReturnIndex);
            #[cfg(target_arch = "riscv64")]
            {
                self.return_type_is_float64_ = true;
            }
        }

        pub fn set_return_float32(&mut self) {
            self.set_float(Self::kReturnIndex);
            #[cfg(target_arch = "riscv64")]
            {
                self.return_type_is_float64_ = false;
            }
        }

        pub fn is_valid(&self) -> bool {
            self.parameter_count_ < Self::kInvalidParamCount
        }

        pub fn parameter_count(&self) -> i32 {
            self.parameter_count_
        }

        pub fn fp_parameter_count(&self) -> i32 {
            if !self.is_valid() {
                return 0;
            }
            (self.bitfield_ & !(1 << Self::kReturnIndex)).count_ones() as i32
        }

        pub fn invalid() -> &'static EncodedCSignature {
            static INVALID: EncodedCSignature = EncodedCSignature {
                bitfield_: 0,
                return_type_is_float64_: false,
                parameter_count_: EncodedCSignature::kInvalidParamCount,
            };
            &INVALID
        }

        pub const kReturnIndex: i32 = 31;
        pub const kInvalidParamCount: i32 = Self::kReturnIndex + 1;
    }

    // Assume CFunctionInfo defined elsewhere
    pub struct CFunctionInfo {
        argument_count: usize,
        has_options: bool,
        return_info: CTypeInfo,
        argument_info: Vec<CTypeInfo>,
    }

    impl CFunctionInfo {
        pub fn argument_count(&self) -> usize {
            self.argument_count
        }

        pub fn has_options(&self) -> bool {
            self.has_options
        }

        pub fn return_info(&self) -> &CTypeInfo {
            &self.return_info
        }

        pub fn argument_info(&self, index: usize) -> &CTypeInfo {
            &self.argument_info[index]
        }
    }

    // Assume CTypeInfo defined elsewhere
    pub struct CTypeInfo {
        sequence_type: SequenceType,
        type_: Type,
    }

    impl CTypeInfo {
        pub fn get_sequence_type(&self) -> SequenceType {
            self.sequence_type
        }

        pub fn get_type(&self) -> Type {
            self.type_
        }

        pub fn is_floating_point_type(&self) -> bool {
            match self.type_ {
                Type::kFloat | Type::kFloat64 => true,
                _ => false,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum SequenceType {
        kScalar,
    }

    #[derive(Clone, Copy)]
    pub enum Type {
        kFloat,
        kFloat64,
        kOther,
    }
}

// src/execution/encoded-c-signature.cc
pub mod encoded_c_signature_impl {
    use crate::encoded_c_signature::encoded_c_signature::*;
    //use crate::fast_api_calls::fast_api_calls::*;
    //use crate::base::bits::bits::*;
    //use crate::base::logging::logging::*;

    impl EncodedCSignature {
        pub fn fp_parameter_count(&self) -> i32 {
            if !self.is_valid() {
                return 0;
            }
            (self.bitfield_ & !(1 << Self::kReturnIndex)).count_ones() as i32
        }

        pub fn from_c_function_info(signature: &CFunctionInfo) -> Self {
            let mut encoded_signature = EncodedCSignature::default();
            encoded_signature.parameter_count_ = signature.argument_count() as i32;
            for i in 0..signature.argument_count() {
                if signature.argument_info(i).get_sequence_type() == SequenceType::kScalar
                    && signature.argument_info(i).is_floating_point_type()
                {
                    encoded_signature.set_float(i as i32);
                }
            }

            if signature.has_options() {
                encoded_signature.parameter_count_ += 1;
            }

            if signature.return_info().get_sequence_type() == SequenceType::kScalar
                && signature.return_info().is_floating_point_type()
            {
                if signature.return_info().get_type() == Type::kFloat64 {
                    encoded_signature.set_return_float64();
                } else {
                    encoded_signature.set_return_float32();
                }
            }

            encoded_signature
        }
    }
}
