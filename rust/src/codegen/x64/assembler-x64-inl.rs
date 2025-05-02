// src/codegen/x64/assembler-x64-inl.rs

//use crate::base::cpu::CpuFeatures;
//use crate::base::memory::*;
//use crate::codegen::flush_instruction_cache::*;
use crate::codegen::x64::assembler_x64::*;
//use crate::debug::debug::*;
//use crate::heap::heap_layout::*;
//use crate::objects::objects::*;

pub mod internal {
    use super::*;
    use std::convert::TryInto;

    pub fn supports_optimizer() -> bool {
        true
    }

    impl Assembler {
        pub fn emit_rex_64(&mut self, reg: Register, rm_reg: Register) {
            self.emit(0x48 | (reg.high_bit() << 2) | rm_reg.high_bit());
        }

        pub fn emit_rex_64_xmm_reg(&mut self, reg: XMMRegister, rm_reg: Register) {
            self.emit(0x48 | ((reg.code() & 0x8) >> 1) | (rm_reg.code() >> 3));
        }

        pub fn emit_rex_64_reg_xmm_reg(&mut self, reg: Register, rm_reg: XMMRegister) {
            self.emit(0x48 | ((reg.code() & 0x8) >> 1) | (rm_reg.code() >> 3));
        }

        pub fn emit_rex_64_xmm_xmm(&mut self, reg: XMMRegister, rm_reg: XMMRegister) {
            self.emit(0x48 | ((reg.code() & 0x8) >> 1) | (rm_reg.code() >> 3));
        }

        pub fn emit_rex_64_reg_operand(&mut self, reg: Register, op: Operand) {
            self.emit(0x48 | (reg.high_bit() << 2) | op.rex());
        }

        pub fn emit_rex_64_xmm_operand(&mut self, reg: XMMRegister, op: Operand) {
            self.emit(0x48 | ((reg.code() & 0x8) >> 1) | op.rex());
        }

        pub fn emit_rex_64_rm_reg(&mut self, rm_reg: Register) {
            debug_assert_eq!(rm_reg.code() & 0xf, rm_reg.code());
            self.emit(0x48 | rm_reg.high_bit());
        }

        pub fn emit_rex_64_operand(&mut self, op: Operand) {
            self.emit(0x48 | op.rex());
        }

        pub fn emit_rex_32(&mut self, reg: Register, rm_reg: Register) {
            self.emit(0x40 | (reg.high_bit() << 2) | rm_reg.high_bit());
        }

        pub fn emit_rex_32_reg_operand(&mut self, reg: Register, op: Operand) {
            self.emit(0x40 | (reg.high_bit() << 2) | op.rex());
        }

        pub fn emit_rex_32_rm_reg(&mut self, rm_reg: Register) {
            self.emit(0x40 | rm_reg.high_bit());
        }

        pub fn emit_rex_32_operand(&mut self, op: Operand) {
            self.emit(0x40 | op.rex());
        }

