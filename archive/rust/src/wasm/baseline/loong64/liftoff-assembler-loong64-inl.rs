// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NOTE: This translation is incomplete due to the size and complexity
// of the original C++ code. Some parts, especially SIMD-related code,
// are stubbed out with bailout calls.  Error handling is simplified.

// #![allow(dead_code)] // Allow unused code during development

// use crate::codegen::interface_descriptors::*;
// use crate::codegen::loong64::*;
// use crate::compiler::linkage::*;
// use crate::heap::mutable_page_metadata::*;
// use crate::wasm::baseline::liftoff_assembler::*;
// use crate::wasm::baseline::parallel_move::*;
// use crate::wasm::object_access::*;
// use crate::wasm::wasm_linkage::*;
// use crate::wasm::wasm_objects::*;
use std::marker::PhantomData;

// Placeholder for architecture-specific constants
const kSystemPointerSize: i32 = 8;
const kStackSlotSize: i32 = 8;
const kInt32Size: i32 = 4;
const kInstrSize: i32 = 4;

macro_rules! is_int31 {
    ($x:expr) => {
        $x >= i32::MIN as i64 && $x <= i32::MAX as i64
    };
}

// Placeholder enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueKind {
    I16,
    I32,
    I64,
    F32,
    F64,
    Ref,
    RefNull,
    S128,
}

fn value_kind_size(kind: ValueKind) -> i32 {
    match kind {
        ValueKind::I16 => 2,
        ValueKind::I32 => 4,
        ValueKind::I64 => 8,
        ValueKind::F32 => 4,
        ValueKind::F64 => 8,
        ValueKind::Ref => 8,
        ValueKind::RefNull => 8,
        ValueKind::S128 => 16,
    }
}

fn is_reference(kind: ValueKind) -> bool {
    match kind {
        ValueKind::Ref | ValueKind::RefNull => true,
        _ => false,
    }
}

// Placeholder struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MemOperand {
    base: Register,
    offset: i32,
}

impl MemOperand {
    fn new(base: Register, offset: i32) -> Self {
        MemOperand { base, offset }
    }
}

// Placeholder enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    Zero,
    A0,
    A1,
    FP,
    SP,
    RA,
    KScratchReg,
    KScratchReg2,
    KLiftoffFrameSetupFunctionReg,
    ReturnRegister0,
    NoReg,
    // Add other registers as needed
}

const zero_reg: Register = Register::Zero;
const a0: Register = Register::A0;
const a1: Register = Register::A1;
const fp: Register = Register::FP;
const sp: Register = Register::SP;
const ra: Register = Register::RA;
const kScratchReg: Register = Register::KScratchReg;
const kScratchReg2: Register = Register::KScratchReg2;

impl Register {
    fn is_valid(&self) -> bool {
        *self != Register::NoReg
    }
}

// Placeholder enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DoubleRegister {
    KScratchDoubleReg,
    KScratchDoubleReg2,
}

// Placeholder struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LiftoffRegister {
    reg: Register,
    is_gp: bool,
}

impl LiftoffRegister {
    fn gp(&self) -> Register {
        if self.is_gp {
            self.reg
        } else {
            panic!("Expected GP register, but got FP register");
        }
    }

    fn fp(&self) -> DoubleRegister {
        if !self.is_gp {
            match self.reg {
                Register::A0 => DoubleRegister::KScratchDoubleReg, // Assuming A0 maps to F0
                Register::A1 => DoubleRegister::KScratchDoubleReg2, // Assuming A1 maps to F1
                _ => panic!("Invalid register for FP: {:?}", self.reg),
            }
        } else {
            panic!("Expected FP register, but got GP register");
        }
    }
}

// Placeholder trait
trait AssemblerInterface {
    fn pc_offset(&self) -> i32;
    fn Add_d(&mut self, dst: Register, src: Register, operand: Operand);
    fn St_d(&mut self, src: Register, dst: MemOperand);
    fn Push(&mut self, reg: Register);
    fn Pop(&mut self, reg1: Register, reg2: Register);
    fn li(&mut self, dst: Register, operand: Operand);
}

