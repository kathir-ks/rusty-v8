// Converted from V8 C++ source files:
// Header: assembler-mips64.h
// Implementation: assembler-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::codegen::assembler::*;
use crate::codegen::code_stub_assembler::*;
use crate::codegen::mips64::constants_mips64::*;
use crate::codegen::mips64::register_mips64::*;
use crate::objects::contexts::*;
use crate::objects::smi::*;

pub const K_SMI_SHIFT: i32 = K_SMI_TAG_SIZE + K_SMI_SHIFT_SIZE;
pub const K_SMI_SHIFT_MASK: u64 = (1 << K_SMI_SHIFT) - 1;

#[derive(Debug, Copy, Clone)]
pub struct Operand {
    rm_: Register,
    value_: Value,
    is_heap_number_request_: bool,
    rmode_: RelocInfo::Mode,
}

impl Operand {
    #[inline]
    pub fn new_immediate(immediate: i64, rmode: RelocInfo::Mode) -> Self {
        Operand {
            rm_: Register::no_reg,
            value_: Value { immediate },
            is_heap_number_request_: false,
            rmode_: rmode,
        }
    }

    #[inline]
    pub fn new_external_reference(f: ExternalReference) -> Self {
        Operand {
            rm_: Register::no_reg,
            value_: Value { immediate: f.address() as i64 },
            is_heap_number_request_: false,
            rmode_: RelocInfo::EXTERNAL_REFERENCE,
        }
    }

    #[inline]
    pub fn new_smi(value: Tagged<Smi>) -> Self {
        Operand::new_immediate(value.ptr() as i64, RelocInfo::NO_INFO)
    }

    pub fn new_heap_object(handle: Handle<HeapObject>) -> Self {
        Operand {
            rm_: Register::no_reg,
            value_: Value { immediate: handle.address() as i64 },
            is_heap_number_request_: false,
            rmode_: RelocInfo::FULL_EMBEDDED_OBJECT,
        }
    }

    pub fn embedded_number(number: f64) -> Self {
        let mut smi: i32 = 0;
        if double_to_smi_integer(number, &mut smi) {
            return Operand::new_smi(Smi::from_int(smi));
        }
        let mut result = Operand::new_immediate(0, RelocInfo::FULL_EMBEDDED_OBJECT);
        result.is_heap_number_request_ = true;
        result.value_.heap_number_request = HeapNumberRequest { number: number };
        return result;
    }

    #[inline]
    pub fn new_register(rm: Register) -> Self {
        Operand {
            rm_: rm,
            value_: Value { immediate: 0 },
            is_heap_number_request_: false,
            rmode_: RelocInfo::NO_INFO,
        }
    }

    #[inline]
    pub fn is_reg(&self) -> bool {
        self.rm_ != Register::no_reg
    }

    #[inline]
    pub fn immediate(&self) -> i64 {
        self.value_.immediate
    }

    pub fn is_immediate(&self) -> bool {
        self.rm_ == Register::no_reg
    }

    pub fn heap_number_request(&self) -> HeapNumberRequest {
        debug_assert!(self.is_heap_number_request());
        self.value_.heap_number_request
    }

    pub fn is_heap_number_request(&self) -> bool {
        debug_assert!(!(self.is_heap_number_request_ && !self.is_immediate()));
        debug_assert!(!(self.is_heap_number_request_ &&
                         (self.rmode_ != RelocInfo::FULL_EMBEDDED_OBJECT &&
                              self.rmode_ != RelocInfo::CODE_TARGET)));
        self.is_heap_number_request_
    }

    pub fn rm(&self) -> Register {
        self.rm_
    }

    pub fn rmode(&self) -> RelocInfo::Mode {
        self.rmode_
    }
}

#[derive(Debug, Copy, Clone)]
union Value {
    heap_number_request: HeapNumberRequest,
    immediate: i64,
}

impl Value {
    const fn new() -> Self {
        Value {
            immediate: 0, // Default value
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemOperand {
    operand: Operand,
    offset_: i32,
}

impl MemOperand {
    pub fn new(rn: Register, offset: i32) -> Self {
        MemOperand {
            operand: Operand::new_register(rn),
            offset_: offset,
        }
    }

    pub fn with_scale(rn: Register, unit: i32, multiplier: i32, offset_addend: OffsetAddend) -> Self {
        MemOperand {
            operand: Operand::new_register(rn),
            offset_: unit * multiplier + offset_addend as i32,
        }
    }

    pub fn offset(&self) -> i32 {
        self.offset_
    }

    pub fn offset_is_int16_encodable(&self) -> bool {
        is_int16(self.offset_)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OffsetAddend {
    offset_minus_one = -1,
    offset_zero = 0,
}

#[derive(Debug)]
pub struct Assembler {
    assembler_base: AssemblerBase,
    scratch_register_list_: RegList,
    next_buffer_check_: i32,
    trampoline_pool_blocked_nesting_: i32,
    no_trampoline_pool_before_: i32,
    last_trampoline_pool_end_: i32,
    block_buffer_growth_: bool,
    reloc_info_writer: RelocInfoWriter,
    last_bound_pos_: i32,
    prev_instr_compact_branch_: bool,
    trampoline_: Trampoline,
    internal_trampoline_exception_: bool,
    unbound_labels_count_: i32,
    internal_reference_positions_: std::collections::HashSet<i64>,
    pc_for_safepoint_: *mut u8,
}

impl Assembler {
    pub fn new(options: &AssemblerOptions, buffer: Option<std::unique_ptr::UniquePtr<AssemblerBuffer>>) -> Self {
      let mut assembler = Self {
        assembler_base: AssemblerBase::new(options.clone(), buffer.map(|b| unsafe { *b })),
        scratch_register_list_: RegList::new(&[at, s0]),
        next_buffer_check_: if V8_FLAGS.force_long_branches {
            i32::MAX
        } else {
          K_MAX_BRANCH_OFFSET - Self::KTRAMPOLINE_SLOTS_SIZE * 16
        },
        trampoline_pool_blocked_nesting_: 0,
        no_trampoline_pool_before_: 0,
        last_trampoline_pool_end_: 0,
        block_buffer_growth_: false,
        reloc_info_writer: RelocInfoWriter::new(),
        last_bound_pos_: 0,
        prev_instr_compact_branch_: false,
        trampoline_: Trampoline::new(),
        internal_trampoline_exception_: false,
        unbound_labels_count_: 0,
        internal_reference_positions_: std::collections::HashSet::new(),
        pc_for_safepoint_: std::ptr::null_mut(),
      };

      if CpuFeatures::is_supported(CpuFeature::MipsSimd) {
        assembler.assembler_base.enable_cpu_feature(CpuFeature::MipsSimd);
      }

      assembler.reloc_info_writer.reposition(
          assembler.assembler_base.buffer_start().add(assembler.assembler_base.buffer_size()),
          assembler.assembler_base.pc());

      assembler
    }
    fn GetCode(
        &mut self,
        isolate: *mut Isolate,
        desc: &mut CodeDesc,
        safepoint_table_builder: *mut SafepointTableBuilderBase,
        handler_table_offset: i32,
    ) {}
    fn MaybeEmitOutOfLineConstantPool(&mut self) {}
    fn RecordDeoptReason(&mut self, arg0: DeoptimizeReason, arg1: u32, arg2: SourcePosition, arg3: i32){}
    fn Align(&mut self, m:i32){}
    fn DataAlign(&mut self, m:i32){}
    fn CodeTargetAlign(&mut self){}
}
