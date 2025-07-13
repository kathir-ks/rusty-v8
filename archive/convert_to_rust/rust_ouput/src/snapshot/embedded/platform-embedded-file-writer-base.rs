// Converted from V8 C++ source files:
// Header: platform-embedded-file-writer-base.h
// Implementation: platform-embedded-file-writer-base.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/snapshot/embedded/platform-embedded-file-writer-base.h
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum DataDirective {
    kByte,
    kLong,
    kQuad,
    kOcta,
}

pub fn pointer_size_directive() -> DataDirective {
    if std::mem::size_of::<usize>() == 8 {
        DataDirective::kQuad
    } else {
        assert_eq!(4, std::mem::size_of::<usize>());
        DataDirective::kLong
    }
}

pub fn data_directive_size(directive: DataDirective) -> usize {
    match directive {
        DataDirective::kByte => 1,
        DataDirective::kLong => 4,
        DataDirective::kQuad => 8,
        DataDirective::kOcta => 16,
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum EmbeddedTargetOs {
    kAIX,
    kChromeOS,
    kFuchsia,
    kMac,
    kWin,
    kStarboard,
    kZOS,
    kGeneric, // Everything not covered above falls in here.
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum EmbeddedTargetArch {
    kArm,
    kArm64,
    kIA32,
    kX64,
    kGeneric, // Everything not covered above falls in here.
}

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub trait PlatformEmbeddedFileWriterBase {
    fn set_file(&mut self, fp: File) -> Result<(), std::io::Error>;
    fn fp(&self) -> &File;

    fn section_text(&mut self) -> Result<(), std::io::Error>;
    fn section_ro_data(&mut self) -> Result<(), std::io::Error>;

    fn align_to_code_alignment(&mut self) -> Result<(), std::io::Error>;
    fn align_to_page_size_if_needed(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn align_to_data_alignment(&mut self) -> Result<(), std::io::Error>;

    fn declare_uint32(&mut self, name: &str, value: u32) -> Result<(), std::io::Error>;

    fn declare_symbol_global(&mut self, name: &str) -> Result<(), std::io::Error>;
    fn declare_label(&mut self, name: &str) -> Result<(), std::io::Error>;
    fn declare_label_prolog(&mut self, name: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn declare_label_epilogue(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> Result<(), std::io::Error>;
    fn declare_function_begin(&mut self, name: &str, size: u32) -> Result<(), std::io::Error>;
    fn declare_function_end(&mut self, name: &str) -> Result<(), std::io::Error>;

    // Returns the number of printed characters.
    fn hex_literal(&mut self, value: u64) -> Result<usize, std::io::Error>;

    fn comment(&mut self, string: &str) -> Result<(), std::io::Error>;
    fn newline(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "")
    }

    fn file_prologue(&mut self) -> Result<(), std::io::Error>;
    fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> Result<(), std::io::Error>;
    fn file_epilogue(&mut self) -> Result<(), std::io::Error>;

    fn indented_data_directive(&mut self, directive: DataDirective) -> Result<usize, std::io::Error>;

    fn byte_chunk_data_directive(&self) -> DataDirective {
        DataDirective::kOcta
    }
    fn write_byte_chunk(&mut self, data: &[u8]) -> Result<usize, std::io::Error>;

    // This awkward interface works around the fact that unwind data emission
    // is both high-level and platform-dependent. The former implies it should
    // live in EmbeddedFileWriter, but code there should be platform-independent.
    //
    // Emits unwinding data on x64 Windows, and does nothing otherwise.
    fn maybe_emit_unwind_data(
        &mut self,
        unwind_info_symbol: &str,
        embedded_blob_data_symbol: &str,
        blob: &EmbeddedData,
        unwind_infos: *const std::ffi::c_void,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct GenericPlatformEmbeddedFileWriter {
    fp_: File,
    target_arch: EmbeddedTargetArch,
    target_os: EmbeddedTargetOs,
}

impl GenericPlatformEmbeddedFileWriter {
    pub fn new(target_arch: EmbeddedTargetArch, target_os: EmbeddedTargetOs) -> Self {
        let fp_ = File::create("embedded.S").expect("Failed to create file");
        GenericPlatformEmbeddedFileWriter { fp_, target_arch, target_os }
    }
}

impl PlatformEmbeddedFileWriterBase for GenericPlatformEmbeddedFileWriter {
    fn set_file(&mut self, fp: File) -> Result<(), std::io::Error> {
        self.fp_ = fp;
        Ok(())
    }

    fn fp(&self) -> &File {
        &self.fp_
    }

    fn section_text(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".text")
    }

    fn section_ro_data(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".rodata")
    }

    fn align_to_code_alignment(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".align 4")
    }

    fn align_to_data_alignment(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".align 8")
    }

    fn declare_uint32(&mut self, name: &str, value: u32) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".global {}", name)?;
        writeln!(self.fp(), "{}: .long {}", name, value)
    }

    fn declare_symbol_global(&mut self, name: &str) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".global {}", name)
    }

    fn declare_label(&mut self, name: &str) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "{}:", name)
    }

    fn source_info(&mut self, fileid: i32, filename: &str, line: i32) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "# source_info: fileid={}, filename={}, line={}", fileid, filename, line)
    }

    fn declare_function_begin(&mut self, name: &str, size: u32) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".type {}, @function", name)?;
        writeln!(self.fp(), "{}:", name)
    }

    fn declare_function_end(&mut self, name: &str) -> Result<(), std::io::Error> {
        writeln!(self.fp(), ".size {}, . - {}", name, name)
    }

    fn hex_literal(&mut self, value: u64) -> Result<usize, std::io::Error> {
        write!(self.fp(), "0x{:x}", value)
    }

    fn comment(&mut self, string: &str) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "// {}", string)
    }

    fn file_prologue(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "// File prologue")
    }

    fn declare_external_filename(&mut self, fileid: i32, filename: &str) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "// Declare external filename: fileid={}, filename={}", fileid, filename)
    }

    fn file_epilogue(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.fp(), "// File epilogue")
    }

    fn indented_data_directive(&mut self, directive: DataDirective) -> Result<usize, std::io::Error> {
        let directive_str = match directive {
            DataDirective::kByte => ".byte",
            DataDirective::kLong => ".long",
            DataDirective::kQuad => ".quad",
            DataDirective::kOcta => ".octa",
        };
        write!(self.fp(), "\t{}", directive_str)
    }

    fn write_byte_chunk(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        let k_size = data_directive_size(self.byte_chunk_data_directive());
        let k_half_size = k_size / 2;
        let mut high: u64 = 0;
        let mut low: u64 = 0;

        match k_size {
            1 => {
                low = data[0] as u64;
            }
            4 => {
                low = u32::from_ne_bytes(data[..4].try_into().unwrap()) as u64;
            }
            8 => {
                low = u64::from_ne_bytes(data[..8].try_into().unwrap());
            }
            16 => {
                high = u64::from_ne_bytes(data[k_half_size..].try_into().unwrap());
                low = u64::from_ne_bytes(data[..k_half_size].try_into().unwrap());
            }
            _ => {
                panic!("UNREACHABLE");
            }
        }

        if high != 0 {
            write!(self.fp(), "0x{:x}{:016x}", high, low)
        } else {
            write!(self.fp(), "0x{:x}", low)
        }
    }
}

