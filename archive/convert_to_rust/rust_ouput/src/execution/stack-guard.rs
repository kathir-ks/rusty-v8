// Converted from V8 C++ source files:
// Header: stack-guard.h
// Implementation: stack-guard.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod base;
mod compiler_dispatcher;
mod execution;
mod logging;
mod objects;
mod roots;
mod tracing;
mod utils;
mod baseline;
mod maglev;
mod wasm;

use std::sync::atomic::{AtomicU32, Ordering};
use std::{cell::RefCell, rc::Rc};
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::sync::atomic::AtomicU16;

use crate::execution::isolate::Isolate;
use crate::execution::interrupts_scope::InterruptsScope;
use crate::execution::simulator::SimulatorStack;
use crate::objects::backing_store::BackingStore;
use crate::wasm::wasm_engine::GetWasmEngine;
use crate::execution::protectors_inl::Protectors;
use crate::tracing::trace_event::TracingFlags;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterruptLevel {
    kNoGC,
    kNoHeapWrites,
    kAnyEffect,
}

const KNUMBER_OF_INTERRUPT_LEVELS: usize = 3;

pub struct StackGuard {
    isolate_: *mut Isolate,
    thread_local_: ThreadLocal,
}

impl StackGuard {
    pub fn new(isolate: *mut Isolate) -> Self {
        StackGuard {
            isolate_: isolate,
            thread_local_: ThreadLocal::new(),
        }
    }

    pub fn set_stack_limit(&mut self, limit: usize) {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        self.set_stack_limit_internal(
            &access,
            limit,
            SimulatorStack::js_limit_from_c_limit(unsafe { &mut *self.isolate_ }, limit),
        );
    }

    fn set_stack_limit_internal(
        &mut self,
        _lock: &ExecutionAccess,
        limit: usize,
        jslimit: usize,
    ) {
        if self.thread_local_.jslimit() == self.thread_local_.real_jslimit_ {
            self.thread_local_.set_jslimit(jslimit);
        }
        self.thread_local_.real_jslimit_ = jslimit;
    }

    pub fn set_stack_limit_for_stack_switching(&mut self, limit: usize) {
        let old_jslimit = base::relaxed_compare_and_swap(
            &self.thread_local_.jslimit,
            self.thread_local_.real_jslimit_,
            limit,
        );
        if old_jslimit != self.thread_local_.real_jslimit_ {
            assert_eq!(old_jslimit, KINTERRUPT_LIMIT);
        }
        self.thread_local_.real_jslimit_ = limit;
    }

    pub fn push_interrupts_scope(&mut self, scope: &mut InterruptsScope) {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        if scope.mode_ == execution::interrupts_scope::Mode::kPostponeInterrupts {
            let intercepted = self.thread_local_.interrupt_flags & scope.intercept_mask_;
            scope.intercepted_flags_ = intercepted;
            self.thread_local_.interrupt_flags &= !intercepted;
        } else {
            assert_eq!(scope.mode_, execution::interrupts_scope::Mode::kRunInterrupts);
            let mut restored_flags: u32 = 0;
            let mut current = self.thread_local_.interrupt_scopes_;

            while !current.is_null() {
                unsafe {
                    restored_flags |= (*current).intercepted_flags_ & scope.intercept_mask_;
                    (*current).intercepted_flags_ &= !scope.intercept_mask_;
                    current = (*current).prev_;
                }
            }
            self.thread_local_.interrupt_flags |= restored_flags;
        }
        self.update_interrupt_requests_and_stack_limits(&access);
        scope.prev_ = self.thread_local_.interrupt_scopes_;
        self.thread_local_.interrupt_scopes_ = scope;
    }

    pub fn pop_interrupts_scope(&mut self) {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        let top = self.thread_local_.interrupt_scopes_;
        unsafe {
            if (*top).mode_ == execution::interrupts_scope::Mode::kPostponeInterrupts {
                assert_eq!(self.thread_local_.interrupt_flags & (*top).intercept_mask_, 0);
                self.thread_local_.interrupt_flags |= (*top).intercepted_flags_;
            } else {
                assert_eq!((*top).mode_, execution::interrupts_scope::Mode::kRunInterrupts);
                if !(*top).prev_.is_null() {
                    let mut interrupt: u32 = 1;
                    while interrupt < ALL_INTERRUPTS {
                        let flag: InterruptFlag = interrupt.into();
                        if (self.thread_local_.interrupt_flags & flag as u32) != 0
                            && (*(*top).prev_).intercept(flag)
                        {
                            self.thread_local_.interrupt_flags &= !(flag as u32);
                        }
                        interrupt = interrupt << 1;
                    }
                }
            }
        }

        self.update_interrupt_requests_and_stack_limits(&access);
        self.thread_local_.interrupt_scopes_ = unsafe { (*top).prev_ };
    }

