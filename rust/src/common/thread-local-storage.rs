// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The equivalent of include/v8config.h would typically be handled through Cargo features
// and conditional compilation attributes in Rust.  This example assumes
// a default configuration where V8_TARGET_OS_WIN, V8_TARGET_OS_ANDROID,
// COMPONENT_BUILD, and V8_TLS_USED_IN_LIBRARY are not defined.  Cargo features
// would allow customization of this setup.
//
// Example Cargo.toml:
//
// [features]
// component_build = []
// v8_tls_used_in_library = []
// target_os_win = []
// target_os_android = []
//
// Then, use cfg!(feature = "component_build") in the code below.

#[cfg(any(feature = "component_build", feature = "v8_tls_used_in_library"))]
const V8_TLS_LIBRARY_MODE: bool = true;
#[cfg(not(any(feature = "component_build", feature = "v8_tls_used_in_library")))]
const V8_TLS_LIBRARY_MODE: bool = false;

#[cfg(V8_TLS_LIBRARY_MODE)]
const V8_TLS_MODEL: &str = "local-dynamic";

#[cfg(not(V8_TLS_LIBRARY_MODE))]
#[cfg(feature = "target_os_win")]
const V8_TLS_MODEL: &str = "initial-exec";

#[cfg(not(V8_TLS_LIBRARY_MODE))]
#[cfg(all(not(feature = "target_os_win"), feature = "target_os_android"))]
const V8_TLS_MODEL: &str = "local-dynamic";

#[cfg(not(V8_TLS_LIBRARY_MODE))]
#[cfg(not(any(feature = "target_os_win", feature = "target_os_android")))]
const V8_TLS_MODEL: &str = "local-exec";

macro_rules! v8_tls_declare_getter {
    ($name:ident, $type:ty, $member:expr) => {
        #[inline(always)]
        fn $name() -> $type;
    };
}

macro_rules! v8_tls_define_getter {
    ($name:ident, $type:ty, $member:expr) => {
        #[inline(always)]
        fn $name() -> $type {
            $member
        }
    };
}

#[cfg(V8_TLS_LIBRARY_MODE)]
mod tls {
    macro_rules! v8_tls_declare_getter {
        ($name:ident, $type:ty, $member:expr) => {
            #[inline(never)]
            pub fn $name() -> $type;
        };
    }

    macro_rules! v8_tls_define_getter {
        ($name:ident, $type:ty, $member:expr) => {
            #[inline(never)]
            pub fn $name() -> $type {
                $member
            }
        };
    }

    pub(crate) use v8_tls_declare_getter;
    pub(crate) use v8_tls_define_getter;
}

#[cfg(not(V8_TLS_LIBRARY_MODE))]
mod tls {
    macro_rules! v8_tls_declare_getter {
        ($name:ident, $type:ty, $member:expr) => {
            #[inline(always)]
            pub fn $name() -> $type {
                $member
            }
        };
    }
    macro_rules! v8_tls_define_getter {
        ($name:ident, $type:ty, $member:expr) => {};
    }

    pub(crate) use v8_tls_declare_getter;
    pub(crate) use v8_tls_define_getter;
}

pub use tls::v8_tls_declare_getter;
pub use tls::v8_tls_define_getter;