// Placeholder struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operand {
    imm: i64,
    is_reg: bool,
    reg: Register,
}

impl Operand {
    fn new(imm: i64) -> Self {
        Operand { imm, is_reg: false, reg: Register::NoReg }
    }

    fn rm(&self) -> Register {
        self.reg
    }

    fn is_reg(&self) -> bool {
        self.is_reg
    }
}

impl From<Register> for Operand {
    fn from(reg: Register) -> Self {
        Operand { imm: 0, is_reg: true, reg }
    }
}

impl From<i32> for Operand {
    fn from(imm: i32) -> Self {
        Operand::new(imm as i64)
    }
}

impl From<i64> for Operand {
    fn from(imm: i64) -> Self {
        Operand::new(imm)
    }
}

// Placeholder struct
struct WasmValue {
    value: i32, // Placeholder
}

impl WasmValue {
    fn new(value: i32) -> Self {
        WasmValue { value }
    }
    fn to_i32(&self) -> i32 {
        self.value
    }
    fn to_i64(&self) -> i64 {
        self.value as i64
    }
    fn type_(&self) -> WasmType {
        WasmType { kind: ValueKind::I32 } // Assuming default type for WasmValue
    }
}

// Placeholder struct
struct WasmType {
    kind: ValueKind, // Placeholder
}

// Placeholder enum
enum Builtin {
    kWasmLiftoffFrameSetup,
    kWasmStackOverflow,
    kWasmHandleStackOverflow
}

// Placeholder struct
struct FreezeCacheState {
    // Placeholder
}

impl FreezeCacheState {
    fn new() -> Self {
        FreezeCacheState {}
    }
}

// Placeholder enum
enum StackFrame {
    WASM,
    WASM_SEGMENT_START,
}

impl StackFrame {
    fn TypeToMarker(frame_type: StackFrame) -> i32 {
        match frame_type {
            StackFrame::WASM => 1,
            StackFrame::WASM_SEGMENT_START => 2,
        }
    }
}

// Placeholder struct
struct SafepointTableBuilder {
    // Placeholder
}

impl SafepointTableBuilder {
    fn new() -> Self {
        SafepointTableBuilder {}
    }
    fn DefineSafepoint(&mut self, assembler: &LiftoffAssembler) {
        // Placeholder
    }
}

// Placeholder struct
struct UseScratchRegisterScope<'a> {
    assembler: &'a mut LiftoffAssembler,
    acquired: bool,
    phantom: PhantomData<&'a mut LiftoffAssembler>, // To hold the lifetime
}

impl<'a> UseScratchRegisterScope<'a> {
    fn new(assembler: &'a mut LiftoffAssembler) -> Self {
        UseScratchRegisterScope { assembler, acquired: false, phantom: PhantomData }
    }

    fn Acquire(&mut self) -> Register {
        self.acquired = true;
        Register::KScratchReg // Simplified: Always return the same scratch register
    }
}

impl<'a> Drop for UseScratchRegisterScope<'a> {
    fn drop(&mut self) {
        // In a real implementation, you would release the register here.
        // This simplified version does nothing.
    }
}

// Placeholder constants
mod WasmLiftoffFrameConstants {
    pub const kInstanceDataOffset: i32 = -2 * 8;
    pub const kFeedbackVectorOffset: i32 = -3 * 8;
}

// Placeholder constants
mod CommonFrameConstants {
    pub const kFixedFrameSizeAboveFp: i32 = 16; // Example value, adjust as needed
}

// Placeholder constants
mod TypedFrameConstants {
    pub const kFrameTypeOffset: i32 = 8;
}

// Placeholder struct
struct LiftoffRegList {
    registers: [bool; 32], // Assuming a maximum of 32 registers
}

impl LiftoffRegList {
    fn new() -> Self {
        LiftoffRegList { registers: [false; 32] }
    }

