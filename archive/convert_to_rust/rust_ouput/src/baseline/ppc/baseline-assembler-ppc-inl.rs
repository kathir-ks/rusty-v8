// Converted from V8 C++ source files:
// Header: baseline-assembler-ppc-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::sync::Arc;

use crate::baseline::baseline_assembler::{BaselineAssembler, CompressionMode, FeedbackSlot, Label, ScratchRegisterScope};
use crate::codegen::interface_descriptors::Register;
use crate::objects::literal_objects_inl::Tagged;

pub mod interpreter {
    #[derive(Clone, Copy)]
    pub struct Register {
        num: i32,
    }
    impl Register {
        pub fn ToOperand(&self) -> i32 {
            self.num
        }
    }

    pub struct RegisterList {
        registers: Vec<Register>,
    }
    impl RegisterList {
        pub fn register_count(&self) -> usize {
            self.registers.len()
        }
        pub fn get(&self, index: usize) -> Register {
            self.registers[index]
        }
    }
}

pub mod codegen {
    pub mod ppc {
        pub mod assembler_ppc_inl {
            use crate::baseline::baseline_assembler::Label;
            use crate::codegen::interface_descriptors::Register;
            use crate::baseline::ppc::baseline_assembler_ppc_inl::Condition;

            pub struct MacroAssembler {}

