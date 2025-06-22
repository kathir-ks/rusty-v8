// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/debug/debug_stack_trace_iterator.rs

use std::ptr;

mod api;
mod debug_evaluate;
mod debug_scope_iterator;
mod debug;
mod execution;

#[cfg(V8_ENABLE_WEBASSEMBLY)]
mod debug_wasm_objects;

pub mod debug {
    use super::*;
    use std::rc::Rc;

    pub struct StackTraceIterator {
        inner: Box<internal::DebugStackTraceIterator>,
    }

    impl StackTraceIterator {
        pub fn create(isolate: *mut api::Isolate, index: i32) -> StackTraceIterator {
            StackTraceIterator {
                inner: internal::DebugStackTraceIterator::new(
                    isolate as *mut internal::Isolate,
                    index,
                ),
            }
        }
    }

    pub struct Location {
        line_number: i32,
        column_number: i32,
    }

    impl Location {
        pub fn new(line_number: i32, column_number: i32) -> Self {
            Location {
                line_number,
                column_number,
            }
        }
    }

    pub struct Script {}

    impl Script {
        pub fn get_source_location(&self, pos: i32) -> Location {
            // Placeholder implementation.  Needs proper implementation based on Script data.
            Location::new(0, 0)
        }
    }
}

mod internal {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct DebugStackTraceIterator {
        isolate_: *mut Isolate,
        iterator_: StackFrameIterator,
        is_top_frame_: bool,
        resumable_fn_on_stack_: bool,
        inlined_frame_index_: i32,
        frame_inspector_: Option<FrameInspector>,
    }

    impl DebugStackTraceIterator {
        pub fn new(isolate: *mut Isolate, index: i32) -> Box<Self> {
            let mut iterator_ = StackFrameIterator::new(isolate, 0); // TODO: break_frame_id
            let mut result = Box::new(DebugStackTraceIterator {
                isolate_: isolate,
                iterator_: iterator_,
                is_top_frame_: true,
                resumable_fn_on_stack_: false,
                inlined_frame_index_: 0,
                frame_inspector_: None,
            });

            if result.iterator_.done() {
                return result;
            }

            result.update_inline_frame_index_and_resumable_fn_on_stack();
            result.advance();
            for _ in 0..index {
                result.advance();
            }

            result
        }

        fn done(&self) -> bool {
            self.iterator_.done()
        }

        fn advance(&mut self) {
            loop {
                self.inlined_frame_index_ -= 1;
                while self.inlined_frame_index_ >= 0 {
                    // Omit functions from native and extension scripts.
                    // FrameSummary::Get(iterator_.frame(), inlined_frame_index_)
                    //     .is_subject_to_debugging() {
                    if true{ //Placeholder, need FrameSummary equivalent
                        break;
                    }
                    self.is_top_frame_ = false;
                }
                if self.inlined_frame_index_ >= 0 {
                    self.frame_inspector_ = Some(FrameInspector::new(ptr::null_mut(), self.inlined_frame_index_, self.isolate_)); // TODO: iterator frame
                    break;
                }
                self.is_top_frame_ = false;
                self.frame_inspector_ = None;
                self.iterator_.advance();
                if self.iterator_.done() {
                    break;
                }
                self.update_inline_frame_index_and_resumable_fn_on_stack();
            }
        }

        fn get_context_id(&self) -> i32 {
            assert!(!self.done());
            //DirectHandle<Object> context = frame_inspector_->GetContext();
            // if (IsContext(*context)) {
            //   Tagged<Object> value =
            //       Cast<Context>(*context)->native_context()->debug_context_id();
            //   if (IsSmi(value)) return Smi::ToInt(value);
            // }
            // return 0;
            0 // Placeholder implementation.
        }

