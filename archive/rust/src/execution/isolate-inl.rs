// src/execution/isolate.rs
mod isolate_impl {
    use std::cell::RefCell;
    use std::ptr;
    use std::rc::Rc;

    use crate::objects::contexts::Context;
    use crate::objects::js_function::JSFunction;
    use crate::objects::lookup::LookupIterator;
    use crate::objects::objects::{JSArray, JSGlobalObject, JSGlobalProxy, JSMessageObject, JSReceiver, Object};
    use crate::objects::oddball::Oddball;
    use crate::objects::property_cell::PropertyCell;
    use crate::objects::regexp_match_info::RegExpMatchInfo;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::objects::source_text_module::SourceTextModule;
    use crate::runtime::runtime_utils;
    use crate::common::ptr_compr;
    use crate::objects::native_context::NativeContext;
    use crate::v8::{Isolate, ReadOnlyRoots, direct_handle, handle};
    use crate::v8::internal::ObjectPair;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum VMState {
        JS,
        EXTERNAL,
        NONE,
    }

    thread_local! {
        pub static CURRENT_PER_ISOLATE_THREAD_DATA: RefCell<Option<*mut PerIsolateThreadData>> = RefCell::new(None);
    }

    pub struct PerIsolateThreadData {
        pub context_: *mut Context,
        pub topmost_script_having_context_: *mut Context,
        pub pending_message_: *mut Object,
        pub exception_: *mut Object,
    }

    impl PerIsolateThreadData {
        pub fn new() -> Self {
            PerIsolateThreadData {
                context_: ptr::null_mut(),
                topmost_script_having_context_: ptr::null_mut(),
                pending_message_: ptr::null_mut(),
                exception_: ptr::null_mut(),
            }
        }
    }

    impl Isolate {

        pub fn current_per_isolate_thread_data() -> *mut PerIsolateThreadData {
            CURRENT_PER_ISOLATE_THREAD_DATA.with(|data| {
                match *data.borrow() {
                    Some(ptr) => ptr,
                    None => panic!("No PerIsolateThreadData set for this thread"),
                }
            })
        }

        pub fn current() -> &'static mut Isolate {
            match Self::try_get_current() {
                Some(isolate) => isolate,
                None => panic!("No current isolate"),
            }
        }

        pub fn is_current(&self) -> bool {
            match Self::try_get_current() {
                Some(current) => self as *const _ == current as *const _,
                None => false,
            }
        }

        pub fn set_context(&mut self, context: *mut Context) {
            unsafe {
                (*Self::current_per_isolate_thread_data()).context_ = context;
            }
        }

        pub fn native_context(&mut self) -> handle<NativeContext> {
            unsafe {
                let context = (*Self::current_per_isolate_thread_data()).context_;
                assert!(!context.is_null());
                handle((*context).native_context(), self)
            }
        }

        pub fn raw_native_context(&self) -> *mut NativeContext {
            unsafe {
                let context = (*Self::current_per_isolate_thread_data()).context_;
                assert!(!context.is_null());
                (*context).native_context()
            }
        }

        pub fn set_topmost_script_having_context(&mut self, context: *mut Context) {
            unsafe {
                (*Self::current_per_isolate_thread_data()).topmost_script_having_context_ = context;
            }
        }

        pub fn clear_topmost_script_having_context(&mut self) {
            unsafe {
                (*Self::current_per_isolate_thread_data()).topmost_script_having_context_ = ptr::null_mut();
            }
        }

        // The original code used `DirectHandle<NativeContext>`, which isn't directly translatable
        // without a deeper understanding of its purpose and implementation.
        // This translation returns a raw pointer.  Review if this is the correct way.
        pub fn get_incumbent_context(&mut self) -> *mut NativeContext {
            unsafe {
                let maybe_topmost_script_having_context =
                    (*Self::current_per_isolate_thread_data()).topmost_script_having_context_;

                if !maybe_topmost_script_having_context.is_null() {
                    // Assuming EXTERNAL and JS can be represented by VMState enum
                    // and current_vm_state() returns a VMState value
                    if self.current_vm_state() == VMState::EXTERNAL ||
                        self.current_vm_state() == VMState::JS {
                        let incumbent_context = (*maybe_topmost_script_having_context).native_context();
                        assert_eq!(incumbent_context, *self.get_incumbent_context_slow());
                        return incumbent_context;
                    }
                }
                *self.get_incumbent_context_slow()
            }
        }

