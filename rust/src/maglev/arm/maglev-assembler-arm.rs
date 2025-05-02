// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/interface-descriptors-inl.h - No direct equivalent in Rust, functionality may be incorporated elsewhere.
// src/deoptimizer/deoptimizer.h - No direct equivalent in Rust, functionality may be incorporated elsewhere.
// src/maglev/maglev-assembler-inl.h - Assuming some definitions are needed, creating a basic module.
mod maglev_assembler_inl {
    pub struct MaglevAssembler {}
    impl MaglevAssembler {
        pub fn allow_allocate(&self) -> bool {
            true // Placeholder
        }
        pub fn new() -> Self {
            MaglevAssembler {}
        }
    }

    pub struct TemporaryRegisterScope<'a> {
        masm: &'a mut MaglevAssembler,
    }

    impl<'a> TemporaryRegisterScope<'a> {
        pub fn new(masm: &'a mut MaglevAssembler) -> Self {
            TemporaryRegisterScope { masm }
        }

        pub fn acquire_scratch(&mut self) -> Register {
            Register {} // Placeholder for acquiring a register
        }

        pub fn include(&mut self, _regs: &[Register]) {
            // Placeholder for including registers
        }

        pub fn available(&self) -> AvailableRegisters {
            AvailableRegisters {}
        }
    }

    pub struct AvailableRegisters {}

    impl AvailableRegisters {
        pub fn has(&self, _reg: Register) -> bool {
            false
        }
    }

    pub struct RegisterSnapshot {
        pub live_tagged_registers: RegisterSet,
        pub live_registers: RegisterSet,
    }

    pub struct RegisterSet {
    }

    impl RegisterSet {
        pub fn set(&mut self, _register: Register) {}
        pub fn has(&self, _register: Register) -> bool { false }
    }

    pub struct ZoneLabelRef<'a> {
        masm: &'a MaglevAssembler,
    }

    impl<'a> ZoneLabelRef<'a> {
        pub fn new(masm: &'a MaglevAssembler) -> Self {
            ZoneLabelRef { masm }
        }
    }

    pub struct SaveRegisterStateForCall<'a> {
        masm: &'a mut MaglevAssembler,
        snapshot: RegisterSnapshot,
    }

    impl<'a> SaveRegisterStateForCall<'a> {
        pub fn new(masm: &'a mut MaglevAssembler, snapshot: RegisterSnapshot) -> Self {
            SaveRegisterStateForCall { masm, snapshot }
        }

        pub fn define_safepoint(&mut self) {}
    }
}
// src/maglev/maglev-graph.h - Assuming some definitions are needed, creating a basic module.
mod maglev_graph {
    pub struct Graph {
        is_osr_val: bool,
        has_recursive_calls_val: bool,
        min_maglev_stackslots_for_unoptimized_frame_size_val: u32,
        tagged_stack_slots_val: u32,
        untagged_stack_slots_val: u32,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                is_osr_val: false,
                has_recursive_calls_val: false,
                min_maglev_stackslots_for_unoptimized_frame_size_val: 0,
                tagged_stack_slots_val: 0,
                untagged_stack_slots_val: 0,
            }
        }

        pub fn is_osr(&self) -> bool {
            self.is_osr_val
        }
        pub fn has_recursive_calls(&self) -> bool {
            self.has_recursive_calls_val
        }
        pub fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {
            self.min_maglev_stackslots_for_unoptimized_frame_size_val
        }
        pub fn tagged_stack_slots(&self) -> u32 {
            self.tagged_stack_slots_val
        }
        pub fn untagged_stack_slots(&self) -> u32 {
            self.untagged_stack_slots_val
        }

        pub fn set_is_osr(&mut self, value: bool) {
            self.is_osr_val = value;
        }

        pub fn set_has_recursive_calls(&mut self, value: bool) {
            self.has_recursive_calls_val = value;
        }

        pub fn set_min_maglev_stackslots_for_unoptimized_frame_size(&mut self, value: u32) {
            self.min_maglev_stackslots_for_unoptimized_frame_size_val = value;
        }

        pub fn set_tagged_stack_slots(&mut self, value: u32) {
            self.tagged_stack_slots_val = value;
        }

        pub fn set_untagged_stack_slots(&mut self, value: u32) {
            self.untagged_stack_slots_val = value;
        }
    }
}

