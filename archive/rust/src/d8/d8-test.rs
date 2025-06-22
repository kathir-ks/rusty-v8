// TODO: Add appropriate Rust crates for V8 and other C++ libraries used

// pub mod v8 {
//     // Placeholder for v8 crate definitions
// }

// Placeholder for other C++ headers

use std::{
    any::Any,
    cell::{Cell, RefCell},
    convert::TryFrom,
    mem,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

macro_rules! check_self_or_throw_fast_options {
    ($self:expr, $options:expr, $return_value:expr) => {
        if $self.is_none() {
            // TODO: ThrowError with isolate from options
            println!("This method is not defined on objects inheriting from FastCAPI.");
            return $return_value;
        }
    };
}

macro_rules! check_self_or_throw_fast {
    ($self:expr, $receiver:expr, $return_value:expr) => {
        if $self.is_none() {
            // TODO: ThrowError with isolate from receiver
            println!("This method is not defined on objects inheriting from FastCAPI.");
            return $return_value;
        }
    };
}

macro_rules! check_self_or_throw_slow {
    ($self:expr, $info:expr) => {
        if $self.is_none() {
            // TODO: ThrowError with isolate from info
            println!("This method is not defined on objects inheriting from FastCAPI.");
            return;
        }
    };
}

// Placeholder for FastApiCallbackOptions.  Needs to be defined more completely
#[derive(Debug, Clone)]
pub struct FastApiCallbackOptions {
    isolate: IsolateRef,
}

// Mock Isolate struct and methods
#[derive(Debug, Clone)]
pub struct Isolate {
    // Add fields if needed, such as a context
}

impl Isolate {
    pub fn new() -> Self {
        Isolate { /* initialize fields */ }
    }

    pub fn throw_error(&self, message: &str) {
        // Simulate throwing an error
        eprintln!("Error: {}", message);
    }

    pub fn get_current_context(&self) -> Context {
        Context::new() // Or somehow get current context
    }
}

#[derive(Debug, Clone)]
struct IsolateRef(Rc<Isolate>);

impl IsolateRef {
    pub fn new(isolate: Isolate) -> Self {
        IsolateRef(Rc::new(isolate))
    }
    pub fn throw_error(&self, message: &str) {
        self.0.throw_error(message);
    }
    pub fn get_current_context(&self) -> Context {
        self.0.get_current_context()
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    // Fields if needed
}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

#[derive(Debug, Clone)]
pub struct HandleScope {}

impl HandleScope {
    pub fn new(isolate: &IsolateRef) -> Self {
        HandleScope {}
    }
}

// Placeholder for ContextScope
#[derive(Debug, Clone)]
pub struct ContextScope {}

impl ContextScope {
    pub fn new(context: Context) -> Self {
        ContextScope {}
    }
}

#[derive(Debug, Clone)]
pub struct Local<'a, T> {
    value: T,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Local<'a, T> {
    pub fn new(value: T) -> Self {
        Local {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a> Local<'a, Object> {
    pub fn is_uint8_array(&self) -> bool {
        // Placeholder implementation
        false
    }
    pub fn as_<U>(&self) -> Local<'a, U>
    where
        U: From<Object>,
    {
        Local::new(U::from(self.value.clone()))
    }
}

impl<'a> Deref for Local<'a, Object> {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Local<'a, Value> {
    pub fn is_number(&self) -> bool {
        // Placeholder implementation
        false
    }

    pub fn int32_value(&self, _context: Context) -> Result<i32, ()> {
        // Placeholder implementation
        Ok(0)
    }
    pub fn uint32_value(&self, _context: Context) -> Result<u32, ()> {
        // Placeholder implementation
        Ok(0)
    }
    pub fn integer_value(&self, _context: Context) -> Result<i64, ()> {
        // Placeholder implementation
        Ok(0)
    }
    pub fn number_value(&self, _context: Context) -> Result<f64, ()> {
        // Placeholder implementation
        Ok(0.0)
    }
    pub fn is_typed_array(&self) -> bool {
        // Placeholder implementation
        false
    }
    pub fn as_<U>(&self) -> Local<'a, U>
    where
        U: From<Value>,
    {
        Local::new(U::from(self.value.clone()))
    }
    pub fn is_array(&self) -> bool {
        false //Placeholder
    }
    pub fn is_undefined(&self) -> bool {
        false //Placeholder
    }
    pub fn is_boolean(&self) -> bool {
        false //Placeholder
    }

    pub fn is_big_int(&self) -> bool {
        false
    }

    pub fn boolean_value(&self, isolate: &Isolate) -> bool {
        false
    }

    pub fn is_external(&self) -> bool {
        false
    }

    pub fn is_null(&self) -> bool {
        false
    }
}

// Mock Value type
#[derive(Debug, Clone)]
pub struct Value {}

impl From<Value> for Value {
    fn from(_: Value) -> Self {
        Value {}
    }
}

#[derive(Debug, Clone)]
pub struct Number {}

impl Number {
    pub fn new(_isolate: &Isolate, _value: f64) -> Local<'static, Number> {
        Local::new(Number {})
    }
}

impl From<Value> for Number {
    fn from(_: Value) -> Self {
        Number {}
    }
}

#[derive(Debug, Clone)]
pub struct Uint8Array {}

impl Uint8Array {
    pub fn length(&self) -> usize {
        0
    }
    pub fn buffer(&self) -> Local<'static, ArrayBuffer> {
        Local::new(ArrayBuffer {})
    }
}

impl From<Object> for Uint8Array {
    fn from(_: Object) -> Self {
        Uint8Array {}
    }
}

#[derive(Debug, Clone)]
pub struct ArrayBuffer {}

impl ArrayBuffer {
    pub fn data(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
}

#[derive(Debug, Clone)]
pub struct Object {}

impl Object {
    pub fn is_int32(&self) -> bool {
        // Placeholder implementation
        false
    }
    pub fn internal_field_count(&self) -> usize {
        0
    }
    pub fn get_aligned_pointer_from_internal_field(&self, _index: usize) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
    pub fn set_accessor_property(
        &self,
        _name: Local<'static, String>,
        _getter: Local<'static, Function>,
        _setter: Local<'static, Function>,
        _attribute: PropertyAttribute,
    ) {
        // Placeholder implementation
    }
    pub fn set_aligned_pointer_in_internal_field(&self, _index: usize, _value: *mut std::ffi::c_void) {
        // Placeholder implementation
    }
    pub fn get_creation_context(&self, isolate: &Isolate) -> Result<Local<'static, Context>, ()> {
        Ok(Local::new(isolate.get_current_context()))
    }
    pub fn get(&self, _context: Context, _name: Local<'static, String>) -> Result<Local<'static, Value>, ()> {
        Ok(Local::new(Value {}))
    }
    pub fn is_uint8_array(&self) -> bool {
        false
    }
    pub fn as_<U>(&self) -> Local<'static, U>
    where
        U: From<Object>,
    {
        Local::new(U::from(self.clone()))
    }
}

impl From<Object> for Object {
    fn from(_: Object) -> Self {
        Object {}
    }
}

#[derive(Debug, Clone)]
pub struct String {}

impl String {
    pub fn new_from_utf8_literal(_isolate: &Isolate, _value: &str) -> Local<'static, String> {
        Local::new(String {})
    }
    pub fn new_from_utf8(_isolate: &Isolate, _value: &str) -> Result<Local<'static, String>, ()> {
        Ok(Local::new(String {}))
    }
}

