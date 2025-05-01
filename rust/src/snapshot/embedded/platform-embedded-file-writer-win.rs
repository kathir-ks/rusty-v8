// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/embedded/platform-embedded-file-writer-win.rs

use std::fs::File;
use std::io::{self, Write};
use std::cmp;

// Placeholder for win64_unwindinfo module, as it's not directly translatable
// due to OS-specific structures and inline assembly.
mod win64_unwindinfo {
    pub struct BuiltinUnwindInfo {
        leaf_function: bool,
        fp_offsets: Vec<i32>,
        fp_adjustments: Vec<FrameOffsets>,
    }

    #[derive(Clone, Copy)]
    pub struct FrameOffsets {
        // Define the fields of FrameOffsets as needed based on C++ code usage.
    }

    impl BuiltinUnwindInfo {
        pub fn is_leaf_function(&self) -> bool {
            self.leaf_function
        }
        pub fn fp_offsets(&self) -> &Vec<i32> {
            &self.fp_offsets
        }
        pub fn fp_adjustments(&self) -> &Vec<FrameOffsets> {
            &self.fp_adjustments
        }
    }

    pub const kRbpPrefixLength: u64 = 3;  // Example value
    pub const kMaxFunctionLength: i32 = 4096;

    pub fn CanEmitUnwindInfoForBuiltins() -> bool {
        true // Placeholder, replace with actual logic
    }

    pub fn GetUnwindInfoForBuiltinFunctions() -> Vec<u8> {
        vec![0u8; 10] // Placeholder, replace with actual logic
    }

    pub fn GetUnwindInfoForBuiltinFunction(code_chunks_i: i32, fp_adjustments_i: FrameOffsets) -> Vec<u8> {
        vec![0u8; 10] // Placeholder, replace with actual logic
    }
}

mod builtins {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Builtin {
        kFirst,
        kLast,
    }
    pub const kAllBuiltinsAreIsolateIndependent: bool = true;
}

mod flags {
    pub static flags: Flags = Flags {
    };
    pub struct Flags {

    }
}

// Mock EmbeddedData
mod embedded_data {
    pub struct EmbeddedData {
        code: Vec<u8>,
    }

    impl EmbeddedData {
        pub fn new(code: Vec<u8>) -> Self {
            EmbeddedData { code }
        }
        pub fn code(&self) -> *const u8 {
            self.code.as_ptr()
        }
        pub fn InstructionStartOf(&self, _builtin: crate::builtins::Builtin) -> *const u8 {
            self.code.as_ptr()
        }
        pub fn InstructionSizeOf(&self, _builtin: crate::builtins::Builtin) -> u32 {
            self.code.len() as u32
        }
    }
}

