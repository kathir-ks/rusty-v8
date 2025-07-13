// Converted from V8 C++ source files:
// Header: N/A
// Implementation: code-generator-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::rc::Rc;
use std::sync::Arc;

use crate::codegen::constants_loong64::*;
use crate::codegen::interface_descriptors::*;
use crate::codegen::macro_assembler::*;
use crate::codegen::machine_type::*;
use crate::compiler::backend::code_generator::*;
use crate::compiler::backend::code_generator_impl::*;
use crate::compiler::backend::gap_resolver::*;
use crate::compiler::backend::instruction::*;
use crate::compiler::node_matchers::*;
use crate::compiler::osr::*;
use crate::heap::mutable_page_metadata::*;
use crate::codegen::assembler::*;
use crate::codegen::callable::*;
//use crate::compiler::backend::instruction_codes::*;

struct Loong64OperandConverter<'a> {
    gen_: &'a mut CodeGenerator,
    instr_: &'a Instruction,
    instruction_operand_converter: InstructionOperandConverter<'a>,
}

impl<'a> Loong64OperandConverter<'a> {
    pub fn new(gen: &'a mut CodeGenerator, instr: &'a Instruction) -> Self {
        Loong64OperandConverter {
            gen_: gen,
            instr_: instr,
            instruction_operand_converter: InstructionOperandConverter::new(gen, instr),
        }
    }

    fn OutputSingleRegister(&self, index: usize) -> DoubleRegister {
        self.ToSingleRegister(self.instr_.OutputAt(index))
    }

    fn InputSingleRegister(&self, index: usize) -> DoubleRegister {
        self.ToSingleRegister(self.instr_.InputAt(index))
    }

    fn ToSingleRegister(&self, op: &InstructionOperand) -> DoubleRegister {
        // Single (Float) and Double register namespace is same on LOONG64,
        // both are typedefs of FPURegister.
        self.ToDoubleRegister(op)
    }

    fn InputOrZeroRegister(&self, index: usize) -> Register {
        if self.instr_.InputAt(index).IsImmediate() {
            assert_eq!(0, self.InputInt32(index));
            zero_reg
        } else {
            self.InputRegister(index)
        }
    }

    fn InputOrZeroDoubleRegister(&self, index: usize) -> DoubleRegister {
        if self.instr_.InputAt(index).IsImmediate() {
            kDoubleRegZero
        } else {
            self.InputDoubleRegister(index)
        }
    }

    fn InputOrZeroSingleRegister(&self, index: usize) -> DoubleRegister {
        if self.instr_.InputAt(index).IsImmediate() {
            kDoubleRegZero
        } else {
            self.InputSingleRegister(index)
        }
    }

    fn InputImmediate(&self, index: usize) -> Operand {
        let constant = self.ToConstant(self.instr_.InputAt(index));
        match constant.type_ {
            ConstantType::kInt32 => Operand::new_i32(constant.ToInt32()),
            ConstantType::kInt64 => Operand::new_i64(constant.ToInt64()),
            ConstantType::kFloat32 => Operand::EmbeddedNumber(constant.ToFloat32() as f64),
            ConstantType::kFloat64 => Operand::EmbeddedNumber(constant.ToFloat64().value()),
            ConstantType::kCompressedHeapObject => {
                if self.gen_.isolate().roots_table().IsRootHandle(
                    constant.ToHeapObject(),
                ) {
                    assert!(COMPRESS_POINTERS_BOOL);
                    assert!(V8_STATIC_ROOTS_BOOL || !self.gen_.isolate().bootstrapper());
                    let ptr = MacroAssemblerBase::ReadOnlyRootPtr(
                        RootIndex::kFirstRootIndex,
                        self.gen_.isolate(),
                    );
                    Operand::new_i64(ptr as i64)
                } else {
                    Operand::new_object(constant.ToHeapObject())
                }
            }
            ConstantType::kExternalReference => {
              
              Operand::ExternalReference(constant.ToExternalReference())
            }
            ConstantType::kHeapObject => Operand::new_object(constant.ToHeapObject()),
            ConstantType::kRpoNumber => {
                unreachable!("RPO immediates on loong64 not implemented")
            }
        }
    }

    fn InputOperand(&self, index: usize) -> Operand {
        let op = self.instr_.InputAt(index);
        if op.IsRegister() {
            Operand::new_register(self.ToRegister(op))
        } else {
            self.InputImmediate(index)
        }
    }

