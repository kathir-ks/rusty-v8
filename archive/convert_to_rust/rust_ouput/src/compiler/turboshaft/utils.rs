// Converted from V8 C++ source files:
// Header: utils.h
// Implementation: utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    pub use std::fmt;
    use std::ops::Deref;
    use std::{
        any::Any,
        fmt::{Debug, Display},
        sync::atomic::{AtomicU64, Ordering},
    };

    use crate::base;
    pub struct V8 {}

    pub struct any_of<T> {
        data: Vec<T>,
    }

    impl<T> any_of<T> {
        pub fn new(data: Vec<T>) -> Self {
            any_of { data }
        }

        pub fn contains<U>(&self, value: &U) -> bool
        where
            T: PartialEq<U>,
        {
            self.data.iter().any(|x| x == value)
        }
    }

    impl<T> From<Vec<T>> for any_of<T> {
        fn from(data: Vec<T>) -> Self {
            any_of { data }
        }
    }

    impl<T> PartialEq<T> for any_of<T>
    where
        T: PartialEq,
    {
        fn eq(&self, value: &T) -> bool {
            self.contains(value)
        }
    }

    impl<T: Debug> fmt::Display for any_of<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut iter = self.data.iter();
            write!(f, "any_of(")?;
            if let Some(first) = iter.next() {
                write!(f, "{:?}", first)?;
                for elem in iter {
                    write!(f, ", {:?}", elem)?;
                }
            }
            write!(f, ")")
        }
    }

    pub struct all_of<T> {
        data: Vec<T>,
    }

    impl<T> all_of<T> {
        pub fn new(data: Vec<T>) -> Self {
            all_of { data }
        }

        pub fn all_equal_to<U>(&self, value: &U) -> bool
        where
            T: PartialEq<U>,
        {
            self.data.iter().all(|x| x == value)
        }
    }

    impl<T> From<Vec<T>> for all_of<T> {
        fn from(data: Vec<T>) -> Self {
            all_of { data }
        }
    }

    impl<T> PartialEq<T> for all_of<T>
    where
        T: PartialEq,
    {
        fn eq(&self, value: &T) -> bool {
            self.all_equal_to(value)
        }
    }

    impl<T: Debug> fmt::Display for all_of<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut iter = self.data.iter();
            write!(f, "all_of(")?;
            if let Some(first) = iter.next() {
                write!(f, "{:?}", first)?;
                for elem in iter {
                    write!(f, ", {:?}", elem)?;
                }
            }
            write!(f, ")")
        }
    }

    #[cfg(debug_assertions)]
    pub fn should_skip_optimization_step() -> bool {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let current = COUNTER.fetch_add(1, Ordering::SeqCst);
        if current == v8_flags::turboshaft_opt_bisect_break {
            std::debugger::breakpoint();
        }
        if current >= v8_flags::turboshaft_opt_bisect_limit {
            return true;
        }
        false
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    pub fn should_skip_optimization_step() -> bool {
        false
    }

    pub struct ScopedModification<'a, T> {
        ptr: &'a mut T,
        old_value: T,
    }

    impl<'a, T> ScopedModification<'a, T> {
        pub fn new(ptr: &'a mut T, new_value: T) -> Self
        where
            T: Clone,
        {
            let old_value = ptr.clone();
            *ptr = new_value;
            ScopedModification { ptr, old_value }
        }

        pub fn old_value(&self) -> &T {
            &self.old_value
        }
    }

    impl<'a, T> Drop for ScopedModification<'a, T> {
        fn drop(&mut self) {
            *self.ptr = self.old_value.clone();
        }
    }

    pub mod detail {
        use std::any::Any;
        use std::marker::PhantomData;

        use super::MultiSwitch;

        pub fn multi_encode<T>(value: &T) -> u64
        where
            T: MultiSwitchTrait,
        {
            T::encode(value)
        }

        pub fn multi_encode_recursive<Head, Next, Rest>(
            head: &Head,
            next: &Next,
            rest: &Rest,
        ) -> u64
        where
            Head: MultiSwitchTrait,
            Next: MultiSwitchTrait,
            Rest: MultiSwitchTrait,
        {
            let v = multi_encode(next);
            assert!(
                v < (u64::MAX / Head::max_value()),
                "Value {} exceeds maximum encodable value for type.",
                v
            );
            (v * Head::max_value()) + Head::encode(head)
        }
    }

    pub fn multi<T>(values: T) -> u64
    where
        T: MultiSwitchTrait,
    {
        T::encode(&values)
    }

    pub fn multi_recursive<Head, Next, Rest>(head: Head, next: Next, rest: Rest) -> u64
    where
        Head: MultiSwitchTrait,
        Next: MultiSwitchTrait,
        Rest: MultiSwitchTrait,
    {
        let next_encoded = detail::multi_encode(&next);
        let rest_encoded = detail::multi_encode(&rest);
        let encoded = (rest_encoded * Next::max_value()) + next_encoded;

        assert!(
            encoded < (u64::MAX / Head::max_value()),
            "Value {} exceeds maximum encodable value for type.",
            encoded
        );

        (encoded * Head::max_value()) + Head::encode(&head)
    }

    pub trait MultiSwitchTrait {
        fn max_value() -> u64;
        fn encode<T>(value: &T) -> u64
        where
            Self: Sized;
    }

    pub struct MultiSwitchIntegral<T, const MAX_VALUE: u64> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const MAX_VALUE: u64> MultiSwitchIntegral<T, MAX_VALUE> {
        const MAX_VALUE_CONST: u64 = MAX_VALUE;
    }

    impl<T, const MAX_VALUE: u64> MultiSwitchTrait for MultiSwitchIntegral<T, MAX_VALUE>
    where
        T: Copy + Into<u64>,
    {
        fn max_value() -> u64 {
            MAX_VALUE
        }

        fn encode<V>(value: &V) -> u64
        where
            Self: Sized,
        {
            let v: u64 = (*value as u64).into();
            assert!(v < MAX_VALUE, "Value {} exceeds maximum value {}", v, MAX_VALUE);
            v
        }
    }

    pub struct MultiSwitch<T> {
        _phantom: PhantomData<T>,
    }

    macro_rules! define_multi_switch_integral {
        ($name:ty, $max_value:expr) => {
            impl MultiSwitchTrait for $name {
                fn max_value() -> u64 {
                    $max_value
                }

                fn encode<V>(value: &V) -> u64
                where
                    Self: Sized,
                {
                    let v = (*value as u64).into();
                    assert!(v < $max_value, "Value {} exceeds maximum value", v);
                    v
                }
            }
        };
    }

    define_multi_switch_integral!(bool, 2);
}

mod v8_flags {
    pub static turboshaft_opt_bisect_break: u64 = 0;
    pub static turboshaft_opt_bisect_limit: u64 = u64::MAX;
}

pub mod base {
    use std::fmt::{Debug, Display, Formatter, Result};

    pub fn PrintCheckOperand<T: Debug>(operand: &T) -> PrintCheckOperandHelper<'_, T> {
        PrintCheckOperandHelper(operand)
    }

    pub struct PrintCheckOperandHelper<'a, T: Debug>(&'a T);

    impl<'a, T: Debug> Display for PrintCheckOperandHelper<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{:?}", self.0)
        }
    }
}
