// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod js_function {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::{cell::Cell, marker::PhantomData};

    //use crate::debug::debug;
    //use crate::diagnostics::code_tracer;
    //use crate::ic::ic;
    //use crate::init::bootstrapper;
    //use crate::objects::abstract_code;
    use crate::objects::feedback_cell::{FeedbackCell, FeedbackCellRef};
    use crate::objects::feedback_vector::FeedbackVector;
    //use crate::objects::instance_type;
    //use crate::objects::map_updater;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    //use crate::sandbox::js_dispatch_table;
    //use crate::snapshot::embedded::embedded_data;
    use crate::objects::map::Map;
    use crate::objects::object::Object;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::context::Context;
    use crate::objects::js_prototype::JSPrototype;
    use crate::isolate::Isolate;
    use crate::builtins::builtin::Builtin;
    use crate::cage_base::CageBase;
    use crate::objects::smi::Smi;

    pub type JSDispatchHandleValueType = u32;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct JSDispatchHandle(JSDispatchHandleValueType);

    impl JSDispatchHandle {
        pub const NULL: JSDispatchHandle = JSDispatchHandle(0);

        pub fn new(value: JSDispatchHandleValueType) -> Self {
            Self(value)
        }

        pub fn value(&self) -> JSDispatchHandleValueType {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct JSFunctionOrBoundFunctionOrWrappedFunction {
        // Assuming this inherits from HeapObject
        heap_object: HeapObject,
    }

    #[derive(Debug)]
    pub struct JSBoundFunction {
        // Assuming this inherits from HeapObject
        heap_object: HeapObject,
    }

    #[derive(Debug)]
    pub struct JSWrappedFunction {
        // Assuming this inherits from HeapObject
        heap_object: HeapObject,
    }

    #[derive(Debug)]
    pub struct JSFunction {
        heap_object: HeapObject,
        feedback_cell: Cell<FeedbackCellRef>,
        context: Cell<*mut Context>,
        prototype_or_initial_map: Cell<*mut Object>,
        shared_function_info: Cell<*mut SharedFunctionInfo>,
        dispatch_handle: Cell<JSDispatchHandleValueType>,
        _phantom: PhantomData<HeapObject>, // Needed for raw pointer methods
    }

    impl JSFunction {
        const K_FEEDBACK_CELL_OFFSET: usize = 8;
        const K_CONTEXT_OFFSET: usize = 16;
        const K_PROTOTYPE_OR_INITIAL_MAP_OFFSET: usize = 24;
        const K_SHARED_FUNCTION_INFO_OFFSET: usize = 32;
        const K_DISPATCH_HANDLE_OFFSET: usize = 40;
        const K_CODE_OFFSET: usize = 48;

        pub fn new(heap_object: HeapObject) -> Self {
            JSFunction {
                heap_object,
                feedback_cell: Cell::new(FeedbackCellRef::null()),
                context: Cell::new(std::ptr::null_mut()),
                prototype_or_initial_map: Cell::new(std::ptr::null_mut()),
                shared_function_info: Cell::new(std::ptr::null_mut()),
                dispatch_handle: Cell::new(JSDispatchHandle::NULL.value()),
                _phantom: PhantomData,
            }
        }

        pub fn raw_feedback_cell(&self) -> FeedbackCellRef {
            self.feedback_cell.get()
        }

        pub fn set_raw_feedback_cell(&self, value: FeedbackCellRef) {
            self.feedback_cell.set(value);
        }

        pub fn feedback_vector(&self, cage_base: CageBase) -> Result<FeedbackVector, String> {
            if !self.has_feedback_vector(cage_base) {
                return Err("Function doesn't have feedback vector".to_string());
            }
            let raw_cell = self.raw_feedback_cell();
            let value = unsafe { (*raw_cell.ptr).value(cage_base) };
            Ok(FeedbackVector::from(value))
        }

        pub fn closure_feedback_cell_array(&self) -> Result<(), String> {
            if !self.has_closure_feedback_cell_array() {
                return Err("Function doesn't have closure feedback cell array".to_string());
            }
            // TODO: implement ClosureFeedbackCellArray and its casting, return that here
            Ok(())
        }

        //pub fn checks_tiering_state(&self, isolate: &Isolate) -> bool {
        //    self.code(isolate).checks_tiering_state()
        //}

        pub fn complete_inobject_slack_tracking_if_active(&self) {
            if !self.has_prototype_slot() {
                return;
            }
            // TODO: MapUpdater::CompleteInobjectSlackTracking
            //if self.has_initial_map() && self.initial_map().is_inobject_slack_tracking_in_progress() {
            //    MapUpdater::complete_inobject_slack_tracking(self.get_isolate(), self.initial_map());
            //}
        }

        //pub fn abstract_code(&self, isolate: &Isolate) -> AbstractCode {
        //    if self.active_tier_is_ignition(isolate) {
        //        self.shared().get_bytecode_array(isolate)
        //    } else {
        //        self.code(isolate)
        //    }
        //}

        pub fn length(&self) -> i32 {
            unsafe { (*self.shared_raw()).length() }
        }

        //void JSFunction::UpdateOptimizedCode(Isolate* isolate, Tagged<Code> code, WriteBarrierMode mode)
        // {
        //   DisallowGarbageCollection no_gc;
        //   DCHECK(code->is_optimized_code());
        // #ifdef V8_ENABLE_LEAPTIERING
        //   if (code->is_context_specialized()) {
        //     // We can only set context-specialized code for single-closure cells.
        //     if (raw_feedback_cell()->map() !=
        //         ReadOnlyRoots(isolate).one_closure_cell_map()) {
        //       return;
        //     }
        //   }
        //   // Required for being able to deoptimize this code.
        //   code->set_js_dispatch_handle(dispatch_handle());
        // #endif  // V8_ENABLE_LEAPTIERING
        //   UpdateCodeImpl(code, mode, false);
        // }

        // pub fn update_optimized_code(&self, isolate: &Isolate, code: Code, mode: WriteBarrierMode) {
        //     if code.is_context_specialized() {
        //         if self.raw_feedback_cell().map() != isolate.read_only_roots().one_closure_cell_map() {
        //             return;
        //         }
        //     }
        //     code.set_js_dispatch_handle(self.dispatch_handle());
        //     self.update_code_impl(code, mode, false);
        // }

        // void JSFunction::UpdateCodeImpl(Tagged<Code> value, WriteBarrierMode mode, bool keep_tiering_request)
        //  {
        //   DisallowGarbageCollection no_gc;

        // #ifdef V8_ENABLE_LEAPTIERING
        //   JSDispatchHandle handle = dispatch_handle();
        //   if (handle == kNullJSDispatchHandle) {
        //     handle = raw_feedback_cell()->dispatch_handle();
        //     DCHECK_NE(handle, kNullJSDispatchHandle);
        //     set_dispatch_handle(handle, mode);
        //   }
        //   if (keep_tiering_request) {
        //     UpdateDispatchEntryKeepTieringRequest(value, mode);
        //   } else {
        //     UpdateDispatchEntry(value, mode);
        //   }

        //   if (V8_UNLIKELY(v8_flags.log_function_events)) {
        //     IsolateGroup::current()->js_dispatch_table()->SetTieringRequest(
        //         dispatch_handle(), TieringBuiltin::kFunctionLogNextExecution,
        //         GetIsolate());
        //   }
        // #else
        //   WriteCodePointerField(kCodeOffset, value);
        //   CONDITIONAL_CODE_POINTER_WRITE_BARRIER(*this, kCodeOffset, value, mode);

        //   if (V8_UNLIKELY(v8_flags.log_function_events && has_feedback_vector())) {
        //     feedback_vector()->set_log_next_execution(true);
        //   }
        // #endif  // V8_ENABLE_LEAPTIERING
        // }
        
        //void JSFunction::UpdateCode(Tagged<Code> code, WriteBarrierMode mode)
        // {
        //   // Optimized code must go through UpdateOptimized code, which sets a
        //   // back-reference in the code object to the dispatch handle for
        //   // deoptimization.
        //   CHECK(!code->is_optimized_code());
        //   UpdateCodeImpl(code, mode, false);
        // }

        // inline void JSFunction::UpdateCodeKeepTieringRequests(Tagged<Code> code,
        //                                                       WriteBarrierMode mode) {
        //   CHECK(!code->is_optimized_code());
        //   UpdateCodeImpl(code, mode, true);
        // }
        

        // Tagged<Code> JSFunction::code(IsolateForSandbox isolate) const {
        // #ifdef V8_ENABLE_LEAPTIERING
        //   return IsolateGroup::current()->js_dispatch_table()->GetCode(
        //       dispatch_handle());
        // #else
        //   return ReadCodePointerField(kCodeOffset, isolate);
        // #endif
        // }

        // Tagged<Code> JSFunction::code(IsolateForSandbox isolate,
        //                               AcquireLoadTag tag) const {
        // #ifdef V8_ENABLE_LEAPTIERING
        //   return IsolateGroup::current()->js_dispatch_table()->GetCode(
        //       dispatch_handle(tag));
        // #else
        //   return ReadCodePointerField(kCodeOffset, isolate);
        // #endif
        // }

        // Tagged<Object> JSFunction::raw_code(IsolateForSandbox isolate) const {
        // #if V8_ENABLE_LEAPTIERING
        //   JSDispatchHandle handle = dispatch_handle();
        //   if (handle == kNullJSDispatchHandle) return Smi::zero();
        //   return IsolateGroup::current()->js_dispatch_table()->GetCode(handle);
        // #elif V8_ENABLE_SANDBOX
        //   return RawIndirectPointerField(kCodeOffset, kCodeIndirectPointerTag)
        //       .Relaxed_Load(isolate);
        // #else
        //   return RELAXED_READ_FIELD(*this, JSFunction::kCodeOffset);
        // #endif  // V8_ENABLE_SANDBOX
        // }

        // Tagged<Object> JSFunction::raw_code(IsolateForSandbox isolate,
        //                                     AcquireLoadTag tag) const {
        // #if V8_ENABLE_LEAPTIERING
        //   JSDispatchHandle handle = dispatch_handle(tag);
        //   if (handle == kNullJSDispatchHandle) return Smi::zero();
        //   return IsolateGroup::current()->js_dispatch_table()->GetCode(handle);
        // #elif V8_ENABLE_SANDBOX
        //   return RawIndirectPointerField(kCodeOffset, kCodeIndirectPointerTag)
        //       .Acquire_Load(isolate);
        // #else
        //   return ACQUIRE_READ_FIELD(*this, JSFunction::kCodeOffset);
        // #endif  // V8_ENABLE_SANDBOX
        // }

        // #ifdef V8_ENABLE_LEAPTIERING
        // // static
        // JSDispatchHandle JSFunction::AllocateDispatchHandle(Handle<JSFunction> function,
        //                                                     Isolate* isolate,
        //                                                     uint16_t parameter_count,
        //                                                     DirectHandle<Code> code,
        //                                                     WriteBarrierMode mode) {
        //   DCHECK_EQ(function->raw_feedback_cell()->dispatch_handle(),
        //             kNullJSDispatchHandle);
        //   return AllocateAndInstallJSDispatchHandle(
        //       function, kDispatchHandleOffset, isolate, parameter_count, code, mode);
        // }

        // void JSFunction::clear_dispatch_handle() {
        //   WriteField<JSDispatchHandle::underlying_type>(kDispatchHandleOffset,
        //                                                 kNullJSDispatchHandle.value());
        // }
        // void JSFunction::set_dispatch_handle(JSDispatchHandle handle,
        //                                      WriteBarrierMode mode) {
        //   Relaxed_WriteField<JSDispatchHandle::underlying_type>(kDispatchHandleOffset,
        //                                                         handle.value());
        //   CONDITIONAL_JS_DISPATCH_HANDLE_WRITE_BARRIER(*this, handle, mode);
        // }
        // void JSFunction::UpdateDispatchEntry(Tagged<Code> new_code,
        //                                      WriteBarrierMode mode) {
        //   JSDispatchHandle handle = dispatch_handle();
        //   IsolateGroup::current()->js_dispatch_table()->SetCodeNoWriteBarrier(handle,
        //                                                                       new_code);
        //   CONDITIONAL_JS_DISPATCH_HANDLE_WRITE_BARRIER(*this, handle, mode);
        // }
        // void JSFunction::UpdateDispatchEntryKeepTieringRequest(Tagged<Code> new_code,
        //                                                        WriteBarrierMode mode) {
        //   JSDispatchHandle handle = dispatch_handle();
        //   IsolateGroup::current()
        //       ->js_dispatch_table()
        //       ->SetCodeKeepTieringRequestNoWriteBarrier(handle, new_code);
        //   CONDITIONAL_JS_DISPATCH_HANDLE_WRITE_BARRIER(*this, handle, mode);
        // }
        // JSDispatchHandle JSFunction::dispatch_handle() const {
        //   return JSDispatchHandle(Relaxed_ReadField<JSDispatchHandle::underlying_type>(
        //       kDispatchHandleOffset));
        // }

        // JSDispatchHandle JSFunction::dispatch_handle(AcquireLoadTag tag) const {
        //   return JSDispatchHandle(Acquire_ReadField<JSDispatchHandle::underlying_type>(
        //       kDispatchHandleOffset));
        // }
        // #endif  // V8_ENABLE_LEAPTIERING

        pub fn context(&self) -> *mut Context {
            self.context.get()
        }

        pub fn set_context(&self, context: *mut Context) {
            self.context.set(context);
        }

        pub fn context_relaxed(&self, cage_base: CageBase) -> *mut Context {
            // TaggedField<Context, kContextOffset>::Relaxed_Load(cage_base, *this);
            unsafe {
                let ptr = self as *const Self as *const u8;
                let context_ptr = ptr.add(JSFunction::K_CONTEXT_OFFSET) as *const *mut Context;
                *context_ptr
            }
        }

        pub fn instruction_start(&self, _isolate: &Isolate) -> usize {
            //self.code(isolate).instruction_start()
            0 // Dummy return
        }

        pub fn shared(&self, cage_base: CageBase) -> *mut SharedFunctionInfo {
            self.shared_relaxed(cage_base)
        }

        pub fn shared_relaxed(&self, cage_base: CageBase) -> *mut SharedFunctionInfo {
            unsafe {
                let ptr = self as *const Self as *const u8;
                let shared_ptr = ptr.add(JSFunction::K_SHARED_FUNCTION_INFO_OFFSET) as *const *mut SharedFunctionInfo;
                *shared_ptr
            }
        }

        pub fn set_shared(&self, value: *mut SharedFunctionInfo) {
            self.shared_function_info.set(value);
        }

        //fn tiering_in_progress(&self) -> bool {
        //    self.feedback_vector().tiering_in_progress()
        //}

        //fn is_tiering_requested_or_in_progress(&self) -> bool {
        //    self.tiering_in_progress() || IsolateGroup::current().js_dispatch_table().is_tiering_requested(self.dispatch_handle())
        //}

        //fn is_logging_requested(&self, isolate: &Isolate) -> bool {
        //    IsolateGroup::current().js_dispatch_table().is_tiering_requested(self.dispatch_handle(), TieringBuiltin::kFunctionLogNextExecution, isolate)
        //}

        //fn is_maglev_requested(&self, isolate: &Isolate) -> bool {
        //    let jdt = IsolateGroup::current().js_dispatch_table();
        //    let entrypoint = jdt.get_entrypoint(self.dispatch_handle());
        //    let embedded_data = EmbeddedData::from_blob(isolate);
        //    if entrypoint == embedded_data.instruction_start_of(Builtin::kOptimizeMaglevEager) {
        //        DCHECK(jdt.is_tiering_requested(self.dispatch_handle(), TieringBuiltin::kOptimizeMaglevEager, isolate));
        //        return TieringBuiltin::kOptimizeMaglevEager != TieringBuiltin::kFunctionLogNextExecution;
        //    }
        //    ...
        //}

        //fn is_turbofan_requested(&self, isolate: &Isolate) -> bool { ... }

        //fn is_optimization_requested(&self, isolate: &Isolate) -> bool {
        //    self.is_maglev_requested(isolate) || self.is_turbofan_requested(isolate)
        //}

        //fn get_requested_optimization_if_any(&self, isolate: &Isolate, mode: ConcurrencyMode) -> Option<CodeKind> { ... }

        //fn reset_tiering_requests(&self) {
        //    IsolateGroup::current().js_dispatch_table().reset_tiering_request(self.dispatch_handle())
        //}

        //fn set_tiering_in_progress(&self, in_progress: bool, osr_offset: BytecodeOffset) { ... }

        // #ifndef V8_ENABLE_LEAPTIERING

        // TieringState JSFunction::tiering_state() const {
        //   if (!has_feedback_vector()) return TieringState::kNone;
        //   return feedback_vector()->tiering_state();
        // }

        // void JSFunction::set_tiering_state(IsolateForSandbox isolate,
        //                                    TieringState state) {
        //   DCHECK(has_feedback_vector());
        //   DCHECK(IsNone(state) || ChecksTieringState(isolate));
        //   feedback_vector()->set_tiering_state(state);
        // }

        // #endif  // !V8_ENABLE_LEAPTIERING

        //bool JSFunction::osr_tiering_in_progress() {
        //    DCHECK(has_feedback_vector());
        //    return feedback_vector()->osr_tiering_in_progress();
        //}

        pub fn has_feedback_vector(&self, cage_base: CageBase) -> bool {
            unsafe {
                (*self.shared_raw()).is_compiled() && {
                    let raw_cell = self.raw_feedback_cell();
                    let value = (*raw_cell.ptr).value(cage_base);
                    value.is_feedback_vector()
                }
            }
        }

        pub fn has_closure_feedback_cell_array(&self) -> bool {
            unsafe {
                (*self.shared_raw()).is_compiled() && {
                    let raw_cell = self.raw_feedback_cell();
                    let value = (*raw_cell.ptr).value(CageBase::GetProcessCageBase());
                    value.is_closure_feedback_cell_array()
                }
            }
        }

        pub fn global_proxy(&self) -> Result<(), String> {
            // TODO: implement global_proxy, JSGlobalProxy
            Ok(())
        }

        pub fn native_context(&self) -> Result<(), String> {
            // TODO: implement native_context, NativeContext
            Ok(())
        }

        pub fn prototype_or_initial_map(&self, cage_base: CageBase) -> *mut Object {
            unsafe {
                let ptr = self as *const Self as *const u8;
                let prototype_or_initial_map_ptr = ptr.add(JSFunction::K_PROTOTYPE_OR_INITIAL_MAP_OFFSET) as *const *mut Object;
                *prototype_or_initial_map_ptr
            }
        }

        pub fn set_prototype_or_initial_map(&self, value: *mut Object) {
            self.prototype_or_initial_map.set(value);
        }

        pub fn has_prototype_slot(&self) -> bool {
            //self.map().has_prototype_slot()
            true // dummy return
        }

        pub fn initial_map(&self, cage_base: CageBase) -> Result<Map, String> {
            //Cast<Map>(prototype_or_initial_map(cage_base, kAcquireLoad));
            let prototype_or_initial_map = self.prototype_or_initial_map(cage_base);
            if prototype_or_initial_map.is_null() {
                return Err("initial_map is null".to_string());
            }
            
            Ok(Map::from(unsafe { *prototype_or_initial_map }))
        }

        pub fn has_initial_map(&self, cage_base: CageBase) -> bool {
            //DCHECK(has_prototype_slot(cage_base));
            //return IsMap(prototype_or_initial_map(cage_base, kAcquireLoad), cage_base);
            !self.prototype_or_initial_map(cage_base).is_null()
        }

        pub fn has_instance_prototype(&self, cage_base: CageBase) -> bool {
            //DCHECK(has_prototype_slot(cage_base));
            //return has_initial_map(cage_base) ||
            //    !IsTheHole(prototype_or_initial_map(cage_base, kAcquireLoad));
            true // dummy return
        }

        pub fn has_prototype(&self, cage_base: CageBase) -> bool {
            //DCHECK(has_prototype_slot(cage_base));
            //return map(cage_base)->has_non_instance_prototype() ||
            //    has_instance_prototype(cage_base);
            true // dummy return
        }

        pub fn has_prototype_property(&self, cage_base: CageBase) -> bool {
            //return (has_prototype_slot(cage_base) && IsConstructor(*this, cage_base)) ||
            //    IsGeneratorFunction(shared(cage_base)->kind());
            true // dummy return
        }

        pub fn prototype_requires_runtime_lookup(&self, cage_base: CageBase) -> bool {
            //!self.has_prototype_property(cage_base) ||
            //map(cage_base)->has_non_instance_prototype()
            false // dummy return
        }

        pub fn instance_prototype(&self, cage_base: CageBase) -> Result<JSPrototype, String> {
            //DCHECK(has_instance_prototype(cage_base));
            if self.has_initial_map(cage_base) {
                let initial_map = self.initial_map(cage_base)?;
                //return initial_map->prototype(cage_base);
                return Ok(initial_map.prototype(cage_base));
            }
            // When there is no initial map and the prototype is a JSReceiver, the
            // initial map field is used for the prototype field.
            //return Cast<JSPrototype>(prototype_or_initial_map(cage_base, kAcquireLoad));
            Err("Not implemented".to_string())
        }

        pub fn prototype(&self, cage_base: CageBase) -> Result<(), String> {
            //DCHECK(has_prototype(cage_base));
            // If the function's prototype property has been set to a non-JSReceiver
            // value, that value is stored in the constructor field of the map.
            //Tagged<Map> map = this->map(cage_base);
            //if (map->has_non_instance_prototype()) {
            //    return map->GetNonInstancePrototype(cage_base);
            //}
            //return instance_prototype(cage_base);
            Ok(()) // dummy return
        }

        pub fn is_compiled(&self, _isolate: &Isolate) -> bool {
            //self.code(isolate, kAcquireLoad)->builtin_id() != Builtin::kCompileLazy &&
            //    self.shared()->is_compiled()
            true // dummy return
        }

        fn shared_raw(&self) -> *mut SharedFunctionInfo {
            self.shared_function_info.get()
        }

        pub fn map(&self, _cage_base: CageBase) -> Result<Map, String> {
            Err("Map function not implemented".to_string())
        }

        // TODO(v8): Implement these functions
        //bool JSFunction::NeedsResetDueToFlushedBytecode(Isolate* isolate);
        //bool JSFunction::NeedsResetDueToFlushedBaselineCode(IsolateForSandbox isolate);
        //void JSFunction::ResetIfCodeFlushed(Isolate* isolate, std::optional<std::function<void(Tagged<HeapObject> object, ObjectSlot slot, Tagged<HeapObject> target)>> gc_notify_updated_slot);
    }

    impl From<HeapObject> for JSFunction {
        fn from(heap_object: HeapObject) -> Self {
            JSFunction::new(heap_object)
        }
    }

    pub struct ClosureFeedbackCellArray {}
    impl ClosureFeedbackCellArray {
        fn from(_object: &Object) -> Self {
            ClosureFeedbackCellArray {}
        }
    }
}