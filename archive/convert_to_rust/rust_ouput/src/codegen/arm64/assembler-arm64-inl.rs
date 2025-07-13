// Converted from V8 C++ source files:
// Header: assembler-arm64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod assembler_arm64_inl {
use std::sync::{Arc, Mutex};
use crate::base::memory::WritableJitAllocation;
use crate::codegen::arm64::assembler_arm64::*;
use crate::codegen::assembler::*;
use crate::codegen::flush_instruction_cache::FlushInstructionCache;
use crate::debug::debug::BREAK;
use crate::heap::heap_layout_inl::HeapLayout;
use crate::objects::objects_inl::*;
use crate::objects::smi::Smi;
use crate::objects::tagged::Tagged;
use crate::codegen::arm64::constants_arm64::{Shift, Extend};

pub struct V8 {}

impl V8 {
    pub fn supports_optimizer() -> bool {
        true
    }
}

impl WritableRelocInfo {
    pub fn apply(&mut self, delta: isize) {
        if RelocInfo::is_internal_reference(self.rmode_) {
            let mut internal_ref = unsafe {
                (self.pc_ as *mut isize).read_unaligned()
            };
            internal_ref += delta;
            unsafe {
                self.jit_allocation_
                    .write_unaligned_value(self.pc_, internal_ref);
            };
        } else {
            let instr = self.pc_ as *mut Instruction;
            unsafe {
                if (*instr).is_branch_and_link() || (*instr).is_unconditional_branch() {
                    let old_target = (*instr).imm_pc_offset_target() as *mut usize;
                    let new_target = old_target.offset(-(delta as isize));
                    (*instr).set_branch_imm_target::<UncondBranchType>(
                        new_target as *mut Instruction,
                        &self.jit_allocation_,
                    );
                }
            }
        }
    }
}

impl CPURegister {
    pub fn is_same_size_and_type(&self, other: &Self) -> bool {
        self.reg_size_ == other.reg_size_ && self.reg_type_ == other.reg_type_
    }

    pub fn is_zero(&self) -> bool {
        debug_assert!(self.is_valid());
        self.is_register() && self.code() == kZeroRegCode
    }

    pub fn is_sp(&self) -> bool {
        debug_assert!(self.is_valid());
        self.is_register() && self.code() == kSPRegInternalCode
    }

    pub fn w(&self) -> Register {
        debug_assert!(self.is_register());
        Register::wreg_from_code(self.code() as u32)
    }

    pub fn reg(&self) -> Register {
        debug_assert!(self.is_register());
        Register::create(self.code(), self.reg_size_)
    }

    pub fn vreg(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::create(self.code(), self.reg_size_)
    }

    pub fn x(&self) -> Register {
        debug_assert!(self.is_register());
        Register::xreg_from_code(self.code() as u32)
    }

    pub fn v(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::vreg_from_code(self.code() as u32)
    }

    pub fn b(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::breg_from_code(self.code() as u32)
    }

    pub fn h(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::hreg_from_code(self.code() as u32)
    }

    pub fn s(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::sreg_from_code(self.code() as u32)
    }

    pub fn d(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::dreg_from_code(self.code() as u32)
    }

    pub fn q(&self) -> VRegister {
        debug_assert!(self.is_v_register());
        VRegister::qreg_from_code(self.code() as u32)
    }
}

impl CPURegList {
    pub fn combine(&mut self, other: &Self) {
        debug_assert!(other.type_ == self.type_);
        debug_assert!(other.size_ == self.size_);
        self.list_ |= other.list_;
    }

    pub fn remove(&mut self, other: &Self) {
        if other.type_ == self.type_ {
            self.list_ &= !other.list_;
        }
    }

    pub fn combine_register(&mut self, other: &CPURegister) {
        debug_assert!(other.type_ == self.type_);
        debug_assert!(other.size_ == self.size_);
        self.combine_code(other.code());
    }

    pub fn remove_registers(
        &mut self,
        other1: &CPURegister,
        other2: &CPURegister,
        other3: &CPURegister,
        other4: &CPURegister,
    ) {
        if !other1.is_none() && (other1.type_ == self.type_) {
            self.remove_code(other1.code());
        }
        if !other2.is_none() && (other2.type_ == self.type_) {
            self.remove_code(other2.code());
        }
        if !other3.is_none() && (other3.type_ == self.type_) {
            self.remove_code(other3.code());
        }
        if !other4.is_none() && (other4.type_ == self.type_) {
            self.remove_code(other4.code());
        }
    }

