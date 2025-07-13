// Converted from V8 C++ source files:
// Header: perfetto-logger.h
// Implementation: perfetto-logger.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod perfetto_logger {
    use std::collections::HashMap;
    use std::sync::{Mutex, MutexGuard};

    pub struct Isolate;
    pub struct AbstractCode;
    pub struct Name;
    pub struct SharedFunctionInfo;
    pub struct String;
    pub struct InstructionStream;
    pub struct BytecodeArray;
    pub struct Code;
    pub struct Address;
    pub struct DeoptimizeKind;
    pub struct RegExpFlags;
    pub struct CodeTag;

    pub mod wasm {
        pub struct WasmCode;
        pub struct WasmName;
    }

    pub trait LogEventListener {
        fn code_create_event_name(&mut self, tag: CodeTag, code: *mut AbstractCode, name: &str);
        fn code_create_event_name_handle(&mut self, tag: CodeTag, code: *mut AbstractCode, name: *mut Name);
        fn code_create_event_shared_name(
            &mut self,
            tag: CodeTag,
            code: *mut AbstractCode,
            shared: *mut SharedFunctionInfo,
            script_name: *mut Name,
        );
        fn code_create_event_shared_name_line_column(
            &mut self,
            tag: CodeTag,
            code: *mut AbstractCode,
            shared: *mut SharedFunctionInfo,
            script_name: *mut Name,
            line: i32,
            column: i32,
        );
        fn code_create_event_wasm(
            &mut self,
            tag: CodeTag,
            code: *const wasm::WasmCode,
            name: wasm::WasmName,
            source_url: &str,
            code_offset: i32,
            script_id: i32,
        );
        fn callback_event(&mut self, name: *mut Name, entry_point: Address);
        fn getter_callback_event(&mut self, name: *mut Name, entry_point: Address);
        fn setter_callback_event(&mut self, name: *mut Name, entry_point: Address);
        fn regexp_code_create_event(&mut self, code: *mut AbstractCode, source: *mut String, flags: RegExpFlags);
        fn code_move_event(&mut self, from: *mut InstructionStream, to: *mut InstructionStream);
        fn bytecode_move_event(&mut self, from: *mut BytecodeArray, to: *mut BytecodeArray);
        fn shared_function_info_move_event(&mut self, from: Address, to: Address);
        fn native_context_move_event(&mut self, from: Address, to: Address);
        fn code_moving_gc_event(&mut self);
        fn code_disable_opt_event(&mut self, code: *mut AbstractCode, shared: *mut SharedFunctionInfo);
        fn code_deopt_event(&mut self, code: *mut Code, kind: DeoptimizeKind, pc: Address, fp_to_sp_delta: i32);
        fn code_dependency_change_event(
            &mut self,
            code: *mut Code,
            shared: *mut SharedFunctionInfo,
            reason: &str,
        );
        fn weak_code_clear_event(&mut self);
        fn is_listening_to_code_events(&self) -> bool;
    }

    pub struct PerfettoLogger {
        isolate_: *mut Isolate,
    }

    impl PerfettoLogger {
        pub fn register_isolate(isolate: *mut Isolate) {
            IsolateRegistry::get_instance().register(isolate);
        }

        pub fn unregister_isolate(isolate: *mut Isolate) {
            IsolateRegistry::get_instance().unregister(isolate);
        }

        pub fn on_code_data_source_start() {
            IsolateRegistry::get_instance().on_code_data_source_start();
        }

        pub fn on_code_data_source_stop() {
            IsolateRegistry::get_instance().on_code_data_source_stop();
        }

        pub fn new(isolate: *mut Isolate) -> Self {
            PerfettoLogger { isolate_: isolate }
        }

        fn log_existing_code(&mut self) {
            println!("PerfettoLogger::LogExistingCode");
        }
    }

    impl LogEventListener for PerfettoLogger {
        fn code_create_event_name(&mut self, tag: CodeTag, code: *mut AbstractCode, name: &str) {
            println!("PerfettoLogger::CodeCreateEvent (name): tag={:?}, code={:?}, name={}", tag, code, name);
        }

        fn code_create_event_name_handle(&mut self, tag: CodeTag, code: *mut AbstractCode, name: *mut Name) {
            println!("PerfettoLogger::CodeCreateEvent (Name handle): tag={:?}, code={:?}, name={:?}", tag, code, name);
        }

        fn code_create_event_shared_name(
            &mut self,
            tag: CodeTag,
            code: *mut AbstractCode,
            shared: *mut SharedFunctionInfo,
            script_name: *mut Name,
        ) {
            println!(
                "PerfettoLogger::CodeCreateEvent (SharedFunctionInfo + Name): tag={:?}, code={:?}, shared={:?}, script_name={:?}",
                tag, code, shared, script_name
            );
        }

        fn code_create_event_shared_name_line_column(
            &mut self,
            tag: CodeTag,
            code: *mut AbstractCode,
            shared: *mut SharedFunctionInfo,
            script_name: *mut Name,
            line: i32,
            column: i32,
        ) {
            println!(
                "PerfettoLogger::CodeCreateEvent (SharedFunctionInfo + Name + Line + Column): tag={:?}, code={:?}, shared={:?}, script_name={:?}, line={}, column={}",
                tag, code, shared, script_name, line, column
            );
        }

        fn code_create_event_wasm(
            &mut self,
            tag: CodeTag,
            code: *const wasm::WasmCode,
            name: wasm::WasmName,
            source_url: &str,
            code_offset: i32,
            script_id: i32,
        ) {
            println!(
                "PerfettoLogger::CodeCreateEvent (WASM): tag={:?}, code={:?}, name={:?}, source_url={}, code_offset={}, script_id={}",
                tag, code, name, source_url, code_offset, script_id
            );
        }

        fn callback_event(&mut self, name: *mut Name, entry_point: Address) {
            println!("PerfettoLogger::CallbackEvent: name={:?}, entry_point={:?}", name, entry_point);
        }

        fn getter_callback_event(&mut self, name: *mut Name, entry_point: Address) {
            println!("PerfettoLogger::GetterCallbackEvent: name={:?}, entry_point={:?}", name, entry_point);
        }

        fn setter_callback_event(&mut self, name: *mut Name, entry_point: Address) {
            println!("PerfettoLogger::SetterCallbackEvent: name={:?}, entry_point={:?}", name, entry_point);
        }

        fn regexp_code_create_event(&mut self, code: *mut AbstractCode, source: *mut String, flags: RegExpFlags) {
            println!(
                "PerfettoLogger::RegExpCodeCreateEvent: code={:?}, source={:?}, flags={:?}",
                code, source, flags
            );
        }

        fn code_move_event(&mut self, from: *mut InstructionStream, to: *mut InstructionStream) {
            println!("PerfettoLogger::CodeMoveEvent: from={:?}, to={:?}", from, to);
        }

        fn bytecode_move_event(&mut self, from: *mut BytecodeArray, to: *mut BytecodeArray) {
            println!("PerfettoLogger::BytecodeMoveEvent: from={:?}, to={:?}", from, to);
        }

        fn shared_function_info_move_event(&mut self, from: Address, to: Address) {
            println!("PerfettoLogger::SharedFunctionInfoMoveEvent: from={:?}, to={:?}", from, to);
        }

        fn native_context_move_event(&mut self, from: Address, to: Address) {
            println!("PerfettoLogger::NativeContextMoveEvent: from={:?}, to={:?}", from, to);
        }

        fn code_moving_gc_event(&mut self) {
            println!("PerfettoLogger::CodeMovingGCEvent");
        }

        fn code_disable_opt_event(&mut self, code: *mut AbstractCode, shared: *mut SharedFunctionInfo) {
            println!(
                "PerfettoLogger::CodeDisableOptEvent: code={:?}, shared={:?}",
                code, shared
            );
        }

        fn code_deopt_event(&mut self, code: *mut Code, kind: DeoptimizeKind, pc: Address, fp_to_sp_delta: i32) {
            println!(
                "PerfettoLogger::CodeDeoptEvent: code={:?}, kind={:?}, pc={:?}, fp_to_sp_delta={}",
                code, kind, pc, fp_to_sp_delta
            );
        }

        fn code_dependency_change_event(
            &mut self,
            code: *mut Code,
            shared: *mut SharedFunctionInfo,
            reason: &str,
        ) {
            println!(
                "PerfettoLogger::CodeDependencyChangeEvent: code={:?}, shared={:?}, reason={}",
                code, shared, reason
            );
        }

        fn weak_code_clear_event(&mut self) {
            println!("PerfettoLogger::WeakCodeClearEvent");
        }

        fn is_listening_to_code_events(&self) -> bool {
            true
        }
    }

    struct IsolateRegistry {
        mutex_: Mutex<()>,
        num_active_data_sources_: i32,
        isolates_: HashMap<*mut Isolate, Box<dyn LogEventListener>>,
    }

    impl IsolateRegistry {
        fn get_instance() -> &'static mut IsolateRegistry {
            static mut INSTANCE: *mut IsolateRegistry = std::ptr::null_mut();
            static INIT: std::sync::Once = std::sync::Once::new();

            INIT.call_once(|| {
                let instance = IsolateRegistry {
                    mutex_: Mutex::new(()),
                    num_active_data_sources_: 0,
                    isolates_: HashMap::new(),
                };
                unsafe {
                    INSTANCE = Box::into_raw(Box::new(instance));
                }
            });

            unsafe { &mut *INSTANCE }
        }

        fn register(&mut self, isolate: *mut Isolate) {
            let logger: Box<dyn LogEventListener> = Box::new(PerfettoLogger::new(isolate));

            let _lock = self.mutex_.lock().unwrap();

            if self.num_active_data_sources_ != 0 {
                println!("Adding listener to isolate (TODO)");
            }

            self.isolates_.insert(isolate, logger);
        }

        fn unregister(&mut self, isolate: *mut Isolate) {
            let _lock = self.mutex_.lock().unwrap();

            if self.num_active_data_sources_ != 0 {
                println!("Removing listener from isolate (TODO)");
            }

            self.isolates_.remove(&isolate);
        }

        fn on_code_data_source_start(&mut self) {
            let _lock = self.mutex_.lock().unwrap();
            self.num_active_data_sources_ += 1;
            if self.num_active_data_sources_ == 1 {
                self.start_logging();
            }
            self.log_existing_code_for_all_isolates();
        }

        fn on_code_data_source_stop(&mut self) {
            let _lock = self.mutex_.lock().unwrap();
            self.num_active_data_sources_ -= 1;
            if self.num_active_data_sources_ == 0 {
                self.stop_logging();
            }
        }

        fn start_logging(&mut self) {
            println!("Starting logging for all isolates (TODO)");
        }

        fn stop_logging(&mut self) {
            println!("Stopping logging for all isolates (TODO)");
        }

        fn log_existing_code_for_all_isolates(&mut self) {
            println!("Logging existing code for all isolates (TODO)");
            for (isolate, listener) in &mut self.isolates_ {
                let logger = &mut **listener as *mut dyn LogEventListener;
                println!("Requesting interrupt for isolate {:?}", isolate);

                unsafe {
                    let logger = logger as *mut PerfettoLogger;
                    if let Some(logger) = logger.as_mut() {
                        logger.log_existing_code();
                    }
                }
            }
        }
    }
}
