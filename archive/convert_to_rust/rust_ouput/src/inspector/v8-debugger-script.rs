// Converted from V8 C++ source files:
// Header: v8-debugger-script.h
// Implementation: v8-debugger-script.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_debugger_script {
    use std::cell::RefCell;
    use std::rc::Rc;
    use v8_inspector::string_util::{String16Builder, StringBuffer, toString16, toStringView};

    use crate::inspector::string_16::String16;
    use std::usize;

    pub struct V8DebuggerAgentImpl {}
    pub struct V8InspectorClient {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Language {
        JavaScript,
        WebAssembly,
    }

    pub struct V8DebuggerScript {
        id: String16,
        url: String16,
        has_source_url_comment: bool,
        execution_context_id: i32,
        isolate: *mut v8::Isolate,
        embedder_name: String16,
    }

    impl V8DebuggerScript {
        pub fn create(
            isolate: *mut v8::Isolate,
            script: v8::Local<v8::debug::Script>,
            is_live_edit: bool,
            agent: *mut V8DebuggerAgentImpl,
            client: *mut V8InspectorClient,
        ) -> Result<Box<dyn ScriptTrait>, String> {
            ActualScript::new(isolate, script, is_live_edit, agent, client)
                .map(|script| Box::new(script) as Box<dyn ScriptTrait>)
        }

        pub fn script_id(&self) -> &String16 {
            &self.id
        }

        pub fn has_source_url_comment(&self) -> bool {
            self.has_source_url_comment
        }

        pub fn source_url(&self) -> &String16 {
            &self.url
        }

        pub fn embedder_name(&self) -> &String16 {
            &self.embedder_name
        }

        pub fn execution_context_id(&self) -> i32 {
            self.execution_context_id
        }

        pub fn set_source_url(&mut self, source_url: &String16) {
            if source_url.len() > 0 {
                self.has_source_url_comment = true;
                self.url = source_url.clone();
            }
        }

        pub fn new(isolate: *mut v8::Isolate, id: String16, url: String16, embedder_name: String16) -> Self {
            V8DebuggerScript {
                id,
                url,
                has_source_url_comment: false,
                execution_context_id: 0,
                isolate,
                embedder_name,
            }
        }
    }

    pub trait ScriptTrait {
        fn source_mapping_url(&self) -> &String16;
        fn source(&self, pos: usize, len: usize) -> Result<String16, String>;
        fn language(&self) -> Language;
        fn hash(&self) -> &String16;
        fn build_id(&self) -> Result<String16, String>;
        fn start_line(&self) -> i32;
        fn start_column(&self) -> i32;
        fn end_line(&self) -> i32;
        fn end_column(&self) -> i32;
        fn code_offset(&self) -> i32;
        fn is_live_edit(&self) -> bool;
        fn is_module(&self) -> bool;
        fn length(&self) -> Result<i32, String>;
        fn set_source_mapping_url(&mut self, source_mapping_url: String16);
        fn set_source(
            &mut self,
            source: &String16,
            preview: bool,
            allow_top_frame_live_editing: bool,
            result: &mut v8::debug::LiveEditResult,
        ) -> Result<(), String>;
        fn get_possible_breakpoints(
            &self,
            start: &v8::debug::Location,
            end: &v8::debug::Location,
            ignore_nested_functions: bool,
            locations: &mut Vec<v8::debug::BreakLocation>,
        ) -> Result<bool, String>;
        fn reset_blackboxed_state_cache(&self) -> Result<(), String>;
        fn offset(&self, line_number: i32, column_number: i32) -> Result<Option<i32>, String>;
        fn location(&self, offset: i32) -> Result<v8::debug::Location, String>;
        fn set_breakpoint(
            &self,
            condition: &String16,
            location: &mut v8::debug::Location,
            id: &mut i32,
        ) -> Result<bool, String>;
        fn make_weak(&self) -> Result<(), String>;
        fn set_instrumentation_breakpoint(&self, id: &mut i32) -> Result<bool, String>;
        #[cfg(feature = "v8_enable_webassembly")]
        fn wasm_bytecode(&self) -> Result<Option<v8::MemorySpan<u8>>, String>;
        #[cfg(feature = "v8_enable_webassembly")]
        fn get_debug_symbols(&self) -> Result<Vec<v8::debug::WasmScript::DebugSymbols>, String>;
        #[cfg(feature = "v8_enable_webassembly")]
        fn disassemble(
            &self,
            collector: &mut v8::debug::DisassemblyCollector,
            function_body_offsets: &mut Vec<i32>,
        ) -> Result<(), String>;
    }

    const K_GLOBAL_DEBUGGER_SCRIPT_HANDLE_LABEL: &str = "DevTools debugger";

    fn calculate_hash(isolate: *mut v8::Isolate, source: v8::Local<v8::String>) -> String16 {
        let length = source.length();
        let mut buffer: Vec<u16> = vec![0; length];
        source.write_v2(
            unsafe { &mut *isolate },
            0,
            length,
            buffer.as_mut_ptr(),
        );

        let data = buffer.as_ptr() as *const u8;
        let size_in_bytes = length * 2;

        let mut hash = [0u8; 32];
        v8::internal::sha256_hash(data, size_in_bytes, &mut hash);

        let mut formatted_hash = String16Builder::new();
        for &byte in &hash {
            formatted_hash.append_unsigned_as_hex(byte);
        }

        formatted_hash.to_string()
    }

    struct ActualScript {
        base: V8DebuggerScript,
        agent: *mut V8DebuggerAgentImpl,
        source_mapping_url: String16,
        language: Language,
        is_live_edit: bool,
        is_module: bool,
        hash: RefCell<String16>,
        start_line: i32,
        start_column: i32,
        end_line: i32,
        end_column: i32,
        script: Rc<RefCell<v8::Global<v8::debug::Script>>>,
        script_source: Rc<RefCell<v8::Global<v8::debug::ScriptSource>>>,
    }

    impl ActualScript {
        fn new(
            isolate: *mut v8::Isolate,
            script: v8::Local<v8::debug::Script>,
            is_live_edit: bool,
            agent: *mut V8DebuggerAgentImpl,
            client: *mut V8InspectorClient,
        ) -> Result<Self, String> {
            let id = String16::from_integer(script.id());
            let url = Self::get_script_url(isolate, script, client);
            let name = Self::get_script_name(isolate, script, client);

            let mut actual_script = ActualScript {
                base: V8DebuggerScript::new(isolate, id, url, name),
                agent,
                source_mapping_url: String16::from_str(""),
                language: Language::JavaScript,
                is_live_edit,
                is_module: false,
                hash: RefCell::new(String16::from_str("")),
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 0,
                script: Rc::new(RefCell::new(v8::Global::new())),
                script_source: Rc::new(RefCell::new(v8::Global::new())),
            };
            actual_script.initialize(script)?;
            Ok(actual_script)
        }

        fn get_script_url(
            isolate: *mut v8::Isolate,
            script: v8::Local<v8::debug::Script>,
            client: *mut V8InspectorClient,
        ) -> String16 {
            let mut source_url = v8::Local::<v8::String>::new();
            if script.source_url().to_local(&mut source_url) && source_url.length() > 0 {
                return source_url.into();
            }
            Self::get_script_name(isolate, script, client)
        }

        fn get_script_name(
            isolate: *mut v8::Isolate,
            script: v8::Local<v8::debug::Script>,
            client: *mut V8InspectorClient,
        ) -> String16 {
            let mut v8_name = v8::Local::<v8::String>::new();
            if script.name().to_local(&mut v8_name) && v8_name.length() > 0 {
                let name: String16 = v8_name.into();
                let url = unsafe {
                    (*client).resource_name_to_url(name.to_string().as_str())
                };
                if let Some(url) = url {
                    return url.to_string().into();
                } else {
                    return name;
                }
            }
            String16::from_str("")
        }

        fn initialize(&mut self, script: v8::Local<v8::debug::Script>) -> Result<(), String> {
            let mut tmp = v8::Local::<v8::String>::new();
            self.base.has_source_url_comment = script.source_url().to_local(&mut tmp) && tmp.length() > 0;

            let mut tmp2 = v8::Local::<v8::String>::new();
            if script.source_mapping_url().to_local(&mut tmp2){
                self.source_mapping_url = tmp2.into();
            }
           
            self.start_line = script.start_line();
            self.start_column = script.start_column();
            self.end_line = script.end_line();
            self.end_column = script.end_column();

            script.context_id();
            self.base.execution_context_id = script.context_id() as i32;
            self.language = Language::JavaScript;

            #[cfg(feature = "v8_enable_webassembly")]
            {
                if script.is_wasm() {
                    self.language = Language::WebAssembly;
                }
            }

            self.is_module = script.is_module();

            let mut tmp = v8::Local::<v8::String>::new();
            let has_hash = script.get_sha256_hash().to_local(&mut tmp) && tmp.length() > 0;
            if has_hash {
                *self.hash.borrow_mut() = tmp.into();
            }

            *self.script.borrow_mut() = v8::Global::new();
            self.script.borrow_mut().set_weak(K_GLOBAL_DEBUGGER_SCRIPT_HANDLE_LABEL);
            *self.script_source.borrow_mut() = v8::Global::new();
            self.script_source.borrow_mut().set_weak(K_GLOBAL_DEBUGGER_SCRIPT_HANDLE_LABEL);
            Ok(())
        }

        fn weak_callback(&self) {
            self.script.borrow_mut().reset();
            unsafe {
                (*self.agent).script_collected(self);
            }
        }

        fn get_v8_script(&self) -> Result<v8::Local<v8::debug::Script>, String> {
           todo!()
        }
    }

    impl ScriptTrait for ActualScript {
        fn source_mapping_url(&self) -> &String16 {
            &self.source_mapping_url
        }

        fn source(&self, pos: usize, len: usize) -> Result<String16, String> {
            let isolate = self.base.isolate;
            let mut v8_source = v8::Local::<v8::String>::new();
            self.script_source.borrow().get(isolate).java_script_code().to_local(&mut v8_source);
            if pos >= v8_source.length() {
                return Ok(String16::from_str(""));
            }
            let substring_length = std::cmp::min(len, v8_source.length() - pos);
            let mut buffer: Vec<u16> = vec![0; substring_length];
            v8_source.write_v2(
                unsafe { &mut *isolate },
                pos,
                substring_length,
                buffer.as_mut_ptr(),
            );
            Ok(String16::from_utf16(&buffer))
        }

        fn language(&self) -> Language {
            self.language
        }

        fn hash(&self) -> &String16 {
            if !self.hash.borrow().is_empty() {
                return &self.hash.borrow();
            }
            let isolate = self.base.isolate;
            let mut v8_source = v8::Local::<v8::String>::new();
            self.script_source.borrow().get(isolate).java_script_code().to_local(&mut v8_source);
            *self.hash.borrow_mut() = calculate_hash(isolate, v8_source);
            assert!(!self.hash.borrow().is_empty());
            &self.hash.borrow()
        }

        fn build_id(&self) -> Result<String16, String> {
            #[cfg(feature = "v8_enable_webassembly")]
            {
                if self.language == Language::WebAssembly {
                    let script = self.get_v8_script()?;
                    let maybe_build_id = script.wasm_script().get_module_build_id();
                    if maybe_build_id.is_just() {
                        let build_id = maybe_build_id.from_just();
                        let mut build_id_formatter = String16Builder::new();
                        for i in 0..build_id.size() {
                            build_id_formatter.append_unsigned_as_hex(build_id[i]);
                        }
                        return Ok(build_id_formatter.to_string());
                    }
                }
            }
            Ok(String16::from_str(""))
        }

        fn start_line(&self) -> i32 {
            self.start_line
        }

        fn start_column(&self) -> i32 {
            self.start_column
        }

        fn end_line(&self) -> i32 {
            self.end_line
        }

        fn end_column(&self) -> i32 {
            self.end_column
        }

        fn code_offset(&self) -> i32 {
            #[cfg(feature = "v8_enable_webassembly")]
            {
                if self.get_v8_script().unwrap().is_wasm() {
                    return self.get_v8_script().unwrap().wasm_script().code_offset();
                }
            }
            0
        }

        fn is_live_edit(&self) -> bool {
            self.is_live_edit
        }

        fn is_module(&self) -> bool {
            self.is_module
        }

        fn length(&self) -> Result<i32, String> {
            Ok(self.script_source.borrow().get(self.base.isolate).length() as i32)
        }

        fn set_source_mapping_url(&mut self, source_mapping_url: String16) {
            self.source_mapping_url = source_mapping_url;
        }

        fn set_source(
            &mut self,
            new_source: &String16,
            preview: bool,
            allow_top_frame_live_editing: bool,
            result: &mut v8::debug::LiveEditResult,
        ) -> Result<(), String> {
            let isolate = self.base.isolate;
            let v8_source = new_source.to_v8_string(unsafe {&mut *isolate});

            if !self.script.borrow().get(isolate).set_script_source(v8_source, preview, allow_top_frame_live_editing, result) {
                result.message = result.message;
                return Ok(());
            }
           
            if preview || result.script.is_empty() {
                return Ok(());
            }

            *self.hash.borrow_mut() = String16::from_str("");
            self.initialize(result.script)?;

            Ok(())
        }

        fn get_possible_breakpoints(
            &self,
            start: &v8::debug::Location,
            end: &v8::debug::Location,
            ignore_nested_functions: bool,
            locations: &mut Vec<v8::debug::BreakLocation>,
        ) -> Result<bool, String> {
            let isolate = self.base.isolate;
            let script = self.get_v8_script()?;
            let mut all_locations: Vec<v8::debug::BreakLocation> = Vec::new();
            if !script.get_possible_breakpoints(start, end, ignore_nested_functions, &mut all_locations) {
                return Ok(false);
            }
            if all_locations.is_empty() {
                return Ok(true);
            }
            let mut current = all_locations[0];
            for i in 1..all_locations.len() {
                if all_locations[i].get_line_number() == current.get_line_number() &&
                    all_locations[i].get_column_number() == current.get_column_number() {
                    if all_locations[i].get_type() != v8::debug::BreakLocationType::kCommonBreakLocation {
                        assert!(all_locations[i].get_type() == v8::debug::BreakLocationType::kCallBreakLocation ||
                            all_locations[i].get_type() == v8::debug::BreakLocationType::kReturnBreakLocation);
                        current = all_locations[i];
                    }
                } else {
                    assert!(
                        all_locations[i].get_line_number() > current.get_line_number() ||
                        (all_locations[i].get_column_number() >= current.get_column_number() &&
                         all_locations[i].get_line_number() == current.get_line_number())
                    );
                    locations.push(current);
                    current = all_locations[i];
                }
            }
            locations.push(current);
            Ok(true)
        }

        fn reset_blackboxed_state_cache(&self) -> Result<(), String> {
           todo!()
        }

        fn offset(&self, line_number: i32, column_number: i32) -> Result<Option<i32>, String> {
            let isolate = self.base.isolate;
            let script = self.get_v8_script()?;
            let maybe_offset = script.get_source_offset(v8::debug::Location::new(line_number, column_number));
            if maybe_offset.is_nothing() {
                return Ok(None);
            }
            Ok(Some(maybe_offset.from_just()))
        }

        fn location(&self, offset: i32) -> Result<v8::debug::Location, String> {
            let isolate = self.base.isolate;
            let script = self.get_v8_script()?;
            Ok(script.get_source_location(offset))
        }

        fn set_breakpoint(
            &self,
            condition: &String16,
            location: &mut v8::debug::Location,
            id: &mut i32,
        ) -> Result<bool, String> {
           todo!()
        }

        fn make_weak(&self) -> Result<(), String> {
           todo!()
        }

        fn set_instrumentation_breakpoint(&self, id: &mut i32) -> Result<bool, String> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        fn wasm_bytecode(&self) -> Result<Option<v8::MemorySpan<u8>>, String> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        fn get_debug_symbols(&self) -> Result<Vec<v8::debug::WasmScript::DebugSymbols>, String> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        fn disassemble(
            &self,
            collector: &mut v8::debug::DisassemblyCollector,
            function_body_offsets: &mut Vec<i32>,
        ) -> Result<(), String> {
            todo!()
        }
    }
}