            impl MacroAssembler {
                pub fn b(&mut self, target: &mut Label) {}
                pub fn bind(&mut self, label: &mut Label) {}
                pub fn Push(&mut self, reg: Register) {}
                pub fn Pop(&mut self, reg: Register) {}
                pub fn LeaveFrame(&mut self, frame: StackFrame) {}
                pub fn DropArguments(&mut self, params_size: Register) {}
                pub fn Ret(&mut self) {}
                pub fn mov(&mut self, output: Register, operand: i32) {}
                pub fn mr(&mut self, output: Register, source: Register) {}
                pub fn LoadTaggedField(&mut self, output: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn StoreTaggedField(&mut self, value: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn CmpS64(&mut self, lhs: Register, rhs: Register) {}
                pub fn CmpU64(&mut self, lhs: Register, rhs: Register) {}
                pub fn CmpS32(&mut self, lhs: Register, rhs: Register) {}
                pub fn CmpU32(&mut self, lhs: Register, rhs: Register) {}
                pub fn AddS64(&mut self, dest: Register, src: Register, operand: crate::baseline::ppc::baseline_assembler_ppc_inl::Operand) {}
                pub fn AddS32(&mut self, dest: Register, src: Register, operand: crate::baseline::ppc::baseline_assembler_ppc_inl::Operand, r0: Register, setrc: SetRC) {}
                pub fn AndU64(&mut self, dest: Register, src: Register, operand: crate::baseline::ppc::baseline_assembler_ppc_inl::Operand, ip: Register, setrc: SetRC) {}
                pub fn LoadU64(&mut self, dest: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn StoreU64(&mut self, src: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn LoadS32(&mut self, dest: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn StoreU32(&mut self, src: Register, operand: FieldMemOperand, r0: Register) {}
                pub fn ShiftLeftU32(&mut self, dest: Register, src: Register, operand: crate::baseline::ppc::baseline_assembler_ppc_inl::Operand) {}
                pub fn CmpU32(&mut self, reg: Register, operand: crate::baseline::ppc::baseline_assembler_ppc_inl::Operand, r0: Register) {}
                pub fn SmiTag(&mut self, reg: Register) {}
                pub fn SmiUntag(&mut self, reg: Register) {}
                pub fn LoadU16(&mut self, type_: Register, field_mem_operand: FieldMemOperand, r0: Register) {}
                pub fn LoadU8(&mut self, output: Register, field_mem_operand: FieldMemOperand, r0: Register) {}
                pub fn LoadCodePointerField(&mut self, scratch_and_result: Register, field_mem_operand: FieldMemOperand, r0: Register) {}
                pub fn CmpU32(&mut self, reg: Register, kInterpreterAccumulatorRegister: Register) {}
                pub fn CmpU64(&mut self, reg: Register, kInterpreterAccumulatorRegister: Register) {}
            }

            #[derive(PartialEq, Eq)]
            pub enum StackFrame {
                BASELINE,
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
            
            pub enum SetRC {
                SetRC,
                LeaveRC
            }

            pub enum SaveFPRegsMode {
                kIgnore
            }
        }
        pub mod register_ppc {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct Register(pub i16);
        }
    }
}

pub mod objects {
    pub mod literal_objects_inl {
        pub struct Tagged<T> {
            ptr: *mut T,
        }
        impl<T> Tagged<T> {
            pub fn ptr(&self) -> *mut T {
                self.ptr
            }
        }
    }
}

pub mod baseline {
    pub mod baseline_assembler {
        use crate::codegen::interface_descriptors::Register;
        use crate::objects::literal_objects_inl::Tagged;

        pub struct BaselineAssembler {}
        impl BaselineAssembler {
            pub fn masm(&mut self) -> &mut crate::codegen::ppc::assembler_ppc_inl::MacroAssembler {
                todo!()
            }
        }

        pub struct Label {}
        impl Label {
            pub fn new() -> Self {
                Label {}
            }
        }

        pub struct ScratchRegisterScope {}

        pub enum CompressionMode {}
        
        #[derive(Clone, Copy)]
        pub struct FeedbackSlot {
            index: i32,
        }

        impl FeedbackSlot {
            pub fn ToInt(&self) -> i32 {
                self.index
            }
        }

        impl ScratchRegisterScope {
            pub fn AcquireScratch(&self) -> Register {
                Register(0) // Replace with actual register allocation logic
            }
        }

        pub struct ExternalReference {}
    }
    pub mod ppc {
        pub mod baseline_assembler_ppc_inl {
            use crate::baseline::baseline_assembler::{BaselineAssembler, Label, ScratchRegisterScope};
            use crate::codegen::ppc::assembler_ppc_inl::{MacroAssembler, FieldMemOperand, StackFrame, SetRC, SaveFPRegsMode};
            use crate::codegen::interface_descriptors::Register;
            use crate::objects::literal_objects_inl::Tagged;

            const r9: Register = Register(9);
            const r10: Register = Register(10);
            const ip: Register = Register(11);
            const fp: Register = Register(30);
            const r0: Register = Register(0);
            const kInterpreterAccumulatorRegister: Register = Register(12);
            const kJSFunctionRegister: Register = Register(13);
            const kContextRegister: Register = Register(14);

            const kScratchRegisters: [Register; 3] = [r9, r10, ip];
            const kNumScratchRegisters: usize = kScratchRegisters.len();

            fn is_signed(cc: Condition) -> bool {
                match cc {
                    Condition::eq | Condition::ne | Condition::lt | Condition::gt | Condition::le | Condition::ge => true,
                    Condition::kUnsignedLessThan | Condition::kUnsignedGreaterThan | Condition::kUnsignedLessThanEqual | Condition::kUnsignedGreaterThanEqual => false,
                }
            }

            fn to_condition(cc: Condition) -> Condition {
                cc
            }

            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum Condition {
                eq,
                ne,
                lt,
                gt,
                le,
                ge,
                kUnsignedLessThan,
                kUnsignedGreaterThan,
                kUnsignedLessThanEqual,
                kUnsignedGreaterThanEqual,
            }

            #[derive(Clone, Copy)]
            pub struct Operand(i32);

            impl Operand {
                pub fn new(value: i32) -> Self {
                    Operand(value)
                }
            }

            pub mod detail {
                use crate::codegen::interface_descriptors::Register;
                use crate::baseline::ppc::baseline_assembler_ppc_inl::{r9, r10, ip};

                pub const kScratchRegisters: [Register; 3] = [r9, r10, ip];
                pub const kNumScratchRegisters: usize = kScratchRegisters.len();
            }

            impl BaselineAssembler {
                pub fn RegisterFrameOperand(
                    &mut self,
                    interpreter_register: crate::interpreter::Register,
                ) -> FieldMemOperand {
                    FieldMemOperand::new(fp, interpreter_register.ToOperand() * kSystemPointerSize)
                }

                pub fn RegisterFrameAddress(
                    &mut self,
                    interpreter_register: crate::interpreter::Register,
                    rscratch: Register,
                ) {
                    let mut masm = self.masm();
                    masm.AddS64(
                        rscratch,
                        fp,
                        Operand::new(interpreter_register.ToOperand() * kSystemPointerSize),
                    );
                }

                pub fn FeedbackVectorOperand(&mut self) -> FieldMemOperand {
                    FieldMemOperand::new(fp, BaselineFrameConstants::kFeedbackVectorFromFp)
                }

                pub fn FeedbackCellOperand(&mut self) -> FieldMemOperand {
                    FieldMemOperand::new(fp, BaselineFrameConstants::kFeedbackCellFromFp)
                }

                pub fn Bind(&mut self, label: &mut Label) {
                    let mut masm = self.masm();
                    masm.bind(label);
                }

                pub fn JumpTarget(&mut self) {}

                pub fn Jump(&mut self, target: &mut Label, _distance: Label::Distance) {
                    let mut masm = self.masm();
                    masm.b(target);
                }

                pub fn JumpIfRoot(
                    &mut self,
                    _value: Register,
                    _index: RootIndex,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    masm.b(target);
                }

                pub fn JumpIfNotRoot(
                    &mut self,
                    _value: Register,
                    _index: RootIndex,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    masm.b(target);
                }

                pub fn JumpIfSmi(&mut self, _value: Register, target: &mut Label, _distance: Label::Distance) {
                    let mut masm = self.masm();
                    masm.b(target);
                }

                pub fn JumpIfImmediate(
                    &mut self,
                    cc: Condition,
                    left: Register,
                    right: i32,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    self.JumpIf(cc, left, Operand::new(right), target, Label::Distance::Near);
                }

                pub fn JumpIfNotSmi(&mut self, _value: Register, target: &mut Label, _distance: Label::Distance) {
                    let mut masm = self.masm();
                    masm.b(target);
                }

                pub fn TestAndBranch(
                    &mut self,
                    value: Register,
                    mask: i32,
                    cc: Condition,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    masm.AndU64(r0, value, Operand::new(mask), ip, SetRC::SetRC);
                    masm.b(to_condition(cc), target);
                }

                pub fn JumpIf(
                    &mut self,
                    cc: Condition,
                    lhs: Register,
                    rhs: Operand,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    if is_signed(cc) {
                        masm.CmpS64(lhs, r0);
                    } else {
                        masm.CmpU64(lhs, r0);
                    }
                    masm.b(to_condition(cc), target);
                }

                pub fn JumpIfObjectTypeFast(
                    &mut self,
                    cc: Condition,
                    object: Register,
                    instance_type: InstanceType,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut temps = ScratchRegisterScope {};
                    let scratch = temps.AcquireScratch();
                    if cc == Condition::eq || cc == Condition::ne {
                        let scratch2 = temps.AcquireScratch();
                        //__ IsObjectType(object, scratch, scratch2, instance_type);
                        let mut masm = self.masm();
                        masm.b(to_condition(cc), target);
                        return;
                    }
                    self.JumpIfObjectType(
                        cc,
                        object,
                        instance_type,
                        scratch,
                        target,
                        Label::Distance::Near,
                    );
                }

                pub fn JumpIfObjectType(
                    &mut self,
                    cc: Condition,
                    object: Register,
                    instance_type: InstanceType,
                    map: Register,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut temps = ScratchRegisterScope {};
                    let type_ = temps.AcquireScratch();
                    let mut masm = self.masm();
                    //__ LoadMap(map, object);
                    masm.LoadU16(type_, FieldMemOperand::new(map, Map::kInstanceTypeOffset), r0);
                    self.JumpIf(cc, type_, Operand::new(instance_type as i32), target, Label::Distance::Near);
                }

                pub fn JumpIfInstanceType(
                    &mut self,
                    map: Register,
                    instance_type: InstanceType,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut temps = ScratchRegisterScope {};
                    let type_ = temps.AcquireScratch();
                    //__ AssertNotSmi(map);
                    //__ CompareObjectType(map, type_, type_, MAP_TYPE);
                    //__ Assert(eq, AbortReason::kUnexpectedValue);
                    let mut masm = self.masm();
                    masm.LoadU16(type_, FieldMemOperand::new(map, Map::kInstanceTypeOffset), r0);
                    self.JumpIf(cc, type_, Operand::new(instance_type as i32), target, Label::Distance::Near);
                }

                pub fn JumpIfPointer(
                    &mut self,
                    cc: Condition,
                    value: Register,
                    operand: FieldMemOperand,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut temps = ScratchRegisterScope {};
                    let tmp = temps.AcquireScratch();
                    let mut masm = self.masm();
                    masm.LoadU64(tmp, operand, r0);
                    Self::JumpIfHelper(masm, cc, value, tmp, target);
                }

                pub fn JumpIfSmi1(
                    &mut self,
                    cc: Condition,
                    value: Register,
                    smi: Tagged<Smi>,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    //__ AssertSmi(value);
                    //__ LoadSmiLiteral(r0, smi);
                    Self::JumpIfHelper(masm, cc, value, r0, target);
                }

                pub fn JumpIfSmi2(
                    &mut self,
                    cc: Condition,
                    lhs: Register,
                    rhs: Register,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    //__ AssertSmi(lhs);
                    //__ AssertSmi(rhs);
                    Self::JumpIfHelper(masm, cc, lhs, rhs, target);
                }

                pub fn JumpIfTagged(
                    &mut self,
                    cc: Condition,
                    value: Register,
                    operand: FieldMemOperand,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    masm.LoadTaggedField(ip, operand, r0);
                    Self::JumpIfHelper::<64>(masm, cc, value, ip, target);
                }

                pub fn JumpIfTagged1(
                    &mut self,
                    cc: Condition,
                    operand: FieldMemOperand,
                    value: Register,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut masm = self.masm();
                    masm.LoadTaggedField(ip, operand, r0);
                    Self::JumpIfHelper::<64>(masm, cc, value, ip, target);
                }

                pub fn JumpIfByte(
                    &mut self,
                    cc: Condition,
                    value: Register,
                    byte: i32,
                    target: &mut Label,
                    _distance: Label::Distance,
                ) {
                    self.JumpIf(cc, value, Operand::new(byte), target, Label::Distance::Near);
                }

                pub fn Move(&mut self, output: crate::interpreter::Register, source: Register) {
                    self.Move1(self.RegisterFrameOperand(output), source);
                }

                pub fn Move2(&mut self, output: Register, value: Tagged<TaggedIndex>) {
                    let mut masm = self.masm();
                    masm.mov(output, value.ptr() as i32);
                }

                pub fn Move1(&mut self, output: FieldMemOperand, source: Register) {
                    let mut masm = self.masm();
                    masm.StoreU64(source, output, r0);
                }

                pub fn Move3(&mut self, output: Register, reference: ExternalReference) {
                   // let mut masm = self.masm();
                   // masm.Move(output, reference);
                }

                pub fn Move4(&mut self, output: Register, value: Handle<HeapObject>) {
                    //let mut masm = self.masm();
                    //masm.Move(output, value);
                }

                pub fn Move5(&mut self, output: Register, value: i32) {
                    let mut masm = self.masm();
                    masm.mov(output, value);
                }

                pub fn MoveMaybeSmi(&mut self, output: Register, source: Register) {
                    let mut masm = self.masm();
                    masm.mr(output, source);
                }

                pub fn MoveSmi(&mut self, output: Register, source: Register) {
                    let mut masm = self.masm();
                    masm.mr(output, source);
                }

                pub fn Push<T>(&mut self, val: T) -> i32
                {
                    detail::PushAllHelper::<T>::Push(self, val)
                }

                pub fn PushReverse<T>(&mut self, val: T) {
                    detail::PushAllHelper::<T>::PushReverse(self, val)
                }

                pub fn Pop<T>(&mut self, registers: T) {
                     detail::PopAllHelper::<T>::Pop(self, registers);
                }

                pub fn LoadTaggedField(&mut self, output: Register, source: Register, offset: i32) {
                    let mut masm = self.masm();
                    masm.LoadTaggedField(output, FieldMemOperand::new(source, offset), r0);
                }

                pub fn LoadTaggedSignedField(&mut self, output: Register, source: Register, offset: i32) {
                    let mut masm = self.masm();
                    masm.LoadTaggedField(output, FieldMemOperand::new(source, offset), r0);
                }

                pub fn LoadTaggedSignedFieldAndUntag(&mut self, output: Register, source: Register, offset: i32) {
                    self.LoadTaggedSignedField(output, source, offset);
                    self.SmiUntag(output);
                }

                pub fn LoadWord16FieldZeroExtend(&mut self, output: Register, source: Register, offset: i32) {
                    let mut masm = self.masm();
                    masm.LoadU16(output, FieldMemOperand::new(source, offset), r0);
                }

                pub fn LoadWord8Field(&mut self, output: Register, source: Register, offset: i32) {
                    let mut masm = self.masm();
                    masm.LoadU8(output, FieldMemOperand::new(source, offset), r0);
                }

                pub fn StoreTaggedSignedField(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
                    let mut temps = ScratchRegisterScope {};
                    let tmp = temps.AcquireScratch();
                    //__ LoadSmiLiteral(tmp, value);
                    let mut masm = self.masm();
                    masm.StoreTaggedField(tmp, FieldMemOperand::new(target, offset), r0);
                }

                pub fn StoreTaggedFieldWithWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
                    //let scratch = WriteBarrierDescriptor::SlotAddressRegister();
                    //DCHECK(!AreAliased(target, value, scratch));
                    let mut masm = self.masm();
                    masm.StoreTaggedField(value, FieldMemOperand::new(target, offset), r0);
                    //__ RecordWriteField(target, offset, value, scratch, kLRHasNotBeenSaved, SaveFPRegsMode::kIgnore);
                }

                pub fn StoreTaggedFieldNoWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
                    let mut masm = self.masm();
                    masm.StoreTaggedField(value, FieldMemOperand::new(target, offset), r0);
                }

                pub fn TryLoadOptimizedOsrCode(
                    &mut self,
                    scratch_and_result: Register,
                    feedback_vector: Register,
                    slot: crate::baseline::baseline_assembler::FeedbackSlot,
                    on_result: &mut Label,
                    _distance: Label::Distance,
                ) {
                    let mut fallthrough = Label::new();
                    self.LoadTaggedField(
                        scratch_and_result,
                        feedback_vector,
                        FeedbackVector::OffsetOfElementAt(slot.ToInt()),
                    );
                    //__ LoadWeakValue(scratch_and_result, scratch_and_result, &fallthrough);

                    // Is it marked_for_deoptimization? If yes, clear the slot.
                    {
                        let mut temps = ScratchRegisterScope {};

                        // The entry references a CodeWrapper object. Unwrap it now.
                        let mut masm = self.masm();
                        masm.LoadCodePointerField(
                            scratch_and_result,
                            FieldMemOperand::new(scratch_and_result, CodeWrapper::kCodeOffset), r0
                        );

                        let scratch = temps.AcquireScratch();
                        //__ TestCodeIsMarkedForDeoptimization(scratch_and_result, scratch, r0);
                        masm.b(on_result,);
                        //__ mov(scratch, __ ClearedValue());
                        self.StoreTaggedFieldNoWriteBarrier(
                            feedback_vector,
                            FeedbackVector::OffsetOfElementAt(slot.ToInt()),
                            scratch,
                        );
                    }

                    self.Bind(&mut fallthrough);
                    self.Move5(scratch_and_result, 0);
                }

                pub fn AddToInterruptBudgetAndJumpIfNotExceeded(
                    &mut self,
                    weight: i32,
                    skip_interrupt_label: &mut Label,
                ) {
                    let mut scratch_scope = ScratchRegisterScope {};
                    let feedback_cell = scratch_scope.AcquireScratch();
                    self.LoadFeedbackCell(feedback_cell);

                    let interrupt_budget = scratch_scope.AcquireScratch();
                    let mut masm = self.masm();
                    masm.LoadU32(
                        interrupt_budget,
                        FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset), r0
                    );
                    // Remember to set flags as part of the add!
                    masm.AddS32(interrupt_budget, interrupt_budget, Operand::new(weight), r0, SetRC::SetRC);
                    masm.StoreU32(
                        interrupt_budget,
                        FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset), r0
                    );
                    if let Some(_skip_interrupt_label) = Some(skip_interrupt_label) {
                        // Use compare flags set by add
                        assert!(weight < 0);
                        masm.bge(skip_interrupt_label,);
                    }
                }

                pub fn AddToInterruptBudgetAndJumpIfNotExceeded1(
                    &mut self,
                    weight: Register,
                    skip_interrupt_label: &mut Label,
                ) {
                    let mut scratch_scope = ScratchRegisterScope {};
                    let feedback_cell = scratch_scope.AcquireScratch();
                    self.LoadFeedbackCell(feedback_cell);

                    let interrupt_budget = scratch_scope.AcquireScratch();
                    let mut masm = self.masm();
                    masm.LoadU32(
                        interrupt_budget,
                        FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset), r0
                    );
                    // Remember to set flags as part of the add!
                    masm.AddS32(interrupt_budget, interrupt_budget, Operand::new(0), r0, SetRC::SetRC);
                    masm.StoreU32(
                        interrupt_budget,
                        FieldMemOperand::new(feedback_cell, FeedbackCell::kInterruptBudgetOffset), r0
                    );
                    if let Some(_skip_interrupt_label) = Some(skip_interrupt_label) {
                        masm.bge(skip_interrupt_label,);
                    }
                }

                pub fn LdaContextSlot(
                    &mut self,
                    context: Register,
                    index: u32,
                    depth: u32,
                    _compression_mode: crate::baseline::baseline_assembler::CompressionMode,
                ) {
                    let mut masm = self.masm();
                    for _i in 0..depth {
                        self.LoadTaggedField(context, context, Context::kPreviousOffset);
                    }
                    self.LoadTaggedField(kInterpreterAccumulatorRegister, context, Context::OffsetOfElementAt(index));
                }

                pub fn StaContextSlot(
                    &mut self,
                    context: Register,
                    value: Register,
                    index: u32,
                    depth: u32,
                ) {
                    for _i in 0..depth {
                        self.LoadTaggedField(context, context, Context::kPreviousOffset);
                    }
                    self.StoreTaggedFieldWithWriteBarrier(context, Context::OffsetOfElementAt(index), value);
                }

                pub fn LdaModuleVariable(
                    &mut self,
                    context: Register,
                    cell_index: i32,
                    depth: u32,
                ) {
                    for _i in 0..depth {
                        self.LoadTaggedField(context, context, Context::kPreviousOffset);
                    }
                    self.LoadTaggedField(context, context, Context::kExtensionOffset);
                    if cell_index > 0 {
                        self.LoadTaggedField(context, context, SourceTextModule::kRegularExportsOffset);
                        // The actual array index is (cell_index - 1).
                        //cell_index -= 1;
                    } else {
                        self.LoadTaggedField(context, context, SourceTextModule::kRegularImportsOffset);
                        // The actual array index is (-cell_index - 1).
                        //cell_index = -cell_index - 1;
                    }
                   // self.LoadFixedArrayElement(context, context, cell_index);
                    self.LoadTaggedField(kInterpreterAccumulatorRegister, context, Cell::kValueOffset);
                }

                pub fn StaModuleVariable(
                    &mut self,
                    context: Register,
                    value: Register,
                    cell_index: i32,
                    depth: u32,
                ) {
                    for _i in 0..depth {
                        self.LoadTaggedField(context, context, Context::kPreviousOffset);
                    }
                    self.LoadTaggedField(context, context, Context::kExtensionOffset);
                    self.LoadTaggedField(context, context, SourceTextModule::kRegularExportsOffset);

                    // The actual array index is (cell_index - 1).
                    //cell_index -= 1;
                   // self.LoadFixedArrayElement(context, context, cell_index);
                    self.StoreTaggedFieldWithWriteBarrier(context, Cell::kValueOffset, value);
                }

                pub fn IncrementSmi(&mut self, lhs: FieldMemOperand) {
                   // Register scratch = ip;
                    if SmiValuesAre31Bits() {
                        let mut masm = self.masm();
                        masm.LoadS32(ip, lhs, r0);
                        masm.AddS64(ip, ip, Operand::new(Smi::FromInt(1)));
                        masm.StoreU32(ip, lhs, r0);
                    } else {
                        let mut masm = self.masm();
                        //masm.SmiUntag(scratch, lhs, LeaveRC, r0);
                        masm.AddS64(ip, ip, Operand::new(1));
                        masm.SmiTag(ip);
                        masm.StoreU64(ip, lhs, r0);
                    }
                }

                pub fn Switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: i32) {
                    let mut fallthrough = Label::new();
                    let mut jump_table = Label::new();
                    if case_value_base != 0 {
                        let mut masm = self.masm();
                        masm.AddS64(reg, reg, Operand::new(-case_value_base));
                    }

                    // Mostly copied from code-generator-arm.cc
                    self.JumpIf(Condition::kUnsignedGreaterThanEqual, reg, Operand::new(num_labels), &mut fallthrough, Label::Distance::Near);
                    // Ensure to emit the constant pool first if necessary.
                    let entry_size_log2 = 3;
                    let mut masm = self.masm();
                    masm.ShiftLeftU32(reg, reg, Operand::new(entry_size_log2));
                   // masm.mov_label_addr(ip, &jump_table);
                    masm.AddS64(reg, reg, Operand::new(0));
                   // masm.Jump(reg);
                    masm.b(&mut fallthrough);
                    masm.bind(&mut jump_table);
                    //Assembler::BlockTrampolinePoolScope block_trampoline_pool(masm_);
                    for i in 0..num_labels {
                         masm.b(labels[i as usize]);
                       // masm.nop();
                    }
                    masm.bind(&mut fallthrough);
                }

