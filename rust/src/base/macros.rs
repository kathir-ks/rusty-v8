// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]

use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
    fmt,
    marker::PhantomData,
    any::Any,
    convert::TryInto,
    sync::atomic::{AtomicUsize, Ordering},
};
use libc::offset_of;

macro_rules! expand {
    ($x:tt) => { $x };
}

macro_rules! nothing {
    ($($tt:tt)*) => {};
}

macro_rules! concat_impl {
    ($a:ident, $($b:tt)*) => {
        paste::expr! {
            $a [< __ $($b)* >]
        }
    };
}

macro_rules! concat {
    ($a:ident, $($b:tt)*) => {
        concat_impl!($a, $($b)*)
    };
}

macro_rules! unique_identifier {
    ($base:ident) => {
        {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let count = COUNTER.fetch_add(1, Ordering::SeqCst);
            paste::expr! {
                [<$base _ $count>]
            }
        }
    };
}

macro_rules! count_macro_args {
    ($($args:tt),*) => {
        <[()]>::len(&[$(count_macro_args!(@replace $args)),*])
    };
    (@replace $_t:tt) => { () };
}

macro_rules! get_nth_arg {
    (0, $arg0:expr, $($args:expr),*) => { $arg0 };
    (1, $arg0:expr, $arg1:expr, $($args:expr),*) => { $arg1 };
    (2, $arg0:expr, $arg1:expr, $arg2:expr, $($args:expr),*) => { $arg2 };
    (3, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $($args:expr),*) => { $arg3 };
    (4, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $($args:expr),*) => { $arg4 };
    (5, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $($args:expr),*) => { $arg5 };
    (6, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $($args:expr),*) => { $arg6 };
    (7, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $($args:expr),*) => { $arg7 };
}

macro_rules! unparen {
    ($x:tt) => {
        drop_unparen!($x)
    };
}

macro_rules! drop_unparen {
    ($($tokens:tt)*) => {};
}

macro_rules! offset_of {
    ($type:ty, $field:tt) => {
        unsafe {
            &(*(ptr::null() as *const $type)).$field as *const _ as usize - ptr::null() as *const $type as usize
        }
    };
}

const LITERAL_COMMA: &str = ",";

macro_rules! arraysize {
    ($array:expr) => {
        {
            #[allow(unused_variables)]
            let array_size_helper = |arr: &[_; N]| N;
            array_size_helper($array)
        }
    };
}

pub mod v8 {
    pub mod base {
        use std::{mem, any::Any};
        use crate::logging;

        pub trait TriviallyCopyable: Copy + 'static {}

        macro_rules! impl_trivially_copyable {
            ($($t:ty),*) => {
                $(
                    impl TriviallyCopyable for $t {}
                )*
            }
        }

        impl_trivially_copyable!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, char);

        pub fn bit_cast<Dest: TriviallyCopyable, Source: TriviallyCopyable>(source: &Source) -> Dest {
            assert_eq!(
                std::mem::size_of::<Dest>(),
                std::mem::size_of::<Source>(),
                "bit_cast requires source and destination types to be the same size"
            );

            unsafe {
                let mut dest: Dest = std::mem::zeroed();
                std::ptr::copy_nonoverlapping(
                    source as *const Source as *const u8,
                    &mut dest as *mut Dest as *mut u8,
                    std::mem::size_of::<Dest>(),
                );
                dest
            }
        }

        macro_rules! disallow_assign {
            ($TypeName:ident) => {
                impl $TypeName {
                    #[allow(dead_code)]
                    fn disallow_assign(&mut self, _: &Self) {
                        panic!("Assignment is disallowed for type {}", stringify!($TypeName));
                    }
                }
            };
        }

        macro_rules! disallow_implicit_constructors {
            ($TypeName:ident) => {
                impl $TypeName {
                    #[allow(dead_code)]
                    fn disallow_implicit_constructors() {
                        panic!("Implicit constructors are disallowed for type {}", stringify!($TypeName));
                    }
                }
            };
        }

        macro_rules! move_only_with_default_constructors {
            ($TypeName:ident) => {
                impl $TypeName {
                    // Default constructor is implicitly defined or explicitly defaulted

                    // Define move constructor and move assignment
                    // impl<T> From<T> for $TypeName {
                    //     fn from(t: T) -> Self {
                    //         Self(t)
                    //     }
                    // }
                    //
                    // impl $TypeName {
                    //     fn move_assign(&mut self, other: Self) {
                    //         *self = other;
                    //     }
                    // }

                    // Delete copy constructor and copy assignment
                    #[allow(dead_code)]
                    fn disallow_copy(&self, _: &Self) {
                        panic!("Copying is disallowed for type {}", stringify!($TypeName));
                    }
                }

                impl Clone for $TypeName {
                    fn clone(&self) -> Self {
                        panic!("Cloning is disallowed for type {}", stringify!($TypeName));
                    }
                }
            };
        }

