// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file mirrors the functionality of the C++ header
// `include/cppgc/internal/compiler-specific.h`. It provides
// compiler-specific attributes and macros for cppgc.

// v8config.h is not directly convertible. Its contents would
// define the `V8_CC_MSVC` macro, which determines the compiler in use.
// In this Rust version, we'll use conditional compilation based on the target
// operating system and environment to achieve the same effect.
//
// Note: The `#[cfg(target_env = "msvc")]` attribute below would need to be
// adjusted based on the configuration defined in `v8config.h`.

pub mod cppgc {

    // Mimic the functionality of CPPGC_HAS_ATTRIBUTE and CPPGC_HAS_CPP_ATTRIBUTE
    // using Rust's built-in conditional compilation features.
    // These macros were designed to detect compiler support for particular language features.
    // Rust's compiler intrinsically supports the features needed for the `cppgc` crate, so
    // these macros are unnecessary.  We simply define empty macros for compatibility.

    macro_rules! CPPGC_HAS_ATTRIBUTE {
        ($feature:ident) => {
            true
        };
    }

    macro_rules! CPPGC_HAS_CPP_ATTRIBUTE {
        ($feature:ident) => {
            true
        };
    }

    // no_unique_address is similar to `std::marker::PhantomData` in Rust,
    // but it guarantees zero-sized optimization.  Since Rust already
    // handles zero-sized types efficiently, we don't need a special
    // attribute here.  Rust has its own approach to zero-sized members.
    //
    // `#[repr(marker)]` could be added to any types that would need this if
    // it were to be a struct.

    // We are simulating the effect of CPPGC_NO_UNIQUE_ADDRESS here. In C++,
    // [[no_unique_address]] could instruct the compiler that a member should not
    // contribute to the size of the containing class, provided it's zero-sized.
    // This is primarily useful for empty base optimization and preventing padding.
    // In Rust, zero-sized types (ZSTs) are already handled optimally by the compiler,
    // and adding a ZST as a field does not increase the size of the struct.
    // Therefore, no special attribute is needed in Rust.

    macro_rules! CPPGC_NO_UNIQUE_ADDRESS {
        () => {};
    }

    // CPPGC_UNUSED is equivalent to `#[allow(dead_code)]` in Rust.
    // It suppresses warnings about unused variables.
    macro_rules! CPPGC_UNUSED {
        () => {};
    }
}