    fn MemoryOperand(&self, first_index: &mut usize) -> MemOperand {
        let index = *first_index;
        match AddressingModeField::decode(self.instr_.opcode()) {
            AddressingMode::kMode_None => {}
            AddressingMode::kMode_Root => {
                *first_index += 1;
                return MemOperand::new_root(
                    RootIndex::kFirstRootIndex,
                    self.InputInt32(index),
                ); // Assuming kFirstRootIndex is a valid default
            }
            AddressingMode::kMode_MRI => {
                *first_index += 2;
                return MemOperand::new_mri(
                    self.InputRegister(index + 0),
                    self.InputInt32(index + 1),
                );
            }
            AddressingMode::kMode_MRR => {
                *first_index += 2;
                return MemOperand::new_mrr(
                    self.InputRegister(index + 0),
                    self.InputRegister(index + 1),
                );
            }
        }
        unreachable!();
    }

    fn ToMemOperand(&self, op: &InstructionOperand) -> MemOperand {
        assert!(op.IsStackSlot() || op.IsFPStackSlot());
        self.SlotToMemOperand(AllocatedOperand::cast(op).index())
    }

    fn SlotToMemOperand(&self, slot: i32) -> MemOperand {
        let offset = self
            .gen_
            .frame_access_state()
            .GetFrameOffset(slot);
        MemOperand::new(if offset.from_stack_pointer() {
            sp
        } else {
            fp
        }, offset.offset())
    }
}

impl<'a> std::ops::Deref for Loong64OperandConverter<'a> {
  type Target = InstructionOperandConverter<'a>;
  fn deref(&self) -> &Self::Target {
    &self.instruction_operand_converter
  }
}

impl<'a> std::ops::DerefMut for Loong64OperandConverter<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.instruction_operand_converter
  }
}

fn HasRegisterInput(instr: &Instruction, index: usize) -> bool {
    instr.InputAt(index).IsRegister()
}

struct OutOfLineRecordWrite<'a> {
    gen_: &'a mut CodeGenerator,
    object_: Register,
    offset_: Operand,
    value_: Register,
    mode_: RecordWriteMode,
    stub_mode_: StubCallMode,
    must_save_lr_: bool,
    zone_: *mut Zone,
    indirect_pointer_tag_: IndirectPointerTag,
    exit_label_: Label,
    entry_label_: Label,
}

impl<'a> OutOfLineRecordWrite<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        object: Register,
        offset: Operand,
        value: Register,
        mode: RecordWriteMode,
        stub_mode: StubCallMode,
        indirect_pointer_tag: IndirectPointerTag,
    ) -> Self {
        OutOfLineRecordWrite {
            gen_: gen,
            object_: object,
            offset_: offset,
            value_: value,
            mode_: mode,
            stub_mode_: stub_mode,
            must_save_lr_: !gen.frame_access_state().has_frame(),
            zone_: gen.zone(),
            indirect_pointer_tag_: indirect_pointer_tag,
            exit_label_: Label::new(),
            entry_label_: Label::new(),
        }
    }

    fn Generate(&mut self) {
        let masm = self.gen_.masm();
        let frame = self.gen_.frame();

        if COMPRESS_POINTERS_BOOL && self.mode_ != RecordWriteMode::kValueIsIndirectPointer {
           
          masm.DecompressTagged(self.value_, self.value_);
        }

        masm.CheckPageFlag(
            self.value_,
            MemoryChunk::kPointersToHereAreInterestingMask,
            Condition::eq,
            &self.exit_label_,
        );

        let save_fp_mode = if frame.DidAllocateDoubleRegisters() {
            SaveFPRegsMode::kSave
        } else {
            SaveFPRegsMode::kIgnore
        };

        if self.must_save_lr_ {
           
          masm.Push(ra);
        }

        if self.mode_ == RecordWriteMode::kValueIsEphemeronKey {
           
          masm.CallEphemeronKeyBarrier(
              self.object_,
              self.offset_,
              save_fp_mode,
          );
        } else if self.mode_ == RecordWriteMode::kValueIsIndirectPointer {
            assert!(IsValidIndirectPointerTag(self.indirect_pointer_tag_));
          masm.CallIndirectPointerBarrier(
                self.object_,
                self.offset_,
                save_fp_mode,
                self.indirect_pointer_tag_,
            );
        } else {
          masm.CallRecordWriteStubSaveRegisters(
                self.object_,
                self.offset_,
                save_fp_mode,
            );
        }

        if self.must_save_lr_ {
           
          masm.Pop(ra);
        }
       
    }

    fn entry(&self) -> &Label {
      &self.entry_label_
    }

    fn exit(&self) -> &Label {
      &self.exit_label_
    }
}

