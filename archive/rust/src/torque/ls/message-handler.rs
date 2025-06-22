// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::io::{self, Read, Write};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Placeholder for TorqueCompilerResult and other types.  Replace with actual definitions.
mod torque {
    pub mod ls {
        pub struct PublishDiagnosticsNotification {
            method: String,
            params: PublishDiagnosticsParams,
        }

        impl PublishDiagnosticsNotification {
            pub fn new() -> Self {
                PublishDiagnosticsNotification {
                    method: String::new(),
                    params: PublishDiagnosticsParams::new(),
                }
            }

            pub fn set_method(&mut self, method: &str) {
                self.method = method.to_string();
            }

            pub fn params(&mut self) -> &mut PublishDiagnosticsParams {
                &mut self.params
            }

            pub fn get_json_value(self) -> JsonValue {
                // Placeholder implementation - replace with actual serialization
                JsonValue::String(format!("PublishDiagnosticsNotification: method={}, uri={}", self.method, self.params.uri))
            }
        }

        pub struct PublishDiagnosticsParams {
            pub uri: String,
            pub diagnostics: Vec<Diagnostic>,
        }

        impl PublishDiagnosticsParams {
            pub fn new() -> Self {
                PublishDiagnosticsParams {
                    uri: String::new(),
                    diagnostics: Vec::new(),
                }
            }

            pub fn set_uri(&mut self, uri: String) {
                self.uri = uri;
            }

            pub fn add_diagnostics(&mut self) -> &mut Diagnostic {
                self.diagnostics.push(Diagnostic::new());
                self.diagnostics.last_mut().unwrap()
            }
            
            pub fn diagnostics_size(&self) -> usize {
                self.diagnostics.len()
            }
        }

        pub struct Diagnostic {
            severity: DiagnosticSeverity,
            message: String,
            source: String,
            range: Range,
        }

        impl Diagnostic {
            pub fn new() -> Self {
                Diagnostic {
                    severity: DiagnosticSeverity::Error, // Default value
                    message: String::new(),
                    source: String::new(),
                    range: Range::new(),
                }
            }

            pub fn set_severity(&mut self, severity: DiagnosticSeverity) {
                self.severity = severity;
            }

            pub fn set_message(&mut self, message: String) {
                self.message = message;
            }

            pub fn set_source(&mut self, source: String) {
                self.source = source;
            }

            pub fn range(&mut self) -> &mut Range {
                &mut self.range
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub enum DiagnosticSeverity {
            Error,
            Warning,
            Information,
            Hint,
        }

        pub struct Range {
            start: Position,
            end: Position,
        }

        impl Range {
            pub fn new() -> Self {
                Range {
                    start: Position::new(),
                    end: Position::new(),
                }
            }

            pub fn start(&mut self) -> &mut Position {
                &mut self.start
            }

            pub fn end(&mut self) -> &mut Position {
                &mut self.end
            }
        }

        pub struct Position {
            line: i32,
            character: i32,
        }

        impl Position {
            pub fn new() -> Self {
                Position {
                    line: 0,
                    character: 0,
                }
            }
            pub fn set_line(&mut self, line: i32) {
                self.line = line;
            }

            pub fn set_character(&mut self, character: i32) {
                self.character = character;
            }
        }

        pub type MessageWriter = Box<dyn FnMut(JsonValue) + Send>;

        #[derive(Debug, Clone)]
        pub enum JsonValue {
            Object(HashMap<String, JsonValue>),
            Array(Vec<JsonValue>),
            String(String),
            Number(f64),
            Boolean(bool),
            Null,
        }
        impl JsonValue {
            pub fn to_string(&self) -> String {
                match self {
                    JsonValue::String(s) => s.clone(),
                    _ => format!("{:?}", self)
                }
            }
        }
        
        impl JsonValue {
            pub fn is_string(&self) -> bool {
                match self {
                    JsonValue::String(_) => true,
                    _ => false,
                }
            }
            
            pub fn to_array(&self) -> &Vec<JsonValue> {
                match self {
                    JsonValue::Array(arr) => arr,
                    _ => panic!("Not an array"),
                }
            }
        }

        pub struct InitializeRequest {
            id: i32,
        }
        impl InitializeRequest {
            pub fn new(json: JsonValue) -> InitializeRequest {
                // Extract id from JsonValue
                InitializeRequest {
                    id: 1, // placeholder
                }
            }
            pub fn id(&self) -> i32 {
                self.id
            }
        
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("initializerequest".to_string())
            }
        }

