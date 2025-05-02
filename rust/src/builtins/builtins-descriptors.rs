// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod builtins_definitions;
pub mod codegen;
pub mod common;

pub mod builtins_descriptors {
    use super::builtins_definitions::*;
    use super::codegen::interface_descriptors::JSTrampolineDescriptor;

    #[cfg(v8_js_linkage_includes_dispatch_handle)]
    macro_rules! define_tfj_parameter_indices {
        ($($tail:tt)*) => {
            #[allow(dead_code)]
            #[derive(Debug, Copy, Clone)]
            pub enum ParameterIndices {
                JSTarget = super::codegen::interface_descriptors::JSCallClosureParameterIndex as isize,
                $($tail,)*
                JSNewTarget,
                JSActualArgumentsCount,
                JSDispatchHandle,
                Context,
                ParameterCount,
            }
            pub const K_JS_BUILTIN_BASE_PARAMETER_COUNT: usize = 4;
        };
    }

    #[cfg(not(v8_js_linkage_includes_dispatch_handle))]
    macro_rules! define_tfj_parameter_indices {
        ($($tail:tt)*) => {
            #[allow(dead_code)]
            #[derive(Debug, Copy, Clone)]
            pub enum ParameterIndices {
                JSTarget = super::codegen::interface_descriptors::JSCallClosureParameterIndex as isize,
                $($tail,)*
                JSNewTarget,
                JSActualArgumentsCount,
                Context,
                ParameterCount,
            }
            pub const K_JS_BUILTIN_BASE_PARAMETER_COUNT: usize = 3;
        };
    }

    macro_rules! define_tfj_interface_descriptor {
        ($name:ident, $argc:expr, $($tail:tt)*) => {
            pub struct $name {
                _private: (),
            }
            impl $name {
                define_tfj_parameter_indices!($($tail,)*);

                pub const K_PARAMETER_COUNT: usize = K_JS_BUILTIN_BASE_PARAMETER_COUNT + ($argc as usize);

                //These asserts cannot be expressed directly in Rust's macro system
                //static_assert!(K_PARAMETER_COUNT == K_JS_BUILTIN_BASE_PARAMETER_COUNT + ($argc));
                //static_assert!(($argc) == (K_PARAMETER_COUNT - K_JS_BUILTIN_BASE_PARAMETER_COUNT) as u16);
                //static_assert!(K_PARAMETER_COUNT - ($argc) == JSTrampolineDescriptor::kParameterCount);
                //static_assert!(ParameterIndices::kJSTarget as isize == -1);
            }
        };
    }

    macro_rules! define_tsj_interface_descriptor {
        ($($args:tt)*) => {
            define_tfj_interface_descriptor!($($args)*);
        };
    }

    macro_rules! define_tsc_interface_descriptor {
        ($name:ident, $interface_descriptor:ident) => {
            pub type $name = $interface_descriptor;
        };
    }

    macro_rules! define_tfc_interface_descriptor {
        ($name:ident, $interface_descriptor:ident) => {
            pub type $name = $interface_descriptor;
        };
    }

    macro_rules! define_tfs_interface_descriptor {
        ($name:ident, $($tail:tt)*) => {
            pub type $name = super::codegen::interface_descriptors::$name;
        };
    }

    macro_rules! define_tfh_interface_descriptor {
        ($name:ident, $interface_descriptor:ident) => {
            pub type $name = $interface_descriptor;
        };
    }

    macro_rules! define_asm_interface_descriptor {
        ($name:ident, $interface_descriptor:ident) => {
            pub type $name = $interface_descriptor;
        };
    }

