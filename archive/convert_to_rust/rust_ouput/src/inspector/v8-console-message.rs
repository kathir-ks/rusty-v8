// Converted from V8 C++ source files:
// Header: v8-console-message.h
// Implementation: v8-console-message.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/inspector/v8-console-message.h
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use crate::inspector::protocol::Console;
use crate::inspector::protocol::Runtime;
use crate::inspector::string_util::String16;

pub enum V8MessageOrigin {
    kConsole,
    kException,
    kRevokedException,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleAPIType {
    kLog,
    kDebug,
    kInfo,
    kError,
    kWarning,
    kDir,
    kDirXML,
    kTable,
    kTrace,
    kStartGroup,
    kStartGroupCollapsed,
    kEndGroup,
    kClear,
    kAssert,
    kTimeEnd,
    kCount,
}

pub struct V8StackTraceImpl {}

impl V8StackTraceImpl {
    pub fn is_empty(&self) -> bool {
        true // Placeholder implementation
    }

    pub fn top_source_url(&self) -> String {
        String::new() // Placeholder implementation
    }

    pub fn top_line_number(&self) -> u32 {
        0 // Placeholder implementation
    }

    pub fn top_column_number(&self) -> u32 {
        0 // Placeholder implementation
    }
}

pub struct V8InspectorImpl {}
impl V8InspectorImpl {
    pub fn client(&self) -> &dyn InspectorClient {
        todo!()
    }
    pub fn debugger(&self) -> &Debugger {
        todo!()
    }
    pub fn for_each_session<F>(&self, context_group_id: i32, mut f: F)
    where
        F: FnMut(&V8InspectorSessionImpl),
    {
    }
    pub fn has_console_message_storage(&self, context_group_id: i32) -> bool {
        true
    }
    pub fn get_associated_exception_data_for_protocol(
        &self,
        _exception: v8::Local<v8::Value>,
    ) -> Option<Box<protocol::DictionaryValue>> {
        None
    }
    pub fn get_context(&self, group_id: i32, context_id: i32) -> Option<&InspectedContext> {
        None
    }
}

pub trait InspectorClient {
    fn console_api_message(
        &self,
        group_id: i32,
        level: v8::Isolate::MessageErrorLevel,
        message: std::string::String,
        url: std::string::String,
        line_number: u32,
        column_number: u32,
        stack_trace: Option<&V8StackTraceImpl>,
    );

    fn current_time_ms(&self) -> f64;
}

pub struct Debugger {}

pub struct InspectedContext {}

pub struct V8ConsoleMessage {
    m_origin: V8MessageOrigin,
    m_timestamp: f64,
    m_message: String16,
    m_url: String16,
    m_lineNumber: u32,
    m_columnNumber: u32,
    m_stackTrace: Option<Box<V8StackTraceImpl>>,
    m_scriptId: i32,
    m_contextId: i32,
    m_type: ConsoleAPIType,
    m_exceptionId: u32,
    m_revokedExceptionId: u32,
    m_v8Size: i32,
    m_arguments: Vec<UniquePersistent<v8::Value>>,
    m_detailedMessage: String16,
    m_consoleContext: String16,
}

impl V8ConsoleMessage {
    fn new(origin: V8MessageOrigin, timestamp: f64, message: String16) -> Self {
        V8ConsoleMessage {
            m_origin: origin,
            m_timestamp: timestamp,
            m_message: message,
            m_url: String16::new(),
            m_lineNumber: 0,
            m_columnNumber: 0,
            m_stackTrace: None,
            m_scriptId: 0,
            m_contextId: 0,
            m_type: ConsoleAPIType::kLog,
            m_exceptionId: 0,
            m_revokedExceptionId: 0,
            m_v8Size: 0,
            m_arguments: Vec::new(),
            m_detailedMessage: String16::new(),
            m_consoleContext: String16::new(),
        }
    }

