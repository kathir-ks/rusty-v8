// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::arch::asm;
use std::mem;
use std::sync::Arc;
use std::convert::TryInto;
//use crate::compiler::wasm_compiler::Assembler;

//use crate::codegen::arm64::assembler_arm64::Assembler;
use crate::codegen::arm64::assembler_arm64_inl::V8;
use crate::sandbox::sandbox::CPU;
use crate::codegen::code_stub_assembler::arm;
use crate::init::v8::OS;
use crate::interpreter::bytecode_generator::v8 as v8_2;
use crate::codegen::code_stub_assembler::CallType;
use crate::ast::ast::instruction;
use crate::codegen::arm64::instructions_arm64::Instruction;
use crate::codegen::assembler_arch::source;
use crate::codegen::register_arch::is;
use crate::init::bootstrapper::Isolate;
use crate::codegen::handler_table::Address;
use crate::codegen::code_stub_assembler::operations;
use crate::compiler::turboshaft::graph_visualizer::Block;
use crate::compiler::turboshaft::graph_visualizer::Operation;
use crate::codegen::register_arch::Register;
use crate::codegen::reloc_info::RelocInfo;
use crate::compiler::js_inlining::Node;
use crate::wasm::module_decoder_impl::U;
use crate::compiler::backend::riscv::instruction_selector_riscv::OpIndex;
use std::sync::Mutex;
use crate::torque::instructions::InstructionBase;
use crate::codegen::external_reference_table::Type;
use crate::torque::ls::message::Args;
use crate::torque::ls::message::R;
use crate::codegen::code_stub_assembler::Data;
use crate::execution::frames_inl::InnerPointerToCodeCacheEntry;
use crate::init::bootstrapper::Map;
use crate::tasks::cancelable_task::Cancelable;
use std::ffi::c_void;
use crate::codegen::constant_pool::Zone;
use std::sync::RwLock;

#[cfg(target_os = "windows")]
extern "system" {
    pub fn FlushInstructionCache(
        hProcess: *mut std::ffi::c_void,
        lpBaseAddress: *mut std::ffi::c_void,
        dwSize: usize,
    ) -> i32;
    pub fn GetCurrentProcess() -> *mut std::ffi::c_void;
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn sys_icache_invalidate(addr: *const c_void, len: usize) -> i32;
}

pub struct CacheLineSizes {
    cache_type_register_: u32,
}

impl CacheLineSizes {
    pub fn new() -> Self {
        let mut cache_type_register_: u32 = 0;
        #[cfg(all(target_arch = "aarch64", not(target_os = "windows"), not(target_os = "macos")))]
        unsafe {
            asm!(
                "mrs {ctr}, ctr_el0",
                ctr = out(reg) cache_type_register_
            );
        }
        CacheLineSizes {
            cache_type_register_: cache_type_register_,
        }
    }

    pub fn icache_line_size(&self) -> u32 {
        self.extract_cache_line_size(0)
    }

    pub fn dcache_line_size(&self) -> u32 {
        self.extract_cache_line_size(16)
    }

    fn extract_cache_line_size(&self, cache_line_size_shift: i32) -> u32 {
        4 << ((self.cache_type_register_ >> cache_line_size_shift) & 0xF)
    }
}

pub struct CpuFeatures {}

impl CpuFeatures {
   pub fn flush_i_cache(address: *mut std::ffi::c_void, length: usize) {
    #[cfg(all(target_arch = "aarch64"))]
    {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                FlushInstructionCache(GetCurrentProcess(), address, length);
            }
        }
        #[cfg(target_os = "macos")]
        {
            unsafe {
                sys_icache_invalidate(address as *const c_void, length);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let begin = address as *mut u8;
            unsafe {
                std::arch::asm!(
                    "
                    clflush {{0}}
                    ",
                    in(reg) begin,
                    options(nostack, preserves_flags)
                );
            }
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            let start = address as usize;
            let sizes = CacheLineSizes::new();
            let dsize = sizes.dcache_line_size() as usize;
            let isize = sizes.icache_line_size() as usize;

            //DCHECK_EQ(CountSetBits(dsize, 64), 1);  //TODO implement CountSetBits function
            //DCHECK_EQ(CountSetBits(isize, 64), 1);

            let dstart = start & !(dsize - 1);
            let istart = start & !(isize - 1);
            let end = start + length;

            let dline = dstart;
            let iline = istart;
            unsafe {
                asm!(
                    "
                    0:
                    dc civac, {dline}
                    add {dline}, {dline}, {dsize}
                    cmp {dline}, {end}
                    b.lt 0b

                    dsb ish

                    1:
                    ic ivau, {iline}
                    add {iline}, {iline}, {isize}
                    cmp {iline}, {end}
                    b.lt 1b

                    dsb ish
                    isb
                    ",
                    dline = inout(reg) dline,
                    iline = inout(reg) iline,
                    dsize = in(reg) dsize,
                    isize = in(reg) isize,
                    end = in(reg) end,
                    options(nostack, preserves_flags, nomem)
                );
            }
        }
    }
}
}
//TODO Implement CountSetBits
pub fn CountSetBits(x: usize, size: i32)->i32{
    x.count_ones().try_into().unwrap()
}
