// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Declares a Simulator for PPC instructions if we are not generating a native
// PPC binary. This Simulator allows us to run and debug PPC code generation on
// regular desktop machines.
// V8 calls into generated code via the GeneratedCode wrapper,
// which will start execution in the Simulator or forwards to the real entry
// on a PPC HW platform.

// globals.h defines USE_SIMULATOR.

#[cfg(feature = "simulator")]
pub mod simulator_ppc {
    use std::any::Any;
    use std::mem::size_of;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use crate::codegen::assembler::Assembler;
    use crate::codegen::ppc::constants_ppc::*;
    use crate::execution::simulator_base::*;
    use crate::utils::allocation::*;

    // use heap::base::StackVisitor; // Assuming StackVisitor is defined elsewhere and accessible
    // use v8::internal::Isolate; // Assuming Isolate is defined elsewhere and accessible

    const KB: usize = 1024;

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

        const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Register {
        no_reg = -1,
        r0 = 0,
        sp,
        r2,
        r3,
        r4,
        r5,
        r6,
        r7,
        r8,
        r9,
        r10,
        r11,
        r12,
        r13,
        r14,
        r15,
        r16,
        r17,
        r18,
        r19,
        r20,
        r21,
        r22,
        r23,
        r24,
        r25,
        r26,
        r27,
        r28,
        r29,
        r30,
        fp,
        kNumGPRs = 32,
        d0 = 0,
        d1,
        d2,
        d3,
        d4,
        d5,
        d6,
        d7,
        d8,
        d9,
        d10,
        d11,
        d12,
        d13,
        d14,
        d15,
        d16,
        d17,
        d18,
        d19,
        d20,
        d21,
        d22,
        d23,
        d24,
        d25,
        d26,
        d27,
        d28,
        d29,
        d30,
        d31,
        kNumFPRs = 32,
        // PPC Simd registers are a serapre set from Floating Point registers. Refer
        // to register-ppc.h for more details.
        v0 = 0,
        v1,
        v2,
        v3,
        v4,
        v5,
        v6,
        v7,
        v8,
        v9,
        v10,
        v11,
        v12,
        v13,
        v14,
        v15,
        v16,
        v17,
        v18,
        v19,
        v20,
        v21,
        v22,
        v23,
        v24,
        v25,
        v26,
        v27,
        v28,
        v29,
        v30,
        v31,
        kNumSIMDRs = 32,
    }

    #[derive(Default)]
    pub struct Simulator {
        // Architecture state.
        // Saturating instructions require a Q flag to indicate saturation.
        // There is currently no way to read the CPSR directly, and thus read the Q
        // flag, so this is left unimplemented.
        registers_: [i64; Register::kNumGPRs as usize],
        condition_reg_: i32,
        fp_condition_reg_: i32,
        special_reg_lr_: i64,
        special_reg_pc_: i64,
        special_reg_ctr_: i64,
        special_reg_xer_: i32,

        fp_registers_: [i64; Register::kNumFPRs as usize],

        // Simd registers.
        simd_registers_: [simdr_t; Register::kNumSIMDRs as usize],

        // Simulator support for the stack.
        stack_: Vec<u8>,
        pc_modified_: bool,
        icount_: i32,

        // Debugger input.
        last_debugger_input_: String, // Using String instead of char*

        // Registered breakpoints.
        break_pc_: i64, // Assuming Instruction* can be represented as an address
        break_instr_: Instr,

        isolate_: Option<Box<dyn Any>>, // Replace Isolate* with a dynamic type.  Need to provide concrete type

        watched_stops_: [StopCountAndDesc; Self::K_NUM_OF_WATCHED_STOPS as usize],

        // // Synchronization primitives. See ARM DDI 0406C.b, A2.9.
        // access_state_: MonitorAccess,
        // tagged_addr_: usize,
        // size_: TransactionSize,
        // thread_id_: ThreadId,
    }

    impl Simulator {
        pub const K_STACK_PROTECTION_SIZE: usize = 256 * std::mem::size_of::<usize>();