    pub fn combine_code(&mut self, code: i8) {
        debug_assert!(CPURegister::create(code, self.size_, self.type_).is_valid());
        self.list_ |= 1u64 << code;
        debug_assert!(self.is_valid());
    }

    pub fn remove_code(&mut self, code: i8) {
        debug_assert!(CPURegister::create(code, self.size_, self.type_).is_valid());
        self.list_ &= !(1u64 << code);
    }
}

impl Register {
    pub fn xreg_from_code(code: u32) -> Self {
        if code == kSPRegInternalCode as u32 {
            return sp;
        } else {
            debug_assert!(code < kNumberOfRegisters as u32);
            return Register::create(code as i8, kXRegSizeInBits);
        }
    }

    pub fn wreg_from_code(code: u32) -> Self {
        if code == kSPRegInternalCode as u32 {
            return wsp;
        } else {
            debug_assert!(code < kNumberOfRegisters as u32);
            return Register::create(code as i8, kWRegSizeInBits);
        }
    }
}

impl VRegister {
    pub fn breg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kBRegSizeInBits);
    }

    pub fn hreg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kHRegSizeInBits);
    }

    pub fn sreg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kSRegSizeInBits);
    }

    pub fn dreg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kDRegSizeInBits);
    }

    pub fn qreg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kQRegSizeInBits);
    }

    pub fn vreg_from_code(code: u32) -> Self {
        debug_assert!(code < kNumberOfVRegisters as u32);
        return VRegister::create(code as i8, kVRegSizeInBits);
    }
}

struct ImmediateInitializer {}

impl ImmediateInitializer {
    pub fn rmode_for<T>(_t: T) -> RelocInfo::Mode {
        RelocInfo::NO_INFO
    }

    pub fn immediate_for<T: Into<i64>>(t: T) -> i64 {
        t.into()
    }
}

impl ImmediateInitializer {
    pub fn rmode_for_tagged_smi(_t: Tagged<Smi>) -> RelocInfo::Mode {
        RelocInfo::NO_INFO
    }

    pub fn immediate_for_tagged_smi(t: Tagged<Smi>) -> i64 {
        t.ptr() as i64
    }
}

impl ImmediateInitializer {
    pub fn rmode_for_external_reference(_t: ExternalReference) -> RelocInfo::Mode {
        RelocInfo::EXTERNAL_REFERENCE
    }

    pub fn immediate_for_external_reference(t: ExternalReference) -> i64 {
        t.raw() as i64
    }
}

impl Immediate {
    pub fn new_handle<T>(handle: Handle<T>, mode: RelocInfo::Mode) -> Self {
        debug_assert!(RelocInfo::is_embedded_object_mode(mode));
        Immediate {
            value_: handle.address() as isize,
            rmode_: mode,
        }
    }

   pub fn new_tagged<T>(t: T) -> Self {
        Immediate {
            value_: ImmediateInitializer::immediate_for(t),
            rmode_: ImmediateInitializer::rmode_for(t),
        }
    }

    pub fn new_tagged_with_mode<T: Into<i64>>(t: T, rmode: RelocInfo::Mode) -> Self {
        Immediate {
            value_: ImmediateInitializer::immediate_for(t),
            rmode_: rmode,
        }
    }
}

impl Operand {
    pub fn new_immediate<T: Into<i64>>(t: T) -> Self {
        Operand {
            immediate_: Immediate::new_tagged(t),
            reg_: NoReg,
            shift_: NO_SHIFT,
            extend_: NO_EXTEND,
            shift_amount_: 0,
            heap_number_request_: None,
        }
    }

    pub fn new_immediate_with_mode<T: Into<i64>>(t: T, rmode: RelocInfo::Mode) -> Self {
        Operand {
            immediate_: Immediate::new_tagged_with_mode(t, rmode),
            reg_: NoReg,
            shift_: NO_SHIFT,
            extend_: NO_EXTEND,
            shift_amount_: 0,
            heap_number_request_: None,
        }
    }

