// Converted from V8 C++ source files:
// Header: cpu.h
// Implementation: cpu.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    //use crate::v8::Isolate;
    use std::sync::Mutex;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::{mem, ptr};
    use std::fs::File;
    use std::io::{Read, BufReader};
    use std::path::Path;
    use std::io;
    use std::str;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::process::Command;

    #[macro_export]
    macro_rules! DCHECK_NOT_NULL {
        ($ptr:expr) => {
            if $ptr.is_null() {
                panic!("DCHECK_NOT_NULL failed: pointer is null");
            }
        };
    }

    #[macro_export]
    macro_rules! DCHECK {
        ($condition:expr) => {
            if !$condition {
                panic!("DCHECK failed: condition is false");
            }
        };
    }

    pub trait BaseExport {}

    pub struct CPU {
        vendor_: [c_char; 13],
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
        has_cetss_: bool,
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
    }

    impl CPU {
        pub fn new() -> CPU {
            let mut cpu = CPU {
                vendor_: [0; 13],
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
                icache_line_size_: CPU::K_UNKNOWN_CACHE_LINE_SIZE,
                dcache_line_size_: CPU::K_UNKNOWN_CACHE_LINE_SIZE,
                num_virtual_address_bits_: CPU::K_UNKNOWN_NUM_VIRTUAL_ADDRESS_BITS,
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
                has_cetss_: false,
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
            };
            let vendor = CString::new("Unknown").unwrap();
            let vendor_bytes = vendor.as_bytes_with_nul();
            let len = vendor_bytes.len();
            cpu.vendor_[..len].copy_from_slice(unsafe {
                std::mem::transmute::<&[u8], &[c_char]>(&vendor_bytes[..len])
            });
            //cpu.vendor_ = *b"Unknown\0\0\0\0\0";

            #[cfg(target_os = "starboard")]
            {
                if cpu.starboard_detect_cpu() {
                    return cpu;
                }
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                let mut cpu_info: [i32; 4] = [0; 4];

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

                let vendor_bytes = unsafe {
                    std::mem::transmute::<&[i32], &[u8]>(&cpu_info[1..4])
                };

                let mut vendor_str = String::new();
                for &byte in vendor_bytes {
                    if byte != 0 {
                        vendor_str.push(byte as char);
                    }
                }
                let c_vendor = CString::new(vendor_str).unwrap();
                let vendor_bytes = c_vendor.as_bytes_with_nul();
                let len = vendor_bytes.len().min(12);

                cpu.vendor_[..len].copy_from_slice(unsafe {
                    std::mem::transmute::<&[u8], &[c_char]>(&vendor_bytes[..len])
                });

                // Interpret CPU feature information.
                if num_ids > 0 {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), 1);
                    }

                    let mut cpu_info70: [i32; 4] = [0; 4];
                    let mut cpu_info71: [i32; 4] = [0; 4];
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
                    // CET shadow stack feature flag. See
                    // https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features
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
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x9 || cpu.stepping_ == 0xA ||
                                    cpu.stepping_ == 0xB || cpu.stepping_ == 0xC;
                            }
                            0x9E => {
                                cpu.has_intel_jcc_erratum_ = cpu.stepping_ == 0x9 || cpu.stepping_ == 0xA ||
                                    cpu.stepping_ == 0xB || cpu.stepping_ == 0xD;
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
                        __cpuid(cpu_info.as_mut_ptr(), PARAMETER_CONTAINING_NON_STOP_TIME_STAMP_COUNTER);
                    }
                    cpu.has_non_stop_time_stamp_counter_ = (cpu_info[3] & (1 << 8)) != 0;
                }

                const VIRTUAL_PHYSICAL_ADDRESS_BITS: u32 = 0x80000008;
                if num_ext_ids >= VIRTUAL_PHYSICAL_ADDRESS_BITS {
                    unsafe {
                        __cpuid(cpu_info.as_mut_ptr(), VIRTUAL_PHYSICAL_ADDRESS_BITS);
                    }
                    cpu.num_virtual_address_bits_ = (cpu_info[0] >> 8) & 0xff;
                }

                // This logic is replicated from cpu.cc present in chromium.src
                if !cpu.has_non_stop_time_stamp_counter_ && cpu.is_running_in_vm_ {
                    let mut cpu_info_hv: [i32; 4] = [0; 4];
                    unsafe {
                        __cpuid(cpu_info_hv.as_mut_ptr(), 0x40000000);
                    }
                    if cpu_info_hv[1] == 0x7263694D &&  // Micr
                        cpu_info_hv[2] == 0x666F736F &&  // osof
                        cpu_info_hv[3] == 0x76482074 {  // t Hv
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
                    let cpu_info = CPUInfo::new();

                    // Extract implementor from the "CPU implementer" field.
                    if let Some(implementer) = cpu_info.extract_field("CPU implementer") {
                        if let Ok(value) = implementer.parse::<i32>() {
                            cpu.implementer_ = value;
                        }
                    }

                    if let Some(variant) = cpu_info.extract_field("CPU variant") {
                        if let Ok(value) = variant.parse::<i32>() {
                            cpu.variant_ = value;
                        }
                    }

                    // Extract part number from the "CPU part" field.
                    if let Some(part) = cpu_info.extract_field("CPU part") {
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
                    if let Some(architecture) = cpu_info.extract_field("CPU architecture") {
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
                            if let Some(processor) = cpu_info.extract_field("Processor") {
                                if has_list_item(&processor, "(v6l)") {
                                    cpu.architecture_ = 6;
                                }
                            }

                            // elf_platform moved to the model name field in Linux v3.8.
                            if cpu.architecture_ == 7 {
                                if let Some(processor) = cpu_info.extract_field("model name") {
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
                        if let Some(features) = cpu_info.extract_field("Features") {
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

                #[cfg(target_os = "qnx")]
                {
                    // Implement QNX specific logic here if needed
                }
            }

            #[cfg(target_arch = "mips64")]
            {
                let cpu_info = CPUInfo::new();
                if let Some(cpu_model) = cpu_info.extract_field("cpu model") {
                    cpu.has_fpu_ = has_list_item(&cpu_model, "FPU");
                }
                if let Some(ases) = cpu_info.extract_field("ASEs implemented") {
                    cpu.has_msa_ = has_list_item(&ases, "msa");
                }
            }

            #[cfg(target_arch = "arm64")]
            {
                #[cfg(target_os = "windows")]
                {
                    // Implement Windows specific logic here if needed
                }

                #[cfg(target_os = "linux")]
                {
                    let (hwcaps, hwcaps2) = read_elf_hwcaps();
                    cpu.has_mte_ = (hwcaps2 & HWCAP2_MTE) != 0;
                    if hwcaps != 0 {
                        cpu.has_jscvt_ = (hwcaps & HWCAP_JSCVT) != 0;
                        cpu.has_dot_prod_ = (hwcaps & HWCAP_ASIMDDP) != 0;
                        cpu.has_lse_ = (hwcaps & HWCAP_ATOMICS) != 0;
                        cpu.has_pmull1q_ = (hwcaps & HWCAP_PMULL) != 0;
                        cpu.has_fp16_ = (hwcaps & HWCAP_FPHP) != 0;
                    } else {
                        let cpu_info = CPUInfo::new();
                        if let Some(features) = cpu_info.extract_field("Features") {
                            cpu.has_jscvt_ = has_list_item(&features, "jscvt");
                            cpu.has_dot_prod_ = has_list_item(&features, "asimddp");
                            cpu.has_lse_ = has_list_item(&features, "atomics");
                            cpu.has_pmull1q_ = has_list_item(&features, "pmull");
                            cpu.has_fp16_ = has_list_item(&features, "half");
                        }
                    }
                }

                #[cfg(target_os = "darwin")]
                {
                    // Implement Darwin specific logic here if needed
                }
            }

            #[cfg(target_arch = "powerpc64")]
            {
                // Implement POWERPC64 specific logic here if needed
            }

            #[cfg(target_arch = "riscv64")]
            {
                #[cfg(target_os = "linux")]
                {
                    let cpu_info = CPUInfo::new();
                    // Extract features from the "isa" field.
                    if let Some(isa) = cpu_info.extract_field("isa") {
                        cpu.has_fpu_ = has_list_item(&isa, "rv64imafdc");
                        cpu.has_rvv_ = has_list_item(&isa, "rv64imafdcv");
                    }
                    
                    // Extract MMU mode from the "mmu" field.
                    if let Some(mmu) = cpu_info.extract_field("mmu") {
                        if has_list_item(&mmu, "sv48") {
                            cpu.riscv_mmu_ = RV_MMU_MODE::kRiscvSV48;
                        } else if has_list_item(&mmu, "sv39") {
                            cpu.riscv_mmu_ = RV_MMU_MODE::kRiscvSV39;
                        } else if has_list_item(&mmu, "sv57")) {
                            cpu.riscv_mmu_ = RV_MMU_MODE::kRiscvSV57;
                        }
                    }
                }
            }
            cpu
        }

        pub fn vendor(&self) -> &str {
            let c_str = unsafe { CString::from_raw(self.vendor_.as_ptr() as *mut c_char) };
            let str_slice: &str = c_str.to_str().unwrap();
            str_slice
        }
        pub fn stepping(&self) -> i32 {
            self.stepping_
        }
        pub fn model(&self) -> i32 {
            self.model_
        }
        pub fn ext_model(&self) -> i32 {
            self.ext_model_
        }
        pub fn family(&self) -> i32 {
            self.family_
        }
        pub fn ext_family(&self) -> i32 {
            self.ext_family_
        }
        pub fn type_(&self) -> i32 {
            self.type_
        }
        pub fn implementer(&self) -> i32 {
            self.implementer_
        }
        pub fn architecture(&self) -> i32 {
            self.architecture_
        }
        pub fn variant(&self) -> i32 {
            self.variant_
        }
        pub fn part(&self) -> i32 {
            self.part_
        }
        pub fn has_fpu(&self) -> bool {
            self.has_fpu_
        }
        pub fn icache_line_size(&self) -> i32 {
            self.icache_line_size_
        }
        pub fn dcache_line_size(&self) -> i32 {
            self.dcache_line_size_
        }
        pub fn has_cmov(&self) -> bool {
            self.has_cmov_
        }
        pub fn has_sahf(&self) -> bool {
            self.has_sahf_
        }
        pub fn has_mmx(&self) -> bool {
            self.has_mmx_
        }
        pub fn has_sse(&self) -> bool {
            self.has_sse_
        }
        pub fn has_sse2(&self) -> bool {
            self.has_sse2_
        }
        pub fn has_sse3(&self) -> bool {
            self.has_sse3_
        }
        pub fn has_ssse3(&self) -> bool {
            self.has_ssse3_
        }
        pub fn has_sse41(&self) -> bool {
            self.has_sse41_
        }
        pub fn has_sse42(&self) -> bool {
            self.has_sse42_
        }
        pub fn has_osxsave(&self) -> bool {
            self.has_osxsave_
        }
        pub fn has_avx(&self) -> bool {
            self.has_avx_
        }
        pub fn has_avx2(&self) -> bool {
            self.has_avx2_
        }
        pub fn has_avx_vnni(&self) -> bool {
            self.has_avx_vnni_
        }
        pub fn has_avx_vnni_int8(&self) -> bool {
            self.has_avx_vnni_int8_
        }
        pub fn has_fma3(&self) -> bool {
            self.has_fma3_
        }
        pub fn has_f16c(&self) -> bool {
            self.has_f16c_
        }
        pub fn has_bmi1(&self) -> bool {
            self.has_bmi1_
        }
        pub fn has_bmi2(&self) -> bool {
            self.has_bmi2_
        }
        pub fn has_lzcnt(&self) -> bool {
            self.has_lzcnt_
        }
        pub fn has_popcnt(&self) -> bool {
            self.has_popcnt_
        }
        pub fn is_atom(&self) -> bool {
            self.is_atom_
        }
        pub fn has_intel_jcc_erratum(&self) -> bool {
            self.has_intel_jcc_erratum_
        }
        pub fn has_cetss(&self) -> bool {
            self.has_cetss_
        }
        pub fn has_non_stop_time_stamp_counter(&self) -> bool {
            self.has_non_stop_time_stamp_counter_
        }
        pub fn is_running_in_vm(&self) -> bool {
            self.is_running_in_vm_

