// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::codegen::interface_descriptors_inl::*; // Assuming this exists
//use crate::deoptimizer::deoptimizer::*; // Assuming this exists
//use crate::maglev::maglev_assembler_inl::*; // Assuming this exists
//use crate::maglev::maglev_graph::*; // Assuming this exists
//use crate::objects::*;

//use std::convert::TryInto;

// Define constants and helper functions based on C++ code

const kHeapObjectTag: i64 = 1; // Example Value
const kSystemPointerSize: i64 = 8; // Example Value
const kTaggedSize: i64 = 8; // Example Value

macro_rules! OFFSET_OF_DATA_START {
    ($struct_name:ident) => {
        16 // Example Value.  Replace with actual offsetof calculation
    };
}

macro_rules! ASM_CODE_COMMENT {
    ($masm:expr) => {
        // Placeholder for assembly code comments
        println!("Assembly code comment");
    };
}

macro_rules! ASM_CODE_COMMENT_STRING {
    ($masm:expr, $string:expr) => {
        // Placeholder for assembly code comments with strings
        println!("Assembly code comment: {}", $string);
    };
}

// Dummy macro for checking conditions. Replace with actual assertions or error handling
macro_rules! CHECK {
    ($condition:expr, $abort_reason:expr, $($arg:tt)*) => {
        if !($condition) {
            eprintln!("Check failed: {:?} - {}", $abort_reason, format!($($arg)*));
            panic!("Check failed");
        }
    };
}

// Dummy macro for assertions
macro_rules! DCHECK {
    ($condition:expr) => {
        if !($condition) {
            eprintln!("DCheck failed");
            panic!("DCheck failed");
        }
    };
}

// Dummy macro for assertions
macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            eprintln!("Check Eq failed: {} != {}", $left, $right);
            panic!("Check Eq failed");
        }
    };
}

// Dummy macro for assertions
macro_rules! CHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            eprintln!("Check LE failed: {} > {}", $left, $right);
            panic!("Check LE failed");
        }
    };
}

// Dummy macro for assertions
macro_rules! CHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            eprintln!("Check NE failed: {} == {}", $left, $right);
            panic!("Check NE failed");
        }
    };
}

mod maglev {
    //use super::*;
    //use crate::codegen::*;
    //use crate::builtins::*;

    // Replace with your actual register types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Register(u32);

