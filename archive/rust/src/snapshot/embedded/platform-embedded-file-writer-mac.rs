// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::File;
use std::io::Write;

mod objects {
    pub mod instruction_stream {
        pub const K_METADATA_ALIGNMENT: usize = 8;
    }
}

const K_CODE_ALIGNMENT: usize = 32; // Default value, overwritten conditionally below

#[cfg(target_arch = "x86_64")]
const K_CODE_ALIGNMENT: usize = 64;

#[cfg(target_arch = "powerpc64")]
const K_CODE_ALIGNMENT: usize = 64;

#[cfg(target_arch = "aarch64")]
const K_CODE_ALIGNMENT: usize = 16384;


enum DataDirective {
    Byte,
    Long,
    Quad,
    Octa,
}

impl DataDirective {
    fn as_str(&self) -> &'static str {
        match self {
            DataDirective::Byte => ".byte",
            DataDirective::Long => ".long",
            DataDirective::Quad => ".quad",
            DataDirective::Octa => ".octa",
        }
    }
}

struct PlatformEmbeddedFileWriterMac {
    fp_: File,
}

impl PlatformEmbeddedFileWriterMac {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let fp_ = File::create(file_path)?;
        Ok(PlatformEmbeddedFileWriterMac { fp_ })
    }

    pub fn section_text(&mut self) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, ".text")
    }

    pub fn section_ro_data(&mut self) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, ".const_data")
    }

    pub fn declare_uint32(&mut self, name: &str, value: u32) -> Result<(), std::io::Error> {
        self.declare_symbol_global(name)?;
        self.declare_label(name)?;
        self.indented_data_directive(DataDirective::Long)?;
        write!(&mut self.fp_, "{}", value)?;
        self.newline()
    }

    pub fn declare_symbol_global(&mut self, name: &str) -> Result<(), std::io::Error> {
        // TODO(jgruber): Investigate switching to .globl. Using .private_extern
        // prevents something along the compilation chain from messing with the
        // embedded blob. Using .global here causes embedded blob hash verification
        // failures at runtime.
        writeln!(&mut self.fp_, ".private_extern _{}", name)
    }

    pub fn align_to_code_alignment(&mut self) -> Result<(), std::io::Error> {
        #[cfg(target_arch = "x86_64")]
        {
            // On x64 use 64-bytes code alignment to allow 64-bytes loop header alignment.
            assert!(64 >= K_CODE_ALIGNMENT);
            writeln!(&mut self.fp_, ".balign 64")
        }
        #[cfg(target_arch = "powerpc64")]
        {
            // 64 byte alignment is needed on ppc64 to make sure p10 prefixed instructions
            // don't cross 64-byte boundaries.
            assert!(64 >= K_CODE_ALIGNMENT);
            writeln!(&mut self.fp_, ".balign 64")
        }
        #[cfg(target_arch = "aarch64")]
        {
            // ARM64 macOS has a 16kiB page size. Since we want to remap it on the heap,
            // needs to be page-aligned.
            writeln!(&mut self.fp_, ".balign 16384")
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "aarch64")))]
        {
            assert!(32 >= K_CODE_ALIGNMENT);
            writeln!(&mut self.fp_, ".balign 32")
        }
    }

    pub fn align_to_page_size_if_needed(&mut self) -> Result<(), std::io::Error> {
        #[cfg(target_arch = "aarch64")]
        {
            // ARM64 macOS has a 16kiB page size. Since we want to remap builtins on the
            // heap, make sure that the trailing part of the page doesn't contain anything
            // dangerous.
            writeln!(&mut self.fp_, ".balign 16384")
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            Ok(())
        }
    }

    pub fn align_to_data_alignment(&mut self) -> Result<(), std::io::Error> {
        assert!(8 >= objects::instruction_stream::K_METADATA_ALIGNMENT);
        writeln!(&mut self.fp_, ".balign 8")
    }

    pub fn comment(&mut self, string: &str) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, "// {}", string)
    }

    pub fn declare_label(&mut self, name: &str) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, "_{}:", name)
    }

    pub fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, ".loc {} {}", fileid, line)
    }

    // TODO(mmarchini): investigate emitting size annotations for OS X
    pub fn declare_function_begin(&mut self, name: &str, _size: u32) -> Result<(), std::io::Error> {
        self.declare_label(name)?;

        // TODO(mvstanton): Investigate the proper incantations to mark the label as
        // a function on OSX.
        Ok(())
    }

    pub fn declare_function_end(&mut self, _name: &str) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn file_prologue(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_, ".file {} \"{}\"", fileid, filename)
    }

    pub fn file_epilogue(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn indented_data_directive(&mut self, directive: DataDirective) -> Result<usize, std::io::Error> {
        write!(&mut self.fp_, "  {} ", directive.as_str())
    }

    fn newline(&mut self) -> Result<(), std::io::Error> {
        writeln!(&mut self.fp_)
    }
}