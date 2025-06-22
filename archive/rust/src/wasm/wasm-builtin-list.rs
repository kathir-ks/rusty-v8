// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This conversion is a high-level approximation and may require
// significant adjustments to integrate correctly with a larger Rust codebase.

#[cfg(not(feature = "webassembly"))]
compile_error!("This module should only be included if WebAssembly is enabled.");

mod macros {
    /// A macro to define the WASM builtins with jump table slots.
    #[macro_export]
    macro_rules! wasm_builtins_with_jump_table_slot {
        ($V:ident, $VTRAP:ident) => {
            $crate::macros::foreach_wasm_trapreason!($VTRAP);
            $V!(WasmCompileLazy);
            $V!(WasmTriggerTierUp);
            $V!(WasmLiftoffFrameSetup);
            $V!(WasmDebugBreak);
            $V!(WasmInt32ToHeapNumber);
            $V!(WasmFloat64ToString);
            $V!(WasmStringToDouble);
            $V!(WasmIntToString);
            $V!(WasmTaggedNonSmiToInt32);
            $V!(WasmFloat32ToNumber);
            $V!(WasmFloat64ToNumber);
            $V!(WasmTaggedToFloat64);
            $V!(WasmAllocateJSArray);
            $V!(WasmI32AtomicWait);
            $V!(WasmI64AtomicWait);
            $V!(WasmGetOwnProperty);
            $V!(WasmRefFunc);
            $V!(WasmInternalFunctionCreateExternal);
            $V!(WasmMemoryGrow);
            $V!(WasmTableInit);
            $V!(WasmTableCopy);
            $V!(WasmTableFill);
            $V!(WasmTableGrow);
            $V!(WasmTableGet);
            $V!(WasmTableSet);
            $V!(WasmTableGetFuncRef);
            $V!(WasmTableSetFuncRef);
            $V!(WasmFunctionTableGet);
            $V!(WasmStackGuard);
            $V!(WasmGrowableStackGuard);
            $V!(WasmStackOverflow);
            $V!(WasmAllocateFixedArray);
            $V!(WasmThrow);
            $V!(WasmRethrow);
            $V!(WasmThrowRef);
            $V!(WasmRethrowExplicitContext);
            $V!(WasmHandleStackOverflow);
            $V!(WasmTraceEnter);
            $V!(WasmTraceExit);
            $V!(WasmTraceMemory);
            $V!(BigIntToI32Pair);
            $V!(BigIntToI64);
            $V!(CallRefIC);
            $V!(CallIndirectIC);
            $V!(DoubleToI);
            $V!(I32PairToBigInt);
            $V!(I64ToBigInt);
            $V!(RecordWriteSaveFP);
            $V!(RecordWriteIgnoreFP);
            $V!(ThrowDataViewTypeError);
            $V!(ThrowDataViewDetachedError);
            $V!(ThrowDataViewOutOfBounds);
            $V!(ThrowIndexOfCalledOnNull);
            $V!(ThrowToLowerCaseCalledOnNull);
            $crate::macros::if_intl!($V, StringToLowerCaseIntl);
            $crate::macros::if_tsan!($V, TSANRelaxedStore8IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore8SaveFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore16IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore16SaveFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore32IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore32SaveFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore64IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedStore64SaveFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore8IgnoreFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore8SaveFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore16IgnoreFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore16SaveFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore32IgnoreFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore32SaveFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore64IgnoreFP);
            $crate::macros::if_tsan!($V, TSANSeqCstStore64SaveFP);
            $crate::macros::if_tsan!($V, TSANRelaxedLoad32IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedLoad32SaveFP);
            $crate::macros::if_tsan!($V, TSANRelaxedLoad64IgnoreFP);
            $crate::macros::if_tsan!($V, TSANRelaxedLoad64SaveFP);
            $V!(WasmAllocateArray_Uninitialized);
            $V!(WasmArrayCopy);
            $V!(WasmArrayNewSegment);
            $V!(WasmArrayInitSegment);
            $V!(WasmAllocateStructWithRtt);
            $V!(WasmOnStackReplace);
            $V!(WasmReject);
            $V!(WasmStringNewWtf8);
            $V!(WasmStringNewWtf16);
            $V!(WasmStringConst);
            $V!(WasmStringMeasureUtf8);
            $V!(WasmStringMeasureWtf8);
            $V!(WasmStringEncodeWtf8);
            $V!(WasmStringEncodeWtf16);
            $V!(WasmStringConcat);
            $V!(WasmStringEqual);
            $V!(WasmStringIsUSVSequence);
            $V!(WasmStringAsWtf16);
            $V!(WasmStringViewWtf16GetCodeUnit);
            $V!(WasmStringCodePointAt);
            $V!(WasmStringViewWtf16Encode);
            $V!(WasmStringViewWtf16Slice);
            $V!(WasmStringNewWtf8Array);
            $V!(WasmStringNewWtf16Array);
            $V!(WasmStringEncodeWtf8Array);
            $V!(WasmStringToUtf8Array);
            $V!(WasmStringEncodeWtf16Array);
            $V!(WasmStringAsWtf8);
            $V!(WasmStringViewWtf8Advance);
            $V!(WasmStringViewWtf8Encode);
            $V!(WasmStringViewWtf8Slice);
            $V!(WasmStringAsIter);
            $V!(WasmStringViewIterNext);
            $V!(WasmStringViewIterAdvance);
            $V!(WasmStringViewIterRewind);
            $V!(WasmStringViewIterSlice);
            $V!(StringCompare);
            $V!(StringIndexOf);
            $V!(WasmStringFromCodePoint);
            $V!(WasmStringHash);
            $V!(WasmAnyConvertExtern);
            $V!(WasmStringFromDataSegment);
            $V!(StringAdd_CheckNone);
            $V!(DebugPrintFloat64);
            $V!(DebugPrintWordPtr);
            $V!(WasmFastApiCallTypeCheckAndUpdateIC);
            $V!(DeoptimizationEntry_Eager);
            $V!(WasmLiftoffDeoptFinish);
            $V!(WasmPropagateException);
            $crate::macros::if_shadow_stack!($V, AdaptShadowStackForDeopt);
        };
    }

