//use std::any::Any;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker;

//use v8_fast_api_calls::*; // Assuming a crate for V8 Fast API calls
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
//use base::*; // Assuming a crate for base functionalities
//use compiler::*; // Assuming a crate for compiler functionalities
//use handles::*; // Assuming a crate for handles functionalities
//use objects::*; // Assuming a crate for objects functionalities

//#[cfg(V8_ENABLE_WEBASSEMBLY)]
//use wasm_compiler_definitions::*; // Assuming a crate for WASM compiler definitions

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseTaggedness {
    kUntaggedBase,
    kTaggedBase,
}

impl fmt::Display for BaseTaggedness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseTaggedness::kUntaggedBase => write!(f, "untagged base"),
            BaseTaggedness::kTaggedBase => write!(f, "tagged base"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapRef {
    //TODO: Properly define MapRef
    object: u64, //Placeholder for MapRef::object()
}

impl MapRef {
    pub fn object(&self) -> &u64 {
        &self.object
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstFieldInfo {
    owner_map: MapRef, // Placeholder
}

impl ConstFieldInfo {
    pub fn is_const(&self) -> bool {
        // TODO: Implement const determination
        false
    }
    //TODO: implement owner_map access
}

impl fmt::Display for ConstFieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_const() {
            write!(f, "const (field owner: unknown)") //Brief(*const_field_info.owner_map->object())
        } else {
            write!(f, "mutable")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccess {
    pub base_is_tagged: BaseTaggedness,
    pub offset: usize,
    pub map: Option<MapRef>,
    pub machine_type: MachineType,
    pub const_field_info: ConstFieldInfo,
    pub is_store_in_literal: bool,
    pub creator_mnemonic: Option<String>,
    pub name: String, // Placeholder
    pub type_: Type,
    pub write_barrier_kind: WriteBarrierKind,
    pub maybe_initializing_or_transitioning_store: bool,
}

impl fmt::Display for FieldAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        if let Some(creator_mnemonic) = &self.creator_mnemonic {
            write!(f, "{}, ", creator_mnemonic)?;
        }
        write!(f, "{}, {}, ", self.base_is_tagged, self.offset)?;
        //TODO: Implement name printing
        //if let Some(name) = &self.name {
        //    write!(f,"{}, ", name)?;
        //}
        //if let Some(map) = &self.map {
        //    write!(f, "{}, ", map)?;
        //}
        write!(
            f,
            "{}, {}, {}, {}",
            self.type_, self.machine_type, self.write_barrier_kind, self.const_field_info
        )?;
        if self.is_store_in_literal {
            write!(f, " (store in literal)")?;
        }
        if self.maybe_initializing_or_transitioning_store {
            write!(f, " (initializing or transitioning store)")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementAccess {
    pub base_is_tagged: BaseTaggedness,
    pub header_size: usize,
    pub machine_type: MachineType,
    pub type_: Type,
    pub write_barrier_kind: WriteBarrierKind,
}

impl fmt::Display for ElementAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.base_is_tagged,
            self.header_size,
            self.type_,
            self.machine_type,
            self.write_barrier_kind
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectAccess {
    pub machine_type: MachineType,
    pub write_barrier_kind: WriteBarrierKind,
}

impl fmt::Display for ObjectAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.machine_type, self.write_barrier_kind)
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WasmFieldInfo {
    pub field_index: u32,
    pub type_: WasmType,
    pub is_signed: bool,
    pub null_check: CheckForNull,
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
impl fmt::Display for WasmFieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.field_index,
            if self.is_signed { "signed" } else { "unsigned" },
            match self.null_check {
                CheckForNull::kWithNullCheck => "null check",
                CheckForNull::kNoNullCheck => "no null check",
            }
        )
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WasmElementInfo {
    pub type_: WasmType,
    pub is_signed: bool,
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
impl fmt::Display for WasmElementInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_signed { "signed" } else { "unsigned" })
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckForNull {
    kWithNullCheck,
    kNoNullCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalArrayType {
    kExternalInt8Array,
    // ... other types
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvertReceiverMode {
    kAny,
    kNullOrUndefined,
    kNotNullOrUndefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckFloat64HoleMode {
    kAllowReturnHole,
    kNeverReturnHole,
}

impl fmt::Display for CheckFloat64HoleMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckFloat64HoleMode::kAllowReturnHole => write!(f, "allow-return-hole"),
            CheckFloat64HoleMode::kNeverReturnHole => write!(f, "never-return-hole"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckFloat64HoleParameters {
    mode: CheckFloat64HoleMode,
    feedback: FeedbackSource,
}

impl CheckFloat64HoleParameters {
    pub fn new(mode: CheckFloat64HoleMode, feedback: FeedbackSource) -> Self {
        CheckFloat64HoleParameters { mode, feedback }
    }

    pub fn mode(&self) -> CheckFloat64HoleMode {
        self.mode
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for CheckFloat64HoleParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.mode, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckForMinusZeroMode {
    kCheckForMinusZero,
    kDontCheckForMinusZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckMinusZeroParameters {
    mode: CheckForMinusZeroMode,
    feedback: FeedbackSource,
}

impl CheckMinusZeroParameters {
    pub fn new(mode: CheckForMinusZeroMode, feedback: FeedbackSource) -> Self {
        CheckMinusZeroParameters { mode, feedback }
    }

    pub fn mode(&self) -> CheckForMinusZeroMode {
        self.mode
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for CheckMinusZeroParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.mode, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckMapsFlag {
    kNone = 0,
    kTryMigrateInstance = 1 << 0,
    kTryMigrateInstanceAndDeopt = 1 << 1,
}

impl fmt::Display for CheckMapsFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckMapsFlag::kTryMigrateInstance => write!(f, "TryMigrateInstance"),
            CheckMapsFlag::kTryMigrateInstanceAndDeopt => {
                write!(f, "TryMigrateInstanceAndDeopt")
            }
            CheckMapsFlag::kNone => write!(f, "None"),
        }
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct CheckMapsFlags: u32 {
        const kTryMigrateInstance = 1 << 0;
        const kTryMigrateInstanceAndDeopt = 1 << 1;
    }
}

impl fmt::Display for CheckMapsFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "None")
        } else {
            fmt::Debug::fmt(self, f)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckMapsParameters {
    flags: CheckMapsFlags,
    maps: Vec<MapRef>, //ZoneRefSet<Map>,
    feedback: FeedbackSource,
}

impl CheckMapsParameters {
    pub fn new(flags: CheckMapsFlags, maps: Vec<MapRef>, feedback: FeedbackSource) -> Self {
        CheckMapsParameters {
            flags,
            maps,
            feedback,
        }
    }

    pub fn flags(&self) -> CheckMapsFlags {
        self.flags
    }

    pub fn maps(&self) -> &Vec<MapRef> {
        &self.maps
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for CheckMapsParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:?}, {}", self.flags, self.maps, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckTaggedInputMode {
    kAdditiveSafeInteger,
    kNumber,
    kNumberOrBoolean,
    kNumberOrOddball,
}

impl fmt::Display for CheckTaggedInputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckTaggedInputMode::kAdditiveSafeInteger => write!(f, "AdditiveSafeInteger"),
            CheckTaggedInputMode::kNumber => write!(f, "Number"),
            CheckTaggedInputMode::kNumberOrBoolean => write!(f, "NumberOrBoolean"),
            CheckTaggedInputMode::kNumberOrOddball => write!(f, "NumberOrOddball"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrowFastElementsMode {
    kDoubleElements,
    kSmiOrObjectElements,
}

impl fmt::Display for GrowFastElementsMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrowFastElementsMode::kDoubleElements => write!(f, "DoubleElements"),
            GrowFastElementsMode::kSmiOrObjectElements => write!(f, "SmiOrObjectElements"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrowFastElementsParameters {
    mode: GrowFastElementsMode,
    feedback: FeedbackSource,
}

impl GrowFastElementsParameters {
    pub fn new(mode: GrowFastElementsMode, feedback: FeedbackSource) -> Self {
        GrowFastElementsParameters { mode, feedback }
    }

    pub fn mode(&self) -> GrowFastElementsMode {
        self.mode
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for GrowFastElementsParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.mode, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementsTransitionMode {
    kFastTransition,
    kSlowTransition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementsTransition {
    mode: ElementsTransitionMode,
    source: MapRef,
    target: MapRef,
}

impl ElementsTransition {
    pub fn new(mode: ElementsTransitionMode, source: MapRef, target: MapRef) -> Self {
        ElementsTransition { mode, source, target }
    }

    pub fn mode(&self) -> ElementsTransitionMode {
        self.mode
    }

    pub fn source(&self) -> &MapRef {
        &self.source
    }

    pub fn target(&self) -> &MapRef {
        &self.target
    }
}

impl fmt::Display for ElementsTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            ElementsTransitionMode::kFastTransition => write!(
                f,
                "fast-transition from unknown to unknown"
            ),
            ElementsTransitionMode::kSlowTransition => write!(
                f,
                "slow-transition from unknown to unknown"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementsTransitionWithMultipleSources {
    sources: Vec<MapRef>,
    target: MapRef,
}

impl ElementsTransitionWithMultipleSources {
    pub fn new(sources: Vec<MapRef>, target: MapRef) -> Self {
        ElementsTransitionWithMultipleSources { sources, target }
    }

    pub fn sources(&self) -> &Vec<MapRef> {
        &self.sources
    }

    pub fn target(&self) -> &MapRef {
        &self.target
    }
}

impl fmt::Display for ElementsTransitionWithMultipleSources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transition from (")?;
        let mut first = true;
        for source in &self.sources {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            //os << Brief(*source.object());
            write!(f, "unknown")?;
        }
        write!(f, ") to unknown")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BigIntOperationHint {
    kBigInt,
    kBigInt64,
}

impl fmt::Display for BigIntOperationHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BigIntOperationHint::kBigInt => write!(f, "BigInt"),
            BigIntOperationHint::kBigInt64 => write!(f, "BigInt64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberOperationHint {
    kSignedSmall,
    kSignedSmallInputs,
    kAdditiveSafeInteger,
    kNumber,
    kNumberOrBoolean,
    kNumberOrOddball,
}

impl fmt::Display for NumberOperationHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberOperationHint::kSignedSmall => write!(f, "SignedSmall"),
            NumberOperationHint::kSignedSmallInputs => write!(f, "SignedSmallInputs"),
            NumberOperationHint::kAdditiveSafeInteger => write!(f, "AdditiveSafeInteger"),
            NumberOperationHint::kNumber => write!(f, "Number"),
            NumberOperationHint::kNumberOrBoolean => write!(f, "NumberOrBoolean"),
            NumberOperationHint::kNumberOrOddball => write!(f, "NumberOrOddball"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberOperationParameters {
    hint: NumberOperationHint,
    feedback: FeedbackSource,
}

impl NumberOperationParameters {
    pub fn new(hint: NumberOperationHint, feedback: FeedbackSource) -> Self {
        NumberOperationParameters { hint, feedback }
    }

    pub fn hint(&self) -> NumberOperationHint {
        self.hint
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for NumberOperationParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.hint, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BigIntOperationParameters {
    hint: BigIntOperationHint,
    feedback: FeedbackSource,
}

impl BigIntOperationParameters {
    pub fn new(hint: BigIntOperationHint, feedback: FeedbackSource) -> Self {
        BigIntOperationParameters { hint, feedback }
    }

    pub fn hint(&self) -> BigIntOperationHint {
        self.hint
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for BigIntOperationParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.hint, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeculativeBigIntAsNParameters {
    bits: i32,
    feedback: FeedbackSource,
}

impl SpeculativeBigIntAsNParameters {
    pub fn new(bits: i32, feedback: FeedbackSource) -> Self {
        SpeculativeBigIntAsNParameters { bits, feedback }
    }

    pub fn bits(&self) -> i32 {
        self.bits
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for SpeculativeBigIntAsNParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.bits, self.feedback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocateParameters {
    type_: Type,
    allocation_type: AllocationType,
}

impl AllocateParameters {
    pub fn new(type_: Type, allocation_type: AllocationType) -> Self {
        AllocateParameters {
            type_,
            allocation_type,
        }
    }

    pub fn type_(&self) -> Type {
        self.type_
    }

    pub fn allocation_type(&self) -> AllocationType {
        self.allocation_type
    }
}

impl fmt::Display for AllocateParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.type_, self.allocation_type)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationType {
    kOld,
    kYoung,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbortReason {
    kNoReason,
    //... other reasons
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckTaggedInputParameters {
    mode: CheckTaggedInputMode,
    feedback: FeedbackSource,
}

impl CheckTaggedInputParameters {
    pub fn new(mode: CheckTaggedInputMode, feedback: FeedbackSource) -> Self {
        CheckTaggedInputParameters { mode, feedback }
    }

    pub fn mode(&self) -> CheckTaggedInputMode {
        self.mode
    }

    pub fn feedback(&self) -> FeedbackSource {
        self.feedback
    }
}

impl fmt::Display for CheckTaggedInputParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.mode, self.feedback)
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssertNotNullParameters {
    pub type_: WasmType,
    pub trap_id: TrapId,
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
impl fmt::Display for AssertNotNullParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.type_, self.trap_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrOpcode {
    kBooleanNot,
    kNumberEqual,
    kNumberLessThan,
    kNumberLessThanOrEqual,
    kNumberAdd,
    kNumberSubtract,
    kNumberMultiply,
    kNumberDivide,
    kNumberModulus,
    kNumberBitwiseOr,
    kNumberBitwiseXor,
    kNumberBitwiseAnd,
    kNumberShiftLeft,
    kNumberShiftRight,
    kNumberShiftRightLogical,
    kNumberImul,
    kNumberAbs,
    kNumberClz32,
    kNumberCeil,
    kNumberFloor,
    kNumberFround,
    kNumberAcos,
    kNumberAcosh,
    kNumberAsin,
    kNumberAsinh,
    kNumberAtan,
    kNumberAtan2,
    kNumberAtanh,
    kNumberCbrt,
    kNumberCos,
    kNumberCosh,
    kNumberExp,
    kNumberExpm1,
    kNumberLog,
    kNumberLog1p,
    kNumberLog10,
    kNumberLog2,
    kNumberMax,
    kNumberMin,
    kNumberPow,
    kNumberRound,
    kNumberSign,
    kNumberSin,
    kNumberSinh,
    kNumberSqrt,
    kNumberTan,
    kNumberTanh,
    kNumberTrunc,
    kNumberToBoolean,
    kNumberToInt32,
    kNumberToString,
    kNumberToUint32,
    kNumberToUint8Clamped,
    kIntegral32OrMinusZeroToBigInt,
    kNumberSilenceNaN,
    kBigIntEqual,
    kBigIntLessThan,
    kBigIntLessThanOrEqual,
    kBigIntNegate,
    kStringConcat,
    kStringToNumber,
    kStringFromSingleCharCode,
    kStringFromSingleCodePoint,
    kStringIndexOf,
    kStringLength,
    kStringWrapperLength,
    kStringToLowerCaseIntl,
    kStringToUpperCaseIntl,
    kTypeOf,
    kPlainPrimitiveToNumber,
    kPlainPrimitiveToWord32,
    kPlainPrimitiveToFloat64,
    kChangeTaggedSignedToInt32,
    kChangeTaggedSignedToInt64,
    kChangeTaggedToInt32,
    kChangeTaggedToInt64,
    kChangeTaggedToUint32,
    kChangeTaggedToFloat64,
    kChangeTaggedToTaggedSigned,
    kChangeFloat64ToTaggedPointer,
    kChangeFloat64HoleToTagged,
    kChangeInt31ToTaggedSigned,
    kChangeInt32ToTagged,
    kChangeInt64ToTagged,
    kChangeUint32ToTagged,
    kChangeUint64ToTagged,
    kChangeTaggedToBit,
    kChangeBitToTagged,
    kTruncateBigIntToWord64,
    kChangeInt64ToBigInt,
    kChangeUint64ToBigInt,
    kTruncateTaggedToBit,
    kTruncateTaggedPointerToBit,
    kTruncateTaggedToWord32,
    kTruncateTaggedToFloat64,
    kObjectIsArrayBufferView,
    kObjectIsBigInt,
    kObjectIsCallable,
    kObjectIsConstructor,
    kObjectIsDetectableCallable,
    kObjectIsMinusZero,
    kNumberIsMinusZero,
    kObjectIsNaN,
    kNumberIsNaN,
    kObjectIsNonCallable,
    kObjectIsNumber,
    kObjectIsReceiver,
    kObjectIsSmi,
    kObjectIsString,
    kObjectIsSymbol,
    kObjectIsUndetectable,
    kNumberIsFloat64Hole,
    kNumberIsFinite,
    kObjectIsFiniteNumber,
    kNumberIsInteger,
    kObjectIsSafeInteger,
    kNumberIsSafeInteger,
    kObjectIsInteger,
    kConvertTaggedHoleToUndefined,
    kSameValue,
    kSameValueNumbersOnly,
    kNumberSameValue,
    kReferenceEqual,
    kStringEqual,
    kStringLessThan,
    kStringLessThanOrEqual,
    kToBoolean,
    kNewConsString,
    kUnsigned32Divide,
    kBigIntAdd,
    kBigIntSubtract,
    kBigIntMultiply,
    kBigIntDivide,
    kBigIntModulus,
    kBigIntBitwiseAnd,
    kBigIntBitwiseOr,
    kBigIntBitwiseXor,
    kBigIntShiftLeft,
    kBigIntShiftRight,
    kStringCharCodeAt,
    kStringCodePointAt,
    kStringFromCodePointAt,
    kStringSubstring,
    kDateNow,
    kDoubleArrayMax,
    kDoubleArrayMin,
    kSpeculativeNumberAdd,
    kSpeculativeNumberSubtract,
    kSpeculativeNumberMultiply,
    kSpeculativeNumberPow,
    kSpeculativeNumberDivide,
    kSpeculativeNumberModulus,
    kSpeculativeNumberShiftLeft,
    kSpeculativeNumberShiftRight,
    kSpeculativeNumberShiftRightLogical,
    kSpeculativeNumberBitwiseAnd,
    kSpeculativeNumberBitwiseOr,
    kSpeculativeNumberBitwiseXor,
    kSpeculativeNumberEqual,
    kSpeculativeNumberLessThan,
    kSpeculativeNumberLessThanOrEqual,
    kSpeculativeAdditiveSafeIntegerAdd,
    kSpeculativeAdditiveSafeIntegerSubtract,
    kSpeculativeSmallIntegerAdd,
    kSpeculativeSmallIntegerSubtract,
    kCheckEqualsInternalizedString,
    kCheckEqualsSymbol,
    kCheckHeapObject,
    kCheckInternalizedString,
    kCheckNotTaggedHole,
    kCheckReceiver,
    kCheckReceiverOrNullOrUndefined,
    kCheckSymbol,
    kCheckedInt32Add,
    kCheckedInt32Div,
    kCheckedInt32Mod,
    kCheckedInt32Sub,
    kCheckedUint32Div,
    kCheckedUint32Mod,
    kCheckedAdditiveSafeIntegerAdd,
    kCheckedAdditiveSafeIntegerSub,
    kCheckedInt64Add,
    kCheckedInt64Sub,
    kCheckedInt64Mul,
    kCheckedInt64Div,
    kCheckedInt64Mod,
    kCheckNumber,
    kCheckNumberFitsInt32,
    kCheckSmi,
    kCheckString,
    kCheckStringOrStringWrapper,
    kCheckBigInt,
    kCheckedBigIntToBigInt64,
    kCheckedInt32ToTaggedSigned,
    kCheckedInt64ToInt32,
    kCheckedInt64ToTaggedSigned,
    kCheckedTaggedToArrayIndex,
    kCheckedTaggedSignedToInt32,
    kCheckedTaggedToTaggedPointer,
    kCheckedTaggedToTaggedSigned,
    kCheckedUint32ToInt32,
    kCheckedUint32ToTaggedSigned,
    kCheckedUint64ToInt32,
    kCheckedUint64ToInt64,
    kCheckedUint64ToTaggedSigned,
    kCheckedUint32Bounds,
    kCheckedUint64Bounds,
    kCheckBounds,
    kCheckIf,
    kFindOrderedHashMapEntry,
    kFindOrderedHashMapEntryForInt32Key,
    kFindOrderedHashSetEntry,
    kChangeFloat64ToTagged,
    kCheckedInt32Mul,
    kCheckedFloat64ToInt32,
    kCheckedFloat64ToAdditiveSafeInteger,
    kCheckedFloat64ToInt64,
    kCheckedTaggedToInt32,
    kCheckedTaggedToAdditiveSafeInteger,
    kCheckedTaggedToInt64,
    kCheckedTaggedToFloat64,
    kCheckedTruncateTaggedToWord32,
    kConvertReceiver,
    kCheckFloat64Hole,
    kEnsureWritableFastElements,
    kMaybeGrowFastElements,
    kLoadFieldByIndex,
    kLoadStackArgument,
    //kLoadField, //TODO: Implement LoadField
    //kLoadElement, //TODO: Implement LoadElement
    //kStoreElement, //TODO: Implement StoreElement
    //kLoadTypedElement, //TODO: Implement LoadTypedElement
    //kStoreTypedElement, //TODO: Implement StoreTypedElement
    kTransitionElementsKind,
    kTransitionElementsKindOrCheckMap,
    kArgumentsLength,
    kRestLength,
    kTypedArrayLength,
    kRuntimeAbort,
    kSpeculativeBigIntAsIntN,
    kSpeculativeBigIntAsUintN,
    kAssertType,
    kVerifyType,
    kCheckTurboshaftTypeOf,
    kNewDoubleElements,
    kNewSmiOrObjectElements,
    kNewArgumentsElements,
    kAllocate,
    kAllocateRaw,
    kFastApiCall,

    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmArrayLength,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmArrayInitializeLength,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kStringAsWtf16,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kStringPrepareForGetCodeunit,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmTypeCheck,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmTypeCheckAbstract,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmTypeCast,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmTypeCastAbstract,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kRttCanon,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kIsNull,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kIsNotNull,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kNull,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kAssertNotNull,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmAnyConvertExtern,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmExternConvertAny,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmStructGet,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmStructSet,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmArrayGet,
    // #[cfg(V8_ENABLE_WEBASSEMBLY)]
    kWasmArraySet,
    kSpeculativeBigIntAdd,
    kSpeculativeBigIntSubtract,
    kSpeculativeBigIntMultiply,
    kSpeculativeBigIntDivide,
    kSpeculativeBigIntModulus,
    kSpeculativeBigIntBitwiseAnd,
    kSpeculativeBigIntBitwiseOr,
    kSpeculativeBigIntBitwiseXor,
    kSpeculativeBigIntShiftLeft,
    kSpeculativeBigIntShiftRight,
    kSpeculativeBigIntEqual,
    kSpeculativeBigIntLessThan,
    kSpeculativeBigIntLessThanOrEqual,
    kSpeculativeBigIntNegate,
    kSpeculativeToBigInt,
    kCheckClosure,

    kLoadField,
    kLoadElement,
    kStoreElement,
    kLoadTypedElement,
    kStoreTypedElement,

    kLoadFromObject,
    kLoadImmutableFromObject,
    kStoreToObject,
    kInitializeImmutableInObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorProperties {
    kNoProperties: u32,
    kCommutative: u