        macro_rules! move_only_no_default_constructor {
            ($TypeName:ident) => {
                impl $TypeName {
                    // Define move constructor and move assignment
                    // impl<T> From<T> for $TypeName {
                    //     fn from(t: T) -> Self {
                    //         Self(t)
                    //     }
                    // }
                    //
                    // impl $TypeName {
                    //     fn move_assign(&mut self, other: Self) {
                    //         *self = other;
                    //     }
                    // }

                    // Delete copy constructor and copy assignment
                    #[allow(dead_code)]
                    fn disallow_copy(&self, _: &Self) {
                        panic!("Copying is disallowed for type {}", stringify!($TypeName));
                    }
                }

                impl Clone for $TypeName {
                    fn clone(&self) -> Self {
                        panic!("Cloning is disallowed for type {}", stringify!($TypeName));
                    }
                }
            };
        }

        macro_rules! disallow_new_and_delete {
            () => {
                compile_error!("Allocation not allowed, use static or stack allocation instead");
                // fn new() -> ! {
                //     panic!("Allocation not allowed, use static or stack allocation instead");
                // }
            };
        }

        pub struct Use;

        impl Use {
            #[allow(clippy::unused_self)]
            #[inline(always)]
            pub const fn new<T>(_t: T) -> Self {
                Use
            }
        }

        #[macro_export]
        macro_rules! use_expr {
            ($($x:expr),*) => {
                {
                    #[allow(unused_mut)]
                    let mut _use = ::v8::base::Use::new(());
                    $(
                        _use = ::v8::base::Use::new($x);
                    )*
                }
            };
        }

        pub fn os_abort() -> ! {
            panic!("OS::Abort() called");
        }
        // Note that some implementations of std::is_trivially_copyable mandate that at
        // least one of the copy constructor, move constructor, copy assignment or move
        // assignment is non-deleted, while others do not. Be aware that also
        // base::is_trivially_copyable will differ for these cases.
        pub struct IsTriviallyCopyable<T>(PhantomData<T>);

        impl<T> IsTriviallyCopyable<T> {
            pub const VALUE: bool = std::mem::needs_drop::<T>();
        }

        macro_rules! assert_trivially_copyable {
            ($t:ty) => {
                const _: () = assert!(
                    std::mem::size_of::<$t>() > 0,
                    concat!("`", stringify!($t), "` must be a sized type")
                );
                const _: () = assert!(
                    ::std::marker::Copy::is_copy::<$t>(),
                    concat!("`", stringify!($t), "` must be copyable")
                );
                const _: () = assert!(
                    !::std::mem::needs_drop::<$t>(),
                    concat!("`", stringify!($t), "` must not need to be dropped")
                );
            };
        }

        macro_rules! assert_not_trivially_copyable {
            ($t:ty) => {
                const _: () = assert!(
                    !::std::marker::Copy::is_copy::<$t>() || ::std::mem::needs_drop::<$t>(),
                    concat!("`", stringify!($t), "` must not be trivially copyable")
                );
            };
        }

        pub struct IsTriviallyDestructible<T>(PhantomData<T>);

        impl<T> IsTriviallyDestructible<T> {
            pub const VALUE: bool = !std::mem::needs_drop::<T>();
        }

        macro_rules! assert_trivially_destructible {
            ($t:ty) => {
                const _: () = assert!(
                    !::std::mem::needs_drop::<$t>(),
                    concat!("`", stringify!($t), "` must not need to be dropped")
                );
            };
        }

        macro_rules! assert_not_trivially_destructible {
            ($t:ty) => {
                const _: () = assert!(
                    ::std::mem::needs_drop::<$t>(),
                    concat!("`", stringify!($t), "` must need to be dropped")
                );
            };
        }

    }
}

#[inline]
pub fn implicit_cast<A>(x: A) -> A {
    x
}

#[cfg(target_pointer_width = "64")]
pub const V8_PTR_PREFIX: &str = "l";
#[cfg(not(target_pointer_width = "64"))]
pub const V8_PTR_PREFIX: &str = "";

pub const V8PRIxPTR: &str = concat!(V8_PTR_PREFIX, "x");
pub const V8PRIdPTR: &str = concat!(V8_PTR_PREFIX, "d");
pub const V8PRIuPTR: &str = concat!(V8_PTR_PREFIX, "u");

#[cfg(target_pointer_width = "64")]
pub const V8_PTR_HEX_DIGITS: usize = 12;
#[cfg(not(target_pointer_width = "64"))]
pub const V8_PTR_HEX_DIGITS: usize = 8;

#[cfg(target_pointer_width = "64")]
pub const V8PRIxPTR_FMT: &str = "0x%012lx";
#[cfg(not(target_pointer_width = "64"))]
pub const V8PRIxPTR_FMT: &str = "0x%08x";

