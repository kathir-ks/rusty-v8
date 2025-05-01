// src/baseline/arm64/baseline_assembler_arm64_inl.rs

// This is a header file translation, so we'll define a module.

pub mod baseline_assembler_arm64_inl {
    //use crate::baseline::baseline_assembler::*; // Assuming baseline_assembler is in a separate file
    //use crate::codegen::arm64::macro_assembler_arm64_inl::*; // Assuming macro_assembler_arm64_inl is in a separate file
    //use crate::codegen::interface_descriptors::*; // Assuming interface_descriptors is in a separate file
    //use crate::objects::literal_objects_inl::*; // Assuming literal_objects_inl is in a separate file
    //use v8::internal::*;

    // Placeholder types and constants.  These need to be fleshed out
    // based on the actual V8 codebase.

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Register(u32);

    impl Register {
        pub fn w(self) -> Self {
            Register(self.0) // Placeholder: Adjust Register type to handle W registers
        }
    }

    pub struct Immediate(i32);

    impl Immediate {
        pub fn new(value: i32) -> Self {
            Immediate(value)
        }
        pub fn ptr(&self) -> i64 {
            self.0 as i64
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RootIndex(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Condition(u32);

    pub const eq: Condition = Condition(0);
    pub const ne: Condition = Condition(1);
    pub const ge: Condition = Condition(2);
    pub const kZero: Condition = Condition(3);
    pub const kNotZero: Condition = Condition(4);
    pub const kLessThan: Condition = Condition(5);
    pub const kUnsignedGreaterThanEqual: Condition = Condition(6);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MemOperand {
        base: Register,
        offset: i32,
    }

    impl MemOperand {
        pub fn new(base: Register, offset: i32) -> Self {
            MemOperand { base, offset }
        }
        pub fn base(&self) -> Register {
            self.base
        }
        pub fn regoffset(&self) -> Register {
            Register(0) // Placeholder: implement proper register offset
        }
    }

    pub struct Label {
        id: u32, //dummy
    }
    impl Label {
        pub fn new(id: u32) -> Self {
            Label{id}
        }
    }

    pub enum LabelDistance {
        Near,
        Far
    }

    pub struct Operand(i32);
    impl Operand {
        pub fn new(value: i32) -> Self {
            Operand(value)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InstanceType(u32);
    pub const MAP_TYPE: InstanceType = InstanceType(10); //dummy

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FeedbackSlot(u32);
    pub struct FeedbackVector(u32);

    pub struct ExternalReference(u32);
    pub struct Handle<T>(u32, std::marker::PhantomData<T>);

    pub struct HeapObject(u32);

    pub struct Tagged<T>(u32, std::marker::PhantomData<T>);
    pub type Smi = i32;
    pub type TaggedIndex = i32;

    impl Tagged<TaggedIndex> {
        pub fn ptr(&self) -> i64 {
            self.0 as i64
        }
    }

    pub struct FieldMemOperand {
        base: Register,
        offset: i32,
    }

    impl FieldMemOperand {
        pub fn new(base: Register, offset: i32) -> Self {
            FieldMemOperand { base, offset }
        }
    }

    pub struct UseScratchRegisterScope {}

    impl UseScratchRegisterScope {
        pub fn Include(&mut self, _reg1:Register, _reg2:Register){}
        pub fn AcquireX(&self) -> Register { Register(0) } //placeholder
    }

    pub struct MacroAssembler{}
    impl MacroAssembler {
        pub fn Push(&mut self, _reg1:Register, _reg2:Register){}
        pub fn Pop(&mut self, _reg1:Register, _reg2:Register){}
        pub fn LeaveFrame(&mut self, _stack_frame:StackFrame){}
        pub fn DropArguments(&mut self, _size:Register){}
        pub fn Ret(&mut self){}
        pub fn SmiTag(&mut self, _reg: Register){}
        pub fn PushArgument(&mut self, _reg: Register){}
        pub fn CompareTagged(&mut self, _reg1: Register, _reg2: Register){}
        pub fn Assert(&mut self, _condition: Condition, _abort_reason: AbortReason){}
        pub fn Cmp(&mut self, _reg1: Register, _reg2: Register){}
        pub fn Csel(&mut self, _dest: Register, _reg1: Register, _reg2: Register, _condition: Condition){}
    }

    pub struct MacroAssemblerBlockPoolsScope<'a> {
        masm: &'a mut MacroAssembler,
    }
    impl <'a> MacroAssemblerBlockPoolsScope<'a> {
        pub fn new(masm: &'a mut MacroAssembler, _size: i32) -> Self {
            MacroAssemblerBlockPoolsScope{masm}
        }
    }

    pub struct SourceTextModule(u32);
    impl SourceTextModule {
        pub const kRegularExportsOffset: i32 = 0;
        pub const kRegularImportsOffset: i32 = 0;
    }

    pub struct Context(u32);
    impl Context {
        pub const kPreviousOffset: i32 = 0;
        pub const kExtensionOffset: i32 = 0;

        pub fn OffsetOfElementAt(_index: u32) -> i32 {
            0
        }
    }

    pub struct Cell(u32);
    impl Cell {
        pub const kValueOffset: i32 = 0;
    }

    pub struct AbortReason(u32);
    impl AbortReason {
        pub const kAccumulatorClobbered: AbortReason = AbortReason(0);
        pub const kUnexpectedValue: AbortReason = AbortReason(1);
    }

    pub struct StackFrame(u32);
    impl StackFrame {
        pub const BASELINE: StackFrame = StackFrame(0);
    }

    pub struct StandardFrameConstants(u32);
    impl StandardFrameConstants {
        pub const kArgCOffset: i32 = 0;
    }

    pub struct BaselineFrameConstants(u32);
    impl BaselineFrameConstants {
        pub const kFeedbackVectorFromFp: i32 = 0;
        pub const kFeedbackCellFromFp: i32 = 0;
    }

    pub struct BaselineLeaveFrameDescriptor(u32);
    impl BaselineLeaveFrameDescriptor {
        pub fn WeightRegister() -> Register { Register(0)}
        pub fn ParamsSizeRegister() -> Register { Register(1)}
    }
    pub struct Runtime(u32);
    impl Runtime {
        pub const kBytecodeBudgetInterrupt_Sparkplug: i32 = 0;
    }

    pub fn SmiValuesAre31Bits() -> bool {
        false
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CodeKind(u32);
    impl CodeKind {
        pub const MAGLEV: Self = CodeKind(0);
    }

    // Globals (or Flags)
    pub struct v8_flags {
        pub debug_code: bool,
    }

    impl v8_flags {
        pub const fn new() -> Self {
            v8_flags {
                debug_code: true, //or false
            }
        }
    }
    pub const v8_flags: v8_flags = v8_flags::new();

    pub struct interpreter {
        
    }
    impl interpreter {
        pub struct Register {
            id: u32,
        }
        impl Register {
            pub fn ToOperand(&self) -> i32 {
                self.id as i32
            }
        }
        pub struct RegisterList {
            registers: Vec<Register>,
        }
        impl RegisterList {
            pub fn new(registers: Vec<Register>) -> Self{
                RegisterList{registers}
            }
            pub fn register_count(&self) -> i32 {
                self.registers.len() as i32
            }
            pub fn PopLeft(&mut self) -> Self {
                Self::new(self.registers[1..].to_vec())
            }
            pub fn get(&self, index: usize) -> Register {
                self.registers[index]
            }
            pub fn index(&self, index: usize) -> Register {
                self.get(index)
            }
        }

    }

    // Actual conversion starts here

    pub struct BaselineAssembler<'a> {
        masm_: &'a mut MacroAssembler,
        scratch_register_scope_: *mut ScratchRegisterScope<'a>, // Raw pointer
    }

    impl <'a> BaselineAssembler<'a> {
        pub fn new(masm_: &'a mut MacroAssembler) -> Self {
            BaselineAssembler {
                masm_: masm_,
                scratch_register_scope_: std::ptr::null_mut(), // Initialize to null
            }
        }

        pub fn masm(&mut self) -> &mut MacroAssembler {
            self.masm_
        }

        pub struct ScratchRegisterScope<'b> {
            assembler_: &'b mut BaselineAssembler<'a>,
            prev_scope_: *mut ScratchRegisterScope<'b>,
            wrapped_scope_: UseScratchRegisterScope,
        }

        impl <'b> ScratchRegisterScope<'b> {
            pub fn new(assembler_: &'b mut BaselineAssembler<'a>) -> Self {
                let mut wrapped_scope_ = UseScratchRegisterScope {};
                if assembler_.scratch_register_scope_.is_null() {
                    wrapped_scope_.Include(Register(14), Register(15));
                    wrapped_scope_.Include(Register(19));
                }
                let prev_scope_ = assembler_.scratch_register_scope_;
                assembler_.scratch_register_scope_ = unsafe { std::mem::transmute::<&mut ScratchRegisterScope, *mut ScratchRegisterScope>(&mut std::mem::transmute_copy(&assembler_))}; //self as *mut Self;

                ScratchRegisterScope {
                    assembler_: assembler_,
                    prev_scope_: prev_scope_,
                    wrapped_scope_: wrapped_scope_,
                }
            }
            pub fn AcquireScratch(&mut self) -> Register {
                self.wrapped_scope_.AcquireX()
            }
        }
        impl <'b> Drop for ScratchRegisterScope<'b> {
            fn drop(&mut self) {
                self.assembler_.scratch_register_scope_ = self.prev_scope_;
            }
        }

        // Placeholder registers, replace with actual registers
        const fp: Register = Register(29);
        const kInterpreterAccumulatorRegister: Register = Register(0);
        const kContextRegister: Register = Register(20);
        const kJSFunctionRegister: Register = Register(21);
        const padreg: Register = Register(30);
        const kLRHasNotBeenSaved: i32 = 0;

        pub fn register_frame_operand(
            &mut self,
            interpreter_register: interpreter::Register,
        ) -> MemOperand {
            MemOperand::new(
                Self::fp,
                interpreter_register.ToOperand() * kSystemPointerSize,
            )
        }

        pub fn register_frame_address(
            &mut self,
            interpreter_register: interpreter::Register,
            rscratch: Register,
        ) {
             self.Add(rscratch, Self::fp,
                interpreter_register.ToOperand() * kSystemPointerSize);
        }

        pub fn feedback_vector_operand(&mut self) -> MemOperand {
            MemOperand::new(Self::fp, BaselineFrameConstants::kFeedbackVectorFromFp)
        }

        pub fn feedback_cell_operand(&mut self) -> MemOperand {
            MemOperand::new(Self::fp, BaselineFrameConstants::kFeedbackCellFromFp)
        }

        pub fn bind(&mut self, label: &mut Label) {
            self.masm_.Bind(label);
        }

        pub fn jump_target(&mut self) {
            self.masm_.JumpTarget();
        }

        pub fn jump(&mut self, target: &mut Label, _distance: LabelDistance) {
            self.masm_.B(target);
        }

        pub fn jump_if_root(&mut self, value: Register, index: RootIndex, target: &mut Label, _distance: LabelDistance) {
            self.masm_.JumpIfRoot(value, index, target);
        }

        pub fn jump_if_not_root(&mut self, value: Register, index: RootIndex, target: &mut Label, _distance: LabelDistance) {
            self.masm_.JumpIfNotRoot(value, index, target);
        }

        pub fn jump_if_smi(&mut self, value: Register, target: &mut Label, _distance: LabelDistance) {
            self.masm_.JumpIfSmi(value, target);
        }

        pub fn jump_if_not_smi(&mut self, value: Register, target: &mut Label, _distance: LabelDistance) {
            self.masm_.JumpIfNotSmi(value, target);
        }

        pub fn jump_if_immediate(
            &mut self,
            cc: Condition,
            left: Register,
            right: i32,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            self.jump_if(cc, left, Operand::new(right), target, _distance);
        }

        pub fn test_and_branch(
            &mut self,
            value: Register,
            mask: i32,
            cc: Condition,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            if cc == kZero {
                self.masm_.TestAndBranchIfAllClear(value, mask, target);
            } else if cc == kNotZero {
                self.masm_.TestAndBranchIfAnySet(value, mask, target);
            } else {
                self.masm_.Tst(value, Immediate::new(mask));
                self.masm_.B(cc, target);
            }
        }

        pub fn jump_if(&mut self, cc: Condition, lhs: Register, rhs: Operand, target: &mut Label, _distance: LabelDistance) {
            self.masm_.CompareAndBranch(lhs, rhs, cc, target);
        }

        // TODO: Implement V8_STATIC_ROOTS_BOOL

        pub fn jump_if_object_type_fast(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.AcquireScratch();
            if cc == eq || cc == ne {
                self.masm_.IsObjectType(object, scratch, scratch, instance_type);
                self.masm_.B(cc, target);
                return;
            }
            self.jump_if_object_type(cc, object, instance_type, scratch, target, _distance);
        }

        pub fn jump_if_object_type(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            map: Register,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.AcquireScratch();
            self.masm_.LoadMap(map, object);
            self.masm_.Ldrh(type_reg, FieldMemOperand::new(map, Map::kInstanceTypeOffset));
            self.jump_if(cc, type_reg, Operand::new(instance_type.0 as i32), target);
        }

        pub fn jump_if_instance_type(
            &mut self,
            map: Register,
            instance_type: InstanceType,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let type_reg = temps.AcquireScratch();
            if v8_flags.debug_code {
                self.masm_.AssertNotSmi(map);
                self.masm_.CompareObjectType(map, type_reg, type_reg, MAP_TYPE);
                self.masm_.Assert(eq, AbortReason::kUnexpectedValue);
            }
            self.masm_.Ldrh(type_reg, FieldMemOperand::new(map, Map::kInstanceTypeOffset));
            self.jump_if(cc, type_reg, Operand::new(instance_type.0 as i32), target);
        }

        pub fn jump_if_pointer(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.AcquireScratch();
            self.masm_.Ldr(tmp, operand);
            self.jump_if(cc, value, Operand::new(tmp.0 as i32), target);
        }

        pub fn jump_if_smi_smi(
            &mut self,
            cc: Condition,
            value: Register,
            smi: Tagged<Smi>,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            self.masm_.AssertSmi(value);
            self.masm_.CompareTaggedAndBranch(value, smi, cc, target);
        }

        pub fn jump_if_smi(
            &mut self,
            cc: Condition,
            lhs: Register,
            rhs: Register,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            self.masm_.AssertSmi(lhs);
            self.masm_.AssertSmi(rhs);
            self.masm_.CompareTaggedAndBranch(lhs, rhs, cc, target);
        }

        pub fn jump_if_tagged(
            &mut self,
            cc: Condition,
            value: Register,
            operand: MemOperand,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.AcquireScratch();
            self.masm_.Ldr(tmp, operand);
            self.masm_.CompareTaggedAndBranch(value, tmp, cc, target);
        }

        pub fn jump_if_tagged_memop(
            &mut self,
            cc: Condition,
            operand: MemOperand,
            value: Register,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.AcquireScratch();
            self.masm_.Ldr(tmp, operand);
            self.masm_.CompareTaggedAndBranch(tmp, value, cc, target);
        }

        pub fn jump_if_byte(
            &mut self,
            cc: Condition,
            value: Register,
            byte: i32,
            target: &mut Label,
            _distance: LabelDistance,
        ) {
            self.jump_if(cc, value, Operand::new(byte), target);
        }

        pub fn move_reg(
            &mut self,
            output: interpreter::Register,
            source: Register,
        ) {
            self.move_memop(self.register_frame_operand(output), source);
        }

        pub fn move_tagged_index(
            &mut self,
            output: Register,
            value: Tagged<TaggedIndex>,
        ) {
            self.masm_.Mov(output, Immediate::new(value.ptr() as i32));
        }

        pub fn move_memop(
            &mut self,
            output: MemOperand,
            source: Register,
        ) {
            self.masm_.Str(source, output);
        }

        pub fn move_external_reference(
            &mut self,
            output: Register,
            reference: ExternalReference,
        ) {
            self.masm_.Mov(output, Operand::new(reference.0 as i32));
        }

        pub fn move_handle(
            &mut self,
            output: Register,
            value: Handle<HeapObject>,
        ) {
            self.masm_.Mov(output, Operand::new(value.0 as i32));
        }

        pub fn move_int32(
            &mut self,
            output: Register,
            value: i32,
        ) {
            self.masm_.Mov(output, Immediate::new(value));
        }

        pub fn move_maybe_smi(
            &mut self,
            output: Register,
            source: Register,
        ) {
            self.masm_.Mov(output, source);
        }

        pub fn move_smi(
            &mut self,
            output: Register,
            source: Register,
        ) {
            self.masm_.Mov(output, source);
        }

        fn load_tagged_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedField(output, FieldMemOperand::new(source, offset));
        }

        fn load_tagged_signed_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedSignedField(output, FieldMemOperand::new(source, offset));
        }

        fn load_tagged_signed_field_and_untag(&mut self, output: Register, source: Register, offset: i32) {
            self.load_tagged_signed_field(output, source, offset);
            self.smi_untag(output);
        }

        fn load_word16_field_zero_extend(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.Ldrh(output, FieldMemOperand::new(source, offset));
        }

        fn load_word8_field(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.Ldrb(output, FieldMemOperand::new(source, offset));
        }

        fn store_tagged_signed_field(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
            //ASM_CODE_COMMENT(masm_);
            let mut temps = ScratchRegisterScope::new(self);
            let tmp = temps.AcquireScratch();
            self.masm_.Mov(tmp, Operand::new(value.0 as i32));
            self.masm_.StoreTaggedField(tmp, FieldMemOperand::new(target, offset));
        }

        fn store_tagged_field_with_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            //ASM_CODE_COMMENT(masm_);
            self.masm_.StoreTaggedField(value, FieldMemOperand::new(target, offset));
            self.masm_.RecordWriteField(target, offset, value, Self::kLRHasNotBeenSaved, SaveFPRegsMode::kIgnore);
        }

