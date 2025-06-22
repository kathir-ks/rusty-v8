#![allow(non_snake_case)]
#![allow(dead_code)]

// src/api/api-arguments.h (Placeholder - functionality to be determined)
mod api_arguments {
    // Placeholder module for api-arguments.h functionality.
}

// src/builtins/builtins-descriptors.h (Placeholder - functionality to be determined)
mod builtins_descriptors {
    // Placeholder module for builtins-descriptors.h functionality.
}

// src/builtins/builtins-inl.h (Placeholder - functionality to be determined)
mod builtins_inl {
    // Placeholder module for builtins-inl.h functionality.
}

// src/codegen/code-factory.h (Placeholder - functionality to be determined)
mod code_factory {
    // Placeholder module for code-factory.h functionality.
}

// src/codegen/interface-descriptors-inl.h (Placeholder - functionality to be determined)
mod interface_descriptors_inl {
    // Placeholder module for interface-descriptors-inl.h functionality.
}

// src/debug/debug.h (Placeholder - functionality to be determined)
mod debug {
    // Placeholder module for debug.h functionality.
}

// src/deoptimizer/deoptimizer.h (Placeholder - functionality to be determined)
mod deoptimizer {
    // Placeholder module for deoptimizer.h functionality.
}

// src/execution/frame-constants.h (Placeholder - functionality to be determined)
mod frame_constants {
    pub const K_JS_ARGC_RECEIVER_SLOTS: i32 = 1;
}

// src/execution/frames.h (Placeholder - functionality to be determined)
mod frames {
    pub enum StackFrame {
        ENTRY,
        CONSTRUCT,
        MANUAL,
        INTERNAL,
        CONSTRUCT_ENTRY,
        BASELINE,
        STUB,
        OUTERMOST_JSENTRY_FRAME,
        INNER_JSENTRY_FRAME
    }

    impl StackFrame {
        pub fn type_to_marker(frame_type: StackFrame) -> i32 {
            match frame_type {
                StackFrame::ENTRY => 1, 
                StackFrame::CONSTRUCT => 2,
                StackFrame::MANUAL => 3,
                StackFrame::INTERNAL => 4,
                StackFrame::CONSTRUCT_ENTRY => 5,
                StackFrame::BASELINE => 6,
                StackFrame::STUB => 7,
                StackFrame::OUTERMOST_JSENTRY_FRAME => 8,
                StackFrame::INNER_JSENTRY_FRAME => 9,
            }
        }
    }
}

// src/logging/counters.h (Placeholder - functionality to be determined)
mod counters {
    // Placeholder module for counters.h functionality.
}

// src/codegen/loong64/constants-loong64.h (Placeholder - functionality to be determined)
mod loong64_constants {
    // Placeholder module for loong64 constants.
}

// src/codegen/macro-assembler-inl.h (Placeholder - functionality to be determined)
mod macro_assembler_inl {
    // Placeholder module for macro-assembler-inl.h functionality.
}

// src/codegen/register-configuration.h (Placeholder - functionality to be determined)
mod register_configuration {
    // Placeholder module for register-configuration.h functionality.
}

// src/heap/heap-inl.h (Placeholder - functionality to be determined)
mod heap_inl {
    // Placeholder module for heap-inl.h functionality.
}

// src/objects/cell.h (Placeholder - functionality to be determined)
mod cell {
    // Placeholder module for cell.h functionality.
}

// src/objects/foreign.h (Placeholder - functionality to be determined)
mod foreign {
    // Placeholder module for foreign.h functionality.
}

// src/objects/heap-number.h (Placeholder - functionality to be determined)
mod heap_number {
    // Placeholder module for heap-number.h functionality.
}

// src/objects/js-generator.h (Placeholder - functionality to be determined)
mod js_generator {
    // Placeholder module for js-generator.h functionality.
}

// src/objects/objects-inl.h (Placeholder - functionality to be determined)
mod objects_inl {
    // Placeholder module for objects-inl.h functionality.
}

// src/objects/smi.h (Placeholder - functionality to be determined)
mod smi {
    // Placeholder module for smi.h functionality.
}

// src/runtime/runtime.h (Placeholder - functionality to be determined)
mod runtime {
    // Placeholder module for runtime.h functionality.
}

