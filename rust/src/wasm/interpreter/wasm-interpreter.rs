// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]

// #![cfg(feature = "enable_webassembly")] // Conditional compilation based on feature flag

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

// use crate::base::platform::time::{ElapsedTimer, TimeTicks, TimeDelta}; // Assuming these are custom types
// use crate::base::small_vector::SmallVector; // Assuming this is a custom type
// use crate::base::vector::Vector; // Assuming this is a custom type
// use crate::common::simd128::Simd128; // Assuming this is a custom type
// use crate::logging::counters::Histogram; // Assuming this is a custom type
// use crate::wasm::function_body_decoder_impl::BodyLocalDecls; // Assuming this is a custom type
// use crate::wasm::interpreter::instruction_handlers; // Assuming this is a module
// use crate::wasm::interpreter::wasm_interpreter_objects::*; // Assuming this is a module
// use crate::wasm::wasm_value::WasmValue; // Assuming this is a custom type
// use crate::wasm::wasm_opcodes::WasmOpcode; // Assuming this is a custom type
// use crate::wasm::decoder::Decoder; // Assuming this is a custom type
// use crate::wasm::wasm_module::WasmModule; // Assuming this is a custom type
// use crate::wasm::function_sig::FunctionSig; // Assuming this is a custom type

// Placeholder types, need to be defined elsewhere
type Address = usize;
type DirectHandle<T> = *mut T;
type Handle<T> = *mut T;
type Isolate = usize;
type Zone = usize;
type ZoneVector<T> = Vec<T>;
type ModuleTypeIndex = u32;
type BodyLocalDecls = usize;
type TrapReason = u32;
type MessageTemplate = u32;
type HeapType = u32;
type ValueKind = u32;
type Object = u32;
type WasmOpcode = u8;
type PageAllocator = u32;
type FunctionSig = u32;
type WasmRef = u32;
const kWasmAnyRef: u32 = 0;

const kSystemPointerSize: usize = 8; // Or 4, depending on architecture

/// DrumBrake: An interpreter for WebAssembly.
pub mod wasm_interpreter {

    use super::*;

    macro_rules! V8_INLINE {
        ($code:block) => {
            #[inline]
            $code
        };
    }

    macro_rules! V8_EXPORT_PRIVATE {
        ($vis:vis struct $name:ident { $($body:tt)* }) => {
            $vis struct $name { $($body)* }
        };
        ($vis:vis class $name:ident { $($body:tt)* }) => {
            $vis struct $name { $($body)* }
        };
        ($vis:vis trait $name:ident { $($body:tt)* }) => {
            $vis trait $name { $($body)* }
        };
    }

    macro_rules! DISABLE_CFI_ICALL {
        () => {}; // no-op in Rust. CFI is handled differently.
    }

    macro_rules! UNREACHABLE {
        () => {
            panic!("Unreachable code");
        };
    }

    macro_rules! CHECK_WITH_MSG {
        ($cond:expr, $msg:expr) => {
            if !($cond) {
                panic!("Check failed: {}", $msg);
            }
        };
    }

    macro_rules! DCHECK {
        ($cond:expr) => {
            if cfg!(debug_assertions) {
                assert!($cond);
            }
        };
    }

    macro_rules! DCHECK_NOT_NULL {
        ($ptr:expr) => {
            if cfg!(debug_assertions) {
                assert!(!$ptr.is_null());
            }
        };
    }
    
    macro_rules! V8_NODISCARD {
        ($struct:item) => {
            #[must_use]
            $struct
        };
    }

    /// Code and metadata needed to execute a function.
    pub struct InterpreterCode {
        pub function: *const WasmFunction, // wasm function
        pub locals: BodyLocalDecls,        // local declarations
        pub start: *const u8,              // start of code
        pub end: *const u8,                // end of code
        pub bytecode: Option<Box<WasmBytecode>>,
    }

    impl InterpreterCode {
        pub fn new(
            function: *const WasmFunction,
            locals: BodyLocalDecls,
            start: *const u8,
            end: *const u8,
        ) -> Self {
            InterpreterCode {
                function,
                locals,
                start,
                end,
                bytecode: None,
            }
        }