mod v8_flags {
    pub static mut single_generation: bool = false;
    pub static mut debug_code: bool = true;
    pub static turbofan: bool = true;
}

mod compilation_info {
    pub struct CompilationInfo {
        toplevel_compilation_unit: ToplevelCompilationUnit
    }
    impl CompilationInfo {
        pub fn new() -> Self {
            CompilationInfo {
                toplevel_compilation_unit: ToplevelCompilationUnit::new()
            }
        }
        pub fn toplevel_compilation_unit(&self) -> &ToplevelCompilationUnit {
            &self.toplevel_compilation_unit
        }
    }
    pub struct ToplevelCompilationUnit {
        feedback: Feedback,
    }
    impl ToplevelCompilationUnit {
        pub fn new() -> Self {
            ToplevelCompilationUnit {
                feedback: Feedback::new(),
            }
        }
        pub fn feedback(&self) -> &Feedback {
            &self.feedback
        }
    }
    pub struct Feedback {
        object: i32,
    }
    impl Feedback {
        pub fn new() -> Self {
            Feedback {
                object: 0,
            }
        }
        pub fn object(&self) -> i32 {
            self.object
        }
    }
}

use std::mem;

use crate::maglev_assembler_inl::{MaglevAssembler, RegisterSnapshot, TemporaryRegisterScope, ZoneLabelRef, SaveRegisterStateForCall};
use crate::maglev_graph::Graph;