pub const V8PRIxPTRDIFF: &str = "tx";
pub const V8PRIdPTRDIFF: &str = "td";
pub const V8PRIuPTRDIFF: &str = "tu";

#[inline]
pub fn make_uint64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) + low as u64
}

pub fn round_down<T>(x: T, m: isize) -> T
where
    T: std::ops::BitAnd<Output = T> + std::ops::Not<Output = T> + From<isize> + Copy,
{
    // m must be a power of two.
    assert!(m != 0 && (m & (m - 1) == 0));
    x & (!T::from(m -1 ))
}

pub fn round_up<T>(x: T, m: isize) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::BitAnd<Output = T>
        + std::ops::Not<Output = T>
        + From<isize>
        + std::cmp::PartialOrd
        + Copy,
{
    assert!(x >= T::from(0));
    // Overflow check.
    //assert!(std::T::MAX - x >= T::from(m - 1));
    round_down(x + T::from(m - 1), m)
}

pub const fn is_aligned<T, U>(value: T, alignment: U) -> bool
where
    T: std::ops::BitAnd<Output = T> + std::ops::Sub<Output = T> + From<U>,
    U: Copy,
    T: PartialEq<T>
{
    (value & (T::from(alignment) - T::from(1 as u8))) == T::from(0 as u8)
}

pub fn aligned_address(address: *mut std::ffi::c_void, alignment: usize) -> *mut std::ffi::c_void {
    round_down(address as usize, alignment as isize) as *mut std::ffi::c_void
}

pub fn round_up_address(address: *mut std::ffi::c_void, alignment: usize) -> *mut std::ffi::c_void {
    round_up(address as usize, alignment as isize) as *mut std::ffi::c_void
}

pub fn is_inbounds<int_t, float_t>(v: float_t) -> bool
where
    int_t: std::convert::TryFrom<float_t> + std::cmp::PartialOrd,
    float_t: Copy,
{
    let min = std::i64::MIN as float_t;
    let max = std::i64::MAX as float_t;

    let lower_bound = std::i64::MIN as float_t - 1.0;
    let upper_bound = std::i64::MAX as float_t + 1.0;

    let lower_bound_is_min = lower_bound == min;
    let upper_bound_is_max = upper_bound == max;

    if lower_bound_is_min {
        if lower_bound <= v {
            return true;
        }
    } else {
        if lower_bound < v {
            return true;
        }
    }

    if upper_bound_is_max {
        if v <= upper_bound {
            return true;
        }
    } else {
        if v < upper_bound {
            return true;
        }
    }

    false
}

// Setup for Windows shared library export.
// TODO: Implement these for Rust
// macro_rules! V8_EXPORT_ENUM {
//     () => {};
// }

// macro_rules! V8_EXPORT_PRIVATE {
//     () => {};
// }

// Defines IF_WASM, to be used in macro lists for elements that should only be
// there if WebAssembly is enabled.
#[macro_export]
macro_rules! if_wasm {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_wasm {
    ($($tokens:tt)*) => {};
}

// Defines IF_TSAN, to be used in macro lists for elements that should only be
// there if TSAN is enabled.
#[macro_export]
macro_rules! if_tsan {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_tsan {
    ($($tokens:tt)*) => {};
}

// Defines IF_INTL, to be used in macro lists for elements that should only be
// there if INTL is enabled.
#[macro_export]
macro_rules! if_intl {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_intl {
    ($($tokens:tt)*) => {};
}

// Defines IF_SHADOW_STACK, to be used in macro lists for elements that should
// only be there if CET shadow stack is enabled.
#[macro_export]
macro_rules! if_shadow_stack {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_shadow_stack {
    ($($tokens:tt)*) => {};
}

// Defines IF_TARGET_ARCH_64_BIT, to be used in macro lists for elements that
// should only be there if the target architecture is a 64-bit one.
#[macro_export]
macro_rules! if_target_arch_64_bit {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_target_arch_64_bit {
    ($($tokens:tt)*) => {};
}

// Defines IF_V8_WASM_RANDOM_FUZZERS and IF_NO_V8_WASM_RANDOM_FUZZERS, to be
// used in macro lists for elements that should only be there/absent when
// building the Wasm fuzzers.
#[macro_export]
macro_rules! if_v8_wasm_random_fuzzers {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

#[macro_export]
macro_rules! if_no_v8_wasm_random_fuzzers {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

pub mod logging {
    #[inline]
    pub fn check(condition: bool) {
        if !condition {
            panic!("Check failed");
        }
    }

    #[inline]
    pub fn dcheck(condition: bool) {
        if !condition {
            panic!("DCheck failed");
        }
    }

    #[inline]
    pub fn dcheck_ge<T: PartialOrd>(a: T, b: T) {
        if !(a >= b) {
            panic!("DCheck failed: {} >= {}", a, b);
        }
    }
}
