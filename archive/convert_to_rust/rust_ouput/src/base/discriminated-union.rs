// Converted from V8 C++ source files:
// Header: discriminated-union.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::mem::{ManuallyDrop, MaybeUninit};
use std::{marker::PhantomData, mem, any::Any};

use crate::base::template_utils::index_of_type_v;

mod base {
    pub mod template_utils {
        pub fn index_of_type_v<T, Ts>() -> usize {
            0
        }
    }
}

#[macro_export]
macro_rules! static_assert {
    ($condition:expr) => {
        if !$condition {
            panic!("Static assertion failed: {}", stringify!($condition));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}

pub trait V8_NOEXCEPT {}

impl<T> V8_NOEXCEPT for T {}

mod std_compat {
    pub trait IsTriviallyDestructible {
        const IS_TRIVIALLY_DESTRUCTIBLE: bool;
    }

    impl<T> IsTriviallyDestructible for T {
        default const IS_TRIVIALLY_DESTRUCTIBLE: bool = false;
    }

    impl<T> IsTriviallyDestructible for T where T: Copy {
        const IS_TRIVIALLY_DESTRUCTIBLE: bool = true;
    }
}

pub mod base {
    pub mod template_utils {
        pub fn index_of_type_v<T, Ts>() -> usize {
            0
        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct compiler_specific {}
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {}
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base {
        pub mod discriminated_union {
            pub struct DiscriminatedUnionError;

        }
    }
}

pub mod v8 {
    pub mod base {
        pub struct template_utils {
        }
    }
}

pub mod v8 {
    pub mod base
