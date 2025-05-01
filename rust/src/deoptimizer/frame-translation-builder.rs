// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/deoptimizer/frame-translation-builder.h
pub mod frame_translation_builder {
    use std::optional::Option;
    use std::vec::Vec;

    // src/base/vlq.h
    pub mod vlq {
        pub const kDataMask: u32 = 0x7F;
        pub const kContinuationBit: u8 = 0x80;

        pub fn VLQEncodeUnsigned(mut write_byte: impl FnMut(u8), value: u32) {
            let mut v = value;
            loop {
                let mut digit = (v & kDataMask) as u8;
                v >>= 7;
                if v != 0 {
                    digit |= kContinuationBit;
                }
                write_byte(digit);
                if v == 0 {
                    break;
                }
            }
        }

        pub fn VLQEncode(mut write_byte: impl FnMut(u8) -> &mut u8, value: i32) {
            VLQEncodeUnsigned(
                |byte| {
                    write_byte(byte);
                },
                zig_zag_encode(value) as u32,
            );
        }

        fn zig_zag_encode(value: i32) -> i32 {
            if value >= 0 {
                value << 1
            } else {
                (value << 1).wrapping_neg()
            }
        }
    }

    // src/deoptimizer/translated-state.h
    pub mod translated_state {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum TranslationOpcode {
            BEGIN_WITH_FEEDBACK,
            BEGIN_WITHOUT_FEEDBACK,
            MATCH_PREVIOUS_TRANSLATION,
            ARGUMENTS_ELEMENTS,
            ARGUMENTS_LENGTH,
            REST_LENGTH,
            CAPTURED_OBJECT,
            DUPLICATED_OBJECT,
            STRING_CONCAT,
            REGISTER,
            INT32_REGISTER,
            INT64_REGISTER,
            SIGNED_BIGINT64_REGISTER,
            UNSIGNED_BIGINT64_REGISTER,
            UINT32_REGISTER,
            BOOL_REGISTER,
            FLOAT_REGISTER,
            DOUBLE_REGISTER,
            HOLEY_DOUBLE_REGISTER,
            SIMD128_REGISTER,
            TAGGED_STACK_SLOT,
            INT32_STACK_SLOT,
            INT64_STACK_SLOT,
            SIGNED_BIGINT64_STACK_SLOT,
            UNSIGNED_BIGINT64_STACK_SLOT,
            UINT32_STACK_SLOT,
            BOOL_STACK_SLOT,
            FLOAT_STACK_SLOT,
            DOUBLE_STACK_SLOT,
            SIMD128_STACK_SLOT,
            HOLEY_DOUBLE_STACK_SLOT,
            LITERAL,
            OPTIMIZED_OUT,
            UPDATE_FEEDBACK,
            BUILTIN_CONTINUATION_FRAME,
            JS_TO_WASM_BUILTIN_CONTINUATION_FRAME,
            WASM_INLINED_INTO_JS_FRAME,
            LIFTOFF_FRAME,
            JAVASCRIPT_BUILTIN_CONTINUATION_FRAME,
            JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME,
            CONSTRUCT_CREATE_STUB_FRAME,
            CONSTRUCT_INVOKE_STUB_FRAME,
            INLINED_EXTRA_ARGUMENTS,
            INTERPRETED_FRAME_WITHOUT_RETURN,
            INTERPRETED_FRAME_WITH_RETURN,
        }

