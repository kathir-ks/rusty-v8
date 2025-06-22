// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod tmp {
    use std::{
        marker::PhantomData,
        cmp::PartialEq,
        limits::*,
    };

    /// Represents a type list.
    pub struct List<T>(PhantomData<T>);

    impl<T> List<T> {
        pub const fn new() -> Self {
            List(PhantomData)
        }
    }

    /// Represents a type list that holds 1-ary template types.
    pub struct List1<T>(PhantomData<T>);

    impl<T> List1<T> {
        pub const fn new() -> Self {
            List1(PhantomData)
        }
    }

    pub trait Length {
        const VALUE: usize;
    }

    impl<T> Length for List<T> {
        const VALUE: usize = 0;
    }

    impl<T> Length for List1<T> {
        const VALUE: usize = 0;
    }
    
    macro_rules! impl_length_for_list {
        ($($T:ident),*) => {
            impl<$($T,)*> Length for List<($($T,)*)> {
                const VALUE: usize = count_idents!($($T)*);
            }
        };
    }

    macro_rules! impl_length_for_list1 {
        ($($T:ident),*) => {
            impl<$($T,)*> Length for List1<($($T,)*)> {
                const VALUE: usize = count_idents!($($T)*);
            }
        };
    }

    macro_rules! count_idents {
        ($($T:ident)*) => {
            <[()]>::len(&[$(replace_expr!($T, ())),*])
        };
    }

    macro_rules! replace_expr {
        ($_t:tt, $sub:expr) => {$sub};
    }

    impl_length_for_list!();
    impl_length_for_list!(T1);
    impl_length_for_list!(T1, T2);
    impl_length_for_list!(T1, T2, T3);
    impl_length_for_list!(T1, T2, T3, T4);
    impl_length_for_list!(T1, T2, T3, T4, T5);
    impl_length_for_list!(T1, T2, T3, T4, T5, T6);
    impl_length_for_list!(T1, T2, T3, T4, T5, T6, T7);
    impl_length_for_list!(T1, T2, T3, T4, T5, T6, T7, T8);

    impl_length_for_list1!();
    impl_length_for_list1!(T1);
    impl_length_for_list1!(T1, T2);
    impl_length_for_list1!(T1, T2, T3);
    impl_length_for_list1!(T1, T2, T3, T4);
    impl_length_for_list1!(T1, T2, T3, T4, T5);
    impl_length_for_list1!(T1, T2, T3, T4, T5, T6);
    impl_length_for_list1!(T1, T2, T3, T4, T5, T6, T7);
    impl_length_for_list1!(T1, T2, T3, T4, T5, T6, T7, T8);

    pub type LengthV<List> = <List as Length>::VALUE;
    pub type Length1V<List1> = <List1 as Length>::VALUE;

    /// Trait for getting an element at a specific index in a type list.
    pub trait ElementAt<const INDEX: usize> {
        type Output;
    }

    impl<H, Tail> ElementAt<0> for List<(H, Tail)> {
        type Output = H;
    }

    macro_rules! impl_element_at_recursive {
        ($($idx:literal),*) => {
            $(
                impl<H, Tail, const INDEX: usize> ElementAt<{ $idx }> for List<(H, Tail)>
                where
                    Tail: ElementAt<{ INDEX - 1 }>,
                {
                    type Output = <Tail as ElementAt<{ INDEX - 1 }>>::Output;
                }
            )*
        };
    }

    impl_element_at_recursive!(1, 2, 3, 4, 5, 6, 7, 8);

    pub type ElementT<List, const Index: usize> = <List as ElementAt<Index>>::Output;

    /// Trait for mapping a type list with a given function.
    pub trait Map<F> {
        type Output;
    }

    impl<F> Map<F> for List<()> {
        type Output = List<()>;
    }

    // Helper trait to apply function F to type T
    pub trait Apply<F> {
        type Output;
    }

    impl<T, F: FnOnce(T) -> U, U> Apply<F> for T {
        type Output = F::Output;
    }

    macro_rules! impl_map_for_list {
        ($($T:ident),*) => {
            impl<F, $($T,)*> Map<F> for List<($($T,)*)>
            where
                $(F: Fn($T) -> $T,)*
            {
                type Output = List::<($(<$T as Apply<F>>::Output,)*)>;
            }
        };
    }

    impl_map_for_list!();
    impl_map_for_list!(T1);
    impl_map_for_list!(T1, T2);
    impl_map_for_list!(T1, T2, T3);
    impl_map_for_list!(T1, T2, T3, T4);
    impl_map_for_list!(T1, T2, T3, T4, T5);
    impl_map_for_list!(T1, T2, T3, T4, T5, T6);
    impl_map_for_list!(T1, T2, T3, T4, T5, T6, T7);
    impl_map_for_list!(T1, T2, T3, T4, T5, T6, T7, T8);

    pub type MapT<F, List> = <List as Map<F>>::Output;

    /// Trait for finding the index of a type in a type list.
    pub trait IndexOf<T, const OTHERWISE: usize> {
        const VALUE: usize;
    }

    impl<T, const OTHERWISE: usize> IndexOf<T, OTHERWISE> for List<()> {
        const VALUE: usize = OTHERWISE;
    }

    impl<T, Head, Tail, const OTHERWISE: usize> IndexOf<T, OTHERWISE> for List<(Head, Tail)>
    where
        Tail: IndexOf<T, { OTHERWISE }>,
    {
        const VALUE: usize = <Tail as IndexOf<T, { OTHERWISE }>>::VALUE + 1;
    }

    impl<T, Tail, const OTHERWISE: usize> IndexOf<T, OTHERWISE> for List<(T, Tail)> {
        const VALUE: usize = 0;
    }

    pub type IndexOfV<List, T, const OTHERWISE: usize> = <List as IndexOf<T, OTHERWISE>>::VALUE;

    pub trait IndexOf1<T, const OTHERWISE: usize> {
        const VALUE: usize;
    }

    impl<T, const OTHERWISE: usize> IndexOf1<T, OTHERWISE> for List1<()> {
        const VALUE: usize = OTHERWISE;
    }

    impl<T, Head, Tail, const OTHERWISE: usize> IndexOf1<T, OTHERWISE> for List1<(Head, Tail)>
    where
        Tail: IndexOf1<T, { OTHERWISE }>,
    {
        const VALUE: usize = <Tail as IndexOf1<T, { OTHERWISE }>>::VALUE + 1;
    }

    impl<T, Tail, const OTHERWISE: usize> IndexOf1<T, OTHERWISE> for List1<(T, Tail)> {
        const VALUE: usize = 0;
    }

    pub type IndexOf1V<List1, T, const OTHERWISE: usize> = <List1 as IndexOf1<T, OTHERWISE>>::VALUE;
    
    /// Trait for checking if a type list contains a specific type.
    pub trait Contains<T> {
        const VALUE: bool;
    }

    impl<T> Contains<T> for List<()> {
        const VALUE: bool = false;
    }

    impl<T, Head, Tail> Contains<T> for List<(Head, Tail)>
    where
        Head: PartialEq<T>,
        Tail: Contains<T>,
    {
        const VALUE: bool = Head == T || <Tail as Contains<T>>::VALUE;
    }

    pub type ContainsV<List, T> = <List as Contains<T>>::VALUE;

    pub trait Contains1<T> {
        const VALUE: bool;
    }

    impl<T> Contains1<T> for List1<()> {
        const VALUE: bool = false;
    }

    impl<T, Head, Tail> Contains1<T> for List1<(Head, Tail)>
    where
        Head: PartialEq<T>,
        Tail: Contains1<T>,
    {
        const VALUE: bool = Head == T || <Tail as Contains1<T>>::VALUE;
    }

    pub type Contains1V<List1, T> = <List1 as Contains1<T>>::VALUE;

    pub trait AllEqual<Cmp = Equals> {
        const VALUE: bool;
    }

    pub struct Equals;

    impl<T: PartialEq<U>, U> FnOnce<(T,U)> for Equals {
        type Output = bool;

        extern "rust-call" fn call_once(self, args: (T,U)) -> Self::Output {
            args.0 == args.1
        }
    }
    impl<T: PartialEq<U>, U> FnMut<(T,U)> for Equals {
        extern "rust-call" fn call_mut(&mut self, args: (T,U)) -> Self::Output {
            args.0 == args.1
        }
    }
    impl<T: PartialEq<U>, U> Fn<(T,U)> for Equals {
        extern "rust-call" fn call(&self, args: (T,U)) -> Self::Output {
            args.0 == args.1
        }
    }

    impl<Cmp> AllEqual<Cmp> for List<()> {
        const VALUE: bool = true;
    }

    impl<Head, Tail, Cmp> AllEqual<Cmp> for List<(Head, Tail)>
    where
        Cmp: Fn(Head, Tail) -> bool,
        Tail: AllEqual<Cmp>,
    {
        const VALUE: bool = <Tail as AllEqual<Cmp>>::VALUE;
    }

    pub type AllEqualV<List, Cmp> = <List as AllEqual<Cmp>>::VALUE;

    /// Trait for inserting a type at a specific index in a type list.
    pub trait InsertAt<const I: usize, T> {
        type Output;
    }

    impl<T, Head, Tail> InsertAt<0, T> for List<(Head, Tail)> {
        type Output = List<(T, Head, Tail)>;
    }

    impl<const I: usize, T, Head, Tail> InsertAt<I, T> for List<(Head, Tail)>
    where
        Tail: InsertAt<{ I - 1 }, T>,
    {
        type Output = List<(Head, <Tail as InsertAt<{ I - 1 }, T>>::Output)>;
    }

    pub type InsertAtT<List, const I: usize, T> = <List as InsertAt<I, T>>::Output;

    pub trait InsertAt1<const I: usize, T> {
        type Output;
    }

    impl<T, Head, Tail> InsertAt1<0, T> for List1<(Head, Tail)> {
        type Output = List1<(T, Head, Tail)>;
    }

    impl<const I: usize, T, Head, Tail> InsertAt1<I, T> for List1<(Head, Tail)>
    where
        Tail: InsertAt1<{ I - 1 }, T>,
    {
        type Output = List1<(Head, <Tail as InsertAt1<{ I - 1 }, T>>::Output)>;
    }

    pub type InsertAt1T<List1, const I: usize, T> = <List1 as InsertAt1<I, T>>::Output;

    /// Trait for folding a type list from the right.
    pub trait FoldRight<F, T> {
        type Output;
    }

    impl<F, T> FoldRight<F, T> for List<()> {
        type Output = T;
    }

    impl<F, T, Head, Tail> FoldRight<F, T> for List<(Head, Tail)>
    where
        F: FnOnce(Head, <Tail as FoldRight<F, T>>::Output) -> U,
        Tail: FoldRight<F, T>,
        U: Sized,
    {
        type Output = F::Output;
    }

    pub type FoldRightT<F, List, T> = <List as FoldRight<F, T>>::Output;

    pub trait FoldRight1<F, T> {
        type Output;
    }

    impl<F, T> FoldRight1<F, T> for List1<()> {
        type Output = T;
    }

    impl<F, T, Head, Tail> FoldRight1<F, T> for List1<(Head, Tail)>
    where
        F: FnOnce(Head, <Tail as FoldRight1<F, T>>::Output) -> U,
        Tail: FoldRight1<F, T>,
        U: Sized,
    {
        type Output = F::Output;
    }

    pub type FoldRight1T<F, List1, T> = <List1 as FoldRight1<F, T>>::Output;
}