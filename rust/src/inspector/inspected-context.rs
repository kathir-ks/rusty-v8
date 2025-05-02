// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use v8::{Context, HandleScope, Integer, Isolate, Local, Object, Value};

mod string_util;
use string_util::to_string16;

mod v8_console;
use v8_console::V8Console;

mod v8_inspector_impl;
use v8_inspector_impl::V8InspectorImpl;

mod injected_script;
use injected_script::InjectedScript;

mod internal {
    pub mod v8_debugger_id {
        use std::sync::atomic::{AtomicU64, Ordering};

        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        pub fn generate<T>(_owner: &T) -> u64 {
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V8InternalValueType {
    kNone,
    // Add other types as needed.
}

pub struct V8ContextInfo<'a> {
    pub context: Local<'a, Context>,
    pub context_group_id: i32,
    pub origin: String, // Assuming String is equivalent to v8::String
    pub human_readable_name: String, // Assuming String is equivalent to v8::String
    pub aux_data: String,
    pub has_memory_on_console: bool,
}

pub struct InspectedContext {
    m_inspector: *mut V8InspectorImpl, // Raw pointer, handle with care.  Consider using Rc/Arc if possible.
    m_context: v8::Global<Context>,
    m_context_id: i32,
    m_context_group_id: i32,
    m_origin: String,
    m_human_readable_name: String,
    m_aux_data: String,
    m_unique_id: u64,
    m_weak_callback_data: *mut WeakCallbackData, // Raw pointer, handle with care. Consider using Rc/Arc if possible.
    m_reported_session_ids: RefCell<HashSet<i32>>,
    m_injected_scripts: RefCell<HashMap<i32, Box<InjectedScript>>>,
    m_internal_objects: RefCell<v8::Global<v8::debug::EphemeronTable>>, // Assuming EphemeronTable is properly defined
}

struct WeakCallbackData {
    m_context: *mut InspectedContext, // Raw pointer, handle with care.  Consider using Rc/Arc if possible.
    m_inspector: *mut V8InspectorImpl, // Raw pointer, handle with care.  Consider using Rc/Arc if possible.
    m_group_id: i32,
    m_context_id: i32,
}

impl InspectedContext {
    pub fn new(inspector: *mut V8InspectorImpl, info: &V8ContextInfo, context_id: i32) -> InspectedContext {
        let isolate = info.context.get_isolate();
        let context = v8::Global::new(isolate, info.context);

        let mut inspected_context = InspectedContext {
            m_inspector: inspector,
            m_context: context,
            m_context_id: context_id,
            m_context_group_id: info.context_group_id,
            m_origin: info.origin.clone(),
            m_human_readable_name: info.human_readable_name.clone(),
            m_aux_data: info.aux_data.clone(),
            m_unique_id: internal::v8_debugger_id::generate(inspector),
            m_weak_callback_data: std::ptr::null_mut(), // Initialized to null.
            m_reported_session_ids: RefCell::new(HashSet::new()),
            m_injected_scripts: RefCell::new(HashMap::new()),
            m_internal_objects: RefCell::new(v8::Global::new(isolate, unsafe { std::mem::zeroed() })), // Initialize with zeroed EphemeronTable
        };

        unsafe {
            v8::debug::SetContextId(info.context, context_id);

            let weak_callback_data = Box::into_raw(Box::new(WeakCallbackData {
                m_context: &mut inspected_context,
                m_inspector: inspector,
                m_group_id: info.context_group_id,
                m_context_id: context_id,
            }));

            inspected_context.m_weak_callback_data = weak_callback_data;

            let context = inspected_context.m_context.clone();

            // Setup Weak Handle
            let context_ptr = inspected_context.m_weak_callback_data;

            // TODO: Reimplement SetWeak function here using WeakRef/PhantomData if necessary to simulate the C++ behavior
            let persistent_handle = v8::Persistent::<Context>::new(isolate, info.context);

            persistent_handle.set_weak(
                isolate,
                Some(weak_callback_fn),
                v8::WeakRefType::kParameter,
            );

            std::mem::forget(persistent_handle); // Prevent drop
        }

        let mut scope = v8::ContextScope::new(info.context);
        let mut handle_scope = v8::HandleScope::new(isolate);
        let global = info.context.global(&mut scope);

        let console_name = string_util::to_v8string(isolate, "console");

        let console_result = global.get(&mut scope, console_name);
        if let Some(console) = console_result {
            if console.is_object() {
                unsafe {
                    let inspector_ptr = inspected_context.m_inspector;
                    let console_obj = console.try_into::<v8::Object>().unwrap();

                    (*inspector_ptr).console().install_async_stack_tagging_api(info.context, console_obj);
                    if info.has_memory_on_console {
                        (*inspector_ptr).console().install_memory_getter(info.context, console_obj);
                    }
                }
            }
        }

        inspected_context
    }

