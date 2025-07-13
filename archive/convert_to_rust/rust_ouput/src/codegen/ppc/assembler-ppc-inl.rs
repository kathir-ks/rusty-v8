// Converted from V8 C++ source files:
// Header: assembler-ppc-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod assembler_ppc_inl {
use crate::codegen::assembler::Assembler;
use crate::codegen::flush_instruction_cache::FlushInstructionCache;
use crate::codegen::ppc::assembler_ppc::*;
use crate::debug::debug::Debug;
use crate::heap::heap_layout_inl::HeapLayout;
use crate::objects::objects_inl::*;
use std::mem::size_of;
use std::ptr::null_mut;
use crate::codegen::assembler::kNullAddress;
use crate::codegen::ppc::reglist_ppc::kConstantPoolRegister;
use crate::codegen::code_stub_assembler::V;
use crate::codegen::code_stub_assembler::WordPtr;
use crate::codegen::code_stub_assembler::MemoryRepresentation;

pub struct CpuFeatures {}

impl CpuFeatures {
    pub fn supports_optimizer() -> bool {
        true
    }
}

pub struct WritableRelocInfo {
    pc_: usize,
    rmode_: RelocInfo_Mode,
    constant_pool_: usize,
    jit_allocation_: WritableJitAllocation,
}

impl WritableRelocInfo {
    pub fn apply(&mut self, delta: isize) {
        if self.rmode_.is_internal_reference() {
            let target_ptr = self.pc_ as *mut usize;
            unsafe {
                let target = *target_ptr;
                *target_ptr = (target as isize + delta) as usize;
            }
        } else {
            if self.rmode_.is_internal_reference_encoded() {
                let target = Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8);
                Assembler::set_target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, (target as isize + delta) as *mut u8, &mut self.jit_allocation_, ICacheFlushMode::SKIP_ICACHE_FLUSH);
            } else {
                println!("WritableRelocInfo::apply: unimplemented rmode_ = {:?}", self.rmode_);
            }
        }
    }
}

pub struct RelocInfo {
    pc_: usize,
    rmode_: RelocInfo_Mode,
    constant_pool_: usize,
}

impl RelocInfo {
    pub fn target_internal_reference(&self) -> *mut u8 {
        if self.rmode_.is_internal_reference() {
            unsafe { *(self.pc_ as *mut *mut u8) }
        } else {
            if self.rmode_.is_internal_reference_encoded() {
                Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
            } else {
                println!("RelocInfo::target_internal_reference: unimplemented rmode_ = {:?}", self.rmode_);
                null_mut()
            }
        }
    }

    pub fn target_internal_reference_address(&self) -> *mut u8 {
        if self.rmode_.is_internal_reference() || self.rmode_.is_internal_reference_encoded() {
            self.pc_ as *mut u8
        } else {
            println!("RelocInfo::target_internal_reference_address: unimplemented rmode_ = {:?}", self.rmode_);
            null_mut()
        }
    }

    pub fn target_address(&self) -> *mut u8 {
        if self.rmode_.is_code_target() || self.rmode_.is_wasm_call() || self.rmode_.is_wasm_stub_call() {
            Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
        } else {
             println!("RelocInfo::target_address: unimplemented rmode_ = {:?}", self.rmode_);
             null_mut()
        }
    }

    pub fn target_address_address(&self) -> *mut u8 {
        if self.has_target_address_address() {
             if V8_EMBEDDED_CONSTANT_POOL_BOOL && Assembler::is_constant_pool_load_start(self.pc_ as *mut u8) {
                 return self.pc_ as *mut u8;
             }
             return self.pc_ as *mut u8;
         } else {
              println!("RelocInfo::target_address_address: unimplemented rmode_ = {:?}", self.rmode_);
              null_mut()
         }
    }