        pub const K_NUM_OF_WATCHED_STOPS: u32 = 256;

        // Breakpoint is disabled if bit 31 is set.
        pub const K_STOP_DISABLED_BIT: u32 = 1 << 31;

        pub fn new(isolate: Option<Box<dyn Any>>) -> Self {
            let stack_size = v8_flags::get_sim_stack_size() * KB;
            let allocated_stack_size = stack_size + (2 * Self::K_STACK_PROTECTION_SIZE);
            let stack_ = vec![0u8; allocated_stack_size];
            Simulator {
                stack_,
                isolate_: isolate,
                last_debugger_input_: String::new(),
                break_pc_: 0,
                break_instr_: 0, //default
                watched_stops_: [StopCountAndDesc { count: 0, desc: String::new() }; Self::K_NUM_OF_WATCHED_STOPS as usize],
                ..Default::default()
            }
        }

        pub fn current(/*isolate: &Isolate*/) -> &'static Mutex<Simulator> {
            // Placeholder - in V8 it fetched simulator per isolate.  Here, making a global
            static SIMULATOR: OnceLock<Mutex<Simulator>> = OnceLock::new();
            SIMULATOR.get_or_init(|| Mutex::new(Simulator::new(None)))
        }

        pub fn set_register(&mut self, reg: Register, value: i64) {
            self.registers_[reg as usize] = value;
        }

        pub fn get_register(&self, reg: Register) -> i64 {
            self.registers_[reg as usize]
        }

        pub fn get_double_from_register_pair(&self, _reg: i32) -> f64 {
            todo!()
            // Placeholder for double register access logic
        }

        pub fn set_d_register_from_double(&mut self, dreg: i32, dbl: f64) {
            assert!((dreg as usize) < (Register::kNumFPRs as usize));
            self.fp_registers_[dreg as usize] = f64::to_bits(dbl) as i64;
        }

        pub fn get_double_from_d_register(&self, dreg: i32) -> f64 {
            assert!((dreg as usize) < (Register::kNumFPRs as usize));
            f64::from_bits(self.fp_registers_[dreg as usize] as u64)
        }

        pub fn set_d_register(&mut self, dreg: i32, value: i64) {
            assert!((dreg as usize) < (Register::kNumFPRs as usize));
            self.fp_registers_[dreg as usize] = value;
        }

        pub fn get_d_register(&self, dreg: i32) -> i64 {
            assert!((dreg as usize) < (Register::kNumFPRs as usize));
            self.fp_registers_[dreg as usize]
        }

        pub fn set_pc(&mut self, value: i64) {
            self.special_reg_pc_ = value;
        }

        pub fn get_pc(&self) -> i64 {
            self.special_reg_pc_
        }

        pub fn get_sp(&self) -> usize {
            self.get_register(Register::sp) as usize
        }

        pub fn get_lr(&self) -> i64 {
            self.special_reg_lr_
        }

        pub fn stack_limit(&self, c_limit: usize) -> usize {
            std::cmp::min(c_limit, self.stack_base() + Self::K_STACK_PROTECTION_SIZE)
        }

        pub fn stack_base(&self) -> usize {
            let stack_size = v8_flags::get_sim_stack_size() * KB;
            let allocated_stack_size = stack_size + (2 * Self::K_STACK_PROTECTION_SIZE);
            let stack_base = self.stack_.as_ptr() as usize;
            stack_base + Self::K_STACK_PROTECTION_SIZE
        }

        pub fn allocated_stack_size() -> usize {
            let stack_size = v8_flags::get_sim_stack_size() * KB;
            stack_size + (2 * Self::K_STACK_PROTECTION_SIZE)
        }

        pub fn usable_stack_size() -> usize {
            Self::allocated_stack_size() - Self::K_STACK_PROTECTION_SIZE
        }

        pub fn get_central_stack_view(&self) -> &[u8] {
            &self.stack_[Self::K_STACK_PROTECTION_SIZE..(Self::stack_.len() - Self::K_STACK_PROTECTION_SIZE)]
        }

