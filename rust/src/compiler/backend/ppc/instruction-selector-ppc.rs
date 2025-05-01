// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// use std::optional::Optional; // `std::option::Option` already exists.
use std::convert::TryInto;

// use crate::base::iterator; // TODO: Implement or find equivalent.
// use crate::compiler::backend::instruction_selector_impl; // TODO: Implement or find equivalent.
// use crate::compiler::turboshaft::opmasks; // TODO: Implement or find equivalent.
// use crate::execution::ppc::frame_constants_ppc; // TODO: Implement or find equivalent.
// use crate::roots::roots_inl; // TODO: Implement or find equivalent.

// use turboshaft; // NOLINT(build/namespaces)

// TODO: Define turboshaft module or find equivalent

#[derive(Debug, Copy, Clone)]
enum ImmediateMode {
    Int16Imm,
    Int16ImmUnsigned,
    Int16ImmNegate,
    Int16Imm4ByteAligned,
    Shift32Imm,
    Int34Imm,
    Shift64Imm,
    NoImmediate,
}

// Adds PPC-specific methods for generating operands.
#[derive(Debug)]
struct PPCOperandGeneratorT<'a> {
    selector: &'a mut InstructionSelectorT, // Assuming InstructionSelectorT is defined
}

impl<'a> PPCOperandGeneratorT<'a> {
    fn new(selector: &'a mut InstructionSelectorT) -> Self {
        PPCOperandGeneratorT { selector }
    }

    fn use_operand(&mut self, node: OpIndex, mode: ImmediateMode) -> InstructionOperand {
        if self.can_be_immediate(node, mode) {
            self.use_immediate(node)
        } else {
            self.use_register(node)
        }
    }

    fn can_be_immediate(&mut self, node: OpIndex, mode: ImmediateMode) -> bool {
        let constant = self.selector.get(node).try_cast_constant_op();
        match constant {
            Some(constant) => {
                match constant.kind {
                    ConstantOpKind::CompressedHeapObject => {
                        if !COMPRESS_POINTERS_BOOL {
                            return false;
                        }
                        // For builtin code we need static roots
                        if self.selector.isolate.bootstrapper() && !V8_STATIC_ROOTS_BOOL {
                            return false;
                        }
                        let roots_table = &self.selector.isolate.roots_table;
                        // let root_index: RootIndex;  //TODO: Add rootindex
                        let value = constant.handle();
                        if roots_table.is_root_handle(&value) {  //&root_index)) {
                            //TODO: FIX
                            //if !RootsTable::IsReadOnly(root_index) {
                            //    return false;
                            //}
                            //TODO: Add read only
                            //return self.can_be_immediate(
                            //    MacroAssemblerBase::ReadOnlyRootPtr(
                            //       root_index, self.selector.isolate()),
                            //    mode,
                            //);
                            return false; // TODO: Replace when roots table is fixed
                        }
                        false
                    }
                    _ => {
                        let mut value: i64 = 0;
                        if !self.selector.match_signed_integral_constant(node, &mut value) {
                            return false;
                        }
                        self.can_be_immediate_value(value, mode)
                    }
                }
            }
            None => false,
        }
    }

    fn can_be_immediate_value(&self, value: i64, mode: ImmediateMode) -> bool {
        match mode {
            ImmediateMode::Int16Imm => is_int16(value),
            ImmediateMode::Int16ImmUnsigned => is_uint16(value),
            ImmediateMode::Int16ImmNegate => is_int16(-value),
            ImmediateMode::Int16Imm4ByteAligned => is_int16(value) && (value & 3) == 0,
            ImmediateMode::Shift32Imm => (0 <= value) && (value < 32),
            ImmediateMode::Int34Imm => is_int34(value),
            ImmediateMode::Shift64Imm => (0 <= value) && (value < 64),
            ImmediateMode::NoImmediate => false,
        }
    }

    fn use_register(&self, node: OpIndex) -> InstructionOperand {
        // Placeholder: Implementation depends on InstructionOperand and related types.
        InstructionOperand::Register(node) // Dummy value
    }

    fn use_immediate(&self, node: OpIndex) -> InstructionOperand {
        // Placeholder: Implementation depends on InstructionOperand and related types.
        InstructionOperand::Immediate(node) // Dummy value
    }

    fn define_as_register(&self, node: OpIndex) -> InstructionOperand {
        // Placeholder: Implementation depends on InstructionOperand and related types.
        InstructionOperand::Register(node) // Dummy value
    }

