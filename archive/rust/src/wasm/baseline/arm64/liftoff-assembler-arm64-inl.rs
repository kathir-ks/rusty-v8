// src/wasm/baseline/arm64/liftoff_assembler_arm64_inl.rs

pub mod liftoff_assembler_arm64_inl {
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Neg, Shl, Shr};

    // Placeholder for crates equivalent to the following C++ includes:
    // - "src/codegen/arm64/macro-assembler-arm64-inl.h"
    // - "src/codegen/interface-descriptors-inl.h"
    // - "src/compiler/linkage.h"
    // - "src/heap/mutable-page-metadata.h"
    // - "src/wasm/baseline/liftoff-assembler.h"
    // - "src/wasm/baseline/parallel-move-inl.h"
    // - "src/wasm/object-access.h"
    // - "src/wasm/wasm-linkage.h"
    // - "src/wasm/wasm-objects.h"
    //
    // Replace with actual crate imports as needed.
    // For now, we'll define some dummy types.

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ValueKind {
        I32,
        I64,
        F32,
        F64,
        Ref,
        RefNull,
        S128,
        I16, // Added for kI16
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct LiftoffRegister {
        code: usize, // Placeholder. Represents the register's code.
    }

    impl LiftoffRegister {
        pub fn gp(&self) -> CPURegister {
            CPURegister { code: self.code }
        }
        pub fn fp(&self) -> DoubleRegister {
            DoubleRegister { code: self.code }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CPURegister {
        code: usize, // Placeholder. Represents the register's code.
    }

    impl CPURegister {
        pub fn W(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }
        pub fn X(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }
        pub fn is_valid(&self) -> bool {
            self.code != 0 // Placeholder. Assuming 0 means invalid.
        }

        pub fn Aliases(&self, other:Self) -> bool {
            self == &other // Placeholder. Adjust as needed.
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister {
        code: usize, // Placeholder. Represents the register's code.
    }

    impl DoubleRegister {
        pub fn S(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }
        pub fn D(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }
        pub fn Q(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V1D(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V2D(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V4S(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V8H(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V16B(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }

        pub fn V2S(&self) -> Self {
            *self // Placeholder. Adjust as needed.
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MemOperand {
        base: CPURegister,
        offset: i32,
    }

    impl MemOperand {
        pub fn new(base: CPURegister, offset: i32) -> Self {
            MemOperand { base, offset }
        }

        pub fn IsPostIndex(&self) -> bool {
            false //Placeholder
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Operand {
        value: i64,
    }

    impl Operand {
        pub fn new(value: i64) -> Self {
            Operand { value }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StackFrame {
        WASM,
        WASM_SEGMENT_START, //Added
    }

    pub struct LiftoffRegList {
       regs: Vec<CPURegister>
    }

    impl LiftoffRegList {
        pub fn new() -> Self {
            LiftoffRegList { regs: Vec::new() }
        }

        pub fn set(&mut self, reg: CPURegister) {
            self.regs.push(reg);
        }

        pub fn has(&self, reg: LiftoffRegister) -> bool {
            self.regs.contains(&reg.gp())
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Condition {
        eq,
        ne,
        hs,
        mi,
        ge,
        gt, // added
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Builtin {
        kWasmLiftoffFrameSetup,
        kWasmStackOverflow,
        kWasmHandleStackOverflow,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RelocInfo {
        WASM_STUB_CALL,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StackLimitKind {
        kRealStackLimit,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StoreType {
        kI32Store8,
        kI64Store8,
        kI32Store16,
        kI64Store16,
        kI32Store,
        kI64Store32,
        kI64Store,
        kF32StoreF16,
        kF32Store,
        kF64Store,
        kS128Store,
    }

    impl StoreType {
        pub fn value(&self) -> i32 {
            match self {
                StoreType::kI32Store8 => 0,
                StoreType::kI64Store8 => 1,
                StoreType::kI32Store16 => 2,
                StoreType::kI64Store16 => 3,
                StoreType::kI32Store => 4,
                StoreType::kI64Store32 => 5,
                StoreType::kI64Store => 6,
                StoreType::kF32StoreF16 => 7,
                StoreType::kF32Store => 8,
                StoreType::kF64Store => 9,
                StoreType::kS128Store => 10,
            }
        }

        pub fn mem_rep(&self) -> MachineRepresentation {
             match self {
                StoreType::kI32Store8 | StoreType::kI64Store8 => MachineRepresentation::kWord8,
                StoreType::kI32Store16 | StoreType::kI64Store16 => MachineRepresentation::kWord16,
                StoreType::kI32Store | StoreType::kI64Store32 => MachineRepresentation::kWord32,
                StoreType::kI64Store => MachineRepresentation::kWord64,
                _ => MachineRepresentation::kNone
             }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LoadType {
        kI32Load8U,
        kI64Load8U,
        kI32Load8S,
        kI64Load8S,
        kI32Load16U,
        kI64Load16U,
        kI32Load16S,
        kI64Load16S,
        kI32Load,
        kI64Load32U,
        kI64Load32S,
        kI64Load,
        kF32Load,
        kF32LoadF16,
        kF64Load,
        kS128Load,
    }

    impl LoadType {
        pub fn value(&self) -> i32 {
            match self {
                LoadType::kI32Load8U => 0,
                LoadType::kI64Load8U => 1,
                LoadType::kI32Load8S => 2,
                LoadType::kI64Load8S => 3,
                LoadType::kI32Load16U => 4,
                LoadType::kI64Load16U => 5,
                LoadType::kI32Load16S => 6,
                LoadType::kI64Load16S => 7,
                LoadType::kI32Load => 8,
                LoadType::kI64Load32U => 9,
                LoadType::kI64Load32S => 10,
                LoadType::kI64Load => 11,
                LoadType::kF32Load => 12,
                LoadType::kF32LoadF16 => 13,
                LoadType::kF64Load => 14,
                LoadType::kS128Load => 15,
            }
        }

         pub fn mem_type(&self) -> MachineType {
            match self {
                LoadType::kI32Load8U | LoadType::kI64Load8U | LoadType::kI32Load8S | LoadType::kI64Load8S => MachineType::Int8,
                LoadType::kI32Load16U | LoadType::kI64Load16U | LoadType::kI32Load16S | LoadType::kI64Load16S => MachineType::Int16,
                LoadType::kI32Load | LoadType::kI64Load32U | LoadType::kI64Load32S => MachineType::Int32,
                LoadType::kI64Load => MachineType::Int64,
                _ => MachineType::Invalid,
            }
        }

        pub fn size_log_2(&self) -> u32 {
            match self {
                LoadType::kI32Load8U | LoadType::kI64Load8U | LoadType::kI32Load8S | LoadType::kI64Load8S => 0,
                LoadType::kI32Load16U | LoadType::kI64Load16U | LoadType::kI32Load16S | LoadType::kI64Load16S => 1,
                LoadType::kI32Load | LoadType::kI64Load32U | LoadType::kI64Load32S => 2,
                LoadType::kI64Load => 3,
                _ => 0, //Adjust default if necessary
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IndirectPointerTag {
        // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SkipWriteBarrier {
        // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SmiCheckMode {
        kJumpOnSmi,
        kJumpOnNotSmi,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LoadTransformationKind {
        kExtend,
        kZeroExtend,
        kSplat,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum VectorFormat {
        kFormat4S,
        kFormat2D,
        kFormat16B,
        kFormat8H
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ScalarFormat {
      // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RegClass {
        kGpReg
    }

    pub const wzr: CPURegister = CPURegister{ code: 1 };
    pub const xzr: CPURegister = CPURegister{ code: 2 };
    pub const fp: CPURegister = CPURegister{ code: 3 };
    pub const sp: CPURegister = CPURegister{ code: 4 };
    pub const lr: CPURegister = CPURegister{ code: 5 };
    pub const kReturnRegister0: CPURegister = CPURegister{ code: 6 };

    pub const padreg: CPURegister = CPURegister{ code: 7 };

    pub const fp_scratch: DoubleRegister = DoubleRegister{ code: 8 };

    pub const kCArgRegs: [CPURegister; 1] = [CPURegister{ code: 9 }];

    pub const kGpParamRegisters: [CPURegister; 1] = [CPURegister{ code: 10 }];
    pub const kFpParamRegisters: [DoubleRegister; 1] = [DoubleRegister{ code: 11 }];
    pub const kGpReturnRegisters: [CPURegister; 1] = [CPURegister{ code: 12 }];
    pub const kFpReturnRegisters: [DoubleRegister; 1] = [DoubleRegister{ code: 13 }];

    pub const kLiftoffFrameSetupFunctionReg: CPURegister = CPURegister{code: 14};
    pub const x16: CPURegister = CPURegister{code: 15};
    pub const x17: CPURegister = CPURegister{code: 16};

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MachineType {
        // Placeholder
        value: i32
    }

     impl MachineType {
        const Invalid : MachineType = MachineType { value: 0 };
        const Int8 : MachineType = MachineType { value: 1 };
        const Uint8 : MachineType = MachineType { value: 2 };
        const Int16 : MachineType = MachineType { value: 3 };
        const Uint16 : MachineType = MachineType { value: 4 };
        const Int32 : MachineType = MachineType { value: 5 };
        const Uint32 : MachineType = MachineType { value: 6 };
        const Int64 : MachineType = MachineType { value: 7 };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MachineRepresentation {
        kNone,
        kWord8,
        kWord16,
        kWord32,
        kWord64,
    }

    // Constants
    pub const kXRegSize: i32 = 8;
    pub const kQRegSizeInBits: i32 = 128;
    pub const kXRegSizeInBits: i32 = 64;
    pub const kStackSlotSize: i32 = 8;
    pub const kSystemPointerSize: i32 = 8;
    pub const kQuadWordSizeInBytes: i32 = 16;
    pub const kInt32Size: i32 = 4;

    // enums
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum UXTW {
        // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LSL {
        // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum UXTX {
        // Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SaveFPRegsMode {
        kSave
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StubCallMode {
        kCallWasmRuntimeStub
    }

    // Structs to represent external functionalities (Placeholder)
    pub struct WasmValue {
        value: i64,
        kind: ValueKind,
    }

    impl WasmValue {
        pub fn to_i32(&self) -> i32 {
            self.value as i32
        }
        pub fn to_i64(&self) -> i64 {
            self.value
        }
        pub fn to_f32(&self) -> f32 {
            self.value as f32
        }
        pub fn to_f64(&self) -> f64 {
            self.value as f64
        }
        pub fn type_(&self) -> ValueKind {
           self.kind
        }
    }

    pub struct WasmTrustedInstanceData {}

    impl WasmTrustedInstanceData {
        const kTieringBudgetArrayOffset: i32 = 0; // Placeholder
    }

    pub struct MemoryChunk {}

    impl MemoryChunk {
        const kPointersFromHereAreInterestingMask: i32 = 0; // Placeholder
        const kPointersToHereAreInterestingMask: i32 = 0; // Placeholder
    }

    pub struct TypedFrameConstants {}
     impl TypedFrameConstants {
        const kFrameTypeOffset: i32 = 0;
     }

    pub struct CommonFrameConstants {}

    impl CommonFrameConstants {
        const kFixedFrameSizeAboveFp : i32 = 0;
    }

    pub struct UseScratchRegisterScope<'a> {
        assm: &'a mut LiftoffAssembler,
        excluded: Vec<CPURegister>,
        acquired: Vec<CPURegister>,
    }

    impl<'a> UseScratchRegisterScope<'a> {
        pub fn new(assm: &'a mut LiftoffAssembler) -> Self {
            UseScratchRegisterScope {
                assm,
                excluded: Vec::new(),
                acquired: Vec::new(),
            }
        }

        pub fn AcquireW(&mut self) -> CPURegister {
            //Placeholder
            let reg = CPURegister{code:100};
            self.acquired.push(reg);
            reg
        }
        pub fn AcquireX(&mut self) -> CPURegister {
            //Placeholder
            let reg = CPURegister{code:101};
            self.acquired.push(reg);
            reg
        }

        pub fn AcquireS(&mut self) -> DoubleRegister {
             //Placeholder
            let reg = DoubleRegister{code:102};
            reg
        }
         pub fn AcquireD(&mut self) -> DoubleRegister {
             //Placeholder
            let reg = DoubleRegister{code:103};
            reg
        }

         pub fn AcquireQ(&mut self) -> DoubleRegister {
             //Placeholder
            let reg = DoubleRegister{code:104};
            reg
        }
        pub fn AcquireV(&mut self, format: VectorFormat) -> DoubleRegister {
            //Placeholder
            let reg = DoubleRegister{code:105};
            reg
        }

        pub fn Exclude(&mut self, r1: CPURegister, r2: CPURegister) {
            self.excluded.push(r1);
            self.excluded.push(r2);
        }
        pub fn Include(&mut self, r1: CPURegister) {
            //TODO
        }

    }

    // Dummy implementations for the purpose of compilation
    pub mod base {
        pub mod bits {
            pub fn IsPowerOfTwo(x: i32) -> bool {
                (x > 0) && ((x & (x - 1)) == 0)
            }
            pub fn WhichPowerOfTwo(x: i32) -> i32 {
                0 //Placeholder
            }
        }
    }

    pub struct AssemblerOptions {}

    pub struct SafepointTableBuilder {}

    impl SafepointTableBuilder {
        pub fn DefineSafepoint(&mut self, assm: &mut LiftoffAssembler) {}
    }

    pub struct PatchingAssembler<'a> {
       zone: &'a Zone,
       options: AssemblerOptions,
       buffer_start: i32,
       factor: i32,
    }

    impl<'a> PatchingAssembler<'a> {
       pub fn new(zone: &'a Zone, options: AssemblerOptions, buffer_start: i32, factor: i32) -> PatchingAssembler<'a> {
            PatchingAssembler { zone, options, buffer_start, factor }
       }

       pub fn PatchSubSp(&mut self, frame_size: i32) {}

       pub fn b(&mut self, offset: i32) {}
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Vector<T> {
        // Placeholder
    }

    pub struct BlockPoolsScope {}
    impl BlockPoolsScope {
        pub fn new() -> BlockPoolsScope {
            BlockPoolsScope {}
        }
    }

    pub struct WasmHandleStackOverflowDescriptor {}
    impl WasmHandleStackOverflowDescriptor {
        pub fn GapRegister() -> CPURegister { CPURegister { code: 0 } }
        pub fn FrameBaseRegister() -> CPURegister { CPURegister { code: 0 } }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BarrierAll {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum InnerShareable {}

    //Implementations for some external helper functions
    pub fn value_kind_size(kind: ValueKind) -> i32 {
        match kind {
            ValueKind::I32 => 4,
            ValueKind::I64 => 8,
            ValueKind::F32 => 4,
            ValueKind::F64 => 8,
            ValueKind::Ref => 8,
            ValueKind::RefNull => 8,
            ValueKind::S128 => 16,
            ValueKind::I16 => 2,
        }
    }

    pub fn is_reference(kind: ValueKind) -> bool {
        match kind {
            ValueKind::Ref | ValueKind::RefNull => true,
            _ => false,
        }
    }

    pub fn LaneCountFromFormat(format: VectorFormat) -> i32 {
       match format {
            VectorFormat::kFormat4S => 4,
            VectorFormat::kFormat2D => 2,
            VectorFormat::kFormat16B => 16,
            VectorFormat::kFormat8H => 8,
       }
    }

    pub fn LaneSizeInBitsFromFormat(format: VectorFormat) -> i32 {
        match format {
            VectorFormat::kFormat4S => 32,
            VectorFormat::kFormat2D => 64,
            VectorFormat::kFormat16B => 8,
            VectorFormat::kFormat8H => 16,
       }
    }

     pub fn ScalarFormatFromFormat(format: VectorFormat) -> VectorFormat {
        VectorFormat::kFormat4S //Placeholder
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FP16 {}

    pub struct Zone{}

    pub struct Instruction {}

    impl Instruction {
        pub fn IsLoad(&self) -> bool {
            false
        }

        pub fn IsStore(&self) -> bool {
            false
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Flag {}
    pub const NoFlag: Flag = Flag {};

    pub struct ExternalReference {}

    impl ExternalReference {
        pub fn isolate_address() -> ExternalReference {ExternalReference{}}
        pub fn wasm_load_old_fp() -> ExternalReference {ExternalReference{}}
        pub fn wasm_shrink_stack() -> ExternalReference {ExternalReference{}}
    }

    pub fn SmiValuesAre31Bits() -> bool {
        true
    }

    pub fn SmiTag(reg: CPURegister) {}
    pub fn SmiUntag(reg: CPURegister, mem: MemOperand) {}

    pub struct Smi {}
    impl Smi {
        pub fn FromInt(val: i32) -> i32 {
            val
        }
    }

    pub const kWasmEntrypointTag: i32 = 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PostIndex {}

    //------------------------------- LiftoffAssembler ------------------------------------

    pub struct LiftoffAssembler {
        buffer_start_: i32,
        options_: LiftoffAssemblerOptions,
        pc_offset_: i32,
        max_used_spill_offset_: i32,
        total_frame_size_: i32,
        cache_state_: FreezeCacheState, // Added cache_state_ field
    }

    impl LiftoffAssembler {
        pub fn new(options: LiftoffAssemblerOptions) -> LiftoffAssembler {
            LiftoffAssembler {
                buffer_start_: 0, // Placeholder
                options_: options,
                pc_offset_: 0,    // Placeholder
                max_used_spill_offset_: 0,
                total_frame_size_: 0,
                cache_state_: FreezeCacheState::new(), // Initialize with a default FreezeCacheState
            }
        }

        fn options(&self) -> &LiftoffAssemblerOptions {
            &self.options_
        }

         fn cache_state(&mut self) -> &mut FreezeCacheState {
            &mut self.cache_state_
        }

        pub fn pc_offset(&self) -> i32 {
            self.pc_offset_
        }

        pub fn AbortedCodeGeneration(&mut self) {}

        pub fn ForceConstantPoolEmissionWithoutJump(&mut self) {}

        pub fn GetTotalFrameSize(&self) -> i32 {
            self.total_frame_size_
        }

        pub fn RecordUsedSpillOffset(&mut self, offset: i32) {
            self.max_used_spill_offset_ = self.max_used_spill_offset_.max(offset);
        }

        pub fn GetUnusedRegister(&self, reg_class: RegClass, pinned: LiftoffRegList) -> LiftoffRegister {
           //Placeholder Implementation
           LiftoffRegister{code: 1}
        }

        pub fn InstructionAt(&self, pc: u32) -> &Instruction {
            //Placeholder Implementation
            &Instruction{}
        }

        pub fn PrepareStackFrame(&mut self) -> i32 {
            let offset = self.pc_offset();
            self.pc_offset_ += 1; // Placeholder for instruction size
                                  // Placeholder for instructionAccurateScope logic
            self.sub(sp, sp, 0); // Placeholder instruction
            offset
        }

        pub fn CallFrameSetupStub(&mut self, declared_function_index: i32) {
            self.EnterFrame(StackFrame::WASM);
            self.LoadConstant(
                LiftoffRegister { code: kLiftoffFrameSetupFunctionReg.code },
                WasmValue { value: declared_function_index as i64, kind:ValueKind::I32},
            );
            self.CallBuiltin(Builtin::kWasmLiftoffFrameSetup);
        }

        pub fn PrepareTailCall(&mut self, num_callee_stack_params: i32, stack_param_delta: i32) {
            let mut temps = UseScratchRegisterScope::new(self);
            temps.Exclude(x16, x17);

            self.Add(x16, fp, 16);

            // Load the fp and lr of the old frame, they will be pushed in the new frame
            // during the actual call.
            self.Ldp(fp, x17, MemOperand::new(fp, 0));
            self.Mov(lr, x17);

            temps.Include(x17);

            let scratch = temps.AcquireX();

            // Shift the whole frame upwards, except for fp and lr.
            // Adjust x16 to be the new stack pointer first, so that {str} doesn't need
            // a temp register to materialize the offset.
            self.Sub(x16, x16, stack_param_delta * 8);
            let slot_count = num_callee_stack_params;
            for i in (0..slot_count).rev() {
                self.ldr(scratch, MemOperand::new(sp, i * 8));
                self.str(scratch, MemOperand::new(x16, i * 8));
            }

            // Set the new stack pointer.
            self.mov(sp, x16);
        }

        pub fn AlignFrameSize(&mut self) {
            let frame_size = self.GetTotalFrameSize() - 2 * kSystemPointerSize;

            // The stack pointer is required to be quadword aligned.
            // Misalignment will cause a stack alignment fault.
            let misalignment = frame_size % kQuadWordSizeInBytes;
            if misalignment != 0 {
                let padding = kQuadWordSizeInBytes - misalignment;
                self.total_frame_size_ += padding;
                self.max_used_spill_offset_ += padding;
            }
        }

        pub fn PatchPrepareStackFrame(
            &mut self,
            offset: i32,
            safepoint_table_builder: &mut SafepointTableBuilder,
            feedback_vector_slot: bool,
            stack_param_slots: usize,
        ) {
            let mut frame_size = self.GetTotalFrameSize() - 2 * kSystemPointerSize;

            if feedback_vector_slot {
                frame_size = std::cmp::max(frame_size - 2 * kSystemPointerSize, 0);
            }

            // The stack pointer is required to be quadword aligned.
            assert_eq!(frame_size % kQuadWordSizeInBytes, 0);

            let zone = Zone{};
            let options = AssemblerOptions{};
            let mut patching_assembler = PatchingAssembler::new(&zone, options, self.buffer_start_ + offset, 1);

            if frame_size < 4 * 1024 {
                assert!(self.IsImmAddSub(frame_size));
                patching_assembler.PatchSubSp(frame_size);
                return;
            }

            patching_assembler.b((self.pc_offset() - offset) >> 2);

            //OOL code part starts
            let continuation = Label{};

            if 0 < 1024 {
                let mut temps = UseScratchRegisterScope::new(self);
                let stack_limit = temps.AcquireX();

                self.LoadStackLimit(stack_limit, StackLimitKind::kRealStackLimit);
                self.Add(stack_limit, stack_limit, frame_size as i64);
                self.Cmp(sp, stack_limit);
                self.B(Condition::hs, &continuation);
            }

            let regs_to_save = LiftoffRegList::new();
            // self.PushRegisters(regs_to_save);
            self.Mov(WasmHandleStackOverflowDescriptor::GapRegister(), frame_size as i64);
            self.Add(WasmHandleStackOverflowDescriptor::FrameBaseRegister(), fp, (stack_param_slots * kStackSlotSize + CommonFrameConstants::kFixedFrameSizeAboveFp) as i64);
            self.CallBuiltin(Builtin::kWasmHandleStackOverflow);
            //self.PopRegisters(regs_to_save);

            // self.Call(
            //     Builtin::kWasmStackOverflow as *const (),
            //     RelocInfo::WASM_STUB_CALL,
            // );
            safepoint_table_builder.DefineSafepoint(self);

            self.bind(&continuation);
            self.Claim(frame_size, 1);

            let func_start_offset = offset + 4; //Placeholder instruction size
            patching_assembler.b((func_start_offset - self.pc_offset()) >> 2);
        }

        pub fn FinishCode(&mut self) {
            self.ForceConstantPoolEmissionWithoutJump();
        }

        pub fn AbortCompilation(&mut self) {
            self.AbortedCodeGeneration();
        }

        pub fn StaticStackFrameSize() -> i32 {
            WasmLiftoffFrameConstants::kFeedbackVectorOffset
        }

        pub fn SlotSizeForType(kind: ValueKind) -> i32 {
            match kind {
                ValueKind::S128 => value_kind_size(kind),
                _ => kStackSlotSize,
            }
        }

        pub fn NeedsAlignment(kind: ValueKind) -> bool {
            kind == ValueKind::S128 || is_reference(kind)
        }

        pub fn CheckTierUp(
            &mut self,
            declared_func_index: i32,
            budget_used: i32,
            ool_label: &mut Label,
            frozen: &FreezeCacheState,
        ) {
             let mut temps = UseScratchRegisterScope::new(self);
            let budget_array = temps.AcquireX();

             let mut instance_data = self.cache_state_.cached_instance_data;
            if instance_data.code == 0 {
                instance_data = budget_array;  // Reuse the temp register.
                self.LoadInstanceDataFromFrame(instance_data);
            }

            const kArrayOffset: i32 = 0; // Placeholder

            self.ldr(budget_array, MemOperand::new(instance_data, kArrayOffset));

            let budget_arr_offset = kInt32Size * declared_func_index;

            if (!self.IsImmLSScaled