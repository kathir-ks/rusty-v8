// Converted from V8 C++ source files:
// Header: v8-debugger.h
// Implementation: v8-debugger.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::sync::{Mutex, Weak};

use std::any::Any;
use std::os::raw::c_void;
use std::ptr;
use std::vec;

use crate::inspector::inspected_context::InspectedContext;
use crate::inspector::protocol::Debugger;
use crate::inspector::protocol::Runtime;
use crate::inspector::string_util::{string16_from_integer, string16_from_utf8, string_view_to_string16, to_protocol_string, to_protocol_string_with_type_check};
use crate::inspector::v8_debugger_agent_impl::V8DebuggerAgentImpl;
use crate::inspector::v8_debugger_id::V8DebuggerId;
use crate::inspector::v8_debugger_script::V8DebuggerScript;
use crate::inspector::v8_heap_profiler_agent_impl::V8HeapProfilerAgentImpl;
use crate::inspector::v8_inspector_impl::V8InspectorImpl;
use crate::inspector::v8_inspector_session_impl::V8InspectorSessionImpl;
use crate::inspector::v8_runtime_agent_impl::V8RuntimeAgentImpl;
use crate::inspector::v8_stack_trace_impl::V8StackTraceImpl;
use crate::inspector::v8_value_utils::create_data_property;
use v8::internal::kMaxInt;

//use v8::debug::{BreakpointId, BreakReasons, DebugDelegate, ExceptionBreakState};
//use v8::debug::Location as V8Location;
//use v8::{Context, Isolate, Local, MaybeLocal, Object, StackFrame as V8StackFrame, String as V8String, Value};

pub mod v8 {
    pub mod debug {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum ExceptionBreakState {
            NoBreakOnException,
            BreakOnUncaught,
            BreakOnAll,
        }
        pub enum BreakReason {
            kOOM,
            kException,
            kAssert,
            kScheduled,
            kAsyncStep,
            kAgent,
        }
        pub struct BreakReasons {
            reasons: Vec<BreakReason>,
        }
        impl BreakReasons {
            pub fn Add(&mut self, reason: BreakReason) {
                self.reasons.push(reason);
            }
            pub fn contains(&self, reason: BreakReason) -> bool {
                self.reasons.contains(&reason)
            }
        }

        impl From<Vec<BreakReason>> for BreakReasons {
            fn from(reasons: Vec<BreakReason>) -> Self {
                BreakReasons { reasons }
            }
        }

        impl BreakReasons {
            pub fn new(reasons: Vec<BreakReason>) -> Self {
                BreakReasons { reasons }
            }
        }

        pub type BreakpointId = i32;
        pub struct Location {
            line_number: i32,
            column_number: i32,
        }
        impl Location {
            pub fn new(line_number: i32, column_number: i32) -> Self {
                Location {
                    line_number,
                    column_number,
                }
            }
            pub fn GetLineNumber(&self) -> i32 {
                self.line_number
            }
            pub fn GetColumnNumber(&self) -> i32 {
                self.column_number
            }
        }
    }
    pub struct Value {}
    pub struct String {}
    pub struct Context {}
    pub struct Isolate {}
    pub struct Object {}
    pub struct Function {}
    pub struct Array {}
    pub struct StackFrame {}
    pub struct Script {}
    impl Script {
        pub fn WasCompiled(&self) -> bool {
            true
        }
        pub fn IsEmbedded(&self) -> bool {
            false
        }
        pub fn ContextId(&self) -> MaybeLocal<i32> {
            MaybeLocal { value: Some(1)}
        }
        pub fn Id(&self) -> i32 {
            1
        }
        pub fn GetNameOrSourceURL(&self) -> String {
            String{}
        }
        pub fn GetScriptNameOrSourceURL(&self) -> String {
            String{}
        }
        pub fn GetScriptName(&self) -> String {
            String{}
        }
    }
    pub struct MaybeLocal<T> {
        pub value: Option<T>
    }
    impl MaybeLocal<i32> {
        pub fn To(&self, contextId: &mut i32) -> bool {
            if let Some(v) = self.value {
                *contextId = v;
                return true;
            }
            false
        }
    }