    fn set(&mut self, reg: Register) -> Register {
        // Placeholder logic for setting a register as used.
        // In a real implementation, this would set the appropriate bit
        // corresponding to the register.
        reg
    }
}

// Placeholder enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Condition {
    Equal,
    NotEqual,
    UnsignedLessThan,
    UnsignedGreaterThanEqual,
    UnsignedLessThanEqual,
    UnsignedGreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
}

// Placeholder enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FPUCondition {
    CEQ,
    CLT,
    CLE,
}

// Placeholder enum
enum RegClass {
    kGpReg,
    kFpReg,
}

// Placeholder enum
enum LoadType {
    kI32Load8U,
    kI64Load8U,
    kI32Load8S,
    kI64Load8S,
    kI32Load16U,
    kI64Load16U,
    kI32Load16S,
    kI64Load16S,
    kI64Load32U,
    kI32Load,
    kI64Load32S,
    kI64Load,
    kF32Load,
    kF32LoadF16,
    kF64Load,
    kS128Load,
}

impl LoadType {
    fn value(&self) -> i32 {
        match self {
            LoadType::kI32Load8U => 0,
            LoadType::kI64Load8U => 1,
            LoadType::kI32Load8S => 2,
            LoadType::kI64Load8S => 3,
            LoadType::kI32Load16U => 4,
            LoadType::kI64Load16U => 5,
            LoadType::kI32Load16S => 6,
            LoadType::kI64Load16S => 7,
            LoadType::kI64Load32U => 8,
            LoadType::kI32Load => 9,
            LoadType::kI64Load32S => 10,
            LoadType::kI64Load => 11,
            LoadType::kF32Load => 12,
            LoadType::kF32LoadF16 => 13,
            LoadType::kF64Load => 14,
            LoadType::kS128Load => 15,
        }
    }

    fn size_log_2(&self) -> u32 {
        match self {
            LoadType::kI32Load8U | LoadType::kI64Load8U | LoadType::kI32Load8S | LoadType::kI64Load8S => 0,
            LoadType::kI32Load16U | LoadType::kI64Load16U | LoadType::kI32Load16S | LoadType::kI64Load16S => 1,
            LoadType::kI64Load32U | LoadType::kI32Load | LoadType::kI64Load32S => 2,
            LoadType::kI64Load => 3,
            LoadType::kF32Load => 2,
            LoadType::kF32LoadF16 => 1,
            LoadType::kF64Load => 3,
            LoadType::kS128Load => 4,
        }
    }
}

// Placeholder enum
enum StoreType {
    kI32Store8,
    kI64Store8,
    kI32Store16,
    kI64Store16,
    kI32Store,
    kI64Store32,
    kI64Store,
    kF32Store,
    kF32StoreF16,
    kF64Store,
    kS128Store,
}

impl StoreType {
    fn value(&self) -> i32 {
        match self {
            StoreType::kI32Store8 => 0,
            StoreType::kI64Store8 => 1,
            StoreType::kI32Store16 => 2,
            StoreType::kI64Store16 => 3,
            StoreType::kI32Store => 4,
            StoreType::kI64Store32 => 5,
            StoreType::kI64Store => 6,
            StoreType::kF32Store => 7,
            StoreType::kF32StoreF16 => 8,
            StoreType::kF64Store => 9,
            StoreType::kS128Store => 10,
        }
    }
}

// Placeholder enum
enum IndirectPointerTag {}

// Placeholder enum
enum LoadTransformationKind {}

// Placeholder enum
enum SmiCheckMode {
    kJumpOnSmi,
    kJumpOnNotSmi,
}

// Placeholder enum
enum RegPairHalf {}

// Placeholder enum
enum SkipWriteBarrier {
    kDoWriteBarrier,
    kSkipWriteBarrier,
}

// Placeholder enum
enum SaveFPRegsMode {
    kSave,
    kDontSave,
}

// Placeholder enum
enum StubCallMode {
    kCallWasmRuntimeStub
}

