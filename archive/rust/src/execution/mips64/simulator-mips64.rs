// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Declares a Simulator for MIPS instructions if we are not generating a native
// MIPS binary. This Simulator allows us to run and debug MIPS code generation
// on regular desktop machines.
// V8 calls into generated code via the GeneratedCode wrapper,
// which will start execution in the Simulator or forwards to the real entry
// on a MIPS HW platform.

// globals.h defines USE_SIMULATOR.

// Note: Globals.h is not directly translated. Conditional compilation
// based on a "USE_SIMULATOR" feature flag would be the Rust equivalent.

// The `common` and `utils` modules are not directly mapped as they represent generic utilities.
// Instead we focus on the core simulation logic and define stubs where necessary.

// Example stubs for types used from these modules:
type Address = usize; // Replace with appropriate address type if needed
type Isolate = u32;
mod base {
    pub type Vector<T> = Vec<T>;
    pub struct EmbeddedVector<T, const N: usize> {
        data: [T; N],
        length: usize,
    }

    impl<T, const N: usize> EmbeddedVector<T, const N> {
        pub fn new() -> Self {
            Self {
                data: core::array::from_fn(|_| unsafe { core::mem::zeroed() }),
                length: 0,
            }
        }
    }

    impl<T: Copy, const N: usize> From<&[T]> for EmbeddedVector<T, N> {
        fn from(slice: &[T]) -> Self {
            let mut ev = EmbeddedVector::<T, N>::new();
            let len = slice.len().min(N);
            ev.length = len;
            ev.data[..len].copy_from_slice(&slice[..len]);
            ev
        }
    }

    pub mod hashmap {
        pub struct CustomMatcherHashMap {}
    }
    pub mod strings {
        // Add string-related types or functions as needed
    }
    pub use std::sync::Mutex;
}

mod codegen {
    pub mod assembler {
        // Add assembler related types or functions as needed
        pub type Instr = u32;
    }
    pub mod mips64 {
        pub mod constants_mips64 {
            // Add constants related types or functions as needed
        }
    }
}

mod execution {
    pub mod simulator_base {
        // Add simulator base related types or functions as needed
        pub struct SimulatorBase {}
    }
}
mod v8 {
    pub mod internal {
        pub use crate::Address;
        pub use crate::Isolate;
    }
}
#[allow(dead_code)]
fn compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
    if a == b {
        0
    } else if a < b {
        -1
    } else {
        1
    }
}

#[allow(dead_code)]
fn nabs<T: std::ops::Neg + Ord + Copy>(a: T) -> T
where
    <T as std::ops::Neg>::Output: Copy,
{
    if a < T::zero() {
        a
    } else {
        -a
    }
}

const KB: usize = 1024; // Define KB here
struct Flags {
    sim_stack_size: usize,
}

static mut v8_flags: Flags = Flags { sim_stack_size: 1 };

#[cfg(feature = "USE_SIMULATOR")]
pub mod simulator_mips64 {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::mem::MaybeUninit;
    use std::ptr;

    const kInstrSize: usize = 4;
    const kMSALanesByte: usize = 16;
    const kMSALanesHalf: usize = 8;
    const kMSALanesWord: usize = 4;
    const kMSALanesDword: usize = 2;
    const kMaxStopCode: usize = 1024;
    const nopInstr: u32 = 0;
    use codegen::assembler::Instr;

    /// Represents a cache page for the simulator.
    pub struct CachePage {
        data_: [u8; Self::K_PAGE_SIZE], // The cached data.
        validity_map_: [u8; Self::K_VALIDITY_MAP_SIZE], // One byte per line.
    }

    impl CachePage {
        pub const LINE_VALID: i32 = 0;
        pub const LINE_INVALID: i32 = 1;

        pub const K_PAGE_SHIFT: usize = 12;
        pub const K_PAGE_SIZE: usize = 1 << Self::K_PAGE_SHIFT;
        pub const K_PAGE_MASK: usize = Self::K_PAGE_SIZE - 1;
        pub const K_LINE_SHIFT: usize = 2; // The cache line is only 4 bytes right now.
        pub const K_LINE_LENGTH: usize = 1 << Self::K_LINE_SHIFT;
        pub const K_LINE_MASK: usize = Self::K_LINE_LENGTH - 1;

        const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;