    pub fn create_for_console_api(
        v8_context: v8::Local<v8::Context>,
        context_id: i32,
        group_id: i32,
        inspector: &V8InspectorImpl,
        timestamp: f64,
        type_: ConsoleAPIType,
        arguments: v8::MemorySpan<const v8::Local<v8::Value>>,
        console_context: String16,
        stack_trace: Option<Box<V8StackTraceImpl>>,
    ) -> Result<Box<V8ConsoleMessage>, String> {
        let isolate = v8_context.GetIsolate();

        let mut message = V8ConsoleMessage::new(V8MessageOrigin::kConsole, timestamp, String16::new());

        if let Some(stack_trace) = &stack_trace {
            if !stack_trace.is_empty() {
                message.m_url = String16::from(stack_trace.top_source_url());
                message.m_lineNumber = stack_trace.top_line_number();
                message.m_columnNumber = stack_trace.top_column_number();
            }
        }
        message.m_stackTrace = stack_trace;
        message.m_consoleContext = console_context;
        message.m_type = type_;
        message.m_contextId = context_id;

        for arg in arguments.into_iter() {
            let argument = UniquePersistent::new(isolate, arg);
            message.m_v8Size += 100; // Placeholder for EstimatedValueSize
            message.m_arguments.push(argument);
        }

        let mut sep = false;
        for arg in arguments.into_iter() {
            if sep {
                message.m_message += String16::from(" ");
            } else {
                sep = true;
            }
            message.m_message += V8ValueStringBuilder::to_string(arg.clone(), v8_context)?;
        }

        let client_level = match type_ {
            ConsoleAPIType::kDebug | ConsoleAPIType::kCount | ConsoleAPIType::kTimeEnd => {
                v8::Isolate::MessageErrorLevel::kMessageDebug
            }
            ConsoleAPIType::kError | ConsoleAPIType::kAssert => {
                v8::Isolate::MessageErrorLevel::kMessageError
            }
            ConsoleAPIType::kWarning => v8::Isolate::MessageErrorLevel::kMessageWarning,
            ConsoleAPIType::kInfo => v8::Isolate::MessageErrorLevel::kMessageInfo,
            ConsoleAPIType::kLog => v8::Isolate::MessageErrorLevel::kMessageLog,
            _ => v8::Isolate::MessageErrorLevel::kMessageInfo,
        };

        if type_ != ConsoleAPIType::kClear {
            inspector.client().console_api_message(
                group_id,
                client_level,
                message.m_message.to_string(),
                message.m_url.to_string(),
                message.m_lineNumber,
                message.m_columnNumber,
                message.m_stackTrace.as_deref(),
            );
        }

        Ok(Box::new(message))
    }

    pub fn create_for_exception(
        timestamp: f64,
        detailed_message: String16,
        url: String16,
        line_number: u32,
        column_number: u32,
        stack_trace: Option<Box<V8StackTraceImpl>>,
        script_id: i32,
        isolate: *mut Isolate,
        message: String16,
        context_id: i32,
        exception: v8::Local<v8::Value>,
        exception_id: u32,
    ) -> Result<Box<V8ConsoleMessage>, String> {
        let mut console_message = V8ConsoleMessage::new(V8MessageOrigin::kException, timestamp, message);
        console_message.set_location(url, line_number, column_number, stack_trace, script_id);
        console_message.m_exceptionId = exception_id;
        console_message.m_detailedMessage = detailed_message;
        if context_id != 0 {
            console_message.m_contextId = context_id;
            let exception_persistent = UniquePersistent::new(unsafe { &mut *isolate }, &exception);
            console_message.m_v8Size += 100; // Placeholder
            console_message.m_arguments.push(exception_persistent);
        }
        Ok(Box::new(console_message))
    }

    pub fn create_for_revoked_exception(
        timestamp: f64,
        message_text: String16,
        revoked_exception_id: u32,
    ) -> Result<Box<V8ConsoleMessage>, String> {
        let mut message = V8ConsoleMessage::new(
            V8MessageOrigin::kRevokedException,
            timestamp,
            message_text,
        );
        message.m_revokedExceptionId = revoked_exception_id;
        Ok(Box::new(message))
    }

