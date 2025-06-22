// src/d8/async_hooks_wrapper.rs

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use lazy_static::lazy_static;

// Mock v8-rs
mod v8 {
    use std::any::Any;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Mutex;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PromiseHookType {
        kInit,
        kBefore,
        kAfter,
        kResolve,
    }

    pub type FunctionCallback = fn(FunctionCallbackInfo);

    #[derive(Debug)]
    pub struct Value {}

    impl Value {
        pub fn is_object(&self) -> bool {
            false
        }
        pub fn is_function(&self) -> bool {
            false
        }
        pub fn is_promise(&self) -> bool {
            false
        }
        pub fn is_undefined(&self) -> bool {
            false
        }
        pub fn as_promise(&self) -> Local<Promise> {
          Local {
            handle: Rc::new(RefCell::new(Promise {
              private_data: HashMap::new(),
            })),
          }
        }
        pub fn as_function(&self) -> Local<Function> {
          Local {
            handle: Rc::new(RefCell::new(Function {})),
          }
        }
    }

    #[derive(Debug)]
    pub struct Function {}

    impl Function {
        pub fn call(&self, _context: Local<Context>, _receiver: Local<Value>, _argc: i32, _argv: &[Local<Value>]) -> Result<(), String>{
            Ok(())
        }
        pub fn is_empty(&self) -> bool {
          true
        }
    }

    #[derive(Debug, Clone)]
    pub struct Local<T> {
        pub handle: Rc<RefCell<T>>,
    }

    impl<T> Local<T> {
      pub fn new(val: T) -> Self {
          Local { handle: Rc::new(RefCell::new(val)) }
      }
    }

    #[derive(Debug)]
    pub struct Object {
        pub internal_fields: RefCell<Vec<Option<Box<dyn Any>>>>,
    }

    impl Object {
        pub fn get_internal_field(&self, index: usize) -> Option<&RefCell<Option<Box<dyn Any>>>> {
            self.internal_fields.borrow().get(index).map(|x| &self.internal_fields)
        }
        pub fn set_internal_field<T: Any>(&self, index: usize, value: T) {
            let mut internal_fields = self.internal_fields.borrow_mut();
            if internal_fields.len() <= index {
                internal_fields.resize_with(index + 1, || None);
            }
            internal_fields[index] = Some(Box::new(value));
        }
        pub fn has_private(&self, _context: Local<Context>, _private: Local<Private>) -> Result<bool,String> {
          Ok(false)
        }
        pub fn get(&self, _context: Local<Context>, _string: Local<String>) -> Result<Local<Value>,String> {
          Ok(Local {handle: Rc::new(RefCell::new(Value {}))})
        }
        pub fn set_private(&self, _context: Local<Context>, _private: Local<Private>, _value: Local<Value>) -> Result<bool,String> {
          Ok(true)
        }
    }

    #[derive(Debug)]
    pub struct Promise {
      private_data: HashMap<String, Rc<RefCell<Value>>>,
    }

    impl Promise {
      pub fn get_private(&self, _context: Local<Context>, _private: Local<Private>) -> Result<Local<Value>, String> {
        Ok(Local {handle: Rc::new(RefCell::new(Value {}))})
      }
      pub fn set_private(&mut self, _context: Local<Context>, private: Local<Private>, value: Local<Value>) -> Result<bool, String> {
          self.private_data.insert(format!("{:p}", Rc::as_ptr(&private.handle)), value.handle.clone());
          Ok(true)
      }
      pub fn has_private(&self, _context: Local<Context>, private: Local<Private>) -> Result<bool, String> {
          Ok(self.private_data.contains_key(&format!("{:p}", Rc::as_ptr(&private.handle))))
      }
    }

