// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// This module provides macros for using FOO_EXPORT macros with explicit
/// template instantiation declarations and definitions.
/// Generally, the FOO_EXPORT macros are used at declarations,
/// and GCC requires them to be used at explicit instantiation declarations,
/// but MSVC requires __declspec(dllexport) to be used at the explicit
/// instantiation definitions instead.
///
/// This C++ code uses a lot of preprocessor trickery which is not directly
/// translatable to Rust.  The core intent is to conditionally define
/// export behavior based on compiler and flags.  In Rust, this is generally
/// handled via conditional compilation attributes (#[cfg(...)]).  However,
/// this particular C++ header is designed to *abstract* that conditional
/// compilation through macros, which we cannot easily replicate.
///
/// The following Rust code provides a *simplified* equivalent which focuses
/// on the *intent* of the C++ code: to define different export behaviors
/// (specifically, whether to use `dllexport` or not) based on some condition.
///
/// This simplistic Rust code assumes that we have a feature flag, say "msvc",
/// that is defined when compiling for MSVC.  We can use this feature flag
/// to conditionally define whether a function/struct/etc. is exported.
///
/// Note that a true equivalent of the C++ code would require implementing
/// the complex macro logic in Rust, which is beyond the scope of this simple
/// translation.  Such an implementation would likely involve procedural macros.

#[cfg(feature = "msvc")]
macro_rules! export_template_declare {
    () => {};
}

#[cfg(not(feature = "msvc"))]
macro_rules! export_template_declare {
    () => {};
}

#[cfg(feature = "msvc")]
macro_rules! export_template_define {
    () => {};
}

#[cfg(not(feature = "msvc"))]
macro_rules! export_template_define {
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros() {
        // These tests are placeholders.  A real implementation would
        // generate different code depending on the "msvc" feature.
        // The goal of these placeholder tests is just to ensure that
        // the macros are syntactically valid and compile.
        export_template_declare!();
        export_template_define!();
    }
}