    pub fn constant_pool_entry_address(&self) -> *mut u8 {
        if V8_EMBEDDED_CONSTANT_POOL_BOOL {
            assert!(self.constant_pool_ != 0);
            let mut access = ConstantPoolEntry_Access::REGULAR;
            if Assembler::is_constant_pool_load_start(self.pc_ as *mut u8, &mut access) {
                return Assembler::target_constant_pool_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, access, ConstantPoolEntry_Type::INTPTR);
            }
        }
        unreachable!();
    }

    pub fn target_address_size(&self) -> i32 {
        if self.is_coded_specially() {
            Assembler::K_SPECIAL_TARGET_SIZE
        } else {
            size_of::<usize>() as i32
        }
    }

    pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
        assert!(self.rmode_.is_code_target() || self.rmode_.is_embedded_object_mode());
        if self.rmode_.is_compressed_embedded_object() {
            let compressed = Assembler::target_compressed_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8);
            assert!(!has_smi_tag(compressed as usize));
            let obj = Tagged::<Object>::new(v8_heap_compression_scheme::decompress_tagged(cage_base, compressed as usize));
            return obj.cast::<HeapObject>();
        } else {
            return Tagged::<Object>::new(Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8) as usize).cast::<HeapObject>();
        }
    }

     pub fn target_external_reference(&self) -> *mut u8 {
        assert_eq!(self.rmode_, RelocInfo_Mode::EXTERNAL_REFERENCE);
        Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
    }

    pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
        assert_eq!(self.rmode_, RelocInfo_Mode::WASM_CODE_POINTER_TABLE_ENTRY);
        WasmCodePointer {
            value_: Assembler::uint32_constant_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
        }
    }

    pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
        assert_eq!(self.rmode_, RelocInfo_Mode::JS_DISPATCH_HANDLE);
        JSDispatchHandle {
            value_: Assembler::uint32_constant_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
        }
    }

    pub fn target_builtin_at(&self, _origin: &mut Assembler) -> Builtin {
        unreachable!()
    }

     pub fn target_off_heap_target(&self) -> *mut u8 {
        assert!(self.rmode_.is_off_heap_target());
        Assembler::target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8)
    }
}

impl WritableRelocInfo {
    pub fn set_target_object(&mut self, target: Tagged<HeapObject>, icache_flush_mode: ICacheFlushMode) {
        assert!(self.rmode_.is_code_target() || self.rmode_.is_embedded_object_mode());
        if self.rmode_.is_compressed_embedded_object() {
            assert!(COMPRESS_POINTERS_BOOL);
            assert!(!V8_ENABLE_SANDBOX_BOOL || !HeapLayout::in_trusted_space(target.into()));
            assert!(!V8_EXTERNAL_CODE_SPACE_BOOL || !HeapLayout::in_code_space(target.into()));

            Assembler::set_target_compressed_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, v8_heap_compression_scheme::compress_object(target.ptr() as usize) as i64, &mut self.jit_allocation_, icache_flush_mode);
        } else {
            assert!(self.rmode_.is_full_embedded_object());
            Assembler::set_target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, target.ptr() as *mut u8, &mut self.jit_allocation_, icache_flush_mode);
        }
    }

    pub fn set_target_external_reference(&mut self, target: *mut u8, icache_flush_mode: ICacheFlushMode) {
        assert_eq!(self.rmode_, RelocInfo_Mode::EXTERNAL_REFERENCE);
        Assembler::set_target_address_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, target, &mut self.jit_allocation_, icache_flush_mode);
    }

    pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
        assert_eq!(self.rmode_, RelocInfo_Mode::WASM_CODE_POINTER_TABLE_ENTRY);
        Assembler::set_uint32_constant_at(self.pc_ as *mut u8, self.constant_pool_ as *mut u8, target.value_, &mut self.jit_allocation_, icache_flush_mode);
    }
}

#[derive(Copy, Clone)]
pub struct Operand {
    rm_: Register,
    rmode_: RelocInfo_Mode,
}

impl Operand {
    pub fn new(rm: Register) -> Self {
        Operand {
            rm_: rm,
            rmode_: RelocInfo_Mode::NO_INFO,
        }
    }
}