pub mod v8 {
    pub mod debug {
        #[derive(Debug, Clone, Copy)]
        pub struct Location {
            line_number: i32,
            column_number: i32,
        }

        impl Location {
            pub fn new(line_number: i32, column_number: i32) -> Self {
                Location { line_number, column_number }
            }

            pub fn get_line_number(&self) -> i32 {
                self.line_number
            }

            pub fn get_column_number(&self) -> i32 {
                self.column_number
            }
        }

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum BreakLocationType {
            kCommonBreakLocation,
            kCallBreakLocation,
            kReturnBreakLocation,
        }

        #[derive(Debug, Clone)]
        pub struct BreakLocation {
            line_number: i32,
            column_number: i32,
            type_: BreakLocationType,
        }

        impl BreakLocation {
            pub fn get_line_number(&self) -> i32 {
                self.line_number
            }

            pub fn get_column_number(&self) -> i32 {
                self.column_number
            }

            pub fn get_type(&self) -> BreakLocationType {
                self.type_
            }
        }

        pub struct Script {}

        impl Script {
            pub fn id(&self) -> i32 {
                0
            }
            pub fn source_url(&self) -> MaybeLocal<String> {
                MaybeLocal::empty()
            }
            pub fn name(&self) -> MaybeLocal<String> {
                MaybeLocal::empty()
            }
             pub fn context_id(&self) -> i32{
                0
            }
            pub fn source_mapping_url(&self) -> MaybeLocal<String> {
                 MaybeLocal::empty()
            }
            pub fn get_sha256_hash(&self) -> MaybeLocal<String> {
                MaybeLocal::empty()
            }
            pub fn is_wasm(&self) -> bool{
                false
            }
            pub fn is_module(&self) -> bool{
                false
            }

