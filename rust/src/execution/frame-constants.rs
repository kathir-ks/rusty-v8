// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides constants related to frame layouts.

#![allow(dead_code)]

mod arch_frame_constants; // Placeholder for architecture-specific includes

// Assuming these are defined elsewhere in the V8 codebase and accessible,
// or suitable replacements exist.
// For now, we define placeholders.
const K_SYSTEM_POINTER_SIZE: i32 = 8; // Example value, should be architecture-dependent
const K_PC_ON_STACK_SIZE: i32 = K_SYSTEM_POINTER_SIZE;
const K_FP_ON_STACK_SIZE: i32 = K_SYSTEM_POINTER_SIZE;
const V8_EMBEDDED_CONSTANT_POOL_BOOL: bool = false;

/// A helper trait to mark a type as being all static, meaning it
/// contains only static constants.  This is a marker trait only.
trait AllStatic {}

/// Constants common to all frame types.
struct CommonFrameConstants {}

impl CommonFrameConstants {
    const K_CALLER_FP_OFFSET: i32 = 0 * K_SYSTEM_POINTER_SIZE;
    const K_CALLER_PC_OFFSET: i32 =
        CommonFrameConstants::K_CALLER_FP_OFFSET + 1 * K_FP_ON_STACK_SIZE;
    const K_CALLER_SP_OFFSET: i32 =
        CommonFrameConstants::K_CALLER_PC_OFFSET + 1 * K_PC_ON_STACK_SIZE;

    const K_FIXED_FRAME_SIZE_ABOVE_FP: i32 = K_PC_ON_STACK_SIZE + K_FP_ON_STACK_SIZE;
    const K_FIXED_SLOT_COUNT_ABOVE_FP: i32 =
        CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP / K_SYSTEM_POINTER_SIZE;
    const K_CP_SLOT_SIZE: i32 = if V8_EMBEDDED_CONSTANT_POOL_BOOL {
        K_SYSTEM_POINTER_SIZE
    } else {
        0
    };
    const K_CP_SLOT_COUNT: i32 = CommonFrameConstants::K_CP_SLOT_SIZE / K_SYSTEM_POINTER_SIZE;
    const K_CONSTANT_POOL_OFFSET: i32 = if CommonFrameConstants::K_CP_SLOT_SIZE != 0 {
        -1 * K_SYSTEM_POINTER_SIZE
    } else {
        0
    };
    const K_CONTEXT_OR_FRAME_TYPE_SIZE: i32 = K_SYSTEM_POINTER_SIZE;
    const K_CONTEXT_OR_FRAME_TYPE_OFFSET: i32 =
        -(CommonFrameConstants::K_CP_SLOT_SIZE + CommonFrameConstants::K_CONTEXT_OR_FRAME_TYPE_SIZE);
}

impl AllStatic for CommonFrameConstants {}

/// Constants specific to standard JavaScript frames.
struct StandardFrameConstants {}

impl StandardFrameConstants {
    const K_FIXED_FRAME_SIZE_FROM_FP: i32 =
        3 * K_SYSTEM_POINTER_SIZE + CommonFrameConstants::K_CP_SLOT_SIZE;
    const K_FIXED_FRAME_SIZE: i32 = CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP
        + StandardFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP;
    const K_FIXED_SLOT_COUNT_FROM_FP: i32 =
        StandardFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP / K_SYSTEM_POINTER_SIZE;
    const K_FIXED_SLOT_COUNT: i32 =
        StandardFrameConstants::K_FIXED_FRAME_SIZE / K_SYSTEM_POINTER_SIZE;
    const K_CONTEXT_OFFSET: i32 = CommonFrameConstants::K_CONTEXT_OR_FRAME_TYPE_OFFSET;
    const K_FUNCTION_OFFSET: i32 = -2 * K_SYSTEM_POINTER_SIZE - CommonFrameConstants::K_CP_SLOT_SIZE;
    const K_ARG_C_OFFSET: i32 = -3 * K_SYSTEM_POINTER_SIZE - CommonFrameConstants::K_CP_SLOT_SIZE;
    const K_EXPRESSIONS_OFFSET: i32 =
        -4 * K_SYSTEM_POINTER_SIZE - CommonFrameConstants::K_CP_SLOT_SIZE;
    const K_FIRST_PUSHED_FRAME_VALUE_OFFSET: i32 = StandardFrameConstants::K_EXPRESSIONS_OFFSET;
    const K_LAST_OBJECT_OFFSET: i32 = StandardFrameConstants::K_CONTEXT_OFFSET;
}