    pub fn new_shifted_register(reg: Register, shift: Shift, shift_amount: u32) -> Self {
        debug_assert!(reg.is_64bits() || (shift_amount < kWRegSizeInBits as u32));
        debug_assert!(reg.is_32bits() || (shift_amount < kXRegSizeInBits as u32));
        debug_assert!(!reg.is_sp() || shift_amount == 0);
        Operand {
            immediate_: Immediate { value_: 0, rmode_: RelocInfo::NO_INFO },
            reg_: reg,
            shift_: shift,
            extend_: NO_EXTEND,
            shift_amount_: shift_amount,
            heap_number_request_: None,
        }
    }

    pub fn new_extended_register(reg: Register, extend: Extend, shift_amount: u32) -> Self {
        debug_assert!(reg.is_valid());
        debug_assert!(shift_amount <= 4);
        debug_assert!(!reg.is_sp());

        debug_assert!(reg.is_64bits() || ((extend != SXTX) && (extend != UXTX)));
        Operand {
            immediate_: Immediate { value_: 0, rmode_: RelocInfo::NO_INFO },
            reg_: reg,
            shift_: NO_SHIFT,
            extend_: extend,
            shift_amount_: shift_amount,
            heap_number_request_: None,
        }
    }

    pub fn is_heap_number_request(&self) -> bool {
        self.heap_number_request_.is_some()
    }

    pub fn heap_number_request(&self) -> HeapNumberRequest {
        debug_assert!(self.is_heap_number_request());
        self.heap_number_request_.unwrap()
    }

    pub fn is_immediate(&self) -> bool {
        self.reg_ == NoReg && !self.is_heap_number_request()
    }

    pub fn is_shifted_register(&self) -> bool {
        self.reg_.is_valid() && (self.shift_ != NO_SHIFT)
    }

    pub fn is_extended_register(&self) -> bool {
        self.reg_.is_valid() && (self.extend_ != NO_EXTEND)
    }

    pub fn is_zero(&self) -> bool {
        if self.is_immediate() {
            self.immediate_value() == 0
        } else {
            self.reg().is_zero()
        }
    }

    pub fn to_extended_register(&self) -> Self {
        debug_assert!(self.is_shifted_register());
        debug_assert!((self.shift_ == LSL) && (self.shift_amount_ <= 4));
        Operand::new_extended_register(self.reg_, if self.reg_.is_64bits() { UXTX } else { UXTW }, self.shift_amount_)
    }

    pub fn to_w(&self) -> Self {
        if self.is_shifted_register() {
            debug_assert!(self.reg_.is_64bits());
            return Operand::new_shifted_register(self.reg_.w(), self.shift(), self.shift_amount());
        } else if self.is_extended_register() {
            debug_assert!(self.reg_.is_64bits());
            return Operand::new_extended_register(self.reg_.w(), self.extend(), self.shift_amount());
        }
        debug_assert!(self.is_immediate());
        return *self;
    }

    pub fn immediate_for_heap_number_request(&self) -> Immediate {
        debug_assert!(self.immediate_.rmode() == RelocInfo::FULL_EMBEDDED_OBJECT);
        self.immediate_
    }

    pub fn immediate(&self) -> Immediate {
        debug_assert!(self.is_immediate());
        self.immediate_
    }

    pub fn immediate_value(&self) -> i64 {
        debug_assert!(self.is_immediate());
        self.immediate_.value() as i64
    }

    pub fn immediate_rmode(&self) -> RelocInfo::Mode {
        debug_assert!(self.is_immediate() || self.is_heap_number_request());
        self.immediate_.rmode()
    }

    pub fn reg(&self) -> Register {
        debug_assert!(self.is_shifted_register() || self.is_extended_register());
        self.reg_
    }

    pub fn shift(&self) -> Shift {
        debug_assert!(self.is_shifted_register());
        self.shift_
    }

    pub fn extend(&self) -> Extend {
        debug_assert!(self.is_extended_register());
        self.extend_
    }

    pub fn shift_amount(&self) -> u32 {
        debug_assert!(self.is_shifted_register() || self.is_extended_register());
        self.shift_amount_
    }
}

impl MemOperand {
    pub fn new() -> Self {
        MemOperand {
            base_: NoReg,
            regoffset_: NoReg,
            offset_: 0,
            addrmode_: AddrMode::Offset,
            shift_: NO_SHIFT,
            extend_: NO_EXTEND,
            shift_amount_: 0,
        }
    }