impl Assembler {
    pub fn untrack_branch(&mut self) {
        assert!(!self.trampoline_emitted_);
        assert!(self.tracked_branch_count_ > 0);
        self.tracked_branch_count_ -= 1;
        let count = self.tracked_branch_count_;

        if count == 0 {
            self.next_trampoline_check_ = i32::MAX;
        } else {
            self.next_trampoline_check_ += Self::K_TRAMPOLINE_SLOTS_SIZE;
        }
    }

    pub fn target_address_at(pc: *mut u8, constant_pool: *mut u8) -> *mut u8 {
        if V8_EMBEDDED_CONSTANT_POOL_BOOL && !constant_pool.is_null() {
            let mut access = ConstantPoolEntry_Access::REGULAR;
            if Assembler::is_constant_pool_load_start(pc, &mut access) {
                return unsafe {
                    *(Assembler::target_constant_pool_address_at(pc, constant_pool, access, ConstantPoolEntry_Type::INTPTR) as *mut *mut u8)
                };
            }
        }

        unsafe {
            let instr1 = *(pc as *mut u32);
            let instr2 = *(pc.add(Self::K_INSTR_SIZE as usize) as *mut u32);

            if Self::is_lis(instr1) && Self::is_ori(instr2) {
                let instr4 = *(pc.add((3 * Self::K_INSTR_SIZE) as usize) as *mut u32);
                let instr5 = *(pc.add((4 * Self::K_INSTR_SIZE) as usize) as *mut u32);

                let hi: u64 = (((instr1 & Self::K_IMM16_MASK) << 16) | (instr2 & Self::K_IMM16_MASK)) as u64;
                let lo: u64 = (((instr4 & Self::K_IMM16_MASK) << 16) | (instr5 & Self::K_IMM16_MASK)) as u64;

                return ((hi << 32) | lo) as *mut u8;
            }
        }
        unreachable!();
    }

    pub const K_LOAD_INTPTR_OPCODE: u32 = LD;

    pub fn is_constant_pool_load_start(pc: *mut u8, access: &mut ConstantPoolEntry_Access) -> bool {
        unsafe {
            let instr = *(pc as *mut u32);
            let opcode = instr & Self::K_OPCODE_MASK;

            if Self::get_ra(instr) != kConstantPoolRegister {
                return false;
            }

            let overflowed = opcode == ADDIS;
            #[cfg(debug_assertions)]
            {
                let mut opcode_inner = opcode;
                if overflowed {
                    opcode_inner = *(pc.add(Self::K_INSTR_SIZE as usize) as *mut u32) & Self::K_OPCODE_MASK;
                }
                assert!(opcode_inner == Self::K_LOAD_INTPTR_OPCODE || opcode_inner == LFD);
            }

            if overflowed {
                *access = ConstantPoolEntry_Access::OVERFLOWED;
            } else {
                *access = ConstantPoolEntry_Access::REGULAR;
            }
            return true;
        }
    }

    pub fn is_constant_pool_load_end(pc: *mut u8, access: &mut ConstantPoolEntry_Access) -> bool {
        unsafe {
            let instr = *(pc as *mut u32);
            let opcode = instr & Self::K_OPCODE_MASK;
            let mut overflowed = false;

            if !(opcode == Self::K_LOAD_INTPTR_OPCODE || opcode == LFD) {
                return false;
            }

            if Self::get_ra(instr) != kConstantPoolRegister {
                let instr_prev = *(pc.sub(Self::K_INSTR_SIZE as usize) as *mut u32);
                let opcode_prev = instr_prev & Self::K_OPCODE_MASK;

                if (opcode_prev != ADDIS) || Self::get_ra(instr_prev) != kConstantPoolRegister {
                    return false;
                }
                overflowed = true;
            }

            if overflowed {
                *access = ConstantPoolEntry_Access::OVERFLOWED;
            } else {
                *access = ConstantPoolEntry_Access::REGULAR;
            }
            return true;
        }
    }