    macro_rules! builtin_list_dispatch {
        (
            $ignore_builtin:ident,
            $define_tsj_interface_descriptor:ident,
            $define_tfj_interface_descriptor:ident,
            $define_tsc_interface_descriptor:ident,
            $define_tfc_interface_descriptor:ident,
            $define_tfs_interface_descriptor:ident,
            $define_tfh_interface_descriptor:ident,
            $ignore_builtin2:ident,
            $define_asm_interface_descriptor:ident,
            $(($builtin_name:ident, $linkage:ident, $($args:tt)*)),*
        ) => {
            $(
                match $linkage {
                    Builtin::TSJ => {
                        $define_tsj_interface_descriptor!($builtin_name, $($args)*);
                    }
                    Builtin::TFJ => {
                        $define_tfj_interface_descriptor!($builtin_name, $($args)*);
                    }
                    Builtin::TSC => {
                        let interface_descriptor = stringify!($($args)*);
                        println!("TSC macro arm needs to be implemented");
                        //Currently the argument passing from C++ macro to Rust macro is not implemented for TSC
                    }
                    Builtin::TFC => {
                        let interface_descriptor = stringify!($($args)*);
                        println!("TFC macro arm needs to be implemented");
                        //Currently the argument passing from C++ macro to Rust macro is not implemented for TFC
                    }
                    Builtin::TFS => {
                        $define_tfs_interface_descriptor!($builtin_name, $($args)*);
                    }
                    Builtin::TFH => {
                        let interface_descriptor = stringify!($($args)*);
                        println!("TFH macro arm needs to be implemented");
                        //Currently the argument passing from C++ macro to Rust macro is not implemented for TFH
                    }
                    Builtin::ASM => {
                        let interface_descriptor = stringify!($($args)*);
                        println!("ASM macro arm needs to be implemented");
                        //Currently the argument passing from C++ macro to Rust macro is not implemented for ASM
                    }
                    _ => {}
                }
            )*
        };
    }