// Placeholder enum
enum StackLimitKind {
    kRealStackLimit
}

// Placeholder struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VarState {
    kind: ValueKind,
    location: VarStateLocation,
    offset: i32,
    i32_const: i32,
    // Add other fields as needed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarStateLocation {
    Reg,
    Stack,
    Const,
}

impl VarState {
    fn is_reg(&self) -> bool {
        self.location == VarStateLocation::Reg
    }

    fn is_stack(&self) -> bool {
        self.location == VarStateLocation::Stack
    }

    fn is_const(&self) -> bool {
        self.location == VarStateLocation::Const
    }

    fn reg(&self) -> LiftoffRegister {
        LiftoffRegister { reg: Register::A0, is_gp: true }
    }

    fn i32_const(&self) -> i32 {
        self.i32_const
    }

    fn offset(&self) -> i32 {
        self.offset
    }

    fn kind(&self) -> ValueKind {
        self.kind
    }
}

// Placeholder struct for compressed pointers.  Assuming this flag is disabled
const COMPRESS_POINTERS_BOOL: bool = false;

// Placeholder struct
struct BlockTrampolinePoolScope<'a> {
    assembler: &'a mut LiftoffAssembler,
}

impl<'a> BlockTrampolinePoolScope<'a> {
    fn new(assembler: &'a mut LiftoffAssembler) -> Self {
        BlockTrampolinePoolScope { assembler }
    }
}

impl<'a> Drop for BlockTrampolinePoolScope<'a> {
    fn drop(&mut self) {
        // Placeholder logic for BlockTrampolinePoolScope destruction
    }
}

// Begin translation of LiftoffAssembler

/// A struct for the Liftoff assembler.
pub struct LiftoffAssembler {
    buffer_start_: *mut u8,
    buffer_cursor_: *mut u8,
    buffer_end_: *mut u8,
    feedback_vector_slot: bool,
    total_frame_size: i32,
    used_spill_offsets: Vec<i32>,
    cache_state_: AssemblerCacheState,
    // Add other necessary fields here
}

struct AssemblerCacheState {
    cached_instance_data: Register,
}

impl LiftoffAssembler {
    /// Creates a new `LiftoffAssembler`.
    pub fn new(buffer_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(buffer_size);
        let buffer_start_ = buffer.as_mut_ptr();
        let buffer_cursor_ = buffer_start_;
        let buffer_end_ = unsafe { buffer_start_.add(buffer_size) };
        std::mem::forget(buffer); // Prevent deallocation
        LiftoffAssembler {
            buffer_start_: buffer_start_,
            buffer_cursor_: buffer_cursor_,
            buffer_end_: buffer_end_,
            feedback_vector_slot: false,
            total_frame_size: 0,
            used_spill_offsets: Vec::new(),
            cache_state_: AssemblerCacheState { cached_instance_data: Register::NoReg },
        }
    }

    fn cache_state(&mut self) -> &mut AssemblerCacheState {
        &mut self.cache_state_
    }

    /// Prepares the stack frame.
    pub fn PrepareStackFrame(&mut self) -> i32 {
        let offset = self.pc_offset();
        // When constant that represents size of stack frame can't be represented
        // as 16bit we need three instructions to add it to sp, so we reserve space
        // for this case.
        self.addi_d(sp, sp, 0);
        self.nop();
        self.nop();
        offset
    }

    /// Sets up the call frame stub.
    pub fn CallFrameSetupStub(&mut self, declared_function_index: i32) {
        // On LOONG64, we must push at least {ra} before calling the stub, otherwise
        // it would get clobbered with no possibility to recover it. So just set
        // up the frame here.
        self.EnterFrame(StackFrame::WASM);
        self.LoadConstant(LiftoffRegister { reg: Register::KLiftoffFrameSetupFunctionReg, is_gp: true }, WasmValue::new(declared_function_index));
        self.CallBuiltin(Builtin::kWasmLiftoffFrameSetup);
    }

