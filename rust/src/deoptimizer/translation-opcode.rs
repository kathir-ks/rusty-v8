// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NOTE: IF_WASM macro is not directly translatable without WASM feature flags.

/// Translation opcodes used in the deoptimization process.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TranslationOpcode {
    INTERPRETED_FRAME_WITH_RETURN,
    INTERPRETED_FRAME_WITHOUT_RETURN,
    JAVASCRIPT_BUILTIN_CONTINUATION_FRAME,
    JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME,
    CONSTRUCT_CREATE_STUB_FRAME,
    CONSTRUCT_INVOKE_STUB_FRAME,
    BUILTIN_CONTINUATION_FRAME,
    // JS_TO_WASM_BUILTIN_CONTINUATION_FRAME, // Conditional WASM
    // WASM_INLINED_INTO_JS_FRAME, // Conditional WASM
    // LIFTOFF_FRAME, // Conditional WASM
    INLINED_EXTRA_ARGUMENTS,
    ARGUMENTS_ELEMENTS,
    ARGUMENTS_LENGTH,
    REST_LENGTH,
    BEGIN_WITHOUT_FEEDBACK,
    BEGIN_WITH_FEEDBACK,
    BOOL_REGISTER,
    BOOL_STACK_SLOT,
    CAPTURED_OBJECT,
    STRING_CONCAT,
    DOUBLE_REGISTER,
    DOUBLE_STACK_SLOT,
    SIMD128_STACK_SLOT,
    HOLEY_DOUBLE_REGISTER,
    HOLEY_DOUBLE_STACK_SLOT,
    SIMD128_REGISTER,
    DUPLICATED_OBJECT,
    FLOAT_REGISTER,
    FLOAT_STACK_SLOT,
    INT32_REGISTER,
    INT32_STACK_SLOT,
    INT64_REGISTER,
    INT64_STACK_SLOT,
    SIGNED_BIGINT64_REGISTER,
    SIGNED_BIGINT64_STACK_SLOT,
    UNSIGNED_BIGINT64_REGISTER,
    UNSIGNED_BIGINT64_STACK_SLOT,
    OPTIMIZED_OUT,
    LITERAL,
    REGISTER,
    TAGGED_STACK_SLOT,
    UINT32_REGISTER,
    UINT32_STACK_SLOT,
    UPDATE_FEEDBACK,
    MATCH_PREVIOUS_TRANSLATION,
}

const K_NUM_TRANSLATION_OPCODES: usize = 41; // Adjusted count
const K_NUM_TRANSLATION_JS_FRAME_OPCODES: usize = 4;
const K_NUM_TRANSLATION_FRAME_OPCODES: usize = 7; // Adjusted count

