#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

// TODO: Add necessary crate imports based on functionality used
// For example:
// use std::mem;
// use std::rc::Rc;
// use std::cell::RefCell;

mod api {
    // Placeholder for api-arguments.h
    pub struct ApiArguments {}
}

mod builtins {
    pub mod builtins_descriptors {
        // Placeholder for builtins-descriptors.h
        pub struct BuiltinsDescriptors {}
    }

    pub mod builtins_inl {
        // Placeholder for builtins-inl.h
        pub struct BuiltinsInl {}
    }

    use crate::api::ApiArguments;
    use crate::builtins::builtins_descriptors::BuiltinsDescriptors;
    use crate::builtins::builtins_inl::BuiltinsInl;
    use crate::codegen::code_factory::CodeFactory;
    use crate::codegen::interface_descriptors_inl::InterfaceDescriptorsInl;
    use crate::codegen::macro_assembler::MacroAssembler;
    use crate::codegen::register_configuration::RegisterConfiguration;
    use crate::debug::debug::Debug;
    use crate::deoptimizer::deoptimizer::Deoptimizer;
    use crate::execution::frame_constants::FrameConstants;
    use crate::execution::frames::Frames;
    use crate::heap::heap_inl::HeapInl;
    use crate::logging::counters::Counters;
    use crate::objects::cell::Cell;
    use crate::objects::foreign::Foreign;
    use crate::objects::heap_number::HeapNumber;
    use crate::objects::js_generator::JSGeneratorObject;
    use crate::objects::smi::Smi;
    use crate::runtime::runtime::Runtime;

    #[cfg(feature = "v8_enable_webassembly")]
    mod wasm {
        pub mod baseline {
            pub mod liftoff_assembler_defs {
                // Placeholder for liftoff-assembler-defs.h
                pub struct LiftoffAssemblerDefs {}
            }
        }
        pub mod object_access {
            // Placeholder for object-access.h
            pub struct ObjectAccess {}
        }
        pub mod wasm_linkage {
            // Placeholder for wasm-linkage.h
            pub struct WasmLinkage {}
        }
        pub mod wasm_objects {
            // Placeholder for wasm-objects.h
            pub struct WasmObjects {}
        }
    }

    use crate::codegen::code_factory;
    use crate::codegen::interface_descriptors_inl;
    use crate::codegen::macro_assembler;
    use crate::codegen::register_configuration;
    use crate::debug::debug;
    use crate::deoptimizer::deoptimizer;
    use crate::execution::frame_constants;
    use crate::execution::frames;
    use crate::heap::heap_inl;
    use crate::logging::counters;
    use crate::objects::cell;
    use crate::objects::foreign;
    use crate::objects::heap_number;
    use crate::objects::js_generator;
    use crate::objects::smi;
    use crate::runtime::runtime;
    use crate::wasm::*;

    // Assuming this macro expands to some sort of accessor for the MacroAssembler
    macro_rules! ACCESS_MASM {
        ($masm:expr) => {
            $masm
        };
    }

    // Placeholder for V8_TARGET_ARCH_S390X check
    const V8_TARGET_ARCH_S390X: bool = true;

    // Placeholder for ABORT_REASON
    enum AbortReason {
        kExpectedBaselineData,
        kMissingBytecodeArray,
        kInvalidBytecodeAdvance,
    }

    // Placeholder for CODE_TYPE, BYTECODE_ARRAY_TYPE, INTERPRETER_DATA_TYPE, FEEDBACK_VECTOR_TYPE
    const CODE_TYPE: i32 = 1;
    const BYTECODE_ARRAY_TYPE: i32 = 2;
    const INTERPRETER_DATA_TYPE: i32 = 3;
    const FEEDBACK_VECTOR_TYPE: i32 = 4;

    const JS_ARGV_RECEIVER_SLOTS: i32 = 1;

    // Placeholder for no_reg
    struct NoReg {}
    const no_reg: NoReg = NoReg {};