        pub fn at(&self, pc: usize) -> *const u8 {
            unsafe { self.start.add(pc) }
        }
    }

    pub struct FrameState {
        pub current_function_: *const WasmBytecode,
        pub previous_frame_: *const FrameState,
        pub current_bytecode_: *const u8,
        pub current_sp_: *mut u8,
        pub thread_: *mut WasmInterpreterThread,
        pub ref_array_current_sp_: u32,
        pub handle_scope_: *mut HandleScope,
        // V8_ENABLE_DRUMBRAKE_TRACING
        // pub current_stack_height_: u32,
        // pub current_stack_start_args_: u32,
        // pub current_stack_start_locals_: u32,
        // pub current_stack_start_stack_: u32,
        pub caught_exceptions_: *mut FixedArray,
    }

    impl FrameState {
        pub fn new() -> Self {
            FrameState {
                current_function_: std::ptr::null(),
                previous_frame_: std::ptr::null(),
                current_bytecode_: std::ptr::null(),
                current_sp_: std::ptr::null_mut(),
                thread_: std::ptr::null_mut(),
                ref_array_current_sp_: 0,
                handle_scope_: std::ptr::null_mut(),
                caught_exceptions_: std::ptr::null_mut(),
                // V8_ENABLE_DRUMBRAKE_TRACING
                // current_stack_height_: 0,
                // current_stack_start_args_: 0,
                // current_stack_start_locals_: 0,
                // current_stack_start_stack_: 0,
            }
        }

        pub fn set_caught_exception(&mut self, isolate: Isolate, catch_block_index: u32, exception: DirectHandle<Object>) {
            // TODO: Implement the logic for setting caught exceptions in a FixedArray
            // Requires working with FixedArray which isn't directly translatable from C++
            unsafe {
                // Accessing raw pointer `caught_exceptions_`
                if self.caught_exceptions_.is_null() {
                    // Allocate a FixedArray if it doesn't exist.  Size depends on the number of catch blocks
                    // self.caught_exceptions_ = FixedArray::New(isolate, num_catch_blocks);
                }
    
                // Assign the exception to the appropriate index in the FixedArray
                // (*self.caught_exceptions_).set(catch_block_index, exception);
            }

        }

        pub fn get_caught_exception(&self, isolate: Isolate, catch_block_index: u32) -> DirectHandle<Object> {
            // TODO: Implement the logic for getting caught exceptions from a FixedArray
            // Requires working with FixedArray which isn't directly translatable from C++
            if self.caught_exceptions_.is_null() {
                return std::ptr::null_mut(); // Or some appropriate null-like value
            }
            unsafe {
                // return (*self.caught_exceptions_).get(catch_block_index);
                std::ptr::null_mut()
            }
        }

        pub fn dispose_caught_exceptions_array(&mut self, isolate: Isolate) {
            // TODO: Implement the logic for disposing the caught exceptions array
            // Requires memory management of FixedArray which isn't directly translatable from C++
            // if (!self.caught_exceptions_.is_null()) {
                // self.caught_exceptions_.Dispose(isolate);
            //    self.caught_exceptions_ = std::ptr::null_mut(); // Make sure it's nulled out after disposing
            // }
        }

        #[inline]
        pub fn reset_handle_scope(&mut self, isolate: Isolate) {
            // TODO: Implement handle scope reset. Requires understanding V8's HandleScope.
            // if !self.handle_scope_.is_null() {
            //     self.handle_scope_.reset();
            // }
            
        }
    }

    /// Manages the calculations of the
    /// V8.WasmInterpreterExecutionInTenSecondsPercentage histogram.
    pub struct WasmExecutionTimer {
        execute_ratio_histogram_: *mut Histogram,
        slow_wasm_histogram_: *mut Histogram,
        window_execute_timer_: ElapsedTimer,
        window_has_started_: bool,
        next_interval_time_: TimeTicks,
        start_interval_time_: TimeTicks,
        window_running_time_: TimeDelta,
        sample_duration_: TimeDelta,
        cooldown_interval_: TimeDelta, // Pause between samples.
        slow_threshold_: i32,
        slow_threshold_samples_count_: usize,
        samples_: Vec<i32>,
        isolate_: Isolate,
    }

