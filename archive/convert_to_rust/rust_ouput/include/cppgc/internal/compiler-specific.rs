// Converted from V8 C++ source files:
// Header: compiler-specific.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: v8config.h is assumed to be provided by the build environment

pub mod cppgc {

    #[cfg(target_env = "msvc")]
    macro_rules! cppgc_no_unique_address {
        () => {
            #[msvc::no_unique_address]
        };
    }

    #[cfg(not(target_env = "msvc"))]
    macro_rules! cppgc_no_unique_address {
        () => {
            #[no_unique_address]
        };
    }

    #[cfg(not(any(target_env = "msvc", feature = "no_unique_address")))]
    macro_rules! cppgc_no_unique_address {
        () => {};
    }
    
    #[cfg(feature = "unused")]
    macro_rules! cppgc_unused {
        () => {
            #[allow(unused)]
        };
    }

    #[cfg(not(feature = "unused"))]
    macro_rules! cppgc_unused {
        () => {};
    }
}
