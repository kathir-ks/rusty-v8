// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ls {
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::collections::HashMap;

    // Assume SourceFileMap and SourcePosition are defined elsewhere
    // and adapt their usage accordingly. Placeholder definitions:
    pub mod source_positions {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub struct SourcePosition {
            pub start: Position,
            pub end: Position,
            pub source: SourceFile,
        }

        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub struct Position {
            pub line: i64,
            pub column: i64,
        }

        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub struct SourceFile {
            pub path: String,
        }
    }

    pub mod source_file_map {
        use std::collections::HashMap;

        use super::source_positions::SourceFile;

        pub struct SourceFileMap {}

        impl SourceFileMap {
            pub fn AbsolutePath(source: SourceFile) -> String {
                source.path
            }
        }
    }

    use source_positions::SourcePosition;

    /// Base class for Messages and Objects that are backed by a JsonValue
    /// or a reference to a JsonObject.
    /// Helper methods are used by macros to implement typed accessors.
    pub trait BaseJsonAccessor {
        fn object(&self) -> &Map<String, Value>;
        fn object_mut(&mut self) -> &mut Map<String, Value>;

        fn get_object<T: FromJsonObject>(&self, property: &str) -> T {
            let obj = self.get_object_property(property);
            T::from_json_object(obj)
        }

        fn has_property(&self, property: &str) -> bool {
            self.object().contains_key(property)
        }

        fn set_null(&mut self, property: &str) {
            self.object_mut().insert(property.to_string(), Value::Null);
        }

        fn is_null(&self, property: &str) -> bool {
            self.has_property(property) && self.object().get(property) == Some(&Value::Null)
        }

        fn get_object_property(&self, property: &str) -> Map<String, Value> {
            if !self
                .object()
                .get(property)
                .map_or(false, |v| v.is_object())
            {
                self.object_mut()
                    .insert(property.to_string(), Value::Object(Map::new()));
            }
            self.object()
                .get(property)
                .unwrap()
                .as_object()
                .unwrap()
                .clone()
        }

        fn get_array_property(&self, property: &str) -> Vec<Value> {
            if !self.object().get(property).map_or(false, |v| v.is_array()) {
                self.object_mut()
                    .insert(property.to_string(), Value::Array(Vec::new()));
            }
            self.object()
                .get(property)
                .unwrap()
                .as_array()
                .unwrap()
                .clone()
        }

        fn add_object_element_to_array_property(&self, property: &str) -> Map<String, Value> {
            let mut array = self.get_array_property(property);
            let obj = Value::Object(Map::new());
            array.push(obj);
            self.object_mut().insert(property.to_string(), Value::Array(array));

            self.object()
                .get(property)
                .unwrap()
                .as_array()
                .unwrap()
                .last()
                .unwrap()
                .as_object()
                .unwrap()
                .clone()
        }
    }

    pub trait ToJsonObject {
        fn to_json_object(&self) -> Map<String, Value>;
    }

    pub trait FromJsonObject: Sized {
        fn from_json_object(object: Map<String, Value>) -> Self;
    }

    macro_rules! impl_json_string_accessors {
        ($struct_name:ident, $field_name:ident) => {
            impl $struct_name {
                pub fn get_$field_name(&self) -> Option<&str> {
                    self.object().get(stringify!($field_name)).and_then(|v| v.as_str())
                }

                pub fn set_$field_name(&mut self, value: &str) {
                    self.object_mut()
                        .insert(stringify!($field_name).to_string(), Value::String(value.to_string()));
                }
            }
        };
    }

    macro_rules! impl_json_int_accessors {
        ($struct_name:ident, $field_name:ident) => {
            impl $struct_name {
                pub fn get_$field_name(&self) -> Option<i64> {
                    self.object().get(stringify!($field_name)).and_then(|v| v.as_i64())
                }

                pub fn set_$field_name(&mut self, value: i64) {
                    self.object_mut()
                        .insert(stringify!($field_name).to_string(), Value::Number(value.into()));
                }
            }
        };
    }

    macro_rules! impl_json_bool_accessors {
        ($struct_name:ident, $field_name:ident) => {
            impl $struct_name {
                pub fn get_$field_name(&self) -> Option<bool> {
                    self.object().get(stringify!($field_name)).and_then(|v| v.as_bool())
                }

                pub fn set_$field_name(&mut self, value: bool) {
                    self.object_mut()
                        .insert(stringify!($field_name).to_string(), Value::Bool(value));
                }
            }
        };
    }

    macro_rules! impl_json_object_accessors {
        ($struct_name:ident, $field_name:ident, $field_type:ident) => {
            impl $struct_name {
                pub fn $field_name(&self) -> $field_type {
                    let object = self.get_object_property(stringify!($field_name));
                    $field_type::from_json_object(object)
                }

                // Consider adding a mutable version if needed.  Requires more care to avoid lifetime issues.
            }
        };
    }

    macro_rules! impl_json_array_object_accessors {
        ($struct_name:ident, $field_type:ident, $field_name:ident) => {
            impl $struct_name {
                pub fn $field_name(&self) -> Vec<$field_type> {
                    let array = self.get_array_property(stringify!($field_name));
                    array.into_iter()
                        .filter_map(|value| {
                            if let Value::Object(obj) = value {
                                Some($field_type::from_json_object(obj))
                            } else {
                                None
                            }
                        })
                        .collect()
                }
            }
        };
    }

    macro_rules! impl_json_dynamic_object_accessors {
        ($struct_name:ident, $field_name:ident) => {
            impl $struct_name {
                // Implementation for dynamic object accessors would require a more specific design
                // decision about how to handle the dynamic nature of the objects.  The C++ code
                // directly manipulates the JsonObject, but Rust requires more type safety.
                // The simplest option would be to return a serde_json::Value.
            }
        };
    }

    /// Base class for Requests, Responses and Notifications.
    /// In contrast to "BaseObject", a Message owns the backing JsonValue of the
    /// whole object tree; i.e. value_ serves as root.
    #[derive(Debug, Clone)]
    pub struct Message {
        value: Value,
    }

    impl Message {
        pub fn new() -> Self {
            let mut map = Map::new();
            map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
            Message {
                value: Value::Object(map),
            }
        }

        pub fn with_value(value: Value) -> Self {
            if !value.is_object() {
                panic!("Value must be an object");
            }
            Message { value }
        }

        pub fn get_json_value(&mut self) -> &mut Value {
            &mut self.value
        }
    }

    impl BaseJsonAccessor for Message {
        fn object(&self) -> &Map<String, Value> {
            self.value.as_object().unwrap()
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            self.value.as_object_mut().unwrap()
        }
    }

    impl_json_string_accessors!(Message, jsonrpc);

    /// Base class for complex type that might be part of a Message.
    /// Instead of creating theses directly, use the accessors on the
    /// root Message or a parent object.
    #[derive(Debug, Clone)]
    pub struct NestedJsonAccessor {
        object: Map<String, Value>,
    }

    impl NestedJsonAccessor {
        pub fn new(object: Map<String, Value>) -> Self {
            NestedJsonAccessor { object }
        }
    }

    impl BaseJsonAccessor for NestedJsonAccessor {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for NestedJsonAccessor {
        fn from_json_object(object: Map<String, Value>) -> Self {
            NestedJsonAccessor::new(object)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseError {
        object: Map<String, Value>,
    }

    impl ResponseError {
        pub fn new(object: Map<String, Value>) -> Self {
            ResponseError { object }
        }
    }

    impl BaseJsonAccessor for ResponseError {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for ResponseError {
        fn from_json_object(object: Map<String, Value>) -> Self {
            ResponseError::new(object)
        }
    }

    impl_json_int_accessors!(ResponseError, code);
    impl_json_string_accessors!(ResponseError, message);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InitializeParams {
        object: Map<String, Value>,
    }

    impl InitializeParams {
        pub fn new(object: Map<String, Value>) -> Self {
            InitializeParams { object }
        }
    }

    impl BaseJsonAccessor for InitializeParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for InitializeParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            InitializeParams::new(object)
        }
    }

    impl_json_int_accessors!(InitializeParams, processId);
    impl_json_string_accessors!(InitializeParams, rootPath);
    impl_json_string_accessors!(InitializeParams, rootUri);
    impl_json_string_accessors!(InitializeParams, trace);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FileListParams {
        object: Map<String, Value>,
    }

    impl FileListParams {
        pub fn new(object: Map<String, Value>) -> Self {
            FileListParams { object }
        }
    }

    impl BaseJsonAccessor for FileListParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for FileListParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            FileListParams::new(object)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FileSystemWatcher {
        object: Map<String, Value>,
    }

    impl FileSystemWatcher {
        pub fn new(object: Map<String, Value>) -> Self {
            FileSystemWatcher { object }
        }
    }

    impl BaseJsonAccessor for FileSystemWatcher {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for FileSystemWatcher {
        fn from_json_object(object: Map<String, Value>) -> Self {
            FileSystemWatcher::new(object)
        }
    }

    impl_json_string_accessors!(FileSystemWatcher, globPattern);
    impl_json_int_accessors!(FileSystemWatcher, kind);

    pub mod WatchKind {
        pub const kCreate: i64 = 1;
        pub const kChange: i64 = 2;
        pub const kDelete: i64 = 4;
        pub const kAll: i64 = kCreate | kChange | kDelete;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DidChangeWatchedFilesRegistrationOptions {
        object: Map<String, Value>,
    }

    impl DidChangeWatchedFilesRegistrationOptions {
        pub fn new(object: Map<String, Value>) -> Self {
            DidChangeWatchedFilesRegistrationOptions { object }
        }
    }

    impl BaseJsonAccessor for DidChangeWatchedFilesRegistrationOptions {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for DidChangeWatchedFilesRegistrationOptions {
        fn from_json_object(object: Map<String, Value>) -> Self {
            DidChangeWatchedFilesRegistrationOptions::new(object)
        }
    }

    impl_json_array_object_accessors!(
        DidChangeWatchedFilesRegistrationOptions,
        FileSystemWatcher,
        watchers
    );

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FileEvent {
        object: Map<String, Value>,
    }

    impl FileEvent {
        pub fn new(object: Map<String, Value>) -> Self {
            FileEvent { object }
        }
    }

    impl BaseJsonAccessor for FileEvent {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for FileEvent {
        fn from_json_object(object: Map<String, Value>) -> Self {
            FileEvent::new(object)
        }
    }

    impl_json_string_accessors!(FileEvent, uri);
    impl_json_int_accessors!(FileEvent, type);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DidChangeWatchedFilesParams {
        object: Map<String, Value>,
    }

    impl DidChangeWatchedFilesParams {
        pub fn new(object: Map<String, Value>) -> Self {
            DidChangeWatchedFilesParams { object }
        }
    }

    impl BaseJsonAccessor for DidChangeWatchedFilesParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for DidChangeWatchedFilesParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            DidChangeWatchedFilesParams::new(object)
        }
    }

    impl_json_array_object_accessors!(DidChangeWatchedFilesParams, FileEvent, changes);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveOptions {
        object: Map<String, Value>,
    }

    impl SaveOptions {
        pub fn new(object: Map<String, Value>) -> Self {
            SaveOptions { object }
        }
    }

    impl BaseJsonAccessor for SaveOptions {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for SaveOptions {
        fn from_json_object(object: Map<String, Value>) -> Self {
            SaveOptions::new(object)
        }
    }

    impl_json_bool_accessors!(SaveOptions, includeText);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextDocumentSyncOptions {
        object: Map<String, Value>,
    }

    impl TextDocumentSyncOptions {
        pub fn new(object: Map<String, Value>) -> Self {
            TextDocumentSyncOptions { object }
        }
    }

    impl BaseJsonAccessor for TextDocumentSyncOptions {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for TextDocumentSyncOptions {
        fn from_json_object(object: Map<String, Value>) -> Self {
            TextDocumentSyncOptions::new(object)
        }
    }

    impl_json_bool_accessors!(TextDocumentSyncOptions, openClose);
    impl_json_int_accessors!(TextDocumentSyncOptions, change);
    impl_json_bool_accessors!(TextDocumentSyncOptions, willSave);
    impl_json_bool_accessors!(TextDocumentSyncOptions, willSaveWaitUntil);
    impl_json_object_accessors!(TextDocumentSyncOptions, save, SaveOptions);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServerCapabilities {
        object: Map<String, Value>,
    }

    impl ServerCapabilities {
        pub fn new(object: Map<String, Value>) -> Self {
            ServerCapabilities { object }
        }
    }

    impl BaseJsonAccessor for ServerCapabilities {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for ServerCapabilities {
        fn from_json_object(object: Map<String, Value>) -> Self {
            ServerCapabilities::new(object)
        }
    }

    impl_json_object_accessors!(
        ServerCapabilities,
        textDocumentSync,
        TextDocumentSyncOptions
    );
    impl_json_bool_accessors!(ServerCapabilities, definitionProvider);
    impl_json_bool_accessors!(ServerCapabilities, documentSymbolProvider);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InitializeResult {
        object: Map<String, Value>,
    }

    impl InitializeResult {
        pub fn new(object: Map<String, Value>) -> Self {
            InitializeResult { object }
        }
    }

    impl BaseJsonAccessor for InitializeResult {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for InitializeResult {
        fn from_json_object(object: Map<String, Value>) -> Self {
            InitializeResult::new(object)
        }
    }

    impl_json_object_accessors!(InitializeResult, capabilities, ServerCapabilities);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Registration {
        object: Map<String, Value>,
    }

    impl Registration {
        pub fn new(object: Map<String, Value>) -> Self {
            Registration { object }
        }
    }

    impl BaseJsonAccessor for Registration {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for Registration {
        fn from_json_object(object: Map<String, Value>) -> Self {
            Registration::new(object)
        }
    }

    impl_json_string_accessors!(Registration, id);
    impl_json_string_accessors!(Registration, method);
    impl_json_dynamic_object_accessors!(Registration, registerOptions);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RegistrationParams {
        object: Map<String, Value>,
    }

    impl RegistrationParams {
        pub fn new(object: Map<String, Value>) -> Self {
            RegistrationParams { object }
        }
    }

    impl BaseJsonAccessor for RegistrationParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for RegistrationParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            RegistrationParams::new(object)
        }
    }

    impl_json_array_object_accessors!(RegistrationParams, Registration, registrations);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JsonPosition {
        object: Map<String, Value>,
    }

    impl JsonPosition {
        pub fn new(object: Map<String, Value>) -> Self {
            JsonPosition { object }
        }
    }

    impl BaseJsonAccessor for JsonPosition {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for JsonPosition {
        fn from_json_object(object: Map<String, Value>) -> Self {
            JsonPosition::new(object)
        }
    }

    impl_json_int_accessors!(JsonPosition, line);
    impl_json_int_accessors!(JsonPosition, character);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Range {
        object: Map<String, Value>,
    }

    impl Range {
        pub fn new(object: Map<String, Value>) -> Self {
            Range { object }
        }
    }

    impl BaseJsonAccessor for Range {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for Range {
        fn from_json_object(object: Map<String, Value>) -> Self {
            Range::new(object)
        }
    }

    impl_json_object_accessors!(Range, start, JsonPosition);
    impl_json_object_accessors!(Range, end, JsonPosition);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Location {
        object: Map<String, Value>,
    }

    impl Location {
        pub fn new(object: Map<String, Value>) -> Self {
            Location { object }
        }
    }

    impl BaseJsonAccessor for Location {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for Location {
        fn from_json_object(object: Map<String, Value>) -> Self {
            Location::new(object)
        }
    }

    impl Location {
        pub fn set_to(&mut self, position: SourcePosition) {
            self.set_uri(&source_file_map::SourceFileMap::AbsolutePath(position.source));
            self.range().start().set_line(position.start.line);
            self.range().start().set_character(position.start.column);
            self.range().end().set_line(position.end.line);
            self.range().end().set_character(position.end.column);
        }
    }

    impl_json_string_accessors!(Location, uri);
    impl_json_object_accessors!(Location, range, Range);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextDocumentIdentifier {
        object: Map<String, Value>,
    }

    impl TextDocumentIdentifier {
        pub fn new(object: Map<String, Value>) -> Self {
            TextDocumentIdentifier { object }
        }
    }

    impl BaseJsonAccessor for TextDocumentIdentifier {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for TextDocumentIdentifier {
        fn from_json_object(object: Map<String, Value>) -> Self {
            TextDocumentIdentifier::new(object)
        }
    }

    impl_json_string_accessors!(TextDocumentIdentifier, uri);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextDocumentPositionParams {
        object: Map<String, Value>,
    }

    impl TextDocumentPositionParams {
        pub fn new(object: Map<String, Value>) -> Self {
            TextDocumentPositionParams { object }
        }
    }

    impl BaseJsonAccessor for TextDocumentPositionParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for TextDocumentPositionParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            TextDocumentPositionParams::new(object)
        }
    }

    impl_json_object_accessors!(
        TextDocumentPositionParams,
        textDocument,
        TextDocumentIdentifier
    );
    impl_json_object_accessors!(TextDocumentPositionParams, position, JsonPosition);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Diagnostic {
        object: Map<String, Value>,
    }

    impl Diagnostic {
        pub fn new(object: Map<String, Value>) -> Self {
            Diagnostic { object }
        }
    }

    impl BaseJsonAccessor for Diagnostic {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for Diagnostic {
        fn from_json_object(object: Map<String, Value>) -> Self {
            Diagnostic::new(object)
        }
    }

    pub mod DiagnosticSeverity {
        pub const kError: i64 = 1;
        pub const kWarning: i64 = 2;
        pub const kInformation: i64 = 3;
        pub const kHint: i64 = 4;
    }

    impl_json_object_accessors!(Diagnostic, range, Range);
    impl_json_int_accessors!(Diagnostic, severity);
    impl_json_string_accessors!(Diagnostic, source);
    impl_json_string_accessors!(Diagnostic, message);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PublishDiagnosticsParams {
        object: Map<String, Value>,
    }

    impl PublishDiagnosticsParams {
        pub fn new(object: Map<String, Value>) -> Self {
            PublishDiagnosticsParams { object }
        }
    }

    impl BaseJsonAccessor for PublishDiagnosticsParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for PublishDiagnosticsParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            PublishDiagnosticsParams::new(object)
        }
    }

    impl_json_string_accessors!(PublishDiagnosticsParams, uri);
    impl_json_array_object_accessors!(PublishDiagnosticsParams, Diagnostic, diagnostics);

    pub mod SymbolKind {
        pub const kFile: i64 = 1;
        pub const kNamespace: i64 = 3;
        pub const kClass: i64 = 5;
        pub const kMethod: i64 = 6;
        pub const kProperty: i64 = 7;
        pub const kField: i64 = 8;
        pub const kConstructor: i64 = 9;
        pub const kFunction: i64 = 12;
        pub const kVariable: i64 = 13;
        pub const kConstant: i64 = 14;
        pub const kStruct: i64 = 23;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DocumentSymbolParams {
        object: Map<String, Value>,
    }

    impl DocumentSymbolParams {
        pub fn new(object: Map<String, Value>) -> Self {
            DocumentSymbolParams { object }
        }
    }

    impl BaseJsonAccessor for DocumentSymbolParams {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for DocumentSymbolParams {
        fn from_json_object(object: Map<String, Value>) -> Self {
            DocumentSymbolParams::new(object)
        }
    }

    impl_json_object_accessors!(
        DocumentSymbolParams,
        textDocument,
        TextDocumentIdentifier
    );

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SymbolInformation {
        object: Map<String, Value>,
    }

    impl SymbolInformation {
        pub fn new(object: Map<String, Value>) -> Self {
            SymbolInformation { object }
        }
    }

    impl BaseJsonAccessor for SymbolInformation {
        fn object(&self) -> &Map<String, Value> {
            &self.object
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            &mut self.object
        }
    }

    impl FromJsonObject for SymbolInformation {
        fn from_json_object(object: Map<String, Value>) -> Self {
            SymbolInformation::new(object)
        }
    }

    impl_json_string_accessors!(SymbolInformation, name);
    impl_json_int_accessors!(SymbolInformation, kind);
    impl_json_object_accessors!(SymbolInformation, location, Location);
    impl_json_string_accessors!(SymbolInformation, containerName);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request<T>
    where
        T: FromJsonObject,
    {
        message: Message,
        phantom: std::marker::PhantomData<T>,
    }

    impl<T: FromJsonObject> Request<T> {
        pub fn new() -> Self {
            Request {
                message: Message::new(),
                phantom: std::marker::PhantomData,
            }
        }

        pub fn with_value(value: Value) -> Self {
            Request {
                message: Message::with_value(value),
                phantom: std::marker::PhantomData,
            }
        }

        pub fn params(&self) -> T {
            let object = self.message.get_object_property("params");
            T::from_json_object(object)
        }
    }

    impl<T: FromJsonObject> BaseJsonAccessor for Request<T> {
        fn object(&self) -> &Map<String, Value> {
            self.message.object()
        }

        fn object_mut(&mut self) -> &mut Map<String, Value> {
            self.message.object_mut()
        }
    }

    impl<T: FromJsonObject> FromJsonObject for Request<T> {
        fn from_json_object(object: Map<String, Value>) -> Self {
            let value = Value::Object(object);
            Request::with_value(value)
        }
    }

    impl_json_int_accessors!(Request<InitializeParams>, id);
    impl_json_string_accessors!(Request<InitializeParams>, method);

    pub type InitializeRequest = Request<InitializeParams>;