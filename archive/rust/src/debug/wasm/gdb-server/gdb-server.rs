// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Replace placeholders with actual crate imports and implementations
//       when available

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::thread;
use std::time::Duration;
//use std::future::Future;

const K_MAX_WASM_CALL_STACK: u32 = 20;

/// A TaskRunner is an object that runs posted tasks (in the form of closure
/// objects). Tasks are queued and run, in order, in the thread where the
/// TaskRunner::RunMessageLoop() is called.
pub struct TaskRunner {
    process_queue_semaphore: Arc<(Mutex<u32>, Condvar)>, // Simulate Semaphore with Mutex and Condvar
    queue: Arc<LockedQueue<Box<dyn FnOnce() + Send + 'static>>>,
    nested_loop_count: Mutex<i32>,
    is_terminated: AtomicBool,
}

impl TaskRunner {
    pub fn new() -> Self {
        TaskRunner {
            process_queue_semaphore: Arc::new((Mutex::new(0), Condvar::new())),
            queue: Arc::new(LockedQueue::new()),
            nested_loop_count: Mutex::new(0),
            is_terminated: AtomicBool::new(false),
        }
    }

    /// Starts the task runner. All tasks posted are run, in order, in the thread
    /// that calls this function.
    pub fn run(&self) {
        self.is_terminated.store(false, Ordering::Relaxed);
        let mut loop_number = self.nested_loop_count.lock().unwrap();
        *loop_number += 1;
        let current_loop_number = *loop_number;
        drop(loop_number); // Release the lock

        while *self.nested_loop_count.lock().unwrap() == current_loop_number && !self.is_terminated.load(Ordering::Relaxed) {
            if let Some(task) = self.get_next() {
                task();
            }
        }
    }

    /// Terminates the task runner. Tasks that are still pending in the queue are
    /// not discarded and will be executed when the task runner is restarted.
    pub fn terminate(&self) {
        let mut loop_count = self.nested_loop_count.lock().unwrap();
        assert!(*loop_count > 0);
        *loop_count -= 1;
        drop(loop_count);

        self.is_terminated.store(true, Ordering::Relaxed);
        self.process_queue_semaphore.1.notify_one();
    }

    /// Posts a task to the task runner, to be executed in the task runner thread.
    pub fn append<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.queue.enqueue(Box::new(task));
        let &(ref lock, ref cvar) = &*self.process_queue_semaphore;
        let mut guard = lock.lock().unwrap();
        *guard += 1;
        cvar.notify_one();
    }

    fn get_next(&self) -> Option<Box<dyn FnOnce() + Send + 'static>> {
        loop {
            if self.is_terminated.load(Ordering::Relaxed) {
                return None;
            }
            if self.queue.is_empty() {
                let &(ref lock, ref cvar) = &*self.process_queue_semaphore;
                let mut guard = lock.lock().unwrap();
                while *guard == 0 && !self.is_terminated.load(Ordering::Relaxed){
                    guard = cvar.wait(guard).unwrap();
                }
                *guard -= 1;
                drop(guard);

            }

            if let Some(task) = self.queue.dequeue() {
                return Some(task);
            }
        }
    }
}

pub struct LockedQueue<T> {
    queue: Mutex<std::collections::VecDeque<T>>,
}

impl<T> LockedQueue<T> {
    pub fn new() -> Self {
        LockedQueue {
            queue: Mutex::new(std::collections::VecDeque::new()),
        }
    }

    pub fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WasmAddr {
    module_id: u32,
    offset: u32,
}

impl WasmAddr {
    pub fn new(module_id: u32, offset: u32) -> Self {
        WasmAddr { module_id, offset }
    }