impl AllStatic for StandardFrameConstants {}

/// Constants specific to typed frames.
struct TypedFrameConstants {}

impl TypedFrameConstants {
    const K_FRAME_TYPE_SIZE: i32 = CommonFrameConstants::K_CONTEXT_OR_FRAME_TYPE_SIZE;
    const K_FRAME_TYPE_OFFSET: i32 = CommonFrameConstants::K_CONTEXT_OR_FRAME_TYPE_OFFSET;
    const K_FIXED_FRAME_SIZE_FROM_FP: i32 =
        CommonFrameConstants::K_CP_SLOT_SIZE + TypedFrameConstants::K_FRAME_TYPE_SIZE;
    const K_FIXED_SLOT_COUNT_FROM_FP: i32 =
        TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP / K_SYSTEM_POINTER_SIZE;
    const K_FIXED_FRAME_SIZE: i32 =
        StandardFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP + TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP;
    const K_FIXED_SLOT_COUNT: i32 = TypedFrameConstants::K_FIXED_FRAME_SIZE / K_SYSTEM_POINTER_SIZE;
    const K_FIRST_PUSHED_FRAME_VALUE_OFFSET: i32 =
        -TypedFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP - K_SYSTEM_POINTER_SIZE;
}

impl AllStatic for TypedFrameConstants {}

macro_rules! frame_pushed_value_offset {
    ($parent:ty, $x:expr) => {
        <$parent>::K_FIRST_PUSHED_FRAME_VALUE_OFFSET - ($x) * K_SYSTEM_POINTER_SIZE
    };
}

macro_rules! frame_size {
    ($parent:ty, $count:expr) => {
        <$parent>::K_FIXED_FRAME_SIZE + ($count) * K_SYSTEM_POINTER_SIZE
    };
}

macro_rules! frame_size_from_fp {
    ($parent:ty, $count:expr) => {
        <$parent>::K_FIXED_FRAME_SIZE_FROM_FP + ($count) * K_SYSTEM_POINTER_SIZE
    };
}

macro_rules! define_frame_sizes {
    ($parent:ty, $count:expr) => {
        const K_FIXED_FRAME_SIZE: i32 = frame_size!($parent, $count);
        const K_FIXED_SLOT_COUNT: i32 = Self::K_FIXED_FRAME_SIZE / K_SYSTEM_POINTER_SIZE;
        const K_FIXED_FRAME_SIZE_FROM_FP: i32 = frame_size_from_fp!($parent, $count);
        const K_FIXED_SLOT_COUNT_FROM_FP: i32 =
            Self::K_FIXED_FRAME_SIZE_FROM_FP / K_SYSTEM_POINTER_SIZE;
        const K_FIRST_PUSHED_FRAME_VALUE_OFFSET: i32 =
            <$parent>::K_FIRST_PUSHED_FRAME_VALUE_OFFSET - ($count) * K_SYSTEM_POINTER_SIZE;

        /* The number of slots added on top of given parent frame type. */
        const fn get_extra_slots_count_from<TParentFrameConstants>() -> i32 {
            Self::K_FIXED_SLOT_COUNT - TParentFrameConstants::K_FIXED_SLOT_COUNT
        }

        /* TODO(ishell): remove in favour of getExtraSlotsCountFrom() because */
        /* it's not clear from which base should we count "extra" - from direct */
        /* parent or maybe from parent's parent? */
        const K_EXTRA_SLOT_COUNT: i32 = Self::K_FIXED_SLOT_COUNT - <$parent>::K_FIXED_SLOT_COUNT;
    };
}