pub struct EmbeddedData {}

// src/snapshot/embedded/platform-embedded-file-writer-base.cc
use std::convert::TryInto;

impl Drop for GenericPlatformEmbeddedFileWriter {
    fn drop(&mut self) {
        // Ensure the file is flushed before dropping
        let _ = self.fp().sync_all();
    }
}

fn default_embedded_target_arch() -> EmbeddedTargetArch {
    if cfg!(target_arch = "arm") {
        EmbeddedTargetArch::kArm
    } else if cfg!(target_arch = "aarch64") {
        EmbeddedTargetArch::kArm64
    } else if cfg!(target_arch = "x86") {
        EmbeddedTargetArch::kIA32
    } else if cfg!(target_arch = "x86_64") {
        EmbeddedTargetArch::kX64
    } else {
        EmbeddedTargetArch::kGeneric
    }
}

fn to_embedded_target_arch(s: Option<&str>) -> EmbeddedTargetArch {
    match s {
        None => default_embedded_target_arch(),
        Some(s) => {
            match s {
                "arm" => EmbeddedTargetArch::kArm,
                "arm64" => EmbeddedTargetArch::kArm64,
                "ia32" => EmbeddedTargetArch::kIA32,
                "x64" => EmbeddedTargetArch::kX64,
                _ => EmbeddedTargetArch::kGeneric,
            }
        }
    }
}

