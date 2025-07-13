// Converted from V8 C++ source files:
// Header: v8-inspector-impl.h
// Implementation: v8-inspector-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_inspector_impl {
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use crate::inspector::string_util::StringView;
use crate::compiler::js_call_reducer::{ClientTrustLevel, SessionPauseState};
use crate::inspector::v8_inspector_session_impl::V8InspectorSessionImpl;
use crate::inspector::v8_debugger_id::V8DebuggerId;
use v8::HandleScope;

pub struct V8InspectorClient {}
pub struct String16 {}
pub struct V8StackTrace {}
pub struct V8ContextInfo {}

pub mod v8 {
    pub struct Isolate {}
    pub struct Context {}
    pub struct Value {}
    pub struct String {}
    pub struct UnboundScript {}
    pub struct Script {}
    pub struct Local<'a, T> {}
    pub struct MaybeLocal<'a, T> {}
    pub struct StackTrace {}
    pub struct Name {}

    impl<'a, T> Local<'a, T> {
        pub fn new(_: &'a Isolate, _: T) -> Self {
            Local{}
        }
    }
    impl<'a, T> MaybeLocal<'a, T> {
        pub fn to_local(&self) -> Local<'a, T> {
            Local{}
        }
    }

    pub mod debug {
        pub struct EphemeronTable {}
    }

    pub struct TryCatch {}
    impl TryCatch {
      pub fn new(_isolate: &Isolate) -> Self {
        TryCatch{}
      }
    }

    pub struct ContextScope<'a> {}
    impl<'a> ContextScope<'a> {
        pub fn new(_context: Local<'a, Context>) -> Self {
            ContextScope{}
        }
    }

    pub struct HandleScope<'a> {}
    impl<'a> HandleScope<'a> {
      pub fn new(_isolate: &Isolate) -> Self {
        HandleScope{}
      }
    }
}

pub mod debug {
    pub fn SetInspector(_isolate: *mut v8::Isolate, _inspector: *mut V8InspectorImpl) {}
    pub fn SetConsoleDelegate(_isolate: *mut v8::Isolate, _console: *mut V8Console) {}
    pub fn SetIsolateId(_isolate: *mut v8::Isolate, _id: i64) {}
}

pub trait Channel {
}

pub struct InspectedContext {
    inspector: *mut V8InspectorImpl,
    context_id: i32,
}
impl InspectedContext {
  pub fn contextId(_context: v8::Local<v8::Context>) -> i32 {
    0
  }
  pub fn context(&self) -> v8::MaybeLocal<'static, v8::Context> {
    v8::MaybeLocal{}
  }
  pub fn uniqueId(&self) -> UniqueId {
    UniqueId{}
  }
  pub fn context_id(&self) -> i32 {
    self.context_id
  }
}
pub struct UniqueId {}
impl UniqueId {
  pub fn pair(&self) -> (i64, i64) {
    (0,0)
  }
}
pub struct V8ConsoleMessageStorage {
    inspector: *mut V8InspectorImpl,
}
impl V8ConsoleMessageStorage {
  pub fn new(_inspector: *mut V8InspectorImpl) -> Self {
    V8ConsoleMessageStorage{inspector: _inspector}
  }
  pub fn contextDestroyed(&self, _context_id: i32) {}
  pub fn addMessage(&mut self, _console_message: std::unique_ptr<V8ConsoleMessage>) {}
}
pub struct V8ConsoleMessage {}
impl V8ConsoleMessage {
  pub fn createForException(
    _current_time_ms: f64,
    _detailed_message: String16,
    _url: String16,
    _line_number: u32,
    _column_number: u32,
    _stack_trace_impl: std::unique_ptr<V8StackTraceImpl>,
    _script_id: i32,
    _isolate: *mut v8::Isolate,
    _message: String16,
    _context_id: i32,
    _exception: v8::Local<v8::Value>,
    _exception_id: u32
  ) -> std::unique_ptr<V8ConsoleMessage> {
    std::unique_ptr::new(V8ConsoleMessage{})
  }
  pub fn createForRevokedException(
    _current_time_ms: f64,
    _message: String16,
    _exception_id: u32,
  ) -> std::unique_ptr<V8ConsoleMessage> {
    std::unique_ptr::new(V8ConsoleMessage{})
  }
}
pub struct V8StackTraceImpl {}
pub struct V8Console {}