        impl TranslationOpcode {
            pub fn operand_count(&self) -> usize {
                match self {
                    TranslationOpcode::BEGIN_WITH_FEEDBACK => 3,
                    TranslationOpcode::BEGIN_WITHOUT_FEEDBACK => 3,
                    TranslationOpcode::MATCH_PREVIOUS_TRANSLATION => 1,
                    TranslationOpcode::ARGUMENTS_ELEMENTS => 1,
                    TranslationOpcode::ARGUMENTS_LENGTH => 0,
                    TranslationOpcode::REST_LENGTH => 0,
                    TranslationOpcode::CAPTURED_OBJECT => 1,
                    TranslationOpcode::DUPLICATED_OBJECT => 1,
                    TranslationOpcode::STRING_CONCAT => 0,
                    TranslationOpcode::REGISTER => 1,
                    TranslationOpcode::INT32_REGISTER => 1,
                    TranslationOpcode::INT64_REGISTER => 1,
                    TranslationOpcode::SIGNED_BIGINT64_REGISTER => 1,
                    TranslationOpcode::UNSIGNED_BIGINT64_REGISTER => 1,
                    TranslationOpcode::UINT32_REGISTER => 1,
                    TranslationOpcode::BOOL_REGISTER => 1,
                    TranslationOpcode::FLOAT_REGISTER => 1,
                    TranslationOpcode::DOUBLE_REGISTER => 1,
                    TranslationOpcode::HOLEY_DOUBLE_REGISTER => 1,
                    TranslationOpcode::SIMD128_REGISTER => 1,
                    TranslationOpcode::TAGGED_STACK_SLOT => 1,
                    TranslationOpcode::INT32_STACK_SLOT => 1,
                    TranslationOpcode::INT64_STACK_SLOT => 1,
                    TranslationOpcode::SIGNED_BIGINT64_STACK_SLOT => 1,
                    TranslationOpcode::UNSIGNED_BIGINT64_STACK_SLOT => 1,
                    TranslationOpcode::UINT32_STACK_SLOT => 1,
                    TranslationOpcode::BOOL_STACK_SLOT => 1,
                    TranslationOpcode::FLOAT_STACK_SLOT => 1,
                    TranslationOpcode::DOUBLE_STACK_SLOT => 1,
                    TranslationOpcode::SIMD128_STACK_SLOT => 1,
                    TranslationOpcode::HOLEY_DOUBLE_STACK_SLOT => 1,
                    TranslationOpcode::LITERAL => 1,
                    TranslationOpcode::OPTIMIZED_OUT => 0,
                    TranslationOpcode::UPDATE_FEEDBACK => 2,
                    TranslationOpcode::BUILTIN_CONTINUATION_FRAME => 3,
                    TranslationOpcode::JS_TO_WASM_BUILTIN_CONTINUATION_FRAME => 4,
                    TranslationOpcode::WASM_INLINED_INTO_JS_FRAME => 3,
                    TranslationOpcode::LIFTOFF_FRAME => 3,
                    TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_FRAME => 3,
                    TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME => 3,
                    TranslationOpcode::CONSTRUCT_CREATE_STUB_FRAME => 2,
                    TranslationOpcode::CONSTRUCT_INVOKE_STUB_FRAME => 1,
                    TranslationOpcode::INLINED_EXTRA_ARGUMENTS => 3,
                    TranslationOpcode::INTERPRETED_FRAME_WITHOUT_RETURN => 4,
                    TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN => 6,
                }
            }
        }
    }

    use self::translated_state::TranslationOpcode;

    // src/objects/fixed-array-inl.h
    pub mod fixed_array_inl {
        // This is a placeholder, as the FixedArray is a V8 concept
        // that would be replaced with a Rust Vec<T> or similar.
    }

    // third_party/zlib/google/compression_utils_portable.h
    pub mod compression_utils_portable {
        // This is a placeholder, as zlib is a C library,
        // and would be accessed through a Rust crate like `flate2`.
    }

    // Assuming base::kDataMask is defined somewhere, likely as a constant.
    pub const K_DATA_MASK: u32 = 0x7F;

    // Mock definitions for V8 types
    pub struct Register {
        code_: u8,
    }

    impl Register {
        pub fn code(&self) -> u8 {
            self.code_
        }
    }

    pub struct FloatRegister {
        code_: u8,
    }

    impl FloatRegister {
        pub fn code(&self) -> u8 {
            self.code_
        }
    }

    pub struct DoubleRegister {
        code_: u8,
    }

    impl DoubleRegister {
        pub fn code(&self) -> u8 {
            self.code_
        }
    }

    pub struct Simd128Register {
        code_: u8,
    }

    impl Simd128Register {
        pub fn code(&self) -> u8 {
            self.code_
        }
    }

    pub struct BytecodeOffset {
        offset: i32,
    }

