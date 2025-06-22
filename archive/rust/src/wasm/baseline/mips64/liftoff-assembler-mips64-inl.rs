pub mod liftoff_assembler_mips64 {
    //use std::arch::mips64::*; // Requires nightly

    use crate::codegen::machine_type::MachineType;
    use crate::compiler::linkage::Linkage;
    use crate::heap::mutable_page_metadata::MemoryChunk;
    use crate::wasm::baseline::liftoff_assembler::{
        FreezeCacheState, LiftoffAssembler, LiftoffRegister, ValueKind,
        WasmValue,
    };
    use crate::wasm::baseline::parallel_move::ParallelMove;
    use crate::wasm::object_access::WasmTrustedInstanceData;
    use crate::wasm::wasm_linkage::IndirectPointerTag;
    use crate::wasm::wasm_objects::WasmOpcode;

    // Placeholder imports
    type Register = i32;
    type DoubleRegister = i32;
    type Label = i32;
    type Condition = i32;
    type StoreType = i32;
    type LoadType = i32;
    type Builtin = i32;

    const kSystemPointerSize: i32 = 8;
    const kInt32Size: i32 = 4;
    const kTaggedSize: i32 = 8;
    const kInt64Size: i32 = 8;
    const kStackSlotSize: i32 = 8;
    const kSmiTagMask: i32 = 1;
    const no_reg: Register = -1;
    const zero_reg: Register = 0;
    const sp: Register = 29;
    const fp: Register = 30;
    const ra: Register = 31;
    const kScratchReg: Register = 1;
    const kScratchReg2: Register = 2;
    const t8: Register = 24;
    const a0: Register = 4;
    const a1: Register = 5;
    const kLiftoffFrameSetupFunctionReg: Register = 3;
    const kScratchDoubleReg: DoubleRegister = 0;
    const kScratchDoubleReg2: DoubleRegister = 1;
    const kSimd128ScratchReg: DoubleRegister = 2;
    const kSimd128RegZero: DoubleRegister = 3;

    // Mock data structures

    pub struct MemOperand {
        base: Register,
        offset: i32,
    }

    impl MemOperand {
        fn new(base: Register, offset: i32) -> MemOperand {
            MemOperand { base, offset }
        }
        fn rm(&self) -> Register {
            self.base
        }
        fn offset(&self) -> i32 {
            self.offset
        }
    }

    struct LoadStoreLaneParams {
        sz: i32,
        lane: i32,
    }
    impl LoadStoreLaneParams {
        fn new(sz: i32, lane: i32) -> Self {
            LoadStoreLaneParams { sz, lane }
        }
    }

    pub struct LiftoffRegList {
        regs: Vec<Register>, // Using Vec for dynamic register list
    }

    impl LiftoffRegList {
        pub fn new(regs: Vec<Register>) -> Self {
            Self { regs }
        }

        pub fn set(&mut self, reg: Register) -> Register {
            // Simple check to avoid duplicates for this example.  A more robust
            // approach, like a HashSet, might be beneficial in real use.
            if !self.regs.contains(&reg) {
                self.regs.push(reg);
            }
            reg
        }
    }
    struct AssemblerOptions {}

    enum CodeObjectRequired {
        kNo,
    }

    struct ExternalAssemblerBuffer {
        start: *mut u8,
        size: usize,
    }
    impl ExternalAssemblerBuffer {
        fn new(start: *mut u8, size: usize) -> Self {
            ExternalAssemblerBuffer { start, size }
        }
    }
    struct MacroAssembler {
        options: AssemblerOptions,
        code_object_required: CodeObjectRequired,
        buffer: Option<ExternalAssemblerBuffer>,
    }

    impl MacroAssembler {
        fn new(
            options: AssemblerOptions,
            code_object_required: CodeObjectRequired,
            buffer: Option<ExternalAssemblerBuffer>,
        ) -> Self {
            MacroAssembler {
                options,
                code_object_required,
                buffer,
            }
        }
    }

    struct StackFrame {}
    impl StackFrame {
        const WASM: i32 = 0;
    }

    struct UseScratchRegisterScope<'a> {
        assembler: &'a LiftoffAssemblerImpl,
    }
    impl<'a> UseScratchRegisterScope<'a> {
        fn new(assembler: &'a LiftoffAssemblerImpl) -> UseScratchRegisterScope<'a> {
            UseScratchRegisterScope { assembler }
        }

        fn Acquire(&mut self) -> Register {
            //Placeholder for real acquisition
            kScratchReg
        }
    }

    // Mock functions for operations
    fn is_int31(x: i64) -> bool {
        x >= -(1 << 30) && x < (1 << 30)
    }

    pub mod wasm {
        pub const kGpParamRegisters: [i32; 4] = [4, 5, 6, 7];
    }

    pub mod liftoff {

        use super::*;

        pub const fn GetStackSlot(offset: i32) -> MemOperand {
            MemOperand::new(fp, -offset)
        }

        pub const fn GetInstanceDataOperand() -> MemOperand {
            GetStackSlot(WasmLiftoffFrameConstants::kInstanceDataOffset)
        }

        pub fn GetMemOp<T>(
            assm: &LiftoffAssemblerImpl,
            addr: Register,
            offset: Register,
            offset_imm: T,
            i64_offset: bool,
            shift_amount: u32,
        ) -> MemOperand
        where
            T: Into<i64> + Copy,
        {
            let offset_imm_i64: i64 = offset_imm.into();

            if offset != no_reg {
                let mut offset_reg = offset;
                if !i64_offset {
                    assm.Dext(kScratchReg, offset, 0, 32);
                    offset_reg = kScratchReg;
                }
                if shift_amount != 0 {
                    assm.Dlsa(kScratchReg, addr, offset_reg, shift_amount);
                } else {
                    assm.daddu(kScratchReg, offset_reg, addr);
                }
                return MemOperand::new(kScratchReg, 0);
            }
            if is_int31(offset_imm_i64) {
                let offset_imm32 = offset_imm_i64 as i32;
                return MemOperand::new(addr, offset_imm32);
            } else {
                assm.li(kScratchReg2, offset_imm_i64);
                assm.daddu(kScratchReg, addr, kScratchReg2);
                return MemOperand::new(kScratchReg, 0);
            }
        }
        // Placeholder impls for Store and Load
        pub fn Load(assm: &LiftoffAssemblerImpl, dst: LiftoffRegister, src: MemOperand, kind: ValueKind) {
                match kind {
                    ValueKind::I16 => assm.Lh(dst.gp(), src),
                    ValueKind::I32 => assm.Lw(dst.gp(), src),
                    ValueKind::I64 | ValueKind::Ref | ValueKind::RefNull => assm.Ld(dst.gp(), src),
                    ValueKind::F32 => assm.Lwc1(dst.fp(), src),
                    ValueKind::F64 => assm.Ldc1(dst.fp(), src),
                    ValueKind::S128 => assm.ld_b(dst.fp().toW(), src),
                    _ => unimplemented!()
                }
            }
        
            pub fn Store(assm: &LiftoffAssemblerImpl, dst: MemOperand, src: LiftoffRegister, kind: ValueKind) {
                match kind {
                    ValueKind::I16 => assm.Ush(src.gp(), dst, t8),
                    ValueKind::I32 => assm.Usw(src.gp(), dst),
                    ValueKind::I64 | ValueKind::RefNull | ValueKind::Ref => assm.Usd(src.gp(), dst),
                    ValueKind::F32 => assm.Uswc1(src.fp(), dst, t8),
                    ValueKind::F64 => assm.Usdc1(src.fp(), dst, t8),
                    ValueKind::S128 => assm.st_b(src.fp().toW(), dst),
                    _ => unimplemented!()
                }
            }
    }

    pub mod v8_flags {
        pub const stack_size: i32 = 16; //Mock value

                                         // Other flags would go here.
    }

    // Mock constants

    // Implement your own MIPS64 LiftoffAssembler
    pub struct LiftoffAssemblerImpl {
        pc_offset: i32,
        frame_size: i32,
        safepoint_table_builder: Option<i32>, //Placeholder
        buffer_start_: *mut u8,                //Placeholder
        cache_state_: CacheState,
        used_spill_offsets: Vec<i32>, // Keep track of used offsets
    }

    struct CacheState {
        cached_instance_data: Register,
    }

    impl LiftoffAssemblerImpl {
        pub fn new(buffer_start_: *mut u8) -> Self {
            LiftoffAssemblerImpl {
                pc_offset: 0,
                frame_size: 0,
                safepoint_table_builder: None,
                buffer_start_: buffer_start_,
                cache_state_: CacheState {
                    cached_instance_data: no_reg,
                },
                used_spill_offsets: Vec::new(),
            }
        }
        fn RecordUsedSpillOffset(&mut self, offset: i32) {
            if !self.used_spill_offsets.contains(&offset) {
                self.used_spill_offsets.push(offset);
            }
        }

        fn set_pc_offset(&mut self, offset: i32) {
            self.pc_offset = offset;
        }

        fn pc_offset(&self) -> i32 {
            self.pc_offset
        }

        fn EnterFrame(&self, frame_type: i32) {
            // Mock for now
        }
        fn LoadConstant(&self, reg: LiftoffRegister, value: WasmValue) {
            // Mock for now
        }
        fn CallBuiltin(&self, builtin: Builtin) {
            //Mock for now
        }
        fn GetTotalFrameSize(&self) -> i32 {
            self.frame_size
        }

        fn Daddu(&self, dst: Register, src: Register, operand: i32) {
            // Mock
        }
        fn Ld(&self, dst: Register, mem: MemOperand) {
            // Mock
        }
        fn Push(&self, reg: Register) {
            // Mock
        }
        fn Pop(&self, reg: Register) {
            // Mock
        }
        fn Call(&self, address: i32, reloc_info: i32) {
            // Mock
        }
        fn Branch(&self, label: *mut i32, cond: i32, reg1: Register, operand: i32) {
            // Mock
        }

        fn Bind(&self, label: *mut i32) {
            // Mock
        }

        fn stop(&self) {}

        fn BranchLong(&self, offset: i32) {
            // Mock
        }

        fn Acquire(&self) -> Register {
            //Mock
            kScratchReg
        }
        fn LoadStackLimit(&self, stack_limit: Register, stack_limit_kind: i32) {
            // Mock
        }

        fn sw(&self, reg: Register, mem: MemOperand) {
            //Mock
        }

        fn Sd(&self, reg: Register, mem: MemOperand) {
            //Mock
        }
        fn Swc1(&self, reg: DoubleRegister, mem: MemOperand) {
            //Mock
        }
        fn Sdc1(&self, reg: DoubleRegister, mem: MemOperand) {
            //Mock
        }
        fn Lw(&self, reg: Register, mem: MemOperand) {
            //Mock
        }
        fn Lwc1(&self, reg: DoubleRegister, mem: MemOperand) {
            //Mock
        }
        fn Ldc1(&self, reg: DoubleRegister, mem: MemOperand) {
            //Mock
        }
        fn li(&self, reg: Register, imm: i64) {
            //Mock
        }

        fn daddiu(&self, reg: Register, reg2: Register, offset: i32) {
            //Mock
        }
        fn Dlsa(&self, dst: Register, src: Register, shift: Register, amount: u32) {
            //Mock
        }
        fn daddu(&self, dst: Register, src: Register, src2: Register) {
            //Mock
        }
        fn Dext(&self, dst: Register, src: Register, start: i32, len: i32) {
            //Mock
        }

        fn Ush(&self, src: Register, dst: MemOperand, t8: Register) {
            //Mock
        }
        fn Usw(&self, src: Register, dst: MemOperand) {
            //Mock
        }
        fn Usd(&self, src: Register, dst: MemOperand) {
            //Mock
        }
        fn Uswc1(&self, src: DoubleRegister, dst: MemOperand, t8: Register) {
            //Mock
        }
        fn Usdc1(&self, src: DoubleRegister, dst: MemOperand, t8: Register) {
            //Mock
        }

        fn CallRecordWriteStubSaveRegisters(&self, reg1: Register, reg2: Register, reg3: i32, reg4: i32) {
            //Mock
        }

        fn BranchMSA(&self, label: *mut i32, a: i32, b: i32, reg1: DoubleRegister, option: i32) {
            //Mock
        }

        fn Ulhu(&self, dst: Register, src: MemOperand) {
            //Mock
        }
        fn Ulh(&self, dst: Register, src: MemOperand) {
            //Mock
        }
        fn Ulwu(&self, dst: Register, src: MemOperand) {
            //Mock
        }
        fn Ulw(&self, dst: Register, src: MemOperand) {
            //Mock
        }
        fn Uld(&self, dst: Register, src: MemOperand) {
            //Mock
        }
        fn Ush(&self, dst: Register, src: MemOperand, t8: Register) {
            //Mock
        }
        fn Uswc1(&self, dst: DoubleRegister, src: MemOperand, t8: Register) {
            //Mock
        }
        fn Usdc1(&self, dst: DoubleRegister, src: MemOperand, t8: Register) {
            //Mock
        }
        fn BranchShort(&self, label: *mut i32, cond: i32, reg1: Register, operand: i32) {
            //Mock
        }
    
        fn GetUnusedRegister(&self, reg_type: i32, pinned: LiftoffRegList) -> LiftoffRegister {
                //Placeholder
                LiftoffRegister {
                    reg_type: 0, //Replace reg_type
                    reg_code: 0, //Replace reg_code
                }
            }
            fn Move(&self, dst: DoubleRegister, src: i64) {
                //Mock
            }
        
            fn CompareIsNanF32(&self, reg1: DoubleRegister, reg2: DoubleRegister) {
                //Mock
            }
            fn CompareIsNanF64(&self, reg1: DoubleRegister, reg2: DoubleRegister) {
                //Mock
            }
        
            fn BranchTrueShortF(&self, label: *mut i32) {
                //Mock
            }
            fn CompareF32(&self, cond: i32, reg1: DoubleRegister, reg2: DoubleRegister) {
                //Mock
            }
            fn CompareF64(&self, cond: i32, reg1: DoubleRegister, reg2: DoubleRegister) {
                //Mock
            }
            fn LoadZeroIfNotFPUCondition(&self, reg: Register) {
                //Mock
            }
            fn LoadZeroIfFPUCondition(&self, reg: Register) {
                //Mock
            }
            fn Sb(&self, reg: Register, mem: MemOperand) {}
            fn Sh(&self, reg: Register, mem: MemOperand) {}
            fn Sw(&self, reg: Register, mem: MemOperand) {}
            fn Sd(&self, reg: Register, mem: MemOperand) {}
            fn Lbu(&self, reg: Register, mem: MemOperand) {}
            fn Lb(&self, reg: Register, mem: MemOperand) {}
            fn Lh(&self, reg: Register, mem: MemOperand) {}
            fn Lwu(&self, reg: Register, mem: MemOperand) {}
            fn Lw(&self, reg: Register, mem: MemOperand) {}
            fn Ld(&self, reg: Register, mem: MemOperand) {}
            fn Lwc1(&self, reg: DoubleRegister, mem: MemOperand) {}
            fn Ldc1(&self, reg: DoubleRegister, mem: MemOperand) {}
            fn st_b(&self, reg: DoubleRegister, mem: MemOperand) {}
            fn ld_b(&self, reg: DoubleRegister, mem: MemOperand) {}
        }

        pub const USE_DELAY_SLOT: i32 = 0;
        pub const MSA_BRANCH_V: i32 = 1;
        pub const all_zero: i32 = 2;
        pub const all_not_zero: i32 = 3;
        pub const MSA_BRANCH_B: i32 = 4;

        const TRUNC_ZERO: i32 = 0;
        const kRelaxedSimd: i32 = 0;

    pub mod WasmLiftoffFrameConstants {
        pub const kInstanceDataOffset: i32 = 8;
        pub const kFeedbackVectorOffset: i32 = 12;
    }

    impl LiftoffAssembler for LiftoffAssemblerImpl {
        fn PrepareStackFrame(&mut self) -> i32 {
            let offset = self.pc_offset();
            self.daddiu(sp, sp, 0);
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            self.set_pc_offset(self.pc_offset() + 4);
            self.nop();
            offset
        }

        fn CallFrameSetupStub(&self, declared_function_index: i32) {
            self.EnterFrame(StackFrame::WASM);
            self.LoadConstant(
                LiftoffRegister {
                    reg_type: 0, //Replace
                    reg_code: kLiftoffFrameSetupFunctionReg,
                },
                WasmValue::I32(declared_function_index),
            );
            self.CallBuiltin(Builtin::kWasmLiftoffFrameSetup);
        }

        fn PrepareTailCall(&self, num_callee_stack_params: i32, stack_param_delta: i32) {
            let temps = UseScratchRegisterScope::new(self);
            let scratch = kScratchReg; //Placeholder
                                        // Push the return address and frame pointer to complete the stack frame.
            self.Ld(scratch, MemOperand::new(fp, 8));
            self.Push(scratch);
            self.Ld(scratch, MemOperand::new(fp, 0));
            self.Push(scratch);

            // Shift the whole frame upwards.
            let slot_count = num_callee_stack_params + 2;
            for i in (0..slot_count).rev() {
                self.Ld(scratch, MemOperand::new(sp, i * 8));
                self.Sd(scratch, MemOperand::new(fp, (i - stack_param_delta) * 8));
            }

            // Set the new stack and frame pointer.
            self.daddiu(sp, fp, -stack_param_delta * 8);
            self.Pop(ra);
            self.Pop(fp);
        }

        fn AlignFrameSize(&self) {}

        fn PatchPrepareStackFrame(
            &mut self,
            offset: i32,
            safepoint_table_builder: &mut i32,
            feedback_vector_slot: bool,
            stack_param_slots: usize,
        ) {
            let mut frame_size = self.GetTotalFrameSize() - 2 * kSystemPointerSize;

            if feedback_vector_slot {
                frame_size -= kSystemPointerSize;
            }

            const kAvailableSpace: i32 = 256;
            let patching_assembler = MacroAssembler::new(
                AssemblerOptions {},
                CodeObjectRequired::kNo,
                Some(ExternalAssemblerBuffer::new(
                    self.buffer_start_.wrapping_add(offset as usize),
                    kAvailableSpace as usize,
                )),
            );
            if frame_size < 4 * 1024 {
                self.Daddu(sp, sp, -frame_size);
                return;
            }

            let imm32 = self.pc_offset() - offset - 3 * 4;
            self.BranchLong(imm32);

            let continuation: i32 = 0; //Placeholder
            if frame_size < v8_flags::stack_size * 1024 {
                let stack_limit = kScratchReg;
                self.LoadStackLimit(stack_limit, 0); //Placeholder
                self.Daddu(stack_limit, stack_limit, frame_size);
                self.Branch(&mut 0, 0, sp, 0); //Placeholder
            }
            self.Call(0, 0); //Placeholder
                                 //self.safepoint_table_builder.DefineSafepoint(self);
            self.stop();
            self.Bind(&mut 0); //Placeholder
            self.Daddu(sp, sp, -frame_size);

            let func_start_offset = offset + 7 * 4;
            let imm32 = func_start_offset - self.pc_offset() - 3 * 4;
            self.BranchLong(imm32);
        }

        fn FinishCode(&self) {}

        fn AbortCompilation(&self) {}

        fn StaticStackFrameSize() -> i32 {
            WasmLiftoffFrameConstants::kFeedbackVectorOffset
        }

        fn SlotSizeForType(kind: ValueKind) -> i32 {
            match kind {
                ValueKind::S128 => 16, //Placeholder
                _ => kStackSlotSize,
            }
        }

        fn NeedsAlignment(kind: ValueKind) -> bool {
            false //Placeholder
        }

        fn CheckTierUp(&self, declared_func_index: i32, budget_used: i32, ool_label: *mut i32, frozen: &FreezeCacheState) {
            //Mock implementation.
        }
        fn LoadOldFramePointer(&self) -> Register {
            fp
        }
        fn CheckStackShrink(&self) {
            unimplemented!();
        }

        fn LoadInstanceDataFromFrame(&self, dst: Register) {
            self.Ld(dst, liftoff::GetInstanceDataOperand());
        }
        fn LoadTrustedPointer(&self, dst: Register, src_addr: Register, offset: i32, tag: IndirectPointerTag) {
            self.Ld(dst, MemOperand::new(src_addr, offset));
        }
        fn LoadFromInstance(&self, dst: Register, instance: Register, offset: i32, size: i32) {
            match size {
                1 => self.Lb(dst, MemOperand::new(instance, offset)),
                4 => self.Lw(dst, MemOperand::new(instance, offset)),
                8 => self.Ld(dst, MemOperand::new(instance, offset)),
                _ => unimplemented!(),
            }
        }
        fn LoadTaggedPointerFromInstance(&self, dst: Register, instance: Register, offset: i32) {
            self.Ld(dst, MemOperand::new(instance, offset));
        }
        fn SpillInstanceData(&self, instance: Register) {
            self.Sd(instance, liftoff::GetInstanceDataOperand());
        }
        fn ResetOSRTarget(&self) {}
        fn LoadTaggedPointer(
            &self,
            dst: Register,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: i32,
            protected_load_pc: *mut u32,
            needs_shift: bool,
        ) {
            let shift_amount = if !needs_shift { 0 } else { 3 };
            let src_op = liftoff::GetMemOp(
                self,
                src_addr,
                offset_reg,
                offset_imm,
                false,
                shift_amount,
            );
            self.Ld(dst, src_op);
        }
        fn LoadProtectedPointer(&self, dst: Register, src_addr: Register, offset_imm: i32) {
            self.LoadTaggedPointer(dst, src_addr, no_reg, offset_imm, std::ptr::null_mut(), false);
        }
        fn LoadFullPointer(&self, dst: Register, src_addr: Register, offset_imm: i32) {
            let src_op = liftoff::GetMemOp(self, src_addr, no_reg, offset_imm, false, 0);
            self.Ld(dst, src_op);
        }

        fn StoreTaggedPointer(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: i32,
            src: Register,
            pinned: LiftoffRegList,
            protected_store_pc: *mut u32,
            skip_write_barrier: i32,
        ) {
            //Mock
        }

        fn Load(
            &self,
            dst: LiftoffRegister,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            load_type: LoadType,
            protected_load_pc: *mut u32,
            is_load_mem: bool,
            i64_offset: bool,
            needs_shift: bool,
        ) {
            //Mock
        }

        fn Store(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            src: LiftoffRegister,
            store_type: StoreType,
            pinned: LiftoffRegList,
            protected_store_pc: *mut u32,
            is_store_mem: bool,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicLoad(
            &self,
            dst: LiftoffRegister,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            load_type: LoadType,
            pinned: LiftoffRegList,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicStore(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            src: LiftoffRegister,
            store_type: StoreType,
            pinned: LiftoffRegList,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicAdd(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicSub(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicAnd(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicOr(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicXor(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicExchange(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicCompareExchange(
            &self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            expected: LiftoffRegister,
            new_value: LiftoffRegister,
            result: LiftoffRegister,
            store_type: StoreType,
            i64_offset: bool,
        ) {
            //Mock
        }

        fn AtomicFence(&self) {
            //Mock
        }

        fn LoadCallerFrameSlot(
            &self,
            dst: LiftoffRegister,
            caller_slot_idx: u32,
            kind: ValueKind,
        ) {
            let src = MemOperand::new(fp, kSystemPointerSize * (caller_slot_idx as i32 + 1));
            liftoff::Load(self, dst, src, kind);
        }

        fn StoreCallerFrameSlot(
            &self,
            src: LiftoffRegister,
            caller_slot_idx: u32,
            kind: ValueKind,
            frame_pointer: Register,
        ) {
            let offset = kSystemPointerSize * (caller_slot_idx as i32 + 1);
            liftoff::Store(self, frame_pointer, offset, src, kind);
        }

        fn LoadReturnStackSlot(&self, dst: LiftoffRegister, offset: i32, kind: ValueKind) {
            liftoff::Load(self, dst, MemOperand::new(sp, offset), kind);
        }

        fn MoveStackValue(&self, dst_offset: u32, src_offset: u32, kind: ValueKind) {
            let scratch = kScratchReg;

            match kind {
                ValueKind::I32 | ValueKind::F32 => {
                    self.Lw(scratch, liftoff::GetStackSlot(src_offset as i32));
                    self.Sw(scratch, liftoff::GetStackSlot(dst_offset as i32));
                }
                ValueKind::I64 | ValueKind::RefNull | ValueKind::Ref | ValueKind::F64 => {
                    self.Ld(scratch, liftoff::GetStackSlot(src_offset as i32));
                    self.Sd(scratch, liftoff::GetStackSlot(dst_offset as i32));
                }
                ValueKind::S128 => unimplemented!(),
                _ => unimplemented!(),
            }
        }

        fn Move(&self, dst: Register, src: Register, kind: ValueKind) {
            if dst != src {
                // TODO(ksreten): Handle different sizes here.
                //Placeholder
            }
        }

        fn MoveDouble(&self, dst: DoubleRegister, src: DoubleRegister, kind: ValueKind) {
            if dst != src {
                // TODO(ksreten): Handle different sizes here.
                //Placeholder
            }
        }

        fn Spill(&mut self, offset: i32, reg: LiftoffRegister, kind: ValueKind) {
            self.RecordUsedSpillOffset(offset);
            let dst = liftoff::GetStackSlot(offset);
            match kind {
                ValueKind::I32 => self.Sw(reg.gp(), dst),
                ValueKind::I64 | ValueKind::Ref | ValueKind::RefNull => self.Sd(reg.gp(), dst),
                ValueKind::F32 =>