    fn check_interrupt(&self, flag: InterruptFlag) -> bool {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        (self.thread_local_.interrupt_flags & flag as u32) != 0
    }

    fn request_interrupt(&mut self, flag: InterruptFlag) {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });

        let mut current = self.thread_local_.interrupt_scopes_;
        unsafe {
            if !current.is_null() && (*current).intercept(flag) {
                return;
            }
        }

        self.thread_local_.interrupt_flags |= flag as u32;
        self.update_interrupt_requests_and_stack_limits(&access);

        unsafe {
            (*self.isolate_).futex_wait_list_node().notify_wake();
        }
    }

    fn clear_interrupt(&mut self, flag: InterruptFlag) {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        let mut current = self.thread_local_.interrupt_scopes_;
        unsafe {
            while !current.is_null() {
                (*current).intercepted_flags_ &= !(flag as u32);
                current = (*current).prev_;
            }
        }

        self.thread_local_.interrupt_flags &= !(flag as u32);
        self.update_interrupt_requests_and_stack_limits(&access);
    }

    pub fn has_termination_request(&self) -> bool {
        if !self.thread_local_.has_interrupt_requested(InterruptLevel::kNoGC) {
            return false;
        }
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        if (self.thread_local_.interrupt_flags & TERMINATE_EXECUTION as u32) != 0 {
            self.thread_local_.interrupt_flags &= !(TERMINATE_EXECUTION as u32);
            self.update_interrupt_requests_and_stack_limits(&access);
            return true;
        }
        false
    }

    fn fetch_and_clear_interrupts(&mut self, level: InterruptLevel) -> i32 {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        let mut mask: InterruptFlag = Self::interrupt_level_mask(level);
        if (self.thread_local_.interrupt_flags & TERMINATE_EXECUTION as u32) != 0 {
            mask = TERMINATE_EXECUTION.into();
        }

        let result = self.thread_local_.interrupt_flags & mask as u32;
        self.thread_local_.interrupt_flags &= !(mask as u32);
        self.update_interrupt_requests_and_stack_limits(&access);
        result as i32
    }

    pub fn archive_stack_guard(&mut self, to: *mut u8) -> *mut u8 {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        unsafe {
            std::ptr::copy_nonoverlapping(
                &self.thread_local_ as *const ThreadLocal as *const u8,
                to,
                std::mem::size_of::<ThreadLocal>(),
            );
        }
        self.thread_local_ = ThreadLocal::new();
        unsafe { to.add(std::mem::size_of::<ThreadLocal>()) }
    }

    pub fn restore_stack_guard(&mut self, from: *mut u8) -> *mut u8 {
        let access = ExecutionAccess::new(unsafe { &mut *self.isolate_ });
        unsafe {
            std::ptr::copy_nonoverlapping(
                from,
                &mut self.thread_local_ as *mut ThreadLocal as *mut u8,
                std::mem::size_of::<ThreadLocal>(),
            );
        }
        unsafe { from.add(std::mem::size_of::<ThreadLocal>()) }
    }

    pub fn free_thread_resources(&mut self) {
        let per_thread = unsafe {
            (*self.isolate_)
                .find_or_allocate_per_thread_data_for_this_thread()
        };
        per_thread.set_stack_limit(self.real_climit());
    }

    pub fn init_thread(&mut self, _lock: &ExecutionAccess) {
        self.thread_local_.initialize(unsafe { &mut *self.isolate_ });
        let per_thread = unsafe {
            (*self.isolate_)
                .find_or_allocate_per_thread_data_for_this_thread()
        };
        let stored_limit = per_thread.stack_limit();

        if stored_limit != 0 {
            self.set_stack_limit(stored_limit);
        }
    }

    pub fn handle_interrupts(&mut self, level: InterruptLevel) -> *mut Object {
        unsafe {
            let isolate = &mut *self.isolate_;
            tracing::trace_event::trace_event0("v8.execute", "V8.HandleInterrupts");

            let interrupt_flags = self.fetch_and_clear_interrupts(level);

            if v8_flags::get().verify_predictable {
                isolate.heap().monotonically_increasing_time_in_ms();
            }

            if (interrupt_flags & TERMINATE_EXECUTION as i32) != 0 {
                tracing::trace_event::trace_event0("v8.execute", "V8.TerminateExecution");
                return isolate.terminate_execution();
            }

            if (interrupt_flags & GC_REQUEST as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.gc"),
                    "V8.GCHandleGCRequest",
                );
                isolate.heap().handle_gc_request();
            }

            if (interrupt_flags & START_INCREMENTAL_MARKING as i32) != 0 {
                isolate.heap().start_incremental_marking_on_interrupt();
            }

            if (interrupt_flags & GLOBAL_SAFEPOINT as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.gc"),
                    "V8.GlobalSafepoint",
                );
                isolate.main_thread_local_heap().safepoint();
            }

            if (interrupt_flags & GROW_SHARED_MEMORY as i32) != 0 {
                tracing::trace_event::trace_event0("v8.wasm", "V8.WasmGrowSharedMemory");
                BackingStore::update_shared_wasm_memory_objects(isolate);
            }

            if (interrupt_flags & LOG_WASM_CODE as i32) != 0 {
                tracing::trace_event::trace_event0("v8.wasm", "V8.LogCode");
                GetWasmEngine().log_outstanding_codes_for_isolate(isolate);
            }

            if (interrupt_flags & WASM_CODE_GC as i32) != 0 {
                tracing::trace_event::trace_event0("v8.wasm", "V8.WasmCodeGC");
                GetWasmEngine().report_live_code_from_stack_for_gc(isolate);
            }

            if (interrupt_flags & DEOPT_MARKED_ALLOCATION_SITES as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.gc"),
                    "V8.GCDeoptMarkedAllocationSites",
                );
                isolate.heap().deopt_marked_allocation_sites();
            }

            if (interrupt_flags & INSTALL_CODE as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.compile"),
                    "V8.InstallOptimizedFunctions",
                );
                if isolate.concurrent_recompilation_enabled() {
                    isolate.optimizing_compile_dispatcher().install_optimized_functions();
                }
            }

            if (interrupt_flags & INSTALL_BASELINE_CODE as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.compile"),
                    "V8.FinalizeBaselineConcurrentCompilation",
                );
                isolate.baseline_batch_compiler().install_batch();
            }

            if (interrupt_flags & INSTALL_MAGLEV_CODE as i32) != 0 {
                tracing::trace_event::trace_event0(
                    tracing::trace_event::TraceCategory::DisabledByDefault("v8.compile"),
                    "V8.FinalizeMaglevConcurrentCompilation",
                );
                isolate.maglev_concurrent_dispatcher().finalize_finished_jobs();
            }

            if (interrupt_flags & API_INTERRUPT as i32) != 0 {
                tracing::trace_event::trace_event0("v8.execute", "V8.InvokeApiInterruptCallbacks");
                isolate.invoke_api_interrupt_callbacks();
            }

            if TracingFlags::is_runtime_stats_enabled()
                && Protectors::is_no_profiling_intact(isolate)
            {
                Protectors::invalidate_no_profiling(isolate);
            }

            isolate.counters().stack_interrupts().increment();

            crate::roots::roots_inl::ReadOnlyRoots(isolate).undefined_value()
        }
    }

    fn update_interrupt_requests_and_stack_limits(&mut self, _lock: &ExecutionAccess) {
        unsafe {
            let isolate = &mut *self.isolate_;
            if self.has_pending_interrupts(_lock) {
                self.thread_local_.set_jslimit(KINTERRUPT_LIMIT);
            } else {
                self.thread_local_.set_jslimit(self.thread_local_.real_jslimit_);
            }

            for level in [
                InterruptLevel::kNoGC,
                InterruptLevel::kNoHeapWrites,
                InterruptLevel::kAnyEffect,
            ]
            .iter()
            {
                self.thread_local_.set_interrupt_requested(
                    *level,
                    (Self::interrupt_level_mask(*level) as u32
                        & self.thread_local_.interrupt_flags)
                        != 0,
                );
            }
        }
    }

    fn has_pending_interrupts(&self, _lock: &ExecutionAccess) -> bool {
        self.thread_local_.interrupt_flags != 0
    }

    pub fn climit(&self) -> usize {
        self.thread_local_.jslimit()
    }

    pub fn jslimit(&self) -> usize {
        self.thread_local_.jslimit()
    }

    pub fn real_climit(&self) -> usize {
        self.thread_local_.real_jslimit_
    }

    pub fn real_jslimit(&self) -> usize {
        self.thread_local_.real_jslimit_
    }

    pub fn address_of_jslimit(&self) -> usize {
        &self.thread_local_.jslimit as *const AtomicUsize as usize
    }

    pub fn address_of_real_jslimit(&self) -> usize {
        &self.thread_local_.real_jslimit_ as *const usize as usize
    }

    pub fn address_of_interrupt_request(&self, level: InterruptLevel) -> usize {
        &self.thread_local_.interrupt_requested[level as usize] as *const AtomicBool as usize
    }

    pub const fn jslimit_offset() -> usize {
        let offset = std::mem::offset_of!(StackGuard, thread_local_)
            + std::mem::offset_of!(ThreadLocal, jslimit);
        offset
    }

    pub const fn real_jslimit_offset() -> usize {
        let offset = std::mem::offset_of!(StackGuard, thread_local_)
            + std::mem::offset_of!(ThreadLocal, real_jslimit_);
        offset
    }

    pub const fn k_size_in_bytes() -> usize {
        8 * std::mem::size_of::<usize>()
    }

    pub fn iterate(v: *mut i32, thread_storage: *mut i8) -> *mut i8 {
        unsafe { thread_storage.add(Self::archive_space_per_thread()) }
    }

    pub fn archive_space_per_thread() -> usize {
        std::mem::size_of::<ThreadLocal>()
    }

    fn interrupt_level_mask(level: InterruptLevel) -> InterruptFlag {
        let mut mask: InterruptFlag = TERMINATE_EXECUTION.into();
        if level >= InterruptLevel::kNoGC {
            mask = mask | GC_REQUEST.into();
        }
        if level >= InterruptLevel::kNoHeapWrites {
            mask = mask
                | INSTALL_CODE.into()
                | INSTALL_BASELINE_CODE.into()
                | API_INTERRUPT.into()
                | DEOPT_MARKED_ALLOCATION_SITES.into()
                | WASM_CODE_GC.into()
                | GLOBAL_SAFEPOINT.into()
                | START_INCREMENTAL_MARKING.into();
        }
        if level >= InterruptLevel::kAnyEffect {
            mask = mask
                | GROW_SHARED_MEMORY.into()
                | LOG_WASM_CODE.into()
                | INSTALL_MAGLEV_CODE.into();
        }
        mask
    }
}

