// Converted from V8 C++ source files:
// Header: assembler-mips64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
pub mod assembler_mips64_inl {
use crate::codegen::assembler::Assembler;
use crate::codegen::flush_instruction_cache::FlushInstructionCache;
use crate::codegen::mips64::assembler_mips64::*;
use crate::debug::debug::Debug;
use crate::objects::objects_inl::*;
use std::mem;
use std::ptr;
use crate::execution::loong64::simulator_loong64::Instruction;
use crate::codegen::register::Register;
use crate::codegen::register::MSAControlRegister;
use crate::codegen::interface_descriptors_mips64_inl::InterfaceDescriptor;
use crate::codegen::interface_descriptors_mips64_inl::kDescriptorIsOptional;
use crate::codegen::interface_descriptors_mips64_inl::kRegisterPassed;
use crate::codegen::interface_descriptors_mips64_inl::kInvalidDescriptor;
use crate::codegen::interface_descriptors_mips64_inl::RegisterArray;
use crate::codegen::code_reference::CodeReference;
use crate::codegen::code_reference::InstructionStream;
use crate::codegen::code_reference::CodeBytes;
use crate::codegen::code_reference::CodeRange;
use crate::codegen::compilation_cache::CompilationCache;
use crate::ast::modules::Module;
use crate::ast::ast_value::AstValue;
use crate::ast::ast_source_ranges::AstNodeSourceRangesMethods;
use crate::compiler::backend::jump_threading::RpoNumber;
use crate::strings::uri::T;
use crate::codegen::ia32::assembler_ia32_inl::Builtin;
use crate::sandbox::js_dispatch_table::JSDispatchHandle;

    pub enum ICacheFlushMode {
        FLUSH,
        SKIP_ICACHE_FLUSH,
    }

    pub struct CpuFeatures {}

    impl CpuFeatures {
        pub fn SupportsOptimizer() -> bool {
            Self::IsSupported(FPU)
        }

        fn IsSupported(_feature: Feature) -> bool {
            true
        }
    }

    // -----------------------------------------------------------------------------
    // Operand and MemOperand.
    impl Operand {
        pub fn is_reg(&self) -> bool {
            self.rm_.is_valid()
        }

        pub fn immediate(&self) -> i64 {
            assert!(!self.is_reg());
            assert!(!self.IsHeapNumberRequest());
            self.value_.immediate
        }

        pub fn IsHeapNumberRequest(&self) -> bool {
            false
        }
    }

    // -----------------------------------------------------------------------------
    // RelocInfo.

    pub struct WritableRelocInfo {
        rmode_: RelocInfo_Mode,
        pc_: Address,
        jit_allocation_: WritableJitAllocation,
        constant_pool_: Address,
    }

    impl WritableRelocInfo {
        pub fn apply(&mut self, delta: intptr_t) {
            if self.IsInternalReference() || self.IsInternalReferenceEncoded() {
                // Absolute code pointer inside code object moves with the code object.
                Assembler::RelocateInternalReference(
                    self.rmode_,
                    self.pc_,
                    delta,
                    &mut self.jit_allocation_,
                );
            }
        }

        pub fn set_target_object(
            &mut self,
            target: Tagged<HeapObject>,
            icache_flush_mode: ICacheFlushMode,
        ) {
            assert!(self.IsCodeTarget() || self.IsFullEmbeddedObject());
            Assembler::set_target_address_at(
                self.pc_,
                self.constant_pool_,
                target.ptr(),
                &mut self.jit_allocation_,
                icache_flush_mode,
            );
        }

        pub fn set_target_external_reference(
            &mut self,
            target: Address,
            icache_flush_mode: ICacheFlushMode,
        ) {
            assert_eq!(self.rmode_, RelocInfo_Mode::EXTERNAL_REFERENCE);
            Assembler::set_target_address_at(
                self.pc_,
                self.constant_pool_,
                target,
                &mut self.jit_allocation_,
                icache_flush_mode,
            );
        }

        pub fn set_wasm_code_pointer_table_entry(
            &mut self,
            target: WasmCodePointer,
            icache_flush_mode: ICacheFlushMode,
        ) {
            assert_eq!(self.rmode_, RelocInfo_Mode::WASM_CODE_POINTER_TABLE_ENTRY);
            Assembler::set_uint32_constant_at(
                self.pc_,
                self.constant_pool_,
                target.value(),
                &mut self.jit_allocation_,
                icache_flush_mode,
            );
        }

        fn IsInternalReference(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::INTERNAL_REFERENCE
        }

