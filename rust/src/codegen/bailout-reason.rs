// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod codegen {
    pub mod bailout_reason {
        /// Represents the reason for a bailout from optimized code.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u8)]
        pub enum BailoutReason {
            kNoReason = 0,
            kBailedOutDueToDependencyChange,
            kConcurrentMapDeprecation,
            kCodeGenerationFailed,
            kFunctionBeingDebugged,
            kGraphBuildingFailed,
            kFunctionTooBig,
            kTooManyArguments,
            kLiveEdit,
            kNativeFunctionLiteral,
            kOptimizationDisabled,
            kHigherTierAvailable,
            kDetachedNativeContext,
            kNeverOptimize,
            kLastErrorMessage,
        }

        impl BailoutReason {
            pub fn as_str(&self) -> &'static str {
                match self {
                    BailoutReason::kNoReason => "no reason",
                    BailoutReason::kBailedOutDueToDependencyChange => "Bailed out due to dependency change",
                    BailoutReason::kConcurrentMapDeprecation => "Maps became deprecated during optimization",
                    BailoutReason::kCodeGenerationFailed => "Code generation failed",
                    BailoutReason::kFunctionBeingDebugged => "Function is being debugged",
                    BailoutReason::kGraphBuildingFailed => "Optimized graph construction failed",
                    BailoutReason::kFunctionTooBig => "Function is too big to be optimized",
                    BailoutReason::kTooManyArguments => "Function contains a call with too many arguments",
                    BailoutReason::kLiveEdit => "LiveEdit",
                    BailoutReason::kNativeFunctionLiteral => "Native function literal",
                    BailoutReason::kOptimizationDisabled => "Optimization disabled",
                    BailoutReason::kHigherTierAvailable => "A higher tier is already available",
                    BailoutReason::kDetachedNativeContext => "The native context is detached",
                    BailoutReason::kNeverOptimize => "Optimization is always disabled",
                    BailoutReason::kLastErrorMessage => "kLastErrorMessage",
                }
            }
        }

        /// Represents the reason for an abort during code execution.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u8)]
        pub enum AbortReason {
            kNoReason = 0,
            k32BitValueInRegisterIsNotZeroExtended,
            kSignedBitOfSmiIsNotZero,
            kAPICallReturnedInvalidObject,
            kAccumulatorClobbered,
            kAllocatingNonEmptyPackedArray,
            kAllocationIsNotDoubleAligned,
            kExpectedOptimizationSentinel,
            kExpectedOsrCode,
            kExpectedUndefinedOrCell,
            kExpectedFeedbackCell,
            kExpectedFeedbackVector,
            kExpectedBaselineData,
            kFloat64IsNotAInt32,
            kFunctionDataShouldBeBytecodeArrayOnInterpreterEntry,
            kInputStringTooLong,
            kInputDoesNotFitSmi,
            kInvalidBytecode,
            kInvalidBytecodeAdvance,
            kInvalidDeoptimizedCode,
            kInvalidHandleScopeLevel,
            kInvalidJumpTableIndex,
            kInvalidParametersAndRegistersInGenerator,
            kMissingBytecodeArray,
            kObjectNotTagged,
            kObjectTagged,
            kOffsetOutOfRange,
            kOperandIsASmi,
            kOperandIsASmiAndNotABoundFunction,
            kOperandIsASmiAndNotAConstructor,
            kOperandIsASmiAndNotAFunction,
            kOperandIsASmiAndNotAGeneratorObject,
            kOperandIsCleared,
            kOperandIsNotABoundFunction,
            kOperandIsNotAConstructor,
            kOperandIsNotAFixedArray,
            kOperandIsNotAFunction,
            kOperandIsNotACallableFunction,
            kOperandIsNotAGeneratorObject,
            kOperandIsNotACode,
            kOperandIsNotAMap,
            kOperandIsNotASmi,
            kPromiseAlreadySettled,
            kReceivedInvalidReturnAddress,
            kRegisterDidNotMatchExpectedRoot,
            kReturnAddressNotFoundInFrame,
            kShouldNotDirectlyEnterOsrFunction,
            kStackAccessBelowStackPointer,
            kOsrUnexpectedStackSize,
            kStackFrameTypesMustMatch,
            kUint32IsNotAInt32,
            kUnalignedCellInWriteBarrier,
            kUnexpectedAdditionalPopValue,
            kUnexpectedElementsKindInArrayConstructor,
            kUnexpectedFPCRMode,
            kUnexpectedFunctionIDForInvokeIntrinsic,
            kUnexpectedInitialMapForArrayFunction,
            kUnexpectedLevelAfterReturnFromApiCall,
            kUnexpectedNegativeValue,
            kUnexpectedReturnFromFrameDropper,
            kUnexpectedReturnFromThrow,
            kUnexpectedReturnFromWasmTrap,
            kUnexpectedStackPointer,
            kUnexpectedValue,
            kUninhabitableType,
            kUnsupportedModuleOperation,
            kUnsupportedNonPrimitiveCompare,
            kWrongAddressOrValuePassedToRecordWrite,
            kWrongArgumentCountForInvokeIntrinsic,
            kWrongFunctionCodeStart,
            kWrongFunctionContext,
            kWrongFunctionDispatchHandle,
            kUnexpectedThreadInWasmSet,
            kUnexpectedThreadInWasmUnset,
            kInvalidReceiver,
            kUnexpectedInstanceType,
            kTurboshaftTypeAssertionFailed,
            kMetadataAreaStartDoesNotMatch,
            kExternalPointerTagMismatch,
            kJSSignatureMismatch,
            kWasmSignatureMismatch,
            kFastCallFallbackInvalid,
            k32BitValueInRegisterIsNotSignExtended,
            kLastErrorMessage,
        }

        impl AbortReason {
            pub fn as_str(&self) -> &'static str {
                match self {
                    AbortReason::kNoReason => "no reason",
                    AbortReason::k32BitValueInRegisterIsNotZeroExtended => {
                        "32 bit value in register is not zero-extended"
                    }
                    AbortReason::kSignedBitOfSmiIsNotZero => "Signed bit of 31 bit smi register is not zero",
                    AbortReason::kAPICallReturnedInvalidObject => "API call returned invalid object",
                    AbortReason::kAccumulatorClobbered => "Accumulator clobbered",
                    AbortReason::kAllocatingNonEmptyPackedArray => "Allocating non-empty packed array",
                    AbortReason::kAllocationIsNotDoubleAligned => "Allocation is not double aligned",
                    AbortReason::kExpectedOptimizationSentinel => {
                        "Expected optimized code cell or optimization sentinel"
                    }
                    AbortReason::kExpectedOsrCode => "Expected OSR code",
                    AbortReason::kExpectedUndefinedOrCell => "Expected undefined or cell in register",
                    AbortReason::kExpectedFeedbackCell => "Expected feedback cell",
                    AbortReason::kExpectedFeedbackVector => "Expected feedback vector",
                    AbortReason::kExpectedBaselineData => "Expected baseline data",
                    AbortReason::kFloat64IsNotAInt32 => {
                        "Float64 cannot be converted to Int32 without loss of precision"
                    }
                    AbortReason::kFunctionDataShouldBeBytecodeArrayOnInterpreterEntry => {
                        "The function_data field should be a BytecodeArray on interpreter entry"
                    }
                    AbortReason::kInputStringTooLong => "Input string too long",
                    AbortReason::kInputDoesNotFitSmi => "Input number is too large to fit in a Smi",
                    AbortReason::kInvalidBytecode => "Invalid bytecode",
                    AbortReason::kInvalidBytecodeAdvance => "Cannot advance current bytecode, ",
                    AbortReason::kInvalidDeoptimizedCode => "Invoked code which is deoptimized",
                    AbortReason::kInvalidHandleScopeLevel => "Invalid HandleScope level",
                    AbortReason::kInvalidJumpTableIndex => "Invalid jump table index",
                    AbortReason::kInvalidParametersAndRegistersInGenerator => {
                        "invalid parameters and registers in generator"
                    }
                    AbortReason::kMissingBytecodeArray => "Missing bytecode array from function",
                    AbortReason::kObjectNotTagged => "The object is not tagged",
                    AbortReason::kObjectTagged => "The object is tagged",
                    AbortReason::kOffsetOutOfRange => "Offset out of range",
                    AbortReason::kOperandIsASmi => "Operand is a smi",
                    AbortReason::kOperandIsASmiAndNotABoundFunction => "Operand is a smi and not a bound function",
                    AbortReason::kOperandIsASmiAndNotAConstructor => "Operand is a smi and not a constructor",
                    AbortReason::kOperandIsASmiAndNotAFunction => "Operand is a smi and not a function",
                    AbortReason::kOperandIsASmiAndNotAGeneratorObject => {
                        "Operand is a smi and not a generator object"
                    }
                    AbortReason::kOperandIsCleared => "Operand is cleared",
                    AbortReason::kOperandIsNotABoundFunction => "Operand is not a bound function",
                    AbortReason::kOperandIsNotAConstructor => "Operand is not a constructor",
                    AbortReason::kOperandIsNotAFixedArray => "Operand is not a fixed array",
                    AbortReason::kOperandIsNotAFunction => "Operand is not a function",
                    AbortReason::kOperandIsNotACallableFunction => "Operand is not a callable function",
                    AbortReason::kOperandIsNotAGeneratorObject => "Operand is not a generator object",
                    AbortReason::kOperandIsNotACode => "Operand is not a Code object",
                    AbortReason::kOperandIsNotAMap => "Operand is not a Map object",
                    AbortReason::kOperandIsNotASmi => "Operand is not a smi",
                    AbortReason::kPromiseAlreadySettled => "Promise already settled",
                    AbortReason::kReceivedInvalidReturnAddress => "Received invalid return address",
                    AbortReason::kRegisterDidNotMatchExpectedRoot => "Register did not match expected root",
                    AbortReason::kReturnAddressNotFoundInFrame => "Return address not found in frame",
                    AbortReason::kShouldNotDirectlyEnterOsrFunction => {
                        "Should not directly enter OSR-compiled function"
                    }
                    AbortReason::kStackAccessBelowStackPointer => "Stack access below stack pointer",
                    AbortReason::kOsrUnexpectedStackSize => "Unexpected stack size on OSR entry",
                    AbortReason::kStackFrameTypesMustMatch => "Stack frame types must match",
                    AbortReason::kUint32IsNotAInt32 => {
                        "Uint32 cannot be converted to Int32 without loss of precision"
                    }
                    AbortReason::kUnalignedCellInWriteBarrier => "Unaligned cell in write barrier",
                    AbortReason::kUnexpectedAdditionalPopValue => "Unexpected additional pop value",
                    AbortReason::kUnexpectedElementsKindInArrayConstructor => {
                        "Unexpected ElementsKind in array constructor"
                    }
                    AbortReason::kUnexpectedFPCRMode => "Unexpected FPCR mode.",
                    AbortReason::kUnexpectedFunctionIDForInvokeIntrinsic => {
                        "Unexpected runtime function id for the InvokeIntrinsic bytecode"
                    }
                    AbortReason::kUnexpectedInitialMapForArrayFunction => {
                        "Unexpected initial map for Array function"
                    }
                    AbortReason::kUnexpectedLevelAfterReturnFromApiCall => {
                        "Unexpected level after return from api call"
                    }
                    AbortReason::kUnexpectedNegativeValue => "Unexpected negative value",
                    AbortReason::kUnexpectedReturnFromFrameDropper => "Unexpectedly returned from dropping frames",
                    AbortReason::kUnexpectedReturnFromThrow => "Unexpectedly returned from a throw",
                    AbortReason::kUnexpectedReturnFromWasmTrap => "Should not return after throwing a wasm trap",
                    AbortReason::kUnexpectedStackPointer => "The stack pointer is not the expected value",
                    AbortReason::kUnexpectedValue => "Unexpected value",
                    AbortReason::kUninhabitableType => "Uninhabitable type",
                    AbortReason::kUnsupportedModuleOperation => "Unsupported module operation",
                    AbortReason::kUnsupportedNonPrimitiveCompare => "Unsupported non-primitive compare",
                    AbortReason::kWrongAddressOrValuePassedToRecordWrite => {
                        "Wrong address or value passed to RecordWrite"
                    }
                    AbortReason::kWrongArgumentCountForInvokeIntrinsic => {
                        "Wrong number of arguments for intrinsic"
                    }
                    AbortReason::kWrongFunctionCodeStart => "Wrong value in code start register passed",
                    AbortReason::kWrongFunctionContext => "Wrong context passed to function",
                    AbortReason::kWrongFunctionDispatchHandle => {
                        "Wrong value in dispatch handle register passed"
                    }
                    AbortReason::kUnexpectedThreadInWasmSet => "thread_in_wasm flag was already set",
                    AbortReason::kUnexpectedThreadInWasmUnset => "thread_in_wasm flag was not set",
                    AbortReason::kInvalidReceiver => "Expected JS object or primitive object",
                    AbortReason::kUnexpectedInstanceType => "Unexpected instance type encountered",
                    AbortReason::kTurboshaftTypeAssertionFailed => {
                        "A type assertion failed in Turboshaft-generated code"
                    }
                    AbortReason::kMetadataAreaStartDoesNotMatch => "The metadata doesn't belong to the chunk",
                    AbortReason::kExternalPointerTagMismatch => "Tag mismatch during external pointer access",
                    AbortReason::kJSSignatureMismatch => "Signature mismatch during JS function call",
                    AbortReason::kWasmSignatureMismatch => "Signature mismatch during Wasm indirect call",
                    AbortReason::kFastCallFallbackInvalid => "Fast call fallback returned incorrect type",
                    AbortReason::k32BitValueInRegisterIsNotSignExtended => {
                        "32 bit value in register is not sign-extended"
                    }
                    AbortReason::kLastErrorMessage => "kLastErrorMessage",
                }
            }
        }

        pub fn get_bailout_reason(reason: BailoutReason) -> &'static str {
            reason.as_str()
        }

        pub fn get_abort_reason(reason: AbortReason) -> &'static str {
            reason.as_str()
        }

        pub fn is_valid_abort_reason(reason_id: i32) -> bool {
            reason_id >= 0 && reason_id < AbortReason::kLastErrorMessage as i32
        }
    }
}