macro_rules! standard_frame_extra_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset!(StandardFrameConstants, $x)
    };
}

macro_rules! define_standard_frame_sizes {
    ($count:expr) => {
        define_frame_sizes!(StandardFrameConstants, $count);
    };
}

macro_rules! typed_frame_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset!(TypedFrameConstants, $x)
    };
}

macro_rules! define_typed_frame_sizes {
    ($count:expr) => {
        define_frame_sizes!(TypedFrameConstants, $count);
    };
}

/// Constants for builtin frames.
struct BuiltinFrameConstants {}

impl BuiltinFrameConstants {
    const K_FUNCTION_OFFSET: i32 = typed_frame_pushed_value_offset!(0);
    const K_LENGTH_OFFSET: i32 = typed_frame_pushed_value_offset!(1);
    define_typed_frame_sizes!(2);
}

impl AllStatic for BuiltinFrameConstants {}

/// Constants for construct frames.
struct ConstructFrameConstants {}

impl ConstructFrameConstants {
    const K_CONTEXT_OFFSET: i32 = typed_frame_pushed_value_offset!(0);
    const K_LENGTH_OFFSET: i32 = typed_frame_pushed_value_offset!(1);
    const K_CONSTRUCTOR_OFFSET: i32 = typed_frame_pushed_value_offset!(2);
    const K_PADDING_OFFSET: i32 = typed_frame_pushed_value_offset!(3);
    const K_NEW_TARGET_OR_IMPLICIT_RECEIVER_OFFSET: i32 = typed_frame_pushed_value_offset!(4);
    define_typed_frame_sizes!(5);
    const K_LAST_OBJECT_OFFSET: i32 = ConstructFrameConstants::K_CONTEXT_OFFSET;
}

impl AllStatic for ConstructFrameConstants {}

/// Constants for fast construct frames.
struct FastConstructFrameConstants {}

impl FastConstructFrameConstants {
    const K_CONTEXT_OFFSET: i32 = typed_frame_pushed_value_offset!(0);
    const K_IMPLICIT_RECEIVER_OFFSET: i32 = typed_frame_pushed_value_offset!(1);
    define_typed_frame_sizes!(2);
}

impl AllStatic for FastConstructFrameConstants {}

#[cfg(feature = "v8_enable_webassembly")]
mod webassembly_frame_constants {
    use super::*;

    /// Constants for C-WASM entry frames.
    struct CWasmEntryFrameConstants {}

    impl CWasmEntryFrameConstants {
        const K_C_ENTRY_FP_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
        define_typed_frame_sizes!(CWasmEntryFrameConstants, 1);
    }

    impl AllStatic for CWasmEntryFrameConstants {}

    /// Constants for WebAssembly frames.
    struct WasmFrameConstants {}

    impl WasmFrameConstants {
        const K_WASM_INSTANCE_DATA_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
        define_typed_frame_sizes!(WasmFrameConstants, 1);
        const K_PROTECTED_INSTRUCTION_RETURN_ADDRESS_OFFSET: i32 = 1;
    }

    impl AllStatic for WasmFrameConstants {}

    #[cfg(feature = "v8_enable_drumbrake")]
    mod drumbrake_webassembly_frame_constants {
        use super::*;

        /// Constants for WebAssembly interpreter frames.
        struct WasmInterpreterFrameConstants {}

        impl WasmInterpreterFrameConstants {
            const K_WASM_INSTANCE_OBJECT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
            define_typed_frame_sizes!(WasmInterpreterFrameConstants, 1);
        }

        impl AllStatic for WasmInterpreterFrameConstants {}

        /// Constants for WebAssembly-to-JS interpreter frames.
        struct WasmToJSInterpreterFrameConstants {}

        impl WasmToJSInterpreterFrameConstants {
            const K_GC_SCAN_SLOT_LIMIT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
            const K_GC_SP_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 1);
        }

        impl AllStatic for WasmToJSInterpreterFrameConstants {}

        struct WasmInterpreterCWasmEntryConstants {}

