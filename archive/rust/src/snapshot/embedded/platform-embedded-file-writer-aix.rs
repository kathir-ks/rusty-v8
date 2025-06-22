// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::File;
use std::io::Write;

// Replaces #include "src/objects/instruction-stream.h"
// Assuming InstructionStream and kMetadataAlignment are defined elsewhere in V8
// We'll define a placeholder for now.
const K_METADATA_ALIGNMENT: usize = 16; // Placeholder value

#[derive(Debug, Clone, Copy)]
enum DataDirective {
    Byte,
    Long,
    Quad,
}

impl DataDirective {
    fn as_str(&self) -> &'static str {
        match self {
            DataDirective::Byte => ".byte",
            DataDirective::Long => ".long",
            DataDirective::Quad => ".llong",
        }
    }
}

struct PlatformEmbeddedFileWriterAIX {
    fp_: File,
}

impl PlatformEmbeddedFileWriterAIX {
    fn new(file: File) -> Self {
        PlatformEmbeddedFileWriterAIX { fp_: file }
    }

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
        self.indented_data_directive(DataDirective::Long)?;
        writeln!(self.fp_, "{}", value)?;
        self.newline()
    }

    fn declare_symbol_global(&mut self, name: &str) -> std::io::Result<()> {
        // These symbols are not visible outside of the final binary, this allows for
        // reduced binary size, and less work for the dynamic linker.
        writeln!(self.fp_, ".globl {}, hidden", name)
    }

    fn align_to_code_alignment(&mut self) -> std::io::Result<()> {
        // Replaces V8_TARGET_ARCH_X64 and V8_TARGET_ARCH_PPC64 checks with a generic check.
        // Needs to be configured at compile time or runtime depending on the architecture.

        // Placeholder for K_CODE_ALIGNMENT - replace with actual value if known.
        const K_CODE_ALIGNMENT: usize = 32;
        if (1 << 6) >= K_CODE_ALIGNMENT {
            writeln!(self.fp_, ".align 6")
        } else {
            writeln!(self.fp_, ".align 5")
        }
    }

    fn align_to_data_alignment(&mut self) -> std::io::Result<()> {
        // Replaces InstructionStream::kMetadataAlignment with K_METADATA_ALIGNMENT
        if (1 << 3) >= K_METADATA_ALIGNMENT {
            writeln!(self.fp_, ".align 3")
        } else {
            //Handle the error case if the metadata alignment is too large.
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Metadata alignment is too large.",
            ))
        }
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

    fn declare_function_begin(&mut self, name: &str, _size: u32) -> std::io::Result<()> {
        self.newline()?;

        //This needs to be replaced with a proper check to the compilation configuration.
        const ENABLE_CONTROL_FLOW_INTEGRITY_BOOL: bool = true;
        if ENABLE_CONTROL_FLOW_INTEGRITY_BOOL {
            self.declare_symbol_global(name)?;
        }
        writeln!(self.fp_, ".csect {}[DS]", name)?; // function descriptor
        writeln!(self.fp_, "{}:", name)?;
        writeln!(self.fp_, ".llong .{}, 0, 0", name)?;
        self.section_text()?;
        writeln!(self.fp_, ".{}:", name)
    }

    fn declare_function_end(&mut self, _name: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn file_prologue(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn declare_external_filename(&mut self, _fileid: i32, _filename: &str) -> std::io::Result<()> {
        // File name cannot be declared with an identifier on AIX.
        // We use the SourceInfo method to emit debug info in
        //.xline <line-number> <file-name> format.
        Ok(())
    }

    fn file_epilogue(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn indented_data_directive(&mut self, directive: DataDirective) -> std::io::Result<usize> {
        write!(self.fp_, "  {} ", directive.as_str())
    }

    fn byte_chunk_data_directive(&self) -> DataDirective {
        // PPC uses a fixed 4 byte instruction set, using .long
        // to prevent any unnecessary padding.
        DataDirective::Long
    }

    fn newline(&mut self) -> std::io::Result<()> {
        writeln!(self.fp_)
    }
}