    impl Register {
        pub fn new(index: u32) -> Self {
            Register(index)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DoubleRegister(u32);

    impl DoubleRegister {
        pub fn new(index: u32) -> Self {
            DoubleRegister(index)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Operand(i64);

    impl Operand {
        pub fn new(value: i64) -> Self {
            Operand(value)
        }
    }

    // Example Enums, Replace with correct definitions
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AllocationType {
        kOld,
        kYoung,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AllocationAlignment {
        kTaggedAligned,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AbortReason {
        kUnexpectedValue,
        kOsrUnexpectedStackSize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RootIndex {
        kSingleCharacterStringTable,
        kempty_string,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Builtin {
        kDeoptimizationEntry_Eager,
        kDeoptimizationEntry_Lazy,
        kDoubleToI,
        kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CodeKind {
        MAGLEV
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StackFrame {
        MAGLEV
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CharCodeMaskMode {
        kMustApplyMask,
        // Other options if there are more
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BuiltinStringPrototypeCharCodeOrCodePointAt {
        kCharCodeAt,
        kCodePointAt,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum InstanceType {
        ONE_BYTE_STRING_TYPE,
        TWO_BYTE_STRING_TYPE
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ZoneLabelRef;

    impl ZoneLabelRef {
        pub fn new() -> Self {
            ZoneLabelRef {}
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Label;

    impl Label {
        pub fn new() -> Self {
            Label {}
        }
        pub const kNear: Label = Label {};
    }

    pub struct RegisterSnapshot {
        pub live_registers: std::collections::HashSet<Register>,
        pub live_tagged_registers: std::collections::HashSet<Register>,
    }

    impl RegisterSnapshot {
        pub fn new() -> Self {
            RegisterSnapshot {
                live_registers: std::collections::HashSet::new(),
                live_tagged_registers: std::collections::HashSet::new(),
            }
        }
    }

    pub struct MaglevAssembler {
        // Placeholder fields.  Replace with actual assembler state.
        isolate_: Isolate,
        code_gen_state_: CodeGenState,
    }

    impl MaglevAssembler {
        pub fn new(isolate_: Isolate, code_gen_state_: CodeGenState) -> Self {
            MaglevAssembler {
                isolate_,
                code_gen_state_
            }
        }
        pub fn allow_allocate(&self) -> bool { true }
        pub fn GetFlagsRegister() -> Register { Register(0) }
        fn MacroAssemblerBranch(&mut self, _label: &Label, _condition: Condition, _reg: Register, _operand: Operand, _label_preference: Label) {}
        //fn MacroAssemblerJump(&mut self, _scratch: Register) {}
        fn MacroAssemblerMove(&mut self, _dest: Register, _src: DoubleRegister) {}
        fn MacroAssembler::Branch(&mut self, _label: &Label, _condition: Condition, _reg1: Register, _operand: Operand) {}
        fn MacroAssembler::Jump(&mut self, _scratch: Register) {}
        fn BranchShort(&mut self, _label: &Label, _condition: Condition, _reg1: Register, _operand: Operand) {}
        pub fn native_context(&self) -> NativeContext { NativeContext::new() }
        pub fn compilation_info(&self) -> CompilationInfo { CompilationInfo::new() }
        pub fn ForceConstantPoolEmissionWithoutJump(&mut self) {}
        pub fn TryInlineTruncateDoubleToI(&mut self, _dst: Register, _src: DoubleRegister, _done: ZoneLabelRef) {}
        pub fn CallBuiltin(&mut self, _builtin: Builtin) {}
        pub fn LoadEntryFromBuiltin(&mut self, _builtin: Builtin, _scratch: Register) {}
        pub fn CodeGenState(&mut self) {}
        pub fn SmiTag(&mut self, _register: Register) {}
        pub fn Push(&mut self, _register: Register) {}
        pub fn CallRuntime(&mut self, _runtime: Runtime) {}
        pub fn SmiUntag(&mut self, _register: Register) {}
        pub fn LoadInstanceType(&mut self, _dest: Register, _object: Register) {}
        pub fn AssertObjectTypeInRange(&mut self, _object: Register, _first: InstanceType, _last: InstanceType, _reason: AbortReason) {}
        //fn CompareRoot(&mut self, _object: Register, _root_index: RootIndex) {}
        pub fn code_gen_state(&self) -> &CodeGenState { &self.code_gen_state_ }
        //fn MacroAssembler::SbxCheck(&mut self, _condition: Condition, _abort_reason: AbortReason, _reg1: Register, _operand: Operand) {}
    }
    
    impl MaglevAssembler {

        fn sub_size_and_tag_object(&mut self, object: Register, size_in_bytes: Register) {
            self.SubWord(object, object, Operand::new(-1 * size_in_bytes.0 as i64));
            self.AddWord(object, object, Operand::new(kHeapObjectTag));
        }

        fn sub_size_and_tag_object_imm(&mut self, object: Register, size_in_bytes: i64) {
            self.AddWord(object, object, Operand::new(kHeapObjectTag - size_in_bytes));
        }

        pub fn allocate(&mut self, register_snapshot: RegisterSnapshot, object: Register, size_in_bytes: i64, alloc_type: AllocationType, alignment: AllocationAlignment) {
            self.allocate_raw(register_snapshot, object, size_in_bytes, alloc_type, alignment);
        }

        pub fn allocate_reg(&mut self, register_snapshot: RegisterSnapshot, object: Register, size_in_bytes: Register, alloc_type: AllocationType, alignment: AllocationAlignment) {
            self.allocate_raw(register_snapshot, object, size_in_bytes, alloc_type, alignment);
        }

        fn allocate_raw<T>(&mut self, register_snapshot: RegisterSnapshot, object: Register, size_in_bytes: T, alloc_type: AllocationType, alignment: AllocationAlignment)
        where T: std::fmt::Debug + Copy {
            //where i64: From<T>
            //where T: Into<i64>
            //where i64: From<T> {
            DCHECK!(self.allow_allocate());
            // TODO(victorgomes): Call the runtime for large object allocation.
            // TODO(victorgomes): Support double alignment.
            DCHECK_EQ!(alignment, AllocationAlignment::kTaggedAligned);
            //let size_in_bytes_i64: i64 = size_in_bytes.into();
            let alloc_type = if v8_flags::single_generation {
                AllocationType::kOld
            } else {
                alloc_type
            };
            let top = SpaceAllocationTopAddress(&self.isolate_, alloc_type);
            let limit = SpaceAllocationLimitAddress(&self.isolate_, alloc_type);

            let done = ZoneLabelRef::new();
            let mut temps = TemporaryRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            // We are a bit short on registers, so we use the same register for {object}
            // and {new_top}. Once we have defined {new_top}, we don't use {object} until
            // {new_top} is used for the last time. And there (at the end of this
            // function), we recover the original {object} from {new_top} by subtracting
            // {size_in_bytes}.
            let new_top = object;
            // Check if there is enough space.
            self.LoadWord(object, ExternalReferenceAsOperand(&top, scratch));
            self.AddWord(new_top, object, Operand::new(match size_in_bytes {
                _ => {
                    if let Some(val) = size_in_bytes.downcast_ref::<i64>() {
                        *val
                    } else {
                        0 //FIXME!!!
                    }
                    //_ => size_in_bytes as i64 //size_in_bytes.try_into().unwrap(),//size_in_bytes.try_into().unwrap(),//Operand::new(size_in_bytes as i64)
                    //_ => size_in_bytes.into()
                }
            }));
            self.LoadWord(scratch, ExternalReferenceAsOperand(&limit, scratch));

            // Call runtime if new_top >= limit.
            let register_snapshot_copy = register_snapshot.clone(); // Clone for the closure
            self.MacroAssembler::Branch(
                &Self::make_deferred_code(
                    move |masm: &mut MaglevAssembler, register_snapshot: RegisterSnapshot, object: Register, alloc_type: AllocationType, size_in_bytes: T, done: ZoneLabelRef| {
                        masm.allocate_slow(register_snapshot, object, masm.allocate_builtin(alloc_type), size_in_bytes, done);
                    },
                    register_snapshot_copy, object, alloc_type, size_in_bytes, done,
                ),
                Condition::kGreaterEqual,
                new_top,
                Operand::new(scratch.0 as i64), //FIXME!!!
            );

            // Store new top and tag object.
            self.Move(ExternalReferenceAsOperand(&top, scratch), new_top);
            match size_in_bytes {
                _ => {
                    if let Some(val) = size_in_bytes.downcast_ref::<i64>() {
                        self.sub_size_and_tag_object_imm(object, *val);
                    } else {
                        () //FIXME!!!
                    }
                    //_ => size_in_bytes as i64 //size_in_bytes.try_into().unwrap(),//size_in_bytes.try_into().unwrap(),//Operand::new(size_in_bytes as i64)
                    //_ => size_in_bytes.into()
                }
            }
            //self.sub_size_and_tag_object(object, size_in_bytes as i64);
            self.bind(done);
        }

        fn allocate_slow<T>(&mut self, _register_snapshot: RegisterSnapshot, _object: Register, _allocate_builtin: Builtin, _size_in_bytes: T, _done: ZoneLabelRef)
        where T: std::fmt::Debug + Copy {
            println!("Implement AllocateSlow");
        }

        fn allocate_builtin(&mut self, _alloc_type: AllocationType) -> Builtin {
            Builtin::kDoubleToI //FIXME!!!!
        }

        fn make_deferred_code<F, T>(closure: F, register_snapshot: RegisterSnapshot, object: Register, alloc_type: AllocationType, size_in_bytes: T, done: ZoneLabelRef) -> Label
            where
                F: FnOnce(&mut MaglevAssembler, RegisterSnapshot, Register, AllocationType, T, ZoneLabelRef) + 'static,
                T: std::fmt::Debug + Copy,
        {
            // Dummy implementation
            let mut assembler = MaglevAssembler::new(Isolate::new(), CodeGenState::new());
            closure(&mut assembler, register_snapshot, object, alloc_type, size_in_bytes, done);
            Label::new()
        }

        fn osr_prologue(&mut self, graph: &Graph) {
            DCHECK!(graph.is_osr());
            CHECK!(!graph.has_recursive_calls(), AbortReason::kUnexpectedValue);
            let source_frame_size = graph.min_maglev_stackslots_for_unoptimized_frame_size();

            if v8_flags::maglev_assert_stack_size && v8_flags::debug_code {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                let expected_osr_stack_size =
                    source_frame_size * kSystemPointerSize + StandardFrameConstants::kFixedFrameSizeFromFp;
                self.AddWord(scratch, Register::new(2), Operand::new(expected_osr_stack_size));
                Self::MacroAssembler::SbxCheck(self, Condition::kEqual, AbortReason::kOsrUnexpectedStackSize, scratch, Operand::new(1));
            }

            let target_frame_size = graph.tagged_stack_slots() + graph.untagged_stack_slots();
            // CHECK_EQ(target_frame_size % 2, 1);
            CHECK_LE!(source_frame_size, target_frame_size);

            if source_frame_size < target_frame_size {
                ASM_CODE_COMMENT_STRING!(self, "Growing frame for OSR");
                let additional_tagged = if source_frame_size < graph.tagged_stack_slots() {
                    graph.tagged_stack_slots() - source_frame_size
                } else {
                    0
                };
                for _i in 0..additional_tagged {
                    self.Push(Register::new(0)); // zero_reg
                }

                let size_so_far = source_frame_size + additional_tagged;
                CHECK_LE!(size_so_far, target_frame_size);
                if size_so_far < target_frame_size {
                    self.Sub64(Register::new(1), Register::new(1), Operand::new((target_frame_size - size_so_far) * kSystemPointerSize)); // sp
                }
            }
        }

        fn prologue(&mut self, graph: &Graph) {
            ASM_CODE_COMMENT!(self);

            let mut temps = TemporaryRegisterScope::new(self);
            // We add two extra registers to the scope. Ideally we could add all the
            // allocatable general registers, except Context, JSFunction, NewTarget and
            // ArgCount. Unfortunately, OptimizeCodeOrTailCallOptimizedCodeSlot and
            // LoadFeedbackVectorFlagsAndJumpIfNeedsProcessing pick random registers and
            // we could alias those.
            // TODO(victorgomes): Fix these builtins to either use the scope or pass the
            // used registers manually.
            temps.include(Register::new(3), Register::new(4)); // use register not overlapping with flags, feedback and so on
                                  // s7 s8
            DCHECK!(!graph.is_osr());

            self.call_target();
            self.bailout_if_deoptimized();

            if graph.has_recursive_calls() {
                self.bind_call_target(self.code_gen_state().entry_label());
            }

            // Tiering support.
            if v8_flags::turbofan {
                //use D = MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor;
                //let flags = D::GetRegisterParameter(D::kFlags);
                //let feedback_vector = D::GetRegisterParameter(D::kFeedbackVector);
                //DCHECK!(!are_aliased(flags, feedback_vector, kJavaScriptCallArgCountRegister, kJSFunctionRegister, kContextRegister, kJavaScriptCallNewTargetRegister));
                //DCHECK!(!temps.available().has(flags));
                //DCHECK!(!temps.available().has(feedback_vector));
                //self.Move(feedback_vector, compilation_info().toplevel_compilation_unit().feedback().object());
                let needs_processing = Label::new();
                let done = Label::new();

                self.load_feedback_vector_flags_and_jump_if_needs_processing(
                    Register::new(3), //flags
                    Register::new(4), //feedback_vector,
                    CodeKind::MAGLEV,
                    &needs_processing,
                );
                self.Jump(&done);
                self.bind(needs_processing);
                self.tail_call_builtin(Builtin::kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot);
                self.bind(done);
            }

            self.enter_frame(StackFrame::MAGLEV);

            // Save arguments in frame.
            // TODO(leszeks): Consider eliding this frame if we don't make any calls
            // that could clobber these registers.
            // Push the context and the JSFunction.
            self.Push(Register::new(5)); // kContextRegister
            self.Push(Register::new(6)); // kJSFunctionRegister
            // Push the actual argument count and a _possible_ stack slot.
            self.Push(Register::new(7)); // kJavaScriptCallArgCountRegister

            // Initialize stack slots.
            if graph.tagged_stack_slots() > 0 {
                ASM_CODE_COMMENT_STRING!(self, "Initializing stack slots");

                // Magic value. Experimentally, an unroll size of 8 doesn't seem any
                // worse than fully unrolled pushes.
                const K_LOOP_UNROLL_SIZE: i32 = 8;
                let tagged_slots = graph.tagged_stack_slots();

                if tagged_slots < 2 * K_LOOP_UNROLL_SIZE as i64 {
                    // If the frame is small enough, just unroll the frame fill
                    // completely.
                    for _i in 0..tagged_slots {
                        self.Push(Register::new(0)); // zero_reg
                    }
                } else {
                    // Extract the first few slots to round to the unroll size.
                    let first_slots = tagged_slots % K_LOOP_UNROLL_SIZE as i64;
                    for _i in 0..first_slots {
                        self.Push(Register::new(0)); // zero_reg
                    }

                    let mut temps = TemporaryRegisterScope::new(self);
                    let count = temps.acquire_scratch();

                    self.Move(count, tagged_slots / K_LOOP_UNROLL_SIZE as i64);
                    // We enter the loop unconditionally, so make sure we need to loop at
                    // least once.
                    DCHECK!((tagged_slots / K_LOOP_UNROLL_SIZE as i64) > 0);

                    let loop_label = Label::new();
                    self.bind(loop_label);
                    for _i in 0..K_LOOP_UNROLL_SIZE {
                        self.Push(Register::new(0)); // zero_reg
                    }

                    self.Sub64(count, count, Operand::new(1));
                    self.MacroAssemblerBranch(&loop_label, Condition::kGreaterThan, count, Operand::new(0), Label::kNear);
                }
            }

            if graph.untagged_stack_slots() > 0 {
                // Extend sp by the size of the remaining untagged part of the frame,
                // no need to initialise these.
                self.Sub64(Register::new(1), Register::new(1), Operand::new(graph.untagged_stack_slots() * kSystemPointerSize)); // sp
            }
        }

        fn maybe_emit_deopt_builtins_call(&mut self, eager_deopt_count: usize, eager_deopt_entry: &mut Label, lazy_deopt_count: usize, lazy_deopt_entry: &mut Label) {
            self.ForceConstantPoolEmissionWithoutJump();
            //DCHECK_GE!(Deoptimizer::kLazyDeoptExitSize, Deoptimizer::kEagerDeoptExitSize);
            let mut scope = TemporaryRegisterScope::new(self);
            let scratch = scope.acquire_scratch();
            if eager_deopt_count > 0 {
                self.bind(*eager_deopt_entry);
                self.LoadEntryFromBuiltin(Builtin::kDeoptimizationEntry_Eager, scratch);
                self.MacroAssembler::Jump(scratch);
            }
            if lazy_deopt_count > 0 {
                self.bind(*lazy_deopt_entry);
                self.LoadEntryFromBuiltin(Builtin::kDeoptimizationEntry_Lazy, scratch);
                self.MacroAssembler::Jump(scratch);
            }
        }

        fn load_single_character_string(&mut self, result: Register, char_code: Register, scratch: Register) {
            DCHECK!(char_code != scratch);
            if v8_flags::debug_code {
                //self.MacroAssembler::Assert(less_equal, AbortReason::kUnexpectedValue, char_code, Operand::new(String::kMaxOneByteCharCode));
            }
            let table = scratch;
            self.LoadRoot(table, RootIndex::kSingleCharacterStringTable);
            self.LoadTaggedFieldByIndex(result, table, char_code, kTaggedSize, OFFSET_OF_DATA_START!(FixedArray));
        }

        fn string_from_char_code(&mut self, register_snapshot: RegisterSnapshot, char_code_fits_one_byte: &mut Label, result: Register, char_code: Register, scratch: Register, mask_mode: CharCodeMaskMode) {
            self.ZeroExtendWord(char_code, char_code);
            //AssertZeroExtended(char_code);
            DCHECK!(char_code != scratch);
            let done = ZoneLabelRef::new();

            if mask_mode == CharCodeMaskMode::kMustApplyMask {
                self.And(char_code, char_code, Operand::new(0xFFFF));
            }

            // Allocate two-bytes string if {char_code} doesn't fit one byte.
            let register_snapshot_copy = register_snapshot.clone(); // Clone for the closure
            self.MacroAssembler::Branch(  // FIXME: reimplement with JumpToDeferredIf
                &Self::make_deferred_code(
                    move |masm: &mut MaglevAssembler, register_snapshot: RegisterSnapshot, done: ZoneLabelRef, result: Register, char_code: Register, scratch: Register| {
                        let mut temps = TemporaryRegisterScope::new(masm);
                        // Ensure that {result} never aliases {scratch}, otherwise use
                        // a temporary register to restore {result} at the end.
                        let need_restore_result = (scratch == result);
                        let string = if need_restore_result {
                            temps.acquire_scratch()
                        } else {
                            result
                        };
                        // Ensure that {char_code} never aliases {result}, otherwise use
                        // the given {scratch} register.
                        let mut char_code_mutable = char_code;

                        if char_code == result {
                            masm.Move(scratch, char_code);
                            char_code_mutable = scratch;
                        }
                        DCHECK!(char_code_mutable != string);
                        DCHECK!(scratch != string);
                        //DCHECK!(!register_snapshot.live_tagged_registers.has(char_code_mutable));
                        //register_snapshot.live_registers.set(char_code_mutable);
                        //__ AllocateTwoByteString(register_snapshot, string, 1);
                        masm.allocate_two_byte_string(register_snapshot, string, 1);
                        masm.And(scratch, char_code_mutable, Operand::new(0xFFFF));
                        masm.store_short(scratch, FieldMemOperand(string, OFFSET_OF_DATA_START!(SeqTwoByteString)));
                        if need_restore_result {
                            masm.Move(result, string);
                        }
                        masm.jump(done);
                    },
                    register_snapshot_copy, done, result, char_code, scratch,
                ),
                Condition::kUnsignedGreaterThanEqual,
                char_code,
                Operand::new(StringConstants::kMaxOneByteCharCode as i64),
            );

            self.bind(*char_code_fits_one_byte);

            self.load_single_character_string(result, char_code, scratch);
            self.bind(done);
        }

        // Sets equality flag in pseudo flags reg.
        fn is_object_type(&mut self, object: Register, scratch1: Register, scratch2: Register, type_: InstanceType) {
            ASM_CODE_COMMENT!(self);
            const FLAGS: Register = Register::new(0); //MaglevAssembler::GetFlagsRegister(); //FIXME
            let condition_met = Label::new();
            let done = Label::new();

            if v8_flags::static_roots_bool && InstanceTypeChecker::unique_map_of_instance_type(type_) {
                self.load_compressed_map(scratch1, object);
                if let Some(expected) = InstanceTypeChecker::unique_map_of_instance_type(type_) {
                    //Tagged_t expected_ptr = ReadOnlyRootPtr(*expected);
                    self.li(scratch2, 0); //expected_ptr
                    self.Sll32(scratch2, scratch2, Operand::new(0));
                    Self::MacroAssembler::Branch(self, &condition_met, Condition::kEqual, scratch1, Operand::new(scratch2.0 as i64), Label::kNear); //FIXME
                }
            } else {
                self.compare_object_type_and_jump(object, scratch1, scratch2, type_, Condition::kEqual, &condition_met, Label::kNear);
            }
            self.Li(FLAGS, 1); // Condition is not met by default and
                                         // flags is set after a scratch is used,
                                         // so no harm if they are aliased.
            self.Jump(&done, Label::kNear);
            self.bind(condition_met);
            self.Mv(FLAGS, Register::new(0)); // zero_reg // Condition is met
            self.bind(done);
        }

        fn string_char_code_or_code_point_at(
            &mut self,
            mode: BuiltinStringPrototypeCharCodeOrCodePointAt::Mode,
            register_snapshot: &mut RegisterSnapshot,
            result: Register,
            string: Register,
            index: Register,
            instance_type: Register,
            scratch2: Register,
            result_fits_one_byte: &mut Label,
        ) {
            let done = ZoneLabelRef::new();
            let seq_string = Label::new();
            let cons_string = Label::new();
            let sliced_string = Label::new();

            let deferred_runtime_call = Self::make_deferred_code(
                move |masm: &mut MaglevAssembler,
                      mode: BuiltinStringPrototypeCharCodeOrCodePointAt::Mode,
                      register_snapshot: RegisterSnapshot,
                      done: ZoneLabelRef,
                      result: Register,
                      string: Register,
                      index: Register| {
                    //DCHECK!(!register_snapshot.live_registers.has(result));
                    //DCHECK!(!register_snapshot.live_registers.has(string));
                    //DCHECK!(!register_snapshot.live_registers.has(index));
                    //{
                        //let save_register_state = SaveRegisterStateForCall::new(masm, register_snapshot);
                        masm.SmiTag(index);
                        masm.Push(string);
                        masm.Push(index);
                        masm.Move(Register::new(5), masm.native_context().object());//kContextRegister
                        // This call does not throw nor can deopt.
                        if mode == BuiltinStringPrototypeCharCodeOrCodePointAt::kCodePointAt {
                            masm.CallRuntime(Runtime::kStringCodePointAt);
                        } else {
                            //DCHECK_EQ!(mode, BuiltinStringPrototypeCharCodeOrCodePointAt::kCharCodeAt);
                            masm.CallRuntime(Runtime::kStringCharCodeAt);
                        }
                        //save_register_state.DefineSafepoint();
                        masm.SmiUntag(Register::new(0)); //kReturnRegister0
                        masm.Move(result, Register::new(0)); //kReturnRegister0
                    //}
                    masm.jump(done);
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
            let loop_label = Label::new();
            self.bind(loop_label);

            if v8_flags::debug_code {
                let scratch = instance_type;

                // Check if {string} is a string.
                //self.AssertObjectTypeInRange(string, FIRST_STRING_TYPE, LAST_STRING_TYPE, AbortReason::kUnexpectedValue);

                self.Lw(scratch, FieldMemOperand(string, offsetof!(StringConstants, length_)));
                CHECK!(Condition::kUnsignedLessThan, AbortReason::kUnexpectedValue, index, Operand::new(scratch.0 as i64)); //FIXME
            }

            // Get instance type.
            self.LoadInstanceType(instance_type, string);

            {
                let mut temps = TemporaryRegisterScope::new(self);
                let representation = temps.acquire_scratch();

                // TODO(victorgomes): Add fast path for external strings.
                self.And(representation, instance_type, Operand::new(StringConstants::kStringRepresentationMask as i64));
                Self::MacroAssembler::Branch(self, &seq_string, Condition::kEqual, representation, Operand::new(StringConstants::kSeqStringTag as i64), Label::kNear); //FIXME
                Self::MacroAssembler::Branch(self, &cons_string, Condition::kEqual, representation, Operand::new(StringConstants::kConsStringTag as i64), Label::kNear); //FIXME
                Self::MacroAssembler::Branch(self, &sliced_string, Condition::kEqual, representation, Operand::new(StringConstants::kSlicedStringTag as i64), Label::kNear); //FIXME
                Self::MacroAssembler::Branch(self, &deferred_runtime_call, Condition::kNotEqual, representation, Operand::new(StringConstants::kThinStringTag as i64)); //FIXME
                // Fallthrough to thin string.
            }

            // Is a thin string.
            {
                self.LoadTaggedField(string, string, offsetof!(ThinString, actual_));
                Self::MacroAssembler::Branch(self, &loop_label, Label::