    pub fn module_id(&self) -> u32 {
        self.module_id
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

// Placeholder for the v8_flags structure
struct V8Flags {
    wasm_gdb_remote: bool,
    wasm_pause_waiting_for_debugger: bool,
}

// Assuming a global v8_flags instance
static mut V8_FLAGS: V8Flags = V8Flags {
    wasm_gdb_remote: false,
    wasm_pause_waiting_for_debugger: false,
};

// Macro to access v8_flags (unsafe)
macro_rules! v8_flags {
    () => {
        unsafe { &V8_FLAGS }
    };
}

pub struct GdbServer {
    task_runner: Box<TaskRunner>,
    thread_: Option<GdbServerThread>,
    isolate_delegates_: Mutex<std::collections::HashMap<usize, DebugDelegate>>, // Isolate* -> DebugDelegate
    scripts_: Mutex<std::collections::HashMap<u32, WasmModuleDebug>>,      // module_id -> WasmModuleDebug
    breakpoints_: Mutex<std::collections::HashMap<WasmAddr, i32>>,       // wasm_addr_t -> breakpoint_id
    has_module_list_changed_: AtomicBool,
}

impl GdbServer {
    pub fn new() -> Self {
        GdbServer {
            task_runner: Box::new(TaskRunner::new()),
            thread_: None,
            isolate_delegates_: Mutex::new(std::collections::HashMap::new()),
            scripts_: Mutex::new(std::collections::HashMap::new()),
            breakpoints_: Mutex::new(std::collections::HashMap::new()),
            has_module_list_changed_: AtomicBool::new(false),
        }
    }

    /// Runs a synchronous task in the isolate thread.
    fn run_sync_task<F>(&self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.task_runner.append(callback);
        self.task_runner.run();
    }

    pub fn create() -> Option<Box<GdbServer>> {
        if !v8_flags!().wasm_gdb_remote {
            return None;
        }

        let mut gdb_server = Box::new(GdbServer::new());

        // Spawns the GDB-stub thread where all the communication with the debugger
        // happens.
        gdb_server.thread_ = Some(GdbServerThread::new(gdb_server.as_mut()));
        if !gdb_server.thread_.as_mut().unwrap().start_and_initialize() {
            eprintln!(
                "Cannot initialize thread, GDB-remote debugging will be disabled."
            );
            return None;
        }
        Some(gdb_server)
    }
    pub fn run_message_loop_on_pause(&self) {
        self.task_runner.run();
    }

    pub fn quit_message_loop_on_pause(&self) {
        self.task_runner.terminate();
    }

    pub fn get_loaded_modules(&self, clear_module_list_changed_flag: bool) -> Vec<WasmModuleInfo> {
        // Executed in the GDBServerThread.
        let modules: Arc<Mutex<Vec<WasmModuleInfo>>> = Arc::new(Mutex::new(Vec::new()));
        let modules_clone = modules.clone();

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let scripts = self.scripts_.lock().unwrap();
            let mut modules = modules_clone.lock().unwrap();
            for (&module_id, module_debug) in scripts.iter() {
                modules.push(WasmModuleInfo {
                    module_id,
                    module_name: module_debug.get_module_name().clone(),
                });
            }

            if clear_module_list_changed_flag {
                self.has_module_list_changed_.store(false, Ordering::Relaxed);
            }
        });

        let modules = Arc::try_unwrap(modules).unwrap().into_inner().unwrap();
        modules
    }

    pub fn get_module_debug_handler(&self, module_id: u32) -> Option<WasmModuleDebug> {
        // Always executed in the isolate thread.
        let scripts = self.scripts_.lock().unwrap();
        scripts.get(&module_id).cloned()
    }

    pub fn get_wasm_global(
        &self,
        frame_index: u32,
        index: u32,
        buffer: &mut [u8],
        buffer_size: u32,
        size: &mut u32,
    ) -> bool {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let result_clone = result.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;
        let size_ptr = size as *mut u32;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut result = result_clone.lock().unwrap();
            // TODO: Implement GetTarget and its methods
            //*result = WasmModuleDebug::get_wasm_global(
            //    GetTarget().get_current_isolate(),
            //    frame_index,
            //    index,
            //    buffer_ptr,
            //    buffer_size,
            //    size_ptr,
            //);
            *result = false;
        });

