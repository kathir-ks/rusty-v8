// Converted from V8 C++ source files:
// Header: assembler-loong64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod loong64_assembler_loong64_inl {
use crate::codegen::assembler::assembler::*;
use crate::codegen::flush_instruction_cache::*;
use crate::codegen::loong64::assembler_loong64::*;
use crate::debug::debug::*;
use crate::heap::heap_layout_inl::*;
use crate::heap::heap_layout::*;
use crate::objects::objects_inl::*;
use crate::codegen::loong64::macro_assembler_loong64::Address;
use crate::codegen::loong64::macro_assembler_loong64::Address;
use crate::codegen::loong64::macro_assembler_loong64::Tagged_t;
use crate::codegen::loong64::macro_assembler_loong64::JSDispatchHandle;
use crate::codegen::s390::assembler_s390::ICacheFlushMode;
use crate::codegen::ia32::assembler_ia32_inl::WasmCodePointer;
use crate::codegen::assembler_arch::EnsureSpace;

  pub struct V8 {}

  impl V8 {
  }

  impl CpuFeatures {
    pub fn supports_optimizer() -> bool {
       CpuFeatures::is_supported(FPU)
    }
  }

  impl Operand {
    pub fn is_reg(&self) -> bool {
      self.rm_.is_valid()
    }

    pub fn immediate(&self) -> i64 {
      assert!(!self.is_reg());
      assert!(!self.IsHeapNumberRequest());
      self.value_.immediate
    }
  }

  impl WritableRelocInfo {
    pub fn apply(&mut self, delta: isize) {
      if IsInternalReference(self.rmode_) {
        let internal_ref = unsafe {
          (self.pc_ as *const isize).read_unaligned()
        };
        let mut internal_ref = internal_ref.wrapping_add(delta);
        unsafe {
          (self.pc_ as *mut isize).write_unaligned(internal_ref);
        }
      } else {
        assert!(IsRelativeCodeTarget(self.rmode_) || IsNearBuiltinEntry(self.rmode_));
        Assembler::relocate_relative_reference(self.rmode_, self.pc_, delta, &mut self.jit_allocation_);
      }
    }
  }

  impl RelocInfo {
    pub fn target_address(&self) -> Address {
      assert!(IsCodeTargetMode(self.rmode_) || IsNearBuiltinEntry(self.rmode_) ||
              IsWasmCall(self.rmode_) || IsWasmStubCall(self.rmode_));
      Assembler::target_address_at(self.pc_, self.constant_pool_)
    }

    pub fn target_address_address(&self) -> Address {
      assert!(self.HasTargetAddressAddress());
      self.pc_.wrapping_add((Assembler::kInstructionsFor64BitConstant as usize).wrapping_mul(kInstrSize as usize))
    }

    pub fn constant_pool_entry_address(&self) -> Address {
      unreachable!()
    }

    pub fn target_address_size(&self) -> i32 {
       Assembler::kSpecialTargetSize
    }
  }

  impl Assembler {
    pub fn deserialization_special_target_size(instruction_payload: Address) -> i32 {
      kSpecialTargetSize
    }

    pub fn deserialization_set_target_internal_reference_at(
        pc: Address,
        target: Address,
        jit_allocation: &mut WritableJitAllocation,
        mode: RelocInfo::Mode,
    ) {
      jit_allocation.WriteUnalignedValue::<Address>(pc, target);
    }

    pub fn compressed_embedded_object_handle_at(
        pc: Address,
        constant_pool: Address,
    ) -> Handle<HeapObject> {
        Assembler::get_embedded_object(Assembler::target_compressed_address_at(pc, constant_pool))
    }

    pub fn embedded_object_handle_at(
        pc: Address,
        constant_pool: Address,
    ) -> Handle<HeapObject> {
      Assembler::get_embedded_object(Assembler::target_address_at(pc, constant_pool))
    }

    pub fn code_target_object_handle_at(
        pc: Address,
        constant_pool: Address,
    ) -> Handle<Code> {
      let index = Assembler::target_address_at(pc, constant_pool) as i32 & 0xFFFFFFFF;
      Assembler::get_code_target(index)
    }

    pub fn target_builtin_at(pc: Address) -> Builtin {
      let builtin_id = ((Assembler::target_address_at(pc) as usize).wrapping_sub(pc as usize) >> 2) as i32;
      assert!(Builtins::IsBuiltinId(builtin_id));
      unsafe { std::mem::transmute::<i32, Builtin>(builtin_id) }
    }
  }

  impl RelocInfo {
    pub fn target_object(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
      assert!(IsCodeTarget(self.rmode_) || IsFullEmbeddedObject(self.rmode_));
      if IsCompressedEmbeddedObject(self.rmode_) {
        let compressed = Assembler::target_compressed_address_at(self.pc_, self.constant_pool_);
        assert!(!HAS_SMI_TAG(compressed));
        let obj = V8HeapCompressionScheme::DecompressTagged(cage_base, compressed);
        unsafe {std::mem::transmute::<Tagged<Object>, Tagged<HeapObject>>(obj)}
      } else {
        let addr = Assembler::target_address_at(self.pc_, self.constant_pool_);
        unsafe {std::mem::transmute::<Tagged<Object>, Tagged<HeapObject>>(Tagged::<Object>(addr))}
      }
    }

    pub fn target_object_handle(&self, origin: &Assembler) -> DirectHandle<HeapObject> {
        if IsCodeTarget(self.rmode_) {
            origin.code_target_object_handle_at(self.pc_, self.constant_pool_)
        } else if IsFullEmbeddedObject(self.rmode_) {
            origin.embedded_object_handle_at(self.pc_, self.constant_pool_)
        } else if IsCompressedEmbeddedObject(self.rmode_) {
            origin.compressed_embedded_object_handle_at(self.pc_, self.constant_pool_)
        } else {
            assert!(IsRelativeCodeTarget(self.rmode_));
            origin.relative_code_target_object_handle_at(self.pc_)
        }
    }

    pub fn target_builtin_at_wrapper(&self, origin: &Assembler) -> Builtin {
        self.target_builtin_at(origin)
    }

    pub fn target_off_heap_target(&self) -> Address {
      assert!(IsOffHeapTarget(self.rmode_));
      Assembler::target_address_at(self.pc_, self.constant_pool_)
    }
  }

  impl WritableRelocInfo {
    pub fn set_target_object(
        &mut self,
        target: Tagged<HeapObject>,
        icache_flush_mode: ICacheFlushMode,
    ) {
      assert!(IsCodeTarget(self.rmode_) || IsEmbeddedObjectMode(self.rmode_));
      if IsCompressedEmbeddedObject(self.rmode_) {
        assert!(COMPRESS_POINTERS_BOOL);
        assert!(!V8_ENABLE_SANDBOX_BOOL || !HeapLayout::InTrustedSpace(target));
        assert!(!V8_EXTERNAL_CODE_SPACE_BOOL || !HeapLayout::InCodeSpace(target));
        Assembler::set_target_compressed_address_at(
            self.pc_,
            self.constant_pool_,
            V8HeapCompressionScheme::CompressObject(target.ptr()),
            &mut self.jit_allocation_,
            icache_flush_mode,
        );
      } else {
        Assembler::set_target_address_at(
            self.pc_,
            self.constant_pool_,
            target.ptr(),
            &mut self.jit_allocation_,
            icache_flush_mode,
        );
      }
    }

    pub fn set_target_external_reference(&mut self, target: Address, icache_flush_mode: ICacheFlushMode) {
        assert_eq!(self.rmode_, RelocInfo::EXTERNAL_REFERENCE);
        Assembler::set_target_address_at(self.pc_, self.constant_pool_, target, &mut self.jit_allocation_, icache_flush_mode);
    }

    pub fn set_wasm_code_pointer_table_entry(&mut self, target: WasmCodePointer, icache_flush_mode: ICacheFlushMode) {
        assert_eq!(self.rmode_, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
        Assembler::set_uint32_constant_at(self.pc_, self.constant_pool_, target.value(), &mut self.jit_allocation_, icache_flush_mode);
    }
  }

  impl RelocInfo {
      pub fn target_external_reference(&self) -> Address {
          assert_eq!(self.rmode_, RelocInfo::EXTERNAL_REFERENCE);
          Assembler::target_address_at(self.pc_, self.constant_pool_)
      }

      pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
          assert_eq!(self.rmode_, RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY);
          WasmCodePointer { value: Assembler::uint32_constant_at(self.pc_, self.constant_pool_) }
      }

      pub fn target_internal_reference(&self) -> Address {
          if self.rmode_ == RelocInfo::INTERNAL_REFERENCE {
              unsafe { (self.pc_ as *const Address).read() }
          } else {
              unreachable!()
          }
      }

      pub fn target_internal_reference_address(&self) -> Address {
          assert_eq!(self.rmode_, RelocInfo::INTERNAL_REFERENCE);
          self.pc_
      }

      pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
          assert_eq!(self.rmode_, RelocInfo::JS_DISPATCH_HANDLE);
          JSDispatchHandle(Assembler::uint32_constant_at(self.pc_, self.constant_pool_))
      }
  }

  impl Assembler {
    pub fn relative_code_target_object_handle_at(
        &self,
        pc: Address,
    ) -> Handle<Code> {
      let instr = self.instr_at(pc);
      let mut code_target_index = instr & kImm26Mask;
      code_target_index = ((code_target_index & 0x3ff) << 22 >> 6) | ((code_target_index >> 10) & kImm16Mask);
      Assembler::get_code_target(code_target_index)
    }
  }

  impl RelocInfo {
    pub fn target_builtin_at(origin: &Assembler) -> Builtin {
      assert!(IsNearBuiltinEntry(origin.rmode_));
      Assembler::target_builtin_at(origin.pc_)
    }
  }

  impl Assembler {
    pub fn uint32_constant_at(pc: Address, _constant_pool: Address) -> u32 {
      Assembler::target_compressed_address_at(pc) as u32
    }

    pub fn set_uint32_constant_at(
        pc: Address,
        _constant_pool: Address,
        new_constant: u32,
        jit_allocation: &mut WritableJitAllocation,
        icache_flush_mode: ICacheFlushMode,
    ) {
      Assembler::set_target_compressed_value_at(pc, new_constant as Address, jit_allocation, icache_flush_mode);
    }
  }

  impl Assembler {
    pub fn check_buffer(&mut self) {
      if self.buffer_space() <= kGap as usize {
        self.grow_buffer();
      }
    }

    pub fn emit_helper(&mut self, x: Instr) {
      unsafe {
        *(self.pc_ as *mut Instr) = x;
      }
      self.pc_ = self.pc_.wrapping_add(kInstrSize as usize);
      self.check_trampoline_pool_quick();
    }

    pub fn emit_helper_u8(&mut self, x: u8) {
      unsafe {
        *(self.pc_ as *mut u8) = x;
      }
      self.pc_ = self.pc_.wrapping_add(std::mem::size_of::<u8>());
      if (self.pc_ as usize) % (kInstrSize as usize) == 0 {
        self.check_trampoline_pool_quick();
      }
    }

    pub fn emit_helper_t<T: Sized>(&mut self, x: T) {
      unsafe {
        *(self.pc_ as *mut T) = x;
      }
      self.pc_ = self.pc_.wrapping_add(std::mem::size_of::<T>());
      self.check_trampoline_pool_quick();
    }

    pub fn emit(&mut self, x: Instr) {
      if !self.is_buffer_growth_blocked() {
        self.check_buffer();
      }
      self.emit_helper(x);
    }

    pub fn emit_u64(&mut self, data: u64) {
      if !self.is_buffer_growth_blocked() {
        self.check_buffer();
      }
      self.emit_helper_t(data);
    }
  }

  impl EnsureSpace {
    pub fn new(assembler: &mut Assembler) -> Self {
      assembler.check_buffer();
      EnsureSpace { }
    }
  }
}