trait OutOfLineCodeTrait {
    fn Generate(&mut self);
}

struct OutOfLineFloat32Max<'a> {
    gen_: &'a mut CodeGenerator,
    dst_: DoubleRegister,
    src1_: DoubleRegister,
    src2_: DoubleRegister,
}

impl<'a> OutOfLineFloat32Max<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        dst: DoubleRegister,
        src1: DoubleRegister,
        src2: DoubleRegister,
    ) -> Self {
        OutOfLineFloat32Max {
            gen_: gen,
            dst_: dst,
            src1_: src1,
            src2_: src2,
        }
    }
}

impl<'a> OutOfLineCodeTrait for OutOfLineFloat32Max<'a> {
    fn Generate(&mut self) {
        self.gen_.masm().Float32MaxOutOfLine(self.dst_, self.src1_, self.src2_);
    }
}

struct OutOfLineFloat32Min<'a> {
    gen_: &'a mut CodeGenerator,
    dst_: DoubleRegister,
    src1_: DoubleRegister,
    src2_: DoubleRegister,
}

impl<'a> OutOfLineFloat32Min<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        dst: DoubleRegister,
        src1: DoubleRegister,
        src2: DoubleRegister,
    ) -> Self {
        OutOfLineFloat32Min {
            gen_: gen,
            dst_: dst,
            src1_: src1,
            src2_: src2,
        }
    }
}

impl<'a> OutOfLineCodeTrait for OutOfLineFloat32Min<'a> {
    fn Generate(&mut self) {
        self.gen_.masm().Float32MinOutOfLine(self.dst_, self.src1_, self.src2_);
    }
}

struct OutOfLineFloat64Max<'a> {
    gen_: &'a mut CodeGenerator,
    dst_: DoubleRegister,
    src1_: DoubleRegister,
    src2_: DoubleRegister,
}

impl<'a> OutOfLineFloat64Max<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        dst: DoubleRegister,
        src1: DoubleRegister,
        src2: DoubleRegister,
    ) -> Self {
        OutOfLineFloat64Max {
            gen_: gen,
            dst_: dst,
            src1_: src1,
            src2_: src2,
        }
    }
}

impl<'a> OutOfLineCodeTrait for OutOfLineFloat64Max<'a> {
    fn Generate(&mut self) {
        self.gen_.masm().Float64MaxOutOfLine(self.dst_, self.src1_, self.src2_);
    }
}

struct OutOfLineFloat64Min<'a> {
    gen_: &'a mut CodeGenerator,
    dst_: DoubleRegister,
    src1_: DoubleRegister,
    src2_: DoubleRegister,
}

impl<'a> OutOfLineFloat64Min<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        dst: DoubleRegister,
        src1: DoubleRegister,
        src2: DoubleRegister,
    ) -> Self {
        OutOfLineFloat64Min {
            gen_: gen,
            dst_: dst,
            src1_: src1,
            src2_: src2,
        }
    }
}

impl<'a> OutOfLineCodeTrait for OutOfLineFloat64Min<'a> {
    fn Generate(&mut self) {
        self.gen_.masm().Float64MinOutOfLine(self.dst_, self.src1_, self.src2_);
    }
}

struct WasmOutOfLineTrap<'a> {
  gen_: &'a mut CodeGenerator,
  instr_: &'a Instruction,
}

impl<'a> WasmOutOfLineTrap<'a> {
    fn new(
        gen: &'a mut CodeGenerator,
        instr: &'a Instruction,
    ) -> Self {
      WasmOutOfLineTrap {
          gen_: gen,
          instr_: instr,
        }
    }
  
    fn GenerateCallToTrap(&mut self, trap_id: TrapId) {
        self.gen_.AssembleSourcePosition(self.instr_);
        let masm = self.gen_.masm();
        // A direct call to a wasm runtime stub defined in this module.
        // Just encode the stub index. This will be patched when the code
        // is added to the native module and copied into wasm code space.
        masm.Call(trap_id as usize, RelocInfo::WASM_STUB_CALL);
        let reference_map = unsafe {
          let zone = self.gen_.zone();
          (*zone).NewReferenceMap(zone)
        };
        self.gen_.RecordSafepoint(reference_map);
        masm.AssertUnreachable(AbortReason::kUnexpectedReturnFromWasmTrap);
    }

    fn GenerateWithTrapId(&mut self, trap_id: TrapId) {
      self.GenerateCallToTrap(trap_id);
    }
}