pub trait V8Inspector {
    fn connect(&mut self, context_group_id: i32, channel: &mut dyn Channel, state: StringView, client_trust_level: ClientTrustLevel, pause_state: SessionPauseState) -> std::unique_ptr<V8InspectorSessionImpl>;
    fn context_created(&mut self, info: &V8ContextInfo);
    fn context_destroyed(&mut self, context: v8::Local<v8::Context>);
    fn context_by_id(&mut self, context_id: i32) -> v8::MaybeLocal<'static, v8::Context>;
    fn unique_debugger_id(&mut self, context_id: i32) -> V8DebuggerId;
    fn isolate_id(&mut self) -> u64;
    fn context_collected(&mut self, context_group_id: i32, context_id: i32);
    fn reset_context_group(&mut self, context_group_id: i32);
    fn idle_started(&mut self);
    fn idle_finished(&mut self);
    fn exception_thrown(&mut self, context: v8::Local<v8::Context>, message: StringView, exception: v8::Local<v8::Value>, detailed_message: StringView, url: StringView, line_number: u32, column_number: u32, stack_trace: std::unique_ptr<V8StackTrace>, script_id: i32) -> u32;
    fn exception_revoked(&mut self, context: v8::Local<v8::Context>, exception_id: u32, message: StringView);
    fn create_stack_trace(&mut self, stack_trace: v8::Local<v8::StackTrace>) -> std::unique_ptr<V8StackTrace>;
    fn capture_stack_trace(&mut self, full_stack: bool) -> std::unique_ptr<V8StackTrace>;
    fn async_task_scheduled(&mut self, task_name: StringView, task: *mut std::ffi::c_void, recurring: bool);
    fn async_task_canceled(&mut self, task: *mut std::ffi::c_void);
    fn async_task_started(&mut self, task: *mut std::ffi::c_void);
    fn async_task_finished(&mut self, task: *mut std::ffi::c_void);
    fn all_async_tasks_canceled(&mut self);
    fn store_current_stack_trace(&mut self, description: StringView) -> V8StackTraceId;
    fn external_async_task_started(&mut self, parent: &V8StackTraceId);
    fn external_async_task_finished(&mut self, parent: &V8StackTraceId);
    fn associate_exception_data(&mut self, _context: v8::Local<v8::Context>, _exception: v8::Local<v8::Value>, _key: v8::Local<v8::Name>, _value: v8::Local<v8::Value>) -> bool;
}

pub struct V8Debugger {
    isolate: *mut v8::Isolate,
    inspector: *mut V8InspectorImpl,
}
impl V8Debugger {
  pub fn debuggerIdFor(&self, _context_group_id: i32) -> internal::V8DebuggerId {
    internal::V8DebuggerId{}
  }
  pub fn createStackTrace(&self, _stack_trace: v8::Local<v8::StackTrace>) -> std::unique_ptr<V8StackTrace> {
    std::unique_ptr::new(V8StackTrace{})
  }
  pub fn captureStackTrace(&self, _full_stack: bool) -> std::unique_ptr<V8StackTrace> {
    std::unique_ptr::new(V8StackTrace{})
  }
  pub fn asyncTaskScheduled(&self, _task_name: StringView, _task: *mut std::ffi::c_void, _recurring: bool) {}
  pub fn asyncTaskCanceled(&self, _task: *mut std::ffi::c_void) {}
  pub fn asyncTaskStarted(&self, _task: *mut std::ffi::c_void) {}
  pub fn asyncTaskFinished(&self, _task: *mut std::ffi::c_void) {}
  pub fn allAsyncTasksCanceled(&self) {}
  pub fn reportTermination(&self) {}
  pub fn externalAsyncTaskStarted(&self, _parent: &V8StackTraceId) {}
  pub fn externalAsyncTaskFinished(&self, _parent: &V8StackTraceId) {}
}