    impl WasmExecutionTimer {
        pub fn new(isolate: Isolate, track_jitless_wasm: bool) -> Self {
            //TODO: Properly fetch histograms from the isolate

            WasmExecutionTimer {
                execute_ratio_histogram_: std::ptr::null_mut(), //Histogram::Find("V8.WasmInterpreterExecutionInTenSecondsPercentage"),
                slow_wasm_histogram_: std::ptr::null_mut(),  //Histogram::Find("V8.SlowWasmInterpreterExecutionInTenSeconds"),
                window_execute_timer_: ElapsedTimer::new(),
                window_has_started_: false,
                next_interval_time_: TimeTicks::new(),
                start_interval_time_: TimeTicks::new(),
                window_running_time_: TimeDelta::new(0),
                sample_duration_: TimeDelta::new(Duration::from_millis(500).as_micros() as i64),
                cooldown_interval_: TimeDelta::new(Duration::from_secs(55).as_micros() as i64),
                slow_threshold_: 50,
                slow_threshold_samples_count_: 6,
                samples_: Vec::new(),
                isolate_: isolate,
            }
        }

        V8_INLINE!({
            pub fn start(&mut self) {
                if !self.execute_ratio_histogram_.is_null() {
                    self.start_internal();
                }
            }
        });

        V8_INLINE!({
            pub fn stop(&mut self) {
                if !self.execute_ratio_histogram_.is_null() {
                    self.stop_internal();
                }
            }
        });

        pub fn terminate(&mut self) {
            //TODO: Terminate the timer and clear resources
        }

        fn start_internal(&mut self) {
            if !self.window_has_started_ {
                self.begin_interval(true);
                self.window_has_started_ = true;
            }
        }

        fn stop_internal(&mut self) {
            if self.window_has_started_ {
                self.end_interval();
            }
        }

        fn begin_interval(&mut self, start_timer: bool) {
            self.start_interval_time_ = TimeTicks::now();
            self.next_interval_time_ = self.start_interval_time_ + self.sample_duration_;
            if start_timer {
                self.window_execute_timer_.start();
            }
        }

        fn end_interval(&mut self) {
            let end_time = TimeTicks::now();
            let running_time = self.window_execute_timer_.elapsed();
            self.window_running_time_ = self.window_running_time_ + running_time;
            self.window_execute_timer_.stop();

            if end_time >= self.next_interval_time_ {
                let total_window_time = end_time - self.start_interval_time_;
                let running_ratio = (self.window_running_time_.as_micros() * 100000 / total_window_time.as_micros()) as i32;
                self.add_sample(running_ratio);
                self.window_running_time_ = TimeDelta::new(0);

                self.start_interval_time_ = end_time;
                self.next_interval_time_ = end_time + self.sample_duration_;
            }

            if end_time >= self.next_interval_time_ + self.cooldown_interval_ {
                self.window_has_started_ = false;
            }
        }

        fn add_sample(&mut self, running_ratio: i32) {
            self.samples_.push(running_ratio);
            if self.samples_.len() > self.slow_threshold_samples_count_ {
                self.samples_.remove(0); // Remove the oldest sample.

                let slow_samples_count = self.samples_.iter().filter(|&x| *x > self.slow_threshold_).count();
                if slow_samples_count == self.slow_threshold_samples_count_ && !self.slow_wasm_histogram_.is_null() {
                    // self.slow_wasm_histogram_.AddSample(1);
                }

                let total = self.samples_.iter().sum::<i32>() as i64;
                let average = total / self.samples_.len() as i64;

                if !self.execute_ratio_histogram_.is_null() {
                    // self.execute_ratio_histogram_.AddSample(average as i32);
                }
            }
        }
    }
    
    struct TimeTicks {
        ticks: i64, // Represents the internal tick count.
    }
    
    impl TimeTicks {
        fn new() -> Self {
            TimeTicks { ticks: 0 }
        }
    