    /// Prepares for a tail call.
    pub fn PrepareTailCall(&mut self, num_callee_stack_params: i32, stack_param_delta: i32) {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();

        // Push the return address and frame pointer to complete the stack frame.
        self.Ld_d(scratch, MemOperand::new(fp, 8));
        self.Push(scratch);
        self.Ld_d(scratch, MemOperand::new(fp, 0));
        self.Push(scratch);

        // Shift the whole frame upwards.
        let slot_count = num_callee_stack_params + 2;
        for i in (0..slot_count).rev() {
            self.Ld_d(scratch, MemOperand::new(sp, i * 8));
            self.St_d(scratch, MemOperand::new(fp, (i - stack_param_delta) * 8));
        }

        // Set the new stack and frame pointer.
        self.addi_d(sp, fp, -stack_param_delta * 8);
        self.Pop(ra, fp);
    }

    /// Aligns the frame size.
    pub fn AlignFrameSize(&mut self) {}

    /// Patches the stack frame preparation.
    pub fn PatchPrepareStackFrame(
        &mut self,
        offset: i32,
        safepoint_table_builder: &mut SafepointTableBuilder,
        feedback_vector_slot: bool,
        stack_param_slots: usize,
    ) {
        // The frame_size includes the frame marker and the instance slot. Both are
        // pushed as part of frame construction, so we don't need to allocate memory
        // for them anymore.
        let mut frame_size = self.GetTotalFrameSize() - 2 * kSystemPointerSize;
        // The frame setup builtin also pushes the feedback vector.
        if feedback_vector_slot {
            frame_size -= kSystemPointerSize;
        }

        if frame_size < 4 * 1024 {
            // This is the standard case for small frames: just subtract from SP and be
            // done with it.
            self.Add_d(sp, sp, Operand::new(-(frame_size as i64)));
            return;
        }

        // The frame size is bigger than 4KB, so we might overflow the available stack
        // space if we first allocate the frame and then do the stack check (we will
        // need some remaining stack space for throwing the exception). That's why we
        // check the available stack space before we allocate the frame. To do this we
        // replace the {__ Add_d(sp, sp, -frame_size)} with a jump to OOL code that
        // does this "extended stack check".
        //
        // The OOL code can simply be generated here with the normal assembler,
        // because all other code generation, including OOL code, has already finished
        // when {PatchPrepareStackFrame} is called. The function prologue then jumps
        // to the current {pc_offset()} to execute the OOL code for allocating the
        // large frame.
        // Emit the unconditional branch in the function prologue (from {offset} to
        // {pc_offset()}).

        let imm32 = self.pc_offset() - offset;
        assert!(is_int26(imm32 as i64));
        self.b(imm32 >> 2);

        // If the frame is bigger than the stack, we throw the stack overflow
        // exception unconditionally. Thereby we can avoid the integer overflow
        // check in the condition code.
        // RecordComment("OOL: stack check for large frame");
        let mut continuation = Label::new();
        if true { // v8_flags.stack_size * 1024
            let stack_limit = kScratchReg;
            self.LoadStackLimit(stack_limit, StackLimitKind::kRealStackLimit);
            self.Add_d(stack_limit, stack_limit, Operand::new(frame_size as i64));
            self.Branch(&mut continuation, Condition::UnsignedGreaterThanEqual, sp, Operand::new(stack_limit as i64));
        }

        if false { // v8_flags.experimental_wasm_growable_stacks
             // Placeholder logic for growable stacks
        } else {
            self.Call(Builtin::kWasmStackOverflow);
            // The call will not return; just define an empty safepoint.
            safepoint_table_builder.DefineSafepoint(self);
            if true { // v8_flags.debug_code
               // self.stop();  // Assuming 'stop' is a function to halt execution.
            }
        }

        self.bind(&mut continuation);

        // Now allocate the stack space. Note that this might do more than just
        // decrementing the SP;
        self.Add_d(sp, sp, Operand::new(-(frame_size as i64)));

        // Jump back to the start of the function, from {pc_offset()} to
        // right after the reserved space for the {__ Add_d(sp, sp, -framesize)}
        // (which is a Branch now).
        let func_start_offset = offset + 3 * kInstrSize;
        let imm32 = func_start_offset - self.pc_offset();
        assert!(is_int26(imm32 as i64));
        self.b(imm32 >> 2);
    }

