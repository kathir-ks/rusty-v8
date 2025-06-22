// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(target_os = "windows")]
mod windows {
    #[cfg(feature = "building_v8_platform_shared")]
    #[macro_export]
    macro_rules! v8_platform_export {
        () => {
            #[cfg(target_os = "windows")]
            #[link_section = ".CRT$XCA"]
            #[used]
            static V8_PLATFORM_EXPORT_MARKER: extern "system" fn() = {
                extern "system" fn marker() {}
                marker
            };
        };
    }

    #[cfg(all(not(feature = "building_v8_platform_shared"), feature = "using_v8_platform_shared"))]
    #[macro_export]
    macro_rules! v8_platform_export {
        () => {}; // Placeholder, dllimport not directly translatable with current Rust features. Needs linker magic.
    }

    #[cfg(not(any(feature = "building_v8_platform_shared", feature = "using_v8_platform_shared")))]
    #[macro_export]
    macro_rules! v8_platform_export {
        () => {};
    }

}

#[cfg(not(target_os = "windows"))]
mod unix {
    #[cfg(feature = "building_v8_platform_shared")]
    #[macro_export]
    macro_rules! v8_platform_export {
        () => {
            #[no_mangle]
            #[allow(dead_code)]
            pub extern "C" fn v8_platform_export_symbol() {}
        };
    }

    #[cfg(not(feature = "building_v8_platform_shared"))]
    #[macro_export]
    macro_rules! v8_platform_export {
        () => {};
    }
}

#[cfg(target_os = "windows")]
pub use windows::v8_platform_export;

#[cfg(not(target_os = "windows"))]
pub use unix::v8_platform_export;

// Example usage
// #[v8_platform_export]
// pub fn exported_function() {
//     println!("This function is exported!");
// }