        fn now() -> Self {
            // Simulate getting the current time.
            TimeTicks { ticks: Instant::now().elapsed().as_micros() as i64 }
        }
    
        fn as_micros(&self) -> i64 {
            self.ticks
        }
    }
    
    impl std::ops::Add<TimeDelta> for TimeTicks {
        type Output = Self;
    
        fn add(self, delta: TimeDelta) -> Self {
            TimeTicks { ticks: self.ticks + delta.micros }
        }
    }
    
    impl std::ops::Sub for TimeTicks {
        type Output = TimeDelta;
    
        fn sub(self, other: Self) -> Self {
            TimeDelta { micros: self.ticks - other.ticks }
        }
    }
    
    struct TimeDelta {
        micros: i64, // Represents time duration in microseconds.
    }
    
    impl TimeDelta {
        fn new(micros: i64) -> Self {
            TimeDelta { micros }
        }
    
        fn as_micros(&self) -> i64 {
            self.micros
        }
    }
    
    struct ElapsedTimer {
        start_time: Option<Instant>,
        elapsed: Duration,
        is_running: bool,
    }
    
    impl ElapsedTimer {
        fn new() -> Self {
            ElapsedTimer {
                start_time: None,
                elapsed: Duration::from_secs(0),
                is_running: false,
            }
        }
    
        fn start(&mut self) {
            if !self.is_running {
                self.start_time = Some(Instant::now());
                self.is_running = true;
            }
        }
    
        fn stop(&mut self) {
            if self.is_running {
                if let Some(start) = self.start_time {
                    self.elapsed += Instant::now().duration_since(start);
                    self.start_time = None;
                    self.is_running = false;
                }
            }
        }
    
        fn elapsed(&self) -> TimeDelta {
            // If the timer is running, calculate the additional elapsed time.
            if self.is_running {
                if let Some(start) = self.start_time {
                     let additional_elapsed = Instant::now().duration_since(start);
                    return TimeDelta::new((self.elapsed + additional_elapsed).as_micros() as i64);
                }
            }
            TimeDelta::new(self.elapsed.as_micros() as i64)
        }
    }

    /// Map to store WasmInterpreterThread instances per Isolate
    V8_EXPORT_PRIVATE!(struct WasmInterpreterThreadMap {
        map_: Mutex<HashMap<i32, Box<WasmInterpreterThread>>>,
        mutex_: Mutex<()>,
    });

    impl WasmInterpreterThreadMap {
        pub fn get_current_interpreter_thread(&self, isolate: Isolate) -> *mut WasmInterpreterThread {
            let isolate_id = isolate as i32;
            let map = self.map_.lock().unwrap();
            if let Some(thread) = map.get(&isolate_id) {
                return &mut **thread;
            }
            std::ptr::null_mut()
        }

        pub fn notify_isolate_disposal(&self, isolate: Isolate) {
            let isolate_id = isolate as i32;
            let mut map = self.map_.lock().unwrap();
            map.remove(&isolate_id);
        }
    }

    /// Representation of a thread in the interpreter.
    V8_EXPORT_PRIVATE!(struct WasmInterpreterThread {
        isolate_: Isolate,
        state_: State,
        trap_reason_: TrapReason,
        current_stack_size_: u32,
        stack_mem_: *mut u8,
        activations_: Vec<Box<Activation>>,
        reference_stack_: *mut FixedArray, //Handle<FixedArray>, // References are kept on an on-heap stack.
        current_ref_stack_size_: usize,
        execution_timer_: WasmExecutionTimer,
    });

    impl Drop for WasmInterpreterThread {
        fn drop(&mut self) {
            unsafe {
                if !self.stack_mem_.is_null() {
                    // deallocate(self.stack_mem_ as *mut u8, Layout::from_size_align(self.current_stack_size_ as usize, 1).unwrap());
                }
            }
        }
    }

    impl WasmInterpreterThread {
        const K_INITIAL_STACK_SIZE: u32 = 1 * 1024 * 1024; // 1 MB
        const K_STACK_SIZE_INCREMENT: u32 = 1 * 1024 * 1024; // 1 MB
        const K_MAX_STACK_SIZE: u32 = 32 * 1024 * 1024; // 32 MB