    /// Finishes code generation.
    pub fn FinishCode(&mut self) {}

    /// Aborts compilation.
    pub fn AbortCompilation(&mut self) {}

    /// Gets the static stack frame size.
    pub fn StaticStackFrameSize() -> i32 {
        WasmLiftoffFrameConstants::kFeedbackVectorOffset
    }

    /// Gets the slot size for a given value kind.
    pub fn SlotSizeForType(kind: ValueKind) -> i32 {
        match kind {
            ValueKind::S128 => value_kind_size(kind),
            _ => kStackSlotSize,
        }
    }

    /// Checks if a given value kind needs alignment.
    pub fn NeedsAlignment(kind: ValueKind) -> bool {
        kind == ValueKind::S128 || is_reference(kind)
    }

    /// Checks if tier-up is needed.
    pub fn CheckTierUp(
        &mut self,
        declared_func_index: i32,
        budget_used: i32,
        ool_label: &mut Label,
        frozen: &FreezeCacheState,
    ) {
        let budget_array = kScratchReg;

        let mut instance_data = self.cache_state_.cached_instance_data;
        if instance_data == Register::NoReg {
            instance_data = budget_array; // Reuse the scratch register.
            self.LoadInstanceDataFromFrame(instance_data);
        }

        let kArrayOffset: i32 = 0; // Assuming this value for now, as crate::wasm::ObjectAccess::ToTagged() is not converted.

        self.Ld_d(budget_array, MemOperand::new(instance_data, kArrayOffset));

        let budget_arr_offset = kInt32Size * declared_func_index;

        let budget = kScratchReg2;
        let budget_addr = MemOperand::new(budget_array, budget_arr_offset);
        self.Ld_w(budget, budget_addr);
        self.Sub_w(budget, budget, budget_used);
        self.St_w(budget, budget_addr);

        self.Branch(ool_label, Condition::LessThan, budget, Operand::new(zero_reg as i64));
    }

    /// Loads the old frame pointer.
    pub fn LoadOldFramePointer(&mut self) -> Register {
        if !false { // v8_flags.experimental_wasm_growable_stacks
            return fp;
        }

        let old_fp = self.GetUnusedRegister(RegClass::kGpReg, LiftoffRegList::new());
        let mut done = Label::new();
        let mut call_runtime = Label::new();
        self.Ld_d(old_fp, MemOperand::new(fp, TypedFrameConstants::kFrameTypeOffset));
        self.BranchShort(
            &mut call_runtime,
            Condition::Equal,
            old_fp,
            Operand::new(StackFrame::TypeToMarker(StackFrame::WASM_SEGMENT_START) as i64),
        );
        self.mov(old_fp, fp);
        self.jmp(&mut done);

        self.bind(&mut call_runtime);
        let mut regs_to_save = LiftoffRegList::new(); // self.cache_state().used_registers;  cannot be done directly due to borrowing
                                                       // self.PushRegisters(regs_to_save);
        self.li(Register::A0, Operand::new(0));  //ExternalReference::isolate_address()
                                                  // self.PrepareCallCFunction(1, kScratchReg);
                                                  // self.CallCFunction(ExternalReference::wasm_load_old_fp(), 1);
        if old_fp != Register::ReturnRegister0 {
            self.mov(old_fp, Register::ReturnRegister0);
        }
        // self.PopRegisters(regs_to_save);

        self.bind(&mut done);
        old_fp
    }