    fn define_same_as_first(&self, node: OpIndex) -> InstructionOperand {
        // Placeholder: Implementation depends on InstructionOperand and related types.
        InstructionOperand::Register(node) // Dummy value
    }
    fn no_output(&self) -> InstructionOperand {
        InstructionOperand::None
    }

    fn use_fixed(&self, node: OpIndex, reg: Register) -> InstructionOperand {
        InstructionOperand::Fixed(node, reg)
    }
    fn temp_register(&self) -> InstructionOperand {
        InstructionOperand::Register(0) // Dummy
    }
    fn temp_immediate(&self, value: i64) -> InstructionOperand {
        InstructionOperand::Immediate(value as OpIndex)
    }

    fn use_register_with_mode(&self, node: OpIndex, mode: OperandGeneratorMode) -> InstructionOperand {
        match mode {
            OperandGeneratorMode::Register => self.use_register(node),
            OperandGeneratorMode::UniqueRegister => self.use_unique_register(node),
        }
    }
    fn use_unique_register(&self, base: OpIndex) -> InstructionOperand {
        InstructionOperand::Register(base)
    }
}

#[derive(Debug)]
enum InstructionOperand {
    Register(OpIndex),
    Immediate(OpIndex),
    None,
    Fixed(OpIndex, Register),
}
#[derive(Debug, Copy, Clone)]
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    // ... other registers
    D1,
    D2,
}

#[derive(Debug, Copy, Clone)]
enum RecordWriteMode {
    NoRecordWrite,
    SlowRecordWrite,
    // Add more modes as needed
}
#[derive(Debug, Copy, Clone)]
enum IndirectPointerTag {
    // Add more tags as needed
}

#[derive(Debug, Copy, Clone)]
enum OperandGeneratorMode {
    Register,
    UniqueRegister,
}

#[derive(Debug, Copy, Clone)]
enum StackCheckKind {
  kJSFunctionEntry,
  kOther,
}
// Helper functions
fn is_int16(value: i64) -> bool {
    (i16::MIN as i64 <= value) && (value <= i16::MAX as i64)
}

fn is_uint16(value: i64) -> bool {
    (0 <= value) && (value <= u16::MAX as i64)
}

fn is_int34(value: i64) -> bool {
    (-(1i64 << 33) <= value) && (value < (1i64 << 33))
}
// TODO: Define other helper functions like is_uint32, is_int32, etc.

// Global Constants (replace preprocessor macros)
const COMPRESS_POINTERS_BOOL: bool = false; // Or true based on configuration.
const V8_STATIC_ROOTS_BOOL: bool = false;
const kSystemPointerSize: i32 = 8;

const kStackFrameExtraParamSlot: i32 = 0;
// InstructionSelectorT definition
#[derive(Debug)]
struct InstructionSelectorT {
    isolate: Isolate, // Assuming Isolate is defined.
    frame_: Frame,
    sequence_: Sequence,
    enable_switch_jump_table_: EnableSwitchJumpTable,
}

// Assuming other necessary structs like Isolate, Frame, Sequence, SwitchInfo are defined.

impl InstructionSelectorT {
    fn new(isolate: Isolate, frame_: Frame, sequence_: Sequence, enable_switch_jump_table_: EnableSwitchJumpTable) -> Self {
        InstructionSelectorT { isolate, frame_, sequence_, enable_switch_jump_table_ }
    }
    fn visit_stack_slot(&mut self, node: OpIndex) {
        let stack_slot = self.get(node).cast_stack_slot_op();
        let slot = self.frame_.allocate_spill_slot(stack_slot.size, stack_slot.alignment, stack_slot.is_tagged);
        let g = OperandGenerator::new(self);

        self.emit(
            ArchOpcode::ArchStackSlot,
            g.define_as_register(node),
            self.sequence_.add_immediate(Constant::new(slot)),
            0,
            &[],
        );
    }

    fn visit_abort_csadcheck(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        self.emit(
            ArchOpcode::ArchAbortCSADcheck,
            g.no_output(),
            g.use_fixed(self.input_at(node, 0), Register::R4),
        );
    }
    fn is_load_root_register(&self, base: OpIndex) -> bool {
        false // Dummy
    }

