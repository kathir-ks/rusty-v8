// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Consider using a build script to handle conditional compilation
// based on compiler, OS, and architecture.

// Example of a simple macro for conditional compilation based on OS
macro_rules! cfg_os {
    ($os:ident, $($item:item)*) => {
        #[cfg(target_os = stringify!($os))]
        $(
            $item
        )*
    }
}

// Example of a simple macro for conditional compilation based on architecture
macro_rules! cfg_arch {
    ($arch:ident, $($item:item)*) => {
        #[cfg(target_arch = stringify!($arch))]
        $(
            $item
        )*
    }
}

// Define constants for operating systems
cfg_os!(android, pub const V8_OS_ANDROID: bool = true; pub const V8_OS_LINUX: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "android";);
cfg_os!(macos, pub const V8_OS_POSIX: bool = true; pub const V8_OS_BSD: bool = true; pub const V8_OS_DARWIN: bool = true; pub const V8_OS_MACOS: bool = true; pub const V8_OS_STRING: &str = "macos";);
cfg_os!(ios, pub const V8_OS_POSIX: bool = true; pub const V8_OS_BSD: bool = true; pub const V8_OS_DARWIN: bool = true; pub const V8_OS_IOS: bool = true; pub const V8_OS_STRING: &str = "ios";);
cfg_os!(cygwin, pub const V8_OS_CYGWIN: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "cygwin";);
cfg_os!(linux, pub const V8_OS_LINUX: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "linux";);
cfg_os!(solaris, pub const V8_OS_POSIX: bool = true; pub const V8_OS_SOLARIS: bool = true; pub const V8_OS_STRING: &str = "sun";);
cfg_os!(starboard, pub const V8_OS_STARBOARD: bool = true; pub const V8_OS_STRING: &str = "starboard";);
cfg_os!(aix, pub const V8_OS_POSIX: bool = true; pub const V8_OS_AIX: bool = true; pub const V8_OS_STRING: &str = "aix";);
cfg_os!(freebsd, pub const V8_OS_BSD: bool = true; pub const V8_OS_FREEBSD: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "freebsd";);
cfg_os!(fuchsia, pub const V8_OS_FUCHSIA: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "fuchsia";);
cfg_os!(dragonfly, pub const V8_OS_BSD: bool = true; pub const V8_OS_DRAGONFLYBSD: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "dragonflybsd";);
cfg_os!(netbsd, pub const V8_OS_BSD: bool = true; pub const V8_OS_NETBSD: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "netbsd";);
cfg_os!(openbsd, pub const V8_OS_BSD: bool = true; pub const V8_OS_OPENBSD: bool = true; pub const V8_OS_POSIX: bool = true; pub const V8_OS_STRING: &str = "openbsd";);
cfg_os!(qnx, pub const V8_OS_POSIX: bool = true; pub const V8_OS_QNX: bool = true; pub const V8_OS_STRING: &str = "qnx";);
cfg_os!(windows, pub const V8_OS_WIN: bool = true; pub const V8_OS_STRING: &str = "windows";);
cfg_os!(zos, pub const V8_OS_POSIX: bool = true; pub const V8_OS_ZOS: bool = true; pub const V8_OS_STRING: &str = "zos";);

// Define a default OS if none of the above are defined
#[cfg(not(any(target_os = "android", target_os = "macos", target_os = "ios", target_os = "cygwin", target_os = "linux", target_os = "solaris", target_os = "starboard", target_os = "aix", target_os = "freebsd", target_os = "fuchsia", target_os = "dragonfly", target_os = "netbsd", target_os = "openbsd", target_os = "qnx", target_os = "windows", target_os = "zos")))]
pub const V8_OS_UNKNOWN: bool = true;

//Define constants for target operating systems
#[cfg(all(V8_OS_ANDROID, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_ANDROID: bool = true;

#[cfg(all(V8_OS_FUCHSIA, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_FUCHSIA: bool = true;

