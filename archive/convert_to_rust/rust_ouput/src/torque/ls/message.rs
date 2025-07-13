// Converted from V8 C++ source files:
// Header: message.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ls {
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
    pub enum JsonValue {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(JsonObject),
    }

    impl JsonValue {
        pub const IS_NULL: i32 = 0;

        pub fn JsonNull() -> Self {
            JsonValue::Null
        }

        pub fn From(object: JsonObject) -> Self {
            JsonValue::Object(object)
        }

        pub fn IsObject(&self) -> bool {
            match self {
                JsonValue::Object(_) => true,
                _ => false,
            }
        }

        pub fn ToObject(&mut self) -> &mut JsonObject {
            match self {
                JsonValue::Object(obj) => obj,
                _ => panic!("Not a JsonObject"),
            }
        }

        pub fn IsArray(&self) -> bool {
            match self {
                JsonValue::Array(_) => true,
                _ => false,
            }
        }

        pub fn ToArray(&mut self) -> &mut JsonArray {
            match self {
                JsonValue::Array(arr) => arr,
                _ => panic!("Not a JsonArray"),
            }
        }
    }

    pub type JsonObject = HashMap<String, JsonValue>;
    pub type JsonArray = Vec<JsonValue>;

    pub struct BaseJsonAccessor {
        object: JsonObject,
    }

    impl BaseJsonAccessor {
        pub fn new() -> Self {
            BaseJsonAccessor {
                object: JsonObject::new(),
            }
        }

        pub fn get_object<T>(&mut self, property: &str) -> T
        where
            T: FromJsonObject,
        {
            let obj = self.get_object_property(property);
            T::from_json_object(obj)
        }

        pub fn has_property(&self, property: &str) -> bool {
            self.object.contains_key(property)
        }

        pub fn set_null(&mut self, property: &str) {
            self.object.insert(property.to_string(), JsonValue::JsonNull());
        }

        pub fn is_null(&self, property: &str) -> bool {
            self.has_property(property)
                && match self.object.get(property) {
                    Some(JsonValue::Null) => true,
                    _ => false,
                }
        }

        pub fn object(&self) -> &JsonObject {
            &self.object
        }

        pub fn object_mut(&mut self) -> &mut JsonObject {
            &mut self.object
        }

        fn get_object_property(&mut self, property: &str) -> &mut JsonObject {
            if !self.object.contains_key(property) {
                self.object.insert(property.to_string(), JsonValue::From(JsonObject::new()));
            }
            match self.object.get_mut(property) {
                Some(JsonValue::Object(obj)) => obj,
                _ => panic!("Expected JsonObject"),
            }
        }

        fn get_array_property(&mut self, property: &str) -> &mut JsonArray {
            if !self.object.contains_key(property) {
                self.object.insert(property.to_string(), JsonValue::From(JsonArray::new()));
            }
            match self.object.get_mut(property) {
                Some(JsonValue::Array(arr)) => arr,
                _ => panic!("Expected JsonArray"),
            }
        }

        fn add_object_element_to_array_property(&mut self, property: &str) -> &mut JsonObject {
            let array = self.get_array_property(property);
            array.push(JsonValue::From(JsonObject::new()));
            match array.last_mut() {
                Some(JsonValue::Object(obj)) => obj,
                _ => panic!("Expected JsonObject"),
            }
        }
    }

    pub trait FromJsonObject {
        fn from_json_object(object: &mut JsonObject) -> Self;
    }

    pub struct Message {
        value_: JsonValue,
    }

    impl Message {
        pub fn new() -> Self {
            let mut message = Message {
                value_: JsonValue::From(JsonObject::new()),
            };
            message.set_jsonrpc("2.0");
            message
        }

        pub fn with_json_value(value: JsonValue) -> Self {
            match &value {
                JsonValue::Object(_) => {}
                _ => panic!("Expected JsonValue::Object"),
            }
            Message { value_: value }
        }

        pub fn get_json_value(&mut self) -> &mut JsonValue {
            &mut self.value_
        }

        pub fn jsonrpc(&self) -> String {
            match &self.value_ {
                JsonValue::Object(obj) => match obj.get("jsonrpc") {
                    Some(JsonValue::String(s)) => s.clone(),
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            }
        }

        pub fn set_jsonrpc(&mut self, value: &str) {
            match &mut self.value_ {
                JsonValue::Object(obj) => {
                    obj.insert("jsonrpc".to_string(), JsonValue::String(value.to_string()));
                }
                _ => {}
            }
        }

        pub fn object(&self) -> &JsonObject {
            match &self.value_ {
                JsonValue::Object(obj) => obj,
                _ => panic!("Expected JsonObject"),
            }
        }

        pub fn object_mut(&mut self) -> &mut JsonObject {
            match &mut self.value_ {
                JsonValue::Object(obj) => obj,
                _ => panic!("Expected JsonObject"),
            }
        }
    }

    pub struct NestedJsonAccessor<'a> {
        object_: &'a mut JsonObject,
    }

    impl<'a> NestedJsonAccessor<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            NestedJsonAccessor { object_: object }
        }

        pub fn object(&self) -> &JsonObject {
            self.object_
        }

        pub fn object_mut(&mut self) -> &mut JsonObject {
            self.object_
        }
    }

    pub struct ResponseError<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> ResponseError<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            ResponseError {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn code(&self) -> i32 {
            match self.nested.object().get("code") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_code(&mut self, value: i32) {
            self.nested.object_mut().insert("code".to_string(), JsonValue::Number(value as f64));
        }

        pub fn message(&self) -> String {
            match self.nested.object().get("message") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_message(&mut self, value: &str) {
            self.nested.object_mut().insert("message".to_string(), JsonValue::String(value.to_string()));
        }
    }

    pub struct InitializeParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> InitializeParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            InitializeParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn processId(&self) -> i32 {
            match self.nested.object().get("processId") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_processId(&mut self, value: i32) {
            self.nested.object_mut().insert("processId".to_string(), JsonValue::Number(value as f64));
        }

        pub fn rootPath(&self) -> String {
            match self.nested.object().get("rootPath") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_rootPath(&mut self, value: &str) {
            self.nested.object_mut().insert("rootPath".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn rootUri(&self) -> String {
            match self.nested.object().get("rootUri") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_rootUri(&mut self, value: &str) {
            self.nested.object_mut().insert("rootUri".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn trace(&self) -> String {
            match self.nested.object().get("trace") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_trace(&mut self, value: &str) {
            self.nested.object_mut().insert("trace".to_string(), JsonValue::String(value.to_string()));
        }
    }

    pub struct FileListParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> FileListParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            FileListParams {
                nested: NestedJsonAccessor::new(object),
            }
        }
    }

    pub struct FileSystemWatcher<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> FileSystemWatcher<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            FileSystemWatcher {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn globPattern(&self) -> String {
            match self.nested.object().get("globPattern") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_globPattern(&mut self, value: &str) {
            self.nested.object_mut().insert("globPattern".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn kind(&self) -> i32 {
            match self.nested.object().get("kind") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_kind(&mut self, value: i32) {
            self.nested.object_mut().insert("kind".to_string(), JsonValue::Number(value as f64));
        }
    }

    pub struct DidChangeWatchedFilesRegistrationOptions<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> DidChangeWatchedFilesRegistrationOptions<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            DidChangeWatchedFilesRegistrationOptions {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn watchers(&mut self) -> Vec<FileSystemWatcher> {
            let mut result = Vec::new();
            if let Some(JsonValue::Array(array)) = self.nested.object_mut().get_mut("watchers") {
                for element in array {
                    if let JsonValue::Object(object) = element {
                        result.push(FileSystemWatcher::new(object));
                    }
                }
            }
            result
        }

        pub fn add_watchers(&mut self) -> FileSystemWatcher {
            let obj = self.nested.add_object_element_to_array_property("watchers");
            FileSystemWatcher::new(obj)
        }
    }

    pub struct FileEvent<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> FileEvent<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            FileEvent {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn uri(&self) -> String {
            match self.nested.object().get("uri") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_uri(&mut self, value: &str) {
            self.nested.object_mut().insert("uri".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn type_(&self) -> i32 {
            match self.nested.object().get("type") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_type(&mut self, value: i32) {
            self.nested.object_mut().insert("type".to_string(), JsonValue::Number(value as f64));
        }
    }

    pub struct DidChangeWatchedFilesParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> DidChangeWatchedFilesParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            DidChangeWatchedFilesParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn changes(&mut self) -> Vec<FileEvent> {
            let mut result = Vec::new();
            if let Some(JsonValue::Array(array)) = self.nested.object_mut().get_mut("changes") {
                for element in array {
                    if let JsonValue::Object(object) = element {
                        result.push(FileEvent::new(object));
                    }
                }
            }
            result
        }

        pub fn add_changes(&mut self) -> FileEvent {
            let obj = self.nested.add_object_element_to_array_property("changes");
            FileEvent::new(obj)
        }
    }

    pub struct SaveOptions<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> SaveOptions<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            SaveOptions {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn includeText(&self) -> bool {
            match self.nested.object().get("includeText") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_includeText(&mut self, value: bool) {
            self.nested.object_mut().insert("includeText".to_string(), JsonValue::Boolean(value));
        }
    }

    pub struct TextDocumentSyncOptions<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> TextDocumentSyncOptions<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            TextDocumentSyncOptions {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn openClose(&self) -> bool {
            match self.nested.object().get("openClose") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_openClose(&mut self, value: bool) {
            self.nested.object_mut().insert("openClose".to_string(), JsonValue::Boolean(value));
        }

        pub fn change(&self) -> i32 {
            match self.nested.object().get("change") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_change(&mut self, value: i32) {
            self.nested.object_mut().insert("change".to_string(), JsonValue::Number(value as f64));
        }

        pub fn willSave(&self) -> bool {
            match self.nested.object().get("willSave") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_willSave(&mut self, value: bool) {
            self.nested.object_mut().insert("willSave".to_string(), JsonValue::Boolean(value));
        }

        pub fn willSaveWaitUntil(&self) -> bool {
            match self.nested.object().get("willSaveWaitUntil") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_willSaveWaitUntil(&mut self, value: bool) {
            self.nested.object_mut().insert("willSaveWaitUntil".to_string(), JsonValue::Boolean(value));
        }

        pub fn save(&mut self) -> SaveOptions {
            let obj = self.nested.get_object_property("save");
            SaveOptions::new(obj)
        }
    }

    pub struct ServerCapabilities<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> ServerCapabilities<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            ServerCapabilities {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn textDocumentSync(&mut self) -> TextDocumentSyncOptions {
            let obj = self.nested.get_object_property("textDocumentSync");
            TextDocumentSyncOptions::new(obj)
        }

        pub fn definitionProvider(&self) -> bool {
            match self.nested.object().get("definitionProvider") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_definitionProvider(&mut self, value: bool) {
            self.nested.object_mut().insert("definitionProvider".to_string(), JsonValue::Boolean(value));
        }

        pub fn documentSymbolProvider(&self) -> bool {
            match self.nested.object().get("documentSymbolProvider") {
                Some(JsonValue::Boolean(b)) => *b,
                _ => false,
            }
        }

        pub fn set_documentSymbolProvider(&mut self, value: bool) {
            self.nested.object_mut().insert("documentSymbolProvider".to_string(), JsonValue::Boolean(value));
        }
    }

    pub struct InitializeResult<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> InitializeResult<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            InitializeResult {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn capabilities(&mut self) -> ServerCapabilities {
            let obj = self.nested.get_object_property("capabilities");
            ServerCapabilities::new(obj)
        }
    }

    pub struct Registration<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> Registration<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            Registration {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn id(&self) -> String {
            match self.nested.object().get("id") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_id(&mut self, value: &str) {
            self.nested.object_mut().insert("id".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn method(&self) -> String {
            match self.nested.object().get("method") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_method(&mut self, value: &str) {
            self.nested.object_mut().insert("method".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn registerOptions(&mut self) -> &mut JsonObject {
            self.nested.get_object_property("registerOptions")
        }
    }

    pub struct RegistrationParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> RegistrationParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            RegistrationParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn registrations(&mut self) -> Vec<Registration> {
            let mut result = Vec::new();
            if let Some(JsonValue::Array(array)) = self.nested.object_mut().get_mut("registrations") {
                for element in array {
                    if let JsonValue::Object(object) = element {
                        result.push(Registration::new(object));
                    }
                }
            }
            result
        }

        pub fn add_registrations(&mut self) -> Registration {
            let obj = self.nested.add_object_element_to_array_property("registrations");
            Registration::new(obj)
        }
    }

    pub struct JsonPosition<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> JsonPosition<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            JsonPosition {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn line(&self) -> i32 {
            match self.nested.object().get("line") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_line(&mut self, value: i32) {
            self.nested.object_mut().insert("line".to_string(), JsonValue::Number(value as f64));
        }

        pub fn character(&self) -> i32 {
            match self.nested.object().get("character") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_character(&mut self, value: i32) {
            self.nested.object_mut().insert("character".to_string(), JsonValue::Number(value as f64));
        }
    }

    pub struct Range<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> Range<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            Range {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn start(&mut self) -> JsonPosition {
            let obj = self.nested.get_object_property("start");
            JsonPosition::new(obj)
        }

        pub fn end(&mut self) -> JsonPosition {
            let obj = self.nested.get_object_property("end");
            JsonPosition::new(obj)
        }
    }

    pub struct Location<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> Location<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            Location {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn uri(&self) -> String {
            match self.nested.object().get("uri") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_uri(&mut self, value: &str) {
            self.nested.object_mut().insert("uri".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn range(&mut self) -> Range {
            let obj = self.nested.get_object_property("range");
            Range::new(obj)
        }

        pub fn set_to(&mut self, position: &SourcePosition) {
            self.set_uri(&SourceFileMap::AbsolutePath(position.source.clone()));
            self.range().start().set_line(position.start.line);
            self.range().start().set_character(position.start.column);
            self.range().end().set_line(position.end.line);
            self.range().end().set_character(position.end.column);
        }
    }

    pub struct TextDocumentIdentifier<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> TextDocumentIdentifier<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            TextDocumentIdentifier {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn uri(&self) -> String {
            match self.nested.object().get("uri") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_uri(&mut self, value: &str) {
            self.nested.object_mut().insert("uri".to_string(), JsonValue::String(value.to_string()));
        }
    }

    pub struct TextDocumentPositionParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> TextDocumentPositionParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            TextDocumentPositionParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn textDocument(&mut self) -> TextDocumentIdentifier {
            let obj = self.nested.get_object_property("textDocument");
            TextDocumentIdentifier::new(obj)
        }

        pub fn position(&mut self) -> JsonPosition {
            let obj = self.nested.get_object_property("position");
            JsonPosition::new(obj)
        }
    }

    pub struct Diagnostic<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> Diagnostic<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            Diagnostic {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn range(&mut self) -> Range {
            let obj = self.nested.get_object_property("range");
            Range::new(obj)
        }

        pub fn severity(&self) -> i32 {
            match self.nested.object().get("severity") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_severity(&mut self, value: i32) {
            self.nested.object_mut().insert("severity".to_string(), JsonValue::Number(value as f64));
        }

        pub fn source(&self) -> String {
            match self.nested.object().get("source") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_source(&mut self, value: &str) {
            self.nested.object_mut().insert("source".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn message(&self) -> String {
            match self.nested.object().get("message") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_message(&mut self, value: &str) {
            self.nested.object_mut().insert("message".to_string(), JsonValue::String(value.to_string()));
        }
    }

    pub struct PublishDiagnosticsParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> PublishDiagnosticsParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            PublishDiagnosticsParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn uri(&self) -> String {
            match self.nested.object().get("uri") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_uri(&mut self, value: &str) {
            self.nested.object_mut().insert("uri".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn diagnostics(&mut self) -> Vec<Diagnostic> {
            let mut result = Vec::new();
            if let Some(JsonValue::Array(array)) = self.nested.object_mut().get_mut("diagnostics") {
                for element in array {
                    if let JsonValue::Object(object) = element {
                        result.push(Diagnostic::new(object));
                    }
                }
            }
            result
        }

        pub fn add_diagnostics(&mut self) -> Diagnostic {
            let obj = self.nested.add_object_element_to_array_property("diagnostics");
            Diagnostic::new(obj)
        }
    }

    pub struct DocumentSymbolParams<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> DocumentSymbolParams<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            DocumentSymbolParams {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn textDocument(&mut self) -> TextDocumentIdentifier {
            let obj = self.nested.get_object_property("textDocument");
            TextDocumentIdentifier::new(obj)
        }
    }

    pub struct SymbolInformation<'a> {
        nested: NestedJsonAccessor<'a>,
    }

    impl<'a> SymbolInformation<'a> {
        pub fn new(object: &'a mut JsonObject) -> Self {
            SymbolInformation {
                nested: NestedJsonAccessor::new(object),
            }
        }

        pub fn name(&self) -> String {
            match self.nested.object().get("name") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_name(&mut self, value: &str) {
            self.nested.object_mut().insert("name".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn kind(&self) -> i32 {
            match self.nested.object().get("kind") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_kind(&mut self, value: i32) {
            self.nested.object_mut().insert("kind".to_string(), JsonValue::Number(value as f64));
        }

        pub fn location(&mut self) -> Location {
            let obj = self.nested.get_object_property("location");
            Location::new(obj)
        }

        pub fn containerName(&self) -> String {
            match self.nested.object().get("containerName") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_containerName(&mut self, value: &str) {
            self.nested.object_mut().insert("containerName".to_string(), JsonValue::String(value.to_string()));
        }
    }

    pub struct Request<T> {
        message: Message,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Request<T> {
        pub fn new() -> Self {
            Request {
                message: Message::new(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn with_json_value(value: JsonValue) -> Self {
            Request {
                message: Message::with_json_value(value),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn id(&self) -> i32 {
            match self.message.object().get("id") {
                Some(JsonValue::Number(n)) => *n as i32,
                _ => 0,
            }
        }

        pub fn set_id(&mut self, value: i32) {
            self.message.object_mut().insert("id".to_string(), JsonValue::Number(value as f64));
        }

        pub fn method(&self) -> String {
            match self.message.object().get("method") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => "".to_string(),
            }
        }

        pub fn set_method(&mut self, value: &str) {
            self.message.object_mut().insert("method".to_string(), JsonValue::String(value.to_string()));
        }

        pub fn params(&mut self) -> T
        where
            T: FromJsonObject,
        {
            let obj = self.message.object_mut().get_mut("params").unwrap();
            match obj {
                JsonValue::Object(o) => {
                    let mut params_obj = o;
                    T::from_json_object(params_obj)
                }
                _ => panic!("Expected JsonObject"),
            }
        }
    }

    impl<T> From