#[derive(Debug, Clone)]
pub struct Function {}

#[derive(Debug, Clone)]
pub struct FunctionTemplate {}

impl FunctionTemplate {
    pub fn new(
        _isolate: &Isolate,
        _callback: fn(&FunctionCallbackInfo<Value>),
    ) -> Local<'static, FunctionTemplate> {
        Local::new(FunctionTemplate {})
    }
    pub fn new_with_c_function_overloads(
        _isolate: &Isolate,
        _callback: fn(&FunctionCallbackInfo<Value>),
        _value: Local<'static, Value>,
        _signature: Local<'static, Signature>,
        _i: i32,
        _behavior: ConstructorBehavior,
        _k: SideEffectType,
        _overloads: OverloadList,
    ) -> Local<'static, FunctionTemplate> {
        Local::new(FunctionTemplate {})
    }
    pub fn get_function(&self, _context: Result<Local<'static, Context>, ()>) -> Result<Local<'static, Function>, ()> {
        Ok(Local::new(Function {}))
    }
}

#[derive(Debug, Clone)]
pub struct Signature {}

impl Signature {
    pub fn new(_isolate: &Isolate, _templ: Local<'static, FunctionTemplate>) -> Local<'static, Signature> {
        Local::new(Signature {})
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCallbackInfo<'a, T> {
    isolate: IsolateRef,
    this: Option<Local<'a, Object>>,
    args: Vec<Local<'a, Value>>,
    return_value: RefCell<Local<'a, Value>>,
}

impl<'a, T> FunctionCallbackInfo<'a, T> {
    pub fn new(isolate: IsolateRef, this: Option<Local<'a, Object>>, args: Vec<Local<'a, Value>>) -> Self {
        FunctionCallbackInfo {
            isolate,
            this,
            args,
            return_value: RefCell::new(Local::new(Value {})),
        }
    }

    pub fn get_isolate(&self) -> IsolateRef {
        self.isolate.clone()
    }

    pub fn this(&self) -> Option<Local<'a, Object>> {
        self.this.clone()
    }

    pub fn args(&self) -> &[Local<'a, Value>] {
        &self.args
    }

    pub fn length(&self) -> usize {
        self.args.len()
    }

    pub fn get_return_value(&self) -> Local<'a, Value> {
        self.return_value.borrow().clone()
    }