    pub fn get_constant_pool_offset(pc: *mut u8, access: ConstantPoolEntry_Access, _type: ConstantPoolEntry_Type) -> i32 {
        let overflowed = access == ConstantPoolEntry_Access::OVERFLOWED;
        #[cfg(debug_assertions)]
        {
            let mut access_check = ConstantPoolEntry_Access::REGULAR;
            assert!(Self::is_constant_pool_load_start(pc, &mut access_check));
            assert!(access_check == access);
        }
        let offset: i32;
        unsafe {
            if overflowed {
                offset = ((*(pc as *mut u32) & Self::K_IMM16_MASK) << 16) as i32;
                offset += Self::sign_ext_imm16((*(pc.add(Self::K_INSTR_SIZE as usize) as *mut u32) & Self::K_IMM16_MASK) as i32);
                assert!(!Self::is_int16(offset));
            } else {
                offset = Self::sign_ext_imm16((*(pc as *mut u32) & Self::K_IMM16_MASK) as i32);
            }
        }
        return offset;
    }

    pub fn patch_constant_pool_access_instruction(&mut self, pc_offset: i32, offset: i32, access: ConstantPoolEntry_Access, _type: ConstantPoolEntry_Type) {
        let pc = self.buffer_start_ as usize + pc_offset as usize;
        let overflowed = access == ConstantPoolEntry_Access::OVERFLOWED;
        assert!(overflowed != Self::is_int16(offset));

        #[cfg(debug_assertions)]
        {
            let mut access_check = ConstantPoolEntry_Access::REGULAR;
            let pc_ptr = pc as *mut u8;
            assert!(Self::is_constant_pool_load_start(pc_ptr, &mut access_check));
            assert!(access_check == access);
        }

        unsafe {
            if overflowed {
                let mut hi_word: i32 = offset >> 16;
                let lo_word: i32 = offset & 0xffff;
                if (lo_word & 0x8000) != 0 {
                    hi_word += 1;
                }

                let mut instr1 = *(pc as *mut u32);
                let mut instr2 = *(pc as *mut u32).add(1);

                instr1 &= !Self::K_IMM16_MASK;
                instr1 |= (hi_word & Self::K_IMM16_MASK) as u32;

                instr2 &= !Self::K_IMM16_MASK;
                instr2 |= (lo_word & Self::K_IMM16_MASK) as u32;
                *(pc as *mut u32) = instr1;
                *(pc as *mut u32).add(1) = instr2;
            } else {
                let mut instr = *(pc as *mut u32);
                instr &= !Self::K_IMM16_MASK;
                instr |= (offset & Self::K_IMM16_MASK) as u32;
                *(pc as *mut u32) = instr;
            }
        }
    }

    pub fn target_constant_pool_address_at(pc: *mut u8, constant_pool: *mut u8, access: ConstantPoolEntry_Access, _type: ConstantPoolEntry_Type) -> *mut u8 {
        let mut addr = constant_pool;
        assert!(!addr.is_null());
        addr = (addr as usize + Self::get_constant_pool_offset(pc, access, _type) as usize) as *mut u8;
        return addr;
    }

    pub fn deserialization_special_target_size(_instruction_payload: *mut u8) -> i32 {
        Self::K_SPECIAL_TARGET_SIZE
    }

    pub fn deserialization_set_target_internal_reference_at(&mut self, pc: *mut u8, target: *mut u8, jit_allocation: &mut WritableJitAllocation, mode: RelocInfo_Mode) {
        if mode.is_internal_reference_encoded() {
            self.set_target_address_at(pc, kNullAddress as *mut u8, target, jit_allocation, ICacheFlushMode::SKIP_ICACHE_FLUSH);
        } else {
            jit_allocation.write_unaligned_value(pc as usize, target as usize);
        }
    }