        pub fn new() -> Self {
            CachePage {
                data_: [0; Self::K_PAGE_SIZE],
                validity_map_: [Self::LINE_INVALID as u8; Self::K_VALIDITY_MAP_SIZE],
            }
        }

        pub fn validity_byte(&mut self, offset: usize) -> &mut u8 {
            &mut self.validity_map_[offset >> Self::K_LINE_SHIFT]
        }

        pub fn cached_data(&mut self, offset: usize) -> &mut u8 {
            &mut self.data_[offset]
        }
    }

    /// Base class for MIPS64 instructions in the simulator.
    pub struct SimInstructionBase {
        operand_: i32,
        instr_: *mut Instr,
        type_: Type,
    }

    impl SimInstructionBase {
        pub fn new() -> Self {
            SimInstructionBase {
                operand_: -1,
                instr_: ptr::null_mut(),
                type_: Type::kUnsupported,
            }
        }

        pub fn new_with_instr(instr: *mut Instr) -> Self {
            SimInstructionBase {
                operand_: -1,
                instr_: instr,
                type_: Type::kUnsupported,
            }
        }

        pub fn instruction_type(&self) -> Type {
            self.type_
        }
        pub fn instr(&self) -> *mut Instr {
            self.instr_
        }
        pub fn operand(&self) -> i32 {
            self.operand_
        }
    }

    #[derive(Clone, Copy)]
    pub enum Type {
        kUnsupported,
        // Add other instruction types here
    }

    pub struct InstructionBase {}

    impl InstructionBase {
        pub fn instruction_type() -> Type {
            Type::kUnsupported
        }
    }

    pub trait InstructionGettersBase {
        fn rs_value(&self) -> i32;
        fn rt_value(&self) -> i32;
        fn rd_value(&self) -> i32;
        fn fr_value(&self) -> i32;
        fn fs_value(&self) -> i32;
        fn ft_value(&self) -> i32;
        fn fd_value(&self) -> i32;
        fn sa_value(&self) -> i32;
        fn lsa_sa_value(&self) -> i32;
        fn ws_value(&self) -> i32;
        fn wt_value(&self) -> i32;
        fn wd_value(&self) -> i32;
        fn opcode_value(&self) -> i32;
        fn instruction_bits(&self) -> i32;
        fn is_forbidden_after_branch(&self) -> bool;
    }
    pub trait InstructionGetters<T: InstructionGettersBase> {
        fn rs_value(&self) -> i32;
        fn rt_value(&self) -> i32;
        fn rd_value(&self) -> i32;
        fn fr_value(&self) -> i32;
        fn fs_value(&self) -> i32;
        fn ft_value(&self) -> i32;
        fn fd_value(&self) -> i32;
        fn sa_value(&self) -> i32;
        fn lsa_sa_value(&self) -> i32;
        fn ws_value(&self) -> i32;
        fn wt_value(&self) -> i32;
        fn wd_value(&self) -> i32;
        fn opcode_value(&self) -> i32;
        fn instruction_bits(&self) -> i32;
        fn is_forbidden_after_branch(&self) -> bool;
    }

    /// Represents a MIPS64 instruction in the simulator.
    pub struct SimInstruction {
        operand_: i32,
        instr_: *mut Instr,
        type_: Type,
    }

    impl SimInstruction {
        pub fn new() -> Self {
            SimInstruction {
                operand_: -1,
                instr_: ptr::null_mut(),
                type_: Type::kUnsupported,
            }
        }

        pub fn new_with_instr(instr: *mut Instr) -> Self {
            let mut sim_instr = SimInstruction::new();
            sim_instr.assign(instr);
            sim_instr
        }

        pub fn assign(&mut self, instr: *mut Instr) -> &mut Self {
            unsafe {
                self.operand_ = *(instr as *const i32);
            }
            self.instr_ = instr;
            self.type_ = InstructionBase::instruction_type();

            // The following assertion is not directly translatable without more
            // context on the layout and purpose of this class. It's a safety check
            // in C++ that ensures memory layout assumptions are correct.
            // DCHECK(reinterpret_cast<void*>(&operand_) == this);
            self
        }
    }

