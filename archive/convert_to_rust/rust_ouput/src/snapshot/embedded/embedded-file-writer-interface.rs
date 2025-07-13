// Converted from V8 C++ source files:
// Header: embedded-file-writer-interface.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct Builtins {}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
mod win64_unwindinfo {
    pub struct BuiltinUnwindInfo {}
}

pub const K_DEFAULT_EMBEDDED_VARIANT: &str = "Default";

pub struct LabelInfo {
    pub offset: i32,
    pub name: String,
}

pub trait EmbeddedFileWriterInterface {
    // We maintain a database of filenames to synthetic IDs.
    fn lookup_or_add_externally_compiled_filename(&mut self, filename: &str) -> i32;
    fn get_externally_compiled_filename(&self, index: i32) -> &str;
    fn get_externally_compiled_filename_count(&self) -> usize;

    // The isolate will call the method below just prior to replacing the
    // compiled builtin InstructionStream objects with trampolines.
    fn prepare_builtin_source_position_map(&mut self, builtins: &mut Builtins);

    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "x86_64")]
    fn set_builtin_unwind_data(
        &mut self,
        builtin: Builtin,
        unwinding_info: &win64_unwindinfo::BuiltinUnwindInfo,
    );
}

pub enum Builtin {
    // Example variants, add more as needed
    kFirstBuiltin,
    kSecondBuiltin,
    kLastBuiltin,
}