        pub fn new(isolate: Isolate) -> Self {
            let mut thread = WasmInterpreterThread {
                isolate_: isolate,
                state_: State::STOPPED,
                trap_reason_: 0,
                current_stack_size_: 0,
                stack_mem_: std::ptr::null_mut(),
                activations_: Vec::new(),
                reference_stack_: std::ptr::null_mut(),
                current_ref_stack_size_: 0,
                execution_timer_: WasmExecutionTimer::new(isolate, false),
            };

            // Allocate the initial stack.
            if thread.expand_stack(WasmInterpreterThread::K_INITIAL_STACK_SIZE as usize) {
                thread
            } else {
                panic!("Failed to allocate initial stack for WasmInterpreterThread");
            }
        }

        pub fn expand_stack(&mut self, additional_required_size: usize) -> bool {
             if self.current_stack_size_ as usize + additional_required_size > WasmInterpreterThread::K_MAX_STACK_SIZE as usize {
                 return false;
             }
        
             let mut new_size = self.current_stack_size_;
             while new_size as usize  < self.current_stack_size_ as usize + additional_required_size {
                 new_size = std::cmp::min(new_size + WasmInterpreterThread::K_STACK_SIZE_INCREMENT, WasmInterpreterThread::K_MAX_STACK_SIZE);
             }
        
             // SetPermissions is not implementable without an OS abstraction.  The code here is only a placeholder.
             let page_allocator: PageAllocator = 0;
             // let success = SetPermissions(page_allocator, self.stack_mem_ as usize, new_size as usize, PageAllocator::Permission::kReadWrite);
             let success = true; // Mock success
        
             if success {
                 self.current_stack_size_ = new_size;
                 return true;
             }
        
             false
         }

        pub fn initialize() {
            unsafe {
                if THREAD_INTERPRETER_MAP_S.is_null() {
                    THREAD_INTERPRETER_MAP_S = Box::into_raw(Box::new(WasmInterpreterThreadMap {
                        map_: Mutex::new(HashMap::new()),
                        mutex_: Mutex::new(()),
                    }));
                }
            }
        }

        pub fn terminate() {
            unsafe {
                if !THREAD_INTERPRETER_MAP_S.is_null() {
                    drop(Box::from_raw(THREAD_INTERPRETER_MAP_S));
                    THREAD_INTERPRETER_MAP_S = std::ptr::null_mut();
                }
            }
        }

        pub fn notify_isolate_disposal(isolate: Isolate) {
            unsafe {
                if !THREAD_INTERPRETER_MAP_S.is_null() {
                    (*THREAD_INTERPRETER_MAP_S).notify_isolate_disposal(isolate);
                }
            }
        }

        pub fn get_current_interpreter_thread(isolate: Isolate) -> *mut WasmInterpreterThread {
            unsafe {
                DCHECK_NOT_NULL!(THREAD_INTERPRETER_MAP_S);
                (*THREAD_INTERPRETER_MAP_S).get_current_interpreter_thread(isolate)
            }
        }

        pub fn get_isolate(&self) -> Isolate {
            self.isolate_
        }

        pub fn state(&self) -> State {
            self.state_
        }

        pub fn run(&mut self) {
            // TODO: Implement trap handler logic. trap_handler is not directly translatable.
            // if !trap_handler::IsThreadInWasm() {
            //     trap_handler::SetThreadInWasm();
            // }
            self.state_ = State::RUNNING;
        }

        pub fn stop(&mut self) {
            self.state_ = State::STOPPED;
        }

        pub fn trap(&mut self, trap_reason: TrapReason, trap_function_index: i32, trap_pc: i32, current_frame: &FrameState) {
            self.state_ = State::TRAPPED;
            self.trap_reason_ = trap_reason;

            DCHECK!(!self.activations_.is_empty());
            if let Some(activation) = self.activations_.last_mut() {
                activation.set_current_frame(current_frame);
                activation.set_trapped(trap_function_index, trap_pc);
            }
        }