    #[derive(Debug, Clone)]
    pub struct Isolate {
        pub promise_hook: Rc<RefCell<Option<PromiseHook>>>,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                promise_hook: Rc::new(RefCell::new(None)),
            }
        }
        pub fn set_promise_hook(&self, hook: Option<PromiseHook>) {
            *self.promise_hook.borrow_mut() = hook;
        }
        pub fn get_current_context(&self) -> Local<Context> {
            Local {handle: Rc::new(RefCell::new(Context {}))}
        }
        pub fn throw_error(&self, _error: &str) {}
    }

    pub type PromiseHook = fn(PromiseHookType, Local<Promise>, Local<Value>);

    #[derive(Debug, Clone)]
    pub struct Context {}

    #[derive(Debug)]
    pub struct FunctionCallbackInfo {
        pub isolate: Rc<Isolate>,
        pub this: Rc<RefCell<Object>>,
        pub args: Vec<Local<Value>>,
    }

    impl FunctionCallbackInfo {
        pub fn get_isolate(&self) -> Rc<Isolate> {
            self.isolate.clone()
        }

        pub fn this(&self) -> Local<Object> {
            Local {handle: self.this.clone()}
        }

        pub fn get(&self, index: usize) -> Local<Value> {
            self.args[index].clone()
        }

        pub fn length(&self) -> usize {
            self.args.len()
        }
    }

    #[derive(Debug)]
    pub struct FunctionTemplate {}

    impl FunctionTemplate {
        pub fn new(_isolate: &Isolate) -> Local<FunctionTemplate> {
            Local {handle: Rc::new(RefCell::new(FunctionTemplate {}))}
        }
        pub fn set_class_name(&self, _name: Local<String>) {}
        pub fn instance_template(&self) -> Local<ObjectTemplate> {
            Local {handle: Rc::new(RefCell::new(ObjectTemplate {}))}
        }
        pub fn new_instance(&self, _context: Local<Context>) -> Result<Local<Object>, String> {
          Ok(Local {handle: Rc::new(RefCell::new(Object {internal_fields: RefCell::new(Vec::new())}))})
        }
    }

    #[derive(Debug)]
    pub struct ObjectTemplate {}

    impl ObjectTemplate {
        pub fn set_internal_field_count(&self, _count: usize) {}
        pub fn set(&self, _isolate: &Isolate, _name: &str, _value: Local<FunctionTemplate>) {}
    }

    #[derive(Debug, Clone)]
    pub struct String {}

    impl String {
        pub fn new_from_utf8_literal(_isolate: &Isolate, _literal: &str) -> Local<String> {
            Local {handle: Rc::new(RefCell::new(String {}))}
        }
    }

    #[derive(Debug, Clone)]
    pub struct Integer {}

    impl Integer {
        pub fn new(_isolate: &Isolate, _value: i32) -> Local<Integer> {
            Local {handle: Rc::new(RefCell::new(Integer {}))}
        }
        pub fn value(&self) -> i64 {
          0
        }
    }

    #[derive(Debug, Clone)]
    pub struct Private {}

    impl Private {
        pub fn new(_isolate: &Isolate) -> Local<Private> {
            Local {handle: Rc::new(RefCell::new(Private {}))}
        }
    }

    pub struct HandleScope<'a> {
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> HandleScope<'a> {
        pub fn new(_isolate: &Isolate) -> Self {
            HandleScope {
                _marker: std::marker::PhantomData,
            }
        }
    }

    #[derive(Debug)]
    pub struct TryCatch {}

    impl TryCatch {
      pub fn new(_isolate: &Isolate) -> Self {
        TryCatch {}
      }
      pub fn set_verbose(&self, _verbose: bool) {}
      pub fn has_caught(&self) -> bool {
        false
      }
    }

    pub fn undefined(_isolate: &Isolate) -> Local<Value> {
      Local {handle: Rc::new(RefCell::new(Value {}))}
    }
}

type AsyncId = u32;

#[derive(Debug, Clone, Copy)]
struct AsyncContext {
    execution_async_id: AsyncId,
    trigger_async_id: AsyncId,
}

#[derive(Debug)]
struct AsyncHooksWrap {
    isolate_: Rc<v8::Isolate>,
    enabled_: RefCell<bool>,
    init_function_: RefCell<Option<v8::Local<v8::Function>>>,
    before_function_: RefCell<Option<v8::Local<v8::Function>>>,
    after_function_: RefCell<Option<v8::Local<v8::Function>>>,
    promise_resolve_function_: RefCell<Option<v8::Local<v8::Function>>>,
}

