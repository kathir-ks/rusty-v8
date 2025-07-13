// Converted from V8 C++ source files:
// Header: union.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct code {}
pub struct V8 {}
pub mod v8 {
  pub mod internal {
    pub struct TaggedObject{dummy : i32}
  }
}
pub enum FieldStyle {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-impl-inl.h
struct UseScratchRegisterScope{dummy : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-impl-inl.h
struct UseScratchRegisterScope{dummy : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/baseline/s390/baseline-assembler-s390-inl.h
pub struct detail {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/mips64/macro-assembler-mips64.h
pub struct Base {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/baseline/s390/baseline-assembler-s390-inl.h
pub struct detail {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/js-duration-format.h
pub enum class FieldStyle {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/map.h
pub struct Union {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-impl-inl.h
struct UseScratchRegisterScope{dummy : i32}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-field.h
pub struct AllStatic {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/code-stub-assembler-inl.h
enum Union<T, U> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/baseline/s390/baseline-assembler-s390-inl.h
pub struct detail {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/mips64/macro-assembler-mips64.h
pub struct Base {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-field.h
pub struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/objects.h
pub struct This{
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/map.h
pub struct Union {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum struct UNumberFormatFields {
// From /home/kathirks_gc/v8_go/archive/codebase/src/baseline/s390/baseline-assembler-s390-inl.h
pub struct detail {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/js-duration-format.h
pub struct V8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/init/bootstrapper.h
pub struct UnionOf {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/tagged-field.h
pub struct Smi {}

mod base {
    pub struct TemplateUtils {}
    impl TemplateUtils {
        
    }
    
}
mod common {
    pub mod globals {
        
    }
}
use std::marker::PhantomData;

pub struct AllStatic {}

// Unions are required to be non-nested (i.e. no unions of unions), and to
// have each type only once. The UnionOf<Ts...> helper can be used to flatten
// nested unions and remove duplicates.
//
// Inheritance from Unions is forbidden because it messes with `is_subtype`
// checking.
pub struct Union<T>(PhantomData<T>);

// is_union<T> is a type trait that returns true if T is a union.
pub struct is_union<T>(PhantomData<T>);
impl<T> is_union<T> {
    const value: bool = false;
}
impl<T> is_union<Union<T>> {
    const value: bool = true;
}
static is_union_v: bool = false;
mod detail {
    pub struct UnionWithoutHelper {}
    impl UnionWithoutHelper {
        
    }
    pub struct FlattenUnionHelper {}
    impl FlattenUnionHelper {
        
    }
}

// UnionOf<Ts...> is a helper that returns a union of multiple V8 types,
// flattening any nested unions and removing duplicate types.
pub struct UnionOf {}
impl UnionOf {
    
}

// Unions of unions are flattened.
// Unions with duplicates are deduplicated.
// Unions with Smis are normalized to have the Smi be the first element.

// Union::Without matches expectations.
// Union::Without that doesn't have a match is a no-op
pub struct JSAny {}
pub struct JSAnyNotSmi {}
pub struct JSAnyNotNumber {}
impl<T> Union<T> {
  pub fn Without<U>() {}
}
}