#[cfg(all(V8_OS_IOS, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_IOS: bool = true;

#[cfg(all(V8_OS_LINUX, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_LINUX: bool = true;

#[cfg(all(V8_OS_MACOS, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_MACOS: bool = true;

#[cfg(all(V8_OS_WIN, not(defined(V8_HAVE_TARGET_OS))))]
pub const V8_TARGET_OS_WIN: bool = true;

//Define target OS string constants
#[cfg(V8_TARGET_OS_ANDROID)]
pub const V8_TARGET_OS_STRING: &str = "android";
#[cfg(V8_TARGET_OS_FUCHSIA)]
pub const V8_TARGET_OS_STRING: &str = "fuchsia";
#[cfg(V8_TARGET_OS_IOS)]
pub const V8_TARGET_OS_STRING: &str = "ios";
#[cfg(V8_TARGET_OS_LINUX)]
pub const V8_TARGET_OS_STRING: &str = "linux";
#[cfg(V8_TARGET_OS_MACOS)]
pub const V8_TARGET_OS_STRING: &str = "macos";
#[cfg(V8_TARGET_OS_WIN)]
pub const V8_TARGET_OS_STRING: &str = "windows";
#[cfg(not(any(V8_TARGET_OS_ANDROID, V8_TARGET_OS_FUCHSIA, V8_TARGET_OS_IOS, V8_TARGET_OS_LINUX, V8_TARGET_OS_MACOS, V8_TARGET_OS_WIN)))]
pub const V8_TARGET_OS_STRING: &str = "unknown";

// Compiler detection macros (using cfg attributes)
#[cfg(all(target_env = "msvc", target_family = "windows"))]
pub const V8_CC_MSVC: bool = true;

#[cfg(all(target_family = "unix", not(target_env = "msvc")))]
pub const V8_CC_GNU: bool = true;

