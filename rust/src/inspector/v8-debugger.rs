// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_debugger {
    use std::any::Any;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::rc::Rc;
    use std::sync::{Arc, Weak, Mutex};

    // Placeholder for v8-inspector.h functionality
    pub struct V8Inspector;

    // Placeholder for v8-inspector.h functionality
    pub trait V8InspectorClient {
        fn base(&self) -> &V8InspectorBase;
    }

    pub struct V8InspectorBase {
        pub dummy: i32, // Add some fields to make struct non-zero sized, if necessary.
    }

    impl V8InspectorBase {
        pub fn new() -> Self {
            V8InspectorBase{dummy: 0}
        }
    }

    // Placeholder for protocol::Debugger.h
    pub mod protocol {
        pub mod Debugger {
            pub struct Location;
        }

        pub mod Runtime {
            pub type TerminateExecutionCallback = Box<dyn FnOnce() + Send + Sync>;

            pub trait Backend {
                fn terminate_execution(&mut self, callback: TerminateExecutionCallback);
            }
        }
        pub struct Response;
    }

    // Placeholder for src/base/macros.h
    macro_rules! delete {
        ($x:expr) => {
            drop($x);
        };
    }

    // Placeholder for String16
    pub type String16 = String;

    // Placeholder for StringView
    pub type StringView = str;

    // Placeholder for InspectedContext
    pub struct InspectedContext;

    // Placeholder for V8DebuggerId
    pub mod internal {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct V8DebuggerId(pub i32);
    }

    // Placeholder for V8DebuggerScript
    pub struct V8DebuggerScript;

    // Placeholder for AsyncStackTrace
    pub struct AsyncStackTrace;

    // Placeholder for StackFrame
    pub struct StackFrame;

    // Placeholder for V8StackTraceImpl
    pub struct V8StackTraceImpl;

    // Placeholder for V8StackTraceId
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct V8StackTraceId(pub i32);

    // Placeholder for V8DebuggerAgentImpl
    pub struct V8DebuggerAgentImpl;

    // Placeholder for V8RuntimeAgentImpl
    pub struct V8RuntimeAgentImpl;

    // Placeholder for kMaxInt
    pub const K_MAX_INT: i32 = i32::MAX;

    // Placeholder for internal value type
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum V8InternalValueType {
        // Dummy value
        None
    }

    #[derive(Debug, Clone, Copy)]
    pub enum WrapMode {
        kJson,
        kIdOnly,
        kPreview,
        kDeep,
    }

    #[derive(Debug, Clone)]
    pub struct WrapSerializationOptions {
        pub max_depth: i32,
        // v8::Global<v8::Object> additionalParameters;
        pub additional_parameters: Option<()>, // Replace with appropriate type if needed
    }

    impl Default for WrapSerializationOptions {
        fn default() -> Self {
            WrapSerializationOptions {
                max_depth: K_MAX_INT,
                additional_parameters: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct WrapOptions {
        pub mode: WrapMode,
        pub serialization_options: WrapSerializationOptions,
    }

    impl Default for WrapOptions {
        fn default() -> Self {
            WrapOptions {
                mode: WrapMode::kJson,
                serialization_options: WrapSerializationOptions::default(),
            }
        }
    }

    // Mock for v8 namespace types, replace with actual v8 crate usage
    pub mod v8 {
        pub type Isolate = usize;
        pub type Context = usize;
        pub type Value = usize;
        pub type Array = usize;
        pub type Object = usize;
        pub type StackTrace = usize;
        pub type StackFrame = usize;
        pub type Script = usize;

        pub mod debug {
            pub type BreakpointId = i32;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum ExceptionBreakState {
                None,
                All,
                Uncaught,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum BreakReasons {
                Ambiguous,
                Breakpoint,
                Exception,
                DebuggerStatement,
                Step,
                Instrumentation,
                Other,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum ExceptionType {
                kException,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum DebugAsyncActionType {
                AsyncActionType1,
                AsyncActionType2,
            }

            pub struct Location;

            pub enum ActionAfterInstrumentation {
                Continue,
                Break,
            }
        }

        pub type Local<'a, T> = &'a T;
        pub type MaybeLocal<'a, T> = Option<&'a T>;
        pub struct Global<T>(pub T);

        impl<T> Global<T> {
            pub fn new(value: T) -> Self {
                Global(value)
            }
        }
    }

    // Mock for v8::debug::DebugDelegate
    pub trait DebugDelegate {
        fn async_event_occurred(&mut self, _type: v8::debug::DebugAsyncActionType, _id: i32, _is_blackboxed: bool) {}
        fn script_compiled(&mut self, _script: v8::Local<v8::debug::Script>, _is_live_edited: bool, _has_compile_error: bool) {}
        fn break_program_requested(&mut self, _paused_context: v8::Local<v8::Context>, _break_points_hit: &std::vec::Vec<v8::debug::BreakpointId>, _break_reasons: v8::debug::BreakReasons) {}
        fn break_on_instrumentation(&mut self, _paused_context: v8::Local<v8::Context>, _breakpoint_id: v8::debug::BreakpointId) -> v8::debug::ActionAfterInstrumentation { v8::debug::ActionAfterInstrumentation::Continue }
        fn exception_thrown(&mut self, _paused_context: v8::Local<v8::Context>, _exception: v8::Local<v8::Value>, _promise: v8::Local<v8::Value>, _is_uncaught: bool, _exception_type: v8::debug::ExceptionType) {}
        fn is_function_blackboxed(&mut self, _script: v8::Local<v8::debug::Script>, _start: &v8::debug::Location, _end: &v8::debug::Location) -> bool { false }
        fn should_be_skipped(&mut self, _script: v8::Local<v8::debug::Script>, _line: i32, _column: i32) -> bool { false }
        fn breakpoint_condition_evaluated(&mut self, _context: v8::Local<v8::Context>, _breakpoint_id: v8::debug::BreakpointId, _exception_thrown: bool, _exception: v8::Local<v8::Value>) {}
    }

    // Mock for v8::debug::AsyncEventDelegate
    pub trait AsyncEventDelegate {
        fn async_event_occurred(&mut self, _type: v8::debug::DebugAsyncActionType, _id: i32, _is_blackboxed: bool) {}
    }

    pub struct V8Debugger {
        m_isolate: v8::Isolate,
        m_inspector: *mut V8InspectorImpl, // Raw pointer
        m_enable_count: i32,

        m_breakpoints_active_count: i32,
        m_ignore_script_parsed_events_counter: i32,
        m_original_heap_limit: usize,
        m_scheduled_oom_break: bool,
        m_target_context_group_id: i32,
        m_paused_context_group_id: i32,
        m_instrumentation_pause: bool,
        m_requested_pause_after_instrumentation: bool,
        m_continue_to_location_breakpoint_id: i32,
        m_continue_to_location_target_call_frames: String16,
        m_continue_to_location_stack: Option<Box<V8StackTraceImpl>>,

        m_cached_stack_frames: Mutex<HashMap<CachedStackFrameKey, Weak<StackFrame>>>,

        m_async_task_stacks: Mutex<HashMap<*mut dyn Any, Weak<AsyncStackTrace>>>,
        m_recurring_tasks: Mutex<HashSet<*mut dyn Any>>,

        m_max_async_call_stacks: usize,
        m_max_async_call_stack_depth: i32,
        m_max_call_stack_size_to_capture: i32,

        m_current_tasks: Mutex<Vec<*mut dyn Any>>,
        m_current_async_parent: Mutex<Vec<Arc<AsyncStackTrace>>>,
        m_current_external_parent: Mutex<Vec<V8StackTraceId>>,

        m_async_parents: Mutex<HashMap<i32, Weak<AsyncStackTrace>>>,
        m_external_parents: Mutex<VecDeque<(i32, V8StackTraceId)>>,

        m_all_async_stacks: Mutex<Vec<Arc<AsyncStackTrace>>>,

        m_max_async_call_stack_depth_map: Mutex<HashMap<*mut V8DebuggerAgentImpl, i32>>,
        m_max_call_stack_size_to_capture_map: Mutex<HashMap<*mut V8RuntimeAgentImpl, i32>>,
        m_task_with_scheduled_break: *mut dyn Any,

        m_external_async_task_pause_requested: bool,
        m_task_with_scheduled_break_pause_requested: bool,
        m_pause_on_next_call_requested: bool,

        m_pause_on_exceptions_state: v8::debug::ExceptionBreakState,
        m_pause_on_async_call: bool,

        m_stored_stack_traces: Mutex<HashMap<usize, Weak<AsyncStackTrace>>>,
        m_last_stack_trace_id: usize,

        m_context_group_id_to_debugger_id: Mutex<HashMap<i32, internal::V8DebuggerId>>,

        m_terminate_execution_callback: Option<protocol::Runtime::TerminateExecutionCallback>,
        m_terminate_execution_callback_context: v8::Global<v8::Context>,
        m_terminate_execution_reported: bool,
    }

    impl V8Debugger {
        pub fn new(isolate: v8::Isolate, inspector: *mut V8InspectorImpl) -> Self {
            V8Debugger {
                m_isolate: isolate,
                m_inspector: inspector,
                m_enable_count: 0,

                m_breakpoints_active_count: 0,
                m_ignore_script_parsed_events_counter: 0,
                m_original_heap_limit: 0,
                m_scheduled_oom_break: false,
                m_target_context_group_id: 0,
                m_paused_context_group_id: 0,
                m_instrumentation_pause: false,
                m_requested_pause_after_instrumentation: false,
                m_continue_to_location_breakpoint_id: 0,
                m_continue_to_location_target_call_frames: String::new(),
                m_continue_to_location_stack: None,

                m_cached_stack_frames: Mutex::new(HashMap::new()),

                m_async_task_stacks: Mutex::new(HashMap::new()),
                m_recurring_tasks: Mutex::new(HashSet::new()),

                m_max_async_call_stacks: 10,
                m_max_async_call_stack_depth: 0,
                m_max_call_stack_size_to_capture: 0,

                m_current_tasks: Mutex::new(Vec::new()),
                m_current_async_parent: Mutex::new(Vec::new()),
                m_current_external_parent: Mutex::new(Vec::new()),

                m_async_parents: Mutex::new(HashMap::new()),
                m_external_parents: Mutex::new(VecDeque::new()),

                m_all_async_stacks: Mutex::new(Vec::new()),

                m_max_async_call_stack_depth_map: Mutex::new(HashMap::new()),
                m_max_call_stack_size_to_capture_map: Mutex::new(HashMap::new()),
                m_task_with_scheduled_break: std::ptr::null_mut(),

                m_external_async_task_pause_requested: false,
                m_task_with_scheduled_break_pause_requested: false,
                m_pause_on_next_call_requested: false,

                m_pause_on_exceptions_state: v8::debug::ExceptionBreakState::None,
                m_pause_on_async_call: false,

                m_stored_stack_traces: Mutex::new(HashMap::new()),
                m_last_stack_trace_id: 0,

                m_context_group_id_to_debugger_id: Mutex::new(HashMap::new()),

                m_terminate_execution_callback: None,
                m_terminate_execution_callback_context: v8::Global::new(0),
                m_terminate_execution_reported: true,
            }
        }

        pub fn enabled(&self) -> bool {
            self.m_enable_count > 0
        }

        pub fn isolate(&self) -> v8::Isolate {
            self.m_isolate
        }

        pub fn set_breakpoints_active(&mut self, active: bool) {
            if active {
                self.m_breakpoints_active_count += 1;
            } else {
                self.m_breakpoints_active_count -= 1;
            }
        }

        pub fn remove_breakpoint(&mut self, _id: v8::debug::BreakpointId) {
            // Implementation details here
        }

        pub fn get_pause_on_exceptions_state(&self) -> v8::debug::ExceptionBreakState {
            self.m_pause_on_exceptions_state
        }

        pub fn set_pause_on_exceptions_state(&mut self, state: v8::debug::ExceptionBreakState) {
            self.m_pause_on_exceptions_state = state;
        }

        pub fn can_break_program(&self) -> bool {
            true // Placeholder
        }

        pub fn is_in_instrumentation_pause(&self) -> bool {
            self.m_instrumentation_pause
        }

        pub fn break_program(&mut self, target_context_group_id: i32) {
            self.interrupt_and_break(target_context_group_id);
        }

        pub fn interrupt_and_break(&mut self, target_context_group_id: i32) {
            self.m_targetContextGroupId = target_context_group_id;
        }

        pub fn request_pause_after_instrumentation(&mut self) {
            self.m_requested_pause_after_instrumentation = true;
        }

        pub fn continue_program(&mut self, _target_context_group_id: i32, _terminate_on_resume: bool) {
            // Implementation here
        }

        pub fn break_program_on_assert(&mut self, _target_context_group_id: i32) {
            // Implementation here
        }

        pub fn set_pause_on_next_call(&mut self, pause: bool, _target_context_group_id: i32) {
            self.m_pause_on_next_call_requested = pause;
        }

        pub fn step_into_statement(&mut self, _target_context_group_id: i32, _break_on_async_call: bool) {
            // Implementation here
        }

        pub fn step_over_statement(&mut self, _target_context_group_id: i32) {
            // Implementation here
        }

        pub fn step_out_of_function(&mut self, _target_context_group_id: i32) {
            // Implementation here
        }

        pub fn terminate_execution(&mut self, _context: v8::Local<v8::Context>, callback: std::unique_ptr::UniquePtr<protocol::Runtime::TerminateExecutionCallback>) {
            self.m_terminate_execution_callback = callback.into(); // Not ideal, check ownership requirements
        }

        pub fn continue_to_location(
            &mut self,
            _target_context_group_id: i32,
            _script: *mut V8DebuggerScript,
            _location: std::unique_ptr::UniquePtr<protocol::Debugger::Location>,
            _target_call_framess: &String16,
        ) -> protocol::Response {
            protocol::Response {} // Placeholder
        }

        pub fn restart_frame(&mut self, _target_context_group_id: i32, _call_frame_ordinal: i32) -> bool {
            false // Placeholder
        }

        pub fn get_compiled_scripts(
            &self,
            _context_group_id: i32,
            _agent: *mut V8DebuggerAgentImpl,
        ) -> Vec<std::unique_ptr::UniquePtr<V8DebuggerScript>> {
            Vec::new() // Placeholder
        }

        pub fn enable(&mut self) {
            self.m_enable_count += 1;
        }

        pub fn disable(&mut self) {
            self.m_enable_count -= 1;
            if self.m_enable_count < 0 {
                self.m_enable_count = 0;
            }
        }

        pub fn is_paused(&self) -> bool {
            self.m_paused_context_group_id != 0
        }

        pub fn is_paused_in_context_group(&self, context_group_id: i32) -> bool {
            self.m_paused_context_group_id == context_group_id
        }

        pub fn max_async_call_chain_depth(&self) -> i32 {
            self.m_max_async_call_stack_depth
        }

        pub fn set_async_call_stack_depth(&mut self, _agent: *mut V8DebuggerAgentImpl, depth: i32) {
            self.m_max_async_call_stack_depth = depth;
        }

        pub fn max_call_stack_size_to_capture(&self) -> i32 {
            self.m_max_call_stack_size_to_capture
        }

        pub fn set_max_call_stack_size_to_capture(&mut self, _agent: *mut V8RuntimeAgentImpl, size: i32) {
            self.m_max_call_stack_size_to_capture = size;
        }

        pub fn current_async_parent(&self) -> Option<Arc<AsyncStackTrace>> {
            self.m_current_async_parent.lock().unwrap().last().cloned()
        }

        pub fn current_external_parent(&self) -> V8StackTraceId {
            self.m_current_external_parent.lock().unwrap().last().copied().unwrap_or_default()
        }

        pub fn symbolize(&self, _v8_frame: v8::Local<v8::StackFrame>) -> Option<Arc<StackFrame>> {
            None // Placeholder
        }

        pub fn create_stack_trace(&self, _v8_stack_trace: v8::Local<v8::StackTrace>) -> std::unique_ptr::UniquePtr<V8StackTraceImpl> {
            std::unique_ptr::UniquePtr::new(Box::new(V8StackTraceImpl {}))// Placeholder
        }

        pub fn capture_stack_trace(&self, _full_stack: bool) -> std::unique_ptr::UniquePtr<V8StackTraceImpl> {
            std::unique_ptr::UniquePtr::new(Box::new(V8StackTraceImpl {}))// Placeholder
        }

        pub fn internal_properties(&self, _context: v8::Local<v8::Context>, _value: v8::Local<v8::Value>) -> v8::MaybeLocal<v8::Array> {
            None // Placeholder
        }

        pub fn query_objects(&self, _context: v8::Local<v8::Context>, _prototype: v8::Local<v8::Object>) -> v8::Local<v8::Array> {
            0 // Placeholder
        }

        pub fn async_task_scheduled(&self, task_name: &StringView, task: *mut dyn Any, recurring: bool) {
            self.async_task_scheduled_for_stack(task_name, task, recurring, false);
        }

        pub fn async_task_canceled(&self, task: *mut dyn Any) {
            self.async_task_canceled_for_stack(task);
        }

        pub fn async_task_started(&self, task: *mut dyn Any) {
            self.async_task_started_for_stack(task);
        }

        pub fn async_task_finished(&self, task: *mut dyn Any) {
            self.async_task_finished_for_stack(task);
        }

        pub fn all_async_tasks_canceled(&self) {
            // Implementation here
        }

        pub fn store_current_stack_trace(&mut self, description: &StringView) -> V8StackTraceId {
            let stack = Arc::new(AsyncStackTrace {}); //Create a new stack.
            let id = self.store_stack_trace(stack);
            V8StackTraceId(id as i32)
        }

        pub fn external_async_task_started(&mut self, parent: &V8StackTraceId) {
            let mut external_parents = self.m_current_external_parent.lock().unwrap();
            external_parents.push(*parent);

            self.m_external_async_task_pause_requested = true;
        }

        pub fn external_async_task_finished(&mut self, parent: &V8StackTraceId) {
            let mut external_parents = self.m_current_external_parent.lock().unwrap();
            if let Some(index) = external_parents.iter().position(|&x| x == *parent) {
                external_parents.remove(index);
            }

            self.m_external_async_task_pause_requested = true;
        }

        pub fn store_stack_trace(&mut self, stack: Arc<AsyncStackTrace>) -> usize {
            self.m_last_stack_trace_id += 1;
            let id = self.m_last_stack_trace_id;
            self.m_stored_stack_traces.lock().unwrap().insert(id, Arc::downgrade(&stack));
            id
        }

        pub fn mute_script_parsed_events(&mut self) {
            self.m_ignore_script_parsed_events_counter += 1;
        }

        pub fn unmute_script_parsed_events(&mut self) {
            self.m_ignore_script_parsed_events_counter -= 1;
        }

        pub fn inspector(&self) -> *mut V8InspectorImpl {
            self.m_inspector
        }

        pub fn set_max_async_task_stacks_for_test(&mut self, limit: i32) {
            self.m_max_async_call_stacks = limit as usize;
        }

        pub fn dump_async_task_stacks_state_for_test(&self) {
            // Implementation here
        }

        pub fn async_parent_for(
            &self,
            stack_trace_id: i32,
            async_parent: &mut Option<Arc<AsyncStackTrace>>,
            external_parent: &mut V8StackTraceId,
        ) -> () {
            let async_parents = self.m_async_parents.lock().unwrap();
            if let Some(weak_parent) = async_parents.get(&stack_trace_id) {
                *async_parent = weak_parent.upgrade();
            }

            let external_parents = self.m_external_parents.lock().unwrap();
            for (id, parent) in external_parents.iter() {
                if *id == stack_trace_id {
                    *external_parent = *parent;
                    break;
                }
            }
        }

        pub fn debugger_id_for(&self, context_group_id: i32) -> internal::V8DebuggerId {
            *self.m_context_group_id_to_debugger_id.lock().unwrap().get(&context_group_id).unwrap()
        }

        pub fn stack_trace_for(&self, context_group_id: i32, id: &V8StackTraceId) -> Option<Arc<AsyncStackTrace>> {
            None // Placeholder
        }

        pub fn report_termination(&mut self) {
            self.m_terminate_execution_reported = true;
        }
    }

    impl Drop for V8Debugger {
        fn drop(&mut self) {
            // Clean up resources here
        }
    }

    //DebugDelegate Implementation
    impl DebugDelegate for V8Debugger{
        fn async_event_occurred(&mut self, _type: v8::debug::DebugAsyncActionType, _id: i32, _is_blackboxed: bool) {
            //println!("AsyncEventOccurred");
        }
        fn script_compiled(&mut self, _script: v8::Local<v8::debug::Script>, _is_live_edited: bool, _has_compile_error: bool) {
            //println!("ScriptCompiled");
        }
        fn break_program_requested(&mut self, _paused_context: v8::Local<v8::Context>, _break_points_hit: &std::vec::Vec<v8::debug::BreakpointId>, _break_reasons: v8::debug::BreakReasons) {
            //println!("BreakProgramRequested");
        }
        fn break_on_instrumentation(&mut self, _paused_context: v8::Local<v8::Context>, _breakpoint_id: v8::debug::BreakpointId) -> v8::debug::ActionAfterInstrumentation {
            //println!("BreakOnInstrumentation");
            v8::debug::ActionAfterInstrumentation::Continue
        }
        fn exception_thrown(&mut self, _paused_context: v8::Local<v8::Context>, _exception: v8::Local<v8::Value>, _promise: v8::Local<v8::Value>, _is_uncaught: bool, _exception_type: v8::debug::ExceptionType) {
            //println!("ExceptionThrown");
        }
        fn is_function_blackboxed(&mut self, _script: v8::Local<v8::debug::Script>, _start: &v8::debug::Location, _end: &v8::debug::Location) -> bool {
            //println!("IsFunctionBlackboxed");
            false
        }
        fn should_be_skipped(&mut self, _script: v8::Local<v8::debug::Script>, _line: i32, _column: i32) -> bool {
            //println!("ShouldBeSkipped");
            false
        }
        fn breakpoint_condition_evaluated(&mut self, _context: v8::Local<v8::Context>, _breakpoint_id: v8::debug::BreakpointId, _exception_thrown: bool, _exception: v8::Local<v8::Value>) {
            //println!("BreakpointConditionEvaluated");
        }
    }

    impl AsyncEventDelegate for V8Debugger {
        fn async_event_occurred(&mut self, _type: v8::debug::DebugAsyncActionType, _id: i32, _is_blackboxed: bool) {
            //println!("AsyncEventOccurred");
        }
    }

    impl V8Debugger {
        fn add_internal_object(&mut self, _context: v8::Local<v8::Context>, _object: v8::Local<v8::Object>, _type: V8InternalValueType) -> bool {
            false // Placeholder
        }

        fn clear_continue_to_location(&mut self) {
            // Implementation here
        }

        fn should_continue_to_current_location(&self) -> bool {
            false // Placeholder
        }

        fn near_heap_limit_callback(_data: *mut (), _current_heap_limit: usize, _initial_heap_limit: usize) -> usize {
            0 // Placeholder
        }

        fn terminate_execution_completed_callback(_isolate: v8::Isolate) {
            // Implementation here
        }

        fn terminate_execution_completed_callback_ignoring_data(_isolate: v8::Isolate, _data: *mut ()) {
            // Implementation here
        }

        fn install_terminate_execution_callbacks(&mut self, _context: v8::Local<v8::Context>) {
            // Implementation here
        }

        fn handle_program_break(
            &mut self,
            _paused_context: v8::Local<v8::Context>,
            _exception: v8::Local<v8::Value>,
            _hit_breakpoints: &Vec<v8::debug::BreakpointId>,
            _break_reasons: v8::debug::BreakReasons,
            _exception_type: v8::debug::ExceptionType,
            _is_uncaught: bool,
        ) {
            // Implementation here
        }

        fn get_target_scopes(&self, _context: v8::Local<v8::Context>, _value: v8::Local<v8::Value>, _kind: ScopeTargetKind) -> v8::MaybeLocal<v8::Value> {
            None // Placeholder
        }

        fn function_scopes(&self, _context: v8::Local<v8::Context>, _function: v8::Local<v8::Object>) -> v8::MaybeLocal<v8::Value> {
            None // Placeholder
        }

        fn generator_scopes(&self, _context: v8::Local<v8::Context>, _value: v8::Local<v8::Value>) -> v8::MaybeLocal<v8::Value> {
            None // Placeholder
        }

        fn collections_entries(&self, _context: v8::Local<v8::Context>, _value: v8::Local<v8::Value>) -> v8::MaybeLocal<v8::Array> {
            None // Placeholder
        }

        fn private_methods(&self, _context: v8::Local<v8::Context>, _value: v8::Local<v8::Value>) -> v8::MaybeLocal<v8::Array> {
            None // Placeholder
        }

        fn async_task_scheduled_for_stack(&self, task_name: &StringView, task: *mut dyn Any, recurring: bool, skip_top_frame: bool) {
            let mut tasks = self.m_current_tasks.lock().unwrap();
            let mut async_parents = self.m_current_async_parent.lock().unwrap();
            let mut external_parents = self.m_current_external_parent.lock().unwrap();

            tasks.push(task);

            let parent = Arc::new(AsyncStackTrace {});

            async_parents.push(parent.clone());

            let mut async_task_stacks = self.m_async_task_stacks.lock().unwrap();
            async_task_stacks.insert(task, Arc::downgrade(&parent));

            if recurring {
                self.m_recurring_tasks.lock().unwrap().insert(task);
            }

            self.m_pause_on_next_call_requested = true;
        }

        fn async_task_canceled_for_stack(&self, task: *mut dyn Any) {
            let mut tasks = self.m_current_tasks.lock().unwrap();
            let mut async_parents = self.m_current_async_parent.lock().unwrap();

            if let Some(index) = tasks.iter().position(|&x| x == task) {
                tasks.remove(index);
            }

            if let Some(index) = async_parents.iter().position(|_| true) {
                async_parents.remove(index);
            }

            let mut async_task_stacks = self.m_async_task_stacks.lock().unwrap();
            async_task_stacks.remove(&task);
        }

        fn async_task_started_for_stack(&self, task: *mut dyn Any) {
            let mut tasks = self.m_current_tasks.lock().unwrap();
            let mut async_parents = self.m_current_async_parent.lock().unwrap();

            tasks.push(task);
            let parent = Arc::new(AsyncStackTrace {});
            async_parents.push(parent.clone());

            let mut async_task_stacks = self.m_async_task_stacks.lock().unwrap();
            async_task_stacks.insert(task, Arc::downgrade(&parent));

            self.m_pause_on_next_call_requested = true;
        }

        fn async_task_finished_for_stack(&self, task: *mut dyn Any) {
            let mut tasks = self.m_current_tasks.lock().unwrap();
            let mut async_parents = self.m_current_async_parent.lock().unwrap();

            if let Some(index) = tasks.iter().position(|&x| x == task) {
                tasks.remove(index);
            }
            if let Some(index) = async_parents.iter().position(|_| true) {
                async_parents.remove(index);
            }

            let mut async_task_stacks = self.m_async_task_stacks.lock().unwrap();
            async_task_stacks.remove(&task);
        }

        fn async_task_candidate_for_stepping(&self, task: *mut