        *result.lock().unwrap()
    }

    pub fn get_wasm_local(
        &self,
        frame_index: u32,
        index: u32,
        buffer: &mut [u8],
        buffer_size: u32,
        size: &mut u32,
    ) -> bool {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let result_clone = result.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;
        let size_ptr = size as *mut u32;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut result = result_clone.lock().unwrap();
            // TODO: Implement GetTarget and its methods
            //*result = WasmModuleDebug::get_wasm_local(
            //    GetTarget().get_current_isolate(),
            //    frame_index,
            //    index,
            //    buffer_ptr,
            //    buffer_size,
            //    size_ptr,
            //);
            *result = false;
        });

        *result.lock().unwrap()
    }

    pub fn get_wasm_stack_value(
        &self,
        frame_index: u32,
        index: u32,
        buffer: &mut [u8],
        buffer_size: u32,
        size: &mut u32,
    ) -> bool {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let result_clone = result.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;
        let size_ptr = size as *mut u32;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut result = result_clone.lock().unwrap();
            // TODO: Implement GetTarget and its methods
            //*result = WasmModuleDebug::get_wasm_stack_value(
            //    GetTarget().get_current_isolate(),
            //    frame_index,
            //    index,
            //    buffer_ptr,
            //    buffer_size,
            //    size_ptr,
            //);
            *result = false;
        });

        *result.lock().unwrap()
    }

    pub fn get_wasm_memory(
        &self,
        module_id: u32,
        offset: u32,
        buffer: &mut [u8],
        size: u32,
    ) -> u32 {
        // Executed in the GDBServerThread.
        let bytes_read: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let bytes_read_clone = bytes_read.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut bytes_read = bytes_read_clone.lock().unwrap();
            if let Some(module_debug) = self.get_module_debug_handler(module_id) {
                // TODO: Implement GetTarget and its methods
                //*bytes_read = module_debug.get_wasm_memory(
                //    GetTarget().get_current_isolate(),
                //    offset,
                //    buffer_ptr,
                //    size,
                //);
                *bytes_read = 0;
            }
        });

        *bytes_read.lock().unwrap()
    }

    pub fn get_wasm_data(
        &self,
        module_id: u32,
        offset: u32,
        buffer: &mut [u8],
        size: u32,
    ) -> u32 {
        // Executed in the GDBServerThread.
        let bytes_read: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let bytes_read_clone = bytes_read.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut bytes_read = bytes_read_clone.lock().unwrap();
            if let Some(module_debug) = self.get_module_debug_handler(module_id) {
                // TODO: Implement GetTarget and its methods
                //*bytes_read = module_debug.get_wasm_data(
                //    GetTarget().get_current_isolate(),
                //    offset,
                //    buffer_ptr,
                //    size,
                //);
                *bytes_read = 0;
            }
        });

        *bytes_read.lock().unwrap()
    }

    pub fn get_wasm_module_bytes(
        &self,
        wasm_addr: WasmAddr,
        buffer: &mut [u8],
        size: u32,
    ) -> u32 {
        // Executed in the GDBServerThread.
        let bytes_read: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let bytes_read_clone = bytes_read.clone();

        let buffer_ptr = buffer as *mut [u8] as *mut u8;

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut bytes_read = bytes_read_clone.lock().unwrap();
            if let Some(module_debug) = self.get_module_debug_handler(wasm_addr.module_id()) {
                *bytes_read = module_debug.get_wasm_module_bytes(wasm_addr, buffer_ptr, size);
            }
        });

        *bytes_read.lock().unwrap()
    }

    pub fn add_breakpoint(&self, wasm_module_id: u32, offset: u32) -> bool {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let result_clone = result.clone();

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut result = result_clone.lock().unwrap();
            if let Some(mut module_debug) = self.get_module_debug_handler(wasm_module_id) {
                let mut breakpoint_id = 0;
                if module_debug.add_breakpoint(offset, &mut breakpoint_id) {
                    let mut breakpoints = self.breakpoints_.lock().unwrap();
                    breakpoints.insert(WasmAddr::new(wasm_module_id, offset), breakpoint_id);
                    *result = true;
                }
            }
        });

        *result.lock().unwrap()
    }

    pub fn remove_breakpoint(&self, wasm_module_id: u32, offset: u32) -> bool {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let result_clone = result.clone();

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut breakpoints = self.breakpoints_.lock().unwrap();
            let addr = WasmAddr::new(wasm_module_id, offset);
            if let Some(breakpoint_id) = breakpoints.remove(&addr) {
                let mut result = result_clone.lock().unwrap();
                if let Some(mut module_debug) = self.get_module_debug_handler(wasm_module_id) {
                    module_debug.remove_breakpoint(offset, breakpoint_id);
                    *result = true;
                }
            }
        });

        *result.lock().unwrap()
    }

    pub fn get_wasm_call_stack(&self) -> Vec<WasmAddr> {
        // Executed in the GDBServerThread.
        let result: Arc<Mutex<Vec<WasmAddr>>> = Arc::new(Mutex::new(Vec::new()));
        let result_clone = result.clone();

        self.run_sync_task(move || {
            // Executed in the isolate thread.
            let mut result = result_clone.lock().unwrap();
            // TODO: Implement GetTarget and its methods
            //*result = GetTarget().get_call_stack();
        });

        Arc::try_unwrap(result).unwrap().into_inner().unwrap()
    }

    pub fn add_isolate(&self, isolate_id: usize) { //Isolate* represented as usize
        // Executed in the isolate thread.
        let mut isolate_delegates = self.isolate_delegates_.lock().unwrap();
        if !isolate_delegates.contains_key(&isolate_id) {
            isolate_delegates.insert(isolate_id, DebugDelegate::new(isolate_id, self));
        }
    }

    pub fn remove_isolate(&self, isolate_id: usize) { //Isolate* represented as usize
        // Executed in the isolate thread.
        let mut isolate_delegates = self.isolate_delegates_.lock().unwrap();
        if let Some(_) = isolate_delegates.remove(&isolate_id) {
            let mut scripts = self.scripts_.lock().unwrap();
            scripts.retain(|_, module_debug| module_debug.isolate_id != isolate_id);

            self.has_module_list_changed_.store(true, Ordering::Relaxed);
        }
    }

    pub fn suspend(&self) {
        // Executed in the GDBServerThread.
        let isolate_delegates = self.isolate_delegates_.lock().unwrap();
        if let Some((isolate_id, _)) = isolate_delegates.iter().next() {
           // TODO: Implement the v8 isolate interrupt
           // let v8_isolate = *isolate_id as *mut v8::Isolate;

           // unsafe {
           //      v8::Isolate::request_interrupt(
           //         v8_isolate,
           //          Some(|isolate, gdb_server_ptr|{
           //              // Executed in the isolate thread.
           //              // TODO: Implement the remaining part of the callback
           //          }),
           //          std::ptr::null_mut()
           //      )
           // }
        }
    }

    pub fn prepare_step(&self) {
        // Executed in the GDBServerThread.
        // TODO: Implement GetTarget and its methods
        // let pc = GetTarget().get_current_pc();
        // self.run_sync_task(|| {
        //     // Executed in the isolate thread.
        //     if let Some(mut module_debug) = self.get_module_debug_handler(pc.module_id()) {
        //         module_debug.prepare_step();
        //     }
        // });
    }

    pub fn add_wasm_module(&self, module_id: u32, wasm_script: WasmScript) {
        // Executed in the isolate thread.
        // TODO: Implement the checks for Script::Type::kWasm and Utils::OpenHandle
        // TODO: Implement the conversion from WasmScript to a type usable in Rust
        let isolate_id = wasm_script.get_isolate_id(); // v8::Isolate* represented as usize
        let mut scripts = self.scripts_.lock().unwrap();
        scripts.insert(module_id, WasmModuleDebug::new(isolate_id, wasm_script));
        self.has_module_list_changed_.store(true, Ordering::Relaxed);

        if v8_flags!().wasm_pause_waiting_for_debugger && scripts.len() == 1 {
            eprintln!("Paused, waiting for a debugger to attach...");
            self.suspend();
        }
    }

    // TODO: Implement GetTarget
    // fn get_target(&self) -> &Target {
    //     self.thread_.as_ref().unwrap().get_target()
    // }
}