    // Placeholder for Static asserts
    const _: () = assert!(InterpreterFrameConstants::kBytecodeOffsetFromFp == BaselineFrameConstants::kFeedbackCellFromFp);

    const _: () = assert!(InterpreterFrameConstants::kFeedbackVectorFromFp == BaselineFrameConstants::kFeedbackVectorFromFp);

    pub mod Builtins {
        use super::*;
        pub fn Generate_InterpreterOnStackReplacement_ToBaseline(masm: &mut MacroAssembler) {
            let mut start = 0; // Dummy label. Rust does not have labels as first class citizens
                                // so we replace it with u64 counter. This should be implemented properly with control flow graph.
            masm.bind(start);

            // Get function from the frame.
            let closure = 3; // r3

            masm.LoadU64(closure, (FrameConstants::kFunctionOffset).into());

            // Get the InstructionStream object from the shared function info.
            let code_obj = 8; //r8
            masm.LoadTaggedField(
                code_obj,
                FieldMemOperand::new(closure, JSFunction::kSharedFunctionInfoOffset),
            );

            ResetSharedFunctionInfoAge(masm, code_obj, 5);

            masm.LoadTaggedField(
                code_obj,
                FieldMemOperand::new(
                    code_obj,
                    SharedFunctionInfo::kTrustedFunctionDataOffset,
                ),
            );

            // For OSR entry it is safe to assume we always have baseline code.
            if cfg!(debug_assertions) {
                masm.CompareObjectType(code_obj, 5, 5, CODE_TYPE);
                masm.Assert(
                    AbortReason::kExpectedBaselineData,
                    eq,
                );
                AssertCodeIsBaseline(masm, code_obj, 5);
            }

            // Load the feedback cell and vector.
            let feedback_cell = 4; // r4
            let feedback_vector = 1; // r1
            masm.LoadTaggedField(
                feedback_cell,
                FieldMemOperand::new(closure, JSFunction::kFeedbackCellOffset),
            );
            masm.LoadTaggedField(
                feedback_vector,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kValueOffset),
            );

            let mut install_baseline_code = 1;
            // Check if feedback vector is valid. If not, call prepare for baseline to
            // allocate it.
            masm.CompareObjectType(feedback_vector, 5, 5, FEEDBACK_VECTOR_TYPE);
            masm.b(neq, install_baseline_code);

            // Save BytecodeOffset from the stack frame.
            masm.LoadU64(kInterpreterBytecodeOffsetRegister,
                         InterpreterFrameConstants::kBytecodeOffsetFromFp.into());
            masm.SmiUntag(kInterpreterBytecodeOffsetRegister);
            // Replace bytecode offset with feedback cell.
            masm.StoreU64(feedback_cell, BaselineFrameConstants::kFeedbackCellFromFp.into());
            // Update feedback vector cache.
            masm.StoreU64(feedback_vector, InterpreterFrameConstants::kFeedbackVectorFromFp.into());
            // Compute baseline pc for bytecode offset.
            let get_baseline_pc = 5; // r5
            masm.Move(get_baseline_pc,
                      ExternalReference::baseline_pc_for_next_executed_bytecode());

            masm.SubS64(kInterpreterBytecodeOffsetRegister,
                      kInterpreterBytecodeOffsetRegister,
                      BytecodeArray::kHeaderSize - kHeapObjectTag);

            // Get bytecode array from the stack frame.
            masm.LoadU64(kInterpreterBytecodeArrayRegister, InterpreterFrameConstants::kBytecodeArrayFromFp.into());
            // Save the accumulator register, since it's clobbered by the below call.
            masm.Push(kInterpreterAccumulatorRegister);
            {
                masm.mov(kCArgRegs[0], code_obj);
                masm.mov(kCArgRegs[1], kInterpreterBytecodeOffsetRegister);
                masm.mov(kCArgRegs[2], kInterpreterBytecodeArrayRegister);
                //FrameScope scope(masm, StackFrame::INTERNAL);
                masm.PrepareCallCFunction(3, 0, 1); //r1
                masm.CallCFunction(get_baseline_pc, 3, 0);
            }
            masm.LoadCodeInstructionStart(code_obj, code_obj);
            masm.AddS64(code_obj, code_obj, kReturnRegister0);
            masm.Pop(kInterpreterAccumulatorRegister);