    fn visit_load(&mut self, node: OpIndex) {
        let load_view = self.load_view(node);
        let mut mode: ImmediateMode;
        let opcode = select_load_opcode(load_view.ts_loaded_rep(), load_view.ts_result_rep(), &mut mode);
        visit_load_common(self, node, mode, opcode);
    }
    fn visit_protected_load(&mut self, _node: OpIndex) {
        unimplemented!()
    }
    fn visit_store(&mut self, node: OpIndex) {
        visit_store_common(self, node, self.store_view(node).stored_rep(), None);
    }
    fn visit_protected_store(&mut self, _node: OpIndex) {
        unimplemented!()
    }
    fn visit_unaligned_load(&mut self, _node: OpIndex) {
        unreachable!()
    }
    fn visit_unaligned_store(&mut self, _node: OpIndex) {
        unreachable!()
    }

    fn visit_word32_and(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);

        let bitwise_and = self.get(node).cast_word_binop_op();
        let mut mb: i32 = 0;
        let mut me: i32 = 0;
        let mut value: i64 = 0;
        if self.match_signed_integral_constant(bitwise_and.right(), &mut value) &&
            is_contiguous_mask32(value as u32, &mut mb, &mut me) {
            let mut sh: i32 = 0;
            let left = bitwise_and.left();
            let lhs = self.get(left);
            if (lhs.is::<Opmask::Word32ShiftRightLogical>() ||
                lhs.is::<Opmask::Word32ShiftLeft>()) &&
                self.can_cover(node, left) {
                // Try to absorb left/right shift into rlwinm
                let mut shift_by: i32 = 0;
                let shift_op = lhs.cast::<ShiftOp>();
                if self.match_integral_word32_constant(shift_op.right(), &mut shift_by) &&
                    base::is_in_range(shift_by, 0, 31) {
                    let left = shift_op.left();
                    sh = shift_by;
                    if lhs.is::<Opmask::Word32ShiftRightLogical>() {
                        // Adjust the mask such that it doesn't include any rotated bits.
                        if mb > 31 - sh { mb = 31 - sh; }
                        sh = (32 - sh) & 0x1F;
                    } else {
                        // Adjust the mask such that it doesn't include any rotated bits.
                        if me < sh { me = sh; }
                    }
                }
            }
            if mb >= me {
                self.emit(ArchOpcode::PPC_RotLeftAndMask32,
                          g.define_as_register(node), g.use_register(left),
                          g.temp_immediate(sh as i64), g.temp_immediate(mb as i64), g.temp_immediate(me as i64));
                return;
            }
        }
        visit_logical(self, node, ArchOpcode::PPC_And, self.can_cover(node, bitwise_and.left()),
                      self.can_cover(node, bitwise_and.right()), ImmediateMode::Int16ImmUnsigned);
    }

    fn visit_word64_and(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);

        let bitwise_and = self.get(node).cast_word_binop_op();
        let mut mb: i32 = 0;
        let mut me: i32 = 0;
        let mut value: i64 = 0;
        if self.match_signed_integral_constant(bitwise_and.right(), &mut value) &&
            is_contiguous_mask64(value as u64, &mut mb, &mut me) {
            let mut sh: i32 = 0;
            let left = bitwise_and.left();
            let lhs = self.get(left);
            if (lhs.is::<Opmask::Word64ShiftRightLogical>() ||
                lhs.is::<Opmask::Word64ShiftLeft>()) &&
                self.can_cover(node, left) {
                // Try to absorb left/right shift into rlwinm
                let mut shift_by: i64 = 0;
                let shift_op = lhs.cast::<ShiftOp>();
                if self.match_integral_word64_constant(shift_op.right(), &mut shift_by) &&
                    base::is_in_range(shift_by, 0, 63) {
                    let left = shift_op.left();
                    sh = shift_by as i32;
                    if lhs.is::<Opmask::Word64ShiftRightLogical>() {
                        // Adjust the mask such that it doesn't include any rotated bits.
                        if mb > 63 - sh { mb = 63 - sh; }
                        sh = (64 - sh) & 0x3F;
                    } else {
                        // Adjust the mask such that it doesn't include any rotated bits.
                        if me < sh { me = sh; }
                    }
                }
            }
            if mb >= me {
                let mut match_ = false;
                let mut opcode: ArchOpcode = ArchOpcode::PPC_Add32;
                let mut mask = 0;
                if me == 0 {
                    match_ = true;
                    opcode = ArchOpcode::PPC_RotLeftAndClearLeft64;
                    mask = mb;
                } else if mb == 63 {
                    match_ = true;
                    opcode = ArchOpcode::PPC_RotLeftAndClearRight64;
                    mask = me;
                } else if sh != 0 && me <= sh && lhs.is::<Opmask::Word64ShiftLeft>() {
                    match_ = true;
                    opcode = ArchOpcode::PPC_RotLeftAndClear64;
                    mask = mb;
                }
                if match_ {
                    self.emit(opcode, g.define_as_register(node), g.use_register(left),
                              g.temp_immediate(sh as i64), g.temp_immediate(mask as i64));
                    return;
                }
            }
        }
        visit_logical(self, node, ArchOpcode::PPC_And, self.can_cover(node, bitwise_and.left()),
                      self.can_cover(node, bitwise_and.right()), ImmediateMode::Int16ImmUnsigned);
    }
    fn visit_word32_or(&mut self, node: OpIndex) {
        let op = self.get(node).cast_word_binop_op();
        visit_logical(self, node, ArchOpcode::PPC_Or, self.can_cover(node, op.left()),
                      self.can_cover(node, op.right()), ImmediateMode::Int16ImmUnsigned);
    }
    fn visit_word64_or(&mut self, node: OpIndex) {
        let op = self.get(node).cast_word_binop_op();
        visit_logical(self, node, ArchOpcode::PPC_Or, self.can_cover(node, op.left()),
                      self.can_cover(node, op.right()), ImmediateMode::Int16ImmUnsigned);
    }
    fn visit_word32_xor(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let bitwise_xor = self.get(node).cast_word_binop_op();
        let mut mask: i32 = 0;
        if self.match_integral_word32_constant(bitwise_xor.right(), &mut mask) &&
            mask == -1 {
            self.emit(ArchOpcode::PPC_Not, g.define_as_register(node), g.use_register(bitwise_xor.left()));
        } else {
            visit_binop(self, node, ArchOpcode::PPC_Xor, ImmediateMode::Int16ImmUnsigned);
        }
    }
    fn visit_stack_pointer_greater_than(&mut self, node: OpIndex, cont: &mut FlagsContinuation) {
        let kind: StackCheckKind;
        let value: OpIndex;
        let op = self.turboshaft_graph().get(node).cast::<StackPointerGreaterThanOp>();
        kind = op.kind;
        value = op.stack_limit();
        let opcode = ArchOpcode::ArchStackPointerGreaterThan | MiscField::encode(kind as i32);

        let g = PPCOperandGeneratorT::new(self);

        // No outputs.
        let outputs: &[InstructionOperand] = &[];
        let output_count = 0;

        // Applying an offset to this stack check requires a temp register. Offsets
        // are only applied to the first stack check. If applying an offset, we must
        // ensure the input and temp registers do not alias, thus kUniqueRegister.
        let temps: [InstructionOperand; 1] = [g.temp_register()];
        let temp_count = if kind == StackCheckKind::kJSFunctionEntry { 1 } else { 0 };
        let register_mode = if kind == StackCheckKind::kJSFunctionEntry {
            OperandGeneratorMode::UniqueRegister
        } else {
            OperandGeneratorMode::Register
        };

        let inputs: [InstructionOperand; 1] = [g.use_register_with_mode(value, register_mode)];
        let input_count = 1; //arraysize(inputs);

        self.emit_with_continuation(opcode, output_count, outputs, input_count, &inputs,
                                     temp_count, &temps, cont);
    }

    fn visit_word64_xor(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let bitwise_xor = self.get(node).cast_word_binop_op();
        let mut mask: i64 = 0;
        if self.match_integral_word64_constant(bitwise_xor.right(), &mut mask) &&
            mask == -1 {
            self.emit(ArchOpcode::PPC_Not, g.define_as_register(node), g.use_register(bitwise_xor.left()));
        } else {
            visit_binop(self, node, ArchOpcode::PPC_Xor, ImmediateMode::Int16ImmUnsigned);
        }
    }
    fn visit_word32_shl(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let shl = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(shl.left());
        let mut value: i64 = 0;
        if lhs.is::<Opmask::Word32BitwiseAnd>() &&
            self.match_signed_integral_constant(shl.right(), &mut value) &&
            base::is_in_range(value, 0, 31) {
            let sh = value as i32;
            let mut mb: i32 = 0;
            let mut me: i32 = 0;
            let bitwise_and = lhs.cast::<WordBinopOp>();
            let mut right_value: i64 = 0;
            if self.match_signed_integral_constant(bitwise_and.right(), &mut right_value) &&
                is_contiguous_mask32((right_value << sh) as u32, &mut mb, &mut me) {
                // Adjust the mask such that it doesn't include any rotated bits.
                if me < sh { me = sh; }
                if mb >= me {
                    self.emit(ArchOpcode::PPC_RotLeftAndMask32,
                              g.define_as_register(node), g.use_register(bitwise_and.left()),
                              g.temp_immediate(sh as i64), g.temp_immediate(mb as i64), g.temp_immediate(me as i64));
                    return;
                }
            }
        }
        visit_rro(self, ArchOpcode::PPC_ShiftLeft32, node, ImmediateMode::Shift32Imm);
    }
    fn visit_word64_shl(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let shl = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(shl.left());
        let mut value: i64 = 0;
        if lhs.is::<Opmask::Word64BitwiseAnd>() &&
            self.match_signed_integral_constant(shl.right(), &mut value) &&
            base::is_in_range(value, 0, 63) {
            let sh = value as i32;
            let mut mb: i32 = 0;
            let mut me: i32 = 0;
            let bitwise_and = lhs.cast::<WordBinopOp>();
            let mut right_value: i64 = 0;
            if self.match_signed_integral_constant(bitwise_and.right(), &mut right_value) &&
                is_contiguous_mask64((right_value << sh) as u64, &mut mb, &mut me) {
                // Adjust the mask such that it doesn't include any rotated bits.
                if me < sh { me = sh; }
                if mb >= me {
                    let mut match_ = false;
                    let mut opcode: ArchOpcode = ArchOpcode::PPC_Add32;
                    let mut mask = 0;
                    if me == 0 {
                        match_ = true;
                        opcode = ArchOpcode::PPC_RotLeftAndClearLeft64;
                        mask = mb;
                    } else if mb == 63 {
                        match_ = true;
                        opcode = ArchOpcode::PPC_RotLeftAndClearRight64;
                        mask = me;
                    } else if sh != 0 && me <= sh && lhs.is::<Opmask::Word64ShiftLeft>() {
                        match_ = true;
                        opcode = ArchOpcode::PPC_RotLeftAndClear64;
                        mask = mb;
                    }
                    if match_ {
                        self.emit(opcode, g.define_as_register(node),
                                  g.use_register(bitwise_and.left()), g.temp_immediate(sh as i64),
                                  g.temp_immediate(mask as i64));
                        return;
                    }
                }
            }
        }
        visit_rro(self, ArchOpcode::PPC_ShiftLeft64, node, ImmediateMode::Shift64Imm);
    }

    fn visit_word32_shr(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let shr = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(shr.left());
        let mut value: i64 = 0;
        if lhs.is::<Opmask::Word32BitwiseAnd>() &&
            self.match_signed_integral_constant(shr.right(), &mut value) &&
            base::is_in_range(value, 0, 31) {
            let sh = value as i32;
            let mut mb: i32 = 0;
            let mut me: i32 = 0;
            let bitwise_and = lhs.cast::<WordBinopOp>();
            let mut right_value: u64 = 0;
            if self.match_unsigned_integral_constant(bitwise_and.right(), &mut right_value) &&
                is_contiguous_mask32((right_value >> sh) as u32, &mut mb, &mut me) {
                // Adjust the mask such that it doesn't include any rotated bits.
                if mb > 31 - sh { mb = 31 - sh; }
                sh = (32 - sh) & 0x1F;
                if mb >= me {
                    self.emit(ArchOpcode::PPC_RotLeftAndMask32,
                              g.define_as_register(node), g.use_register(bitwise_and.left()),
                              g.temp_immediate(sh as i64), g.temp_immediate(mb as i64), g.temp_immediate(me as i64));
                    return;
                }
            }
        }
        visit_rro(self, ArchOpcode::PPC_ShiftRight32, node, ImmediateMode::Shift32Imm);
    }
    fn visit_word64_shr(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let shr = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(shr.left());
        let mut value: i64 = 0;
        if lhs.is::<Opmask::Word64BitwiseAnd>() &&
            self.match_signed_integral_constant(shr.right(), &mut value) &&
            base::is_in_range(value, 0, 63) {
            let sh = value as i32;
            let mut mb: i32 = 0;
            let mut me: i32 = 0;
            let bitwise_and = lhs.cast::<WordBinopOp>();
            let mut right_value: u64 = 0;
            if self.match_unsigned_integral_constant(bitwise_and.right(), &mut right_value) &&
                is_contiguous_mask64((right_value >> sh) as u64, &mut mb, &mut me) {
                // Adjust the mask such that it doesn't include any rotated bits.
                if mb > 63 - sh { mb = 63 - sh; }
                sh = (64 - sh) & 0x3F;
                if mb >= me {
                    let mut match_ = false;
                    let mut opcode: ArchOpcode = ArchOpcode::PPC_Add32;
                    let mut mask = 0;
                    if me == 0 {
                        match_ = true;
                        opcode = ArchOpcode::PPC_RotLeftAndClearLeft64;
                        mask = mb;
                    } else if mb == 63 {
                        match_ = true;
                        opcode = ArchOpcode::PPC_RotLeftAndClearRight64;
                        mask = me;
                    }
                    if match_ {
                        self.emit(opcode, g.define_as_register(node),
                                  g.use_register(bitwise_and.left()), g.temp_immediate(sh as i64),
                                  g.temp_immediate(mask as i64));
                        return;
                    }
                }
            }
        }
        visit_rro(self, ArchOpcode::PPC_ShiftRight64, node, ImmediateMode::Shift64Imm);
    }
    fn visit_word32_sar(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        let sar = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(sar.left());
        if self.can_cover(node, sar.left()) && lhs.is::<Opmask::Word32ShiftLeft>() {
            let shl = lhs.cast::<ShiftOp>();
            let mut sar_value: u64 = 0;
            let mut shl_value: u64 = 0;
            if self.match_unsigned_integral_constant(sar.right(), &mut sar_value) &&
                self.match_unsigned_integral_constant(shl.right(), &mut shl_value) {
                let sar_by = sar_value as u32;
                let shl_by = shl_value as u32;
                if (sar_by == shl_by) && (sar_by == 16) {
                    self.emit(ArchOpcode::PPC_ExtendSignWord16, g.define_as_register(node),
                              g.use_register(shl.left()));
                    return;
                } else if (sar_by == shl_by) && (sar_by == 24) {
                    self.emit(ArchOpcode::PPC_ExtendSignWord8, g.define_as_register(node),
                              g.use_register(shl.left()));
                    return;
                }
            }
        }
        visit_rro(self, ArchOpcode::PPC_ShiftRightAlg32, node, ImmediateMode::Shift32Imm);
    }

    fn visit_word64_sar(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        assert!(self.get(node).cast::<ShiftOp>().is_right_shift());
        let shift = self.get(node).cast::<ShiftOp>();
        let lhs = self.get(shift.left());
        let mut constant_rhs: i64 = 0;

        if lhs.is::<LoadOp>() &&
            self.match_integral_word64_constant(shift.right(), &mut constant_rhs) &&
            constant_rhs == 32 && self.can_cover(node, shift.left()) {
            // Just load and sign-extend the interesting 4 bytes instead. This
            // happens, for example, when we're loading and untagging SMIs.
            let load = lhs.cast::<LoadOp>();
            let mut offset: i64 = 0;
            if load.index().is_some() {
                let mut index_constant: i64 = 0;
                if self.match_integral_word64_constant(load.index().unwrap(),
                                                        &mut index_constant) {
                    assert_eq!(load.element_size_log2, 0);
                    offset = index_constant;
                }
            } else {
                offset = load.offset;
            }
            offset = smi_word_offset(offset);
            if g.can_be_immediate_value(offset, ImmediateMode::Int16Imm4ByteAligned) {
                self.emit(ArchOpcode::PPC_LoadWordS32 | AddressingModeField::encode(AddressingMode::kMode_MRI),
                          g.define_as_register(node), g.use_register(load.base()),
                          g.temp_immediate(offset), g.use_immediate(0));
                return;
            }
        }

        visit_rro(self, ArchOpcode::PPC_ShiftRightAlg64, node, ImmediateMode::Shift64Imm);
    }
    fn visit_word32_rol(&mut self, _node: OpIndex) {
        unreachable!()
    }
    fn visit_word64_rol(&mut self, _node: OpIndex) {
        unreachable!()
    }
    fn visit_word32_ror(&mut self, node: OpIndex) {
        visit_rro(self, ArchOpcode::PPC_RotRight32, node, ImmediateMode::Shift32Imm);
    }
    fn visit_word64_ror(&mut self, node: OpIndex) {
        visit_rro(self, ArchOpcode::PPC_RotRight64, node, ImmediateMode::Shift64Imm);
    }
    fn visit_word32_clz(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        self.emit(ArchOpcode::PPC_Cntlz32, g.define_as_register(node),
                  g.use_register(self.input_at(node, 0)));
    }
    fn visit_word64_clz(&mut self, node: OpIndex) {
        let g = PPCOperandGeneratorT::new(self);
        self.emit(