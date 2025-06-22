// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/tracing/code-data-source.rs

use protobuf::Message;
use std::collections::HashMap;
use std::sync::Mutex;

// Placeholder for protos
mod perfetto {
    pub mod protos {
        pub mod common {
            pub mod data_source_descriptor {
                include!(concat!(env!("OUT_DIR"), "/perfetto_common_data_source_descriptor.rs"));
            }
        }
        pub mod config {
            pub mod chrome {
                pub mod v8_config {
                    include!(concat!(env!("OUT_DIR"), "/perfetto_config_chrome_v8_config.rs"));
                }
            }
        }
        pub mod trace {
            pub mod chrome {
                pub mod v8_pbzero {
                    include!(concat!(env!("OUT_DIR"), "/perfetto_trace_chrome_v8.rs"));
                }
            }
        }
    }
}

use perfetto::protos::common::data_source_descriptor::DataSourceDescriptor;
use perfetto::protos::config::chrome::v8_config::V8Config;
use perfetto::protos::trace::chrome::v8_pbzero::{
    InternedV8JsFunction, InternedV8JsScript, InternedV8String, TracePacket,
};

// Placeholder types and enums from V8.
// These need to be properly defined based on V8's internal representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IsolateId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ScriptId(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Address(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Size(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CodeRange {}

impl CodeRange {
    fn base(&self) -> Address {
        Address(0)
    }
    fn size(&self) -> Size {
        Size(0)
    }
    fn embedded_blob_code_copy(&self) -> Option<Address> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IsolateGroup {}

impl IsolateGroup {
    fn current() -> Self {
        IsolateGroup {}
    }

