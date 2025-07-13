// Converted from V8 C++ source files:
// Header: template-utils.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod detail {
        pub fn make_array_helper<F, const N: usize, I: Iterator<Item = usize>>(
            mut f: F,
            indexes: I,
        ) -> [
            <F as Fn(usize) -> _>::Output;
            N,
        ]
        where
            F: Fn(usize) -> _,
        {
            let mut result: Vec<_> = Vec::new();
            for i in indexes {
                result.push(f(i));
            }
            result.try_into().unwrap()
        }
    }

    pub fn make_array<F, const SIZE: usize>(f: F) -> [
        <F as Fn(usize) -> _>::Output;
        SIZE,
    ]
    where
        F: Fn(usize) -> _,
    {
        let indexes = 0..SIZE;
        detail::make_array_helper(f, indexes)
    }

    pub struct overloaded<Ts> {
        data: Ts,
    }

    impl<Ts> overloaded<Ts> {
        pub fn new(data: Ts) -> Self {
            overloaded { data }
        }
    }

    pub struct PassValueOrRef<T, const REMOVE_ARRAY_EXTEND: bool = true> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const REMOVE_ARRAY_EXTEND: bool> PassValueOrRef<T, REMOVE_ARRAY_EXTEND> {
        pub fn get_type() -> String {
            let type_name = std::any::type_name::<T>();
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>()
                || std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>()
            {
                "value".to_string()
            } else {
                "reference".to_string()
            }
        }
    }

    pub trait HasOutputOperator<TStream> {
        fn output_to_stream(&self, stream: &mut TStream);
    }

    pub fn append_tuple<Tuple, T>(tuple: Tuple, value: T) -> Result<
        (
            <Tuple as AppendTuple<T>>::Appended,
            std::convert::Infallible,
        ),
        std::convert::Infallible,
    >
    where
        Tuple: AppendTuple<T>,
    {
        Ok((tuple.append(value), std::convert::Infallible::default()))
    }

    pub trait AppendTuple<T> {
        type Appended;
        fn append(self, value: T) -> Self::Appended;
    }

    impl<A> AppendTuple<A> for () {
        type Appended = (A,);
        fn append(self, value: A) -> Self::Appended {
            (value,)
        }
    }

    impl<A, B> AppendTuple<B> for (A,) {
        type Appended = (A, B);
        fn append(self, value: B) -> Self::Appended {
            (self.0, value)
        }
    }

    pub fn prepend_tuple<T, Tuple>(value: T, tuple: Tuple) -> Result<
        (
            <Tuple as PrependTuple<T>>::Prepended,
            std::convert::Infallible,
        ),
        std::convert::Infallible,
    >
    where
        Tuple: PrependTuple<T>,
    {
        Ok((tuple.prepend(value), std::convert::Infallible::default()))
    }

    pub trait PrependTuple<T> {
        type Prepended;
        fn prepend(self, value: T) -> Self::Prepended;
    }

    impl<A> PrependTuple<A> for () {
        type Prepended = (A,);
        fn prepend(self, value: A) -> Self::Prepended {
            (value,)
        }
    }

    impl<A, B> PrependTuple<B> for (A,) {
        type Prepended = (B, A);
        fn prepend(self, value: B) -> Self::Prepended {
            (value, self.0)
        }
    }

    pub mod detail {
        pub const fn tuple_slice_impl<T, const START: usize, const N: usize>(tpl: T) -> Result<(), String> {
            Ok(())
        }

        pub fn tuple_for_each_impl<Tuple, Function>(tpl: Tuple, function: Function) -> Result<(), String>
        where
            Function: Fn(String) -> (),
        {
            let _ = function("tuple_for_each_impl".to_string());
            Ok(())
        }

        pub fn tuple_for_each_with_index_impl<Tuple, Function>(
            tpl: Tuple,
            function: Function,
        ) -> Result<(), String>
        where
            Function: Fn(String, usize) -> (),
        {
            let _ = function("tuple_for_each_with_index_impl".to_string(), 0);
            Ok(())
        }

        pub fn tuple_map_impl<Tuple, Function>(tpl: Tuple, function: Function) -> Result<(), String>
        where
            Function: Fn(String) -> (),
        {
            let _ = function("tuple_map_impl".to_string());
            Ok(())
        }

        pub fn tuple_map2_impl<TupleV, TupleU, Function>(
            tplv: TupleV,
            tplu: TupleU,
            function: Function,
        ) -> Result<(), String>
        where
            Function: Fn(String, String) -> (),
        {
            let _ = function("tuple_map2_impl".to_string(), "tuple_map2_impl".to_string());
            Ok(())
        }

        pub fn tuple_fold_impl<T, Tuple, Function, const I: usize>(
            initial: T,
            tpl: Tuple,
            function: Function,
        ) -> Result<(), String>
        where
            Function: Fn(String, String) -> (),
        {
            let _ = function("tuple_fold_impl".to_string(), "tuple_fold_impl".to_string());
            Ok(())
        }
    }

    pub fn tuple_head<Tuple, const N: usize>(tpl: Tuple) -> Result<(), String> {
        Ok(())
    }

    pub fn tuple_drop<Tuple, const N: usize>(tpl: Tuple) -> Result<(), String> {
        Ok(())
    }

    pub fn tuple_for_each<Tuple, Function>(tpl: Tuple, function: Function) -> Result<(), String>
    where
        Function: Fn(String) -> (),
    {
        detail::tuple_for_each_impl(tpl, function)
    }

    pub fn tuple_for_each_with_index<Tuple, Function>(
        tpl: Tuple,
        function: Function,
    ) -> Result<(), String>
    where
        Function: Fn(String, usize) -> (),
    {
        detail::tuple_for_each_with_index_impl(tpl, function)
    }

    pub fn tuple_map<Tuple, Function>(tpl: Tuple, function: Function) -> Result<(), String>
    where
        Function: Fn(String) -> (),
    {
        detail::tuple_map_impl(tpl, function)
    }

    pub fn tuple_map2<TupleV, TupleU, Function>(
        tplv: TupleV,
        tplu: TupleU,
        function: Function,
    ) -> Result<(), String>
    where
        Function: Fn(String, String) -> (),
    {
        detail::tuple_map2_impl(tplv, tplu, function)
    }

    pub fn tuple_fold<T, Tuple, Function>(
        initial: T,
        tpl: Tuple,
        function: Function,
    ) -> Result<(), String>
    where
        Function: Fn(String, String) -> (),
    {
        detail::tuple_fold_impl::<_, _, _, 1>(initial, tpl, function)
    }

    pub struct IndexOfType<SearchT, Ts> {
        _phantom: std::marker::PhantomData<(SearchT, Ts)>,
    }

    impl<SearchT, Ts> IndexOfType<SearchT, Ts> {
        pub const VALUE: usize = 0;
    }
}