            pub fn get_possible_breakpoints(
                &self,
                start: &Location,
                end: &Location,
                ignore_nested_functions: bool,
                locations: &mut Vec<BreakLocation>,
            ) -> bool {
                true
            }

             pub fn get_source_offset(&self, location: Location) -> Maybe<i32> {
                 Maybe::nothing()
             }

             pub fn get_source_location(&self, offset: i32) -> Location {
                 Location::new(0, 0)
             }

             pub fn wasm_script(&self) -> WasmScript{
                 WasmScript{}
             }
            pub fn set_script_source(
                &self,
                v8_source: Local<String>,
                preview: bool,
                allow_top_frame_live_editing: bool,
                result: &mut LiveEditResult,
            ) -> bool{
                false
            }
        }

         pub struct WasmScript {}

        impl WasmScript {
            pub fn code_offset(&self) -> i32 {
                0
            }

            pub fn get_module_build_id(&self) -> Maybe<MemorySpan<u8>> {
                Maybe::nothing()
            }
        }

        pub struct LiveEditResult {
           pub message: Local<String>,
           pub script: Local<Script>,
        }

        pub struct ScriptSource {
             }
         impl ScriptSource {
            pub fn java_script_code(&self) -> MaybeLocal<String> {
                 MaybeLocal::empty()
             }
             pub fn wasm_bytecode(&self) -> MaybeLocal<MemorySpan<u8>> {
                 MaybeLocal::empty()
             }