        fn store_tagged_field_no_write_barrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_.StoreTaggedField(value, FieldMemOperand::new(target, offset));
        }

        fn try_load_optimized_osr_code(&mut self, scratch_and_result: Register, feedback_vector: Register, slot: FeedbackSlot, on_result: &mut Label, _distance: LabelDistance) {
            self.masm_.TryLoadOptimizedOsrCode(scratch_and_result, CodeKind::MAGLEV, feedback_vector, slot, on_result, LabelDistance::Far);
        }

        fn add_to_interrupt_budget_and_jump_if_not_exceeded(&mut self, weight: i32, skip_interrupt_label: &mut Label) {
            //ASM_CODE_COMMENT(masm_);
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.AcquireScratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.AcquireScratch().w();
            self.masm_.Ldr(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset));
            // Remember to set flags as part of the add!
            self.masm_.Adds(interrupt_budget, interrupt_budget, weight);
            self.masm_.Str(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset));
            if skip_interrupt_label.id != 0 {
                // Use compare flags set by Adds
                assert!(weight < 0);
                self.masm_.B(ge, skip_interrupt_label);
            }
        }

        fn add_to_interrupt_budget_and_jump_if_not_exceeded_reg(&mut self, weight: Register, skip_interrupt_label: &mut Label) {
            //ASM_CODE_COMMENT(masm_);
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.AcquireScratch();
            self.load_feedback_cell(feedback_cell);

            let interrupt_budget = scratch_scope.AcquireScratch().w();
            self.masm_.Ldr(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset));
            // Remember to set flags as part of the add!
            self.masm_.Adds(interrupt_budget, interrupt_budget, weight.w().0 as i32);
            self.masm_.Str(interrupt_budget, FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset));
            if skip_interrupt_label.id != 0 {
                self.masm_.B(ge, skip_interrupt_label);
            }
        }

        fn lda_context_slot(&mut self, context: Register, index: u32, depth: u32, _compression_mode: i32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(Self::kInterpreterAccumulatorRegister, context, Context::OffsetOfElementAt(index));
        }

        fn sta_context_slot(&mut self, context: Register, value: Register, index: u32, depth: u32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.store_tagged_field_with_write_barrier(context, Context::OffsetOfElementAt(index), value);
        }

        fn lda_module_variable(&mut self, context: Register, cell_index: i32, depth: u32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset);
            if cell_index > 0 {
                self.load_tagged_field(context, context, SourceTextModule::kRegularExportsOffset);
                // The actual array index is (cell_index - 1).
                let cell_index = cell_index - 1;
            } else {
                self.load_tagged_field(context, context, SourceTextModule::kRegularImportsOffset);
                // The actual array index is (-cell_index - 1).
                let cell_index = -cell_index - 1;
            }
            self.load_fixed_array_element(context, context, cell_index);
            self.load_tagged_field(Self::kInterpreterAccumulatorRegister, context, Cell::kValueOffset);
        }

        fn sta_module_variable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
            for _ in 0..depth {
                self.load_tagged_field(context, context, Context::kPreviousOffset);
            }
            self.load_tagged_field(context, context, Context::kExtensionOffset);
            self.load_tagged_field(context, context, SourceTextModule::kRegularExportsOffset);

            // The actual array index is (cell_index - 1).
            let cell_index = cell_index - 1;
            self.load_fixed_array_element(context, context, cell_index);
            self.store_tagged_field_with_write_barrier(context, Cell::kValueOffset, value);
        }

        fn increment_smi(&mut self, lhs: MemOperand) {
            let mut temps = ScratchRegisterScope::new(self);
            let mut tmp = temps.AcquireScratch();
            if SmiValuesAre31Bits() {
                tmp = tmp.w();
            }
            self.masm_.Ldr(tmp, lhs);
            self.masm_.Add(tmp, tmp, Operand::new(Smi::FromInt(1)));
            self.masm_.Str(tmp, lhs);
        }

        fn word32_and(&mut self, output: Register, lhs: Register, rhs: i32) {
            self.masm_.And(output, lhs, Immediate::new(rhs));
        }

        fn switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: i32) {
            //ASM_CODE_COMMENT(masm_);
            let mut fallthrough = Label::new(0);
            if case_value_base != 0 {
                self.masm_.Sub(reg, reg, Immediate::new(case_value_base));
            }

            // Mostly copied from code-generator-arm64.cc
            let mut scope = ScratchRegisterScope::new(self);
            let temp = scope.AcquireScratch();
            let mut table = Label::new(1);
            self.jump_if(kUnsignedGreaterThanEqual, reg, Immediate::new(num_labels), &mut fallthrough, LabelDistance::Near);
            self.masm_.Adr(temp, &mut table);
            let entry_size_log2 = 2;
            let instructions_per_jump_target = 0;
            let instructions_per_label = 1 + instructions_per_jump_target;
            self.masm_.Add(temp, temp, Operand::new((reg.0 as i32) << entry_size_log2));
            self.masm_.Br(temp);
            {
                let instruction_count = num_labels * instructions_per_label + instructions_per_jump_target;
                let mut block_pools = MacroAssemblerBlockPoolsScope::new(self.masm_, instruction_count * kInstrSize);
                self.masm_.Bind(&mut table);
                for i in 0..num_labels {
                    self.masm_.JumpTarget();
                    self.masm_.B(labels[i]);
                }
                self.masm_.JumpTarget();
                self.masm_.Bind(&mut fallthrough);
            }
        }

        fn emit_return(&mut self, masm: &mut MacroAssembler) {
            //ASM_CODE_COMMENT(masm);
            let mut basm = BaselineAssembler::new(masm);

            let weight = BaselineLeaveFrameDescriptor::WeightRegister();
            let params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();

            {
                //ASM_CODE_COMMENT_STRING(masm, "Update Interrupt Budget");

                let mut skip_interrupt_label = Label::new(2);
                basm.add_to_interrupt_budget_and_jump_if_not_exceeded(10, &mut skip_interrupt_label); //weight, &skip_interrupt_label);
                basm.masm().SmiTag(params_size);
                basm.masm().Push(params_size, Self::kInterpreterAccumulatorRegister);

                basm.load_context(Self::kContextRegister);
                basm.load_function(Self::kJSFunctionRegister);
                basm.masm().PushArgument(Self::kJSFunctionRegister);
                basm.masm().CallRuntime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

                basm.masm().Pop(Self::kInterpreterAccumulatorRegister, params_size);
                basm.masm().SmiUntag(params_size);

                basm.masm().Bind(&mut skip_interrupt_label);
            }

            let mut temps = ScratchRegisterScope::new(&mut basm);
            let actual_params_size = temps.AcquireScratch();
            // Compute the size of the actual parameters + receiver.
            basm.move_reg(actual_params_size, MemOperand::new(Self::fp, StandardFrameConstants::kArgCOffset));

            // If actual is bigger than formal, then we should use it to free up the stack
            // arguments.
            basm.masm().Cmp(params_size, actual