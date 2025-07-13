// Converted from V8 C++ source files:
// Header: builtins-lazy-gen.h
// Implementation: builtins-lazy-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
pub mod builtins_lazy_gen {
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use crate::builtins::builtins_utils_gen::*;
use crate::builtins::builtins::*;
use crate::common::globals::*;
use crate::objects::code_inl::*;
use crate::objects::feedback_vector_inl::*;
use crate::objects::shared_function_info::*;
use crate::codegen::code_stub_assembler::*;

    pub struct LazyBuiltinsAssembler {
        pub code_stub_assembler: CodeStubAssembler,
    }

    impl LazyBuiltinsAssembler {
        pub type Descriptor = JSTrampolineDescriptor;

        pub fn new(state: &mut compiler::CodeAssemblerState) -> Self {
            LazyBuiltinsAssembler {
                code_stub_assembler: CodeStubAssembler::new(state),
            }
        }

        pub fn GenerateTailCallToJSCode(&mut self, code: &TNode<Code>, function: &TNode<JSFunction>) {
            let argc = self.code_stub_assembler.UncheckedParameter::<Int32T>(Descriptor::kActualArgumentsCount as usize);
            let context = self.code_stub_assembler.Parameter::<Context>(Descriptor::kContext as usize);
            let new_target = self.code_stub_assembler.Parameter::<Object>(Descriptor::kNewTarget as usize);
            
            let dispatch_handle = self.code_stub_assembler.InvalidDispatchHandleConstant();
            
            self.code_stub_assembler.TailCallJSCode(code, &context, function, &new_target, &argc, dispatch_handle);
        }

        pub fn GenerateTailCallToReturnedCode(&mut self, function_id: Runtime::FunctionId, function: &TNode<JSFunction>) {
            let context = self.code_stub_assembler.Parameter::<Context>(Descriptor::kContext as usize);
            let code = self.code_stub_assembler.CAST::<Code>(self.code_stub_assembler.CallRuntime(function_id, &context, function));
            self.GenerateTailCallToJSCode(&code, function);
        }


        pub fn MaybeTailCallOptimizedCodeSlot(&mut self, function: &TNode<JSFunction>, feedback_vector: &TNode<FeedbackVector>) {
            let mut fallthrough = Label::new("fallthrough".to_string());
            let mut may_have_optimized_code = Label::new("may_have_optimized_code".to_string());
            let mut maybe_needs_logging = Label::new("maybe_needs_logging".to_string());

            let flags = self.code_stub_assembler.LoadObjectField::<Uint16T>(feedback_vector, FeedbackVector::kFlagsOffset as usize);

            // Fall through if no optimization trigger or optimized code.
            let kFlagMask =
                FeedbackVector::FlagMaskForNeedsProcessingCheckFrom(
                    CodeKind::INTERPRETED_FUNCTION
                );
            self.code_stub_assembler.GotoIfNot(self.code_stub_assembler.IsSetWord32(&flags, kFlagMask as u32), &mut fallthrough);

            self.code_stub_assembler.GotoIfNot(
                self.code_stub_assembler.IsSetWord32(&flags, FeedbackVector::kFlagsTieringStateIsAnyRequested as u32),
                &mut maybe_needs_logging
            );
            self.GenerateTailCallToReturnedCode(Runtime::kCompileOptimized, function);

            self.code_stub_assembler.Bind(&mut maybe_needs_logging);
            {
                self.code_stub_assembler.GotoIfNot(self.code_stub_assembler.IsSetWord32(&flags, FeedbackVector::kFlagsLogNextExecution as u32),
                                  &mut may_have_optimized_code);
                self.GenerateTailCallToReturnedCode(Runtime::kFunctionLogNextExecution,
                                       function);
            }

            self.code_stub_assembler.Bind(&mut may_have_optimized_code);
            {
                let mut heal_optimized_code_slot = Label::new("heal_optimized_code_slot".to_string());
                let maybe_optimized_code_entry = self.code_stub_assembler.LoadMaybeWeakObjectField(
                    feedback_vector, FeedbackVector::kMaybeOptimizedCodeOffset as usize
                );

                // Optimized code slot is a weak reference to Code object.
                let code_wrapper = self.code_stub_assembler.CAST::<CodeWrapper>(self.code_stub_assembler.GetHeapObjectAssumeWeak(
                    &maybe_optimized_code_entry, &mut heal_optimized_code_slot
                ));
                let optimized_code =
                    self.code_stub_assembler.LoadCodePointerFromObject(&code_wrapper, CodeWrapper::kCodeOffset as usize);

                // Check if the optimized code is marked for deopt. If it is, call the
                // runtime to clear it.
                self.code_stub_assembler.GotoIf(self.code_stub_assembler.IsMarkedForDeoptimization(&optimized_code),
                           &mut heal_optimized_code_slot);

                // Optimized code is good, get it into the closure and link the closure into
                // the optimized functions list, then tail call the optimized code.
                self.code_stub_assembler.StoreCodePointerField(function, JSFunction::kCodeOffset as usize, &optimized_code);
                self.code_stub_assembler.Comment("MaybeTailCallOptimizedCodeSlot:: GenerateTailCallToJSCode".to_string());
                self.GenerateTailCallToJSCode(&optimized_code, function);

                // Optimized code slot contains deoptimized code, or the code is cleared
                // and tiering state hasn't yet been updated. Evict the code, update the
                // state and re-enter the closure's code.
                self.code_stub_assembler.Bind(&mut heal_optimized_code_slot);
                self.GenerateTailCallToReturnedCode(Runtime::kHealOptimizedCodeSlot, function);
            }

            // Fall-through if the optimized code cell is clear and the tiering state is
            // kNone.
            self.code_stub_assembler.Bind(&mut fallthrough);
        }


