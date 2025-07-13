// Converted from V8 C++ source files:
// Header: frame-translation-builder.h
// Implementation: frame-translation-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/deoptimizer/frame-translation-builder.h
use std::optional::Option;

use crate::codegen::register::Register;
use crate::deoptimizer::translation_opcode::TranslationOpcode;
//use crate::objects::deoptimization_data::DeoptimizationFrameTranslation;  // Assuming this is the correct path
use crate::zone::zone_containers::ZoneVector;

//#[cfg(V8_ENABLE_WEBASSEMBLY)]
//use crate::wasm::value_type::ValueKind; // Assuming this is the correct path

// Assuming these are defined elsewhere, creating dummy structs to satisfy the compiler
pub struct DeoptimizationFrameTranslation {}
pub struct LocalFactory {}
pub struct Zone { }
pub struct DoubleRegister {}
pub struct FloatRegister {}
pub struct Simd128Register {}

impl DeoptimizationFrameTranslation {
    pub const kUncompressedSizeSize: usize = 4; // Assuming 4 bytes for size
    pub const kUncompressedSizeOffset: usize = 0; // Assuming offset 0
    pub const kCompressedDataOffset: usize = 4; // Assuming offset after size

    pub fn set_int(&mut self, offset: usize, value: usize) {}  // Placeholder
    pub fn begin(&mut self) -> *mut u8 {
        std::ptr::null_mut()
    } // Placeholder
}

impl LocalFactory {
    pub fn new_deoptimization_frame_translation(&self, size: usize) -> DirectHandle<DeoptimizationFrameTranslation> {
        DirectHandle { }
    } // Placeholder
}

pub struct DirectHandle<T> {}

pub mod v8_flags {
    pub static mut turbo_compress_frame_translations: bool = false;
    pub static mut enable_slow_asserts: bool = false;
}

pub mod base {
    pub const kDataMask: u32 = 0xFF; // Some reasonable default value.
}

pub mod kNoWasmReturnKind {
    pub const kNoWasmReturnKind: i32 = -1;  // Some reasonable default value.
}

pub mod StandardFrameConstants {
    pub const kCallerPCOffset: i32 = 8;  // Some reasonable default value.
    pub const kFunctionOffset: i32 = 0;  // Some reasonable default value.
}

pub const kSystemPointerSize: usize = 8;  // Assuming 64 bit system

pub enum CreateArgumentsType {
    MappedArguments,
    UnmappedArguments,
    RestArguments,
}

pub struct BytecodeOffset {
    offset: i32,
}

impl BytecodeOffset {
    pub fn to_int(&self) -> i32 {
        self.offset
    }
}

impl BytecodeOffset {
    pub fn new(offset: i32) -> Self {
        BytecodeOffset { offset }
    }
    pub fn ToInt(&self) -> i32 {
        self.offset
    }
}


pub struct FrameTranslationBuilder {
    contents_: ZoneVector<u8>,
    contents_for_compression_: ZoneVector<i32>,
    basis_instructions_: ZoneVector<Instruction>,
    zone_: *mut Zone,
    matching_instructions_count_: usize,
    total_matching_instructions_in_current_translation_: usize,
    instruction_index_within_translation_: usize,
    index_of_basis_translation_start_: i32,
    match_previous_allowed_: bool,
}

impl FrameTranslationBuilder {
    pub fn new(zone: *mut Zone) -> Self {
        FrameTranslationBuilder {
            contents_: ZoneVector::new(),
            contents_for_compression_: ZoneVector::new(),
            basis_instructions_: ZoneVector::new(),
            zone_: zone,
            matching_instructions_count_: 0,
            total_matching_instructions_in_current_translation_: 0,
            instruction_index_within_translation_: 0,
            index_of_basis_translation_start_: 0,
            match_previous_allowed_: true,
        }
    }