impl AsyncHooksWrap {
    fn new(isolate: Rc<v8::Isolate>) -> Self {
        AsyncHooksWrap {
            isolate_: isolate,
            enabled_: RefCell::new(false),
            init_function_: RefCell::new(None),
            before_function_: RefCell::new(None),
            after_function_: RefCell::new(None),
            promise_resolve_function_: RefCell::new(None),
        }
    }

    fn enable(&self) {
        *self.enabled_.borrow_mut() = true;
    }

    fn disable(&self) {
        *self.enabled_.borrow_mut() = false;
    }

    fn is_enabled(&self) -> bool {
        *self.enabled_.borrow()
    }

    fn init_function(&self) -> Option<v8::Local<v8::Function>> {
        self.init_function_.borrow().clone()
    }

    fn set_init_function(&self, value: v8::Local<v8::Function>) {
        *self.init_function_.borrow_mut() = Some(value);
    }

    fn before_function(&self) -> Option<v8::Local<v8::Function>> {
        self.before_function_.borrow().clone()
    }

    fn set_before_function(&self, value: v8::Local<v8::Function>) {
        *self.before_function_.borrow_mut() = Some(value);
    }

    fn after_function(&self) -> Option<v8::Local<v8::Function>> {
        self.after_function_.borrow().clone()
    }

    fn set_after_function(&self, value: v8::Local<v8::Function>) {
        *self.after_function_.borrow_mut() = Some(value);
    }

    fn promise_resolve_function(&self) -> Option<v8::Local<v8::Function>> {
        self.promise_resolve_function_.borrow().clone()
    }

    fn set_promise_resolve_function(&self, value: v8::Local<v8::Function>) {
        *self.promise_resolve_function_.borrow_mut() = Some(value);
    }
}

#[derive(Debug)]
pub struct AsyncHooks {
    v8_isolate_: Rc<v8::Isolate>,
    async_contexts: Mutex<VecDeque<AsyncContext>>,
    current_async_id: AtomicU32,
    async_hook_ctor: RefCell<Option<v8::Local<v8::FunctionTemplate>>>,
    async_hooks_templ: RefCell<Option<v8::Local<v8::ObjectTemplate>>>,
    async_id_symbol: RefCell<Option<v8::Local<v8::Private>>>,
    trigger_id_symbol: RefCell<Option<v8::Local<v8::Private>>>,
    async_wraps_: Mutex<Vec<Rc<AsyncHooksWrap>>>,
    skip_after_termination_: RefCell<bool>, // TODO: make this atomic bool
}

impl AsyncHooks {
    pub fn new(v8_isolate: Rc<v8::Isolate>) -> Self {
        let mut async_contexts = VecDeque::new();
        async_contexts.push_back(AsyncContext {
            execution_async_id: 1,
            trigger_async_id: 0,
        });

        let async_hook_ctor: RefCell<Option<v8::Local<v8::FunctionTemplate>>> = RefCell::new(None);
        let async_hooks_templ: RefCell<Option<v8::Local<v8::ObjectTemplate>>> = RefCell::new(None);
        let async_id_symbol: RefCell<Option<v8::Local<v8::Private>>> = RefCell::new(None);
        let trigger_id_symbol: RefCell<Option<v8::Local<v8::Private>>> = RefCell::new(None);
        let async_wraps_: Mutex<Vec<Rc<AsyncHooksWrap>>> = Mutex::new(Vec::new());
        let skip_after_termination_: RefCell<bool> = RefCell::new(false);

        let mut async_hooks = AsyncHooks {
            v8_isolate_: v8_isolate.clone(),
            async_contexts: Mutex::new(async_contexts),
            current_async_id: AtomicU32::new(1),
            async_hook_ctor,
            async_hooks_templ,
            async_id_symbol,
            trigger_id_symbol,
            async_wraps_,
            skip_after_termination_,
        };

        let isolate = &v8_isolate;
        let mut handle_scope = v8::HandleScope::new(isolate);

        let ft = v8::FunctionTemplate::new(isolate);
        ft.set_class_name(v8::String::new_from_utf8_literal(isolate, "AsyncHook"));
        *async_hooks.async_hook_ctor.borrow_mut() = Some(v8::Local { handle: Rc::new(RefCell::new(ft)) });

        let instance_template = async_hooks.async_hook_ctor.borrow().as_ref().unwrap().handle.borrow().instance_template();
        instance_template.handle.borrow().set_internal_field_count(1);

        let enable_hook_template = v8::FunctionTemplate::new(isolate);
        instance_template.handle.borrow().set(isolate, "enable", enable_hook_template);

        let disable_hook_template = v8::FunctionTemplate::new(isolate);
        instance_template.handle.borrow().set(isolate, "disable", disable_hook_template);

        *async_hooks.async_hooks_templ.borrow_mut() = Some(v8::Local { handle: Rc::new(RefCell::new(instance_template)) });

        *async_hooks.async_id_symbol.borrow_mut() = Some(v8::Private::new(isolate));
        *async_hooks.trigger_id_symbol.borrow_mut() = Some(v8::Private::new(isolate));

        v8_isolate.set_promise_hook(Some(Self::shell_promise_hook));

        async_hooks
    }