            Generate_OSREntry(masm, code_obj, Operand::new(0));
            masm.Trap();  // Unreachable.

            masm.bind(install_baseline_code);
            {
                //FrameScope scope(masm, StackFrame::INTERNAL);
                masm.Push(kInterpreterAccumulatorRegister);
                masm.Push(closure);
                masm.CallRuntime(Runtime::kInstallBaselineCode, 1);
                masm.Pop(kInterpreterAccumulatorRegister);
            }
            // Retry from the start after installing baseline code.
            masm.b(start);
        }
        // TODO: Implement the rest of the Builtins methods
        fn Generate_Adaptor(masm: &mut MacroAssembler, formal_parameter_count: i32, address: usize){
            masm.Move(kJavaScriptCallExtraArg1Register, ExternalReference::Create(address));
            masm.TailCallBuiltin(
                Builtins::AdaptorWithBuiltinExitFrame(formal_parameter_count as i32)
            );
        }
        fn Generate_JSBuiltinsConstructStubHelper(masm: &mut MacroAssembler){
            // ----------- S t a t e -------------
            //  -- r2     : number of arguments
            //  -- r3     : constructor function
            //  -- r5     : new target
            //  -- cp     : context
            //  -- lr     : return address
            //  -- sp[...]: constructor arguments
            // -----------------------------------

            let scratch = 4; // r4
            let mut stack_overflow = 1;

            masm.StackOverflowCheck(2, scratch, stack_overflow);

            // Enter a construct frame.
            {
                //FrameAndConstantPoolScope scope(masm, StackFrame::CONSTRUCT);

                // Preserve the incoming parameters on the stack.
                masm.Push(13, 2); // cp, r2

                // TODO(victorgomes): When the arguments adaptor is completely removed, we
                // should get the formal parameter count and copy the arguments in its
                // correct position (including any undefined), instead of delaying this to
                // InvokeFunction.

                // Set up pointer to first argument (skip receiver).
                masm.la(6, MemOperand::new(11, FrameConstants::kCallerSPOffset + kSystemPointerSize)); // r6
                // Copy arguments and receiver to the expression stack.
                // r6: Pointer to start of arguments.
                // r2: Number of arguments.
                Generate_PushArguments(masm, 6, 2, 1, ArgumentsElementType::kRaw);

                // The receiver for the builtin/api call.
                masm.PushRoot(RootIndex::kTheHoleValue);

                // Call the function.
                // r2: number of arguments
                // r3: constructor function
                // r5: new target
                masm.InvokeFunctionWithNewTarget(3, 5, 2, InvokeType::kCall);

                // Restore context from the frame.
                masm.LoadU64(13, MemOperand::new(11, ConstructFrameConstants::kContextOffset)); // cp
                // Restore arguments count from the frame.
                masm.LoadU64(scratch, MemOperand::new(11, ConstructFrameConstants::kLengthOffset));

                // Leave construct frame.
            }
            // Remove caller arguments from the stack and return.
            masm.DropArguments(scratch);
            masm.Ret();

            masm.bind(stack_overflow);
            {
                //FrameScope scope(masm, StackFrame::INTERNAL);
                masm.CallRuntime(Runtime::kThrowStackOverflow, 0);
                masm.bkpt(0);  // Unreachable code.
            }
        }
        fn Generate_JSConstructStubGeneric(masm: &mut MacroAssembler){
            // ----------- S t a t e -------------
            //  --      r2: number of arguments (untagged)
            //  --      r3: constructor function
            //  --      r5: new target
            //  --      cp: context
            //  --      lr: return address
            //  -- sp[...]: constructor arguments
            // -----------------------------------
            let post_instantiation_deopt_entry=1;
            let mut not_create_implicit_receiver=2;
            let mut stack_overflow=3;
            //FrameScope scope(masm, StackFrame::MANUAL);
            // Enter a construct frame.
            masm.EnterFrame(StackFrame::CONSTRUCT);

            // Preserve the incoming parameters on the stack.
            masm.Push(13, 2, 3); // cp, r2, r3
            masm.PushRoot(RootIndex::kUndefinedValue);
            masm.Push(5); // r5

            // ----------- S t a t e -------------
            //  --        sp[0*kSystemPointerSize]: new target
            //  --        sp[1*kSystemPointerSize]: padding
            //  -- r3 and sp[2*kSystemPointerSize]: constructor function
            //  --        sp[3*kSystemPointerSize]: number of arguments
            //  --        sp[4*kSystemPointerSize]: context
            // -----------------------------------

            masm.LoadTaggedField(
                6, FieldMemOperand::new(3, JSFunction::kSharedFunctionInfoOffset)); // r6
            masm.LoadU32(6, FieldMemOperand::new(6, SharedFunctionInfo::kFlagsOffset));
            masm.DecodeField::<SharedFunctionInfo::FunctionKindBits>(6);
            masm.JumpIfIsInRange(
                6, 6, FunctionKind::kDefaultDerivedConstructor as u8,
                FunctionKind::kDerivedConstructor as u8,
                not_create_implicit_receiver,
            );

            // If not derived class constructor: Allocate the new receiver object.
            masm.CallBuiltin(Builtin::kFastNewObject);
            masm.b(post_instantiation_deopt_entry);

            // Else: use TheHoleValue as receiver for constructor call
            masm.bind(not_create_implicit_receiver);
            masm.LoadRoot(2, RootIndex::kTheHoleValue);

            // ----------- S t a t e -------------
            //  --                          r2: receiver
            //  -- Slot 4 / sp[0*kSystemPointerSize]: new target
            //  -- Slot 3 / sp[1*kSystemPointerSize]: padding
            //  -- Slot 2 / sp[2*kSystemPointerSize]: constructor function
            //  -- Slot 1 / sp[3*kSystemPointerSize]: number of arguments
            //  -- Slot 0 / sp[4*kSystemPointerSize]: context
            // -----------------------------------
            // Deoptimizer enters here.
            //masm.isolate()->heap()->SetConstructStubCreateDeoptPCOffset(
             //   masm.pc_offset());
            masm.bind(post_instantiation_deopt_entry);

            // Restore new target.
            masm.Pop(5); // r5

            // Push the allocated receiver to the stack.
            masm.Push(2); // r2
            // We need two copies because we may have to return the original one
            // and the calling conventions dictate that the called function pops the
            // receiver. The second copy is pushed after the arguments, we saved in r6
            // since r0 needs to store the number of arguments before
            // InvokingFunction.
            masm.mov(8, 2); // r8

            // Set up pointer to first argument (skip receiver).
            masm.la(6, MemOperand::new(11, FrameConstants::kCallerSPOffset + kSystemPointerSize)); // r6

            // ----------- S t a t e -------------
            //  --                 r5: new target
            //  -- sp[0*kSystemPointerSize]: implicit receiver
            //  -- sp[1*kSystemPointerSize]: implicit receiver
            //  -- sp[2*kSystemPointerSize]: padding
            //  -- sp[3*kSystemPointerSize]: constructor function
            //  -- sp[4*kSystemPointerSize]: number of arguments
            //  -- sp[5*kSystemPointerSize]: context
            // -----------------------------------

            // Restore constructor function and argument count.
            masm.LoadU64(3, MemOperand::new(11, ConstructFrameConstants::kConstructorOffset)); // r3
            masm.LoadU64(2, MemOperand::new(11, ConstructFrameConstants::kLengthOffset)); // r2

            masm.StackOverflowCheck(2, 7, stack_overflow);

            // Copy arguments and receiver to the expression stack.
            // r6: Pointer to start of argument.
            // r2: Number of arguments.
            Generate_PushArguments(masm, 6, 2, 1, ArgumentsElementType::kRaw);

            // Push implicit receiver.
            masm.Push(8); // r8

            // Call the function.
            masm.InvokeFunctionWithNewTarget(3, 5, 2, InvokeType::kCall);

            // If the result is an object (in the ECMA sense), we should get rid
            // of the receiver and use the result; see ECMA-262 section 13.2.2-7
            // on page 74.
            let mut use_receiver=4;
            let mut do_throw=5;
            let mut leave_and_return=6;
            let mut check_receiver=7;

            // If the result is undefined, we jump out to using the implicit receiver.
            masm.JumpIfNotRoot(2, RootIndex::kUndefinedValue, check_receiver);

            // Otherwise we do a smi check and fall through to check if the return value
            // is a valid receiver.

            // Throw away the result of the constructor invocation and use the
            // on-stack receiver as the result.
            masm.bind(use_receiver);
            masm.LoadU64(2, MemOperand::new(11, 0)); // r2
            masm.JumpIfRoot(2, RootIndex::kTheHoleValue, do_throw);

            masm.bind(leave_and_return);
            // Restore arguments count from the frame.
            masm.LoadU64(3, MemOperand::new(11, ConstructFrameConstants::kLengthOffset)); // r3
            // Leave construct frame.
            masm.LeaveFrame(StackFrame::CONSTRUCT);

            // Remove caller arguments from the stack and return.
            masm.DropArguments(3);
            masm.Ret();

            masm.bind(check_receiver);
            // If the result is a smi, it is *not* an object in the ECMA sense.
            masm.JumpIfSmi(2, use_receiver);

            // If the type of the result (stored in its map) is less than
            // FIRST_JS_RECEIVER_TYPE, it is not an object in the ECMA sense.
            static_assert!(LAST_JS_RECEIVER_TYPE == LAST_TYPE);
            masm.CompareObjectType(2, 6, 7, FIRST_JS_RECEIVER_TYPE); // r2,r6,r7
            masm.b(ge, leave_and_return);
            masm.b(use_receiver);

            masm.bind(do_throw);
            // Restore the context from the frame.
            masm.LoadU64(13, MemOperand::new(11, ConstructFrameConstants::kContextOffset)); //cp
            masm.CallRuntime(Runtime::kThrowConstructorReturnedNonObject, 0);
            masm.bkpt(0);

            masm.bind(stack_overflow);
            // Restore the context from the frame.
            masm.LoadU64(13, MemOperand::new(11, ConstructFrameConstants::kContextOffset)); //cp
            masm.CallRuntime(Runtime::kThrowStackOverflow, 0);
            // Unreachable code.
            masm.bkpt(0);
        }
        fn Generate_ResumeGeneratorTrampoline(masm: &mut MacroAssembler){
            // ----------- S t a t e -------------
            //  -- r2 : the value to pass to the generator
            //  -- r3 : the JSGeneratorObject to resume
            //  -- lr : return address
            // -----------------------------------
            // Store input value into generator object.
            masm.StoreTaggedField(
                2, FieldMemOperand::new(3, JSGeneratorObject::kInputOrDebugPosOffset), 0,
            );
            masm.RecordWriteField(3, JSGeneratorObject::kInputOrDebugPosOffset, 2, 5,
                                 kLRHasNotBeenSaved, SaveFPRegsMode::kIgnore);
            // Check that r3 is still valid, RecordWrite might have clobbered it.
            masm.AssertGeneratorObject(3);

            // Load suspended function and context.
            masm.LoadTaggedField(6,
                                 FieldMemOperand::new(3, JSGeneratorObject::kFunctionOffset));
            masm.LoadTaggedField(13, FieldMemOperand::new(6, JSFunction::kContextOffset)); // cp

            // Flood function if we are stepping.
            let mut prepare_step_in_if_stepping=1;
            let mut prepare_step_in_suspended_generator=2;
            let mut stepping_prepared=3;
            let scratch = 7; // r7

            let debug_hook =
                ExternalReference::debug_hook_on_function_call_address(1); //masm.isolate());
            masm.Move(scratch, debug_hook);
            masm.LoadS8(scratch, MemOperand::new(scratch, 0));
            masm.CmpSmiLiteral(scratch, Smi::zero(), 0); //r0
            masm.b(neq, prepare_step_in_if_stepping);

            // Flood function if we need to continue stepping in the suspended generator.
            let debug_suspended_generator =
                ExternalReference::debug_suspended_generator_address(1); //masm.isolate());

            masm.Move(scratch, debug_suspended_generator);
            masm.LoadU64(scratch, MemOperand::new(scratch, 0));
            masm.CmpS64(scratch, 3); // r3
            masm.beq(prepare_step_in_suspended_generator);
            masm.bind(stepping_prepared);

            // Check the stack for overflow. We are not trying to catch interruptions
            // (i.e. debug break and preemption) here, so check the "real stack limit".
            let mut stack_overflow=4;
            masm.LoadU64(scratch, MacroAssembler::StackLimitAsMemOperand(StackLimitKind::kRealStackLimit).into()); // __ StackLimitAsMemOperand(StackLimitKind::kRealStackLimit));
            masm.CmpU64(11, scratch);
            masm.blt(stack_overflow);

            // ----------- S t a t e -------------
            //  -- r3    : the JSGeneratorObject to resume
            //  -- r6    : generator function
            //  -- cp    : generator context
            //  -- lr    : return address
            // -----------------------------------

            // Copy the function arguments from the generator object's register file.
            masm.LoadTaggedField(
                5, FieldMemOperand::new(6, JSFunction::kSharedFunctionInfoOffset));
            masm.LoadU16(
                5, FieldMemOperand::new(5, SharedFunctionInfo::kFormalParameterCountOffset));
            masm.SubS64(5, 5, JS_ARGV_RECEIVER_SLOTS);
            masm.LoadTaggedField(
                4,
                FieldMemOperand::new(3, JSGeneratorObject::kParametersAndRegistersOffset));
            {
                let mut done_loop=5;
                let mut loop_b=6;
                masm.bind(loop_b);
                masm.SubS64(5, 5, 1);
                masm.blt(done_loop);
                masm.ShiftLeftU64(1, 5, kTaggedSizeLog2);
                masm.la(scratch, MemOperand::new(4, 1));
                masm.LoadTaggedField(
                    scratch, FieldMemOperand::new(scratch, FixedArray::OffsetOfElementAt(0)));
                masm.Push(scratch);
                masm.b(loop_b);
                masm.bind(done_loop);

                // Push receiver.
                masm.LoadTaggedField(scratch,
                                     FieldMemOperand::new(3, JSGeneratorObject::kReceiverOffset));
                masm.Push(scratch);
            }

            // Underlying function needs to have bytecode available.
            if cfg!(debug_assertions) {
                let mut is_baseline=7;
                let mut is_unavailable=8;
                let mut ok=9;
                masm.LoadTaggedField(
                    5, FieldMemOperand::new(6, JSFunction::kSharedFunctionInfoOffset));
                GetSharedFunctionInfoBytecodeOrBaseline(masm, 5, 5, 1, is_baseline,
                                                        is_unavailable);
                masm.jmp(ok);

                masm.bind(is_unavailable);
                masm.Abort(AbortReason::kMissingBytecodeArray);

                masm.bind(is_baseline);
                masm.CompareObjectType(5, 5, 5, CODE_TYPE);
                masm.Assert(AbortReason::kMissingBytecodeArray, eq);

                masm.bind(ok);
            }

            // Resume (Ignition/TurboFan) generator object.
            {
                masm.LoadTaggedField(
                    2, FieldMemOperand::new(6, JSFunction::kSharedFunctionInfoOffset));
                masm.LoadS16(2, FieldMemOperand::new(
                    2, SharedFunctionInfo::kFormalParameterCountOffset));
                // We abuse new.target both to indicate that this is a resume call and to
                // pass in the generator object.  In ordinary calls, new.target is always
                // undefined because generator functions are non-constructable.
                masm.mov(5, 3);
                masm.mov(3, 6);
                masm.JumpJSFunction(3);
            }

            masm.bind(prepare_step_in_if_stepping);
            {
                //FrameAndConstantPoolScope scope(masm, StackFrame::INTERNAL);
                masm.Push(3, 6);
                // Push hole as receiver since we do not use it for stepping.
                masm.PushRoot(RootIndex::kTheHoleValue);
                masm.CallRuntime(Runtime::kDebugOnFunctionCall, 0);
                masm.Pop(3);
                masm.LoadTaggedField(6,
                                     FieldMemOperand::new(3, JSGeneratorObject::kFunctionOffset));
            }
            masm.b(stepping_prepared);

            masm.bind(prepare_step_in_suspended_generator);
            {
                //FrameAndConstantPoolScope scope(masm, StackFrame::INTERNAL);
                masm.Push(3);
                masm.CallRuntime(Runtime::kDebugPrepareStepInSuspendedGenerator, 0);
                masm.Pop(3);
                masm.LoadTaggedField(6,
                                     FieldMemOperand::new(3, JSGeneratorObject::kFunctionOffset));
            }
            masm.b(stepping_prepared);

            masm.bind(stack_overflow);
            {
                //FrameScope scope(masm, StackFrame::INTERNAL);
                masm.CallRuntime(Runtime::kThrowStackOverflow, 0);
                masm.bkpt(0);  // This should be unreachable.
            }
        }
        fn Generate_ConstructedNonConstructable(masm: &mut MacroAssembler){
            //FrameAndConstantPoolScope scope(masm, StackFrame::INTERNAL);
            masm.push(3); // r3
            masm.CallRuntime(Runtime::kThrowConstructedNonConstructable, 0);
            masm.Trap();  // Unreachable.
        }
        fn Generate_JSEntry(masm: &mut MacroAssembler){
            Generate_JSEntryVariant(masm, StackFrame::ENTRY, Builtin::kJSEntryTrampoline);
        }
        fn Generate_JSConstructEntry(masm: &mut MacroAssembler){
            Generate_JSEntryVariant(masm, StackFrame::CONSTRUCT_ENTRY,
                                     Builtin::kJSConstructEntryTrampoline);
        }
        fn Generate_JSRunMicrotasksEntry(masm: &mut MacroAssembler){
            Generate_JSEntryVariant(masm, StackFrame::ENTRY,
                                     Builtin::kRunMicrotasksTrampoline);
        }
        fn Generate_JSEntryTrampoline(masm: &mut MacroAssembler){
            Generate_JSEntryTrampolineHelper(masm, false);
        }
        fn Generate_JSConstructEntryTrampoline(masm: &mut MacroAssembler){
            Generate_JSEntryTrampolineHelper(masm, true);
        }
        fn Generate_RunMicrotasksTrampoline(masm: &mut MacroAssembler){
            // This expects two C++ function parameters passed by Invoke() in
            // execution.cc.
            //   r2: root_register_value
            //   r3: microtask_queue
            masm.mov(RunMicrotasksDescriptor::MicrotaskQueueRegister(), 3); //r3
            masm.TailCallBuiltin(Builtin::kRunMicrotasks);
        }
        fn Generate_BaselineOutOfLinePrologue(masm: &mut MacroAssembler){
            // UseScratchRegisterScope temps(masm);
            // Need a few extra registers
            // temps.Include(r8, r9);

            let descriptor =
                Builtins::CallInterfaceDescriptorFor(Builtin::kBaselineOutOfLinePrologue);
            let closure = descriptor.GetRegisterParameter(
                BaselineOutOfLinePrologueDescriptor::kClosure);
            // Load the feedback cell and vector from the closure.
            let feedback_cell = 6; // r6
            let feedback_vector = 9; //ip
            masm.LoadTaggedField(feedback_cell,
                                 FieldMemOperand::new(closure, JSFunction::kFeedbackCellOffset));
            masm.LoadTaggedField(
                feedback_vector,
                FieldMemOperand::new(feedback_cell, FeedbackCell::kValueOffset));
            masm.AssertFeedbackVector(feedback_vector, 1);

            //#ifndef V8_ENABLE_LEAPTIERING
            // Check for an tiering state.
            let mut flags_need_processing = 1;
            let flags = 8; // r8
            {
                masm.LoadFeedbackVectorFlagsAndJumpIfNeedsProcessing(
                    flags, feedback_vector, CodeKind::BASELINE, flags_need_processing);
            }
            //#endif  // !V8_ENABLE_LEAPTIERING

            {
                //UseScratchRegisterScope temps(masm);
                ResetFeedbackVectorOsrUrgency(masm, feedback_vector, 1);
            }

            // Increment invocation count for the function.
            {
                let invocation_count = 1; // r1
                masm.LoadU32(invocation_count,
                             FieldMemOperand::new(feedback_vector,
                                                     FeedbackVector::kInvocationCountOffset));
                masm.AddU32(invocation_count, 1);
                masm.StoreU32(invocation_count,
                             FieldMemOperand::new(feedback_vector,
                                                     FeedbackVector::kInvocationCountOffset));
            }

            //FrameScope frame_scope(masm, StackFrame::MANUAL);
            {
                masm.RecordComment("--- Frame Setup");
                // Normally the first thing we'd do here is Push(lr, fp), but we already
                // entered the frame in BaselineCompiler::Prologue, as we had to use the
                // value lr before the call to this BaselineOutOfLinePrologue builtin.

                let callee_context = descriptor.GetRegisterParameter(
                    BaselineOutOfLinePrologueDescriptor::kCalleeContext);
                let callee_js_function = descriptor.GetRegisterParameter(
                    BaselineOutOfLinePrologueDescriptor::kCalleeContext);
                ResetJSFunctionAge(masm, callee_js_function, 1, 0);
                masm.Push(callee_context, callee_js_function);
                //DCHECK_EQ(callee_js_function, kJavaScriptCallTargetRegister);
                //DCHECK_EQ(callee_js_function, kJSFunctionRegister);

                let argc = descriptor.GetRegisterParameter(
                    BaselineOutOfLinePrologueDescriptor::kJavaScriptCallArgCount);
                // We'll use the bytecode for both code age/OSR resetting, and pushing onto
                // the frame, so load it into a register.
                let bytecodeArray = descriptor.GetRegisterParameter(
                    BaselineOutOfLinePrologueDescriptor::kInterpreterBytecodeArray);

                masm.Push(argc, bytecodeArray);

                if cfg!(debug_assertions) {
                    let scratch = 1;
                    masm.CompareObjectType(feedback_vector, scratch, scratch,
                                         FEEDBACK_VECTOR_TYPE);
                    masm.Assert(AbortReason::kExpectedFeedbackVector, eq);
                }
                masm.Push(feedback_cell);
                masm.Push(feedback_vector);
            }

            let mut call_stack_guard = 2;
            let frame_size = descriptor.GetRegisterParameter(
                BaselineOutOfLinePrologueDescriptor::kStackFrameSize);
            {
                masm.RecordComment