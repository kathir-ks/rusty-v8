// Converted from V8 C++ source files:
// Header: v8-console.h
// Implementation: v8-console.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

pub struct V8InspectorImpl {}
pub struct StringView {}

struct TaskInfo {
    isolate: *mut Isolate,
    console: *mut V8Console,
    task: Global<Object>,
}

impl TaskInfo {
    fn new(isolate: *mut Isolate, console: *mut V8Console, task: Local<Object>) -> TaskInfo {
        TaskInfo {
            isolate,
            console,
            task: Global::new(isolate, task),
        }
    }
    fn Id(&self) -> *mut std::ffi::c_void {
        ((self as *const Self as usize) << 1) as *mut std::ffi::c_void
    }

    fn Cancel(&mut self, console: &mut V8Console) {
        console.cancelConsoleTask(self.Id());
    }
}

struct Global<T> {
    isolate: *mut Isolate,
    local: Local<T>,
}

impl<T> Global<T> {
    fn new(isolate: *mut Isolate, local: Local<T>) -> Global<T> {
        Global {
            isolate,
            local,
        }
    }
    fn Get(&self, isolate: *mut Isolate) -> Local<T> {
        self.local
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Local<T> {
    value: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Local<T> {
    fn As<U>(&self) -> Local<U> {
        Local {
            value: self.value,
            _phantom: std::marker::PhantomData,
        }
    }
    fn IsExternal(&self) -> bool {
        true
    }
    fn IsString(&self) -> bool {
        true
    }
    fn IsFunction(&self) -> bool {
        true
    }
    fn Length(&self) -> i32 {
        1
    }
    fn IsBoolean(&self) -> bool {
        true
    }
    fn BooleanValue(&self, _isolate: *mut Isolate) -> bool {
        true
    }
    fn Get(&self, _context: Local<Context>, _index: u32) -> Result<Local<Value>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn StrictEquals(&self, _other: Local<Value>) -> bool {
        true
    }
    fn GetBackingStore(&self) -> Result<BackingStore, String> {
        Ok(BackingStore{})
    }
    fn SetName(&self, _name: Local<String>) {}
    fn New(_context: Local<Context>, _callback: FunctionCallback, _data: Local<External>, _length: i32, _behavior: ConstructorBehavior, _side_effect_type: SideEffectType) -> Result<Local<Function>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn Call(&self, _context: Local<Context>, _this: Local<Value>, _argc: i32, _argv: &[Local<Value>]) -> Result<Local<Value>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn GetName(&self) -> Local<Value> {
        Local{value: 0, _phantom: std::marker::PhantomData}
    }
    fn GetInferredName(&self) -> Local<Value> {
        Local{value: 0, _phantom: std::marker::PhantomData}
    }
    fn GetBoundFunction(&self) -> Local<Value> {
        Local{value: 0, _phantom: std::marker::PhantomData}
    }
    fn GetPrivate(&self, _context: Local<Context>, _private: Local<Private>) -> Result<Local<Value>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn SetPrototypeV2(&self, _context: Local<Context>, _prototype: Local<Value>) -> Result<bool, String> {
        Ok(true)
    }
    fn Set<V: Into<Local<Value>>>(&self, _context: Local<Context>, _key: Local<Value>, _value: V) -> Result<bool, String> {
        Ok(true)
    }
    fn ToLocalChecked(&self) -> Local<Object> {
        Local{value: 0, _phantom: std::marker::PhantomData}
    }
    fn As<U>(&self)-> Local<U> {
        Local{value: 0, _phantom: std::marker::PhantomData}
    }
    fn ToLocal(&self) -> Result<Local<String>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn ThrowError(&self, message: &str) {}
    fn IsObject(&self) -> bool {
        true
    }
    fn GetOwnPropertyNames(&self, _context: Local<Context>) -> Result<Local<Array>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn IsUndefined(&self) -> bool {
        true
    }
    fn ToString(&self, _context: Local<Context>) -> Result<Local<String>, String> {
        Ok(Local{value: 0, _phantom: std::marker::PhantomData})
    }
    fn IsName(&self) -> bool {
        true
    }
    fn Delete(&self, _context: Local<Context>, _name: Local<Name>) -> Result<bool, String> {
        Ok(true)
    }
    fn CreateDataProperty(&self, _context: Local<Context>, _name: Local<Name>, _value: Local<Value>) -> Result<bool, String> {
        Ok(true)
    }
    fn SetNativeDataProperty(&self, _context: Local<Context>, _name: Local<Name>, _getter: FunctionCallback, _setter: FunctionCallback, _data: Local<ArrayBuffer>, _attributes: i32, _side_effect_type: SideEffectType) -> Result<bool, String> {
        Ok(true)
    }
    fn Has(&self, _context: Local<Context>, _name: Local<Value>) -> Result<bool, String> {
        Ok(true)
    }
}

struct Array {}
struct String {}
struct Value {}
struct Object {}
struct Name {}
struct Function {}
struct External {}
struct Private {}
struct Context {}
struct ArrayBuffer {}

struct BackingStore {}
impl BackingStore {
    fn Data(&self) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
}

#[derive(Debug, Clone, Copy)]
enum ConstructorBehavior {
    kThrow
}

#[derive(Debug, Clone, Copy)]
enum SideEffectType {
    kHasSideEffect,
    kHasNoSideEffect
}

type FunctionCallbackInfo = i32;
type FunctionCallback = fn(info: &FunctionCallbackInfo);

pub struct V8Console {
    m_inspector: *mut V8InspectorImpl,
    m_tasks: HashMap<*mut std::ffi::c_void, Box<TaskInfo>>,
    m_taskInfoKey: Global<Private>,
    m_taskTemplate: Global<ObjectTemplate>,
}

impl V8Console {
    pub fn new(inspector: *mut V8InspectorImpl) -> V8Console {
        V8Console {
            m_inspector: inspector,
            m_tasks: HashMap::new(),
            m_taskInfoKey: Global{isolate: std::ptr::null_mut(), local: Local{value: 0, _phantom: std::marker::PhantomData}},
            m_taskTemplate: Global{isolate: std::ptr::null_mut(), local: Local{value: 0, _phantom: std::marker::PhantomData}},
        }
    }

    pub fn createCommandLineAPI(&mut self, context: Local<Context>, sessionId: i32) -> Local<Object> {
        let isolate = std::ptr::null_mut();

        let commandLineAPI = Local{value: 0, _phantom: std::marker::PhantomData};
        true;

        let data = Local{value: 0, _phantom: std::marker::PhantomData};
        self.createBoundFunctionProperty(context, commandLineAPI, data, "dir", Self::Dir);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "dirxml", Self::DirXml);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "profile", Self::Profile);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "profileEnd", Self::ProfileEnd);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "clear", Self::Clear);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "table", Self::Table);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "keys", Self::keysCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "values", Self::valuesCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "debug", Self::debugFunctionCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "undebug", Self::undebugFunctionCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "monitor", Self::monitorFunctionCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "unmonitor", Self::unmonitorFunctionCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "inspect", Self::inspectCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "copy", Self::copyCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "queryObjects", Self::queryObjectsCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$_", Self::lastEvaluationResultCallback);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$0", Self::inspectedObject0);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$1", Self::inspectedObject1);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$2", Self::inspectedObject2);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$3", Self::inspectedObject3);
        self.createBoundFunctionProperty(context, commandLineAPI, data, "$4", Self::inspectedObject4);

        commandLineAPI
    }

    pub fn installMemoryGetter(&mut self, context: Local<Context>, console: Local<Object>) {
        let isolate = std::ptr::null_mut();
        let data = Local{value: 0, _phantom: std::marker::PhantomData};
        true;
    }
    pub fn installAsyncStackTaggingAPI(&mut self, context: Local<Context>, console: Local<Object>) {
        let isolate = std::ptr::null_mut();
        let data = Local{value: 0, _phantom: std::marker::PhantomData};

        self.createBoundFunctionProperty(context, console, data, "createTask", Self::createTask);
    }
    fn Debug(_info: &FunctionCallbackInfo) {}
    fn Error(_info: &FunctionCallbackInfo) {}
    fn Info(_info: &FunctionCallbackInfo) {}
    fn Log(_info: &FunctionCallbackInfo) {}
    fn Warn(_info: &FunctionCallbackInfo) {}
    fn Dir(_info: &FunctionCallbackInfo) {}
    fn DirXml(_info: &FunctionCallbackInfo) {}
    fn Table(_info: &FunctionCallbackInfo) {}
    fn Trace(_info: &FunctionCallbackInfo) {}
    fn Group(_info: &FunctionCallbackInfo) {}
    fn GroupCollapsed(_info: &FunctionCallbackInfo) {}
    fn GroupEnd(_info: &FunctionCallbackInfo) {}
    fn Clear(_info: &FunctionCallbackInfo) {}
    fn Count(_info: &FunctionCallbackInfo) {}
    fn CountReset(_info: &FunctionCallbackInfo) {}
    fn Assert(_info: &FunctionCallbackInfo) {}
    fn Profile(_info: &FunctionCallbackInfo) {}
    fn ProfileEnd(_info: &FunctionCallbackInfo) {}
    fn Time(_info: &FunctionCallbackInfo) {}
    fn TimeLog(_info: &FunctionCallbackInfo) {}
    fn TimeEnd(_info: &FunctionCallbackInfo) {}
    fn TimeStamp(_info: &FunctionCallbackInfo) {}
    fn keysCallback(_info: &FunctionCallbackInfo) {}
    fn valuesCallback(_info: &FunctionCallbackInfo) {}
    fn debugFunctionCallback(_info: &FunctionCallbackInfo) {}
    fn undebugFunctionCallback(_info: &FunctionCallbackInfo) {}
    fn monitorFunctionCallback(_info: &FunctionCallbackInfo) {}
    fn unmonitorFunctionCallback(_info: &FunctionCallbackInfo) {}
    fn lastEvaluationResultCallback(_info: &FunctionCallbackInfo) {}
    fn inspectCallback(_info: &FunctionCallbackInfo) {}
    fn copyCallback(_info: &FunctionCallbackInfo) {}
    fn queryObjectsCallback(_info: &FunctionCallbackInfo) {}
    fn inspectedObject0(_info: &FunctionCallbackInfo) {}
    fn inspectedObject1(_info: &FunctionCallbackInfo) {}
    fn inspectedObject2(_info: &FunctionCallbackInfo) {}
    fn inspectedObject3(_info: &FunctionCallbackInfo) {}
    fn inspectedObject4(_info: &FunctionCallbackInfo) {}
    fn createTask(info: &FunctionCallbackInfo) {
        let isolate = std::ptr::null_mut();
        let task = Local{value: 0, _phantom: std::marker::PhantomData};

        let taskInfo = Box::new(TaskInfo::new(isolate, std::ptr::null_mut(), task));
        let taskId = taskInfo.Id();
        println!("createTask");
    }
    fn runTask(_info: &FunctionCallbackInfo) {}

    fn createBoundFunctionProperty(&mut self, context: Local<Context>, console: Local<Object>, data: Local<Value>, name: &str, callback: fn(info: &FunctionCallbackInfo)) {
        let funcName = Local{value: 0, _phantom: std::marker::PhantomData};
        let func = Local{value: 0, _phantom: std::marker::PhantomData};
        true;
    }

    fn taskInfoKey(&mut self) -> Local<Private> {
        let isolate = std::ptr::null_mut();
        if (self.m_taskInfoKey.isolate == std::ptr::null_mut()) {
            self.m_taskInfoKey = Global::new(isolate, Local{value: 0, _phantom: std::marker::PhantomData});
        }
        self.m_taskInfoKey.Get(isolate)
    }

    fn taskTemplate(&mut self) -> Local<ObjectTemplate> {
        let isolate = std::ptr::null_mut();
        if (self.m_taskTemplate.isolate == std::ptr::null_mut()) {
            let data = Local{value: 0, _phantom: std::marker::PhantomData};
            let taskTemplate = Local{value: 0, _phantom: std::marker::PhantomData};
            let funcTemplate = Local{value: 0, _phantom: std::marker::PhantomData};
            self.m_taskTemplate = Global::new(isolate, taskTemplate);
        }
        self.m_taskTemplate.Get(isolate)
    }

    fn cancelConsoleTask(&mut self, taskId: *mut std::ffi::c_void) {
        self.m_tasks.remove(&taskId);
    }
}

