// Converted from V8 C++ source files:
// Header: platform-embedded-file-writer-win.h
// Implementation: platform-embedded-file-writer-win.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::ptr;
use std::slice;

//use crate::base::macros::USE;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::PlatformEmbeddedFileWriterBase;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::EmbeddedTargetArch;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::EmbeddedTargetOs;

//use crate::common::globals::V8_OS_WIN64;

//#[cfg(V8_OS_WIN64)]
//use crate::builtins::builtins::Builtins;
//#[cfg(V8_OS_WIN64)]
//use crate::diagnostics::unwinding_info_win64::win64_unwindinfo;
//#[cfg(V8_OS_WIN64)]
//use crate::snapshot::embedded::embedded_data_inl::EmbeddedData;
//#[cfg(V8_OS_WIN64)]
//use crate::snapshot::embedded::embedded_file_writer::EmbeddedFileWriter;

//use crate::flags::flags::FLAG_enable_arm_dynamic_shadow_stack;

//use crate::base::macros::DCHECK;
//use crate::base::macros::UNREACHABLE;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::DataDirective;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::DataDirective::kByte;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::DataDirective::kLong;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::DataDirective::kQuad;
//use crate::snapshot::embedded::platform_embedded_file_writer_base::DataDirective::kOcta;

pub enum EmbeddedTargetArch {
    kX64,
    kArm64,
    kArm,
    kX86,
    kPPC64,
    kS390,
    kLoong64,
    kRiscv64
}

pub enum EmbeddedTargetOs {
    kAndroid,
    kLinux,
    kMac,
    kWin,
    kFuchsia
}

pub enum DataDirective {
    kByte,
    kLong,
    kQuad,
    kOcta
}

struct PlatformEmbeddedFileWriterBase {
    fp_: File
}

impl PlatformEmbeddedFileWriterBase {
    fn new(fp: File) -> Self {
        PlatformEmbeddedFileWriterBase {
            fp_: fp,
        }
    }

    fn fp(&mut self) -> &mut File {
        &mut self.fp_
    }

    fn newline(&mut self) -> std::io::Result<()> {
        writeln!(self.fp())
    }

    fn byte_chunk_data_directive(&self) -> DataDirective {
        DataDirective::kOcta
    }

    fn write_byte_chunk(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let len = data.len();
        self.fp().write_all(data)?;
        Ok(len)
    }

    fn indented_data_directive(&mut self, directive: DataDirective) -> std::io::Result<usize> {
        let directive_str = match directive {
            DataDirective::kByte => ".byte",
            DataDirective::kLong => ".long",
            DataDirective::kQuad => ".quad",
            DataDirective::kOcta => ".octa",
        };
        write!(self.fp(), "  {} ", directive_str)
    }
}

struct PlatformEmbeddedFileWriterWin {
    target_arch_: EmbeddedTargetArch,
    target_os_: EmbeddedTargetOs,
    base_: PlatformEmbeddedFileWriterBase
}

impl PlatformEmbeddedFileWriterWin {
    fn new(target_arch: EmbeddedTargetArch, target_os: EmbeddedTargetOs, fp: File) -> Self {
        assert!(match target_os {
            EmbeddedTargetOs::kWin => true,
            _ => false
        });
        PlatformEmbeddedFileWriterWin {
            target_arch_: target_arch,
            target_os_: target_os,
            base_: PlatformEmbeddedFileWriterBase::new(fp)
        }
    }

    fn fp(&mut self) -> &mut File {
        self.base_.fp()
    }

    fn newline(&mut self) -> std::io::Result<()> {
        self.base_.newline()
    }

