// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides compiler-specific definitions.

// Annotation to silence compiler warnings about unused
// types/functions/variables.
#[macro_export]
macro_rules! allow_unused {
    ($item:item) => {
        #[allow(unused)]
        $item
    };
}

// Tell the compiler a function is using a printf-style format string.
// In Rust, this is handled by the format! macro and related functionalities,
// so no direct equivalent is needed.

// The C++ standard requires that static const members have an out-of-class
// definition (in a single compilation unit), but MSVC chokes on this.
// This is not directly applicable in Rust as static const members are handled differently.
// We will declare static constants directly.

// Macros for suppressing and disabling warnings on MSVC.
// These are not needed directly in Rust as warnings are handled differently.

// Allows exporting a class that inherits from a non-exported base class.
// This uses suppress instead of push/pop because the delimiter after the
// declaration (either "," or "{") has to be placed before the pop macro.
// In Rust, inheritance is handled differently (traits).  We don't need a direct
// equivalent.
#[macro_export]
macro_rules! non_exported_base {
    ($code:item) => {
        $code
    };
}

// Allowing the use of noexcept by removing the keyword on older compilers that
// do not support adding noexcept to default members.
// Rust handles exceptions differently; `noexcept` is not a direct concept.
// We use the `panic` system, and `Result` for recoverable errors.

// Specify memory alignment for structs, classes, etc.
// Rust provides the `align` attribute for this purpose.
#[macro_export]
macro_rules! alignas {
    ($alignment:expr, $item:item) => {
        #[repr(align($alignment))]
        $item
    };
}

// Functions called from GDB.
// Forces the linker to not optimize out the function.
// This can be achieved with appropriate compiler attributes in Rust.
#[macro_export]
macro_rules! debugging_export {
    ($item:item) => {
        #[cfg(debug_assertions)] // Only apply in debug builds
        $item

        #[cfg(not(debug_assertions))]
        $item // Still compile the item, but without special attributes
    };
}

// Check for C++20 feature availability.
// This can be checked in Rust using feature flags and conditional compilation.
#[cfg(feature = "cpp20")]
const HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS: bool = true;

#[cfg(not(feature = "cpp20"))]
const HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS: bool = false;