const kSystemPointerSize: usize = 8;
const kTaggedSizeLog2: usize = 2;
const kHeapObjectTag: i32 = 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Register {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DoubleRegister {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SwVfpRegister {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct LowDwVfpRegister {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AllocationType {
    kOld,
    kYoung, // Example
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AllocationAlignment {
    kTaggedAligned,
    // Other alignments
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AbortReason {
    kUnexpectedValue,
    kOsrUnexpectedStackSize
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RootIndex {
    kSingleCharacterStringTable,
    kempty_string,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CodeKind {
    MAGLEV,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Builtin {
    kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
    kDoubleToI,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BuiltinStringPrototypeCharCodeOrCodePointAtMode {
    kCharCodeAt,
    kCodePointAt,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CharCodeMaskMode {
    kMustApplyMask
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum StackFrame {
    MAGLEV,
}

const FIRST_STRING_TYPE: i32 = 1;
const LAST_STRING_TYPE: i32 = 2;
const kStringRepresentationMask: i32 = 4;
const kSeqStringTag: i32 = 5;
const kConsStringTag: i32 = 6;
const kSlicedStringTag: i32 = 7;
const kThinStringTag: i32 = 8;
const kOneByteStringTag: i32 = 9;
const kJavaScriptCallArgCountRegister: Register = Register {};
const kJSFunctionRegister: Register = Register {};
const kContextRegister: Register = Register {};
const kJavaScriptCallNewTargetRegister: Register = Register {};
const kReturnRegister0: Register = Register {};

const OFFSET_OF_DATA_START_FIXED_ARRAY: usize = 0; // Assuming offset for FixedArray
const OFFSET_OF_DATA_START_SEQ_ONE_BYTE_STRING: usize = 0; // Assuming offset
const OFFSET_OF_DATA_START_SEQ_TWO_BYTE_STRING: usize = 0; // Assuming offset

const kDoubleSize: usize = 8;

const STANDARD_FRAME_CONSTANTS_K_FIXED_FRAME_SIZE_FROM_FP: usize = 16;

struct Isolate {}
impl Isolate {
    fn new() -> Self {
        Isolate {}
    }
}

struct Operand {}

impl Operand {
    fn new(_value: i32) -> Self {
        Operand {}
    }
}

impl MaglevAssembler {
    fn sub(&mut self, _dest: Register, _src: Register, _op: Operand, _leave_cc: ()) {}
    fn add(&mut self, _dest: Register, _src: Register, _op: Operand, _leave_cc: ()) {}
    fn ldr(&mut self, _dest: Register, _mem: Operand) {}
    fn cmp(&mut self, _reg1: Register, _reg2: Register) {}
    fn move_(&mut self, _dest: Operand, _src: Register) {}
    fn bind(&mut self, _label: &ZoneLabelRef) {}
    fn push(&mut self, _reg: Register) {}
    fn enter_frame(&mut self, _frame_type: StackFrame) {}
    fn call_runtime(&mut self, _runtime_function: ()) {}
    fn and_(&mut self, _dest: Register, _src: Register, _op: Operand) {}
    fn tst(&mut self, _reg: Register, _op: Operand) {}
    fn ldrb(&mut self, _dest: Register, _mem: Operand) {}
    fn ldrh(&mut self, _dest: Register, _mem: Operand) {}
    fn lsl(&mut self, _dest: Register, _src: Register, _op: Operand) {}
    fn vcvt_s32_f64(&mut self, _dest: SwVfpRegister, _src: DoubleRegister) {}
    fn vmov(&mut self, _dest: Register, _src: SwVfpRegister) {}
    fn vcvt_f64_s32(&mut self, _dest: DoubleRegister, _src: SwVfpRegister) {}
    fn jump_if(&mut self, _condition: Condition, _label: &ZoneLabelRef) {}
    fn vcvt_u32_f64(&mut self, _dest: SwVfpRegister, _src: DoubleRegister) {}
    fn vcvt_f64_u32(&mut self, _dest: DoubleRegister, _src: SwVfpRegister) {}

    fn allocate(&mut self, register_snapshot: RegisterSnapshot, object: Register, size_in_bytes: i32, alloc_type: AllocationType, alignment: AllocationAlignment) {
        allocate_raw(self, &mut self.isolate, register_snapshot, object, size_in_bytes, alloc_type, alignment);
    }

    fn allocate_register(&mut self, register_snapshot: RegisterSnapshot, object: Register, size_in_bytes: Register, alloc_type: AllocationType, alignment: AllocationAlignment) {
        allocate_raw(self, &mut self.isolate, register_snapshot, object, size_in_bytes, alloc_type, alignment);
    }

    fn osr_prologue(&mut self, graph: &mut Graph) {
        let mut temps = TemporaryRegisterScope::new(self);
        let scratch = temps.acquire_scratch();

        assert!(graph.is_osr());
        assert!(!graph.has_recursive_calls());

        let source_frame_size =
            graph.min_maglev_stackslots_for_unoptimized_frame_size();

        if unsafe { v8_flags::debug_code } {
            self.add(scratch, sp, Operand::new(
                (source_frame_size as usize * kSystemPointerSize +
                    STANDARD_FRAME_CONSTANTS_K_FIXED_FRAME_SIZE_FROM_FP) as i32));
            self.cmp(scratch, fp);
            self.assert(Condition::Eq, AbortReason::kOsrUnexpectedStackSize);
        }

        let target_frame_size =
            graph.tagged_stack_slots() + graph.untagged_stack_slots();
        assert!(source_frame_size <= target_frame_size);

        if source_frame_size < target_frame_size {
            //ASM_CODE_COMMENT_STRING(self, "Growing frame for OSR");
            let additional_tagged =
                if source_frame_size < graph.tagged_stack_slots() {
                    graph.tagged_stack_slots() - source_frame_size
                } else {
                    0
                };
            if additional_tagged > 0 {
                self.move_(Operand::new(0), scratch);
            }
            for _i in 0..additional_tagged {
                self.push(scratch);
            }
            let size_so_far = source_frame_size + additional_tagged;
            assert!(size_so_far <= target_frame_size);
            if size_so_far < target_frame_size {
                self.sub(sp, sp, Operand::new(((target_frame_size - size_so_far) as usize * kSystemPointerSize) as i32));
            }
        }
    }

    fn prologue(&mut self, graph: &mut Graph) {
        let mut temps = TemporaryRegisterScope::new(self);
        temps.include(&[r4, r8]);

        assert!(!graph.is_osr());

        self.bailout_if_deoptimized();

        if graph.has_recursive_calls() {
            self.bind(&self.code_gen_state.entry_label);
        }

        if unsafe { v8_flags::turbofan } {
            // Tiering support.
            //Using D = MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor;
            let flags: Register = Register {}; //D::GetRegisterParameter(D::kFlags);
            let feedback_vector: Register = Register {}; //D::GetRegisterParameter(D::kFeedbackVector);
            //DCHECK(!AreAliased(flags, feedback_vector, kJavaScriptCallArgCountRegister,
            //                   kJSFunctionRegister, kContextRegister,
            //                   kJavaScriptCallNewTargetRegister));
            assert!(!temps.available().has(flags));
            assert!(!temps.available().has(feedback_vector));
            self.move_(Operand::new(compilation_info().toplevel_compilation_unit().feedback().object()), feedback_vector);
            //Condition needs_processing =
            //    LoadFeedbackVectorFlagsAndCheckIfNeedsProcessing(flags, feedback_vector,
            //                                                     CodeKind::MAGLEV);
            // Tail call on Arm produces 3 instructions, so we emit that in deferred
            // code.
            //JumpToDeferredIf(needs_processing, [](MaglevAssembler* masm) {
            //    __ TailCallBuiltin(
            //        Builtin::kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot);
            //});
        }

        self.enter_frame(StackFrame::MAGLEV);
        // Save arguments in frame.
        // TODO(leszeks): Consider eliding this frame if we don't make any calls
        // that could clobber these registers.
        self.push(kContextRegister);
        self.push(kJSFunctionRegister);              // Callee's JS function.
        self.push(kJavaScriptCallArgCountRegister);  // Actual argument count.

        // Initialize stack slots.
        if graph.tagged_stack_slots() > 0 {
            //ASM_CODE_COMMENT_STRING(this, "Initializing stack slots");
            let mut temps = TemporaryRegisterScope::new(self);
            let scratch = temps.acquire_scratch();
            self.move_(Operand::new(0), scratch);

            // Magic value. Experimentally, an unroll size of 8 doesn't seem any
            // worse than fully unrolled pushes.
            const K_LOOP_UNROLL_SIZE: u32 = 8;
            let tagged_slots: u32 = graph.tagged_stack_slots();
            if tagged_slots < K_LOOP_UNROLL_SIZE {
                // If the frame is small enough, just unroll the frame fill
                // completely.
                for _i in 0..tagged_slots {
                    self.push(scratch);
                }
            } else {
                // Extract the first few slots to round to the unroll size.
                let first_slots = tagged_slots % K_LOOP_UNROLL_SIZE;
                for _i in 0..first_slots {
                    self.push(scratch);
                }
                let unroll_counter_reg = temps.acquire_scratch();
                let unroll_counter = tagged_slots / K_LOOP_UNROLL_SIZE;
                self.move_(Operand::new(unroll_counter as i32), unroll_counter_reg);
                // We enter the loop unconditionally, so make sure we need to loop at
                // least once.
                assert!(tagged_slots / K_LOOP_UNROLL_SIZE > 0);
                let mut loop_label = ZoneLabelRef::new(self);
                self.bind(&loop_label);
                for _i in 0..K_LOOP_UNROLL_SIZE {
                    self.push(scratch);
                }
                self.sub(unroll_counter_reg, unroll_counter_reg, Operand::new(1), ());
                self.jump_if(Condition::GreaterThan, &loop_label);
            }
        }
        if graph.untagged_stack_slots() > 0 {
            // Extend rsp by the size of the remaining untagged part of the frame,
            // no need to initialise these.
            self.sub(sp, sp, Operand::new((graph.untagged_stack_slots() as usize * kSystemPointerSize) as i32));
        }
    }

    fn maybe_emit_deopt_builtins_call(&mut self, _eager_deopt_count: usize, _eager_deopt_entry: &ZoneLabelRef, _lazy_deopt_count: usize, _lazy_deopt_entry: &ZoneLabelRef) {
        self.check_const_pool(true, false);
    }

    fn load_single_character_string(&mut self, result: Register, char_code: Register, scratch: Register) {
        assert_ne!(char_code, scratch);
        if unsafe { v8_flags::debug_code } {
            self.cmp(char_code, Operand::new(StringConstants::K_MAX_ONE_BYTE_CHAR_CODE));
            self.assert(Condition::UnsignedLessThanEqual, AbortReason::kUnexpectedValue);
        }
        let table = scratch;
        self.load_root(table, RootIndex::kSingleCharacterStringTable);
        self.add(table, table, Operand::new((char_code as i32) << kTaggedSizeLog2));
        self.ldr(result, Operand::new(0)); //FieldMemOperand(table, OFFSET_OF_DATA_START(FixedArray)));
    }

    fn string_from_char_code(&mut self, register_snapshot: RegisterSnapshot, char_code_fits_one_byte: Option<&ZoneLabelRef>, result: Register, char_code: Register, scratch: Register, mask_mode: CharCodeMaskMode) {
        self.assert_zero_extended(char_code);
        assert_ne!(char_code, scratch);
        let done = ZoneLabelRef::new(self);
        if mask_mode == CharCodeMaskMode::kMustApplyMask {
            self.and_(char_code, char_code, Operand::new(0xFFFF));
        }
        self.cmp(char_code, Operand::new(StringConstants::K_MAX_ONE_BYTE_CHAR_CODE));

        self.jump_to_deferred_if(
            Condition::UnsignedGreaterThan,
            |masm: &mut MaglevAssembler, register_snapshot: RegisterSnapshot, done: ZoneLabelRef, result: Register, char_code: Register, scratch: Register| {
                // Be sure to save {char_code}. If it aliases with {result}, use
                // the scratch register.
                // TODO(victorgomes): This is probably not needed any more, because
                // we now ensure that results registers don't alias with inputs/temps.
                // Confirm, and drop this check.
                let mut char_code = char_code;
                if char_code == result {
                    masm.move_(Operand::new(char_code as i32), scratch);
                    char_code = scratch;
                }
                assert_ne!(char_code, result);
                assert!(!register_snapshot.live_tagged_registers.has(char_code));
                register_snapshot.live_registers.set(char_code);
                //__ AllocateTwoByteString(register_snapshot, result, 1);
                //__ strh(char_code, FieldMemOperand(
                //    result, OFFSET_OF_DATA_START(SeqTwoByteString)));
                masm.jump(&done);
            },
            register_snapshot, done, result, char_code, scratch);

        if let Some(char_code_fits_one_byte) = char_code_fits_one_byte {
            self.bind(char_code_fits_one_byte);
        }

        self.load_single_character_string(result, char_code, scratch);
        self.bind(&done);
    }

    fn string_char_code_or_code_point_at(&mut self, mode: BuiltinStringPrototypeCharCodeOrCodePointAtMode, register_snapshot: &mut RegisterSnapshot, result: Register, string: Register, index: Register, instance_type: Register, scratch2: Register, result_fits_one_byte: Option<&ZoneLabelRef>) {
        let done = ZoneLabelRef::new(self);
        let seq_string_label = ZoneLabelRef::new(self);
        let cons_string_label = ZoneLabelRef::new(self);
        let sliced_string_label = ZoneLabelRef::new(self);

        let deferred_runtime_call = self.make_deferred_code(
            move |masm: &mut MaglevAssembler, mode: BuiltinStringPrototypeCharCodeOrCodePointAtMode, register_snapshot: RegisterSnapshot, done: ZoneLabelRef, result: Register, string: Register, index: Register| {
                assert!(!register_snapshot.live_registers.has(result));
                assert!(!register_snapshot.live_registers.has(string));
                assert!(!register_snapshot.live_registers.has(index));

                let mut save_register_state = SaveRegisterStateForCall::new(masm, register_snapshot);

                //__ SmiTag(index);
                masm.push(string);
                masm.push(index);
                //__ Move(kContextRegister, masm->native_context().object());

                // This call does not throw nor can deopt.
                //let runtime_function = match mode {
                //    BuiltinStringPrototypeCharCodeOrCodePointAtMode::kCodePointAt => Runtime::kStringCodePointAt,
                //    BuiltinStringPrototypeCharCodeOrCodePointAtMode::kCharCodeAt => Runtime::kStringCharCodeAt,
                //};
                //__ CallRuntime(runtime_function);

                save_register_state.define_safepoint();

                //__ SmiUntag(kReturnRegister0);
                masm.move_(Operand::new(kReturnRegister0 as i32), result);

                masm.jump(&done);
            },
            mode, *register_snapshot, done, result, string, index);

        // We might need to try more than one time for ConsString, SlicedString and
        // ThinString.
        let mut loop_label = ZoneLabelRef::new(self);
        self.bind(&loop_label);

        if unsafe { v8_flags::debug_code } {
            // Check if {string} is a string.
            //AssertObjectTypeInRange(string, FIRST_STRING_TYPE, LAST_STRING_TYPE,
            //    AbortReason::kUnexpectedValue);

            let scratch = instance_type;
            //ldr(scratch, FieldMemOperand(string, offsetof(String, length_)));
            self.ldr(scratch, Operand::new(0));
            self.cmp(index, scratch);
            self.check(Condition::LessThan, AbortReason::kUnexpectedValue);
        }

        // Get instance type.
        self.load_instance_type(instance_type, string);

        {
            let mut temps = TemporaryRegisterScope::new(self);
            let representation = temps.acquire_scratch();

            // TODO(victorgomes): Add fast path for external strings.
            self.and_(representation, instance_type, Operand::new(kStringRepresentationMask));
            self.cmp(representation, Operand::new(kSeqStringTag));
            self.jump_if(Condition::Eq, &seq_string_label);
            self.cmp(representation, Operand::new(kConsStringTag));
            self.jump_if(Condition::Eq, &cons_string_label);
            self.cmp(representation, Operand::new(kSlicedStringTag));
            self.jump_if(Condition::Eq, &sliced_string_label);
            self.cmp(representation, Operand::new(kThinStringTag));
            //self.jump_if(Condition::NotEqual, deferred_runtime_call);

            // Fallthrough to thin string.
        }

        // Is a thin string.
        {
            //ldr(string, FieldMemOperand(string, offsetof(ThinString, actual_)));
            self.ldr(string, Operand::new(0));
            self.jump(&loop_label);
        }

        self.bind(&sliced_string_label);
        {
            let mut temps = TemporaryRegisterScope::new(self);
            let offset = temps.acquire_scratch();

            //LoadAndUntagTaggedSignedField(offset, string,
            //    offsetof(SlicedString, offset_));
            //LoadTaggedField(string, string, offsetof(SlicedString, parent_));
            self.ldr(offset, Operand::new(0));
            self.ldr(string, Operand::new(0));
            self.add(index, index, Operand::new(0)); //offset);
            self.jump(&loop_label);
        }

        self.bind(&cons_string_label);
        {
            // Reuse {instance_type} register here, since CompareRoot requires a scratch
            // register as well.
            let second_string = instance_type;
            //ldr(second_string, FieldMemOperand(string, offsetof(ConsString, second_)));
            self.ldr(second_string, Operand::new(0));
            //CompareRoot(second_string, RootIndex::kempty_string);
            //self.jump_if(Condition::NotEqual, deferred_runtime_call);
            //ldr(string, FieldMemOperand(string, offsetof(ConsString, first_)));
            self.ldr(string, Operand::new(0));
            self.jump(&loop_label);  // Try again with first string.
        }

        self.bind(&seq_string_label);
        {
            let two_byte_string_label = ZoneLabelRef::new(self);
            self.tst(instance_type, Operand::new(kOneByteStringTag));
            self.jump_if(Condition::Eq, &two_byte_string_label);

            // The result of one-byte string will be the same for both modes
            // (CharCodeAt/CodePointAt), since it cannot be the first half of a
            // surrogate pair.
            self.add(index, index, Operand::new(0)); //OFFSET_OF_DATA_START(SeqOneByteString) - kHeapObjectTag));
            self.ldrb(result, Operand::new(0)); //MemOperand(string, index));

            if let Some(result_fits_one_byte) = result_fits_one_byte {
                //FIXME was b
                self.jump(result_fits_one_byte);
            } else {
                self.jump(&done);
            }

            self.bind(&two_byte_string_label);
            // {instance_type} is unused from this point, so we can use as scratch.
            let scratch = instance_type;
            self.lsl(scratch, index, Operand::new(1));
            self.add(scratch, scratch, Operand::new(0)); //OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag));

            if mode == BuiltinStringPrototypeCharCodeOrCodePointAtMode::kCharCodeAt {
                self.ldrh(result, Operand::new(0)); //MemOperand(string, scratch));
            } else {
                assert_eq!(mode, BuiltinStringPrototypeCharCodeOrCodePointAtMode::kCodePointAt);
                let string_backup = string;
                let mut string_backup = string_backup;
                if result == string {
                    string_backup = scratch2;
                    self.move_(Operand::new(string as i32), string_backup);
                }
                self.ldrh(result, Operand::new(0)); //MemOperand(string, scratch));

                let first_code_point = scratch;
                self.and_(first_code_point, result, Operand::new(0xfc00));
                self.cmp(first_code_point, Operand::new(0xd800));
                self.jump_if(Condition::NotEqual, &done);

                let length = scratch;
                //ldr(length, FieldMemOperand(string_backup, offsetof(String, length_)));
                self.ldr(length, Operand::new(0));
                self.add(index, index, Operand::new(1));
                self.cmp(index, length);
                self.jump_if(Condition::GreaterThanOrEqual, &done);

                let second_code_point = scratch;
                self.lsl(index, index, Operand::new(1));
                self.add(index, index, Operand::new(0)); //OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag));
                self.ldrh(second_code_point, Operand::new(0)); //MemOperand(string_backup, index));

                // {index} is not needed at this point.
                let scratch2_reg = index;
                self.and_(scratch2_reg, second_code_point, Operand::new(0xfc00));
                self.cmp(scratch2_reg, Operand::new(0xdc00));
                self.jump_if(Condition::NotEqual, &done);

                let surrogate_offset = 0x10000 - (0xd800 << 10) - 0xdc00;
                self.add(second_code_point, second_code_point, Operand::new(surrogate_offset));
                self.lsl(result, result, Operand::new(10));
                self.add(result, result, second_code_point);
            }
        }

        self.bind(&done);

        if unsafe { v8_flags::debug_code } {
            // We make sure that the user of this macro is not relying in string and
            // index to not be clobbered.
            if result != string {
                self.move_(Operand::new(0xdeadbeef), string);
            }
            if result != index {
                self.move_(Operand::new(0xdeadbeef), index);
            }
        }
    }

    fn truncate_double_to_int32(&mut self, dst: Register, src: DoubleRegister) {
        let done = ZoneLabelRef::new(self);
        let slow_path = self.make_deferred_code(
            move |masm: &mut MaglevAssembler, src: DoubleRegister, dst: Register, done: ZoneLabelRef| {
                //__ push(lr);
                //__ AllocateStackSpace(kDoubleSize);
                //__ vstr(src, MemOperand(sp, 0));
                //__ CallBuiltin(Builtin::kDoubleToI);
                //__ ldr(dst, MemOperand(sp, 0));
                //__ add(sp, sp, Operand(kDoubleSize));
                //__ pop(lr);
                masm.jump(&done);
            },
            src, dst, done);
        //self.try_inline_truncate_double_to_i(dst, src, done);
        self.jump(slow_path);
        self.bind(&done);
    }

    fn try_truncate_double_to_int32(&mut self, dst: Register, src: DoubleRegister, fail: &ZoneLabelRef) {
        let mut temps = UseScratchRegisterScope::new(self);
        let low_double = temps.acquire_low_d();
        let temp_vfps = low_double.low();
        let converted_back = low_double;
        let done = ZoneLabelRef::new(self);

        // Convert the input float64 value to int32.