    pub fn to_frame_translation(
        &mut self,
        factory: *mut LocalFactory,
    ) -> DirectHandle<DeoptimizationFrameTranslation> {
        self.finish_pending_instruction_if_needed();

        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                // Compression logic (simplified)
                let input_size = self.size_in_bytes();
                let translation_array_size = input_size + DeoptimizationFrameTranslation::kUncompressedSizeSize;
                let mut result = (*factory).new_deoptimization_frame_translation(translation_array_size);
                (*factory).new_deoptimization_frame_translation(0)
            } else {
                let mut result = (*factory).new_deoptimization_frame_translation(self.size_in_bytes());
                let size_in_bytes = self.size_in_bytes();
                if size_in_bytes > 0 {
                     if self.contents_.len() > 0 {
                        //std::ptr::copy_nonoverlapping(self.contents_.as_ptr(), (*result).begin(), self.contents_.len());
                     }
                }
                result
            }
        }
    }

    pub fn to_frame_translation_wasm(&mut self) -> Vec<u8> {
        unsafe {
            assert!(!v8_flags::turbo_compress_frame_translations);
        }
        self.finish_pending_instruction_if_needed();
        self.contents_.data.clone()
    }

    pub fn begin_translation(
        &mut self,
        frame_count: i32,
        jsframe_count: i32,
        update_feedback: bool,
    ) -> i32 {
        self.finish_pending_instruction_if_needed();
        let start_index = self.size() as i32;
        let mut distance_from_last_start = 0;

        if !self.match_previous_allowed_
            || self.total_matching_instructions_in_current_translation_ as f64
                > (self.instruction_index_within_translation_ as f64 / 4.0 * 3.0)
        {
            distance_from_last_start = start_index - self.index_of_basis_translation_start_;
            self.match_previous_allowed_ = true;
        } else {
            self.basis_instructions_.clear();
            self.index_of_basis_translation_start_ = start_index;
            self.match_previous_allowed_ = false;
        }

        self.total_matching_instructions_in_current_translation_ = 0;
        self.instruction_index_within_translation_ = 0;

        self.add_raw_begin(
            update_feedback,
            UnsignedOperand::new(distance_from_last_start as u32),
            SignedOperand::new(frame_count),
            SignedOperand::new(jsframe_count),
        );
        start_index
    }

    fn finish_pending_instruction_if_needed(&mut self) {
        if self.matching_instructions_count_ > 0 {
            self.total_matching_instructions_in_current_translation_ +=
                self.matching_instructions_count_;

            const K_MAX_SHORTENABLE_OPERAND: usize = 255 - kNumTranslationOpcodes as usize;
            if self.matching_instructions_count_ <= K_MAX_SHORTENABLE_OPERAND {
                self.contents_.push(kNumTranslationOpcodes as u8 + self.matching_instructions_count_ as u8);
            } else {
                self.add_raw_to_contents(
                    TranslationOpcode::MATCH_PREVIOUS_TRANSLATION,
                    UnsignedOperand::new(self.matching_instructions_count_ as u32),
                );
            }
            self.matching_instructions_count_ = 0;
        }
    }

    fn add<T: Operand>(&mut self, opcode: TranslationOpcode, operands: T) {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
               self.add_raw_to_contents_for_compression(opcode, operands);
               return;
            }
        }
        if self.match_previous_allowed_
            && self.instruction_index_within_translation_ < self.basis_instructions_.len()
            && opcode
                == self.basis_instructions_[self.instruction_index_within_translation_].opcode
        {
            self.matching_instructions_count_ += 1;
        } else {
            self.finish_pending_instruction_if_needed();
            self.add_raw_to_contents(opcode, operands);
            if !self.match_previous_allowed_ {
                assert_eq!(
                    self.basis_instructions_.len(),
                    self.instruction_index_within_translation_
                );
                //self.basis_instructions_.push(Instruction::new(opcode, operands)); //TODO: Figure out how to properly clone operands
            }
        }
        self.instruction_index_within_translation_ += 1;
    }

    fn add<T1: Operand, T2: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2) {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operand1, operand2);
                return;
            }
        }
        if self.match_previous_allowed_
            && self.instruction_index_within_translation_ < self.basis_instructions_.len()
            && opcode
                == self.basis_instructions_[self.instruction_index_within_translation_].opcode
        {
            self.matching_instructions_count_ += 1;
        } else {
            self.finish_pending_instruction_if_needed();
            self.add_raw_to_contents(opcode, operand1, operand2);
            if !self.match_previous_allowed_ {
                assert_eq!(
                    self.basis_instructions_.len(),
                    self.instruction_index_within_translation_
                );
                //self.basis_instructions_.push(Instruction::new(opcode, operand1, operand2)); //TODO: Figure out how to properly clone operands
            }
        }
        self.instruction_index_within_translation_ += 1;
    }

        fn add<T1: Operand, T2: Operand, T3: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3) {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operand1, operand2, operand3);
                return;
            }
        }
        if self.match_previous_allowed_
            && self.instruction_index_within_translation_ < self.basis_instructions_.len()
            && opcode
                == self.basis_instructions_[self.instruction_index_within_translation_].opcode
        {
            self.matching_instructions_count_ += 1;
        } else {
            self.finish_pending_instruction_if_needed();
            self.add_raw_to_contents(opcode, operand1, operand2, operand3);
            if !self.match_previous_allowed_ {
                assert_eq!(
                    self.basis_instructions_.len(),
                    self.instruction_index_within_translation_
                );
                //self.basis_instructions_.push(Instruction::new(opcode, operand1, operand2, operand3)); //TODO: Figure out how to properly clone operands
            }
        }
        self.instruction_index_within_translation_ += 1;
    }

        fn add<T1: Operand, T2: Operand, T3: Operand, T4: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3, operand4: T4) {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operand1, operand2, operand3, operand4);
                return;
            }
        }
        if self.match_previous_allowed_
            && self.instruction_index_within_translation_ < self.basis_instructions_.len()
            && opcode
                == self.basis_instructions_[self.instruction_index_within_translation_].opcode
        {
            self.matching_instructions_count_ += 1;
        } else {
            self.finish_pending_instruction_if_needed();
            self.add_raw_to_contents(opcode, operand1, operand2, operand3, operand4);
            if !self.match_previous_allowed_ {
                assert_eq!(
                    self.basis_instructions_.len(),
                    self.instruction_index_within_translation_
                );
                //self.basis_instructions_.push(Instruction::new(opcode, operand1, operand2, operand3, operand4)); //TODO: Figure out how to properly clone operands
            }
        }
        self.instruction_index_within_translation_ += 1;
    }
    fn size(&self) -> usize {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.contents_for_compression_.len()
            } else {
                self.contents_.len()
            }
        }
    }

    fn size_in_bytes(&self) -> usize {
        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.size() * 4 // Assuming kInt32Size = 4
            } else {
                self.size()
            }
        }
    }

    fn zone(&self) -> *mut Zone {
        self.zone_
    }

    fn add_raw_to_contents<T: Operand>(&mut self, opcode: TranslationOpcode, operand: T) {
        unsafe {
            assert!(!v8_flags::turbo_compress_frame_translations);
        }
        self.contents_.push(opcode as u8);
        operand.write_vlq(&mut self.contents_);
    }

    fn add_raw_to_contents<T1: Operand, T2: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2) {
        unsafe {
            assert!(!v8_flags::turbo_compress_frame_translations);
        }
        self.contents_.push(opcode as u8);
        operand1.write_vlq(&mut self.contents_);
        operand2.write_vlq(&mut self.contents_);
    }

        fn add_raw_to_contents<T1: Operand, T2: Operand, T3: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3) {
        unsafe {
            assert!(!v8_flags::turbo_compress_frame_translations);
        }
        self.contents_.push(opcode as u8);
        operand1.write_vlq(&mut self.contents_);
        operand2.write_vlq(&mut self.contents_);
        operand3.write_vlq(&mut self.contents_);
    }

    fn add_raw_to_contents<T1: Operand, T2: Operand, T3: Operand, T4: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3, operand4: T4) {
        unsafe {
            assert!(!v8_flags::turbo_compress_frame_translations);
        }
        self.contents_.push(opcode as u8);
        operand1.write_vlq(&mut self.contents_);
        operand2.write_vlq(&mut self.contents_);
        operand3.write_vlq(&mut self.contents_);
        operand4.write_vlq(&mut self.contents_);
    }

    fn add_raw_to_contents_for_compression<T: Operand>(&mut self, opcode: TranslationOpcode, operand: T) {
        unsafe {
            assert!(v8_flags::turbo_compress_frame_translations);
        }
        self.contents_for_compression_.push(opcode as i32);
        self.contents_for_compression_.push(operand.value() as i32);
    }
    fn add_raw_to_contents_for_compression<T1: Operand, T2: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2) {
        unsafe {
            assert!(v8_flags::turbo_compress_frame_translations);
        }
        self.contents_for_compression_.push(opcode as i32);
        self.contents_for_compression_.push(operand1.value() as i32);
        self.contents_for_compression_.push(operand2.value() as i32);
    }

    fn add_raw_to_contents_for_compression<T1: Operand, T2: Operand, T3: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3) {
        unsafe {
            assert!(v8_flags::turbo_compress_frame_translations);
        }
        self.contents_for_compression_.push(opcode as i32);
        self.contents_for_compression_.push(operand1.value() as i32);
        self.contents_for_compression_.push(operand2.value() as i32);
        self.contents_for_compression_.push(operand3.value() as i32);
    }

    fn add_raw_to_contents_for_compression<T1: Operand, T2: Operand, T3: Operand, T4: Operand>(&mut self, opcode: TranslationOpcode, operand1: T1, operand2: T2, operand3: T3, operand4: T4) {
        unsafe {
            assert!(v8_flags::turbo_compress_frame_translations);
        }
        self.contents_for_compression_.push(opcode as i32);
        self.contents_for_compression_.push(operand1.value() as i32);
        self.contents_for_compression_.push(operand2.value() as i32);
        self.contents_for_compression_.push(operand3.value() as i32);
        self.contents_for_compression_.push(operand4.value() as i32);
    }

    fn add_raw_begin<T: Operand>(&mut self, update_feedback: bool, operand: T) {
        let opcode = if update_feedback {
            TranslationOpcode::BEGIN_WITH_FEEDBACK
        } else {
            TranslationOpcode::BEGIN_WITHOUT_FEEDBACK
        };

        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operand);
            } else {
                self.add_raw_to_contents(opcode, operand);
            }
        }
    }

        fn add_raw_begin<T1: Operand, T2: Operand, T3: Operand>(&mut self, update_feedback: bool, operand1: T1, operand2: T2, operand3: T3) {
        let opcode = if update_feedback {
            TranslationOpcode::BEGIN_WITH_FEEDBACK
        } else {
            TranslationOpcode::BEGIN_WITHOUT_FEEDBACK
        };

        unsafe {
            if v8_flags::turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operand1, operand2, operand3);
            } else {
                self.add_raw_to_contents(opcode, operand1, operand2, operand3);
            }
        }
    }

    pub fn begin_interpreted_frame(
        &mut self,
        bytecode_offset: BytecodeOffset,
        literal_id: i32,
        bytecode_array_id: i32,
        height: u32,
        return_value_offset: i32,
        return_value_count: i32,
    ) {
        if return_value_count == 0 {
            let opcode = TranslationOpcode::INTERPRETED_FRAME_WITHOUT_RETURN;
            self.add(
                opcode,
                SignedOperand::new(bytecode_offset.to_int()),
                SignedOperand::new(literal_id),
                SignedOperand::new(bytecode_array_id),
                UnsignedOperand::new(height),
            );
        } else {
            let opcode = TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN;
            self.add(
                opcode,
                SignedOperand::new(bytecode_offset.to_int()),
                SignedOperand::new(literal_id),
                SignedOperand::new(bytecode_array_id),
                UnsignedOperand::new(height),
                SignedOperand::new(return_value_offset),
                SignedOperand::new(return_value_count),
            );
        }
    }

    pub fn begin_inlined_extra_arguments(
        &mut self,
        literal_id: i32,
        height: u32,
        parameter_count: u32,
    ) {
        let opcode = TranslationOpcode::INLINED_EXTRA_ARGUMENTS;
        self.add(
            opcode,
            SignedOperand::new(literal_id),
            UnsignedOperand::new(height),
            UnsignedOperand::new(parameter_count),
        );
    }

    pub fn begin_construct_create_stub_frame(&mut self, literal_id: i32, height: u32) {
        let opcode = TranslationOpcode::CONSTRUCT_CREATE_STUB_FRAME;
        self.add(
            opcode,
            SignedOperand::new(literal_id),
            UnsignedOperand::new(height),
        );
    }

    pub fn begin_construct_invoke_stub_frame(&mut self, literal_id: i32) {
        let opcode = TranslationOpcode::CONSTRUCT_INVOKE_STUB_FRAME;
        self.add(opcode, SignedOperand::new(literal_id));
    }

    pub fn begin_builtin_continuation_frame(
        &mut self,
        bytecode_offset: BytecodeOffset,
        literal_id: i32,
        height: u32,
    ) {
        let opcode = TranslationOpcode::BUILTIN_CONTINUATION_FRAME;
        self.add(
            opcode,
            SignedOperand::new(bytecode_offset.to_int()),
            SignedOperand::new(literal_id),
            UnsignedOperand::new(height),
        );
    }

    pub fn begin_javascript_builtin_continuation_frame(
        &mut self,
        bytecode_offset: BytecodeOffset,
        literal_id: i32,
        height: u32,
    ) {
        let opcode = TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_FRAME;
        self.add(
            opcode,
            SignedOperand::new(bytecode_offset.to_int()),
            SignedOperand::new(literal_id),
            UnsignedOperand::new(height),
        );
    }

    pub fn begin_javascript_builtin_continuation_with_catch_frame(
        &mut self,
        bytecode_offset: BytecodeOffset,
        literal_id: i32,
        height: u32,
    ) {
        let opcode = TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME;
        self.add(
            opcode,
            SignedOperand::new(bytecode_offset.to_int()),
            SignedOperand::new(literal_id),
            UnsignedOperand::new(height),
        );
    }

    pub fn arguments_elements(&mut self, type_: CreateArgumentsType) {
        let opcode = TranslationOpcode::ARGUMENTS_ELEMENTS;
        self.add(opcode, SignedOperand::new(type_ as u8 as i32));
    }

    pub fn arguments_length(&mut self) {
        let opcode = TranslationOpcode::ARGUMENTS_LENGTH;
        self.add(opcode);
    }

    pub fn rest_length(&mut self) {
        let opcode = TranslationOpcode::REST_LENGTH;
        self.add(opcode);
    }

    pub fn begin_captured_object(&mut self, length: i32) {
        let opcode = TranslationOpcode::CAPTURED_OBJECT;
        self.add(opcode, SignedOperand::new(length));
    }

    pub fn duplicate_object(&mut self, object_index: i32) {
        let opcode = TranslationOpcode::DUPLICATED_OBJECT;
        self.add(opcode, SignedOperand::new(object_index));
    }

    pub fn string_concat(&mut self) {
        let opcode = TranslationOpcode::STRING_CONCAT;
        self.add(opcode);
    }

    fn store_register(&mut self, opcode: TranslationOpcode, reg: Register) {
        self.add(opcode, SmallUnsignedOperand::new(reg.code() as u32));
    }

    pub fn store_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_int32_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::INT32_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_int_ptr_register(&mut self, reg: Register) {
        let opcode = if kSystemPointerSize == 4 {
            TranslationOpcode::INT32_REGISTER
        } else {
            TranslationOpcode::INT64_REGISTER
        };
        self.store_register(opcode, reg);
    }

    pub fn store_int64_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::INT64_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_signed_bigint64_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::SIGNED_BIGINT64_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_unsigned_bigint64_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::UNSIGNED_BIGINT64_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_uint32_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::UINT32_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_bool_register(&mut self, reg: Register) {
        let opcode = TranslationOpcode::BOOL_REGISTER;
        self.store_register(opcode, reg);
    }

    pub fn store_float_register(&mut self, reg: FloatRegister) {
        let opcode = TranslationOpcode::FLOAT_REGISTER;
        self.add(opcode, SmallUnsignedOperand::new(reg.code() as u32));
    }

    pub fn store_double_register(&mut self, reg: DoubleRegister) {
        let opcode = TranslationOpcode::DOUBLE_REGISTER;
        self.add(opcode, SmallUnsignedOperand::new(reg.code() as u32));
    }

    pub fn store_holey_double_register(&mut self, reg: DoubleRegister) {
        let opcode = TranslationOpcode::HOLEY_DOUBLE_REGISTER;
        self.add(opcode, SmallUnsignedOperand::new(reg.code() as u32));
    }

    pub fn store_simd128_register(&mut self, reg: Simd128Register) {
        let opcode = TranslationOpcode::SIMD128_REGISTER;
        self.add(opcode, SmallUnsignedOperand::new(reg.code() as u32));
    }

    pub fn store_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::TAGGED_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_int32_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::INT32_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_int_ptr_stack_slot(&mut self, index: i32) {
        let opcode = if kSystemPointerSize == 4 {
            TranslationOpcode::INT32_STACK_SLOT
        } else {
            TranslationOpcode::INT64_STACK_SLOT
        };
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_int64_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::INT64_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_signed_bigint64_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::SIGNED_BIGINT64_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_unsigned_bigint64_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::UNSIGNED_BIGINT64_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_uint32_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::UINT32_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_bool_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::BOOL_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_float_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::FLOAT_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_double_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::DOUBLE_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_simd128_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::SIMD128_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_holey_double_stack_slot(&mut self, index: i32) {
        let opcode = TranslationOpcode::HOLEY_DOUBLE_STACK_SLOT;
        self.add(opcode, SignedOperand::new(index));
    }

    pub fn store_literal(&mut self, literal_id: i32) {
        let opcode = TranslationOpcode::LITERAL;
        assert!(literal_id >= 0);
        self.add(opcode, SignedOperand::new(literal_id));
    }

    pub fn store_optimized_out(&mut self) {
        let opcode = TranslationOpcode::OPTIMIZED_OUT;
        self.add(opcode);
    }

    pub fn add_update_feedback(&mut self, vector_literal: i32, slot: i32) {
        let opcode = TranslationOpcode::UPDATE_FEEDBACK;
        self.add(opcode, SignedOperand::new(vector_literal), SignedOperand::new(slot));
    }

    pub fn store_js_frame_function(&mut self) {
        self.store_stack_slot(
            (StandardFrameConstants::kCallerPCOffset - StandardFrameConstants::kFunctionOffset)
                / kSystemPointerSize as i32,
        );
    }
}