    impl InstructionGettersBase for SimInstruction {
        fn rs_value(&self) -> i32 {
            0
        }
        fn rt_value(&self) -> i32 {
            0
        }
        fn rd_value(&self) -> i32 {
            0
        }
        fn fr_value(&self) -> i32 {
            0
        }
        fn fs_value(&self) -> i32 {
            0
        }
        fn ft_value(&self) -> i32 {
            0
        }
        fn fd_value(&self) -> i32 {
            0
        }
        fn sa_value(&self) -> i32 {
            0
        }
        fn lsa_sa_value(&self) -> i32 {
            0
        }
        fn ws_value(&self) -> i32 {
            0
        }
        fn wt_value(&self) -> i32 {
            0
        }
        fn wd_value(&self) -> i32 {
            0
        }
        fn opcode_value(&self) -> i32 {
            0
        }
        fn instruction_bits(&self) -> i32 {
            0
        }
        fn is_forbidden_after_branch(&self) -> bool {
            false
        }
    }

    pub struct Simulator {
        registers_: [i64; Self::K_NUM_SIMU_REGISTERS],
        FPUregisters_: [i64; Self::K_NUM_FPUR_REGISTERS * 2],
        FCSR_: u32,
        MSACSR_: u32,
        stack_: usize,
        stack_limit_: usize,
        pc_modified_: bool,
        icount_: i64,
        break_count_: i32,
        trace_buf_: base::EmbeddedVector<char, 128>,
        last_debugger_input_: *mut char,
        isolate_: *mut Isolate,
        break_pc_: *mut Instr,
        break_instr_: Instr,
        watched_stops_: [StopCountAndDesc; kMaxStopCode + 1],
        local_monitor_: LocalMonitor,
        global_monitor_thread_: GlobalMonitor::LinkedAddress,
    }

    impl Simulator {
        // Registers are declared in order. See SMRL chapter 2.
        pub const NO_REG: i32 = -1;
        pub const ZERO_REG: usize = 0;
        pub const AT: usize = 1;
        pub const V0: usize = 2;
        pub const V1: usize = 3;
        pub const A0: usize = 4;
        pub const A1: usize = 5;
        pub const A2: usize = 6;
        pub const A3: usize = 7;
        pub const A4: usize = 8;
        pub const A5: usize = 9;
        pub const A6: usize = 10;
        pub const A7: usize = 11;
        pub const T0: usize = 12;
        pub const T1: usize = 13;
        pub const T2: usize = 14;
        pub const T3: usize = 15;
        pub const S0: usize = 16;
        pub const S1: usize = 17;
        pub const S2: usize = 18;
        pub const S3: usize = 19;
        pub const S4: usize = 20;
        pub const S5: usize = 21;
        pub const S6: usize = 22;
        pub const S7: usize = 23;
        pub const T8: usize = 24;
        pub const T9: usize = 25;
        pub const K0: usize = 26;
        pub const K1: usize = 27;
        pub const GP: usize = 28;
        pub const SP: usize = 29;
        pub const S8: usize = 30;
        pub const RA: usize = 31;
        // LO, HI, and pc.
        pub const LO: usize = 32;
        pub const HI: usize = 33;
        pub const PC: usize = 34; // pc must be the last register.
        pub const K_NUM_SIMU_REGISTERS: usize = 35;
        // aliases
        pub const FP: usize = Self::S8;

        // Coprocessor registers.
        // Generated code will always use doubles. So we will only use even registers.
        pub const F0: usize = 0;
        pub const F1: usize = 1;
        pub const F2: usize = 2;
        pub const F3: usize = 3;
        pub const F4: usize = 4;
        pub const F5: usize = 5;
        pub const F6: usize = 6;
        pub const F7: usize = 7;
        pub const F8: usize = 8;
        pub const F9: usize = 9;
        pub const F10: usize = 10;
        pub const F11: usize = 11;
        pub const F12: usize = 12;
        pub const F13: usize = 13;
        pub const F14: usize = 14;
        pub const F15: usize = 15; // f12 and f14 are arguments FPURegisters.
        pub const F16: usize = 16;
        pub const F17: usize = 17;
        pub const F18: usize = 18;
        pub const F19: usize = 19;
        pub const F20: usize = 20;
        pub const F21: usize = 21;
        pub const F22: usize = 22;
        pub const F23: usize = 23;
        pub const F24: usize = 24;
        pub const F25: usize = 25;
        pub const F26: usize = 26;
        pub const F27: usize = 27;
        pub const F28: usize = 28;
        pub const F29: usize = 29;
        pub const F30: usize = 30;
        pub const F31: usize = 31;
        pub const K_NUM_FPUR_REGISTERS: usize = 32;

