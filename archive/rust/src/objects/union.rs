// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::marker::PhantomData;
//use std::any::Any;

/// Represents a union of multiple V8 types.
///
/// Unions are required to be non-nested (i.e. no unions of unions), and to
/// have each type only once. The `UnionOf` helper can be used to flatten
/// nested unions and remove duplicates.
///
/// Inheritance from Unions is forbidden because it messes with `is_subtype`
/// checking.
pub struct Union<T>(std::marker::PhantomData<T>);

// TODO: Implement AllStatic trait equivalent in Rust.
// This represents a class with only static members, 
// which doesn't have a direct equivalent in Rust without enforcing it 
// through other means (e.g., a marker trait and a lint).
pub trait AllStatic {}

//impl<T> AllStatic for Union<T> {}

pub trait Without<U> {
    type Output;
}

// Helper trait to check if a type is a Union.
pub trait IsUnion {
    const VALUE: bool;
}

impl<T> IsUnion for Union<T> {
    const VALUE: bool = true;
}

impl<T> IsUnion for T {
    const VALUE: bool = false;
}

// Convenience constant for the associated constant.
const IS_UNION_V: bool = false;

macro_rules! is_union_v {
    ($t:ty) => {
        <$t as IsUnion>::VALUE
    };
}

//macro_rules! static_assert {
//    ($condition:expr, $message:expr) => {
//        if !$condition {
//            panic!($message);
//        }
//    };
//}

mod detail {
    use super::*;

    // TODO: Implement UnionWithoutHelper
    // Implementing a similar type transformation in Rust requires complex 
    // type-level programming with traits, which might not be directly 
    // translatable from the C++ template metaprogramming approach.
    pub struct UnionWithoutHelper {}
}

impl<T> Union<T> {
    // TODO: Implement Without associated type in Rust.
    // Implementing a similar type transformation in Rust requires complex 
    // type-level programming with traits, which might not be directly 
    // translatable from the C++ template metaprogramming approach.
   // pub type Without<U> = detail::UnionWithoutHelper;
}

// TODO: Implement FlattenUnionHelper.  This functionality is not possible in Rust without a macro and trait magic.
//mod detail {
//    use super::*;
//
//    pub struct FlattenUnionHelper {}
//}

// TODO: Implement UnionOf type alias in Rust.
//type UnionOf<T> = detail::FlattenUnionHelper;
//
//// Unions of unions are flattened.
//static_assert!(std::is_same_v<Union<Smi, HeapObject>,
//                             UnionOf<UnionOf<Smi>, UnionOf<HeapObject>>>);
//// Unions with duplicates are deduplicated.
//static_assert!(std::is_same_v<Union<Smi, HeapObject>,
//                             UnionOf<HeapObject, Smi, Smi, HeapObject>>);
//// Unions with Smis are normalized to have the Smi be the first element.
//static_assert!(std::is_same_v<Union<Smi, HeapObject>, UnionOf<HeapObject, Smi>>);
//
//// Union::Without matches expectations.
//static_assert!(
//    std::is_same_v<Union<Smi, HeapObject>::Without<Smi>, Union<HeapObject>>);
//static_assert!(std::is_same_v<JSAny::Without<Smi>, JSAnyNotSmi>);
//static_assert!(
//    std::is_same_v<JSAny::Without<Smi>::Without<HeapNumber>, JSAnyNotNumber>);
//
//// Union::Without that doesn't have a match is a no-op
//static_assert!(std::is_same_v<Union<Smi, HeapObject>::Without<HeapNumber>,
//                             Union<Smi, HeapObject>>);

// Placeholder types, replace with actual definitions.
pub struct Smi;
pub struct HeapObject;
pub struct JSAny;
pub struct JSAnyNotSmi;
pub struct HeapNumber;
pub struct JSAnyNotNumber;