pub struct PromiseHandlerTracker {}

pub struct V8InspectorImpl {
    isolate: *mut v8::Isolate,
    client: *mut V8InspectorClient,
    debugger: std::unique_ptr<V8Debugger>,
    regex_context: RefCell<v8::Global<v8::Context>>,
    exception_meta_data_context: RefCell<v8::Global<v8::Context>>,
    exception_meta_data: RefCell<v8::Global<v8::debug::EphemeronTable>>,
    last_exception_id: u32,
    last_context_id: i32,
    last_session_id: i32,
    mute_exceptions_map: RefCell<HashMap<i32, i32>>,
    contexts: RefCell<HashMap<i32, std::unique_ptr<HashMap<i32, std::unique_ptr<InspectedContext>>>>>,
    sessions: RefCell<HashMap<i32, HashMap<i32, *mut V8InspectorSessionImpl>>>,
    debugger_barriers: RefCell<HashMap<i32, Weak<V8DebuggerBarrier>>>,
    console_storage_map: RefCell<HashMap<i32, std::unique_ptr<V8ConsoleMessageStorage>>>,
    context_id_to_group_id_map: RefCell<HashMap<i32, i32>>,
    unique_id_to_context_id: RefCell<HashMap<(i64, i64), i32>>,
    console: RefCell<std::unique_ptr<V8Console>>,
    promise_handler_tracker: PromiseHandlerTracker,
}

impl V8InspectorImpl {
    pub fn new(isolate: *mut v8::Isolate, client: *mut V8InspectorClient) -> V8InspectorImpl {
        let mut inspector = V8InspectorImpl {
            isolate,
            client,
            debugger: std::unique_ptr::new(V8Debugger { isolate, inspector: std::ptr::null_mut() }),
            regex_context: RefCell::new(v8::Global{}),
            exception_meta_data_context: RefCell::new(v8::Global{}),
            exception_meta_data: RefCell::new(v8::Global{}),
            last_exception_id: 0,
            last_context_id: 0,
            last_session_id: 0,
            mute_exceptions_map: RefCell::new(HashMap::new()),
            contexts: RefCell::new(HashMap::new()),
            sessions: RefCell::new(HashMap::new()),
            debugger_barriers: RefCell::new(HashMap::new()),
            console_storage_map: RefCell::new(HashMap::new()),
            context_id_to_group_id_map: RefCell::new(HashMap::new()),
            unique_id_to_context_id: RefCell::new(HashMap::new()),
            console: RefCell::new(std::unique_ptr::new(V8Console{})),
            promise_handler_tracker: PromiseHandlerTracker {},
        };
        inspector.debugger = std::unique_ptr::new(V8Debugger { isolate, inspector: &mut inspector });

        unsafe {
            debug::SetInspector(isolate, &mut inspector);
            debug::SetConsoleDelegate(isolate, inspector.console().as_mut().unwrap());
            debug::SetIsolateId(isolate, inspector.generateUniqueId());
        }
        inspector
    }

    pub fn isolate(&self) -> *mut v8::Isolate {
        self.isolate
    }

    pub fn client(&mut self) -> *mut V8InspectorClient {
        self.client
    }

    pub fn debugger(&mut self) -> &mut V8Debugger {
        self.debugger.as_mut().unwrap()
    }

    pub fn promiseHandlerTracker(&mut self) -> &mut PromiseHandlerTracker {
        &mut self.promise_handler_tracker
    }

    pub fn contextGroupId_local(&self, context: v8::Local<v8::Context>) -> i32 {
        self.contextGroupId(InspectedContext::contextId(context))
    }

    pub fn contextGroupId(&self, context_id: i32) -> i32 {
        self.context_id_to_group_id_map.borrow().get(&context_id).copied().unwrap_or(0)
    }

    pub fn resolveUniqueContextId(&self, unique_id: internal::V8DebuggerId) -> i32 {
        self.unique_id_to_context_id.borrow().get(&unique_id.pair()).copied().unwrap_or(0)
    }

