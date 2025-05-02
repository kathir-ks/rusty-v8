pub mod loong64 {
    pub mod assembler_loong64_inl {
        use crate::codegen::assembler::Assembler;
        use crate::codegen::flush_instruction_cache::ICacheFlushMode;
        use crate::codegen::loong64::assembler_loong64::*;
        use crate::debug::debug::DebugDelegate;
        use crate::heap::heap_layout_inl::*;
        use crate::heap::heap_layout::HeapLayout;
        use crate::objects::objects::*;
        use crate::objects::objects_inl::*;
        use crate::base::*;
        use std::mem;
        use std::ptr;
        use crate::codegen::reloc_info::*;
        use crate::codegen::code_factory::*;
        use crate::handles::*;
        use crate::wasm::wasm_code_manager::*;
        use crate::common::address::*;

        pub fn supports_optimizer() -> bool {
            is_supported(CpuFeature::FPU)
        }

        impl Operand {
            pub fn is_reg(&self) -> bool {
                self.rm.is_valid()
            }

            pub fn immediate(&self) -> i64 {
                debug_assert!(!self.is_reg());
                debug_assert!(!self.is_heap_number_request());
                self.value.immediate
            }
        }

        impl WritableRelocInfo {
            pub fn apply(&mut self, delta: isize) {
                if self.rmode == RelocInfoMode::INTERNAL_REFERENCE {
                    let internal_ref = unsafe {
                        ptr::read_unaligned(self.pc as *const Address)
                    };
                    let internal_ref = internal_ref.wrapping_add(delta as usize);

                    unsafe {
                        self.jit_allocation.write_unaligned_value(self.pc, internal_ref);
                    }
                } else {
                    debug_assert!(self.rmode == RelocInfoMode::RELATIVE_CODE_TARGET || self.rmode == RelocInfoMode::NEAR_BUILTIN_ENTRY);
                    Assembler::relocate_relative_reference(self.rmode, self.pc, delta, &mut self.jit_allocation);
                }
            }
        }

        impl RelocInfo {
            pub fn target_address(&self) -> Address {
                debug_assert!(is_code_target_mode(self.rmode) || self.rmode == RelocInfoMode::NEAR_BUILTIN_ENTRY ||
                              self.rmode == RelocInfoMode::WASM_CALL || self.rmode == RelocInfoMode::WASM_STUB_CALL);
                Assembler::target_address_at(self.pc, self.constant_pool)
            }

            pub fn target_address_address(&self) -> Address {
                debug_assert!(self.has_target_address_address());
                self.pc.wrapping_add(Assembler::k_instructions_for_64_bit_constant as usize * k_instr_size as usize)
            }

            pub fn constant_pool_entry_address(&self) -> Address {
                unreachable!()
            }

            pub fn target_address_size(&self) -> i32 {
                Assembler::k_special_target_size
            }
        }

        impl Assembler {
            pub fn deserialization_special_target_size(_instruction_payload: Address) -> i32 {
                k_special_target_size
            }

            pub fn deserialization_set_target_internal_reference_at(pc: Address, target: Address, jit_allocation: &mut WritableJitAllocation, _mode: RelocInfoMode) {
                unsafe {
                    jit_allocation.write_unaligned_value(pc, target);
                }
            }

            pub fn compressed_embedded_object_handle_at(&self, pc: Address, constant_pool: Address) -> Handle<HeapObject> {
                get_embedded_object(self.target_compressed_address_at(pc, constant_pool))
            }

            pub fn embedded_object_handle_at(&self, pc: Address, constant_pool: Address) -> Handle<HeapObject> {
                get_embedded_object(self.target_address_at(pc, constant_pool))
            }

            pub fn code_target_object_handle_at(&self, pc: Address, constant_pool: Address) -> Handle<Code> {
                let index = self.target_address_at(pc, constant_pool) as u32 as i32;
                get_code_target(index)
            }
        }

        pub fn target_builtin_at(pc: Address) -> Builtin {
            let builtin_id = ((Assembler::target_address_at(pc, 0) as isize - pc as isize) >> 2) as i32;
            debug_assert!(Builtins::is_builtin_id(builtin_id));
            unsafe { mem::transmute(builtin_id) }
        }

        impl RelocInfo {
            pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
                debug_assert!(is_code_target_mode(self.rmode) || self.is_full_embedded_object(self.rmode));
                if self.is_compressed_embedded_object(self.rmode) {
                    debug_assert!(COMPRESS_POINTERS_BOOL);
                    let compressed = Assembler::target_compressed_address_at(self.pc, self.constant_pool);
                    debug_assert!(!has_smi_tag(compressed));
                    let obj = V8HeapCompressionScheme::decompress_tagged(cage_base, compressed);
                    Tagged::<Object>::unchecked_cast(obj)
                } else {
                    Tagged::<Object>::unchecked_cast(Tagged::<Object>::from_ptr(Assembler::target_address_at(self.pc, self.constant_pool)))
                }
            }

            pub fn target_object_handle(&self, origin: &Assembler) -> DirectHandle<HeapObject> {
                if is_code_target_mode(self.rmode) {
                    origin.code_target_object_handle_at(self.pc, self.constant_pool).into()
                } else if self.is_full_embedded_object(self.rmode) {
                    origin.embedded_object_handle_at(self.pc, self.constant_pool).into()
                } else if self.is_compressed_embedded_object(self.rmode) {
                    origin.compressed_embedded_object_handle_at(self.pc, self.constant_pool).into()
                } else {
                    debug_assert!(self.is_relative_code_target(self.rmode));
                    origin.relative_code_target_object_handle_at(self.pc).into()
                }
            }
        }

        impl WritableRelocInfo {
            pub fn set_target_object(&mut self, target: Tagged<HeapObject>, icache_flush_mode: ICacheFlushMode) {
                debug_assert!(is_code_target_mode(self.rmode) || self.is_embedded_object_mode(self.rmode));
                if self.is_compressed_embedded_object(self.rmode) {
                    debug_assert!(COMPRESS_POINTERS_BOOL);
                    debug_assert!(!HeapLayout::in_trusted_space(target.into()));
                    debug_assert!(!HeapLayout::in_code_space(target.into()));
                    Assembler::set_target_compressed_address_at(
                        self.pc,
                        self.constant_pool,
                        V8HeapCompressionScheme::compress_object(target.ptr()),
                        &mut self.jit_allocation,
                        icache_flush_mode,
                    );
                } else {
                    Assembler::set_target_address_at(self.pc, self.constant_pool, target.ptr(), &mut self.jit_allocation, icache_flush_mode);
                }
            }
        }

        impl RelocInfo {
            pub fn target_external_reference(&self) -> Address {
                debug_assert!(self.rmode == RelocInfoMode::EXTERNAL_REFERENCE);
                Assembler::target_address_at(self.pc, self.constant_pool)
            }
        }

        impl WritableRelocInfo {
            pub fn set_target_external_reference(&mut self, target: Address, icache_flush_mode: ICacheFlushMode) {
                debug_assert!(self.rmode == RelocInfoMode::EXTERNAL_REFERENCE);
                Assembler::set_target_address_at(self.pc, self.constant_pool, target, &mut self.jit_allocation, icache_flush_mode);
            }
        }

        impl RelocInfo {
            pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
                debug_assert!(self.rmode == RelocInfoMode::WASM_CODE_POINTER_TABLE_ENTRY);
                WasmCodePointer { value: Assembler::uint32_constant_at(self.pc, self.constant_pool) }
            }
        }

        impl WritableRelocInfo {
            pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
                debug_assert!(self.rmode == RelocInfoMode::WASM_CODE_POINTER_TABLE_ENTRY);
                Assembler::set_uint32_constant_at(self.pc, self.constant_pool, target.value, &mut self.jit_allocation, icache_flush_mode);
            }
        }

        impl RelocInfo {
            pub fn target_internal_reference(&self) -> Address {
                if self.rmode == RelocInfoMode::INTERNAL_REFERENCE {
                    unsafe { *self.pc as Address } // TODO: check this conversion
                } else {
                    unreachable!()
                }
            }

            pub fn target_internal_reference_address(&self) -> Address {
                debug_assert!(self.rmode == RelocInfoMode::INTERNAL_REFERENCE);
                self.pc
            }
        }

        impl RelocInfo {
            pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
                debug_assert!(self.rmode == RelocInfoMode::JS_DISPATCH_HANDLE);
                JSDispatchHandle(Assembler::uint32_constant_at(self.pc, self.constant_pool))
            }
        }

        impl Assembler {
            pub fn relative_code_target_object_handle_at(&self, pc: Address) -> Handle<Code> {
                let instr = self.instr_at(pc);
                let mut code_target_index = instr & k_imm26_mask;
                code_target_index = ((code_target_index & 0x3ff) << 22 >> 6) | ((code_target_index >> 10) & k_imm16_mask);
                get_code_target(code_target_index as i32)
            }
        }

        impl RelocInfo {
            pub fn target_builtin_at(&self, origin: &Assembler) -> Builtin {
                debug_assert!(self.is_near_builtin_entry(self.rmode));
                target_builtin_at(self.pc)
            }
        }

        impl RelocInfo {
            pub fn target_off_heap_target(&self) -> Address {
                debug_assert!(self.is_off_heap_target(self.rmode));
                Assembler::target_address_at(self.pc, self.constant_pool)
            }
        }

        impl Assembler {
            pub fn uint32_constant_at(pc: Address, _constant_pool: Address) -> u32 {
                Assembler::target_compressed_address_at(pc) as u32
            }

            pub fn set_uint32_constant_at(pc: Address, _constant_pool: Address, new_constant: u32, jit_allocation: &mut WritableJitAllocation, icache_flush_mode: ICacheFlushMode) {
                Assembler::set_target_compressed_value_at(pc, new_constant as usize, jit_allocation, icache_flush_mode);
            }

            pub fn check_buffer(&mut self) {
                if self.buffer_space() <= k_gap as usize {
                    self.grow_buffer();
                }
            }

            pub fn emit_helper<T>(&mut self, x: T) {
                let size = mem::size_of::<T>();
                unsafe {
                    ptr::write_unaligned(self.pc as *mut T, x);
                }
                self.pc = self.pc.wrapping_add(size);
                self.check_trampoline_pool_quick();
            }

            pub fn emit(&mut self, x: Instr) {
                if !self.is_buffer_growth_blocked() {
                    self.check_buffer();
                }
                self.emit_helper(x);
            }

            pub fn emit_data(&mut self, data: u64) {
                if !self.is_buffer_growth_blocked() {
                    self.check_buffer();
                }
                self.emit_helper(data);
            }
        }

        pub struct EnsureSpace<'a> {
            assembler: &'a mut Assembler,
        }

        impl<'a> EnsureSpace<'a> {
            pub fn new(assembler: &'a mut Assembler) -> Self {
                assembler.check_buffer();
                EnsureSpace { assembler }
            }
        }
    }
}