const CRASH_HANDLER_FUNCTION_NAME_STRING: &str = "CrashHandler";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDirective {
    Byte,
    Long,
    Quad,
    Octa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddedTargetArch {
    X64,
    Arm64,
}

#[derive(Debug)]
pub struct PlatformEmbeddedFileWriterWin {
    fp: File,
    target_arch_: EmbeddedTargetArch,
}

impl PlatformEmbeddedFileWriterWin {
    pub fn new(fp: File, target_arch_: EmbeddedTargetArch) -> Self {
        PlatformEmbeddedFileWriterWin { fp, target_arch_ }
    }

    fn directive_as_string(&self, directive: DataDirective) -> &'static str {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ != EmbeddedTargetArch::Arm64 {
                match directive {
                    DataDirective::Byte => "BYTE",
                    DataDirective::Long => "DWORD",
                    DataDirective::Quad => "QWORD",
                    _ => panic!("Unexpected DataDirective"),
                }
            } else {
                match directive {
                    DataDirective::Byte => "DCB",
                    DataDirective::Long => "DCDU",
                    DataDirective::Quad => "DCQU",
                    _ => panic!("Unexpected DataDirective"),
                }
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            match directive {
                DataDirective::Byte => ".byte",
                DataDirective::Long => ".long",
                DataDirective::Quad => ".quad",
                DataDirective::Octa => ".octa",
                _ => panic!("Unexpected DataDirective"),
            }
        }
    }

    pub fn maybe_emit_unwind_data(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        blob: &embedded_data::EmbeddedData,
        unwind_infos: *const win64_unwindinfo::BuiltinUnwindInfo,
    ) -> io::Result<()> {
        // Windows ARM64 supports cross build which could require unwind info for
        // host_os. Ignore this case because it is only used in build time.
        #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
        {
            if self.target_arch_ != EmbeddedTargetArch::Arm64 {
                return Ok(());
            }
        }

        #[cfg(all(target_os = "windows", target_pointer_width = "64"))]
        {
            if win64_unwindinfo::CanEmitUnwindInfoForBuiltins() {
                unsafe {
                    self.emit_unwind_data(
                        unwind_info_symbol,
                        embedded_blob_data_symbol,
                        blob,
                        &*unwind_infos,
                    )?;
                }
            }
        }
        Ok(())
    }

    #[cfg(all(target_os = "windows", any(target_arch = "x86_64", target_arch = "aarch64")))]
    fn emit_unwind_data(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        blob: &embedded_data::EmbeddedData,
        unwind_infos: &win64_unwindinfo::BuiltinUnwindInfo,
    ) -> io::Result<()> {
        #[cfg(target_arch = "x86_64")]
        {
            self.emit_unwind_data_x64(unwind_info_symbol, embedded_blob_data_symbol, blob, unwind_infos)
        }
        #[cfg(target_arch = "aarch64")]
        {
            self.emit_unwind_data_arm64(unwind_info_symbol, embedded_blob_data_symbol, blob, unwind_infos)
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Ok(())
        }
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    fn emit_unwind_data_x64(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        blob: &embedded_data::EmbeddedData,
        unwind_infos: &win64_unwindinfo::BuiltinUnwindInfo,
    ) -> io::Result<()> {
        // Emit an UNWIND_INFO (XDATA) struct, which contains the unwinding
        // information that is used for all builtin functions.
        assert!(win64_unwindinfo::CanEmitUnwindInfoForBuiltins());
        self.comment("xdata for all the code in the embedded blob.")?;
        self.declare_external_function(CRASH_HANDLER_FUNCTION_NAME_STRING)?;

        self.start_xdata_section()?;
        {
            self.declare_label(unwind_info_symbol)?;

            let xdata = win64_unwindinfo::GetUnwindInfoForBuiltinFunctions();
            assert!(!xdata.is_empty());

            self.indented_data_directive(DataDirective::Byte)?;
            for (i, &byte) in xdata.iter().enumerate() {
                if i > 0 {
                    write!(self.fp, ",")?;
                }
                self.hex_literal(byte as u64)?;
            }
            writeln!(self.fp)?;

            self.comment("    ExceptionHandler")?;
            self.declare_rva_to_symbol(CRASH_HANDLER_FUNCTION_NAME_STRING, 0)?;
        }
        self.end_xdata_section()?;
        writeln!(self.fp)?;

        // Emit a RUNTIME_FUNCTION (PDATA) entry for each builtin function, as
        // documented here:
        // https://docs.microsoft.com/en-us/cpp/build/exception-handling-x64.
        self.comment(
            "pdata for all the code in the embedded blob (structs of type \
             RUNTIME_FUNCTION).",
        )?;
        self.comment("    BeginAddress")?;
        self.comment("    EndAddress")?;
        self.comment("    UnwindInfoAddress")?;
        self.start_pdata_section()?;
        {
            assert!(builtins::Builtin::kAllBuiltinsAreIsolateIndependent);
            let mut prev_builtin_end_offset: u64 = 0;
            let builtin_range = unsafe {
                std::mem::transmute::<_, i32>(builtins::Builtin::kFirst)..=std::mem::transmute::<_, i32>(builtins::Builtin::kLast)
            };
            for builtin_index in builtin_range {
                let builtin = unsafe { std::mem::transmute::<i32, builtins::Builtin>(builtin_index) };
                // Some builtins are leaf functions from the point of view of Win64 stack
                // walking: they do not move the stack pointer and do not require a PDATA
                // entry because the return address can be retrieved from [rsp].
                if unwind_infos[builtin_index as usize].is_leaf_function() {
                    continue;
                }

                let builtin_start_offset =
                    (blob.InstructionStartOf(builtin) as u64) - (blob.code() as u64);
                let builtin_size = blob.InstructionSizeOf(builtin);

                let xdata_desc = unwind_infos[builtin_index as usize].fp_offsets();
                if xdata_desc.is_empty() {
                    // Some builtins do not have any "push rbp - mov rbp, rsp" instructions
                    // to start a stack frame. We still emit a PDATA entry as if they had,
                    // relying on the fact that we can find the previous frame address from
                    // rbp in most cases. Note that since the function does not really start
                    // with a 'push rbp' we need to specify the start RVA in the PDATA entry
                    // a few bytes before the beginning of the function, if it does not
                    // overlap the end of the previous builtin.
                    self.write_unwind_info_entry(
                        unwind_info_symbol,
                        embedded_blob_data_symbol,
                        cmp::max(
                            prev_builtin_end_offset,
                            builtin_start_offset - win64_unwindinfo::kRbpPrefixLength,
                        ),
                        builtin_start_offset + builtin_size as u64,
                    )?;
                } else {
                    // Some builtins have one or more "push rbp - mov rbp, rsp" sequences,
                    // but not necessarily at the beginning of the function. In this case
                    // we want to yield a PDATA entry for each block of instructions that
                    // emit an rbp frame. If the function does not start with 'push rbp'
                    // we also emit a PDATA entry for the initial block of code up to the
                    // first 'push rbp', like in the case above.
                    if xdata_desc[0] > 0 {
                        self.write_unwind_info_entry(
                            unwind_info_symbol,
                            embedded_blob_data_symbol,
                            cmp::max(
                                prev_builtin_end_offset,
                                builtin_start_offset - win64_unwindinfo::kRbpPrefixLength,
                            ),
                            builtin_start_offset + xdata_desc[0] as u64,
                        )?;
                    }

                    for j in 0..xdata_desc.len() {
                        let chunk_start = xdata_desc[j];
                        let chunk_end = if j < xdata_desc.len() - 1 {
                            xdata_desc[j + 1]
                        } else {
                            builtin_size as i32
                        };
                        self.write_unwind_info_entry(
                            unwind_info_symbol,
                            embedded_blob_data_symbol,
                            builtin_start_offset + chunk_start as u64,
                            builtin_start_offset + chunk_end as u64,
                        )?;
                    }
                }

                prev_builtin_end_offset = builtin_start_offset + builtin_size as u64;
                writeln!(self.fp)?;
            }
        }
        self.end_pdata_section()?;
        writeln!(self.fp)?;
        Ok(())
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    fn write_unwind_info_entry(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        rva_start: u64,
        rva_end: u64,
    ) -> io::Result<()> {
        self.declare_rva_to_symbol(embedded_blob_data_symbol, rva_start)?;
        self.declare_rva_to_symbol(embedded_blob_data_symbol, rva_end)?;
        self.declare_rva_to_symbol(unwind_info_symbol, 0)?;
        Ok(())
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    fn emit_unwind_data_arm64(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        blob: &embedded_data::EmbeddedData,
        unwind_infos: &win64_unwindinfo::BuiltinUnwindInfo,
    ) -> io::Result<()> {
        assert!(win64_unwindinfo::CanEmitUnwindInfoForBuiltins());

        // Emit a RUNTIME_FUNCTION (PDATA) entry for each builtin function, as
        // documented here:
        // https://docs.microsoft.com/en-us/cpp/build/arm64-exception-handling.
        self.comment(
            "pdata for all the code in the embedded blob (structs of type \
             RUNTIME_FUNCTION).",
        )?;
        self.comment("    BeginAddress")?;
        self.start_pdata_section()?;
        let mut code_chunks: Vec<i32> = Vec::new();
        let mut fp_adjustments: Vec<win64_unwindinfo::FrameOffsets> = Vec::new();

        assert!(builtins::Builtin::kAllBuiltinsAreIsolateIndependent);
        let builtin_range = unsafe {
            std::mem::transmute::<_, i32>(builtins::Builtin::kFirst)..=std::mem::transmute::<_, i32>(builtins::Builtin::kLast)
        };
        for builtin_index in builtin_range {
            let builtin = unsafe { std::mem::transmute::<i32, builtins::Builtin>(builtin_index) };

            if unwind_infos[builtin_index as usize].is_leaf_function() {
                continue;
            }

            let builtin_start_offset =
                (blob.InstructionStartOf(builtin) as u64) - (blob.code() as u64);
            let builtin_size = blob.InstructionSizeOf(builtin);

            let xdata_desc = unwind_infos[builtin_index as usize].fp_offsets();
            let xdata_fp_adjustments = unwind_infos[builtin_index as usize].fp_adjustments();
            assert_eq!(xdata_desc.len(), xdata_fp_adjustments.len());

            for j in 0..xdata_desc.len() {
                let chunk_start = xdata_desc[j];
                let chunk_end = if j < xdata_desc.len() - 1 {
                    xdata_desc[j + 1]
                } else {
                    builtin_size as i32
                };
                let chunk_len = round_up(chunk_end - chunk_start, kInstrSize);

                let mut current_chunk_len = chunk_len;
                while current_chunk_len > 0 {
                    let allowed_chunk_len =
                        cmp::min(current_chunk_len, win64_unwindinfo::kMaxFunctionLength);
                    current_chunk_len -= win64_unwindinfo::kMaxFunctionLength;

                    // Record the chunk length and fp_adjustment for emitting UNWIND_INFO
                    // later.
                    code_chunks.push(allowed_chunk_len);
                    fp_adjustments.push(xdata_fp_adjustments[j]);
                    let unwind_info_full_symbol =
                        format!("{}_{}", unwind_info_symbol, code_chunks.len());
                    self.declare_rva_to_symbol(
                        embedded_blob_data_symbol,
                        builtin_start_offset + chunk_start as u64,
                    )?;
                    self.declare_rva_to_symbol(&unwind_info_full_symbol, 0)?;
                }
            }
        }
        self.end_pdata_section()?;
        writeln!(self.fp)?;

        // Emit an UNWIND_INFO (XDATA) structs, which contains the unwinding
        // information.
        self.declare_external_function(CRASH_HANDLER_FUNCTION_NAME_STRING)?;
        self.start_xdata_section()?;
        {
            for i in 0..code_chunks.len() {
                let unwind_info_full_symbol = format!("{}_{}", unwind_info_symbol, i + 1);
                self.declare_label(&unwind_info_full_symbol)?;
                let xdata = win64_unwindinfo::GetUnwindInfoForBuiltinFunction(
                    code_chunks[i as usize],
                    fp_adjustments[i as usize],
                );

                self.indented_data_directive(DataDirective::Byte)?;
                for (j, &byte) in xdata.iter().enumerate() {
                    if j > 0 {
                        write!(self.fp, ",")?;
                    }
                    self.hex_literal(byte as u64)?;
                }
                writeln!(self.fp)?;
                self.declare_rva_to_symbol(CRASH_HANDLER_FUNCTION_NAME_STRING, 0)?;
            }
        }
        self.end_xdata_section()?;
        writeln!(self.fp)?;
        Ok(())
    }

    pub fn section_text(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(
                    self.fp,
                    "  AREA |.text|, CODE, ALIGN={}, READONLY",
                    ARM64_CODE_ALIGNMENT_POWER
                )
            } else {
                writeln!(self.fp, ".CODE")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, ".section .text$hot,\"xr\"")
        }
    }

    pub fn section_ro_data(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(
                    self.fp,
                    "  AREA |.rodata|, DATA, ALIGN={}, READONLY",
                    ARM64_DATA_ALIGNMENT_POWER
                )
            } else {
                writeln!(self.fp, ".CONST")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, ".section .rdata")
        }
    }

    pub fn declare_uint32(&mut self, name: &str, value: u32) -> io::Result<()> {
        self.declare_symbol_global(name)?;
        write!(
            self.fp,
            "{}{} {} {}\n",
            SYMBOL_PREFIX,
            name,
            self.directive_as_string(DataDirective::Long),
            value
        )
    }

    pub fn start_pdata_section(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(
                    self.fp,
                    "  AREA |.pdata|, DATA, ALIGN={}, READONLY",
                    ARM64_DATA_ALIGNMENT_POWER
                )
            } else {
                writeln!(self.fp, "OPTION DOTNAME")?;
                writeln!(self.fp, ".pdata SEGMENT DWORD READ ''")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, ".section .pdata")
        }
    }

    pub fn end_pdata_section(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ != EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, ".pdata ENDS")
            } else {
                Ok(())
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            Ok(())
        }
    }

    pub fn start_xdata_section(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(
                    self.fp,
                    "  AREA |.xdata|, DATA, ALIGN={}, READONLY",
                    ARM64_DATA_ALIGNMENT_POWER
                )
            } else {
                writeln!(self.fp, "OPTION DOTNAME")?;
                writeln!(self.fp, ".xdata SEGMENT DWORD READ ''")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, ".section .xdata")
        }
    }

    pub fn end_xdata_section(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ != EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, ".xdata ENDS")
            } else {
                Ok(())
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            Ok(())
        }
    }

    pub fn declare_external_function(&mut self, name: &str) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "  EXTERN {} ", name)
            } else {
                writeln!(self.fp, "EXTERN {} : PROC", name)
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            Ok(())
        }
    }

    pub fn declare_rva_to_symbol(&mut self, name: &str, offset: u64) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                if offset > 0 {
                    writeln!(self.fp, "  DCD  {} + {}", name, offset)?;
                } else {
                    writeln!(self.fp, "  DCD  {}", name)?;
                }
                // The default relocation entry generated by MSVC armasm64.exe for DCD
                // directive is IMAGE_REL_ARM64_ADDR64 which represents relocation for
                // 64-bit pointer instead of 32-bit RVA. Append RELOC with
                // IMAGE_REL_ARM64_ADDR32NB(2) to generate correct relocation entry for
                // 32-bit RVA.
                writeln!(self.fp, "  RELOC 2")
            } else {
                if offset > 0 {
                    write!(self.fp, "DD IMAGEREL {}+{}\n", name, offset)?;
                } else {
                    write!(self.fp, "DD IMAGEREL {}\n", name)?;
                }
                Ok(())
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            if offset > 0 {
                writeln!(self.fp, ".rva {} + {}", name, offset)
            } else {
                writeln!(self.fp, ".rva {}", name)
            }
        }
    }

    pub fn declare_symbol_global(&mut self, name: &str) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "  EXPORT {}{}", SYMBOL_PREFIX, name)
            } else {
                writeln!(self.fp, "PUBLIC {}{}", SYMBOL_PREFIX, name)
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, ".global {}{}", SYMBOL_PREFIX, name)
        }
    }

    pub fn align_to_code_alignment(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "  ALIGN {}", ARM64_CODE_ALIGNMENT)
            } else {
                // Diverges from other platforms due to compile error
                // 'invalid combination with segment alignment'.
                writeln!(self.fp, "ALIGN 4")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            #[cfg(target_arch = "x86_64")]
            {
                // On x64 use 64-bytes code alignment to allow 64-bytes loop header alignment.
                writeln!(self.fp, ".balign 64")
            }
            #[cfg(target_arch = "powerpc64")]
            {
                // 64 byte alignment is needed on ppc64 to make sure p10 prefixed instructions
                // don't cross 64-byte boundaries.
                writeln!(self.fp, ".balign 64")
            }
            #[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc64")))]
            {
                writeln!(self.fp, ".balign 32")
            }
        }
    }

    pub fn align_to_data_alignment(&mut self) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "  ALIGN {}", ARM64_DATA_ALIGNMENT)
            } else {
                writeln!(self.fp, "ALIGN 4")
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            // On Windows ARM64, s390, PPC and possibly more platforms, aligned load
            // instructions are used to retrieve v8_Default_embedded_blob_ and/or
            // v8_Default_embedded_blob_size_. The generated instructions require the
            // load target to be aligned at 8 bytes (2^3).
            writeln!(self.fp, ".balign 8")
        }
    }

    pub fn comment(&mut self, string: &str) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            writeln!(self.fp, "; {}", string)
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, "// {}", string)
        }
    }

    pub fn declare_label(&mut self, name: &str) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "{}{}", SYMBOL_PREFIX, name)
            } else {
                writeln!(
                    self.fp,
                    "{}{} LABEL {}",
                    SYMBOL_PREFIX,
                    name,
                    self.directive_as_string(DataDirective::Byte)
                )
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            writeln!(self.fp, "{}{}:", SYMBOL_PREFIX, name)
        }
    }

    pub fn source_info(&mut self, _fileid: i32, _filename: &str, _line: i32) -> io::Result<()> {
        // TODO(mvstanton): output source information for MSVC.
        // Its syntax is #line <line> "<filename>"
        #[cfg(not(target_env = "msvc"))]
        {
            // BUG(9944): Use .cv_loc to ensure CodeView information is used on
            // Windows.
        }
        Ok(())
    }

    // TODO(mmarchini): investigate emitting size annotations for Windows
    pub fn declare_function_begin(&mut self, name: &str, _size: u32) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "{}{} FUNCTION", SYMBOL_PREFIX, name)
            } else {
                writeln!(self.fp, "{}{} PROC", SYMBOL_PREFIX, name)
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            self.declare_label(name)?;

            #[cfg(target_arch = "aarch64")]
            {
                self.declare_symbol_global(name)?;
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                // The directives for inserting debugging information on Windows come
                // from the PE (Portable Executable) and COFF (Common Object File Format)
                // standards. Documented here:
                // https://docs.microsoft.com/en-us/windows/desktop/debug/pe-format
                //
                // .scl 2 means StorageClass external.
                // .type 32 means Type Representation Function.
                writeln!(
                    self.fp,
                    ".def {}{}; .scl 2; .type 32; .endef;",
                    SYMBOL_PREFIX, name
                )
            }
        }
    }

    pub fn declare_function_end(&mut self, name: &str) -> io::Result<()> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                writeln!(self.fp, "  ENDFUNC")
            } else {
                writeln!(self.fp, "{}{} ENDP", SYMBOL_PREFIX, name)
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            Ok(())
        }
    }

    pub fn hex_literal(&mut self, value: u64) -> io::Result<usize> {
        #[cfg(target_env = "msvc")]
        {
            if self.target_arch_ == EmbeddedTargetArch::Arm64 {
                write!(self.fp, "0x{:x}", value)
            } else {
                write!(self.fp, "0{:x}h", value)
            }
        }
        #[cfg(not(target_env = "msvc"))]
        {
            write!(self.fp, "0x{:x}", value)
        }
    }

    pub fn file_prologue(&mut self) -> io::Result<()> {
        #[cfg(all(target_env = "msvc", not(any(target_arch = "aarch64", target_arch = "x86_64"))))]
        {
            // x86 falls into this case
            writeln!(self.fp, ".MODEL FLAT")
        }
        #[cfg(not(all(target_env = "msvc", not(any(target_arch = "aarch64", target_arch = "x86_64")))))]
        {
            Ok(())
        }
    }

    pub fn declare_external_filename(
        &mut self,
        _fileid: i32,
        _filename: &str,
    ) -> io::Result<()> {
        #[cfg(not(target_env = "msvc"))]
        {
            // BUG(9944): Use .cv_filename to ensure CodeView information is used on
            // Windows.
        }
