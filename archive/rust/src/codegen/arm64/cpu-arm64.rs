// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for arm independent of OS goes here.

#[cfg(target_arch = "aarch64")]
mod cpu_arm64 {
    //use crate::codegen::arm64::utils_arm64; // Assuming a corresponding Rust module exists
    //use crate::codegen::cpu_features; // Assuming a corresponding Rust module exists
    //use crate::codegen::flush_instruction_cache; // Assuming a corresponding Rust module exists
    
    #[cfg(target_os = "macos")]
    extern crate libc;
    
    // TODO: Find an equivalent Rust crate for <windows.h> functionality
    #[cfg(target_os = "windows")]
    extern crate winapi;

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    use std::arch::asm;

    struct CacheLineSizes {
        cache_type_register: u32,
    }

    impl CacheLineSizes {
        fn new() -> Self {
            #[cfg(not(all(target_arch = "aarch64", not(target_os = "windows"), not(target_os = "macos"))))]
            let cache_type_register = 0;

            #[cfg(all(target_arch = "aarch64", not(target_os = "windows"), not(target_os = "macos")))]
            let cache_type_register: u32;
            #[cfg(all(target_arch = "aarch64", not(target_os = "windows"), not(target_os = "macos")))]
            unsafe {
                 asm!(
                    "mrs {ctr}, ctr_el0",
                    ctr = out(reg) cache_type_register,
                );
            }


            CacheLineSizes {
                cache_type_register,
            }
        }

        fn icache_line_size(&self) -> u32 {
            self.extract_cache_line_size(0)
        }

        fn dcache_line_size(&self) -> u32 {
            self.extract_cache_line_size(16)
        }

        fn extract_cache_line_size(&self, cache_line_size_shift: i32) -> u32 {
            // The cache type register holds the size of cache lines in words as a
            // power of two.
            4 << ((self.cache_type_register >> cache_line_size_shift) & 0xF)
        }
    }

    pub struct CpuFeatures;

    impl CpuFeatures {
        pub fn flush_icache(address: *mut std::ffi::c_void, length: usize) {
            #[cfg(target_arch = "aarch64")]
            {
                #[cfg(target_os = "windows")]
                unsafe {
                    use winapi::um::processthreadsapi::GetCurrentProcess;
                    use winapi::um::memoryapi::FlushInstructionCache;
                    FlushInstructionCache(GetCurrentProcess(), address, length);
                }
                #[cfg(target_os = "macos")]
                unsafe {
                    use libc::sys_icache_invalidate;
                    sys_icache_invalidate(address, length);
                }
                #[cfg(target_os = "linux")]
                unsafe {
                    let begin = address as *mut u8;
                    std::arch::asm!(
                        "clflush",
                        in("rdi") begin,
                        options(nostack, preserves_flags)
                    );
                }
                #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
                {
                    // The code below assumes user space cache operations are allowed. The goal
                    // of this routine is to make sure the code generated is visible to the I
                    // side of the CPU.

                    let start = address as usize;
                    // Sizes will be used to generate a mask big enough to cover a pointer.
                    let sizes = CacheLineSizes::new();
                    let dsize = sizes.dcache_line_size() as usize;
                    let isize = sizes.icache_line_size() as usize;

                    // Cache line sizes are always a power of 2.
                    // TODO: Implement CountSetBits or find an equivalent function
                    //assert_eq!(utils_arm64::CountSetBits(dsize as u64, 64), 1);
                    //assert_eq!(utils_arm64::CountSetBits(isize as u64, 64), 1);

                    let dstart = start & !(dsize - 1);
                    let istart = start & !(isize - 1);
                    let end = start + length;

                    unsafe {
                        asm!(
                            // Clean every line of the D cache containing the target data.
                            "0:",
                            // dc       : Data Cache maintenance
                            //    c     : Clean
                            //     i    : Invalidate
                            //      va  : by (Virtual) Address
                            //        c : to the point of Coherency
                            // See ARM DDI 0406B page B2-12 for more information.
                            // We would prefer to use "cvau" (clean to the point of unification) here
                            // but we use "civac" to work around Cortex-A53 errata 819472, 826319,
                            // 827319 and 824069.
                            "dc   civac, {dline}",
                            "add  {dline}, {dline}, {dsize}",
                            "cmp  {dline}, {end}",
                            "b.lt 0b",
                            // Barrier to make sure the effect of the code above is visible to the
                            // rest of the world. dsb    : Data Synchronisation Barrier
                            //    ish : Inner SHareable domain
                            // The point of unification for an Inner Shareable shareability domain is
                            // the point by which the instruction and data caches of all the
                            // processors in that Inner Shareable shareability domain are guaranteed
                            // to see the same copy of a memory location.  See ARM DDI 0406B page
                            // B2-12 for more information.
                            "dsb  ish",
                            // Invalidate every line of the I cache containing the target data.
                            "1:",
                            // ic      : instruction cache maintenance
                            //    i    : invalidate
                            //     va  : by address
                            //       u : to the point of unification
                            "ic   ivau, {iline}",
                            "add  {iline}, {iline}, {isize}",
                            "cmp  {iline}, {end}",
                            "b.lt 1b",
                            // Barrier to make sure the effect of the code above is visible to the
                            // rest of the world.
                            "dsb  ish",
                            // Barrier to ensure any prefetching which happened before this code is
                            // discarded.
                            // isb : Instruction Synchronisation Barrier
                            "isb",
                            dline = inout(reg) dstart,
                            iline = inout(reg) istart,
                            dsize = in(reg) dsize,
                            isize = in(reg) isize,
                            end = in(reg) end,
                            options(preserves_flags, nostack, nomem),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
pub use cpu_arm64::CpuFeatures;

#[cfg(not(target_arch = "aarch64"))]
pub struct CpuFeatures;

#[cfg(not(target_arch = "aarch64"))]
impl CpuFeatures {
    pub fn flush_icache(_address: *mut std::ffi::c_void, _length: usize) {
        // Do nothing if not on ARM64.
    }
}