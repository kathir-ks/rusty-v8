// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod arm64 {
    pub mod assembler_arm64_inl {
        use std::mem;
        use std::ops::{BitAnd, BitOr, BitXor};

        //use crate::base::memory::*; // Assuming memory.h functionality can be handled with Rust primitives
        use crate::codegen::arm64::assembler_arm64::*;
        use crate::codegen::assembler::*;
        use crate::codegen::flush_instruction_cache::*;
        use crate::debug::debug::*;
        //use crate::heap::heap_layout_inl::*; // Assuming heap layout is handled elsewhere
        //use crate::objects::objects_inl::*; // Assuming object representations are handled elsewhere
        use crate::objects::smi::*;
        //use crate::objects::tagged::*; // Assuming Tagged is defined elsewhere

        pub struct CpuFeatures {}

        impl CpuFeatures {
            pub fn supports_optimizer() -> bool {
                true
            }
        }

        pub struct WritableRelocInfo<'a> {
            pc_: usize, // Assuming Address is usize
            rmode_: RelocInfoMode,
            jit_allocation_: &'a mut WritableJitAllocation, // Assuming WritableJitAllocation exists
            constant_pool_: usize,                                // Assuming Address is usize
        }

        impl<'a> WritableRelocInfo<'a> {
            pub fn apply(&mut self, delta: isize) {
                // On arm64 only internal references and immediate branches need extra work.
                if RelocInfoMode::is_internal_reference(self.rmode_) {
                    // Absolute code pointer inside code object moves with the code object.
                    let internal_ref = unsafe {
                        (self.pc_ as *const usize).read_unaligned() as isize
                    };
                    let internal_ref = internal_ref + delta; // Relocate entry.
                    unsafe {
                        (self.pc_ as *mut usize).write_unaligned(internal_ref as usize)
                    };
                } else {
                    let instr = self.pc_ as *mut Instruction;
                    unsafe {
                        if (*instr).is_branch_and_link() || (*instr).is_unconditional_branch() {
                            let old_target = (*instr).imm_pc_offset_target() as usize;
                            let new_target = (old_target as isize - delta) as usize;
                            (*instr).set_branch_imm_target::<UncondBranchType>(
                                new_target as *mut Instruction,
                                &mut self.jit_allocation_,
                            );
                        }
                    }
                }
            }
        }

        impl<'a> RelocInfo<'a> for WritableRelocInfo<'a> {
            fn pc(&self) -> usize {
                self.pc_
            }
            fn rmode(&self) -> RelocInfoMode {
                self.rmode_
            }
            fn constant_pool(&self) -> usize {
                self.constant_pool_
            }
        }

        pub trait RelocInfo<'a> {
            fn pc(&self) -> usize;
            fn rmode(&self) -> RelocInfoMode;
            fn constant_pool(&self) -> usize;

            fn target_address_size(&self) -> i32 {
                if self.rmode().is_coded_specially() {
                    Assembler::k_special_target_size
                } else {
                    let instr = self.pc() as *mut Instruction;
                    unsafe {
                        if (*instr).is_ldr_literal_x() || (*instr).is_ldr_literal_w() {
                            if (*instr).is_ldr_literal_w() {
                                Assembler::k_tagged_size
                            } else {
                                Assembler::k_system_pointer_size
                            }
                        } else {
                            panic!("Instruction is not a LDR literal");
                        }
                    }
                }
            }

            fn target_address(&self) -> usize {
                if self.rmode().is_code_target()
                    || self.rmode().is_near_builtin_entry()
                    || self.rmode().is_wasm_call()
                    || self.rmode().is_wasm_stub_call()
                {
                    Assembler::target_address_at(self.pc(), self.constant_pool())
                } else {
                    panic!("rmode_ is not CodeTarget, NearBuiltinEntry, WasmCall, or WasmStubCall");
                }
            }

            fn target_address_address(&self) -> usize {
                if self.rmode().has_target_address_address() {
                    let instr = self.pc() as *mut Instruction;
                    unsafe {
                        if (*instr).is_ldr_literal_x() {
                            self.constant_pool_entry_address()
                        } else {
                            if (*instr).is_branch_and_link() || (*instr).is_unconditional_branch() {
                                self.pc()
                            } else {
                                panic!("Instruction is not a BranchAndLink or UnconditionalBranch")
                            }
                        }
                    }
                } else {
                    panic!("RelocInfo does not have target address address")
                }
            }

            fn constant_pool_entry_address(&self) -> usize {
                if self.rmode().is_in_constant_pool() {
                    Assembler::target_pointer_address_at(self.pc())
                } else {
                    panic!("RelocInfo is not in constant pool")
                }
            }

            fn target_object(&self, cage_base: PtrComprCageBase) -> TaggedHeapObject {
                if self.rmode().is_code_target() || self.rmode().is_embedded_object_mode() {
                    if self.rmode().is_compressed_embedded_object() {
                        if COMPRESS_POINTERS_BOOL {
                            let compressed = Assembler::target_compressed_address_at(self.pc(), self.constant_pool());
                            if !HAS_SMI_TAG(compressed as usize) {
                                let obj = V8HeapCompressionScheme::decompress_tagged(cage_base, compressed);
                                return TaggedHeapObject::cast(TaggedObject::from_ptr(obj as usize));
                            } else {
                                panic!("Compressed address has SMI tag");
                            }
                        } else {
                            panic!("COMPRESS_POINTERS_BOOL is false");
                        }
                    } else {
                        return TaggedHeapObject::cast(TaggedObject::from_ptr(Assembler::target_address_at(self.pc(), self.constant_pool())));
                    }
                } else {
                    panic!("RelocInfo is not CodeTarget or EmbeddedObjectMode");
                }
            }

            fn target_internal_reference(&self) -> usize {
                if self.rmode().is_internal_reference() {
                    unsafe { *(self.pc() as *const usize) }
                } else {
                    panic!("RelocInfo is not InternalReference");
                }
            }

            fn target_internal_reference_address(&self) -> usize {
                if self.rmode().is_internal_reference() {
                    self.pc()
                } else {
                    panic!("RelocInfo is not InternalReference");
                }
            }
        }

        impl CPURegister {
            pub fn is_same_size_and_type(&self, other: &CPURegister) -> bool {
                (self.reg_size_ == other.reg_size_) && (self.reg_type_ == other.reg_type_)
            }

            pub fn is_zero(&self) -> bool {
                debug_assert!(self.is_valid());
                self.is_register() && (self.code() == k_zero_reg_code)
            }

            pub fn is_sp(&self) -> bool {
                debug_assert!(self.is_valid());
                self.is_register() && (self.code() == k_sp_reg_internal_code)
            }

            pub fn w(&self) -> Register {
                debug_assert!(self.is_register());
                Register::w_reg_from_code(self.code())
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
                Register::x_reg_from_code(self.code())
            }

            pub fn v(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::v_reg_from_code(self.code())
            }

            pub fn b(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::b_reg_from_code(self.code())
            }

            pub fn h(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::h_reg_from_code(self.code())
            }

            pub fn s(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::s_reg_from_code(self.code())
            }

            pub fn d(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::d_reg_from_code(self.code())
            }

            pub fn q(&self) -> VRegister {
                debug_assert!(self.is_v_register());
                VRegister::q_reg_from_code(self.code())
            }
        }

        impl CPURegList {
            pub fn combine(&mut self, other: &CPURegList) {
                debug_assert!(other.type_ == self.type_);
                debug_assert!(other.size_ == self.size_);
                self.list_ |= other.list_;
            }

            pub fn remove(&mut self, other: &CPURegList) {
                if other.type_ == self.type_ {
                    self.list_ &= !other.list_;
                }
            }

            pub fn combine_reg(&mut self, other: &CPURegister) {
                debug_assert!(other.type_ == self.type_);
                debug_assert!(other.size_ == self.size_);
                self.combine_code(other.code());
            }

            pub fn remove_regs(
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

            pub fn combine_code(&mut self, code: i32) {
                debug_assert!(CPURegister::create(code, self.size_, self.type_).is_valid());
                self.list_ |= (1u64 << code);
                debug_assert!(self.is_valid());
            }

            pub fn remove_code(&mut self, code: i32) {
                debug_assert!(CPURegister::create(code, self.size_, self.type_).is_valid());
                self.list_ &= !(1u64 << code);
            }
        }

        impl Register {
            pub fn x_reg_from_code(code: u32) -> Self {
                if code == k_sp_reg_internal_code as u32 {
                    Register::sp
                } else {
                    debug_assert!(code < k_number_of_registers as u32);
                    Register::create(code as i32, k_x_reg_size_in_bits)
                }
            }

            pub fn w_reg_from_code(code: u32) -> Self {
                if code == k_sp_reg_internal_code as u32 {
                    Register::wsp
                } else {
                    debug_assert!(code < k_number_of_registers as u32);
                    Register::create(code as i32, k_w_reg_size_in_bits)
                }
            }
        }

        impl VRegister {
            pub fn b_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_b_reg_size_in_bits)
            }

            pub fn h_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_h_reg_size_in_bits)
            }

            pub fn s_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_s_reg_size_in_bits)
            }

            pub fn d_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_d_reg_size_in_bits)
            }

            pub fn q_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_q_reg_size_in_bits)
            }

            pub fn v_reg_from_code(code: u32) -> Self {
                debug_assert!(code < k_number_of_v_registers as u32);
                VRegister::create(code as i32, k_v_reg_size_in_bits)
            }
        }

        // Immediate.
        // Default initializer is for int types
        pub struct ImmediateInitializer {}

        impl ImmediateInitializer {
            pub fn rmode_for<T>(_t: T) -> RelocInfoMode {
                RelocInfoMode::NoInfo
            }
            pub fn immediate_for<T: Into<i64>>(t: T) -> i64 {
                //where T: std::marker::Sized
                let _ = {
                    assert!(mem::size_of::<T>() <= 8);
                    assert!(std::any::TypeId::of::<T>() == std::any::TypeId::of::<i64>());
                };
                t.into()
            }
        }

        impl ImmediateInitializer {
            pub fn rmode_for_tagged_smi(_t: TaggedSmi) -> RelocInfoMode {
                RelocInfoMode::NoInfo
            }

            pub fn immediate_for_tagged_smi(t: TaggedSmi) -> i64 {
                t.ptr() as i64
            }
        }

        impl ImmediateInitializer {
            pub fn rmode_for_external_reference(_t: ExternalReference) -> RelocInfoMode {
                RelocInfoMode::ExternalReference
            }
            pub fn immediate_for_external_reference(t: ExternalReference) -> i64 {
                t.raw() as i64
            }
        }

        impl Immediate {
            pub fn new_handle<T>(handle: Handle<T>, mode: RelocInfoMode) -> Self {
                Immediate {
                    value_: handle.address() as i64,
                    rmode_: mode,
                }
            }
        }

        impl<T: Into<i64> + Copy> Immediate {
            pub fn new_tagged(t: T) -> Self {
                Immediate {
                    value_: ImmediateInitializer::immediate_for(t),
                    rmode_: ImmediateInitializer::rmode_for(t),
                }
            }

            pub fn new_tagged_with_mode(t: T, rmode: RelocInfoMode) -> Self {
                Immediate {
                    value_: ImmediateInitializer::immediate_for(t),
                    rmode_: rmode,
                }
            }
        }

        impl Operand {
            pub fn new_tagged<T: Into<i64> + Copy>(t: T) -> Self {
                Operand {
                    immediate_: Immediate::new_tagged(t),
                    reg_: NoReg,
                    shift_: NO_SHIFT,
                    extend_: NO_EXTEND,
                    shift_amount_: 0,
                    heap_number_request_: None,
                }
            }

            pub fn new_tagged_with_mode<T: Into<i64> + Copy>(t: T, rmode: RelocInfoMode) -> Self {
                Operand {
                    immediate_: Immediate::new_tagged_with_mode(t, rmode),
                    reg_: NoReg,
                    shift_: NO_SHIFT,
                    extend_: NO_EXTEND,
                    shift_amount_: 0,
                    heap_number_request_: None,
                }
            }
        }

        impl Operand {
            pub fn new_shifted_register(
                reg: Register,
                shift: Shift,
                shift_amount: u32,
            ) -> Self {
                debug_assert!(reg.is_64_bits() || (shift_amount < k_w_reg_size_in_bits));
                debug_assert!(reg.is_32_bits() || (shift_amount < k_x_reg_size_in_bits));
                debug_assert!(!reg.is_sp() || shift_amount == 0);
                Operand {
                    immediate_: Immediate {
                        value_: 0,
                        rmode_: RelocInfoMode::NoInfo,
                    },
                    reg_: reg,
                    shift_: shift,
                    extend_: NO_EXTEND,
                    shift_amount_: shift_amount,
                    heap_number_request_: None,
                }
            }

            pub fn new_extended_register(
                reg: Register,
                extend: Extend,
                shift_amount: u32,
            ) -> Self {
                debug_assert!(reg.is_valid());
                debug_assert!(shift_amount <= 4);
                debug_assert!(!reg.is_sp());

                // Extend modes SXTX and UXTX require a 64-bit register.
                debug_assert!(reg.is_64_bits() || ((extend != SXTX) && (extend != UXTX)));
                Operand {
                    immediate_: Immediate {
                        value_: 0,
                        rmode_: RelocInfoMode::NoInfo,
                    },
                    reg_: reg,
                    shift_: NO_SHIFT,
                    extend_: extend,
                    shift_amount_: shift_amount,
                    heap_number_request_: None,
                }
            }

            pub fn to_extended_register(&self) -> Self {
                debug_assert!(self.is_shifted_register());
                debug_assert!((self.shift_ == LSL) && (self.shift_amount_ <= 4));
                Operand::new_extended_register(
                    self.reg_,
                    if self.reg_.is_64_bits() {
                        UXTX
                    } else {
                        UXTW
                    },
                    self.shift_amount_,
                )
            }

            pub fn to_w(&self) -> Self {
                if self.is_shifted_register() {
                    debug_assert!(self.reg_.is_64_bits());
                    Operand::new_shifted_register(self.reg_.w(), self.shift(), self.shift_amount())
                } else if self.is_extended_register() {
                    debug_assert!(self.reg_.is_64_bits());
                    Operand::new_extended_register(self.reg_.w(), self.extend(), self.shift_amount())
                } else {
                    debug_assert!(self.is_immediate());
                    *self
                }
            }
            pub fn immediate_for_heap_number_request(&self) -> Immediate {
                debug_assert_eq!(
                    self.immediate_.rmode(),
                    RelocInfoMode::FullEmbeddedObject
                );
                self.immediate_
            }

            pub fn immediate(&self) -> Immediate {
                debug_assert!(self.is_immediate());
                self.immediate_
            }

            pub fn immediate_value(&self) -> i64 {
                debug_assert!(self.is_immediate());
                self.immediate_.value()
            }

            pub fn immediate_rmode(&self) -> RelocInfoMode {
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

            pub fn is_heap_number_request(&self) -> bool {
                self.heap_number_request_.is_some()
            }

            pub fn heap_number_request(&self) -> HeapNumberRequest {
                debug_assert!(self.is_heap_number_request());
                self.heap_number_request_.unwrap()
            }
        }

        impl MemOperand {
            pub fn new(base: Register, offset: i64, addrmode: AddrMode) -> Self {
                debug_assert!(base.is_64_bits() && !base.is_zero());
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

            pub fn new_register_offset(
                base: Register,
                regoffset: Register,
                extend: Extend,
                shift_amount: u32,
            ) -> Self {
                debug_assert!(base.is_64_bits() && !base.is_zero());
                debug_assert!(!regoffset.is_sp());
                debug_assert!((extend == UXTW) || (extend == SXTW) || (extend == SXTX));

                // SXTX extend mode requires a 64-bit offset register.
                debug_assert!(regoffset.is_64_bits() || (extend != SXTX));
                MemOperand {
                    base_: base,
                    regoffset_: regoffset,
                    offset_: 0,
                    addrmode_: Offset,
                    shift_: NO_SHIFT,
                    extend_: extend,
                    shift_amount_: shift_amount,
                }
            }

            pub fn new_shifted_register_offset(
                base: Register,
                regoffset: Register,
                shift: Shift,
                shift_amount: u32,
            ) -> Self {
                debug_assert!(base.is_64_bits() && !base.is_zero());
                debug_assert!(regoffset.is_64_bits() && !regoffset.is_sp());
                debug_assert!(shift == LSL);
                MemOperand {
                    base_: base,
                    regoffset_: regoffset,
                    offset_: 0,
                    addrmode_: Offset,
                    shift_: shift,
                    extend_: NO_EXTEND,
                    shift_amount_: shift_amount,
                }
            }

            pub fn new_operand_offset(base: Register, offset: &Operand, addrmode: AddrMode) -> Self {
                debug_assert!(base.is_64_bits() && !base.is_zero());

                if offset.is_immediate() {
                    MemOperand {
                        base_: base,
                        regoffset_: NoReg,
                        offset_: offset.immediate_value(),
                        addrmode_: addrmode,
                        shift_: NO_SHIFT,
                        extend_: NO_EXTEND,
                        shift_amount_: 0,
                    }
                } else if offset.is_shifted_register() {
                    debug_assert!((addrmode == Offset) || (addrmode == PostIndex));

                    // These assertions match those in the shifted-register constructor.
                    debug_assert!(offset.reg().is_64_bits() && !offset.reg().is_sp());
                    debug_assert!(offset.shift() == LSL);

                    MemOperand {
                        base_: base,
                        regoffset_: offset.reg(),
                        offset_: 0,
                        addrmode_: addrmode,
                        shift_: offset.shift(),
                        extend_: NO_EXTEND,
                        shift_amount_: offset.shift_amount(),
                    }
                } else {
                    debug_assert!(offset.is_extended_register());
                    debug_assert!(addrmode == Offset);

                    // These assertions match those in the extended-register constructor.
                    debug_assert!(!offset.reg().is_sp());
                    debug_assert!((offset.extend() == UXTW) || (offset.extend() == SXTW) || (offset.extend() == SXTX));
                    debug_assert!((offset.reg().is_64_bits() || (offset.extend() != SXTX)));

                    MemOperand {
                        base_: base,
                        regoffset_: offset.reg(),
                        offset_: 0,
                        addrmode_: addrmode,
                        shift_: NO_SHIFT,
                        extend_: offset.extend(),
                        shift_amount_: offset.shift_amount(),
                    }
                }
            }

            pub fn is_immediate_offset(&self) -> bool {
                (self.addrmode_ == Offset) && self.regoffset_ == NoReg
            }

            pub fn is_register_offset(&self) -> bool {
                (self.addrmode_ == Offset) && self.regoffset_ != NoReg
            }

            pub fn is_pre_index(&self) -> bool {
                self.addrmode_ == PreIndex
            }

            pub fn is_post_index(&self) -> bool {
                self.addrmode_ == PostIndex
            }
        }

        impl Assembler {
            pub fn unreachable(&mut self) {
                debug("UNREACHABLE", 0, BREAK); // Assuming BREAK is a const
            }

            pub fn target_pointer_address_at(pc: usize) -> usize {
                let instr = pc as *mut Instruction;
                unsafe {
                    debug_assert!((*instr).is_ldr_literal_x() || (*instr).is_ldr_literal_w());
                    (*instr).imm_pc_offset_target() as usize
                }
            }

            // Read/Modify the code target address in the branch/call instruction at pc.
            pub fn target_address_at(pc: usize, _constant_pool: usize) -> usize {
                let instr = pc as *mut Instruction;
                unsafe {
                    if (*instr).is_ldr_literal_x() {
                        *(Self::target_pointer_address_at(pc) as *mut usize)
                    } else {
                        debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
                        (*instr).imm_pc_offset_target() as usize
                    }
                }
            }

            pub fn target_compressed_address_at(pc: usize, _constant_pool: usize) -> Tagged_t {
                let instr = pc as *mut Instruction;
                unsafe {
                    debug_assert!((*instr).is_ldr_literal_w());
                    *(Self::target_pointer_address_at(pc) as *mut Tagged_t)
                }
            }

            // This function's return type is a placeholder.  The proper implementation depends on the definition of Handle<Code>.
            pub fn code_target_object_handle_at(_pc: usize) -> i32 {
                // TODO Implement code_target_object_handle_at
                0
            }

            pub fn embedded_object_index_referenced_from(pc: usize) -> AssemblerBaseEmbeddedObjectIndex {
                let instr = pc as *mut Instruction;
                unsafe {
                    if (*instr).is_ldr_literal_x() {
                        // AssemblerBase::EmbeddedObjectIndex
                        *(Self::target_pointer_address_at(pc) as *mut AssemblerBaseEmbeddedObjectIndex)
                    } else {
                        debug_assert!((*instr).is_ldr_literal_w());
                        *(Self::target_pointer_address_at(pc) as *mut u32) as AssemblerBaseEmbeddedObjectIndex
                    }
                }
            }

            pub fn set_embedded_object_index_referenced_from(
                pc: usize,
                data: AssemblerBaseEmbeddedObjectIndex,
            ) {
                let instr = pc as *mut Instruction;
                unsafe {
                    if (*instr).is_ldr_literal_x() {
                        *(Self::target_pointer_address_at(pc) as *mut AssemblerBaseEmbeddedObjectIndex) = data;
                    } else {
                        debug_assert!((*instr).is_ldr_literal_w());
                        debug_assert!(is_uint32(data));
                        (Self::target_pointer_address_at(pc) as *mut u32)
                            .write_unaligned(data as u32);
                    }
                }
            }

            pub fn target_object_handle_at(_pc: usize) -> i32 {
                // TODO Implement target_object_handle_at
                0
            }

            pub fn target_builtin_at(_pc: usize) -> i32 {
                // TODO Implement target_builtin_at
                0
            }

            pub fn deserialization_special_target_size(location: usize) -> i32 {
                let instr = location as *mut Instruction;
                unsafe {
                    if (*instr).is_branch_and_link() || (*instr).is_unconditional_branch() {
                        Self::k_special_target_size
                    } else {
                        debug_assert_eq!((*instr).instruction_bits(), 0);
                        Self::k_system_pointer_size
                    }
                }
            }

            pub fn deserialization_set_target_internal_reference_at(
                pc: usize,
                target: usize,
                jit_allocation: &mut WritableJitAllocation,
                _mode: RelocInfoMode,
            ) {
                jit_allocation.write_unaligned_value(pc, target);
            }

            pub fn set_target_address_at(
                pc: usize,
                _constant_pool: usize,
                target: usize,
                jit_allocation: Option<&mut WritableJitAllocation>,
                icache_flush_mode: ICacheFlushMode,
            ) {
                let instr = pc as *mut Instruction;
                unsafe {
                    if (*instr).is_ldr_literal_x() {
                        let target_ptr_addr = Self::target_pointer_address_at(pc);
                        if let Some(jit_alloc) = jit_allocation {
                            jit_alloc.write_value(target_ptr_addr, target);
                        } else {
                            *(target_ptr_addr as *mut usize) = target;
                        }
                        // Intuitively, we would think it is necessary to always flush the
                        // instruction cache after patching a target address in the code. However,
                        // in this case, only the constant pool contents change. The instruction
                        // accessing the constant pool remains unchanged, so a flush is not
                        // required.
                    } else {
                        debug_assert!((*instr).is_branch_and_link() || (*instr).is_unconditional_branch());
                        let mut target_val = target;
                        if target == 0 {
                            // We are simply wiping the target out for serialization. Set the offset
                            // to zero instead.
                            target_val = pc;
                        }
                        (*instr).set_branch_imm_target::<UncondBranchType>(
                            target_val as *mut Instruction,
                            jit_allocation.as_mut().map(|x| &mut **x),
                        );
                        if icache_flush_mode != SKIP_ICACHE_FLUSH {
                            flush_instruction_cache(pc, Assembler::k_instr_size);
                        }
                    }
                }
            }

            pub fn set