        // MSA registers
        pub const W0: usize = 0;
        pub const W1: usize = 1;
        pub const W2: usize = 2;
        pub const W3: usize = 3;
        pub const W4: usize = 4;
        pub const W5: usize = 5;
        pub const W6: usize = 6;
        pub const W7: usize = 7;
        pub const W8: usize = 8;
        pub const W9: usize = 9;
        pub const W10: usize = 10;
        pub const W11: usize = 11;
        pub const W12: usize = 12;
        pub const W13: usize = 13;
        pub const W14: usize = 14;
        pub const W15: usize = 15;
        pub const W16: usize = 16;
        pub const W17: usize = 17;
        pub const W18: usize = 18;
        pub const W19: usize = 19;
        pub const W20: usize = 20;
        pub const W21: usize = 21;
        pub const W22: usize = 22;
        pub const W23: usize = 23;
        pub const W24: usize = 24;
        pub const W25: usize = 25;
        pub const W26: usize = 26;
        pub const W27: usize = 27;
        pub const W28: usize = 28;
        pub const W29: usize = 29;
        pub const W30: usize = 30;
        pub const W31: usize = 31;
        pub const K_NUM_MSAREGISTERS: usize = 32;

        const K_STACK_PROTECTION_SIZE: usize = KB;

        fn allocated_stack_size() -> usize {
            unsafe { (super::v8_flags.sim_stack_size * KB) + (2 * Self::K_STACK_PROTECTION_SIZE) }
        }
        fn usable_stack_size() -> usize {
            unsafe { super::v8_flags.sim_stack_size * KB }
        }

        const K_ADDITIONAL_STACK_MARGIN: usize = 4 * KB;

        const BAD_RA: i64 = -1;
        const END_SIM_PC: i64 = -2;
        const UNPREDICTABLE: i64 = 0xbadbeaf;

        pub fn new(isolate: *mut Isolate) -> Self {
            let stack_size = Self::allocated_stack_size();
            let stack = Self::allocate_stack(stack_size).expect("Failed to allocate stack");
            let stack_limit = stack + Self::K_STACK_PROTECTION_SIZE; // Base of stack
            let trace_buf = base::EmbeddedVector::new();

            Simulator {
                registers_: [0; Self::K_NUM_SIMU_REGISTERS],
                FPUregisters_: [0; Self::K_NUM_FPUR_REGISTERS * 2],
                FCSR_: 0,
                MSACSR_: 0,
                stack_: stack,
                stack_limit_: stack_limit,
                pc_modified_: false,
                icount_: 0,
                break_count_: 0,
                trace_buf_: trace_buf,
                last_debugger_input_: ptr::null_mut(),
                isolate_: isolate,
                break_pc_: ptr::null_mut(),
                break_instr_: 0,
                watched_stops_: [StopCountAndDesc { count: 0, desc: ptr::null_mut() }; kMaxStopCode + 1],
                local_monitor_: LocalMonitor::new(),
                global_monitor_thread_: GlobalMonitor::LinkedAddress::new(),
            }
        }

        fn allocate_stack(size: usize) -> Result<usize, std::io::Error> {
            use std::alloc::{alloc, Layout, dealloc, handle_alloc_error};
            use std::ptr::NonNull;

            let layout = Layout::array::<u8>(size).unwrap().align_to(16).unwrap();

            unsafe {
                let ptr = alloc(layout);
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                Ok(ptr as usize)
            }
        }

        fn deallocate_stack(&mut self) {
            use std::alloc::{alloc, Layout, dealloc, handle_alloc_error};
            use std::ptr::NonNull;

            let size = Self::allocated_stack_size();
            let layout = Layout::array::<u8>(size).unwrap().align_to(16).unwrap();

            unsafe {
                dealloc(self.stack_ as *mut u8, layout);
            }
        }
    }

    impl Drop for Simulator {
        fn drop(&mut self) {
            self.deallocate_stack();
        }
    }