impl<'a> OutOfLineCodeTrait for WasmOutOfLineTrap<'a> {
    fn Generate(&mut self) {
        let i = Loong64OperandConverter::new(self.gen_, self.instr_);
        let trap_id = self.InputInt32(self.instr_.InputCount() - 1) as TrapId;
        self.GenerateCallToTrap(trap_id);
    }
}

fn RecordTrapInfoIfNeeded(
    zone: *mut Zone,
    codegen: &mut CodeGenerator,
    opcode: InstructionCode,
    instr: &Instruction,
    pc: i32,
) {
  let access_mode = AccessModeField::decode(opcode);
  if access_mode == AccessMode::kMemoryAccessProtectedMemOutOfBounds ||
    access_mode == AccessMode::kMemoryAccessProtectedNullDereference {
    let reference_map = unsafe {
      (*codegen.zone()).NewReferenceMap(zone)
    };

    // The safepoint has to be recorded at the return address of a call. Address
    // we use as the fake return address in the case of the trap handler is the
    // fault address (here `pc`) + 1. Therefore the safepoint here has to be
    // recorded at pc + 1;
    codegen.RecordSafepoint(reference_map, pc + 1);
    codegen.RecordProtectedInstruction(pc);
  } else {
    assert_eq!(access_mode, AccessMode::kMemoryAccessDirect);
  }
}

fn FlagsConditionToConditionCmp(condition: FlagsCondition) -> Condition {
    match condition {
        FlagsCondition::kEqual => Condition::eq,
        FlagsCondition::kNotEqual => Condition::ne,
        FlagsCondition::kSignedLessThan => Condition::lt,
        FlagsCondition::kSignedGreaterThanOrEqual => Condition::ge,
        FlagsCondition::kSignedLessThanOrEqual => Condition::le,
        FlagsCondition::kSignedGreaterThan => Condition::gt,
        FlagsCondition::kUnsignedLessThan => Condition::lo,
        FlagsCondition::kUnsignedGreaterThanOrEqual => Condition::hs,
        FlagsCondition::kUnsignedLessThanOrEqual => Condition::ls,
        FlagsCondition::kUnsignedGreaterThan => Condition::hi,
        _ => unreachable!(),
    }
}

fn FlagsConditionToConditionTst(condition: FlagsCondition) -> Condition {
    match condition {
        FlagsCondition::kNotEqual => Condition::ne,
        FlagsCondition::kEqual => Condition::eq,
        _ => unreachable!(),
    }
}

fn FlagsConditionToConditionOvf(condition: FlagsCondition) -> Condition {
  match condition {
      FlagsCondition::kOverflow => Condition::ne,
      FlagsCondition::kNotOverflow => Condition::eq,
      _ => unreachable!(),
  }
}

fn FlagsConditionToConditionCmpFPU(predicate: &mut bool, condition: FlagsCondition) -> FPUCondition {
    match condition {
        FlagsCondition::kEqual => {
            *predicate = true;
            FPUCondition::CEQ
        }
        FlagsCondition::kNotEqual => {
            *predicate = false;
            FPUCondition::CEQ
        }
        FlagsCondition::kUnsignedLessThan | FlagsCondition::kFloatLessThan => {
            *predicate = true;
            FPUCondition::CLT
        }
        FlagsCondition::kUnsignedGreaterThanOrEqual => {
            *predicate = false;
            FPUCondition::CLT
        }
        FlagsCondition::kUnsignedLessThanOrEqual | FlagsCondition::kFloatLessThanOrEqual => {
            *predicate = true;
            FPUCondition::CLE
        }
        FlagsCondition::kUnsignedGreaterThan => {
            *predicate = false;
            FPUCondition::CLE
        }
        FlagsCondition::kFloatGreaterThan => {
            *predicate = false;
            FPUCondition::CULE
        }
        FlagsCondition::kFloatGreaterThanOrEqual => {
            *predicate = false;
            FPUCondition::CULT
        }
        FlagsCondition::kFloatLessThanOrUnordered => {
            *predicate = true;
            FPUCondition::CULT
        }
        FlagsCondition::kFloatGreaterThanOrUnordered => {
            *predicate = false;
            FPUCondition::CLE
        }
        FlagsCondition::kFloatGreaterThanOrEqualOrUnordered => {
            *predicate = false;
            FPUCondition::CLT
        }
        FlagsCondition::kFloatLessThanOrEqualOrUnordered => {
            *predicate = true;
            FPUCondition::CULE
        }
        FlagsCondition::kUnorderedEqual | FlagsCondition::kUnorderedNotEqual => {
            *predicate = true;
            FPUCondition::CUN
        }
        _ => unreachable!(),
    }
}