             pub fn length(&self) -> usize{
                 0
             }
         }
    }

    pub struct Isolate {}
    pub struct String {}
    pub struct Global<T> {
        _marker: std::marker::PhantomData<T>,
        is_weak: bool,
        weak_callback: Option<fn()>,
        label: String,
    }

    impl<T> Global<T> {
        pub fn new() -> Self {
            Global {
                _marker: std::marker::PhantomData,
                is_weak: false,
                weak_callback: None,
                label: String::from("")
            }
        }

        pub fn set_weak(&mut self, label: &str) {
            self.is_weak = true;
            self.label = String::from(label);
        }

        pub fn reset(&mut self) {
            self.is_weak = false;
            self.weak_callback = None;
        }

         pub fn get(&self, _isolate: *mut Isolate) -> Local<T>{
            Local::new()
        }
    }

    pub struct Local<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Local<T> {
        pub fn new() -> Self {
            Local {
                _marker: std::marker::PhantomData,
            }
        }

        pub fn is_empty(&self) -> bool{
            true
        }
    }

    pub struct MaybeLocal<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> MaybeLocal<T> {
        pub fn empty() -> Self {
            MaybeLocal {
                _marker: std::marker::PhantomData,
            }
        }

         pub fn to_local(&mut self, out: &mut Local<T>) -> bool{
             true
         }
    }

    pub struct Maybe<T> {
        _marker: std::marker::PhantomData<T>,
        is_nothing: bool,
        value: T,
    }

    impl<T> Maybe<T>
        where T: Default
    {
        pub fn nothing() -> Self {
            Maybe {
                _marker: std::marker::PhantomData,
                is_nothing: true,
                value: T::default(),
            }
        }

        pub fn is_just(&self) -> bool {
            !self.is_nothing
        }

        pub fn from_just(&self) -> T {
            self.value
        }
         pub fn is_nothing(&self) -> bool {
             self.is_nothing
         }
    }

    pub struct MemorySpan<T> {
        _marker: std::marker::PhantomData<T>,
        size: usize,
    }

    impl<T> MemorySpan<T> {
        pub fn size(&self) -> usize {
            self.size
        }
    }

    impl String {
        pub fn length(&self) -> usize {
            0
        }

        pub fn write_v2(&self, _isolate: &mut Isolate, _start_index: usize, _max_length: usize, _buffer: *mut u16) {

        }
    }
}
