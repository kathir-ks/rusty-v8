// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod tmp {

    /// A trait to represent a compile-time boolean constant.
    pub trait BoolConstant {
        const VALUE: bool;
    }

    /// Implements `BoolConstant` for a given boolean value.
    macro_rules! impl_bool_constant {
        ($name:ident, $value:expr) => {
            pub struct $name;

            impl BoolConstant for $name {
                const VALUE: bool = $value;
            }
        };
    }

    impl_bool_constant!(False, false);
    impl_bool_constant!(True, true);

    /// Trait to check equality of types
    pub trait Equals<U> : BoolConstant {}

    impl<T, U> Equals<U> for T where False: BoolConstant {}

    impl<T> Equals<T> for T where True: BoolConstant {}

    pub trait Equals1<U> : BoolConstant {}

    impl<T, U> Equals1<U> for T where False: BoolConstant {}

    impl<T> Equals1<T> for T where True: BoolConstant {}


    /// A struct to instantiate a template.
    pub struct Instantiate<T, U> {
        _phantom: std::marker::PhantomData<(T, U)>,
    }

    impl<T, U> Instantiate<T, U> {
        pub type Output = T;
    }

    /// A trait to check if a type is an instantiation of another type.
    pub trait IsInstantiationOf<T> : BoolConstant {}

    impl<I, T> IsInstantiationOf<T> for I where False: BoolConstant {}

    impl<U, T> IsInstantiationOf<T> for T where True: BoolConstant {}
}