impl CodeGenerator {
    fn AssembleDeconstructFrame(&mut self) {
        self.masm().mov(sp, fp);
        self.masm().Pop(ra, fp);
    }

    fn AssemblePrepareTailCall(&mut self) {
        if self.frame_access_state().has_frame() {
            self.masm().Ld_d(ra, MemOperand::new(fp, StandardFrameConstants::kCallerPCOffset));
            self.masm().Ld_d(fp, MemOperand::new(fp, StandardFrameConstants::kCallerFPOffset));
        }
        self.frame_access_state().SetFrameAccessToSP();
    }

    fn AssembleCodeStartRegisterCheck(&mut self) {
        let mut temps = UseScratchRegisterScope::new(self.masm());
        let scratch = temps.Acquire();
        self.masm().ComputeCodeStartAddress(scratch);
        self.masm().Assert(
            Condition::eq,
            AbortReason::kWrongFunctionCodeStart,
            kJavaScriptCallCodeStartRegister,
            Operand::new_register(scratch),
        );
    }

    fn AssembleReturn(&mut self, additional_pop_count: &InstructionOperand) {
      let call_descriptor = self.linkage().GetIncomingDescriptor();
      let returns = self.frame().GetReturnSlotCount();
      if returns != 0 {
        self.masm().Add_d(sp, sp, Operand::new_i32(returns * kSystemPointerSize));
      }
  
      // Restore GP registers.
      let saves = call_descriptor.CalleeSavedRegisters();
      if !saves.is_empty() {
          self.masm().MultiPop(saves);
      }
  
      // Restore FPU registers.
      let saves_fpu = call_descriptor.CalleeSavedFPRegisters();
      if !saves_fpu.is_empty() {
          self.masm().MultiPopFPU(saves_fpu);
      }
  
      let g = Loong64OperandConverter::new(self,  unsafe {std::mem::transmute(std::ptr::null_mut())});
  
      let parameter_slots = call_descriptor.ParameterSlotCount() as i32;
  
      // {aditional_pop_count} is only greater than zero if {parameter_slots = 0}.
      // Check RawMachineAssembler::PopAndReturn.
      if parameter_slots != 0 {
          if additional_pop_count.IsImmediate() {
              assert_eq!(g.ToConstant(additional_pop_count).ToInt32(), 0);
          } else if v8_flags.debug_code {
            /*  self.masm().Assert(
                  Condition::eq,
                  AbortReason::kUnexpectedAdditionalPopValue,
                  g.ToRegister(additional_pop_count),
                  Operand::new_i32(0),
              );*/
          }
      }
  
      // Functions with JS linkage have at least one parameter (the receiver).
      // If {parameter_slots} == 0, it means it is a builtin with
      // kDontAdaptArgumentsSentinel, which takes care of JS arguments popping
      // itself.
      let drop_jsargs = self.frame_access_state().has_frame()
          && call_descriptor.IsJSFunctionCall()
          && parameter_slots != 0;
  
      if call_descriptor.IsCFunctionCall() {
        self.AssembleDeconstructFrame();
      } else if self.frame_access_state().has_frame() {
          // Canonicalize JSFunction return sites for now unless they have an variable
          // number of stack slot pops.
          /*if additional_pop_count.IsImmediate() &&
              g.ToConstant(additional_pop_count).ToInt32() == 0 {
              if self.return_label_.is_bound() {
                  self.masm().Branch(&self.return_label_);
                  return;
              } else {
                  self.masm().bind(&self.return_label_);
              }
          }*/
          if drop_jsargs {
              // Get the actual argument count
              self.masm().Ld_d(t0, MemOperand::new(fp, StandardFrameConstants::kArgCOffset));
          }
          self.AssembleDeconstructFrame();
      }
  
      if drop_jsargs {
          // We must pop all arguments from the stack (including the receiver). This
          // number of arguments is given by max(1 + argc_reg, parameter_count).
          if parameter_slots > 1 {
              self.masm().li(t1, parameter_slots);
              self.masm().slt(t2, t0, t1);
              self.masm().Movn(t0, t1, t2);
          }
          self.masm().Alsl_d(sp, t0, sp, kSystemPointerSizeLog2);
      } else if additional_pop_count.IsImmediate() {
          let additional_count = g.ToConstant(additional_pop_count).ToInt32();
          self.masm().Drop(parameter_slots + additional_count);
      } else {
          let pop_reg = g.ToRegister(additional_pop_count);
          self.masm().Drop(parameter_slots);
          self.masm().Alsl_d(sp, pop_reg, sp, kSystemPointerSizeLog2);
      }
  
      self.masm().Ret();
    }
}