    fn GetCodeRange(&self) -> *const CodeRange {
        std::ptr::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IsolateHeap {
    code_range: Option<CodeRange>,
}

impl IsolateHeap {
    fn code_range(&self) -> Option<&CodeRange> {
        self.code_range.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Isolate {
    id: IsolateId,
    embedded_blob_code_address: Address,
    embedded_blob_code_size_: Size,
    heap: IsolateHeap,
}

impl Isolate {
    fn new(id: IsolateId) -> Self {
        Isolate {
            id,
            embedded_blob_code_address: Address(0),
            embedded_blob_code_size_: Size(0),
            heap: IsolateHeap { code_range: None },
        }
    }
    fn id(&self) -> IsolateId {
        self.id
    }
    fn embedded_blob_code(&self) -> Address {
        self.embedded_blob_code_address
    }
    fn embedded_blob_code_size(&self) -> Size {
        self.embedded_blob_code_size_
    }
    fn heap(&self) -> &IsolateHeap {
        &self.heap
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Script {
    id: ScriptId,
    compilation_type: CompilationType,
    script_type: ScriptType,
    name: Option<String>,
    source: Option<String>,
}

impl Script {
    fn id(&self) -> ScriptId {
        self.id
    }
    fn compilation_type(&self) -> CompilationType {
        self.compilation_type
    }
    fn script_type(&self) -> ScriptType {
        self.script_type
    }
    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    fn source(&self) -> Option<&String> {
        self.source.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CompilationType {
    kEval,
    kOther, //Added as placeholder
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ScriptType {
    kNative,
    kExtension,
    kNormal,
    kWasm,
    kInspector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SharedFunctionInfo {}

impl SharedFunctionInfo {
    fn DebugName<'a>(_isolate: &'a Isolate, _info: &SharedFunctionInfo) -> String {
        String::from("DebugName")
    }
    fn StartPosition(&self) -> i32 {
        0
    }
    fn is_toplevel(&self) -> bool {
        false
    }
    fn kind(&self) -> FunctionKind {
        FunctionKind::kNormalFunction
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FunctionKind {
    kNormalFunction,
    kModule,
    kModuleWithTopLevelAwait,
    kBaseConstructor,
    kDefaultBaseConstructor,
    kDefaultDerivedConstructor,
    kDerivedConstructor,
    kGetterFunction,
    kStaticGetterFunction,
    kSetterFunction,
    kStaticSetterFunction,
    kArrowFunction,
    kAsyncArrowFunction,
    kAsyncFunction,
    kAsyncConciseMethod,
    kStaticAsyncConciseMethod,
    kAsyncConciseGeneratorMethod,
    kStaticAsyncConciseGeneratorMethod,
    kAsyncGeneratorFunction,
    kGeneratorFunction,
    kConciseGeneratorMethod,
    kStaticConciseGeneratorMethod,
    kConciseMethod,
    kStaticConciseMethod,
    kClassMembersInitializerFunction,
    kClassStaticInitializerFunction,
    kInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct String {
    data: std::string::String,
}

impl String {
    fn data(&self) -> &std::string::String {
        &self.data
    }
}

// Helper functions to simulate V8's IsString.
fn IsString(_obj: Option<&String>) -> bool {
    true // In Rust, Option<&String> is already a safe way to handle potentially null strings.
}

fn Cast<T>(s: Option<&String>) -> Option<&String> {
    s //No cast is needed here
}

mod base {
    pub mod os {
        pub fn GetCurrentProcessId() -> i32 {
            0
        }
    }
}

mod perfetto_utils {
    use super::{InternedV8String, String};
    use protobuf::Message;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PerfettoV8String(pub String);

    impl PerfettoV8String {
        pub fn WriteToProto(&self, proto: &mut InternedV8String) {
            proto.set_str(self.0.data().clone());
        }
    }
}

use base::os;
use perfetto_utils::PerfettoV8String;

mod perfetto_logger {
    pub fn OnCodeDataSourceStart() {}
    pub fn OnCodeDataSourceStop() {}
}

#[derive(Default)]
struct SerializedInternedData {
    v8_isolates: Vec<InternedV8IsolateProto>,
    v8_js_scripts: Vec<InternedV8JsScript>,
    v8_js_functions: Vec<InternedV8JsFunction>,
    v8_wasm_scripts: Vec<InternedV8WasmScriptProto>,
    v8_js_function_names: Vec<InternedV8JsFunctionNameProto>,
}

impl SerializedInternedData {
    fn add_v8_isolate(&mut self) -> &mut InternedV8IsolateProto {
        self.v8_isolates.push(InternedV8IsolateProto::new());
        self.v8_isolates.last_mut().unwrap()
    }

    fn add_v8_js_script(&mut self) -> &mut InternedV8JsScript {
        self.v8_js_scripts.push(InternedV8JsScript::new());
        self.v8_js_scripts.last_mut().unwrap()
    }

    fn add_v8_js_function(&mut self) -> &mut InternedV8JsFunction {
        self.v8_js_functions.push(InternedV8JsFunction::new());
        self.v8_js_functions.last_mut().unwrap()
    }

    fn add_v8_wasm_script(&mut self) -> &mut InternedV8WasmScriptProto {
        self.v8_wasm_scripts.push(InternedV8WasmScriptProto::new());
        self.v8_wasm_scripts.last_mut().unwrap()
    }

    fn add_v8_js_function_name(&mut self) -> &mut InternedV8JsFunctionNameProto {
        self.v8_js_function_names.push(InternedV8JsFunctionNameProto::new());
        self.v8_js_function_names.last_mut().unwrap()
    }

    fn GetRanges(&mut self) -> Vec<Vec<u8>> {
        let mut ranges = Vec::new();

        for isolate in &self.v8_isolates {
            let mut buffer = Vec::new();
            isolate.write_to_vec(&mut buffer).unwrap();
            ranges.push(buffer);
        }

        for script in &self.v8_js_scripts {
            let mut buffer = Vec::new();
            script.write_to_vec(&mut buffer).unwrap();
            ranges.push(buffer);
        }

        for function in &self.v8_js_functions {
            let mut buffer = Vec::new();
            function.write_to_vec(&mut buffer).unwrap();
            ranges.push(buffer);
        }

        for wasm_script in &self.v8_wasm_scripts {
            let mut buffer = Vec::new();
            wasm_script.write_to_vec(&mut buffer).unwrap();
            ranges.push(buffer);
        }

        for function_name in &self.v8_js_function_names {
            let mut buffer = Vec::new();
            function_name.write_to_vec(&mut buffer).unwrap();
            ranges.push(buffer);
        }

        ranges
    }

    fn Reset(&mut self) {
        self.v8_isolates.clear();
        self.v8_js_scripts.clear();
        self.v8_js_functions.clear();
        self.v8_wasm_scripts.clear();
        self.v8_js_function_names.clear();
    }
}

#[derive(protobuf::Message, Clone, PartialEq, Debug)]
struct InternedV8IsolateProto {
    #[protobuf(varint, tag = "1")]
    iid: u64,
    #[protobuf(varint, tag = "2")]
    isolate_id: u64,
    #[protobuf(varint, tag = "3")]
    pid: i32,
    #[protobuf(varint, tag = "4")]
    embedded_blob_code_start_address: u64,
    #[protobuf(varint, tag = "5")]
    embedded_blob_code_size: u64,
    #[protobuf(singular, name = "code_range", tag = "6")]
    code_range: ::std::option::Option<V8CodeRangeProto>,
}

impl InternedV8IsolateProto {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_iid(&mut self, v: u64) {
        self.iid = v;
    }
    pub fn set_isolate_id(&mut self, v: u64) {
        self.isolate_id = v;
    }
    pub fn set_pid(&mut self, v: i32) {
        self.pid = v;
    }
    pub fn set_embedded_blob_code_start_address(&mut self, v: u64) {
        self.embedded_blob_code_start_address = v;
    }
    pub fn set_embedded_blob_code_size(&mut self, v: u64) {
        self.embedded_blob_code_size = v;
    }
    pub fn set_code_range(&mut self) -> &mut V8CodeRangeProto {
        self.code_range.get_or_insert_with(|| V8CodeRangeProto::new())
    }
}

#[derive(protobuf::Message, Clone, PartialEq, Debug)]
struct V8CodeRangeProto {
    #[protobuf(varint, tag = "1")]
    base_address: u64,
    #[protobuf(varint, tag = "2")]
    size: u64,
    #[protobuf(bool, tag = "3")]
    is_process_wide: bool,
    #[protobuf(varint, tag = "4")]
    embedded_blob_code_copy_start_address: u64,
}

impl V8CodeRangeProto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_base_address(&mut self, v: u64) {
        self.base_address = v;
    }

    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }

    pub fn set_is_process_wide(&mut self, v: bool) {
        self.is_process_wide = v;
    }

    pub fn set_embedded_blob_code_copy_start_address(&mut self, v: u64) {
        self.embedded_blob_code_copy_start_address = v;
    }
}

#[derive(protobuf::Message, Clone, PartialEq, Debug)]
struct InternedV8WasmScriptProto {
    #[protobuf(varint, tag = "1")]
    iid: u64,
    #[protobuf(varint, tag = "2")]
    script_id: i32,
    #[protobuf(string, tag = "3")]
    url: String,
}

impl InternedV8WasmScriptProto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_iid(&mut self, v: u64) {
        self.iid = v;
    }

    pub fn set_script_id(&mut self, v: i32) {
        self.script_id = v;
    }

    pub fn set_url(&mut self, v: String) {
        self.url = v;
    }
}

#[derive(protobuf::Message, Clone, PartialEq, Debug)]
struct InternedV8JsFunctionNameProto {
    #[protobuf(varint, tag = "1")]
    iid: u64,
    #[protobuf(string, tag = "2")]
    str: String,
}

impl InternedV8JsFunctionNameProto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_iid(&mut self, v: u64) {
        self.iid = v;
    }

    pub fn set_str(&mut self, v: String) {
        self.str = v;
    }
}

pub struct CodeDataSourceIncrementalState {
    log_script_sources_: bool,
    log_instructions_: bool,
    initialized_: bool,
    isolates_: HashMap<IsolateId, u64>,
    scripts_: HashMap<CodeDataSourceIncrementalState::ScriptUniqueId, u64>,
    functions_: HashMap<CodeDataSourceIncrementalState::Function, u64>,
    js_function_names_: HashMap<PerfettoV8String, u64>,
    next_isolate_iid_: u64,
    next_script_iid_: u64,
    next_function_iid_: u64,
    next_js_function_name_iid_: u64,
    serialized_interned_data_: SerializedInternedData,
}

impl CodeDataSourceIncrementalState {
    type ScriptUniqueId = (IsolateId, ScriptId);
    type Function = (u64, bool, i32); //v8_js_script_iid, is_toplevel, start_position

    pub fn new() -> Self {
        CodeDataSourceIncrementalState {
            log_script_sources_: false,
            log_instructions_: false,
            initialized_: false,
            isolates_: HashMap::new(),
            scripts_: HashMap::new(),
            functions_: HashMap::new(),
            js_function_names_: HashMap::new(),
            next_isolate_iid_: 1,
            next_script_iid_: 1,
            next_function_iid_: 1,
            next_js_function_name_iid_: 1,
            serialized_interned_data_: SerializedInternedData::default(),
        }
    }

    pub fn Init(&mut self, context: &CodeDataSource::TraceContext) {
        if let Some(ds) = context.GetDataSourceLocked() {
            let config = &ds.config;
            self.log_script_sources_ = config.log_script_sources;
            self.log_instructions_ = config.log_instructions;
        }
        self.initialized_ = true;
    }

    pub fn FlushInternedData(&mut self, packet: &mut CodeDataSource::TraceContext::TracePacketHandle) {
        let ranges = self.serialized_interned_data_.GetRanges();

        for buffer in ranges.iter() {
            packet.AppendScatteredBytes(TracePacket::kInternedDataFieldNumber, buffer);
        }

        self.serialized_interned_data_.Reset();
    }

    pub fn InternIsolate(&mut self, isolate: &Isolate) -> u64 {
        if let Some(&iid) = self.isolates_.get(&isolate.id()) {
            return iid;
        }

        let iid = self.next_isolate_iid();
        self.isolates_.insert(isolate.id(), iid);

        let isolate_proto = self.serialized_interned_data_.add_v8_isolate();
        isolate_proto.set_iid(iid);
        isolate_proto.set_isolate_id(isolate.id().0 as u64);
        isolate_proto.set_pid(os::GetCurrentProcessId());
        isolate_proto.set_embedded_blob_code_start_address(isolate.embedded_blob_code().0);
        isolate_proto.set_embedded_blob_code_size(isolate.embedded_blob_code_size().0 as u64);

        if let Some(code_range) = isolate.heap().code_range() {
            let v8_code_range = isolate_proto.set_code_range();
            v8_code_range.set_base_address(code_range.base().0);
            v8_code_range.set_size(code_range.size().0 as u64);

            //FIXME(42204573): Belongs to isolate group, not process.
            // if code_range == IsolateGroup::current().GetCodeRange() {
            //     v8_code_range.set_is_process_wide(true);
            // }

            if let Some(embedded_builtins_start) = code_range.embedded_blob_code_copy() {
                v8_code_range.set_embedded_blob_code_copy_start_address(embedded_builtins_start.0);
            }
        }

        iid
    }

    pub fn InternJsScript(&mut self, isolate: &Isolate, script: &Script) -> u64 {
        let script_unique_id = (isolate.id(), script.id());
        if let Some(&iid) = self.scripts_.get(&script_unique_id) {
            return iid;
        }

        let iid = self.next_script_iid();
        self.scripts_.insert(script_unique_id, iid);

        let proto = self.serialized_interned_data_.add_v8_js_script();
        proto.set_iid(iid);
        proto.set_script_id(script.id().0);
        proto.set_type(GetJsScriptType(script));

        if let Some(name) = script.name() {
            PerfettoV8String(String { data: name.clone() }).WriteToProto(proto.mut_name());
        }

        if self.log_script_sources() {
            if let Some(source) = script.source() {
                PerfettoV8String(String { data: source.clone() }).WriteToProto(proto.mut_source());
            }
        }

        iid
    }

    pub fn InternJsFunction(
        &mut self,
        isolate: &Isolate,
        info: &SharedFunctionInfo,
        v8_js_script_iid: u64,
        line_num: i32,
        column_num: i32,
    ) -> u64 {
        let function_name = SharedFunctionInfo::DebugName(isolate, info);
        let v8_js_function_name_iid = self.InternJsFunctionName(String { data: function_name });

        let function_key = (
            v8_js_script_iid,
            info.is_toplevel(),
            info.StartPosition(),
        );

        if let Some(&iid) = self.functions_.get(&function_key) {
            return iid;
        }

        let iid = self.next_function_iid();
        self.functions_.insert(function_key, iid);

        let function_proto = self.serialized_interned_data_.add_v8_js_function();
        function_proto.set_iid(iid);
        function_proto.set_v8_js_function_name_iid(v8_js_function_name_iid);
        function_proto.set_v8_js_script_iid(v8_js_script_iid);
        function_proto.set_kind(GetJsFunctionKind(info.kind()));

        let start_position = info.StartPosition();
        if start_position >= 0 {
            function_proto.set_byte_offset(start_position as u32);
        }

        if line_num > 0 && column_num > 0 {
            function_proto.set_line(line_num as u32);
            function_proto.set_column(column_num as u32);
        }

        iid
    }

    pub fn InternWasmScript(
        &mut self,
        isolate: &Isolate,
        script_id: i32,
        url: &str,
    ) -> u64 {
        let script_unique_id = (isolate.id(), ScriptId(script_id));
        if let Some(&iid) = self.scripts_.get(&script_unique_id) {
            return iid;
        }

        let iid = self.next_script_iid();
        self.scripts_.insert(script_unique_id, iid);

        let script = self.serialized_interned_data_.add_v8_wasm_script();
        script.set_iid(iid);
        script.set_script_id(script_id);
        script.set_url(String::from(url));

        iid
    }

    pub fn InternJsFunctionName(&mut self, function_name: String) -> u64 {
        let perfetto_v8_string = PerfettoV8String(function_name.clone());
        if let Some(&iid) = self.js_function_names_.get(&perfetto_v8_string) {
            return iid;
        }

        let iid = self.next_js_function_name_iid();
        self.js_function_names_.insert(perfetto_v8_string.clone(), iid);

        let v8_function_name = self.serialized_interned_data_.add_v8_js_function_name();
        v8_function_name.set_iid(iid);
        PerfettoV8String(function_name).WriteToProto(v8_function_name);
        iid
    }

    fn log_script_sources(&self) -> bool {
        self.log_script_sources_
    }

    #[allow(dead_code)]
    fn log_instructions(&self) -> bool {
        self.log_instructions_
    }

    fn next_isolate_iid(&mut self) -> u64 {
        let iid = self.next_isolate_iid_;
        self.next_isolate_iid_ += 1;
        iid
    }

    fn next_script_iid(&mut self) -> u64 {
        let iid = self.next_script_iid_;
        self.next_script_iid_ += 1;
        iid
    }

    fn next_function_iid(&mut self) -> u64 {
        let iid = self.next_function_iid_;
        self.next_function_iid_ += 1;
        iid
    }

    fn next_js_function_name_iid(&mut self) -> u64 {
        let iid = self.next_js_function_name_iid_;
        self.next_js_function_name_iid_ += 1;
        iid
    }
}

fn GetJsScriptType(script: &Script) -> InternedV8JsScript::Type {
    if script.compilation_type() == CompilationType::kEval {
        return InternedV8JsScript::Type::TYPE_EVAL;
    }

    match script.script_type() {
        ScriptType::kNative => InternedV8JsScript::Type::TYPE_NATIVE,
        ScriptType::kExtension => InternedV8JsScript::Type::TYPE_EXTENSION,
        ScriptType::kNormal => InternedV8JsScript::Type::TYPE_NORMAL,
        ScriptType::kWasm => InternedV8JsScript::Type::TYPE_UNSET, //UNREACHABLE in c++
        ScriptType::kInspector => InternedV8JsScript::Type::TYPE_INSPECTOR,
    }
}

fn GetJsFunctionKind(kind: FunctionKind) -> InternedV8JsFunction::Kind {
    match kind {
        FunctionKind::kNormalFunction => InternedV8JsFunction::Kind::KIND_NORMAL_FUNCTION,
        FunctionKind::kModule => InternedV8JsFunction::Kind::KIND_MODULE,
        FunctionKind::kModuleWithTopLevelAwait => InternedV8JsFunction::Kind::KIND_ASYNC_MODULE,
        FunctionKind::kBaseConstructor => InternedV8JsFunction::Kind::KIND_BASE_CONSTRUCTOR,
        FunctionKind::kDefaultBaseConstructor => {
            InternedV8JsFunction::Kind::KIND_DEFAULT_BASE_CONSTRUCTOR
        }
        FunctionKind::kDefaultDerivedConstructor => {
            InternedV8JsFunction::Kind::KIND_DEFAULT_DERIVED_CONSTRUCTOR
        }
        FunctionKind::kDerivedConstructor => InternedV8JsFunction::Kind::KIND_DERIVED_CONSTRUCTOR,
        FunctionKind::kGetterFunction => InternedV8JsFunction::Kind::KIND_GETTER_FUNCTION,
        FunctionKind::kStaticGetterFunction => InternedV8JsFunction::Kind::KIND_STATIC_GETTER_FUNCTION,
        FunctionKind::kSetterFunction => InternedV8JsFunction::Kind::KIND_SETTER_FUNCTION,
        FunctionKind::kStaticSetterFunction => InternedV8JsFunction::Kind::KIND_STATIC_SETTER_FUNCTION,
        FunctionKind::kArrowFunction => InternedV8JsFunction::Kind::KIND_ARROW_FUNCTION,
        FunctionKind::kAsyncArrowFunction => InternedV8JsFunction::Kind::KIND_ASYNC_ARROW_FUNCTION,
        FunctionKind::kAsyncFunction => InternedV8JsFunction::Kind::KIND_ASYNC_FUNCTION,
        FunctionKind::kAsyncConciseMethod => InternedV8JsFunction::Kind::KIND_ASYNC_CONCISE_METHOD,
        FunctionKind::kStaticAsyncConciseMethod => {
            InternedV8JsFunction::Kind::KIND_STATIC_ASYNC_CONCISE_METHOD
        }
        FunctionKind::kAsyncConciseGeneratorMethod => {
            InternedV8JsFunction::Kind::KIND_ASYNC_CONCISE_GENERATOR_METHOD
        }
        FunctionKind::kStaticAsyncConciseGeneratorMethod => {
            InternedV8JsFunction::Kind::KIND_STATIC_ASYNC_CONCISE_GENERATOR_METHOD
        }
        FunctionKind::kAsyncGeneratorFunction => {
            InternedV8JsFunction::Kind::KIND_ASYNC_GENERATOR_FUNCTION
        }
        FunctionKind::kGeneratorFunction => InternedV8JsFunction::Kind::KIND_GENERATOR_FUNCTION,
        FunctionKind::kConciseGeneratorMethod => {
            InternedV8JsFunction::Kind::KIND_CONCISE_GENERATOR_METHOD
        }
        FunctionKind::kStaticConciseGeneratorMethod => {
            InternedV8JsFunction::Kind::KIND_STATIC_CONCISE_GENERATOR_METHOD
        }
        FunctionKind::kConciseMethod => InternedV8JsFunction::Kind::KIND_CONCISE_METHOD,
        FunctionKind::kStaticConciseMethod => InternedV8JsFunction::Kind::KIND_STATIC_CONCISE_METHOD,
        FunctionKind::kClassMembersInitializerFunction => {
            InternedV8JsFunction::Kind::KIND_CLASS_MEMBERS_INITIALIZER_FUNCTION
        }
        FunctionKind::kClassStaticInitializerFunction => {
            InternedV8JsFunction::Kind::KIND_CLASS_STATIC_INITIALIZER_FUNCTION
        }
        FunctionKind::kInvalid => InternedV8JsFunction::Kind::KIND_INVALID,
    }
}

pub struct CodeDataSource {
    config: V8Config,
}

impl CodeDataSource {
    pub type TraceContext = CodeDataSourceTraceContext;
    pub type CodeDataSourceTraits = CodeDataSourceTraitsImpl;

    pub fn new() -> Self {
        CodeDataSource {
            config: V8Config::new(),
        }
    }

    pub fn Register() {
        let mut desc = DataSourceDescriptor::new();
        desc.set_name(String::from("dev.v8.code"));
        CodeDataSourceTraitsImpl::Register(desc);
    }

    pub fn OnSetup(&mut self, args: &SetupArgs) {
        self.config.merge_from_bytes(args.config.v8_config_raw.as_bytes()).unwrap();
    }

    pub fn OnStart(&self, _args: &StartArgs) {
        perfetto_logger::OnCodeDataSourceStart();
    }

    pub fn OnStop(&self, _args: &StopArgs) {
        perfetto_logger::OnCodeDataSourceStop();
    }

    struct CodeDataSourceTraceContext {
        data_source: Mutex<Option<CodeDataSource>>,
    }

    impl CodeDataSourceTraceContext {
        fn GetDataSourceLocked(&self) -> Option<&CodeDataSource> {
            self.data_source.lock().unwrap().as_ref()
        }
    }

    struct CodeDataSourceTraitsImpl;
    impl CodeDataSourceTraitsImpl {
        fn Register(_desc: DataSourceDescriptor) {}
    }
}

pub struct SetupArgs {
    config: Config,
}

pub struct Config {
    v8_config_raw: String,
}

pub struct StartArgs {}
pub struct StopArgs {}

// Placeholder for Base::Register
mod Base {
    pub fn Register(_desc: DataSourceDescriptor) {}
}

impl Default for CodeDataSource {
    fn default() -> Self {
        Self::new()
    }
}