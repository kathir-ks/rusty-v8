// Converted from V8 C++ source files:
// Header: assembler-s390-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod assembler_s390_inl {
use crate::codegen::assembler::*;
use crate::codegen::flush_instruction_cache::*;
use crate::codegen::s390::assembler_s390::*;
use crate::debug::debug::*;
use crate::objects::objects_inl::*;
use crate::execution::s390::simulator_s390::SixByteInstr;
use crate::codegen::s390::register_s390::Register;
use crate::codegen::s390::register_s390::MSAControlRegister;
use crate::codegen::s390::register_s390::Opcode;
use crate::codegen::s390::register_s390::BRASL;
use crate::codegen::s390::register_s390::BRCL;
use crate::codegen::s390::register_s390::IIHF;
use crate::codegen::s390::register_s390::IILF;
use crate::codegen::code_stub_assembler::V8HeapCompressionScheme;
use crate::codegen::code_stub_assembler::PtrComprCageBase;
use crate::codegen::ia32::assembler_ia32_inl::Builtin;
use crate::sandbox::js_dispatch_table::JSDispatchHandle;
use crate::codegen::ia32::assembler_ia32_inl::WasmCodePointer;
use std::ptr::null_mut;

    pub struct CpuFeatures {}
    impl CpuFeatures {
        pub fn SupportsOptimizer() -> bool {
            return true;
        }
    }

    impl WritableRelocInfo {
        pub fn apply(&mut self, delta: i64) {
            // Absolute code pointer inside code object moves with the code object.
            if RelocInfo::IsInternalReference(self.rmode_) {
                // Jump table entry
                let target = unsafe { *(self.pc_ as *mut Address) };
                self.jit_allocation_.WriteValue(self.pc_, target + delta);
            } else if RelocInfo::IsCodeTarget(self.rmode_) {
                let instr = unsafe {
                    Instruction::InstructionBits(self.pc_ as *const u8)
                };
                let mut dis = (instr & 0xFFFFFFFF) as i32 * 2; // halfwords
                dis -= delta as i32;
                let mut instr_shifted = instr >> 32; // Clear the 4-byte displacement field.
                instr_shifted <<= 32;
                instr_shifted |= (dis / 2) as u32 as u64;
                unsafe {
                    Instruction::SetInstructionBits::<SixByteInstr>(
                        self.pc_ as *mut u8,
                        instr_shifted,
                        &mut self.jit_allocation_,
                    );
                }
            } else {
                // mov sequence
                debug_assert!(RelocInfo::IsInternalReferenceEncoded(self.rmode_));
                let target = Assembler::target_address_at(self.pc_, self.constant_pool_);
                Assembler::set_target_address_at(
                    self.pc_,
                    self.constant_pool_,
                    target + delta,
                    &mut self.jit_allocation_,
                    ICacheFlushMode::SKIP_ICACHE_FLUSH,
                );
            }
        }
    }

    impl RelocInfo {
        pub fn target_internal_reference(&self) -> Address {
            if RelocInfo::IsInternalReference(self.rmode_) {
                // Jump table entry
                return unsafe { *(self.pc_ as *mut Address) };
            } else {
                // mov sequence
                debug_assert!(RelocInfo::IsInternalReferenceEncoded(self.rmode_));
                return Assembler::target_address_at(self.pc_, self.constant_pool_);
            }
        }

        pub fn target_internal_reference_address(&self) -> Address {
            debug_assert!(RelocInfo::IsInternalReference(self.rmode_) || RelocInfo::IsInternalReferenceEncoded(self.rmode_));
            return self.pc_;
        }

        pub fn target_address(&self) -> Address {
            debug_assert!(RelocInfo::IsRelativeCodeTarget(self.rmode_) || RelocInfo::IsCodeTarget(self.rmode_) ||
                         RelocInfo::IsWasmCall(self.rmode_) || RelocInfo::IsWasmStubCall(self.rmode_));
            return Assembler::target_address_at(self.pc_, self.constant_pool_);
        }

        pub fn target_address_address(&self) -> Address {
            debug_assert!(self.HasTargetAddressAddress());

            // Read the address of the word containing the target_address in an
            // instruction stream.
            // The only architecture-independent user of this function is the serializer.
            // The serializer uses it to find out how many raw bytes of instruction to
            // output before the next target.
            // For an instruction like LIS/ORI where the target bits are mixed into the
            // instruction bits, the size of the target will be zero, indicating that the
            // serializer should not step forward in memory after a target is resolved
            // and written.
            return self.pc_;
        }

        pub fn constant_pool_entry_address(&self) -> Address {
            unreachable!();
        }

        pub fn target_address_size(&self) -> i32 {
            if self.IsCodedSpecially() {
                Assembler::kSpecialTargetSize
            } else {
                kSystemPointerSize
            }
        }

        pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
            debug_assert!(RelocInfo::IsCodeTarget(self.rmode_) || RelocInfo::IsEmbeddedObjectMode(self.rmode_));
            if RelocInfo::IsCompressedEmbeddedObject(self.rmode_) {
                return unsafe { std::mem::transmute(Tagged::<Object>::from(V8HeapCompressionScheme::DecompressTagged(
                    cage_base,
                    Assembler::target_compressed_address_at(self.pc_, self.constant_pool_),
                ))) };
            } else {
                return unsafe { std::mem::transmute(Tagged::<Object>::from(Assembler::target_address_at(self.pc_, self.constant_pool_))) };
            }
        }

         pub fn target_object_handle(&self, origin: &mut Assembler) -> DirectHandle<HeapObject> {
            debug_assert!(RelocInfo::IsRelativeCodeTarget(self.rmode_) || RelocInfo::IsCodeTarget(self.rmode_) ||
                         RelocInfo::IsEmbeddedObjectMode(self.rmode_));
            if RelocInfo::IsCodeTarget(self.rmode_) || RelocInfo::IsRelativeCodeTarget(self.rmode_) {
                return unsafe { std::mem::transmute(origin.code_target_object_handle_at(self.pc_)) };
            } else {
                if RelocInfo::IsCompressedEmbeddedObject(self.rmode_) {
                    return unsafe { std::mem::transmute(origin.compressed_embedded_object_handle_at(self.pc_, self.constant_pool_)) };
                }
                return DirectHandle::<HeapObject>::FromSlot(unsafe {std::mem::transmute(Assembler::target_address_at(self.pc_, self.constant_pool_) as *mut Address)});
            }
        }

        pub fn target_external_reference(&self) -> Address {
            debug_assert_eq!(self.rmode_, RelocInfo::EXTERNAL_REFERENCE);
            return Assembler::target_address_at(self.pc_, self.constant_pool_);
        }

        pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
            debug_assert_eq!(self.rmode_, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
            return WasmCodePointer { value: Assembler::uint32_constant_at(self.pc_, self.constant_pool_) };
        }

         pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
            debug_assert_eq!(self.rmode_, RelocInfo::JS_DISPATCH_HANDLE);
            return JSDispatchHandle(Assembler::uint32_constant_at(self.pc_, self.constant_pool_));
        }

        pub fn target_builtin_at(&self, _origin: &mut Assembler) -> Builtin {
            unreachable!();
        }

        pub fn target_off_heap_target(&self) -> Address {
            debug_assert!(RelocInfo::IsOffHeapTarget(self.rmode_));
            return Assembler::target_address_at(self.pc_, self.constant_pool_);
        }
    }

    impl WritableRelocInfo {
        pub fn set_target_object(&mut self, target: Tagged<HeapObject>, icache_flush_mode: ICacheFlushMode) {
            debug_assert!(RelocInfo::IsCodeTarget(self.rmode_) || RelocInfo::IsEmbeddedObjectMode(self.rmode_));
            if RelocInfo::IsCompressedEmbeddedObject(self.rmode_) {
                Assembler::set_target_compressed_address_at(
                    self.pc_,
                    self.constant_pool_,
                    unsafe {std::mem::transmute(V8HeapCompressionScheme::CompressObject(target.ptr()))},
                    &mut self.jit_allocation_,
                    icache_flush_mode,
                );
            } else {
                debug_assert!(RelocInfo::IsFullEmbeddedObject(self.rmode_));
                Assembler::set_target_address_at(
                    self.pc_,
                    self.constant_pool_,
                    unsafe {std::mem::transmute(target.ptr())},
                    &mut self.jit_allocation_,
                    icache_flush_mode,
                );
            }
        }

         pub fn set_target_external_reference(&mut self, target: Address, icache_flush_mode: ICacheFlushMode) {
            debug_assert_eq!(self.rmode_, RelocInfo::EXTERNAL_REFERENCE);
            Assembler::set_target_address_at(
                self.pc_,
                self.constant_pool_,
                target,
                &mut self.jit_allocation_,
                icache_flush_mode,
            );
        }

        pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
            debug_assert_eq!(self.rmode_, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
            Assembler::set_uint32_constant_at(self.pc_, self.constant_pool_, target.value(),
                                        &mut self.jit_allocation_, icache_flush_mode);
        }
    }

    // Operand constructors
    impl Operand {
        pub fn new(rm: Register) -> Self {
            Operand { rm_: rm, rmode_: RelocInfo::NO_INFO }
        }
    }

    // Fetch the 32bit value from the FIXED_SEQUENCE IIHF / IILF
    impl Assembler {
        pub fn target_address_at(pc: Address, constant_pool: Address) -> Address {
            // S390 Instruction!
            // We want to check for instructions generated by Asm::mov()
            let op1 = unsafe {
                Instruction::S390OpcodeValue(pc as *const u8)
            };
            let instr_1 = unsafe {
                Instruction::InstructionBits(pc as *const u8)
            };

            if op1 == BRASL || op1 == BRCL {
                let dis = (instr_1 & 0xFFFFFFFF) as i32 * 2;
                return pc.wrapping_add(dis as usize);
            }

            let instr1_length = unsafe {
                Instruction::InstructionLength(pc as *const u8)
            };
            let op2 = unsafe {
                Instruction::S390OpcodeValue(pc.wrapping_add(instr1_length) as *const u8)
            };
            let instr_2 = unsafe {
                Instruction::InstructionBits(pc.wrapping_add(instr1_length) as *const u8)
            };
            // IIHF for hi_32, IILF for lo_32
            if op1 == IIHF && op2 == IILF {
                let high = (instr_1 & 0xFFFFFFFF) << 32;
                let low = instr_2 & 0xFFFFFFFF;
                return (high | low) as Address;
            }
            unsafe{UNIMPLEMENTED()};
            0
        }

        pub fn deserialization_special_target_size(instruction_payload: Address) -> i32 {
            Assembler::kSpecialTargetSize
        }

        pub fn deserialization_set_target_internal_reference_at(
            pc: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
            mode: RelocInfo::Mode,
        ) {
            if RelocInfo::IsInternalReferenceEncoded(mode) {
                Assembler::set_target_address_at(
                    pc,
                    kNullAddress,
                    target,
                    jit_allocation,
                    ICacheFlushMode::SKIP_ICACHE_FLUSH,
                );
            } else {
                jit_allocation.WriteUnalignedValue::<Address>(pc, target);
            }
        }

        // This code assumes the FIXED_SEQUENCE of IIHF/IILF
        pub fn set_target_address_at(
            pc: Address,
            constant_pool: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            // Check for instructions generated by Asm::mov()
            let op1 = unsafe {
                Instruction::S390OpcodeValue(pc as *const u8)
            };
            let instr_1 = unsafe {
                Instruction::InstructionBits(pc as *const u8)
            };
            let mut patched = false;

            if op1 == BRASL || op1 == BRCL {
                let mut instr_1_shifted = instr_1 >> 32; // Zero out the lower 32-bits
                instr_1_shifted <<= 32;
                let halfwords = (target as i64 - pc as i64) / 2; // number of halfwords
                instr_1_shifted |= halfwords as u32 as u64;
                unsafe {
                    Instruction::SetInstructionBits::<SixByteInstr>(
                        pc as *mut u8,
                        instr_1_shifted,
                        jit_allocation,
                    );
                }
                if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                    FlushInstructionCache(pc, 6);
                }
                patched = true;
            } else {
                let instr1_length = unsafe {
                    Instruction::InstructionLength(pc as *const u8)
                };
                let op2 = unsafe {
                    Instruction::S390OpcodeValue(pc.wrapping_add(instr1_length) as *const u8)
                };
                let instr_2 = unsafe {
                    Instruction::InstructionBits(pc.wrapping_add(instr1_length) as *const u8)
                };
                // IIHF for hi_32, IILF for lo_32
                if op1 == IIHF && op2 == IILF {
                    // IIHF
                    let mut instr_1_shifted = instr_1 >> 32; // Zero out the lower 32-bits
                    instr_1_shifted <<= 32;
                    instr_1_shifted |= (target as u64) >> 32;

                    unsafe {
                        Instruction::SetInstructionBits::<SixByteInstr>(
                            pc as *mut u8,
                            instr_1_shifted,
                            jit_allocation,
                        );
                    }

                    // IILF
                    let mut instr_2_shifted = instr_2 >> 32;
                    instr_2_shifted <<= 32;
                    instr_2_shifted |= (target as u64) & 0xFFFFFFFF;

                    unsafe {
                        Instruction::SetInstructionBits::<SixByteInstr>(
                            pc.wrapping_add(instr1_length) as *mut u8,
                            instr_2_shifted,
                            jit_allocation,
                        );
                    }
                    if icache_flush_mode != ICacheFlushMode::SKIP_ICACHE_FLUSH {
                        FlushInstructionCache(pc, 12);
                    }
                    patched = true;
                }
            }
            if !patched {
                unsafe{UNREACHABLE()};
            }
        }

        pub fn target_compressed_address_at(pc: Address, const_pool: Address) -> Tagged_t {
            Assembler::target_address_at(pc, const_pool) as Tagged_t
        }

        pub fn set_target_compressed_address_at(
            pc: Address,
            constant_pool: Address,
            target: Tagged_t,
            jit_allocation: &mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            Assembler::set_target_address_at(
                pc,
                constant_pool,
                target as Address,
                jit_allocation,
                icache_flush_mode,
            );
        }

         pub fn code_target_object_handle_at(pc: Address) -> Handle<Object> {
            let instr = unsafe {
                Instruction::InstructionBits(pc as *const u8)
            };
            let index = instr & 0xFFFFFFFF;
            Assembler::GetCodeTarget(index as i32)
        }

        pub fn compressed_embedded_object_handle_at(pc: Address, const_pool: Address) -> Handle<HeapObject> {
             Assembler::GetEmbeddedObject(Assembler::target_compressed_address_at(pc, const_pool))
        }

        pub fn uint32_constant_at(pc: Address, constant_pool: Address) -> u32 {
            Assembler::target_address_at(pc, constant_pool) as u32
        }

        pub fn set_uint32_constant_at(pc: Address, constant_pool: Address, new_constant: u32,
                                    jit_allocation: &mut WritableJitAllocation,
                                    icache_flush_mode: ICacheFlushMode) {
            Assembler::set_target_address_at(pc, constant_pool, new_constant as Address,
                                        jit_allocation, icache_flush_mode);
        }
    }
}