        pub struct InitializeResponse {
            id: i32,
            result: InitializeResult,
        }
        impl InitializeResponse {
            pub fn new() -> InitializeResponse {
                InitializeResponse {
                    id: 0,
                    result: InitializeResult::new()
                }
            }
            pub fn id(&mut self, id: i32) {
                self.id = id;
            }

            pub fn result(&mut self) -> &mut InitializeResult {
                &mut self.result
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("initializerequest".to_string())
            }
        }

        pub struct InitializeResult {
            capabilities: Capabilities,
        }

        impl InitializeResult {
            pub fn new() -> InitializeResult {
                InitializeResult {
                    capabilities: Capabilities::new(),
                }
            }
            pub fn capabilities(&mut self) -> &mut Capabilities {
                &mut self.capabilities
            }
        }

        pub struct Capabilities {

        }

        impl Capabilities {
             pub fn new() -> Capabilities {
                 Capabilities {}
             }
             pub fn text_document_sync(&mut self) {}
             pub fn set_definition_provider(&mut self, val: bool) {}
             pub fn set_document_symbol_provider(&mut self, val: bool) {}
        }

        pub struct RegistrationRequest {
            id: i32,
            method: String,
            params: RegistrationParams,
        }

        impl RegistrationRequest {
            pub fn new() -> RegistrationRequest {
                RegistrationRequest {
                    id: 0,
                    method: String::new(),
                    params: RegistrationParams::new(),
                }
            }

            pub fn set_id(&mut self, id: i32) {
                self.id = id;
            }

            pub fn set_method(&mut self, method: &str) {
                self.method = method.to_string();
            }

            pub fn params(&mut self) -> &mut RegistrationParams {
                &mut self.params
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("registrationrequest".to_string())
            }
        }

        pub struct RegistrationParams {
            registrations: Vec<Registration>,
        }

        impl RegistrationParams {
            pub fn new() -> RegistrationParams {
                RegistrationParams {
                    registrations: Vec::new(),
                }
            }

            pub fn add_registrations(&mut self) -> &mut Registration {
                self.registrations.push(Registration::new());
                self.registrations.last_mut().unwrap()
            }
        }

        pub struct Registration {
            id: String,
            method: String,
            options: DidChangeWatchedFilesRegistrationOptions,
        }

        impl Registration {
            pub fn new() -> Registration {
                Registration {
                    id: String::new(),
                    method: String::new(),
                    options: DidChangeWatchedFilesRegistrationOptions::new(),
                }
            }

            pub fn set_id(&mut self, id: &str) {
                self.id = id.to_string();
            }

            pub fn set_method(&mut self, method: &str) {
                self.method = method.to_string();
            }

            pub fn register_options<T>(&mut self) -> &mut DidChangeWatchedFilesRegistrationOptions {
                &mut self.options
            }
        }

        pub struct DidChangeWatchedFilesRegistrationOptions {
            watchers: Vec<FileSystemWatcher>,
        }

        impl DidChangeWatchedFilesRegistrationOptions {
            pub fn new() -> DidChangeWatchedFilesRegistrationOptions {
                DidChangeWatchedFilesRegistrationOptions {
                    watchers: Vec::new(),
                }
            }

            pub fn add_watchers(&mut self) -> &mut FileSystemWatcher {
                self.watchers.push(FileSystemWatcher::new());
                self.watchers.last_mut().unwrap()
            }
        }

        pub struct FileSystemWatcher {
            glob_pattern: String,
            kind: WatchKind,
        }

        impl FileSystemWatcher {
            pub fn new() -> FileSystemWatcher {
                FileSystemWatcher {
                    glob_pattern: String::new(),
                    kind: WatchKind::All,
                }
            }

