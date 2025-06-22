// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/baseline/x64/baseline-assembler-x64-inl.h

mod baseline_assembler_x64 {
    use crate::base::macros::*;
    use crate::baseline::baseline_assembler::*;
    use crate::codegen::x64::register_x64::*;
    use crate::objects::feedback_vector::*;
    use crate::objects::literal_objects::*;
    // use crate::assembler::*; // Assuming MacroAssembler is in assembler module

    //use v8::internal::wasm::macro_assembler::MacroAssembler;
    use std::ops::Deref;

    // Define dummy versions of types and functions from other modules.
    // Ensure they compile and have the expected behaviour.
    pub struct MacroAssembler {}

    impl MacroAssembler {
        pub fn PushRoot(&mut self, _source: RootIndex) {}
        pub fn Push(&mut self, _reg: Register) {}
        pub fn PushTagged(&mut self, _value: TaggedIndex) {}
        pub fn PushSmi(&mut self, _value: Smi) {}
        pub fn PushHeapObject(&mut self, _object: Handle<HeapObject>) {}
        pub fn PushImmediate(&mut self, _immediate: i32) {}
        pub fn PushMemOperand(&mut self, _operand: MemOperand) {}

        pub fn Pop(&mut self, _registers: ()) {}

        pub fn Move(&mut self, _output: Register, _source: Register) {}
        pub fn MoveTagged(&mut self, _output: Register, _source: TaggedIndex) {}
        pub fn MoveHandle(&mut self, _output: Register, _handle: Handle<HeapObject>) {}
        pub fn MoveExternalReference(&mut self, _output: Register, _reference: ExternalReference) {}
        pub fn MoveInt32(&mut self, _output: Register, _value: i32) {}

        pub fn LoadTaggedField(&mut self, _output: Register, _operand: FieldOperand) {}
        pub fn LoadTaggedSignedField(&mut self, _output: Register, _operand: FieldOperand) {}

        pub fn SmiUntagField(&mut self, _output: Register, _operand: FieldOperand) {}

        pub fn StoreTaggedSignedField(&mut self, _operand: FieldOperand, _value: Tagged<Smi>) {}
        pub fn StoreTaggedField(&mut self, _operand: FieldOperand, _value: Register) {}

        pub fn SmiTag(&mut self, _reg: Register) {}
        pub fn SmiUntagUnsigned(&mut self, _reg: Register) {}

        pub fn LeaveFrame(&mut self, _frame_type: StackFrame) {}
        pub fn DropArguments(&mut self, _params_size: Register, _scratch: Register) {}
        pub fn Ret(&mut self) {}

        pub fn TryLoadOptimizedOsrCode(
            &mut self,
            _scratch_and_result: Register,
            _code_kind: CodeKind,
            _feedback_vector: Register,
            _slot: FeedbackSlot,
            _on_result: &mut Label,
            _distance: Label::Distance,
        ) {}

        pub fn addl(&mut self, _op: FieldOperand, _im: Immediate) {}

