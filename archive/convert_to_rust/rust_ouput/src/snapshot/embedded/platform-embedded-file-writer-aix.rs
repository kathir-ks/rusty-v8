// Converted from V8 C++ source files:
// Header: platform-embedded-file-writer-aix.h
// Implementation: platform-embedded-file-writer-aix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod platform_embedded_file_writer_aix {
    use std::fs::File;
    use std::io::Write;

    use crate::objects::instruction_stream::InstructionStream;
    use crate::snapshot::embedded::platform_embedded_file_writer_base::{
        DataDirective, EmbeddedTargetArch, EmbeddedTargetOs,
        PlatformEmbeddedFileWriterBase,
    };

    pub struct PlatformEmbeddedFileWriterAIX {
        target_arch_: EmbeddedTargetArch,
        target_os_: EmbeddedTargetOs,
        fp_: File, // Assuming fp_ is a file pointer
    }

    impl PlatformEmbeddedFileWriterAIX {
        pub fn new(
            target_arch: EmbeddedTargetArch,
            target_os: EmbeddedTargetOs,
            file: File,
        ) -> Self {
            assert_eq!(target_os, EmbeddedTargetOs::kAIX);
            PlatformEmbeddedFileWriterAIX {
                target_arch_: target_arch,
                target_os_: target_os,
                fp_: file,
            }
        }
    }

    impl PlatformEmbeddedFileWriterBase for PlatformEmbeddedFileWriterAIX {
        fn section_text(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_, ".csect [GL], 6")
        }

        fn section_ro_data(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_, ".csect[RO]")
        }

        fn declare_uint32(&mut self, name: &str, value: u32) -> std::io::Result<()> {
            self.declare_symbol_global(name)?;
            writeln!(self.fp_, ".align 2")?;
            writeln!(self.fp_, "{}:", name)?;
            self.indented_data_directive(DataDirective::kLong)?;
            writeln!(self.fp_, "{}", value)?;
            self.newline()
        }

        fn declare_symbol_global(&mut self, name: &str) -> std::io::Result<()> {
            // These symbols are not visible outside of the final binary, this allows for
            // reduced binary size, and less work for the dynamic linker.
            writeln!(self.fp_, ".globl {}, hidden", name)
        }

        fn align_to_code_alignment(&mut self) -> std::io::Result<()> {
            // On x64 use 64-bytes code alignment to allow 64-bytes loop header alignment.
            #[cfg(target_arch = "x86_64")]
            {
                //static_assert((1 << 6) >= kCodeAlignment);
                writeln!(self.fp_, ".align 6")?;
            }
            #[cfg(target_arch = "powerpc64")]
            {
                // 64 byte alignment is needed on ppc64 to make sure p10 prefixed instructions
                // don't cross 64-byte boundaries.
                //static_assert((1 << 6) >= kCodeAlignment);
                writeln!(self.fp_, ".align 6")?;
            }
            #[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc64")))]
            {
                //static_assert((1 << 5) >= kCodeAlignment);
                writeln!(self.fp_, ".align 5")?;
            }
            Ok(())
        }

        fn align_to_data_alignment(&mut self) -> std::io::Result<()> {
            //static_assert((1 << 3) >= InstructionStream::kMetadataAlignment);
            writeln!(self.fp_, ".align 3")
        }

        fn comment(&mut self, string: &str) -> std::io::Result<()> {
            writeln!(self.fp_, "// {}", string)
        }

        fn declare_label(&mut self, name: &str) -> std::io::Result<()> {
            // .global is required on AIX, if the label is used/referenced in another file
            // later to be linked.
            writeln!(self.fp_, ".globl {}", name)?;
            writeln!(self.fp_, "{}:", name)
        }

        fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> std::io::Result<()> {
            writeln!(self.fp_, ".xline {}, \"{}\"", line, filename)
        }

        fn declare_function_begin(&mut self, name: &str, size: u32) -> std::io::Result<()> {
            self.newline()?;
            if cfg!(feature = "enable_control_flow_integrity") {
                self.declare_symbol_global(name)?;
            }
            writeln!(self.fp_, ".csect {}[DS]", name)?; // function descriptor
            writeln!(self.fp_, "{}:", name)?;
            writeln!(self.fp_, ".llong .{}, 0, 0", name)?;
            self.section_text()?;
            writeln!(self.fp_, ".{}:", name)
        }

        fn declare_function_end(&mut self, name: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn file_prologue(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> std::io::Result<()> {
            // File name cannot be declared with an identifier on AIX.
            // We use the SourceInfo method to emit debug info in
            //.xline <line-number> <file-name> format.
            Ok(())
        }

        fn file_epilogue(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        fn indented_data_directive(&mut self, directive: DataDirective) -> std::io::Result<i32> {
            let directive_str = match directive {
                DataDirective::kByte => ".byte",
                DataDirective::kLong => ".long",
                DataDirective::kQuad => ".llong",
            };
            write!(self.fp_, "  {} ", directive_str)?;
            Ok(0) // Returning 0 as C++ fprintf returns the number of characters written, which is not needed here.
        }

        fn byte_chunk_data_directive(&self) -> DataDirective {
            // PPC uses a fixed 4 byte instruction set, using .long
            // to prevent any unnecessary padding.
            DataDirective::kLong
        }
        fn set_file(&mut self, file: File) -> std::io::Result<()> {
            self.fp_ = file;
            Ok(())
        }
        fn newline(&mut self) -> std::io::Result<()> {
            writeln!(self.fp_,"")
        }
    }
}