        // Placeholder functions, replace with actual implementation
        fn current_vm_state(&self) -> VMState {
            VMState::NONE // Replace with proper state retrieval
        }

        fn get_incumbent_context_slow(&self) -> *mut *mut NativeContext {
            panic!("get_incumbent_context_slow not implemented");
        }

        pub fn set_pending_message(&mut self, message_obj: *mut Object) {
            unsafe {
                // Placeholder for IsTheHole and IsJSMessageObject checks
                (*Self::current_per_isolate_thread_data()).pending_message_ = message_obj;
            }
        }

        pub fn pending_message(&self) -> *mut Object {
            unsafe {
                (*Self::current_per_isolate_thread_data()).pending_message_
            }
        }

        pub fn clear_pending_message(&mut self) {
            let read_only_roots = ReadOnlyRoots::new(self);
            self.set_pending_message(read_only_roots.the_hole_value());
        }

        pub fn has_pending_message(&self) -> bool {
            let read_only_roots = ReadOnlyRoots::new(self);
            unsafe {
                !self.is_the_hole( (*Self::current_per_isolate_thread_data()).pending_message_ , self)
            }
        }

        pub fn exception(&self) -> *mut Object {
            self.check_has_exception();
            unsafe {
                (*Self::current_per_isolate_thread_data()).exception_
            }
        }

        pub fn set_exception(&mut self, exception_obj: *mut Object) {
            unsafe {
                // Placeholder for IsException check
                (*Self::current_per_isolate_thread_data()).exception_ = exception_obj;
            }
        }

        pub fn clear_internal_exception(&mut self) {
            unsafe {
                // Placeholder for IsException check
                let read_only_roots = ReadOnlyRoots::new(self);
                (*Self::current_per_isolate_thread_data()).exception_ = read_only_roots.the_hole_value();
            }
        }

        pub fn clear_exception(&mut self) {
            self.clear_internal_exception();
            if let Some(handler) = self.try_catch_handler() {
                handler.reset();
            }
        }

        pub fn has_exception(&self) -> bool {
            let top = self.thread_local_top();
            unsafe {
                // Placeholder for IsException check
                !self.is_the_hole(top.exception_, self)
            }
        }

        pub fn is_execution_terminating(&self) -> bool {
            unsafe {
                let top = self.thread_local_top();
                let read_only_roots = ReadOnlyRoots::new(self);
                top.exception_ == read_only_roots.termination_exception()
            }
        }

        // Placeholder functions for debugging and pointer compression.
        #[cfg(debug_assertions)]
        pub fn verify_builtins_result(&self, result: *mut Object) -> *mut Object {
            if self.is_execution_terminating() && !self.flags().strict_termination_checks {
                let read_only_roots = ReadOnlyRoots::new(self);
                return read_only_roots.exception();
            }
            // Placeholder for SafeEquals and GetIsolateFromHeapObject
            //assert_eq!(self.has_exception(), result.safe_equals(ReadOnlyRoots::new(self).exception()));

            // Placeholder for V8_COMPRESS_POINTERS check
            /*
            if !is_smi(result) {
                if let Some((isolate, _)) = get_isolate_from_heap_object(result) {
                    assert!(isolate as *const _ == self as *const _ || isolate as *const _ == self.shared_space_isolate() as *const _);
                }
            }
            */
            result
        }