impl Drop for GdbServer {
    fn drop(&mut self) {
        // All Isolates have been deregistered.
        assert!(self.isolate_delegates_.lock().unwrap().is_empty());

        if let Some(thread_) = self.thread_.take() {
            // Waits for the GDB-stub thread to terminate.
            thread_.stop();
            thread_.join().unwrap();
        }
    }
}

#[derive(Clone)]
pub struct WasmScript {
    isolate_id: usize, // v8::Isolate* represented as usize
}

impl WasmScript {
    pub fn get_isolate_id(&self) -> usize {
        self.isolate_id
    }
}

pub struct DebugDelegate {
    isolate_id: usize, //Isolate* represented as usize
    id: u32,
    gdb_server: *mut GdbServer, // Raw pointer to avoid lifetime issues, requires careful management
}

impl DebugDelegate {
    // static
    static ID_S: AtomicU32 = AtomicU32::new(0);

    pub fn new(isolate_id: usize, gdb_server: &GdbServer) -> Self {
        let id = DebugDelegate::ID_S.fetch_add(1, Ordering::Relaxed);

        // Register the delegate
        // TODO: Implement the delegate registration and function calls
        //isolate.debug().SetDebugDelegate(this);
        //v8::debug::EnterDebuggingForIsolate((v8::Isolate*)isolate_);
        //v8::debug::ChangeBreakOnException((v8::Isolate*)isolate_,
        //                                   v8::debug::BreakOnUncaughtException);

        DebugDelegate {
            isolate_id,
            id,
            gdb_server: gdb_server as *const _ as *mut _, // Convert to raw pointer
        }
    }