    pub fn compileAndRunInternalScript(&mut self, context: v8::Local<v8::Context>, source: v8::Local<v8::String>) -> v8::MaybeLocal<v8::Value> {
        v8::MaybeLocal{}
    }

    pub fn compileScript(&mut self, context: v8::Local<v8::Context>, code: String16, file_name: String16) -> v8::MaybeLocal<v8::Script> {
        v8::MaybeLocal{}
    }

    pub fn muteExceptions(&self, context_group_id: i32) {
        let mut map = self.mute_exceptions_map.borrow_mut();
        *map.entry(context_group_id).or_insert(0) += 1;
    }

    pub fn unmuteExceptions(&self, context_group_id: i32) {
        let mut map = self.mute_exceptions_map.borrow_mut();
        if let Some(entry) = map.get_mut(&context_group_id) {
            *entry -= 1;
        }
    }

    pub fn ensureConsoleMessageStorage(&self, context_group_id: i32) -> *mut V8ConsoleMessageStorage {
        let mut storage_map = self.console_storage_map.borrow_mut();
        let entry = storage_map.entry(context_group_id).or_insert_with(|| {
            std::unique_ptr::new(V8ConsoleMessageStorage { inspector: self })
        });
        entry.as_mut().unwrap()
    }

    pub fn hasConsoleMessageStorage(&self, context_group_id: i32) -> bool {
        self.console_storage_map.borrow().contains_key(&context_group_id)
    }

    pub fn createStackTrace(&mut self, stack_trace: v8::Local<v8::StackTrace>) -> std::unique_ptr<V8StackTrace> {
        self.debugger().createStackTrace(stack_trace)
    }
    pub fn nextExceptionId(&mut self) -> u32 {
      self.last_exception_id += 1;
      self.last_exception_id
    }

    pub fn contextCollected(&self, group_id: i32, context_id: i32) {
        self.context_id_to_group_id_map.borrow_mut().remove(&context_id);

        if let Some(storage_it) = self.console_storage_map.borrow().get(&group_id) {
            storage_it.contextDestroyed(context_id);
        }

        let inspected_context = self.getContext(group_id, context_id);
        if inspected_context.is_null() {
            return;
        }

        self.forEachSession(group_id, &|session: *mut V8InspectorSessionImpl| {
            unsafe {
                (*session).runtimeAgent().reportExecutionContextDestroyed(&(*inspected_context));
            }
        });
        self.discardInspectedContext(group_id, context_id);
    }

    pub fn resetContextGroup(&self, context_group_id: i32) {
        self.console_storage_map.borrow_mut().remove(&context_group_id);
        self.mute_exceptions_map.borrow_mut().remove(&context_group_id);
        if let Some(contexts_it) = self.contexts.borrow().get(&context_group_id) {
            for map_entry in contexts_it.values() {
                self.unique_id_to_context_id.borrow_mut().remove(&map_entry.uniqueId().pair());
            }
        }

        self.forEachSession(context_group_id, &|session: *mut V8InspectorSessionImpl| {
            unsafe {
                (*session).reset();
            }
        });
    }

    pub fn idleStarted(&self) {
      unsafe {
        //m_isolate->SetIdle(true);
      }
    }

    pub fn idleFinished(&self) {
      unsafe {
        //m_isolate->SetIdle(false);
      }
    }

    pub fn exceptionThrown(&mut self, context: v8::Local<v8::Context>, message: StringView, exception: v8::Local<v8::Value>, detailed_message: StringView, url: StringView, line_number: u32, column_number: u32, stack_trace: std::unique_ptr<V8StackTrace>, script_id: i32) -> u32 {
        let group_id = self.contextGroupId_local(context);
        if group_id == 0 || self.mute_exceptions_map.borrow().contains_key(&group_id) {
            return 0;
        }

        let stack_trace_impl = stack_trace;
        let exception_id = self.nextExceptionId();
        let console_message = V8ConsoleMessage::createForException(
          0.0, //m_client->currentTimeMS(),
          detailed_message.into(),
          url.into(),
          line_number,
          column_number,
          stack_trace_impl,
          script_id,
          self.isolate,
          message.into(),
          InspectedContext::contextId(context),
          exception,
          exception_id,
        );

        unsafe {
          (*self.ensureConsoleMessageStorage(group_id)).addMessage(console_message);
        }
        exception_id
    }