        // pub fn iterate_registers_and_stack(&self, visitor: &mut StackVisitor) {
        //     todo!()
        //     // Placeholder for stack iteration
        // }

        pub fn execute(&mut self) {
            todo!()
            // Placeholder for execution logic
        }

        pub fn call<Return, Args>(&mut self, entry: usize, args: Args) -> Return {
            todo!()
            // Placeholder for call logic
        }

        pub fn call_fp(&mut self, _entry: usize, _d0: f64, _d1: f64) {
            todo!()
            // Placeholder for FP call logic
        }
        pub fn call_fp_returns_int(&mut self, _entry: usize, _d0: f64, _d1: f64) -> i32 {
            todo!()
        }
        pub fn call_fp_returns_double(&mut self, _entry: usize, _d0: f64, _d1: f64) -> f64 {
            todo!()
        }

        pub fn push_address(&mut self, address: usize) -> usize {
            todo!()
            // Placeholder for stack push logic
        }

        pub fn pop_address(&mut self) -> usize {
            todo!()
            // Placeholder for stack pop logic
        }

        pub fn set_last_debugger_input(&mut self, input: &str) {
            self.last_debugger_input_ = input.to_string();
        }

        pub fn last_debugger_input(&self) -> &str {
            &self.last_debugger_input_
        }

        pub fn set_redirect_instruction(_instruction: *mut Instruction) {
            todo!()
            // Placeholder for redirection
        }

        pub fn icache_match(_one: *mut std::ffi::c_void, _two: *mut std::ffi::c_void) -> bool {
            todo!()
            // Placeholder for ICache matching
        }

        pub fn flush_i_cache(_i_cache: *mut std::ffi::c_void, _start: *mut std::ffi::c_void, _size: usize) {
            todo!()
            // Placeholder for ICache flushing
        }

        pub fn has_bad_pc(&self) -> bool {
            self.special_reg_pc_ == Self::BAD_LR || self.special_reg_pc_ == Self::END_SIM_PC
        }

        pub fn instruction_tracing_enabled(&self) -> bool {
            v8_flags::get_trace_sim() //use v8_flags accessor
        }

        pub fn toggle_instruction_tracing(&mut self) {
            v8_flags::toggle_trace_sim()
        }

        pub const BAD_LR: i64 = -1;
        pub const END_SIM_PC: i64 = -2;

        pub fn call_impl(&mut self, _entry: usize, _argument_count: i32, _arguments: &[i64]) -> i64 {
            todo!()
            // Placeholder for call implementation
        }

        pub fn format(&mut self, _instr: *mut Instruction, _format: &str) {
            todo!()
            // Placeholder for format implementation
        }

        pub fn carry_from(&self, left: i32, right: i32, carry: i32) -> bool {
            ((left as i64 + right as i64 + carry as i64) & (1 << 32)) != 0
        }

        pub fn borrow_from(&self, left: i32, right: i32) -> bool {
            (left as i64) < (right as i64)
        }

        pub fn overflow_from(&self, alu_out: i32, left: i32, right: i32, addition: bool) -> bool {
            if addition {
                ((!(left ^ right)) & (left ^ alu_out) & (1 << 31)) != 0
            } else {
                (((left ^ right)) & (left ^ alu_out) & (1 << 31)) != 0
            }
        }

        pub fn get_shift_rm(&mut self, _instr: *mut Instruction, _carry_out: &mut bool) -> i32 {
            todo!()
            // Placeholder for shift implementation
        }

        pub fn get_imm(&mut self, _instr: *mut Instruction, _carry_out: &mut bool) -> i32 {
            todo!()
            // Placeholder for immediate implementation
        }

        pub fn process_puw(&mut self, _instr: *mut Instruction, _num_regs: i32, _operand_size: i32, _start_address: *mut i64, _end_address: *mut i64) {
            todo!()
            // Placeholder for PUW implementation
        }