    pub mod debug {
        pub struct GeneratorObject {}
        impl GeneratorObject {
            pub fn Cast(value: super::Value) -> GeneratorObject {
                GeneratorObject{}
            }
            pub fn IsSuspended(&self) -> bool {
                true
            }
        }
        pub enum ScopeType {
            Global,
            Local,
            With,
            Closure,
            Catch,
            Block,
            Script,
            Eval,
            Module,
            WasmExpressionStack,
        }
    }

    pub mod util {
        pub struct ScriptOrigin {}
        impl ScriptOrigin {
            pub fn ResourceName(&self) -> ResourceName {
                ResourceName{}
            }
            pub fn ScriptId(&self) -> i32 {
                1
            }
        }
        pub struct ResourceName {}
        impl ResourceName {
            pub fn IsString(&self) -> bool {
                true
            }
            pub fn As<T>(&self) -> T {
                T{}
            }
        }
    }
}

pub mod perfetto {
    pub struct Flow {}
    impl Flow {
        pub fn ProcessScoped(value: usize) -> Flow {
            Flow{}
        }
    }
}

pub mod tracing {
    pub fn TRACE_EVENT(arg1:(), arg2:&str, arg3:&str, arg4:usize){}
    pub fn TRACE_EVENT_BEGIN(arg1:(), arg2:&str, arg3:super::perfetto::Flow){}
    pub fn TRACE_EVENT_END0(arg1:(), arg2:&str){}
}
pub mod i {
    pub struct IsolateForSandbox {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapMode {
    kJson,
    kIdOnly,
    kPreview,
    kDeep,
}

#[derive(Debug)]
pub struct WrapSerializationOptions {
    pub maxDepth: i32,
    pub additionalParameters: v8::Object,
}

impl Default for WrapSerializationOptions {
    fn default() -> Self {
        WrapSerializationOptions {
            maxDepth: kMaxInt,
            additionalParameters: v8::Object {},
        }
    }
}

#[derive(Debug)]
pub struct WrapOptions {
    pub mode: WrapMode,
    pub serializationOptions: WrapSerializationOptions,
}

impl WrapOptions {
    pub fn new(mode: WrapMode) -> Self {
        WrapOptions {
            mode,
            serializationOptions: WrapSerializationOptions::default(),
        }
    }
}

pub type Response = crate::inspector::v8_inspector_session_impl::Response;
pub type TerminateExecutionCallback =
    crate::inspector::v8_inspector_session_impl::TerminateExecutionCallback;

pub struct AsyncStackTrace {
    description: String16,
    parent: Weak<AsyncStackTrace>,
    stack_trace: Option<V8StackTraceImpl>,
}

impl AsyncStackTrace {
    fn capture(debugger: &V8Debugger, description: String16, skip_top_frame:bool) -> std::shared_ptr::AsyncStackTrace {
        let async_stack = AsyncStackTrace{
            description: description,
            parent: Weak::new(),
            stack_trace: None,
        };

        std::shared_ptr::new(async_stack)
    }

