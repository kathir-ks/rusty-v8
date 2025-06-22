// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// The `yield_processor` macro (function) wraps an architecture-specific
/// instruction that informs the processor we're in a busy wait, so it can
/// handle the branch more intelligently and e.g. reduce power to our core or
/// give more resources to the other hyper-thread on this core. See the
/// following for context:
/// https://software.intel.com/en-us/articles/benefitting-power-and-performance-sleep-loops

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "macos"))]
macro_rules! yield_processor {
    () => {
        ()
    };
}

#[cfg(target_os = "linux")]
#[cfg(not(feature = "thread_sanitizer"))]
macro_rules! yield_processor {
    () => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            unsafe {
                core::arch::x86_64::_mm_pause();
            }
        }
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            unsafe {
                core::arch::aarch64::__yield();
            }
        }
        #[cfg(all(target_arch = "mips64", target_endian = "little_endian"))]
        {
            unsafe {
                core::arch::asm!("pause");
            }
        }
        #[cfg(target_arch = "powerpc64")]
        {
            unsafe {
                core::arch::asm!("or 31,31,31");
            }
        }
    };
}

#[cfg(target_os = "windows")]
#[cfg(not(feature = "thread_sanitizer"))]
macro_rules! yield_processor {
    () => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            unsafe {
                core::arch::x86_64::_mm_pause();
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            unsafe {
                core::arch::aarch64::__yield();
            }
        }
    };
}

#[cfg(target_os = "macos")]
#[cfg(not(feature = "thread_sanitizer"))]
macro_rules! yield_processor {
    () => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            unsafe {
                core::arch::x86_64::_mm_pause();
            }
        }
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            unsafe {
                core::arch::aarch64::__yield();
            }
        }
    };
}

#[cfg(feature = "thread_sanitizer")]
macro_rules! yield_processor {
    () => {
        std::thread::sleep(std::time::Duration::from_millis(1));
    };
}

pub(crate) use yield_processor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yield_processor() {
        yield_processor!();
    }
}