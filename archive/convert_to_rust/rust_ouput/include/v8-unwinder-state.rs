// Converted from V8 C++ source files:
// Header: v8-unwinder-state.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {

    #[cfg(all(
        target_arch = "arm",
        not(target_os = "windows")
    ))]
    #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Copy, Clone)]
    pub struct CalleeSavedRegisters {}
}