        impl WasmInterpreterCWasmEntryConstants {
            const K_C_ENTRY_FP_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
            const K_SP_FP_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 1);
            define_typed_frame_sizes!(WasmInterpreterCWasmEntryConstants, 2);
        }
        impl AllStatic for WasmInterpreterCWasmEntryConstants {}
    }

    /// Constants for WebAssembly exit frames.
    struct WasmExitFrameConstants {}

    impl WasmExitFrameConstants {
        const K_CALLING_PC_OFFSET: i32 = typed_frame_pushed_value_offset!(WasmFrameConstants, 1);
        define_typed_frame_sizes!(WasmExitFrameConstants, 2);
    }

    impl AllStatic for WasmExitFrameConstants {}

    /// Constants for JS-to-WebAssembly wrapper frames.
    struct JSToWasmWrapperFrameConstants {}

    impl JSToWasmWrapperFrameConstants {
        const K_RESULT_ARRAY_PARAM_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
        const K_IMPLICIT_ARG_OFFSET: i32 = 3 * K_SYSTEM_POINTER_SIZE;

        const K_WRAPPER_BUFFER_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);

        const K_WRAPPER_BUFFER_RETURN_COUNT: usize = 0;
        const K_WRAPPER_BUFFER_REF_RETURN_COUNT: usize = 4;
        const K_WRAPPER_BUFFER_SIG_REPRESENTATION_ARRAY: usize = 8;
        const K_WRAPPER_BUFFER_STACK_RETURN_BUFFER_SIZE: usize = 16;
        const K_WRAPPER_BUFFER_CALL_TARGET: usize = 24;
        const K_WRAPPER_BUFFER_PARAM_START: usize = 32;
        const K_WRAPPER_BUFFER_PARAM_END: usize = 40;

        const K_WRAPPER_BUFFER_STACK_RETURN_BUFFER_START: usize = 16;
        const K_WRAPPER_BUFFER_FP_RETURN_REGISTER_1: usize = 24;
        const K_WRAPPER_BUFFER_FP_RETURN_REGISTER_2: usize = 32;
        const K_WRAPPER_BUFFER_GP_RETURN_REGISTER_1: usize = 40;
        const K_WRAPPER_BUFFER_GP_RETURN_REGISTER_2: usize =
            JSToWasmWrapperFrameConstants::K_WRAPPER_BUFFER_GP_RETURN_REGISTER_1 + K_SYSTEM_POINTER_SIZE as usize;

        const K_WRAPPER_BUFFER_SIZE: usize =
            JSToWasmWrapperFrameConstants::K_WRAPPER_BUFFER_GP_RETURN_REGISTER_2 + K_SYSTEM_POINTER_SIZE as usize;
    }

    impl AllStatic for JSToWasmWrapperFrameConstants {}

    /// Constants for stack switch frames
    struct StackSwitchFrameConstants {}

    impl StackSwitchFrameConstants {
        const K_GC_SCAN_SLOT_COUNT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 1);
        const K_IMPLICIT_ARG_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 2);
        const K_RESULT_ARRAY_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 3);

        const K_LAST_SPILL_OFFSET: i32 = StackSwitchFrameConstants::K_RESULT_ARRAY_OFFSET;
        const K_NUM_SPILL_SLOTS: i32 = 4;
    }

    impl AllStatic for StackSwitchFrameConstants {}

    /// Constants for WebAssembly-to-JS wrapper constants.
    struct WasmToJSWrapperConstants {}

    impl WasmToJSWrapperConstants {
        const K_SIGNATURE_OFFSET: i32 = 2 * K_SYSTEM_POINTER_SIZE;
    }

    impl AllStatic for WasmToJSWrapperConstants {}

    #[cfg(feature = "v8_enable_drumbrake")]
    mod drumbrake_webassembly_frame_constants_builtin {
        use super::*;
        struct BuiltinWasmInterpreterWrapperConstants {}

        impl BuiltinWasmInterpreterWrapperConstants {
            const K_GC_SCAN_SLOT_COUNT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
            const K_IN_PARAM_COUNT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 1);
            const K_PARAM_COUNT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 2);
            const K_RETURN_COUNT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 3);
            const K_SIG_REPS_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 4);
            const K_VALUE_TYPES_ARRAY_START_OFFSET: i32 =
                typed_frame_pushed_value_offset!(TypedFrameConstants, 5);
            const K_ARG_RETS_ADDRESS_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 6);
            const K_ARG_RETS_IS_ARGS_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 7);
            const K_CURRENT_INDEX_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 8);
            const K_SIGNATURE_DATA_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 9);
        }
    }
}