impl TranslationOpcode {
    /// Returns the number of operands for a given TranslationOpcode.
    pub fn operand_count(&self) -> usize {
        match self {
            TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN => 6,
            TranslationOpcode::INTERPRETED_FRAME_WITHOUT_RETURN => 4,
            TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_FRAME => 3,
            TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME => 3,
            TranslationOpcode::CONSTRUCT_CREATE_STUB_FRAME => 2,
            TranslationOpcode::CONSTRUCT_INVOKE_STUB_FRAME => 1,
            TranslationOpcode::BUILTIN_CONTINUATION_FRAME => 3,
            // TranslationOpcode::JS_TO_WASM_BUILTIN_CONTINUATION_FRAME => 4, // Conditional WASM
            // TranslationOpcode::WASM_INLINED_INTO_JS_FRAME => 3, // Conditional WASM
            // TranslationOpcode::LIFTOFF_FRAME => 3, // Conditional WASM
            TranslationOpcode::INLINED_EXTRA_ARGUMENTS => 3,
            TranslationOpcode::ARGUMENTS_ELEMENTS => 1,
            TranslationOpcode::ARGUMENTS_LENGTH => 0,
            TranslationOpcode::REST_LENGTH => 0,
            TranslationOpcode::BEGIN_WITHOUT_FEEDBACK => 3,
            TranslationOpcode::BEGIN_WITH_FEEDBACK => 3,
            TranslationOpcode::BOOL_REGISTER => 1,
            TranslationOpcode::BOOL_STACK_SLOT => 1,
            TranslationOpcode::CAPTURED_OBJECT => 1,
            TranslationOpcode::STRING_CONCAT => 0,
            TranslationOpcode::DOUBLE_REGISTER => 1,
            TranslationOpcode::DOUBLE_STACK_SLOT => 1,
            TranslationOpcode::SIMD128_STACK_SLOT => 1,
            TranslationOpcode::HOLEY_DOUBLE_REGISTER => 1,
            TranslationOpcode::HOLEY_DOUBLE_STACK_SLOT => 1,
            TranslationOpcode::SIMD128_REGISTER => 1,
            TranslationOpcode::DUPLICATED_OBJECT => 1,
            TranslationOpcode::FLOAT_REGISTER => 1,
            TranslationOpcode::FLOAT_STACK_SLOT => 1,
            TranslationOpcode::INT32_REGISTER => 1,
            TranslationOpcode::INT32_STACK_SLOT => 1,
            TranslationOpcode::INT64_REGISTER => 1,
            TranslationOpcode::INT64_STACK_SLOT => 1,
            TranslationOpcode::SIGNED_BIGINT64_REGISTER => 1,
            TranslationOpcode::SIGNED_BIGINT64_STACK_SLOT => 1,
            TranslationOpcode::UNSIGNED_BIGINT64_REGISTER => 1,
            TranslationOpcode::UNSIGNED_BIGINT64_STACK_SLOT => 1,
            TranslationOpcode::OPTIMIZED_OUT => 0,
            TranslationOpcode::LITERAL => 1,
            TranslationOpcode::REGISTER => 1,
            TranslationOpcode::TAGGED_STACK_SLOT => 1,
            TranslationOpcode::UINT32_REGISTER => 1,
            TranslationOpcode::UINT32_STACK_SLOT => 1,
            TranslationOpcode::UPDATE_FEEDBACK => 2,
            TranslationOpcode::MATCH_PREVIOUS_TRANSLATION => 1,
        }
    }

    /// Checks if the opcode is a begin opcode.
    pub fn is_begin(&self) -> bool {
        *self == TranslationOpcode::BEGIN_WITH_FEEDBACK || *self == TranslationOpcode::BEGIN_WITHOUT_FEEDBACK
    }

    /// Checks if the opcode is a frame opcode.
    pub fn is_translation_frame_opcode(&self) -> bool {
        (*self as usize) < K_NUM_TRANSLATION_FRAME_OPCODES
    }

    /// Checks if the opcode is a JS frame opcode.
    pub fn is_translation_js_frame_opcode(&self) -> bool {
        (*self as usize) < K_NUM_TRANSLATION_JS_FRAME_OPCODES
    }

    pub fn is_translation_interpreter_frame_opcode(&self) -> bool {
        *self == TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN || *self == TranslationOpcode::INTERPRETED_FRAME_WITHOUT_RETURN
    }
}

use std::fmt;

