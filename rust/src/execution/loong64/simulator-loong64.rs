// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Declares a Simulator for loongisa instructions if we are not generating a
// native loongisa binary. This Simulator allows us to run and debug loongisa
// code generation on regular desktop machines. V8 calls into generated code via
// the GeneratedCode wrapper, which will start execution in the Simulator or
// forwards to the real entry on a loongisa HW platform.

// globals.h defines USE_SIMULATOR.

// use std::cmp::Ordering;
// use std::mem;
// use std::ptr;

// use crate::base::hashmap::CustomMatcherHashMap;
// use crate::base::strings::EmbeddedVector;
// use crate::codegen::assembler::Assembler;
// use crate::codegen::loong64::constants_loong64::*;
// use crate::execution::simulator_base::SimulatorBase;
// use crate::utils::allocation::Allocate;
// use crate::common::globals::USE_SIMULATOR; // Assuming USE_SIMULATOR is handled elsewhere

#[inline]
fn compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
    if a == b {
        0
    } else if a < b {
        -1
    } else {
        1
    }
}

// Returns the negative absolute value of its argument.
fn nabs<T: std::ops::Neg<Output = T> + std::cmp::PartialOrd + Copy>(a: T) -> T {
    if a < a.neg() {
        a
    } else {
        a.neg()
    }
}

#[cfg(feature = "simulator")]
pub mod simulator {
    use std::sync::Mutex;

    // use crate::base::hashmap::CustomMatcherHashMap;
    // use crate::base::strings::EmbeddedVector;
    // use crate::codegen::assembler::Assembler;
    // use crate::codegen::loong64::constants_loong64::*;
    // use crate::execution::simulator_base::SimulatorBase;
    // use crate::utils::allocation::Allocate;
    // use crate::v8::internal::{Address, Isolate, Instr};
    // use crate::v8::internal::codegen::InstructionGetters;

    const KB: usize = 1024; // Define KB here or import from a common module

    /// Utility functions
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

        pub const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;