    pub fn set_target_address_at(pc: *mut u8, constant_pool: *mut u8, target: *mut u8, jit_allocation: &mut WritableJitAllocation, icache_flush_mode: ICacheFlushMode) {
        if V8_EMBEDDED_CONSTANT_POOL_BOOL && !constant_pool.is_null() {
            let mut access = ConstantPoolEntry_Access::REGULAR;
            if Self::is_constant_pool_load_start(pc, &mut access) {
                if jit_allocation.size != 0 {
                    jit_allocation.write_unaligned_value(
                        Self::target_constant_pool_address_at(pc, constant_pool, access, ConstantPoolEntry_Type::INTPTR) as usize,
                        target as usize,
                    );
                } else {
                    unsafe {
                        *(Self::target_constant_pool_address_at(pc, constant_pool, access, ConstantPoolEntry_Type::INTPTR) as *mut *mut u8) = target;
                    }
                }
                return;
            }
        }
        unsafe {
            let instr1 = *(pc as *mut u32);
            let instr2 = *(pc.add(Self::K_INSTR_SIZE as usize) as *mut u32);
            if Self::is_lis(instr1) && Self::is_ori(instr2) {
                let instr4 = *(pc.add((3 * Self::K_INSTR_SIZE) as usize) as *mut u32);
                let instr5 = *(pc.add((4 * Self::K_INSTR_SIZE) as usize) as *mut u32);
                let mut p = pc as *mut u32;
                let mut itarget = target as usize;

                let mut instr5_modified = instr5 & !Self::K_IMM16_MASK;
                instr5_modified |= (itarget & Self::K_IMM16_MASK) as u32;
                itarget = itarget >> 16;

                let mut instr4_modified = instr4 & !Self::K_IMM16_MASK;
                instr4_modified |= (itarget & Self::K_IMM16_MASK) as u32;
                itarget = itarget >> 16;

                let mut instr2_modified = instr2 & !Self::K_IMM16_MASK;
                instr2_modified |= (itarget & Self::K_IMM16_MASK) as u32;
                itarget = itarget >> 16;

                let mut instr1_modified = instr1 & !Self::K_IMM16_MASK;
                instr1_modified |= (itarget & Self::K_IMM16_MASK) as u32;
                itarget = itarget >> 16;

                if jit_allocation.size != 0 {
                    jit_allocation.write_unaligned_value(p as usize, instr1_modified as usize);
                    jit_allocation.write_unaligned_value(p.add(1) as usize, instr2_modified as usize);
                    jit_allocation.write_unaligned_value(p.add(3) as usize, instr4_modified as usize);
                    jit_allocation.write_unaligned_value(p.add(4) as usize, instr5_modified as usize);
                } else {
                    *p = instr1_modified;
                    *(p.add(1)) = instr2_modified;
                    *(p.add(3)) = instr4_modified;
                    *(p.add(4)) = instr5_modified;
                }

                if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                    FlushInstructionCache(p as *mut _, 5 * Self::K_INSTR_SIZE);
                }
                return;
            }
        }
        unreachable!();
    }

    pub fn uint32_constant_at(pc: *mut u8, constant_pool: *mut u8) -> u32 {
        Self::target_address_at(pc, constant_pool) as u32
    }

    pub fn set_uint32_constant_at(&mut self, pc: *mut u8, constant_pool: *mut u8, new_constant: u32, jit_allocation: &mut WritableJitAllocation, icache_flush_mode: ICacheFlushMode) {
        Self::set_target_address_at(pc, constant_pool, new_constant as *mut u8, jit_allocation, icache_flush_mode);
    }

    fn is_lis(instr: u32) -> bool {
        (instr >> 26) == LIS >> 26
    }

    fn is_ori(instr: u32) -> bool {
        (instr >> 26) == ORI >> 26
    }

    fn get_ra(instr: u32) -> i32 {
        ((instr >> 16) & 0x1F) as i32
    }

    const K_IMM16_MASK: u32 = 0xFFFF;
    const K_OPCODE_MASK: u32 = 0xFC000000;
    const K_TRAMPOLINE_SLOTS_SIZE: i32 = 1;
}

fn has_smi_tag(value: usize) -> bool {
    (value & 1) == 1
}

mod v8_heap_compression_scheme {
    pub fn compress_object(object: usize) -> i64 {
        object as i64
    }

    pub fn decompress_tagged(_cage_base: PtrComprCageBase, compressed: usize) -> usize {
        compressed
    }
}
}