        fn get_receiver(&self) -> Result<(), String> {
            assert!(!self.done());
            // if (frame_inspector_->IsJavaScript() &&
            //   frame_inspector_->GetFunction()->shared()->kind() ==
            //       FunctionKind::kArrowFunction) {
            //   // FrameInspector is not able to get receiver for arrow function.
            //   // So let's try to fetch it using same logic as is used to retrieve 'this'
            //   // during DebugEvaluate::Local.
            //   DirectHandle<JSFunction> function = frame_inspector_->GetFunction();
            //   DirectHandle<Context> context(function->context(), isolate_);
            //   // Arrow function defined in top level function without references to
            //   // variables may have NativeContext as context.
            //   if (!context->IsFunctionContext()) return v8::MaybeLocal<v8::Value>();
            //   ScopeIterator scope_iterator(
            //       isolate_, frame_inspector_.get(),
            //       ScopeIterator::ReparseStrategy::kFunctionLiteral);
            //   // We lookup this variable in function context only when it is used in arrow
            //   // function otherwise V8 can optimize it out.
            //   if (!scope_iterator.ClosureScopeHasThisReference()) {
            //     return v8::MaybeLocal<v8::Value>();
            //   }
            //   DisallowGarbageCollection no_gc;
            //   int slot_index = context->scope_info()->ContextSlotIndex(
            //       isolate_->factory()->this_string());
            //   if (slot_index < 0) return v8::MaybeLocal<v8::Value>();
            //   DirectHandle<Object> value(context->get(slot_index), isolate_);
            //   if (IsTheHole(*value, isolate_)) return v8::MaybeLocal<v8::Value>();
            //   return Utils::ToLocal(value);
            // }

            // DirectHandle<Object> value = frame_inspector_->GetReceiver();
            // if (value.is_null() || (IsSmi(*value) || !IsTheHole(*value, isolate_))) {
            //   return Utils::ToLocal(value);
            // }
            // return v8::MaybeLocal<v8::Value>();
            Ok(()) // Placeholder implementation.
        }

        fn get_return_value(&self) {
            assert!(!self.done());
            // #if V8_ENABLE_WEBASSEMBLY
            // if (frame_inspector_ && frame_inspector_->IsWasm()) {
            //   return v8::Local<v8::Value>();
            // }
            // #endif  // V8_ENABLE_WEBASSEMBLY
            // CHECK_NOT_NULL(iterator_.frame());
            // bool is_optimized = iterator_.frame()->is_optimized_js();
            // if (is_optimized || !is_top_frame_ ||
            //     !isolate_->debug()->IsBreakAtReturn(iterator_.javascript_frame())) {
            //   return v8::Local<v8::Value>();
            // }
            // return Utils::ToLocal(isolate_->debug()->return_value_handle());
            // Placeholder implementation.
        }

        fn get_function_debug_name(&self) {
            assert!(!self.done());
            // return Utils::ToLocal(frame_inspector_->GetFunctionName());
            // Placeholder implementation.
        }

        fn get_script(&self) -> debug::Script {
            assert!(!self.done());
            // DirectHandle<Object> value = frame_inspector_->GetScript();
            // if (!IsScript(*value)) return v8::Local<v8::debug::Script>();
            // return ToApiHandle<debug::Script>(Cast<Script>(value));
            debug::Script{} // Placeholder implementation.
        }

        fn get_source_location(&self) -> debug::Location {
            assert!(!self.done());
            // v8::Local<v8::debug::Script> script = GetScript();
            // if (script.IsEmpty()) return v8::debug::Location();
            // return script->GetSourceLocation(frame_inspector_->GetSourcePosition());
            debug::Location::new(0,0) // Placeholder implementation.
        }

        fn get_function_location(&self) -> debug::Location {
            assert!(!self.done());

            // v8::Local<v8::Function> func = this->GetFunction();
            // if (!func.IsEmpty()) {
            //   return v8::debug::Location(func->GetScriptLineNumber(),
            //                              func->GetScriptColumnNumber());
            // }
            // #if V8_ENABLE_WEBASSEMBLY
            // #if V8_ENABLE_DRUMBRAKE
            // if (iterator_.frame()->is_wasm_interpreter_entry()) {
            //   auto frame = WasmInterpreterEntryFrame::cast(iterator_.frame());
            //   Handle<WasmInstanceObject> instance(frame->wasm_instance(), isolate_);
            //   auto offset =
            //       instance->module()->functions[frame->function_index(0)].code.offset();
            //   return v8::debug::Location(inlined_frame_index_, offset);
            // }
            // #endif  // V8_ENABLE_DRUMBRAKE
            // if (iterator_.frame()->is_wasm()) {
            //   auto frame = WasmFrame::cast(iterator_.frame());
            //   const wasm::WasmModule* module = frame->trusted_instance_data()->module();
            //   auto offset = module->functions[frame->function_index()].code.offset();
            //   return v8::debug::Location(0, offset);
            // }
            // #endif
            // return v8::debug::Location();
            debug::Location::new(0,0) // Placeholder implementation.
        }

        fn get_function(&self) {
            assert!(!self.done());
            // if (!frame_inspector_->IsJavaScript()) return v8::Local<v8::Function>();
            // return Utils::ToLocal(frame_inspector_->GetFunction());
            // Placeholder implementation.
        }

