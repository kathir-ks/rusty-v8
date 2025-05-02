pub mod s390 {
    pub mod assembler_s390_inl {
        use crate::codegen::assembler::Assembler;
        use crate::codegen::flush_instruction_cache::FlushInstructionCache;
        use crate::codegen::s390::assembler_s390::*;
        use crate::debug::debug::UNREACHABLE;
        use crate::objects::objects::*;
        use crate::objects::objects_inl::*;
        use std::ptr::null_mut;

        pub struct CpuFeatures {}

        impl CpuFeatures {
            pub fn supports_optimizer() -> bool {
                true
            }
        }

        pub struct WritableRelocInfo<'a> {
            pub pc_: usize,
            pub rmode_: RelocInfoMode,
            pub constant_pool_: usize,
            pub jit_allocation_: &'a mut WritableJitAllocation,
        }

        impl<'a> WritableRelocInfo<'a> {
            pub fn apply(&mut self, delta: isize) {
                if RelocInfo::is_internal_reference(self.rmode_) {
                    let target = unsafe { *(self.pc_ as *mut usize) };
                    self.jit_allocation_.write_value(self.pc_, (target as isize + delta) as usize);
                } else if RelocInfo::is_code_target(self.rmode_) {
                    let instr =
                        Instruction::instruction_bits(self.pc_ as *const u8);
                    let mut dis = (instr & 0xFFFFFFFF) as i32 * 2 - delta as i32;
                    let mut instr = instr >> 32;
                    instr <<= 32;
                    instr |= (dis / 2) as u64;

                    Instruction::set_instruction_bits(
                        self.pc_ as *mut u8,
                        instr,
                        self.jit_allocation_,
                    );
                } else {
                    debug_assert!(RelocInfo::is_internal_reference_encoded(self.rmode_));
                    let target = Assembler::target_address_at(self.pc_, self.constant_pool_);
                    Assembler::set_target_address_at(
                        self.pc_,
                        self.constant_pool_,
                        (target as isize + delta) as usize,
                        self.jit_allocation_,
                        ICacheFlushMode::SkipICacheFlush,
                    );
                }
            }

            pub fn set_target_object(&mut self, target: Tagged<HeapObject>, icache_flush_mode: ICacheFlushMode) {
                debug_assert!(RelocInfo::is_code_target(self.rmode_) || RelocInfo::is_embedded_object_mode(self.rmode_));
                if RelocInfo::is_compressed_embedded_object(self.rmode_) {
                    Assembler::set_target_compressed_address_at(
                        self.pc_,
                        self.constant_pool_,
                        V8HeapCompressionScheme::compress_object(target.ptr()),
                        self.jit_allocation_,
                        icache_flush_mode,
                    );
                } else {
                    debug_assert!(RelocInfo::is_full_embedded_object(self.rmode_));
                    Assembler::set_target_address_at(
                        self.pc_,
                        self.constant_pool_,
                        target.ptr(),
                        self.jit_allocation_,
                        icache_flush_mode,
                    );
                }
            }

            pub fn set_target_external_reference(&mut self, target: usize, icache_flush_mode: ICacheFlushMode) {
                debug_assert_eq!(self.rmode_, RelocInfoMode::ExternalReference);
                Assembler::set_target_address_at(self.pc_, self.constant_pool_, target, self.jit_allocation_, icache_flush_mode);
            }

            pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
                debug_assert_eq!(self.rmode_, RelocInfoMode::WasmCodePointerTableEntry);
                Assembler::set_uint32_constant_at(self.pc_, self.constant_pool_, target.value(), self.jit_allocation_, icache_flush_mode);
            }
        }

        pub struct RelocInfo {
            pub pc_: usize,
            pub rmode_: RelocInfoMode,
            pub constant_pool_: usize,
        }

        impl RelocInfo {
            pub fn is_internal_reference(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::InternalReference
            }

            pub fn is_code_target(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::CodeTarget
            }

            pub fn is_relative_code_target(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::RelativeCodeTarget
            }

             pub fn is_wasm_call(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::WasmCall
            }

            pub fn is_wasm_stub_call(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::WasmStubCall
            }

            pub fn is_internal_reference_encoded(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::InternalReferenceEncoded
            }

            pub fn is_embedded_object_mode(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::EmbeddedObject
            }

            pub fn is_compressed_embedded_object(rmode_: RelocInfoMode) -> bool {
                 rmode_ == RelocInfoMode::CompressedEmbeddedObject
            }

            pub fn is_full_embedded_object(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::FullEmbeddedObject
            }

            pub fn is_off_heap_target(rmode_: RelocInfoMode) -> bool {
                 rmode_ == RelocInfoMode::OffHeapTarget
            }

            pub fn is_coded_specially(rmode_: RelocInfoMode) -> bool {
                rmode_ == RelocInfoMode::CodeTarget
            }

            pub fn has_target_address_address(&self) -> bool {
                true
            }

            pub fn target_internal_reference(&self) -> usize {
                if RelocInfo::is_internal_reference(self.rmode_) {
                    unsafe { *(self.pc_ as *mut usize) }
                } else {
                    debug_assert!(RelocInfo::is_internal_reference_encoded(self.rmode_));
                    Assembler::target_address_at(self.pc_, self.constant_pool_)
                }
            }

            pub fn target_internal_reference_address(&self) -> usize {
                debug_assert!(RelocInfo::is_internal_reference(self.rmode_) || RelocInfo::is_internal_reference_encoded(self.rmode_));
                self.pc_
            }

            pub fn target_address(&self) -> usize {
                debug_assert!(RelocInfo::is_relative_code_target(self.rmode_) || RelocInfo::is_code_target(self.rmode_) || RelocInfo::is_wasm_call(self.rmode_) || RelocInfo::is_wasm_stub_call(self.rmode_));
                Assembler::target_address_at(self.pc_, self.constant_pool_)
            }

            pub fn target_address_address(&self) -> usize {
                self.pc_
            }

            pub fn constant_pool_entry_address(&self) -> usize {
                UNREACHABLE()
            }

            pub fn target_address_size(&self) -> i32 {
                if RelocInfo::is_coded_specially(self.rmode_) {
                    Assembler::k_special_target_size
                } else {
                    std::mem::size_of::<usize>() as i32 // kSystemPointerSize
                }
            }

            pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
                debug_assert!(RelocInfo::is_code_target(self.rmode_) || RelocInfo::is_embedded_object_mode(self.rmode_));
                if RelocInfo::is_compressed_embedded_object(self.rmode_) {
                    let compressed = Assembler::target_compressed_address_at(self.pc_, self.constant_pool_);
                    let decompressed = V8HeapCompressionScheme::decompress_tagged(cage_base, compressed);
                    Tagged::<Object>::from_ptr(decompressed as *mut _).unchecked_cast()
                } else {
                    Tagged::<Object>::from_ptr(Assembler::target_address_at(self.pc_, self.constant_pool_) as *mut _).unchecked_cast()
                }
            }

            pub fn target_object_handle<'a>(&self, origin: &Assembler) -> DirectHandle<HeapObject> {
                debug_assert!(RelocInfo::is_relative_code_target(self.rmode_) || RelocInfo::is_code_target(self.rmode_) || RelocInfo::is_embedded_object_mode(self.rmode_));
                if RelocInfo::is_code_target(self.rmode_) || RelocInfo::is_relative_code_target(self.rmode_) {
                    Cast::<HeapObject>::unchecked_from_handle(origin.code_target_object_handle_at(self.pc_))
                } else {
                    if RelocInfo::is_compressed_embedded_object(self.rmode_) {
                        Cast::<HeapObject>::unchecked_from_handle(origin.compressed_embedded_object_handle_at(self.pc_, self.constant_pool_))
                    } else {
                        DirectHandle::<HeapObject>::from_slot(Assembler::target_address_at(self.pc_, self.constant_pool_) as *mut _)
                    }
                }
            }

            pub fn target_external_reference(&self) -> usize {
                debug_assert_eq!(self.rmode_, RelocInfoMode::ExternalReference);
                Assembler::target_address_at(self.pc_, self.constant_pool_)
            }

            pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
                debug_assert_eq!(self.rmode_, RelocInfoMode::WasmCodePointerTableEntry);
                WasmCodePointer {
                    value: Assembler::uint32_constant_at(self.pc_, self.constant_pool_),
                }
            }

            pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
                debug_assert_eq!(self.rmode_, RelocInfoMode::JSDispatchHandle);
                JSDispatchHandle(Assembler::uint32_constant_at(self.pc_, self.constant_pool_))
            }

            pub fn target_builtin_at<'a>(&self, origin: &Assembler) -> Builtin {
                UNREACHABLE()
            }

             pub fn target_off_heap_target(&self) -> usize {
                debug_assert!(RelocInfo::is_off_heap_target(self.rmode_));
                Assembler::target_address_at(self.pc_, self.constant_pool_)
            }
        }

        pub struct Operand {
            rm_: Register,
            rmode_: RelocInfoMode,
        }

        impl Operand {
            pub fn new(rm: Register) -> Self {
                Operand {
                    rm_: rm,
                    rmode_: RelocInfoMode::NoInfo,
                }
            }
        }

        impl Assembler {
            pub fn target_address_at(pc: usize, constant_pool: usize) -> usize {
                let op1 = Instruction::s390_opcode_value(pc as *const u8);
                let instr_1 = Instruction::instruction_bits(pc as *const u8);

                if op1 == Opcode::BRASL || op1 == Opcode::BRCL {
                    let dis = (instr_1 & 0xFFFFFFFF) as i32 * 2;
                    return (pc as isize + dis as isize) as usize;
                }

                let instr1_length = Instruction::instruction_length(pc as *const u8);
                let op2 = Instruction::s390_opcode_value((pc as isize + instr1_length as isize) as *const u8);
                let instr_2 = Instruction::instruction_bits((pc as isize + instr1_length as isize) as *const u8);

                if op1 == Opcode::IIHF && op2 == Opcode::IILF {
                    return (((instr_1 & 0xFFFFFFFF) << 32) | (instr_2 & 0xFFFFFFFF)) as usize;
                }

                UNREACHABLE();
                0
            }

            pub fn deserialization_special_target_size(instruction_payload: usize) -> i32 {
                Assembler::k_special_target_size
            }

            pub fn deserialization_set_target_internal_reference_at(
                pc: usize,
                target: usize,
                jit_allocation: &mut WritableJitAllocation,
                mode: RelocInfoMode,
            ) {
                if RelocInfo::is_internal_reference_encoded(mode) {
                    Assembler::set_target_address_at(
                        pc,
                        0, // kNullAddress
                        target,
                        jit_allocation,
                        ICacheFlushMode::SkipICacheFlush,
                    );
                } else {
                    jit_allocation.write_unaligned_value(pc, target);
                }
            }

            pub fn set_target_address_at(
                pc: usize,
                constant_pool: usize,
                target: usize,
                jit_allocation: &mut WritableJitAllocation,
                icache_flush_mode: ICacheFlushMode,
            ) {
                let op1 = Instruction::s390_opcode_value(pc as *const u8);
                let mut instr_1 = Instruction::instruction_bits(pc as *const u8);
                let mut patched = false;

                if op1 == Opcode::BRASL || op1 == Opcode::BRCL {
                    instr_1 >>= 32;
                    instr_1 <<= 32;
                    let halfwords = ((target as isize - pc as isize) / 2) as u32;
                    instr_1 |= halfwords as u64;
                    Instruction::set_instruction_bits(pc as *mut u8, instr_1, jit_allocation);
                    if icache_flush_mode != ICacheFlushMode::SkipICacheFlush {
                        FlushInstructionCache(pc, 6);
                    }
                    patched = true;
                } else {
                    let instr1_length = Instruction::instruction_length(pc as *const u8);
                    let op2 = Instruction::s390_opcode_value((pc as isize + instr1_length as isize) as *const u8);
                    let mut instr_2 = Instruction::instruction_bits((pc as isize + instr1_length as isize) as *const u8);

                    if op1 == Opcode::IIHF && op2 == Opcode::IILF {
                        instr_1 >>= 32;
                        instr_1 <<= 32;
                        instr_1 |= (target as u64) >> 32;

                        Instruction::set_instruction_bits(pc as *mut u8, instr_1, jit_allocation);

                        instr_2 >>= 32;
                        instr_2 <<= 32;
                        instr_2 |= (target as u64) & 0xFFFFFFFF;

                        Instruction::set_instruction_bits(
                            (pc as isize + instr1_length as isize) as *mut u8,
                            instr_2,
                            jit_allocation,
                        );

                        if icache_flush_mode != ICacheFlushMode::SkipICacheFlush {
                            FlushInstructionCache(pc, 12);
                        }
                        patched = true;
                    }
                }
                if !patched {
                    UNREACHABLE();
                }
            }

            pub fn target_compressed_address_at(pc: usize, constant_pool: usize) -> Tagged_t {
                Assembler::target_address_at(pc, constant_pool) as Tagged_t
            }

             pub fn code_target_object_handle_at(&self, pc: usize) -> Handle<Object> {
                let instr = Instruction::instruction_bits(pc as *const u8);
                let index = instr & 0xFFFFFFFF;
                self.get_code_target(index as usize) // TODO: Verify the cast
            }

            pub fn compressed_embedded_object_handle_at(&self, pc: usize, const_pool: usize) -> Handle<HeapObject> {
                self.get_embedded_object(Assembler::target_compressed_address_at(pc, const_pool))
            }

            pub fn set_target_compressed_address_at(
                pc: usize,
                constant_pool: usize,
                target: Tagged_t,
                jit_allocation: &mut WritableJitAllocation,
                icache_flush_mode: ICacheFlushMode,
            ) {
                Assembler::set_target_address_at(
                    pc,
                    constant_pool,
                    target as usize,
                    jit_allocation,
                    icache_flush_mode,
                );
            }

            pub fn uint32_constant_at(pc: usize, constant_pool: usize) -> u32 {
                Assembler::target_address_at(pc, constant_pool) as u32
            }

            pub fn set_uint32_constant_at(
                pc: usize,
                constant_pool: usize,
                new_constant: u32,
                jit_allocation: &mut WritableJitAllocation,
                icache_flush_mode: ICacheFlushMode,
            ) {
                Assembler::set_target_address_at(
                    pc,
                    constant_pool,
                    new_constant as usize,
                    jit_allocation,
                    icache_flush_mode,
                );
            }
        }
    }
}