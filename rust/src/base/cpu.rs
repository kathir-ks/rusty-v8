// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO(you): Add corresponding Rust modules for the C++ headers
// src/base/logging.h
// src/base/platform/wrappers.h

#[cfg(target_os = "starboard")]
use starboard::cpu_features;

#[cfg(all(target_env = "msvc", target_arch = "x86_64"))]
use std::arch::x86_64::__cpuid;

#[cfg(all(target_env = "msvc", target_arch = "x86"))]
use std::arch::x86::__cpuid;

#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::Read;

#[cfg(any(target_os = "android", target_os = "linux"))]
use libc::getauxval;

#[cfg(target_os = "qnx")]
use libc::syspage_entry; // requires syspage.h, functionality likely needs OS-specific implementations

#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
use libc::Elf64_auxv_t;

#[cfg(target_os = "aix")]
use libc::_system_configuration;

#[cfg(target_os = "darwin")]
use libc::sysctlbyname;

#[cfg(target_family = "unix")]
use libc::sysconf;

use std::cmp;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;
use std::{
    fmt,
    str,
};

#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::IsProcessorFeaturePresent;

pub mod base {
    use std::{
        fmt,
        str,
    };
    /// Unknown cache line size.
    pub const K_UNKNOWN_CACHE_LINE_SIZE: i32 = 0;
    /// Unknown number of virtual address bits.
    pub const K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS: i32 = 0;

    #[derive(Debug, PartialEq, Eq)]
    pub enum RV_MMU_MODE {
        kRiscvSV48,
        kRiscvSV39,
        kRiscvSV57,
    }