            pub fn set_glob_pattern(&mut self, glob_pattern: &str) {
                self.glob_pattern = glob_pattern.to_string();
            }

            pub fn set_kind(&mut self, kind: WatchKind) {
                self.kind = kind;
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub enum WatchKind {
            Create,
            Change,
            Delete,
            All,
        }

        pub struct TorqueFileListNotification {
            params: TorqueFileListParams
        }

        impl TorqueFileListNotification {
            pub fn new(json: JsonValue) -> TorqueFileListNotification {
                TorqueFileListNotification {
                    params: TorqueFileListParams::new()
                }
            }
            pub fn params(&mut self) -> &mut TorqueFileListParams {
                &mut self.params
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("TorqueFileListNotification".to_string())
            }
        }

        pub struct TorqueFileListParams {
            object: HashMap<String, JsonValue>
        }

        impl TorqueFileListParams {
            pub fn new() -> TorqueFileListParams {
                TorqueFileListParams {
                    object: HashMap::new()
                }
            }

            pub fn object(&mut self) -> &mut HashMap<String, JsonValue> {
                &mut self.object
            }
        }

        pub struct GotoDefinitionRequest {
             id: i32,
             params: GotoDefinitionParams
        }

        impl GotoDefinitionRequest {
            pub fn new(json: JsonValue) -> GotoDefinitionRequest {
                GotoDefinitionRequest {
                    id: 1, // placeholder
                    params: GotoDefinitionParams::new()
                }
            }
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn params(&mut self) -> &mut GotoDefinitionParams {
                &mut self.params
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("GotoDefinitionRequest".to_string())
            }
        }

        pub struct GotoDefinitionParams {
            text_document: TextDocumentIdentifier,
            position: Position
        }

        impl GotoDefinitionParams {
            pub fn new() -> GotoDefinitionParams {
                GotoDefinitionParams {
                    text_document: TextDocumentIdentifier::new(),
                    position: Position::new()
                }
            }
            pub fn text_document(&mut self) -> &mut TextDocumentIdentifier {
                &mut self.text_document
            }
            pub fn position(&mut self) -> &mut Position {
                &mut self.position
            }
        }

        pub struct TextDocumentIdentifier {
            uri: String
        }
        impl TextDocumentIdentifier {
            pub fn new() -> TextDocumentIdentifier {
                TextDocumentIdentifier {
                    uri: String::new()
                }
            }
            pub fn uri(&self) -> String {
                self.uri.clone()
            }
        }

        pub struct GotoDefinitionResponse {
             id: i32,
             result: Option<SourcePosition>
        }
        impl GotoDefinitionResponse {
            pub fn new() -> GotoDefinitionResponse {
                GotoDefinitionResponse {
                    id: 0,
                    result: None,
                }
            }
            pub fn set_id(&mut self, id: i32) {
                self.id = id;
            }

            pub fn result(&mut self) -> &mut SourcePosition {
                if self.result.is_none() {
                    self.result = Some(SourcePosition::new());
                }
                self.result.as_mut().unwrap()
            }
            pub fn set_null(&mut self, _field_name: &str) {
                self.result = None;
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("GotoDefinitionRequest".to_string())
            }
        }

        pub struct DidChangeWatchedFilesNotification {

        }

        impl DidChangeWatchedFilesNotification {
            pub fn new(json: JsonValue) -> DidChangeWatchedFilesNotification {
                DidChangeWatchedFilesNotification { }
            }
        }

        pub struct DocumentSymbolRequest {
             id: i32,
             params: DocumentSymbolParams
        }

        impl DocumentSymbolRequest {
            pub fn new(json: JsonValue) -> DocumentSymbolRequest {
                DocumentSymbolRequest {
                    id: 1, // placeholder
                    params: DocumentSymbolParams::new()
                }
            }
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn params(&mut self) -> &mut DocumentSymbolParams {
                &mut self.params
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("DocumentSymbolRequest".to_string())
            }
        }

        pub struct DocumentSymbolParams {
            text_document: TextDocumentIdentifier,
        }
        impl DocumentSymbolParams {
            pub fn new() -> DocumentSymbolParams {
                DocumentSymbolParams {
                    text_document: TextDocumentIdentifier::new(),
                }
            }
            pub fn text_document(&mut self) -> &mut TextDocumentIdentifier {
                &mut self.text_document
            }
        }

        pub struct DocumentSymbolResponse {
            id: i32,
            result: Vec<SymbolInformation>,
        }
        impl DocumentSymbolResponse {
            pub fn new() -> DocumentSymbolResponse {
                DocumentSymbolResponse {
                    id: 0,
                    result: Vec::new(),
                }
            }
            pub fn set_id(&mut self, id: i32) {
                self.id = id;
            }

            pub fn add_result(&mut self) -> &mut SymbolInformation {
                self.result.push(SymbolInformation::new());
                self.result.last_mut().unwrap()
            }

            pub fn result_size(&self) -> usize {
                self.result.len()
            }
            
            pub fn get_json_value(self) -> JsonValue {
                 JsonValue::String("DocumentSymbolRequest".to_string())
            }
        }

        pub struct SymbolInformation {
            name: String,
            kind: SymbolKind,
            location: Location
        }

        impl SymbolInformation {
            pub fn new() -> SymbolInformation {
                SymbolInformation {
                    name: String::new(),
                    kind: SymbolKind::Function,
                    location: Location::new()
                }
            }

            pub fn set_name(&mut self, name: String) {
                self.name = name;
            }
            pub fn set_kind(&mut self, kind: SymbolKind) {
                self.kind = kind;
            }
            pub fn location(&mut self) -> &mut Location {
                &mut self.location
            }
        }

        pub struct Location {
            source_position: SourcePosition
        }
        impl Location {
            pub fn new() -> Location {
                Location {
                    source_position: SourcePosition::new()
                }
            }
            pub fn set_to(&mut self, source_position: SourcePosition) {
                self.source_position = source_position;
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub enum SymbolKind {
            File,
            Module,
            Namespace,
            Package,
            Class,
            Method,
            Property,
            Field,
            Constructor,
            Enum,
            Interface,
            Function,
            Variable,
            Constant,
            String,
            Number,
            Boolean,
            Array,
            Object,
            Key,
            Null,
            EnumMember,
            Struct,
            Event,
            Operator,
            TypeParameter
        }

        pub struct Request<T> {
            json: JsonValue,
            method: Option<String>,
            id: i32,
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> Request<T> {
            pub fn new(json: JsonValue) -> Request<T> {
                Request {
                    method: Some("method".to_string()),
                    id: 1,
                    json,
                    _phantom: std::marker::PhantomData
                }
            }
            pub fn has_method(&self) -> bool {
                self.method.is_some()
            }
            pub fn method(&self) -> String {
                self.method.clone().unwrap_or_default()
            }
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn get_json_value(self) -> JsonValue {
                self.json
            }
        }

        // Placeholder definitions.  Replace with actual definitions.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct SourceId(u32);
        impl SourceId {
            pub fn invalid() -> Self {
                SourceId(0)
            }
            pub fn is_valid(&self) -> bool {
                self.0 != 0
            }
        }

        pub struct LineAndColumn {
           line: i32,
           column: i32
        }

        impl LineAndColumn {
           pub fn with_unknown_offset(line: i32, column: i32) -> LineAndColumn {
                LineAndColumn {
                    line, column
                }
           }
        }

        pub struct LanguageServerData {}
        impl LanguageServerData {
            pub fn find_definition(id: SourceId, pos: LineAndColumn) -> Option<SourcePosition> {
                Some(SourcePosition::new()) // Placeholder implementation.
            }
            pub fn symbols_for_source_id(id: SourceId) -> Vec<Box<dyn Symbol>> {
                 vec![]
            }
        }

        // Placeholder
        pub trait Symbol {
            fn is_user_defined(&self) -> bool;
            fn is_macro(&self) -> bool;
            fn is_builtin(&self) -> bool;
            fn is_generic_callable(&self) -> bool;
            fn is_type_alias(&self) -> bool;
            fn position(&self) -> SourcePosition;
        }

        pub struct Macro {}
        impl Macro {
            pub fn cast(_symbol: &dyn Symbol) -> &Macro {
                &Macro{} // Placeholder
            }
            pub fn readable_name(&self) -> String {
                "Macro".to_string() // Placeholder
            }
        }
        impl Symbol for Macro {
            fn is_user_defined(&self) -> bool { true }
            fn is_macro(&self) -> bool { true }
            fn is_builtin(&self) -> bool { false }
            fn is_generic_callable(&self) -> bool { false }
            fn is_type_alias(&self) -> bool { false }
            fn position(&self) -> SourcePosition {
                SourcePosition::new() // Placeholder
            }
        }

        pub struct Builtin {}
        impl Builtin {
            pub fn cast(_symbol: &dyn Symbol) -> &Builtin {
                &Builtin{} // Placeholder
            }
            pub fn readable_name(&self) -> String {
                "Builtin".to_string() // Placeholder
            }
        }

        impl Symbol for Builtin {
            fn is_user_defined(&self) -> bool { true }
            fn is_macro(&self) -> bool { false }
            fn is_builtin(&self) -> bool { true }
            fn is_generic_callable(&self) -> bool { false }
            fn is_type_alias(&self) -> bool { false }
            fn position(&self) -> SourcePosition {
                SourcePosition::new() // Placeholder
            }
        }

        pub struct GenericCallable {}
        impl GenericCallable {
            pub fn cast(_symbol: &dyn Symbol) -> &GenericCallable {
                &GenericCallable{} // Placeholder
            }
            pub fn name(&self) -> String {
                "GenericCallable".to_string() // Placeholder
            }
        }

        impl Symbol for GenericCallable {
            fn is_user_defined(&self) -> bool { true }
            fn is_macro(&self) -> bool { false }
            fn is_builtin(&self) -> bool { false }
            fn is_generic_callable(&self) -> bool { true }
            fn is_type_alias(&self) -> bool { false }
            fn position(&self) -> SourcePosition {
                SourcePosition::new() // Placeholder
            }
        }

        pub struct TypeAlias {}
        impl TypeAlias {
            pub fn cast(_symbol: &dyn Symbol) -> &TypeAlias {
                &TypeAlias{} // Placeholder
            }
            pub fn type_(&self) -> &Type {
                &Type{} // Placeholder
            }
        }
        impl Symbol for TypeAlias {
            fn is_user_defined(&self) -> bool { true }
            fn is_macro(&self) -> bool { false }
            fn is_builtin(&self) -> bool { false }
            fn is_generic_callable(&self) -> bool { false }
            fn is_type_alias(&self) -> bool { true }
            fn position(&self) -> SourcePosition {
                SourcePosition::new() // Placeholder
            }
        }

        pub struct Type {}
        impl Type {
            pub fn is_class_type(&self) -> bool {
                true
            }
            pub fn to_string(&self) -> String {
                "Type".to_string()
            }
        }
        
        #[derive(Clone, Copy)]
        pub struct SourcePosition {
            // Placeholder implementation
        }

        impl SourcePosition {
            pub fn new() -> Self {
                SourcePosition {}
            }
        }
    }
    pub struct TorqueCompilerResult {
        pub messages: Vec<TorqueMessage>,
        pub language_server_data: ls::LanguageServerData,
        pub source_file_map: Box<SourceFileMap>,
    }
    pub struct SourceFileMap {}

    impl SourceFileMap {
        pub fn get_source_id(uri: String) -> ls::SourceId {
            ls::SourceId::invalid() // Placeholder implementation.
        }
        pub fn absolute_path(source: ls::SourceId) -> String {
            "path".to_string()
        }
    }
    pub struct TorqueMessage {
        pub kind: TorqueMessageKind,
        pub message: String,
        pub position: Option<SourcePosition>
    }

    #[derive(Debug, Copy, Clone)]
    pub enum TorqueMessageKind {
        Error,
        Lint
    }
    
    #[derive(Clone, Copy)]
    pub struct SourcePosition {}

    impl SourcePosition {
        pub fn new() -> Self {
            SourcePosition {}
        }
    }
}

mod v8 {
    pub mod base {
        pub mod os {
            pub fn abort() -> ! {
                panic!("Aborted!");
            }
        }
    }
    pub mod internal {
        pub mod torque {
            use super::super::super::ls::SourceId;

            pub static DIAGNOSTICS_FILES: Lazy<Mutex<Vec<SourceId>>> = Lazy::new(|| Mutex::new(Vec::new()));
        }
    }
}

mod logger {
    pub fn log(args: impl std::fmt::Display, args2: impl std::fmt::Display, args3: impl std::fmt::Display) {
        println!("{}{}{}", args, args2, args3);
    }

    pub fn log(args: impl std::fmt::Display) {
        println!("{}", args);
    }
}

mod json {
    use super::torque::ls::JsonValue;

    pub fn parse_json(content: String) -> Result<WrappedJsonValue, String> {
        // Placeholder: Implement JSON parsing here.
        Ok(WrappedJsonValue {value: JsonValue::String(content)})
    }

    pub fn serialize_to_string(message: JsonValue) -> String {
        // Placeholder: Implement JSON serialization here.
        format!("{:?}", message)
    }

    pub struct WrappedJsonValue {
        pub value: JsonValue
    }
}

mod message_pipe {
    // Placeholder
}

mod message {
    // Placeholder
}

mod server_data {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    use super::torque::ls::LanguageServerData;

    pub static LANGUAGE_SERVER_DATA: Lazy<Mutex<Option<LanguageServerData>>> = Lazy::new(|| Mutex::new(None));

}

mod source_positions {
    // Placeholder
}

mod torque_compiler {
    use super::torque::{TorqueCompilerResult, SourceFileMap, TorqueMessage, TorqueMessageKind};
    use super::torque::ls::SourceId;

    pub struct TorqueCompilerOptions {
        pub output_directory: String,
        pub collect_language_server_data: bool,
        pub force_assert_statements: bool,
    }

    impl TorqueCompilerOptions {
        pub fn new() -> Self {
            TorqueCompilerOptions {
                output_directory: String::new(),
                collect_language_server_data: false,
                force_assert_statements: false,
            }
        }
    }

    pub fn compile_torque(file_list: &Vec<String>, options: TorqueCompilerOptions) -> TorqueCompilerResult {
        // Placeholder
        TorqueCompilerResult {
            messages: vec![TorqueMessage { kind: TorqueMessageKind::Lint, message: "message".to_string(), position: None }],
            language_server_data: super::torque::ls::LanguageServerData {},
            source_file_map: Box::new(SourceFileMap {}),
        }
    }
}

mod torque_file_list {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    pub static TORQUE_FILE_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
}

mod globals {
    // Placeholder
}

use json::{parse_json, serialize_to_string};
use logger::log;
use torque::ls::{PublishDiagnosticsNotification, Diagnostic, Range, SourceId, LineAndColumn, LanguageServerData, SourcePosition};
use torque_compiler::{compile_torque, TorqueCompilerOptions};
use torque_file_list::TORQUE_FILE_LIST;
use torque::ls::JsonValue;
use json::WrappedJsonValue;

//use v8::internal::torque::DIAGNOSTICS_FILES;

const K_CONTENT_LENGTH: &str = "Content-Length: ";
const K_CONTENT_LENGTH_SIZE: usize = K_CONTENT_LENGTH.len();

#[cfg(target_os = "windows")]
const K_PROTOCOL_LINE_ENDING: &str = "\n\n";
#[cfg(not(target_os = "windows"))]
const K_PROTOCOL_LINE_ENDING: &str = "\r\n\r\n";

fn read_message() -> WrappedJsonValue {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    if !line.starts_with(K_CONTENT_LENGTH) {
        logger::log("[fatal] Did not find Content-Length ...");
        v8::base::os::abort();
    }

    let content_length = line[K_CONTENT_LENGTH_SIZE..].trim().parse::<usize>().expect("Invalid Content-Length");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let mut content = String::with_capacity(content_length);
    io::stdin().take(content_length as u64).read_to_string(&mut content).expect("Failed to read content");
    
    log("[incoming] ", content.clone(), "\n\n");
    
    parse_json(content).unwrap()
}

fn write_message(message: JsonValue) {
    let content = serialize_to_string(message);

    log("[outgoing] ", content.clone(), "\n\n");

    print!("{}{}{}", K_CONTENT_LENGTH, content.len(), K_PROTOCOL_LINE_ENDING);
    io::stdout().write_all(content.as_bytes()).expect("Failed to write content");
    io::stdout().flush().expect("Failed to flush");
}

fn reset_compilation_error_diagnostics(mut writer: torque::ls::MessageWriter) {
    let diagnostics_files = v8::internal::torque::DIAGNOSTICS_FILES.lock().unwrap();
    for &source in diagnostics_files.iter() {
        let mut notification = PublishDiagnosticsNotification::new();
        notification.set_method("textDocument/publishDiagnostics");

        let error_file = torque::SourceFileMap::absolute_path(source);
        notification.params().set_uri(error_file);
        notification.params().diagnostics_size();

        writer(notification.get_json_value());
    }
    drop(diagnostics_files);
    *v8::internal::torque::DIAGNOSTICS_FILES.lock().unwrap() = Vec::new();
}

struct DiagnosticCollector {
    notifications: HashMap<SourceId, PublishDiagnosticsNotification>,
    suppress_lint_messages: bool,
}

impl DiagnosticCollector {
    fn new() -> Self {
        DiagnosticCollector {
            notifications: HashMap::new(),
            suppress_lint_messages: false,
        }
    }

    fn add_torque_message(&mut self, message: &torque::TorqueMessage) {
        if !self.should_add_message_of_kind(message.kind) {
            return;
        }

        let id = match message.position {
            Some(_) => {
                 //message.position.unwrap().source   // This line will cause a compile error in Rust. `unwrap()` should be avoided on Option types.
                 SourceId::invalid() // Placeholder
            },
            None => SourceId::invalid(),
        };

        let notification = self.get_or_create_notification_for_source(id);

        let diagnostic = notification.params().add_diagnostics();
        diagnostic.set_severity(self.serverity_for(message.kind));
        diagnostic.set_message(message.message.clone());
        diagnostic.set_source("Torque Compiler".to_string());

        if let Some(_position) = message.position {
             //self.populate_range_from_source_position(diagnostic.range(), *position); // This code cannot be directly translated because source_position field is not defined in SourcePosition
        }
    }

    fn notifications(&mut self) -> &mut HashMap<SourceId, PublishDiagnosticsNotification> {
        &mut self.notifications
    }

    fn get_or_create_notification_for_source(&mut self, id: SourceId) -> &mut PublishDiagnosticsNotification {
        if !self.notifications.contains_key(&id) {
            let mut notification = PublishDiagnosticsNotification::new();
            notification.set_method("textDocument/publishDiagnostics");

            let file = if id.is_valid() {
                torque::SourceFileMap::absolute_path(id)
            } else {
                "<unknown>".to_string()
            };
            notification.params().set_uri(file);
            self.notifications.insert(id, notification);
        }
        self.notifications.get_mut(&id).unwrap()
    }

    fn should_add_message_of_kind(&mut self, kind: torque::TorqueMessageKind) -> bool {
        match kind {
            torque::TorqueMessageKind::Error => {
                self.suppress_lint_messages = true;
                true
            }
            torque::TorqueMessageKind::Lint => {
                if self.suppress_lint_messages {
                    return false;
                }
                true
            }
        }
    }

    fn populate_range_from_source_position(&self, range: &mut Range, position: torque::SourcePosition) {
        //range.start().set_line(position.start.line); // This code cannot be directly translated because start field is not defined in SourcePosition
        //range.start().set_character(position.start.column);
        //range.end().set_line(position.end.line);
        //range.end().set_character(position.end.column);
    }

    fn serverity_for(&self, kind: torque::TorqueMessageKind) -> torque::ls::DiagnosticSeverity {
        match kind {
            torque::TorqueMessageKind::Error => torque::ls::DiagnosticSeverity::Error,
            torque::TorqueMessageKind::Lint => torque::ls::DiagnosticSeverity::Warning,
        }
    }
}

fn send_compilation_diagnostics(result: &torque::TorqueCompilerResult, mut