//Define architecture constants
cfg_arch!(x86_64, pub const V8_HOST_ARCH_X64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(x86, pub const V8_HOST_ARCH_IA32: bool = true; pub const V8_HOST_ARCH_32_BIT: bool = true;);
cfg_arch!(aarch64, pub const V8_HOST_ARCH_ARM64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(arm, pub const V8_HOST_ARCH_ARM: bool = true; pub const V8_HOST_ARCH_32_BIT: bool = true;);
cfg_arch!(mips64, pub const V8_HOST_ARCH_MIPS64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(loongarch64, pub const V8_HOST_ARCH_LOONG64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(powerpc64, pub const V8_HOST_ARCH_PPC64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(s390x, pub const V8_HOST_ARCH_S390X: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(riscv64, pub const V8_HOST_ARCH_RISCV64: bool = true; pub const V8_HOST_ARCH_64_BIT: bool = true;);
cfg_arch!(riscv32, pub const V8_HOST_ARCH_RISCV32: bool = true; pub const V8_HOST_ARCH_32_BIT: bool = true;);

//Target architecture
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(x86_64, pub const V8_TARGET_ARCH_X64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(x86, pub const V8_TARGET_ARCH_IA32: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(aarch64, pub const V8_TARGET_ARCH_ARM64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(arm, pub const V8_TARGET_ARCH_ARM: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(mips64, pub const V8_TARGET_ARCH_MIPS64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(loongarch64, pub const V8_TARGET_ARCH_LOONG64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(powerpc64, pub const V8_TARGET_ARCH_PPC64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(s390x, pub const V8_TARGET_ARCH_S390X: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(riscv64, pub const V8_TARGET_ARCH_RISCV64: bool = true;);
#[cfg(not(any(V8_TARGET_ARCH_X64, V8_TARGET_ARCH_IA32, V8_TARGET_ARCH_ARM, V8_TARGET_ARCH_ARM64, V8_TARGET_ARCH_MIPS64, V8_TARGET_ARCH_PPC64, V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_LOONG64, V8_TARGET_ARCH_RISCV32)))]
cfg_arch!(riscv32, pub const V8_TARGET_ARCH_RISCV32: bool = true;);

//Architecture pointer size
#[cfg(V8_TARGET_ARCH_IA32)]
pub const V8_TARGET_ARCH_32_BIT: bool = true;
#[cfg(all(V8_TARGET_ARCH_X64, not(V8_TARGET_ARCH_32_BIT), not(V8_TARGET_ARCH_64_BIT)))]
cfg_arch!(x86_64, pub const V8_TARGET_ARCH_32_BIT: bool = true;);
#[cfg(all(V8_TARGET_ARCH_X64, not(V8_TARGET_ARCH_32_BIT), not(V8_TARGET_ARCH_64_BIT)))]
cfg_arch!(x86_64, pub const V8_TARGET_ARCH_64_BIT: bool = true;);
#[cfg(V8_TARGET_ARCH_ARM)]
pub const V8_TARGET_ARCH_32_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_ARM64)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_MIPS)]
pub const V8_TARGET_ARCH_32_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_MIPS64)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_LOONG64)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_PPC64)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_S390X)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_RISCV64)]
pub const V8_TARGET_ARCH_64_BIT: bool = true;
#[cfg(V8_TARGET_ARCH_RISCV32)]
pub const V8_TARGET_ARCH_32_BIT: bool = true;

//Endianness
#[cfg(V8_TARGET_ARCH_IA32)]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(V8_TARGET_ARCH_X64)]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(V8_TARGET_ARCH_ARM)]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(V8_TARGET_ARCH_ARM64)]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(V8_TARGET_ARCH_LOONG64)]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_MIPS64, not(defined(V8_TARGET_ARCH_MIPS64_BE))))]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_MIPS64, defined(V8_TARGET_ARCH_MIPS64_BE)))]
pub const V8_TARGET_BIG_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_PPC64, V8_OS_AIX))]
pub const V8_TARGET_BIG_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_PPC64, not(V8_OS_AIX)))]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_S390X, V8_TARGET_ARCH_S390X_LE_SIM))]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(all(V8_TARGET_ARCH_S390X, not(V8_TARGET_ARCH_S390X_LE_SIM)))]
pub const V8_TARGET_BIG_ENDIAN: bool = true;
#[cfg(any(V8_TARGET_ARCH_RISCV32, V8_TARGET_ARCH_RISCV64))]
pub const V8_TARGET_LITTLE_ENDIAN: bool = true;

// Static roots and endianness boolean
#[cfg(not(defined(V8_STATIC_ROOTS)))]
pub const V8_STATIC_ROOTS_BOOL: bool = false;
#[cfg(defined(V8_STATIC_ROOTS))]
pub const V8_STATIC_ROOTS_BOOL: bool = true;

#[cfg(V8_TARGET_BIG_ENDIAN)]
pub const V8_TARGET_BIG_ENDIAN_BOOL: bool = true;
#[cfg(not(V8_TARGET_BIG_ENDIAN))]
pub const V8_TARGET_BIG_ENDIAN_BOOL: bool = false;

// Inline macro (simpler version for demonstration)
#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn v8_inline<T>(f: impl Fn() -> T) -> T {
    f()
}

#[cfg(debug_assertions)]
#[inline]
pub fn v8_inline<T>(f: impl Fn() -> T) -> T {
    f()
}

// No-inline macro (simpler version for demonstration)
#[inline(never)]
pub fn v8_noinline<T>(f: impl Fn() -> T) -> T {
    f()
}

// Warn unused result
#[must_use]
pub fn v8_warn_unused_result<T>(f: impl Fn() -> T) -> T {
    f()
}

//Unlikely and likely
#[inline]
pub fn v8_unlikely(condition: bool) -> bool {
    if condition {
        false
    } else {
        true
    }
}

#[inline]
pub fn v8_likely(condition: bool) -> bool {
    condition
}