    builtin_list_dispatch!(
        ignore_builtin,
        define_tsj_interface_descriptor,
        define_tfj_interface_descriptor,
        define_tsc_interface_descriptor,
        define_tfc_interface_descriptor,
        define_tfs_interface_descriptor,
        define_tfh_interface_descriptor,
        ignore_builtin,
        define_asm_interface_descriptor,
        (Abort, Builtin::TSJ, 1),
        (Add, Builtin::TSJ, 2),
        (ArgumentsAdaptorTrampoline, Builtin::TFS),
        (ArrayConstructor, Builtin::TSJ, 2),
        (ArrayPrototypeJoin, Builtin::TSJ, 1),
        (ArrayPrototypePop, Builtin::TSJ, 0),
        (ArrayPrototypePush, Builtin::TSJ, 2, Arg0),
        (ArrayPrototypeShift, Builtin::TSJ, 0),
        (ArrayPrototypeSlice, Builtin::TSJ, 2),
        (ArrayPrototypeSplice, Builtin::TSJ, 3, Arg0, Arg1),
        (ArrayPrototypeUnshift, Builtin::TSJ, 2, Arg0),
        (Call, Builtin::TSJ, 1),
        (CallWithArrayLike, Builtin::TSJ, 2),
        (ClassConstructor, Builtin::TSJ, 1),
        (Construct, Builtin::TSJ, 2),
        (ConstructWithArrayLike, Builtin::TSJ, 3),
        (CreateAsyncFromSyncIterator, Builtin::TSJ, 1),
        (DebugPrint, Builtin::TSJ, 1),
        (DefaultErrorThrower, Builtin::TSJ, 1),
        (DeleteProperty, Builtin::TSJ, 2),
        (DeserializeLazy, Builtin::TSJ, 0),
        (GeneratorPrototypeNext, Builtin::TSJ, 1),
        (GeneratorPrototypeReturn, Builtin::TSJ, 1),
        (GeneratorPrototypeThrow, Builtin::TSJ, 1),
        (GetSuperConstructor, Builtin::TSJ, 0),
        (GlobalPrint, Builtin::TSJ, 1),
        (HandleDebuggerStatement, Builtin::TSJ, 0),
        (HasProperty, Builtin::TSJ, 2),
        (Increment, Builtin::TSJ, 1),
        (IncorporateModule, Builtin::TSJ, 1),
        (InstanceOf, Builtin::TSJ, 2),
        (InternalArrayConstructor, Builtin::TSJ, 2),
        (InternalArrayPop, Builtin::TSJ, 0),
        (InternalArrayPush, Builtin::TSJ, 2, Arg0),
        (InternalArrayShift, Builtin::TSJ, 0),
        (InternalArrayUnshift, Builtin::TSJ, 2, Arg0),
        (JsonStringify, Builtin::TSJ, 3),
        (KeyedHasIC, Builtin::TFH, NameDictionary),
        (LoadIC, Builtin::TFH, Load),
        (LoadWithVectorIC, Builtin::TFH, LoadWithVector),
        (MathAcos, Builtin::TSJ, 1),
        (MathAcosh, Builtin::TSJ, 1),
        (MathAsin, Builtin::TSJ, 1),
        (MathAsinh, Builtin::TSJ, 1),
        (MathAtan, Builtin::TSJ, 1),
        (MathAtan2, Builtin::TSJ, 2),
        (MathAtanh, Builtin::TSJ, 1),
        (MathCbrt, Builtin::TSJ, 1),
        (MathCeil, Builtin::TSJ, 1),
        (MathClz32, Builtin::TSJ, 1),
        (MathCos, Builtin::TSJ, 1),
        (MathCosh, Builtin::TSJ, 1),
        (MathExp, Builtin::TSJ, 1),
        (MathExpm1, Builtin::TSJ, 1),
        (MathFloor, Builtin::TSJ, 1),
        (MathFround, Builtin::TSJ, 1),
        (MathHypot, Builtin::TSJ, 2, Arg0),
        (MathImul, Builtin::TSJ, 2),
        (MathLog, Builtin::TSJ, 1),
        (MathLog1p, Builtin::TSJ, 1),
        (MathLog2, Builtin::TSJ, 1),
        (MathLog10, Builtin::TSJ, 1),
        (MathMax, Builtin::TSJ, 2, Arg0),
        (MathMin, Builtin::TSJ, 2, Arg0),
        (MathPow, Builtin::TSJ, 2),
        (MathRound, Builtin::TSJ, 1),
        (MathSign, Builtin::TSJ, 1),
        (MathSin, Builtin::TSJ, 1),
        (MathSinh, Builtin::TSJ, 1),
        (MathSqrt, Builtin::TSJ, 1),
        (MathTan, Builtin::TSJ, 1),
        (MathTanh, Builtin::TSJ, 1),
        (MathTrunc, Builtin::TSJ, 1),
        (NewConsString, Builtin::TSJ, 2),
        (NumberConstructor, Builtin::TSJ, 1),
        (NumberPrototypeToString, Builtin::TSJ, 1),
        (ObjectConstructor, Builtin::TSJ, 1),
        (ObjectCreate, Builtin::TSJ, 2),
        (ObjectEntries, Builtin::TSJ, 1),
        (ObjectGetOwnPropertyDescriptor, Builtin::TSJ, 2),
        (ObjectHasOwn, Builtin::TSJ, 2),
        (ObjectIs, Builtin::TSJ, 2),
        (ObjectKeys, Builtin::TSJ, 1),
        (ObjectValues, Builtin::TSJ, 1),
        (OptimizeOsr, Builtin::TSJ, 0),
        (ParseJson, Builtin::TSJ, 2),
        (PromiseConstructor, Builtin::TSJ, 1),
        (PromisePrototypeThen, Builtin::TSJ, 2),
        (ProxyConstructor, Builtin::TSJ, 2),
        (ReflectApply, Builtin::TSJ, 3),
        (ReflectConstruct, Builtin::TSJ, 2),
        (RegExpConstructor, Builtin::TSJ, 2),
        (RegExpPrototypeExec, Builtin::TSJ, 2),
        (RegExpPrototypeTest, Builtin::TSJ, 1),
        (ReturnReceiver, Builtin::TSJ, 0),
        (StringConstructor, Builtin::TSJ, 1),
        (StringPrototypeReplaceAll, Builtin::TSJ, 2),
        (StringPrototypeReplaceRegExp, Builtin::TSJ, 2),
        (StoreIC, Builtin::TFH, Store),
        (StoreWithVectorIC, Builtin::TFH, StoreWithVector),
        (Sub, Builtin::TSJ, 2),
        (SymbolConstructor, Builtin::TSJ, 1),
        (ThrowInvalidStringLength, Builtin::TSJ, 0),
        (ToBoolean, Builtin::TSJ, 1),
        (ToInteger, Builtin::TSJ, 1),
        (ToInt32, Builtin::TSJ, 1),
        (ToName, Builtin::TSJ, 1),
        (ToObject, Builtin::TSJ, 1),
        (ToString, Builtin::TSJ, 1),
        (ToUint32, Builtin::TSJ, 1),
        (TryCatch, Builtin::TSJ, 3),
        (TypedArrayConstructor, Builtin::TSJ, 1),
        (TypedArrayPrototypeAt, Builtin::TSJ, 1),
        (TypedArrayPrototypeSet, Builtin::TSJ, 3),
        (WeakMapConstructor, Builtin::TSJ, 1),
        (WeakSetConstructor, Builtin::TSJ, 1),
        (AsyncFunctionAwaitResume, Builtin::TSJ, 1),
        (AsyncGeneratorAwaitReturn, Builtin::TSJ, 2),
        (AsyncGeneratorAwaitThrow, Builtin::TSJ, 2),
        (GetIterator, Builtin::TSJ, 1),
        (Next, Builtin::TSJ, 1),
        (ObjectPrototypeToString, Builtin::TSJ, 0),
        (OrdinaryHasInstance, Builtin::TSJ, 2),
        (CreateIterResultObject, Builtin::TSJ, 2),
        (AsyncGeneratorCloseReturn, Builtin::TSJ, 1),
        (AsyncGeneratorCloseThrow, Builtin::TSJ, 1),
        (AsyncGeneratorCloseDone, Builtin::TSJ, 1),
        (GetOwnProperty, Builtin::TSJ, 2),
        (SetFunctionName, Builtin::TSJ, 2),
        (AsyncGeneratorResumedCheck, Builtin::TSJ, 1),
        (AsyncGeneratorUnwrap, Builtin::TSJ, 1),
        (ToBigInt, Builtin::TSJ, 1),
        (BigIntConstructor, Builtin::TSJ, 1),
        (StringPrototypeCodePointAt, Builtin::TSJ, 2),
        (StringPrototypeNormalize, Builtin::TSJ, 1),
        (StringPrototypeToWellFormed, Builtin::TSJ, 0),
        (PromiseAll, Builtin::TSJ, 2),
        (PromiseAny, Builtin::TSJ, 2),
        (PromiseRace, Builtin::TSJ, 2),
        (PromiseResolve, Builtin::TSJ, 1),
        (PromiseReject, Builtin::TSJ, 1),
        (PromiseAllSettled, Builtin::TSJ, 1),
        (IsRegisteredFormat, Builtin::TSJ, 1),
        (CreateDateTimeFormat, Builtin::TSJ, 2),
        (CreateNumberFormat, Builtin::TSJ, 2),
        (CreatePluralRules, Builtin::TSJ, 2),
        (CreateRelativeTimeFormat, Builtin::TSJ, 2),
        (CreateListFormat, Builtin::TSJ, 2),
        (CreateDisplayNames, Builtin::TSJ, 3),
        (GetNumberFormatInternalField, Builtin::TSJ, 2),
        (NumberFormatFormatToParts, Builtin::TSJ, 2),
        (DateTimeFormatFormatToParts, Builtin::TSJ, 2),
        (RelativeTimeFormatFormatToParts, Builtin::TSJ, 2),
        (GetIteratorNext, Builtin::TSJ, 1),
        (SetHostError, Builtin::TSJ, 2),
        (TryCatchPromise, Builtin::TSJ, 3),
        (GetTemplateObject, Builtin::TSJ, 2),
        (IsKeyedLookupElement, Builtin::TSJ, 2),
        (StringPrototypeIsWellFormed, Builtin::TSJ, 0),
        (FindOrderedExport, Builtin::TSJ, 2)
    );
}