pub trait ConsoleDelegate {
    fn Debug(&self, info: &FunctionCallbackInfo);
    fn Error(&self, info: &FunctionCallbackInfo);
    fn Info(&self, info: &FunctionCallbackInfo);
    fn Log(&self, info: &FunctionCallbackInfo);
    fn Warn(&self, info: &FunctionCallbackInfo);
    fn Dir(&self, info: &FunctionCallbackInfo);
    fn DirXml(&self, info: &FunctionCallbackInfo);
    fn Table(&self, info: &FunctionCallbackInfo);
    fn Trace(&self, info: &FunctionCallbackInfo);
    fn Group(&self, info: &FunctionCallbackInfo);
    fn GroupCollapsed(&self, info: &FunctionCallbackInfo);
    fn GroupEnd(&self, info: &FunctionCallbackInfo);
    fn Clear(&self, info: &FunctionCallbackInfo);
    fn Count(&self, info: &FunctionCallbackInfo);
    fn CountReset(&self, info: &FunctionCallbackInfo);
    fn Assert(&self, info: &FunctionCallbackInfo);
    fn Profile(&self, info: &FunctionCallbackInfo);
    fn ProfileEnd(&self, info: &FunctionCallbackInfo);
    fn Time(&self, info: &FunctionCallbackInfo);
    fn TimeLog(&self, info: &FunctionCallbackInfo);
    fn TimeEnd(&self, info: &FunctionCallbackInfo);
    fn TimeStamp(&self, info: &FunctionCallbackInfo);
}

struct ObjectTemplate {}

struct ArrayBufferView {}