    pub fn origin(&self) -> &V8MessageOrigin {
        &self.m_origin
    }

    pub fn report_to_frontend(&self, frontend: &mut dyn Console::Frontend) {
        assert_eq!(self.m_origin, V8MessageOrigin::kConsole);

        let level = match self.m_type {
            ConsoleAPIType::kDebug | ConsoleAPIType::kCount | ConsoleAPIType::kTimeEnd => {
                Console::ConsoleMessage::LevelEnum::Debug
            }
            ConsoleAPIType::kError | ConsoleAPIType::kAssert => {
                Console::ConsoleMessage::LevelEnum::Error
            }
            ConsoleAPIType::kWarning => Console::ConsoleMessage::LevelEnum::Warning,
            ConsoleAPIType::kInfo => Console::ConsoleMessage::LevelEnum::Info,
            _ => Console::ConsoleMessage::LevelEnum::Log,
        };

        let mut result = Console::ConsoleMessage::create()
            .set_source(Console::ConsoleMessage::SourceEnum::ConsoleApi)
            .set_level(level)
            .set_text(self.m_message.to_string())
            .build();

        if self.m_lineNumber != 0 {
            result.set_line(self.m_lineNumber);
        }
        if self.m_columnNumber != 0 {
            result.set_column(self.m_columnNumber);
        }
        if !self.m_url.is_empty() {
            result.set_url(self.m_url.to_string());
        }

        frontend.message_added(result);
    }

    pub fn report_to_frontend_runtime(
        &self,
        frontend: &mut dyn Runtime::Frontend,
        session: &mut V8InspectorSessionImpl,
        generate_preview: bool,
    ) {
        let context_group_id = session.context_group_id();
        let inspector = session.inspector();

        if self.m_origin == V8MessageOrigin::kException {
            let exception = self.wrap_exception(session, generate_preview);
            if !inspector.has_console_message_storage(context_group_id) {
                return;
            }

            let mut exception_details = Runtime::ExceptionDetails::create()
                .set_exception_id(self.m_exceptionId)
                .set_text(match &exception {
                    Some(_) => self.m_message.to_string(),
                    None => self.m_detailedMessage.to_string(),
                })
                .set_line_number(if self.m_lineNumber != 0 {
                    self.m_lineNumber - 1
                } else {
                    0
                })
                .set_column_number(if self.m_columnNumber != 0 {
                    self.m_columnNumber - 1
                } else {
                    0
                })
                .build();

            if self.m_scriptId != 0 {
                exception_details.set_script_id(String16::from(self.m_scriptId.to_string()));
            }

            if !self.m_url.is_empty() {
                exception_details.set_url(self.m_url.to_string());
            }

            if let Some(ref stack_trace) = self.m_stackTrace {
                exception_details.set_stack_trace(
                    stack_trace.build_inspector_object_impl(inspector.debugger()),
                );
            }
            if self.m_contextId != 0 {
                exception_details.set_execution_context_id(self.m_contextId);
            }
            if let Some(exception) = exception {
                exception_details.set_exception(exception);
            }
            let data = self.get_associated_exception_data(inspector, session);
            if let Some(data) = data {
                exception_details.set_exception_meta_data(data);
            }
            frontend.exception_thrown(self.m_timestamp, exception_details);
            return;
        }
        if self.m_origin == V8MessageOrigin::kRevokedException {
            frontend.exception_revoked(self.m_message.to_string(), self.m_revokedExceptionId);
            return;
        }
        if self.m_origin == V8MessageOrigin::kConsole {
            let mut arguments = self.wrap_arguments(session, generate_preview);
            if !inspector.has_console_message_storage(context_group_id) {
                return;
            }
            if arguments.is_none() {
                arguments = Some(Box::new(Vec::new()));
                if !self.m_message.is_empty() {
                    let mut message_arg = Runtime::RemoteObject::create()
                        .set_type(Runtime::RemoteObject::TypeEnum::String)
                        .build();
                    message_arg.set_value(protocol::StringValue::create(self.m_message.to_string()));
                    arguments.as_mut().unwrap().push(message_arg);
                }
            }

            let console_context = if !self.m_consoleContext.is_empty() {
                Some(self.m_consoleContext.to_string())
            } else {
                None
            };

            let stack_trace = self.m_stackTrace.as_ref().map(|st| match self.m_type {
                ConsoleAPIType::kAssert
                | ConsoleAPIType::kError
                | ConsoleAPIType::kTrace
                | ConsoleAPIType::kWarning => st.build_inspector_object_impl(inspector.debugger()),
                _ => st.build_inspector_object_impl(inspector.debugger(), 0),
            });

            frontend.console_api_called(
                console_api_type_value(self.m_type),
                arguments.unwrap(),
                self.m_contextId,
                self.m_timestamp,
                stack_trace,
                console_context,
            );
            return;
        }
        unreachable!();
    }