                pub fn Word32And(&mut self, output: Register, lhs: Register, rhs: i32) {
                    let mut masm = self.masm();
                   // masm.AndU32(output, lhs, Operand::new(rhs));
                }

                fn EmitReturn(masm: &mut MacroAssembler) {
                   // ASM_CODE_COMMENT(masm);
                    let mut basm = BaselineAssembler {};

                   // Register weight = BaselineLeaveFrameDescriptor::WeightRegister();
                   // Register params_size = BaselineLeaveFrameDescriptor::ParamsSizeRegister();
                    let params_size: Register = Register(1);
                    let weight: Register = Register(2);

                    {
                      //  ASM_CODE_COMMENT_STRING(masm, "Update Interrupt Budget");

                        let mut skip_interrupt_label = Label::new();
                       // basm.AddToInterruptBudgetAndJumpIfNotExceeded(weight, &mut skip_interrupt_label);
                        {
                            masm.SmiTag(params_size);
                            basm.Push(params_size);
                            basm.Push(kInterpreterAccumulatorRegister);

                           // masm.LoadContext(kContextRegister);
                           // masm.LoadFunction(kJSFunctionRegister);
                            basm.Push(kJSFunctionRegister);
                           // masm.CallRuntime(Runtime::kBytecodeBudgetInterrupt_Sparkplug, 1);

                            basm.Pop(kInterpreterAccumulatorRegister);
                            basm.Pop(params_size);
                            masm.SmiUntag(params_size);
                        }

                        basm.Bind(&mut skip_interrupt_label);
                    }

                    let mut temps = ScratchRegisterScope {};
                    let actual_params_size = temps.AcquireScratch();
                    // Compute the size of the actual parameters + receiver.
                    //basm.Move(actual_params_size, MemOperand(fp, StandardFrameConstants::kArgCOffset));

                    // If actual is bigger than formal, then we should use it to free up the stack
                    // arguments.
                    let mut corrected