    impl Simulator {
        // The currently executing Simulator instance. Potentially there can be one
        // for each native thread.
        pub fn current(isolate: *mut Isolate) -> *mut Simulator {
            // This part requires thread-local storage to be properly implemented
            // to ensure that each thread has its own simulator instance.
            // In this simple conversion, we'll just return a static mutable reference
            // which is inherently unsafe and not thread-safe.
            static mut SIMULATOR: *mut Simulator = ptr::null_mut();
            unsafe {
                if SIMULATOR.is_null() {
                    SIMULATOR = Box::into_raw(Box::new(Simulator::new(isolate)));
                }
                SIMULATOR
            }
        }

        // Accessors for register state. Reading the pc value adheres to the MIPS
        // architecture specification and is off by a 8 from the currently executing
        // instruction.
        pub fn set_register(&mut self, reg: usize, value: i64) {
            self.registers_[reg] = value;
        }
        pub fn set_register_word(&mut self, reg: usize, value: i32) {
            self.registers_[reg] = value as i64;
        }
        pub fn set_dw_register(&mut self, dreg: usize, dbl: &[i32]) {
            let value = ((dbl[1] as i64) << 32) | (dbl[0] as u32 as i64);
            self.set_register(dreg, value);
        }

        pub fn get_register(&self, reg: usize) -> i64 {
            self.registers_[reg]
        }

        pub fn get_double_from_register_pair(&self, reg: usize) -> f64 {
            f64::from_bits(self.get_register(reg) as u64)
        }