    impl fmt::Display for RV_MMU_MODE {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RV_MMU_MODE::kRiscvSV48 => write!(f, "sv48"),
                RV_MMU_MODE::kRiscvSV39 => write!(f, "sv39"),
                RV_MMU_MODE::kRiscvSV57 => write!(f, "sv57"),
            }
        }
    }

    // These values are used to index the part_ field of CPU.
    pub const K_PPC_POWER8: i32 = 8;
    pub const K_PPC_POWER9: i32 = 9;
    pub const K_PPC_POWER10: i32 = 10;

    pub struct CPU {
        stepping_: i32,
        model_: i32,
        ext_model_: i32,
        family_: i32,
        ext_family_: i32,
        type_: i32,
        implementer_: i32,
        architecture_: i32,
        variant_: i32,
        part_: i32,
        icache_line_size_: i32,
        dcache_line_size_: i32,
        num_virtual_address_bits_: i32,
        has_fpu_: bool,
        has_cmov_: bool,
        has_sahf_: bool,
        has_mmx_: bool,
        has_sse_: bool,
        has_sse2_: bool,
        has_sse3_: bool,
        has_ssse3_: bool,
        has_sse41_: bool,
        has_sse42_: bool,
        is_atom_: bool,
        has_intel_jcc_erratum_: bool,
        has_osxsave_: bool,
        has_avx_: bool,
        has_avx2_: bool,
        has_avx_vnni_: bool,
        has_avx_vnni_int8_: bool,
        has_fma3_: bool,
        has_f16c_: bool,
        has_bmi1_: bool,
        has_bmi2_: bool,
        has_lzcnt_: bool,
        has_popcnt_: bool,
        has_idiva_: bool,
        has_neon_: bool,
        has_thumb2_: bool,
        has_vfp_: bool,
        has_vfp3_: bool,
        has_vfp3_d32_: bool,
        has_jscvt_: bool,
        has_dot_prod_: bool,
        has_lse_: bool,
        has_mte_: bool,
        has_pmull1q_: bool,
        has_fp16_: bool,
        is_fp64_mode_: bool,
        has_non_stop_time_stamp_counter_: bool,
        is_running_in_vm_: bool,
        has_msa_: bool,
        riscv_mmu_: RV_MMU_MODE,
        has_rvv_: bool,
        has_zba_: bool,
        has_zbb_: bool,
        has_zbs_: bool,
        vendor_: [u8; 16], // Increased the size to accommodate the null terminator and more
        has_cetss_: bool, //Added for CET shadow stack
    }

    impl CPU {
        /// Creates a new `CPU` instance and detects CPU features.
        pub fn new() -> Self {
            let mut cpu = CPU {
                stepping_: 0,
                model_: 0,
                ext_model_: 0,
                family_: 0,
                ext_family_: 0,
                type_: 0,
                implementer_: 0,
                architecture_: 0,
                variant_: -1,
                part_: 0,
                icache_line_size_: K_UNKNOWN_CACHE_LINE_SIZE,
                dcache_line_size_: K_UNKNOWN_CACHE_LINE_SIZE,
                num_virtual_address_bits_: K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS,
                has_fpu_: false,
                has_cmov_: false,
                has_sahf_: false,
                has_mmx_: false,
                has_sse_: false,
                has_sse2_: false,
                has_sse3_: false,
                has_ssse3_: false,
                has_sse41_: false,
                has_sse42_: false,
                is_atom_: false,
                has_intel_jcc_erratum_: false,
                has_osxsave_: false,
                has_avx_: false,
                has_avx2_: false,
                has_avx_vnni_: false,
                has_avx_vnni_int8_: false,
                has_fma3_: false,
                has_f16c_: false,
                has_bmi1_: false,
                has_bmi2_: false,
                has_lzcnt_: false,
                has_popcnt_: false,
                has_idiva_: false,
                has_neon_: false,
                has_thumb2_: false,
                has_vfp_: false,
                has_vfp3_: false,
                has_vfp3_d32_: false,
                has_jscvt_: false,
                has_dot_prod_: false,
                has_lse_: false,
                has_mte_: false,
                has_pmull1q_: false,
                has_fp16_: false,
                is_fp64_mode_: false,
                has_non_stop_time_stamp_counter_: false,
                is_running_in_vm_: false,
                has_msa_: false,
                riscv_mmu_: RV_MMU_MODE::kRiscvSV48,
                has_rvv_: false,
                has_zba_: false,
                has_zbb_: false,
                has_zbs_: false,
                vendor_: [0u8; 16],
                has_cetss_: false,
            };
            cpu.vendor_[..7].copy_from_slice(b"Unknown"); // Copy "Unknown" to the first 7 bytes
            cpu.vendor_[7] = 0; // Null-terminate the string

            #[cfg(target_os = "starboard")]
            {
                if cpu.starboard_detect_cpu() {
                    return cpu;
                }
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                let mut cpu_info = [0i32; 4];

                // __cpuid with an InfoType argument of 0 returns the number of
                // valid Ids in CPUInfo[0] and the CPU identification string in
                // the other three array elements. The CPU identification string is
                // not in linear order. The code below arranges the information
                // in a human readable form. The human readable order is CPUInfo[1] |
                // CPUInfo[3] | CPUInfo[2]. CPUInfo[2] and CPUInfo[3] are swapped
                // before using memcpy to copy these three array elements to cpu_string.
                unsafe {
                    __cpuid(cpu_info.as_mut_ptr(), 0);
                }
                let num_ids = cpu_info[0] as u32;
                cpu_info.swap(2, 3);
                cpu.vendor_[0..4].copy_from_slice(&(cpu_info[1] as u32).to_le_bytes());
                cpu.vendor_[4..8].copy_from_slice(&(cpu_info[2] as u32).to_le_bytes());
                cpu.vendor_[8..12].copy_from_slice(&(cpu_info[3] as u32).to_le_bytes());
                cpu.vendor_[12] = 0; // Null terminate

                // Interpret CPU feature information.
                if num_ids > 0 {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), 1);
                    }

                    let mut cpu_info70 = [0i32; 4];
                    let mut cpu_info71 = [0i32; 4];
                    if num_ids >= 7 {
                        unsafe {
                            __cpuid(cpu_info70.as_mut_ptr(), 7);
                            // Check the maximum input value for supported leaf 7 sub-leaves
                            if cpu_info70[0] >= 1 {
                                __cpuidex(cpu_info71.as_mut_ptr(), 7, 1);
                            }
                        }
                    }

                    cpu.stepping_ = cpu_info[0] & 0xF;
                    cpu.model_ = ((cpu_info[0] >> 4) & 0xF) + ((cpu_info[0] >> 12) & 0xF0);
                    cpu.family_ = (cpu_info[0] >> 8) & 0xF;
                    cpu.type_ = (cpu_info[0] >> 12) & 0x3;
                    cpu.ext_model_ = (cpu_info[0] >> 16) & 0xF;
                    cpu.ext_family_ = (cpu_info[0] >> 20) & 0xFF;
                    cpu.has_fpu_ = (cpu_info[3] & 0x00000001) != 0;
                    cpu.has_cmov_ = (cpu_info[3] & 0x00008000) != 0;
                    cpu.has_mmx_ = (cpu_info[3] & 0x00800000) != 0;
                    cpu.has_sse_ = (cpu_info[3] & 0x02000000) != 0;
                    cpu.has_sse2_ = (cpu_info[3] & 0x04000000) != 0;
                    cpu.has_sse3_ = (cpu_info[2] & 0x00000001) != 0;
                    cpu.has_ssse3_ = (cpu_info[2] & 0x00000200) != 0;
                    cpu.has_sse41_ = (cpu_info[2] & 0x00080000) != 0;
                    cpu.has_sse42_ = (cpu_info[2] & 0x00100000) != 0;
                    cpu.has_popcnt_ = (cpu_info[2] & 0x00800000) != 0;
                    cpu.has_osxsave_ = (cpu_info[2] & 0x08000000) != 0;
                    cpu.has_avx_ = (cpu_info[2] & 0x10000000) != 0;
                    cpu.has_avx2_ = (cpu_info70[1] & 0x00000020) != 0;
                    cpu.has_avx_vnni_ = (cpu_info71[0] & 0x00000010) != 0;
                    cpu.has_avx_vnni_int8_ = (cpu_info71[3] & 0x00000020) != 0;
                    cpu.has_fma3_ = (cpu_info[2] & 0x00001000) != 0;
                    cpu.has_f16c_ = (cpu_info[2] & 0x20000000) != 0;
                    cpu.has_cetss_ = (cpu_info70[2] & 0x00000080) != 0;

                    // "Hypervisor Present Bit: Bit 31 of ECX of CPUID leaf 0x1."
                    // See https://lwn.net/Articles/301888/
                    // This is checking for any hypervisor. Hypervisors may choose not to
                    // announce themselves. Hypervisors trap CPUID and sometimes return
                    // different results to underlying hardware.
                    cpu.is_running_in_vm_ = (cpu_info[2] & 0x80000000) != 0;

                    if cpu.family_ == 0x6 {
                        match cpu.model_ {
                            0x1C | 0x26 | 0x36 | 0x27 | 0x35 | 0x37 | 0x4A | 0x4D | 0x4C | 0x6E => {
                                cpu.is_atom_ = true;
                            }
                            _ => {}
                        }

                        // CPUs that are affected by Intel JCC erratum:
                        // https://www.intel.com/content/dam/support/us/en/documents/processors/mitigations-jump-conditional-code-erratum.pdf
                        match cpu.model_ {
                            0x4E => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x3;
                            }
                            0x55 => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x4 || cpu.stepping_ == 0x7;
                            }
                            0x5E => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x3;
                            }
                            0x8E => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x9 || cpu.stepping_ == 0xA || cpu.stepping_ == 0xB || cpu.stepping_ == 0xC;
                            }
                            0x9E => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x9 || cpu.stepping_ == 0xA || cpu.stepping_ == 0xB || cpu.stepping_ == 0xD;
                            }
                            0xA6 => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x0;
                            }
                            0xAE => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0xA;
                            }
                            _ => {}
                        }
                    }
                }

                // There are separate feature flags for VEX-encoded GPR instructions.
                if num_ids >= 7 {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), 7);
                    }
                    cpu.has_bmi1_ = (cpu_info[1] & 0x00000008) != 0;
                    cpu.has_bmi2_ = (cpu_info[1] & 0x00000100) != 0;
                }

                // Query extended IDs.
                unsafe {
                    __cpuid(cpu_info.as_mut_ptr(), 0x80000000);
                }
                let num_ext_ids = cpu_info[0] as u32;

                // Interpret extended CPU feature information.
                if num_ext_ids > 0x80000000 {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), 0x80000001);
                    }
                    cpu.has_lzcnt_ = (cpu_info[2] & 0x00000020) != 0;
                    // SAHF must be probed in long mode.
                    cpu.has_sahf_ = (cpu_info[2] & 0x00000001) != 0;
                }

                // Check if CPU has non stoppable time stamp counter.
                const PARAMETER_CONTAINING_NON_STOP_TIME_STAMP_COUNTER: u32 = 0x80000007;
                if num_ext_ids >= PARAMETER_CONTAINING_NON_STOP_TIME_STAMP_COUNTER {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), PARAMETER_CONTAINING_NON_STOP_TIME_STAMP_COUNTER as i32);
                    }
                    cpu.has_non_stop_time_stamp_counter_ = (cpu_info[3] & (1 << 8)) != 0;
                }

                const VIRTUAL_PHYSICAL_ADDRESS_BITS: u32 = 0x80000008;
                if num_ext_ids >= VIRTUAL_PHYSICAL_ADDRESS_BITS {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), VIRTUAL_PHYSICAL_ADDRESS_BITS as i32);
                    }
                    cpu.num_virtual_address_bits_ = (cpu_info[0] >> 8) & 0xff;
                }

                // This logic is replicated from cpu.cc present in chromium.src
                if !cpu.has_non_stop_time_stamp_counter_ && cpu.is_running_in_vm_ {
                    let mut cpu_info_hv = [0i32; 4];
                    unsafe {
                        __cpuid(cpu_info_hv.as_mut_ptr(), 0x40000000);
                    }
                    if cpu_info_hv[1] == 0x7263694D &&  // Micr
                       cpu_info_hv[2] == 0x666F736F &&  // osof
                       cpu_info_hv[3] == 0x76482074 {
                        // If CPUID says we have a variant TSC and a hypervisor has identified
                        // itself and the hypervisor says it is Microsoft Hyper-V, then treat
                        // TSC as invariant.
                        //
                        // Microsoft Hyper-V hypervisor reports variant TSC as there are some
                        // scenarios (eg. VM live migration) where the TSC is variant, but for
                        // our purposes we can treat it as invariant.
                        cpu.has_non_stop_time_stamp_counter_ = true;
                    }
                }
            }
            #[cfg(target_arch = "arm")]
            {
                #[cfg(target_os = "linux")]
                {
                    let cpu_info_struct = CPUInfo::new();

                    // Extract implementer from the "CPU implementer" field.
                    if let Some(implementer) = cpu_info_struct.extract_field("CPU implementer") {
                        if let Ok(value) = implementer.parse::<i32>() {
                            cpu.implementer_ = value;
                        }
                    }

                    if let Some(variant) = cpu_info_struct.extract_field("CPU variant") {
                        if let Ok(value) = variant.parse::<i32>() {
                            cpu.variant_ = value;
                        }
                    }

                    // Extract part number from the "CPU part" field.
                    if let Some(part) = cpu_info_struct.extract_field("CPU part") {
                        if let Ok(value) = part.parse::<i32>() {
                            cpu.part_ = value;
                        }
                    }

                    // Extract architecture from the "CPU Architecture" field.
                    // The list is well-known, unlike the the output of
                    // the 'Processor' field which can vary greatly.
                    // See the definition of the 'proc_arch' array in
                    // $KERNEL/arch/arm/kernel/setup.c and the 'c_show' function in
                    // same file.
                    if let Some(architecture) = cpu_info_struct.extract_field("CPU architecture") {
                        if let Ok(value) = architecture.parse::<i32>() {
                            cpu.architecture_ = value;
                        } else {
                            // Kernels older than 3.18 report "CPU architecture: AArch64" on ARMv8.
                            if architecture == "AArch64" {
                                cpu.architecture_ = 8;
                            } else {
                                cpu.architecture_ = 0;
                            }
                        }

                        // Unfortunately, it seems that certain ARMv6-based CPUs
                        // report an incorrect architecture number of 7!
                        //
                        // See http://code.google.com/p/android/issues/detail?id=10812
                        //
                        // We try to correct this by looking at the 'elf_platform'
                        // field reported by the 'Processor' field, which is of the
                        // form of "(v7l)" for an ARMv7-based CPU, and "(v6l)" for
                        // an ARMv6-one. For example, the Raspberry Pi is one popular
                        // ARMv6 device that reports architecture 7.
                        if cpu.architecture_ == 7 {
                            if let Some(processor) = cpu_info_struct.extract_field("Processor") {
                                if has_list_item(&processor, "(v6l)") {
                                    cpu.architecture_ = 6;
                                }
                            }

                            // elf_platform moved to the model name field in Linux v3.8.
                            if cpu.architecture_ == 7 {
                                if let Some(processor) = cpu_info_struct.extract_field("model name") {
                                    if has_list_item(&processor, "(v6l)") {
                                        cpu.architecture_ = 6;
                                    }
                                }
                            }
                        }
                    }

                    // Try to extract the list of CPU features from ELF hwcaps.
                    let (hwcaps, hwcaps2) = read_elf_hwcaps();
                    if hwcaps != 0 {
                        cpu.has_idiva_ = (hwcaps & HWCAP_IDIVA) != 0;
                        cpu.has_neon_ = (hwcaps & HWCAP_NEON) != 0;
                        cpu.has_vfp_ = (hwcaps & HWCAP_VFP) != 0;
                        cpu.has_vfp3_ = (hwcaps & (HWCAP_VFPV3 | HWCAP_VFPV3D16 | HWCAP_VFPV4)) != 0;
                        cpu.has_vfp3_d32_ = (cpu.has_vfp3_ && ((hwcaps & HWCAP_VFPV3D16) == 0 ||
                                                           (hwcaps & HWCAP_VFPD32) != 0));
                    } else {
                        // Try to fallback to "Features" CPUInfo field.
                        if let Some(features) = cpu_info_struct.extract_field("Features") {
                            cpu.has_idiva_ = has_list_item(&features, "idiva");
                            cpu.has_neon_ = has_list_item(&features, "neon");
                            cpu.has_thumb2_ = has_list_item(&features, "thumb2");
                            cpu.has_vfp_ = has_list_item(&features, "vfp");
                            if has_list_item(&features, "vfpv3d16") {
                                cpu.has_vfp3_ = true;
                            } else if has_list_item(&features, "vfpv3") {
                                cpu.has_vfp3_ = true;
                                cpu.has_vfp3_d32_ = true;
                            }
                        }
                    }

                    // Some old kernels will report vfp not vfpv3. Here we make an attempt
                    // to detect vfpv3 by checking for vfp *and* neon, since neon is only
                    // available on architectures with vfpv3. Checking neon on its own is
                    // not enough as it is possible to have neon without vfp.
                    if cpu.has_vfp_ && cpu.has_neon_ {
                        cpu.has_vfp3_ = true;
                    }

                    // VFPv3 implies ARMv7, see ARM DDI 0406B, page A1-6.
                    if cpu.architecture_ < 7 && cpu.has_vfp3_ {
                        cpu.architecture_ = 7;
                    }

                    // ARMv7 implies Thumb2.
                    if cpu.architecture_ >= 7 {
                        cpu.has_thumb2_ = true;
                    }

                    // The earliest architecture with Thumb2 is ARMv6T2.
                    if cpu.has_thumb2_ && cpu.architecture_ < 6 {
                        cpu.architecture_ = 6;
                    }

                    // We don't support any FPUs other than VFP.
                    cpu.has_fpu_ = cpu.has_vfp_;
                }
                //TODO: Implement for other OS
            }
            #[cfg(target_arch = "mips64")]
            {
                #[cfg(target_os = "linux")]
                {
                    let cpu_info_struct = CPUInfo::new();
                    if let Some(cpu_model) = cpu_info_struct.extract_field("cpu model") {
                        cpu.has_fpu_ = has_list_item(&cpu_model, "FPU");
                    }
                    if let Some(ases) = cpu_info_struct.extract_field("ASEs implemented") {
                        cpu.has_msa_ = has_list_item(&ases, "msa");
                    }
                }
            }
            #[cfg(target_arch = "arm64")]
            {
                #[cfg(target_os = "windows")]
                {
                    // Windows makes high-resolution thread timing information available in
                    // user-space.
                    cpu.has_non_stop_time_stamp_counter_ = true;

                    // Defined in winnt.h, but only in 10.0.20348.0 version of the Windows SDK.
                    // Copy the value here to support older versions as well.
                    const PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE: i32 = 44;
                    const PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE: i32 = 43;
                    const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: i32 = 34;
                    const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: i32 = 30;

                    unsafe {
                        cpu.has_jscvt_ = IsProcessorFeaturePresent(PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE as u32) != 0;
                        cpu.has_dot_prod_ = IsProcessorFeaturePresent(PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE as u32) != 0;
                        cpu.has_lse_ = IsProcessorFeaturePresent(PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE as u32) != 0;
                        cpu.has_pmull1q_ = IsProcessorFeaturePresent(PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE as u32) != 0;
                    }
                }
                #[cfg(target_os = "linux")]
                {
                    // Try to extract the list of CPU features from ELF hwcaps.
                    let (hwcaps, hwcaps2) = read_elf_hwcaps();
                    cpu.has_mte_ = (hwcaps2 & HWCAP2_MTE) != 0;
                    if hwcaps != 0 {
                        cpu.has_jscvt_ = (hwcaps & HWCAP_JSCVT) != 0;
                        cpu.has_dot_prod_ = (hwcaps & HWCAP_ASIMDDP) != 0;
                        cpu.has_lse_ = (hwcaps & HWCAP_ATOMICS) != 0;
                        cpu.has_pmull1q_ = (hwcaps & HWCAP_PMULL) != 0;
                        cpu.has_fp16_ = (hwcaps & HWCAP_FPHP) != 0;
                    } else {
                        // Try to fallback to "Features" CPUInfo field
                        let cpu_info_struct = CPUInfo::new();
                        if let Some(features) = cpu_info_struct.extract_field("Features") {
                            cpu.has_jscvt_ = has_list_item(&features, "jscvt");
                            cpu.has_dot_prod_ = has_list_item(&features, "asimddp");
                            cpu.has_lse_ = has_list_item(&features, "atomics");
                            cpu.has_pmull1q_ = has_list_item(&features, "pmull");
                            cpu.has_fp16_ = has_list_item(&features, "half");
                        }
                    }
                }
            }
            #[cfg(target_arch = "powerpc64")]
            {
                #[cfg(target_os = "linux")]
                {
                    // Read processor info from /proc/self/auxv.
                    let mut auxv_cpu_type: Option<String> = None;
                    if let Ok(file) = File::open("/proc/self/auxv") {
                        let mut reader = std::io::BufReader::new(file);
                        loop {
                            let mut entry: Elf64_auxv_t = unsafe { mem::zeroed() };
                            let n = unsafe {
                                let ptr = &mut entry as *mut Elf64_auxv_t as *mut u8;
                                let s = slice::from_raw_parts_mut(ptr, mem::size_of::<Elf64_auxv_t>());
                                reader.read_exact(s).ok()?
                            };
                            if n == 0 || entry.a_type == libc::AT_NULL {
                                break;
                            }
                            match entry.a_type {
                                libc::AT_PLATFORM => {
                                    let ptr = entry.a_un.a_val as *const i8;
                                    let c_str = unsafe { CString::from_raw(ptr as *mut i8) };
                                    auxv_cpu_type = c_str.into_string().ok().map(|s| s.clone());
                                }
                                libc::AT_ICACHEBSIZE => {
                                    cpu.icache_line_size_ = entry.a_un.a_val as i32;
                                }
                                libc::AT_DCACHEBSIZE => {
                                    cpu.dcache_line_size_ = entry.a_un.a_val as i32;
                                }
                                _ => {}
                            }
                        }
                    }

                    cpu.part_ = -1;
                    if let Some(cpu_type) = auxv_cpu_type {
                        match cpu_type.as_str() {
                            "power10" => {
                                cpu.part_ = K_PPC_POWER10;
                            }
                            "power9" => {
                                cpu.part_ = K_PPC_POWER9;
                            }
                            "power8" => {
                                cpu.part_ = K_PPC_POWER8;
                            }
                            _ => {}
                        }
                    }
                }
            }
            #[cfg(target_arch = "riscv64")]
            {
                #[cfg(target_os = "linux")]
                {
                    let cpu_info = CPUInfo::new();
                    let mut has_fpu = false;
                    let mut has_rvv = false;
                    #[cfg(all(target_env = "gnu", target_os = "linux"))]
                    {
                        use libc::{syscall, __NR_riscv_hwprobe};
                        const RISCV_HWPROBE_KEY_IMA_EXT_0: i32 = 0;
                        const RISCV_HWPROBE_IMA_V: u