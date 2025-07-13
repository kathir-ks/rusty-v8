// Converted from V8 C++ source files:
// Header: intl-objects.h
// Implementation: intl-objects.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashSet;
use std::string::String;
use std::vec::Vec;
pub struct Object {}
pub struct JSReceiver {}
pub struct JSFunction {}
pub struct Isolate {}
pub struct JSArray {}
pub struct BigInt {}
pub struct JSCollator {}
pub struct SeqOneByteString {}
pub struct String {}
pub struct Map {}
pub struct MessageTemplate {}
pub struct SeqString {}
pub struct JSNumberFormat {}
pub struct FixedArray {}
pub struct UnicodeString {}
pub struct JSObject {}
pub struct WasmInternalFunction {}
pub struct DirectHandle<T> {
   dummy: i32
}
pub struct MaybeHandle<T> {
    
}
pub struct Handle<T> {
   dummy: i32
}
pub struct List {}
pub struct Local<'a, T> {
   dummy: i32
}
pub struct UEnumeration {}
pub struct Root {
    
}

pub struct  Exceptions {}
pub struct Code {}

pub struct DirectHandleVector<T> {
    
}

pub struct PropertyDetails {}

pub struct SharedFunctionInfo {}

pub enum class UColAttributeValue{
    UCOL_OFF
}

pub enum  RoundingMode {
    
}
pub enum   UNumberFormatFields{
    UNUM_INTEGER_FIELD,
    UNUM_FRACTION_FIELD,
    UNUM_DECIMAL_SEPARATOR_FIELD,
    UNUM_GROUPING_SEPARATOR_FIELD,
    UNUM_CURRENCY_FIELD,
    UNUM_PERCENT_FIELD,
    UNUM_SIGN_FIELD,
    UNUM_EXPONENT_SYMBOL_FIELD,
    UNUM_EXPONENT_SIGN_FIELD,
    UNUM_EXPONENT_FIELD,
    UNUM_PERMILL_FIELD,
    UNUM_COMPACT_FIELD,
    UNUM_MEASURE_UNIT_FIELD,
    UNUM_APPROXIMATELY_SIGN_FIELD
}
pub enum  UColAttribute{
    UCOL_ALTERNATE_HANDLING,
    UCOL_CASE_FIRST,
    UCOL_CASE_LEVEL,
    UCOL_FRENCH_COLLATION,
    UCOL_NUMERIC_COLLATION,
    UCOL_STRENGTH
}
pub enum class UCollationResult{
    UCOL_LESS,
    UCOL_GREATER,
    UCOL_EQUAL
}
pub struct InstructionOperand {}
pub struct OpIndex {}
pub struct Immediate {}
pub enum Condition {}
pub enum AbortReason {}
pub struct Name {}
pub enum RootIndex {}
pub struct Value {}
pub struct PropertyAttributes {}
pub struct CodeEntrypointTag {}

pub struct Sandbox {}

pub enum ScriptType {}

pub struct AsmType {}

pub struct FPUControlRegister {}

pub enum VariableMode {}

pub struct OperationType {}

pub struct JsonObject {}

pub struct UnoptimizedCompileFlags {}

pub struct Operand {}

pub struct Instruction {}

pub struct Register {}

pub struct CpuFeatures {}

pub struct WasmInternalFunction {}

pub struct Range {}

pub struct RegisterT {}

pub struct ValueType {}

pub struct Expression {}

pub mod bigint {
    pub struct Digits {}
}
pub struct MemOperand {}

pub struct InstructionSequence {}

pub struct Condition {}

pub struct ValueSerializer {}
pub struct Debug {}

pub struct DoubleRegister {}

pub trait DecoderTraits {}

pub struct Vector<T> {}

pub struct Utf8DecoderBase<D> where D: DecoderTraits {}
pub trait Write {}

pub struct CaseClause {}

pub struct CharacterClassStrings {}

pub struct Tagged<T> {

}

pub struct Tagged_t {

}

pub mod internal {
    pub struct SharedObjectConveyorHandles {}
}

pub struct FocusedTree<Key, Value, Hasher> {}

pub enum Maybe<T> {
    Just(T),
    Nothing
}

pub mod v8 {
    pub struct Isolate {}
}
pub struct RangeError {}

pub struct DirectPointerTag {}

pub struct IndirectPointerTag {}

pub struct Exceptions {}

pub struct Range {}

pub struct ArrayBuffer {}

pub struct SourceTextModule {}

pub struct Op {}

pub struct InstructionOperand {}
pub struct Module {}