        // Same for FPURegisters.
        pub fn set_fpu_register(&mut self, fpureg: usize, value: i64) {
            self.FPUregisters_[fpureg] = value;
        }
        pub fn set_fpu_register_word(&mut self, fpureg: usize, value: i32) {
            self.FPUregisters_[fpureg] = value as i64;
        }
        pub fn set_fpu_register_hi_word(&mut self, fpureg: usize, value: i32) {
            self.FPUregisters_[fpureg] = (self.FPUregisters_[fpureg] & 0xFFFFFFFF) | ((value as i64) << 32);
        }
        pub fn set_fpu_register_float(&mut self, fpureg: usize, value: f32) {
            self.set_fpu_register(fpureg, (value.to_bits() as i64));
        }
        pub fn set_fpu_register_double(&mut self, fpureg: usize, value: f64) {
            self.set_fpu_register(fpureg, value.to_bits() as i64);
        }
        pub fn set_fpu_register_invalid_result64(&mut self, original: f32, rounded: f32) {
            // Implement set_fpu_register_invalid_result64 logic here
            println!("set_fpu_register_invalid_result64(f32, f32) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn set_fpu_register_invalid_result(&mut self, original: f32, rounded: f32) {
            // Implement set_fpu_register_invalid_result logic here
            println!("set_fpu_register_invalid_result(f32, f32) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn set_fpu_register_word_invalid_result(&mut self, original: f32, rounded: f32) {
            // Implement set_fpu_register_word_invalid_result logic here
            println!("set_fpu_register_word_invalid_result(f32, f32) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn set_fpu_register_invalid_result64_double(&mut self, original: f64, rounded: f64) {
             // Implement set_fpu_register_invalid_result64 logic here
            println!("set_fpu_register_invalid_result64(f64, f64) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn set_fpu_register_invalid_result_double(&mut self, original: f64, rounded: f64) {
            // Implement set_fpu_register_invalid_result logic here
            println!("set_fpu_register_invalid_result(f64, f64) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn set_fpu_register_word_invalid_result_double(&mut self, original: f64, rounded: f64) {
            // Implement set_fpu_register_word_invalid_result logic here
            println!("set_fpu_register_word_invalid_result(f64, f64) is unimplemented. Args: {} {}", original, rounded);
        }
        pub fn get_fpu_register(&self, fpureg: usize) -> i64 {
            self.FPUregisters_[fpureg]
        }
        pub fn get_fpu_register_word(&self, fpureg: usize) -> i32 {
            self.FPUregisters_[fpureg] as i32
        }
        pub fn get_fpu_register_signed_word(&self, fpureg: usize) -> i32 {
            self.FPUregisters_[fpureg] as i32
        }
        pub fn get_fpu_register_hi_word(&self, fpureg: usize) -> i32 {
            (self.FPUregisters_[fpureg] >> 32) as i32
        }
        pub fn get_fpu_register_float(&self, fpureg: usize) -> f32 {
            f32::from_bits(self.FPUregisters_[fpureg] as u32)
        }
        pub fn get_fpu_register_double(&self, fpureg: usize) -> f64 {
            f64::from_bits(self.FPUregisters_[fpureg] as u64)
        }

        pub fn get_msa_register<T>(&self, wreg: usize, value: &mut T) {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.FPUregisters_.as_ptr().add(wreg) as *const i64,
                    value as *mut T as *mut i64,
                    1,
                );
            }
        }

        pub fn set_msa_register<T>(&mut self, wreg: usize, value: &T) {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    value as *const T as *const i64,
                    self.FPUregisters_.as_mut_ptr().add(wreg) as *mut i64,
                    1,
                );
            }
        }

        pub fn set_fcsr_bit(&mut self, cc: u32, value: bool) {
            if value {
                self.FCSR_ |= (1 << cc);
            } else {
                self.FCSR_ &= !(1 << cc);
            }
        }

        pub fn test_fcsr_bit(&self, cc: u32) -> bool {
            (self.FCSR_ & (1 << cc)) != 0
        }

        pub fn set_fcsr_round_error(&mut self, original: f64, rounded: f64) -> bool {
            println!("set_fcsr_round_error(f64, f64) unimplemented. Args: {} {}", original, rounded);
            false
        }

        pub fn set_fcsr_round64_error(&mut self, original: f64, rounded: f64) -> bool {
            println!("set_fcsr_round64_error(f64, f64) unimplemented. Args: {} {}", original, rounded);
            false
        }

        pub fn set_fcsr_round_error_float(&mut self, original: f32, rounded: f32) -> bool {
            println!("set_fcsr_round_error(f32, f32) unimplemented. Args: {} {}", original, rounded);
            false
        }

        pub fn set_fcsr_round64_error_float(&mut self, original: f32, rounded: f32) -> bool {
            println!("set_fcsr_round64_error(f32, f32) unimplemented. Args: {} {}", original, rounded);
            false
        }

        pub fn round_according_to_fcsr(&mut self, to_round: f64, rounded: &mut f64, rounded_int: &mut i32, fs: f64) {
            println!("round_according_to_fcsr(f64, f64, i32, f64) unimplemented. Args: {} {} {} {}", to_round, rounded, rounded_int, fs);
        }

        pub fn round64_according_to_fcsr(&mut self, to_round: f64, rounded: &mut f64, rounded_int: &mut i64, fs: f64) {
            println!("round64_according_to_fcsr(f64, f64, i64, f64) unimplemented. Args: {} {} {} {}", to_round, rounded, rounded_int, fs);
        }

        pub fn round_according_to_fcsr_float(&mut self, to_round: f32, rounded: &mut f32, rounded_int: &mut i32, fs: f32) {
            println!("round_according_to_fcsr(f32, f32, i32, f32) unimplemented. Args: {} {} {} {}", to_round, rounded, rounded_int, fs);
        }

        pub fn round64_according_to_fcsr_float(&mut self, to_round: f32, rounded: &mut f32, rounded_int: &mut i64, fs: f32) {
            println!("round64_according_to_fcsr(f32, f32, i64, f32) unimplemented. Args: {} {} {} {}", to_round, rounded, rounded_int, fs);
        }

        pub fn round_according_to_msacsr<T_fp, T_int>(&mut self, to_round: T_fp, rounded: &mut T_fp, rounded_int: &mut T_int) {
            println!("round_according_to_msacsr<T_fp, T_int> unimplemented. Args: {} {} {}", to_round, rounded, rounded_int);
        }

        pub fn clear_fcsr_cause(&mut self) {
            self.FCSR_ = 0;
        }

        pub fn set_fcsr_rounding_mode(&mut self, mode: FPURoundingMode) {
            println!("set_fcsr_rounding_mode unimplemented. Arg: {:?}", mode);
        }

        pub fn set_msacsr_rounding_mode(&mut self, mode: FPURoundingMode) {
            println!("set_msacsr_rounding_mode unimplemented. Arg: {:?}", mode);
        }

        pub fn get_fcsr_rounding_mode(&self) -> u32 {
            println!("get_fcsr_rounding_mode unimplemented");
            0
        }

        pub fn get_msacsr_rounding_mode(&self) -> u32 {
            println!("get_msacsr_rounding_mode unimplemented");
            0
        }
        // Special case of set_register and get_register to access the raw PC value.
        pub fn set_pc(&mut self, value: i64) {
            self.set_register(Self::PC, value);
        }
        pub fn get_pc(&self) -> i64 {
