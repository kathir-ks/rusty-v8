// Converted from V8 C++ source files:
// Header: v8.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod v8_array_buffer;
mod v8_container;
mod v8_context;
mod v8_data;
mod v8_date;
mod v8_debug;
mod v8_exception;
mod v8_extension;
mod v8_external;
mod v8_function;
mod v8_initialization;
mod v8_internal;
mod v8_isolate;
mod v8_json;
mod v8_local_handle;
mod v8_locker;
mod v8_maybe;
mod v8_memory_span;
mod v8_message;
mod v8_microtask_queue;
mod v8_microtask;
mod v8_object;
mod v8_persistent_handle;
mod v8_primitive_object;
mod v8_primitive;
mod v8_promise;
mod v8_proxy;
mod v8_regexp;
mod v8_script;
mod v8_snapshot;
mod v8_statistics;
mod v8_template;
mod v8_traced_handle;
mod v8_typed_array;
mod v8_unwinder;
mod v8_value_serializer;
mod v8_value;
mod v8_version;
mod v8_wasm;
mod v8config;

pub mod cppgc {
    pub mod common {
        pub struct EnableIf<const B: bool, T> {
            _marker: std::marker::PhantomData<T>,
        }
        impl<T> EnableIf<true, T> {
            pub fn new() -> Self {
                EnableIf {
                    _marker: std::marker::PhantomData,
                }
            }
        }

        pub type FalseType = FalseTypeImpl;

        pub struct FalseTypeImpl {}
        impl FalseTypeImpl {
           pub fn new() -> Self {
                FalseTypeImpl {}
           }
        }

        pub trait BoolConstant {
            const VALUE: bool;
        }

        impl BoolConstant for FalseTypeImpl {
            const VALUE: bool = false;
        }
    }
}

pub mod std {
    pub mod memory {
        pub struct UniquePtr<T> {
            ptr: *mut T,
        }

        impl<T> UniquePtr<T> {
            pub fn new(ptr: *mut T) -> Self {
                UniquePtr { ptr }
            }

            pub fn get(&self) -> *mut T {
                self.ptr
            }
        }
    }
}

pub struct Local<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Local<'a, T> {
    pub fn new() -> Self {
        Local {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Clone for Local<'a, T> {
    fn clone(&self) -> Self {
        Local::new()
    }
}

impl<'a, T> Copy for Local<'a, T> {}

pub struct Isolate {
}

impl Isolate {
    pub fn new(params:CreateParams) -> Box<Isolate> {
        Box::new(Isolate{})
    }

    pub fn get_current_context(&mut self) -> Local<'static, Context> {
        Local::new()
    }

    pub fn enter(&mut self) {
    }

    pub fn exit(&mut self) {
    }

    pub fn dispose(self: Box<Self>) {}

    pub fn new_handle_scope(&mut self) -> HandleScope {
        HandleScope{}
    }
}

pub struct HandleScope{
}

impl HandleScope {
   pub fn new(_isolate: &mut Isolate) -> Self {
        HandleScope {}
   }
}

pub struct Context{
}

impl Context{
    pub fn new(isolate: &mut Isolate) -> Local<'static, Context> {
        Local::new()
    }

    pub fn get_isolate(&self) -> &mut Isolate {
       todo!()
    }
}

pub struct Script {
}

impl Script{
    pub fn compile(context: Local<'static, Context>, source: Local<'static, String>) -> MaybeLocal<'static, Script> {
        MaybeLocal::new()
    }

    pub fn run(self: Local<'static, Script>, context: Local<'static, Context>) -> MaybeLocal<'static, Value> {
        MaybeLocal::new()
    }
}

pub struct String{
}

impl String{
    pub fn new_from_utf8(isolate: &mut Isolate, buffer: &[u8], new_type: NewStringType) -> MaybeLocal<'static, String> {
        MaybeLocal::new()
    }

    pub fn empty(isolate: &mut Isolate) -> Local<'static, String> {
        Local::new()
    }
}

#[derive(Debug)]
pub enum NewStringType {
    kNormal,
    kInternalized,
}

pub struct Value{
}

impl Value{
    pub fn to_string(self: Local<'static, Value>, context: Local<'static, Context>) -> MaybeLocal<'static, String> {
        MaybeLocal::new()
    }
}

pub struct MaybeLocal<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> MaybeLocal<'a, T> {
    pub fn new() -> Self {
        MaybeLocal {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn to_local_checked(&self) -> Local<'static, T> {
        Local::new()
    }
}

pub struct Object{
}

impl Object {
    pub fn new(isolate: &mut Isolate) -> Local<'static, Object> {
        Local::new()
    }
}

pub struct Function {
}