        pub fn handle_r_list(&mut self, _instr: *mut Instruction, _load: bool) {
            todo!()
            // Placeholder for RList implementation
        }

        pub fn handle_v_list(&mut self, _inst: *mut Instruction) {
            todo!()
        }
        pub fn software_interrupt(&mut self, _instr: *mut Instruction) {
            todo!()
        }

        pub fn debug_at_next_pc(&mut self) {
            todo!()
            // Placeholder for debug implementation
        }

        pub fn is_stop_instruction(&self, _instr: *mut Instruction) -> bool {
            todo!()
            // Placeholder for stop instruction check
        }

        pub fn is_watched_stop(&self, _bkpt_code: u32) -> bool {
            todo!()
            // Placeholder for watched stop check
        }

        pub fn is_enabled_stop(&self, bkpt_code: u32) -> bool {
            (self.watched_stops_[bkpt_code as usize].count & Self::K_STOP_DISABLED_BIT) == 0
        }

        pub fn enable_stop(&mut self, bkpt_code: u32) {
            self.watched_stops_[bkpt_code as usize].count &= !Self::K_STOP_DISABLED_BIT;
        }

        pub fn disable_stop(&mut self, bkpt_code: u32) {
            self.watched_stops_[bkpt_code as usize].count |= Self::K_STOP_DISABLED_BIT;
        }

        pub fn increase_stop_counter(&mut self, bkpt_code: u32) {
            self.watched_stops_[bkpt_code as usize].count = self.watched_stops_[bkpt_code as usize].count.wrapping_add(1);
        }

        pub fn print_stop_info(&mut self, _code: u32) {
            todo!()
            // Placeholder for print stop info
        }

        pub fn read<T: Sized>(&self, address: usize, value: &mut T) {
            let lock_guard = GlobalMonitor::get().mutex.lock().unwrap();
            unsafe {
                std::ptr::copy_nonoverlapping(address as *const u8, value as *mut T as *mut u8, size_of::<T>());
            }
        }

        pub fn read_ex<T: Sized>(&self, address: usize, value: &mut T) {
            let lock_guard = GlobalMonitor::get().mutex.lock().unwrap();
            if let Some(isolate) = &self.isolate_ {
                GlobalMonitor::get().notify_load_excl(
                    address as _,
                    Self::transaction_size_from_type::<T>() as _,
                    0, //isolate.thread_id()
                );
            }
            unsafe {
                std::ptr::copy_nonoverlapping(address as *const u8, value as *mut T as *mut u8, size_of::<T>());
            }
        }

        pub fn write<T: Sized>(&mut self, address: usize, value: T) {
            let lock_guard = GlobalMonitor::get().mutex.lock().unwrap();
            if let Some(isolate) = &self.isolate_ {
                GlobalMonitor::get().notify_store(
                    address as _,
                    Self::transaction_size_from_type::<T>() as _,
                    0, //isolate.thread_id()
                );
            }
            unsafe {
                std::ptr::copy_nonoverlapping(&value as *const T as *const u8, address as *mut u8, size_of::<T>());
            }
        }

        pub fn write_ex<T: Sized>(&mut self, address: usize, value: T) -> i32 {
            let lock_guard = GlobalMonitor::get().mutex.lock().unwrap();
            if let Some(isolate) = &self.isolate_ {
                if GlobalMonitor::get().notify_store_excl(
                    address as _,
                    Self::transaction_size_from_type::<T>() as _,
                    0, //isolate.thread_id()
                ) {
                    unsafe {
                        std::ptr::copy_nonoverlapping(&value as *const T as *const u8, address as *mut u8, size_of::<T>());
                    }
                    return 0;
                } else {
                    return 1;
                }
            }
            1
        }

        fn transaction_size_from_type<T>() -> MonitorAccess {
            match size_of::<T>() {
                1 => MonitorAccess::Open,
                2 => MonitorAccess::Open,
                4 => MonitorAccess::Open,
                8 => MonitorAccess::Open,
                _ => MonitorAccess::Open,
            }
        }

