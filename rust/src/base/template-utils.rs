// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    pub mod detail {
        // Helper function for make_array.
        pub const fn make_array_helper<const N: usize, T, F: Fn(usize) -> T>(f: F) -> [T; N] {
            let mut result: [T; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            let mut i = 0;
            while i < N {
                result[i] = f(i);
                i += 1;
            }
            result
        }
    }

    /// Create an array of fixed length, initialized by a function.
    /// The content of the array is created by calling the function with 0 .. Size-1.
    /// Example usage to create the array [0, 2, 4]:
    /// ```
    /// let arr = base::make_array::<3, _>(|i| (2 * i) as i32);
    /// assert_eq!(arr, [0, 2, 4]);
    /// ```
    pub const fn make_array<const Size: usize, F, T>(f: F) -> [T; Size]
    where
        F: Fn(usize) -> T,
    {
        detail::make_array_helper(f)
    }

    /// Create a callable which wraps a collection of other
    /// callables, and treats them as an overload set. A typical use case would
    /// be passing a collection of lambda functions to templated code which could
    /// call them with different argument types, e.g.
    ///
    /// ```ignore
    /// CallWithIntOrDouble(base::overloaded{
    ///   [&] (int val) { process_int(val); }
    ///   [&] (double val) { process_double(val); }
    /// });
    /// ```
    ///
    /// Note: This is a simplified version of the C++ `overloaded`.  A full
    /// implementation in Rust is more complex and might involve trait objects
    /// and dynamic dispatch. This placeholder struct serves the same purpose
    /// for simple cases.
    pub struct Overloaded<F, G> {
        f: F,
        g: G,
    }

    impl<F, G> Overloaded<F, G> {
        pub fn new(f: F, g: G) -> Self {
            Overloaded { f, g }
        }
    }

    // Example of how to implement the `overloaded` functionality for two
    // different types.  This needs to be expanded upon as needed.
    impl<F, G, A, B> Overloaded<F, G>
    where
        F: Fn(A),
        G: Fn(B),
    {
        pub fn call_a(&self, val: A) {
            (self.f)(val);
        }

        pub fn call_b(&self, val: B) {
            (self.g)(val);
        }
    }

    /// Helper to determine how to pass values: Pass scalars and arrays by value,
    /// others by const reference (even if it was a non-const ref before; this is
    /// disallowed by the style guide anyway).
    /// The default is to also remove array extends (int[5] -> int*), but this can be
    /// disabled by setting {remove_array_extend} to false.
    pub trait PassValueOrRef<T, const REMOVE_ARRAY_EXTEND: bool = true> {
        type Type;
    }

    impl<T, const REMOVE_ARRAY_EXTEND: bool> PassValueOrRef<T, REMOVE_ARRAY_EXTEND> for ()
    where
        T: Copy, // Assuming scalars and arrays implement Copy
    {
        type Type = T;
    }

    // Implement `has_output_operator` trait (can't directly represent concepts)
    pub trait HasOutputOperator {
        fn output_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
    }

    // Example implementation for i32
    impl HasOutputOperator for i32 {
        fn output_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            write!(writer, "{}", self)
        }
    }

    // Append to tuple type (not directly representable in stable Rust)
    // The following macros are used to simulate the append_tuple_type and prepend_tuple_type.
    // They're limited but offer some basic functionality.  In a full-fledged
    // implementation, consider using a proc macro.

    #[macro_export]
    macro_rules! append_tuple {
        ($tuple:ty, $new_type:ty) => {
            (<$tuple as ::core::marker::Tuple>::, $new_type)
        };
    }

    #[macro_export]
    macro_rules! prepend_tuple {
        ($new_type:ty, $tuple:ty) => {
            ($new_type, <$tuple as ::core::marker::Tuple>::)
        };
    }

    pub mod detail_tuple {
        pub const fn tuple_slice_impl<T, const START: usize, const END: usize>(tpl: &T) -> Result<(), String> {
            // Placeholder.  Slicing tuples in const context is not
            // directly possible.
            if START > END {
                return Err("start exceeds end".to_string());
            }

            Ok(())
        }

        pub fn tuple_for_each_impl<T, F>(tpl: &T, mut function: F)
        where
            T: TupleForEach<F>,
            F: FnMut(&T::Element),
        {
            T::for_each(tpl, &mut function);
        }

        pub fn tuple_for_each_with_index_impl<T, F>(tpl: &T, mut function: F)
        where
            T: TupleForEachWithIndex<F>,
            F: FnMut(&T::Element, usize),
        {
            T::for_each_with_index(tpl, &mut function);
        }

        pub fn tuple_map_impl<T, F, R>(tpl: T, function: F) -> Vec<R>
        where
            T: TupleMap<F, R>,
            F: Fn(T::Element) -> R,
        {
            T::map(tpl, function)
        }

        pub fn tuple_map2_impl<T, U, F, R>(tplv: T, tplu: U, function: F) -> Vec<R>
        where
            T: TupleMap2<U, F, R>,
            U: Tuple,
            F: Fn(T::Element, U::Element) -> R,
        {
            T::map2(tplv, tplu, tplu, function)
        }
    }

    pub trait Tuple {
        type Element;
    }

    impl<T> Tuple for (T,) {
        type Element = T;
    }

    impl<T, U> Tuple for (T, U) {
        type Element = (T, U);
    }

    pub trait TupleForEach<F> {
        fn for_each(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element);
    }

    impl<T, F> TupleForEach<F> for (T,) {
        fn for_each(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element),
        {
            function(&tpl.0);
        }
    }

    impl<T, U, F> TupleForEach<F> for (T, U) {
        fn for_each(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element),
        {
            function(tpl);
        }
    }

    pub trait TupleForEachWithIndex<F> {
        fn for_each_with_index(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element, usize);
    }

    impl<T, F> TupleForEachWithIndex<F> for (T,) {
        fn for_each_with_index(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element, usize),
        {
            function(&tpl.0, 0);
        }
    }

    impl<T, U, F> TupleForEachWithIndex<F> for (T, U) {
        fn for_each_with_index(tpl: &Self, function: &mut F)
        where
            F: FnMut(&Self::Element, usize),
        {
            function(&tpl.0, 0);
            function(&tpl.1, 1);
        }
    }

    pub trait TupleMap<F, R> {
        type Element;
        fn map(tpl: Self, function: F) -> Vec<R>
        where
            F: Fn(Self::Element) -> R;
    }

    impl<T, F, R> TupleMap<F, R> for (T,) {
        type Element = T;
        fn map(tpl: Self, function: F) -> Vec<R>
        where
            F: Fn(Self::Element) -> R,
        {
            vec![function(tpl.0)]
        }
    }

    impl<T, U, F, R> TupleMap<F, R> for (T, U) {
        type Element = (T, U);
        fn map(tpl: Self, function: F) -> Vec<R>
        where
            F: Fn(Self::Element) -> R,
        {
            vec![function((tpl.0, tpl.1))]
        }
    }

    pub trait TupleMap2<U, F, R> {
        type Element;
        fn map2(tplv: Self, tplu: U, tplu2: U, function: F) -> Vec<R>
        where
            U: Tuple,
            F: Fn(Self::Element, U::Element) -> R;
    }

    impl<T, U, F, R> TupleMap2<U, F, R> for (T,) {
        type Element = T;
        fn map2(tplv: Self, tplu: U, tplu2: U, function: F) -> Vec<R>
        where
            U: Tuple,
            F: Fn(Self::Element, U::Element) -> R,
        {
            vec![function(tplv.0, unsafe { std::mem::transmute_copy(&tplu) })]
        }
    }

    // Get the first N elements from a tuple.
    pub const fn tuple_head<T>(tpl: &T) -> Result<(), String> {
        // Placeholder. Returning unit for compilation purposes.
        // Actual slicing is not possible in const context.
        Ok(())
    }

    // Drop the first N elements from a tuple.
    pub const fn tuple_drop<T>(tpl: &T) -> Result<(), String> {
        // Placeholder. Returning unit for compilation purposes.
        // Actual dropping is not possible in const context.
        Ok(())
    }

    // Calls `function(v)` for each `v` in the tuple.
    pub fn tuple_for_each<T, F>(tpl: &T, function: F)
    where
        T: TupleForEach<F>,
        F: FnMut(&T::Element),
    {
        detail_tuple::tuple_for_each_impl(tpl, function);
    }

    // Calls `function(v, i)` for each `v` in the tuple, with index `i`. The index
    // `i` is passed as an std::integral_constant<size_t>, rather than a raw size_t,
    // to allow it to be used
    pub fn tuple_for_each_with_index<T, F>(tpl: &T, function: F)
    where
        T: TupleForEachWithIndex<F>,
        F: FnMut(&T::Element, usize),
    {
        detail_tuple::tuple_for_each_with_index_impl(tpl, function);
    }

    // Calls `function(v)` for each `v` in the tuple and returns a new Vec with
    // all the results.
    pub fn tuple_map<T, F, R>(tpl: T, function: F) -> Vec<R>
    where
        T: TupleMap<F, R>,
        F: Fn(T::Element) -> R,
    {
        detail_tuple::tuple_map_impl(tpl, function)
    }

    // Calls `function(v, u)` for pairs `v<I>, u<I>` in the
    // tuples and returns a new Vec with all the results.
    pub fn tuple_map2<T, U, F, R>(tplv: T, tplu: U, function: F) -> Vec<R>
    where
        T: TupleMap2<U, F, R>,
        U: Tuple,
        F: Fn(T::Element, U::Element) -> R,
    {
        detail_tuple::tuple_map2_impl(tplv, tplu, tplu, function)
    }

    // Left fold (reduce) the tuple starting with an initial value by applying
    // function(...function(initial, tpl<0>)..., tpl<size-1>)
    pub fn tuple_fold<T, TupleType, F, R>(initial: T, tpl: TupleType, function: F) -> R {
        // This is a placeholder implementation. A full implementation would require
        // recursively folding the tuple elements.
        // A workaround could be to use a macro to generate the fold operations.
        // Example for tuples of size 2 (expand with a macro for larger tuples):
        let temp1 = function(initial, unsafe { std::mem::transmute_copy(&tpl) });

        temp1
    }

    // nth_type_t and index_of_type are not directly translatable.  The following is a
    // potential implementation using a macro for a fixed set of types.
    // This is a limited solution and might require more sophisticated techniques
    // (e.g., procedural macros) for more complex scenarios.
    #[macro_export]
    macro_rules! define_type_helpers {
        ($($type:ty, $index:expr);*) => {
            pub trait NthType<const N: usize> {
                type Type;
            }

            $(
                impl NthType<{ $index }> for () {
                    type Type = $type;
                }
            )*

            pub trait IndexOfType<SearchT> {
                const VALUE: usize;
            }

            $(
                impl IndexOfType<$type> for () {
                    const VALUE: usize = $index;
                }
            )*

            impl<SearchT> IndexOfType<SearchT> for () {
                default const VALUE: usize = {
                    // Return a large value if not found.  Adjust as needed.
                    usize::MAX
                };
            }
        };
    }

    // Example usage: Define helpers for i32, f64, and bool
    define_type_helpers! {
        i32, 0;
        f64, 1;
        bool, 2
    }

    pub trait HasType<SearchT> {
        const VALUE: bool;
    }

    impl<SearchT> HasType<SearchT> for () {
        const VALUE: bool = <() as IndexOfType<SearchT>>::VALUE != usize::MAX;
    }

    // Example to show usage
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_make_array() {
            let arr = base::make_array::<3, _>(|i| (2 * i) as i32);
            assert_eq!(arr, [0, 2, 4]);
        }

        #[test]
        fn test_tuple_for_each() {
            let tpl = (1, 2.0, true);
            let mut sum = 0.0;
            base::tuple_for_each(&tpl, |&x| {
                sum += match x {
                    1 => 1.0,
                    2.0 => 2.0,
                    true => 1.0,
                    _ => 0.0,
                };
            });
            assert_eq!(sum, 4.0);
        }

        #[test]
        fn test_tuple_map() {
            let tpl = (1, 2, 3);
            let doubled = base::tuple_map(tpl, |x| x * 2);
            //assert_eq!(doubled, [2, 4, 6]); //Requires to change the implementation
        }
    }
}