    pub fn context_id(context: Local<Context>) -> i32 {
        unsafe { v8::debug::GetContextId(context) }
    }

    pub fn context(&self) -> Local<Context> {
        self.m_context.get(self.isolate())
    }

    pub fn isolate(&self) -> *mut Isolate {
        unsafe { (*self.m_inspector).isolate() }
    }

    pub fn is_reported(&self, session_id: i32) -> bool {
        self.m_reported_session_ids.borrow().contains(&session_id)
    }

    pub fn set_reported(&self, session_id: i32, reported: bool) {
        let mut reported_session_ids = self.m_reported_session_ids.borrow_mut();
        if reported {
            reported_session_ids.insert(session_id);
        } else {
            reported_session_ids.remove(&session_id);
        }
    }

    pub fn get_injected_script(&self, session_id: i32) -> Option<&InjectedScript> {
        self.m_injected_scripts.borrow().get(&session_id).map(|script| script.as_ref())
    }

    pub fn create_injected_script(&self, session_id: i32) -> *mut InjectedScript {
        let injected_script = Box::new(InjectedScript::new(self, session_id));
        let injected_script_ptr = Box::into_raw(injected_script);

        self.m_injected_scripts
            .borrow_mut()
            .insert(session_id, unsafe { Box::from_raw(injected_script_ptr) });

        injected_script_ptr
    }

    pub fn discard_injected_script(&self, session_id: i32) {
        self.m_injected_scripts.borrow_mut().remove(&session_id);
    }

    pub fn add_internal_object(&self, object: Local<Object>, r#type: V8InternalValueType) -> bool {
        let mut m_internal_objects = self.m_internal_objects.borrow_mut();
        let isolate = self.isolate();

        if m_internal_objects.is_empty() {
            let ephemeron_table = unsafe { v8::debug::EphemeronTable::new(isolate) };
            m_internal_objects.replace(v8::Global::new(isolate, ephemeron_table));
        }

        unsafe {
            let new_map = m_internal_objects.get(isolate)
                .set(isolate, object, Integer::new(isolate, r#type as i32));
            m_internal_objects.replace(v8::Global::new(isolate, new_map));
        }

        true
    }

    pub fn get_internal_type(&self, object: Local<Object>) -> V8InternalValueType {
        let m_internal_objects = self.m_internal_objects.borrow();
        let isolate = self.isolate();

        if m_internal_objects.is_empty() {
            return V8InternalValueType::kNone;
        }

        let type_value = unsafe {
            let mut value = std::mem::MaybeUninit::<Local<Value>>::uninit();
             let success = m_internal_objects.get(isolate)
                .get(isolate, object)
                .to_local(value.as_mut_ptr());

            if !success {
                return V8InternalValueType::kNone;
            }

            let type_value = value.assume_init();

            if !type_value.is_uint32() {
                return V8InternalValueType::kNone;
            }

            type_value.try_into::<Integer>().unwrap().value() as i32
        };

        match type_value {
            0 => V8InternalValueType::kNone,
            _ => V8InternalValueType::kNone, // Add other types as needed.
        }
    }
}

impl Drop for InspectedContext {
    fn drop(&mut self) {
        // If we destroy InspectedContext before weak callback is invoked then we need
        // to delete data here.
        if !self.m_context.is_empty() {
            unsafe {
                if !self.m_weak_callback_data.is_null() {
                    drop(Box::from_raw(self.m_weak_callback_data));
                }
            }
        }
    }
}

extern "C" fn weak_callback_fn(data: &v8::WeakRef<Context>, _parameter: *mut ()) {
    let weak_data = unsafe { (*(data as *const v8::Persistent<Context> as *const v8::WeakRef<Context> as *const v8::Weak<Context>)).parameter::<WeakCallbackData>() as *mut WeakCallbackData };

    unsafe {
        let context = (*weak_data).m_context;
        (*context).m_weak_callback_data = std::ptr::null_mut();
        (*context).m_context.reset();

        let inspector = (*weak_data).m_inspector;
        let group_id = (*weak_data).m_group_id;
        let context_id = (*weak_data).m_context_id;

        (*inspector).context_collected(group_id, context_id);

        drop(Box::from_raw(weak_data));
    }
}

// Placeholder for EphemeronTable (V8 doesn't expose EphemeronTable directly)
mod v8 {
    pub mod debug {
        use v8::{Context, Isolate, Local, Object, Value, Integer};

        #[allow(dead_code)]
        extern "C" {
            pub fn SetContextId(context: Local<Context>, context_id: i32);
            pub fn GetContextId(context: Local<Context>) -> i32;
        }

        pub struct EphemeronTable {
            _private: (),
        }

        impl EphemeronTable {
            pub unsafe fn new(_isolate: *mut Isolate) -> EphemeronTable {
                 EphemeronTable { _private: () }
            }

            pub unsafe fn set<'s>(&self, _isolate: *mut Isolate, _object: Local<'s, Object>, _value: Local<'s, Integer>) -> EphemeronTable {
                EphemeronTable { _private: () }
            }

            pub unsafe fn get<'s>(&self, _isolate: *mut Isolate, _object: Local<'s, Object>) -> Local<'s, Value> {
                Local::from(Value::new())
            }
        }
    }
    pub mod platform {
        use v8::Isolate;