    pub fn get_return_value_mut(&self) -> std::cell::RefMut<'_, Local<'a, Value>> {
        self.return_value.borrow_mut()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnValue<'a, T> {
    value: Local<'a, T>,
}

impl<'a, T> ReturnValue<'a, T> {
    pub fn set(&mut self, value: Local<'a, T>) {
        self.value = value;
    }
}

#[derive(Debug, Clone)]
pub struct FastOneByteString {
    data: *const u8,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyAttribute {
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorBehavior {
    kThrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideEffectType {
    kHasSideEffect,
    kHasNoSideEffect,
}

#[derive(Debug, Clone)]
pub struct CFunction {
    function: fn(&FunctionCallbackInfo<Value>),
}

impl CFunction {
    pub fn make(function: fn(&FunctionCallbackInfo<Value>)) -> Self {
        CFunction { function }
    }
}

#[derive(Debug, Clone)]
pub struct OverloadList {
    overloads: Vec<CFunction>,
    size: usize,
}

impl OverloadList {
    pub fn new(overloads: Vec<CFunction>, size: usize) -> Self {
        OverloadList { overloads, size }
    }
}

#[derive(Debug, Clone)]
pub struct CFunctionBuilder {
    function: Option<fn(&FunctionCallbackInfo<Value>)>,
    int64_representation: CFunctionInfoInt64Representation,
    args: Vec<(usize, CTypeInfoFlags)>,
    patch: Option<fn(AnyCType, AnyCType, AnyCType, AnyCType) -> AnyCType>,
}

impl CFunctionBuilder {
    pub fn new() -> Self {
        CFunctionBuilder {
            function: None,
            int64_representation: CFunctionInfoInt64Representation::Default,
            args: Vec::new(),
            patch: None,
        }
    }

    pub fn fn_mut(&mut self, function: fn(&FunctionCallbackInfo<Value>)) -> &mut Self {
        self.function = Some(function);
        self
    }

    pub fn fn_(mut self, function: fn(&FunctionCallbackInfo<Value>)) -> Self {
        self.function = Some(function);
        self
    }

    pub fn build(self) -> CFunction {
        CFunction {
            function: self.function.unwrap(),
        }
    }

    pub fn arg<const INDEX: usize, const FLAGS: CTypeInfoFlags>(mut self) -> Self {
        self.args.push((INDEX, FLAGS));
        self
    }

    pub fn patch(mut self, patch: fn(AnyCType, AnyCType, AnyCType, AnyCType) -> AnyCType) -> Self {
        self.patch = Some(patch);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CFunctionInfoInt64Representation {
    Default,
    kBigInt,
}

#[derive(Debug, Clone)]
pub struct CFunctionInfo {}

impl CFunctionInfo {
    pub struct Builder {
        int64_representation: CFunctionInfoInt64Representation,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                int64_representation: CFunctionInfoInt64Representation::Default,
            }
        }

        pub fn int64_representation(mut self, representation: CFunctionInfoInt64Representation) -> Self {
            self.int64_representation = representation;
            self
        }

        pub fn build(self) -> CFunctionInfo {
            CFunctionInfo {}
        }
    }

    pub const fn new() -> Self {
        CFunctionInfo {}
    }
}

// Placeholder for CTypeInfo::Flags enum
type CTypeInfoFlags = u32;

pub mod CTypeInfo {
    pub struct Flags;

    impl Flags {
        pub const kEnforceRangeBit: u32 = 1 << 0;
        pub const kClampBit: u32 = 1 << 1;
    }
}

#[derive(Debug, Clone)]
pub struct AnyCType {
    pub object_value: Object,
    pub string_value: *const FastOneByteString,
    pub options_value: *mut FastApiCallbackOptions,
    pub int32_value: i32,
    pub uint32_value: u32,
    pub int64_value: i64,
    pub uint64_value: u64,
    pub float_value: f32,
    pub double_value: f64,
    pub bool_value: bool,
    pub sequence_value: Object, //Local<Object>,
}

// Mock BigInt
#[derive(Debug, Clone)]
pub struct BigInt {}

impl BigInt {
    pub fn new(_isolate: &Isolate, _value: i64) -> Local<'static, BigInt> {
        Local::new(BigInt {})
    }

    pub fn new_from_unsigned(_isolate: &Isolate, _value: u64) -> Local<'static, BigInt> {
        Local::new(BigInt {})
    }

    pub fn int64_value(&self) -> i64 {
        0
    }

    pub fn uint64_value(&self) -> u64 {
        0
    }
}

// Mock TypedArray
#[derive(Debug, Clone)]
pub struct TypedArray {}

impl TypedArray {
    pub fn buffer(&self) -> Local<'static, ArrayBuffer> {
        Local::new(ArrayBuffer {})
    }
    pub fn byte_length(&self) -> usize {
        0
    }
    pub fn length(&self) -> u32 {
        0
    }
    pub fn is_uint8_array(&self) -> bool {
        false
    }
    pub fn is_int32_array(&self) -> bool {
        false
    }
    pub fn is_uint32_array(&self) -> bool {
        false
    }
    pub fn is_big_int64_array(&self) -> bool {
        false
    }
    pub fn is_big_uint64_array(&self) -> bool {
        false
    }
    pub fn is_float32_array(&self) -> bool {
        false
    }
    pub fn is_float64_array(&self) -> bool {
        false
    }
}

impl From<Value> for TypedArray {
    fn from(_: Value) -> Self {
        TypedArray {}
    }
}

#[derive(Debug, Clone)]
pub struct WasmMemoryObject {}

impl WasmMemoryObject {
    pub fn buffer(&self) -> Local<'static, ArrayBuffer> {
        Local::new(ArrayBuffer {})
    }
}