        pub fn trace(&mut self, _instr: *mut Instruction) {
            todo!()
            // Placeholder for tracing implementation
        }

        pub fn set_cr0(&mut self, _result: i64, _set_so: bool) {
            todo!()
            // Placeholder for CR0 setting
        }

        pub fn set_cr6(&mut self, _true_for_all: bool) {
            todo!()
            // Placeholder for CR6 setting
        }

        pub fn execute_branch_conditional(&mut self, _instr: *mut Instruction, _type: BCType) {
            todo!()
            // Placeholder for branch execution
        }

        pub fn execute_generic(&mut self, _instr: *mut Instruction) {
            todo!()
            // Placeholder for generic execution
        }

        pub fn set_fpscr(&mut self, bit: i32) {
            self.fp_condition_reg_ |= (1 << (31 - bit));
        }

        pub fn clear_fpscr(&mut self, bit: i32) {
            self.fp_condition_reg_ &= !(1 << (31 - bit));
        }

        pub fn execute_instruction(&mut self, _instr: *mut Instruction) {
            todo!()
            // Placeholder for instruction execution
        }

        pub fn check_i_cache(_i_cache: *mut std::ffi::c_void, _instr: *mut Instruction) {
            todo!()
            // Placeholder for ICache check
        }

        pub fn flush_one_page(_i_cache: *mut std::ffi::c_void, _start: i64, _size: usize) {
            todo!()
            // Placeholder for one page flush
        }

        pub fn get_cache_page(_i_cache: *mut std::ffi::c_void, _page: *mut std::ffi::c_void) -> *mut CachePage {
            todo!()
            // Placeholder for cache page retrieval
        }

        pub fn get_fp_args(&mut self, _x: *mut f64, _y: *mut f64, _z: *mut i64) {
            todo!()
            // Placeholder for FP arguments
        }

        pub fn set_fp_result(&mut self, _result: &f64) {
            todo!()
            // Placeholder for FP result
        }

        pub fn trash_caller_save_registers(&mut self) {
            todo!()
            // Placeholder for trashing registers
        }

        pub fn call_internal(&mut self, _entry: usize) {
            todo!()
            // Placeholder for internal call
        }

        pub fn get_simd_register_by_lane<T>(&self, reg: i32, lane: i32, force_ibm_lane_numbering: bool) -> T
            where
                T: Sized + Copy,
        {
            let mut lane = lane;
            if force_ibm_lane_numbering {
                lane = ((size_of::<[u8; 16]>() / size_of::<T>()) as i32) - 1 - lane;
            }
            assert!(lane <= ((size_of::<[u8; 16]>() / size_of::<T>()) as i32));
            assert!((reg as usize) < (Register::kNumSIMDRs as usize));
            assert!(lane >= 0);
            assert!(reg >= 0);
            let simd_reg = &self.simd_registers_[reg as usize];
            unsafe {
                let ptr = &simd_reg.int8 as *const i8 as *const T;
                *ptr.add(lane as usize)
            }
        }

        pub fn get_simd_register_bytes<T>(&self, reg: i32, byte_from: i32) -> T
            where
                T: Sized + Copy,
        {
            let from = size_of::<[u8; 16]>() as i32 - 1 - (byte_from + size_of::<T>() as i32 - 1);
            let src = (self.simd_registers_[reg as usize].uint8.as_ptr() as *const u8).wrapping_add(from as usize);
            unsafe {
                let mut dst: T = std::mem::zeroed();
                std::ptr::copy_nonoverlapping(src, &mut dst as *mut T as *mut u8, size_of::<T>());
                dst
            }
        }