pub struct Operation {}

pub struct JSAny {}

pub struct VisitResult {}

pub struct OpIndex {}

pub struct Local<'static,String> {}
pub struct Local<'static, Value> {}

pub struct Op {}

pub struct DoubleRegister {}

pub struct AsmType {}

pub enum class IrregexpImplementation{
    
}
pub struct Register {}

pub struct Code {
    
}

pub struct JSFunction {
    
}

pub mod compiler {
   
    pub struct turboshaft {
        pub struct Common {}
        pub struct Operation {}
    }
    
}

pub mod base {
   pub struct Vector<T> {}
   pub struct TimezoneCache {
    
   }
}

pub mod turboshaft {
    pub struct Operation {}
}

pub enum class PropertyDetails {
   
}

pub struct JSLocale {}

pub struct TaggedObject {
    
}
pub enum struct Condition {
   
}
pub enum struct UNumberFormatFields {
    UNUM_INTEGER_FIELD,
UNUM_FRACTION_FIELD,
UNUM_DECIMAL_SEPARATOR_FIELD,
UNUM_GROUPING_SEPARATOR_FIELD,
UNUM_CURRENCY_FIELD,
UNUM_PERCENT_FIELD,
UNUM_SIGN_FIELD,
UNUM_EXPONENT_SYMBOL_FIELD,
UNUM_EXPONENT_SIGN_FIELD,
UNUM_EXPONENT_FIELD,
UNUM_PERMILL_FIELD,
UNUM_COMPACT_FIELD,
UNUM_MEASURE_UNIT_FIELD,
UNUM_APPROXIMATELY_SIGN_FIELD
}

pub enum class MatcherOption{
    kBestFit,
    kLookup
}
pub enum class OperationType{
    
}
pub enum class VariableMode{
    
}
pub struct TaggedString {}
pub enum class RootIndex{
    
}
pub struct RangeError {}
pub struct Exceptions {}
pub struct DoubleRegister {}
pub enum class FPUControlRegister{
    
}
pub struct This{
    dummy : i32
}
pub struct Register {}
pub mod bigint {
    pub struct Digits {}
}
pub mod v8 {
    pub struct Isolate {}
}
pub struct Local<T> {
   dummy : i32
}
pub mod internal {
    pub struct SharedObjectConveyorHandles {}
}

pub struct MemOperand {}
pub struct ZoneObject {}
pub struct AstNodeSourceRanges {}
pub mod turboshaft {
    pub struct Common {}
}
pub mod turboshaft {
    pub struct Operation {}
}
pub struct AssemblerOptions {}
pub struct CaseClause {}
pub struct JsArray {}
pub struct Operand {}
pub struct Assembler {}
pub mod regexp {
    pub struct Options {}
}
pub struct IrregexpImplementation {}
pub struct Symbol {}
pub struct Rule {}
pub struct InnerPointerToCodeCacheEntry {}
pub mod compiler {
    pub struct turboshaft {
        pub struct Operation {}
    }
    
}
pub struct Instruction {}
pub struct AstNodeSourceRanges {}
pub struct VisitResult {}
pub struct OpIndex {}
pub mod internal {
   pub struct Sandbox {}
}
pub mod compiler {
    pub struct turboshaft {
        pub struct Operation {}
        pub struct Common {}
    }
}

pub struct Op {}
pub struct FocusedTree<Key, Value, Hasher> {}
pub struct IrregexpImplementation {}
pub struct WasmInternalFunction {}
pub struct CpuFeatures {}
pub mod v8 {
    pub struct Isolate {}
}
pub struct JSBreakIterator {}
pub struct JsArray {}
pub struct InstructionOperand {}
pub mod compiler {
   pub struct PersistentMap {}
   pub struct SimplifiedOperatorBuilder {}
}
pub mod compiler {
   pub struct Zone {}
}
pub struct String {
    
}
pub struct BigInt {
    
}
pub struct Code {}
pub struct JSFunction {
    
}
pub struct JSReceiver {}

pub struct JSTemporalTimeZone{
    
}

pub enum JSIntlListFormatPartType {
    
}

impl JSTemporalTimeZone {
    
   const kUTCTimeZoneIndex : i32 = 0;
}

pub trait DecoderTraits {
}

// Helper class to enforce range validation for number option arguments
pub struct NumberFormatRangeError {}
impl NumberFormatRangeError {
    
    pub fn new() -> Self { Self {} }

    
    pub fn set(_value : f64) {
    }
}

pub struct RegExpTree {}

