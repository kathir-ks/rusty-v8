// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/runtime/runtime.rs

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::{Once, ONCE_INIT};

// Placeholder types and functions.  These would need to be defined
// elsewhere based on the broader V8 architecture.
type Address = usize; // Or a more specific pointer type
type Isolate = usize; // Or a more specific struct
type ObjectPair = (Address, Address); // Or a more specific struct
type FunctionAddr = Address;
type CompareCharsEqualFn = fn(*const u8, *const u8, usize) -> bool; // Define the function type
extern "C" {
    fn CompareCharsEqual(a: *const u8, b: *const u8, len: usize) -> bool;
}

const ZERO_HASH_SEED: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FunctionId {
    // Add function IDs from FOR_EACH_INTRINSIC here
    kAbort, // Example
    kAbortCSADcheck,
    kAbortJS,
    kSystemBreak,
    kBenchMaglev,
    kBenchTurbofan,
    kDebugPrint,
    kDisassembleFunction,
    kGetFunctionForCurrentFrame,
    kGetCallable,
    kGetAbstractModuleSource,
    kTurbofanStaticAssert,
    kClearFunctionFeedback,
    kCompleteInobjectSlackTracking,
    kCompleteInobjectSlackTrackingForMap,
    kGlobalPrint,
    kLeakHole,
    kShareObject,
    kForceFlush,
    kArrayBufferDetach,
    kDeoptimizeFunction,
    kDeoptimizeNow,
    kDisableOptimizationFinalization,
    kEnableCodeLoggingForTesting,
    kFinalizeOptimization,
    kGetUndetectable,
    kNeverOptimizeFunction,
    kOptimizeFunctionOnNextCall,
    kOptimizeMaglevOnNextCall,
    kOptimizeOsr,
    kPrepareFunctionForOptimization,
    kPretenureAllocationSite,
    kSetAllocationTimeout,
    kSetForceSlowPath,
    kSimulateNewspaceFull,
    kWaitForBackgroundOptimization,
    kSetBatterySaverMode,
    kSetPriorityBestEffort,
    kSetPriorityUserVisible,
    kSetPriorityUserBlocking,
    kIsEfficiencyModeEnabled,
    kBaselineOsr,
    kCompileBaseline,
    kWasmGenerateRandomModule,
    kWasmArray,
    kWasmStruct,
    kWasmTierUpFunction,
    kWasmTriggerTierUpForTesting,
    kConstructDouble,
    kConstructConsString,
    kConstructSlicedString,
    kConstructInternalizedString,
    kConstructThinString,
    kSerializeDeserializeNow,
    kCreatePrivateAccessors,
    kCopyDataProperties,
    kCreateDataProperty,
    kCreatePrivateNameSymbol,
    kCreatePrivateBrandSymbol,
    kLoadPrivateGetter,
    kLoadPrivateSetter,
    kReThrow,
    kReThrowWithMessage,
    kThrow,
    kThrowApplyNonFunction,
    kThrowCalledNonCallable,
    kThrowConstAssignError,
    kThrowConstructorNonCallableError,
    kThrowConstructedNonConstructable,
    kThrowConstructorReturnedNonObject,
    kThrowInvalidStringLength,
    kThrowInvalidTypedArrayAlignment,
    kThrowIteratorError,
    kThrowIteratorResultNotAnObject,
    kThrowNotConstructor,
    kThrowRangeError,
    kThrowReferenceError,
    kThrowAccessedUninitializedVariable,
    kThrowStackOverflow,
    kThrowStaticPrototypeError,
    kThrowSuperAlreadyCalledError,
    kThrowSuperNotCalled,
    kThrowSymbolAsyncIteratorInvalid,
    kThrowSymbolIteratorInvalid,
    kThrowThrowMethodMissing,
    kThrowTypeError,
    kThrowUnsupportedSuperError,
    kTerminateExecution,
    kThrowWasmError,
    kThrowWasmStackOverflow,
    kThrowWasmSuspendError,
    kWasmTraceEnter,
    kWasmTraceExit,
    kWasmTraceMemory,
    kCheckIsOnCentralStack,
    kSetWasmInstantiateControls,
    kWasmNull,
    kFreezeWasmLazyCompilation,
    kDeserializeWasmModule,
    kInlineAsyncFunctionReject,
    kInlineAsyncFunctionResolve,

    // Add more IDs as needed
}

#[derive(Clone, Copy, Debug)]
pub enum RuntimeType {
    RUNTIME,
    INLINE,
}

#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub id: FunctionId,
    pub runtime_type: RuntimeType,
    pub name: &'static str,
    pub entry: FunctionAddr,
    pub number_of_args: i32,
    pub result_size: i32,
}