    /// A macro to define the WASM builtins without jump table slots.
    #[macro_export]
    macro_rules! wasm_builtins_without_jump_table_slot {
        ($V:ident) => {
            $V!(IterableToFixedArrayForWasm);
            $V!(WasmAllocateInYoungGeneration);
            $V!(WasmAllocateInOldGeneration);
            $V!(WasmAllocateZeroedFixedArray);
            $V!(WasmSuspend);
            $V!(WasmToJsWrapperInvalidSig);
            $V!(WasmTrap);
            $V!(WasmTrapHandlerThrowTrap);
        };
    }

    /// A macro to define the complete list of WASM builtins.
    #[macro_export]
    macro_rules! wasm_builtin_list {
        ($V:ident, $VTRAP:ident) => {
            $crate::macros::wasm_builtins_with_jump_table_slot!($V, $VTRAP);
            $crate::macros::wasm_builtins_without_jump_table_slot!($V);
        };
    }

    /// Placeholder for FOREACH_WASM_TRAPREASON.  Needs to be replaced with actual trap reasons.
    #[macro_export]
    macro_rules! foreach_wasm_trapreason {
        ($VTRAP:ident) => {
            $VTRAP!(DivByZero);
            $VTRAP!(Unreachable);
            $VTRAP!(OOB);
            // Add other trap reasons as needed
        };
    }

    /// Placeholder for IF_INTL.  Needs to be conditionally compiled.
    #[macro_export]
    macro_rules! if_intl {
        ($V:ident, $NAME:ident) => {
            // Conditional compilation based on 'intl' feature flag
            #[cfg(feature = "intl")]
            $V!($NAME);
        };
    }