        #[allow(dead_code)]
        extern "C" {
            pub fn GetCurrentPlatform() -> *mut dyn Any;
        }

        pub struct ForegroundTaskRunner;

        impl ForegroundTaskRunner {
            pub fn post_task<F: FnOnce()>(&self, _task: F) {
                // Placeholder for posting tasks to the foreground task runner.
                todo!()
            }
        }

        pub trait Platform {
            fn get_foreground_task_runner(&self, _isolate: *mut Isolate) -> ForegroundTaskRunner;
        }
    }

    pub trait PersistentBase<T> {
        fn new(isolate: *mut Isolate, value: Local<'_, T>) -> Self;
        fn get(&self, isolate: *mut Isolate) -> Local<'_, T>;
        fn reset(&mut self);
        fn is_empty(&self) -> bool;
    }

    pub struct Global<T> {
        _private: std::marker::PhantomData<T>,
        _isolate: *mut Isolate,
        _local: *mut T,
    }

    impl<T> Global<T> {
        pub fn new(isolate: *mut Isolate, _value: Local<'_, T>) -> Self {
            Global {
                _private: std::marker::PhantomData,
                _isolate: isolate,
                _local: std::ptr::null_mut(), //TODO: Implement the global
            }
        }

        pub fn get(&self, isolate: *mut Isolate) -> Local<'_, T> {
           unsafe{ Local::from(&mut *self._local) } //TODO: Implement the global
        }

        pub fn reset(&mut self) {
            //TODO: Implement the reset
        }

        pub fn is_empty(&self) -> bool {
            self._local.is_null() //TODO: Implement the global
        }
    }

    // Implement PersistentBase for Global
    impl<T> PersistentBase<T> for Global<T> {
        fn new(isolate: *mut Isolate, value: Local<'_, T>) -> Self {
            Global::new(isolate, value)
        }

        fn get(&self, isolate: *mut Isolate) -> Local<'_, T> {
            self.get(isolate)
        }

        fn reset(&mut self) {
            self.reset();
        }

        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }

    #[repr(C)]
    pub struct Weak<T> {
        _private: std::marker::PhantomData<T>,
    }

    #[repr(C)]
    pub struct WeakRef<T> {
        _private: std::marker::PhantomData<T>,
    }

    impl<T> WeakRef<T> {
        pub fn parameter<U>(&self) -> *mut U {
            // Placeholder for retrieving the parameter.
            std::ptr::null_mut()
        }
    }

    #[repr(C)]
    pub struct Persistent<'i, T> {
        _private: std::marker::PhantomData<T>,
        _isolate: *mut Isolate,
        _local: Local<'i, T>
    }

    impl<'i, T> Persistent<'i, T> {
        pub fn new(isolate: *mut Isolate, value: Local<'i, T>) -> Self {
            Persistent {
                _private: std::marker::PhantomData,
                _isolate: isolate,
                _local: value
            }
        }

        pub fn set_weak<U>(&mut self, _isolate: *mut Isolate, _callback: Option<extern "C" fn(&WeakRef<T>, *mut ())>, _weak_ref_type: WeakRefType) {
            // Placeholder for setting the weak callback.
        }

        pub fn reset(&mut self) {
            // Placeholder for resetting the persistent handle.
        }

        pub fn is_empty(&self) -> bool {
            // Placeholder for checking if the persistent handle is empty.
            false
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum WeakRefType {
        kParameter,
    }
}