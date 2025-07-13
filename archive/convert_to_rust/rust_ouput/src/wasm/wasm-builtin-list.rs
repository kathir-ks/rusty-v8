// Converted from V8 C++ source files:
// Header: wasm-builtin-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::sync::Mutex;

// src/base/macros.h - No conversion needed, handled by Rust's features
// src/builtins/builtins.h
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Builtin {
    kNoBuiltinId, // Add this as the default value
    kFirstBytecodeHandler, // Add this to represent the first bytecode handler
    kWasmCompileLazy,
    kWasmTriggerTierUp,
    kWasmLiftoffFrameSetup,
    kWasmDebugBreak,
    kWasmInt32ToHeapNumber,
    kWasmFloat64ToString,
    kWasmStringToDouble,
    kWasmIntToString,
    kWasmTaggedNonSmiToInt32,
    kWasmFloat32ToNumber,
    kWasmFloat64ToNumber,
    kWasmTaggedToFloat64,
    kWasmAllocateJSArray,
    kWasmI32AtomicWait,
    kWasmI64AtomicWait,
    kWasmGetOwnProperty,
    kWasmRefFunc,
    kWasmInternalFunctionCreateExternal,
    kWasmMemoryGrow,
    kWasmTableInit,
    kWasmTableCopy,
    kWasmTableFill,
    kWasmTableGrow,
    kWasmTableGet,
    kWasmTableSet,
    kWasmTableGetFuncRef,
    kWasmTableSetFuncRef,
    kWasmFunctionTableGet,
    kWasmStackGuard,
    kWasmGrowableStackGuard,
    kWasmStackOverflow,
    kWasmAllocateFixedArray,
    kWasmThrow,
    kWasmRethrow,
    kWasmThrowRef,
    kWasmRethrowExplicitContext,
    kWasmHandleStackOverflow,
    kWasmTraceEnter,
    kWasmTraceExit,
    kWasmTraceMemory,
    kBigIntToI32Pair,
    kBigIntToI64,
    kCallRefIC,
    kCallIndirectIC,
    kDoubleToI,
    kI32PairToBigInt,
    kI64ToBigInt,
    kRecordWriteSaveFP,
    kRecordWriteIgnoreFP,
    kThrowDataViewTypeError,
    kThrowDataViewDetachedError,
    kThrowDataViewOutOfBounds,
    kThrowIndexOfCalledOnNull,
    kThrowToLowerCaseCalledOnNull,
    kStringToLowerCaseIntl,
    kTSANRelaxedStore8IgnoreFP,
    kTSANRelaxedStore8SaveFP,
    kTSANRelaxedStore16IgnoreFP,
    kTSANRelaxedStore16SaveFP,
    kTSANRelaxedStore32IgnoreFP,
    kTSANRelaxedStore32SaveFP,
    kTSANRelaxedStore64IgnoreFP,
    kTSANRelaxedStore64SaveFP,
    kTSANSeqCstStore8IgnoreFP,
    kTSANSeqCstStore8SaveFP,
    kTSANSeqCstStore16IgnoreFP,
    kTSANSeqCstStore16SaveFP,
    kTSANSeqCstStore32IgnoreFP,
    kTSANSeqCstStore32SaveFP,
    kTSANSeqCstStore64IgnoreFP,
    kTSANSeqCstStore64SaveFP,
    kTSANRelaxedLoad32IgnoreFP,
    kTSANRelaxedLoad32SaveFP,
    kTSANRelaxedLoad64IgnoreFP,
    kTSANRelaxedLoad64SaveFP,
    kWasmAllocateArray_Uninitialized,
    kWasmArrayCopy,
    kWasmArrayNewSegment,
    kWasmArrayInitSegment,
    kWasmAllocateStructWithRtt,
    kWasmOnStackReplace,
    kWasmReject,
    kWasmStringNewWtf8,
    kWasmStringNewWtf16,
    kWasmStringConst,
    kWasmStringMeasureUtf8,
    kWasmStringMeasureWtf8,
    kWasmStringEncodeWtf8,
    kWasmStringEncodeWtf16,
    kWasmStringConcat,
    kWasmStringEqual,
    kWasmStringIsUSVSequence,
    kWasmStringAsWtf16,
    kWasmStringViewWtf16GetCodeUnit,
    kWasmStringCodePointAt,
    kWasmStringViewWtf16Encode,
    kWasmStringViewWtf16Slice,
    kWasmStringNewWtf8Array,
    kWasmStringNewWtf16Array,
    kWasmStringEncodeWtf8Array,
    kWasmStringToUtf8Array,
    kWasmStringEncodeWtf16Array,
    kWasmStringAsWtf8,
    kWasmStringViewWtf8Advance,
    kWasmStringViewWtf8Encode,
    kWasmStringViewWtf8Slice,
    kWasmStringAsIter,
    kWasmStringViewIterNext,
    kWasmStringViewIterAdvance,
    kWasmStringViewIterRewind,
    kWasmStringViewIterSlice,
    kStringCompare,
    kStringIndexOf,
    kWasmStringFromCodePoint,
    kWasmStringHash,
    kWasmAnyConvertExtern,
    kWasmStringFromDataSegment,
    kStringAdd_CheckNone,
    kDebugPrintFloat64,
    kDebugPrintWordPtr,
    kWasmFastApiCallTypeCheckAndUpdateIC,
    kDeoptimizationEntry_Eager,
    kWasmLiftoffDeoptFinish,
    kWasmPropagateException,
    kAdaptShadowStackForDeopt,
    kIterableToFixedArrayForWasm,
    kWasmAllocateInYoungGeneration,
    kWasmAllocateInOldGeneration,
    kWasmAllocateZeroedFixedArray,
    kWasmSuspend,
    kWasmToJsWrapperInvalidSig,
    kWasmTrap,
    kWasmTrapHandlerThrowTrap,
    kThrowWasmUnreachable,
    kThrowWasmIllegalOperation,
    kThrowWasmInvalidTarget,
    kThrowWasmCallIndirect,
    kThrowWasmOutOfBounds,
    kThrowWasmUnalignedAccess,
    kThrowWasmDivByZero,
    kThrowWasmRemByZero,
    kThrowWasmOvfToInt,
    kThrowWasmTrapUnreachable,
    kThrowWasmTrapIllegalOperation,
    kThrowWasmTrapInvalidTarget,
    kThrowWasmTrapCallIndirect,
    kThrowWasmTrapOutOfBounds,
    kThrowWasmTrapUnalignedAccess,
    kThrowWasmTrapDivByZero,
    kThrowWasmTrapRemByZero,
    kThrowWasmTrapOvfToInt,

}