impl fmt::Display for TranslationOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationOpcode::INTERPRETED_FRAME_WITH_RETURN => write!(f, "INTERPRETED_FRAME_WITH_RETURN"),
            TranslationOpcode::INTERPRETED_FRAME_WITHOUT_RETURN => write!(f, "INTERPRETED_FRAME_WITHOUT_RETURN"),
            TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_FRAME => write!(f, "JAVASCRIPT_BUILTIN_CONTINUATION_FRAME"),
            TranslationOpcode::JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME => write!(f, "JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME"),
            TranslationOpcode::CONSTRUCT_CREATE_STUB_FRAME => write!(f, "CONSTRUCT_CREATE_STUB_FRAME"),
            TranslationOpcode::CONSTRUCT_INVOKE_STUB_FRAME => write!(f, "CONSTRUCT_INVOKE_STUB_FRAME"),
            TranslationOpcode::BUILTIN_CONTINUATION_FRAME => write!(f, "BUILTIN_CONTINUATION_FRAME"),
            //TranslationOpcode::JS_TO_WASM_BUILTIN_CONTINUATION_FRAME => write!(f, "JS_TO_WASM_BUILTIN_CONTINUATION_FRAME"), // Conditional WASM
            //TranslationOpcode::WASM_INLINED_INTO_JS_FRAME => write!(f, "WASM_INLINED_INTO_JS_FRAME"), // Conditional WASM
            //TranslationOpcode::LIFTOFF_FRAME => write!(f, "LIFTOFF_FRAME"), // Conditional WASM
            TranslationOpcode::INLINED_EXTRA_ARGUMENTS => write!(f, "INLINED_EXTRA_ARGUMENTS"),
            TranslationOpcode::ARGUMENTS_ELEMENTS => write!(f, "ARGUMENTS_ELEMENTS"),
            TranslationOpcode::ARGUMENTS_LENGTH => write!(f, "ARGUMENTS_LENGTH"),
            TranslationOpcode::REST_LENGTH => write!(f, "REST_LENGTH"),
            TranslationOpcode::BEGIN_WITHOUT_FEEDBACK => write!(f, "BEGIN_WITHOUT_FEEDBACK"),
            TranslationOpcode::BEGIN_WITH_FEEDBACK => write!(f, "BEGIN_WITH_FEEDBACK"),
            TranslationOpcode::BOOL_REGISTER => write!(f, "BOOL_REGISTER"),
            TranslationOpcode::BOOL_STACK_SLOT => write!(f, "BOOL_STACK_SLOT"),
            TranslationOpcode::CAPTURED_OBJECT => write!(f, "CAPTURED_OBJECT"),
            TranslationOpcode::STRING_CONCAT => write!(f, "STRING_CONCAT"),
            TranslationOpcode::DOUBLE_REGISTER => write!(f, "DOUBLE_REGISTER"),
            TranslationOpcode::DOUBLE_STACK_SLOT => write!(f, "DOUBLE_STACK_SLOT"),
            TranslationOpcode::SIMD128_STACK_SLOT => write!(f, "SIMD128_STACK_SLOT"),
            TranslationOpcode::HOLEY_DOUBLE_REGISTER => write!(f, "HOLEY_DOUBLE_REGISTER"),
            TranslationOpcode::HOLEY_DOUBLE_STACK_SLOT => write!(f, "HOLEY_DOUBLE_STACK_SLOT"),
            TranslationOpcode::SIMD128_REGISTER => write!(f, "SIMD128_REGISTER"),
            TranslationOpcode::DUPLICATED_OBJECT => write!(f, "DUPLICATED_OBJECT"),
            TranslationOpcode::FLOAT_REGISTER => write!(f, "FLOAT_REGISTER"),
            TranslationOpcode::FLOAT_STACK_SLOT => write!(f, "FLOAT_STACK_SLOT"),
            TranslationOpcode::INT32_REGISTER => write!(f, "INT32_REGISTER"),
            TranslationOpcode::INT32_STACK_SLOT => write!(f, "INT32_STACK_SLOT"),
            TranslationOpcode::INT64_REGISTER => write!(f, "INT64_REGISTER"),
            TranslationOpcode::INT64_STACK_SLOT => write!(f, "INT64_STACK_SLOT"),
            TranslationOpcode::SIGNED_BIGINT64_REGISTER => write!(f, "SIGNED_BIGINT64_REGISTER"),
            TranslationOpcode::SIGNED_BIGINT64_STACK_SLOT => write!(f, "SIGNED_BIGINT64_STACK_SLOT"),
            TranslationOpcode::UNSIGNED_BIGINT64_REGISTER => write!(f, "UNSIGNED_BIGINT64_REGISTER"),
            TranslationOpcode::UNSIGNED_BIGINT64_STACK_SLOT => write!(f, "UNSIGNED_BIGINT64_STACK_SLOT"),
            TranslationOpcode::OPTIMIZED_OUT => write!(f, "OPTIMIZED_OUT"),
            TranslationOpcode::LITERAL => write!(f, "LITERAL"),
            TranslationOpcode::REGISTER => write!(f, "REGISTER"),
            TranslationOpcode::TAGGED_STACK_SLOT => write!(f, "TAGGED_STACK_SLOT"),
            TranslationOpcode::UINT32_REGISTER => write!(f, "UINT32_REGISTER"),
            TranslationOpcode::UINT32_STACK_SLOT => write!(f, "UINT32_STACK_SLOT"),
            TranslationOpcode::UPDATE_FEEDBACK => write!(f, "UPDATE_FEEDBACK"),
            TranslationOpcode::MATCH_PREVIOUS_TRANSLATION => write!(f, "MATCH_PREVIOUS_TRANSLATION"),
        }
    }
}