    fn store(debugger: &V8Debugger, async_stack: std::shared_ptr::AsyncStackTrace) -> usize {
        1
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn parent(&self) -> Weak<AsyncStackTrace> {
        Weak::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CachedStackFrameKey {
    scriptId: i32,
    lineNumber: i32,
    columnNumber: i32,
}

impl CachedStackFrameKey {
    pub fn new(scriptId: i32, lineNumber: i32, columnNumber: i32) -> Self {
        CachedStackFrameKey {
            scriptId,
            lineNumber,
            columnNumber,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    functionName: String16,
    scriptId: i32,
    sourceURL: String16,
    lineNumber: i32,
    columnNumber: i32,
    hasSourceURLComment: bool,
}

impl StackFrame {
    pub fn new(functionName: String16, scriptId: i32, sourceURL: String16, lineNumber: i32, columnNumber: i32, hasSourceURLComment: bool) -> Self {
        StackFrame {
            functionName,
            scriptId,
            sourceURL,
            lineNumber,
            columnNumber,
            hasSourceURLComment,
        }
    }
    pub fn functionName(&self) -> String16 {
        String16{}
    }
    pub fn sourceURL(&self) -> String16 {
        String16{}
    }
}

const kMaxAsyncTaskStacks: usize = 8 * 1024;
const kMaxExternalParents: usize = 1 * 1024;
const kNoBreakpointId: i32 = 0;

fn cleanupExpiredWeakPointers<K, V>(map: &mut HashMap<K, Weak<V>>)
where
    K: Eq + std::hash::Hash,
{
    map.retain(|_, v| v.strong_count() > 0);
}

struct MatchPrototypePredicate<'a> {
    m_inspector: &'a V8InspectorImpl,
    m_context: v8::Context,
    m_prototype: v8::Object,
}

impl<'a> MatchPrototypePredicate<'a> {
    fn new(
        m_inspector: &'a V8InspectorImpl,
        m_context: v8::Context,
        m_prototype: v8::Object,
    ) -> Self {
        MatchPrototypePredicate {
            m_inspector,
            m_context,
            m_prototype,
        }
    }

    fn Filter(&self, object: v8::Object) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug)]
pub struct String16 {
    data: String,
}

impl String16 {
    pub fn from_utf8(s: &str) -> String16 {
        String16 { data: s.to_string() }
    }

    pub fn from_integer(i: i32) -> String16 {
        String16 { data: i.to_string() }
    }
    pub fn utf8(&self) -> String {
        String::from("")
    }

    pub fn length(&self) -> i32 {
        0
    }
}

impl fmt::Display for String16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum V8InternalValueType {
    kScope,
    kScopeList,
    kEntry,
    kPrivateMethod,
    kPrivateMethodList,
}

pub struct V8Debugger {
    m_isolate: *mut v8::Isolate,
    m_inspector: *mut V8InspectorImpl,
    m_enableCount: i32,
    m_breakpointsActiveCount: i32,
    m_ignoreScriptParsedEventsCounter: i32,
    m_originalHeapLimit: usize,
    m_scheduledOOMBreak: bool,
    m_targetContextGroupId: i32,
    m_pausedContextGroupId: i32,
    m_instrumentationPause: bool,
    m_requestedPauseAfterInstrumentation: bool,
    m_continueToLocationBreakpointId: i32,
    m_continueToLocationTargetCallFrames: String16,
    m_continueToLocationStack: Option<std::unique_ptr::UniquePtr<V8StackTraceImpl>>,
    m_cachedStackFrames: HashMap<CachedStackFrameKey, Weak<StackFrame>>,
    m_asyncTaskStacks: HashMap<*mut c_void, Weak<AsyncStackTrace>>,
    m_recurringTasks: HashSet<*mut c_void>,
    m_maxAsyncCallStacks: usize,
    m_maxAsyncCallStackDepth: i32,
    m_maxCallStackSizeToCapture: i32,
    m_currentTasks: Vec<*mut c_void>,
    m_currentAsyncParent: Vec<std::shared_ptr::SharedPtr<AsyncStackTrace>>,
    m_currentExternalParent: Vec<V8StackTraceId>,
    m_asyncParents: HashMap<i32, Weak<AsyncStackTrace>>,
    m_externalParents: Vec<(i32, V8StackTraceId)>,
    m_allAsyncStacks: Vec<std::shared_ptr::SharedPtr<AsyncStackTrace>>,
    m_maxAsyncCallStackDepthMap: HashMap<*mut V8DebuggerAgentImpl, i32>,
    m_maxCallStackSizeToCaptureMap: HashMap<*mut V8RuntimeAgentImpl, i32>,
    m_taskWithScheduledBreak: *mut c_void,
    m_externalAsyncTaskPauseRequested: bool,
    m_taskWithScheduledBreakPauseRequested: bool,
    m_pauseOnNextCallRequested: bool,
    m_pauseOnAsyncCall: bool,
    m_pauseOnExceptionsState: v8::debug::ExceptionBreakState,
    m_storedStackTraces: HashMap<usize, Weak<AsyncStackTrace>>,
    m_lastStackTraceId: usize,
    m_contextGroupIdToDebuggerId: HashMap<i32, V8DebuggerId>,
    m_terminateExecutionCallback: Option<std::unique_ptr::UniquePtr<TerminateExecutionCallback>>,
    m_terminateExecutionCallbackContext: v8::Context,
    m_terminateExecutionReported: bool,
}

impl V8Debugger {
    pub fn new(isolate: *mut v8::Isolate, inspector: *mut V8InspectorImpl) -> V8Debugger {
        V8Debugger {
            m_isolate: isolate,
            m_inspector: inspector,
            m_enableCount: 0,
            m_breakpointsActiveCount: 0,
            m_ignoreScriptParsedEventsCounter: 0,
            m_originalHeapLimit: 0,
            m_scheduledOOMBreak: false,
            m_targetContextGroupId: 0,
            m_pausedContextGroupId: 0,
            m_instrumentationPause: false,
            m_requestedPauseAfterInstrumentation: false,
            m_continueToLocationBreakpointId: kNoBreakpointId,
            m_continueToLocationTargetCallFrames: String16::default(),
            m_continueToLocationStack: None,
            m_cachedStackFrames: HashMap::new(),
            m_asyncTaskStacks: HashMap::new(),
            m_recurringTasks: HashSet::new(),
            m_maxAsyncCallStacks: kMaxAsyncTaskStacks,
            m_maxAsyncCallStackDepth: 0,
            m_maxCallStackSizeToCapture: V8StackTraceImpl::kDefaultMaxCallStackSizeToCapture,
            m_currentTasks: Vec::new(),
            m_currentAsyncParent: Vec::new(),
            m_currentExternalParent: Vec::new(),
            m_asyncParents: HashMap::new(),
            m_externalParents: Vec::new(),
            m_allAsyncStacks: Vec::new(),
            m_maxAsyncCallStackDepthMap: HashMap::new(),
            m_maxCallStackSizeToCaptureMap: HashMap::new(),
            m_taskWithScheduledBreak: ptr::null_mut(),
            m_externalAsyncTaskPauseRequested: false,
            m_taskWithScheduledBreakPauseRequested: false,
            m_pauseOnNextCallRequested: false,
            m_pauseOnAsyncCall: false,
            m_pauseOnExceptionsState: v8::debug::ExceptionBreakState::NoBreakOnException,
            m_storedStackTraces: HashMap::new(),
            m_lastStackTraceId: 0,
            m_contextGroupIdToDebuggerId: HashMap::new(),
            m_terminateExecutionCallback: None,
            m_terminateExecutionCallbackContext: v8::Context {},
            m_terminateExecutionReported: true,
        }
    }

    pub fn enabled(&self) -> bool {
        self.m_enableCount > 0
    }

    pub fn isolate(&self) -> *mut v8::Isolate {
        self.m_isolate
    }

    pub fn setBreakpointsActive(&mut self, active: bool) {
        if !self.enabled() {
            panic!("Should be unreachable");
        }
        self.m_breakpointsActiveCount += if active { 1 } else { -1 };
        assert!(self.m_breakpointsActiveCount >= 0);
    }

    pub fn removeBreakpoint(&mut self, id: v8::debug::BreakpointId) {

    }

    pub fn getPauseOnExceptionsState(&self) -> v8::debug::ExceptionBreakState {
        assert!(self.enabled());
        self.m_pauseOnExceptionsState
    }

    pub fn setPauseOnExceptionsState(&mut self, pauseOnExceptionsState: v8::debug::ExceptionBreakState) {
        assert!(self.enabled());
        if self.m_pauseOnExceptionsState == pauseOnExceptionsState {
            return;
        }
        self.m_pauseOnExceptionsState = pauseOnExceptionsState;
    }

    pub fn canBreakProgram(&self) -> bool {
        true
    }

    pub fn isInInstrumentationPause(&self) -> bool {
        self.m_instrumentationPause
    }

    pub fn breakProgram(&mut self, targetContextGroupId: i32) {
        assert!(self.canBreakProgram());
        if self.isPaused() {
            return;
        }
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
    }

    pub fn interruptAndBreak(&mut self, targetContextGroupId: i32) {
        if self.isPaused() {
            return;
        }
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
    }

    pub fn requestPauseAfterInstrumentation(&mut self) {
        self.m_requestedPauseAfterInstrumentation = true;
    }

    pub fn continueProgram(&mut self, targetContextGroupId: i32, terminateOnResume: bool) {
        if self.m_pausedContextGroupId != targetContextGroupId {
            return;
        }
        if self.isPaused() {
            if self.m_instrumentationPause {
                self.quitMessageLoopIfAgentsFinishedInstrumentation();
            } else if terminateOnResume {
                let targetContextGroupId = 0;
                let context = v8::Context {};
                self.installTerminateExecutionCallbacks(context);
                self.inspector();
            } else {
                self.inspector();
            }
        }
    }

    pub fn breakProgramOnAssert(&mut self, targetContextGroupId: i32) {
        if !self.enabled() {
            return;
        }
        if self.m_pauseOnExceptionsState == v8::debug::ExceptionBreakState::NoBreakOnException {
            return;
        }
        if self.isPaused() {
            return;
        }
        if !self.canBreakProgram() {
            return;
        }
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
    }

    pub fn setPauseOnNextCall(&mut self, pause: bool, targetContextGroupId: i32) {
        if self.isPaused() {
            return;
        }
        assert!(targetContextGroupId != 0);
        if !pause && self.m_targetContextGroupId != 0 && self.m_targetContextGroupId != targetContextGroupId {
            return;
        }
        if pause) {
            let didHaveBreak = self.hasScheduledBreakOnNextFunctionCall();
            self.m_pauseOnNextCallRequested = true;
            if !didHaveBreak {
                self.m_targetContextGroupId = targetContextGroupId;
            }
        } else {
            self.m_pauseOnNextCallRequested = false;
            if !self.hasScheduledBreakOnNextFunctionCall() {
            }
        }
    }

    pub fn stepIntoStatement(&mut self, targetContextGroupId: i32, breakOnAsyncCall: bool) {
        assert!(self.isPaused());
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
        self.m_pauseOnAsyncCall = breakOnAsyncCall;
        self.continueProgram(targetContextGroupId, false);
    }

    pub fn stepOverStatement(&mut self, targetContextGroupId: i32) {
        assert!(self.isPaused());
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
        self.continueProgram(targetContextGroupId, false);
    }

    pub fn stepOutOfFunction(&mut self, targetContextGroupId: i32) {
        assert!(self.isPaused());
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
        self.continueProgram(targetContextGroupId, false);
    }

    pub fn terminateExecution(
        &mut self,
        context: v8::Context,
        callback: std::unique_ptr::UniquePtr<TerminateExecutionCallback>,
    ) {
        if !self.m_terminateExecutionReported {
            return;
        }
        self.m_terminateExecutionCallback = Some(callback);
        self.installTerminateExecutionCallbacks(context);
    }

    fn installTerminateExecutionCallbacks(&mut self, context: v8::Context) {
        self.m_terminateExecutionReported = false;
    }

    fn reportTermination(&mut self) {
        if self.m_terminateExecutionReported {
            return;
        }
        if let Some(callback) = &mut self.m_terminateExecutionCallback {
            self.m_terminateExecutionCallback = None;
        }
        self.m_terminateExecutionReported = true;
    }

    pub fn continueToLocation(
        &mut self,
        targetContextGroupId: i32,
        script: &mut V8DebuggerScript,
        location: std::unique_ptr::UniquePtr<Debugger::Location>,
        targetCallFrames: String16,
    ) -> Response {
        assert!(self.isPaused());
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
        self.clearContinueToLocation();
        Response::Success()
    }

    pub fn restartFrame(&mut self, targetContextGroupId: i32, callFrameOrdinal: i32) -> bool {
        assert!(self.isPaused());
        assert!(targetContextGroupId != 0);
        self.m_targetContextGroupId = targetContextGroupId;
        self.continueProgram(targetContextGroupId, false);
        true
    }

    pub fn getCompiledScripts(
        &mut self,
        contextGroupId: i32,
        agent: *mut V8DebuggerAgentImpl,
    ) -> Vec<std::unique_ptr::UniquePtr<V8DebuggerScript>> {
        let mut result = Vec::new();
        return result;
    }

    pub fn enable(&mut self) {
        if self.m_enableCount > 0 {
            self.m_enableCount += 1;
            return;
        }
        self.m_enableCount = 1;
        self.m_pauseOnExceptionsState = v8::debug::ExceptionBreakState::NoBreakOnException;
    }

    pub fn disable(&mut self) {
        if self.isPaused() {
            let hasAgentAcceptsPause = false;
            if self.m_instrumentationPause {
                self.quitMessageLoopIfAgentsFinishedInstrumentation();
            } else {
                self.inspector();
            }
        }
        self.m_enableCount -= 1;
        if self.m_enableCount > 0 {
            return;
        }
        self.clearContinueToLocation();
        self.m_taskWithScheduledBreak = ptr::null_mut();
        self.m_externalAsyncTaskPauseRequested = false;
        self.m_taskWithScheduledBreakPauseRequested = false;
        self.m_pauseOnNextCallRequested = false;
        self.m_pauseOnAsyncCall = false;
    }

    pub fn isPaused(&self) -> bool {
        self.m_pausedContextGroupId != 0
    }

    pub fn isPausedInContextGroup(&self, contextGroupId: i32) -> bool {
        self.isPaused() && self.m_pausedContextGroupId == contextGroupId
    }

    pub fn maxAsyncCallChainDepth(&self) -> i32 {
        self.m_maxAsyncCallStackDepth
    }

    pub fn setAsyncCallStackDepth(&mut self, agent: *mut V8DebuggerAgentImpl, depth: i32) {
        if depth <= 0 {
            self.m_maxAsyncCallStackDepthMap.remove(&agent);
        } else {
            self.m_maxAsyncCallStackDepthMap.insert(agent, depth);
        }

        let mut maxAsyncCallStackDepth = 0;
        for pair in self.m_maxAsyncCallStackDepthMap.iter() {
            if *pair.1 > maxAsyncCallStackDepth {
                maxAsyncCallStackDepth = *pair.1;
            }
        }

        if self.m_maxAsyncCallStackDepth == maxAsyncCallStackDepth {
            return;
        }

        self.m_maxAsyncCallStackDepth = maxAsyncCallStackDepth;
        if maxAsyncCallStackDepth == 0 {
            self.allAsyncTasksCanceled();
        }
    }

    pub fn maxCallStackSizeToCapture(&self) -> i32 {
        self.m_maxCallStackSizeToCapture
    }

    pub fn setMaxCallStackSizeToCapture(&mut self, agent: *mut V8RuntimeAgentImpl, size: i32) {
        if size < 0 {
            self.m_maxCallStackSizeToCaptureMap.remove(&agent);
        } else {
            self.m_maxCallStackSizeToCaptureMap.insert(agent, size);
        }

        if self.m_maxCallStackSizeToCaptureMap.is_empty() {
            self.m_maxCallStackSizeToCapture = V8StackTraceImpl::kDefaultMaxCallStackSizeToCapture;
        } else {
            self.m_maxCallStackSizeToCapture = 0;
            for pair in self.m_maxCallStackSizeToCaptureMap.iter() {
                if self.m_maxCallStackSizeToCapture < *pair.1 {
                    self.m_maxCallStackSizeToCapture = *pair.1;
                }
            }
        }
    }

    pub fn currentAsyncParent(&self) -> std::shared_ptr::SharedPtr<AsyncStackTrace> {
        if self.m_currentAsyncParent.is_empty() {
            std::shared_ptr::SharedPtr::null()
        } else {
            self.m_currentAsyncParent.last().cloned().unwrap()
        }
    }

    pub fn currentExternalParent(&self) -> V8StackTraceId {
        if self.m_currentExternalParent.is_empty() {
            V8StackTraceId::default()
        } else {
            self.m_currentExternalParent.last().cloned().unwrap()
        }
    }

    pub fn symbolize(&mut self, v8Frame: v8::StackFrame) -> std::shared_ptr::SharedPtr<StackFrame> {
        let scriptId = 0;
        let lineNumber = 0;
        let columnNumber = 0;
        let key = CachedStackFrameKey::new(scriptId, lineNumber, columnNumber);
        let functionName = String16::default();

        let stackFrame = std::shared_ptr::SharedPtr::null();

        std::shared_ptr::SharedPtr::null()
    }

    pub fn createStackTrace(
        &mut self,
        v8StackTrace: v8::StackFrame,
    ) -> std::unique_ptr::UniquePtr<V8StackTraceImpl> {
        V8StackTraceImpl::create(self, v8StackTrace, V8StackTraceImpl::kDefaultMaxCallStackSizeToCapture)
    }

    pub fn captureStackTrace(&mut self, fullStack: bool) -> std::unique_ptr::UniquePtr<V8StackTraceImpl> {
        let contextGroupId = self.currentContextGroupId();
        if contextGroupId == 0 {
            return std::unique_ptr::UniquePtr::null();
        }

        let mut stackSize = 1;
        stackSize = V8StackTraceImpl::kDefaultMaxCallStackSizeToCapture;
        V8StackTraceImpl::capture(self, stackSize)
    }

    pub fn internalProperties(
        &mut self,
        context: v8::Context,
        value: v8::Value,
    ) -> v8::MaybeLocal<v8::Array> {
        let properties = v8::Array {};
        v8::MaybeLocal { value: Some(properties)}
    }

    pub fn queryObjects(
        &mut self,
        context: v8::Context,
        prototype: v8::Object,
    ) -> v8::Array {
        let mut v8_objects: Vec<v8::Object> = Vec::new();
        let predicate = MatchPrototypePredicate::new(
            unsafe { &*self.m_inspector },
            context,
            prototype,
        );

        let resultArray = v8::Array {};
        return resultArray;
    }

    pub fn asyncTaskScheduled(&mut self, taskName: StringView, task: *mut c_void, recurring: bool) {
        self.asyncTaskScheduledForStack(taskName, task, recurring, false);
        self.asyncTaskCandidateForStepping(task);
    }

    pub fn asyncTaskCanceled(&mut self, task: *mut c_void) {
        self.asyncTaskCanceledForStack(task);
        self.asyncTaskCanceledForStepping(task);
    }

    pub fn asyncTaskStarted(&mut self, task: *mut c_void) {
        self.asyncTaskStartedForStack(task);
        self.asyncTaskStartedForStepping(task);
    }

    pub fn asyncTaskFinished(&mut self, task: *mut c_void) {
        self.asyncTaskFinishedForStepping(task);
        self.asyncTaskFinishedForStack(task);
    }

    pub fn allAsyncTasksCanceled(&mut self) {
        self.m_asyncTaskStacks.clear();
        self.m_recurringTasks.clear();
        self.m_currentAsyncParent.clear();
        self.m_currentExternalParent.clear();
        self.m_currentTasks.clear();
        self.m_currentAsyncParent.clear();
        self.m_externalParents.clear();
        self.m_allAsyncStacks.clear();
    }

    pub fn storeCurrentStackTrace(&mut self, description: StringView) -> V8StackTraceId {
        if self.m_maxAsyncCallStackDepth == 0 {
            return V8StackTraceId::default();
        }

        let contextGroupId = self.currentContextGroupId();
        if contextGroupId == 0 {
            return V8StackTraceId::default();
        }

        let asyncStack = AsyncStackTrace::capture(self, string_view_to_string16(description), false);

        let id = AsyncStackTrace::store(self, asyncStack);

        self.m_allAsyncStacks.push(asyncStack);
        self.collectOldAsyncStacksIfNeeded();

        let shouldPause = self.m_pauseOnAsyncCall && contextGroupId == self.m_targetContextGroupId;
        if shouldPause {
            self.m_pauseOnAsyncCall = false;
        }
        V8StackTraceId::new(id, self.debuggerIdFor(contextGroupId).pair(), shouldPause)
    }

    pub fn externalAsyncTaskStarted(&mut self, parent: &V8StackTraceId) {
        if self.m_maxAsyncCallStackDepth == 0 || parent.IsInvalid() {
            return;
        }
        self.m_currentExternalParent.push(parent.clone());
        self.m_currentAsyncParent.push(std::shared_ptr::SharedPtr::null());
        self.m_currentTasks.push(parent.id as *mut c_void);

        if !parent.should_pause {
            return;
        }
    }

    pub fn externalAsyncTaskFinished(&mut self, parent: &V8StackTraceId) {
        if self.m_maxAsyncCallStackDepth == 0 || self.m_currentExternalParent.is_empty() {
            return;
        }
        self.m_currentExternalParent.pop();
        self.m_currentAsyncParent.pop();
        self.m_currentTasks.pop();
    }

    pub fn muteScriptParsedEvents(&mut self) {
        self.m_ignoreScriptParsedEventsCounter += 1;
    }

    pub fn unmuteScriptParsedEvents(&mut self) {
        self.m_ignoreScriptParsedEventsCounter -= 1;
        assert!(self.m_ignoreScriptParsedEventsCounter >= 0);
    }

    pub fn inspector(&self) -> *mut V8InspectorImpl {
        self.m_inspector
    }

    pub fn setMaxAsyncTaskStacksForTest(&mut self, limit: i32) {}

    pub fn dumpAsyncTaskStacksStateForTest(&mut self) {}

    pub fn asyncParentFor(
        &self,
        stackTraceId: i32,
        asyncParent: &mut std::shared_ptr::SharedPtr<AsyncStackTrace>,
        externalParent: &mut V8
