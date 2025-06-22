// TODO: Add necessary crate imports (e.g., `libc`, `bitflags`).

// pub mod codegen {
//     pub mod interface_descriptors_inl; // Implement this module
// }
// pub mod deoptimizer {
//     pub mod deoptimizer; // Implement this module
// }
// pub mod maglev {
//     pub mod maglev_assembler_inl; // Implement this module
//     pub mod maglev_graph; // Implement this module
// }

pub mod maglev_s390 {
    // use crate::codegen::interface_descriptors_inl::*;
    // use crate::deoptimizer::deoptimizer::*;
    // use crate::maglev::maglev_assembler_inl::*;
    // use crate::maglev::maglev_graph::*;

    // Assuming these constants are defined elsewhere, mimicking C++ defines
    const K_HEAP_OBJECT_TAG: i64 = 0; // Replace with actual value if known
    const K_SYSTEM_POINTER_SIZE: i32 = 8; //Size of a pointer on the target system
    const K_DOUBLE_SIZE: i32 = 8; //Size of a double (64-bit floating point number)

    macro_rules! offset_of_data_start {
        ($struct_name:ident) => {
            0 // Replace with actual offset calculation if needed
        };
    }

    // Placeholder enums and structs. Replace with actual definitions.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AllocationType {
        kOld,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AllocationAlignment {
        kTaggedAligned,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AbortReason {
        kOsrUnexpectedStackSize,
        kUnexpectedValue,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RootIndex {
        kSingleCharacterStringTable,
        kempty_string,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Builtin {
        kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
        kDoubleToI,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CodeKind {
        MAGLEV,
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BuiltinStringPrototypeCharCodeOrCodePointAt {
        kCharCodeAt,
        kCodePointAt,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CharCodeMaskMode {
        kMustApplyMask
    }

    struct Isolate {}
    struct Graph {}

    impl Graph {
        fn is_osr(&self) -> bool {
            false // Placeholder
        }
        fn has_recursive_calls(&self) -> bool {
            false // Placeholder
        }
        fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {
            0 // Placeholder
        }
        fn tagged_stack_slots(&self) -> u32 {
            0 // Placeholder
        }
        fn untagged_stack_slots(&self) -> u32 {
            0 // Placeholder
        }
    }

    struct CompilationInfo {}
    impl CompilationInfo {
        fn toplevel_compilation_unit(&self) -> CompilationUnit {
            CompilationUnit{}
        }
    }
    
    struct CompilationUnit {}
    impl CompilationUnit {
        fn feedback(&self) -> Feedback {
            Feedback{}
        }
    }
    
    struct Feedback {}
    impl Feedback {
        fn object(&self) -> i32 {
            0 // Placeholder
        }
    }
    

    struct ZoneLabelRef {} // Placeholder

    struct RegisterSnapshot {
        live_registers: RegisterSet,
        live_tagged_registers: RegisterSet,
    }
    
    impl RegisterSnapshot {
        fn new() -> Self {
            RegisterSnapshot {
                live_registers: RegisterSet::new(),
                live_tagged_registers: RegisterSet::new(),
            }
        }
    }

    struct SaveRegisterStateForCall<'a> {
        masm: &'a mut MaglevAssembler,
        register_snapshot: RegisterSnapshot,
    }
    
    impl<'a> SaveRegisterStateForCall<'a> {
        fn new(masm: &'a mut MaglevAssembler, register_snapshot: RegisterSnapshot) -> Self {
            SaveRegisterStateForCall { masm, register_snapshot }
        }
        
        fn DefineSafepoint(&self) {}
    }

    struct MaglevAssembler {
        isolate_: Isolate,
        compilation_info_: CompilationInfo,
        code_gen_state_: CodeGenState,
        allow_allocate_: bool, // Added field
    }
    
    impl MaglevAssembler {
        fn new(isolate: Isolate, compilation_info: CompilationInfo) -> Self {
            MaglevAssembler {
                isolate_: isolate,
                compilation_info_: compilation_info,
                code_gen_state_: CodeGenState{},
                allow_allocate_: true,
            }
        }

        fn native_context(&mut self) -> NativeContext {
            NativeContext {}
        }

        fn allow_allocate(&self) -> bool {
            self.allow_allocate_
        }
    }

    struct CodeGenState {
    }

    impl CodeGenState {
        fn entry_label(&self) -> Label {
            Label{}
        }
    }
    
    struct NativeContext {}
    impl NativeContext {
        fn object(&self) -> i32 {
            0 // Placeholder
        }
    }

    // Flags placeholder struct. Replace with actual implementation.
    pub struct Flags {
        pub single_generation: bool,
        pub debug_code: bool,
        pub turbofan: bool,
    }
    
    // Mock implementation of flags
    static mut FLAGS: Flags = Flags {
        single_generation: false,
        debug_code: false,
        turbofan: false,
    };
    
    // Mock implementation of flags getter
    pub fn v8_flags() -> &'static Flags {
        unsafe { &FLAGS }
    }

    struct RegisterSet {
        // Placeholder bitset.  Replace with a bitset implementation.
    }
    
    impl RegisterSet {
        fn new() -> Self {
            RegisterSet {} // Placeholder
        }

        fn has(&self, _reg: Register) -> bool {
            false // Placeholder
        }

        fn set(&mut self, _reg: Register) {}
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Register(i32);
    impl Register {
        const r1: Self = Register(1);
        const r6: Self = Register(6);
        const r8: Self = Register(8);
        const sp: Self = Register(13);
        const fp: Self = Register(14);
        const kJavaScriptCallArgCountRegister: Self = Register(15);
        const kJSFunctionRegister: Self = Register(16);
        const kContextRegister: Self = Register(17);
        const kJavaScriptCallNewTargetRegister: Self = Register(18);
        const kReturnRegister0: Self = Register(19);
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct DoubleRegister(i32);

    // Dummy implementation of scratch register allocation.
    struct TemporaryRegisterScope<'a> {
        masm: &'a mut MaglevAssembler,
        excluded: RegisterSet,
    }
    
    impl<'a> TemporaryRegisterScope<'a> {
        fn new(masm: &'a mut MaglevAssembler) -> Self {
            TemporaryRegisterScope { masm, excluded: RegisterSet::new() }
        }

        fn Include( &mut self, regs: &[Register]) {
           //Placeholder
        }

        fn AcquireScratch(&mut self) -> Register {
            Register(0) // Placeholder
        }

        fn AcquireScratchDouble(&mut self) -> DoubleRegister {
            DoubleRegister(0) // Placeholder
        }
        
        fn Available(&mut self) -> RegisterSet{
            RegisterSet::new() // Placeholder
        }
    }

    struct Label {}
    impl Label {
        
    }

    // Mock implementations for assembler instructions.
    impl MaglevAssembler {
        fn SubS64(&mut self, _dst: Register, _src: Register) {}
        fn AddS64(&mut self, _dst: Register, _operand: Operand) {}
        fn lay(&mut self, _dst: Register, _mem_operand: MemOperand) {}
        fn LoadU64(&mut self, _dst: Register, _operand: Operand) {}
        fn CmpU64(&mut self, _reg1: Register, _reg2: Register) {}
        fn JumpToDeferredIf<T>(
            &mut self,
            _condition: Condition,
            _f: fn(&mut MaglevAssembler, RegisterSnapshot, Register, Builtin, T, ZoneLabelRef),
            _register_snapshot: RegisterSnapshot,
            _object: Register,
            _builtin: Builtin,
            _size_in_bytes: T,
            _done: ZoneLabelRef,
        ) {
        }
        fn Move(&mut self, _dst: Operand, _src: Register) {}
        fn bind(&mut self, _label: Label) {}
        fn Push(&mut self, _reg: Register) {}
        fn CmpS32(&mut self, _reg1: Register, _reg2: Register) {}
        fn Assert(&mut self, _condition: Condition, _reason: AbortReason) {}
        fn EnterFrame(&mut self, _frame_type: StackFrame) {}
        fn TailCallBuiltin(&mut self, _builtin: Builtin, _flags: i32) {}
        fn LoadFeedbackVectorFlagsAndCheckIfNeedsProcessing(&mut self, _flags: Register, _feedback_vector: i32, _code_kind: CodeKind) -> i32 {
            0 // Placeholder
        }
        fn BailoutIfDeoptimized(&mut self, _scratch: Register) {}
        fn ASM_CODE_COMMENT_STRING(&mut self, _comment: &str) {}
        fn SubS32(&mut self, _reg: Register, _operand: Operand) {}
        fn bgt(&mut self, _label: &Label) {}
        fn LoadTaggedFieldByIndex(&mut self, _dst: Register, _src: Register, _index: Register, _tagged_size: i32, _offset: i32) {}
        fn LoadRoot(&mut self, _dst: Register, _root_index: RootIndex) {}
        fn AllocateTwoByteString(&mut self, _register_snapshot: RegisterSnapshot, _result: Register, _i: i32) {}
        fn StoreU16(&mut self, _char_code: Register, _field_mem_operand: FieldMemOperand) {}
        fn LoadTaggedField(&mut self, _dst: Register, _mem_operand: FieldMemOperand) {}
        fn Jump(&mut self, _label: *const Label) {}
        fn AssertObjectTypeInRange(&mut self, _reg: Register, _first: i32, _last: i32, _reason: AbortReason) {}
        fn LoadU32(&mut self, _dst: Register, _mem_operand: FieldMemOperand) {}
        fn Check(&mut self, _condition: Condition, _reason: AbortReason) {}
        fn LoadInstanceType(&mut self, _dst: Register, _object: Register) {}
        fn And(&mut self, _dst: Register, _operand: Operand) {}
        fn beq(&mut self, _label: *const Label) {}
        fn bne(&mut self, _label: *const Label) {}
        fn LoadAndUntagTaggedSignedField(&mut self, _dst: Register, _src: Register, _offset: i32) {}
        fn CompareRoot(&mut self, _reg: Register, _root_index: RootIndex) {}
        fn LoadU8(&mut self, _dst: Register, _mem_operand: FieldMemOperand) {}
        fn ShiftLeftU64(&mut self, _dst: Register, _src: Register, _operand: Operand) {}
        fn AddU64(&mut self, _dst: Register, _operand: Operand) {}
        fn JumpToDeferredIf<F, T>(
            &mut self,
            _condition: Condition,
            _callback: F,
            _register_snapshot: RegisterSnapshot,
            _done: ZoneLabelRef,
            _result: Register,
            _char_code: Register,
            _scratch: Register,
        ) where
            F: Fn(&mut MaglevAssembler, RegisterSnapshot, ZoneLabelRef, Register, Register, Register),
        {
        }

        fn SmiTag(&mut self, _index: Register) {}
        fn CallRuntime(&mut self, _runtime_kStringCodePointAt: i32) {}
        fn SmiUntag(&mut self, _ret: Register) {}
        fn AllocateStackSpace(&mut self, _size: i32) {}
        fn StoreF64(&mut self, _src: DoubleRegister, _mem_operand: MemOperand) {}
        fn CallBuiltin(&mut self, _builtin: Builtin) {}
        fn pop(&mut self, _reg: Register) {}
        fn TryInlineTruncateDoubleToI(&mut self, _dst: Register, _src: DoubleRegister, _done: Label) {}
        fn ConvertDoubleToInt32(&mut self, _dst: Register, _src: DoubleRegister) {}
        fn ConvertIntToDouble(&mut self, _dst: DoubleRegister, _src: Register) {}
        fn CmpF64(&mut self, _src: DoubleRegister, _temp: DoubleRegister) {}
        fn JumpIf(&mut self, _condition: Condition, _label: *const Label) {}
        fn MovDoubleToInt64(&mut self, _dst: Register, _src: DoubleRegister) {}
        fn ShiftRightS64(&mut self, _dst: Register, _src: Register, _operand: Operand) {}
        fn ConvertDoubleToUnsignedInt32(&mut self, _dst: Register, _src: DoubleRegister) {}
        fn ConvertUnsignedIntToDouble(&mut self, _dst: DoubleRegister, _src: Register) {}
        fn TryChangeFloat64ToIndex(&mut self, _result: Register, _value: DoubleRegister, _success: *const Label, _fail: *const Label) {}
    }

    impl MaglevAssembler {
        pub fn Allocate(
            &mut self,
            register_snapshot: RegisterSnapshot,
            object: Register,
            size_in_bytes: i32,
            alloc_type: AllocationType,
            alignment: AllocationAlignment,
        ) {
            allocate_raw(
                self,
                &self.isolate_,
                register_snapshot,
                object,
                size_in_bytes,
                alloc_type,
                alignment,
            );
        }

        pub fn Allocate2(
            &mut self,
            register_snapshot: RegisterSnapshot,
            object: Register,
            size_in_bytes: Register,
            alloc_type: AllocationType,
            alignment: AllocationAlignment,
        ) {
            allocate_raw(
                self,
                &self.isolate_,
                register_snapshot,
                object,
                size_in_bytes,
                alloc_type,
                alignment,
            );
        }

        pub fn OSRPrologue(&mut self, graph: &mut Graph) {
            let mut temps = TemporaryRegisterScope::new(self);
            let scratch = temps.AcquireScratch();

            assert!(graph.is_osr());
            assert!(!graph.has_recursive_calls());

            let source_frame_size =
                graph.min_maglev_stackslots_for_unoptimized_frame_size();

            if v8_flags().debug_code {
                self.lay(scratch, MemOperand::new(Register::sp, 0));
                self.lay(
                    scratch,
                    MemOperand::new(
                        scratch,
                        (source_frame_size * K_SYSTEM_POINTER_SIZE as u32
                            + StackFrameConstants::kFixedFrameSizeFromFp)
                            as i32,
                    ),
                );
                self.CmpU64(scratch, Register::fp);
                self.Assert(Condition::eq, AbortReason::kOsrUnexpectedStackSize);
            }

            let target_frame_size =
                graph.tagged_stack_slots() + graph.untagged_stack_slots();
            assert!(source_frame_size <= target_frame_size);

            if source_frame_size < target_frame_size {
                self.ASM_CODE_COMMENT_STRING("Growing frame for OSR");
                let additional_tagged = if source_frame_size < graph.tagged_stack_slots()
                {
                    graph.tagged_stack_slots() - source_frame_size
                } else {
                    0
                };
                if additional_tagged > 0 {
                    self.Move(Operand::new(0), scratch);
                }
                for _i in 0..additional_tagged {
                    self.Push(scratch);
                }
                let size_so_far = source_frame_size + additional_tagged;
                assert!(size_so_far <= target_frame_size);
                if size_so_far < target_frame_size {
                    self.lay(
                        Register::sp,
                        MemOperand::new(
                            Register::sp,
                            -((target_frame_size - size_so_far) * K_SYSTEM_POINTER_SIZE as u32)
                                as i32,
                        ),
                    );
                }
            }
        }

        pub fn Prologue(&mut self, graph: &mut Graph) {
            let mut temps = TemporaryRegisterScope::new(self);
            temps.Include(&[Register::r6, Register::r8]);
            let scratch = temps.AcquireScratch();
            assert!(!graph.is_osr());

            self.BailoutIfDeoptimized(scratch);

            if graph.has_recursive_calls() {
                self.bind(self.code_gen_state_.entry_label());
            }

            if v8_flags().turbofan {
                let flags = MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor::GetRegisterParameter(MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor::kFlags);
                let feedback_vector = MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor::GetRegisterParameter(MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor::kFeedbackVector);
                assert!(!temps.Available().has(flags));
                assert!(!temps.Available().has(feedback_vector));

                //self.Move(feedback_vector, self.compilation_info_.toplevel_compilation_unit().feedback().object());
                self.TailCallBuiltin(Builtin::kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
                        self.LoadFeedbackVectorFlagsAndCheckIfNeedsProcessing(
                            flags,
                            0, //PlaceHolder for feedback_vector
                            CodeKind::MAGLEV
                        )
                );
            }

            self.EnterFrame(StackFrame::MAGLEV);
            // Save arguments in frame.
            // TODO(leszeks): Consider eliding this frame if we don't make any calls
            // that could clobber these registers.
            self.Push(Register::kContextRegister);
            self.Push(Register::kJSFunctionRegister); // Callee's JS function.
            self.Push(Register::kJavaScriptCallArgCountRegister); // Actual argument count.

            // Initialize stack slots.
            if graph.tagged_stack_slots() > 0 {
                self.ASM_CODE_COMMENT_STRING("Initializing stack slots");
                self.Move(Operand::new(0), scratch);

                // Magic value. Experimentally, an unroll size of 8 doesn't seem any
                // worse than fully unrolled pushes.
                const K_LOOP_UNROLL_SIZE: i32 = 8;
                let tagged_slots = graph.tagged_stack_slots();
                if tagged_slots < K_LOOP_UNROLL_SIZE as u32 {
                    // If the frame is small enough, just unroll the frame fill
                    // completely.
                    for _i in 0..tagged_slots {
                        self.Push(scratch);
                    }
                } else {
                    // Extract the first few slots to round to the unroll size.
                    let first_slots = tagged_slots % K_LOOP_UNROLL_SIZE as u32;
                    for _i in 0..first_slots {
                        self.Push(scratch);
                    }
                    let mut unroll_counter = temps.AcquireScratch();
                    self.Move(Operand::new((tagged_slots / K_LOOP_UNROLL_SIZE as u32) as i32), unroll_counter);
                    // We enter the loop unconditionally, so make sure we need to loop at
                    // least once.
                    assert!((tagged_slots / K_LOOP_UNROLL_SIZE as u32) > 0);
                    let mut loop_label = Label {};
                    self.bind(loop_label);
                    for _i in 0..K_LOOP_UNROLL_SIZE {
                        self.Push(scratch);
                    }
                    self.SubS32(unroll_counter, Operand::new(1));
                    self.bgt(&loop_label);
                }
            }
            if graph.untagged_stack_slots() > 0 {
                // Extend rsp by the size of the remaining untagged part of the frame,
                // no need to initialise these.
                self.lay(
                    Register::sp,
                    MemOperand::new(
                        Register::sp,
                        -(graph.untagged_stack_slots() * K_SYSTEM_POINTER_SIZE as u32) as i32,
                    ),
                );
            }
        }

        pub fn MaybeEmitDeoptBuiltinsCall(
            &mut self,
            _eager_deopt_count: usize,
            _eager_deopt_entry: *mut Label,
            _lazy_deopt_count: usize,
            _lazy_deopt_entry: *mut Label,
        ) {
        }

        pub fn LoadSingleCharacterString(&mut self, result: Register, char_code: Register, scratch: Register) {
            if v8_flags().debug_code {
                self.CmpS32(char_code, Operand::new(StringConstants::kMaxOneByteCharCode as i32));
                self.Assert(Condition::le, AbortReason::kUnexpectedValue);
            }
            let table = scratch;
            self.LoadRoot(table, RootIndex::kSingleCharacterStringTable);
            self.LoadTaggedFieldByIndex(result, table, char_code, kTaggedSize as i32,
                                         offset_of_data_start!(FixedArray));
        }

        pub fn StringFromCharCode(&mut self, register_snapshot: RegisterSnapshot, char_code_fits_one_byte: *mut Label,
                                    result: Register, char_code: Register, scratch: Register,
                                    mask_mode: CharCodeMaskMode) {
            self.AssertZeroExtended(char_code);
            let mut done = ZoneLabelRef {};
            if mask_mode == CharCodeMaskMode::kMustApplyMask {
                self.And(char_code, Operand::new(0xFFFF));
            }
            self.CmpS32(char_code, Operand::new(StringConstants::kMaxOneByteCharCode as i32));
            self.JumpToDeferredIf(
                Condition::kUnsignedGreaterThan,
                |masm: &mut MaglevAssembler, register_snapshot: RegisterSnapshot, done: ZoneLabelRef, result: Register, char_code: Register, scratch: Register| {
                    // Be sure to save {char_code}. If it aliases with {result}, use
                    // the scratch register.
                    // TODO(victorgomes): This is probably not needed any more, because
                    // we now ensure that results registers don't alias with inputs/temps.
                    // Confirm, and drop this check.
                    let mut char_code_copy = char_code;
                    if char_code == result {
                        masm.Move(Operand::new(char_code.0), scratch);
                        char_code_copy = scratch;
                    }
                    assert!(char_code_copy != result);
                    //DCHECK(!register_snapshot.live_tagged_registers.has(char_code));
                    //register_snapshot.live_registers.set(char_code);
                    masm.AllocateTwoByteString(register_snapshot, result, 1);
                    masm.StoreU16(
                        char_code_copy,
                        FieldMemOperand::new(result, offset_of_data_start!(SeqTwoByteString)),
                    );
                    //__ b(*done); //Implement this jump.
                    masm.bne(&Label{});
                },
                register_snapshot,
                done,
                result,
                char_code,
                scratch,
            );
            if char_code_fits_one_byte as usize != 0 {
                self.bind(Label {});
            }
            self.LoadSingleCharacterString(result, char_code, scratch);
            self.bind(done);
        }

        pub fn StringCharCodeOrCodePointAt(
            &mut self,
            mode: BuiltinStringPrototypeCharCodeOrCodePointAt::Mode,
            register_snapshot: &mut RegisterSnapshot,
            result: Register,
            string: Register,
            index: Register,
            instance_type: Register,
            scratch2: Register,
            result_fits_one_byte: *mut Label,
        ) {
            let mut done = ZoneLabelRef {};
            let mut seq_string = Label {};
            let mut cons_string = Label {};
            let mut sliced_string = Label {};

            let deferred_runtime_call = self.MakeDeferredCode(
                |masm: &mut MaglevAssembler,
                 mode: BuiltinStringPrototypeCharCodeOrCodePointAt::Mode,
                 register_snapshot: RegisterSnapshot,
                 done: ZoneLabelRef,
                 result: Register,
                 string: Register,
                 index: Register| {
                    //DCHECK(!register_snapshot.live_registers.has(result));
                    //DCHECK(!register_snapshot.live_registers.has(string));
                    //DCHECK(!register_snapshot.live_registers.has(index));
                    {
                        let save_register_state = SaveRegisterStateForCall::new(masm, register_snapshot);
                        masm.SmiTag(index);
                        masm.Push(string);
                        masm.Push(index);
                        masm.Move(Operand::new(masm.native_context().object()), Register::kContextRegister);
                        // This call does not throw nor can deopt.
                        if mode == BuiltinStringPrototypeCharCodeOrCodePointAt::kCodePointAt {
                            masm.CallRuntime(0 /*Runtime::kStringCodePointAt*/);
                        } else {
                            assert_eq!(mode, BuiltinStringPrototypeCharCodeOrCodePointAt::kCharCodeAt);
                            masm.CallRuntime(0 /*Runtime::kStringCharCodeAt*/);
                        }
                        save_register_state.DefineSafepoint();
                        masm.SmiUntag(Register::kReturnRegister0);
                        masm.Move(Operand::new(Register::kReturnRegister0.0), result);
                    }
                    masm.bne(&Label{});
                },
                mode,
                *register_snapshot,
                done,
                result,
                string,
                index,
            );

            // We might need to try more than one time for ConsString, SlicedString and
            // ThinString.
            let mut loop_label = Label {};
            self.bind(loop_label);

            if v8_flags().debug_code {
                // Check if {string} is a string.
                self.AssertObjectTypeInRange(string, 0 /*FIRST_STRING_TYPE*/, 0 /*LAST_STRING_TYPE*/,
                                             AbortReason::kUnexpectedValue);

                let scratch = instance_type;

                self.LoadU32(scratch, FieldMemOperand::new(string, 0 /*offsetof(String, length_)*/));
                self.CmpS32(index, scratch);
                self.Check(Condition::lt, AbortReason::kUnexpectedValue);
            }

            // Get instance type.
            self.LoadInstanceType(instance_type, string);

            {
                let mut temps = TemporaryRegisterScope::new(self);
                let representation = temps.AcquireScratch();

                // TODO(victorgomes): Add fast path for external strings.
                self.And(representation, Operand::new(0 /*kStringRepresentationMask*/));
                self.CmpS32(representation, Operand::new(0 /*kSeqStringTag*/));
                self.beq(&seq_string);
                //And(representation, Operand(kConsStringTag));
                self.CmpS32(representation, Operand::new(0 /*kConsStringTag*/));
                self.beq(&cons_string);
                self.CmpS32(representation, Operand::new(0 /*kSlicedStringTag*/));
                self.beq(&sliced_string);
                self.CmpS32(representation, Operand::new(0 /*kThinStringTag*/));
                self.bne(deferred_runtime_call);
                // Fallthrough to thin string.
            }

            // Is a thin string.
            {
                self.LoadTaggedField(string,
                                     FieldMemOperand::new(string, 0 /*offsetof(ThinString, actual_)*/));
                self.bne(&loop_label);
            }

            self.bind(sliced_string);
            {
                let mut temps = TemporaryRegisterScope::new(self);
                let offset = temps.AcquireScratch();

                self.LoadAndUntagTaggedSignedField(offset, string,
                                                     0 /*offsetof(SlicedString, offset_)*/);
                self.LoadTaggedField(string, string, 0 /*offsetof(SlicedString, parent_)*/);
                self.AddS32(index, Operand::new(offset.0));
                self.bne(&loop_label);
            }

            self.bind(cons_string);
            {
                // Reuse {instance_type} register here, since CompareRoot requires a scratch
                // register as well.
                let second_string = instance_type;
                self.LoadU64(second_string,
                             FieldMemOperand::new(string, 0 /*offsetof(ConsString, second_)*/));
                self.CompareRoot(second_string, RootIndex::kempty_string);
                self.bne(deferred_runtime_call);
                self.LoadTaggedField(string,
                                     FieldMemOperand::new(string, 0 /*offsetof(ConsString, first_)*/));
                self.bne(&loop_label); // Try again with first string.
            }

            self.bind(seq_string);
            {
                let mut two_byte_string = Label {};
                self.And(instance_type, Operand::new(0 /*kStringEncodingMask*/));
                self.CmpS32(instance_type, Operand::new(0 /*kTwoByteStringTag*/));
                self.beq(&two_byte_string);
                // The result of one-byte string will be the same for both modes
                // (CharCodeAt/CodePointAt), since it cannot be the first half of a
                // surrogate pair.
                // AndP(index, Operand(SeqOneByteString::kHeaderSize - kHeapObjectTag));
                self.LoadU8(result, FieldMemOperand::new(string, 0 /*index*/));
                self.bne(&Label{}); //Placeholder for result_fits_one_byte

                self.bind(two_byte_string);
                // {instance_type} is unused from this point, so we can use as scratch.
                let scratch = instance_type;
                self.ShiftLeftU64(scratch, index, Operand::new(1));
                self.AddU64(scratch,
                             Operand::new(0 /*OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag*/));

                if mode == BuiltinStringPrototypeCharCodeOrCodePointAt::kCharCodeAt {
                    //LoadU16(result, MemOperand(string, scratch)); Implement
                } else {
                    assert_eq!(mode, BuiltinStringPrototypeCharCodeOrCodePointAt::kCodePointAt);
                    let string_backup = string;
                    let mut scratch2_val = scratch2;
                    if result == string {
                        scratch2_val = scratch2;
                        self.Move(Operand::new(string.0), string_backup);
                    }
                    //LoadU16(result, MemOperand(string, scratch)); Implement

                    let first_code_point = scratch;
                    //And(first_code_point, result, Operand(0xfc00)); Implement
                    self.CmpS32(first_code_point, Operand::new(0 /*0xd800*/));
                    self.bne(&Label{});

                    let length = scratch;
                    self.LoadU32(length, FieldMemOperand::new(string, 0 /*offsetof(String, length_)*/));
                    self.AddS32(index, Operand::new(1));
                    self.CmpS32(index, length);
                    self.bge(&Label{});

                    let second_code_point = scratch;
                    self.ShiftLeftU32(index, index, Operand::new(1));
                    self.AddU32(index,
                                 Operand::new(0 /*OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag*/));
                    //LoadU16(second_code_point, MemOperand(string_backup, index)); Implement

                    // {index} is not needed at this point.
                    let scratch2 = index;
                    //And(scratch2, second_code_point, Operand(0xfc00)); Implement
                    self.CmpS32(scratch2, Operand::new(0 /*0xdc00*/));
                    self.bne(&Label{});

                    let surrogate_offset = 0x10000 - (0xd800 << 10) - 0xdc00;
                    self.AddS32(second_code_point, Operand::new(surrogate_offset));
                    self.ShiftLeftU32(result, result, Operand::new