    fn get_execution_async_id(&self) -> AsyncId {
        self.async_contexts
            .lock()
            .unwrap()
            .back()
            .map(|ctx| ctx.execution_async_id)
            .unwrap_or(0) // Or some other default value if the queue is empty
    }

    fn get_trigger_async_id(&self) -> AsyncId {
        self.async_contexts
            .lock()
            .unwrap()
            .back()
            .map(|ctx| ctx.trigger_async_id)
            .unwrap_or(0) // Or some other default value if the queue is empty
    }

    fn create_hook(
        &self,
        info: &v8::FunctionCallbackInfo,
    ) -> Result<v8::Local<v8::Object>, String> {
        let isolate = info.get_isolate();
        let mut handle_scope = v8::HandleScope::new(&isolate);

        // if isolate.is_execution_terminating() {
        //     return Ok(None);
        // }

        let current_context = isolate.get_current_context();

        if info.length() != 1 || !info.get(0).is_object() {
            isolate.throw_error("Invalid arguments passed to createHook");
            return Err("Invalid arguments passed to createHook".to_string());
        }

        let wrap = Rc::new(AsyncHooksWrap::new(isolate.clone()));

        let fn_obj = info.get(0).as_object();

        // TODO: Implement TryCatch
        // let mut try_catch = v8::TryCatch::new(&isolate);
        macro_rules! set_hook_fn {
            ($name:ident) => {
                let name##_maybe_func = fn_obj.handle.borrow().get(current_context.clone(), v8::String::new_from_utf8_literal(&isolate, stringify!($name))).map_err(|e| e.to_string())?;
                let name##_func = name##_maybe_func;
                if name##_func.handle.borrow().is_function() {
                    let func = name##_func.as_function();
                    wrap.set_##name##_function(func);
                } else {
                    // try_catch.re_throw();
                    return Err(format!("Failed to set {} function", stringify!($name)));
                }
            };
        }

        set_hook_fn!(init);
        set_hook_fn!(before);
        set_hook_fn!(after);
        set_hook_fn!(promiseResolve);

        let obj_template = self.async_hooks_templ.borrow().as_ref().unwrap().handle.borrow();
        let obj = obj_template.new_instance(current_context.clone()).map_err(|e| e.to_string())?;
        // TODO: Implement Managed
        // let managed = i::Managed<AsyncHooksWrap>::from(
        //     reinterpret_cast<i::Isolate*>(v8_isolate),
        //     std::mem::size_of::<AsyncHooksWrap>(),
        //     wrap,
        // );
        obj.handle.borrow().set_internal_field(0, wrap.clone());

        self.async_wraps_.lock().unwrap().push(wrap.clone());