    /// Placeholder for IF_TSAN. Needs to be conditionally compiled.
    #[macro_export]
    macro_rules! if_tsan {
        ($V:ident, $NAME:ident) => {
            // Conditional compilation based on 'tsan' feature flag.
            #[cfg(feature = "tsan")]
            $V!($NAME);
        };
    }

    /// Placeholder for IF_SHADOW_STACK. Needs to be conditionally compiled.
    #[macro_export]
    macro_rules! if_shadow_stack {
        ($V:ident, $NAME:ident) => {
            // Conditional compilation based on 'shadow_stack' feature flag.
            #[cfg(feature = "shadow_stack")]
            $V!($NAME);
        };
    }
}

pub mod wasm {
    use std::array;
    use std::convert::TryFrom;
    use std::fmt;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Builtin {
        NoBuiltinId, // Added to represent the C++ kNoBuiltinId
        ThrowWasmDivByZero,
        ThrowWasmUnreachable,
        ThrowWasmOOB,
        WasmCompileLazy,
        WasmTriggerTierUp,
        WasmLiftoffFrameSetup,
        WasmDebugBreak,
        WasmInt32ToHeapNumber,
        WasmFloat64ToString,
        WasmStringToDouble,
        WasmIntToString,
        WasmTaggedNonSmiToInt32,
        WasmFloat32ToNumber,
        WasmFloat64ToNumber,
        WasmTaggedToFloat64,
        WasmAllocateJSArray,
        WasmI32AtomicWait,
        WasmI64AtomicWait,
        WasmGetOwnProperty,
        WasmRefFunc,
        WasmInternalFunctionCreateExternal,
        WasmMemoryGrow,
        WasmTableInit,
        WasmTableCopy,
        WasmTableFill,
        WasmTableGrow,
        WasmTableGet,
        WasmTableSet,
        WasmTableGetFuncRef,
        WasmTableSetFuncRef,
        WasmFunctionTableGet,
        WasmStackGuard,
        WasmGrowableStackGuard,
        WasmStackOverflow,
        WasmAllocateFixedArray,
        WasmThrow,
        WasmRethrow,
        WasmThrowRef,
        WasmRethrowExplicitContext,
        WasmHandleStackOverflow,
        WasmTraceEnter,
        WasmTraceExit,
        WasmTraceMemory,
        BigIntToI32Pair,
        BigIntToI64,
        CallRefIC,
        CallIndirectIC,
        DoubleToI,
        I32PairToBigInt,
        I64ToBigInt,
        RecordWriteSaveFP,
        RecordWriteIgnoreFP,
        ThrowDataViewTypeError,
        ThrowDataViewDetachedError,
        ThrowDataViewOutOfBounds,
        ThrowIndexOfCalledOnNull,
        ThrowToLowerCaseCalledOnNull,
        #[cfg(feature = "intl")]
        StringToLowerCaseIntl,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore8IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore8SaveFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore16IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore16SaveFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore32IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore32SaveFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore64IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedStore64SaveFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore8IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore8SaveFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore16IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore16SaveFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore32IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore32SaveFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore64IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANSeqCstStore64SaveFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedLoad32IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedLoad32SaveFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedLoad64IgnoreFP,
        #[cfg(feature = "tsan")]
        TSANRelaxedLoad64SaveFP,
        WasmAllocateArray_Uninitialized,
        WasmArrayCopy,
        WasmArrayNewSegment,
        WasmArrayInitSegment,
        WasmAllocateStructWithRtt,
        WasmOnStackReplace,
        WasmReject,
        WasmStringNewWtf8,
        WasmStringNewWtf16,
        WasmStringConst,
        WasmStringMeasureUtf8,
        WasmStringMeasureWtf8,
        WasmStringEncodeWtf8,
        WasmStringEncodeWtf16,
        WasmStringConcat,
        WasmStringEqual,
        WasmStringIsUSVSequence,
        WasmStringAsWtf16,
        WasmStringViewWtf16GetCodeUnit,
        WasmStringCodePointAt,
        WasmStringViewWtf16Encode,
        WasmStringViewWtf16Slice,
        WasmStringNewWtf8Array,
        WasmStringNewWtf16Array,
        WasmStringEncodeWtf8Array,
        WasmStringToUtf8Array,
        WasmStringEncodeWtf16Array,
        WasmStringAsWtf8,
        WasmStringViewWtf8Advance,
        WasmStringViewWtf8Encode,
        WasmStringViewWtf8Slice,
        WasmStringAsIter,
        WasmStringViewIterNext,
        WasmStringViewIterAdvance,
        WasmStringViewIterRewind,
        WasmStringViewIterSlice,
        StringCompare,
        StringIndexOf,
        WasmStringFromCodePoint,
        WasmStringHash,
        WasmAnyConvertExtern,
        WasmStringFromDataSegment,
        StringAdd_CheckNone,
        DebugPrintFloat64,
        DebugPrintWordPtr,
        WasmFastApiCallTypeCheckAndUpdateIC,
        DeoptimizationEntry_Eager,
        WasmLiftoffDeoptFinish,
        WasmPropagateException,
        #[cfg(feature = "shadow_stack")]
        AdaptShadowStackForDeopt,

        IterableToFixedArrayForWasm,
        WasmAllocateInYoungGeneration,
        WasmAllocateInOldGeneration,
        WasmAllocateZeroedFixedArray,
        WasmSuspend,
        WasmToJsWrapperInvalidSig,
        WasmTrap,
        WasmTrapHandlerThrowTrap,

        kFirstBytecodeHandler,
    }

