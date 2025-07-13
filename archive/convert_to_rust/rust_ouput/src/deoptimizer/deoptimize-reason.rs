// Converted from V8 C++ source files:
// Header: deoptimize-reason.h
// Implementation: deoptimize-reason.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod deoptimize_reason {
    use std::fmt;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum DeoptimizeReason {
        kArrayBufferWasDetached,
        kArrayLengthChanged,
        kBigIntTooBig,
        kConstTrackingLet,
        kCouldNotGrowElements,
        kCowArrayElementsChanged,
        kDeoptimizeNow,
        kDeprecatedMap,
        kDivisionByZero,
        kFloat16NotYetSupported,
        kGreaterThanMaxFastElementArray,
        kHole,
        kInstanceMigrationFailed,
        kInsufficientTypeFeedbackForArrayLiteral,
        kInsufficientTypeFeedbackForBinaryOperation,
        kInsufficientTypeFeedbackForCall,
        kInsufficientTypeFeedbackForCompareOperation,
        kInsufficientTypeFeedbackForConstruct,
        kInsufficientTypeFeedbackForForIn,
        kInsufficientTypeFeedbackForGenericGlobalAccess,
        kInsufficientTypeFeedbackForGenericKeyedAccess,
        kInsufficientTypeFeedbackForGenericNamedAccess,
        kInsufficientTypeFeedbackForInstanceOf,
        kInsufficientTypeFeedbackForObjectLiteral,
        kInsufficientTypeFeedbackForTypeOf,
        kInsufficientTypeFeedbackForUnaryOperation,
        kKeyedAccessChanged,
        kLostPrecision,
        kLostPrecisionOrNaN,
        kMinusZero,
        kNaN,
        kNoCache,
        kNoInitialElement,
        kNotABigInt,
        kNotABigInt64,
        kNotAHeapNumber,
        kNotAJavaScriptObject,
        kNotAJavaScriptObjectOrNullOrUndefined,
        kNotANumber,
        kNotANumberOrBoolean,
        kNotANumberOrOddball,
        kNotASmi,
        kNotAString,
        kNotAStringOrStringWrapper,
        kNotAStringWrapper,
        kNotASymbol,
        kNotAdditiveSafeInteger,
        kNotAnArrayIndex,
        kNotDetectableReceiver,
        kNotInt32,
        kNotUint32,
        kOSREarlyExit,
        kOutOfBounds,
        kOverflow,
        kPrepareForOnStackReplacement,
        kSmi,
        kStoreToConstant,
        kStringTooLarge,
        kSuspendGeneratorIsDead,
        kUnexpectedContextExtension,
        kUnknown,
        kUnoptimizedCatch,
        kValueMismatch,
        kWrongCallTarget,
        kWrongConstructor,
        kWrongEnumIndices,
        kWrongFeedbackCell,
        kWrongInstanceType,
        kWrongMap,
        kWrongMapDynamic,
        kWrongName,
        kWrongValue,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum LazyDeoptimizeReason {
        kMapDeprecated,
        kPrototypeChange,
        kPropertyCellChange,
        kFieldTypeConstChange,
        kFieldTypeChange,
        kFieldRepresentationChange,
        kInitialMapChange,
        kAllocationSiteTenuringChange,
        kAllocationSiteTransitionChange,
        kScriptContextSlotPropertyChange,
        kEmptyContextExtensionChange,
        kWeakObjects,
        kDebugger,
        kTesting,
        kExceptionCaught,
        kEagerDeopt,
        kFrameValueMaterialized,
    }

    impl fmt::Display for DeoptimizeReason {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
                DeoptimizeReason::kArrayBufferWasDetached => "ArrayBufferWasDetached",
                DeoptimizeReason::kArrayLengthChanged => "ArrayLengthChanged",
                DeoptimizeReason::kBigIntTooBig => "BigIntTooBig",
                DeoptimizeReason::kConstTrackingLet => "ConstTrackingLet",
                DeoptimizeReason::kCouldNotGrowElements => "CouldNotGrowElements",
                DeoptimizeReason::kCowArrayElementsChanged => "CowArrayElementsChanged",
                DeoptimizeReason::kDeoptimizeNow => "DeoptimizeNow",
                DeoptimizeReason::kDeprecatedMap => "DeprecatedMap",
                DeoptimizeReason::kDivisionByZero => "DivisionByZero",
                DeoptimizeReason::kFloat16NotYetSupported => "Float16NotYetSupported",
                DeoptimizeReason::kGreaterThanMaxFastElementArray => "GreaterThanMaxFastElementArray",
                DeoptimizeReason::kHole => "Hole",
                DeoptimizeReason::kInstanceMigrationFailed => "InstanceMigrationFailed",
                DeoptimizeReason::kInsufficientTypeFeedbackForArrayLiteral => "InsufficientTypeFeedbackForArrayLiteral",
                DeoptimizeReason::kInsufficientTypeFeedbackForBinaryOperation => "InsufficientTypeFeedbackForBinaryOperation",
                DeoptimizeReason::kInsufficientTypeFeedbackForCall => "InsufficientTypeFeedbackForCall",
                DeoptimizeReason::kInsufficientTypeFeedbackForCompareOperation => "InsufficientTypeFeedbackForCompareOperation",
                DeoptimizeReason::kInsufficientTypeFeedbackForConstruct => "InsufficientTypeFeedbackForConstruct",
                DeoptimizeReason::kInsufficientTypeFeedbackForForIn => "InsufficientTypeFeedbackForForIn",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericGlobalAccess => "InsufficientTypeFeedbackForGenericGlobalAccess",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericKeyedAccess => "InsufficientTypeFeedbackForGenericKeyedAccess",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericNamedAccess => "InsufficientTypeFeedbackForGenericNamedAccess",
                DeoptimizeReason::kInsufficientTypeFeedbackForInstanceOf => "InsufficientTypeFeedbackForInstanceOf",
                DeoptimizeReason::kInsufficientTypeFeedbackForObjectLiteral => "InsufficientTypeFeedbackForObjectLiteral",
                DeoptimizeReason::kInsufficientTypeFeedbackForTypeOf => "InsufficientTypeFeedbackForTypeOf",
                DeoptimizeReason::kInsufficientTypeFeedbackForUnaryOperation => "InsufficientTypeFeedbackForUnaryOperation",
                DeoptimizeReason::kKeyedAccessChanged => "KeyedAccessChanged",
                DeoptimizeReason::kLostPrecision => "LostPrecision",
                DeoptimizeReason::kLostPrecisionOrNaN => "LostPrecisionOrNaN",
                DeoptimizeReason::kMinusZero => "MinusZero",
                DeoptimizeReason::kNaN => "NaN",
                DeoptimizeReason::kNoCache => "NoCache",
                DeoptimizeReason::kNoInitialElement => "NoInitialElement",
                DeoptimizeReason::kNotABigInt => "NotABigInt",
                DeoptimizeReason::kNotABigInt64 => "NotABigInt64",
                DeoptimizeReason::kNotAHeapNumber => "NotAHeapNumber",
                DeoptimizeReason::kNotAJavaScriptObject => "NotAJavaScriptObject",
                DeoptimizeReason::kNotAJavaScriptObjectOrNullOrUndefined => "NotAJavaScriptObjectOrNullOrUndefined",
                DeoptimizeReason::kNotANumber => "NotANumber",
                DeoptimizeReason::kNotANumberOrBoolean => "NotANumberOrBoolean",
                DeoptimizeReason::kNotANumberOrOddball => "NotANumberOrOddball",
                DeoptimizeReason::kNotASmi => "NotASmi",
                DeoptimizeReason::kNotAString => "NotAString",
                DeoptimizeReason::kNotAStringOrStringWrapper => "NotAStringOrStringWrapper",
                DeoptimizeReason::kNotAStringWrapper => "NotAStringWrapper",
                DeoptimizeReason::kNotASymbol => "NotASymbol",
                DeoptimizeReason::kNotAdditiveSafeInteger => "NotAdditiveSafeInteger",
                DeoptimizeReason::kNotAnArrayIndex => "NotAnArrayIndex",
                DeoptimizeReason::kNotDetectableReceiver => "NotDetectableReceiver",
                DeoptimizeReason::kNotInt32 => "NotInt32",
                DeoptimizeReason::kNotUint32 => "NotUint32",
                DeoptimizeReason::kOSREarlyExit => "OSREarlyExit",
                DeoptimizeReason::kOutOfBounds => "OutOfBounds",
                DeoptimizeReason::kOverflow => "Overflow",
                DeoptimizeReason::kPrepareForOnStackReplacement => "PrepareForOnStackReplacement",
                DeoptimizeReason::kSmi => "Smi",
                DeoptimizeReason::kStoreToConstant => "StoreToConstant",
                DeoptimizeReason::kStringTooLarge => "StringTooLarge",
                DeoptimizeReason::kSuspendGeneratorIsDead => "SuspendGeneratorIsDead",
                DeoptimizeReason::kUnexpectedContextExtension => "UnexpectedContextExtension",
                DeoptimizeReason::kUnknown => "Unknown",
                DeoptimizeReason::kUnoptimizedCatch => "UnoptimizedCatch",
                DeoptimizeReason::kValueMismatch => "ValueMismatch",
                DeoptimizeReason::kWrongCallTarget => "WrongCallTarget",
                DeoptimizeReason::kWrongConstructor => "WrongConstructor",
                DeoptimizeReason::kWrongEnumIndices => "WrongEnumIndices",
                DeoptimizeReason::kWrongFeedbackCell => "WrongFeedbackCell",
                DeoptimizeReason::kWrongInstanceType => "WrongInstanceType",
                DeoptimizeReason::kWrongMap => "WrongMap",
                DeoptimizeReason::kWrongMapDynamic => "WrongMapDynamic",
                DeoptimizeReason::kWrongName => "WrongName",
                DeoptimizeReason::kWrongValue => "WrongValue",
            };
            write!(f, "{}", name)
        }
    }

    impl DeoptimizeReason {
        pub fn to_string(&self) -> &'static str {
            match self {
                DeoptimizeReason::kArrayBufferWasDetached => "array buffer was detached",
                DeoptimizeReason::kArrayLengthChanged => "the array length changed",
                DeoptimizeReason::kBigIntTooBig => "BigInt too big",
                DeoptimizeReason::kConstTrackingLet => "const tracking let constness invalidated",
                DeoptimizeReason::kCouldNotGrowElements => "failed to grow elements store",
                DeoptimizeReason::kCowArrayElementsChanged => "copy-on-write array's elements changed",
                DeoptimizeReason::kDeoptimizeNow => "%_DeoptimizeNow",
                DeoptimizeReason::kDeprecatedMap => "deprecated map",
                DeoptimizeReason::kDivisionByZero => "division by zero",
                DeoptimizeReason::kFloat16NotYetSupported => "float16 is not supported as machine operation",
                DeoptimizeReason::kGreaterThanMaxFastElementArray => "length is greater than the maximum for fast elements array",
                DeoptimizeReason::kHole => "hole",
                DeoptimizeReason::kInstanceMigrationFailed => "instance migration failed",
                DeoptimizeReason::kInsufficientTypeFeedbackForArrayLiteral => "Insufficient type feedback for array literal",
                DeoptimizeReason::kInsufficientTypeFeedbackForBinaryOperation => "Insufficient type feedback for binary operation",
                DeoptimizeReason::kInsufficientTypeFeedbackForCall => "Insufficient type feedback for call",
                DeoptimizeReason::kInsufficientTypeFeedbackForCompareOperation => "Insufficient type feedback for compare operation",
                DeoptimizeReason::kInsufficientTypeFeedbackForConstruct => "Insufficient type feedback for construct",
                DeoptimizeReason::kInsufficientTypeFeedbackForForIn => "Insufficient type feedback for for-in",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericGlobalAccess => "Insufficient type feedback for generic global access",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericKeyedAccess => "Insufficient type feedback for generic keyed access",
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericNamedAccess => "Insufficient type feedback for generic named access",
                DeoptimizeReason::kInsufficientTypeFeedbackForInstanceOf => "Insufficient type feedback for instanceof",
                DeoptimizeReason::kInsufficientTypeFeedbackForObjectLiteral => "Insufficient type feedback for object literal",
                DeoptimizeReason::kInsufficientTypeFeedbackForTypeOf => "Insufficient type feedback for typeof",
                DeoptimizeReason::kInsufficientTypeFeedbackForUnaryOperation => "Insufficient type feedback for unary operation",
                DeoptimizeReason::kKeyedAccessChanged => "unexpected name in keyed access",
                DeoptimizeReason::kLostPrecision => "lost precision",
                DeoptimizeReason::kLostPrecisionOrNaN => "lost precision or NaN",
                DeoptimizeReason::kMinusZero => "minus zero",
                DeoptimizeReason::kNaN => "NaN",
                DeoptimizeReason::kNoCache => "no cache",
                DeoptimizeReason::kNoInitialElement => "no initial element",
                DeoptimizeReason::kNotABigInt => "not a BigInt",
                DeoptimizeReason::kNotABigInt64 => "not a BigInt64",
                DeoptimizeReason::kNotAHeapNumber => "not a heap number",
                DeoptimizeReason::kNotAJavaScriptObject => "not a JavaScript object",
                DeoptimizeReason::kNotAJavaScriptObjectOrNullOrUndefined => "not a JavaScript object, Null or Undefined",
                DeoptimizeReason::kNotANumber => "not a Number",
                DeoptimizeReason::kNotANumberOrBoolean => "not a Number or Boolean",
                DeoptimizeReason::kNotANumberOrOddball => "not a Number or Oddball",
                DeoptimizeReason::kNotASmi => "not a Smi",
                DeoptimizeReason::kNotAString => "not a String",
                DeoptimizeReason::kNotAStringOrStringWrapper => "not a String or a string wrapper",
                DeoptimizeReason::kNotAStringWrapper => "not a string wrapper",
                DeoptimizeReason::kNotASymbol => "not a Symbol",
                DeoptimizeReason::kNotAdditiveSafeInteger => "not AdditiveSafeInteger",
                DeoptimizeReason::kNotAnArrayIndex => "not an array index",
                DeoptimizeReason::kNotDetectableReceiver => "not a detectable receiver",
                DeoptimizeReason::kNotInt32 => "not int32",
                DeoptimizeReason::kNotUint32 => "not unsigned int32",
                DeoptimizeReason::kOSREarlyExit => "exit from OSR'd inner loop",
                DeoptimizeReason::kOutOfBounds => "out of bounds",
                DeoptimizeReason::kOverflow => "overflow",
                DeoptimizeReason::kPrepareForOnStackReplacement => "prepare for on stack replacement (OSR)",
                DeoptimizeReason::kSmi => "Smi",
                DeoptimizeReason::kStoreToConstant => "Storing to a constant field",
                DeoptimizeReason::kStringTooLarge => "Result string larger than String::kMaxLength",
                DeoptimizeReason::kSuspendGeneratorIsDead => "SuspendGenerator is in a dead branch",
                DeoptimizeReason::kUnexpectedContextExtension => "unexpected context extension",
                DeoptimizeReason::kUnknown => "(unknown)",
                DeoptimizeReason::kUnoptimizedCatch => "First use of catch block",
                DeoptimizeReason::kValueMismatch => "value mismatch",
                DeoptimizeReason::kWrongCallTarget => "wrong call target",
                DeoptimizeReason::kWrongConstructor => "wrong call target constructor",
                DeoptimizeReason::kWrongEnumIndices => "wrong enum indices",
                DeoptimizeReason::kWrongFeedbackCell => "wrong feedback cell",
                DeoptimizeReason::kWrongInstanceType => "wrong instance type",
                DeoptimizeReason::kWrongMap => "wrong map",
                DeoptimizeReason::kWrongMapDynamic => "map changed during operation",
                DeoptimizeReason::kWrongName => "wrong name",
                DeoptimizeReason::kWrongValue => "wrong value",
            }
        }
    }

    impl LazyDeoptimizeReason {
        pub fn to_string(&self) -> &'static str {
            match self {
                LazyDeoptimizeReason::kMapDeprecated => "dependent map was deprecated",
                LazyDeoptimizeReason::kPrototypeChange => "dependent prototype chain changed",
                LazyDeoptimizeReason::kPropertyCellChange => "dependent property cell changed",
                LazyDeoptimizeReason::kFieldTypeConstChange => "dependent field type constness changed",
                LazyDeoptimizeReason::kFieldTypeChange => "dependent field type changed",
                LazyDeoptimizeReason::kFieldRepresentationChange => "dependent field representation changed",
                LazyDeoptimizeReason::kInitialMapChange => "dependent initial map changed",
                LazyDeoptimizeReason::kAllocationSiteTenuringChange => "dependent allocation site tenuring changed",
                LazyDeoptimizeReason::kAllocationSiteTransitionChange => "dependent allocation site transition changed",
                LazyDeoptimizeReason::kScriptContextSlotPropertyChange => "dependent script context slot property changed",
                LazyDeoptimizeReason::kEmptyContextExtensionChange => "dependent empty context extension changed",
                LazyDeoptimizeReason::kWeakObjects => "embedded weak objects cleared",
                LazyDeoptimizeReason::kDebugger => "JS debugger attached",
                LazyDeoptimizeReason::kTesting => "for testing",
                LazyDeoptimizeReason::kExceptionCaught => "exception with omitted catch handler",
                LazyDeoptimizeReason::kEagerDeopt => "marked due to eager deopt",
                LazyDeoptimizeReason::kFrameValueMaterialized => "value in stack frame was materialized",
            }
        }
    }

    impl fmt::Display for LazyDeoptimizeReason {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
                LazyDeoptimizeReason::kMapDeprecated => "MapDeprecated",
                LazyDeoptimizeReason::kPrototypeChange => "PrototypeChange",
                LazyDeoptimizeReason::kPropertyCellChange => "PropertyCellChange",
                LazyDeoptimizeReason::kFieldTypeConstChange => "FieldTypeConstChange",
                LazyDeoptimizeReason::kFieldTypeChange => "FieldTypeChange",
                LazyDeoptimizeReason::kFieldRepresentationChange => "FieldRepresentationChange",
                LazyDeoptimizeReason::kInitialMapChange => "InitialMapChange",
                LazyDeoptimizeReason::kAllocationSiteTenuringChange => "AllocationSiteTenuringChange",
                LazyDeoptimizeReason::kAllocationSiteTransitionChange => "AllocationSiteTransitionChange",
                LazyDeoptimizeReason::kScriptContextSlotPropertyChange => "ScriptContextSlotPropertyChange",
                LazyDeoptimizeReason::kEmptyContextExtensionChange => "EmptyContextExtensionChange",
                LazyDeoptimizeReason::kWeakObjects => "WeakObjects",
                LazyDeoptimizeReason::kDebugger => "Debugger",
                LazyDeoptimizeReason::kTesting => "Testing",
                LazyDeoptimizeReason::kExceptionCaught => "ExceptionCaught",
                LazyDeoptimizeReason::kEagerDeopt => "EagerDeopt",
                LazyDeoptimizeReason::kFrameValueMaterialized => "FrameValueMaterialized",
            };
            write!(f, "{}", name)
        }
    }

    pub fn deoptimize_reason_to_string(reason: DeoptimizeReason) -> &'static str {
        reason.to_string()
    }

    pub fn lazy_deoptimize_reason_to_string(reason: LazyDeoptimizeReason) -> &'static str {
        reason.to_string()
    }

    pub fn is_deoptimization_without_code_invalidation(reason: DeoptimizeReason) -> bool {
        reason == DeoptimizeReason::kPrepareForOnStackReplacement ||
            reason == DeoptimizeReason::kOSREarlyExit
    }

    impl Hash for LazyDeoptimizeReason {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (*self as u8).hash(state);
        }
    }
}