        pub fn get_trap_reason(&self) -> TrapReason {
            self.trap_reason_
        }

        pub fn unwinding(&mut self) {
            self.state_ = State::EH_UNWINDING;
        }

        #[inline]
        pub fn start_activation(
            &mut self,
            wasm_runtime: *mut WasmInterpreterRuntime,
            frame_pointer: Address,
            interpreter_fp: *mut u8,
            frame_state: &FrameState,
        ) -> *mut Activation {
            let activation = Box::new(Activation::new(self, wasm_runtime, frame_pointer, interpreter_fp, frame_state));
            let activation_ptr = Box::into_raw(activation);
            self.activations_.push(unsafe { Box::from_raw(activation_ptr) }); // avoid a double free
            activation_ptr
        }

        #[inline]
        pub fn finish_activation(&mut self) {
            DCHECK!(!self.activations_.is_empty());
            self.activations_.pop();
            if self.activations_.is_empty() {
                self.finish();
            }
        }

        #[inline]
        pub fn get_current_activation_for(&self, wasm_runtime: *const WasmInterpreterRuntime) -> Option<&FrameState> {
            self.activations_.last().map(|activation| activation.get_current_frame())
        }

        #[inline]
        pub fn set_current_frame(&mut self, frame_state: &FrameState) {
            DCHECK!(!self.activations_.is_empty());
            if let Some(activation) = self.activations_.last_mut() {
                activation.set_current_frame(frame_state);
            }
        }

        #[inline]
        pub fn set_current_activation_frame(
            &mut self,
            fp: *mut u32,
            current_frame_size: u32,
            current_stack_size: u32,
            current_ref_stack_fp: u32,
            current_ref_stack_frame_size: u32,
        ) {
            DCHECK!(!self.activations_.is_empty());
            if let Some(activation) = self.activations_.last_mut() {
                activation.set_current_activation_frame(
                    fp as *mut u8,
                    current_frame_size,
                    current_stack_size,
                    current_ref_stack_fp,
                    current_ref_stack_frame_size,
                );
            }
        }

        pub fn get_activation(&self, frame_pointer: Address) -> Option<&Activation> {
            self.activations_.iter().find(|activation| activation.get_frame_pointer() == frame_pointer).map(|boxed| boxed.as_ref())
        }

        pub fn next_frame_address(&self) -> *mut u8 {
            if self.activations_.is_empty() {
                self.stack_mem_
            } else {
                self.activations_.last().map(|activation| activation.next_frame_address()).unwrap()
            }
        }

        pub fn next_ref_stack_offset(&self) -> u32 {
            if self.activations_.is_empty() {
                0
            } else {
                self.activations_.last().map(|activation| activation.next_ref_stack_offset()).unwrap()
            }
        }

        pub fn stack_limit_address(&self) -> *const u8 {
            unsafe { self.stack_mem_.add(self.current_stack_size_ as usize) as *const u8}
        }

        pub fn ensure_ref_stack_space(&mut self, new_size: usize) {
            // TODO: Implement EnsureRefStackSpace. Requires FixedArray and Isolate interaction
            // if new_size > self.current_ref_stack_size_ {
            //    let isolate = self.GetIsolate();
            //    let new_array = FixedArray::New(isolate, new_size);
            //    if !self.reference_stack_.is_null() {
            //        new_array.CopyFrom(self.reference_stack_, 0, 0, self.current_ref_stack_size_);
            //    }
            //    self.reference_stack_ = new_array;
            //    self.current_ref_stack_size_ = new_size;
            // }
        }

        pub fn clear_ref_stack_values(&mut self, index: usize, count: usize) {
            // TODO: Implement ClearRefStackValues. Requires FixedArray interaction.
            // Requires unsafe mutable access and setting values within a FixedArray
            // for i in index..(index+count) {
            //  unsafe {
            //     (*self.reference_stack_).set(i,null());
            //   }
            // }
        }

        pub fn start_execution_timer(&mut self) {
            self.execution_timer_.start();
        }

        pub fn stop_execution_timer(&mut self) {
            self.execution_timer_.stop();
        }

