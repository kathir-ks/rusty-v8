// NOTE: This is a partial translation. Some parts, especially related to architecture-specific
// assembly and memory management, are difficult or impossible to directly translate to Rust
// and are marked with comments.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

// use std::optional::Optional; // Rust has Option

mod codegen {
    pub mod assembler {
        // Placeholder
        pub struct Assembler {}

        impl Assembler {
            pub fn sub_sp_32(&mut self, size: i32) {}
            pub fn pc_offset(&self) -> i32 { 0 }
            pub fn jmp_rel(&mut self, offset: i32) {}
            pub fn Nop(&mut self, size: i32) {}
            pub fn push(&mut self, src: Register) {}
            pub fn pop(&mut self, dst: Register) {}
            pub fn mov(&mut self, dst: Register, src: Register) {}
            pub fn mov_w(&mut self, dst: Operand, src: Register) {}
            pub fn sar(&mut self, dst: Register, imm: i32) {}
            pub fn mov_b(&mut self, dst: Operand, src: Register) {}

            pub fn sub(&mut self, dst: Register, imm: Immediate) {}
            pub fn CompareStackLimit(&mut self, stack_limit: Register, limit_kind: StackLimitKind) {}
            pub fn j(&mut self, cond: Condition, target: &Label) {}
            pub fn add(&mut self, dst: Register, imm: Immediate) {}
            pub fn wasm_call(&mut self, target: i64, reloc_info: RelocInfo) {}
            pub fn movzx_b(&mut self, dst: Register, src: Operand) {}
            pub fn movsx_b(&mut self, dst: Register, src: Operand) {}
            pub fn xor_(&mut self, dst: Register, src: Register) {}
            pub fn div(&mut self, rhs: Register) {}
            pub fn idiv(&mut self, rhs: Register) {}
            pub fn cdq(&mut self) {}
            pub fn test(&mut self, reg: Register, imm: Immediate) {}
            pub fn imul(&mut self, dst: Register, imm: i32) {}

