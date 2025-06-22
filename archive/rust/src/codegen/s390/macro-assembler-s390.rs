// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header must be included via macro-assembler.h
// #![error = "This header must be included via macro-assembler.h"]

mod macro_assembler_s390 {
    use std::convert::TryInto;
    use crate::assembler_s390::*;
    use crate::globals::*;
    use crate::frame_constants::*;
    use crate::isolate_data::*;
    use crate::contexts::*;
    use crate::bailout_reason::*;

    #[derive(Debug, Copy, Clone)]
    pub enum StackLimitKind {
        InterruptStackLimit,
        RealStackLimit,
    }

    // Generate a MemOperand for loading a field from an object.
    #[inline]
    pub fn field_mem_operand(object: Register, offset: i32) -> MemOperand {
        MemOperand::new(object, offset - k_heap_object_tag)
    }

    // Generate a MemOperand for loading a field from an object.
    #[inline]
    pub fn field_mem_operand_indexed(object: Register, index: Register, offset: i32) -> MemOperand {
        MemOperand::new_indexed(object, index, offset - k_heap_object_tag)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum LinkRegisterStatus {
        LRHasNotBeenSaved,
        LRHasBeenSaved,
    }

    pub fn get_register_that_is_not_one_of(reg1: Register, reg2: Option<Register>, reg3: Option<Register>, reg4: Option<Register>, reg5: Option<Register>, reg6: Option<Register>) -> Register {
        // Placeholder implementation.  Needs to actually implement the logic.
        // In C++, `no_reg` is likely a sentinel value.  Here, we use `Option<Register>`.
        // This is a simplification and might need more sophisticated handling.
        if reg1 != Register::R0 {
          return Register::R0;
        } else {
          return Register::R1; // Default, implement properly.
        }
    }

    // V8_EXPORT_PRIVATE MacroAssembler
    pub struct MacroAssembler {
        base: MacroAssemblerBase,
    }

    impl MacroAssembler {
        pub fn new(assembler: Assembler) -> Self {
            MacroAssembler { base: MacroAssemblerBase::new(assembler) }
        }
        
        pub fn get_assembler(&self) -> &Assembler {
          self.base.get_assembler()
        }

        pub fn get_assembler_mut(&mut self) -> &mut Assembler {
            self.base.get_assembler_mut()
        }

        pub fn CallBuiltin(&mut self, builtin: Builtin, cond: Condition) {
            self.base.CallBuiltin(builtin, cond);
        }

        pub fn TailCallBuiltin(&mut self, builtin: Builtin, cond: Condition) {
            self.base.TailCallBuiltin(builtin, cond);
        }

        pub fn AtomicCmpExchangeHelper(&mut self, addr: Register, output: Register, old_value: Register, new_value: Register, start: i32, end: i32, shift_amount: i32, offset: i32, temp0: Register, temp1: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AtomicCmpExchangeU8(&mut self, addr: Register, output: Register, old_value: Register, new_value: Register, temp0: Register, temp1: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AtomicCmpExchangeU16(&mut self, addr: Register, output: Register, old_value: Register, new_value: Register, temp0: Register, temp1: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AtomicExchangeHelper(&mut self, addr: Register, value: Register, output: Register, start: i32, end: i32, shift_amount: i32, offset: i32, scratch: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AtomicExchangeU8(&mut self, addr: Register, value: Register, output: Register, scratch: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AtomicExchangeU16(&mut self, addr: Register, value: Register, output: Register, scratch: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        
        pub fn DoubleMax(&mut self, result_reg: DoubleRegister, left_reg: DoubleRegister, right_reg: DoubleRegister) {
            // Placeholder implementation. Needs actual implementation.
            unimplemented!();
        }
        
        pub fn DoubleMin(&mut self, result_reg: DoubleRegister, left_reg: DoubleRegister, right_reg: DoubleRegister) {
            // Placeholder implementation. Needs actual implementation.
            unimplemented!();
        }
        
        pub fn FloatMax(&mut self, result_reg: DoubleRegister, left_reg: DoubleRegister, right_reg: DoubleRegister) {
            // Placeholder implementation. Needs actual implementation.
            unimplemented!();
        }
        
        pub fn FloatMin(&mut self, result_reg: DoubleRegister, left_reg: DoubleRegister, right_reg: DoubleRegister) {
            // Placeholder implementation. Needs actual implementation.
            unimplemented!();
        }
        
        pub fn CeilF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {
          unimplemented!();
        }
        
        pub fn CeilF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn FloorF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn FloorF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn TruncF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn TruncF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn NearestIntF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }
        
        pub fn NearestIntF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            unimplemented!();
        }

        pub fn LoadFromConstantsTable(&mut self, destination: Register, constant_index: i32) {
            self.base.LoadFromConstantsTable(destination, constant_index);
        }

        pub fn LoadRootRegisterOffset(&mut self, destination: Register, offset: i64) {
            self.base.LoadRootRegisterOffset(destination, offset);
        }

        pub fn LoadRootRelative(&mut self, destination: Register, offset: i32) {
          self.base.LoadRootRelative(destination, offset);
        }

        pub fn StoreRootRelative(&mut self, offset: i32, value: Register) {
          self.base.StoreRootRelative(offset, value);
        }

        // Operand pointing to an external reference.
        // May emit code to set up the scratch register. The operand is
        // only guaranteed to be correct as long as the scratch register
        // isn't changed.
        // If the operand is used more than once, use a scratch register
        // that is guaranteed not to be clobbered.
        pub fn ExternalReferenceAsOperand(&mut self, reference: ExternalReference, scratch: Register) -> MemOperand {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn ExternalReferenceAsOperand_id(&mut self, id: IsolateFieldId) -> MemOperand {
            self.ExternalReferenceAsOperand(ExternalReference::Create(id), Register::NoReg)
        }

        // Jump, Call, and Ret pseudo instructions implementing inter-working.
        pub fn Jump_reg(&mut self, target: Register, cond: Condition) {
            self.base.Jump_reg(target, cond);
        }

        pub fn Jump_addr(&mut self, target: usize, rmode: RelocInfoMode, cond: Condition) {
            self.base.Jump_addr(target, rmode, cond);
        }

        pub fn Jump_code(&mut self, code: Handle<Code>, rmode: RelocInfoMode, cond: Condition) {
            self.base.Jump_code(code, rmode, cond);
        }

        pub fn Jump_ext(&mut self, reference: &ExternalReference) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Jump the register contains a smi.
        #[inline]
        pub fn JumpIfSmi(&mut self, value: Register, smi_label: &mut Label) {
            self.TestIfSmi(value);
            self.beq(smi_label);  // branch if SMI
        }

        pub fn CheckSmi(&mut self, src: Register) -> Condition {
            self.TestIfSmi(src);
            Condition::Eq
        }

        pub fn JumpIfEqual(&mut self, x: Register, y: i32, dest: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn JumpIfLessThan(&mut self, x: Register, y: i32, dest: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Caution: if {reg} is a 32-bit negative int, it should be sign-extended to
        // 64-bit before calling this function.
        pub fn Switch(&mut self, scrach: Register, reg: Register, case_base_value: i32, labels: &mut [&mut Label], num_labels: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn JumpIfCodeIsMarkedForDeoptimization(&mut self, code: Register, scratch: Register, if_marked_for_deoptimization: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn JumpIfCodeIsTurbofanned(&mut self, code: Register, scratch: Register, if_turbofanned: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn LoadMap(&mut self, destination: Register, object: Register) {
          self.base.LoadMap(destination, object);
        }

        pub fn LoadCompressedMap(&mut self, destination: Register, object: Register) {
          self.base.LoadCompressedMap(destination, object);
        }

        pub fn LoadFeedbackVector(&mut self, dst: Register, closure: Register, scratch: Register, fbv_undef: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn Call_reg(&mut self, target: Register) {
            self.base.Call_reg(target);
        }

        pub fn Call_addr(&mut self, target: usize, rmode: RelocInfoMode, cond: Condition) {
            self.base.Call_addr(target, rmode, cond);
        }

        pub fn Call_code(&mut self, code: Handle<Code>, rmode: RelocInfoMode, cond: Condition) {
            self.base.Call_code(code, rmode, cond);
        }

        pub fn Ret(&mut self) {
            self.b(Register::R14);
        }

        pub fn Ret_cond(&mut self, cond: Condition) {
            self.b_cond(cond, Register::R14);
        }

        // TODO(olivf, 42204201) Rename this to AssertNotDeoptimized once
        // non-leaptiering is removed from the codebase.
        pub fn BailoutIfDeoptimized(&mut self, scratch: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CallForDeoptimization(&mut self, target: Builtin, deopt_id: i32, exit: &mut Label, kind: DeoptimizeKind, ret: &mut Label, jump_deoptimization_entry_label: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Emit code to discard a non-negative number of pointer-sized elements
        // from the stack, clobbering only the sp register.
        pub fn Drop_imm(&mut self, count: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn Drop_reg(&mut self, count: Register, scratch: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn Ret_drop(&mut self, drop: i32) {
            self.Drop_imm(drop);
            self.Ret();
        }

        pub fn Call_label(&mut self, target: &mut Label) {
            self.b_label(target); // Simple branch for label call?
        }

        pub fn GetLabelAddress(&mut self, dst: Register, target: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Load the builtin given by the Smi in |builtin_index| into |target|.
        pub fn LoadEntryFromBuiltinIndex(&mut self, builtin_index: Register, target: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn LoadEntryFromBuiltin(&mut self, builtin: Builtin, destination: Register) {
          self.base.LoadEntryFromBuiltin(builtin, destination);
        }

        pub fn EntryFromBuiltinAsOperand(&mut self, builtin: Builtin) -> MemOperand {
          self.base.EntryFromBuiltinAsOperand(builtin)
        }

        // Load the code entry point from the Code object.
        pub fn LoadCodeInstructionStart(&mut self, destination: Register, code_object: Register, tag: CodeEntrypointTag) {
          self.base.LoadCodeInstructionStart(destination, code_object, tag);
        }

        pub fn CallCodeObject(&mut self, code_object: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn JumpCodeObject(&mut self, code_object: Register, jump_mode: JumpMode) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CallBuiltinByIndex(&mut self, builtin_index: Register, target: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Register move. May do nothing if the registers are identical.
        pub fn Move_smi(&mut self, dst: Register, smi: Tagged<Smi>) {
            self.LoadSmiLiteral(dst, smi);
        }

        pub fn Move_handle(&mut self, dst: Register, source: Handle<HeapObject>, rmode: RelocInfoMode) {
            self.base.Move_handle(dst, source, rmode);
        }

        pub fn Move_extref(&mut self, dst: Register, reference: ExternalReference) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn LoadIsolateField(&mut self, dst: Register, id: IsolateFieldId) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn Move_memop(&mut self, dst: Register, src: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn Move_reg(&mut self, dst: Register, src: Register, cond: Condition) {
            self.base.Move_reg(dst, src, cond);
        }

        pub fn Move_freg(&mut self, dst: DoubleRegister, src: DoubleRegister) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MoveChar(&mut self, opnd1: &MemOperand, opnd2: &MemOperand, length: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CompareLogicalChar(&mut self, opnd1: &MemOperand, opnd2: &MemOperand, length: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn ExclusiveOrChar(&mut self, opnd1: &MemOperand, opnd2: &MemOperand, length: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn RotateInsertSelectBits(&mut self, dst: Register, src: Register, startBit: &Operand, endBit: &Operand, shiftAmt: &Operand, zeroBits: bool) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn BranchRelativeOnIdxHighP(&mut self, dst: Register, inc: Register, L: &mut Label) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MaybeSaveRegisters(&mut self, registers: RegList) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MaybeRestoreRegisters(&mut self, registers: RegList) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CallEphemeronKeyBarrier(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CallRecordWriteStubSaveRegisters(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode, mode: StubCallMode) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn CallRecordWriteStub(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode, mode: StubCallMode) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MultiPush(&mut self, regs: RegList, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MultiPop(&mut self, regs: RegList, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        
        }

        pub fn MultiPushDoubles(&mut self, dregs: DoubleRegList, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }
    
        pub fn MultiPopDoubles(&mut self, dregs: DoubleRegList, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }
    
        pub fn MultiPushV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }
    
        pub fn MultiPopV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }
    
        pub fn MultiPushF64OrV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }
    
        pub fn MultiPopF64OrV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn PushAll(&mut self, registers: RegList) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn PopAll(&mut self, registers: RegList) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn PushAll_doubles(&mut self, registers: DoubleRegList, stack_slot_size: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn PopAll_doubles(&mut self, registers: DoubleRegList, stack_slot_size: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Calculate how much stack space (in bytes) are required to store caller
        // registers excluding those specified in the arguments.
        pub fn RequiredStackSizeForCallerSaved(&self, fp_mode: SaveFPRegsMode, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Push caller saved registers on the stack, and return the number of bytes
        // stack pointer is adjusted.
        pub fn PushCallerSaved(&mut self, fp_mode: SaveFPRegsMode, scratch: Register, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Restore caller saved registers from the stack, and return the number of
        // bytes stack pointer is adjusted.
        pub fn PopCallerSaved(&mut self, fp_mode: SaveFPRegsMode, scratch: Register, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Load an object from the root table.
        pub fn LoadRoot(&mut self, destination: Register, index: RootIndex) {
            self.base.LoadRoot(destination, index, Condition::Al);
        }
        pub fn LoadRoot_cond(&mut self, destination: Register, index: RootIndex, cond: Condition) {
            self.base.LoadRoot(destination, index, cond);
        }

        pub fn LoadTaggedRoot(&mut self, destination: Register, index: RootIndex) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        //--------------------------------------------------------------------------
        // S390 Macro Assemblers for Instructions
        //--------------------------------------------------------------------------

        // Arithmetic Operations

        // Add (Register - Immediate)
        pub fn AddS32(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS32_regimm(&mut self, dst: Register, src: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_regimm(&mut self, dst: Register, src: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS32_regimm32(&mut self, dst: Register, src: Register, imm: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_regimm32(&mut self, dst: Register, src: Register, imm: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Add (Register - Register)
        pub fn AddS32_reg(&mut self, dst: Register, src: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_reg(&mut self, dst: Register, src: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS32_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Add (Register - Mem)
        pub fn AddS32_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Add (Mem - Immediate)
        pub fn AddS32_memimm(&mut self, opnd: &MemOperand, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddS64_memimm(&mut self, opnd: &MemOperand, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Add Logical (Register - Register)
        pub fn AddU32_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Add Logical (Register - Immediate)
        pub fn AddU32(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddU64(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddU64_imm(&mut self, dst: Register, imm: i32) {
            self.AddU64(dst, &Operand::Imm(imm as i64));
        }

        pub fn AddU64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddU64_reg(&mut self, dst: Register, src: Register) {
            self.algr(dst, src);
        }

        // Add Logical (Register - Mem)
        pub fn AddU32_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn AddU64_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Subtract (Register - Immediate)
        pub fn SubS32(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64(&mut self, dst: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS32_regimm(&mut self, dst: Register, src: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64_regimm(&mut self, dst: Register, src: Register, imm: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS32_regimm32(&mut self, dst: Register, src: Register, imm: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64_regimm32(&mut self, dst: Register, src: Register, imm: i32) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Subtract (Register - Register)
        pub fn SubS32_reg(&mut self, dst: Register, src: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64_reg(&mut self, dst: Register, src: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS32_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Subtract (Register - Mem)
        pub fn SubS32_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubS64_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn LoadAndSub32(&mut self, dst: Register, src: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn LoadAndSub64(&mut self, dst: Register, src: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Subtract Logical (Register - Mem)
        pub fn SubU32_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn SubU64_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Subtract Logical 32-bit
        pub fn SubU32_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        // Multiply
        pub fn MulS64(&mut self, dst: Register, opnd: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS64_reg(&mut self, dst: Register, src: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS64_mem(&mut self, dst: Register, opnd: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            if crate::assembler_s390::CpuFeatures::IsSupported(CpuFeatures::MISC_INSTR_EXT2) {
                self.msgrkc(dst, src1, src2);
            } else {
                if dst == src2 {
                    self.MulS64_reg(dst, src1);
                } else if dst == src1 {
                    self.MulS64_reg(dst, src2);
                } else {
                    self.mov(dst, src1);
                    self.MulS64_reg(dst, src2);
                }
            }
        }

        pub fn MulS32_mem(&mut self, dst: Register, src1: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS32_reg(&mut self, dst: Register, src1: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS32_op(&mut self, dst: Register, src1: &Operand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulS32_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            if crate::assembler_s390::CpuFeatures::IsSupported(CpuFeatures::MISC_INSTR_EXT2) {
                self.msrkc(dst, src1, src2);
            } else {
                if dst == src2 {
                    self.MulS32_reg(dst, src1);
                } else if dst == src1 {
                    self.MulS32_reg(dst, src2);
                } else {
                    self.mov(dst, src1);
                    self.MulS32_reg(dst, src2);
                }
            }
        }

        pub fn MulHighS64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulHighS64_regmem(&mut self, dst: Register, src1: Register, src2: &MemOperand) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulHighU64_regreg(&mut self, dst: Register, src1: Register, src2: Register) {
            // Placeholder implementation.  Needs actual implementation.
            unimplemented!();
        }

        pub fn MulHighU64_regmem(&mut self, dst: Register, src