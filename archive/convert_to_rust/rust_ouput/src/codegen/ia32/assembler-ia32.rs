// Converted from V8 C++ source files:
// Header: assembler-ia32.h
// Implementation: assembler-ia32.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::rc::Rc;

use crate::codegen::assembler::{AssemblerBase, AssemblerOptions};
use crate::codegen::reloc_info::RelocInfo;
use crate::codegen::reloc_info::WritableRelocIterator;
use crate::codegen::reloc_info::ICacheFlushMode;
use crate::codegen::reloc_info::FLUSH_ICACHE_IF_NEEDED;
use crate::codegen::reloc_info::HeapNumberRequest;
use crate::execution::isolate::Isolate;
use crate::init::v8::V8;
use crate::objects::smi::Smi;
use crate::codegen::label::Label;
use crate::codegen::x64::assembler_x64::CodeDesc;
use crate::local_isolate::LocalIsolate;
use crate::codegen::x64::macro_assembler_x64::ExternalReference;
use crate::codegen::x64::register_x64::Register;
use crate::codegen::x64::register_x64::XMMRegister;
use crate::codegen::x64::assembler_x64::AssemblerBuffer;
use crate::objects::heap_object::HeapObject;
use crate::codegen::reloc_info::WritableJitAllocation;
use crate::base::bits::is_int8;
use crate::base::bits::is_uint8;
use crate::base::bits::is_int16;
use crate::base::bits::is_uint16;
use crate::codegen::code_stub_assembler::InstructionStream;
use crate::codegen::interface_descriptors_x64_inl::CallInterfaceDescriptor;
use crate::deoptimizer::deoptimizer::DeoptimizeReason;
use crate::codegen::assembler_inl::ShouldRecordRelocInfo;
use crate::compiler::backend::safepoint_table_builder::SafepointTableBuilder;
use crate::codegen::reloc_info::ModeMask;
use crate::codegen::thread_isolation::ThreadIsolation;
use crate::codegen::compilation_dependencies::OptionalOpIndex;
use crate::compiler::turboshaft;
use crate::codegen::code_comments::ShouldEmitComments;
use crate::codegen::code_comments::CodeComments;
use crate::codegen::arm64::assembler_arm64::Shift;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use crate::builtins::builtins::Builtins;
use crate::codegen::compilation_dependencies::Name;
use crate::codegen::code_stub_assembler::RegisterRepresentation;
use crate::codegen::zone::Zone;
use crate::codegen::arm64::instructions_arm64::Instr;
use crate::codegen::arm64::register_arm64::CPURegister;
use crate::codegen::interface_descriptors_x64_inl::V8;
use crate::wasm::wasm_features::string;
use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo;
use std::collections::HashMap;
use crate::codegen::arm64::instructions_arm64::Mode;
use crate::handles::handles::Local;
use crate::execution::vm_state::OpIndex;
use crate::objects::field::Field;
use crate::codegen::arm64::assembler_arm64::AssemblerBufferTrait;
use crate::codegen::arm64::instructions_arm64::RegSize;
use crate::compiler::backend::move_optimizer::Instruction;
use std::borrow::Cow;
use crate::codegen::reloc_info::RelocInfoWriter;
use std::sync::Mutex;
use crate::compiler::backend::linearscan::instruction_sequence::InstructionSequence;
use crate::handles::handles::Handle;
use crate::compiler::backend::move_optimizer::MoveOptimizer;
use crate::compiler::turboshaft::block::Block;
use crate::compiler::backend::linearscan::register_allocator_verifier::V;
use crate::compiler::turboshaft::select_lowering_reducer::Implementation;
use crate::compiler::backend::jump_threading::RpoNumber;
use crate::codegen::interface_descriptors_x64_inl::CallInterfaceDescriptorData;
use crate::codegen::x64::assembler_x64::AssemblerBuffer;
use crate::compiler::compilation_dependencies::DependenceInfo;
use crate::compiler::turboshaft::opcodes::Opcode;
use crate::codegen::machine_type::MachineType;
use crate::codegen::arm64::assembler_arm64::Assembler::Options;
use crate::execution::vm_state::Scope;
use std::fmt;
use std::marker::PhantomData;
use crate::codegen::register_configuration::RegisterConfiguration;
use crate::codegen::macro_assembler::AbortReason;