    pub fn offset(base: Register, offset: i64, addrmode: AddrMode) -> Self {
        debug_assert!(base.is_64bits() && !base.is_zero());
        MemOperand {
            base_: base,
            regoffset_: NoReg,
            offset_: offset,
            addrmode_: addrmode,
            shift_: NO_SHIFT,
            extend_: NO_EXTEND,
            shift_amount_: 0,
        }
    }

    pub fn extended_register_offset(
        base: Register,
        regoffset: Register,
        extend: Extend,
        shift_amount: u32,
    ) -> Self {
        debug_assert!(base.is_64bits() && !base.is_zero());
        debug_assert!(!regoffset.is_sp());
        debug_assert!((extend == UXTW) || (extend == SXTW) || (extend == SXTX));
        debug_assert!(regoffset.is_64bits() || (extend != SXTX));
        MemOperand {
            base_: base,
            regoffset_: regoffset,
            offset_: 0,
            addrmode_: AddrMode::Offset,
            shift_: NO_SHIFT,
            extend_: extend,
            shift_amount_: shift_amount,
        }
    }

    pub fn shifted_register_offset(
        base: Register,
        regoffset: Register,
        shift: Shift,
        shift_amount: u32,
    ) -> Self {
        debug_assert!(base.is_64bits() && !base.is_zero());
        debug_assert!(regoffset.is_64bits() && !regoffset.is_sp());
        debug_assert!(shift == LSL);
        MemOperand {
            base_: base,
            regoffset_: regoffset,
            offset_: 0,
            addrmode_: AddrMode::Offset,
            shift_: shift,
            extend_: NO_EXTEND,
            shift_amount_: shift_amount,
        }
    }

    pub fn operand_offset(base: Register, offset: &Operand, addrmode: AddrMode) -> Self {
        debug_assert!(base.is_64bits() && !base.is_zero());

        let mut result = MemOperand {
            base_: base,
            regoffset_: NoReg,
            offset_: 0,
            addrmode_: addrmode,
            shift_: NO_SHIFT,
            extend_: NO_EXTEND,
            shift_amount_: 0,
        };

        if offset.is_immediate() {
            result.offset_ = offset.immediate_value();
        } else if offset.is_shifted_register() {
            debug_assert!((addrmode == AddrMode::Offset) || (addrmode == AddrMode::PostIndex));

            result.regoffset_ = offset.reg();
            result.shift_ = offset.shift();
            result.shift_amount_ = offset.shift_amount();

            result.extend_ = NO_EXTEND;
            result.offset_ = 0;

            debug_assert!(result.regoffset_.is_64bits() && !result.regoffset_.is_sp());
            debug_assert!(result.shift_ == LSL);
        } else {
            debug_assert!(offset.is_extended_register());
            debug_assert!(addrmode == AddrMode::Offset);

            result.regoffset_ = offset.reg();
            result.extend_ = offset.extend();
            result.shift_amount_ = offset.shift_amount();

            result.shift_ = NO_SHIFT;
            result.offset_ = 0;

            debug_assert!(!result.regoffset_.is_sp());
            debug_assert!((result.extend_ == UXTW) || (result.extend_ == SXTW) || (result.extend_ == SXTX));
            debug_assert!(result.regoffset_.is_64bits() || (result.extend_ != SXTX));
        }

        result
    }

    pub fn is_immediate_offset(&self) -> bool {
        self.addrmode_ == AddrMode::Offset && self.regoffset_ == NoReg
    }

    pub fn is_register_offset(&self) -> bool {
        self.addrmode_ == AddrMode::Offset && self.regoffset_ != NoReg
    }

    pub fn is_pre_index(&self) -> bool {
        self.addrmode_ == AddrMode::PreIndex
    }

    pub fn is_post_index(&self) -> bool {
        self.addrmode_ == AddrMode::PostIndex
    }
}

impl Assembler {
    pub fn unreachable(&mut self) {
        self.debug("UNREACHABLE", line!(), BREAK);
    }

    pub fn target_pointer_address_at(&mut self, pc: Address) -> Address {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe{
            debug_assert!((*instr).is_ldr_literal_x() || (*instr).is_ldr_literal_w());
            (*instr).imm_pc_offset_target() as Address
        }
    }