atomic_assert_eq_size!(StackGuard, [u8; 64]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExecutionAccess<'a> {
    isolate: &'a mut Isolate,
}

impl<'a> ExecutionAccess<'a> {
    pub fn new(isolate: &'a mut Isolate) -> Self {
        ExecutionAccess { isolate }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterruptFlag {
    TERMINATE_EXECUTION,
    GC_REQUEST,
    INSTALL_CODE,
    INSTALL_BASELINE_CODE,
    API_INTERRUPT,
    DEOPT_MARKED_ALLOCATION_SITES,
    GROW_SHARED_MEMORY,
    LOG_WASM_CODE,
    WASM_CODE_GC,
    INSTALL_MAGLEV_CODE,
    GLOBAL_SAFEPOINT,
    START_INCREMENTAL_MARKING,
    ALL_INTERRUPTS,
}

impl From<u32> for InterruptFlag {
    fn from(value: u32) -> Self {
        match value {
            1 => InterruptFlag::TERMINATE_EXECUTION,
            2 => InterruptFlag::GC_REQUEST,
            4 => InterruptFlag::INSTALL_CODE,
            8 => InterruptFlag::INSTALL_BASELINE_CODE,
            16 => InterruptFlag::API_INTERRUPT,
            32 => InterruptFlag::DEOPT_MARKED_ALLOCATION_SITES,
            64 => InterruptFlag::GROW_SHARED_MEMORY,
            128 => InterruptFlag::LOG_WASM_CODE,
            256 => InterruptFlag::WASM_CODE_GC,
            512 => InterruptFlag::INSTALL_MAGLEV_CODE,
            1024 => InterruptFlag::GLOBAL_SAFEPOINT,
            2048 => InterruptFlag::START_INCREMENTAL_MARKING,
            _ => InterruptFlag::ALL_INTERRUPTS,
        }
    }
}

impl From<InterruptFlag> for u32 {
    fn from(flag: InterruptFlag) -> Self {
        match flag {
            InterruptFlag::TERMINATE_EXECUTION => 1,
            InterruptFlag::GC_REQUEST => 2,
            InterruptFlag::INSTALL_CODE => 4,
            InterruptFlag::INSTALL_BASELINE_CODE => 8,
            InterruptFlag::API_INTERRUPT => 16,
            InterruptFlag::DEOPT_MARKED_ALLOCATION_SITES => 32,
            InterruptFlag::GROW_SHARED_MEMORY => 64,
            InterruptFlag::LOG_WASM_CODE => 128,
            InterruptFlag::WASM_CODE_GC => 256,
            InterruptFlag::INSTALL_MAGLEV_CODE => 512,
            InterruptFlag::GLOBAL_SAFEPOINT => 1024,
            InterruptFlag::START_INCREMENTAL_MARKING => 2048,
            InterruptFlag::ALL_INTERRUPTS => 4095,
        }
    }
}

const TERMINATE_EXECUTION: InterruptFlag = InterruptFlag::TERMINATE_EXECUTION;
const GC_REQUEST: InterruptFlag = InterruptFlag::GC_REQUEST;
const INSTALL_CODE: InterruptFlag = InterruptFlag::INSTALL_CODE;
const INSTALL_BASELINE_CODE: InterruptFlag = InterruptFlag::INSTALL_BASELINE_CODE;
const API_INTERRUPT: InterruptFlag = InterruptFlag::API_INTERRUPT;
const DEOPT_MARKED_ALLOCATION_SITES: InterruptFlag =
    InterruptFlag::DEOPT_MARKED_ALLOCATION_SITES;
const GROW_SHARED_MEMORY: InterruptFlag = InterruptFlag::GROW_SHARED_MEMORY;
const LOG_WASM_CODE: InterruptFlag = InterruptFlag::LOG_WASM_CODE;
const WASM_CODE_GC: InterruptFlag = InterruptFlag::WASM_CODE_GC;
const INSTALL_MAGLEV_CODE: InterruptFlag = InterruptFlag::INSTALL_MAGLEV_CODE;
const GLOBAL_SAFEPOINT: InterruptFlag = InterruptFlag::GLOBAL_SAFEPOINT;
const START_INCREMENTAL_MARKING: InterruptFlag = InterruptFlag::START_INCREMENTAL_MARKING;
const ALL_INTERRUPTS: u32 = 4095;

const KINTERRUPT_LIMIT: usize = 0xfffffffffffffffe;
const KILLEGAL_LIMIT: usize = 0xfffffffffffffff8;

use std::sync::atomic::{AtomicBool, AtomicUsize};

#[derive(Debug, Copy, Clone)]
struct ThreadLocal {
    real_jslimit_: usize,
    jslimit: AtomicUsize,
    interrupt_requested: [AtomicBool; KNUMBER_OF_INTERRUPT_LEVELS],
    interrupt_scopes_: *mut InterruptsScope,
    interrupt_flags: u32,
}

impl ThreadLocal {
    pub fn new() -> Self {
        ThreadLocal {
            real_jslimit_: KILLEGAL_LIMIT,
            jslimit: AtomicUsize::new(KILLEGAL_LIMIT),
            interrupt_requested: [
                AtomicBool::new(false),
                AtomicBool::new(false),
                AtomicBool::new(false),
            ],
            interrupt_scopes_: std::ptr::null_mut(),
            interrupt_flags: 0,
        }
    }

    fn initialize(&mut self, isolate: &mut Isolate) {
        let k_limit_size = v8_flags::get().stack_size * 1024;
        let limit = base::stack::get_stack_start() - k_limit_size as usize;
        self.real_jslimit_ = SimulatorStack::js_limit_from_c_limit(isolate, limit);
        self.set_jslimit(SimulatorStack::js_limit_from_c_limit(isolate, limit));
        self.interrupt_scopes_ = std::ptr::null_mut();
        self.interrupt_flags = 0;
    }

    fn jslimit(&self) -> usize {
        self.jslimit.load(Ordering::Relaxed)
    }

    fn set_jslimit(&mut self, limit: usize) {
        self.jslimit.store(limit, Ordering::Relaxed);
    }

    fn set_interrupt_requested(&mut self, level: InterruptLevel, requested: bool) {
        self.interrupt_requested[level as usize].store(requested, Ordering::Relaxed);
    }

    fn has_interrupt_requested(&self, level: InterruptLevel) -> bool {
        self.interrupt_requested[level as usize].load(Ordering::Relaxed)
    }
}

atomic_assert_eq_size!(ThreadLocal, [u8; 48]);

mod v8_flags {
    pub struct Flags {
        pub stack_size: u32,
        pub verify_predictable: bool,
    }

    impl Flags {
        pub const fn new() -> Self {
            Self {
                stack_size: 2048,
                verify_predictable: false,
            }
        }
    }

    const FLAGS: Flags = Flags::new();

    pub fn get() -> &'static Flags {
        &FLAGS
    }
}