fn default_embedded_target_os() -> EmbeddedTargetOs {
    if cfg!(target_os = "aix") {
        EmbeddedTargetOs::kAIX
    } else if cfg!(target_os = "macos") {
        EmbeddedTargetOs::kMac
    } else if cfg!(target_os = "windows") {
        EmbeddedTargetOs::kWin
    } else if cfg!(target_os = "zos") {
        EmbeddedTargetOs::kZOS
    } else {
        EmbeddedTargetOs::kGeneric
    }
}

fn to_embedded_target_os(s: Option<&str>) -> EmbeddedTargetOs {
    match s {
        None => default_embedded_target_os(),
        Some(s) => {
            match s {
                "aix" | "os400" => EmbeddedTargetOs::kAIX,
                "chromeos" => EmbeddedTargetOs::kChromeOS,
                "fuchsia" => EmbeddedTargetOs::kFuchsia,
                "ios" | "mac" => EmbeddedTargetOs::kMac,
                "win" => EmbeddedTargetOs::kWin,
                "starboard" => EmbeddedTargetOs::kStarboard,
                "zos" => EmbeddedTargetOs::kZOS,
                _ => EmbeddedTargetOs::kGeneric,
            }
        }
    }
}

pub fn new_platform_embedded_file_writer(
    target_arch: Option<&str>,
    target_os: Option<&str>,
) -> Box<dyn PlatformEmbeddedFileWriterBase> {
    let embedded_target_arch = to_embedded_target_arch(target_arch);
    let embedded_target_os = to_embedded_target_os(target_os);

    if embedded_target_os == EmbeddedTargetOs::kStarboard {
        match default_embedded_target_os() {
            EmbeddedTargetOs::kMac | EmbeddedTargetOs::kWin => {
                // V8_TARGET_OS_WIN is used to enable WINDOWS-specific assembly code,
                // for windows-hosted non-windows targets, we should still fallback to
                // the generic writer.
                //embedded_target_os = DefaultEmbeddedTargetOs();
            }
            _ => {}
        }
    }

    if embedded_target_os == EmbeddedTargetOs::kAIX {
        Box::new(GenericPlatformEmbeddedFileWriter::new(embedded_target_arch, embedded_target_os)) // Replace with actual AIX writer
    } else if embedded_target_os == EmbeddedTargetOs::kMac {
        Box::new(GenericPlatformEmbeddedFileWriter::new(embedded_target_arch, embedded_target_os)) // Replace with actual Mac writer
    } else if embedded_target_os == EmbeddedTargetOs::kWin {
        Box::new(GenericPlatformEmbeddedFileWriter::new(embedded_target_arch, embedded_target_os)) // Replace with actual Win writer
    } else if embedded_target_os == EmbeddedTargetOs::kZOS {
        Box::new(GenericPlatformEmbeddedFileWriter::new(embedded_target_arch, embedded_target_os)) // Replace with actual ZOS writer
    } else {
        Box::new(GenericPlatformEmbeddedFileWriter::new(embedded_target_arch, embedded_target_os))
    }
}

#[allow(dead_code)]
pub fn is_drum_brake_instruction_handler(name: &str) -> bool {
    name.starts_with("Builtins_r2r_")
        || name.starts_with("Builtins_r2s_")
        || name.starts_with("Builtins_s2r_")
        || name.starts_with("Builtins_s2s_")
}
