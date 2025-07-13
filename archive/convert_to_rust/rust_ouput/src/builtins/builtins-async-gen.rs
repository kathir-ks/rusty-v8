// Converted from V8 C++ source files:
// Header: builtins-async-gen.h
// Implementation: builtins-async-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins_async_gen {
use crate::builtins::builtins_promise_gen::PromiseBuiltinsAssembler;
use crate::objects::js_generator::JSGeneratorObject;
use crate::objects::js_promise::JSPromise;
use crate::objects::shared_function_info::SharedFunctionInfo;
use crate::compiler::code_assembler_state::CodeAssemblerState;
use crate::objects::contexts::Context;
use crate::heap::root_index::RootIndex;
use crate::builtins::builtins_utils_gen::*;
use crate::heap::factory_inl::*;
use crate::isolate::isolate::Isolate;
use crate::objects::fixed_array::FixedArray;
use crate::codegen::code_stub_assembler::CodeStubAssembler;
use crate::objects::map::Map;
use crate::objects::contexts::NativeContext;
use crate::codegen::code_stub_assembler::Label;
use crate::codegen::code_stub_assembler::Variable;
use crate::objects::contexts::Context::*;
use std::cell::RefCell;
use std::rc::Rc;

    pub struct AsyncBuiltinsAssembler {
        pub promise_builtins_assembler: PromiseBuiltinsAssembler,
    }

    impl AsyncBuiltinsAssembler {
        pub fn new(state: &mut CodeAssemblerState) -> Self {
            AsyncBuiltinsAssembler {
                promise_builtins_assembler: PromiseBuiltinsAssembler::new(state),
            }
        }

        pub fn await_(&mut self, context: &TNode<Context>, generator: &TNode<JSGeneratorObject>, value: &TNode<JSAny>, outer_promise: &TNode<JSPromise>, on_resolve_sfi: RootIndex, on_reject_sfi: RootIndex) -> TNode<Object> {
            let create_closures = |context: TNode<Context>, native_context: TNode<NativeContext>| {
                let on_resolve = self.allocate_root_function_with_context(on_resolve_sfi, &context, &native_context);
                let on_reject = self.allocate_root_function_with_context(on_reject_sfi, &context, &native_context);
                (on_resolve, on_reject)
            };
            self.await_internal(context, generator, value, outer_promise, Box::new(create_closures))
        }

        fn allocate_root_function_with_context(&mut self, root_index: RootIndex, context: &TNode<Context>, native_context: &TNode<NativeContext>) -> TNode<JSFunction> {
            TNode::<JSFunction>::default()
        }

        fn await_internal(&mut self, context: &TNode<Context>, generator: &TNode<JSGeneratorObject>, value: &TNode<JSAny>, outer_promise: &TNode<JSPromise>, create_closures: Box<dyn Fn(TNode<Context>, TNode<NativeContext>) -> (TNode<JSFunction>, TNode<JSFunction>)>) -> TNode<Object> {
            let native_context = self.load_native_context(context);

            // We do the `PromiseResolve(%Promise%,value)` avoiding to unnecessarily
            // create wrapper promises. Now if {value} is already a promise with the
            // intrinsics %Promise% constructor as its "constructor", we don't need
            // to allocate the wrapper promise.
            let mut var_value = Variable::<JSAny>::new(value.clone());
            let mut if_slow_path = Label::new();
            let mut if_done = Label::new();
            let mut if_slow_constructor = Label::new();
            if self.tagged_is_smi(value) {
                if_slow_path.goto();
            }
            let value_object: TNode<JSAnyNotSmi> = value.clone().into();
            let value_map = self.load_map(&value_object);
            if !self.is_js_promise_map(&value_map) {
                if_slow_path.goto();
            }
            // We can skip the "constructor" lookup on {value} if it's [[Prototype]]
            // is the (initial) Promise.prototype and the @@species protector is
            // intact, as that guards the lookup path for "constructor" on
            // JSPromise instances which have the (initial) Promise.prototype.
            let promise_prototype = self.load_context_element(&native_context, Context::PROMISE_PROTOTYPE_INDEX as i32);
            if !self.tagged_equal(&self.load_map_prototype(&value_map), &promise_prototype) {
                if_slow_constructor.goto();
            }
            if self.is_promise_species_protector_cell_invalid() {
                if_slow_constructor.goto();
            } else {
                if_done.goto();
            }

            // At this point, {value} doesn't have the initial promise prototype or
            // the promise @@species protector was invalidated, but {value} could still
            // have the %Promise% as its "constructor", so we need to check that as
            // well.
            if_slow_constructor.bind(|| {
                let value_constructor = self.get_property(context, value, &self.isolate().factory().constructor_string());
                let promise_function = self.load_context_element(&native_context, Context::PROMISE_FUNCTION_INDEX as i32);
                if self.tagged_equal(&value_constructor, &promise_function) {
                    if_done.goto();
                } else {
                    if_slow_path.goto();
                }
            });

            if_slow_path.bind(|| {
                // We need to mark the {value} wrapper as having {outer_promise}
                // as its parent, which is why we need to inline a good chunk of
                // logic from the `PromiseResolve` builtin here.
                var_value.set(self.new_js_promise(&native_context, outer_promise));
                self.call_builtin(Builtin::kResolvePromise, &native_context, &var_value.value(), value);
                if_done.goto();
            });

            if_done.bind(|| {
                *value = var_value.value().clone();
            });

            let k_closure_context_size = FixedArray::size_for(Context::MIN_CONTEXT_EXTENDED_SLOTS as usize);
            let closure_context: TNode<Context> = unsafe { std::mem::transmute(self.allocate_in_new_space(k_closure_context_size)) };
            {
                // Initialize the await context, storing the {generator} as extension.
                let map: TNode<Map> = unsafe { std::mem::transmute(self.load_context_element(&native_context, Context::AWAIT_CONTEXT_MAP_INDEX as i32)) };
                self.store_map_no_write_barrier(&closure_context, &map);
                self.store_object_field_no_write_barrier(&closure_context, Context::kLengthOffset as usize, self.smi_constant(Context::MIN_CONTEXT_EXTENDED_SLOTS as i32));
                let empty_scope_info = self.load_context_element(&native_context, Context::SCOPE_INFO_INDEX as i32);
                self.store_context_element_no_write_barrier(&closure_context, Context::SCOPE_INFO_INDEX as i32, &empty_scope_info);
                self.store_context_element_no_write_barrier(&closure_context, Context::PREVIOUS_INDEX as i32, &native_context);
                self.store_context_element_no_write_barrier(&closure_context, Context::EXTENSION_INDEX as i32, generator);
            }

            // Allocate and initialize resolve and reject handlers
            let (on_resolve, on_reject) = create_closures(closure_context.clone(), native_context.clone());

            // Deal with PromiseHooks and debug support in the runtime. This
            // also allocates the throwaway promise, which is only needed in
            // case of PromiseHooks or debugging.
            let mut var_throwaway = Variable::<Object>::new(self.undefined_constant());
            let mut if_instrumentation = Label::new();
            let mut if_instrumentation_done = Label::new();
            let promise_hook_flags = self.promise_hook_flags();
            if self.is_isolate_promise_hook_enabled_or_debug_is_active_or_has_async_event_delegate(promise_hook_flags) {
                if_instrumentation.goto();
            } else {
                if_instrumentation_done.goto();
            }
            
            if_instrumentation.bind(|| {
                var_throwaway.set(unsafe { std::mem::transmute(self.call_runtime(Runtime::kDebugAsyncFunctionSuspended, &native_context, value, outer_promise, &on_reject, generator)) });
                if_instrumentation_done.goto();
            });

            if_instrumentation_done.bind(|| {
                let throwaway = var_throwaway.value();
                return unsafe { std::mem::transmute(self.call_builtin(Builtin::kPerformPromiseThen, &native_context, value, &on_resolve, &on_reject, &throwaway)) };
            });
        }

        fn new_js_promise(&mut self, native_context: &TNode<NativeContext>, outer_promise: &TNode<JSPromise>) -> TNode<JSPromise> {
            TNode::<JSPromise>::default()
        }

        fn load_map_prototype(&mut self, map: &TNode<Map>) -> TNode<Object> {
            TNode::<Object>::default()
        }

        fn is_promise_species_protector_cell_invalid(&mut self) -> bool {
            false
        }

        fn get_property(&mut self, context: &TNode<Context>, value: &TNode<JSAny>, name: &str) -> TNode<Object> {
            TNode::<Object>::default()
        }

        fn undefined_constant(&mut self) -> TNode<Object> {
            TNode::<Object>::default()
        }

        fn promise_hook_flags(&mut self) -> TNode<Uint32T> {
            TNode::<Uint32T>::default()
        }

        fn is_isolate_promise_hook_enabled_or_debug_is_active_or_has_async_event_delegate(&mut self, promise_hook_flags: TNode<Uint32T>) -> bool {
            false
        }

        fn call_runtime(&mut self, runtime_function: Runtime, native_context: &TNode<NativeContext>, value: &TNode<JSAny>, outer_promise: &TNode<JSPromise>, on_reject: &TNode<JSFunction>, generator: &TNode<JSGeneratorObject>) -> *mut Object {
            std::ptr::null_mut()
        }

        fn allocate_in_new_space(&mut self, size: usize) -> *mut Object {
            std::ptr::null_mut()
        }

        fn store_map_no_write_barrier(&mut self, context: &TNode<Context>, map: &TNode<Map>) {}

        fn store_object_field_no_write_barrier(&mut self, context: &TNode<Context>, offset: usize, value: TNode<Smi>) {}

        fn store_context_element_no_write_barrier(&mut self, context: &TNode<Context>, index: i32, value: &TNode<Object>) {}

        fn smi_constant(&mut self, value: i32) -> TNode<Smi> {
            TNode::<Smi>::default()
        }

        fn tagged_is_smi(&mut self, value: &TNode<JSAny>) -> bool {
            false
        }

        fn load_map(&mut self, value_object: &TNode<JSAnyNotSmi>) -> TNode<Map> {
            TNode::<Map>::default()
        }

        fn is_js_promise_map(&mut self, value_map: &TNode<Map>) -> bool {
            false
        }

        fn tagged_equal(&mut self, a: &TNode<Object>, b: &TNode<Object>) -> bool {
            false
        }

        fn load_native_context(&mut self, context: &TNode<Context>) -> TNode<NativeContext> {
            TNode::<NativeContext>::default()
        }

        pub fn create_unwrap_closure(&mut self, native_context: &TNode<NativeContext>, done: &TNode<Boolean>) -> TNode<JSFunction> {
            let closure_context = self.allocate_async_iterator_value_unwrap_context(native_context, done);
            self.allocate_root_function_with_context(RootIndex::kAsyncIteratorValueUnwrapSharedFun, &closure_context, native_context)
        }

        fn allocate_async_iterator_value_unwrap_context(&mut self, native_context: &TNode<NativeContext>, done: &TNode<Boolean>) -> TNode<Context> {
            let context: TNode<Context> = unsafe { std::mem::transmute(self.allocate_synthetic_function_context(native_context, ValueUnwrapContext::kLength as i32)) };
            self.store_context_element_no_write_barrier(&context, ValueUnwrapContext::kDoneSlot as i32, unsafe {std::mem::transmute(done)});
            context
        }

        fn allocate_synthetic_function_context(&mut self, native_context: &TNode<NativeContext>, length: i32) -> *mut Object {
            std::ptr::null_mut()
        }
        
        fn isolate(&mut self) -> &Isolate {
            todo!()
        }
    }

    pub struct ValueUnwrapContext {}

    impl ValueUnwrapContext {
        pub const kDoneSlot: i32 = Context::MIN_CONTEXT_SLOTS as i32;
        pub const kLength: i32 = 0;
    }

    pub type TNode<T> = T;
    pub type JSAny = Object;
    pub type JSAnyNotSmi = Object;
    pub type Uint32T = u32;
    pub type Smi = i32;
    pub type Object = *mut std::ffi::c_void;
    pub type Boolean = bool;
    pub struct Builtin {}

    impl Builtin {
        pub const kResolvePromise: Self = Builtin{};
        pub const kPerformPromiseThen: Self = Builtin{};
        pub const kCreateIterResultObject: Self = Builtin{};
    }

    pub struct Runtime {}

    impl Runtime {
        pub const kDebugAsyncFunctionSuspended: Self = Runtime{};
    }

    pub struct TF_BUILTIN {}
}