    impl BytecodeOffset {
        pub fn ToInt(&self) -> i32 {
            self.offset
        }

        pub fn new(offset: i32) -> Self {
            BytecodeOffset { offset }
        }
    }

    pub enum CreateArgumentsType {
        MappedArguments,
        UnmappedArguments,
        RestParameter,
    }

    pub struct LocalFactory {}

    impl LocalFactory {
        pub fn NewDeoptimizationFrameTranslation(&self, size: usize) -> DeoptimizationFrameTranslation {
            DeoptimizationFrameTranslation::new(size)
        }
    }

    // Mock implementation for DeoptimizationFrameTranslation, since it is V8 specific.
    #[derive(Debug)]
    pub struct DeoptimizationFrameTranslation {
        data: Vec<u8>,
    }

    impl DeoptimizationFrameTranslation {
        const K_UNCOMPRESSED_SIZE_OFFSET: usize = 0;
        const K_COMPRESSED_DATA_OFFSET: usize = 4; // Assuming 4 bytes for uncompressed size.

        fn new(size: usize) -> Self {
            DeoptimizationFrameTranslation { data: vec![0; size] }
        }

        fn begin(&mut self) -> &mut [u8] {
            &mut self.data
        }

        fn set_int(&mut self, offset: usize, value: usize) {
            self.data[offset..offset + std::mem::size_of::<usize>()].copy_from_slice(&value.to_ne_bytes());
        }
    }