    pub fn script_compiled(&mut self, script: WasmScript, is_live_edited: bool, has_compile_error: bool) {
        // Executed in the isolate thread.
        //if script.IsWasm() {
        //  DCHECK_EQ(reinterpret_cast<v8::Isolate*>(isolate_), script->GetIsolate());
        unsafe {
            (*self.gdb_server).add_wasm_module(get_module_id(0), script); // Placeholder for GetModuleId
        }
        //}
    }

    pub fn break_program_requested(
        &mut self,
        paused_context: usize, //v8::Context represented by usize,
        inspector_break_points_hit: Vec<i32>, //debug::BreakpointId represented as i32
        break_reasons: u32 //v8::debug::BreakReasons represented as u32
    ) {
        // TODO: Implement GetTarget and its methods
        //gdb_server_->GetTarget().OnProgramBreak(
        //    isolate_, WasmModuleDebug::GetCallStack(id_, isolate_));
        unsafe {
            (*self.gdb_server).run_message_loop_on_pause();
        }

    }

    pub fn exception_thrown(
        &mut self,
        paused_context: usize, //v8::Context represented by usize,
        exception: usize, //v8::Value represented by usize,
        promise: usize,  //v8::Value represented by usize,
        is_uncaught: bool,
        exception_type: u32  //debug::ExceptionType represented by u32
    ) {
        if exception_type == 0 && is_uncaught { //v8::debug::kException == 0
            //gdb_server_->GetTarget().OnException(
            //    isolate_, WasmModuleDebug::GetCallStack(id_, isolate_));
             unsafe {
                (*self.gdb_server).run_message_loop_on_pause();
            }
        }
    }

