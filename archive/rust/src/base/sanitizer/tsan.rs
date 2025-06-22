// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// ThreadSanitizer support.

#[cfg(feature = "thread_sanitizer")]
macro_rules! disable_tsan {
    () => {
        #[cfg_attr(target_os = "linux", link_section = ".attribute_no_sanitize_thread")]
        fn dummy() {} // Dummy function to attach the attribute. Might need adjustment for different platforms.
    };
}

#[cfg(not(feature = "thread_sanitizer"))]
macro_rules! disable_tsan {
    () => {};
}

// Example usage:
// disable_tsan!();