impl Function {
    pub fn call(context: Local<'static, Context>, recv: Local<'static, Value>, args: &[Local<'static, Value>]) -> MaybeLocal<'static, Value> {
        MaybeLocal::new()
    }
}

pub struct FunctionTemplate {
}

impl FunctionTemplate {
    pub fn new(isolate: &mut Isolate, callback: FunctionCallback) -> Local<'static, FunctionTemplate> {
        Local::new()
    }

     pub fn get_function(self: Local<'static, FunctionTemplate>, context: Local<'static, Context>) -> MaybeLocal<'static, Function> {
        MaybeLocal::new()
     }
}

pub type FunctionCallback = fn(CallbackInfo);

pub struct CallbackInfo {
}

impl CallbackInfo {
    pub fn data(&self) -> Local<'static, Value> {
        Local::new()
    }

    pub fn get_isolate(&self) -> &mut Isolate {
        todo!()
    }

    pub fn this(&self) -> Local<'static, Object> {
        Local::new()
    }

    pub fn length(&self) -> i32 {
        0
    }

    pub fn get(self, i:i32) -> Local<'static, Value> {
        Local::new()
    }
}

pub struct External {
}

impl External {
    pub fn new(isolate: &mut Isolate, value: *mut std::ffi::c_void) -> Local<'static, External> {
        Local::new()
    }
}

pub struct Template {
}

pub struct Data {
}

pub struct Primitive {
}

pub struct Promise {
}

pub struct Proxy {
}

pub struct RegExp {
}

pub struct Snapshot {
}

pub struct Statistics {
}

pub struct TracedHandle {
}

pub struct TypedArray {
}

pub struct Unwinder {
}

pub struct ValueSerializer {
}

pub struct Version {
}

pub struct Wasm {
}

pub struct StartupData {
}

pub struct ArrayBuffer{
}

impl ArrayBuffer {
    pub fn new(isolate: &mut Isolate, byte_length: usize) -> Local<'static, ArrayBuffer> {
        Local::new()
    }
}

pub struct Exception{
}

impl Exception {
    pub fn type_error(isolate: &mut Isolate, message: Local<'static, String>) -> Local<'static, Value> {
        Local::new()
    }
}

pub struct TryCatch {
}

impl TryCatch {
    pub fn new(isolate: &mut Isolate) -> Self {
        TryCatch {}
    }

    pub fn has_caught(&self) -> bool {
        false
    }

    pub fn exception(&self) -> Local<'static, Value> {
        Local::new()
    }

    pub fn rethrow(&mut self) -> MaybeLocal<'static, Value> {
        MaybeLocal::new()
    }
}

pub struct Message{
}

impl Message{
    pub fn get(self) -> Local<'static, String> {
       Local::new()
    }
}

pub struct JSON{
}

impl JSON {
    pub fn stringify(context: Local<'static, Context>, json_object: Local<'static, Value>) -> MaybeLocal<'static, String> {
        MaybeLocal::new()
    }
}

pub struct ScriptCompiler{
}

impl ScriptCompiler{
    pub fn compile_module(isolate: &mut Isolate, source: ScriptCompilerSource, params: ScriptCompilerCompileOptions) -> MaybeLocal<'static, Module> {
        MaybeLocal::new()
    }
}

pub struct ScriptCompilerSource{
}

pub struct ScriptCompilerCompileOptions{
}

pub struct Module{
}

impl Module {
    pub fn evaluate(self: Local<'static, Module>, context: Local<'static, Context>) -> MaybeLocal<'static, Value> {
        MaybeLocal::new()
    }

     pub fn get_module_requests_length(&self) -> i32 {
        0
    }

    pub fn get_module_request(&self, i: i32) -> Local<'static, String> {
        Local::new()
    }

     pub fn status(&self) -> ModuleStatus {
        ModuleStatus::kUninstantiated
    }

    pub fn instantiate_module(self: Local<'static, Module>, context: Local<'static, Context>, resolver: ModuleResolver) -> Maybe<bool> {
        Maybe::new()
    }
}

pub type ModuleResolver = fn(context: Local<'static, Context>, specifier: Local<'static, String>, referrer: Local<'static, Module>) -> Local<'static, Module>;

pub enum ModuleStatus {
    kUninstantiated,
    kInstantiating,
    kInstantiated,
    kEvaluating,
    kEvaluated,
    kErrored,
}

pub struct Maybe<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Maybe<T> {
    pub fn new() -> Self {
        Maybe {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_nothing(&self) -> bool {
        true
    }

    pub fn from_just(value: T) -> Self {
        Maybe {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_just(&self) -> Option<T> {
        None
    }
}

pub struct CreateParams {
}

impl CreateParams {
    pub fn default() -> CreateParams {
        CreateParams{}
    }
}

/**
 * The v8 JavaScript engine.
 */
pub mod v8 {
    pub struct Platform;

    /**
     * \example shell.cc
     * A simple shell that takes a list of expressions on the
     * command-line and executes them.
     */

    /**
     * \example process.cc
     */
} // namespace v8