    pub fn is_function_blackboxed(
        &self,
        script: WasmScript, //debug::Script represented by WasmScript,
        start: usize, //debug::Location represented by usize
        end: usize //debug::Location represented by usize
    ) -> bool {
        false
    }
}

impl Drop for DebugDelegate {
    fn drop(&mut self) {
        // Deregister the delegate
        // TODO: Implement the delegate deregistration
        //isolate_->debug()->SetDebugDelegate(nullptr);
    }
}

fn get_module_id(_script_id: i32) -> u32 {
    // Placeholder for GetModuleId implementation.
    // This function should map a script ID to a module ID.
    0
}

#[derive(Clone)]
pub struct WasmModuleDebug {
    isolate_id: usize, //Isolate* represented as usize
    module_name: String,
    // Add other relevant fields for debugging
}

impl WasmModuleDebug {
    pub fn new(isolate_id: usize, _wasm_script: WasmScript) -> Self {
        WasmModuleDebug {
            isolate_id,
            module_name: "PlaceholderModule".to_string(),
            // Initialize other fields as needed
        }
    }

    pub fn get_module_name(&self) -> &String {
        &self.module_name
    }

    pub fn add_breakpoint(&mut self, _offset: u32, breakpoint_id: &mut i32) -> bool {
        // Placeholder for AddBreakpoint implementation
        *breakpoint_id = 123; // Assign a dummy breakpoint ID
        true
    }

    pub fn remove_breakpoint(&mut self, _offset: u32, _breakpoint_id: i32) {
        // Placeholder for RemoveBreakpoint implementation
    }

    pub fn get_wasm_memory(&self, _isolate: usize, _offset: u32, _buffer: *mut u8, _size: u32) -> u32 {
        // Placeholder for GetWasmMemory implementation
        0
    }

    pub fn get_wasm_data(&self, _isolate: usize, _offset: u32, _buffer: *mut u8, _size: u32) -> u32 {
        // Placeholder for GetWasmData implementation
        0
    }

    pub fn get_wasm_module_bytes(&self, _wasm_addr: WasmAddr, _buffer: *mut u8, _size: u32) -> u32 {
        // Placeholder for GetWasmModuleBytes implementation
        0
    }
}

#[derive(Clone)]
pub struct WasmModuleInfo {
    module_id: u32,
    module_name: String,
}

struct GdbServerThread {
    gdb_server: *mut GdbServer,
    thread: Option<thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
    //target: Arc<Mutex<Target>>,
}

impl GdbServerThread {
    fn new(gdb_server: *mut GdbServer) -> Self {
        GdbServerThread {
            gdb_server,
            thread: None,
            is_running: Arc::new(AtomicBool::new(false)),
            //target: Arc::new(Mutex::new(Target::new())),
        }
    }

    fn start_and_initialize(&mut self) -> bool {
        self.is_running.store(true, Ordering::Relaxed);
        let is_running = self.is_running.clone();
        let gdb_server_ptr = self.gdb_server;

        self.thread = Some(thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {
                // TODO: Implement GDB server thread logic here
                // Access GdbServer using the raw pointer:
                // unsafe { (*gdb_server_ptr).some_method(); }
                thread::sleep(Duration::from_millis(100));
            }
        }));

        // Initialization logic here (if any)
        true
    }

    fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    fn join(self) -> Result<(), ()> {
        match self.thread {
            Some(thread) => {
                thread.join().unwrap();
                Ok(())
            }
            None => Err(()),
        }
    }
    // TODO: Implement the GetTarget method.
    //fn get_target(&self) -> &Target {
    //    &self.target.lock().unwrap()
    //}
}