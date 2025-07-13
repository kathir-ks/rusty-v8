// Converted from V8 C++ source files:
// Header: bailout-reason.h
// Implementation: bailout-reason.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bailout_reason {
    use std::fmt;
    use std::mem::transmute;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    impl Default for BailoutReason {
        fn default() -> Self {
            BailoutReason::kNoReason
        }
    }

    impl fmt::Display for BailoutReason {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", get_bailout_reason(*self))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    impl Default for AbortReason {
        fn default() -> Self {
            AbortReason::kNoReason
        }
    }

    impl fmt::Display for AbortReason {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", get_abort_reason(*self))
        }
    }

    const BAILOUT_MESSAGES: [&str; 14] = [
        "no reason",
        "Bailed out due to dependency change",
        "Maps became deprecated during optimization",
        "Code generation failed",
        "Function is being debugged",
        "Optimized graph construction failed",
        "Function is too big to be optimized",
        "Function contains a call with too many arguments",
        "LiveEdit",
        "Native function literal",
        "Optimization disabled",
        "A higher tier is already available",
        "The native context is detached",
        "Optimization is always disabled",
    ];

    pub fn get_bailout_reason(reason: BailoutReason) -> &'static str {
        BAILOUT_MESSAGES[reason as usize]
    }

    const ABORT_MESSAGES: [&str; 85] = [
        "no reason",
        "32 bit value in register is not zero-extended",
        "Signed bit of 31 bit smi register is not zero",
        "API call returned invalid object",
        "Accumulator clobbered",
        "Allocating non-empty packed array",
        "Allocation is not double aligned",
        "Expected optimized code cell or optimization sentinel",
        "Expected OSR code",
        "Expected undefined or cell in register",
        "Expected feedback cell",
        "Expected feedback vector",
        "Expected baseline data",
        "Float64 cannot be converted to Int32 without loss of precision",
        "The function_data field should be a BytecodeArray on interpreter entry",
        "Input string too long",
        "Input number is too large to fit in a Smi",
        "Invalid bytecode",
        "Cannot advance current bytecode, ",
        "Invoked code which is deoptimized",
        "Invalid HandleScope level",
        "Invalid jump table index",
        "invalid parameters and registers in generator",
        "Missing bytecode array from function",
        "The object is not tagged",
        "The object is tagged",
        "Offset out of range",
        "Operand is a smi",
        "Operand is a smi and not a bound function",
        "Operand is a smi and not a constructor",
        "Operand is a smi and not a function",
        "Operand is a smi and not a generator object",
        "Operand is cleared",
        "Operand is not a bound function",
        "Operand is not a constructor",
        "Operand is not a fixed array",
        "Operand is not a function",
        "Operand is not a callable function",
        "Operand is not a generator object",
        "Operand is not a Code object",
        "Operand is not a Map object",
        "Operand is not a smi",
        "Promise already settled",
        "Received invalid return address",
        "Register did not match expected root",
        "Return address not found in frame",
        "Should not directly enter OSR-compiled function",
        "Stack access below stack pointer",
        "Unexpected stack size on OSR entry",
        "Stack frame types must match",
        "Uint32 cannot be converted to Int32 without loss of precision",
        "Unaligned cell in write barrier",
        "Unexpected additional pop value",
        "Unexpected ElementsKind in array constructor",
        "Unexpected FPCR mode.",
        "Unexpected runtime function id for the InvokeIntrinsic bytecode",
        "Unexpected initial map for Array function",
        "Unexpected level after return from api call",
        "Unexpected negative value",
        "Unexpectedly returned from dropping frames",
        "Unexpectedly returned from a throw",
        "Should not return after throwing a wasm trap",
        "The stack pointer is not the expected value",
        "Unexpected value",
        "Uninhabitable type",
        "Unsupported module operation",
        "Unsupported non-primitive compare",
        "Wrong address or value passed to RecordWrite",
        "Wrong number of arguments for intrinsic",
        "Wrong value in code start register passed",
        "Wrong context passed to function",
        "Wrong value in dispatch handle register passed",
        "thread_in_wasm flag was already set",
        "thread_in_wasm flag was not set",
        "Expected JS object or primitive object",
        "Unexpected instance type encountered",
        "A type assertion failed in Turboshaft-generated code",
        "The metadata doesn't belong to the chunk",
        "Tag mismatch during external pointer access",
        "Signature mismatch during JS function call",
        "Signature mismatch during Wasm indirect call",
        "Fast call fallback returned incorrect type",
        "32 bit value in register is not sign-extended",
    ];

    pub fn get_abort_reason(reason: AbortReason) -> &'static str {
        ABORT_MESSAGES[reason as usize]
    }

    pub fn is_valid_abort_reason(reason_id: i32) -> bool {
        reason_id >= AbortReason::kNoReason as i32 && reason_id < AbortReason::kLastErrorMessage as i32
    }
}
