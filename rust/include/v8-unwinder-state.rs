// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8 {

    #[cfg(target_arch = "arm")]
    #[repr(C)]
    pub struct CalleeSavedRegisters {
        pub arm_r4: *mut std::ffi::c_void,
        pub arm_r5: *mut std::ffi::c_void,
        pub arm_r6: *mut std::ffi::c_void,
        pub arm_r7: *mut std::ffi::c_void,
        pub arm_r8: *mut std::ffi::c_void,
        pub arm_r9: *mut std::ffi::c_void,
        pub arm_r10: *mut std::ffi::c_void,
    }

    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "loongarch64",
        target_arch = "riscv32"
    ))]
    #[repr(C)]
    pub struct CalleeSavedRegisters {}

    #[cfg(not(any(
        target_arch = "arm",
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "loongarch64",
        target_arch = "riscv32"
    )))]
    compile_error!("Target architecture was not detected as supported by v8");
}