    pub fn exceptionRevoked(&mut self, context: v8::Local<v8::Context>, exception_id: u32, message: StringView) {
        let group_id = self.contextGroupId_local(context);
        if group_id == 0 {
            return;
        }

        let console_message = V8ConsoleMessage::createForRevokedException(
          0.0, //m_client->currentTimeMS(),
          message.into(),
          exception_id,
        );

        unsafe {
            (*self.ensureConsoleMessageStorage(group_id)).addMessage(console_message);
        }
    }

    pub fn captureStackTrace(&mut self, full_stack: bool) -> std::unique_ptr<V8StackTrace> {
        self.debugger().captureStackTrace(full_stack)
    }

    pub fn storeCurrentStackTrace(&mut self, description: StringView) -> V8StackTraceId {
        self.debugger().storeCurrentStackTrace(description)
    }

    pub fn externalAsyncTaskStarted(&mut self, parent: &V8StackTraceId) {
        self.debugger().externalAsyncTaskStarted(parent);
    }

    pub fn externalAsyncTaskFinished(&mut self, parent: &V8StackTraceId) {
        self.debugger().externalAsyncTaskFinished(parent);
    }

    pub fn asyncTaskScheduled(&mut self, task_name: StringView, task: *mut std::ffi::c_void, recurring: bool) {
        if task.is_null() {
            return;
        }
        self.debugger().asyncTaskScheduled(task_name, task, recurring);
    }

    pub fn asyncTaskCanceled(&mut self, task: *mut std::ffi::c_void) {
        if task.is_null() {
            return;
        }
        self.debugger().asyncTaskCanceled(task);
    }

    pub fn asyncTaskStarted(&mut self, task: *mut std::ffi::c_void) {
        if task.is_null() {
            return;
        }
        self.debugger().asyncTaskStarted(task);
    }

    pub fn asyncTaskFinished(&mut self, task: *mut std::ffi::c_void) {
        if task.is_null() {
            return;
        }
        self.debugger().asyncTaskFinished(task);
    }