        Ok(obj)
    }

    fn shell_promise_hook(
        type_: v8::PromiseHookType,
        promise: v8::Local<v8::Promise>,
        parent: v8::Local<v8::Value>,
    ) {
        //println!("Promise hook: {:?} {:?} {:?}", type_, promise, parent);
        let isolate = promise.handle.borrow().get_private(_isolate, _private);
        let hooks = PER_ISOLATE_DATA.lock().unwrap().get(&format!("{:p}", Rc::as_ptr(&isolate))).unwrap().get_async_hooks();
        //println!("hooks: {:?}", hooks);
        let hooks = hooks.borrow();

        if isolate.promise_hook.borrow().is_none() {
            return;
        }

        //if v8_isolate.is_execution_terminating() || hooks.skip_after_termination_.load(Ordering::Relaxed) {
        //    hooks.skip_after_termination_.store(true, Ordering::Relaxed);
        //    return;
        //}
        // let i_isolate = unsafe { std::mem::transmute::<&v8::Isolate, &i::Isolate>(v8_isolate) };

        let mut handle_scope = v8::HandleScope::new(&isolate);
        // let mut exception: Option<i::DirectHandle<i::Object>> = None;

        // Keep track of any previously thrown exception.
        // if i_isolate.has_exception() {
        //     exception = Some(i::DirectHandle::new(i_isolate.exception(), i_isolate));
        // }

        {
            let mut try_catch = v8::TryCatch::new(&isolate);
            try_catch.set_verbose(true);

            let current_context = isolate.get_current_context();
            // assert!(!current_context.is_empty());
            let trigger_id_symbol = hooks.trigger_id_symbol.borrow();
            let async_id_symbol = hooks.async_id_symbol.borrow();

            match type_ {
                v8::PromiseHookType::kInit => {
                    let async_id = hooks.current_async_id.fetch_add(1, Ordering::Relaxed);
                    let async_id_local = v8::Integer::new(&isolate, async_id as i32);
                    //println!("async_id: {:?}", async_id);

                    if promise.handle.borrow().has_private(current_context.clone(), async_id_symbol.as_ref().unwrap().clone()).unwrap() {
                        panic!("Promise already has async_id");
                    }
                    promise.handle.borrow_mut().set_private(current_context.clone(), async_id_symbol.as_ref().unwrap().clone(), v8::Local {handle: Rc::new(RefCell::new(v8::Value {}))}).unwrap();

                    if parent.is_promise() {
                        let parent_promise = parent.as_promise();
                        //let parent_async_id =
                        //    parent_promise
                        //        .get_private(current_context.clone(), hooks.async_id_symbol.as_ref().unwrap().clone())
                        //        .unwrap();
                        promise.handle.borrow_mut().set_private(current_context.clone(), trigger_id_symbol.as_ref().unwrap().clone(), v8::Local {handle: Rc::new(RefCell::new(v8::Value {}))}).unwrap();
                    } else {
                        assert!(parent.is_undefined());
                        promise.handle.borrow_mut().set_private(current_context.clone(), trigger_id_symbol.as_ref().unwrap().clone(), v8::Local {handle: Rc::new(RefCell::new(v8::Value {}))}).unwrap();
                    }
                }
                v8::PromiseHookType::kBefore => {
                    let ctx = AsyncContext {
                        execution_async_id: 1,
                        trigger_async_id: 0,
                    };
                    hooks.async_contexts.lock().unwrap().push_back(ctx);
                }
                v8::PromiseHookType::kAfter => {
                    hooks.async_contexts.lock().unwrap().pop_back();
                }
                _ => {}
            }

            // if !i::StackLimitCheck{i_isolate}.has_overflowed() {
            let async_wraps = hooks.async_wraps_.lock().unwrap();
            for wrap in async_wraps.iter() {
                Self::promise_hook_dispatch(type_, promise.clone(), parent.clone(), wrap, &hooks);
                if try_catch.has_caught() {
                    break;
                }
            }

            if try_catch.has_caught() {
                // Shell::report_exception(v8_isolate, try_catch);
            }
            //}
        }

        // if let Some(exception) = exception {
        //     i_isolate.set_exception(exception.location());
        // }
    }

    fn promise_hook_dispatch(
        type_: v8::PromiseHookType,
        promise: v8::Local<v8::Promise>,
        parent: v8::Local<v8::Value>,
        wrap: &AsyncHooksWrap,
        hooks: &AsyncHooks,
    ) {
        if !wrap.is_enabled() {
            return;
        }

        let v8_isolate = hooks.v8_isolate_.clone();

        //if v8_isolate.is_execution_terminating() {
        //    return;
        //}

        let mut handle_scope = v8::HandleScope::new(&v8_isolate);

        let rcv = v8::undefined(&v8_isolate);
        let context = v8_isolate.get_current_context();
        let async_id_symbol = hooks.async_id_symbol.borrow();
        let async_id = promise.handle.borrow().get_private(context.clone(), async_id_symbol.as_ref().unwrap().clone()).unwrap();
        let args: [v8::Local<v8::Value>; 1] = [async_id];

        match type_ {
            v8::PromiseHookType::kInit => {
                if let Some(init_function) = wrap.init_function() {
                    let trigger_id_symbol = hooks.trigger_id_symbol.borrow();
                    let init_args: [v8::Local<v8::Value>; 4] = [
                        async_id,
                        v8::String::new_from_utf8_literal(&v8_isolate, "PROMISE"),
                        v8::Local {handle: Rc::new(RefCell::new(v8::Value {}))}, //promise
                            //.get_private(context, hooks.trigger_id_symbol.as_ref().unwrap().clone())
                            //.unwrap(),
                        v8::Local {handle: Rc::new(RefCell::new(v8::Value {}))}
                    ];
                    init_function.call(context, rcv, 4, &init_args).unwrap();
                }
            }
            v8::PromiseHookType::kBefore => {
                if let Some(before_function) = wrap.before_function() {
                    before_function.call(context, rcv, 1, &args).unwrap();
                }
            }
            v8::PromiseHookType::kAfter => {
                if let Some(after_function) = wrap.after_function() {
                    after_function.call(context, rcv, 1, &args).unwrap();
                }
            }
            v8::PromiseHookType::kResolve => {
                if let Some(promise_resolve_function) = wrap.promise_resolve_function() {
                    promise_resolve_function.call(context, rcv, 1, &args).unwrap();
                }
            }
        }
    }
}

