// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_debugger_agent_impl {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::error::Error;
    use std::fmt;
    use std::rc::Rc;

    //use crate::base::enum_set::EnumSet; // Assuming a similar enum-set functionality is needed
    //use crate::debug::debug_interface; // Assuming a similar debug interface is needed
    //use crate::inspector::protocol::Debugger; // Assuming protocol definitions are available
    //use crate::inspector::protocol::Forward; // Assuming forward declarations are available

    //type String16 = String; // Assuming String16 is equivalent to String
    pub type Response = Result<(), Box<dyn Error>>;

    #[derive(Debug, Clone)]
    pub struct String16(String);

    impl String16 {
        pub fn new(s: String) -> Self {
            String16(s)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }
    }
    
    #[derive(Debug)]
    pub struct ProtocolError(String);

    impl fmt::Display for ProtocolError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Protocol Error: {}", self.0)
        }
    }

    impl Error for ProtocolError {}

    pub mod protocol {
        pub mod Debugger {
            use super::super::String16;

            pub trait Backend {
                fn enable(&mut self, max_scripts_cache_size: Option<f64>, out_debugger_id: &mut String16) -> super::Response;
                fn disable(&mut self) -> super::Response;
                fn set_breakpoints_active(&mut self, active: bool) -> super::Response;
                fn set_skip_all_pauses(&mut self, skip: bool) -> super::Response;
                fn set_breakpoint_by_url(
                    &mut self,
                    line_number: i32,
                    optional_url: Option<String16>,
                    optional_url_regex: Option<String16>,
                    optional_script_hash: Option<String16>,
                    optional_column_number: Option<i32>,
                    optional_condition: Option<String16>,
                    out_breakpoint_id: &mut String16,
                    locations: &mut Option<std::vec::Vec<Location>>,
                ) -> super::Response;
                fn set_breakpoint(
                    &mut self,
                    location: Box<Location>,
                    optional_condition: Option<String16>,
                    out_breakpoint_id: &mut String16,
                    actual_location: &mut Option<Box<Location>>,
                ) -> super::Response;
                fn set_breakpoint_on_function_call(
                    &mut self,
                    function_object_id: &String16,
                    optional_condition: Option<String16>,
                    out_breakpoint_id: &mut String16,
                ) -> super::Response;
                fn set_instrumentation_breakpoint(
                    &mut self,
                    instrumentation: &String16,
                    out_breakpoint_id: &mut String16,
                ) -> super::Response;
                fn remove_breakpoint(&mut self, breakpoint_id: &String16) -> super::Response;
                fn continue_to_location(
                    &mut self,
                    location: Box<Location>,
                    target_call_frames: Option<String16>,
                ) -> super::Response;
                fn get_stack_trace(
                    &mut self,
                    stack_trace_id: Box<super::Runtime::StackTraceId>,
                    stack_trace: &mut Option<Box<super::Runtime::StackTrace>>,
                ) -> super::Response;
                fn search_in_content(
                    &mut self,
                    script_id: &String16,
                    query: &String16,
                    optional_case_sensitive: Option<bool>,
                    optional_is_regex: Option<bool>,
                    results: &mut Option<std::vec::Vec<SearchMatch>>,
                ) -> super::Response;
                fn get_possible_breakpoints(
                    &mut self,
                    start: Box<Location>,
                    end: Box<Location>,
                    restrict_to_function: Option<bool>,
                    locations: &mut Option<std::vec::Vec<BreakLocation>>,
                ) -> super::Response;
                fn set_script_source(
                    &mut self,
                    script_id: &String16,
                    script_source: &String16,
                    dry_run: Option<bool>,
                    allow_top_frame_editing: Option<bool>,
                    opt_out_call_frames: &mut Option<std::vec::Vec<CallFrame>>,
                    opt_out_stack_changed: &mut Option<bool>,
                    opt_out_async_stack_trace: &mut Option<Box<super::Runtime::StackTrace>>,
                    opt_out_async_stack_trace_id: &mut Option<Box<super::Runtime::StackTraceId>>,
                    out_status: &mut String16,
                    opt_out_compile_error: &mut Option<Box<super::Runtime::ExceptionDetails>>,
                ) -> super::Response;
                fn restart_frame(
                    &mut self,
                    call_frame_id: &String16,
                    mode: Option<String16>,
                    new_call_frames: &mut Option<std::vec::Vec<CallFrame>>,
                    async_stack_trace: &mut Option<Box<super::Runtime::StackTrace>>,
                    async_stack_trace_id: &mut Option<Box<super::Runtime::StackTraceId>>,
                ) -> super::Response;
                fn get_script_source(
                    &mut self,
                    script_id: &String16,
                    script_source: &mut String16,
                    bytecode: &mut Option<super::Binary>,
                ) -> super::Response;
                fn disassemble_wasm_module(
                    &mut self,
                    script_id: &String16,
                    out_stream_id: &mut Option<String16>,
                    out_total_number_of_lines: &mut i32,
                    out_function_body_offsets: &mut Option<std::vec::Vec<i32>>,
                    out_chunk: &mut Option<Box<WasmDisassemblyChunk>>,
                ) -> super::Response;
                fn next_wasm_disassembly_chunk(
                    &mut self,
                    stream_id: &String16,
                    out_chunk: &mut Option<Box<WasmDisassemblyChunk>>,
                ) -> super::Response;
                fn get_wasm_bytecode(&mut self, script_id: &String16, bytecode: &mut super::Binary) -> super::Response;
                fn pause(&mut self) -> super::Response;
                fn resume(&mut self, terminate_on_resume: Option<bool>) -> super::Response;
                fn step_over(
                    &mut self,
                    skip_list: Option<std::vec::Vec<LocationRange>>,
                ) -> super::Response;
                fn step_into(
                    &mut self,
                    break_on_async_call: Option<bool>,
                    skip_list: Option<std::vec::Vec<LocationRange>>,
                ) -> super::Response;
                fn step_out(&mut self) -> super::Response;
                fn pause_on_async_call(&mut self, parent_stack_trace_id: Box<super::Runtime::StackTraceId>) -> super::Response;
                fn set_pause_on_exceptions(&mut self, pause_state: &String16) -> super::Response;
                fn evaluate_on_call_frame(
                    &mut self,
                    call_frame_id: &String16,
                    expression: &String16,
                    object_group: Option<String16>,
                    include_command_line_api: Option<bool>,
                    silent: Option<bool>,
                    return_by_value: Option<bool>,
                    generate_preview: Option<bool>,
                    throw_on_side_effect: Option<bool>,
                    timeout: Option<f64>,
                    result: &mut Option<Box<super::Runtime::RemoteObject>>,
                    exception_details: &mut Option<Box<super::Runtime::ExceptionDetails>>,
                ) -> super::Response;
                fn set_variable_value(
                    &mut self,
                    scope_number: i32,
                    variable_name: &String16,
                    new_value: Box<super::Runtime::CallArgument>,
                    call_frame: &String16,
                ) -> super::Response;
                fn set_return_value(
                    &mut self,
                    new_value: Box<super::Runtime::CallArgument>,
                ) -> super::Response;
                fn set_async_call_stack_depth(&mut self, depth: i32) -> super::Response;
                fn set_blackbox_patterns(
                    &mut self,
                    patterns: Option<std::vec::Vec<String16>>,
                    skip_anonymous: Option<bool>,
                ) -> super::Response;
                fn set_blackbox_execution_contexts(
                    &mut self,
                    unique_ids: Option<std::vec::Vec<String16>>,
                ) -> super::Response;
                fn set_blackboxed_ranges(
                    &mut self,
                    script_id: &String16,
                    positions: Option<std::vec::Vec<ScriptPosition>>,
                ) -> super::Response;
            }
            
            #[derive(Debug)]
            pub struct Location {
                pub script_id: String16,
                pub line_number: i32,
                pub column_number: Option<i32>,
            }

            #[derive(Debug)]
            pub struct SearchMatch {
                pub line_number: i32,
                pub line_content: String16,
            }

            #[derive(Debug)]
            pub struct BreakLocation {
                pub script_id: String16,
                pub line_number: i32,
                pub column_number: Option<i32>,
                pub type_: Option<String16>, // Assuming a String16 for the type
            }

            #[derive(Debug)]
            pub struct CallFrame {
                pub call_frame_id: String16,
                pub function_name: String16,
                pub function_location: Option<Location>,
                pub location: Location,
                pub scope_chain: std::vec::Vec<Scope>,
                pub this: Box<super::Runtime::RemoteObject>,
                pub return_value: Option<Box<super::Runtime::RemoteObject>>,
            }

            #[derive(Debug)]
            pub struct Scope {
                pub type_: String16,
                pub object: Box<super::Runtime::RemoteObject>,
                pub name: Option<String16>,
                pub start_location: Option<Location>,
                pub end_location: Option<Location>,
            }

            #[derive(Debug)]
            pub struct ScriptPosition {
                pub line_number: i32,
                pub column_number: i32,
            }

            #[derive(Debug)]
            pub struct LocationRange {
                pub script_id: String16,
                pub start: ScriptPosition,
                pub end: ScriptPosition,
            }

            #[derive(Debug)]
            pub struct WasmDisassemblyChunk {
                pub lines: std::vec::Vec<String16>,
            }
        }

        pub mod Runtime {
            use super::super::String16;
            #[derive(Debug)]
            pub struct RemoteObject {
                pub type_: String16,
                pub subtype: Option<String16>,
                pub class_name: Option<String16>,
                pub value: Option<String16>, // Assuming String16 can represent any value type
                pub description: Option<String16>,
                pub object_id: Option<String16>,
                pub unserializable_value: Option<String16>,
                pub preview: Option<Box<ObjectPreview>>,
            }

            #[derive(Debug)]
            pub struct ObjectPreview {
                pub type_: String16,
                pub subtype: Option<String16>,
                pub description: Option<String16>,
                pub overflow: bool,
                pub properties: std::vec::Vec<PropertyPreview>,
                pub entries: Option<std::vec::Vec<EntryPreview>>,
            }

            #[derive(Debug)]
            pub struct PropertyPreview {
                pub name: String16,
                pub type_: String16,
                pub value: String16,
                pub value_preview: Option<Box<ObjectPreview>>,
            }

            #[derive(Debug)]
            pub struct EntryPreview {
                pub key: Option<Box<ObjectPreview>>,
                pub value: Box<ObjectPreview>,
            }

            #[derive(Debug)]
            pub struct CallArgument {
                pub value: Option<String16>,
                pub unserializable_value: Option<String16>,
                pub object_id: Option<String16>,
            }

            #[derive(Debug)]
            pub struct StackTraceId {
                pub id: String16,
                pub debugger_id: String16,
            }

            #[derive(Debug)]
            pub struct StackTrace {
                pub call_frames: std::vec::Vec<super::Debugger::CallFrame>,
                pub parent_id: Option<Box<StackTraceId>>,
                pub parent_debugger_id: Option<String16>,
            }

            #[derive(Debug)]
            pub struct ExceptionDetails {
                pub exception_id: i32,
                pub text: String16,
                pub line_number: i32,
                pub column_number: i32,
                pub script_id: String16,
                pub url: Option<String16>,
                pub stack_trace: Option<Box<StackTrace>>,
                pub exception: Option<Box<RemoteObject>>,
                pub error_group: Option<String16>,
            }
        }

        #[derive(Debug)]
        pub struct DictionaryValue {
            pub data: HashMap<String, String>,
        }
        
        #[derive(Debug)]
        pub struct Binary {
            pub data: Vec<u8>,
        }
        
        impl Binary {
            pub fn new(data: Vec<u8>) -> Self {
                Binary { data }
            }
        }
    }

    //struct ScriptBreakpoint; // Assuming definition is not needed for conversion

    struct DisassemblyCollectorImpl; // Assuming definition is not needed for conversion
    struct V8Debugger; // Assuming definition is not needed for conversion
    struct V8DebuggerScript; // Assuming definition is not needed for conversion
    struct V8InspectorImpl; // Assuming definition is not needed for conversion
    struct V8InspectorSessionImpl; // Assuming definition is not needed for conversion
    struct V8Regex; // Assuming definition is not needed for conversion

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub enum BreakpointSource {
        UserBreakpointSource,
        DebugCommandBreakpointSource,
        MonitorCommandBreakpointSource,
    }

    #[derive(Debug)]
    enum EnableState {
        Disabled,
        Enabled,
        Stopping, // This is the same as 'disabled', but it cannot become enabled again.
    }

    pub struct V8DebuggerAgentImpl {
        m_inspector: *mut V8InspectorImpl, // raw pointer
        m_debugger: *mut V8Debugger,          // raw pointer
        m_session: *mut V8InspectorSessionImpl, // raw pointer
        m_enable_state: EnableState,
        m_state: *mut protocol::DictionaryValue, // raw pointer
        m_frontend: FrontendChannel,
        m_isolate: *mut u8, //v8::Isolate, // Assuming v8::Isolate is a raw pointer type
        m_scripts: HashMap<String16, Box<V8DebuggerScript>>,
        m_breakpoint_id_to_debugger_breakpoint_ids: HashMap<String16, Vec<i32>>, // Assuming i32 is a suitable replacement for v8::debug::BreakpointId
        m_debugger_breakpoint_id_to_breakpoint_id: HashMap<i32, String16>, // Assuming i32 is a suitable replacement for v8::debug::BreakpointId
        m_wasm_disassemblies: HashMap<String16, Box<DisassemblyCollectorImpl>>,
        m_next_wasm_disassembly_stream_id: usize,

        m_max_script_cache_size: usize,
        m_cached_script_size: usize,

        m_cached_scripts: VecDeque<CachedScript>,

        m_break_reason: Vec<BreakReason>,

        m_skip_all_pauses: bool,
        m_breakpoints_active: bool,
        m_instrumentation_finished: bool,
        m_skip_anonymous_scripts: bool,

        m_blackbox_pattern: Option<Box<V8Regex>>,
        m_blackboxed_positions: HashMap<String16, Vec<(int, int)>>,
        m_skip_list: HashMap<String16, Vec<(int, int)>>,
        m_blackboxed_execution_contexts: HashSet<String16>,
    }

    #[derive(Debug)]
    struct CachedScript {
        script_id: String16,
        source: String16,
        bytecode: Vec<u8>,
    }

    impl CachedScript {
        fn size(&self) -> usize {
            self.source.len() * 2 + self.bytecode.len() // Assuming UChar is 2 bytes
        }
    }

    type BreakReason = (String16, Box<protocol::DictionaryValue>);

    impl V8DebuggerAgentImpl {
        pub fn new(
            inspector_session: *mut V8InspectorSessionImpl, // raw pointer
            frontend: FrontendChannel,
            state: *mut protocol::DictionaryValue, // raw pointer
        ) -> V8DebuggerAgentImpl {
            V8DebuggerAgentImpl {
                m_inspector: std::ptr::null_mut(), //nullptr, // TODO: Initialize properly
                m_debugger: std::ptr::null_mut(), //nullptr, // TODO: Initialize properly
                m_session: inspector_session,
                m_enable_state: EnableState::Disabled,
                m_state: state,
                m_frontend: frontend,
                m_isolate: std::ptr::null_mut(),
                m_scripts: HashMap::new(),
                m_breakpoint_id_to_debugger_breakpoint_ids: HashMap::new(),
                m_debugger_breakpoint_id_to_breakpoint_id: HashMap::new(),
                m_wasm_disassemblies: HashMap::new(),
                m_next_wasm_disassembly_stream_id: 0,
                m_max_script_cache_size: 0,
                m_cached_script_size: 0,
                m_cached_scripts: VecDeque::new(),
                m_break_reason: Vec::new(),
                m_skip_all_pauses: false,
                m_breakpoints_active: false,
                m_instrumentation_finished: true,
                m_skip_anonymous_scripts: false,
                m_blackbox_pattern: None,
                m_blackboxed_positions: HashMap::new(),
                m_skip_list: HashMap::new(),
                m_blackboxed_execution_contexts: HashSet::new(),
            }
        }

        pub fn restore(&mut self) {
            //TODO: Implement restore functionality.
        }

        pub fn stop(&mut self) {
            self.m_enable_state = EnableState::Stopping;
        }

        pub fn enabled(&self) -> bool {
            self.m_enable_state == EnableState::Enabled
        }

        pub fn set_breakpoint_for(
            &mut self,
            function: u32, //v8::Local<v8::Function>,
            condition: u32, //v8::Local<v8::String>,
            source: BreakpointSource,
        ) {
            //TODO: Implement set_breakpoint_for functionality.
            println!("set_breakpoint_for called");
        }

        pub fn remove_breakpoint_for(
            &mut self,
            function: u32, //v8::Local<v8::Function>,
            source: BreakpointSource,
        ) {
            //TODO: Implement remove_breakpoint_for functionality.
            println!("remove_breakpoint_for called");
        }

        pub fn schedule_pause_on_next_statement(
            &mut self,
            break_reason: &String16,
            data: Box<protocol::DictionaryValue>,
        ) {
            //TODO: Implement schedule_pause_on_next_statement functionality.
            println!("schedule_pause_on_next_statement called");
        }

        pub fn cancel_pause_on_next_statement(&mut self) {
            //TODO: Implement cancel_pause_on_next_statement functionality.
            println!("cancel_pause_on_next_statement called");
        }

        pub fn break_program(
            &mut self,
            break_reason: &String16,
            data: Box<protocol::DictionaryValue>,
        ) {
            //TODO: Implement break_program functionality.
            println!("break_program called");
        }

        pub fn reset(&mut self) {
            //TODO: Implement reset functionality.
        }

        pub fn instrumentation_finished(&self) -> bool {
            self.m_instrumentation_finished
        }

        pub fn did_pause_on_instrumentation(&mut self, instrumentation_id: i32) {
            // Assuming i32 is a suitable replacement for v8::debug::BreakpointId
            //TODO: Implement did_pause_on_instrumentation functionality.
            println!("did_pause_on_instrumentation called");
        }

        pub fn did_pause(
            &mut self,
            context_id: int,
            exception: u32, //v8::Local<v8::Value>,
            hit_breakpoints: &Vec<i32>, // Assuming i32 is a suitable replacement for v8::debug::BreakpointId
            exception_type: u32,       //v8::debug::ExceptionType,
            is_uncaught: bool,
            break_reasons: u32, //v8::debug::BreakReasons,
        ) {
            //TODO: Implement did_pause functionality.
            println!("did_pause called");
        }

        pub fn did_continue(&mut self) {
            //TODO: Implement did_continue functionality.
            println!("did_continue called");
        }

        pub fn did_parse_source(&mut self, script: Box<V8DebuggerScript>, success: bool) {
            //TODO: Implement did_parse_source functionality.
            println!("did_parse_source called");
        }

        pub fn is_function_blackboxed(
            &self,
            script_id: &String16,
            start: &u32, //v8::debug::Location,
            end: &u32,   //v8::debug::Location,
        ) -> bool {
            //TODO: Implement is_function_blackboxed functionality.
            println!("is_function_blackboxed called");
            false
        }

        pub fn should_be_skipped(&self, script_id: &String16, line: int, column: int) -> bool {
            //TODO: Implement should_be_skipped functionality.
            println!("should_be_skipped called");
            false
        }

        pub fn accepts_pause(&self, is_oom_break: bool) -> bool {
            //TODO: Implement accepts_pause functionality.
            println!("accepts_pause called");
            true
        }

        pub fn script_collected(&mut self, script: *const V8DebuggerScript) {
            //TODO: Implement ScriptCollected functionality.
            println!("script_collected called");
        }

        pub fn isolate(&self) -> *mut u8 {//v8::Isolate {
            self.m_isolate
        }

        pub fn clear_break_details(&mut self) {
            self.m_break_reason.clear();
        }

        fn enable_impl(&mut self) {
            self.m_enable_state = EnableState::Enabled;
        }

        fn current_call_frames(
            &mut self,
            _call_frames: &mut Option<Vec<protocol::Debugger::CallFrame>>,
        ) -> Response {
            println!("current_call_frames called");
            Err(Box::new(ProtocolError("Not implemented".to_string())))
        }

        fn current_async_stack_trace(&mut self) -> Option<Box<protocol::Runtime::StackTrace>> {
            println!("current_async_stack_trace called");
            None
        }

        fn current_external_stack_trace(&mut self) -> Option<Box<protocol::Runtime::StackTraceId>> {
            println!("current_external_stack_trace called");
            None
        }

        fn set_pause_on_exceptions_impl(&mut self, _state: int) {
            println!("set_pause_on_exceptions_impl called");
        }

        fn set_breakpoint_impl(
            &mut self,
            breakpoint_id: &String16,
            script_id: &String16,
            condition: &String16,
            line_number: int,
            column_number: int,
        ) -> Option<Box<protocol::Debugger::Location>> {
            println!("set_breakpoint_impl called");
            None
        }

        fn set_breakpoint_impl1(
            &mut self,
            breakpoint_id: &String16,
            function: u32, //v8::Local<v8::Function>,
            condition: u32,  //v8::Local<v8::String>,
        ) {
            println!("set_breakpoint_impl1 called");
        }

        fn remove_breakpoint_impl(
            &mut self,
            breakpoint_id: &String16,
            scripts: &Vec<*mut V8DebuggerScript>,
        ) {
            println!("remove_breakpoint_impl called");
        }

        fn internal_set_async_call_stack_depth(&mut self, _depth: int) {
            println!("internal_set_async_call_stack_depth called");
        }

        fn increase_cached_skip_stack_generation(&mut self) {
            println!("increase_cached_skip_stack_generation called");
        }

        fn set_blackbox_pattern(&mut self, pattern: &String16) -> Response {
            println!("set_blackbox_pattern called");
            Err(Box::new(ProtocolError("Not implemented".to_string())))
        }

        fn reset_blackboxed_state_cache(&mut self) {
            println!("reset_blackboxed_state_cache called");
        }

        fn is_paused(&self) -> bool {
            println!("is_paused called");
            false
        }

        fn set_script_instrumentation_breakpoint_if_needed(&mut self, _script: *mut V8DebuggerScript) {
            println!("set_script_instrumentation_breakpoint_if_needed called");
        }

        fn process_skip_list(
            &mut self,
            _skip_list: &mut Vec<protocol::Debugger::LocationRange>,
        ) -> Response {
            println!("process_skip_list called");
            Err(Box::new(ProtocolError("Not implemented".to_string())))
        }

        fn push_break_details(
            &mut self,
            break_reason: &String16,
            break_aux_data: Box<protocol::DictionaryValue>,
        ) {
            self.m_break_reason.push((break_reason.clone(), break_aux_data));
        }

        fn pop_break_details(&mut self) {
            self.m_break_reason.pop();
        }
    }

    // Implement the protocol::Debugger::Backend trait for V8DebuggerAgentImpl
    impl protocol::Debugger::Backend for V8DebuggerAgentImpl {
        fn enable(&mut self, max_scripts_cache_size: Option<f64>, out_debugger_id: &mut String16) -> Response {
            println!("enable called");
            if let Some(size) = max_scripts_cache_size {
                self.m_max_script_cache_size = size as usize;
            }
            self.enable_impl();
            *out_debugger_id = String16::new("debuggerId".to_string());
            Ok(())
        }

        fn disable(&mut self) -> Response {
            println!("disable called");
            self.m_enable_state = EnableState::Disabled;
            Ok(())
        }

        fn set_breakpoints_active(&mut self, active: bool) -> Response {
            println!("set_breakpoints_active called");
            self.m_breakpoints_active = active;
            Ok(())
        }

        fn set_skip_all_pauses(&mut self, skip: bool) -> Response {
            println!("set_skip_all_pauses called");
            self.m_skip_all_pauses = skip;
            Ok(())
        }

        fn set_breakpoint_by_url(
            &mut self,
            line_number: i32,
            optional_url: Option<String16>,
            optional_url_regex: Option<String16>,
            optional_script_hash: Option<String16>,
            optional_column_number: Option<i32>,
            optional_condition: Option<String16>,
            out_breakpoint_id: &mut String16,
            locations: &mut Option<Vec<protocol::Debugger::Location>>,
        ) -> Response {
            println!("set_breakpoint_by_url called");
            *out_breakpoint_id = String16::new("breakpointId".to_string());
            *locations = Some(vec![]);
            Ok(())
        }

        fn set_breakpoint(
            &mut self,
            location: Box<protocol::Debugger::Location>,
            optional_condition: Option<String16>,
            out_breakpoint_id: &mut String16,
            actual_location: &mut Option<Box<protocol::Debugger::Location>>,
        ) -> Response {
            println!("set_breakpoint called");
            *out_breakpoint_id = String16::new("breakpointId".to_string());
            *actual_location = Some(location);
            Ok(())
        }

        fn set_breakpoint_on_function_call(
            &mut self,
            function_object_id: &String16,
            optional_condition: Option<String16>,
            out_breakpoint_id: &mut String16,
        ) -> Response {
            println!("set_breakpoint_on_function_call called");
            *out_breakpoint_id = String16::new("breakpointId".to_string());
            Ok(())
        }

        fn set_instrumentation_breakpoint(
            &mut self,
            instrumentation: &String16,
            out_breakpoint_id: &mut String16,
        ) -> Response {
            println!("set_instrumentation_breakpoint called");
            *out_breakpoint_id = String16::new("breakpointId".to_string());
            Ok(())
        }

        fn remove_breakpoint(&mut self, breakpoint_id: &String16) -> Response {
            println!("remove_breakpoint called");
            Ok(())
        }

        fn continue_to_location(
            &mut self,
            location: Box<protocol::Debugger::Location>,
            target_call_frames: Option<String16>,
        ) -> Response {
            println!("continue_to_location called");
            Ok(())
        }

        fn get_stack_trace(
            &mut self,
            stack_trace_id: Box<protocol::Runtime::StackTraceId>,
            stack_trace: &mut Option<Box<protocol::Runtime::StackTrace>>,
        ) -> Response {
            println!("get_stack_trace called");
            Ok(())
        }

        fn search_in_content(
            &mut self,
            script_id: &String16,
            query: &String16,
            optional_case_sensitive: Option<bool>,
            optional_is_regex: Option<bool>,
            results: &mut Option<Vec<protocol::Debugger::SearchMatch>>,
        ) -> Response {
            println!("search_in_content called");
            *results = Some(vec![]);
            Ok(())
        }

        fn get_possible_breakpoints(
            &mut self,
            start: Box<protocol::Debugger::Location>,
            end: Box<protocol::Debugger::Location>,
            restrict_to_function: Option<bool>,
            locations: &mut Option<Vec<protocol::Debugger::BreakLocation>>,
        ) -> Response {
            println!("get_possible_breakpoints called");
            *locations = Some(vec![]);
            Ok(())
        }

        fn set_script_source(
            &mut self,
            script_id: &String16,
            script_source: &String16,
            dry_run: Option<bool>,
            allow_top_frame_editing: Option<bool>,
            opt_out_call_frames: &mut Option<Vec<protocol::Debugger::CallFrame>>,
            opt_out_stack_changed: &mut Option<bool>,
            opt_out_async_stack_trace: &mut Option<Box<protocol::Runtime::StackTrace>>,
            opt_out_async_stack_trace_id: &mut Option<Box<protocol::Runtime::StackTraceId>>,
            out_status: &mut String16,
            opt_out_compile_error: &mut Option<Box<protocol::Runtime::ExceptionDetails>>,
        ) -> Response {
            println!("set_script_source called");
            *out_status = String16::new("status".to_string());
            Ok(())
        }

        fn restart_frame(
            &mut self,
            call_frame_id: &String16,
            mode: Option<String16>,
            new_call_frames: &mut Option<Vec<protocol::Debugger::CallFrame>>,
            async_stack_trace: &mut Option<Box<protocol::Runtime::StackTrace>>,
            async_stack_trace_id: &mut Option<Box<protocol::Runtime::StackTraceId>>,
        ) -> Response {
            println!("restart_frame called");
            Ok(())
        }

        fn get_script