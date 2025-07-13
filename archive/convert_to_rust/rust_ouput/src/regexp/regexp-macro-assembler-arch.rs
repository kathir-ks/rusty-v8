// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-arch.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod regexp_macro_assembler_arch {

    use crate::regexp::regexp_macro_assembler::*;

    #[cfg(target_arch = "x86")]
    pub mod regexp_macro_assembler_ia32 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerIA32 {
            assembler: Assembler,
            current_code_block_offset: usize,
            code_blocks: Vec<Vec<u8>>,
        }

        impl RegExpMacroAssemblerIA32 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerIA32 {
                    assembler: Assembler::new(0),
                    current_code_block_offset: 0,
                    code_blocks: vec![Vec::new()],
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerIA32 {
            fn AssembleReturn(&mut self) {
                self.assembler.ret();
            }
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
            pub fn ret(&mut self) {
                self.buffer.push(0xC3); // RET instruction
                self.position += 1;
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub mod regexp_macro_assembler_x64 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerX64 {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerX64 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerX64 {
                    assembler: Assembler::new(0),
                }
            }
        }

        impl RegExpMacroAssembler for RegExpMacroAssemblerX64 {
            fn AssembleReturn(&mut self) {
                self.assembler.ret();
            }
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }

            pub fn ret(&mut self) {
                self.buffer.push(0xC3); // RET instruction
                self.position += 1;
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    pub mod regexp_macro_assembler_arm64 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerArm64 {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerArm64 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerArm64 {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerArm64 {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(target_arch = "arm")]
    pub mod regexp_macro_assembler_arm {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerArm {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerArm {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerArm {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerArm {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod regexp_macro_assembler_ppc {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerPPC {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerPPC {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerPPC {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerPPC {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(target_arch = "mips64")]
    pub mod regexp_macro_assembler_mips64 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerMIPS64 {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerMIPS64 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerMIPS64 {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerMIPS64 {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(target_arch = "loongarch64")]
    pub mod regexp_macro_assembler_loong64 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerLoong64 {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerLoong64 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerLoong64 {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerLoong64 {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(target_arch = "s390x")]
    pub mod regexp_macro_assembler_s390 {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerS390 {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerS390 {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerS390 {
                    assembler: Assembler::new(0),
                }
            }
        }
        impl RegExpMacroAssembler for RegExpMacroAssemblerS390 {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub mod regexp_macro_assembler_riscv {
        use crate::regexp::regexp_macro_assembler::*;

        pub struct RegExpMacroAssemblerRISCV {
            assembler: Assembler,
        }

        impl RegExpMacroAssemblerRISCV {
            pub fn new(
                options: RegExpMacroAssemblerOptions,
                code_generator: Option<Box<dyn CodeGenerator>>,
            ) -> Self {
                RegExpMacroAssemblerRISCV {
                    assembler: Assembler::new(0),
                }
            }
        }

        impl RegExpMacroAssembler for RegExpMacroAssemblerRISCV {
            fn AssembleReturn(&mut self) {}
        }

        pub struct Assembler {
            buffer: Vec<u8>,
            position: usize,
        }

        impl Assembler {
            pub fn new(capacity: usize) -> Assembler {
                Assembler {
                    buffer: Vec::with_capacity(capacity),
                    position: 0,
                }
            }
        }
    }

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "powerpc64",
        target_arch = "mips64",
        target_arch = "loongarch64",
        target_arch = "s390x",
        target_arch = "riscv32",
        target_arch = "riscv64"
    )))]
    compile_error!("Unsupported target architecture.");
}