    pub fn allAsyncTasksCanceled(&mut self) {
        self.debugger().allAsyncTasksCanceled();
    }
    pub fn regexContext(&self) -> v8::MaybeLocal<v8::Context> {
        if self.regex_context.borrow().IsEmpty() {
            let mut regex_context = v8::Context::New(self.isolate);
            self.regex_context.borrow_mut().Reset(self.isolate, regex_context);
            if self.regex_context.borrow().IsEmpty() {
                //DCHECK(m_isolate->IsExecutionTerminating());
                return v8::MaybeLocal{};
            }
        }
        self.regex_context.borrow().Get(self.isolate)
    }
    pub fn exceptionMetaDataContext(&self) -> v8::MaybeLocal<v8::Context> {
        if self.exception_meta_data_context.borrow().IsEmpty() {
            let mut context = v8::Context::New(self.isolate);
            self.exception_meta_data_context.borrow_mut().Reset(self.isolate, context);
            if self.exception_meta_data_context.borrow().IsEmpty() {
                //DCHECK(m_isolate->IsExecutionTerminating());
                return v8::MaybeLocal{};
            }
        }
        self.exception_meta_data_context.borrow().Get(self.isolate)
    }
    pub fn discardInspectedContext(&self, context_group_id: i32, context_id: i32) {
        let context = self.getContext(context_group_id, context_id);
        if context.is_null() {
            return;
        }
        self.unique_id_to_context_id.borrow_mut().remove(&(*unsafe{&*context}.uniqueId().pair()));
        self.contexts.borrow_mut().get_mut(&context_group_id).unwrap().remove(&context_id);
        if self.contexts.borrow().get(&context_group_id).unwrap().is_empty() {
            self.contexts.borrow_mut().remove(&context_group_id);
        }
    }
    pub fn sessionById(&self, context_group_id: i32, session_id: i32) -> *mut V8InspectorSessionImpl {
        if let Some(it) = self.sessions.borrow().get(&context_group_id) {
            if let Some(it2) = it.get(&session_id) {
                return *it2;
            }
        }
        std::ptr::null_mut()
    }
    pub fn console(&self) -> std::unique_ptr<V8Console> {
        let mut console = self.console.borrow_mut();
        if console.is_null() {
            *console = std::unique_ptr::new(V8Console{});
        }
        console.clone()
    }
    pub fn forEachContext(
        &self,
        context_group_id: i32,
        callback: &dyn Fn(*mut InspectedContext),
    ) {
        let it = self.contexts.borrow().get(&context_group_id).map(|x| x.clone());
        if it.is_none() {
            return;
        }
        let it = it.unwrap();

        let mut ids = Vec::new();
        ids.reserve(it.len());
        for context_it in it.iter() {
            ids.push(*context_it.0);
        }

        for context_id in ids.iter() {
            let contexts_it = self.contexts.borrow().get(&context_group_id).map(|x| x.clone());
            if contexts_it.is_none() {
                continue;
            }
            let contexts_it = contexts_it.unwrap();

            if let Some(context_it) = contexts_it.get(context_id) {
                callback(context_it.as_ref().unwrap());
            }
        }
    }
    pub fn forEachSession(
        &self,
        context_group_id: i32,
        callback: &dyn Fn(*mut V8InspectorSessionImpl),
    ) {
        let it = self.sessions.borrow().get(&context_group_id).map(|x| x.clone());
        if it.is_none() {
            return;
        }
        let it = it.unwrap();

        let mut ids = Vec::new();
        ids.reserve(it.len());
        for session_it in it.iter() {
            ids.push(*session_it.0);
        }

        for session_id in ids.iter() {
            let sessions_it = self.sessions.borrow().get(&context_group_id).map(|x| x.clone());
            if sessions_it.is_none() {
                continue;
            }
            let sessions_it = sessions_it.unwrap();

            if let Some(session_it) = sessions_it.get(session_id) {
                callback(*session_it);
            }
        }
    }
    pub fn generateUniqueId(&self) -> i64 {
      0
    }

    pub fn getAssociatedExceptionData(
      &self,
      _exception: v8::Local<v8::Value>,
    ) -> v8::MaybeLocal<v8::Object> {
      v8::MaybeLocal{}
    }
    pub fn getAssociatedExceptionDataForProtocol(
      &self,
      _exception: v8::Local<v8::Value>
    ) -> std::unique_ptr<protocol::DictionaryValue> {
      std::unique_ptr::new(protocol::DictionaryValue{})
    }

    pub fn getContext(&self, group_id: i32, context_id: i32) -> *mut InspectedContext {
        if group_id == 0 || context_id == 0 {
            return std::ptr::null_mut();
        }

        let context_group_it = self.contexts.borrow().get(&group_id).map(|x| x.clone());
        if context_group_it.is_none() {
            return std::ptr::null_mut();
        }
        let context_group_it = context_group_it.unwrap();

        let context_it = context_group_it.get(&context_id).map(|x| x.clone());
        if context_it.is_none() {
            return std::ptr::null_mut();
        }
        let context_it = context_it.unwrap();

        context_it.as_ref().unwrap()
    }
    pub fn getContext_int(&self, context_id: i32) -> *mut InspectedContext {
      self.getContext(self.contextGroupId(context_id), context_id)
    }
}

impl Drop for V8InspectorImpl {
    fn drop(&mut self) {
        unsafe {
            debug::SetInspector(self.isolate, std::ptr::null_mut());
            debug::SetConsoleDelegate(self.isolate, std::ptr::null_mut());
        }
    }
}

