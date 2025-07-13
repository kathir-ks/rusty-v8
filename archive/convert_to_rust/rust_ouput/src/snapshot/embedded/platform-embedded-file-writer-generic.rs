// Converted from V8 C++ source files:
// Header: platform-embedded-file-writer-generic.h
// Implementation: platform-embedded-file-writer-generic.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/snapshot/embedded/platform-embedded-file-writer-generic.h

pub mod platform_embedded_file_writer_generic {
    use crate::snapshot::embedded::platform_embedded_file_writer_base::{
        DataDirective, EmbeddedTargetArch, EmbeddedTargetOs,
        PlatformEmbeddedFileWriterBase,
    };
    use std::io::Write;

    pub struct PlatformEmbeddedFileWriterGeneric {
        target_arch_: EmbeddedTargetArch,
        target_os_: EmbeddedTargetOs,
        fp_: Box<dyn Write>, // Using a Box<dyn Write> for file writing
    }

    impl PlatformEmbeddedFileWriterGeneric {
        pub fn new(
            target_arch: EmbeddedTargetArch,
            target_os: EmbeddedTargetOs,
            fp: Box<dyn Write>,
        ) -> Self {
            assert!(
                target_os == EmbeddedTargetOs::kChromeOS
                    || target_os == EmbeddedTargetOs::kFuchsia
                    || target_os == EmbeddedTargetOs::kGeneric
            );
            PlatformEmbeddedFileWriterGeneric {
                target_arch_: target_arch,
                target_os_: target_os,
                fp_: fp,
            }
        }

        pub fn section_text(&mut self) -> std::io::Result<()> {
            if self.target_os_ == EmbeddedTargetOs::kChromeOS {
                writeln!(self.fp_, ".section .text.hot.embedded")
            } else {
                writeln!(self.fp_, ".section .text")
            }
        }

        pub fn section_ro_data(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_, ".section .rodata")
        }

        pub fn align_to_code_alignment(&mut self) -> std::io::Result<()> {
            #[cfg(all(
                any(target_os = "android", target_os = "linux"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            ))]
            {
                writeln!(self.fp_, ".balign 4096")?;
            }

            #[cfg(target_arch = "x86_64")]
            {
                writeln!(self.fp_, ".balign 64")?;
            }

            #[cfg(target_arch = "powerpc64")]
            {
                writeln!(self.fp_, ".balign 64")?;
            }

            #[cfg(not(any(
                all(
                    any(target_os = "android", target_os = "linux"),
                    any(target_arch = "x86_64", target_arch = "aarch64")
                ),
                target_arch = "x86_64",
                target_arch = "powerpc64"
            )))]
            {
                writeln!(self.fp_, ".balign 32")?;
            }

            Ok(())
        }

        pub fn align_to_page_size_if_needed(&mut self) -> std::io::Result<()> {
            #[cfg(all(
                any(target_os = "android", target_os = "linux"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            ))]
            {
                writeln!(self.fp_, ".balign 4096")?;
            }
            Ok(())
        }

        pub fn align_to_data_alignment(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_, ".balign 8")
        }

        pub fn declare_uint32(&mut self, name: &str, value: u32) -> std::io::Result<()> {
            self.declare_symbol_global(name)?;
            self.declare_label(name)?;
            self.indented_data_directive(DataDirective::kLong)?;
            writeln!(self.fp_, "{}", value)
        }

        pub fn declare_symbol_global(&mut self, name: &str) -> std::io::Result<()> {
            let symbol_prefix = "";
            writeln!(self.fp_, ".global {}{}", symbol_prefix, name)?;
            writeln!(self.fp_, ".hidden {}", name)
        }

        pub fn declare_label(&mut self, name: &str) -> std::io::Result<()> {
            let symbol_prefix = "";
            writeln!(self.fp_, "{}{}:", symbol_prefix, name)
        }

        pub fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> std::io::Result<()> {
            writeln!(self.fp_, ".loc {} {} {}", fileid, line)
        }

        pub fn declare_function_begin(&mut self, name: &str, size: u32) -> std::io::Result<()> {
            self.declare_label(name)?;

            if self.target_arch_ == EmbeddedTargetArch::kArm
                || self.target_arch_ == EmbeddedTargetArch::kArm64
            {
                writeln!(self.fp_, ".type {}, %function", name)?;
            } else {
                writeln!(self.fp_, ".type {}, @function", name)?;
            }
            writeln!(self.fp_, ".size {}, {}", name, size)
        }

        pub fn declare_function_end(&mut self, name: &str) -> std::io::Result<()> {
            Ok(())
        }

        pub fn comment(&mut self, string: &str) -> std::io::Result<()> {
            writeln!(self.fp_, "// {}", string)
        }

        pub fn file_prologue(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        pub fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> std::io::Result<()> {
            let fixed_filename = filename.replace("\\", "/");
            writeln!(self.fp_, ".file {} \"{}\"", fileid, fixed_filename)
        }

        pub fn file_epilogue(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_, ".section .note.GNU-stack,\"\",%progbits")
        }

        pub fn indented_data_directive(&mut self, directive: DataDirective) -> std::io::Result<usize> {
            let directive_str = match directive {
                DataDirective::kByte => ".byte",
                DataDirective::kLong => ".long",
                DataDirective::kQuad => ".quad",
                DataDirective::kOcta => ".octa",
            };
            write!(self.fp_, "  {} ", directive_str)?;
            Ok(0)
        }

        pub fn byte_chunk_data_directive(&self) -> DataDirective {
            #[cfg(any(target_arch = "mips64", target_arch = "loongarch64"))]
            {
                DataDirective::kLong
            }
            #[cfg(not(any(target_arch = "mips64", target_arch = "loongarch64")))]
            {
                DataDirective::kByte
            }
        }
    }

    impl PlatformEmbeddedFileWriterBase for PlatformEmbeddedFileWriterGeneric {
        fn fp(&mut self) -> &mut dyn Write {
            &mut *self.fp_
        }
    }
}