// src/wasm/baseline/liftoff-assembler-defs.h (Placeholder - functionality to be determined)
#[cfg(feature = "v8_enable_webassembly")]
mod liftoff_assembler_defs {
    // Placeholder module for liftoff-assembler-defs.h functionality.
}

// src/wasm/object-access.h (Placeholder - functionality to be determined)
#[cfg(feature = "v8_enable_webassembly")]
mod wasm_object_access {
    // Placeholder module for object-access.h functionality.
}

// src/wasm/wasm-linkage.h (Placeholder - functionality to be determined)
#[cfg(feature = "v8_enable_webassembly")]
mod wasm_linkage {
    // Placeholder module for wasm-linkage.h functionality.
}

// src/wasm/wasm-objects.h (Placeholder - functionality to be determined)
#[cfg(feature = "v8_enable_webassembly")]
mod wasm_objects {
    // Placeholder module for wasm-objects.h functionality.
}

mod internal {
    use super::frames::StackFrame;
    use frame_constants::K_JS_ARGC_RECEIVER_SLOTS;

    // Placeholder types and constants
    type Address = usize;
    type Register = i32;
    type RootIndex = i32;
    const ZERO_REG: Register = 0;
    const RA: Register = 1;
    const CP: Register = 2;
    const A0: Register = 3;
    const A1: Register = 4;
    const A2: Register = 5;
    const A3: Register = 6;
    const A4: Register = 7;
    const A5: Register = 8;
    const A6: Register = 9;
    const T0: Register = 10;
    const T1: Register = 11;
    const T2: Register = 12;
    const T3: Register = 13;
    const S1: Register = 14;
    const S2: Register = 15;
    const S3: Register = 16;
    const S4: Register = 17;
    const S5: Register = 18;
    const S8: Register = 19;
    const K_JAVASCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = 20;
    const FP: Register = 21;
    const SP: Register = 22;
    const K_SCRATCH_REG: Register = 23;
    const K_ROOT_REGISTER: Register = 24;
    const K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER: Register = 25;
    const K_JAVASCRIPT_CALL_CODE_START_REGISTER: Register = 26;
    const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = 27;

    const CONSTRUCT_FRAME_CONSTANTS_K_CONTEXT_OFFSET: i32 = 8;
    const CONSTRUCT_FRAME_CONSTANTS_K_LENGTH_OFFSET: i32 = 16;
    const CONSTRUCT_FRAME_CONSTANTS_K_CONSTRUCTOR_OFFSET: i32 = 24;

    const FRAME_SCOPE_MANUAL: i32 = 0;
    const FEEDBACK_VECTOR_TYPE: i32 = 1;
    const BYTECODE_ARRAY_TYPE: i32 = 2;

    const JAVASCRIPT_CALL_TYPE: i32 = 0;
    const JS_BOUND_FUNCTION_TYPE: i32 = 1;

    const CODE_TYPE: i32 = 3;
    const INTERPRETER_DATA_TYPE: i32 = 4;
    const FIXED_ARRAY_TYPE: i32 = 5;

    pub struct MemOperand {
        register: Register,
        offset: i32,
    }

    impl MemOperand {
        pub fn new(register: Register, offset: i32) -> Self {
            MemOperand { register, offset }
        }
    }

    pub enum Operand {
        Register(Register),
        Immediate(i32),
    }

    // Placeholder MacroAssembler struct and its methods
    pub struct MacroAssembler {
        isolate: Isolate
    }

    impl MacroAssembler {
        pub fn new(isolate: Isolate) -> Self {
            MacroAssembler{isolate}
        }

        pub fn isolate(&self) -> &Isolate {
            &self.isolate
        }

        pub fn pc_offset(&self) -> i32 {
            0 // Dummy value
        }
    }

    // Placeholder Isolate struct and its methods
    pub struct Isolate {
        heap: Heap
    }

    impl Isolate {
        pub fn new(heap: Heap) -> Self {
            Isolate {heap}
        }

        pub fn heap(&self) -> &Heap {
            &self.heap
        }

        pub fn builtins(&self) -> &Builtins {
            &self.heap.builtins
        }
    }

    // Placeholder Heap struct and its methods
    pub struct Heap {
        interpreter_entry_return_pc_offset: i32,
        construct_stub_create_deopt_pc_offset: i32,
        construct_stub_invoke_deopt_pc_offset: i32,
        builtins: Builtins
    }

