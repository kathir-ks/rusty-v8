// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Equivalent of V8_BASE_BUILD_CONFIG_H_

// include/v8config.h is assumed to be handled externally during compilation

// These flags are typically set during compilation based on the target architecture.
// In Rust, we would likely handle these with conditional compilation using cfg attributes.
// For example:
// #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
// const CAN_USE_ARMV7_INSTRUCTIONS: bool = true;

// Define constants based on preprocessor defines
// NOTE: The actual values will depend on the build configuration and target architecture.
//       These are placeholders and should be replaced with actual cfg-based logic.

#[cfg(any(
    feature = "arm_arch_7a",
    feature = "arm_arch_7r",
    feature = "arm_arch_7"
))]
const CAN_USE_ARMV7_INSTRUCTIONS: bool = true;
#[cfg(not(any(
    feature = "arm_arch_7a",
    feature = "arm_arch_7r",
    feature = "arm_arch_7"
)))]
const CAN_USE_ARMV7_INSTRUCTIONS: bool = false;


#[cfg(feature = "arm_arch_ext_idiv")]
const CAN_USE_SUDIV: bool = true;
#[cfg(not(feature = "arm_arch_ext_idiv"))]
const CAN_USE_SUDIV: bool = false;

#[cfg(not(feature = "can_use_vfp3_instructions"))]
const CAN_USE_VFP3_INSTRUCTIONS: bool = true;
#[cfg(feature = "can_use_vfp3_instructions")]
const CAN_USE_VFP3_INSTRUCTIONS: bool = true;

#[cfg(feature = "arm_arch_8a")]
const CAN_USE_ARMV8_INSTRUCTIONS: bool = true;
#[cfg(not(feature = "arm_arch_8a"))]
const CAN_USE_ARMV8_INSTRUCTIONS: bool = false;


#[cfg(all(feature = "v8_host_arch_arm64", feature = "v8_os_macos"))]
const V8_HAS_PTHREAD_JIT_WRITE_PROTECT: bool = true;
#[cfg(not(all(feature = "v8_host_arch_arm64", feature = "v8_os_macos")))]
const V8_HAS_PTHREAD_JIT_WRITE_PROTECT: bool = false;

#[cfg(all(
    feature = "v8_host_arch_arm64",
    feature = "v8_os_ios",
    feature = "iphone_17_4",
    feature = "min_iphone_os_version_17_4"
))]
const V8_HAS_BECORE_JIT_WRITE_PROTECT: bool = true;
#[cfg(not(all(
    feature = "v8_host_arch_arm64",
    feature = "v8_os_ios",
    feature = "iphone_17_4",
    feature = "min_iphone_os_version_17_4"
)))]
const V8_HAS_BECORE_JIT_WRITE_PROTECT: bool = false;

#[cfg(all(feature = "v8_os_linux", feature = "v8_host_arch_x64"))]
const V8_HAS_PKU_JIT_WRITE_PROTECT: bool = true;
#[cfg(not(all(feature = "v8_os_linux", feature = "v8_host_arch_x64")))]
const V8_HAS_PKU_JIT_WRITE_PROTECT: bool = false;

#[cfg(any(feature = "v8_target_arch_ia32", feature = "v8_target_arch_x64"))]
const V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK: bool = true;
#[cfg(not(any(feature = "v8_target_arch_ia32", feature = "v8_target_arch_x64")))]
const V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK: bool = false;

const K_RETURN_ADDRESS_STACK_SLOT_COUNT: i32 =
    if V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK {
        1
    } else {
        0
    };

#[cfg(all(feature = "v8_host_arch_ppc64", not(feature = "v8_os_aix")))]
const K_PAGE_SIZE_BITS: i32 = 19;
#[cfg(feature = "enable_hugepage")]
const K_HUGE_PAGE_BITS: i32 = 21;
#[cfg(feature = "enable_hugepage")]
const K_HUGE_PAGE_SIZE: i32 = 1 << K_HUGE_PAGE_BITS;
#[cfg(feature = "enable_hugepage")]
const K_PAGE_SIZE_BITS: i32 = K_HUGE_PAGE_BITS;
#[cfg(not(any(all(feature = "v8_host_arch_ppc64", not(feature = "v8_os_aix")),feature = "enable_hugepage")))]
const K_PAGE_SIZE_BITS: i32 = 18;

const K_REGULAR_PAGE_SIZE: i32 = 1 << K_PAGE_SIZE_BITS;

#[cfg(any(
    all(feature = "v8_os_macos", feature = "v8_host_arch_arm64"),
    all(
        feature = "v8_os_android",
        any(feature = "v8_host_arch_arm64", feature = "v8_host_arch_x64")
    ),
    feature = "v8_host_arch_loong64",
    feature = "v8_host_arch_mips64",
    feature = "v8_os_ios"
))]
const K_MINIMUM_OS_PAGE_SIZE: i32 = 16 * 1024;
#[cfg(all(
    feature = "v8_os_linux",
    not(feature = "v8_os_android"),
    any(feature = "v8_host_arch_arm64", feature = "v8_host_arch_ppc64")
))]
const K_MINIMUM_OS_PAGE_SIZE: i32 = 64 * 1024;
#[cfg(not(any(
    any(
        all(feature = "v8_os_macos", feature = "v8_host_arch_arm64"),
        all(
            feature = "v8_os_android",
            any(feature = "v8_host_arch_arm64", feature = "v8_host_arch_x64")
        ),
        feature = "v8_host_arch_loong64",
        feature = "v8_host_arch_mips64",
        feature = "v8_os_ios"
    ),
    all(
        feature = "v8_os_linux",
        not(feature = "v8_os_android"),
        any(feature = "v8_host_arch_arm64", feature = "v8_host_arch_ppc64")
    )
)))]
const K_MINIMUM_OS_PAGE_SIZE: i32 = 4 * 1024;