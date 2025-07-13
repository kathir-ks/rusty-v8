// Converted from V8 C++ source files:
// Header: platform-embedded-file-writer-zos.h
// Implementation: platform-embedded-file-writer-zos.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/snapshot/embedded/platform-embedded-file-writer-zos.h
pub mod platform_embedded_file_writer_zos {
    use crate::snapshot::embedded::platform_embedded_file_writer_base::{
        DataDirective, EmbeddedTargetArch, EmbeddedTargetOs,
        PlatformEmbeddedFileWriterBase,
    };
    use std::fs::File;
    use std::io::{Result, Write};

    pub struct PlatformEmbeddedFileWriterZOS {
        target_arch_: EmbeddedTargetArch,
        target_os_: EmbeddedTargetOs,
        fp_: File,
        suffix: u32,
    }

    impl PlatformEmbeddedFileWriterZOS {
        pub fn new(
            target_arch: EmbeddedTargetArch,
            target_os: EmbeddedTargetOs,
            file: File,
        ) -> Self {
            assert_eq!(target_os, EmbeddedTargetOs::kZOS);
            PlatformEmbeddedFileWriterZOS {
                target_arch_: target_arch,
                target_os_: target_os,
                fp_: file,
                suffix: 0,
            }
        }

        pub fn section_text(&mut self) -> Result<()> {
            Ok(())
        }

        pub fn section_ro_data(&mut self) -> Result<()> {
            Ok(())
        }

        pub fn align_to_code_alignment(&mut self) -> Result<()> {
            Ok(())
        }

        pub fn align_to_data_alignment(&mut self) -> Result<()> {
            Ok(())
        }

        pub fn declare_uint32(&mut self, name: &str, value: u32) -> Result<()> {
            self.declare_symbol_global(name)?;
            self.suffix += 1;
            let output = format!(
                "&suffix SETA {}\nCEECWSA LOCTR\nAL&suffix ALIAS C'{}'\nC_WSA64 CATTR DEFLOAD,RMODE(64),PART(AL&suffix)\nAL&suffix XATTR REF(DATA),LINKAGE(XPLINK),SCOPE(EXPORT)\n DC F'{}'\nC_WSA64 CATTR PART(PART1)\nLBL&suffix DC AD(AL&suffix)\n",
                self.suffix, name, value
            );
            self.fp_.write_all(output.as_bytes())?;
            Ok(())
        }

        pub fn declare_label(&mut self, name: &str) -> Result<()> {
            let line1 = format!("*--------------------------------------------\n");
            let line2 = format!("* Label {}\n", name);
            let line3 = format!("*--------------------------------------------\n");
            let line4 = format!("{} DS 0H\n", name);
            self.fp_.write_all(line1.as_bytes())?;
            self.fp_.write_all(line2.as_bytes())?;
            self.fp_.write_all(line3.as_bytes())?;
            self.fp_.write_all(line4.as_bytes())?;
            Ok(())
        }

        pub fn declare_label_prolog(&mut self, name: &str) -> Result<()> {
            self.suffix += 1;
            let output = format!(
                "&suffix SETA {}\nCEECWSA LOCTR\nAL&suffix ALIAS C'{}'\nC_WSA64 CATTR DEFLOAD,RMODE(64),PART(AL&suffix)\nAL&suffix XATTR REF(DATA),LINKAGE(XPLINK),SCOPE(EXPORT)\n",
                self.suffix, name
            );
            self.fp_.write_all(output.as_bytes())?;
            Ok(())
        }

        pub fn declare_label_epilogue(&mut self) -> Result<()> {
            let output = format!(
                "C_WSA64 CATTR PART(PART1)\nLBL&suffix DC AD(AL&suffix)\n"
            );
            self.fp_.write_all(output.as_bytes())?;
            Ok(())
        }

        pub fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> Result<()> {
            self.hlasm_print_line(format!("* line {} \"{}\"\n", line, filename).as_str())
        }

        pub fn declare_function_begin(&mut self, name: &str, size: u32) -> Result<()> {
            let line1 = format!("*--------------------------------------------\n");
            let line2 = format!("* Builtin {}\n", name);
            let line3 = format!("*--------------------------------------------\n");
            let line4 = format!("{} DS 0H\n", name);

            self.fp_.write_all(line1.as_bytes())?;
            self.fp_.write_all(line2.as_bytes())?;
            self.fp_.write_all(line3.as_bytes())?;
            self.fp_.write_all(line4.as_bytes())?;
            Ok(())
        }

        pub fn declare_function_end(&mut self, name: &str) -> Result<()> {
            Ok(())
        }

        pub fn hex_literal(&mut self, value: u64) -> Result<usize> {
            let hex_string = format!("{:016x}", value);
            let bytes = hex_string.as_bytes();
            self.fp_.write(bytes)
        }

        pub fn comment(&mut self, string: &str) -> Result<()> {
            self.hlasm_print_line(format!("* {}\n", string).as_str())
        }

        pub fn file_prologue(&mut self) -> Result<()> {
            let output = format!(
                "&C SETC 'embed'\n SYSSTATE AMODE64=YES\n&C csect\n&C amode 64\n&C rmode 64\n"
            );
            self.fp_.write_all(output.as_bytes())?;
            Ok(())
        }

        pub fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> Result<()> {
            Ok(())
        }

        pub fn file_epilogue(&mut self) -> Result<()> {
            self.fp_.write_all(" end\n".as_bytes())?;
            Ok(())
        }

        pub fn indented_data_directive(&mut self, directive: DataDirective) -> Result<i32> {
            Ok(0)
        }

        pub fn byte_chunk_data_directive(&self) -> DataDirective {
            DataDirective::kQuad
        }

        pub fn write_byte_chunk(&mut self, data: &[u8]) -> Result<usize> {
            assert_eq!(self.byte_chunk_data_directive(), DataDirective::kQuad);
            let quad_ptr: *const u64 = data.as_ptr() as *const u64;
            let quad_value: u64 = unsafe { *quad_ptr };
            self.hex_literal(quad_value)
        }

        fn declare_symbol_global(&mut self, name: &str) -> Result<()> {
            self.hlasm_print_line(format!("* Global Symbol {}\n", name).as_str())
        }

        fn hlasm_print_line(&mut self, fmt: &str) -> Result<()> {
            let k_asm_max_line_len = 71;
            let k_asm_cont_indent_len = 15;
            let k_asm_cont_max_len = k_asm_max_line_len - k_asm_cont_indent_len;
            let indent = " ".repeat(k_asm_cont_indent_len);

            let mut buffer = String::from(fmt);
            let mut offset = 0;

            if buffer.len() > k_asm_max_line_len && buffer.chars().nth(k_asm_max_line_len).unwrap() != '\n' {
                self.fp_.write_all(buffer[offset..k_asm_max_line_len].as_bytes())?;
                self.fp_.write_all("-\n".as_bytes())?;
                offset += k_asm_max_line_len;

                let mut remaining = buffer.len() - k_asm_max_line_len;
                while remaining > k_asm_cont_max_len {
                    self.fp_.write_all(indent.as_bytes())?;
                    self.fp_.write_all(buffer[offset..offset + k_asm_cont_max_len].as_bytes())?;
                    self.fp_.write_all("-\n".as_bytes())?;
                    offset += k_asm_cont_max_len;
                    remaining -= k_asm_cont_max_len;
                }
                if remaining > 0 {
                    self.fp_.write_all(indent.as_bytes())?;
                    self.fp_.write_all(buffer[offset..].as_bytes())?;
                }
            } else {
                self.fp_.write_all(buffer.as_bytes())?;
            }
            Ok(())
        }
    }
}