/// Constants for builtin continuation frames.
struct BuiltinContinuationFrameConstants {}

impl BuiltinContinuationFrameConstants {
    const K_FUNCTION_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
    const K_FRAME_SP_TO_FP_DELTA_AT_DEOPTIMIZE: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 1);
    const K_BUILTIN_CONTEXT_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 2);
    const K_BUILTIN_INDEX_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 3);

    const K_ARG_C_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 4);
    define_typed_frame_sizes!(4);

    fn padding_slot_count(register_count: i32) -> i32 {
        // Placeholder - needs architecture-specific implementation
        0
    }
}

impl AllStatic for BuiltinContinuationFrameConstants {}

/// Constants for exit frames.
struct ExitFrameConstants {}

impl ExitFrameConstants {
    const K_SP_OFFSET: i32 = typed_frame_pushed_value_offset!(TypedFrameConstants, 0);
    const K_LAST_EXIT_FRAME_FIELD: i32 = ExitFrameConstants::K_SP_OFFSET;
    define_typed_frame_sizes!(1);

    const K_CALLER_SP_DISPLACEMENT: i32 = CommonFrameConstants::K_CALLER_SP_OFFSET;
}

impl AllStatic for ExitFrameConstants {}

macro_rules! exit_frame_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset!(ExitFrameConstants, $x)
    };
}

macro_rules! define_exit_frame_sizes {
    ($x:expr) => {
        define_frame_sizes!(ExitFrameConstants, $x);
    };
}

/// Constants for builtin exit frames.
struct BuiltinExitFrameConstants {}

impl BuiltinExitFrameConstants {
    const K_NEW_TARGET_INDEX: i32 = 0;
    const K_TARGET_INDEX: i32 = 1;
    const K_ARG_C_INDEX: i32 = 2;
    const K_PADDING_INDEX: i32 = 3;
    const K_NUM_EXTRA_ARGS: i32 = 4;
    const K_NUM_EXTRA_ARGS_WITH_RECEIVER: i32 = BuiltinExitFrameConstants::K_NUM_EXTRA_ARGS + 1;

    const K_ARGUMENTS_ARRAY_OFFSET: i32 = CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP;
    const K_TARGET_OFFSET: i32 = BuiltinExitFrameConstants::K_ARGUMENTS_ARRAY_OFFSET
        + BuiltinExitFrameConstants::K_TARGET_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_NEW_TARGET_OFFSET: i32 = BuiltinExitFrameConstants::K_ARGUMENTS_ARRAY_OFFSET
        + BuiltinExitFrameConstants::K_NEW_TARGET_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_ARG_C_OFFSET: i32 = BuiltinExitFrameConstants::K_ARGUMENTS_ARRAY_OFFSET
        + BuiltinExitFrameConstants::K_ARG_C_INDEX * K_SYSTEM_POINTER_SIZE;

    const K_RECEIVER_OFFSET: i32 = BuiltinExitFrameConstants::K_ARGUMENTS_ARRAY_OFFSET
        + BuiltinExitFrameConstants::K_NUM_EXTRA_ARGS * K_SYSTEM_POINTER_SIZE;

    const K_FIRST_ARGUMENT_OFFSET: i32 =
        BuiltinExitFrameConstants::K_RECEIVER_OFFSET + K_SYSTEM_POINTER_SIZE;
}

impl AllStatic for BuiltinExitFrameConstants {}

/// Constants for API callback exit frames.
struct ApiCallbackExitFrameConstants {}