// src/common/globals.h - No conversion needed, handled by Rust's features

// This cfg attribute disables the module if V8_ENABLE_WEBASSEMBLY is not set
// However we assume it *is* enabled for this conversion
//#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
pub mod wasm {
    use super::*;
    use std::array;

    pub mod detail {
        use super::*;

        const BUILTIN_COUNT: usize = {
            let mut count = 0;
            macro_rules! builtin_counter {
                ($name:ident) => {
                    count += 1;
                };
            }
            macro_rules! builtin_counter_trap {
                ($name:ident) => {
                    builtin_counter!(ThrowWasm##$name);
                };
            }
            macro_rules! wasm_builtins_with_jump_table_slot {
                ($callback:ident, $trap_callback:ident) => {
                    $trap_callback!(Unreachable);
                    $trap_callback!(IllegalOperation);
                    $trap_callback!(InvalidTarget);
                    $trap_callback!(CallIndirect);
                    $trap_callback!(OutOfBounds);
                    $trap_callback!(UnalignedAccess);
                    $trap_callback!(DivByZero);
                    $trap_callback!(RemByZero);
                    $trap_callback!(OvfToInt);
                    $trap_callback!(TrapUnreachable);
                    $trap_callback!(TrapIllegalOperation);
                    $trap_callback!(TrapInvalidTarget);
                    $trap_callback!(TrapCallIndirect);
                    $trap_callback!(TrapOutOfBounds);
                    $trap_callback!(TrapUnalignedAccess);
                    $trap_callback!(TrapDivByZero);
                    $trap_callback!(TrapRemByZero);
                    $trap_callback!(TrapOvfToInt);
                    $callback!(WasmCompileLazy);
                    $callback!(WasmTriggerTierUp);
                    $callback!(WasmLiftoffFrameSetup);
                    $callback!(WasmDebugBreak);
                    $callback!(WasmInt32ToHeapNumber);
                    $callback!(WasmFloat64ToString);
                    $callback!(WasmStringToDouble);
                    $callback!(WasmIntToString);
                    $callback!(WasmTaggedNonSmiToInt32);
                    $callback!(WasmFloat32ToNumber);
                    $callback!(WasmFloat64ToNumber);
                    $callback!(WasmTaggedToFloat64);
                    $callback!(WasmAllocateJSArray);
                    $callback!(WasmI32AtomicWait);
                    $callback!(WasmI64AtomicWait);
                    $callback!(WasmGetOwnProperty);
                    $callback!(WasmRefFunc);
                    $callback!(WasmInternalFunctionCreateExternal);
                    $callback!(WasmMemoryGrow);
                    $callback!(WasmTableInit);
                    $callback!(WasmTableCopy);
                    $callback!(WasmTableFill);
                    $callback!(WasmTableGrow);
                    $callback!(WasmTableGet);
                    $callback!(WasmTableSet);
                    $callback!(WasmTableGetFuncRef);
                    $callback!(WasmTableSetFuncRef);
                    $callback!(WasmFunctionTableGet);
                    $callback!(WasmStackGuard);
                    $callback!(WasmGrowableStackGuard);
                    $callback!(WasmStackOverflow);
                    $callback!(WasmAllocateFixedArray);
                    $callback!(WasmThrow);
                    $callback!(WasmRethrow);
                    $callback!(WasmThrowRef);
                    $callback!(WasmRethrowExplicitContext);
                    $callback!(WasmHandleStackOverflow);
                    $callback!(WasmTraceEnter);
                    $callback!(WasmTraceExit);
                    $callback!(WasmTraceMemory);
                    $callback!(BigIntToI32Pair);
                    $callback!(BigIntToI64);
                    $callback!(CallRefIC);
                    $callback!(CallIndirectIC);
                    $callback!(DoubleToI);
                    $callback!(I32PairToBigInt);
                    $callback!(I64ToBigInt);
                    $callback!(RecordWriteSaveFP);
                    $callback!(RecordWriteIgnoreFP);
                    $callback!(ThrowDataViewTypeError);
                    $callback!(ThrowDataViewDetachedError);
                    $callback!(ThrowDataViewOutOfBounds);
                    $callback!(ThrowIndexOfCalledOnNull);
                    $callback!(ThrowToLowerCaseCalledOnNull);
                    $callback!(StringToLowerCaseIntl);
                    $callback!(TSANRelaxedStore8IgnoreFP);
                    $callback!(TSANRelaxedStore8SaveFP);
                    $callback!(TSANRelaxedStore16IgnoreFP);
                    $callback!(TSANRelaxedStore16SaveFP);
                    $callback!(TSANRelaxedStore32IgnoreFP);
                    $callback!(TSANRelaxedStore32SaveFP);
                    $callback!(TSANRelaxedStore64IgnoreFP);
                    $callback!(TSANRelaxedStore64SaveFP);
                    $callback!(TSANSeqCstStore8IgnoreFP);
                    $callback!(TSANSeqCstStore8SaveFP);
                    $callback!(TSANSeqCstStore16IgnoreFP);
                    $callback!(TSANSeqCstStore16SaveFP);
                    $callback!(TSANSeqCstStore32IgnoreFP);
                    $callback!(TSANSeqCstStore32SaveFP);
                    $callback!(TSANSeqCstStore64IgnoreFP);
                    $callback!(TSANSeqCstStore64SaveFP);
                    $callback!(TSANRelaxedLoad32IgnoreFP);
                    $callback!(TSANRelaxedLoad32SaveFP);
                    $callback!(TSANRelaxedLoad64IgnoreFP);
                    $callback!(TSANRelaxedLoad64SaveFP);
                    $callback!(WasmAllocateArray_Uninitialized);
                    $callback!(WasmArrayCopy);
                    $callback!(WasmArrayNewSegment);
                    $callback!(WasmArrayInitSegment);
                    $callback!(WasmAllocateStructWithRtt);
                    $callback!(WasmOnStackReplace);
                    $callback!(WasmReject);
                    $callback!(WasmStringNewWtf8);
                    $callback!(WasmStringNewWtf16);
                    $callback!(WasmStringConst);
                    $callback!(WasmStringMeasureUtf8);
                    $callback!(WasmStringMeasureWtf8);
                    $callback!(WasmStringEncodeWtf8);
                    $callback!(WasmStringEncodeWtf16);
                    $callback!(WasmStringConcat);
                    $callback!(WasmStringEqual);
                    $callback!(WasmStringIsUSVSequence);
                    $callback!(WasmStringAsWtf16);
                    $callback!(WasmStringViewWtf16GetCodeUnit);
                    $callback!(WasmStringCodePointAt);
                    $callback!(WasmStringViewWtf16Encode);
                    $callback!(WasmStringViewWtf16Slice);
                    $callback!(WasmStringNewWtf8Array);
                    $callback!(WasmStringNewWtf16Array);
                    $callback!(WasmStringEncodeWtf8Array);
                    $callback!(WasmStringToUtf8Array);
                    $callback!(WasmStringEncodeWtf16Array);
                    $callback!(WasmStringAsWtf8);
                    $callback!(WasmStringViewWtf8Advance);
                    $callback!(WasmStringViewWtf8Encode);
                    $callback!(WasmStringViewWtf8Slice);
                    $callback!(WasmStringAsIter);
                    $callback!(WasmStringViewIterNext);
                    $callback!(WasmStringViewIterAdvance);
                    $callback!(WasmStringViewIterRewind);
                    $callback!(WasmStringViewIterSlice);
                    $callback!(StringCompare);
                    $callback!(StringIndexOf);
                    $callback!(WasmStringFromCodePoint);
                    $callback!(WasmStringHash);
                    $callback!(WasmAnyConvertExtern);
                    $callback!(WasmStringFromDataSegment);
                    $callback!(StringAdd_CheckNone);
                    $callback!(DebugPrintFloat64);
                    $callback!(DebugPrintWordPtr);
                    $callback!(WasmFastApiCallTypeCheckAndUpdateIC);
                    $callback!(DeoptimizationEntry_Eager);
                    $callback!(WasmLiftoffDeoptFinish);
                    $callback!(WasmPropagateException);
                    $callback!(AdaptShadowStackForDeopt);
                };
            }
            macro_rules! wasm_builtins_without_jump_table_slot {
                ($callback:ident) => {
                    $callback!(IterableToFixedArrayForWasm);
                    $callback!(WasmAllocateInYoungGeneration);
                    $callback!(WasmAllocateInOldGeneration);
                    $callback!(WasmAllocateZeroedFixedArray);
                    $callback!(WasmSuspend);
                    $callback!(WasmToJsWrapperInvalidSig);
                    $callback!(WasmTrap);
                    $callback!(WasmTrapHandlerThrowTrap);
                };
            }

            macro_rules! wasm_builtin_list {
                ($callback:ident, $trap_callback:ident) => {
                    wasm_builtins_with_jump_table_slot!($callback, $trap_callback);
                    wasm_builtins_without_jump_table_slot!($callback);
                };
            }

            wasm_builtin_list!(builtin_counter, builtin_counter_trap);
            count
        };
        pub(crate) fn init_builtin_to_far_jump_table_index() -> [u8; Builtin::kFirstBytecodeHandler as usize] {
            let mut result = [0u8; Builtin::kFirstBytecodeHandler as usize];
            let mut next_index: u8 = 0;

            macro_rules! def_init_lookup {
                ($name:ident) => {
                    result[Builtin::k##$name as usize] = next_index;
                    next_index += 1;
                };
            }

            macro_rules! def_init_lookup_trap {
                ($name:ident) => {
                    def_init_lookup!(ThrowWasm##$name);
                };
            }

            macro_rules! wasm_builtins_with_jump_table_slot {
                ($callback:ident, $trap_callback:ident) => {
                    $trap_callback!(Unreachable);
                    $trap_callback!(IllegalOperation);
                    $trap_callback!(InvalidTarget);
                    $trap_callback!(CallIndirect);
                    $trap_callback!(OutOfBounds);
                    $trap_callback!(UnalignedAccess);
                    $trap_callback!(DivByZero);
                    $trap_callback!(RemByZero);
                    $trap_callback!(OvfToInt);
                    $trap_callback!(TrapUnreachable);
                    $trap_callback!(TrapIllegalOperation);
                    $trap_callback!(TrapInvalidTarget);
                    $trap_callback!(TrapCallIndirect);
                    $trap_callback!(TrapOutOfBounds);
                    $trap_callback!(TrapUnalignedAccess);
                    $trap_callback!(TrapDivByZero);
                    $trap_callback!(TrapRemByZero);
                    $trap_callback!(TrapOvfToInt);
                    $callback!(WasmCompileLazy);
                    $callback!(WasmTriggerTierUp);
                    $callback!(WasmLiftoffFrameSetup);
                    $callback!(WasmDebugBreak);
                    $callback!(WasmInt32ToHeapNumber);
                    $callback!(WasmFloat64ToString);
                    $callback!(WasmStringToDouble);
                    $callback!(WasmIntToString);
                    $callback!(WasmTaggedNonSmiToInt32);
                    $callback!(WasmFloat32ToNumber);
                    $callback!(WasmFloat64ToNumber);
                    $callback!(WasmTaggedToFloat64);
                    $callback!(WasmAllocateJSArray);
                    $callback!(WasmI32AtomicWait);
                    $callback!(WasmI64AtomicWait);
                    $callback!(WasmGetOwnProperty);
                    $callback!(WasmRefFunc);
                    $callback!(WasmInternalFunctionCreateExternal);
                    $callback!(WasmMemoryGrow);
                    $callback!(WasmTableInit);
                    $callback!(WasmTableCopy);
                    $callback!(WasmTableFill);
                    $callback!(WasmTableGrow);
                    $callback!(WasmTableGet);
                    $callback!(WasmTableSet);
                    $callback!(WasmTableGetFuncRef);
                    $callback!(WasmTableSetFuncRef);
                    $callback!(WasmFunctionTableGet);
                    $callback!(WasmStackGuard);
                    $callback!(WasmGrowableStackGuard);
                    $callback!(WasmStackOverflow);
                    $callback!(WasmAllocateFixedArray);
                    $callback!(WasmThrow);
                    $callback!(WasmRethrow);
                    $callback!(WasmThrowRef);
                    $callback!(WasmRethrowExplicitContext);
                    $callback!(WasmHandleStackOverflow);
                    $callback!(WasmTraceEnter);
                    $callback!(WasmTraceExit);
                    $callback!(WasmTraceMemory);
                    $callback!(BigIntToI32Pair);
                    $callback!(BigIntToI64);
                    $callback!(CallRefIC);
                    $callback!(CallIndirectIC);
                    $callback!(DoubleToI);
                    $callback!(I32PairToBigInt);
                    $callback!(I64ToBigInt);
                    $callback!(RecordWriteSaveFP);
                    $callback!(RecordWriteIgnoreFP);
                    $callback!(ThrowDataViewTypeError);
                    $callback!(ThrowDataViewDetachedError);
                    $callback!(ThrowDataViewOutOfBounds);
                    $callback!(ThrowIndexOfCalledOnNull);
                    $callback!(ThrowToLowerCaseCalledOnNull);
                    $callback!(StringToLowerCaseIntl);
                    $callback!(TSANRelaxedStore8IgnoreFP);
                    $callback!(TSANRelaxedStore8SaveFP);
                    $callback!(TSANRelaxedStore16IgnoreFP);
                    $callback!(TSANRelaxedStore16SaveFP);
                    $callback!(TSANRelaxedStore32IgnoreFP);
                    $callback!(TSANRelaxedStore32SaveFP);
                    $callback!(TSANRelaxedStore64IgnoreFP);
                    $callback!(TSANRelaxedStore64SaveFP);
                    $callback!(TSANSeqCstStore8IgnoreFP);
                    $callback!(TSANSeqCstStore8SaveFP);
                    $callback!(TSANSeqCstStore16IgnoreFP);
                    $callback!(TSANSeqCstStore16SaveFP);
                    $callback!(TSANSeqCstStore32IgnoreFP);
                    $callback!(TSANSeqCstStore32SaveFP);
                    $callback!(TSANSeqCstStore64IgnoreFP);
                    $callback!(TSANSeqCstStore64SaveFP);
                    $callback!(TSANRelaxedLoad32IgnoreFP);
                    $callback!(TSANRelaxedLoad32SaveFP);
                    $callback!(TSANRelaxedLoad64IgnoreFP);
                    $callback!(TSANRelaxedLoad64SaveFP);
                    $callback!(WasmAllocateArray_Uninitialized);
                    $callback!(WasmArrayCopy);
                    $callback!(WasmArrayNewSegment);
                    $callback!(WasmArrayInitSegment);
                    $callback!(WasmAllocateStructWithRtt);
                    $callback!(WasmOnStackReplace);
                    $callback!(WasmReject);
                    $callback!(WasmStringNewWtf8);
                    $callback!(WasmStringNewWtf16);
                    $callback!(WasmStringConst);
                    $callback!(WasmStringMeasureUtf8);
                    $callback!(WasmStringMeasureWtf8);
                    $callback!(WasmStringEncodeWtf8);
                    $callback!(WasmStringEncodeWtf16);
                    $callback!(WasmStringConcat);
                    $callback!(WasmStringEqual);
                    $callback!(WasmStringIsUSVSequence);
                    $callback!(WasmStringAsWtf16);
                    $callback!(WasmStringViewWtf16GetCodeUnit);
                    $callback!(WasmStringCodePointAt);
                    $callback!(WasmStringViewWtf16Encode);
                    $callback!(WasmStringViewWtf16Slice);
                    $callback!(WasmStringNewWtf8Array);
                    $callback!(WasmStringNewWtf16Array);
                    $callback!(WasmStringEncodeWtf8Array);
                    $callback!(WasmStringToUtf8Array);
                    $callback!(WasmStringEncodeWtf16Array);
                    $callback!(WasmStringAsWtf8);
                    $callback!(WasmStringViewWtf8Advance);
                    $callback!(WasmStringViewWtf8Encode);
                    $callback!(WasmStringViewWtf8Slice);
                    $callback!(WasmStringAsIter);
                    $callback!(WasmStringViewIterNext);
                    $callback!(WasmStringViewIterAdvance);
                    $callback!(WasmStringViewIterRewind);
                    $callback!(WasmStringViewIterSlice);
                    $callback!(StringCompare);
                    $callback!(StringIndexOf);
                    $callback!(WasmStringFromCodePoint);
                    $callback!(WasmStringHash);
                    $callback!(WasmAnyConvertExtern);
                    $callback!(WasmStringFromDataSegment);
                    $callback!(StringAdd_CheckNone);
                    $callback!(DebugPrintFloat64);
                    $callback!(DebugPrintWordPtr);
                    $callback!(WasmFastApiCallTypeCheckAndUpdateIC);
                    $callback!(DeoptimizationEntry_Eager);
                    $callback!(WasmLiftoffDeoptFinish);
                    $callback!(WasmPropagateException);
                    $callback!(AdaptShadowStackForDeopt);
                };
            }
            wasm_builtins_with_jump_table_slot!(def_init_lookup, def_init_lookup_trap);
            result
        }
    }
    pub struct BuiltinLookup {
    }

    impl BuiltinLookup {
        pub const fn jumptable_index_for_builtin(builtin: Builtin) -> usize {
            let result = Self::K_BUILTIN_TO_FAR_JUMP_TABLE_INDEX[builtin as usize] as usize;
             assert_eq!(builtin, Self::K_FAR_JUMP_TABLE_INDEX_TO_BUILTIN[result]);
            result
        }

        pub const fn builtin_for_jumptable_index(index: usize) -> Builtin {
            let result = Self::K_FAR_JUMP_TABLE_INDEX_TO_BUILTIN[index];
             assert_eq!(index, Self::K_BUILTIN_TO_FAR_JUMP_TABLE_INDEX[result as usize] as usize);
            result
        }

        pub const fn builtin_count() -> usize {
            Self::K_BUILTIN_COUNT
        }

        pub fn is_wasm_builtin_id(id: Builtin) -> bool {
            match id {
                Builtin::kWasmCompileLazy |
                Builtin::kWasmTriggerTierUp |
                Builtin::kWasmLiftoffFrameSetup |
                Builtin::kWasmDebugBreak |
                Builtin::kWasmInt32ToHeapNumber |
                Builtin::kWasmFloat64ToString |
                Builtin::kWasmStringToDouble |
                Builtin::kWasmIntToString |
                Builtin::kWasmTaggedNonSmiToInt32 |
                Builtin::kWasmFloat32ToNumber |
                Builtin::kWasmFloat64ToNumber |
                Builtin::kWasmTaggedToFloat64 |
                Builtin::kWasmAllocateJSArray |
                Builtin::kWasmI32AtomicWait |
                Builtin::kWasmI64AtomicWait |
                Builtin::kWasmGetOwnProperty |
                Builtin::kWasmRefFunc |
                Builtin::kWasmInternalFunctionCreateExternal |
                Builtin::kWasmMemoryGrow |
                Builtin::kWasmTableInit |
                Builtin::kWasmTableCopy |
                Builtin::kWasmTableFill |
                Builtin::kWasmTableGrow |
                Builtin::kWasmTableGet |
                Builtin::kWasmTableSet |
                Builtin::kWasmTableGetFuncRef |
                Builtin::kWasmTableSetFuncRef |
                Builtin::kWasmFunctionTableGet |
                Builtin::kWasmStackGuard |
                Builtin::kWasmGrowableStackGuard |
                Builtin::kWasmStackOverflow |
                Builtin::kWasmAllocateFixedArray |
                Builtin::kWasmThrow |
                Builtin::kWasmRethrow |
                Builtin::kWasmThrowRef |
                Builtin::kWasmRethrowExplicitContext |
                Builtin::kWasmHandleStackOverflow |
                Builtin::kWasmTraceEnter |
                Builtin::kWasmTraceExit |
                Builtin::kWasmTraceMemory |
                Builtin::kBigIntToI32Pair |
                Builtin::kBigIntToI64 |
                Builtin::kCallRefIC |
                Builtin::kCallIndirectIC |
                Builtin::kDoubleToI |
                Builtin::kI32PairToBigInt |
                Builtin::kI64ToBigInt |
                Builtin::kRecordWriteSaveFP |
                Builtin::kRecordWriteIgnoreFP |
                Builtin::kThrowDataViewTypeError |
                Builtin::kThrowDataViewDetachedError |
                Builtin::kThrowDataViewOutOfBounds |
                Builtin::kThrowIndexOfCalledOnNull |
                Builtin::kThrowToLowerCaseCalledOnNull |
                Builtin::kStringToLowerCaseIntl |
                Builtin::kTSANRelaxedStore8IgnoreFP |
                Builtin::kTSANRelaxedStore8SaveFP |
                Builtin::kTSANRelaxedStore16IgnoreFP |
                Builtin::kTSANRelaxedStore16SaveFP |
                Builtin::kTSANRelaxedStore32IgnoreFP |
                Builtin::kTSANRelaxedStore32SaveFP |
                Builtin::kTSANRelaxedStore64IgnoreFP |
                Builtin::kTSANRelaxedStore64SaveFP |
                Builtin::kTSANSeqCstStore8IgnoreFP |
                Builtin::kTSANSeqCstStore8SaveFP |
                Builtin::kTSANSeqCstStore16IgnoreFP |
                Builtin::kTSANSeqCstStore16SaveFP |
                Builtin::kTSANSeqCstStore32IgnoreFP |
                Builtin::kTSANSeqCstStore32SaveFP |
                Builtin::kTSANSeqCstStore64IgnoreFP |
                Builtin::kTSANSeqCstStore64SaveFP |
                Builtin::kTSANRelaxedLoad32IgnoreFP |
                Builtin::kTSANRelaxedLoad32SaveFP |
                Builtin::kTSANRelaxedLoad64IgnoreFP |
                Builtin::kTSANRelaxedLoad64SaveFP |
                Builtin::kWasmAllocateArray_Uninitialized |
                Builtin::kWasmArrayCopy |
                Builtin::kWasmArrayNewSegment |
                Builtin::kWasmArrayInitSegment |
                Builtin::kWasmAllocateStructWithRtt |
                Builtin::kWasmOnStackReplace |
                Builtin::kWasmReject |
                Builtin::kWasmStringNewWtf8 |
                Builtin::kWasmStringNewWtf16 |
                Builtin::kWasmStringConst |
                Builtin::kWasmStringMeasureUtf8 |
                Builtin::kWasmStringMeasureWtf8 |
                Builtin::kWasmStringEncodeWtf8 |
                Builtin::kWasmStringEncodeWtf16 |
                Builtin::kWasmStringConcat |
                Builtin::kWasmStringEqual |
                Builtin::kWasmStringIsUSVSequence |
                Builtin::kWasmStringAsWtf16 |
                Builtin::kWasmStringViewWtf16GetCodeUnit |
                Builtin::kWasmStringCodePointAt |
                Builtin::kWasmStringViewWtf16Encode |
                Builtin::kWasmStringViewWtf16Slice |
                Builtin::kWasmStringNewWtf8Array |
                Builtin::kWasmStringNewWtf16Array |
                Builtin::kWasmStringEncodeWtf8Array |
                Builtin::kWasmStringToUtf8Array |
                Builtin::kWasmStringEncodeWtf16Array |
                Builtin::kWasmStringAsWtf8 |
                Builtin::kWasmStringViewWtf8Advance |
                Builtin::kWasmStringViewWtf8Encode |
                Builtin::kWasmStringViewWtf8Slice |
                Builtin::kWasmStringAsIter |
                Builtin::kWasmStringViewIterNext |
                Builtin::kWasmStringViewIterAdvance |
                Builtin::kWasmStringViewIterRewind |
                Builtin::kWasmStringViewIterSlice |
                Builtin::kStringCompare |
                Builtin::kStringIndexOf |
                Builtin::kWasmStringFromCodePoint |
                Builtin::kWasmStringHash |
                Builtin::kWasmAnyConvertExtern |
                Builtin::kWasmStringFromDataSegment |
                Builtin::kStringAdd_CheckNone |
                Builtin::kDebugPrintFloat64 |
                Builtin::kDebugPrintWordPtr |
                Builtin::kWasmFastApiCallTypeCheckAndUpdateIC |
                Builtin::kDeoptimizationEntry_Eager |
                Builtin::kWasmLiftoffDeoptFinish |
                Builtin::kWasmPropagateException |
                Builtin::kAdaptShadowStackForDeopt |
                Builtin::kIterableToFixedArrayForWasm |
                Builtin::kWasmAllocateInYoungGeneration |
                Builtin::kWasmAllocateInOldGeneration |
                Builtin::kWasmAllocateZeroedFixedArray |
                Builtin::kWasmSuspend |
                Builtin::kWasmToJsWrapperInvalidSig |
                Builtin::kWasmTrap |
                Builtin::kWasmTrapHandlerThrowTrap |
                Builtin::kThrowWasmUnreachable |
                Builtin::kThrowWasmIllegalOperation |
                Builtin::kThrowWasmInvalidTarget |
                Builtin::kThrowWasmCallIndirect |
                Builtin::kThrowWasmOutOfBounds |
                Builtin::kThrowWasmUnalignedAccess |
                Builtin::kThrowWasmDivByZero |
                Builtin::kThrowWasmRemByZero |
                Builtin::kThrowWasmOvfToInt |
                Builtin::kThrowWasmTrapUnreachable |
                Builtin::kThrowWasmTrapIllegalOperation |
                Builtin::kThrowWasmTrapInvalidTarget |
                Builtin::kThrowWasmTrapCallIndirect |
                Builtin::kThrowWasmTrapOutOfBounds |
                Builtin::kThrowWasmTrapUnalignedAccess |
                Builtin::kThrowWasmTrapDivByZero |
                Builtin::kThrowWasmTrapRemByZero |
                Builtin::kThrowWasmTrapOvfToInt => true,
                _ => false,
            }
        }

        const K_BUILTIN_COUNT: usize = {
            let mut count = 0;
            macro_rules! builtin_counter {
                ($name:ident) => {
                    count += 1;
                };
            }

            macro_rules! builtin_counter_trap {
                ($name:ident) => {
                    builtin_counter!(ThrowWasm##$name);
                };
            }

            macro_rules! wasm_builtins_with_jump_table_slot {
                ($callback:ident, $trap_callback:ident) => {
                    $trap_callback!(Unreachable);
                    $trap_callback!(IllegalOperation);
                    $trap_callback!(InvalidTarget);
                    $trap_callback!(CallIndirect);
                    $trap_callback!(OutOfBounds);
                    $trap_callback!(UnalignedAccess);
                    $trap_callback!(DivByZero);
                    $trap_callback!(RemByZero);
                    $trap_callback!(OvfToInt);
                    $trap_callback!(TrapUnreachable);
                    $trap_callback!(TrapIllegalOperation);
                    $trap_callback!(TrapInvalidTarget);
                    $trap_callback!(TrapCallIndirect);
                    $trap_callback!(TrapOutOfBounds);
                    $trap_callback!(TrapUnalignedAccess);
                    $trap_callback!(TrapDivByZero);
                    $trap_