    /// Checks for stack shrinking.
    pub fn CheckStackShrink(&mut self) {
        let mut done = Label::new();
        {
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            self.Ld_d(scratch, MemOperand::new(fp, TypedFrameConstants::kFrameTypeOffset));
            self.BranchShort(
                &mut done,
                Condition::NotEqual,
                scratch,
                Operand::new(StackFrame::TypeToMarker(StackFrame::WASM_SEGMENT_START) as i64),
            );
        }
        let mut regs_to_save = LiftoffRegList::new();
        // for reg in kGpReturnRegisters { regs_to_save.set(reg); }
        // for reg in kFpReturnRegisters { regs_to_save.set(reg); }
        // self.PushRegisters(regs_to_save);
        self.li(Register::A0, Operand::new(0));  //ExternalReference::isolate_address()
                                                  // self.PrepareCallCFunction(1, kScratchReg);
                                                  // self.CallCFunction(ExternalReference::wasm_shrink_stack(), 1);
        self.mov(fp, Register::ReturnRegister0);
        // self.PopRegisters(regs_to_save);
        self.bind(&mut done);
    }

    /// Loads a constant into a register.
    pub fn LoadConstant(&mut self, reg: LiftoffRegister, value: WasmValue) {
        match value.type_().kind {
            ValueKind::I32 => {
                self.li(reg.gp(), Operand::new(value.to_i32() as i64));
            }
            ValueKind::I64 => {
                self.li(reg.gp(), Operand::new(value.to_i64()));
            }
            ValueKind::F32 => {
                //MacroAssembler::Move(reg.fp(), value.to_f32_boxed().get_bits());
                println!("F32 constant load unimplemented");
                self.li(reg.gp(), Operand::new(0)); // Placeholder
            }
            ValueKind::F64 => {
                //MacroAssembler::Move(reg.fp(), value.to_f64_boxed().get_bits());
                println!("F64 constant load unimplemented");
                self.li(reg.gp(), Operand::new(0)); // Placeholder
            }
            _ => {
                panic!("Unsupported value kind");
            }
        }
    }

    /// Loads instance data from the frame.
    pub fn LoadInstanceDataFromFrame(&mut self, dst: Register) {
        self.Ld_d(dst, liftoff::GetInstanceDataOperand());
    }

    /// Loads a trusted pointer.
    pub fn LoadTrustedPointer(&mut self, dst: Register, src_addr: Register, offset: i32, tag: IndirectPointerTag) {
        let src = MemOperand::new(src_addr, offset);
        self.LoadTrustedPointerField(dst, src, tag);
    }

    /// Loads data from an instance.
    pub fn LoadFromInstance(&mut self, dst: Register, instance: Register, offset: i32, size: i32) {
        assert!(offset >= 0);
        match size {
            1 => {
                self.Ld_b(dst, MemOperand::new(instance, offset));
            }
            4 => {
                self.Ld_w(dst, MemOperand::new(instance, offset));
            }
            8 => {
                self.Ld_d(dst, MemOperand::new(instance, offset));
            }
            _ => {
                panic!("LoadFromInstance: size {} unimplemented", size);
            }
        }
    }

    /// Loads a tagged pointer from an instance.
    pub fn LoadTaggedPointerFromInstance(&mut self, dst: Register, instance: Register, offset: i32) {
        self.LoadTaggedField(dst, MemOperand::new(instance, offset));
    }

    /// Spills instance data.
    pub fn SpillInstanceData(&mut self, instance: Register) {
        self.St_d(instance, liftoff::GetInstanceDataOperand());
    }

    /// Resets the OSR target.
    pub fn ResetOSRTarget(&mut self) {}

    /// Loads a tagged pointer.
    pub fn LoadTaggedPointer(
        &mut self,
        dst: Register,
        src_addr: Register,
        offset_reg: Register,
        offset_imm: i32,
        protected_load_pc: *mut u32,
        needs_shift: bool,
    ) {
        let shift_amount = if !needs_shift {
            0
        } else {
            if COMPRESS_POINTERS_BOOL {
                2
            } else {
                3
            }
        };
        let src_op = liftoff::GetMemOp(self, src_addr, offset_reg, offset_imm as i64, false, shift_amount);