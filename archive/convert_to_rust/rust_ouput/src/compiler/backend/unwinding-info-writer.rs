// Converted from V8 C++ source files:
// Header: unwinding-info-writer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod flags {
    pub struct Flags {
        pub perf_prof_unwinding_info: PerfProfUnwindingInfo,
    }

    impl Flags {
        pub fn new() -> Self {
            Self {
                perf_prof_unwinding_info: PerfProfUnwindingInfo { value: false },
            }
        }
    }

    pub struct PerfProfUnwindingInfo {
        pub value: bool,
    }
}

#[cfg(all(
    not(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "x86_64",
        target_arch = "s390x",
        target_arch = "powerpc64"
    ))
))]
pub mod unsupported {
    use crate::flags;

    use std::ptr::null_mut;

    pub struct EhFrameWriter {}

    pub struct Zone {}

    pub struct InstructionBlock {}

    pub struct UnwindingInfoWriter {
        zone: *mut Zone,
    }

    impl UnwindingInfoWriter {
        pub fn new(zone: *mut Zone) -> Self {
            assert!(!flags::Flags::new().perf_prof_unwinding_info.value,
                "--perf-prof-unwinding-info should be statically disabled if not supported");
            Self { zone }
        }

        pub fn set_number_of_instruction_blocks(&mut self, _number: i32) {}

        pub fn begin_instruction_block(&mut self, _pc_offset: i32, _instruction_block: *const InstructionBlock) {}

        pub fn end_instruction_block(&mut self, _instruction_block: *const InstructionBlock) {}

        pub fn finish(&mut self, _code_size: i32) {}

        pub fn eh_frame_writer(&self) -> *mut EhFrameWriter {
            null_mut()
        }
    }
}

#[cfg(all(
    not(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "x86_64",
        target_arch = "s390x",
        target_arch = "powerpc64"
    ))
))]
pub use unsupported::*;