    pub fn target_address_at(&mut self, pc: Address, _constant_pool: Address) -> Address {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe{
            if (*instr).is_ldr_literal_x() {
                self.target_pointer_address_at(pc) as Address
            } else {
                debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
                (*instr).imm_pc_offset_target() as Address
            }
        }
    }

    pub fn target_compressed_address_at(&mut self, pc: Address, _constant_pool: Address) -> Tagged_t {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            debug_assert!((*instr).is_ldr_literal_w());
            (self.target_pointer_address_at(pc) as *mut Tagged_t).read_unaligned()
        }
    }

    pub fn code_target_object_handle_at(&mut self, pc: Address) -> Handle<Code> {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            if (*instr).is_ldr_literal_x() {
                Handle::from_address(*(self.target_address_at(pc, 0 as Address) as *mut Address))
            } else {
                debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
                debug_assert_eq!((*instr).imm_pc_offset() % kInstrSize, 0);
                let obj = self.get_embedded_object((*instr).imm_pc_offset() >> kInstrSizeLog2);
                Handle::from_address(obj as Address)
            }
        }
    }

    pub fn embedded_object_index_referenced_from(&mut self, pc: Address) -> AssemblerBase::EmbeddedObjectIndex {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            if (*instr).is_ldr_literal_x() {
                *(self.target_pointer_address_at(pc) as *mut AssemblerBase::EmbeddedObjectIndex)
            } else {
                debug_assert!((*instr).is_ldr_literal_w());
                *(self.target_pointer_address_at(pc) as *mut u32) as AssemblerBase::EmbeddedObjectIndex
            }
        }
    }

    pub fn set_embedded_object_index_referenced_from(&mut self, pc: Address, data: AssemblerBase::EmbeddedObjectIndex) {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            if (*instr).is_ldr_literal_x() {
                *(self.target_pointer_address_at(pc) as *mut AssemblerBase::EmbeddedObjectIndex) = data;
            } else {
                debug_assert!((*instr).is_ldr_literal_w());
                debug_assert!(is_uint32(data as i64));
                (self.target_pointer_address_at(pc) as *mut u32).write_unaligned(data as u32);
            }
        }
    }

    pub fn target_object_handle_at(&mut self, pc: Address) -> Handle<HeapObject> {
        self.get_embedded_object(self.embedded_object_index_referenced_from(pc))
    }

    pub fn target_builtin_at(&mut self, pc: Address) -> Builtin {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
            debug_assert_eq!((*instr).imm_pc_offset() % kInstrSize, 0);
            let builtin_id = (*instr).imm_pc_offset() / kInstrSize;
            debug_assert!(Builtins::is_builtin_id(builtin_id as i32));
            builtin_id as Builtin
        }
    }

    pub fn deserialization_special_target_size(&mut self, location: Address) -> i32 {
        let instr = unsafe {
            (location as *mut Instruction)
        };
        unsafe {
            if (*instr).is_branch_and_link() || (*instr).is_unconditional_branch() {
                kSpecialTargetSize as i32
            } else {
                debug_assert_eq!((*instr).instruction_bits(), 0);
                kSystemPointerSize as i32
            }
        }
    }

    pub fn deserialization_set_target_internal_reference_at(
        &mut self,
        pc: Address,
        target: Address,
        jit_allocation: &mut WritableJitAllocation,
        _mode: RelocInfo::Mode,
    ) {
        unsafe {
            jit_allocation.write_unaligned_value(pc, target);
        }
    }

    pub fn set_target_address_at(
        &mut self,
        pc: Address,
        _constant_pool: Address,
        target: Address,
        jit_allocation: Option<&mut WritableJitAllocation>,
        icache_flush_mode: ICacheFlushMode,
    ) {
        let instr = unsafe {
            (pc as *mut Instruction)
        };

        unsafe {
            if (*instr).is_ldr_literal_x() {
                match jit_allocation {
                    Some(jit_allocation) => {
                        jit_allocation.write_value(self.target_pointer_address_at(pc), target);
                    }
                    None => {
                        *(self.target_pointer_address_at(pc) as *mut Address) = target;
                    }
                }
            } else {
                debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
                let mut target_addr = target;
                if target_addr as usize == 0 {
                    target_addr = pc;
                }
                (*instr).set_branch_imm_target::<UncondBranchType>(
                    target_addr as *mut Instruction,
                    jit_allocation.as_deref_mut(),
                );
                if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                    FlushInstructionCache(pc, kInstrSize);
                }
            }
        }
    }

    pub fn set_target_compressed_address_at(
        &mut self,
        pc: Address,
        _constant_pool: Address,
        target: Tagged_t,
        jit_allocation: Option<&mut WritableJitAllocation>,
        _icache_flush_mode: ICacheFlushMode,
    ) {
        let instr = unsafe {
            (pc as *mut Instruction)
        };

        unsafe {
            debug_assert!((*instr).is_ldr_literal_w());
            match jit_allocation {
                Some(jit_allocation) => {
                    jit_allocation.write_value(self.target_pointer_address_at(pc), target);
                }
                None => {
                    *(self.target_pointer_address_at(pc) as *mut Tagged_t) = target;
                }
            }
        }
    }

    pub fn uint32_constant_at(&mut self, pc: Address, _constant_pool: Address) -> u32 {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            debug_assert!((*instr).is_ldr_literal_w());
            (self.target_pointer_address_at(pc) as *mut u32).read_unaligned()
        }
    }

    pub fn set_uint32_constant_at(
        &mut self,
        pc: Address,
        _constant_pool: Address,
        new_constant: u32,
        jit_allocation: Option<&mut WritableJitAllocation>,
        _icache_flush_mode: ICacheFlushMode,
    ) {
        let instr = unsafe {
            (pc as *mut Instruction)
        };
        unsafe {
            debug_assert!((*instr).is_ldr_literal_w());
            match jit_allocation {
                Some(jit_allocation) => {
                    jit_allocation.write_unaligned_value(self.target_pointer_address_at(pc), new_constant);
                }
                None => {
                    (self.target_pointer_address_at(pc) as *mut u32).write_unaligned(new_constant);
                }
            }
        }
    }

    pub fn load_op_for(&self, rt: &CPURegister) -> LoadStoreOp {
        debug_assert!(rt.is_valid());
        if rt.is_register() {
            if rt.is_64bits() {
                LoadStoreOp::LDR_x
            } else {
                LoadStoreOp::LDR_w
            }
        } else {
            debug_assert!(rt.is_v_register());
            match rt.size_in_bits() {
                kBRegSizeInBits => LoadStoreOp::LDR_b,
                kHRegSizeInBits => LoadStoreOp::LDR_h,
                kSRegSizeInBits => LoadStoreOp::LDR_s,
                kDRegSizeInBits => LoadStoreOp::LDR_d,
                _ => {
                    debug_assert!(rt.is_q());
                    LoadStoreOp::LDR_q
                }
            }
        }
    }

    pub fn store_op_for(&self, rt: &CPURegister) -> LoadStoreOp {
        debug_assert!(rt.is_valid());
        if rt.is_register() {
            if rt.is_64bits() {
                LoadStoreOp::STR_x
            } else {
                LoadStoreOp::STR_w
            }
        } else {
            debug_assert!(rt.is_v_register());
            match rt.size_in_bits() {
                kBRegSizeInBits => LoadStoreOp::STR_b,
                kHRegSizeInBits => LoadStoreOp::STR_h,
                kSRegSizeInBits => LoadStoreOp::STR_s,
                kDRegSizeInBits => LoadStoreOp::STR_d,
                _ => {
                    debug_assert!(rt.is_q());
                    LoadStoreOp::STR_q
                }
            }
        }
    }

    pub fn load_pair_op_for(&self, rt: &CPURegister, rt2: &CPURegister) -> LoadStorePairOp {
        debug_assert_eq!(LoadStorePairOp::STP_w as u32 | LoadStorePairLBit as u32, LoadStorePairOp::LDP_w as u32);
        (self.store_pair_op_for(rt, rt2) as u32 | LoadStorePairLBit as u32) as LoadStorePairOp
    }

    pub fn store_pair_op_for(&self, rt: &CPURegister, rt2: &CPURegister) -> LoadStorePairOp {
        debug_assert!(are_same_size_and_type(rt, rt2));
        if rt.is_register() {
            if rt.is_64bits() {
                LoadStorePairOp::STP_x
            } else {
                LoadStorePairOp::STP_w
            }
        } else {
            debug_assert!(rt.is_v_register());
            match rt.size_in_bits() {
                kSRegSizeInBits => LoadStorePairOp::STP_s,
                kDRegSizeInBits =>