        pub fn Switch(&mut self, _scratch: Register, _reg: Register, _case_value_base: i32, _labels: &mut [&mut Label], _num_labels: usize) {}
        pub fn Assert(&mut self, _condition: Condition, _reason: AbortReason) {}
    }
    pub enum StackFrame {
        BASELINE
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Condition {
        Equal,
        NotEqual,
        LessThan,
        GreaterThanOrEqual
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AbortReason {
        kUnexpectedValue
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CodeKind {
        MAGLEV
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Immediate(pub i32);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FieldOperand(pub Register, pub i32);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MemOperand(pub Register, pub i32);

    impl MemOperand {
        fn AddressUsesRegister(&self, target: Register) -> bool {
            self.0 == target
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Smi(pub i64);

    impl Smi {
        pub fn FromInt(i: i32) -> Self {
            Smi(i as i64)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Handle<T>(pub *mut T); // Replace *mut T with appropriate smart pointer if needed

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct HeapObject {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct TaggedIndex(pub i64);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ExternalReference {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FeedbackSlot {}

    // Placeholder constants, replace with actual values
    pub const MAP_TYPE: InstanceType = InstanceType::MAP_TYPE;
    pub const OFFSET_OF_DATA_START_FIXED_ARRAY: usize = 8;
    pub const kTaggedSize: usize = 8;
    pub const kSystemPointerSize: i32 = 8;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum InstanceType {
        MAP_TYPE,
        // ... other instance types
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RootIndex {
        // ... root indices
    }

    pub mod interpreter {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Register(pub i32);

        impl Register {
            pub fn ToOperand(self) -> i32 {
                self.0
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct RegisterList {
            registers: Vec<Register>,
        }

        impl RegisterList {
            pub fn new(registers: Vec<Register>) -> Self {
                RegisterList { registers }
            }

            pub fn register_count(&self) -> usize {
                self.registers.len()
            }
        }

        impl std::ops::Index<usize> for RegisterList {
            type Output = Register;

            fn index(&self, index: usize) -> &Self::Output {
                &self.registers[index]
            }
        }
    }

    pub mod BaselineFrameConstants {
        pub const kFeedbackVectorFromFp: i32 = 0; // Replace with actual offset
        pub const kFeedbackCellFromFp: i32 = 8; // Replace with actual offset
    }
    pub mod StandardFrameConstants {
        pub const kArgCOffset: i32 = 16;
    }
    pub mod InterpreterFrameConstants {
        pub const kFunctionOffset: i32 = 24;
    }

    pub mod Context {
        pub const kPreviousOffset: i32 = 0; // Replace with actual offset
        pub const kExtensionOffset: i32 = 8; // Replace with actual offset
        pub fn OffsetOfElementAt(index: u32) -> i32 {
            index as i32 * 8 // Replace with actual calculation
        }
    }

    pub mod SourceTextModule {
        pub const kRegularExportsOffset: i32 = 0; // Replace with actual offset
        pub const kRegularImportsOffset: i32 = 8; // Replace with actual offset
    }
    pub mod Cell {
        pub const kValueOffset: i32 = 0; // Replace with actual offset
    }
    pub mod FeedbackCell {
        pub const kInterruptBudgetOffset: i32 = 0; // Replace with actual offset
    }
    pub mod WriteBarrierDescriptor {
        pub fn SlotAddressRegister() -> Register {
            r13 // Replace with actual register
        }
    }
    pub const kPtrComprCageBaseRegister: Register = r14; // Replace with actual register
    pub const COMPRESS_POINTERS_BOOL: bool = false;

    pub mod runtime {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Runtime {
            kBytecodeBudgetInterrupt_Sparkplug
        }
    }

    pub struct BaselineAssembler<'a> {
        masm_: &'a mut MacroAssembler,
        scratch_register_scope_: Option<Box<ScratchRegisterScope<'a>>>,
    }

    impl<'a> BaselineAssembler<'a> {
        pub fn new(masm: &'a mut MacroAssembler) -> Self {
            BaselineAssembler {
                masm_: masm,
                scratch_register_scope_: None,
            }
        }

        pub fn masm(&mut self) -> &mut MacroAssembler {
            self.masm_
        }

        pub fn RegisterFrameOperand(
            &self,
            interpreter_register: interpreter::Register,
        ) -> MemOperand {
            MemOperand(rbp, interpreter_register.ToOperand() * kSystemPointerSize)
        }

        pub fn RegisterFrameAddress(
            &mut self,
            interpreter_register: interpreter::Register,
            rscratch: Register,
        ) {
            self.masm_
                .Move(&mut rscratch.clone(), TaggedIndex((interpreter_register.ToOperand() * kSystemPointerSize).into()));// leaq(rscratch, MemOperand(rbp, interpreter_register.ToOperand() * kSystemPointerSize));
        }

        pub fn FeedbackVectorOperand(&self) -> MemOperand {
            MemOperand(rbp, BaselineFrameConstants::kFeedbackVectorFromFp)
        }

        pub fn FeedbackCellOperand(&self) -> MemOperand {
            MemOperand(rbp, BaselineFrameConstants::kFeedbackCellFromFp)
        }

        pub fn Bind(&mut self, _label: &mut Label) {
            // self.masm_.bind(label);
        }

        pub fn JumpTarget(&mut self) {
            // self.masm_.endbr64();
        }

        pub fn Jump(&mut self, _target: &mut Label, _distance: Label::Distance) {
            // self.masm_.jmp(target, distance);
        }

        pub fn JumpIfRoot(
            &mut self,
            _value: Register,
            _index: RootIndex,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.JumpIfRoot(value, index, target, distance);
        }

        pub fn JumpIfNotRoot(
            &mut self,
            _value: Register,
            _index: RootIndex,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.JumpIfNotRoot(value, index, target, distance);
        }

        pub fn JumpIfSmi(&mut self, _value: Register, _target: &mut Label, _distance: Label::Distance) {
            // self.masm_.JumpIfSmi(value, target, distance);
        }

        pub fn JumpIfNotSmi(&mut self, _value: Register, _target: &mut Label, _distance: Label::Distance) {
            // self.masm_.JumpIfNotSmi(value, target, distance);
        }

        pub fn TestAndBranch(
            &mut self,
            value: Register,
            mask: i32,
            cc: Condition,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            if (mask & 0xff) == mask {
                // self.masm_.testb(value, Immediate(mask));
            } else {
                // self.masm_.testl(value, Immediate(mask));
            }
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIf(
            &mut self,
            _cc: Condition,
            _lhs: Register,
            _rhs: &Operand,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmpq(lhs, rhs);
            // self.masm_.j(cc, target, distance);
        }

        #[cfg(V8_STATIC_ROOTS_BOOL)]
        pub fn JumpIfJSAnyIsPrimitive(
            &mut self,
            heap_object: Register,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            // self.masm_.AssertNotSmi(heap_object);
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.AcquireScratch();
            // self.masm_.JumpIfJSAnyIsPrimitive(heap_object, scratch, target, distance);
        }

        pub fn JumpIfObjectTypeFast(
            &mut self,
            cc: Condition,
            object: Register,
            instance_type: InstanceType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            // self.masm_.AssertNotSmi(object);
            let mut temps = ScratchRegisterScope::new(self);
            let scratch = temps.AcquireScratch();
            if cc == Condition::Equal || cc == Condition::NotEqual {
                // self.masm_.IsObjectType(object, instance_type, scratch);
            } else {
                // self.masm_.CmpObjectType(object, instance_type, scratch);
            }
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfObjectType(
            &mut self,
            _cc: Condition,
            _object: Register,
            _instance_type: InstanceType,
            _map: Register,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.AssertNotSmi(object);
            // self.masm_.CmpObjectType(object, instance_type, map);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfInstanceType(
            &mut self,
            _cc: Condition,
            _map: Register,
            _instance_type: InstanceType,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // if v8_flags.debug_code {
            //     self.masm_.AssertNotSmi(map);
            //     self.masm_.CmpObjectType(map, MAP_TYPE, kScratchRegister);
            //     self.masm_.Assert(equal, AbortReason::kUnexpectedValue);
            // }
            // self.masm_.CmpInstanceType(map, instance_type);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfPointer(
            &mut self,
            _cc: Condition,
            _value: Register,
            _operand: MemOperand,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmpq(value, operand);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfSmiSmi(
            &mut self,
            _cc: Condition,
            _lhs: Register,
            _smi: Tagged<Smi>,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.SmiCompare(lhs, smi);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfSmiReg(
            &mut self,
            _cc: Condition,
            _lhs: Register,
            _rhs: Register,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.SmiCompare(lhs, rhs);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfImmediate(
            &mut self,
            _cc: Condition,
            _left: Register,
            _right: i32,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmpq(left, Immediate(right));
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfTagged(
            &mut self,
            _cc: Condition,
            _value: Register,
            _operand: MemOperand,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmp_tagged(value, operand);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfTaggedMem(
            &mut self,
            _cc: Condition,
            _operand: MemOperand,
            _value: Register,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmp_tagged(operand, value);
            // self.masm_.j(cc, target, distance);
        }

        pub fn JumpIfByte(
            &mut self,
            _cc: Condition,
            _value: Register,
            _byte: i32,
            _target: &mut Label,
            _distance: Label::Distance,
        ) {
            // self.masm_.cmpb(value, Immediate(byte));
            // self.masm_.j(cc, target, distance);
        }

        pub fn Move(&mut self, output: interpreter::Register, source: Register) {
            self.masm_.Move(&mut RegisterFrameOperand(self, output).0, source); // movq(RegisterFrameOperand(output), source);
        }

        pub fn MoveTaggedIndex(&mut self, output: Register, value: Tagged<TaggedIndex>) {
            self.masm_.MoveTagged(&mut output.clone(), value); // Move(output, value);
        }

        pub fn MoveMemOperand(&mut self, output: MemOperand, source: Register) {
            self.masm_.Move(&mut output.0, source); // movq(output, source);
        }

        pub fn MoveExternalReference(&mut self, output: Register, reference: ExternalReference) {
            self.masm_.MoveExternalReference(&mut output.clone(), reference); // Move(output, reference);
        }

        pub fn MoveHandle(&mut self, output: Register, value: Handle<HeapObject>) {
            self.masm_.MoveHandle(&mut output.clone(), value); // Move(output, value);
        }

        pub fn MoveInt32(&mut self, output: Register, value: i32) {
            self.masm_.MoveInt32(&mut output.clone(), value); // Move(output, value);
        }

        pub fn MoveMaybeSmi(&mut self, output: Register, source: Register) {
            self.masm_.Move(&mut output.clone(), source); // mov_tagged(output, source);
        }

        pub fn MoveSmi(&mut self, output: Register, source: Register) {
            self.masm_.Move(&mut output.clone(), source); // mov_tagged(output, source);
        }

        pub fn Push<T: Pushable>(&mut self, vals: T) -> i32 {
            vals.push(self.masm_)
        }

        pub fn PushReverse<T: PushableReverse>(&mut self, vals: T) {
            vals.push_reverse(self.masm_)
        }
        // TODO: implement the Pop method based on the variadic macros

        pub fn LoadTaggedField(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedField(
                output,
                FieldOperand(source, offset),
            ); // LoadTaggedField(output, FieldOperand(source, offset));
        }

        pub fn LoadTaggedSignedField(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.LoadTaggedSignedField(
                output,
                FieldOperand(source, offset),
            ); // LoadTaggedSignedField(output, FieldOperand(source, offset));
        }

        pub fn LoadTaggedSignedFieldAndUntag(&mut self, output: Register, source: Register, offset: i32) {
            self.masm_.SmiUntagField(
                output,
                FieldOperand(source, offset),
            ); // LoadTaggedSignedFieldAndUntag(output, source, offset);
        }

        pub fn LoadWord16FieldZeroExtend(&mut self, _output: Register, _source: Register, _offset: i32) {
            // self.masm_.movzxwq(output, FieldOperand(source, offset));
        }

        pub fn LoadWord8Field(&mut self, _output: Register, _source: Register, _offset: i32) {
            // self.masm_.movb(output, FieldOperand(source, offset));
        }

        pub fn StoreTaggedSignedField(&mut self, target: Register, offset: i32, value: Tagged<Smi>) {
            self.masm_.StoreTaggedSignedField(FieldOperand(target, offset), value); // StoreTaggedSignedField(FieldOperand(target, offset), value);
        }

        pub fn StoreTaggedFieldWithWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
             //ASM_CODE_COMMENT(masm_);
            let scratch = WriteBarrierDescriptor::SlotAddressRegister();
            //DCHECK(!AreAliased(target, value, scratch));
            self.masm_.StoreTaggedField(FieldOperand(target, offset), value);
            //self.masm_.RecordWriteField(target, offset, value, scratch, SaveFPRegsMode::kIgnore);
        }

        pub fn StoreTaggedFieldNoWriteBarrier(&mut self, target: Register, offset: i32, value: Register) {
            self.masm_.StoreTaggedField(FieldOperand(target, offset), value); // StoreTaggedField(FieldOperand(target, offset), value);
        }

        pub fn LoadTaggedFieldTaggedReg(
            &mut self,
            _output: TaggedRegister,
            _source: Register,
            _offset: i32,
        ) {
            // self.masm_.LoadTaggedField(output, FieldOperand(source, offset));
        }

        pub fn LoadTaggedFieldTaggedRegTaggedReg(
            &mut self,
            _output: TaggedRegister,
            _source: TaggedRegister,
            _offset: i32,
        ) {
            // self.masm_.LoadTaggedField(output, FieldOperand(source, offset));
        }

        pub fn LoadTaggedFieldRegTaggedReg(
            &mut self,
            _output: Register,
            _source: TaggedRegister,
            _offset: i32,
        ) {
            // self.masm_.LoadTaggedField(output, FieldOperand(source, offset));
        }

        pub fn LoadFixedArrayElement(
            &mut self,
            _output: Register,
            _array: TaggedRegister,
            index: i32,
        ) {
            self.LoadTaggedField(
                _output,
                _array.reg(),
                (OFFSET_OF_DATA_START_FIXED_ARRAY + index as usize * kTaggedSize) as i32,
            );
        }

        pub fn LoadFixedArrayElementTaggedReg(
            &mut self,
            _output: TaggedRegister,
            _array: TaggedRegister,
            index: i32,
        ) {
            self.LoadTaggedFieldTaggedReg(
                _output,
                _array.reg(),
                (OFFSET_OF_DATA_START_FIXED_ARRAY + index as usize * kTaggedSize) as i32,
            );
        }

        pub fn TryLoadOptimizedOsrCode(
            &mut self,
            scratch_and_result: Register,
            feedback_vector: Register,
            slot: FeedbackSlot,
            on_result: &mut Label,
            distance: Label::Distance,
        ) {
            self.masm_.TryLoadOptimizedOsrCode(
                scratch_and_result,
                CodeKind::MAGLEV,
                feedback_vector,
                slot,
                on_result,
                distance,
            );
        }

        pub fn AddToInterruptBudgetAndJumpIfNotExceeded(
            &mut self,
            weight: i32,
            skip_interrupt_label: &mut Label,
        ) {
             //ASM_CODE_COMMENT(masm_);
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.AcquireScratch();
            self.LoadFeedbackCell(feedback_cell);
            self.masm_.addl(FieldOperand(feedback_cell, FeedbackCell::kInterruptBudgetOffset), Immediate(weight));
            if skip_interrupt_label != &mut Label {} // Replace with actual skip_interrupt_label check
        }

        pub fn AddToInterruptBudgetAndJumpIfNotExceededReg(
            &mut self,
            weight: Register,
            skip_interrupt_label: &mut Label,
        ) {
             //ASM_CODE_COMMENT(masm_);
            let mut scratch_scope = ScratchRegisterScope::new(self);
            let feedback_cell = scratch_scope.AcquireScratch();
            self.LoadFeedbackCell(feedback_cell);
            self.masm_.addl(FieldOperand(feedback_cell, FeedbackCell::kInterruptBudgetOffset), Immediate(weight.code() as i32));
            if skip_interrupt_label != &mut Label {} // Replace with actual skip_interrupt_label check
        }

        pub fn LdaContextSlot(
            &mut self,
            context: Register,
            index: u32,
            depth: u32,
            compression_mode: CompressionMode,
        ) {
            // [context] is coming from interpreter frame so it is already decompressed
            // when pointer compression is enabled. In order to make use of complex
            // addressing mode, any intermediate context pointer is loaded in compressed
            // form.
            if depth == 0 {
                self.LoadTaggedField(kInterpreterAccumulatorRegister, context, Context::OffsetOfElementAt(index));
            } else {
                let tagged = TaggedRegister::new(context);
                self.LoadTaggedField(tagged.reg(), context, Context::kPreviousOffset);
                let mut depth_mut = depth;
                depth_mut -= 1;
                for _ in 0..depth_mut {
                    self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kPreviousOffset);
                }
                self.LoadTaggedField(kInterpreterAccumulatorRegister, tagged.reg(), Context::OffsetOfElementAt(index));
                if COMPRESS_POINTERS_BOOL && compression_mode == CompressionMode::kForceDecompression {
                    //self.masm_.addq(tagged.reg(), kPtrComprCageBaseRegister);
                }
            }
        }

        pub fn StaContextSlot(&mut self, context: Register, value: Register, index: u32, depth: u32) {
            // [context] is coming from interpreter frame so it is already decompressed
            // when pointer compression is enabled. In order to make use of complex
            // addressing mode, any intermediate context pointer is loaded in compressed
            // form.
            let mut context_mut = context;
            if depth > 0 {
                let tagged = TaggedRegister::new(context_mut);
                self.LoadTaggedField(tagged.reg(), context_mut, Context::kPreviousOffset);
                let mut depth_mut = depth;
                depth_mut -= 1;
                for _ in 0..depth_mut {
                    self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kPreviousOffset);
                }
                if COMPRESS_POINTERS_BOOL {
                    // Decompress tagged pointer.
                    //self.masm_.addq(tagged.reg(), kPtrComprCageBaseRegister);
                }
                context_mut = tagged.reg();
            }
            self.StoreTaggedFieldWithWriteBarrier(context_mut, Context::OffsetOfElementAt(index), value);
        }

        pub fn LdaModuleVariable(&mut self, context: Register, cell_index: i32, depth: u32) {
            // [context] is coming from interpreter frame so it is already decompressed.
            // In order to make use of complex addressing mode when pointer compression is
            // enabled, any intermediate context pointer is loaded in compressed form.
            let tagged = TaggedRegister::new(context);
            let mut context_mut = context;
            if depth == 0 {
                self.LoadTaggedField(tagged.reg(), context_mut, Context::kExtensionOffset);
            } else {
                self.LoadTaggedField(tagged.reg(), context_mut, Context::kPreviousOffset);
                let mut depth_mut = depth;
                depth_mut -= 1;
                for _ in 0..depth_mut {
                    self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kPreviousOffset);
                }
                self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kExtensionOffset);
            }
            if cell_index > 0 {
                self.LoadTaggedField(tagged.reg(), tagged.reg(), SourceTextModule::kRegularExportsOffset);
                // The actual array index is (cell_index - 1).
                // cell_index -= 1;
            } else {
                self.LoadTaggedField(tagged.reg(), tagged.reg(), SourceTextModule::kRegularImportsOffset);
                // The actual array index is (-cell_index - 1).
                // cell_index = -cell_index - 1;
            }
            let cell_index_corrected = if cell_index > 0 {cell_index - 1} else {-cell_index -1};
            self.LoadFixedArrayElementTaggedReg(tagged, tagged, cell_index_corrected);
            self.LoadTaggedField(kInterpreterAccumulatorRegister, tagged.reg(), Cell::kValueOffset);
        }

        pub fn StaModuleVariable(&mut self, context: Register, value: Register, cell_index: i32, depth: u32) {
            // [context] is coming from interpreter frame so it is already decompressed.
            // In order to make use of complex addressing mode when pointer compression is
            // enabled, any intermediate context pointer is loaded in compressed form.
            let tagged = TaggedRegister::new(context);
            let mut context_mut = context;
            if depth == 0 {
                self.LoadTaggedField(tagged.reg(), context_mut, Context::kExtensionOffset);
            } else {
                self.LoadTaggedField(tagged.reg(), context_mut, Context::kPreviousOffset);
                let mut depth_mut = depth;
                depth_mut -= 1;
                for _ in 0..depth_mut {
                    self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kPreviousOffset);
                }
                self.LoadTaggedField(tagged.reg(), tagged.reg(), Context::kExtensionOffset);
            }
            self.LoadTaggedField(tagged.reg(), tagged.reg(), SourceTextModule::kRegularExportsOffset);

            let cell_index_corrected = cell_index - 1;
            self.LoadFixedArrayElement(context_mut, tagged, cell_index_corrected);
            self.StoreTaggedFieldWithWriteBarrier(context_mut, Cell::kValueOffset, value);
        }

        pub fn IncrementSmi(&mut self, _lhs: MemOperand) {
            // self.masm_.SmiAddConstant(lhs, Smi::FromInt(1));
        }

        pub fn Word32And(&mut self, _output: Register, _lhs: Register, _rhs: i32) {
            // self.Move(output, lhs);
            // self.masm_.andq(output, Immediate(rhs));
        }

        pub fn Switch(&mut self, reg: Register, case_value_base: i32, labels: &mut [&mut Label], num_labels: usize) {
            // ASM_CODE_COMMENT(masm_);
            let mut scope = ScratchRegisterScope::new(self);
            // self.masm_.Switch(scope.AcquireScratch(), reg, case_value_base, labels, num_labels);
            let scratch = scope.AcquireScratch();
            self.masm_.Switch(scratch, reg, case_value_base, labels, num_labels);
        }

        pub fn MaybeEmitPlaceHolderForDeopt(&mut self) {
            // if v8_flags.cet_compatible {
            //     self.masm_.Nop(Assembler::kIntraSegmentJmpInstrSize);
            // }
        }

        fn LoadFeedbackCell(&mut self, _feedback_cell: Register) {
            // Load the feedback cell into the given register
        }
    }

    pub trait Pushable {
        fn push(self, masm: &mut MacroAssembler) -> i32;
    }
    pub trait PushableReverse {
        fn push_reverse(self, masm: &mut MacroAssembler);
    }

    impl Pushable for RootIndex {
        fn push(self, masm: &mut MacroAssembler) -> i32 {
            masm.PushRoot(self);
            1
        }
    }
    impl Pushable for Register {
        fn push(self, masm: &mut MacroAssembler) -> i32 {
            masm.Push(self);
            1
        }
    }
    impl Pushable for Tagged<TaggedIndex> {
        fn push(self, masm: &mut MacroAssembler) -> i32 {
            masm.PushTagged(self);
            1
        }
    }
    impl Pushable for Tagged<Smi> {
        fn push(self, masm: &mut MacroAssembler) -> i32 {
            masm.PushSmi(self);
            1
        }
    }
    impl Pushable for Handle<