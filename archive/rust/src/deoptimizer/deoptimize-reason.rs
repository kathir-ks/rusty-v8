// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod deoptimize_reason {
    use std::fmt;
    use std::hash::{Hash, Hasher};

    macro_rules! deoptimize_reason_list {
        ($V:ident) => {
            $V(ArrayBufferWasDetached, "array buffer was detached");
            $V(ArrayLengthChanged, "the array length changed");
            $V(BigIntTooBig, "BigInt too big");
            $V(ConstTrackingLet, "const tracking let constness invalidated");
            $V(CouldNotGrowElements, "failed to grow elements store");
            $V(CowArrayElementsChanged, "copy-on-write array's elements changed");
            $V(DeoptimizeNow, "%_DeoptimizeNow");
            $V(DeprecatedMap, "deprecated map");
            $V(DivisionByZero, "division by zero");
            $V(Float16NotYetSupported, "float16 is not supported as machine operation");
            $V(GreaterThanMaxFastElementArray, "length is greater than the maximum for fast elements array");
            $V(Hole, "hole");
            $V(InstanceMigrationFailed, "instance migration failed");
            $V(InsufficientTypeFeedbackForArrayLiteral, "Insufficient type feedback for array literal");
            $V(InsufficientTypeFeedbackForBinaryOperation, "Insufficient type feedback for binary operation");
            $V(InsufficientTypeFeedbackForCall, "Insufficient type feedback for call");
            $V(InsufficientTypeFeedbackForCompareOperation, "Insufficient type feedback for compare operation");
            $V(InsufficientTypeFeedbackForConstruct, "Insufficient type feedback for construct");
            $V(InsufficientTypeFeedbackForForIn, "Insufficient type feedback for for-in");
            $V(InsufficientTypeFeedbackForGenericGlobalAccess, "Insufficient type feedback for generic global access");
            $V(InsufficientTypeFeedbackForGenericKeyedAccess, "Insufficient type feedback for generic keyed access");
            $V(InsufficientTypeFeedbackForGenericNamedAccess, "Insufficient type feedback for generic named access");
            $V(InsufficientTypeFeedbackForInstanceOf, "Insufficient type feedback for instanceof");
            $V(InsufficientTypeFeedbackForObjectLiteral, "Insufficient type feedback for object literal");
            $V(InsufficientTypeFeedbackForTypeOf, "Insufficient type feedback for typeof");
            $V(InsufficientTypeFeedbackForUnaryOperation, "Insufficient type feedback for unary operation");
            $V(KeyedAccessChanged, "unexpected name in keyed access");
            $V(LostPrecision, "lost precision");
            $V(LostPrecisionOrNaN, "lost precision or NaN");
            $V(MinusZero, "minus zero");
            $V(NaN, "NaN");
            $V(NoCache, "no cache");
            $V(NoInitialElement, "no initial element");
            $V(NotABigInt, "not a BigInt");
            $V(NotABigInt64, "not a BigInt64");
            $V(NotAHeapNumber, "not a heap number");
            $V(NotAJavaScriptObject, "not a JavaScript object");
            $V(NotAJavaScriptObjectOrNullOrUndefined, "not a JavaScript object, Null or Undefined");
            $V(NotANumber, "not a Number");
            $V(NotANumberOrBoolean, "not a Number or Boolean");
            $V(NotANumberOrOddball, "not a Number or Oddball");
            $V(NotASmi, "not a Smi");
            $V(NotAString, "not a String");
            $V(NotAStringOrStringWrapper, "not a String or a string wrapper");
            $V(NotAStringWrapper, "not a string wrapper");
            $V(NotASymbol, "not a Symbol");
            $V(NotAdditiveSafeInteger, "not AdditiveSafeInteger");
            $V(NotAnArrayIndex, "not an array index");
            $V(NotDetectableReceiver, "not a detectable receiver");
            $V(NotInt32, "not int32");
            $V(NotUint32, "not unsigned int32");
            $V(OSREarlyExit, "exit from OSR'd inner loop");
            $V(OutOfBounds, "out of bounds");
            $V(Overflow, "overflow");
            $V(PrepareForOnStackReplacement, "prepare for on stack replacement (OSR)");
            $V(Smi, "Smi");
            $V(StoreToConstant, "Storing to a constant field");
            $V(StringTooLarge, "Result string larger than String::kMaxLength");
            $V(SuspendGeneratorIsDead, "SuspendGenerator is in a dead branch");
            $V(UnexpectedContextExtension, "unexpected context extension");
            $V(Unknown, "(unknown)");
            $V(UnoptimizedCatch, "First use of catch block");
            $V(ValueMismatch, "value mismatch");
            $V(WrongCallTarget, "wrong call target");
            $V(WrongConstructor, "wrong call target constructor");
            $V(WrongEnumIndices, "wrong enum indices");
            $V(WrongFeedbackCell, "wrong feedback cell");
            $V(WrongInstanceType, "wrong instance type");
            $V(WrongMap, "wrong map");
            $V(WrongMapDynamic, "map changed during operation");
            $V(WrongName, "wrong name");
            $V(WrongValue, "wrong value");
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum DeoptimizeReason {
        #[allow(non_camel_case_types)]
        Unknown, // Added default value

        #[allow(non_camel_case_types)]
        ArrayBufferWasDetached,
        #[allow(non_camel_case_types)]
        ArrayLengthChanged,
        #[allow(non_camel_case_types)]
        BigIntTooBig,
        #[allow(non_camel_case_types)]
        ConstTrackingLet,
        #[allow(non_camel_case_types)]
        CouldNotGrowElements,
        #[allow(non_camel_case_types)]
        CowArrayElementsChanged,
        #[allow(non_camel_case_types)]
        DeoptimizeNow,
        #[allow(non_camel_case_types)]
        DeprecatedMap,
        #[allow(non_camel_case_types)]
        DivisionByZero,
        #[allow(non_camel_case_types)]
        Float16NotYetSupported,
        #[allow(non_camel_case_types)]
        GreaterThanMaxFastElementArray,
        #[allow(non_camel_case_types)]
        Hole,
        #[allow(non_camel_case_types)]
        InstanceMigrationFailed,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForArrayLiteral,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForBinaryOperation,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForCall,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForCompareOperation,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForConstruct,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForForIn,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForGenericGlobalAccess,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForGenericKeyedAccess,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForGenericNamedAccess,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForInstanceOf,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForObjectLiteral,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForTypeOf,
        #[allow(non_camel_case_types)]
        InsufficientTypeFeedbackForUnaryOperation,
        #[allow(non_camel_case_types)]
        KeyedAccessChanged,
        #[allow(non_camel_case_types)]
        LostPrecision,
        #[allow(non_camel_case_types)]
        LostPrecisionOrNaN,
        #[allow(non_camel_case_types)]
        MinusZero,
        #[allow(non_camel_case_types)]
        NaN,
        #[allow(non_camel_case_types)]
        NoCache,
        #[allow(non_camel_case_types)]
        NoInitialElement,
        #[allow(non_camel_case_types)]
        NotABigInt,
        #[allow(non_camel_case_types)]
        NotABigInt64,
        #[allow(non_camel_case_types)]
        NotAHeapNumber,
        #[allow(non_camel_case_types)]
        NotAJavaScriptObject,
        #[allow(non_camel_case_types)]
        NotAJavaScriptObjectOrNullOrUndefined,
        #[allow(non_camel_case_types)]
        NotANumber,
        #[allow(non_camel_case_types)]
        NotANumberOrBoolean,
        #[allow(non_camel_case_types)]
        NotANumberOrOddball,
        #[allow(non_camel_case_types)]
        NotASmi,
        #[allow(non_camel_case_types)]
        NotAString,
        #[allow(non_camel_case_types)]
        NotAStringOrStringWrapper,
        #[allow(non_camel_case_types)]
        NotAStringWrapper,
        #[allow(non_camel_case_types)]
        NotASymbol,
        #[allow(non_camel_case_types)]
        NotAdditiveSafeInteger,
        #[allow(non_camel_case_types)]
        NotAnArrayIndex,
        #[allow(non_camel_case_types)]
        NotDetectableReceiver,
        #[allow(non_camel_case_types)]
        NotInt32,
        #[allow(non_camel_case_types)]
        NotUint32,
        #[allow(non_camel_case_types)]
        OSREarlyExit,
        #[allow(non_camel_case_types)]
        OutOfBounds,
        #[allow(non_camel_case_types)]
        Overflow,
        #[allow(non_camel_case_types)]
        PrepareForOnStackReplacement,
        #[allow(non_camel_case_types)]
        Smi,
        #[allow(non_camel_case_types)]
        StoreToConstant,
        #[allow(non_camel_case_types)]
        StringTooLarge,
        #[allow(non_camel_case_types)]
        SuspendGeneratorIsDead,
        #[allow(non_camel_case_types)]
        UnexpectedContextExtension,
        #[allow(non_camel_case_types)]
        UnoptimizedCatch,
        #[allow(non_camel_case_types)]
        ValueMismatch,
        #[allow(non_camel_case_types)]
        WrongCallTarget,
        #[allow(non_camel_case_types)]
        WrongConstructor,
        #[allow(non_camel_case_types)]
        WrongEnumIndices,
        #[allow(non_camel_case_types)]
        WrongFeedbackCell,
        #[allow(non_camel_case_types)]
        WrongInstanceType,
        #[allow(non_camel_case_types)]
        WrongMap,
        #[allow(non_camel_case_types)]
        WrongMapDynamic,
        #[allow(non_camel_case_types)]
        WrongName,
        #[allow(non_camel_case_types)]
        WrongValue,
    }

    impl Default for DeoptimizeReason {
        fn default() -> Self {
            DeoptimizeReason::Unknown
        }
    }

    impl DeoptimizeReason {
        pub fn as_str(&self) -> &'static str {
            match self {
                DeoptimizeReason::ArrayBufferWasDetached => "array buffer was detached",
                DeoptimizeReason::ArrayLengthChanged => "the array length changed",
                DeoptimizeReason::BigIntTooBig => "BigInt too big",
                DeoptimizeReason::ConstTrackingLet => "const tracking let constness invalidated",
                DeoptimizeReason::CouldNotGrowElements => "failed to grow elements store",
                DeoptimizeReason::CowArrayElementsChanged => "copy-on-write array's elements changed",
                DeoptimizeReason::DeoptimizeNow => "%_DeoptimizeNow",
                DeoptimizeReason::DeprecatedMap => "deprecated map",
                DeoptimizeReason::DivisionByZero => "division by zero",
                DeoptimizeReason::Float16NotYetSupported => "float16 is not supported as machine operation",
                DeoptimizeReason::GreaterThanMaxFastElementArray => "length is greater than the maximum for fast elements array",
                DeoptimizeReason::Hole => "hole",
                DeoptimizeReason::InstanceMigrationFailed => "instance migration failed",
                DeoptimizeReason::InsufficientTypeFeedbackForArrayLiteral => "Insufficient type feedback for array literal",
                DeoptimizeReason::InsufficientTypeFeedbackForBinaryOperation => "Insufficient type feedback for binary operation",
                DeoptimizeReason::InsufficientTypeFeedbackForCall => "Insufficient type feedback for call",
                DeoptimizeReason::InsufficientTypeFeedbackForCompareOperation => "Insufficient type feedback for compare operation",
                DeoptimizeReason::InsufficientTypeFeedbackForConstruct => "Insufficient type feedback for construct",
                DeoptimizeReason::InsufficientTypeFeedbackForForIn => "Insufficient type feedback for for-in",
                DeoptimizeReason::InsufficientTypeFeedbackForGenericGlobalAccess => "Insufficient type feedback for generic global access",
                DeoptimizeReason::InsufficientTypeFeedbackForGenericKeyedAccess => "Insufficient type feedback for generic keyed access",
                DeoptimizeReason::InsufficientTypeFeedbackForGenericNamedAccess => "Insufficient type feedback for generic named access",
                DeoptimizeReason::InsufficientTypeFeedbackForInstanceOf => "Insufficient type feedback for instanceof",
                DeoptimizeReason::InsufficientTypeFeedbackForObjectLiteral => "Insufficient type feedback for object literal",
                DeoptimizeReason::InsufficientTypeFeedbackForTypeOf => "Insufficient type feedback for typeof",
                DeoptimizeReason::InsufficientTypeFeedbackForUnaryOperation => "Insufficient type feedback for unary operation",
                DeoptimizeReason::KeyedAccessChanged => "unexpected name in keyed access",
                DeoptimizeReason::LostPrecision => "lost precision",
                DeoptimizeReason::LostPrecisionOrNaN => "lost precision or NaN",
                DeoptimizeReason::MinusZero => "minus zero",
                DeoptimizeReason::NaN => "NaN",
                DeoptimizeReason::NoCache => "no cache",
                DeoptimizeReason::NoInitialElement => "no initial element",
                DeoptimizeReason::NotABigInt => "not a BigInt",
                DeoptimizeReason::NotABigInt64 => "not a BigInt64",
                DeoptimizeReason::NotAHeapNumber => "not a heap number",
                DeoptimizeReason::NotAJavaScriptObject => "not a JavaScript object",
                DeoptimizeReason::NotAJavaScriptObjectOrNullOrUndefined => "not a JavaScript object, Null or Undefined",
                DeoptimizeReason::NotANumber => "not a Number",
                DeoptimizeReason::NotANumberOrBoolean => "not a Number or Boolean",
                DeoptimizeReason::NotANumberOrOddball => "not a Number or Oddball",
                DeoptimizeReason::NotASmi => "not a Smi",
                DeoptimizeReason::NotAString => "not a String",
                DeoptimizeReason::NotAStringOrStringWrapper => "not a String or a string wrapper",
                DeoptimizeReason::NotAStringWrapper => "not a string wrapper",
                DeoptimizeReason::NotASymbol => "not a Symbol",
                DeoptimizeReason::NotAdditiveSafeInteger => "not AdditiveSafeInteger",
                DeoptimizeReason::NotAnArrayIndex => "not an array index",
                DeoptimizeReason::NotDetectableReceiver => "not a detectable receiver",
                DeoptimizeReason::NotInt32 => "not int32",
                DeoptimizeReason::NotUint32 => "not unsigned int32",
                DeoptimizeReason::OSREarlyExit => "exit from OSR'd inner loop",
                DeoptimizeReason::OutOfBounds => "out of bounds",
                DeoptimizeReason::Overflow => "overflow",
                DeoptimizeReason::PrepareForOnStackReplacement => "prepare for on stack replacement (OSR)",
                DeoptimizeReason::Smi => "Smi",
                DeoptimizeReason::StoreToConstant => "Storing to a constant field",
                DeoptimizeReason::StringTooLarge => "Result string larger than String::kMaxLength",
                DeoptimizeReason::SuspendGeneratorIsDead => "SuspendGenerator is in a dead branch",
                DeoptimizeReason::UnexpectedContextExtension => "unexpected context extension",
                DeoptimizeReason::Unknown => "(unknown)",
                DeoptimizeReason::UnoptimizedCatch => "First use of catch block",
                DeoptimizeReason::ValueMismatch => "value mismatch",
                DeoptimizeReason::WrongCallTarget => "wrong call target",
                DeoptimizeReason::WrongConstructor => "wrong call target constructor",
                DeoptimizeReason::WrongEnumIndices => "wrong enum indices",
                DeoptimizeReason::WrongFeedbackCell => "wrong feedback cell",
                DeoptimizeReason::WrongInstanceType => "wrong instance type",
                DeoptimizeReason::WrongMap => "wrong map",
                DeoptimizeReason::WrongMapDynamic => "map changed during operation",
                DeoptimizeReason::WrongName => "wrong name",
                DeoptimizeReason::WrongValue => "wrong value",
            }
        }
    }

    impl fmt::Display for DeoptimizeReason {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }

    impl Hash for DeoptimizeReason {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (*self as u8).hash(state);
        }
    }

    pub const K_FIRST_DEOPTIMIZE_REASON: DeoptimizeReason = DeoptimizeReason::Unknown; // Properly set to 0
    pub const K_DEOPTIMIZE_REASON_COUNT: usize = {
        #[allow(dead_code)]
        const fn count_reasons() -> usize {
            #[derive(PartialEq)]
            enum Dummy {
                ArrayBufferWasDetached,
                ArrayLengthChanged,
                BigIntTooBig,
                ConstTrackingLet,
                CouldNotGrowElements,
                CowArrayElementsChanged,
                DeoptimizeNow,
                DeprecatedMap,
                DivisionByZero,
                Float16NotYetSupported,
                GreaterThanMaxFastElementArray,
                Hole,
                InstanceMigrationFailed,
                InsufficientTypeFeedbackForArrayLiteral,
                InsufficientTypeFeedbackForBinaryOperation,
                InsufficientTypeFeedbackForCall,
                InsufficientTypeFeedbackForCompareOperation,
                InsufficientTypeFeedbackForConstruct,
                InsufficientTypeFeedbackForForIn,
                InsufficientTypeFeedbackForGenericGlobalAccess,
                InsufficientTypeFeedbackForGenericKeyedAccess,
                InsufficientTypeFeedbackForGenericNamedAccess,
                InsufficientTypeFeedbackForInstanceOf,
                InsufficientTypeFeedbackForObjectLiteral,
                InsufficientTypeFeedbackForTypeOf,
                InsufficientTypeFeedbackForUnaryOperation,
                KeyedAccessChanged,
                LostPrecision,
                LostPrecisionOrNaN,
                MinusZero,
                NaN,
                NoCache,
                NoInitialElement,
                NotABigInt,
                NotABigInt64,
                NotAHeapNumber,
                NotAJavaScriptObject,
                NotAJavaScriptObjectOrNullOrUndefined,
                NotANumber,
                NotANumberOrBoolean,
                NotANumberOrOddball,
                NotASmi,
                NotAString,
                NotAStringOrStringWrapper,
                NotAStringWrapper,
                NotASymbol,
                NotAdditiveSafeInteger,
                NotAnArrayIndex,
                NotDetectableReceiver,
                NotInt32,
                NotUint32,
                OSREarlyExit,
                OutOfBounds,
                Overflow,
                PrepareForOnStackReplacement,
                Smi,
                StoreToConstant,
                StringTooLarge,
                SuspendGeneratorIsDead,
                UnexpectedContextExtension,
                Unknown,
                UnoptimizedCatch,
                ValueMismatch,
                WrongCallTarget,
                WrongConstructor,
                WrongEnumIndices,
                WrongFeedbackCell,
                WrongInstanceType,
                WrongMap,
                WrongMapDynamic,
                WrongName,
                WrongValue
            }

            let mut count = 0;
            if Dummy::ArrayBufferWasDetached != Dummy::ArrayLengthChanged { count += 1; }
            if Dummy::ArrayLengthChanged != Dummy::BigIntTooBig { count += 1; }
            if Dummy::BigIntTooBig != Dummy::ConstTrackingLet { count += 1; }
            if Dummy::ConstTrackingLet != Dummy::CouldNotGrowElements { count += 1; }
            if Dummy::CouldNotGrowElements != Dummy::CowArrayElementsChanged { count += 1; }
            if Dummy::CowArrayElementsChanged != Dummy::DeoptimizeNow { count += 1; }
            if Dummy::DeoptimizeNow != Dummy::DeprecatedMap { count += 1; }
            if Dummy::DeprecatedMap != Dummy::DivisionByZero { count += 1; }
            if Dummy::DivisionByZero != Dummy::Float16NotYetSupported { count += 1; }
            if Dummy::Float16NotYetSupported != Dummy::GreaterThanMaxFastElementArray { count += 1; }
            if Dummy::GreaterThanMaxFastElementArray != Dummy::Hole { count += 1; }
            if Dummy::Hole != Dummy::InstanceMigrationFailed { count += 1; }
            if Dummy::InstanceMigrationFailed != Dummy::InsufficientTypeFeedbackForArrayLiteral { count += 1; }
            if Dummy::InsufficientTypeFeedbackForArrayLiteral != Dummy::InsufficientTypeFeedbackForBinaryOperation { count += 1; }
            if Dummy::InsufficientTypeFeedbackForBinaryOperation != Dummy::InsufficientTypeFeedbackForCall { count += 1; }
            if Dummy::InsufficientTypeFeedbackForCall != Dummy::InsufficientTypeFeedbackForCompareOperation { count += 1; }
            if Dummy::InsufficientTypeFeedbackForCompareOperation != Dummy::InsufficientTypeFeedbackForConstruct { count += 1; }
            if Dummy::InsufficientTypeFeedbackForConstruct != Dummy::InsufficientTypeFeedbackForForIn { count += 1; }
            if Dummy::InsufficientTypeFeedbackForForIn != Dummy::InsufficientTypeFeedbackForGenericGlobalAccess { count += 1; }
            if Dummy::InsufficientTypeFeedbackForGenericGlobalAccess != Dummy::InsufficientTypeFeedbackForGenericKeyedAccess { count += 1; }
            if Dummy::InsufficientTypeFeedbackForGenericKeyedAccess != Dummy::InsufficientTypeFeedbackForGenericNamedAccess { count += 1; }
            if Dummy::InsufficientTypeFeedbackForGenericNamedAccess != Dummy::InsufficientTypeFeedbackForInstanceOf { count += 1; }
            if Dummy::InsufficientTypeFeedbackForInstanceOf != Dummy::InsufficientTypeFeedbackForObjectLiteral { count += 1; }
            if Dummy::InsufficientTypeFeedbackForObjectLiteral != Dummy::InsufficientTypeFeedbackForTypeOf { count += 1; }
            if Dummy::InsufficientTypeFeedbackForTypeOf != Dummy::InsufficientTypeFeedbackForUnaryOperation { count += 1; }
            if Dummy::InsufficientTypeFeedbackForUnaryOperation != Dummy::KeyedAccessChanged { count += 1; }
            if Dummy::KeyedAccessChanged != Dummy::LostPrecision { count += 1; }
            if Dummy::LostPrecision != Dummy::LostPrecisionOrNaN { count += 1; }
            if Dummy::LostPrecisionOrNaN != Dummy::MinusZero { count += 1; }
            if Dummy::MinusZero != Dummy::NaN { count += 1; }
            if Dummy::NaN != Dummy::NoCache { count += 1; }
            if Dummy::NoCache != Dummy::NoInitialElement { count += 1; }
            if Dummy::NoInitialElement != Dummy::NotABigInt { count += 1; }
            if Dummy::NotABigInt != Dummy::NotABigInt64 { count += 1; }
            if Dummy::NotABigInt64 != Dummy::NotAHeapNumber { count += 1; }
            if Dummy::NotAHeapNumber != Dummy::NotAJavaScriptObject { count += 1; }
            if Dummy::NotAJavaScriptObject != Dummy::NotAJavaScriptObjectOrNullOrUndefined { count += 1; }
            if Dummy::NotAJavaScriptObjectOrNullOrUndefined != Dummy::NotANumber { count += 1; }
            if Dummy::NotANumber != Dummy::NotANumberOrBoolean { count += 1; }
            if Dummy::NotANumberOrBoolean != Dummy::NotANumberOrOddball { count += 1; }
            if Dummy::NotANumberOrOddball != Dummy::NotASmi { count += 1; }
            if Dummy::NotASmi != Dummy::NotAString { count += 1; }
            if Dummy::NotAString != Dummy::NotAStringOrStringWrapper { count += 1; }
            if Dummy::NotAStringOrStringWrapper != Dummy::NotAStringWrapper { count += 1; }
            if Dummy::NotAStringWrapper != Dummy::NotASymbol { count += 1; }
            if Dummy::NotASymbol != Dummy::NotAdditiveSafeInteger { count += 1; }
            if Dummy::NotAdditiveSafeInteger != Dummy::NotAnArrayIndex { count += 1; }
            if Dummy::NotAnArrayIndex != Dummy::NotDetectableReceiver { count += 1; }
            if Dummy::NotDetectableReceiver != Dummy::NotInt32 { count += 1; }
            if Dummy::NotInt32 != Dummy::NotUint32 { count += 1; }
            if Dummy::NotUint32 != Dummy::OSREarlyExit { count += 1; }
            if Dummy::OSREarlyExit != Dummy::OutOfBounds { count += 1; }
            if Dummy::OutOfBounds != Dummy::Overflow { count += 1; }
            if Dummy::Overflow != Dummy::PrepareForOnStackReplacement { count += 1; }
            if Dummy::PrepareForOnStackReplacement != Dummy::Smi { count += 1; }
            if Dummy::Smi != Dummy::StoreToConstant { count += 1; }
            if Dummy::StoreToConstant != Dummy::StringTooLarge { count += 1; }
            if Dummy::StringTooLarge != Dummy::SuspendGeneratorIsDead { count += 1; }
            if Dummy::SuspendGeneratorIsDead != Dummy::UnexpectedContextExtension { count += 1; }
            if Dummy::UnexpectedContextExtension != Dummy::Unknown { count += 1; }
            if Dummy::Unknown != Dummy::UnoptimizedCatch { count += 1; }
            if Dummy::UnoptimizedCatch != Dummy::ValueMismatch { count += 1; }
            if Dummy::ValueMismatch != Dummy::WrongCallTarget { count += 1; }
            if Dummy::WrongCallTarget != Dummy::WrongConstructor { count += 1; }
            if Dummy::WrongConstructor != Dummy::WrongEnumIndices { count += 1; }
            if Dummy::WrongEnumIndices != Dummy::WrongFeedbackCell { count += 1; }
            if Dummy::WrongFeedbackCell != Dummy::WrongInstanceType { count += 1; }
            if Dummy::WrongInstanceType != Dummy::WrongMap { count += 1; }
            if Dummy::WrongMap != Dummy::WrongMapDynamic { count += 1; }
            if Dummy::WrongMapDynamic != Dummy::WrongName { count += 1; }
            if Dummy::WrongName != Dummy::WrongValue { count += 1; }

            count + 1
        }

        count_reasons()
    };
    pub const K_LAST_DEOPTIMIZE_REASON: DeoptimizeReason =
        unsafe { std::mem::transmute((K_DEOPTIMIZE_REASON_COUNT - 1) as u8) };

    macro_rules! lazy_deoptimize_reason_list {
        ($V:ident) => {
            $V(MapDeprecated, "dependent map was deprecated");
            $V(PrototypeChange, "dependent prototype chain changed");
            $V(PropertyCellChange, "dependent property cell changed");
            $V(FieldTypeConstChange, "dependent field type constness changed");
            $V(FieldTypeChange, "dependent field type changed");
            $V(FieldRepresentationChange, "dependent field representation changed");
            $V(InitialMapChange, "dependent initial map changed");
            $V(AllocationSiteTenuringChange, "dependent allocation site tenuring changed");
            $V(AllocationSiteTransitionChange, "dependent allocation site transition changed");
            $V(ScriptContextSlotPropertyChange, "dependent script context slot property changed");
            $V(EmptyContextExtensionChange, "dependent empty context extension changed");
            $V(WeakObjects, "embedded weak objects cleared");
            $V(Debugger, "JS debugger attached");
            $V(Testing, "for testing");
            $V(ExceptionCaught, "exception with omitted catch handler");
            $V(EagerDeopt, "marked due to eager deopt");
            $V(FrameValueMaterialized, "value in stack frame was materialized");
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum LazyDeoptimizeReason {
        #[allow(non_camel_case_types)]
        MapDeprecated,
        #[allow(non_camel_case_types)]
        PrototypeChange,
        #[allow(non_camel_case_types)]
        PropertyCellChange,
        #[allow(non_camel_case_types)]
        FieldTypeConstChange,
        #[allow(non_camel_case_types)]
        FieldTypeChange,
        #[allow(non_camel_case_types)]
        FieldRepresentationChange,
        #[allow(non_camel_case_types)]
        InitialMapChange,
        #[allow(non_camel_case_types)]
        AllocationSiteTenuringChange,
        #[allow(non_camel_case_types)]
        AllocationSiteTransitionChange,
        #[allow(non_camel_case_types)]
        ScriptContextSlotPropertyChange,
        #[allow(non_camel_case_types)]
        EmptyContextExtensionChange,
        #[allow(non_camel_case_types)]
        WeakObjects,
        #[allow(non_camel_case_types)]
        Debugger,
        #[allow(non_camel_case_types)]
        Testing,
        #[allow(non_camel_case_types)]
        ExceptionCaught,
        #[allow(non_camel_case_types)]
        EagerDeopt,
        #[allow(non_camel_case_types)]
        FrameValueMaterialized,
    }

    impl LazyDeoptimizeReason {
        pub fn as_str(&self) -> &'static str {
            match self {
                LazyDeoptimizeReason::MapDeprecated => "dependent map was deprecated",
                LazyDeoptimizeReason::PrototypeChange => "dependent prototype chain changed",
                LazyDeoptimizeReason::PropertyCellChange => "dependent property cell changed",
                LazyDeoptimizeReason::FieldTypeConstChange => "dependent field type constness changed",
                LazyDeoptimizeReason::FieldTypeChange => "dependent field type changed",
                LazyDeoptimizeReason::FieldRepresentationChange => "dependent field representation changed",
                LazyDeoptimizeReason::InitialMapChange => "dependent initial map changed",
                LazyDeoptimizeReason::AllocationSiteTenuringChange => "dependent allocation site tenuring changed",
                LazyDeoptimizeReason::AllocationSiteTransitionChange => "dependent allocation site transition changed",
                LazyDeoptimizeReason::ScriptContextSlotPropertyChange => "dependent script context slot property changed",
                LazyDeoptimizeReason::EmptyContextExtensionChange => "dependent empty context extension changed",
                LazyDeoptimizeReason::WeakObjects => "embedded weak objects cleared",
                LazyDeoptimizeReason::Debugger => "JS debugger attached",
                LazyDeoptimizeReason::Testing => "for testing",
                LazyDeoptimizeReason::ExceptionCaught => "exception with omitted catch handler",
                LazyDeoptimizeReason::EagerDeopt => "marked due to eager deopt",
                LazyDeoptimizeReason::FrameValueMaterialized => "value in stack frame was materialized",
            }
        }
    }

    impl fmt::Display for LazyDeoptimizeReason {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }

    impl Hash for LazyDeoptimizeReason {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (*self as u8).hash(state);
        }
    }

    pub fn deoptimize_reason_to_string(reason: DeoptimizeReason) -> &'static str {
        reason.as_str()
    }

    pub fn deoptimize_reason_to_string_lazy(reason: LazyDeoptimizeReason) -> &'static str {
        reason.as_str()
    }

    pub const fn is_deoptimization_without_code_invalidation(reason: DeoptimizeReason) -> bool {
        reason == DeoptimizeReason::PrepareForOnStackReplacement || reason == DeoptimizeReason::OSREarlyExit
    }
}