    impl Heap {
        pub fn new(builtins: Builtins) -> Self {
            Heap {
                interpreter_entry_return_pc_offset: 0,
                construct_stub_create_deopt_pc_offset: 0,
                construct_stub_invoke_deopt_pc_offset: 0,
                builtins
            }
        }

        pub fn set_interpreter_entry_return_pc_offset(&mut self, offset: i32) {
            self.interpreter_entry_return_pc_offset = offset;
        }

        pub fn interpreter_entry_return_pc_offset(&self) -> DummySmi {
            DummySmi {value: self.interpreter_entry_return_pc_offset}
        }

        pub fn set_construct_stub_create_deopt_pc_offset(&mut self, offset: i32) {
            self.construct_stub_create_deopt_pc_offset = offset;
        }

        pub fn set_construct_stub_invoke_deopt_pc_offset(&mut self, offset: i32) {
            self.construct_stub_invoke_deopt_pc_offset = offset;
        }
    }

    #[derive(Clone, Copy)]
    pub struct DummySmi {
        value: i32
    }

    impl DummySmi {
        pub fn zero() -> Self {
            DummySmi {value: 0}
        }

        pub fn from_int(value: i32) -> Self {
            DummySmi {value}
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    pub enum Builtin {
        AdaptorWithBuiltinExitFrame(i32),
        Call,
        Construct,
        kFastNewObject,
        JSEntryTrampoline,
        JSConstructEntryTrampoline,
        RunMicrotasksTrampoline,
        kThrowConstructedNonConstructable,
        kRunMicrotasks,
        kCompileLazy,
        kInstallBaselineCode,
        kStackGuard,
        kCallWithSpread,
        kConstructWithSpread,
        kArrayConstructorImpl,
        ConstructedNonConstructable,
        kCallWithArrayLike,
        kToObject,
        CallFunction,
        kCallBoundFunction,
        kBaselineOutOfLinePrologue,
        kInterpreterEntryTrampoline
    }

    // Placeholder Builtins struct and its methods
    pub struct Builtins {}

    impl Builtins {
        pub fn new() -> Self {
            Builtins {}
        }

        pub fn adaptor_with_builtin_exit_frame(formal_parameter_count: i32) -> Builtin {
            Builtin::AdaptorWithBuiltinExitFrame(formal_parameter_count)
        }

        pub fn call() -> Builtin {
            Builtin::Call
        }

        pub fn construct() -> Builtin {
            Builtin::Construct
        }

        pub fn call_function(mode: ConvertReceiverMode) -> Builtin {
            Builtin::CallFunction
        }

        pub fn generate_adaptor(
            masm: &mut MacroAssembler,
            formal_parameter_count: i32,
            address: Address,
        ) {
            // __ ACCESS_MASM(masm)
            // __ li(kJavaScriptCallExtraArg1Register, ExternalReference::Create(address));
            // __ TailCallBuiltin(Builtins::AdaptorWithBuiltinExitFrame(formal_parameter_count));
        }

        pub fn generate_js_builtins_construct_stub(masm: &mut MacroAssembler) {
            // Generate_JSBuiltinsConstructStubHelper(masm);
        }

        pub fn generate_js_construct_stub_generic(masm: &mut MacroAssembler) {
            // ----------- S t a t e -------------
            //  --      a0: number of arguments (untagged)
            //  --      a1: constructor function
            //  --      a3: new target
            //  --      cp: context
            //  --      ra: return address
            //  -- sp[...]: constructor arguments
            // -----------------------------------

            // Enter a construct frame.
            // FrameScope scope(masm, StackFrame::MANUAL);
            // Label post_instantiation_deopt_entry, not_create_implicit_receiver;
            // __ EnterFrame(StackFrame::CONSTRUCT);

            // Preserve the incoming parameters on the stack.
            // __ Push(cp, a0, a1);
            // __ PushRoot(RootIndex::kUndefinedValue);
            // __ Push(a3);

            // ----------- S t a t e -------------
            //  --        sp[0*kSystemPointerSize]: new target
            //  --        sp[1*kSystemPointerSize]: padding
            //  -- a1 and sp[2*kSystemPointerSize]: constructor function
            //  --        sp[3*kSystemPointerSize]: number of arguments
            //  --        sp[4*kSystemPointerSize]: context
            // -----------------------------------

            // __ LoadTaggedField(
            //     t2, FieldMemOperand(a1, JSFunction::kSharedFunctionInfoOffset));
            // __ Ld_wu(t2, FieldMemOperand(t2, SharedFunctionInfo::kFlagsOffset));
            // __ DecodeField<SharedFunctionInfo::FunctionKindBits>(t2);
            // __ JumpIfIsInRange(
            //     t2, static_cast<uint32_t>(FunctionKind::kDefaultDerivedConstructor),
            //     static_cast<uint32_t>(FunctionKind::kDerivedConstructor),
            //     &not_create_implicit_receiver);

            // If not derived class constructor: Allocate the new receiver object.
            // __ CallBuiltin(Builtin::kFastNewObject);
            // __ Branch(&post_instantiation_deopt_entry);

            // Else: use TheHoleValue as receiver for constructor call
            // __ bind(&not_create_implicit_receiver);
            // __ LoadRoot(a0, RootIndex::kTheHoleValue);

            // ----------- S t a t e -------------
            //  --                          a0: receiver
            //  -- Slot 4 / sp[0*kSystemPointerSize]: new target
            //  -- Slot 3 / sp[1*kSystemPointerSize]: padding
            //  -- Slot 2 / sp[2*kSystemPointerSize]: constructor function
            //  -- Slot 1 / sp[3*kSystemPointerSize]: number of arguments
            //  -- Slot 0 / sp[4*kSystemPointerSize]: context
            // -----------------------------------
            // Deoptimizer enters here.
            // masm.isolate().heap().set_construct_stub_create_deopt_pc_offset(
            //     masm.pc_offset());
            // __ bind(&post_instantiation_deopt_entry);

            // Restore new target.
            // __ Pop(a3);

            // Push the allocated receiver to the stack.
            // __ Push(a0);

            // We need two copies because we may have to return the original one
            // and the calling conventions dictate that the called function pops the
            // receiver. The second copy is pushed after the arguments, we saved in a6
            // since a0 will store the return value of callRuntime.
            // __ mov(a6, a0);

            // Set up pointer to last argument.
            // __ Add_d(
            //     t2, fp,
            //     Operand(StandardFrameConstants::kCallerSPOffset + kSystemPointerSize));

            // ----------- S t a t e -------------
            //  --                 r3: new target
            //  -- sp[0*kSystemPointerSize]: implicit receiver
            //  -- sp[1*kSystemPointerSize]: implicit receiver
            //  -- sp[2*kSystemPointerSize]: padding
            //  -- sp[3*kSystemPointerSize]: constructor function
            //  -- sp[4*kSystemPointerSize]: number of arguments
            //  -- sp[5*kSystemPointerSize]: context
            // -----------------------------------

            // Restore constructor function and argument count.
            // __ Ld_d(a1, MemOperand(fp, ConstructFrameConstants::kConstructorOffset));
            // __ Ld_d(a0, MemOperand(fp, ConstructFrameConstants::kLengthOffset));

            // Label stack_overflow;
            // __ StackOverflowCheck(a0, t0, t1, &stack_overflow);

            // TODO(victorgomes): When the arguments adaptor is completely removed, we
            // should get the formal parameter count and copy the arguments in its
            // correct position (including any undefined), instead of delaying this to
            // InvokeFunction.

            // Copy arguments and receiver to the expression stack.
            // t2: Pointer to start of argument.
            // a0: Number of arguments.
            // Generate_PushArguments(masm, t2, a0, t0, t1, ArgumentsElementType::kRaw);
            // We need two copies because we may have to return the original one
            // and the calling conventions dictate that the called function pops the
            // receiver. The second copy is pushed after the arguments,
            // __ Push(a6);

            // Call the function.
            // __ InvokeFunctionWithNewTarget(a1, a3, a0, InvokeType::kCall);

            // If the result is an object (in the ECMA sense), we should get rid
            // of the receiver and use the result; see ECMA-262 section 13.2.2-7
            // on page 74.
            // Label use_receiver, do_throw, leave_and_return, check_receiver;

            // If the result is undefined, we jump out to using the implicit receiver.
            // __ JumpIfNotRoot(a0, RootIndex::kUndefinedValue, &check_receiver);

            // Otherwise we do a smi check and fall through to check if the return value
            // is a valid receiver.

            // Throw away the result of the constructor invocation and use the
            // on-stack receiver as the result.
            // __ bind(&use_receiver);
            // __ Ld_d(a0, MemOperand(sp, 0 * kSystemPointerSize));
            // __ JumpIfRoot(a0, RootIndex::kTheHoleValue, &do_throw);

            // __ bind(&leave_and_return);
            // Restore arguments count from the frame.
            // __ Ld_d(a1, MemOperand(fp, ConstructFrameConstants::kLengthOffset));
            // Leave construct frame.
            // __ LeaveFrame(StackFrame::CONSTRUCT);

            // Remove caller arguments from the stack and return.
            // __ DropArguments(a1);
            // __ Ret();

            // __ bind(&check_receiver);
            // __ JumpIfSmi(a0, &use_receiver);

            // Check if the type of the result is not an object in the ECMA sense.
            // __ JumpIfJSAnyIsNotPrimitive(a0, t2, &leave_and_return);
            // __ Branch(&use_receiver);

            // __ bind(&do_throw);
            // Restore the context from the frame.
            // __ Ld_d(cp, MemOperand(fp, ConstructFrameConstants::kContextOffset));
            // __ CallRuntime(Runtime::kThrowConstructorReturnedNonObject);
            // __ break_(0xCC);

            // __ bind(&stack_overflow);
            // Restore the context from the frame.
            // __ Ld_d(cp, MemOperand(fp, ConstructFrameConstants::kContextOffset));
            // __ CallRuntime(Runtime::kThrowStackOverflow);
            // __ break_(0xCC);
        }

        pub fn generate_resume_generator_trampoline(masm: &mut MacroAssembler) {
            // ----------- S t a t e -------------
            //  -- a0 : the value to pass to the generator
            //  -- a1 : the JSGeneratorObject to resume
            //  -- ra : return address
            // -----------------------------------
            // Store input value into generator object.
            // __ StoreTaggedField(
            //     a0, FieldMemOperand(a1, JSGeneratorObject::kInputOrDebugPosOffset));
            // __ RecordWriteField(a1, JSGeneratorObject::kInputOrDebugPosOffset, a0,
            //                     kRAHasNotBeenSaved, SaveFPRegsMode::kIgnore);
            // Check that a1 is still valid, RecordWrite might have clobbered it.
            // __ AssertGeneratorObject(a1);

            // Load suspended function and context.
            // __ LoadTaggedField(a5,
            //                    FieldMemOperand(a1, JSGeneratorObject::kFunctionOffset));
            // __ LoadTaggedField(cp, FieldMemOperand(a5, JSFunction::kContextOffset));

            // Flood function if we are stepping.
            // Label prepare_step_in_if_stepping, prepare_step_in_suspended_generator;
            // Label stepping_prepared;
            // ExternalReference debug_hook =
            //     ExternalReference::debug_hook_on_function_call_address(masm.isolate());
            // __ li(a6, debug_hook);
            // __ Ld_b(a6, MemOperand(a6, 0));
            // __ Branch(&prepare_step_in_if_stepping, ne, a6, Operand(zero_reg));

            // Flood function if we need to continue stepping in the suspended generator.
            // ExternalReference debug_suspended_generator =
            //     ExternalReference::debug_suspended_generator_address(masm.isolate());
            // __ li(a6, debug_suspended_generator);
            // __ Ld_d(a6, MemOperand(a6, 0));
            // __ Branch(&prepare_step_in_suspended_generator, eq, a1, Operand(a6));
            // __ bind(&stepping_prepared);

            // Check the stack for overflow. We are not trying to catch interruptions
            // (i.e. debug break and preemption) here, so check the "real stack limit".
            // Label stack_overflow;
            // __ LoadStackLimit(kScratchReg,
            //                   MacroAssembler::StackLimitKind::kRealStackLimit);
            // __ Branch(&stack_overflow, lo, sp, Operand(kScratchReg));

            // Register argc = kJavaScriptCallArgCountRegister;

            // Compute actual arguments count value as a formal parameter count without
            // receiver, loaded from the dispatch table entry or shared function info.
        }

        pub fn generate_constructed_non_constructable(masm: &mut MacroAssembler) {
            // FrameScope scope(masm, StackFrame::INTERNAL);
            // __ Push(a1);
            // __ CallRuntime(Runtime::kThrowConstructedNonConstructable);
        }

        pub fn generate_js_entry(masm: &mut MacroAssembler) {
            Self::generate_js_entry_variant(masm, StackFrame::ENTRY, Builtin::JSEntryTrampoline);
        }

        pub fn generate_js_construct_entry(masm: &mut MacroAssembler) {
            Self::generate_js_entry_variant(masm, StackFrame::CONSTRUCT_ENTRY, Builtin::JSConstructEntryTrampoline);
        }

        pub fn generate_js_run_microtasks_entry(masm: &mut MacroAssembler) {
            Self::generate_js_entry_variant(masm, StackFrame::ENTRY, Builtin::RunMicrotasksTrampoline);
        }

        fn generate_js_entry_variant(masm: &mut MacroAssembler, type_: StackFrame, entry_trampoline: Builtin) {
            // Generate_JSEntryVariant(masm, StackFrame::Type type, Builtin entry_trampoline)
            // Label invoke, handler_entry, exit;

            // {
            //   NoRootArrayScope no_root_array(masm);

            //   Registers:
            //    either
            //     a0: root register value
            //     a1: entry address
            //     a2: function
            //     a3: receiver
            //     a4: argc
            //     a5: argv
            //    or
            //     a0: root register value
            //     a1: microtask_queue

            //   Save callee saved registers on the stack.
            //   __ MultiPush(kCalleeSaved | ra);

            //   Save callee-saved FPU registers.
            //   __ MultiPushFPU(kCalleeSavedFPU);
            //   Set up the reserved register for 0.0.
            //   __ Move(kDoubleRegZero, 0.0);

            //   Initialize the root register.
            //   C calling convention. The first argument is passed in a0.
            //   __ mov(kRootRegister, a0);

            // #ifdef V8_COMPRESS_POINTERS
            //   Initialize the pointer cage base register.
            //   __ LoadRootRelative(kPtrComprCageBaseRegister,
            //                       IsolateData::cage_base_offset());
            // #endif
            // }

            // a1: entry address
            // a2: function
            // a3: receiver
            // a4: argc
            // a5: argv

            // We build an EntryFrame.
            // __ li(s1, Operand(-1));  // Push a bad frame pointer to fail if it is used.
            // __ li(s2, Operand(StackFrame::TypeToMarker(type)));
            // __ li(s3, Operand(StackFrame::TypeToMarker(type)));
            // ExternalReference c_entry_fp = ExternalReference::Create(
            //     IsolateAddressId::kCEntryFPAddress, masm.isolate());
            // __ li(s5, c_entry_fp);
            // __ Ld_d(s4, MemOperand(s5, 0));
            // __ Push(s1, s2, s3, s4);

            // Clear c_entry_fp, now we've pushed its previous value to the stack.
            // If the c_entry_fp is not already zero and we don't clear it, the
            // StackFrameIteratorForProfiler will assume we are executing C++ and miss the
            // JS frames on top.
            // __ St_d(zero_reg, MemOperand(s5, 0));

            // __ LoadIsolateField(s1, IsolateFieldId::kFastCCallCallerFP);
            // __ Ld_d(s2, MemOperand(s1, 0));
            // __ St_d(zero_reg, MemOperand(s1, 0));
            // __ LoadIsolateField(s1, IsolateFieldId::kFastCCallCallerPC);
            // __ Ld_d(s3, MemOperand(s1, 0));
            // __ St_d(zero_reg, MemOperand(s1, 0));
            // __ Push(s2, s3);

            // Set up frame pointer for the frame to be pushed.
            // __ addi_d(fp, sp, -EntryFrameConstants::kNextFastCallFramePCOffset);

            // Registers:
            //  either
            //   a1: entry address
            //   a2: function
            //   a3: receiver
            //   a4: argc
            //   a5: argv
            //  or
            //   a1: microtask_queue

            // Stack:
            // fast api call pc   |
            // fast api call fp   |
            // C entry FP         |
            // function slot      | entry frame
            // context slot       |
            // bad fp (0xFF...F)  |
            // callee saved registers + ra

            // If this is the outermost JS call, set js_entry_sp value.
            // Label non_outermost_js;
            // ExternalReference js_entry_sp = ExternalReference::Create(
            //     IsolateAddressId::kJSEntrySPAddress, masm.isolate());
            // __ li(s1, js_entry_sp);
            // __ Ld_d(s2, MemOperand(s1, 0));
            // __ Branch(&non_outermost_js, ne, s2, Operand(zero_reg));
            // __ St_d(fp, MemOperand(s1, 0));
            // __ li(s3, Operand(StackFrame::OUTERMOST_JSENTRY_FRAME));
            // Label cont;
            // __ b(&cont);
            // __ nop();  // Branch delay slot nop.
            // __ bind(&non_outermost_js);
            // __ li(s3, Operand(StackFrame::INNER_JSENTRY_FRAME));
            // __ bind(&cont);
            // __ Push(s3);

            // Jump to a faked try block that does the invoke, with a faked catch
            // block that sets the exception.
            // __ jmp(&invoke);
            // __ bind(&handler_entry);

            // Store the current pc as the handler offset. It's used later to create the
            // handler table.
            // masm.isolate().builtins().SetJSEntryHandlerOffset(handler_entry.pos());

            // Caught exception: Store result (exception) in the exception
            // field in the JSEnv and return a failure sentinel.  Coming in here the
            // fp will be invalid because the PushStackHandler below sets it to 0 to
            // signal the existence of the JSEntry frame.
            // __ li(s1, ExternalReference::Create(IsolateAddressId::kExceptionAddress,
            //                                     masm.isolate()));
            // __ St_d(a0,
            //         MemOperand(s1, 0));  // We come back from 'invoke'. result is in a0.
            // __ LoadRoot(a0, RootIndex::kException);
            // __ b(&exit);  // b exposes branch delay slot.
            // __ nop();     // Branch delay slot nop.

            // Invoke: Link this frame into the handler chain.
            // __ bind(&invoke);
            // __ PushStackHandler();
            // If an exception not caught by another handler occurs, this handler
            // returns control to the code after the bal(&invoke) above, which
            // restores all kCalleeSaved registers (including cp and fp) to their
            // saved values before returning a failure to C.
            //
            // Registers:
            //  either
            //   a0: root register value
            //   a1: entry address
            //   a2: function
            //   a3: receiver
            //   a4: argc
            //   a5: argv
            //  or
            //   a0: root register value
            //   a1: microtask_queue
            //
            // Stack:
            // handler frame
            // entry frame
            // fast api call pc
            // fast api call fp
            // C entry FP
            // function slot
            // context slot
            // bad fp (0xFF...F)
            // callee saved registers + ra

            // Invoke the function by calling through JS entry trampoline builtin and
            // pop the faked function when we return.
            // __ CallBuiltin(entry_trampoline);

            // Unlink this frame from the handler chain.
            // __ PopStackHandler();

            // __ bind(&exit);  // a0 holds result
            // Check if the current stack frame is marked as the outermost JS frame.
            // Label non_outermost_js_2;
            // __ Pop(a5);
            // __ Branch(&non_outermost_js_2, ne, a5,
            //           Operand(StackFrame::OUTERMOST_JSENTRY_FRAME));
            // __ li(a5, js_entry_sp);
            // __ St_d(zero_reg, MemOperand(a5, 0));
            // __ bind(&non_outermost_js_2);

            // Restore the top frame descriptors from the stack.
            // __ Pop(a4, a5);
            // __ LoadIsolateField(a6, IsolateFieldId::kFastCCallCallerFP);
            // __ St_d(a4, MemOperand(a6, 0));
            // __ LoadIsolateField(a6, IsolateFieldId::kFastCCallCallerPC);
            // __ St_d(a5, MemOperand(a6, 0));

            // __ Pop(a5);
            // __ li(a4, ExternalReference::Create(IsolateAddressId::kCEntryFPAddress,
            //                                     masm.isolate()));
            // __ St_d(a5, MemOperand(a4, 0));

            // Reset the stack to the callee saved registers.
            // __ addi_d(sp, sp, -EntryFrameConstants::kNextExitFrameFPOffset);

            // Restore callee-saved fpu registers.
            // __ MultiPopFPU(kCalleeSavedFPU);

            // Restore cal