             //Additions from the later section of the file
            pub fn movss(&mut self, dst: DoubleRegister, src: Operand) {}
            pub fn movsd(&mut self, dst: DoubleRegister, src: Operand) {}
            pub fn movdqu(&mut self, dst: DoubleRegister, src: Operand) {}
            pub fn AllocateStackSpace(&mut self, size: i32) {}
            pub fn movss_reg(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn movsd_reg(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Movaps(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Move(&mut self, dst: DoubleRegister, imm: i32) {}
            pub fn Andps(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Xorps(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Sqrtsd(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn cvtsi2ss(&mut self, dst: DoubleRegister, src: Register) {}
            pub fn cvtsd2ss(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn cvtss2sd(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Punpckldq(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Movd(&mut self, dst: Register, src: DoubleRegister) {}
            pub fn Movd_reg(&mut self, dst: DoubleRegister, src: Register) {}
            pub fn Pextrd(&mut self, dst: Register, src: DoubleRegister, imm: i32) {}
            pub fn Pinsrd(&mut self, dst: DoubleRegister, src: Register, imm: i32) {}
            pub fn Andpd(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn Xorpd(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
            pub fn setcc(&mut self, cond: Condition, dst: Register) {}
            pub fn jmp(&mut self, label: &Label) {}
            pub fn ShlPair(&mut self, high: Register, low: Register, amount: i32) {}
            pub fn SarPair(&mut self, high: Register, low: Register, amount: i32) {}
            pub fn ShrPair(&mut self, high: Register, low: Register, amount: i32) {}

            //More addition from atomic methods
            pub fn mfence(&mut self) {}

            // Atomic methods
            pub fn lock(&mut self) {}
            pub fn or_(&mut self, dst: Operand, src: Immediate) {}
            pub fn xchg_b(&mut self, reg: Register, mem: Operand) {}
            pub fn xchg_w(&mut self, reg: Register, mem: Operand) {}
            pub fn xchg(&mut self, reg: Register, mem: Operand) {}
            pub fn xadd_b(&mut self, mem: Operand, reg: Register) {}
            pub fn xadd_w(&mut self, mem: Operand, reg: Register) {}
            pub fn xadd(&mut self, mem: Operand, reg: Register) {}
            pub fn cmpxchg_b(&mut self, mem: Operand, reg: Register) {}
            pub fn cmpxchg_w(&mut self, mem: Operand, reg: Register) {}
            pub fn cmpxchg(&mut self, mem: Operand, reg: Register) {}
            pub fn cmpxchg8b(&mut self, mem: Operand) {}
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum RelocInfo {
            WASM_STUB_CALL,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum StackLimitKind {
            kRealStackLimit
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Condition {
            equal,
            not_equal,
            above_equal,
            above,
            below_equal,
            below,
            zero,
            not_zero,
            negative,
            above_equal_unsigned,
            above_unsigned,
            below_equal_unsigned,
            below_unsigned,
            parity_even,
            parity_odd,
            greater_equal,
            greater,
            less_equal,
            less,
            unsigned_greater,
            unsigned_greater_equal,
            unsigned_less,
            unsigned_less_equal
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Immediate(pub i32);

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Operand(pub Register, pub i32); //Simplified

        impl Operand {
            pub fn new(base: Register, offset: i32) -> Self {
                Operand(base, offset)
            }
        }
    }
    pub mod interface_descriptors_inl {
        // Placeholder
    }
}

mod heap {
    pub mod mutable_page_metadata {
        // Placeholder
        pub struct MemoryChunk {}
    }
}

mod wasm {
    pub mod baseline {
        pub mod liftoff_assembler {
            // Placeholder
        }
        pub mod liftoff_register {
            // Placeholder
        }
    }
    pub mod object_access {
        // Placeholder
    }
    pub mod simd_shuffle {
        // Placeholder
    }
    pub mod value_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum ValueKind {
            kI32,
            kI64,
            kF32,
            kF64,
            kS128,
            kVoid,
            kTop,
            kBottom,
            kI8,
            kI16,
            kF16,
            kRefNull,
            kRef
        }
    }
    pub mod wasm_linkage {
        // Placeholder
    }
    pub mod wasm_objects {
        // Placeholder
    }
}

mod v8_internal {
    pub mod wasm {
        use super::codegen::assembler::{Assembler, Condition, Immediate, Operand, RelocInfo, StackLimitKind};
        use super::wasm::value_type::ValueKind;

        // NOTE: This is simplified. Many details of V8's internal register usage,
        // memory management and calling conventions are abstracted.

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Register(pub i32); //Simplified register

        impl Register {
            pub fn is_byte_register(&self) -> bool { false }
            pub fn is_valid(&self) -> bool { true }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct DoubleRegister(pub i32);

        const eax: Register = Register(0);
        const ecx: Register = Register(1);
        const edx: Register = Register(2);
        const ebx: Register = Register(3);
        const esp: Register = Register(4);
        const ebp: Register = Register(5);
        const esi: Register = Register(6);
        const edi: Register = Register(7);
        const no_reg: Register = Register(-1);
        const kReturnRegister0: Register = eax;
        const kRootRegister: Register = ebx;
        const kLiftoffFrameSetupFunctionReg: Register = edi;

        const xmm0: DoubleRegister = DoubleRegister(0);
        const xmm7: DoubleRegister = DoubleRegister(7);

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum RegPairHalf {
            kLowWord,
            kHighWord,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct LiftoffRegister {
            low_gp: Register,
            high_gp: Register, //For I64
            fp: DoubleRegister
        }

        impl LiftoffRegister {
            pub fn new(low_gp: Register, high_gp: Register, fp: DoubleRegister) -> Self {
                LiftoffRegister { low_gp, high_gp, fp }
            }

             pub fn ForPair(low_gp: Register, high_gp: Register) -> Self{
                LiftoffRegister { low_gp, high_gp, fp: DoubleRegister(-1) }
            }

            pub fn gp(&self) -> Register { self.low_gp }
            pub fn low(&self) -> LiftoffRegister { LiftoffRegister{ low_gp: self.low_gp, high_gp: Register(-1), fp: DoubleRegister(-1)} }
            pub fn high(&self) -> LiftoffRegister { LiftoffRegister{ low_gp: self.high_gp, high_gp: Register(-1), fp: DoubleRegister(-1)} }
            pub fn low_gp(&self) -> Register { self.low_gp }
            pub fn high_gp(&self) -> Register { self.high_gp }
            pub fn fp(&self) -> DoubleRegister { self.fp }
            pub fn is_gp_pair(&self) -> bool { self.high_gp.0 >= 0 }
            pub fn is_pair(&self) -> bool { self.high_gp.0 >= 0 }
        }

        impl From<Register> for LiftoffRegister {
            fn from(reg: Register) -> Self {
                LiftoffRegister { low_gp: reg, high_gp: Register(-1), fp: DoubleRegister(-1) }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct LiftoffRegList(u32);

        impl LiftoffRegList {
            pub fn new(bits: u32) -> Self {
                LiftoffRegList(bits)
            }

            pub fn FromBits<T>(input: T) -> Self where T: GetBits{
                LiftoffRegList(input.bits())
            }

            pub fn set(&mut self, reg: Register) {
                self.0 |= 1 << reg.0;
            }

            pub fn clear(&mut self, reg: LiftoffRegister) {
                self.0 &= !(1 << reg.gp().0);
            }

            pub fn has(&self, reg: LiftoffRegister) -> bool {
                (self.0 & (1 << reg.gp().0)) != 0
            }

            pub fn MaskOut(&self, other: LiftoffRegList) -> Self {
                LiftoffRegList(self.0 & !other.0)
            }

            pub fn GetGpList(&self) -> RegList {
                RegList{bits: self.0}
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct RegList{
            bits: u32
        }

        impl RegList{
            pub fn last(&self) -> Register {
                Register(self.bits.trailing_zeros() as i32)
            }

            pub fn is_empty(&self) -> bool {
                self.bits == 0
            }
        }

        pub trait GetBits {
            fn bits(&self) -> u32;
        }

        impl GetBits for [Register; 3]{
            fn bits(&self) -> u32 {
                let mut bits = 0;
                for reg in self{
                    bits |= 1 << reg.0;
                }
                bits
            }
        }

        const kGpReg: i32 = 0;
        const kFpReg: i32 = 1;
        const kSystemPointerSize: i32 = 4;
        const kTaggedSize: i32 = kSystemPointerSize;
        const kInt32Size: i32 = 4;
        const kStackSlotSize: i32 = 8; // Not used in rust code
        const kMinInt: i32 = i32::min_value();
        const kSubSpSize: i32 = 6;  // 6 bytes for "sub esp, <imm32>"
        const kSmiTagMask: i32 = 1;
        const kFeedbackVectorOffset: i32 = 0;
        const kAvailableSpace: i32 = 64;

        const kLiftoffAssemblerGpCacheRegs: RegList = RegList{ bits: 0b11111111 }; //Example register set
        const kGpCacheRegList: LiftoffRegList = LiftoffRegList::FromBits(RegList{bits: 0b11111111}); //Example register set
        const kByteRegs: LiftoffRegList = LiftoffRegList::FromBits([eax, ecx, edx]);

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum IndirectPointerTag {}

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SkipWriteBarrier {
            kDoSkipWriteBarrier,
            kDoNotSkipWriteBarrier
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SmiCheckMode {
            kJumpOnSmi,
            kJumpOnNotSmi,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum AbortReason {
            kUnexpectedReturnFromWasmTrap
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Builtin {
            kWasmLiftoffFrameSetup,
            kWasmHandleStackOverflow,
            kWasmStackOverflow
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SaveFPRegsMode {
            kSave,
            kDontSave
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum StubCallMode {
            kCallWasmRuntimeStub
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LoadType {
            kI32Load8U,
            kI32Load8S,
            kI64Load8U,
            kI64Load8S,
            kI32Load16U,
            kI32Load16S,
            kI64Load16U,
            kI64Load16S,
            kI32Load,
            kI64Load32U,
            kI64Load32S,
            kI64Load,
            kF32Load,
            kF64Load,
            kS128Load,
            kF32LoadF16
        }

        impl LoadType{
            pub fn value_type(&self) -> ValueKind{
                match self{
                    LoadType::kI32Load8U | LoadType::kI32Load8S | LoadType::kI32Load16U |
                    LoadType::kI32Load16S | LoadType::kI32Load => ValueKind::kI32,
                    LoadType::kI64Load8U | LoadType::kI64Load8S | LoadType::kI64Load16U |
                    LoadType::kI64Load16S | LoadType::kI64Load32U | LoadType::kI64Load32S |
                    LoadType::kI64Load => ValueKind::kI64,
                    LoadType::kF32Load | LoadType::kF32LoadF16 => ValueKind::kF32,
                    LoadType::kF64Load => ValueKind::kF64,
                    LoadType::kS128Load => ValueKind::kS128
                }
            }

            pub fn size_log_2(&self) -> i32 {
                match self{
                     LoadType::kI32Load8U | LoadType::kI32Load8S |
                    LoadType::kI64Load8U | LoadType::kI64Load8S => 0,
                     LoadType::kI32Load16U | LoadType::kI32Load16S |
                    LoadType::kI64Load16U | LoadType::kI64Load16S => 1,
                     LoadType::kI32Load | LoadType::kI64Load32U | LoadType::kI64Load32S => 2,
                    LoadType::kI64Load | LoadType::kF32Load | LoadType::kF64Load | LoadType::kS128Load
                    | LoadType::kF32LoadF16 => 3,
                }
            }
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
            kF32Store,
            kF64Store,
            kS128Store,
            kF32StoreF16
        }

        impl StoreType{
            pub fn value_type(&self) -> ValueKind{
                match self{
                    StoreType::kI32Store8 | StoreType::kI64Store8 | StoreType::kI32Store16 |
                    StoreType::kI64Store16 | StoreType::kI32Store | StoreType::kI64Store32 => ValueKind::kI32,
                    StoreType::kI64Store => ValueKind::kI64,
                    StoreType::kF32Store | StoreType::kF32StoreF16 => ValueKind::kF32,
                    StoreType::kF64Store => ValueKind::kF64,
                    StoreType::kS128Store => ValueKind::kS128
                }
            }

             pub fn size(&self) -> i32 {
                match self{
                    StoreType::kI32Store8 | StoreType::kI64Store8 => 1,
                    StoreType::kI32Store16 | StoreType::kI64Store16 => 2,
                    StoreType::kI32Store | StoreType::kI64Store32 => 4,
                    StoreType::kI64Store |  StoreType::kF32Store | StoreType::kF32StoreF16 => 4,
                    StoreType::kF64Store => 8,
                    StoreType::kS128Store => 16,

                }
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct WasmValue {
            value: i64,
            kind: ValueKind
        }

        impl WasmValue {
            pub fn new(value: i64, kind: ValueKind) -> Self {
                WasmValue { value, kind }
            }

            pub fn to_i32(&self) -> i32 {
                self.value as i32
            }
            pub fn to_i64(&self) -> i64 {
                self.value
            }
            pub fn to_f32_boxed(&self) -> BoxedFloat {
                BoxedFloat{bits: self.value as u32}
            }
            pub fn to_f64_boxed(&self) -> BoxedDouble {
                BoxedDouble{bits: self.value as u64}
            }
            pub fn type_(&self) -> ValueKind{
                self.kind
            }
        }

        struct BoxedFloat{
            bits: u32
        }

        impl BoxedFloat{
            pub fn get_bits(&self) -> u32{
                self.bits
            }
        }

        struct BoxedDouble{
            bits: u64
        }

         impl BoxedDouble{
            pub fn get_bits(&self) -> u64{
                self.bits
            }
        }

        pub struct AssemblerOptions{}

        pub struct ExternalAssemblerBuffer{
            buffer_start_: *mut u8,
            available_space: i32,
        }

        impl ExternalAssemblerBuffer{
            pub fn new(buffer_start_: *mut u8, available_space: i32) -> Self{
                ExternalAssemblerBuffer{
                    buffer_start_: buffer_start_,
                    available_space: available_space
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum ScaleFactor{
            times_1
        }

        pub struct LiftoffAssembler {
            assembler: Assembler,
            cache_state_: CacheState,
        }

        impl LiftoffAssembler {
            pub fn new() -> Self {
                LiftoffAssembler {
                    assembler: Assembler {},
                    cache_state_: CacheState::new()
                }
            }

            pub fn PrepareStackFrame(&mut self) -> i32 {
                let offset = self.assembler.pc_offset();
                self.assembler.sub_sp_32(0);
                assert_eq!(6, self.assembler.pc_offset() - offset);
                return offset;
            }

             pub fn CallFrameSetupStub(&mut self, declared_function_index: i32) {
                self.LoadConstant(LiftoffRegister::from(kLiftoffFrameSetupFunctionReg), WasmValue::new(declared_function_index as i64, ValueKind::kI32));
                self.CallBuiltin(Builtin::kWasmLiftoffFrameSetup);
            }

             pub fn PrepareTailCall(&mut self, num_callee_stack_params: i32, stack_param_delta: i32) {
                // Push the return address and frame pointer to complete the stack frame.
                self.push(Operand::new(ebp, 4));
                self.push(Operand::new(ebp, 0));

                // Shift the whole frame upwards.
                let scratch = eax;
                self.push(scratch);
                let slot_count = num_callee_stack_params + 2;
                for i in (1..=slot_count).rev() {
                    self.assembler.mov(scratch, Operand::new(esp, i * 4));
                    self.assembler.mov(Operand::new(ebp, (i - stack_param_delta - 1) * 4), scratch);
                }
                self.assembler.pop(scratch);

                // Set the new stack and frame pointers.
                // NOTE: lea instruction not implemented in the example Assembler
                //self.lea(esp, Operand::new(ebp, -stack_param_delta * 4));
                self.assembler.pop(ebp);
            }

            pub fn AlignFrameSize(&mut self) {}

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
                let frame_size = self.GetTotalFrameSize() - 2 * kSystemPointerSize;
                // The frame setup builtin also pushes the feedback vector.
                let mut frame_size = frame_size;
                if feedback_vector_slot {
                    frame_size -= kSystemPointerSize;
                }
                assert_eq!(0, frame_size % kSystemPointerSize);

                // We can't run out of space when patching, just pass anything big enough to
                // not cause the assembler to try to grow the buffer.

                // Placeholder implementation, actual ExternalAssemblerBuffer and Assembler not implemented
                let mut patching_assembler = Assembler{};// Assembler::new(AssemblerOptions {},ExternalAssemblerBuffer::new(std::ptr::null_mut(), kAvailableSpace));

                if frame_size < 4 * 1024 {
                    // This is the standard case for small frames: just subtract from SP and be
                    // done with it.
                    patching_assembler.sub_sp_32(frame_size);
                    assert_eq!(6, patching_assembler.pc_offset());
                    return;
                }
                panic!("Frame size exceeds 4KB, extended stack check is not implemented in this example.");
            }

            pub fn FinishCode(&mut self) {}
            pub fn AbortCompilation(&mut self) {}

            pub fn StaticStackFrameSize() -> i32 {
                0 //WasmLiftoffFrameConstants::kFeedbackVectorOffset
            }

            pub fn SlotSizeForType(&self, kind: ValueKind) -> i32 {
                self.value_kind_full_size(kind)
            }

            pub fn NeedsAlignment(&self, kind: ValueKind) -> bool {
                self.is_reference(kind)
            }

            pub fn CheckTierUp(&mut self, declared_func_index: i32, budget_used: i32, ool_label: &mut Label, frozen: &FreezeCacheState) {
                let mut temps = liftoff::CacheStatePreservingTempRegisters::new(self, LiftoffRegList::new(0));
                let budget_array = temps.Acquire();

                let mut instance_data = self.cache_state_.cached_instance_data;
                if instance_data == no_reg {
                    instance_data = budget_array;  // Reuse the temp register.
                    self.LoadInstanceDataFromFrame(instance_data);
                }

                let kArrayOffset = 0; //wasm::ObjectAccess::ToTagged(WasmTrustedInstanceData::kTieringBudgetArrayOffset);
                self.assembler.mov(budget_array, Operand::new(instance_data, kArrayOffset));

                let array_offset = kInt32Size * declared_func_index;
                self.assembler.sub(Operand::new(budget_array, array_offset), Immediate(budget_used));
                self.assembler.j(Condition::negative, ool_label);
            }

            pub fn LoadOldFramePointer(&mut self) -> Register {
                if false {
                    return ebp;
                }
                let mut done = Label{};
                let mut call_runtime = Label{};
                self.assembler.cmp(Operand::new(ebp, 0), Immediate(0));
                    // StackFrame::TypeToMarker(StackFrame::WASM_SEGMENT_START)));
                self.assembler.j(Condition::equal, &mut call_runtime);
                let mut old_fp = self.GetUnusedRegister(0, LiftoffRegList::new(0));
                self.assembler.mov(old_fp.gp(), ebp);
                self.assembler.jmp(&mut done);

                self.bind(&mut call_runtime);
                let mut regs_to_save = self.cache_state().used_registers;
                //self.PushRegisters(regs_to_save);
                //self.PrepareCallCFunction(1, eax);
                //MacroAssembler::Move(Operand(esp, 0 * kSystemPointerSize), Immediate(ExternalReference::isolate_address()));
                //self.CallCFunction(ExternalReference::wasm_load_old_fp(), 1);
                //if (old_fp.gp() != kReturnRegister0) {
                //    self.assembler.mov(old_fp.gp(), kReturnRegister0);
                //}
                //self.PopRegisters(regs_to_save);

                self.bind(&mut done);
                return old_fp.gp();
            }

             pub fn CheckStackShrink(&mut self) {
                let mut regs_to_save = LiftoffRegList::new(0);
                 //for (auto reg : kGpReturnRegisters) regs_to_save.set(reg);
                //for (auto reg : kFpReturnRegisters) regs_to_save.set(reg);
                self.assembler.cmp(Operand::new(ebp, 0), Immediate(0));
                   // StackFrame::TypeToMarker(StackFrame::WASM_SEGMENT_START)));
                let mut done = Label{};
                self.assembler.j(Condition::not_equal, &mut done);
                //self.PushRegisters(regs_to_save);
                //self.PrepareCallCFunction(1, kReturnRegister0);
                //MacroAssembler::Move(Operand(esp, 0 * kSystemPointerSize), Immediate(ExternalReference::isolate_address()));
                //self.CallCFunction(ExternalReference::wasm_shrink_stack(), 1);
                // Restore old ebp. We don't need to restore old esp explicitly, because
                // it will be restored from ebp in LeaveFrame before return.
                self.assembler.mov(ebp, kReturnRegister0);
                //self.PopRegisters(regs_to_save);
                self.bind(&mut done);
            }

            pub fn LoadConstant(&mut self, reg: LiftoffRegister, value: WasmValue) {
                match value.type_() {
                    ValueKind::kI32 => {
                        self.assembler.Move(reg.gp(), value.to_i32());
                    }
                    ValueKind::kI64 => {
                        let low_word = value.to_i64() as i32;
                        let high_word = (value.to_i64() >> 32) as i32;
                        self.assembler.Move(reg.low_gp(), low_word);
                        self.assembler.Move(reg.high_gp(), high_word);
                    }
                    ValueKind::kF32 => {
                        self.assembler.Move(reg.fp(), value.to_f32_boxed().get_bits() as i32);
                    }
                    ValueKind::kF64 => {
                        self.assembler.Move(reg.fp(), value.to_f64_boxed().get_bits() as i32);
                    }
                    _ => unreachable!(),
                }
            }

             pub fn LoadInstanceDataFromFrame(&mut self, dst: Register) {
                self.assembler.mov(dst, liftoff::GetInstanceDataOperand());
            }

             pub fn LoadTrustedPointer(&mut self, dst: Register, src_addr: Register, offset: i32, tag: IndirectPointerTag) {
                self.assembler.mov(dst, Operand::new(src_addr, offset));
            }

             pub fn LoadFromInstance(&mut self, dst: Register, instance: Register, offset: i32, size: i32) {
                assert!(offset >= 0);
                let src = Operand::new(instance, offset);
                match size {
                    1 => {
                        self.assembler.movzx_b(dst, src);
                    }
                    4 => {
                        self.assembler.mov(dst, src);
                    }
                    _ => unimplemented!(),
                }
            }

             pub fn LoadTaggedPointerFromInstance(&mut self, dst: Register, instance: Register, offset: i32) {
                assert_eq!(kTaggedSize, kSystemPointerSize);
                self.assembler.mov(dst, Operand::new(instance, offset));
            }

             pub fn SpillInstanceData(&mut self, instance: Register) {
                self.assembler.mov(liftoff::GetInstanceDataOperand(), instance);
            }

            pub fn ResetOSRTarget(&mut self) {}

            pub fn LoadTaggedPointer(&mut self, dst: Register, src_addr: Register, offset_reg: Register, offset_imm: i32, protected_load_pc: *mut u32, needs_shift: bool) {
                assert!(offset_imm >= 0);
                assert_eq!(kTaggedSize, kInt32Size);
                self.Load(LiftoffRegister::from(dst), src_addr, offset_reg, offset_imm as u32, LoadType::kI32Load, protected_load_pc, false, false, needs_shift);
            }

            pub fn LoadProtectedPointer(&