    impl Builtin {
        pub fn is_wasm_builtin_id(self) -> bool {
            match self {
                Builtin::ThrowWasmDivByZero |
                Builtin::ThrowWasmUnreachable |
                Builtin::ThrowWasmOOB |
                Builtin::WasmCompileLazy |
                Builtin::WasmTriggerTierUp |
                Builtin::WasmLiftoffFrameSetup |
                Builtin::WasmDebugBreak |
                Builtin::WasmInt32ToHeapNumber |
                Builtin::WasmFloat64ToString |
                Builtin::WasmStringToDouble |
                Builtin::WasmIntToString |
                Builtin::WasmTaggedNonSmiToInt32 |
                Builtin::WasmFloat32ToNumber |
                Builtin::WasmFloat64ToNumber |
                Builtin::WasmTaggedToFloat64 |
                Builtin::WasmAllocateJSArray |
                Builtin::WasmI32AtomicWait |
                Builtin::WasmI64AtomicWait |
                Builtin::WasmGetOwnProperty |
                Builtin::WasmRefFunc |
                Builtin::WasmInternalFunctionCreateExternal |
                Builtin::WasmMemoryGrow |
                Builtin::WasmTableInit |
                Builtin::WasmTableCopy |
                Builtin::WasmTableFill |
                Builtin::WasmTableGrow |
                Builtin::WasmTableGet |
                Builtin::WasmTableSet |
                Builtin::WasmTableGetFuncRef |
                Builtin::WasmTableSetFuncRef |
                Builtin::WasmFunctionTableGet |
                Builtin::WasmStackGuard |
                Builtin::WasmGrowableStackGuard |
                Builtin::WasmStackOverflow |
                Builtin::WasmAllocateFixedArray |
                Builtin::WasmThrow |
                Builtin::WasmRethrow |
                Builtin::WasmThrowRef |
                Builtin::WasmRethrowExplicitContext |
                Builtin::WasmHandleStackOverflow |
                Builtin::WasmTraceEnter |
                Builtin::WasmTraceExit |
                Builtin::WasmTraceMemory |
                Builtin::BigIntToI32Pair |
                Builtin::BigIntToI64 |
                Builtin::CallRefIC |
                Builtin::CallIndirectIC |
                Builtin::DoubleToI |
                Builtin::I32PairToBigInt |
                Builtin::I64ToBigInt |
                Builtin::RecordWriteSaveFP |
                Builtin::RecordWriteIgnoreFP |
                Builtin::ThrowDataViewTypeError |
                Builtin::ThrowDataViewDetachedError |
                Builtin::ThrowDataViewOutOfBounds |
                Builtin::ThrowIndexOfCalledOnNull |
                Builtin::ThrowToLowerCaseCalledOnNull |
                #[cfg(feature = "intl")]
                Builtin::StringToLowerCaseIntl |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore8IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore8SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore16IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore16SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore32IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore32SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore64IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedStore64SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore8IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore8SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore16IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore16SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore32IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore32SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore64IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANSeqCstStore64SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedLoad32IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedLoad32SaveFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedLoad64IgnoreFP |
                #[cfg(feature = "tsan")]
                Builtin::TSANRelaxedLoad64SaveFP |
                Builtin::WasmAllocateArray_Uninitialized |
                Builtin::WasmArrayCopy |
                Builtin::WasmArrayNewSegment |
                Builtin::WasmArrayInitSegment |
                Builtin::WasmAllocateStructWithRtt |
                Builtin::WasmOnStackReplace |
                Builtin::WasmReject |
                Builtin::WasmStringNewWtf8 |
                Builtin::WasmStringNewWtf16 |
                Builtin::WasmStringConst |
                Builtin::WasmStringMeasureUtf8 |
                Builtin::WasmStringMeasureWtf8 |
                Builtin::WasmStringEncodeWtf8 |
                Builtin::WasmStringEncodeWtf16 |
                Builtin::WasmStringConcat |
                Builtin::WasmStringEqual |
                Builtin::WasmStringIsUSVSequence |
                Builtin::WasmStringAsWtf16 |
                Builtin::WasmStringViewWtf16GetCodeUnit |
                Builtin::WasmStringCodePointAt |
                Builtin::WasmStringViewWtf16Encode |
                Builtin::WasmStringViewWtf16Slice |
                Builtin::WasmStringNewWtf8Array |
                Builtin::WasmStringNewWtf16Array |
                Builtin::WasmStringEncodeWtf8Array |
                Builtin::WasmStringToUtf8Array |
                Builtin::WasmStringEncodeWtf16Array |
                Builtin::WasmStringAsWtf8 |
                Builtin::WasmStringViewWtf8Advance |
                Builtin::WasmStringViewWtf8Encode |
                Builtin::WasmStringViewWtf8Slice |
                Builtin::WasmStringAsIter |
                Builtin::WasmStringViewIterNext |
                Builtin::WasmStringViewIterAdvance |
                Builtin::WasmStringViewIterRewind |
                Builtin::WasmStringViewIterSlice |
                Builtin::StringCompare |
                Builtin::StringIndexOf |
                Builtin::WasmStringFromCodePoint |
                Builtin::WasmStringHash |
                Builtin::WasmAnyConvertExtern |
                Builtin::WasmStringFromDataSegment |
                Builtin::StringAdd_CheckNone |
                Builtin::DebugPrintFloat64 |
                Builtin::DebugPrintWordPtr |
                Builtin::WasmFastApiCallTypeCheckAndUpdateIC |
                Builtin::DeoptimizationEntry_Eager |
                Builtin::WasmLiftoffDeoptFinish |
                Builtin::WasmPropagateException |
                #[cfg(feature = "shadow_stack")]
                Builtin::AdaptShadowStackForDeopt |
                Builtin::IterableToFixedArrayForWasm |
                Builtin::WasmAllocateInYoungGeneration |
                Builtin::WasmAllocateInOldGeneration |
                Builtin::WasmAllocateZeroedFixedArray |
                Builtin::WasmSuspend |
                Builtin::WasmToJsWrapperInvalidSig |
                Builtin::WasmTrap |
                Builtin::WasmTrapHandlerThrowTrap => true,
                _ => false,
            }
        }
    }