// Define the intrinsic function table.  The actual function implementations
// are just placeholders here and would need to be defined elsewhere.
static K_INTRINSIC_FUNCTIONS: &[Function] = &[
    // Example entry, replace with actual definitions
    //  Note: In Rust, you can't have a direct function pointer to another function defined in Rust
    //  that accepts Address, Isolate as parameters when Address is defined as usize. It's better
    //  to create a wrapper function with C ABI that calls the Rust functions.
    Function {
        id: FunctionId::kAbort,
        runtime_type: RuntimeType::RUNTIME,
        name: "Abort",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kAbortCSADcheck,
        runtime_type: RuntimeType::RUNTIME,
        name: "AbortCSADcheck",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kAbortJS,
        runtime_type: RuntimeType::RUNTIME,
        name: "AbortJS",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSystemBreak,
        runtime_type: RuntimeType::RUNTIME,
        name: "SystemBreak",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kBenchMaglev,
        runtime_type: RuntimeType::RUNTIME,
        name: "BenchMaglev",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kBenchTurbofan,
        runtime_type: RuntimeType::RUNTIME,
        name: "BenchTurbofan",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kDebugPrint,
        runtime_type: RuntimeType::RUNTIME,
        name: "DebugPrint",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kDisassembleFunction,
        runtime_type: RuntimeType::RUNTIME,
        name: "DisassembleFunction",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kGetFunctionForCurrentFrame,
        runtime_type: RuntimeType::RUNTIME,
        name: "GetFunctionForCurrentFrame",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kGetCallable,
        runtime_type: RuntimeType::RUNTIME,
        name: "GetCallable",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kGetAbstractModuleSource,
        runtime_type: RuntimeType::RUNTIME,
        name: "GetAbstractModuleSource",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kTurbofanStaticAssert,
        runtime_type: RuntimeType::RUNTIME,
        name: "TurbofanStaticAssert",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kClearFunctionFeedback,
        runtime_type: RuntimeType::RUNTIME,
        name: "ClearFunctionFeedback",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCompleteInobjectSlackTracking,
        runtime_type: RuntimeType::RUNTIME,
        name: "CompleteInobjectSlackTracking",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCompleteInobjectSlackTrackingForMap,
        runtime_type: RuntimeType::RUNTIME,
        name: "CompleteInobjectSlackTrackingForMap",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kGlobalPrint,
        runtime_type: RuntimeType::RUNTIME,
        name: "GlobalPrint",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kLeakHole,
        runtime_type: RuntimeType::RUNTIME,
        name: "LeakHole",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kShareObject,
        runtime_type: RuntimeType::RUNTIME,
        name: "ShareObject",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kForceFlush,
        runtime_type: RuntimeType::RUNTIME,
        name: "ForceFlush",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kArrayBufferDetach,
        runtime_type: RuntimeType::RUNTIME,
        name: "ArrayBufferDetach",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kDeoptimizeFunction,
        runtime_type: RuntimeType::RUNTIME,
        name: "DeoptimizeFunction",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kDeoptimizeNow,
        runtime_type: RuntimeType::RUNTIME,
        name: "DeoptimizeNow",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kDisableOptimizationFinalization,
        runtime_type: RuntimeType::RUNTIME,
        name: "DisableOptimizationFinalization",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kEnableCodeLoggingForTesting,
        runtime_type: RuntimeType::RUNTIME,
        name: "EnableCodeLoggingForTesting",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kFinalizeOptimization,
        runtime_type: RuntimeType::RUNTIME,
        name: "FinalizeOptimization",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kGetUndetectable,
        runtime_type: RuntimeType::RUNTIME,
        name: "GetUndetectable",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kNeverOptimizeFunction,
        runtime_type: RuntimeType::RUNTIME,
        name: "NeverOptimizeFunction",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kOptimizeFunctionOnNextCall,
        runtime_type: RuntimeType::RUNTIME,
        name: "OptimizeFunctionOnNextCall",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kOptimizeMaglevOnNextCall,
        runtime_type: RuntimeType::RUNTIME,
        name: "OptimizeMaglevOnNextCall",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kOptimizeOsr,
        runtime_type: RuntimeType::RUNTIME,
        name: "OptimizeOsr",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kPrepareFunctionForOptimization,
        runtime_type: RuntimeType::RUNTIME,
        name: "PrepareFunctionForOptimization",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kPretenureAllocationSite,
        runtime_type: RuntimeType::RUNTIME,
        name: "PretenureAllocationSite",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetAllocationTimeout,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetAllocationTimeout",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetForceSlowPath,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetForceSlowPath",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSimulateNewspaceFull,
        runtime_type: RuntimeType::RUNTIME,
        name: "SimulateNewspaceFull",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWaitForBackgroundOptimization,
        runtime_type: RuntimeType::RUNTIME,
        name: "WaitForBackgroundOptimization",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetBatterySaverMode,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetBatterySaverMode",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetPriorityBestEffort,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetPriorityBestEffort",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetPriorityUserVisible,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetPriorityUserVisible",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSetPriorityUserBlocking,
        runtime_type: RuntimeType::RUNTIME,
        name: "SetPriorityUserBlocking",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kIsEfficiencyModeEnabled,
        runtime_type: RuntimeType::RUNTIME,
        name: "IsEfficiencyModeEnabled",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kBaselineOsr,
        runtime_type: RuntimeType::RUNTIME,
        name: "BaselineOsr",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCompileBaseline,
        runtime_type: RuntimeType::RUNTIME,
        name: "CompileBaseline",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWasmGenerateRandomModule,
        runtime_type: RuntimeType::RUNTIME,
        name: "WasmGenerateRandomModule",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWasmArray,
        runtime_type: RuntimeType::RUNTIME,
        name: "WasmArray",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWasmStruct,
        runtime_type: RuntimeType::RUNTIME,
        name: "WasmStruct",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWasmTierUpFunction,
        runtime_type: RuntimeType::RUNTIME,
        name: "WasmTierUpFunction",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kWasmTriggerTierUpForTesting,
        runtime_type: RuntimeType::RUNTIME,
        name: "WasmTriggerTierUpForTesting",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kConstructDouble,
        runtime_type: RuntimeType::RUNTIME,
        name: "ConstructDouble",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kConstructConsString,
        runtime_type: RuntimeType::RUNTIME,
        name: "ConstructConsString",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kConstructSlicedString,
        runtime_type: RuntimeType::RUNTIME,
        name: "ConstructSlicedString",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kConstructInternalizedString,
        runtime_type: RuntimeType::RUNTIME,
        name: "ConstructInternalizedString",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kConstructThinString,
        runtime_type: RuntimeType::RUNTIME,
        name: "ConstructThinString",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kSerializeDeserializeNow,
        runtime_type: RuntimeType::RUNTIME,
        name: "SerializeDeserializeNow",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCreatePrivateAccessors,
        runtime_type: RuntimeType::RUNTIME,
        name: "CreatePrivateAccessors",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCopyDataProperties,
        runtime_type: RuntimeType::RUNTIME,
        name: "CopyDataProperties",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCreateDataProperty,
        runtime_type: RuntimeType::RUNTIME,
        name: "CreateDataProperty",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCreatePrivateNameSymbol,
        runtime_type: RuntimeType::RUNTIME,
        name: "CreatePrivateNameSymbol",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kCreatePrivateBrandSymbol,
        runtime_type: RuntimeType::RUNTIME,
        name: "CreatePrivateBrandSymbol",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kLoadPrivateGetter,
        runtime_type: RuntimeType::RUNTIME,
        name: "LoadPrivateGetter",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kLoadPrivateSetter,
        runtime_type: RuntimeType::RUNTIME,
        name: "LoadPrivateSetter",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kReThrow,
        runtime_type: RuntimeType::RUNTIME,
        name: "ReThrow",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kReThrowWithMessage,
        runtime_type: RuntimeType::RUNTIME,
        name: "ReThrowWithMessage",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrow,
        runtime_type: RuntimeType::RUNTIME,
        name: "Throw",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowApplyNonFunction,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowApplyNonFunction",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowCalledNonCallable,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowCalledNonCallable",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowConstAssignError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowConstAssignError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowConstructorNonCallableError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowConstructorNonCallableError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowConstructedNonConstructable,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowConstructedNonConstructable",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowConstructorReturnedNonObject,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowConstructorReturnedNonObject",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowInvalidStringLength,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowInvalidStringLength",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowInvalidTypedArrayAlignment,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowInvalidTypedArrayAlignment",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowIteratorError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowIteratorError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowIteratorResultNotAnObject,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowIteratorResultNotAnObject",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowNotConstructor,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowNotConstructor",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowRangeError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowRangeError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowReferenceError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowReferenceError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowAccessedUninitializedVariable,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowAccessedUninitializedVariable",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowStackOverflow,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowStackOverflow",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowStaticPrototypeError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowStaticPrototypeError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowSuperAlreadyCalledError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowSuperAlreadyCalledError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowSuperNotCalled,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowSuperNotCalled",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowSymbolAsyncIteratorInvalid,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowSymbolAsyncIteratorInvalid",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowSymbolIteratorInvalid,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowSymbolIteratorInvalid",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowThrowMethodMissing,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowThrowMethodMissing",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowTypeError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowTypeError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0,
    },
    Function {
        id: FunctionId::kThrowUnsupportedSuperError,
        runtime_type: RuntimeType::RUNTIME,
        name: "ThrowUnsupportedSuperError",
        entry: 0, // Replace with actual address, consider using extern "C"
        number_of_args: 0,
        result_size: 0