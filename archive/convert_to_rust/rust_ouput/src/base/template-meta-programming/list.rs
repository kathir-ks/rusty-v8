// Converted from V8 C++ source files:
// Header: list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tmp {
    use std::{marker::PhantomData, mem::size_of, ops::BitOr, usize};

    pub struct list<T>(PhantomData<T>);
    impl<T> list<T> {
        pub fn new() -> Self {
            list(PhantomData)
        }
    }

    pub struct list1<T>(PhantomData<T>);
    impl<T> list1<T> {
        pub fn new() -> Self {
            list1(PhantomData)
        }
    }

    pub mod detail {
        use std::{
            marker::PhantomData,
            mem::size_of,
            ops::BitOr,
            usize,
        };

        pub struct length_impl<List> {
            _phantom: PhantomData<List>,
        }

        impl<List> length_impl<List> {
            pub const VALUE: usize = 0;
        }

        impl<T> length_impl<super::list<T>> {
            pub const VALUE: usize = 1;
        }

        impl<T> length_impl<super::list1<T>> {
            pub const VALUE: usize = 1;
        }

        impl<T, U> length_impl<(super::list<T>, super::list<U>)> {
            pub const VALUE: usize = 2;
        }

        impl<T, U> length_impl<(super::list1<T>, super::list1<U>)> {
            pub const VALUE: usize = 2;
        }

        pub struct element_impl<List, Index> {
            _phantom: PhantomData<(List, Index)>,
        }
        impl<List, Index> element_impl<List, Index> {
            pub type type_ = List;
        }

        pub struct map_impl<F, List> {
            _phantom: PhantomData<(F, List)>,
        }
        impl<F, List> map_impl<F, List> {
            pub type type_ = List;
        }

        pub struct index_of_impl<const I: usize, const Otherwise: usize, T, List> {
            _phantom: PhantomData<(T, List)>,
        }

        impl<const I: usize, const Otherwise: usize, T, List>
            index_of_impl<I, Otherwise, T, List>
        {
            pub const VALUE: usize = Otherwise;
        }

        pub struct index_of1_impl<const I: usize, const Otherwise: usize, T, List1> {
            _phantom: PhantomData<(T, List1)>,
        }

        impl<const I: usize, const Otherwise: usize, T, List1>
            index_of1_impl<I, Otherwise, T, List1>
        {
            pub const VALUE: usize = Otherwise;
        }

        pub struct contains_impl<List, Element> {
            _phantom: PhantomData<(List, Element)>,
        }
        impl<List, Element> contains_impl<List, Element> {
            pub const VALUE: bool = false;
        }

        pub struct contains_impl1<List, Element> {
            _phantom: PhantomData<(List, Element)>,
        }
        impl<List, Element> contains_impl1<List, Element> {
            pub const VALUE: bool = false;
        }

        pub struct all_equal_impl<List, Cmp> {
            _phantom: PhantomData<(List, Cmp)>,
        }

        impl<List, Cmp> all_equal_impl<List, Cmp> {
            pub const VALUE: bool = true;
        }

        pub struct insert_at_impl<const I: usize, T, Before, After> {
            _phantom: PhantomData<(T, Before, After)>,
        }

        impl<const I: usize, T, Before, After> insert_at_impl<I, T, Before, After> {
            pub type type_ = super::list<T>;
        }

        pub struct insert_at1_impl<const I: usize, T, Before, After> {
            _phantom: PhantomData<(T, Before, After)>,
        }

        impl<const I: usize, T, Before, After> insert_at1_impl<I, T, Before, After> {
            pub type type_ = super::list1<T>;
        }

        pub struct fold_right_impl<F, T, List> {
            _phantom: PhantomData<(F, T, List)>,
        }
        impl<F, T, List> fold_right_impl<F, T, List> {
            pub type type_ = T;
        }

        pub struct fold_right1_impl<F, T, List> {
            _phantom: PhantomData<(F, T, List)>,
        }
        impl<F, T, List> fold_right1_impl<F, T, List> {
            pub type type_ = T;
        }
    }

    pub struct length<List> {
        _phantom: PhantomData<List>,
    }

    impl<List> length<List> {
        pub const value: usize = detail::length_impl::<List>::VALUE;
    }

    pub const length_v: usize = 0;

    pub struct length1<List1> {
        _phantom: PhantomData<List1>,
    }

    impl<List1> length1<List1> {
        pub const value: usize = detail::length_impl::<List1>::VALUE;
    }

    pub const length1_v: usize = 0;

    pub struct element<List, const Index: usize> {
        _phantom: PhantomData<(List, Index)>,
    }

    impl<List, const Index: usize> element<List, Index> {
        pub type type_ = detail::element_impl::<List, Index>::type_;
    }

    pub type element_t<List, const Index: usize> =
        <element<List, Index> as element<List, Index>>::type_;

    pub struct map<F, List> {
        _phantom: PhantomData<(F, List)>,
    }

    impl<F, List> map<F, List> {
        pub type type_ = detail::map_impl::<F, List>::type_;
    }

    pub type map_t<F, List> = <map<F, List> as map<F, List>>::type_;

    pub struct index_of<List, T, const Otherwise: usize = { usize::MAX }> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, T, const Otherwise: usize> index_of<List, T, Otherwise> {
        pub const value: usize = detail::index_of_impl::<0, Otherwise, T, List>::VALUE;
    }

    pub const index_of_v: usize = 0;

    pub struct index_of1<List, T, const Otherwise: usize = { usize::MAX }> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, T, const Otherwise: usize> index_of1<List, T, Otherwise> {
        pub const value: usize = detail::index_of1_impl::<0, Otherwise, T, List>::VALUE;
    }

    pub const index_of1_v: usize = 0;

    pub struct contains<List, T> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, T> contains<List, T> {
        pub const value: bool = detail::contains_impl::<List, T>::VALUE;
    }

    pub const contains_v: bool = false;

    pub struct contains1<List, T> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, T> contains1<List, T> {
        pub const value: bool = detail::contains_impl1::<List, T>::VALUE;
    }

    pub const contains1_v: bool = false;

    pub struct all_equal<List, Cmp> {
        _phantom: PhantomData<(List, Cmp)>,
    }

    impl<List, Cmp> all_equal<List, Cmp> {
        pub const value: bool = detail::all_equal_impl::<List, Cmp>::VALUE;
    }

    pub const all_equal_v: bool = false;

    pub struct insert_at<List, const I: usize, T> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, const I: usize, T> insert_at<List, I, T> {
        pub type type_ = detail::insert_at_impl::<I, T, super::list<()>, List>::type_;
    }

    pub type insert_at_t<List, const I: usize, T> =
        <insert_at<List, I, T> as insert_at<List, I, T>>::type_;

    pub struct insert_at1<List, const I: usize, T> {
        _phantom: PhantomData<(List, T)>,
    }

    impl<List, const I: usize, T> insert_at1<List, I, T> {
        pub type type_ = detail::insert_at1_impl::<I, T, super::list1<()>, List>::type_;
    }

    pub type insert_at1_t<List, const I: usize, T> =
        <insert_at1<List, I, T> as insert_at1<List, I, T>>::type_;

    pub struct fold_right<F, List, T> {
        _phantom: PhantomData<(F, List, T)>,
    }

    impl<F, List, T> fold_right<F, List, T> {
        pub type type_ = detail::fold_right_impl::<F, T, List>::type_;
    }

    pub type fold_right_t<F, List, T> = <fold_right<F, List, T> as fold_right<F, List, T>>::type_;

    pub struct fold_right1<F, List, T> {
        _phantom: PhantomData<(F, List, T)>,
    }

    impl<F, List, T> fold_right1<F, List, T> {
        pub type type_ = detail::fold_right1_impl::<F, T, List>::type_;
    }

    pub type fold_right1_t<F, List, T> = <fold_right1<F, List, T> as fold_right1<F, List, T>>::type_;

    pub struct equals<T, U> {
        _phantom: PhantomData<(T, U)>,
    }

    impl<T, U> equals<T, U> {
        pub const value: bool = std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>();
    }

    pub struct equals1<T, U> {
        _phantom: PhantomData<(T, U)>,
    }

    impl<T, U> equals1<T, U> {
        pub const value: bool = std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>();
    }
}