    fn section_text(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  AREA |.text|, CODE, ALIGN=2, READONLY")
        } else {
            writeln!(self.fp(), ".CODE")
        }
    }

    fn section_ro_data(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  AREA |.rodata|, DATA, ALIGN=3, READONLY")
        } else {
            writeln!(self.fp(), ".CONST")
        }
    }

    fn declare_uint32(&mut self, name: &str, value: u32) -> std::io::Result<()> {
        self.declare_symbol_global(name)?;
        let directive = self.directive_as_string(DataDirective::kLong);
        writeln!(self.fp(), "{}{} {} {}", self.symbol_prefix(), name, directive, value)
    }

    fn declare_symbol_global(&mut self, name: &str) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  EXPORT {}{}", self.symbol_prefix(), name)
        } else {
            writeln!(self.fp(), "PUBLIC {}{}", self.symbol_prefix(), name)
        }
    }

    fn declare_label(&mut self, name: &str) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "{}{}", self.symbol_prefix(), name)
        } else {
            let directive = self.directive_as_string(DataDirective::kByte);
            writeln!(self.fp(), "{}{} LABEL {}", self.symbol_prefix(), name, directive)
        }
    }

    fn source_info(&mut self, _fileid: i32, _filename: &str, _line: i32) -> std::io::Result<()> {
        Ok(())
    }

    fn declare_function_begin(&mut self, name: &str, _size: u32) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "{}{} FUNCTION", self.symbol_prefix(), name)
        } else {
            writeln!(self.fp(), "{}{} PROC", self.symbol_prefix(), name)
        }
    }

    fn declare_function_end(&mut self, name: &str) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  ENDFUNC")
        } else {
            writeln!(self.fp(), "{}{} ENDP", self.symbol_prefix(), name)
        }
    }

    fn hex_literal(&mut self, value: u64) -> std::io::Result<usize> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            write!(self.fp(), "0x{:x}", value)
        } else {
            write!(self.fp(), "0{:x}h", value)
        }
    }

    fn comment(&mut self, string: &str) -> std::io::Result<()> {
        writeln!(self.fp(), "; {}", string)
    }

    fn file_prologue(&mut self) -> std::io::Result<()> {
        if self.target_arch_ != EmbeddedTargetArch::kArm64 && self.target_arch_ != EmbeddedTargetArch::kX64 && self.target_arch_ != EmbeddedTargetArch::kRiscv64 {
            writeln!(self.fp(), ".MODEL FLAT")
        } else {
            Ok(())
        }
    }

    fn declare_external_filename(&mut self, _fileid: i32, _filename: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn file_epilogue(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  END")
        } else {
            writeln!(self.fp(), "END")
        }
    }

    fn indented_data_directive(&mut self, directive: DataDirective) -> std::io::Result<usize> {
        let directive_str = self.directive_as_string(directive);
        write!(self.fp(), "  {} ", directive_str)
    }

    fn byte_chunk_data_directive(&self) -> DataDirective {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            DataDirective::kQuad
        } else {
            self.base_.byte_chunk_data_directive()
        }
    }

    fn write_byte_chunk(&mut self, data: &[u8]) -> std::io::Result<usize> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            assert_eq!(self.byte_chunk_data_directive(), DataDirective::kQuad);
            let quad_ptr: &[u64] = unsafe {
                slice::from_raw_parts(
                    data.as_ptr() as *const u64,
                    data.len() / mem::size_of::<u64>(),
                )
            };

            let mut total_written = 0;
            for value in quad_ptr {
                total_written += self.hex_literal(*value)?;
            }
            Ok(total_written)
        } else {
            self.base_.write_byte_chunk(data)
        }
    }

    fn align_to_code_alignment(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  ALIGN 4")
        } else {
            writeln!(self.fp(), "ALIGN 4")
        }
    }

    fn align_to_data_alignment(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  ALIGN 8")
        } else {
            writeln!(self.fp(), "ALIGN 4")
        }
    }

    fn start_pdata_section(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  AREA |.pdata|, DATA, ALIGN=3, READONLY")
        } else {
            writeln!(self.fp(), "OPTION DOTNAME")?;
            writeln!(self.fp(), ".pdata SEGMENT DWORD READ ''")
        }
    }

    fn end_pdata_section(&mut self) -> std::io::Result<()> {
        if self.target_arch_ != EmbeddedTargetArch::kArm64 {
            writeln!(self.fp(), ".pdata ENDS")
        } else {
            Ok(())
        }
    }

    fn start_xdata_section(&mut self) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  AREA |.xdata|, DATA, ALIGN=3, READONLY")
        } else {
            writeln!(self.fp(), "OPTION DOTNAME")?;
            writeln!(self.fp(), ".xdata SEGMENT DWORD READ ''")
        }
    }

    fn end_xdata_section(&mut self) -> std::io::Result<()> {
        if self.target_arch_ != EmbeddedTargetArch::kArm64 {
            writeln!(self.fp(), ".xdata ENDS")
        } else {
            Ok(())
        }
    }

    fn declare_external_function(&mut self, name: &str) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            writeln!(self.fp(), "  EXTERN {} ", name)
        } else {
            writeln!(self.fp(), "EXTERN {} : PROC", name)
        }
    }

    fn declare_rva_to_symbol(&mut self, name: &str, offset: u64) -> std::io::Result<()> {
        if let EmbeddedTargetArch::kArm64 = self.target_arch_ {
            if offset > 0 {
                writeln!(self.fp(), "  DCD  {} + {}", name, offset)?;
            } else {
                writeln!(self.fp(), "  DCD  {}", name)?;
            }
            writeln!(self.fp(), "  RELOC 2")
        } else {
            if offset > 0 {
                writeln!(self.fp(), "DD IMAGEREL {}+{}", name, offset)
            } else {
                writeln!(self.fp(), "DD IMAGEREL {}", name)
            }
        }
    }

    fn maybe_emit_unwind_data(&mut self, _unwind_info_symbol: &str, _embedded_blob_data_symbol: &str, _blob: *const std::ffi::c_void, _unwind_infos: *const std::ffi::c_void) -> std::io::Result<()> {
        Ok(())
    }

    fn directive_as_string(&self, directive: DataDirective) -> &'static str {
        match self.target_arch_ {
            EmbeddedTargetArch::kArm64 => {
                match directive {
                    DataDirective::kByte => "DCB",
                    DataDirective::kLong => "DCDU",
                    DataDirective::kQuad => "DCQU",
                    _ => panic!("Unsupported directive for Arm64"),
                }
            }
            _ => {
                match directive {
                    DataDirective::kByte => "BYTE",
                    DataDirective::kLong => "DWORD",
                    DataDirective::kQuad => "QWORD",
                    _ => panic!("Unsupported directive"),
                }
            }
        }
    }

    fn symbol_prefix(&self) -> &'static str {
        match self.target_arch_ {
            EmbeddedTargetArch::kX64 | EmbeddedTargetArch::kArm64 => "",
            _ => "_"
        }
    }
}
