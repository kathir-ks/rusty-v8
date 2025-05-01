// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Identity type.  This is a no-op type transformation.
pub struct Identity<T>(pub std::marker::PhantomData<T>);

impl<T> Identity<T> {
    pub type Type = T;
}

/// Helper struct for extracting the underlying type of an enum.
pub struct UnderlyingTypeHelper<T>(pub std::marker::PhantomData<T>);

impl<T: std::marker::Copy> UnderlyingTypeHelper<T> {
  // This is a bit tricky as we cannot access the underlying type of an enum directly in Rust
  // without additional dependencies like `num-traits`.  We'll use a trait to
  // provide a default implementation, but users can override it for their specific enums.
  pub type Type = <T as UnderlyingTypeTrait>::UnderlyingType;
}

/// Trait to define the underlying type for enums.
/// Users can implement this trait for their enums to specify the underlying type.
pub trait UnderlyingTypeTrait {
    type UnderlyingType;
}

/// A type alias that resolves to the underlying type of an enum if `T` is an enum,
/// otherwise resolves to `T` itself.
pub type UnderlyingTypeIfEnum<T> =
    <Conditional<std::is_enum::IsEnum<T>, UnderlyingTypeHelper<T>, Identity<T>> as SelectType>::Type;


/// Trait for selecting a type based on a boolean condition.
pub trait SelectType {
    type Type;
}

/// Conditional type selection.
pub enum Conditional<const B: bool, T, F> {
    _True(std::marker::PhantomData<T>),
    _False(std::marker::PhantomData<F>),
}

impl<T, F> SelectType for Conditional<true, T, F> {
    type Type = T;
}

impl<T, F> SelectType for Conditional<false, T, F> {
    type Type = F;
}

/// A trait mimicking std::is_enum in C++.  Requires manual implementation for each enum type.
pub mod std {
    pub mod is_enum {
        pub struct IsEnum<T>(pub std::marker::PhantomData<T>);
    }
}

/// Casts a value to its underlying type if it's an enum. Otherwise, returns the value as is.
pub fn cast_to_underlying_type_if_enum<T: Copy>(x: T) -> UnderlyingTypeIfEnum<T> {
    // In Rust, casting between types requires explicit conversion functions,
    // and is further complicated by the lack of direct access to the underlying
    // representation of an enum without knowing its specific type.
    // The following is a generic implementation which relies on an associated type.
    //
    // NOTE: This requires that the 'UnderlyingTypeTrait' be implemented for each enum used,
    // specifying the correct underlying type.
    x
}