        pub fn terminate_execution_timers(&mut self) {
            self.execution_timer_.terminate();
        }

        pub fn set_runtime_last_wasm_error(isolate: Isolate, message: MessageTemplate) {
            // TODO: Implement SetRuntimeLastWasmError. Requires interacting with Isolate state
            // isolate.set_runtime_last_wasm_error(message);
        }

        pub fn get_runtime_last_wasm_error(isolate: Isolate) -> TrapReason {
            // TODO: Implement GetRuntimeLastWasmError. Requires interacting with Isolate state
            // isolate.get_runtime_last_wasm_error()
            0 // Dummy return
        }

        // V8_ENABLE_DRUMBRAKE_TRACING
        // pub fn current_stack_frame_start(&self) -> u32 {
        //     if self.activations_.is_empty() {
        //         0
        //     } else {
        //         self.activations_.last().unwrap().current_stack_frame_start()
        //     }
        // }

        // V8_ENABLE_DRUMBRAKE_TRACING
        // pub fn current_stack_frame_size(&self) -> u32 {
        //     if self.activations_.is_empty() {
        //         0
        //     } else {
        //         self.activations_.last().unwrap().current_stack_frame_size()
        //     }
        // }

        pub fn raise_exception(&mut self, isolate: Isolate, message: MessageTemplate) {
            // TODO: Implement exception raising. Requires interacting with Isolate to throw JS exceptions.
            // isolate.Throw(message);
        }

        fn finish(&mut self) {
            self.state_ = State::FINISHED;
        }

        #[inline]
        fn stack_mem(&self) -> *mut u8 {
            self.stack_mem_
        }
    }

    /// State machine for a WasmInterpreterThread
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum State {
        STOPPED,
        RUNNING,
        FINISHED,
        TRAPPED,
        EH_UNWINDING,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ExceptionHandlingResult {
        HANDLED,
        UNWOUND,
    }

    pub struct TrapStatus {
        pub trap_function_index: i32,
        pub trap_pc: i32,
    }

    impl TrapStatus {
        pub fn new(trap_function_index: i32, trap_pc: i32) -> Self {
            TrapStatus {
                trap_function_index,
                trap_pc,
            }
        }
    }
    
    pub struct WasmInterpreterStackEntry {
        // Placeholder for stack trace entry data
    }

    impl WasmInterpreterStackEntry {
        pub fn new() -> Self {
            WasmInterpreterStackEntry {}
        }
    }

    pub struct Activation {
        thread_: *mut WasmInterpreterThread,
        wasm_runtime_: *mut WasmInterpreterRuntime,
        frame_pointer_: Address,
        current_frame_size_: u32,
        current_ref_stack_fp_: u32,
        current_ref_stack_frame_size_: u32,
        current_fp_: *mut u8,
        current_frame_state_: FrameState,
        trap_stack_trace_: Option<Box<Vec<WasmInterpreterStackEntry>>>,

        // V8_ENABLE_DRUMBRAKE_TRACING
        // current_stack_start_: u32,
        // current_stack_size_: u32,
    }

    impl Activation {
        pub fn new(
            thread: *mut WasmInterpreterThread,
            wasm_runtime: *mut WasmInterpreterRuntime,
            frame_pointer: Address,
            start_fp: *mut u8,
            callee_frame_state: &FrameState,
        ) -> Self {
            Activation {
                thread_: thread,
                wasm_runtime_: wasm_runtime,
                frame_pointer_: frame_pointer,
                current_frame_size_: 0,
                current_ref_stack_fp_: 0,
                current_ref_stack_frame_size_: 0,
                current_fp_: start_fp,
                current_frame_state_: FrameState {
                    ..(*callee_frame_state)
                },
                trap_stack_trace_: None,
                // V8_ENABLE_DRUMBRAKE_TRACING
                // current_stack_start_: callee_frame_state.current_stack_start_args_ + thread.current_stack_frame_size(),
                // current_stack_size_: 0,
            }
        }

        pub fn thread(&self) -> *mut WasmInterpreterThread {
            self.