pub enum Condition {
  overflow = 0,
  no_overflow = 1,
  below = 2,
  above_equal = 3,
  equal = 4,
  not_equal = 5,
  below_equal = 6,
  above = 7,
  negative = 8,
  positive = 9,
  parity_even = 10,
  parity_odd = 11,
  less = 12,
  greater_equal = 13,
  less_equal = 14,
  greater = 15,
  carry = below,
  not_carry = above_equal,
  zero = equal,
  not_zero = not_equal,
  sign = negative,
  not_sign = positive,
  kEqual = equal,
  kNotEqual = not_equal,
  kLessThan = less,
  kGreaterThan = greater,
  kLessThanEqual = less_equal,
  kGreaterThanEqual = greater_equal,
  kUnsignedLessThan = below,
  kUnsignedGreaterThan = above,
  kUnsignedLessThanEqual = below_equal,
  kUnsignedGreaterThanEqual = above_equal,
  kOverflow = overflow,
  kNoOverflow = no_overflow,
  kZero = equal,
  kNotZero = not_equal,
}

impl Condition {
  pub fn negate(&self) -> Self {
    match self {
      Condition::overflow => Condition::no_overflow,
      Condition::no_overflow => Condition::overflow,
      Condition::below => Condition::above_equal,
      Condition::above_equal => Condition::below,
      Condition::equal => Condition::not_equal,
      Condition::not_equal => Condition::equal,
      Condition::below_equal => Condition::above,
      Condition::above => Condition::below_equal,
      Condition::negative => Condition::positive,
      Condition::positive => Condition::negative,
      Condition::parity_even => Condition::parity_odd,
      Condition::parity_odd => Condition::parity_even,
      Condition::less => Condition::greater_equal,
      Condition::greater_equal => Condition::less,
      Condition::less_equal => Condition::greater,
      Condition::greater => Condition::less_equal,
      Condition::carry => Condition::not_carry,
      Condition::not_carry => Condition::carry,
      Condition::zero => Condition::not_zero,
      Condition::not_zero => Condition::zero,
      Condition::sign => Condition::not_sign,
      Condition::not_sign => Condition::sign,
      Condition::kEqual => Condition::kNotEqual,
      Condition::kNotEqual => Condition::kEqual,
      Condition::kLessThan => Condition::kGreaterThanEqual,
      Condition::kGreaterThan => Condition::kLessThanEqual,
      Condition::kLessThanEqual => Condition::kGreaterThan,
      Condition::kGreaterThanEqual => Condition::kLessThan,
      Condition::kUnsignedLessThan => Condition::kUnsignedGreaterThanEqual,
      Condition::kUnsignedGreaterThan => Condition::kUnsignedLessThanEqual,
      Condition::kUnsignedLessThanEqual => Condition::kUnsignedGreaterThan,
      Condition::kUnsignedGreaterThanEqual => Condition::kUnsignedLessThan,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum RoundingMode {
  kRoundToNearest = 0x0,
  kRoundDown = 0x1,
  kRoundUp = 0x2,
  kRoundToZero = 0x3,
}

#[derive(Copy, Clone)]
pub struct Immediate {
  value_: Value,
  is_heap_number_request_: bool,
  rmode_: RelocInfo::Mode,
}

impl Immediate {
  pub fn new_i32(x: i32, rmode: RelocInfo::Mode) -> Self {
      Immediate {
          value_: Value { immediate: x },
          is_heap_number_request_: false,
          rmode_: rmode,
      }
  }

  pub fn new_external_reference(ext: &ExternalReference) -> Self {
      Immediate {
          value_: Value { immediate: ext.raw() as i32 },
          is_heap_number_request_: false,
          rmode_: RelocInfo::EXTERNAL_REFERENCE,
      }
  }

  pub fn new_heap_object(handle: &Handle<HeapObject>) -> Self {
      Immediate {
          value_: Value { immediate: handle.address() as i32 },
          is_heap_number_request_: false,
          rmode_: RelocInfo::FULL_EMBEDDED_OBJECT,
      }
  }

  pub fn new_smi(value: Smi) -> Self {
      Immediate {
          value_: Value { immediate: value.ptr() as i32 },
          is_heap_number_request_: false,
          rmode_: RelocInfo::NO_INFO,
      }
  }

  pub fn embedded_number(number: f64) -> Self {
      let mut result = Immediate {
          value_: Value { immediate: 0 },
          is_heap_number_request_: true,
          rmode_: RelocInfo::FULL_EMBEDDED_OBJECT,
      };
      result.value_.heap_number_request = HeapNumberRequest { number: number };
      result
  }

  pub fn code_relative_offset(label: &mut Label) -> Self {
      Immediate {
          value_: Value { immediate: label as *mut Label as i32 },
          is_heap_number_request_: false,
          rmode_: RelocInfo::INTERNAL_REFERENCE,
      }
  }

  pub fn is_heap_number_request(&self) -> bool {
      if self.is_heap_number_request_ {
          assert!(self.rmode_ == RelocInfo::FULL_EMBEDDED_OBJECT || self.rmode_ == RelocInfo::CODE_TARGET);
      }
      self.is_heap_number_request_
  }

  pub fn heap_number_request(&self) -> HeapNumberRequest {
      assert!(self.is_heap_number_request());
      self.value_.heap_number_request
  }

  pub fn immediate(&self) -> i32 {
      assert!(!self.is_heap_number_request());
      self.value_.immediate
  }

  pub fn is_embedded_object(&self) -> bool {
      !self.is_heap_number_request() && self.rmode() == RelocInfo::FULL_EMBEDDED_OBJECT
  }

  pub fn embedded_object(&self) -> Handle<HeapObject> {
      Handle::from_address(self.immediate() as usize as *mut HeapObject)
  }

  pub fn is_external_reference(&self) -> bool {
      self.rmode() == RelocInfo::EXTERNAL_REFERENCE
  }

  pub fn external_reference(&self) -> ExternalReference {
      assert!(self.is_external_reference());
      unsafe { std::mem::transmute(self.immediate()) }
  }

  pub fn is_zero(&self) -> bool {
      RelocInfo::IsNoInfo(self.rmode_) && self.immediate() == 0
  }

  pub fn is_int8(&self) -> bool {
      RelocInfo::IsNoInfo(self.rmode_) && is_int8(self.immediate())
  }

  pub fn is_uint8(&self) -> bool {
      RelocInfo::IsNoInfo(self.rmode_) && is_uint8(self.immediate())
  }

  pub fn is_int16(&self) -> bool {
      RelocInfo::IsNoInfo(self.rmode_) && is_int16(self.immediate())
  }

  pub fn is_uint16(&self) -> bool {
      RelocInfo::IsNoInfo(self.rmode_) && is_uint16(self.immediate())
  }

  pub fn rmode(&self) -> RelocInfo::Mode {
      self.rmode_
  }
}

union Value {
  heap_number_request: HeapNumberRequest,
  immediate: i32,
}

impl Copy for Value {}
impl Clone for Value {}

#[derive(Copy, Clone, Debug)]
pub enum ScaleFactor {
  times_1 = 0,
  times_2 = 1,
  times_4 = 2,
  times_8 = 3,
  times_int_size = times_4,
  times_half_system_pointer_size = times_2,
  times_system_pointer_size = times_4,
  times_tagged_size = times_4,
}

#[derive(Copy, Clone)]
pub struct Operand {
  buf_: [u8; 6],
  len_: u8,
  rmode_: RelocInfo::Mode,
}

impl Operand {
  pub fn new_register(reg: Register) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    operand.set_modrm(3, reg);
    operand
  }

  pub fn new_xmm_register(xmm_reg: XMMRegister) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    let reg = Register::from_code(xmm_reg.code());
    operand.set_modrm(3, reg);
    operand
  }

  pub fn new_disp_r(disp: i32, rmode: RelocInfo::Mode) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    operand.set_modrm(0, ebp);
    operand.set_dispr(disp, rmode);
    operand
  }