impl ApiCallbackExitFrameConstants {
    const K_FUNCTION_CALLBACK_INFO_CONTEXT_INDEX: i32 = 2;
    const K_FUNCTION_CALLBACK_INFO_RETURN_VALUE_INDEX: i32 = 3;
    const K_FUNCTION_CALLBACK_INFO_TARGET_INDEX: i32 = 4;
    const K_FUNCTION_CALLBACK_INFO_NEW_TARGET_INDEX: i32 = 5;
    const K_FUNCTION_CALLBACK_INFO_ARGS_LENGTH: i32 = 6;

    const K_FCI_ARG_C_OFFSET: i32 = exit_frame_pushed_value_offset!(0);
    const K_FCI_VALUES_OFFSET: i32 = exit_frame_pushed_value_offset!(1);
    const K_FCI_IMPLICIT_ARGS_OFFSET: i32 = exit_frame_pushed_value_offset!(2);

    define_exit_frame_sizes!(3);
    //static_assert(kSPOffset - kSystemPointerSize == kFCIArgcOffset);

    const K_FUNCTION_CALLBACK_INFO_OFFSET: i32 = ApiCallbackExitFrameConstants::K_FCIImplicitArgsOffset;

    const K_IMPLICIT_ARGS_ARRAY_OFFSET: i32 = CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP;
    const K_TARGET_OFFSET: i32 = ApiCallbackExitFrameConstants::K_IMPLICIT_ARGS_ARRAY_OFFSET
        + ApiCallbackExitFrameConstants::K_FUNCTION_CALLBACK_INFO_TARGET_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_NEW_TARGET_OFFSET: i32 = ApiCallbackExitFrameConstants::K_IMPLICIT_ARGS_ARRAY_OFFSET
        + ApiCallbackExitFrameConstants::K_FUNCTION_CALLBACK_INFO_NEW_TARGET_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_CONTEXT_OFFSET: i32 = ApiCallbackExitFrameConstants::K_IMPLICIT_ARGS_ARRAY_OFFSET
        + ApiCallbackExitFrameConstants::K_FUNCTION_CALLBACK_INFO_CONTEXT_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_RETURN_VALUE_OFFSET: i32 = ApiCallbackExitFrameConstants::K_IMPLICIT_ARGS_ARRAY_OFFSET
        + ApiCallbackExitFrameConstants::K_FUNCTION_CALLBACK_INFO_RETURN_VALUE_INDEX * K_SYSTEM_POINTER_SIZE;

    const K_RECEIVER_OFFSET: i32 = ApiCallbackExitFrameConstants::K_IMPLICIT_ARGS_ARRAY_OFFSET
        + ApiCallbackExitFrameConstants::K_FUNCTION_CALLBACK_INFO_ARGS_LENGTH * K_SYSTEM_POINTER_SIZE;

    const K_FIRST_ARGUMENT_OFFSET: i32 =
        ApiCallbackExitFrameConstants::K_RECEIVER_OFFSET + K_SYSTEM_POINTER_SIZE;
}

impl AllStatic for ApiCallbackExitFrameConstants {}

/// Constants for API accessor exit frames.
struct ApiAccessorExitFrameConstants {}

impl ApiAccessorExitFrameConstants {
    const K_PROPERTY_CALLBACK_INFO_PROPERTY_KEY_INDEX: i32 = 0;
    const K_PROPERTY_CALLBACK_INFO_HOLDER_INDEX: i32 = 2;
    const K_PROPERTY_CALLBACK_INFO_RETURN_VALUE_INDEX: i32 = 5;
    const K_PROPERTY_CALLBACK_INFO_RECEIVER_INDEX: i32 = 7;
    const K_PROPERTY_CALLBACK_INFO_ARGS_LENGTH: i32 = 8;