        pub fn set_simd_register_by_lane<T>(&mut self, reg: i32, lane: i32, value: T, force_ibm_lane_numbering: bool)
            where
                T: Sized + Copy,
        {
            let mut lane = lane;
            if force_ibm_lane_numbering {
                lane = ((size_of::<[u8; 16]>() / size_of::<T>()) as i32) - 1 - lane;
            }
            assert!(lane <= ((size_of::<[u8; 16]>() / size_of::<T>()) as i32));
            assert!((reg as usize) < (Register::kNumSIMDRs as usize));
            assert!(lane >= 0);
            assert!(reg >= 0);
            let simd_reg = &mut self.simd_registers_[reg as usize];
            unsafe {
                let ptr = &mut simd_reg.int8 as *mut i8 as *mut T;
                *ptr.add(lane as usize) = value;
            }
        }

        pub fn set_simd_register_bytes<T>(&mut self, reg: i32, byte_from: i32, value: T)
            where
                T: Sized + Copy,
        {
            let from = size_of::<[u8; 16]>() as i32 - 1 - (byte_from + size_of::<T>() as i32 - 1);
            let dst = (self.simd_registers_[reg as usize].uint8.as_mut_ptr()).wrapping_add(from as usize);
            unsafe {
                std::ptr::copy_nonoverlapping(&value as *const T as *const u8, dst, size_of::<T>());
            }
        }

        pub fn get_simd_register(&mut self, reg: i32) -> &mut simdr_t {
            &mut self.simd_registers_[reg as usize]
        }

        pub fn set_simd_register(&mut self, reg: i32, value: simdr_t) {
            self.simd_registers_[reg as usize] = value;
        }

        pub fn byte_reverse128(v: u128) -> u128 {
            let val = v.to_be_bytes();
            u128::from_be_bytes(val)
        }

        pub fn read_quw(&self, addr: usize) -> u128 {
            let mut value: u128 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_quw(&self, addr: usize) -> u128 {
            let mut value: u128 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_quw(&mut self, addr: usize, value: u128) {
            self.write(addr, value);
        }
        pub fn write_ex_quw(&mut self, addr: usize, value: u128) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_qw(&self, addr: usize) -> i128 {
            let mut value: i128 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_qw(&self, addr: usize) -> i128 {
            let mut value: i128 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_qw(&mut self, addr: usize, value: i128) {
            self.write(addr, value);
        }
        pub fn write_ex_qw(&mut self, addr: usize, value: i128) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_dwu(&self, addr: usize) -> u64 {
            let mut value: u64 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_dwu(&self, addr: usize) -> u64 {
            let mut value: u64 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_dwu(&mut self, addr: usize, value: u64) {
            self.write(addr, value);
        }
        pub fn write_ex_dwu(&mut self, addr: usize, value: u64) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_dw(&self, addr: usize) -> i64 {
            let mut value: i64 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_dw(&self, addr: usize) -> i64 {
            let mut value: i64 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_dw(&mut self, addr: usize, value: i64) {
            self.write(addr, value);
        }
        pub fn write_ex_dw(&mut self, addr: usize, value: i64) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_wu(&self, addr: usize) -> u32 {
            let mut value: u32 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_wu(&self, addr: usize) -> u32 {
            let mut value: u32 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_wu(&mut self, addr: usize, value: u32) {
            self.write(addr, value);
        }
        pub fn write_ex_wu(&mut self, addr: usize, value: u32) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_w(&self, addr: usize) -> i32 {
            let mut value: i32 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_w(&self, addr: usize) -> i32 {
            let mut value: i32 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_w(&mut self, addr: usize, value: i32) {
            self.write(addr, value);
        }
        pub fn write_ex_w(&mut self, addr: usize, value: i32) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_hu(&self, addr: usize) -> u16 {
            let mut value: u16 = 0;
            self.read(addr, &mut value);
            value
        }
        pub fn read_ex_hu(&self, addr: usize) -> u16 {
            let mut value: u16 = 0;
            self.read_ex(addr, &mut value);
            value
        }
        pub fn write_hu(&mut self, addr: usize, value: u16) {
            self.write(addr, value);
        }
        pub fn write_ex_hu(&mut self, addr: usize, value: u16) -> i32 {
            self.write_ex(addr, value);
        }

        pub fn read_h(&self, addr: usize) -> i16 {
            let mut