trait Operand {
    fn value(&self) -> u32;
    fn write_vlq(&self, buffer: &mut ZoneVector<u8>);
}

struct SmallUnsignedOperand {
    value_: u32,
}

impl SmallUnsignedOperand {
    fn new(value: u32) -> Self {
        assert!(value <= base::kDataMask);
        SmallUnsignedOperand { value_: value }
    }
}

impl Operand for SmallUnsignedOperand {
    fn value(&self) -> u32 {
        self.value_
    }

    fn write_vlq(&self, buffer: &mut ZoneVector<u8>) {
        buffer.push(self.value_ as u8);
    }
}

struct UnsignedOperand {
    value_: u32,
}

impl UnsignedOperand {
    fn new(value: i32) -> Self {
        assert!(value >= 0);
        UnsignedOperand {
            value_: value as u32,
        }
    }
    fn new_u32(value: u32) -> Self {
        UnsignedOperand {
            value_: value,
        }
    }
        
    fn new(value: u32) -> Self {
        UnsignedOperand {
            value_: value,
        }
    }
}

impl Operand for UnsignedOperand {
    fn value(&self) -> u32 {
        self.value_
    }

    fn write_vlq(&self, buffer: &mut ZoneVector<u8>) {
         vlq::encode(self.value_ as u64, |byte| buffer.push(byte));
    }
}

struct SignedOperand {
    value_: i32,
}

impl SignedOperand {
    fn new(value: i32) -> Self {
        SignedOperand { value_: value }
    }
}

impl Operand for SignedOperand {
    fn value(&self) -> u32 {
        self.value_ as u32
    }

    fn write_vlq(&self, buffer: &mut ZoneVector<u8>) {
        vlq::encode_signed(self.value_ as i64, |byte| buffer.push(byte));
    }
}

mod vlq {