        #[cfg(debug_assertions)]
        pub fn verify_builtins_result_pair(&self, pair: ObjectPair) -> ObjectPair {
            // Placeholder for SafeEquals and GetIsolateFromHeapObject
            /*
            let x = pair.x;
            let y = pair.y;
            assert_eq!(self.has_exception(), x.safe_equals(ReadOnlyRoots::new(self).exception()));
            if let Some((isolate, _)) = get_isolate_from_heap_object(x) {
                assert!(isolate as *const _ == self as *const _ || isolate as *const _ == self.shared_space_isolate() as *const _);
            }
            if let Some((isolate, _)) = get_isolate_from_heap_object(y) {
                assert!(isolate as *const _ == self as *const _ || isolate as *const _ == self.shared_space_isolate() as *const _);
            }
            */
            pair
        }

        pub fn is_catchable_by_javascript(&self, exception: *mut Object) -> bool {
            let read_only_roots = ReadOnlyRoots::new(self);
            exception != read_only_roots.termination_exception()
        }

        pub fn in_fast_c_call(&self) -> bool {
            self.isolate_data().fast_c_call_caller_fp() != 0 // kNullAddress is 0 in Rust
        }

        pub fn is_catchable_by_wasm(&self, exception: *mut Object) -> bool {
            if !self.is_catchable_by_javascript(exception) {
                return false;
            }
            //Placeholder for JSObject check
            //if !is_js_object(exception) {
            //    return true;
            //}
            // Placeholder for LookupIterator::HasInternalMarkerProperty
            //return !LookupIterator::has_internal_marker_property(self, exception as *mut JSReceiver, self.factory().wasm_uncatchable_symbol());
            true // Default, adjust after implementing the placeholder
        }

        pub fn fire_before_call_entered_callback(&mut self) {
            for callback in &self.before_call_entered_callbacks_ {
                callback(self); // Reinterpret cast not needed since it is Rust.
            }
        }

        pub fn global_object(&self) -> handle<JSGlobalObject> {
            handle(unsafe{(*(*Self::current_per_isolate_thread_data()).context_).global_object()}, self)
        }

        pub fn global_proxy(&self) -> handle<JSGlobalProxy> {
            handle(unsafe{(*(*Self::current_per_isolate_thread_data()).context_).global_proxy()}, self)
        }

        pub fn is_initial_array_prototype(&self, array: *mut JSArray) -> bool {
            // Placeholder for DisallowGarbageCollection and IsInCreationContext
            // let no_gc = DisallowGarbageCollection::new();
            //IsInCreationContext(array, Context::INITIAL_ARRAY_PROTOTYPE_INDEX)
            true // Placeholder, needs actual implementation
        }

        unsafe fn is_the_hole(&self, obj: *mut Object, _isolate: &Isolate) -> bool {
            obj == ReadOnlyRoots::new(self).the_hole_value()
        }

        fn thread_local_top(&self) -> &PerIsolateThreadData {
            unsafe {
                &*Self::current_per_isolate_thread_data()
            }
        }

        fn check_has_exception(&self) {
            assert!(self.has_exception());
        }

        fn flags(&self) -> &Flags {
            &Flags{}
        }

        fn isolate_data(&self) -> &IsolateData {
            &IsolateData{}
        }

        fn try_catch_handler(&self) -> Option<&TryCatchHandler> {
            None
        }

        fn shared_space_isolate(&self) -> &Isolate {
            self
        }

