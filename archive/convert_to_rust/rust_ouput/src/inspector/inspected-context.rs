// Converted from V8 C++ source files:
// Header: inspected-context.h
// Implementation: inspected-context.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::string_util::V8;

//use v8::debug::SetContextId;

//use v8::debug::EphemeronTable;

use crate::InjectedScript;
use crate::String16;
use crate::V8InspectorImpl;

//use crate::v8_inspector::V8InternalValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum V8InternalValueType {
    kNone,
    kEntry,
    kScope,
    kScopeList,
    kPrivateMethodList,
    kPrivateMethod,
}

pub struct InspectedContext {
    m_inspector: *mut V8InspectorImpl,
    m_context: i32, //Dummy type
    m_contextId: i32,
    m_contextGroupId: i32,
    m_origin: String16,
    m_humanReadableName: String16,
    m_auxData: String16,
    m_uniqueId: i32,      //Dummy type internal::V8DebuggerId
    m_reportedSessionIds: HashSet<i32>,
    m_injectedScripts: HashMap<i32, Box<InjectedScript>>,
    m_weakCallbackData: *mut WeakCallbackData, //Box<WeakCallbackData>,
    m_internalObjects: i32, //Dummy type v8::Global<v8::debug::EphemeronTable>
}

impl fmt::Debug for InspectedContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InspectedContext")
            .field("m_contextId", &self.m_contextId)
            .field("m_contextGroupId", &self.m_contextGroupId)
            .field("m_origin", &self.m_origin)
            .field("m_humanReadableName", &self.m_humanReadableName)
            .field("m_auxData", &self.m_auxData)
            .field("m_uniqueId", &self.m_uniqueId)
            .field("m_reportedSessionIds", &self.m_reportedSessionIds)
            .field("m_injectedScripts", &self.m_injectedScripts.keys())
            .finish()
    }
}

impl Drop for InspectedContext {
    fn drop(&mut self) {
        //drop(self.m_weakCallbackData);
        //if !self.m_context.IsEmpty() {
        unsafe {
            if !self.m_context == 0 {
                drop(Box::from_raw(self.m_weakCallbackData));
            }
        }
    }
}

impl InspectedContext {
    pub fn contextId(_context: i32) -> i32 {
        //v8::debug::GetContextId(context)
        0
    }

    pub fn context(&self) -> i32 {
        //v8::Local<v8::Context>
        self.m_context //self.m_context.Get(self.isolate())
    }

    pub fn contextId_(&self) -> i32 {
        self.m_contextId
    }

    pub fn contextGroupId(&self) -> i32 {
        self.m_contextGroupId
    }

    pub fn origin(&self) -> String16 {
        self.m_origin.clone()
    }

    pub fn humanReadableName(&self) -> String16 {
        self.m_humanReadableName.clone()
    }

    pub fn uniqueId(&self) -> i32 {
        self.m_uniqueId
    }

    pub fn auxData(&self) -> String16 {
        self.m_auxData.clone()
    }

    pub fn isReported(&self, sessionId: i32) -> bool {
        self.m_reportedSessionIds.contains(&sessionId)
    }

    pub fn setReported(&mut self, sessionId: i32, reported: bool) {
        if reported {
            self.m_reportedSessionIds.insert(sessionId);
        } else {
            self.m_reportedSessionIds.remove(&sessionId);
        }
    }

    pub fn isolate(&self) -> *mut V8 {
        //v8::Isolate*
        unsafe {
            (*(self.m_inspector)).isolate()
        }
    }

    pub fn inspector(&self) -> *mut V8InspectorImpl {
        self.m_inspector
    }

    pub fn getInjectedScript(&self, sessionId: i32) -> Option<&InjectedScript> {
        self.m_injectedScripts.get(&sessionId).map(|x| x.as_ref())
    }

