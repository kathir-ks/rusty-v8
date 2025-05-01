// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod reducer_traits {
    use std::{
        marker::PhantomData,
        cmp::PartialEq,
        fmt::Debug,
        ops::Deref,
        any::Any,
        convert::AsRef,
        borrow::Borrow,
        hash::Hash,
        num::TryFromIntError,
        usize::MAX,
    };

    pub trait ReducerTrait<Next> {}

    pub struct GenericReducerBase<Next> {
        _phantom: PhantomData<Next>,
    }

    impl<Next> ReducerTrait<Next> for GenericReducerBase<Next> {}

    pub struct EmitProjectionReducer<Next> {
        _phantom: PhantomData<Next>,
    }

    impl<Next> ReducerTrait<Next> for EmitProjectionReducer<Next> {}

    pub struct TSReducerBase<Next> {
        _phantom: PhantomData<Next>,
    }

    impl<Next> ReducerTrait<Next> for TSReducerBase<Next> {}

    // Mimic the `reducer_list` using tuples for now.  A proper list type
    // would be more flexible, but this at least allows simple cases to be
    // expressed.

    // Currently, `reducer_list` is a type alias, so there's nothing to convert here
    // except to note the limitations.  We can express `reducer_list<A, B, C>` in
    // Rust as `(A, B, C)`, but we lose the template-template parameter.  This
    // is because Rust's type system doesn't have higher-kinded types natively
    // (though there are experimental crates that add this).

    // We can define helper traits for working with the tuple "reducer_list".

    pub trait ReducerListLength {
        const VALUE: usize;
    }

    impl ReducerListLength for () {
        const VALUE: usize = 0;
    }

    impl<A> ReducerListLength for (A,) {
        const VALUE: usize = 1;
    }

    impl<A, B> ReducerListLength for (A, B) {
        const VALUE: usize = 2;
    }

    impl<A, B, C> ReducerListLength for (A, B, C) {
        const VALUE: usize = 3;
    }

    // Add more impls as needed.

    pub trait ReducerListContains<R> {
        const VALUE: bool;
    }

    impl<R> ReducerListContains<R> for () {
        const VALUE: bool = false;
    }

    impl<A, R> ReducerListContains<R> for (A,) where A: PartialEq<R> {
        const VALUE: bool = std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>();
    }

    impl<A, B, R> ReducerListContains<R> for (A, B)
        where A: PartialEq<R>, B: PartialEq<R>
    {
        const VALUE: bool = std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>() ||
                            std::any::TypeId::of::<B>() == std::any::TypeId::of::<R>();
    }

    // Add more impls as needed.

    pub trait ReducerListStartsWith<R> {
        const VALUE: bool;
    }

    impl<R> ReducerListStartsWith<R> for () {
        const VALUE: bool = false;
    }

    impl<A, R> ReducerListStartsWith<R> for (A,) where A: PartialEq<R> {
        const VALUE: bool = std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>();
    }

    impl<A, B, R> ReducerListStartsWith<R> for (A, B) where A: PartialEq<R> {
        const VALUE: bool = std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>();
    }
    // Add more impls as needed

    pub trait ReducerListIndexOf<R> {
        const VALUE: usize;
    }

    impl<R> ReducerListIndexOf<R> for () {
        const VALUE: usize = MAX;
    }

    impl<A, R> ReducerListIndexOf<R> for (A,) where A: PartialEq<R> {
       const VALUE: usize = if std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>() {0} else {MAX};
    }

    impl<A, B, R> ReducerListIndexOf<R> for (A, B)
        where A: PartialEq<R>, B: PartialEq<R>
    {
        const VALUE: usize =
            if std::any::TypeId::of::<A>() == std::any::TypeId::of::<R>() {
                0
            } else if std::any::TypeId::of::<B>() == std::any::TypeId::of::<R>() {
                1
            } else {
                MAX
            };
    }

    // Add more impls as needed.

    // reducer_list_insert_at is difficult to implement generically in Rust
    // without type-level programming or macros.  A macro approach would
    // likely be the most practical, but it's beyond the scope of a direct
    // translation.

    // `reducer_list_to_stack` also requires more advanced type manipulation,
    // specifically a right fold over the tuple to build the stack.  This
    // could also be implemented with a macro.

    pub trait NextReducerIs<Reducer> {
        const VALUE: bool;
    }

    impl<Reducer> NextReducerIs<Reducer> for () {
        const VALUE: bool = false;
    }

    impl<Next, Reducer> NextReducerIs<Reducer> for Next {
        default const VALUE: bool = false;
    }

    impl<Next> NextReducerIs<GenericReducerBase<()>> for GenericReducerBase<Next> {
        const VALUE: bool = true;
    }

    impl<Next> NextReducerIs<EmitProjectionReducer<()>> for EmitProjectionReducer<Next> {
        const VALUE: bool = true;
    }

     impl<Next> NextReducerIs<TSReducerBase<()>> for TSReducerBase<Next> {
        const VALUE: bool = true;
    }

    pub trait NextContainsReducer<Reducer> {
        const VALUE: bool;
    }

    impl<Reducer> NextContainsReducer<Reducer> for () {
        const VALUE: bool = false;
    }

    impl<R, T, Reducer> NextContainsReducer<Reducer> for R
        where R: PartialEq<Reducer> + NextContainsReducer<Reducer> + 'static,
              T: NextContainsReducer<Reducer> + 'static
    {
       default const VALUE: bool = false;
    }

     impl<R, T, Reducer> NextContainsReducer<Reducer> for R
        where R: PartialEq<Reducer> + 'static,
              T: NextContainsReducer<Reducer> + 'static
    {
       default const VALUE: bool = false;
    }

   impl<R, T, Reducer> NextContainsReducer<Reducer> for R<T>
    where R<T>: PartialEq<Reducer> + 'static,
          T: NextContainsReducer<Reducer> + 'static,
          Reducer: 'static
{
        const VALUE: bool = std::any::TypeId::of::<R<T>>() == std::any::TypeId::of::<Reducer>() ||
                            <T as NextContainsReducer<Reducer>>::VALUE;
    }

    pub trait NextIsBottomOfAssemblerStack {
        const VALUE: bool;
    }

    impl<Next> NextIsBottomOfAssemblerStack for Next {
        default const VALUE: bool = false;
    }

    impl<Next> NextIsBottomOfAssemblerStack for GenericReducerBase<Next> {
        const VALUE: bool = true;
    }

    impl<Next> NextIsBottomOfAssemblerStack for EmitProjectionReducer<Next> {
        const VALUE: bool = true;
    }

    impl<Next> NextIsBottomOfAssemblerStack for TSReducerBase<Next> {
        const VALUE: bool = true;
    }
}