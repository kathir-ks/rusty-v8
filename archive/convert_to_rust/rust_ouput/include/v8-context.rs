// Converted from V8 C++ source files:
// Header: v8-context.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
use std::ptr::null;

use crate::v8_template::{Data, Local, Name, ObjectTemplate, PropertyAttribute};

pub struct Isolate {}

pub struct MicrotaskQueue {}

pub struct Object {}

pub struct Value {}

pub struct String {}

pub type DeserializeInternalFieldsCallback = *const std::ffi::c_void;
pub type DeserializeContextDataCallback = *const std::ffi::c_void;
pub type DeserializeAPIWrapperCallback = *const std::ffi::c_void;

pub struct ExtensionConfiguration {
    name_count_: i32,
    names_: *const *const char,
}

impl ExtensionConfiguration {
    pub fn new() -> Self {
        ExtensionConfiguration {
            name_count_: 0,
            names_: null(),
        }
    }

    pub fn new_with_names(name_count: i32, names: *const *const char) -> Self {
        ExtensionConfiguration {
            name_count_: name_count,
            names_: names,
        }
    }

    pub fn begin(&self) -> *const *const char {
        unsafe { &*self.names_ }
    }
    pub fn end(&self) -> *const *const char {
        unsafe { &*self.names_.offset(self.name_count_ as isize) }
    }
}

pub struct Context {
}

impl Context {
    pub fn Global(&self) -> Local<Object> {
        Local::empty()
    }

    pub fn DetachGlobal(&self) {}

    pub fn New(
        isolate: *mut Isolate,
        extensions: *mut ExtensionConfiguration,
        global_template: MaybeLocal<ObjectTemplate>,
        global_object: MaybeLocal<Value>,
        internal_fields_deserializer: DeserializeInternalFieldsCallback,
        microtask_queue: *mut MicrotaskQueue,
        context_data_deserializer: DeserializeContextDataCallback,
        api_wrapper_deserializer: DeserializeAPIWrapperCallback,
    ) -> Local<Context> {
        Local::empty()
    }

    pub fn FromSnapshot(
        isolate: *mut Isolate,
        context_snapshot_index: usize,
        internal_fields_deserializer: DeserializeInternalFieldsCallback,
        extensions: *mut ExtensionConfiguration,
        global_object: MaybeLocal<Value>,
        microtask_queue: *mut MicrotaskQueue,
        context_data_deserializer: DeserializeContextDataCallback,
        api_wrapper_deserializer: DeserializeAPIWrapperCallback,
    ) -> MaybeLocal<Context> {
        MaybeLocal::empty()
    }

    pub fn NewRemoteContext(
        isolate: *mut Isolate,
        global_template: Local<ObjectTemplate>,
        global_object: MaybeLocal<Value>,
    ) -> MaybeLocal<Object> {
        MaybeLocal::empty()
    }

    pub fn SetSecurityToken(&self, token: Local<Value>) {}

    pub fn UseDefaultSecurityToken(&self) {}

    pub fn GetSecurityToken(&self) -> Local<Value> {
        Local::empty()
    }

    pub fn Enter(&self) {}

    pub fn Exit(&self) {}

    pub struct DeepFreezeDelegate {}

    impl DeepFreezeDelegate {
        pub fn FreezeEmbedderObjectAndGetChildren(
            obj: Local<Object>,
            children_out: &mut LocalVector<Object>,
        ) -> bool {
            true
        }
    }

    pub fn DeepFreeze(delegate: *mut DeepFreezeDelegate) -> Maybe<void> {
        Maybe::empty()
    }

    pub fn GetIsolate(&self) -> *mut Isolate {
        null_mut()
    }

    pub fn GetMicrotaskQueue(&self) -> *mut MicrotaskQueue {
        null_mut()
    }

    pub fn SetMicrotaskQueue(&self, queue: *mut MicrotaskQueue) {}

    pub fn GetNumberOfEmbedderDataFields(&self) -> u32 {
        0
    }

    pub fn GetEmbedderData(&self, index: i32) -> Local<Value> {
        Local::empty()
    }

    pub fn GetExtrasBindingObject(&self) -> Local<Object> {
        Local::empty()
    }

    pub fn SetEmbedderData(&self, index: i32, value: Local<Value>) {}

    pub fn GetAlignedPointerFromEmbedderData(&self, isolate: *mut Isolate, index: i32) -> *mut std::ffi::c_void {
        null_mut()
    }

    pub fn GetAlignedPointerFromEmbedderData_no_iso(&self, index: i32) -> *mut std::ffi::c_void {
        null_mut()
    }

    pub fn SetAlignedPointerInEmbedderData(&self, index: i32, value: *mut std::ffi::c_void) {}

    pub fn AllowCodeGenerationFromStrings(&self, allow: bool) {}

    pub fn IsCodeGenerationFromStringsAllowed(&self) -> bool {
        false
    }

    pub fn SetErrorMessageForCodeGenerationFromStrings(&self, message: Local<String>) {}

    pub fn SetErrorMessageForWasmCodeGeneration(&self, message: Local<String>) {}

    pub fn GetDataFromSnapshotOnce<T>(&self, index: usize) -> MaybeLocal<T> {
        MaybeLocal::empty()
    }

    pub type AbortScriptExecutionCallback =
        Option<unsafe extern "C" fn(isolate: *mut Isolate, context: Local<Context>)>;
    pub fn SetAbortScriptExecution(&self, callback: AbortScriptExecutionCallback) {}

    pub fn SetPromiseHooks(
        &self,
        init_hook: Local<Function>,
        before_hook: Local<Function>,
        after_hook: Local<Function>,
        resolve_hook: Local<Function>,
    ) {
    }

    pub fn HasTemplateLiteralObject(&self, object: Local<Value>) -> bool {
        false
    }

    pub struct Scope {
        context_: Local<Context>,
    }

    impl Scope {
        pub fn new(context: Local<Context>) -> Self {
            let scope = Scope { context_: context };
            scope.context_.Enter();
            scope
        }
    }

    impl Drop for Scope {
        fn drop(&mut self) {
            self.context_.Exit();
        }
    }

    pub struct BackupIncumbentScope {
        backup_incumbent_context_: Local<Context>,
        js_stack_comparable_address_: usize,
        prev_: *const BackupIncumbentScope,
    }

    impl BackupIncumbentScope {
        pub fn new(backup_incumbent_context: Local<Context>) -> Self {
            BackupIncumbentScope {
                backup_incumbent_context_: backup_incumbent_context,
                js_stack_comparable_address_: 0,
                prev_: null(),
            }
        }
    }

    impl Drop for BackupIncumbentScope {
        fn drop(&mut self) {}
    }

    pub fn Cast(data: *mut Data) -> *mut Context {
        data as *mut Context
    }
}

pub struct MaybeLocal<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MaybeLocal<T> {
    pub fn empty() -> Self {
        MaybeLocal {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct LocalVector<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> LocalVector<T> {
    pub fn new() -> Self {
        LocalVector {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct Maybe<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Maybe<T> {
    pub fn empty() -> Self {
        Maybe {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub enum void {}

pub struct Function {}

use std::ptr::null_mut;