        pub fn emit_optional_rex_32(&mut self, reg: Register, rm_reg: Register) {
            let rex_bits = (reg.high_bit() << 2) | rm_reg.high_bit();
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_reg_operand(&mut self, reg: Register, op: Operand) {
            let rex_bits = (reg.high_bit() << 2) | op.rex();
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_xmm_operand(&mut self, reg: XMMRegister, op: Operand) {
            let rex_bits = ((reg.code() & 0x8) >> 1) | op.rex();
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_xmm_xmm(&mut self, reg: XMMRegister, base: XMMRegister) {
            let rex_bits = ((reg.code() & 0x8) >> 1) | ((base.code() & 0x8) >> 3);
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_xmm_reg(&mut self, reg: XMMRegister, base: Register) {
            let rex_bits = ((reg.code() & 0x8) >> 1) | ((base.code() & 0x8) >> 3);
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_reg_xmm(&mut self, reg: Register, base: XMMRegister) {
            let rex_bits = ((reg.code() & 0x8) >> 1) | ((base.code() & 0x8) >> 3);
            if rex_bits != 0 {
                self.emit(0x40 | rex_bits);
            }
        }

        pub fn emit_optional_rex_32_rm_reg(&mut self, rm_reg: Register) {
            if rm_reg.high_bit() {
                self.emit(0x41);
            }
        }

        pub fn emit_optional_rex_32_xmm_reg(&mut self, rm_reg: XMMRegister) {
            if rm_reg.high_bit() {
                self.emit(0x41);
            }
        }

        pub fn emit_optional_rex_32_operand(&mut self, op: Operand) {
            if op.rex() != 0 {
                self.emit(0x40 | op.rex());
            }
        }

        pub fn emit_optional_rex_8_reg(&mut self, reg: Register) {
            if !reg.is_byte_register() {
                self.emit_rex_32_rm_reg(reg);
            }
        }

        pub fn emit_optional_rex_8_reg_operand(&mut self, reg: Register, op: Operand) {
            if !reg.is_byte_register() {
                self.emit_rex_32_reg_operand(reg, op);
            } else {
                self.emit_optional_rex_32_reg_operand(reg, op);
            }
        }

        pub fn emit_vex3_byte1_xmm_xmm(&mut self, reg: XMMRegister, rm: XMMRegister, m: LeadingOpcode) {
            let rxb = (!((reg.high_bit() << 2) | rm.high_bit()) as u8) << 5;
            self.emit(rxb | (m as u8));
        }

        pub fn emit_vex3_byte1_xmm_operand(&mut self, reg: XMMRegister, rm: Operand, m: LeadingOpcode) {
            let rxb = (!(reg.high_bit() << 2 | rm.rex()) as u8) << 5;
            self.emit(rxb | (m as u8));
        }

        pub fn emit_vex2_byte1(&mut self, reg: XMMRegister, v: XMMRegister, l: VectorLength, pp: SIMDPrefix) {
            let rv = (!(reg.high_bit() << 4 | v.code()) as u8) << 3;
            self.emit(rv | (l as u8) | (pp as u8));
        }

        pub fn emit_vex3_byte2(&mut self, w: VexW, v: XMMRegister, l: VectorLength, pp: SIMDPrefix) {
            self.emit((w as u8) | (((!v.code() & 0xf) << 3) as u8) | (l as u8) | (pp as u8));
        }

        pub fn emit_vex_prefix_xmm(&mut self, reg: XMMRegister, vreg: XMMRegister, rm: XMMRegister, l: VectorLength, pp: SIMDPrefix, mm: LeadingOpcode, w: VexW) {
            if rm.high_bit() || mm != LeadingOpcode::k0F || w != VexW::kW0 {
                self.emit_vex3_byte0();
                self.emit_vex3_byte1_xmm_xmm(reg, rm, mm);
                self.emit_vex3_byte2(w, vreg, l, pp);
            } else {
                self.emit_vex2_byte0();
                self.emit_vex2_byte1(reg, vreg, l, pp);
            }
        }

        pub fn emit_vex_prefix_reg(&mut self, reg: Register, vreg: Register, rm: Register, l: VectorLength, pp: SIMDPrefix, mm: LeadingOpcode, w: VexW) {
            let ireg = XMMRegister::from_code(reg.code());
            let ivreg = XMMRegister::from_code(vreg.code());
            let irm = XMMRegister::from_code(rm.code());
            self.emit_vex_prefix_xmm(ireg, ivreg, irm, l, pp, mm, w);
        }

        pub fn emit_vex_prefix_xmm_operand(&mut self, reg: XMMRegister, vreg: XMMRegister, rm: Operand, l: VectorLength, pp: SIMDPrefix, mm: LeadingOpcode, w: VexW) {
            if rm.rex() != 0 || mm != LeadingOpcode::k0F || w != VexW::kW0 {
                self.emit_vex3_byte0();
                self.emit_vex3_byte1_xmm_operand(reg, rm, mm);
                self.emit_vex3_byte2(w, vreg, l, pp);
            } else {
                self.emit_vex2_byte0();
                self.emit_vex2_byte1(reg, vreg, l, pp);
            }
        }

        pub fn emit_vex_prefix_reg_operand(&mut self, reg: Register, vreg: Register, rm: Operand, l: VectorLength, pp: SIMDPrefix, mm: LeadingOpcode, w: VexW) {
            let ireg = XMMRegister::from_code(reg.code());
            let ivreg = XMMRegister::from_code(vreg.code());
            self.emit_vex_prefix_xmm_operand(ireg, ivreg, rm, l, pp, mm, w);
        }

        pub fn target_address_at(&self, pc: Address, _constant_pool: Address) -> Address {
            let offset = self.read_unaligned_value::<i32>(pc) as i64;
            (pc as i64 + offset + 4) as Address
        }

        pub fn set_target_address_at(
            &mut self,
            pc: Address,
            _constant_pool: Address,
            target: Address,
            jit_allocation: Option<&mut WritableJitAllocation>,
            icache_flush_mode: ICacheFlushMode,
        ) {
            let offset = self.relative_target_offset(target, pc);
            if let Some(jit_allocation) = jit_allocation {
                jit_allocation.write_unaligned_value(pc, offset);
            } else {
                self.write_unaligned_value(pc, offset);
            }
            if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                flush_instruction_cache(pc, std::mem::size_of::<i32>());
            }
        }

        pub fn relative_target_offset(&self, target: Address, pc: Address) -> i32 {
            let offset: Address = target.wrapping_sub(pc).wrapping_sub(4);
            assert!(is_int32(offset));
            offset as i32
        }

        //The original code used RelocInfo::Mode. Since this is not implemented, I'll just use a u32 for now.
        pub fn deserialization_set_target_internal_reference_at(
            &mut self,
            pc: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
            _mode: u32, //RelocInfo::Mode
        ) {
            jit_allocation.write_unaligned_value(pc, target);
        }

        pub fn deserialization_special_target_size(&self, _instruction_payload: Address) -> usize {
            kSpecialTargetSize
        }

        //DirectHandle and Code are not translated
        // pub fn code_target_object_handle_at(&self, pc: Address) -> DirectHandle<Code> {
        //     GetCodeTarget(self.read_unaligned_value::<i32>(pc))
        // }

        //DirectHandle and HeapObject are not translated
        // pub fn compressed_embedded_object_handle_at(&self, pc: Address) -> DirectHandle<HeapObject> {
        //     GetEmbeddedObject(self.read_unaligned_value::<u32>(pc))
        // }

        pub fn target_builtin_at(&self, pc: Address) -> Builtin {
            let builtin_id = self.read_unaligned_value::<i32>(pc);
            debug_assert!(Builtins::is_builtin_id(builtin_id));
            unsafe { std::mem::transmute::<i32, Builtin>(builtin_id) }
        }

        pub fn uint32_constant_at(&self, pc: Address, _constant_pool: Address) -> u32 {
            self.read_unaligned_value::<u32>(pc)
        }

        pub fn set_uint32_constant_at(
            &mut self,
            pc: Address,
            _constant_pool: Address,
            new_constant: u32,
            jit_allocation: Option<&mut WritableJitAllocation>,
            icache_flush_mode: ICacheFlushMode,
        ) {
            if let Some(jit_allocation) = jit_allocation {
                jit_allocation.write_unaligned_value::<u32>(pc, new_constant);
            } else {
                self.write_unaligned_value::<u32>(pc, new_constant);
            }
            if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                flush_instruction_cache(pc, std::mem::size_of::<u32>());
            }
        }
    }

    // Implementation of RelocInfo
    // The modes possibly affected by apply must be in kApplyMask.
    impl WritableRelocInfo {
        pub fn apply(&mut self, delta: isize) {
            if self.is_code_target() || self.is_near_builtin_entry() || self.is_wasm_stub_call() {
                let current_value = self.jit_allocation.read_unaligned_value::<i32>(self.pc);
                self.jit_allocation.write_unaligned_value(
                    self.pc,
                    current_value - delta.try_into().unwrap(), //static_cast<int32_t>(delta)
                );
            } else if self.is_internal_reference() {
                // Absolute code pointer inside code object moves with the code object.
                let current_value = self.jit_allocation.read_unaligned_value::<Address>(self.pc);
                self.jit_allocation.write_unaligned_value(self.pc, current_value + delta as Address);
            }
        }
    }

    impl RelocInfo {
        pub fn target_address(&self) -> Address {
            debug_assert!(self.is_code_target() || self.is_near_builtin_entry() || self.is_wasm_call() || self.is_wasm_stub_call());
            Assembler::target_address_at(&Assembler{}, self.pc, self.constant_pool)
        }

        pub fn target_address_address(&self) -> Address {
            debug_assert!(
                self.is_code_target()
                    || self.is_wasm_call()
                    || self.is_wasm_stub_call()
                    || self.is_full_embedded_object()
                    || self.is_compressed_embedded_object()
                    || self.is_external_reference()
                    || self.is_off_heap_target()
            );
            self.pc
        }

        pub fn constant_pool_entry_address(&self) -> Address {
            unreachable!()
        }

        pub fn target_address_size(&self) -> usize {
            if self.is_coded_specially() {
                kSpecialTargetSize
            } else {
                if self.is_compressed_embedded_object() {
                    kTaggedSize
                } else {
                    kSystemPointerSize
                }
            }
        }

        // Tagged<HeapObject> is not translated
        // pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
        //     debug_assert!(self.is_code_target() || self.is_embedded_object_mode());
        //     if self.is_compressed_embedded_object() {
        //         let compressed = self.read_unaligned_value::<Tagged_t>(self.pc);
        //         debug_assert!(!HAS_SMI_TAG(compressed));
        //         let obj = V8HeapCompressionScheme::DecompressTagged(cage_base, compressed);
        //         return obj.unchecked_cast::<HeapObject>();
        //     }
        //     debug_assert!(self.is_full_embedded_object());
        //     return self.read_unaligned_value::<Address>(self.pc).unchecked_cast::<HeapObject>();
        // }

        //DirectHandle<HeapObject> is not translated
        // pub fn target_object_handle(&self, origin: &Assembler) -> DirectHandle<HeapObject> {
        //     debug_assert!(self.is_code_target() || self.is_embedded_object_mode());
        //     if self.is_code_target() {
        //         return origin.code_target_object_handle_at(self.pc);
        //     } else {
        //         if self.is_compressed_embedded_object() {
        //             return origin.compressed_embedded_object_handle_at(self.pc);
        //         }
        //         debug_assert!(self.is_full_embedded_object());
        //         return self.read_unaligned_value::<IndirectHandle<Object>>(self.pc).unchecked_cast::<HeapObject>();
        //     }
        // }

        pub fn target_external_reference(&self) -> Address {
            debug_assert_eq!(self.rmode, RelocInfo::EXTERNAL_REFERENCE);
            self.read_unaligned_value::<Address>(self.pc)
        }
    }

    impl WritableRelocInfo {
        pub fn set_target_external_reference(&mut self, target: Address, icache_flush_mode: ICacheFlushMode) {
            debug_assert_eq!(self.rmode, RelocInfo::EXTERNAL_REFERENCE);
            self.jit_allocation.write_unaligned_value(self.pc, target);
            if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                flush_instruction_cache(self.pc, std::mem::size_of::<Address>());
            }
        }

        pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
            debug_assert_eq!(self.rmode, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
            self.jit_allocation.write_unaligned_value(self.pc, target.value());
            if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                flush_instruction_cache(self.pc, std::mem::size_of::<Address>());
            }
        }

    // Tagged<HeapObject> is not translated
    //     pub fn set_target_object(&mut self, target: Tagged<HeapObject>, icache_flush_mode: ICacheFlushMode) {
    //         debug_assert!(self.is_code_target() || self.is_embedded_object_mode());
    //         if self.is_compressed_embedded_object() {
    //             debug_assert!(COMPRESS_POINTERS_BOOL);
    //             // We must not compress pointers to objects outside of the main pointer
    //             // compression cage as we wouldn't be able to decompress them with the
    //             // correct cage base.
    //             //debug_assert_implies!(V8_ENABLE_SANDBOX_BOOL, !HeapLayout::InTrustedSpace(target));
    //             //debug_assert_implies!(V8_EXTERNAL_CODE_SPACE_BOOL, !HeapLayout::InCodeSpace(target));
    //             //Tagged_t tagged = V8HeapCompressionScheme::CompressObject(target.ptr());
    //             //self.jit_allocation.WriteUnalignedValue(self.pc_, tagged);
    //         } else {
    //             debug_assert!(self.is_full_embedded_object());
    //             self.jit_allocation.write_unaligned_value(self.pc, target.ptr());
    //         }
    //         if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
    //             flush_instruction_cache(self.pc, std::mem::size_of::<Address>());
    //         }
    //     }
    }

    impl RelocInfo {
        pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
            debug_assert_eq!(self.rmode, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
            WasmCodePointer {
                value: self.read_unaligned_value::<u32>(self.pc),
            }
        }

        pub fn target_internal_reference(&self) -> Address {
            debug_assert_eq!(self.rmode, RelocInfo::INTERNAL_REFERENCE);
            self.read_unaligned_value::<Address>(self.pc)
        }

        pub fn target_internal_reference_address(&self) -> Address {
            debug_assert_eq!(self.rmode, RelocInfo::INTERNAL_REFERENCE);
            self.pc
        }

        //JSDispatchHandle is not translated
        // pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
        //     debug_assert_eq!(self.rmode, RelocInfo::JS_DISPATCH_HANDLE);
        //     self.read_unaligned_value::<JSDispatchHandle>(self.pc)
        // }

        pub fn target_builtin_at(&self) -> Builtin {
            debug_assert!(self.is_near_builtin_entry());
            Assembler::target_builtin_at(&Assembler{}, self.pc)
        }

        pub fn target_off_heap_target(&self) -> Address {
            debug_assert!(self.is_off_heap_target());
            self.read_unaligned_value::<Address>(self.pc)
        }
    }
}