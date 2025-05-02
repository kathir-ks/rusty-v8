// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a macro for exporting symbols from the base crate.

#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! v8_base_export {
    () => {
        #[cfg(feature = "building_v8_base_shared")]
        #[link_section = ".exports"]
        extern "C" {
            // Marks symbol for export on windows when building shared library.
            // This is a no-op on other platforms.
            // The actual attributes are handled by the linker.
            // Example: #[v8_base_export] pub fn my_function() {}
            #[link_name = "_"]
            fn __declspec_dllexport();
        }

        #[cfg(all(feature = "using_v8_base_shared", not(feature = "building_v8_base_shared")))]
        #[link_section = ".imports"]
        extern "C" {
            // Marks symbol for import on windows when using shared library.
            // This is a no-op on other platforms.
            // The actual attributes are handled by the linker.
            // Example: #[v8_base_export] pub fn my_function() {}
            #[link_name = "_"]
            fn __declspec_dllimport();
        }
    };
}

#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! v8_base_export {
    () => {
        #[cfg(feature = "building_v8_base_shared")]
        #[link_section = ".init_array"]
        extern "C" {
            // Marks symbol with visibility("default") on non-windows.
            // This is a no-op if not building shared library.
            // Example: #[v8_base_export] pub fn my_function() {}
            #[link_name = "_"]
            fn __attribute__((visibility("default")));
        }
    };
}

#[cfg(not(any(target_os = "windows", feature = "building_v8_base_shared", feature = "using_v8_base_shared")))]
#[macro_export]
macro_rules! v8_base_export {
    () => {
        // No-op when not on Windows and not building or using a shared library.
    };
}