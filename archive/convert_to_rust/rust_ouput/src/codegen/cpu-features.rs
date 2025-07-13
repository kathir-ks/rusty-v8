// Converted from V8 C++ source files:
// Header: cpu-features.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cpu_features {
    use crate::AllStatic;
    use std::sync::Once;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CpuFeature {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        SSE4_2,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        SSE4_1,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        SSSE3,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        SSE3,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        SAHF,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        AVX,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        AVX2,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        AVX_VNNI,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        AVX_VNNI_INT8,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        FMA3,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        BMI1,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        BMI2,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        LZCNT,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        POPCNT,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        INTEL_ATOM,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        INTEL_JCC_ERRATUM_MITIGATION,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        CETSS,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        F16C,

        #[cfg(target_arch = "arm")]
        ARMv7,
        #[cfg(target_arch = "arm")]
        ARMv7_SUDIV,
        #[cfg(target_arch = "arm")]
        ARMv8,

        #[cfg(target_arch = "arm")]
        VFPv3,
        #[cfg(target_arch = "arm")]
        NEON,
        #[cfg(target_arch = "arm")]
        VFP32DREGS,
        #[cfg(target_arch = "arm")]
        SUDIV,

        #[cfg(target_arch = "aarch64")]
        JSCVT,
        #[cfg(target_arch = "aarch64")]
        DOTPROD,
        #[cfg(target_arch = "aarch64")]
        LSE,
        #[cfg(target_arch = "aarch64")]
        PMULL1Q,
        #[cfg(target_arch = "aarch64")]
        FP16,

        #[cfg(target_arch = "mips64")]
        FPU,
        #[cfg(target_arch = "mips64")]
        FP64FPU,
        #[cfg(target_arch = "mips64")]
        MIPSr1,
        #[cfg(target_arch = "mips64")]
        MIPSr2,
        #[cfg(target_arch = "mips64")]
        MIPSr6,
        #[cfg(target_arch = "mips64")]
        MIPS_SIMD,

        #[cfg(target_arch = "loongarch64")]
        FPU,

        #[cfg(target_arch = "powerpc64")]
        PPC_8_PLUS,
        #[cfg(target_arch = "powerpc64")]
        PPC_9_PLUS,
        #[cfg(target_arch = "powerpc64")]
        PPC_10_PLUS,

        #[cfg(target_arch = "s390x")]
        FPU,
        #[cfg(target_arch = "s390x")]
        DISTINCT_OPS,
        #[cfg(target_arch = "s390x")]
        GENERAL_INSTR_EXT,
        #[cfg(target_arch = "s390x")]
        FLOATING_POINT_EXT,
        #[cfg(target_arch = "s390x")]
        VECTOR_FACILITY,
        #[cfg(target_arch = "s390x")]
        VECTOR_ENHANCE_FACILITY_1,
        #[cfg(target_arch = "s390x")]
        VECTOR_ENHANCE_FACILITY_2,
        #[cfg(target_arch = "s390x")]
        MISC_INSTR_EXT2,

        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        FPU,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        FP64FPU,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        RISCV_SIMD,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        ZBA,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        ZBB,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        ZBS,
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        ZICOND,

        NUMBER_OF_CPU_FEATURES,
    }

    const K_BITS_PER_INT: usize = 32;

    pub struct CpuFeatures {
        supported: u32,
        icache_line_size: u32,
        dcache_line_size: u32,
        supports_wasm_simd_128: bool,
        supports_cetss: bool,
    }

    impl CpuFeatures {
        fn new() -> Self {
            CpuFeatures {
                supported: 0,
                icache_line_size: 64,
                dcache_line_size: 64,
                supports_wasm_simd_128: false,
                supports_cetss: false,
            }
        }

        pub fn probe(cross_compile: bool) {
            static INIT: Once = Once::new();
            INIT.call_once(|| {
                assert!(CpuFeature::NUMBER_OF_CPU_FEATURES as usize <= K_BITS_PER_INT);
                CpuFeatures::probe_impl(cross_compile);
            });
        }

        pub fn supported_features() -> u32 {
            CpuFeatures::probe(false);
            CpuFeatures::instance().supported
        }

        pub fn is_supported(f: CpuFeature) -> bool {
            CpuFeatures::supported_features();
            (CpuFeatures::instance().supported & (1u32 << (f as u32))) != 0
        }

        pub fn set_supported(f: CpuFeature) {
            CpuFeatures::instance().supported |= 1u32 << (f as u32);
        }

        pub fn set_unsupported(f: CpuFeature) {
            CpuFeatures::instance().supported &= !(1u32 << (f as u32));
        }

        pub fn supports_wasm_simd_128() -> bool {
            CpuFeatures::supported_features();
            CpuFeatures::instance().supports_wasm_simd_128
        }

        pub fn supports_optimizer() -> bool {
            true
        }

        pub fn icache_line_size() -> u32 {
            CpuFeatures::supported_features();
            let size = CpuFeatures::instance().icache_line_size;
            assert_ne!(size, 0);
            size
        }

        pub fn dcache_line_size() -> u32 {
            CpuFeatures::supported_features();
            let size = CpuFeatures::instance().dcache_line_size;
            assert_ne!(size, 0);
            size
        }

        pub fn print_target() {
            println!("Target Architecture: {}", std::env::consts::ARCH);
        }

        pub fn print_features() {
            CpuFeatures::supported_features();
            println!("Supported CPU Features:");
            for i in 0..(CpuFeature::NUMBER_OF_CPU_FEATURES as u32) {
                if (CpuFeatures::instance().supported & (1 << i)) != 0 {
                    if let Some(feature) = Self::cpu_feature_from_u32(i) {
                        println!("  {:?}", feature);
                    }
                }
            }
        }

        fn cpu_feature_from_u32(value: u32) -> Option<CpuFeature> {
            match value {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                0 => Some(CpuFeature::SSE4_2),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                1 => Some(CpuFeature::SSE4_1),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                2 => Some(CpuFeature::SSSE3),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                3 => Some(CpuFeature::SSE3),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                4 => Some(CpuFeature::SAHF),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                5 => Some(CpuFeature::AVX),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                6 => Some(CpuFeature::AVX2),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                7 => Some(CpuFeature::AVX_VNNI),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                8 => Some(CpuFeature::AVX_VNNI_INT8),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                9 => Some(CpuFeature::FMA3),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                10 => Some(CpuFeature::BMI1),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                11 => Some(CpuFeature::BMI2),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                12 => Some(CpuFeature::LZCNT),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                13 => Some(CpuFeature::POPCNT),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                14 => Some(CpuFeature::INTEL_ATOM),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                15 => Some(CpuFeature::INTEL_JCC_ERRATUM_MITIGATION),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                16 => Some(CpuFeature::CETSS),
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                17 => Some(CpuFeature::F16C),

                #[cfg(target_arch = "arm")]
                0 => Some(CpuFeature::ARMv7),
                #[cfg(target_arch = "arm")]
                1 => Some(CpuFeature::ARMv7_SUDIV),
                #[cfg(target_arch = "arm")]
                2 => Some(CpuFeature::ARMv8),

                #[cfg(target_arch = "arm")]
                3 => Some(CpuFeature::VFPv3),
                #[cfg(target_arch = "arm")]
                4 => Some(CpuFeature::NEON),
                #[cfg(target_arch = "arm")]
                5 => Some(CpuFeature::VFP32DREGS),
                #[cfg(target_arch = "arm")]
                6 => Some(CpuFeature::SUDIV),

                #[cfg(target_arch = "aarch64")]
                0 => Some(CpuFeature::JSCVT),
                #[cfg(target_arch = "aarch64")]
                1 => Some(CpuFeature::DOTPROD),
                #[cfg(target_arch = "aarch64")]
                2 => Some(CpuFeature::LSE),
                #[cfg(target_arch = "aarch64")]
                3 => Some(CpuFeature::PMULL1Q),
                #[cfg(target_arch = "aarch64")]
                4 => Some(CpuFeature::FP16),

                #[cfg(target_arch = "mips64")]
                0 => Some(CpuFeature::FPU),
                #[cfg(target_arch = "mips64")]
                1 => Some(CpuFeature::FP64FPU),
                #[cfg(target_arch = "mips64")]
                2 => Some(CpuFeature::MIPSr1),
                #[cfg(target_arch = "mips64")]
                3 => Some(CpuFeature::MIPSr2),
                #[cfg(target_arch = "mips64")]
                4 => Some(CpuFeature::MIPSr6),
                #[cfg(target_arch = "mips64")]
                5 => Some(CpuFeature::MIPS_SIMD),

                #[cfg(target_arch = "loongarch64")]
                0 => Some(CpuFeature::FPU),

                #[cfg(target_arch = "powerpc64")]
                0 => Some(CpuFeature::PPC_8_PLUS),
                #[cfg(target_arch = "powerpc64")]
                1 => Some(CpuFeature::PPC_9_PLUS),
                #[cfg(target_arch = "powerpc64")]
                2 => Some(CpuFeature::PPC_10_PLUS),

                #[cfg(target_arch = "s390x")]
                0 => Some(CpuFeature::FPU),
                #[cfg(target_arch = "s390x")]
                1 => Some(CpuFeature::DISTINCT_OPS),
                #[cfg(target_arch = "s390x")]
                2 => Some(CpuFeature::GENERAL_INSTR_EXT),
                #[cfg(target_arch = "s390x")]
                3 => Some(CpuFeature::FLOATING_POINT_EXT),
                #[cfg(target_arch = "s390x")]
                4 => Some(CpuFeature::VECTOR_FACILITY),
                #[cfg(target_arch = "s390x")]
                5 => Some(CpuFeature::VECTOR_ENHANCE_FACILITY_1),
                #[cfg(target_arch = "s390x")]
                6 => Some(CpuFeature::VECTOR_ENHANCE_FACILITY_2),
                #[cfg(target_arch = "s390x")]
                7 => Some(CpuFeature::MISC_INSTR_EXT2),

                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                0 => Some(CpuFeature::FPU),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                1 => Some(CpuFeature::FP64FPU),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                2 => Some(CpuFeature::RISCV_SIMD),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                3 => Some(CpuFeature::ZBA),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                4 => Some(CpuFeature::ZBB),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                5 => Some(CpuFeature::ZBS),
                #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                6 => Some(CpuFeature::ZICOND),
                _ => None,
            }
        }

        fn flush_i_cache(start: *mut std::ffi::c_void, size: usize) {
            println!("Flushing instruction cache at {:p} with size {}", start, size);
        }

        fn probe_impl(cross_compile: bool) {
            if cross_compile {
                println!("Cross-compilation detected, using default CPU features.");
                return;
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                if is_x86_feature_detected!("sse4.2") {
                    CpuFeatures::set_supported(CpuFeature::SSE4_2);
                }
                if is_x86_feature_detected!("sse4.1") {
                    CpuFeatures::set_supported(CpuFeature::SSE4_1);
                }
                if is_x86_feature_detected!("ssse3") {
                    CpuFeatures::set_supported(CpuFeature::SSSE3);
                }
                if is_x86_feature_detected!("sse3") {
                    CpuFeatures::set_supported(CpuFeature::SSE3);
                }
                //SAHF is always supported in x64, probe is not needed
                CpuFeatures::set_supported(CpuFeature::SAHF);

                if is_x86_feature_detected!("avx") {
                    CpuFeatures::set_supported(CpuFeature::AVX);
                }
                if is_x86_feature_detected!("avx2") {
                    CpuFeatures::set_supported(CpuFeature::AVX2);
                }
                //avx_vnni and avx_vnni_int8 requires cpuid 0x7 , EDX[12] and EDX[42] respectively
                if is_x86_feature_detected!("fma") {
                    CpuFeatures::set_supported(CpuFeature::FMA3);
                }
                if is_x86_feature_detected!("bmi1") {
                    CpuFeatures::set_supported(CpuFeature::BMI1);
                }
                if is_x86_feature_detected!("bmi2") {
                    CpuFeatures::set_supported(CpuFeature::BMI2);
                }
                if is_x86_feature_detected!("lzcnt") {
                    CpuFeatures::set_supported(CpuFeature::LZCNT);
                }
                if is_x86_feature_detected!("popcnt") {
                    CpuFeatures::set_supported(CpuFeature::POPCNT);
                }

                //intel_atom - no direct feature detection, requires vendor_id and model in cpuid 0x1
                //intel_jcc_erratum_mitigation - requires cpuid 0x1, family > 6 and model = 158 or 166 or 94. Microcode level also matters

                //cetss - requires cpuid 0x7 ,ECX[18], probe is not needed
                //CpuFeatures::set_supported(CpuFeature::CETSS);

                if is_x86_feature_detected!("f16c") {
                    CpuFeatures::set_supported(CpuFeature::F16C);
                }

            }

            #[cfg(target_arch = "arm")]
            {
                // Feature detection for ARM would go here.  Since this is a cross-platform
                // example, we'll just enable a reasonable baseline.  In a real ARM build,
                // you'd use system calls or inline assembly to detect the CPU features.
                CpuFeatures::set_supported(CpuFeature::ARMv7); // Enable NEON and VFPv3
                CpuFeatures::set_supported(CpuFeature::ARMv7_SUDIV); // Enable SUDIV
            }

            #[cfg(target_arch = "aarch64")]
            {
                CpuFeatures::set_supported(CpuFeature::JSCVT);
                CpuFeatures::set_supported(CpuFeature::DOTPROD);
                CpuFeatures::set_supported(CpuFeature::LSE);
                CpuFeatures::set_supported(CpuFeature::PMULL1Q);
                CpuFeatures::set_supported(CpuFeature::FP16);
            }

            #[cfg(target_arch = "mips64")]
            {
                CpuFeatures::set_supported(CpuFeature::FPU);
                CpuFeatures::set_supported(CpuFeature::FP64FPU);
                CpuFeatures::set_supported(CpuFeature::MIPSr1);
                CpuFeatures::set_supported(CpuFeature::MIPSr2);
                CpuFeatures::set_supported(CpuFeature::MIPSr6);
                CpuFeatures::set_supported(CpuFeature::MIPS_SIMD);
            }

            #[cfg(target_arch = "loongarch64")]
            {
                CpuFeatures::set_supported(CpuFeature::FPU);
            }

            #[cfg(target_arch = "powerpc64")]
            {
                CpuFeatures::set_supported(CpuFeature::PPC_8_PLUS);
                CpuFeatures::set_supported(CpuFeature::PPC_9_PLUS);
                CpuFeatures::set_supported(CpuFeature::PPC_10_PLUS);
            }

            #[cfg(target_arch = "s390x")]
            {
                CpuFeatures::set_supported(CpuFeature::FPU);
                CpuFeatures::set_supported(CpuFeature::DISTINCT_OPS);
                CpuFeatures::set_supported(CpuFeature::GENERAL_INSTR_EXT);
                CpuFeatures::set_supported(CpuFeature::FLOATING_POINT_EXT);
                CpuFeatures::set_supported(CpuFeature::VECTOR_FACILITY);
                CpuFeatures::set_supported(CpuFeature::VECTOR_ENHANCE_FACILITY_1);
                CpuFeatures::set_supported(CpuFeature::VECTOR_ENHANCE_FACILITY_2);
                CpuFeatures::set_supported(CpuFeature::MISC_INSTR_EXT2);
            }

            #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
            {
                CpuFeatures::set_supported(CpuFeature::FPU);
                CpuFeatures::set_supported(CpuFeature::FP64FPU);
                CpuFeatures::set_supported(CpuFeature::RISCV_SIMD);
                CpuFeatures::set_supported(CpuFeature::ZBA);
                CpuFeatures::set_supported(CpuFeature::ZBB);
                CpuFeatures::set_supported(CpuFeature::ZBS);
                CpuFeatures::set_supported(CpuFeature::ZICOND);
            }
            CpuFeatures::instance().supports_wasm_simd_128 = true;
            CpuFeatures::instance().supports_cetss = true;

        }

        fn instance() -> &'static mut CpuFeatures {
            static mut INSTANCE: CpuFeatures = CpuFeatures::new();
            unsafe { &mut INSTANCE }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cpu_features() {
            CpuFeatures::probe(false);
            CpuFeatures::print_features();
            assert!(CpuFeatures::icache_line_size() > 0);
            assert!(CpuFeatures::dcache_line_size() > 0);
            CpuFeatures::print_target();
        }
    }
}