        pub fn CompileLazy(&mut self, function: &TNode<JSFunction>) {
            // First lookup code, maybe we don't need to compile!
            let mut compile_function = Label::new("compile_function".to_string());
            compile_function.set_deferred(true);

            // Check the code object for the SFI. If SFI's code entry points to
            // CompileLazy, then we need to lazy compile regardless of the function or
            // tiering state.
            let shared =
                self.code_stub_assembler.CAST::<SharedFunctionInfo>(self.code_stub_assembler.LoadObjectField(function, JSFunction::kSharedFunctionInfoOffset as usize));
            let mut sfi_data_type = TVARIABLE::<Uint16T>::new();
            let sfi_code =
                self.code_stub_assembler.GetSharedFunctionInfoCode(&shared, &mut sfi_data_type, &mut compile_function);

            let feedback_cell_value = self.code_stub_assembler.LoadFeedbackCellValue(function);

            // If feedback cell isn't initialized, compile function
            self.code_stub_assembler.GotoIf(self.code_stub_assembler.IsUndefined(&feedback_cell_value), &mut compile_function);

            self.code_stub_assembler.CSA_DCHECK(self.code_stub_assembler.TaggedNotEqual(&sfi_code, self.code_stub_assembler.HeapConstantNoHole(BUILTIN_CODE(
                                                           /*isolate()*/0, CompileLazy))));
            
            
            self.code_stub_assembler.StoreCodePointerField(function, JSFunction::kCodeOffset as usize, &sfi_code);
            

            let mut maybe_use_sfi_code = Label::new("maybe_use_sfi_code".to_string());
            // If there is no feedback, don't check for optimized code.
            self.code_stub_assembler.GotoIf(self.code_stub_assembler.HasInstanceType(&feedback_cell_value, CLOSURE_FEEDBACK_CELL_ARRAY_TYPE),
                           &mut maybe_use_sfi_code);

            // If it isn't undefined or fixed array it must be a feedback vector.
            self.code_stub_assembler.CSA_DCHECK(self.code_stub_assembler.IsFeedbackVector(&feedback_cell_value));

            
            // Is there a tiering state or optimized code in the feedback vector?
            self.MaybeTailCallOptimizedCodeSlot(function, &self.code_stub_assembler.CAST::<FeedbackVector>(&feedback_cell_value));
            
            self.code_stub_assembler.Goto(&mut maybe_use_sfi_code);

            // At this point we have a candidate InstructionStream object. It's *not* a
            // cached optimized InstructionStream object (we'd have tail-called it above).
            // A usual case would be the InterpreterEntryTrampoline to start executing
            // existing bytecode.
            self.code_stub_assembler.Bind(&mut maybe_use_sfi_code);
            
            // In the leaptiering case, we now simply install the code of the SFI on the
            // function's dispatch table entry and call it. Installing the code is
            // necessary as the dispatch table entry may still contain the CompileLazy
            // builtin at this point (we can only update dispatch table code from C++).
            self.GenerateTailCallToReturnedCode(Runtime::kInstallSFICode, function);
            
            

            self.code_stub_assembler.Bind(&mut compile_function);
            self.GenerateTailCallToReturnedCode(Runtime::kCompileLazy, function);
        }
    }

    pub fn TF_BUILTIN_CompileLazy(assembler: &mut LazyBuiltinsAssembler) {
        let function = assembler.code_stub_assembler.Parameter::<JSFunction>(Descriptor::kTarget as usize);

        assembler.CompileLazy(&function);
    }
}