impl From<Value> for WasmMemoryObject {
    fn from(_: Value) -> Self {
        WasmMemoryObject {}
    }
}

// Mock External
#[derive(Debug, Clone)]
pub struct External {}

impl External {
    pub fn new(_isolate: &Isolate, _value: *mut std::ffi::c_void) -> Local<'static, External> {
        Local::new(External {})
    }
    pub fn value(&self) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
}

// Mock Array
#[derive(Debug, Clone)]
pub struct Array {}

impl Array {
    pub fn length(&self) -> u32 {
        0
    }
    pub fn get(&self, _context: Context, _index: Local<'static, Integer>) -> Result<Local<'static, Value>, ()> {
        Ok(Local::new(Value {}))
    }
}

impl From<Value> for Array {
    fn from(_: Value) -> Self {
        Array {}
    }
}

// Mock Integer
#[derive(Debug, Clone)]
pub struct Integer {}

impl Integer {
    pub fn new_from_unsigned(_isolate: &Isolate, _value: u32) -> Local<'static, Integer> {
        Local::new(Integer {})
    }
}

#[derive(Debug, Clone)]
struct FastCApiObject {
    fast_call_count_: Cell<i32>,
    slow_call_count_: Cell<i32>,
    attribute_value_: Cell<i32>,
    supports_fp_params_: bool,
}

impl FastCApiObject {
    fn new() -> Self {
        FastCApiObject {
            fast_call_count_: Cell::new(0),
            slow_call_count_: Cell::new(0),
            attribute_value_: Cell::new(0),
            supports_fp_params_: cfg!(feature = "V8_ENABLE_FP_PARAMS_IN_C_LINKAGE"),
        }
    }

