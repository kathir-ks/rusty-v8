// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// UndefinedBehaviorSanitizer support.

#[cfg(not(feature = "UNDEFINED_SANITIZER"))]
macro_rules! disable_ubsan {
    () => {};
}

#[cfg(feature = "UNDEFINED_SANITIZER")]
macro_rules! disable_ubsan {
    () => {
        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.disable_ubsan")]
        #[cfg_attr(target_os = "macos", link_section = "__TEXT,__text,regular,no_dead_strip")]
        #[cfg_attr(target_os = "windows", link_section = ".text$disable_ubsan")]
        #[inline(never)]
        pub unsafe fn disable_ubsan<F: FnOnce()>(f: F) {
            f()
        }
    };
}

#[cfg(feature = "UNDEFINED_SANITIZER")]
pub use disable_ubsan;

#[cfg(not(feature = "UNDEFINED_SANITIZER"))]
pub use disable_ubsan;