    pub fn createInjectedScript(&mut self, sessionId: i32) -> *mut InjectedScript {
        let injected_script = InjectedScript::new();
        let injected_script_ptr = Box::into_raw(Box::new(injected_script));
        self.m_injectedScripts.insert(sessionId, unsafe{Box::from_raw(injected_script_ptr)});
        
        let script = self.m_injectedScripts.get(&sessionId).unwrap();
        script.as_ref() as *const InjectedScript as *mut InjectedScript
    }

    pub fn discardInjectedScript(&mut self, sessionId: i32) {
        self.m_injectedScripts.remove(&sessionId);
    }

    pub fn addInternalObject(&mut self, _object: i32, _type: V8InternalValueType) -> bool {
        //v8::Local<v8::Object> object,
        //V8InternalValueType type
        true
    }

    pub fn getInternalType(&self, _object: i32) -> V8InternalValueType {
        //v8::Local<v8::Object> object
        V8InternalValueType::kNone
    }
}

impl InspectedContext {
    pub fn new(inspector: *mut V8InspectorImpl, info: &V8ContextInfo, contextId: i32) -> InspectedContext {
        let mut inspected_context = InspectedContext {
            m_inspector: inspector,
            m_context: info.context,
            m_contextId: contextId,
            m_contextGroupId: info.contextGroupId,
            m_origin: String16::from(info.origin.as_str()),
            m_humanReadableName: String16::from(info.humanReadableName.as_str()),
            m_auxData: String16::from(info.auxData.as_str()),
            m_uniqueId: 0, //internal::V8DebuggerId::generate(inspector),
            m_reportedSessionIds: HashSet::new(),
            m_injectedScripts: HashMap::new(),
            m_weakCallbackData: std::ptr::null_mut(),
            m_internalObjects: 0,
        };
        unsafe {
            inspected_context.m_uniqueId = (*inspector).generate_id();
        }
        let weak_callback_data = Box::new(WeakCallbackData::new(&inspected_context, inspector, inspected_context.m_contextGroupId, inspected_context.m_contextId));
        inspected_context.m_weakCallbackData = Box::into_raw(weak_callback_data);

        inspected_context
    }
}

struct ContextCollectedCallbacks {
    data_: *mut WeakCallbackData, //std::unique_ptr<InspectedContext::WeakCallbackData>
}

impl ContextCollectedCallbacks {
    fn new(data: *mut WeakCallbackData) -> ContextCollectedCallbacks {
        ContextCollectedCallbacks { data_: data }
    }

    fn Run(&mut self) {
        unsafe {
            (*(self.data_)).m_inspector.as_mut().unwrap().contextCollected((*(self.data_)).m_groupId, (*(self.data_)).m_contextId);
            //self.data_.m_inspector.contextCollected(self.data_.m_groupId, self.data_.m_contextId);
        }
    }
}

struct WeakCallbackData {
    m_context: *const InspectedContext, //InspectedContext*
    m_inspector: *mut V8InspectorImpl, //V8InspectorImpl*
    m_groupId: i32,
    m_contextId: i32,
}

impl WeakCallbackData {
    fn new(context: &InspectedContext, inspector: *mut V8InspectorImpl, groupId: i32, contextId: i32) -> WeakCallbackData {
        WeakCallbackData {
            m_context: context,
            m_inspector: inspector,
            m_groupId: groupId,
            m_contextId: contextId,
        }
    }

    fn resetContext(data: &WeakCallbackData) {
        //const v8::WeakCallbackInfo<WeakCallbackData>& data
        // InspectedContext is alive here because weak handler is still alive.
        //data.GetParameter()->m_context->m_weakCallbackData = nullptr;
        //data.GetParameter()->m_context->m_context.Reset();
        //v8::debug::GetCurrentPlatform()
        //    ->GetForegroundTaskRunner(data.GetIsolate())
        //    ->PostTask(
        //        std::make_unique<ContextCollectedCallbacks>(data.GetParameter()));
        println!("WeakCallbackData::resetContext");
    }
}

pub struct V8ContextInfo {
    pub context: i32,
    pub contextGroupId: i32,
    pub origin: String,
    pub humanReadableName: String,
    pub auxData: String,
    pub hasMemoryOnConsole: bool,
}