    // Mock implementation for DeoptTranslationIterator
    pub struct DeoptTranslationIterator<'a> {
        data: &'a [u8],
        current_index: usize,
    }

    impl<'a> DeoptTranslationIterator<'a> {
        pub fn new(data: &'a [u8], start_index: usize) -> Self {
            DeoptTranslationIterator {
                data,
                current_index: start_index,
            }
        }

        pub fn HasNextOpcode(&self) -> bool {
            self.current_index < self.data.len()
        }

        pub fn NextOpcode(&mut self) -> TranslationOpcode {
            // Dummy implementation for opcode retrieval
            if self.current_index < self.data.len() {
                self.current_index += 1;
                TranslationOpcode::REGISTER // Replace with proper logic
            } else {
                panic!("No more opcodes");
            }
        }

        pub fn NextOperand(&mut self) -> u32 {
            // Dummy implementation for operand retrieval
            if self.current_index < self.data.len() {
                self.current_index += 1;
                0 // Replace with proper logic
            } else {
                panic!("No more operands");
            }
        }

        pub fn NextOperandUnsigned(&mut self) -> u32 {
            // Dummy implementation for unsigned operand retrieval
            if self.current_index < self.data.len() {
                self.current_index += 1;
                0 // Replace with proper logic
            } else {
                panic!("No more operands");
            }
        }
    }

    // Mock implementation for StandardFrameConstants
    pub mod StandardFrameConstants {
        pub const kCallerPCOffset: i32 = 8;
        pub const kFunctionOffset: i32 = 16;
    }

    pub const K_SYSTEM_POINTER_SIZE: usize = 8; //Example Value, can be 4 or 8
    pub const K_NUM_TRANSLATION_OPCODES: usize = 50; // Example value

    // Mock implementations for wasm types
    pub mod wasm {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum ValueKind {
            I32,
            I64,
            F32,
            F64,
            V128,
            Ref,
        }
    }

    pub const K_NO_WASM_RETURN_KIND: i32 = -1;

    lazy_static::lazy_static! {
        pub static ref V8_FLAGS: V8Flags = V8Flags {
            turbo_compress_frame_translations: false,
            enable_slow_asserts: false,
        };
    }

    pub struct V8Flags {
        turbo_compress_frame_translations: bool,
        enable_slow_asserts: bool,
    }

    #[derive(Debug)]
    struct Instruction {
        opcode: TranslationOpcode,
        operands: Vec<u32>,
        is_operand_signed: Vec<bool>,
    }

    impl Instruction {
        fn new(opcode: TranslationOpcode, operands: Vec<u32>, is_operand_signed: Vec<bool>) -> Self {
            Instruction {
                opcode,
                operands,
                is_operand_signed,
            }
        }
    }

    pub struct FrameTranslationBuilder {
        contents_: Vec<u8>,
        contents_for_compression_: Vec<u8>,
        basis_instructions_: Vec<Instruction>,
        index_of_basis_translation_start_: usize,
        instruction_index_within_translation_: usize,
        matching_instructions_count_: usize,
        total_matching_instructions_in_current_translation_: usize,
        match_previous_allowed_: bool,
        all_instructions_: Vec<Instruction>,
    }

    impl FrameTranslationBuilder {
        pub fn new() -> Self {
            FrameTranslationBuilder {
                contents_: Vec::new(),
                contents_for_compression_: Vec::new(),
                basis_instructions_: Vec::new(),
                index_of_basis_translation_start_: 0,
                instruction_index_within_translation_: 0,
                matching_instructions_count_: 0,
                total_matching_instructions_in_current_translation_: 0,
                match_previous_allowed_: true,
                all_instructions_: Vec::new(),
            }
        }

        fn add_raw_to_contents<T: Operand>(&mut self, opcode: TranslationOpcode, operands: Vec<T>) {
            assert_eq!(operands.len(), opcode.operand_count());
            assert!(!V8_FLAGS.turbo_compress_frame_translations);
            self.contents_.push(opcode as u8);
            for operand in operands {
                operand.write_vlq(&mut self.contents_);
            }
        }

        fn add_raw_to_contents_for_compression<T: Operand>(&mut self, opcode: TranslationOpcode, operands: Vec<T>) {
            assert_eq!(operands.len(), opcode.operand_count());
            assert!(V8_FLAGS.turbo_compress_frame_translations);
            self.contents_for_compression_.push(opcode as u8);
            for operand in operands {
                self.contents_for_compression_.push(operand.value() as u8);
            }
        }

        fn add_raw_begin<T: Operand>(&mut self, update_feedback: bool, operands: Vec<T>) {
            let opcode = if update_feedback {
                TranslationOpcode::BEGIN_WITH_FEEDBACK
            } else {
                TranslationOpcode::BEGIN_WITHOUT_FEEDBACK
            };

            if V8_FLAGS.turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operands);
            } else {
                self.add_raw_to_contents(opcode, operands);
                if V8_FLAGS.enable_slow_asserts {
                    let operands_values: Vec<u32> = operands.iter().map(|o| o.value()).collect();
                    let is_signed: Vec<bool> = operands.iter().map(|o| o.is_signed()).collect();
                    self.all_instructions_.push(Instruction::new(opcode, operands_values, is_signed));
                }
            }
        }

        pub fn begin_translation(&mut self, frame_count: i32, jsframe_count: i32, update_feedback: bool) -> usize {
            self.finish_pending_instruction_if_needed();
            let start_index = self.Size();
            let mut distance_from_last_start = 0;

            if !self.match_previous_allowed_ ||
                self.total_matching_instructions_in_current_translation_ >
                    self.instruction_index_within_translation_ / 4 * 3 {
                distance_from_last_start = start_index - self.index_of_basis_translation_start_;
                self.match_previous_allowed_ = true;
            } else {
                self.basis_instructions_.clear();
                self.index_of_basis_translation_start_ = start_index;
                self.match_previous_allowed_ = false;
            }

            self.total_matching_instructions_in_current_translation_ = 0;
            self.instruction_index_within_translation_ = 0;

            self.add_raw_begin(update_feedback, vec![
                UnsignedOperand::new(distance_from_last_start as u32),
                SignedOperand::new(frame_count),
                SignedOperand::new(jsframe_count),
            ]);

            start_index
        }

        fn finish_pending_instruction_if_needed(&mut self) {
            if self.matching_instructions_count_ > 0 {
                self.total_matching_instructions_in_current_translation_ += self.matching_instructions_count_;

                let k_max_shortenable_operand = (std::u8::MAX as usize) - K_NUM_TRANSLATION_OPCODES;
                if self.matching_instructions_count_ <= k_max_shortenable_operand {
                    self.contents_.push((K_NUM_TRANSLATION_OPCODES + self.matching_instructions_count_) as u8);
                } else {
                    self.add_raw_to_contents(
                        TranslationOpcode::MATCH_PREVIOUS_TRANSLATION,
                        vec![UnsignedOperand::new(self.matching_instructions_count_ as u32)],
                    );
                }
                self.matching_instructions_count_ = 0;
            }
        }

        fn add<T: Operand>(&mut self, opcode: TranslationOpcode, operands: Vec<T>) {
            assert_eq!(operands.len(), opcode.operand_count());

            if V8_FLAGS.turbo_compress_frame_translations {
                self.add_raw_to_contents_for_compression(opcode, operands);
                return;
            }

            if V8_FLAGS.enable_slow_asserts {
                let operands_values: Vec<u32> = operands.iter().map(|o| o.value()).collect();
                let is_signed: Vec<bool> = operands.iter().map(|o| o.is_signed()).collect();
                self.all_instructions_.push(Instruction::new(opcode, operands_values, is_signed));
            }

            if self.match_previous_allowed_ &&
                self.instruction_index_within_translation_ < self.basis_instructions_.len() &&
                opcode == self.basis_instructions_[self.instruction_index_within_translation_].opcode &&
                Self::operands_equal(&self.basis_instructions_[self.instruction_index_within_translation_].operands, &operands) {
                self.matching_instructions_count_ += 1;
            } else {
                self.finish_pending_instruction_if_needed();
                self.add_raw_to_contents(opcode, operands);
                if !self.match_previous_allowed_ {
                    assert_eq!(self.basis_instructions_.len(), self.instruction_index_within_translation_);
                    let operands_values: Vec<u32> = operands.iter().map(|o| o.value()).collect();
                    let is_signed: Vec<bool> = operands.iter().map(|o| o.is_signed()).collect();
                    self.basis_instructions_.push(Instruction::new(opcode, operands_values, is_signed));
                }
            }
            self.instruction_index_within_translation_ += 1;
        }

        fn operands_equal<T: Operand>(expected_operands: &Vec<u32>, operands: &Vec<T>) -> bool {
            if expected_operands.len() != operands.len() {
                return false;
            }

            for (expected, operand) in expected_operands.iter().zip(operands.iter()) {
                if *expected != operand.value() {
                    return false;
                }
            }

            true
        }

        // Need to mock LocalFactory and DirectHandle
        pub fn to_frame_translation(&mut self, factory: &LocalFactory) -> DeoptimizationFrameTranslation {
            if V8_FLAGS.turbo_compress_frame_translations {
                // Placeholder for zlib compression. Need to find Rust equivalent and integrate.
                /*
                let input_size = self.SizeInBytes();
                let compressed_data_size = compressBound(input_size);

                let mut compressed_data: Vec<u8> = vec![0; compressed_data_size];

                CHECK_EQ(
                    zlib_internal::CompressHelper(
                        zlib_internal::ZRAW,
                        compressed_data.data(),
                        &compressed_data_size,
                        reinterpret_cast<const Bytef*>(self.contents_for_compression_.data()),
                        input_size,
                        Z_DEFAULT_COMPRESSION,
                        nullptr,
                        nullptr,
                    ),
                    Z_OK,
                );

                let translation_array_size = compressed_data_size + DeoptimizationFrameTranslation::kUncompressedSizeSize;
                let result = factory.NewDeoptimizationFrameTranslation(translation_array_size);

                result.set_int(DeoptimizationFrameTranslation::kUncompressedSizeOffset, self.Size());
                std::memcpy(
                    result.begin() + DeoptimizationFrameTranslation::kCompressedDataOffset,
                    compressed_data.data(),
                    compressed_data_size,
                );

                return result;
                */
                todo!("Implement ZLIB Compression");
            }

            assert!(!V8_FLAGS.turbo_compress_frame_translations);
            self.finish_pending_instruction_if_needed();
            let mut result = factory.NewDeoptimizationFrameTranslation(self.SizeInBytes());
            if self.SizeInBytes() == 0 { return result; }
            result.data[..self.contents_.len()].copy_from_slice(&self.contents_);

            if V8_FLAGS.enable_slow_asserts {
                let mut iter = DeoptTranslationIterator::new(&result.data, 0);
                self.validate_bytes(&mut iter);
            }

            result
        }

        pub fn to_frame_translation_wasm(&mut self) -> Vec<u8> {
            assert!(!V8_FLAGS.turbo_compress_frame_translations);
            self.finish_pending_instruction_if_needed();
            let result = self.contents_.clone();

            if V8_FLAGS.enable_slow_asserts {
                let mut iter = DeoptTranslationIterator::new(&result, 0);
                self.validate_bytes(&mut iter);
            }

            result
        }

        fn validate_bytes(&self, iter: &mut DeoptTranslationIterator) {
            if V8_FLAGS.enable_slow_asserts {
                for instruction in &self.all_instructions_ {
                    assert!(iter.HasNextOpcode());
                    assert_eq!(instruction.opcode, iter.NextOpcode());
                    for j in 0..instruction.opcode.operand_count() {
                        let operand = if instruction.is_operand_signed[j] {
                            iter.NextOperand()
                        } else {
                            iter.NextOperandUnsigned()
                        };
                        assert_eq!(instruction.operands[j], operand);
                    }
                }
            }
        }

        pub fn begin_builtin_continuation_frame(&mut self, bytecode_offset: BytecodeOffset, literal_id: i32, height: u32) {
            let opcode = TranslationOpcode::BUILTIN_CONTINUATION_FRAME;
            self.add(opcode, vec![SignedOperand::new(bytecode_offset.ToInt()), SignedOperand::new(literal_id), UnsignedOperand::new(height)]);
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn begin_js_to_wasm_builtin_continuation_frame(
            &mut self,
            bytecode_offset: BytecodeOffset,
            literal_id: i32,
            height: u32,
            return_kind: Option<wasm::ValueKind>,
        ) {
            let opcode = TranslationOpcode::JS_TO_WASM_BUILTIN_CONTINUATION_FRAME;
            self.add(
                opcode,
                vec![
                    SignedOperand::new(bytecode_offset.ToInt()),
                    SignedOperand::new(literal_id),
                    UnsignedOperand::new(height),
                    SignedOperand::new(return_kind.map_or(K_NO_WASM_RETURN_KIND, |kind| kind as i32)),
                ],
            );
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn begin_wasm_inlined_into_js_frame(
            &mut self,
            bailout_id: BytecodeOffset,
            literal_id: i32,
            height: u32,
        ) {
            let opcode = TranslationOpcode::WASM_INLINED_INTO_JS_FRAME;
            self.add(
                opcode,
                vec![
                    SignedOperand::new(bailout_id.ToInt()),
                    SignedOperand::new(literal_id),
                    UnsignedOperand::new(height),
                ],
            );
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn begin_liftoff_frame(
            &mut self,
            bailout_id: BytecodeOffset,
            height: u32,
            wasm_function_index: u32,
        ) {
            let opcode = TranslationOpcode::LIFTOFF_FRAME;
            self.add(
                opcode,
                vec![
                    SignedOperand::new(bailout_id.ToInt()),
                    UnsignedOperand::new(height),
                    UnsignedOperand::new(wasm_function_index),
                ],
            );
        }

        pub fn begin_javascript_builtin_continuation_frame(&mut self, bytecode_offset: BytecodeOffset, literal_id: i32, height: u32) {
            let opcode = TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_FRAME;
            self.add(opcode, vec![SignedOperand::new(bytecode_offset.ToInt()), SignedOperand::new(literal_id), UnsignedOperand::new(height)]);
        }

        pub fn begin_javascript_builtin_continuation_with_catch_frame(&mut self, bytecode_offset: BytecodeOffset, literal_id: i32, height: u32) {
            let opcode = TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME;
            self.add(opcode, vec![SignedOperand::new(bytecode_offset.ToInt()), SignedOperand::new(literal_id), UnsignedOperand::new(height)]);
        }

        pub fn begin_construct_create_stub_frame(&mut self, literal_id: i32, height: u32) {
            let opcode = TranslationOpcode::CONSTRUCT_CREATE_STUB_FRAME;
            self.add(opcode, vec![SignedOperand::new(literal_id), UnsignedOperand::new(height)]);
        }

        pub fn begin_construct_invoke_stub_frame(&mut self, literal_id: i32) {
            let opcode = TranslationOpcode::CONSTRUCT_INVOKE_STUB_FRAME;
            self.add(opcode, vec![SignedOperand::new(literal_id)]);
        }

        pub fn begin_inlined_extra_arguments(&mut self, literal_id: i32, height: u32, parameter_count: u32) {
            let opcode = TranslationOpcode::INLINED_EXTRA_ARGUMENTS;
            self.add(opcode, vec![SignedOperand::new(literal_id), UnsignedOperand::new(height), UnsignedOperand::new(parameter_count)]);
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
                    vec![
                        SignedOperand::new(bytecode_offset.ToInt()),
                        SignedOperand::new(literal_id),
                        SignedOperand::new(bytecode_array_id),
                        UnsignedOperand::new(height),
                    ],
                );
            } else {
                let opcode = TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN;
                self.add(
                    opcode,
                    vec![
                        SignedOperand::new(bytecode_offset.ToInt()),
                        SignedOperand::new(literal_id),
                        SignedOperand::new(bytecode_array_id),
                        UnsignedOperand::new(height),
                        SignedOperand::new(return_value_offset),
                        SignedOperand::new(return_value_count),
                    ],
                );
            }
        }

        pub fn arguments_elements(&mut self, type_: CreateArgumentsType) {
            let opcode = TranslationOpcode::ARGUMENTS_ELEMENTS;
            self.add(opcode, vec![SignedOperand::new(type_ as u8 as i32)]);
        }

        pub fn arguments_length(&mut self) {
            let opcode = TranslationOpcode::ARGUMENTS_LENGTH;
            self.add(opcode, vec![]);
        }

        pub fn rest_length(&mut self) {
            let opcode = TranslationOpcode::REST_LENGTH;
            self.add(opcode, vec![]);
        }

        pub fn begin_captured_object(&mut self, length: i32) {
            let opcode = TranslationOpcode::CAPTURED_OBJECT;
            self.add(opcode, vec![SignedOperand::new(length)]);
        }

        pub fn duplicate_object(&mut self, object_index: i32) {
            let opcode = TranslationOpcode::DUPLICATED_OBJECT;
            self.add(opcode, vec![SignedOperand::new(object_index)]);
        }

        pub fn string_concat(&mut self) {
            let opcode = TranslationOpcode::STRING_CONCAT;
            self.add(opcode, vec![]);
        }

        fn store_register(&mut self, opcode: TranslationOpcode, reg: Register) {
            assert!(Register { code_: Register { code_: 127 }.code_ } .code_ <= K_DATA_MASK as u8); // Assuming Register::kNumRegisters - 1 <= base::kDataMask
            self.add(opcode, vec![SmallUnsignedOperand::new(reg.code() as u32)]);
        }

        pub fn store_register_reg(&mut self, reg: Register) {
            let opcode = TranslationOpcode::REGISTER;
            self.store_register(opcode, reg);
        }

        pub fn store_int32_register(&mut self, reg: Register) {
            let opcode = TranslationOpcode::INT32_REGISTER;
            self.store_register(opcode, reg);
        }

        pub fn store_intptr_register(&mut self, reg: Register) {
            let opcode = if K_SYSTEM_POINTER_SIZE == 4 {
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
            assert!(FloatRegister { code_: FloatRegister { code_: 127 }.code_ } .code_ <= K_DATA_MASK as u8); // Assuming FloatRegister::kNumRegisters - 1 <= base::kDataMask
            let opcode = TranslationOpcode::FLOAT_REGISTER;
            self.add(opcode, vec![SmallUnsignedOperand::new(reg.code() as u32)]);
        }

        pub fn store_double_register(&mut self, reg: DoubleRegister) {
            assert!(DoubleRegister { code_: DoubleRegister { code_: 1