        fn get_shared_function_info(&self) {
            assert!(!self.done());
            // if (!frame_inspector_->IsJavaScript()) return Handle<SharedFunctionInfo>();
            // return handle(frame_inspector_->GetFunction()->shared(), isolate_);
            // Placeholder implementation.
        }

        fn get_scope_iterator(&self) {
            assert!(!self.done());
            // #if V8_ENABLE_WEBASSEMBLY
            // #if V8_ENABLE_DRUMBRAKE
            // if (iterator_.frame()->is_wasm_interpreter_entry()) {
            //   return GetWasmInterpreterScopeIterator(
            //       WasmInterpreterEntryFrame::cast(iterator_.frame()));
            // } else {
            // #endif  // V8_ENABLE_DRUMBRAKE
            //   if (iterator_.frame()->is_wasm()) {
            //     return GetWasmScopeIterator(WasmFrame::cast(iterator_.frame()));
            //   }
            // #if V8_ENABLE_DRUMBRAKE
            // }
            // #endif  // V8_ENABLE_DRUMBRAKE
            // #endif  // V8_ENABLE_WEBASSEMBLY
            // return std::make_unique<DebugScopeIterator>(isolate_, frame_inspector_.get());
            // Placeholder implementation.
        }

        fn can_be_restarted(&self) -> bool {
            assert!(!self.done());

            if self.resumable_fn_on_stack_ {
                return false;
            }

            // StackFrame* frame = iterator_.frame();
            // // We do not support restarting WASM frames.
            // #if V8_ENABLE_WEBASSEMBLY
            // if (frame->is_wasm()) return false;
            // #endif  // V8_ENABLE_WEBASSEMBLY

            // // Check that no embedder API calls are between the top-most frame, and the
            // // current frame. While we *could* determine whether embedder
            // // frames are safe to terminate (via the CallDepthScope chain), we don't know
            // // if embedder frames would cancel the termination effectively breaking
            // // restart frame.
            // if (isolate_->thread_local_top()->last_api_entry_ < frame->fp()) {
            //   return false;
            // }

            // return true;
            true // Placeholder implementation.
        }

        fn update_inline_frame_index_and_resumable_fn_on_stack(&mut self) {
            assert!(!self.iterator_.done());

            // FrameSummaries summaries = iterator_.frame()->Summarize();
            // inlined_frame_index_ = static_cast<int>(summaries.size());
            self.inlined_frame_index_ = 0; // Placeholder.  Needs FrameSummaries implementation.

            if self.resumable_fn_on_stack_ {
                return;
            }

            // StackFrame* frame = iterator_.frame();
            // if (!frame->is_javascript()) return;

            // std::vector<Handle<SharedFunctionInfo>> shareds;
            // JavaScriptFrame::cast(frame)->GetFunctions(&shareds);
            // for (auto& shared : shareds) {
            //   if (IsResumableFunction(shared->kind())) {
            //     resumable_fn_on_stack_ = true;
            //     return;
            //   }
            // }
            // Placeholder implementation.
        }

        fn evaluate(&self) {
            assert!(!self.done());
            // DirectHandle<Object> value;

            // i::SafeForInterruptsScope safe_for_interrupt_scope(isolate_);
            // if (!DebugEvaluate::Local(
            //          isolate_, iterator_.frame()->id(), inlined_frame_index_,
            //          Utils::OpenDirectHandle(*source), throw_on_side_effect)
            //          .ToHandle(&value)) {
            //   return v8::MaybeLocal<v8::Value>();
            // }
            // return Utils::ToLocal(value);
            // Placeholder implementation.
        }

        fn prepare_restart(&self) {
            assert!(!self.done());
            assert!(self.can_be_restarted());

            // isolate_->debug()->PrepareRestartFrame(iterator_.javascript_frame(),
            //                                      inlined_frame_index_);
            // Placeholder implementation.
        }
    }

    // Placeholder structs and impls. Need actual implementation details.
    struct Isolate {}

    struct StackFrameIterator {
        isolate: *mut Isolate,
        done_: bool,
        break_frame_id: i32,
    }

    impl StackFrameIterator {
        fn new(isolate: *mut Isolate, break_frame_id: i32) -> Self {
            StackFrameIterator {
                isolate,
                done_: false,
                break_frame_id,
            }
        }

        fn done(&self) -> bool {
            self.done_
        }

        fn advance(&mut self) {
            self.done_ = true; // Simplest implementation: advance makes it 'done'.
        }
    }

    struct FrameInspector {
        // Holds relevant frame information.
    }

    impl FrameInspector {
        fn new(frame: *mut (), index: i32, isolate: *mut Isolate) -> Self {
            FrameInspector {}
        }
    }
}