impl V8Inspector for V8InspectorImpl {
    fn connect(&mut self, context_group_id: i32, channel: &mut dyn Channel, state: StringView, client_trust_level: ClientTrustLevel, pause_state: SessionPauseState) -> std::unique_ptr<V8InspectorSessionImpl> {
        self.last_session_id += 1;
        let session_id = self.last_session_id;

        let debugger_barrier = if pause_state == SessionPauseState::kWaitingForDebugger {
            let mut debugger_barriers = self.debugger_barriers.borrow_mut();
            if let Some(it) = debugger_barriers.get(&context_group_id) {
                it.upgrade()
            } else {
                let debugger_barrier = Arc::new(V8DebuggerBarrier::new(self.client, context_group_id));
                debugger_barriers.insert(context_group_id, Arc::downgrade(&debugger_barrier));
                Some(debugger_barrier)
            }
        } else {
            None
        };

        let mut session = V8InspectorSessionImpl::create(
            self,
            context_group_id,
            session_id,
            channel,
            state,
            client_trust_level,
            debugger_barrier,
        );

        self.sessions.borrow_mut()
            .entry(context_group_id)
            .or_insert_with(HashMap::new)
            .insert(session_id, session.as_mut().unwrap());

        session
    }

    fn context_created(&mut self, info: &V8ContextInfo) {
        self.last_context_id += 1;
        let context_id = self.last_context_id;

        let context = Box::into_raw(Box::new(InspectedContext { inspector: self, context_id: context_id }));
        self.context_id_to_group_id_map.borrow_mut().insert(context_id, info.contextGroupId);

        let unique_id = unsafe{&*context}.uniqueId();
        //DCHECK(m_uniqueIdToContextId.find(context->uniqueId().pair()) ==
        //       m_uniqueIdToContextId.end());
        self.unique_id_to_context_id.borrow_mut().insert(unique_id.pair(), context_id);

        let mut contexts = self.contexts.borrow_mut();
        let context_by_id = contexts.entry(info.contextGroupId).or_insert_with(|| {
            std::unique_ptr::new(HashMap::new())
        });

        if !context_by_id.contains_key(&context_id) {
          context_by_id.insert(context_id, std::unique_ptr::new(unsafe{&*context}));
        }

        let self_ptr = self as *mut Self;
        self.forEachSession(
            info.contextGroupId,
            &|session: *mut V8InspectorSessionImpl| {
                unsafe {
                    (*session).runtimeAgent().addBindings(&(*context));
                    (*session).runtimeAgent().reportExecutionContextCreated(&(*context));
                }
            },
        );
    }

    fn context_destroyed(&mut self, context: v8::Local<v8::Context>) {
        let context_id = InspectedContext::contextId(context);
        let group_id = self.contextGroupId_local(context);
        self.contextCollected(group_id, context_id);
    }

    fn context_by_id(&mut self, context_id: i32) -> v8::MaybeLocal<'static, v8::Context> {
        let context = self.getContext_int(context_id);
        if context.is_null() {
          return v8::MaybeLocal{};
        }
        unsafe {
          (*context).context()
        }
    }

    fn unique_debugger_id(&mut self, context_id: i32) -> V8DebuggerId {
        let context = self.getContext_int(context_id);
        let mut unique_id = internal::V8DebuggerId{};
        if !context.is_null() {
          unique_id = self.debugger().debuggerIdFor(self.contextGroupId(unsafe{&*context}.context_id()));
        }
        unique_id.toV8DebuggerId()
    }

    fn isolate_id(&mut self) -> u64 {
        0
    }
    fn associate_exception_data(&mut self, _context: v8::Local<v8::Context>, _exception: v8::Local<v8::Value>, _key: v8::Local<v8::Name>, _value: v8::Local<v8::Value>) -> bool {
      false
    }
}

pub mod internal {
  pub struct V8DebuggerId {}
  impl V8DebuggerId {
    pub fn pair(&self) -> (i64, i64) {
      (0,0)
    }
    pub fn toV8DebuggerId(&self) -> super::V8DebuggerId {
      super::V8DebuggerId{}
    }
  }
}

pub struct V8DebuggerBarrier {
    client: *mut V8InspectorClient,
    context_group_id: i32,
}

impl V8DebuggerBarrier {
    pub fn new(client: *mut V8InspectorClient, context_group_id: i32) -> Self {
        V8DebuggerBarrier {
            client,
            context_group_id,
        }
    }
}
}
