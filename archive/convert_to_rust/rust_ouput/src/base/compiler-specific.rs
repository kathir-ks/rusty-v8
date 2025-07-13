// Converted from V8 C++ source files:
// Header: compiler-specific.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::io::Write;

// Placeholder for v8config.h contents
pub struct V8Config {}

// Annotation to silence compiler warnings about unused
// types/functions/variables. Use like:
//
//   using V8_ALLOW_UNUSED Bar = Foo;
//   V8_ALLOW_UNUSED void foo() {}
#[cfg(feature = "v8_has_attribute_unused")]
macro_rules! V8_ALLOW_UNUSED {
    () => {
        #[allow(unused)]
    };
}

#[cfg(not(feature = "v8_has_attribute_unused"))]
macro_rules! V8_ALLOW_UNUSED {
    () => {};
}

// Tell the compiler a function is using a printf-style format string.
// |format_param| is the one-based index of the format string parameter;
// |dots_param| is the one-based index of the "..." parameter.
// For v*printf functions (which take a va_list), pass 0 for dots_param.
// (This is undocumented but matches what the system C headers do.)
macro_rules! PRINTF_FORMAT {
    ($format_param:expr, $dots_param:expr) => {};
}

// The C++ standard requires that static const members have an out-of-class
// definition (in a single compilation unit), but MSVC chokes on this (when
// language extensions, which are required, are enabled). (You're only likely to
// notice the need for a definition if you take the address of the member or,
// more commonly, pass it to a function that takes it as a reference argument --
// probably an STL function.) This macro makes MSVC do the right thing. See
// http://msdn.microsoft.com/en-us/library/34h23df8(v=vs.100).aspx for more
// information. Use like:
//
// In .h file:
//   struct Foo {
//     static const int kBar = 5;
//   };
//
// In .cc file:
//   STATIC_CONST_MEMBER_DEFINITION const int Foo::kBar;
macro_rules! STATIC_CONST_MEMBER_DEFINITION {
    () => {};
}

// Macros for suppressing and disabling warnings on MSVC.
//
// Warning numbers are enumerated at:
// http://msdn.microsoft.com/en-us/library/8x5x43k7(VS.80).aspx
//
// The warning pragma:
// http://msdn.microsoft.com/en-us/library/2c8f766e(VS.80).aspx
//
// Using __pragma instead of #pragma inside macros:
// http://msdn.microsoft.com/en-us/library/d9x1s805.aspx

// MSVC_SUPPRESS_WARNING disables warning |n| for the remainder of the line and
// for the next line of the source file.
macro_rules! MSVC_SUPPRESS_WARNING {
    ($n:expr) => {};
}

// Allows exporting a class that inherits from a non-exported base class.
// This uses suppress instead of push/pop because the delimiter after the
// declaration (either "," or "{") has to be placed before the pop macro.
//
// Example usage:
// class EXPORT_API Foo : NON_EXPORTED_BASE(public Bar) {
//
// MSVC Compiler warning C4275:
// non dll-interface class 'Bar' used as base for dll-interface class 'Foo'.
// Note that this is intended to be used only when no access to the base class'
// static data is done through derived classes or inline methods. For more info,
// see http://msdn.microsoft.com/en-us/library/3tdb471s(VS.80).aspx
macro_rules! NON_EXPORTED_BASE {
    ($code:tt) => {
        $code
    };
}

// Allowing the use of noexcept by removing the keyword on older compilers that
// do not support adding noexcept to default members.
// Disabled on MSVC because constructors of standard containers are not noexcept
// there.
#[cfg(all(
    not(feature = "v8_cc_gnu"),
    not(feature = "v8_cc_msvc"),
    not(feature = "v8_target_arch_mips64"),
    not(feature = "v8_target_arch_ppc64"),
    not(feature = "v8_target_arch_riscv64"),
    not(feature = "v8_target_arch_riscv32")
))]
macro_rules! V8_NOEXCEPT {
    () => {
        std::panic::catch_unwind
    };
}

#[cfg(feature = "v8_cc_gnu")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

#[cfg(feature = "v8_cc_msvc")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

#[cfg(feature = "v8_target_arch_mips64")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

#[cfg(feature = "v8_target_arch_ppc64")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

#[cfg(feature = "v8_target_arch_riscv64")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

#[cfg(feature = "v8_target_arch_riscv32")]
macro_rules! V8_NOEXCEPT {
    () => {
        ||
    };
}

// Specify memory alignment for structs, classes, etc.
// Use like:
//   class ALIGNAS(16) MyClass { ... }
//   ALIGNAS(16) int array[4];
//
// In most places you can use the C++11 keyword "alignas", which is preferred.
//
// But compilers have trouble mixing __attribute__((...)) syntax with
// alignas(...) syntax.
//
// Doesn't work in clang or gcc:
//   struct alignas(16) __attribute__((packed)) S { char c; };
// Works in clang but not gcc:
//   struct __attribute__((packed)) alignas(16) S2 { char c; };
// Works in clang and gcc:
//   struct alignas(16) S3 { char c; } __attribute__((packed));
//
// There are also some attributes that must be specified *before* a class
// definition: visibility (used for exporting functions/classes) is one of
// these attributes. This means that it is not possible to use alignas() with a
// class that is marked as exported.
#[cfg(feature = "v8_cc_msvc")]
macro_rules! ALIGNAS {
    ($byte_alignment:expr) => {
        #[repr(align($byte_alignment))]
    };
}

#[cfg(not(feature = "v8_cc_msvc"))]
macro_rules! ALIGNAS {
    ($byte_alignment:expr) => {
        #[repr(align($byte_alignment))]
    };
}

// Functions called from GDB.
// Forces the linker to not optimize out the function.
#[cfg(all(
    feature = "v8_has_attribute_used",
    feature = "v8_has_attribute_retain",
    feature = "v8_has_attribute_optnone",
    feature = "v8_has_attribute_visibility"
))]
macro_rules! V8_DEBUGGING_EXPORT {
    () => {
        #[used]
        #[no_mangle]
        #[inline(never)]
    };
}

#[cfg(not(all(
    feature = "v8_has_attribute_used",
    feature = "v8_has_attribute_retain",
    feature = "v8_has_attribute_optnone",
    feature = "v8_has_attribute_visibility"
)))]
macro_rules! V8_DEBUGGING_EXPORT {
    () => {};
}

#[cfg(target_feature = "c++20")]
const HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS: i32 = 1;