        // Placeholder functions and structs - replace with actual implementations
        fn try_get_current() -> Option<&'static mut Isolate> {
            unsafe {
                static mut ISOLATE: Isolate = Isolate{}; // Dummy static instance
                Some(&mut ISOLATE)
            }
        }

        // NATIVE_CONTEXT_FIELDS macro expansion

        pub fn array_prototype(&mut self) -> handle<JSArray> {
            handle(self.raw_native_context().array_prototype(), self)
        }
        pub fn is_array_prototype(&mut self, value: *mut JSArray) -> bool {
            self.raw_native_context().is_array_prototype(value)
        }

        pub fn array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().array_constructor(), self)
        }
        pub fn is_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_array_constructor(value)
        }

        pub fn boolean_prototype(&mut self) -> handle<Oddball> {
            handle(self.raw_native_context().boolean_prototype(), self)
        }
        pub fn is_boolean_prototype(&mut self, value: *mut Oddball) -> bool {
            self.raw_native_context().is_boolean_prototype(value)
        }

        pub fn boolean_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().boolean_constructor(), self)
        }
        pub fn is_boolean_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_boolean_constructor(value)
        }

        pub fn number_prototype(&mut self) -> handle<Oddball> {
            handle(self.raw_native_context().number_prototype(), self)
        }
        pub fn is_number_prototype(&mut self, value: *mut Oddball) -> bool {
            self.raw_native_context().is_number_prototype(value)
        }

        pub fn number_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().number_constructor(), self)
        }
        pub fn is_number_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_number_constructor(value)
        }

        pub fn string_prototype(&mut self) -> handle<Oddball> {
            handle(self.raw_native_context().string_prototype(), self)
        }
        pub fn is_string_prototype(&mut self, value: *mut Oddball) -> bool {
            self.raw_native_context().is_string_prototype(value)
        }

        pub fn string_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().string_constructor(), self)
        }
        pub fn is_string_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_string_constructor(value)
        }

        pub fn symbol_prototype(&mut self) -> handle<Oddball> {
            handle(self.raw_native_context().symbol_prototype(), self)
        }
        pub fn is_symbol_prototype(&mut self, value: *mut Oddball) -> bool {
            self.raw_native_context().is_symbol_prototype(value)
        }

        pub fn symbol_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().symbol_constructor(), self)
        }
        pub fn is_symbol_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_symbol_constructor(value)
        }

        pub fn function_prototype(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().function_prototype(), self)
        }
        pub fn is_function_prototype(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_function_prototype(value)
        }

        pub fn function_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().function_constructor(), self)
        }
        pub fn is_function_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_function_constructor(value)
        }

        pub fn object_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().object_prototype(), self)
        }
        pub fn is_object_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_object_prototype(value)
        }

        pub fn object_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().object_constructor(), self)
        }
        pub fn is_object_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_object_constructor(value)
        }

        pub fn regexp_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().regexp_prototype(), self)
        }
        pub fn is_regexp_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_regexp_prototype(value)
        }

        pub fn regexp_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().regexp_constructor(), self)
        }
        pub fn is_regexp_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_regexp_constructor(value)
        }

        pub fn error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().error_prototype(), self)
        }
        pub fn is_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_error_prototype(value)
        }

        pub fn error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().error_constructor(), self)
        }
        pub fn is_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_error_constructor(value)
        }

        pub fn eval_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().eval_error_prototype(), self)
        }
        pub fn is_eval_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_eval_error_prototype(value)
        }

        pub fn eval_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().eval_error_constructor(), self)
        }
        pub fn is_eval_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_eval_error_constructor(value)
        }

        pub fn range_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().range_error_prototype(), self)
        }
        pub fn is_range_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_range_error_prototype(value)
        }

        pub fn range_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().range_error_constructor(), self)
        }
        pub fn is_range_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_range_error_constructor(value)
        }

        pub fn reference_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().reference_error_prototype(), self)
        }
        pub fn is_reference_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_reference_error_prototype(value)
        }

        pub fn reference_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().reference_error_constructor(), self)
        }
        pub fn is_reference_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_reference_error_constructor(value)
        }

        pub fn syntax_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().syntax_error_prototype(), self)
        }
        pub fn is_syntax_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_syntax_error_prototype(value)
        }

        pub fn syntax_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().syntax_error_constructor(), self)
        }
        pub fn is_syntax_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_syntax_error_constructor(value)
        }

        pub fn type_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().type_error_prototype(), self)
        }
        pub fn is_type_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_type_error_prototype(value)
        }

        pub fn type_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().type_error_constructor(), self)
        }
        pub fn is_type_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_type_error_constructor(value)
        }

        pub fn uri_error_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().uri_error_prototype(), self)
        }
        pub fn is_uri_error_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_uri_error_prototype(value)
        }

        pub fn uri_error_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().uri_error_constructor(), self)
        }
        pub fn is_uri_error_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_uri_error_constructor(value)
        }

        pub fn promise_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().promise_prototype(), self)
        }
        pub fn is_promise_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_promise_prototype(value)
        }

        pub fn promise_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().promise_constructor(), self)
        }
        pub fn is_promise_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_promise_constructor(value)
        }

        pub fn map_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().map_prototype(), self)
        }
        pub fn is_map_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_map_prototype(value)
        }

        pub fn map_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().map_constructor(), self)
        }
        pub fn is_map_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_map_constructor(value)
        }

        pub fn set_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().set_prototype(), self)
        }
        pub fn is_set_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_set_prototype(value)
        }

        pub fn set_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().set_constructor(), self)
        }
        pub fn is_set_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_set_constructor(value)
        }

        pub fn weak_map_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().weak_map_prototype(), self)
        }
        pub fn is_weak_map_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_weak_map_prototype(value)
        }

        pub fn weak_map_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().weak_map_constructor(), self)
        }
        pub fn is_weak_map_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_weak_map_constructor(value)
        }

        pub fn weak_set_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().weak_set_prototype(), self)
        }
        pub fn is_weak_set_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_weak_set_prototype(value)
        }

        pub fn weak_set_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().weak_set_constructor(), self)
        }
        pub fn is_weak_set_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_weak_set_constructor(value)
        }

        pub fn array_buffer_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().array_buffer_prototype(), self)
        }
        pub fn is_array_buffer_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_array_buffer_prototype(value)
        }

        pub fn array_buffer_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().array_buffer_constructor(), self)
        }
        pub fn is_array_buffer_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_array_buffer_constructor(value)
        }

        pub fn typed_array_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().typed_array_prototype(), self)
        }
        pub fn is_typed_array_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_typed_array_prototype(value)
        }

        pub fn int8_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().int8_array_constructor(), self)
        }
        pub fn is_int8_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_int8_array_constructor(value)
        }

        pub fn uint8_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().uint8_array_constructor(), self)
        }
        pub fn is_uint8_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_uint8_array_constructor(value)
        }

        pub fn uint8_clamped_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().uint8_clamped_array_constructor(), self)
        }
        pub fn is_uint8_clamped_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_uint8_clamped_array_constructor(value)
        }

        pub fn int16_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().int16_array_constructor(), self)
        }
        pub fn is_int16_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_int16_array_constructor(value)
        }

        pub fn uint16_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().uint16_array_constructor(), self)
        }
        pub fn is_uint16_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_uint16_array_constructor(value)
        }

        pub fn int32_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().int32_array_constructor(), self)
        }
        pub fn is_int32_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_int32_array_constructor(value)
        }

        pub fn uint32_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().uint32_array_constructor(), self)
        }
        pub fn is_uint32_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_uint32_array_constructor(value)
        }

        pub fn float32_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().float32_array_constructor(), self)
        }
        pub fn is_float32_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_float32_array_constructor(value)
        }

        pub fn float64_array_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().float64_array_constructor(), self)
        }
        pub fn is_float64_array_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_float64_array_constructor(value)
        }

        pub fn data_view_prototype(&mut self) -> handle<JSReceiver> {
            handle(self.raw_native_context().data_view_prototype(), self)
        }
        pub fn is_data_view_prototype(&mut self, value: *mut JSReceiver) -> bool {
            self.raw_native_context().is_data_view_prototype(value)
        }

        pub fn data_view_constructor(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().data_view_constructor(), self)
        }
        pub fn is_data_view_constructor(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_data_view_constructor(value)
        }

        pub fn promise_resolve(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().promise_resolve(), self)
        }
        pub fn is_promise_resolve(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_promise_resolve(value)
        }

        pub fn promise_reject(&mut self) -> handle<JSFunction> {
            handle(self.raw_native_context().promise_reject(), self)
        }
        pub fn is_promise_reject(&mut self, value: *mut JSFunction) -> bool {
            self.raw_native_context().is_promise_reject(value)
        }

        pub fn promise_