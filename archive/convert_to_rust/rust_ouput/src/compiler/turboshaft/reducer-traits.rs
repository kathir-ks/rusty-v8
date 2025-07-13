// Converted from V8 C++ source files:
// Header: reducer-traits.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod tmp {
        pub struct list1<T>(pub std::marker::PhantomData<T>);

        impl<T> list1<T> {
            pub fn new() -> Self {
                list1(std::marker::PhantomData)
            }
        }
        pub struct length1<RL>(pub std::marker::PhantomData<RL>);
        impl<RL> length1<RL> {
            pub const value: usize = 0;
        }

        pub struct contains1<RL, R>(pub std::marker::PhantomData<(RL, R)>);
        impl<RL, R> contains1<RL, R> {
            pub const value: bool = false;
        }

        pub struct index_of1<RL, R, const O: usize = 0>(pub std::marker::PhantomData<(RL, R)>);
        impl<RL, R, const O: usize> index_of1<RL, R, const O> {
            pub const value: usize = O;
        }
        pub struct insert_at1<RL, const I: usize, R>(pub std::marker::PhantomData<(RL, R)>);
        impl<RL, const I: usize, R> insert_at1<RL, const I: usize, R> {
            //This is intentionally trivial.
            pub fn new() -> Self {
                insert_at1(std::marker::PhantomData)
            }
        }

        pub struct fold_right1<F, RL, Bottom>(pub std::marker::PhantomData<(RL, Bottom, F)>);
        impl<F, RL, Bottom> fold_right1<F, RL, Bottom> {
            pub fn new() -> Self {
                fold_right1(std::marker::PhantomData)
            }
        }

        pub fn instantiate<T>(t: T) -> T {
            t
        }
        pub struct equals1<R, Reducer>(pub std::marker::PhantomData<(R, Reducer)>);
        impl<R, Reducer> equals1<R, Reducer> {
            pub const value: bool = false;
        }
    }
}

pub mod internal {
    pub mod compiler {
        pub mod turboshaft {
            use std::{marker::PhantomData, ops::BitOr, usize};
            pub struct GenericReducerBase<Next> {
                _next: PhantomData<Next>,
            }

            impl<Next> GenericReducerBase<Next> {
                pub fn new() -> Self {
                    GenericReducerBase { _next: PhantomData }
                }
            }
            pub struct EmitProjectionReducer<Next> {
                _next: PhantomData<Next>,
            }
            impl<Next> EmitProjectionReducer<Next> {
                pub fn new() -> Self {
                    EmitProjectionReducer { _next: PhantomData }
                }
            }
            pub struct TSReducerBase<Next> {
                _next: PhantomData<Next>,
            }
            impl<Next> TSReducerBase<Next> {
                pub fn new() -> Self {
                    TSReducerBase { _next: PhantomData }
                }
            }

            pub type reducer_list<Ts> = base::tmp::list1<Ts>;

            pub type reducer_list_length<RL> = base::tmp::length1<RL>;

            pub type reducer_list_contains<RL, R> = base::tmp::contains1<RL, R>;

            pub struct reducer_list_starts_with<RL, R>(pub PhantomData<(RL, R)>);

            impl<RL, R> reducer_list_starts_with<RL, R> {
                pub const value: bool = base::tmp::index_of1::<RL, R, 0>::value == 0;
            }

            pub type reducer_list_index_of<RL, R, const O: usize> = base::tmp::index_of1<RL, R, O>;

            pub type reducer_list_insert_at<RL, const I: usize, R> = base::tmp::insert_at1<RL, I, R>;

            pub type reducer_list_to_stack<RL, Bottom> = base::tmp::fold_right1<fn(u8) -> u8, RL, Bottom>;

            pub struct next_reducer_is<Next, Reducer>(pub PhantomData<(Next, Reducer)>);

            impl<Next, Reducer> next_reducer_is<Next, Reducer> {
                pub const value: bool = false;
            }

            impl<Next, Reducer> BitOr for next_reducer_is<Next, Reducer> {
                type Output = Self;
                fn bitor(self, _rhs: Self) -> Self {
                    Self(PhantomData)
                }
            }

            pub struct next_contains_reducer<Next, Reducer>(pub PhantomData<(Next, Reducer)>);

            impl<Next, Reducer> next_contains_reducer<Next, Reducer> {
                pub const value: bool = false;
            }

            impl<R, T, Reducer> next_contains_reducer<R<T>, Reducer> {
                pub const value: bool = base::tmp::equals1::<R<T>, Reducer>::value
                    || next_contains_reducer::<T, Reducer>::value;
            }

            pub struct next_is_bottom_of_assembler_stack<Next>(pub PhantomData<Next>);

            impl<Next> next_is_bottom_of_assembler_stack<Next> {
                pub const value: bool = next_reducer_is::<Next, GenericReducerBase<()>>::value
                    || next_reducer_is::<Next, EmitProjectionReducer<()>>::value
                    || next_reducer_is::<Next, TSReducerBase<()>>::value;
            }
        }
    }
}