    pub fn type_(&self) -> ConsoleAPIType {
        self.m_type
    }

    pub fn context_destroyed(&mut self, context_id: i32) {
        if context_id != self.m_contextId {
            return;
        }
        self.m_contextId = 0;
        if self.m_message.is_empty() {
            self.m_message = String16::from("<message collected>");
        }
        self.m_arguments.clear();
        self.m_v8Size = 0;
    }

    pub fn estimated_size(&self) -> i32 {
        self.m_v8Size + (self.m_message.len() * mem::size_of::<u16>()) as i32
    }

    fn wrap_arguments(
        &self,
        session: &mut V8InspectorSessionImpl,
        generate_preview: bool,
    ) -> Option<Box<Vec<protocol::Runtime::RemoteObject>>> {
        let inspector = session.inspector();
        let context_group_id = session.context_group_id();
        let context_id = self.m_contextId;
        if self.m_arguments.is_empty() || context_id == 0 {
            return None;
        }

        let inspected_context = inspector.get_context(context_group_id, context_id)?;
        let isolate = inspected_context.isolate();
        let context = inspected_context.context();

        let mut args = Vec::new();

        if let Some(value) = self.m_arguments.get(0) {
          
            if value.get(isolate).IsObject() && self.m_type == ConsoleAPIType::kTable && generate_preview {
              
                let second_argument: Option<v8::Local<v8::Value>> = self.m_arguments.get(1).map(|v| v.get(isolate));
                let columns = match second_argument {
                    Some(arg) if arg.IsArray() => Some(arg.As::<v8::Array>()),
                    Some(arg) if arg.IsString() => {
                      todo!()
                    }
                    _ => None,
                };

                let wrapped = session.wrap_table(context, value.get(isolate).As::<v8::Object>(), columns);
              
                if let Some(inspected_context) = inspector.get_context(context_group_id, context_id) {
                  
                    if let Some(wrapped) = wrapped {
                        args.push(wrapped);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
              
                for argument in &self.m_arguments {
                    let wrapped = session.wrap_object(
                        context,
                        argument.get(isolate),
                        "console",
                        generate_preview,
                    );
                  
                    if let Some(inspected_context) = inspector.get_context(context_group_id, context_id) {
                        if let Some(wrapped) = wrapped {
                            args.push(wrapped);
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(Box::new(args))
    }

    fn wrap_exception(
        &self,
        session: &mut V8InspectorSessionImpl,
        generate_preview: bool,
    ) -> Option<Box<protocol::Runtime::RemoteObject>> {
        if self.m_arguments.is_empty() || self.m_contextId == 0 {
            return None;
        }
        assert_eq!(self.m_arguments.len(), 1);

        let inspected_context = session
            .inspector()
            .get_context(session.context_group_id(), self.m_contextId)?;

        let isolate = inspected_context.isolate();
        let context = inspected_context.context();
        let value = self.m_arguments[0].get(isolate);

        session.wrap_object(context, value, "console", generate_preview)
    }

    fn set_location(
        &mut self,
        url: String16,
        line_number: u32,
        column_number: u32,
        stack_trace: Option<Box<V8StackTraceImpl>>,
        script_id: i32,
    ) {
        let data_uri_prefix = "data:";
        if url.starts_with(data_uri_prefix) {
            self.m_url = String16::new();
        } else {
            self.m_url = url;
        }
        self.m_lineNumber = line_number;
        self.m_columnNumber = column_number;
        self.m_stackTrace = stack_trace;
        self.m_scriptId = script_id;
    }
    fn get_associated_exception_data(
        &self,
        inspector: &V8InspectorImpl,
        session: &mut V8InspectorSessionImpl,
    ) -> Option<Box<protocol::DictionaryValue>> {
        if self.m_arguments.is_empty() || self.m_contextId == 0 {
            return None;
        }
        assert_eq!(self.m_arguments.len(), 1);

        let inspected_context = session
            .inspector()
            .get_context(session.context_group_id(), self.m_contextId)?;

        let isolate = inspected_context.isolate();
        let exception = self.m_arguments[0].get(isolate);
        inspector.get_associated_exception_data_for_protocol(exception)
    }
}

impl Drop for V8ConsoleMessage {
    fn drop(&mut self) {
        // drop all persistant v8 values
    }
}

fn console_api_type_value(type_: ConsoleAPIType) -> String {
    match type_ {
        ConsoleAPIType::kLog => Runtime::ConsoleAPICalled::TypeEnum::Log.to_string(),
        ConsoleAPIType::kDebug => Runtime::ConsoleAPICalled::TypeEnum::Debug.to_string(),
        ConsoleAPIType::kInfo => Runtime::ConsoleAPICalled::TypeEnum::Info.to_string(),
        ConsoleAPIType::kError => Runtime::ConsoleAPICalled::TypeEnum::Error.to_string(),
        ConsoleAPIType::kWarning => Runtime::ConsoleAPICalled::TypeEnum::Warning.to_string(),
        ConsoleAPIType::kClear => Runtime::ConsoleAPICalled::TypeEnum::Clear.to_string(),
        ConsoleAPIType::kDir => Runtime::ConsoleAPICalled::TypeEnum::Dir.to_string(),
        ConsoleAPIType::kDirXML => Runtime::ConsoleAPICalled::TypeEnum::Dirxml.to_string(),
        ConsoleAPIType::kTable => Runtime::ConsoleAPICalled::TypeEnum::Table.to_string(),
        ConsoleAPIType::kTrace => Runtime::ConsoleAPICalled::TypeEnum::Trace.to_string(),
        ConsoleAPIType::kStartGroup => Runtime::ConsoleAPICalled::TypeEnum::StartGroup.to_string(),
        ConsoleAPIType::kStartGroupCollapsed => {
            Runtime::ConsoleAPICalled::TypeEnum::StartGroupCollapsed.to_string()
        }
        ConsoleAPIType::kEndGroup => Runtime::ConsoleAPICalled::TypeEnum::EndGroup.to_string(),
        ConsoleAPIType::kAssert => Runtime::ConsoleAPICalled::TypeEnum::Assert.to_string(),
        ConsoleAPIType::kTimeEnd => Runtime::ConsoleAPICalled::TypeEnum::TimeEnd.to_string(),
        ConsoleAPIType::kCount => Runtime::ConsoleAPICalled::TypeEnum::Count.to_string(),
    }
}

struct V8ValueStringBuilder {}

impl V8ValueStringBuilder {
    pub fn to_string(
        value: v8::Local<v8::Value>,
        context: v8::Local<v8::Context>,
    ) -> Result<String16, String> {
        let builder = V8ValueStringBuilder {};
        let result = builder.append(value, context)?;
        Ok(result)
    }

    fn append(
        &self,
        value: v8::Local<v8::Value>,
        context: v8::Local<v8::Context>,
    ) -> Result<String16, String> {
        if value.IsEmpty() {
            return Ok(String16::new());
        }
        Ok(String16::new())
    }
}

const MAX_CONSOLE_MESSAGE_COUNT: usize = 1000;
const MAX_CONSOLE_MESSAGE_V8_SIZE: usize = 10 * 1024 * 1024;

pub struct V8ConsoleMessageStorage {
    m_inspector: *mut V8InspectorImpl,
    m_contextGroupId: i32,
    m_estimatedSize: i32,
    m_messages: Vec<Box<V8ConsoleMessage>>,
    m_data: HashMap<i32, PerContextData>,
}

impl V8ConsoleMessageStorage {
    pub fn new(inspector: *mut V8InspectorImpl, context_group_id: i32) -> Self {
        V8ConsoleMessageStorage {
            m_inspector: inspector,
            m_contextGroupId: context_group_id,
            m_estimatedSize: 0,
            m_messages: Vec::new(),
            m_data: HashMap::new(),
        }
    }

    pub fn context_group_id(&self) -> i32 {
        self.m_contextGroupId
    }

    pub fn messages(&self) -> &Vec<Box<V8ConsoleMessage>> {
        &self.m_messages
    }

    pub fn add_message(&mut self, message: Box<V8ConsoleMessage>) {
        let context_group_id = self.m_contextGroupId;
        let inspector = unsafe { &*self.m_inspector };

        if message.type_() == ConsoleAPIType::kClear {
            self.clear();
        }

        trace_v8_console_message_event(message.origin(), message.type_());

        inspector.for_each_session(context_group_id, |session| {
            if *message.origin() == V8MessageOrigin::kConsole {
                session.console_agent().message_added(&*message);
            }
            session.runtime_agent().message_added(&*message);
        });

        if !inspector.has_console_message_storage(context_group_id) {
            return;
        }

        assert!(self.m_messages.len() <= MAX_CONSOLE_MESSAGE_COUNT);
        if self.m_messages.len() == MAX_CONSOLE_MESSAGE_COUNT {
            self.m_estimatedSize -= self.m_messages.front().as_ref().unwrap().estimated_size();
            self.m_messages.remove(0);
        }

        while (self.m_estimatedSize + message.estimated_size()) as usize > MAX_CONSOLE_MESSAGE_V8_SIZE
            && !self.m_messages.is_empty()
        {
            self.m_estimatedSize -= self.m_messages.front().as_ref().unwrap().estimated_size();
            self.m_messages.remove(0);
        }

        self.m_estimatedSize += message.estimated_size();
        self.m_messages.push(message);
    }

    pub fn context_destroyed(&mut self, context_id: i32) {
        self.m_estimatedSize = 0;
        for message in &mut self.m_messages {
            message.context_destroyed(context_id);
            self.m_estimatedSize += message.estimated_size();
        }

        self.m_data.remove(&context_id);
    }

    pub fn clear(&mut self) {
        self.m_messages.clear();
        self.m_estimatedSize = 0;
        let inspector = unsafe { &*self.m_inspector };
        inspector.for_each_session(self.m_contextGroupId, |session| {
            session.release_object_group("console");
        });
        for data in &mut self.m_data {
            data.1.m_counters.clear();
            data.1.m_reportedDeprecationMessages.clear();
        }
    }

    pub fn should_report_deprecation_message(&mut self, context_id: i32, method: String16) -> bool {
        let entry = self.m_data.entry(context_id).or_insert(PerContextData::default());
        if entry.m_reportedDeprecationMessages.contains(&method) {
            return false;
        }
        entry.m_reportedDeprecationMessages.insert(method);
        true
    }

    pub fn count(&mut self, context_id: i32, console_context_id: i32, label: String16) -> i32 {
        let entry = self.m_data.entry(context_id).or_insert(PerContextData::default());
        let key = LabelKey {
            context_id: console_context_id,
            label,
        };
        let counter = entry.m_counters.entry(key).or_insert(0);
        *counter += 1;
        *counter
    }

    pub fn count_reset(&mut self, context_id: i32, console_context_id: i32, label: String16) -> bool {
        let entry = self.m_data.entry(context_id).or_insert(PerContextData::default());
        let key = LabelKey {
            context_id: console_context_id,
            label,
        };
        if entry.m_counters.contains_key(&key) {
            entry.m_counters.remove(&key);
            true
        } else {
            false
        }
    }

    pub fn time(&mut self, context_id: i32, console_context_id: i32, label: String16) -> bool {
        let entry = self.m_data.entry(context_id).or_insert(PerContextData::default());
        let key = LabelKey {
            context_id: console_context_id,
            label,
        };
        let inspector = unsafe { &*self.m_inspector };
        entry
            .m_timers
            .entry(key)
            .or_insert_with(|| inspector.client().current_time_ms());
        true
    }

    pub fn time_log(&mut self, context_id: i32, console_context_id: i32, label: String16) -> Option<f64> {
        let entry = self.m_data.get_mut(&context_id)?;
        let key = LabelKey {
            context_id: console_context_id,
            label,
        };
        let timer = entry.m_timers.get(&key)?;
        let inspector = unsafe { &*self.m_inspector };
        Some(inspector.client().current_time_ms() - timer)
    }

    pub fn time_end(&mut self, context_id: i32, console_context_id: i32, label: String16) -> Option<f64> {
        let entry = self.m_data.get_mut(&context_id)?;
        let key = LabelKey {
            context_id: console_context_id,
            label,
        };
        let timer = entry.m_timers.remove(&key)?;
        let inspector = unsafe { &*self.m_inspector };
        Some(inspector.client().current_time_ms() - timer)
    }
}

impl Drop for V8ConsoleMessageStorage {
    fn drop(&mut self) {
        self.clear();
    }
}

fn trace_v8_console_message_event(origin: &V8MessageOrigin, type_: ConsoleAPIType) {
    match origin {
        V8MessageOrigin::kException => {
            // Placeholder
        }
        _ => match type_ {
            ConsoleAPIType::kError => {
                // Placeholder
            }
            ConsoleAPIType::kAssert => {
                // Placeholder
            }
            _ => {}
        },
    }
}

#[derive(Default)]
struct PerContextData {
    m_reportedDeprecationMessages: HashSet<String16>,
    m_counters: HashMap<LabelKey, i32>,
    m_timers: HashMap<LabelKey, f64>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct LabelKey {
    context_id: i32,
    label: String16,
}

pub struct V8InspectorSessionImpl {}
impl V8InspectorSessionImpl {
    pub fn context_group_id(&self) -> i32 {
        0
    }
    pub fn inspector(&mut self) -> &mut V8InspectorImpl {
        todo!()
    }
    pub fn console_agent(&mut self) -> &ConsoleAgentImpl {
        todo!()
    }
    pub fn runtime_agent(&mut self) -> &RuntimeAgentImpl {
        todo!()
    }
    pub fn release_object_group(&mut self, group_name: &str) {}
    pub fn wrap_object(
        &mut self,
        context: v8::Local<v8::Context>,
        object: v8::Local<v8::Value>,
        group_name: &str,
        generate_preview: bool,
    ) -> Option<Box<protocol::Runtime::RemoteObject>> {
        Some(Box::new(protocol::Runtime::RemoteObject::create().set_type(protocol::Runtime::RemoteObject::TypeEnum::String).build()))
    }
    fn wrap_table(
        &mut self,
        context: v8::Local<v8::Context>,
        object: v8::Local<v8::Object>,
        columns: Option<v8::Local<v8::Array>>,
    ) -> Option<Box<protocol::Runtime::RemoteObject>>{
        Some(Box::new(protocol::Runtime::RemoteObject::create().set_type(protocol::Runtime::RemoteObject::TypeEnum::String).build()))
    }
}

pub struct ConsoleAgentImpl {}
impl ConsoleAgentImpl {
    pub fn message_added(&mut self, message: &V8ConsoleMessage) {}
}
pub struct RuntimeAgentImpl {}
impl RuntimeAgentImpl {
    pub fn message_added(&mut self, message: &V8ConsoleMessage