        fn IsInternalReferenceEncoded(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::INTERNAL_REFERENCE_ENCODED
        }

        fn IsCodeTarget(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::CODE_TARGET
        }

        fn IsFullEmbeddedObject(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::FULL_EMBEDDED_OBJECT
        }

        fn HasTargetAddressAddress(&self) -> bool {
            true
        }

        fn target_address_address(&self) -> Address {
            self.pc_ + Assembler::kInstructionsFor64BitConstant as isize * kInstrSize as isize
        }
    }

    pub struct RelocInfo {
        rmode_: RelocInfo_Mode,
        pc_: Address,
        constant_pool_: Address,
    }

    impl RelocInfo {
        pub fn target_address(&self) -> Address {
            assert!(
                self.IsCodeTarget()
                    || self.IsWasmCall()
                    || self.IsWasmStubCall()
            );
            Assembler::target_address_at(self.pc_, self.constant_pool_)
        }

        pub fn target_address_address(&self) -> Address {
            assert!(self.HasTargetAddressAddress());
            self.pc_ + Assembler::kInstructionsFor64BitConstant as isize * kInstrSize as isize
        }

        pub fn constant_pool_entry_address(&self) -> Address {
            unreachable!()
        }

        pub fn target_address_size(&self) -> i32 {
            Assembler::kSpecialTargetSize
        }

        pub fn target_object(&self, _cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
            assert!(self.IsCodeTarget() || self.IsFullEmbeddedObject());
            let addr = Assembler::target_address_at(self.pc_, self.constant_pool_);
            unsafe {
                Tagged::<HeapObject>::cast(Tagged::<Object>::unchecked_cast(addr as *mut _))
            }
        }

        pub fn target_object_handle(&self, _origin: &mut Assembler) -> DirectHandle<HeapObject> {
            assert!(self.IsCodeTarget() || self.IsFullEmbeddedObject());
            let addr = Assembler::target_address_at(self.pc_, self.constant_pool_);
            DirectHandle::<HeapObject>::from_slot(addr as *mut Address)
        }

        pub fn target_external_reference(&self) -> Address {
            assert_eq!(self.rmode_, RelocInfo_Mode::EXTERNAL_REFERENCE);
            Assembler::target_address_at(self.pc_, self.constant_pool_)
        }

        pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
            assert_eq!(self.rmode_, RelocInfo_Mode::WASM_CODE_POINTER_TABLE_ENTRY);
            WasmCodePointer {
                value_: Assembler::uint32_constant_at(self.pc_, self.constant_pool_),
            }
        }

        pub fn target_internal_reference(&self) -> Address {
            if self.rmode_ == RelocInfo_Mode::INTERNAL_REFERENCE {
                unsafe { *(self.pc_ as *mut Address) }
            } else {
                // Encoded internal references are j/jal instructions.
                assert_eq!(self.rmode_, RelocInfo_Mode::INTERNAL_REFERENCE_ENCODED);
                let instr = Assembler::instr_at(self.pc_ + 0 * kInstrSize as isize);
                let instr = instr & kImm26Mask;
                let imm28 = instr << 2;
                let segment = self.pc_ & !kImm28Mask as isize;
                segment | imm28 as isize
            }
        }

        pub fn target_internal_reference_address(&self) -> Address {
            assert!(
                self.rmode_ == RelocInfo_Mode::INTERNAL_REFERENCE
                    || self.rmode_ == RelocInfo_Mode::INTERNAL_REFERENCE_ENCODED
            );
            self.pc_
        }

        pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
            assert_eq!(self.rmode_, RelocInfo_Mode::JS_DISPATCH_HANDLE);
            unsafe { ptr::read_unaligned(self.pc_ as *const JSDispatchHandle) }
        }

        pub fn target_builtin_at(&self, _origin: &mut Assembler) -> Builtin {
            unreachable!()
        }

        pub fn target_off_heap_target(&self) -> Address {
            assert!(self.IsOffHeapTarget());
            Assembler::target_address_at(self.pc_, self.constant_pool_)
        }

        fn IsCodeTarget(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::CODE_TARGET
        }

        fn IsWasmCall(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::WASM_CALL
        }

        fn IsWasmStubCall(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::WASM_STUB_CALL
        }

        fn IsFullEmbeddedObject(&self) -> bool {
            self.rmode_ == RelocInfo_Mode::FULL_EMBEDDED_OBJECT
        }

        fn IsOffHeapTarget(&self) -> bool {
            match self.rmode_ {
                RelocInfo_Mode::EMBEDDED_OBJECT
                | RelocInfo_Mode::EXTERNAL_REFERENCE
                | RelocInfo_Mode::FULL_EMBEDDED_OBJECT
                | RelocInfo_Mode::CODE_TARGET => true,
                _ => false,
            }
        }

        fn HasTargetAddressAddress(&self) -> bool {
            true
        }
    }

    impl Assembler {
        pub fn deserialization_special_target_size(_instruction_payload: Address) -> i32 {
            Self::kSpecialTargetSize
        }

        pub fn set_target_internal_reference_encoded_at(
            pc: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
        ) {
            // Encoded internal references are j/jal instructions.
            let mut instr = Assembler::instr_at(pc + 0 * kInstrSize as isize);

            let imm28 = target & kImm28Mask as isize;

            instr &= !kImm26Mask;
            let imm26 = imm28 >> 2;
            assert!(Self::is_uint26(imm26 as u64));

            Self::instr_at_put(pc, instr | (imm26 & kImm26Mask), jit_allocation);
            // Currently used only by deserializer, and all code will be flushed
            // after complete deserialization, no need to flush on each reference.
        }

        pub fn deserialization_set_target_internal_reference_at(
            pc: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
            mode: RelocInfo_Mode,
        ) {
            if mode == RelocInfo_Mode::INTERNAL_REFERENCE_ENCODED {
                assert!(Self::IsJ(Self::instr_at(pc)));
                Self::set_target_internal_reference_encoded_at(pc, target, jit_allocation);
            } else {
                assert_eq!(mode, RelocInfo_Mode::INTERNAL_REFERENCE);
                jit_allocation.WriteUnalignedValue::<Address>(pc, target);
            }
        }

        pub fn uint32_constant_at(pc: Address, _constant_pool: Address) -> u32 {
            let instr0 = Self::instr_at(pc);
            let instr1 = Self::instr_at(pc + 1 * kInstrSize as isize);

            assert!((Self::GetOpcodeField(instr0) == LUI) && (Self::GetOpcodeField(instr1) == ORI));

            // Assemble the 32 bit value.
            let upper16 = (Self::GetImmediate16(instr0) as u32) << 16;
            let lower16 = Self::GetImmediate16(instr1) as u32;
            upper16 | lower16
        }

        pub fn set_uint32_constant_at(
            pc: Address,
            _constant_pool: Address,
            new_constant: u32,
            jit_allocation: &mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            let instr1 = Self::instr_at(pc + kInstrSize as isize);
            let rt_code = Self::GetRt(instr1);

            #[cfg(debug_assertions)]
            {
                // Check we have the result from a li macro-instruction.
                let instr0 = Self::instr_at(pc);
                assert!((Self::GetOpcodeField(instr0) == LUI)
                    && (Self::GetOpcodeField(instr1) == ORI)
                    && (Self::GetRt(instr0) == rt_code));
            }

            // Must use 2 instructions to insure patchable 32-bit value.
            // lui rt, upper-16.
            // ori rt, rt, lower-16.
            let new_instr0 =
                LUI | ((rt_code as Instr) << kRtShift) | (((new_constant >> 16) & kImm16Mask as u32) as Instr);
            let new_instr1 = ORI
                | ((rt_code as Instr) << kRtShift)
                | ((rt_code as Instr) << kRsShift)
                | ((new_constant & kImm16Mask as u32) as Instr);
            Self::instr_at_put(pc, new_instr0, jit_allocation);
            Self::instr_at_put(pc + kInstrSize as isize, new_instr1, jit_allocation);

            if let ICacheFlushMode::FLUSH = icache_flush_mode {
                FlushInstructionCache(pc, 2 * kInstrSize as i32);
            }
        }

        // -----------------------------------------------------------------------------
        // Assembler.

        pub fn CheckBuffer(&mut self) {
            if self.buffer_space() <= Self::kGap {
                self.GrowBuffer();
            }
        }

        pub fn CheckForEmitInForbiddenSlot(&mut self) {
            if !self.is_buffer_growth_blocked() {
                self.CheckBuffer();
            }
            if self.IsPrevInstrCompactBranch() {
                // Nop instruction to precede a CTI in forbidden slot:
                let nop = SPECIAL | SLL;
                unsafe {
                    *(self.pc_ as *mut Instr) = nop;
                }
                self.pc_ += kInstrSize as isize;

                self.ClearCompactBranchState();
            }
        }

        pub fn EmitHelper(&mut self, x: Instr, is_compact_branch: CompactBranchType) {
            if self.IsPrevInstrCompactBranch() {
                if Instruction::IsForbiddenAfterBranchInstr(x) {
                    // Nop instruction to precede a CTI in forbidden slot:
                    let nop = SPECIAL | SLL;
                    unsafe {
                        *(self.pc_ as *mut Instr) = nop;
                    }
                    self.pc_ += kInstrSize as isize;
                }
                self.ClearCompactBranchState();
            }
            unsafe {
                *(self.pc_ as *mut Instr) = x;
            }
            self.pc_ += kInstrSize as isize;
            if let CompactBranchType::COMPACT_BRANCH = is_compact_branch {
                self.EmittedCompactBranchInstruction();
            }
            self.CheckTrampolinePoolQuick();
        }

        fn emit_helper_u8(&mut self, x: u8) {
            unsafe {
                *(self.pc_ as *mut u8) = x;
            }
            self.pc_ += std::mem::size_of::<u8>() as isize;
            if (self.pc_ as i64) % kInstrSize == 0 {
                self.CheckTrampolinePoolQuick();
            }
        }

        fn emit_helper<T>(&mut self, x: T) {
            unsafe {
                *(self.pc_ as *mut T) = x;
            }
            self.pc_ += std::mem::size_of::<T>() as isize;
            self.CheckTrampolinePoolQuick();
        }

        pub fn emit(&mut self, x: Instr, is_compact_branch: CompactBranchType) {
            if !self.is_buffer_growth_blocked() {
                self.CheckBuffer();
            }
            self.EmitHelper(x, is_compact_branch);
        }

        pub fn emit_u64(&mut self, data: u64) {
            self.CheckForEmitInForbiddenSlot();
            self.emit_helper(data);
        }

        fn ClearCompactBranchState(&mut self) {}
        fn EmittedCompactBranchInstruction(&mut self) {}
        fn is_buffer_growth_blocked(&self) -> bool { false }
        fn buffer_space(&self) -> usize { 1024 }
        fn GrowBuffer(&mut self) {}
        fn CheckTrampolinePoolQuick(&mut self) {}
        fn instr_at_put(pc: Address, instr: Instr, jit_allocation: &mut WritableJitAllocation) {}

        pub const kGap: usize = 16;
        pub const kSpecialTargetSize: i32 = 8;
        pub const kInstructionsFor64BitConstant: i32 = 2;

        pub fn IsPrevInstrCompactBranch(&self) -> bool {
            false
        }
    }

    pub struct EnsureSpace<'a> {
        assembler: &'a mut Assembler,
    }

    impl<'a> EnsureSpace<'a> {
        pub fn new(assembler: &'a mut Assembler) -> Self {
            assembler.CheckBuffer();
            Self { assembler }
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum CompactBranchType {
        DEFAULT,
        COMPACT_BRANCH,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Feature {
        FPU,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RelocInfo_Mode {
        NO_INFO,
        LITERAL_CONSTANT,
        CODE_TARGET,
        JS_FUNCTION_ENTRY,
        EMBEDDED_OBJECT,
        EXTERNAL_REFERENCE,
        INTERNAL_REFERENCE,
        INTERNAL_REFERENCE_ENCODED,
        OFF_HEAP_TARGET,
        FULL_EMBEDDED_OBJECT,
        WASM_CALL,
        WASM_STUB_CALL,
        WASM_CODE_POINTER_TABLE_ENTRY,
        JS_DISPATCH_HANDLE,
    }

    pub struct WritableJitAllocation {}

    impl WritableJitAllocation {
        fn WriteUnalignedValue<T>(&mut self, _pc: Address, _target: T) {}
    }

    const LUI: Instr = 0x3d;
    const ORI: Instr = 0x0d;
    const SPECIAL: Instr = 0x00;
    const SLL: Instr = 0x00;

    fn GetOpcodeField(instr: Instr) -> Instr {
        instr & 0x3f
    }

    fn GetImmediate16(instr: Instr) -> i16 {
        (instr & 0xffff) as i16
    }

    const kImm26Mask: Instr = 0x03FFFFFF;
    const kImm28Mask: isize = 0x0FFFFFFF;
    const kImm16Mask: Instr = 0xFFFF;

    fn GetRt(instr: Instr) -> u32 {
        ((instr >> 16) & 0x1f) as u32
    }

    const kRtShift: u32 = 16;
    const kRsShift: u32 = 21;
}