    const K_ARGS_ARRAY_OFFSET: i32 = CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP;
    const K_PROPERTY_NAME_OFFSET: i32 = ApiAccessorExitFrameConstants::K_ARGS_ARRAY_OFFSET
        + ApiAccessorExitFrameConstants::K_PROPERTY_CALLBACK_INFO_PROPERTY_KEY_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_RETURN_VALUE_OFFSET: i32 = ApiAccessorExitFrameConstants::K_ARGS_ARRAY_OFFSET
        + ApiAccessorExitFrameConstants::K_PROPERTY_CALLBACK_INFO_RETURN_VALUE_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_RECEIVER_OFFSET: i32 = ApiAccessorExitFrameConstants::K_ARGS_ARRAY_OFFSET
        + ApiAccessorExitFrameConstants::K_PROPERTY_CALLBACK_INFO_RECEIVER_INDEX * K_SYSTEM_POINTER_SIZE;
    const K_HOLDER_OFFSET: i32 = ApiAccessorExitFrameConstants::K_ARGS_ARRAY_OFFSET
        + ApiAccessorExitFrameConstants::K_PROPERTY_CALLBACK_INFO_HOLDER_INDEX * K_SYSTEM_POINTER_SIZE;

    const K_PROPERTY_CALLBACK_INFO_OFFSET: i32 = ApiAccessorExitFrameConstants::K_ARGS_ARRAY_OFFSET;
}

impl AllStatic for ApiAccessorExitFrameConstants {}

/// Constants for unoptimized frames.
struct UnoptimizedFrameConstants {}

impl UnoptimizedFrameConstants {
    const K_BYTECODE_ARRAY_FROM_FP: i32 = standard_frame_extra_pushed_value_offset!(0);
    const K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_FROM_FP: i32 = standard_frame_extra_pushed_value_offset!(1);
    const K_FEEDBACK_VECTOR_FROM_FP: i32 = standard_frame_extra_pushed_value_offset!(2);
    define_standard_frame_sizes!(3);

    const K_FIRST_PARAM_FROM_FP: i32 = StandardFrameConstants::K_CALLER_SP_OFFSET;
    const K_REGISTER_FILE_FROM_FP: i32 =
        -StandardFrameConstants::K_FIXED_FRAME_SIZE_FROM_FP - K_SYSTEM_POINTER_SIZE;
    const K_EXPRESSIONS_OFFSET: i32 = UnoptimizedFrameConstants::K_REGISTER_FILE_FROM_FP;

    const K_BYTECODE_ARRAY_EXPRESSION_INDEX: i32 = -3;
    const K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_EXPRESSION_INDEX: i32 = -2;
    const K_FEEDBACK_VECTOR_EXPRESSION_INDEX: i32 = -1;
    const K_REGISTER_FILE_EXPRESSION_INDEX: i32 = 0;

    fn register_stack_slot_count(register_count: i32) -> i32 {
        // Placeholder - needs architecture-specific implementation
        register_count
    }
}

impl AllStatic for UnoptimizedFrameConstants {}

/// Constants for interpreter frames.
struct InterpreterFrameConstants {}

impl InterpreterFrameConstants {
    const K_BYTECODE_OFFSET_EXPRESSION_INDEX: i32 =
        UnoptimizedFrameConstants::K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_EXPRESSION_INDEX;

    const K_BYTECODE_OFFSET_FROM_FP: i32 =
        UnoptimizedFrameConstants::K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_FROM_FP;
}

impl AllStatic for InterpreterFrameConstants {}

/// Constants for baseline frames.
struct BaselineFrameConstants {}

impl BaselineFrameConstants {
    const K_FEEDBACK_CELL_EXPRESSION_INDEX: i32 =
        UnoptimizedFrameConstants::K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_EXPRESSION_INDEX;

    const K_FEEDBACK_CELL_FROM_FP: i32 =
        UnoptimizedFrameConstants::K_BYTECODE_OFFSET_OR_FEEDBACK_CELL_FROM_FP;
}

impl AllStatic for BaselineFrameConstants {}

fn fp_offset_to_frame_slot(frame_offset: i32) -> i32 {
    StandardFrameConstants::K_FIXED_SLOT_COUNT_ABOVE_FP - 1 - frame_offset / K_SYSTEM_POINTER_SIZE
}

fn frame_slot_to_fp_offset(slot: i32) -> i32 {
    (StandardFrameConstants::K_FIXED_SLOT_COUNT_ABOVE_FP - 1 - slot) * K_SYSTEM_POINTER_SIZE
}