    fn instance() -> &'static FastCApiObject {
        thread_local! {
            static FAST_C_API_OBJECT: FastCApiObject = FastCApiObject::new();
        }
        FAST_C_API_OBJECT.with(|o| o)
    }

    #[cfg(feature = "V8_USE_SIMULATOR_WITH_GENERIC_C_CALLS")]
    fn throw_no_fallback_fast_callback_patch(_receiver: AnyCType) -> AnyCType {
        let mut ret = AnyCType {
            object_value: Object {},
            string_value: std::ptr::null(),
            options_value: std::ptr::null_mut(),
            int32_value: 0,
            uint32_value: 0,
            int64_value: 0,
            uint64_value: 0,
            float_value: 0.0,
            double_value: 0.0,
            bool_value: false,
            sequence_value: Object {},
        };
        Self::throw_no_fallback_fast_callback(Local::new(ret.object_value.clone()));
        ret
    }

    fn throw_no_fallback_fast_callback(receiver: Local<'static, Object>) -> i32 {
        let self_ = Self::unwrap_object(receiver);
        let self_ref = match self_ {
            Some(s) => s,
            None => Self::instance(),
        };
        self_ref.fast_call_count_.set(self_ref.fast_call_count_.get() + 1);

        let isolate = IsolateRef::new(Isolate::new()); // TODO: GetCurrentIsolate?
        let handle_scope = HandleScope::new(&isolate);
        let context = isolate.get_current_context();
        let context_scope = ContextScope::new(context);
        isolate.throw_error("Exception from fast callback");

        0
    }

    fn throw_fallback_slow_callback(info: &FunctionCallbackInfo<Value>) {
        // TODO: ValidateCallbackInfo
        if let Some(this) = info.this() {
            let self_ = Self::unwrap_object(this);
            check_self_or_throw_slow!(self_, info);
            if let Some(s) = self_ {
                s.slow_call_count_.set(s.slow_call_count_.get() + 1);
            }

            info.get_isolate().throw_error("Exception from slow callback");
        } else {
            info.get_isolate().throw_error("No 'this' object found.");
        }
    }

    #[cfg(feature = "V8_USE_SIMULATOR_WITH_GENERIC_C_CALLS")]
    fn copy_string_fast_callback_patch(
        receiver: AnyCType,
        source: AnyCType,
        out: AnyCType,
        options: AnyCType,
    ) -> AnyCType {
        let mut ret = AnyCType {
            object_value: Object {},
            string_value: std::ptr::null(),
            options_value: std::ptr::null_mut(),
            int32_value: 0,
            uint32_value: 0,
            int64_value: 0,
            uint64_value: 0,
            float_value: 0.0,
            double_value: 0.0,
            bool_value: false,
            sequence_value: Object {},
        };
        // TODO: Implement the correct method calls
        // Self::copy_string_fast_callback(
        //     receiver.object_value,
        //     unsafe { &*source.string_value },
        //     out.object_value,
        //     unsafe { &mut *options.options_value },
        // );
        ret
    }

    fn copy_string_fast_callback(
        receiver: Local<'static, Object>,
        source: &FastOneByteString,
        out: Local<'static, Object>,
        options: &mut FastApiCallbackOptions,
    ) {
        let self_ = Self::unwrap_object(receiver).unwrap();
        self_.fast_call_count_.set(self_.fast_call_count_.get() + 1);

        let handle_scope = HandleScope::new(&options.isolate);

        if !out.is_uint8_array() {
            options
                .isolate
                .throw_error("Invalid parameter, the second parameter has to be a a Uint8Array.");
            return;
        }

        let array = out.as_::<Uint8Array>();

        if array.length() < source.length {
            options
                .isolate
                .throw_error("Invalid parameter, destination array is too small.");
            return;
        }

        let memory = array.buffer().data(); // TODO: Should actually get the u8 pointer here
                                             // let memory = unsafe { array.buffer().data() as *mut u8 };
        unsafe {
            std::ptr::copy_nonoverlapping(source.data, memory, source.length);
        }
    }

    fn copy_string_slow_callback(info: &FunctionCallbackInfo<Value>) {
        if let Some(this) = info.this() {
            let self_ = Self::unwrap_object(this);
            check_self_or_throw_slow!(self_, info);
            if let Some(s) = self_ {
                s.slow_call_count_.set(s.slow_call_count_.get() + 1);
            }
        } else {
            info.get_isolate().throw_error("No 'this' object found.");
        }
    }

    #[cfg(feature = "V8_USE_SIMULATOR_WITH_GENERIC_C_CALLS")]
    fn add_all_fast_callback_patch(
        receiver: AnyCType,
        arg_i32: AnyCType,
        arg_u32: AnyCType,
        arg_i64: AnyCType,
        arg_u64: AnyCType,
        arg_f32: AnyCType,
        arg_f64: AnyCType,
        options: AnyCType,
    ) -> AnyCType {
        let mut ret = AnyCType {
            object_value: Object {},
            string_value: std::ptr::null(),
            options_value: std::ptr::null_mut(),
            int32_value: 0,
            uint32_value: 0,
            int64_value: 0,
            uint64_value: 0,
            float_value: 0.0,
            double_value: 0.0,
            bool_value: false,
            sequence_value: Object {},
        };
        // TODO: Implement the correct method calls
        // ret.double_value = Self::add_all_fast_callback(
        //     receiver.object_value,
        //     arg_i32.int32_value,
        //     arg_u32.uint32_value,
        //     arg_i64.int64_value,
        //     arg_u64.uint64_value,
        //     arg_f32.float_value,
        //     arg_f64.double_value,
        //     unsafe { &mut *options.options_value },
        // );
        ret
    }

    fn add_all_fast_callback(
        receiver: Local<'static, Object>,
        arg_i32: i32,
        arg_u32: u32,
        arg_i64: i64,
        arg_u64: u64,
        arg_f32: f32,
        arg_f64: f64,
        options: &mut FastApiCallbackOptions,
    ) -> f64 {
        let self_ = Self::unwrap_object(receiver);
        check_self_or_throw_fast_options!(self_, options, 0.0);
        if let Some(s) = self_ {
            s.fast_call_count_.set(s.fast_call_count_.get() + 1);
        }

        (arg_i32 as f64) + (arg_u32 as f64) + (arg_i64 as f64) + (arg_u64 as f64)
            + (arg_f32 as f64)
            + arg_f64
    }

    #[cfg(feature = "V8_USE_SIMULATOR_WITH_GENERIC_C_CALLS")]
    fn add_all_fast_callback_no_options_patch(
        receiver: AnyCType,
        arg_i32: AnyCType,
        arg_u32: AnyCType,
        arg_i64: AnyCType,
        arg_u64: AnyCType,
        arg_f32: AnyCType,
        arg_f64: AnyCType,
    ) -> AnyCType {
        let mut ret = AnyCType {
            object_value: Object {},
            string_value: std::ptr::null(),
            options_value: std::ptr::null_mut(),
            int32_value: 0,
            uint32_value: 0,
            int64_value: 0,
            uint64_value: 0,
            float_value: 0.0,
            double_value: 0.0,
            bool_value: false,
            sequence_value: Object {},
        };
        // TODO: Implement the correct method calls
        // ret.double_value = Self::add_all_fast_callback_no_options(
        //     receiver.object_value,
        //     arg_i32.int32_value,
        //     arg_u32.uint32_value,
        //     arg_i64.int64_value,
        //     arg_u64.uint64_value,
        //     arg_f32.float_value,
        //     arg_f64.double_value,
        // );
        ret
    }

    fn add_all_fast_callback_no_options(
        receiver: Local<'static, Object>,
        arg_i32: i32,
        arg_u32: u32,
        arg_i64: i64,
        arg_u64: u64,
        arg_f32: f32,
        arg_f64: f64,
    ) -> f64 {
        let self_ = Self::unwrap_object(receiver);
        let self_ref = match self_ {
            Some(s) => s,
            None => Self::instance(),
        };
        self_ref.fast_call_count_.set(self_ref.fast_call_count_.get() + 1);

        (arg_i32 as f64) + (arg_u32 as f64) + (arg_i64 as f64) + (arg_u64 as f64)
            + (arg_f32 as f64)
            + arg_f64
    }

    fn add_all_slow_callback(info: &FunctionCallbackInfo<Value>) {
        let isolate = info.get_isolate();

        if let Some(this) = info.this() {
            let self_ = Self::unwrap_object(this);
            check_self_or_throw_slow!(self_, info);
            if let Some(s) = self_ {
                s.slow_call_count_.set(s.slow_call_count_.get() + 1);
            }

            let handle_scope = HandleScope::new(&isolate);

            let mut sum = 0.0;
            if info.length() > 0 && info.args()[0].is_number() {
                sum += info.args()[0].int32_value(isolate.get_current_context()).unwrap() as f64;
            }
            if info.length() > 1 && info.args()[1].is_number() {
                sum += info.args()[1].uint32_value(isolate.get_current_context()).unwrap() as f64;
            }
            if info.length() > 2 && info.args()[2].is_number() {
                sum += info.args()[2].integer_value(isolate.get_current_context()).unwrap() as f64;
            }
            if info.length() > 3 && info.args()[3].is_number() {
                sum += info.args()[3].integer_value(isolate.get_current_context()).unwrap() as f64;
            }
            if info.length() > 4 && info.args()[4].is_number() {
                sum += info.args()[4].number_value(isolate.get_current_context()).unwrap();
            } else {
                sum += f64::NAN;
            }
            if info.length() >