lazy_static! {
    static ref PER_ISOLATE_DATA: Mutex<HashMap<String, PerIsolateData>> = Mutex::new(HashMap::new());
    static ref _isolate: Rc<v8::Isolate> = Rc::new(v8::Isolate::new());
    static ref _async_hooks: RefCell<AsyncHooks> = RefCell::new(AsyncHooks::new(_isolate.clone()));
}

#[derive(Debug)]
pub struct PerIsolateData {
    async_hooks: Rc<RefCell<AsyncHooks>>,
}

impl PerIsolateData {
    pub fn new(async_hooks: Rc<RefCell<AsyncHooks>>) -> Self {
        PerIsolateData { async_hooks }
    }

    pub fn get_async_hooks(&self) -> Rc<RefCell<AsyncHooks>> {
        self.async_hooks.clone()
    }
}

pub fn get_async_hooks() -> Rc<RefCell<AsyncHooks>> {
  _async_hooks.clone()
}

pub fn create_hook(info: &v8::FunctionCallbackInfo) -> Result<v8::Local<v8::Object>, String> {
    _async_hooks.borrow().create_hook(info)
}

pub fn setup_isolate() {
  let async_hooks = Rc::new(_async_hooks.clone());
  let data = PerIsolateData::new(async_hooks);
  PER_ISOLATE_DATA.lock().unwrap().insert(format!("{:p}", Rc::as_ptr(&_isolate)), data);
}

impl v8::Isolate {
    pub fn get_per_isolate_data(&self) -> Option<PerIsolateData> {
      PER_ISOLATE_DATA.lock().unwrap().get(&format!("{:p}", Rc::as_ptr(&self))).map(|d| d.clone())
    }
}