    impl TryFrom<usize> for Builtin {
        type Error = ();

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Builtin::NoBuiltinId),
                1 => Ok(Builtin::ThrowWasmDivByZero),
                2 => Ok(Builtin::ThrowWasmUnreachable),
                3 => Ok(Builtin::ThrowWasmOOB),
                4 => Ok(Builtin::WasmCompileLazy),
                5 => Ok(Builtin::WasmTriggerTierUp),
                6 => Ok(Builtin::WasmLiftoffFrameSetup),
                7 => Ok(Builtin::WasmDebugBreak),
                8 => Ok(Builtin::WasmInt32ToHeapNumber),
                9 => Ok(Builtin::WasmFloat64ToString),
                10 => Ok(Builtin::WasmStringToDouble),
                11 => Ok(Builtin::WasmIntToString),
                12 => Ok(Builtin::WasmTaggedNonSmiToInt32),
                13 => Ok(Builtin::WasmFloat32ToNumber),
                14 => Ok(Builtin::WasmFloat64ToNumber),
                15 => Ok(Builtin::WasmTaggedToFloat64),
                16 => Ok(Builtin::WasmAllocateJSArray),
                17 => Ok(Builtin::WasmI32AtomicWait),
                18 => Ok(Builtin::WasmI64AtomicWait),
                19 => Ok(Builtin::WasmGetOwnProperty),
                20 => Ok(Builtin::WasmRefFunc),
                21 => Ok(Builtin::WasmInternalFunctionCreateExternal),
                22 => Ok(Builtin::WasmMemoryGrow),
                23 => Ok(Builtin::WasmTableInit),
                24 => Ok(Builtin::WasmTableCopy),
                25 => Ok(Builtin::WasmTableFill),
                26 => Ok(Builtin::WasmTableGrow),
                27 => Ok(Builtin::WasmTableGet),
                28 => Ok(Builtin::WasmTableSet),
                29 => Ok(Builtin::WasmTableGetFuncRef),
                30 => Ok(Builtin::WasmTableSetFuncRef),
                31 => Ok(Builtin::WasmFunctionTableGet),
                32 => Ok(Builtin::WasmStackGuard),
                33 => Ok(Builtin::WasmGrowableStackGuard),
                34 => Ok(Builtin::WasmStackOverflow),
                35 => Ok(Builtin::WasmAllocateFixedArray),
                36 => Ok(Builtin::WasmThrow),
                37 => Ok(Builtin::WasmRethrow),
                38 => Ok(Builtin::WasmThrowRef),
                39 => Ok(Builtin::WasmRethrowExplicitContext),
                40 => Ok(Builtin::WasmHandleStackOverflow),
                41 => Ok(Builtin::WasmTraceEnter),
                42 => Ok(Builtin::WasmTraceExit),
                43 => Ok(Builtin::WasmTraceMemory),
                44 => Ok(Builtin::BigIntToI32Pair),
                45 => Ok(Builtin::BigIntToI64),
                46 => Ok(Builtin::CallRefIC),
                47 => Ok(Builtin::CallIndirectIC),
                48 => Ok(Builtin::DoubleToI),
                49 => Ok(Builtin::I32PairToBigInt),
                50 => Ok(Builtin::I64ToBigInt),
                51 => Ok(Builtin::RecordWriteSaveFP),
                52 => Ok(Builtin::RecordWriteIgnoreFP),
                53 => Ok(Builtin::ThrowDataViewTypeError),
                54 => Ok(Builtin::ThrowDataViewDetachedError),
                55 => Ok(Builtin::ThrowDataViewOutOfBounds),
                56 => Ok(Builtin::ThrowIndexOfCalledOnNull),
                57 => Ok(Builtin::ThrowToLowerCaseCalledOnNull),
                #[cfg(feature = "intl")]
                58 => Ok(Builtin::StringToLowerCaseIntl),
                #[cfg(feature = "tsan")]
                59 => Ok(Builtin::TSANRelaxedStore8IgnoreFP),
                #[cfg(feature = "tsan")]
                60 => Ok(Builtin::TSANRelaxedStore8SaveFP),
                #[cfg(feature = "tsan")]
                61 => Ok(Builtin::TSANRelaxedStore16IgnoreFP),
                #[cfg(feature = "tsan")]
                62 => Ok(Builtin::TSANRelaxedStore16SaveFP),
                #[cfg(feature = "tsan")]
                63 => Ok(Builtin::TSANRelaxedStore32IgnoreFP),
                #[cfg(feature = "tsan")]
                64 => Ok(Builtin::TSANRelaxedStore32SaveFP),
                #[cfg(feature = "tsan")]
                65 => Ok(Builtin::TSANRelaxedStore64IgnoreFP),
                #[cfg(feature = "tsan")]
                66 => Ok(Builtin::TSANRelaxedStore64SaveFP),
                #[cfg(feature = "tsan")]
                67 => Ok(Builtin::TSANSeqCstStore8IgnoreFP),
                #[cfg(feature = "tsan")]
                68 => Ok(Builtin::TSANSeqCstStore8SaveFP),
                #[cfg(feature = "tsan")]
                69 => Ok(Builtin::TSANSeqCstStore16IgnoreFP),
                #[cfg(feature = "tsan")]
                70 => Ok(Builtin::TSANSeqCstStore16SaveFP),
                #[cfg(feature = "tsan")]
                71 => Ok(Builtin::TSANSeqCstStore32IgnoreFP),
                #[cfg(feature = "tsan")]
                72 => Ok(Builtin::TSANSeqCstStore32SaveFP),
                #[cfg(feature = "tsan")]
                73 => Ok(Builtin::TSANSeqCstStore64IgnoreFP),
                #[cfg(feature = "tsan")]
                74 => Ok(Builtin::TSANSeqCstStore64SaveFP),
                #[cfg(feature = "tsan")]
                75 => Ok(Builtin::TSANRelaxedLoad32IgnoreFP),
                #[cfg(feature = "tsan")]
                76 => Ok(Builtin::TSANRelaxedLoad32SaveFP),
                #[cfg(feature = "tsan")]
                77 => Ok(Builtin::TSANRelaxedLoad64IgnoreFP),
                #[cfg(feature = "tsan")]
                78 => Ok(Builtin::TSANRelaxedLoad64SaveFP),
                79 => Ok(Builtin::WasmAllocateArray_Uninitialized),
                80 => Ok(Builtin::WasmArrayCopy),
                81 => Ok(Builtin::WasmArrayNewSegment),
                82 => Ok(Builtin::WasmArrayInitSegment),
                83 => Ok(Builtin::WasmAllocateStructWithRtt),
                84 => Ok(Builtin::WasmOnStackReplace),
                85 => Ok(Builtin::WasmReject),
                86 => Ok(Builtin::WasmStringNewWtf8),
                87 => Ok(Builtin::WasmStringNewWtf16),
                88 => Ok(Builtin::WasmStringConst),
                89 => Ok(Builtin::WasmStringMeasureUtf8),
                90 => Ok(Builtin::WasmStringMeasureWtf8),
                91 => Ok(Builtin::WasmStringEncodeWtf8),
                92 => Ok(Builtin::WasmStringEncodeWtf16),
                93 => Ok(Builtin::WasmStringConcat),
                94 => Ok(Builtin::WasmStringEqual),
                95 => Ok(Builtin::WasmStringIsUSVSequence),
                96 => Ok(Builtin::WasmStringAsWtf16),
                97 => Ok(Builtin::WasmStringViewWtf16GetCodeUnit),
                98 => Ok(Builtin::WasmStringCodePointAt),
                99 => Ok(Builtin::WasmStringViewWtf16Encode),
                100 => Ok(Builtin::WasmStringViewWtf16Slice),
                101 => Ok(Builtin::WasmStringNewWtf8Array),
                102 => Ok(Builtin::WasmStringNewWtf16Array),
                103 => Ok(Builtin::WasmStringEncodeWtf8Array),
                104 => Ok(Builtin::WasmStringToUtf8Array),
                105 => Ok(Builtin::WasmStringEncodeWtf16Array),
                106 => Ok(Builtin::WasmStringAsWtf8),