        pub fn new() -> Self {
            CachePage {
                data_: [0u8; Self::K_PAGE_SIZE],
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

    // Assuming InstructionBase, Instruction, and InstructionGetters are defined elsewhere
    // and their Rust equivalents exist.  Placeholders are used here.
    pub struct InstructionBase {
        // Placeholders for fields and methods
    }

    impl InstructionBase {
        pub fn instruction_type(&self) -> Type {
            Type::Unsupported
        }
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Type {
        Unsupported
    }

    pub struct SimInstructionBase {
        operand_: i32,
        instr_: *mut Instruction, // Placeholder: Replace with appropriate Rust type
        type_: Type,
    }

    impl SimInstructionBase {
        pub fn new() -> Self {
            SimInstructionBase {
                operand_: -1,
                instr_: std::ptr::null_mut(),
                type_: Type::Unsupported,
            }
        }
        pub fn instruction_type(&self) -> Type {
            self.type_
        }

        pub fn instr(&self) -> *mut Instruction {
            self.instr_
        }

        pub fn operand(&self) -> i32 {
            self.operand_
        }
    }

    pub struct SimInstruction {
        base: SimInstructionBase,
    }
    impl SimInstruction {
        pub fn new() -> Self {
            SimInstruction {
                base: SimInstructionBase::new(),
            }
        }
        pub fn instr(&self) -> *mut Instruction {
            self.base.instr_
        }

        pub fn operand(&self) -> i32 {
            self.base.operand_
        }
    }

    impl From<*mut Instruction> for SimInstruction {
        fn from(instr: *mut Instruction) -> Self {
             let operand_ = unsafe { *(instr as *const i32) };
            SimInstruction {
                base: SimInstructionBase {
                    operand_: operand_,
                    instr_: instr,
                    type_: Type::Unsupported, //Placeholder InstructionBase::InstructionType(),
                },
            }
        }
    }

    impl std::ops::Deref for SimInstruction {
        type Target = SimInstructionBase;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl std::ops::DerefMut for SimInstruction {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    //Instruction placeholder struct
    pub struct Instruction {}

    pub struct Simulator {
        registers_: [i64; Self::K_NUM_SIMU_REGISTERS],
        f_pu_registers_: [i64; Self::K_NUM_FPU_REGISTERS],
        c_f_registers_: [bool; Self::K_NUM_CF_REGISTERS],
        fcsr_: u32,
        stack_: usize,
        stack_limit_: usize,
        pc_modified_: bool,
        icount_: i64,
        break_count_: i32,
        // trace_buf_: EmbeddedVector<char, 128>, // Placeholder
        last_debugger_input_: *mut i8, // Placeholder: Replace with appropriate Rust type
        // isolate_: *mut Isolate, // Placeholder: Replace with appropriate Rust type
        break_pc_: *mut Instruction, // Placeholder: Replace with appropriate Rust type
        break_instr_: Instr,    // Placeholder: Replace with appropriate Rust type
        watched_stops_: [StopCountAndDesc; Self::K_MAX_STOP_CODE_PLUS_ONE], // Placeholder
        local_monitor_: LocalMonitor,
        global_monitor_thread_: GlobalMonitor::LinkedAddress,

    }

    impl Simulator {
        pub const K_NUM_SIMU_REGISTERS: usize = 36; // pc is last register
        pub const K_NUM_CF_REGISTERS: usize = 8;
        pub const K_NUM_FPU_REGISTERS: usize = 32;
        const K_STACK_PROTECTION_SIZE: usize = KB;
        const K_ADDITIONAL_STACK_MARGIN: usize = 4 * KB;
        const K_MAX_STOP_CODE: usize = 1024;
        const K_MAX_STOP_CODE_PLUS_ONE: usize = Self::K_MAX_STOP_CODE + 1;

        pub const ZERO_REG: usize = 0;
        pub const RA: usize = 1;
        pub const GP: usize = 2;
        pub const SP: usize = 3;
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
        pub const T4: usize = 16;
        pub const T5: usize = 17;
        pub const T6: usize = 18;
        pub const T7: usize = 19;
        pub const T8: usize = 20;
        pub const TP: usize = 21;
        pub const FP: usize = 22;
        pub const S0: usize = 23;
        pub const S1: usize = 24;
        pub const S2: usize = 25;
        pub const S3: usize = 26;
        pub const S4: usize = 27;
        pub const S5: usize = 28;
        pub const S6: usize = 29;
        pub const S7: usize = 30;
        pub const S8: usize = 31;
        pub const PC: usize = 32; // pc must be the last register.
        // aliases
        pub const V0: usize = Self::A0;
        pub const V1: usize = Self::A1;

        pub fn new() -> Self {
            let stack_size = Self::allocated_stack_size();
            let mut stack = vec![0u8; stack_size];
            let stack_ptr = stack.as_mut_ptr() as usize;
            let stack_limit = stack_ptr + Self::K_STACK_PROTECTION_SIZE;

            Simulator {
                registers_: [0; Self::K_NUM_SIMU_REGISTERS],
                f_pu_registers_: [0; Self::K_NUM_FPU_REGISTERS],
                c_f_registers_: [false; Self::K_NUM_CF_REGISTERS],
                fcsr_: 0,
                stack_: stack_ptr,
                stack_limit_: stack_limit,
                pc_modified_: false,
                icount_: 0,
                break_count_: 0,
                // trace_buf_: EmbeddedVector::new(),
                last_debugger_input_: std::ptr::null_mut(),
                // isolate_: ptr::null_mut(),
                break_pc_: std::ptr::null_mut(),
                break_instr_: Instr::default(),
                watched_stops_: [StopCountAndDesc { count: 0, desc: std::ptr::null_mut() }; Self::K_MAX_STOP_CODE_PLUS_ONE], // Initialize with a default value

                local_monitor_: LocalMonitor::new(),
                global_monitor_thread_: GlobalMonitor::LinkedAddress::new(),
            }
        }

        fn allocated_stack_size() -> usize {
            (1 * KB) + (2 * Self::K_STACK_PROTECTION_SIZE) //Placeholder: replace 1 with v8_flags.sim_stack_size
        }

        pub fn set_register(&mut self, reg: usize, value: i64) {
            self.registers_[reg] = value;
        }

        pub fn get_register(&self, reg: usize) -> i64 {
            self.registers_[reg]
        }

        pub fn get_pc(&self) -> i64 {
            self.registers_[Self::PC]
        }
        pub fn set_pc(&mut self, value: i64) {
            self.registers_[Self::PC] = value;
        }

        // Accessor to the internal simulator stack area. Adds a safety
        // margin to prevent overflows (kAdditionalStackMargin).
        pub fn stack_limit(&self, _c_limit: usize) -> usize {
            self.stack_limit_
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct Instr {} //Placeholder struct

    #[derive(Debug, Copy, Clone)]
    pub struct StopCountAndDesc {
        count: u32,
        desc: *mut i8, // Placeholder
    }

    pub struct LocalMonitor {
        access_state_: MonitorAccess,
        tagged_addr_: usize,
        size_: TransactionSize,
    }

    impl LocalMonitor {
        pub fn new() -> Self {
            LocalMonitor {
                access_state_: MonitorAccess::Open,
                tagged_addr_: 0,
                size_: TransactionSize::None,
            }
        }
        // These functions manage the state machine for the local monitor, but do
        // not actually perform loads and stores. NotifyStoreConditional only
        // returns true if the store conditional is allowed; the global monitor will
        // still have to be checked to see whether the memory should be updated.
        pub fn notify_load(&mut self) {}
        pub fn notify_load_linked(&mut self, addr: usize, size: TransactionSize) {
            self.access_state_ = MonitorAccess::RMW;
            self.tagged_addr_ = addr;
            self.size_ = size;
        }
        pub fn notify_store(&mut self) {
            self.access_state_ = MonitorAccess::Open;
        }
        pub fn notify_store_conditional(&mut self, addr: usize, size: TransactionSize) -> bool {
            if self.access_state_ == MonitorAccess::RMW && self.tagged_addr_ == addr && self.size_ == size {
                self.access_state_ = MonitorAccess::Open;
                true
            } else {
                false
            }
        }

        fn clear(&mut self) {
            self.access_state_ = MonitorAccess::Open;
            self.tagged_addr_ = 0;
            self.size_ = TransactionSize::None;
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum MonitorAccess {
        Open,
        RMW,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum TransactionSize {
        None = 0,
        Word = 4,
        DoubleWord = 8,
    }

    pub struct GlobalMonitor {
        mutex: Mutex<GlobalMonitorInternal>,
    }

    impl GlobalMonitor {
        pub fn get() -> &'static GlobalMonitor {
            use std::sync::Once;
            use std::mem::MaybeUninit;

            static mut INSTANCE: MaybeUninit<GlobalMonitor> = MaybeUninit::uninit();
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                let gm = GlobalMonitor {
                    mutex: Mutex::new(GlobalMonitorInternal::new()),
                };
                unsafe { INSTANCE.as_mut_ptr().write(gm) };
            });

            unsafe { &*INSTANCE.as_ptr() }
        }

        //TODO Implement
        // Called when the simulator is destroyed.
        pub fn remove_linked_address(&self, _linked_address: &LinkedAddress) {}

        pub fn notify_load_linked_locked(&self, addr: usize, linked_address: &mut LinkedAddress) {
            let mut guard = self.mutex.lock().unwrap();
            guard.notify_load_linked_locked(addr, linked_address);
        }
        pub fn notify_store_locked(&self, linked_address: &mut LinkedAddress) {
            let mut guard = self.mutex.lock().unwrap();
            guard.notify_store_locked(linked_address);
        }
        pub fn notify_store_conditional_locked(&self, addr: usize, linked_address: &mut LinkedAddress) -> bool {
            let mut guard = self.mutex.lock().unwrap();
            guard.notify_store_conditional_locked(addr, linked_address)
        }
    }

    struct GlobalMonitorInternal {
        head_: Option<Box<LinkedAddress>>,
    }

    impl GlobalMonitorInternal {
        fn new() -> Self {
            GlobalMonitorInternal {
                head_: None,
            }
        }
        fn notify_load_linked_locked(&mut self, addr: usize, linked_address: &mut LinkedAddress) {
             linked_address.access_state_ = MonitorAccess::RMW;
            linked_address.tagged_addr_ = addr;
        }
        fn notify_store_locked(&mut self, linked_address: &mut LinkedAddress) {
            linked_address.access_state_ = MonitorAccess::Open;
            linked_address.failure_counter_ = 0;
        }
        fn notify_store_conditional_locked(&mut self, addr: usize, linked_address: &mut LinkedAddress) -> bool {
            if linked_address.access_state_ == MonitorAccess::RMW && linked_address.tagged_addr_ == addr {
                linked_address.access_state_ = MonitorAccess::Open;
                linked_address.failure_counter_ = linked_address.failure_counter_.wrapping_add(1); //Prevent overflow on increment
                if linked_address.failure_counter_ >= LinkedAddress::K_MAX_FAILURE_COUNTER {
                    linked_address.failure_counter_ = 0;
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }
    }

    pub struct LinkedAddress {
        access_state_: MonitorAccess,
        tagged_addr_: usize,
        failure_counter_: i32,
    }

    impl LinkedAddress {
        const K_MAX_FAILURE_COUNTER: i32 = 5;
        pub fn new() -> Self {
            LinkedAddress {
                access_state_: MonitorAccess::Open,
                tagged_addr_: 0,
                failure_counter_: 0,
            }
        }
    }
}