  pub fn new_immediate(imm: Immediate) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    operand.set_modrm(0, ebp);
    operand.set_dispr(imm.immediate(), imm.rmode_);
    operand
  }

  pub fn new_base_disp_r(base: Register, disp: i32, rmode: RelocInfo::Mode) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    if disp == 0 && RelocInfo::IsNoInfo(rmode) && base != ebp {
      operand.set_modrm(0, base);
      if base == esp {
        operand.set_sib(ScaleFactor::times_1, esp, base);
      }
    } else if is_int8(disp) && RelocInfo::IsNoInfo(rmode) {
      operand.set_modrm(1, base);
      if base == esp {
        operand.set_sib(ScaleFactor::times_1, esp, base);
      }
      operand.set_disp8(disp as i8);
    } else {
      operand.set_modrm(2, base);
      if base == esp {
        operand.set_sib(ScaleFactor::times_1, esp, base);
      }
      operand.set_dispr(disp, rmode);
    }
    operand
  }

  pub fn new_label(label: &mut Label) -> Self {
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };
    operand.set_modrm(0, ebp);
    operand.set_dispr(label as *mut Label as i32, RelocInfo::INTERNAL_REFERENCE);
    operand
  }

  pub fn new_base_index_scale_disp_r(base: Register, index: Register, scale: ScaleFactor, disp: i32, rmode: RelocInfo::Mode) -> Self {
    assert!(index != esp);
    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };

    if disp == 0 && RelocInfo::IsNoInfo(rmode) && base != ebp {
      operand.set_modrm(0, esp);
      operand.set_sib(scale, index, base);
    } else if is_int8(disp) && RelocInfo::IsNoInfo(rmode) {
      operand.set_modrm(1, esp);
      operand.set_sib(scale, index, base);
      operand.set_disp8(disp as i8);
    } else {
      operand.set_modrm(2, esp);
      operand.set_sib(scale, index, base);
      operand.set_dispr(disp, rmode);
    }
    operand
  }

  pub fn new_index_scale_disp_r(index: Register, scale: ScaleFactor, disp: i32, rmode: RelocInfo::Mode) -> Self {
    assert!(index != esp);

    let mut operand = Self {
      buf_: [0; 6],
      len_: 0,
      rmode_: RelocInfo::NO_INFO,
    };

    operand.set_modrm(0, esp);
    operand.set_sib(scale, index, ebp);
    operand.set_dispr(disp, rmode);
    operand
  }

  pub fn jump_table(index: Register, scale: ScaleFactor, table: &mut Label) -> Self {
    Self::new_index_scale_disp_r(index, scale, table as *mut Label as i32, RelocInfo::INTERNAL_REFERENCE)
  }

  pub fn for_register_plus_immediate(base: Register, imm: Immediate) -> Self {
    Self::new_base_disp_r(base, imm.value_.immediate, imm.rmode_)
  }

  pub fn is_reg(&self, reg: Register) -> bool {
    self.is_reg_code(reg.code())
  }

    pub fn is_xmm_reg(&self, reg: XMMRegister) -> bool {
      self.is_reg_code(reg.code())
    }

  fn is_reg_code(&self, reg_code: i32) -> bool {
    (self.buf_[0] & 0xF8) == 0xC0 && (self.buf_[0] & 0x07) == reg_code
  }

  pub fn is_reg_only(&self) -> bool {
    (self.buf_[0] & 0xF8) == 0xC0
  }

  pub fn reg(&self) -> Register {
    assert!(self.is_reg_only());
    Register::from_code(self.buf_[0] & 0x07)
  }

  pub fn encoded_bytes(&self) -> Cow<[u8]> {
    Cow::Borrowed(&self.buf_[..self.len_ as usize])
  }

  pub fn rmode(&self) -> RelocInfo::Mode {
    self.rmode_
  }

  fn set_modrm(&mut self, mod_: i32, rm: Register) {
    assert_eq!(mod_ & -4, 0);
    self.buf_[0] = (mod_ << 6 | rm.code()) as u8;
    self.len_ = 1;
  }

  fn set_sib(&mut self, scale: ScaleFactor, index: Register, base: Register) {
    self.buf_[1] = ((scale as i32) << 6 | index.code() << 3 | base.code()) as u8;
    self.len_ = 2;
  }

  fn set_disp8(&mut self, disp: i8) {
    self.buf_[1] = disp as u8;
    self.len_ = 2;
  }

  fn set_dispr(&mut self, disp: i32, rmode: RelocInfo::Mode) {
    assert!(self.len_ == 1 || self.len_ == 2);
    let p = &mut self.buf_[self.len_ as usize] as *mut u8 as *mut i32;
    unsafe { *p = disp };
    self.len_ += 4;
    self.rmode_ = rmode;
  }
}

impl fmt::Debug for Operand {
  fn fmt(&self, f: &mut fmt::Formatter<
