// Converted from V8 C++ source files:
// Header: assembler-ppc.h
// Implementation: assembler-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod assembler_ppc {
use std::mem;
use std::rc::Rc;

use crate::base::numbers::double::DoubleToSmiInteger;
use crate::codegen::assembler::{AssemblerBase, AssemblerOptions, ICacheFlushMode,
                                 AssemblerBuffer, CPURegister, WritableJitAllocation,
                                 kMinimalBufferSize};
use crate::codegen::constant_pool::{ConstantPoolBuilder, ConstantPoolEntry,
                                     HeapNumberRequest};
use crate::codegen::external_reference::ExternalReference;
use crate::codegen::label::Label;
use crate::codegen::ppc::constants_ppc::*;
use crate::codegen::ppc::register_ppc::*;
use crate::objects::smi::Smi;
use crate::objects::Tagged;
use crate::codegen::reloc_info::{RelocInfo, RelocInfoWriter};
use crate::codegen::code_desc::CodeDesc;
use crate::codegen::safepoint_table_builder::SafepointTableBuilderBase;
use crate::local_isolate::LocalIsolate;
use crate::isolate::Isolate;
use crate::codegen::code_comments::StdoutStream;
use crate::strings::uri::Memory;
use crate::execution::simulator::Simulator;
use std::sync::Arc;
use crate::tasks::cancelable_task::Cancelable;
use crate::sandbox::sandbox::SandboxedPointerConstants;
use crate::codegen::code_reference::instruction;
use crate::codegen::interface_descriptors::CallInterfaceDescriptor;
use crate::codegen::pending_optimization_table::BytecodeArrayWrapper;
use crate::compiler::backend::arm::unwinding_info_writer_arm::Offset;
use crate::compiler::js_type_hint_lowering::Node;
use crate::handles::handles::DirectHandle;
use crate::heap::safepoint::Safepoint;
use crate::compiler::turboshaft::growable_stacks_reducer::Load;
use crate::compiler::js_inlining::SourcePosition;
use std::io::Write;
use crate::ast::ast_value_factory::AstRawString;
use crate::compiler::backend::move_optimizer::InstructionOperand;
use crate::compiler::backend::mips64::code_generator_mips64::Ignore;
use crate::compiler::node_matchers::VariableMode;
use crate::codegen::code_reference::Address;
use crate::codegen::code_reference::code_comments_size;
use crate::ast::scopes::CaseClause;
use crate::base::cpu::CPU;
use crate::wasm::wasm_error::WasmError;
use crate::compiler::compilation_cache::UnoptimizedCompileFlags;
use crate::codegen::code_stub_assembler::TVariable;
use crate::handles::handles::HeapObject;
use crate::init::bootstrapper::Root;
use crate::codegen::code_stub_assembler::Builtin;
use std::convert::Into;
use crate::compiler_dispatcher::lazy_compile_dispatcher::Operation;
use crate::compiler_dispatcher::lazy_compile_dispatcher::Block;
use crate::base::strings::SplitAt;
use crate::codegen::loong64::constants_loong64::*;
use crate::codegen::signature::T;
use crate::handles::handles::IsolateFieldId;
use crate::codegen::code_comments::CodeComments;
use crate::codegen::code_comments::kMaxCommentLength;
use crate::codegen::loong64::register_loong64::FPURegister;
use crate::flags::Flags;

    /// Class Operand represents a shifter operand in data processing instructions
    #[derive(Clone, Copy, Debug)]
    pub struct Operand {
        rm_: Register,
        value_: Value,
        is_heap_number_request_: bool,
        rmode_: RelocInfo::Mode,
    }

    impl Operand {
        /// immediate
        #[inline]
        pub fn new(immediate: i64, rmode: RelocInfo::Mode) -> Self {
            Self {
                rm_: no_reg,
                value_: Value { immediate },
                is_heap_number_request_: false,
                rmode_: rmode,
            }
        }

        #[inline]
        pub fn zero() -> Self {
            Operand::new(0, RelocInfo::NO_INFO)
        }

        #[inline]
        pub fn new_external_reference(f: &ExternalReference) -> Self {
           
            Self {
                rm_: no_reg,
                value_: Value {
                    immediate: unsafe { mem::transmute::<*const (), i64>(f.address()) },
                },
                is_heap_number_request_: false,
                rmode_: RelocInfo::EXTERNAL_REFERENCE,
            }
        }

        pub fn new_heap_object(handle: Handle<HeapObject>) -> Self {
            Self {
                rm_: no_reg,
                value_: Value {
                    immediate: unsafe { mem::transmute::<Address, i64>(handle.address()) },
                },
                is_heap_number_request_: false,
                rmode_: RelocInfo::FULL_EMBEDDED_OBJECT,
            }
        }

        #[inline]
        pub fn new_smi(value: Tagged<Smi>) -> Self {
            Self {
                rm_: no_reg,
                value_: Value {
                    immediate: unsafe { mem::transmute::<*const (), i64>(value.ptr()) },
                },
                is_heap_number_request_: false,
                rmode_: RelocInfo::NO_INFO,
            }
        }
    
        /// rm
        #[inline]
        pub fn new_register(rm: Register) -> Self {
           Self {
                rm_: rm,
                value_: Value {
                    immediate: 0,
                },
                is_heap_number_request_: false,
                rmode_: RelocInfo::NO_INFO,
            }
        }

        pub fn embedded_number(number: f64) -> Self {
           let mut result = Self::new(0,RelocInfo::FULL_EMBEDDED_OBJECT);
            let mut smi: i32 = 0;
            if DoubleToSmiInteger(number, &mut smi) {
               return Self::new_smi(Smi::from_int(smi));
            }
            result.is_heap_number_request_ = true;
            result.value_.heap_number_request = HeapNumberRequest{ heap_number_: number };
            return result;
        }

        /// Return true if this is a register operand.
        #[inline]
        pub fn is_reg(&self) -> bool {
            self.rm_.is_valid()
        }

        pub fn must_output_reloc_info(&self, assembler: *const Assembler) -> bool {
            if self.rmode_ == RelocInfo::EXTERNAL_REFERENCE {
                //if (assembler != nullptr && assembler.predictable_code_size()) return true;
                return true; //assembler.options().record_reloc_info_for_serialization;
            } else if RelocInfo::IsNoInfo(self.rmode_) {
                return false;
            }
            true
        }

        #[inline]
        pub fn immediate(&self) -> i64 {
            assert!(self.is_immediate());
            assert!(!self.is_heap_number_request());
            self.value_.immediate
        }

        pub fn is_immediate(&self) -> bool {
            !self.rm_.is_valid()
        }

        pub fn heap_number_request(&self) -> HeapNumberRequest {
            assert!(self.is_heap_number_request());
            self.value_.heap_number_request
        }

        pub fn rm(&self) -> Register {
            self.rm_
        }

        pub fn is_heap_number_request(&self) -> bool {
            if self.is_heap_number_request_ == true {
                assert!(self.is_immediate() == true);
                assert!(self.rmode_ == RelocInfo::FULL_EMBEDDED_OBJECT
                    || self.rmode_ == RelocInfo::CODE_TARGET);
            }

            self.is_heap_number_request_
        }
    }

    /// Class MemOperand represents a memory operand in load and store instructions
    /// On PowerPC we have base register + 16bit signed value
    /// Alternatively we can have a 16bit signed value immediate
    #[derive(Clone, Copy, Debug)]
    pub struct MemOperand {
        ra_: Register, // base
        offset_: i64,   // offset
        rb_: Register, // index
    }

    impl MemOperand {
        pub fn new_with_offset(rn: Register, offset: i64) -> Self {
           Self {
                ra_: rn,
                offset_: offset,
                rb_: no_reg,
            }
        }

        pub fn new_with_registers(ra: Register, rb: Register) -> Self {
            Self {
                ra_: ra,
                offset_: 0,
                rb_: rb,
            }
        }

        pub fn new_with_registers_and_offset(ra: Register, rb: Register, offset: i64) -> Self {
            Self {
                ra_: ra,
                offset_: offset,
                rb_: rb,
            }
        }

        pub fn offset(&self) -> i64 {
            self.offset_
        }

        /// PowerPC - base register
        pub fn ra(&self) -> Register {
            self.ra_
        }

        pub fn rb(&self) -> Register {
            self.rb_
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DeferredRelocInfo {
        position_: i32,
        rmode_: RelocInfo::Mode,
        data_: i64,
    }

    impl DeferredRelocInfo {
        pub fn new_with_data(position: i32, rmode: RelocInfo::Mode, data: i64) -> Self {
            Self {
                position_: position,
                rmode_: rmode,
                data_: data,
            }
        }
        pub fn new_without_data(position: i32, rmode: RelocInfo::Mode) -> Self {
            Self {
                position_: position,
                rmode_: rmode,
                data_: 0,
            }
        }

        pub fn position(&self) -> i32 {
            self.position_
        }
        pub fn rmode(&self) -> RelocInfo::Mode {
            self.rmode_
        }
        pub fn data(&self) -> i64 {
            self.data_
        }
    }

#[derive(Debug)]
pub struct Assembler {
    base: AssemblerBase,
    scratch_register_list_: RegList,
    constant_pool_builder_: ConstantPoolBuilder,
    reloc_info_writer: RelocInfoWriter,
    next_trampoline_check_: i32,
    trampoline_pool_blocked_nesting_: i32,
    no_trampoline_pool_before_: i32,
    constant_pool_entry_sharing_blocked_nesting_: i32,
    relocations_: Vec<DeferredRelocInfo>,
    last_bound_pos_: i32,
    optimizable_cmpi_pos_: i32,
    cmpi_cr_: CRegister,
    trampoline_emitted_: bool,
    tracked_branch_count_: i32,
    trampoline_: Trampoline,
    internal_trampoline_exception_: bool,
    heap_number_requests_: Vec<HeapNumberRequest>
}

impl Assembler {
    pub fn new(options: AssemblerOptions, buffer: Option<std::unique_ptr<AssemblerBuffer>>) -> Self {
    
    let mut assembler = Self {
        base: AssemblerBase::new(options, buffer.map(|b| *b).unwrap_or_default().into()),
        scratch_register_list_: RegList { registers: vec![ip] },
        constant_pool_builder_: ConstantPoolBuilder::new(kLoadPtrMaxReachBits as i32, kLoadDoubleMaxReachBits as i32),
        reloc_info_writer: RelocInfoWriter::new(),
        next_trampoline_check_: i32::MAX,
        trampoline_pool_blocked_nesting_: 0,
        no_trampoline_pool_before_: 0,
        constant_pool_entry_sharing_blocked_nesting_: 0,
        relocations_: Vec::new(),
        last_bound_pos_: 0,
        optimizable_cmpi_pos_: -1,
        cmpi_cr_: CRegister::no_reg(),
        trampoline_emitted_: v8_flags.force_long_branches,
        tracked_branch_count_: 0,
        trampoline_: Trampoline::new(),
        internal_trampoline_exception_: false,
        heap_number_requests_: Vec::new(),

    };
    assembler.reloc_info_writer.reposition(assembler.base.buffer_start_() + assembler.base.buffer_size(), assembler.base.pc_offset());
    assembler.relocations_.reserve(128);

    assembler

    }

    pub fn new_with_zone(zone: &MaybeAssemblerZone, options: AssemblerOptions, buffer: Option<std::unique_ptr<AssemblerBuffer>>) -> Self {
       Self::new(options, buffer)
    }

    pub fn get_code(&mut self, isolate: &mut LocalIsolate, desc: &mut CodeDesc) {
        self.get_code_with_safepoint_and_handler(isolate, desc, None, 0);
    }
     
    pub fn get_code_with_isolate(&mut self, isolate: &mut Isolate, desc: &mut CodeDesc) {
         self.get_code(isolate.main_thread_local_isolate(), desc);
    }


    pub fn get_code_with_safepoint_and_handler(
        &mut self,
        isolate: &mut LocalIsolate,
        desc: &mut CodeDesc,
        safepoint_table_builder: Option<&mut SafepointTableBuilderBase>,
        handler_table_offset: i32,
    ) {
        self.data_align(InstructionStream::kMetadataAlignment as i32);

        let constant_pool_size = self.emit_constant_pool();

        self.emit_relocations();

        let code_comments_size = self.write_code_comments();

        self.allocate_and_install_requested_heap_numbers(isolate);

        // Set up code descriptor.
        const kBuiltinJumpTableInfoSize: usize = 0;
        let instruction_size = self.base.pc_offset();
        let builtin_jump_table_info_offset =
            instruction_size - kBuiltinJumpTableInfoSize;
        let code_comments_offset =
            builtin_jump_table_info_offset - code_comments_size;
        let constant_pool_offset = code_comments_offset - constant_pool_size;
        let handler_table_offset2 = if handler_table_offset == 0 {
            constant_pool_offset
        } else {
            handler_table_offset
        };

        let safepoint_table_offset = match safepoint_table_builder {
            Some(builder) => builder.safepoint_table_offset() as i32,
            None => handler_table_offset2,
        };

        let reloc_info_offset =
            (self.reloc_info_writer.pos() - self.base.buffer_start_()) as i32;

        CodeDesc::initialize(
            desc,
            &self.base,
            safepoint_table_offset,
            handler_table_offset2,
            constant_pool_offset,
            code_comments_offset,
            builtin_jump_table_info_offset,
            reloc_info_offset,
        );
    }
    fn allocate_and_install_requested_heap_numbers(&mut self, isolate: &mut LocalIsolate) {
        assert!((isolate == crate::ptr::null_mut::<LocalIsolate>() )== (self.heap_number_requests_.len() == 0));
        for request in &self.heap_number_requests_ {
            let object = isolate.factory().new_heap_number::<AllocationType>(request.heap_number());
           let mut pc = self.base.buffer_start_() + request.offset();
           let constant_pool = crate::ptr::null_mut::<Instruction>();
           Self::set_target_address_at(pc, unsafe{std::mem::transmute(constant_pool)}, unsafe{std::mem::transmute(object.address())},
                                                None,ICacheFlushMode::SKIP_ICACHE_FLUSH);
        }
    }

   

    fn data_align(&mut self, m: i32) {
        assert!(m >= 2 && (m & (m - 1)) == 0);

        while (self.base.pc_offset() & (m - 1)) != 0 {
            self.db(0);
        }
    }

    fn emit_relocations(&mut self) {
        self.ensure_space_for(self.relocations_.len() * Assembler::K_MAX_RELOC_SIZE);

        for rinfo in &self.relocations_ {
            let rmode = rinfo.rmode();
            let pc = self.base.buffer_start_() + rinfo.position();

            if RelocInfo::IsInternalReference(rmode) {
                let pos: i64 = unsafe {Memory::<Address>::load(pc as *const Address) as i64};
               unsafe { Memory::<Address>::store(pc as *mut Address, (self.base.buffer_start_() + pos) as Address)};
            } else if RelocInfo::IsInternalReferenceEncoded(rmode) {
                let pos: i64 = Self::target_address_at(pc as *const u8, crate::ptr::null_mut::<u8>()) as i64;
               Self::set_target_address_at(pc as *mut u8, unsafe{std::mem::transmute(crate::ptr::null_mut::<u8>())}, (self.base.buffer_start_() + pos) as Address,
                                                  None,
                                                ICacheFlushMode::SKIP_ICACHE_FLUSH);
            }
            let reloc_info = RelocInfo::new(pc as *mut u8, rmode, rinfo.data());
            self.reloc_info_writer.write(&reloc_info);
        }
    }
       fn ensure_space_for(&mut self, space_needed: usize) {
        if (self.buffer_space() as usize) <= (Self::K_GAP + space_needed) {
            self.grow_buffer(space_needed as i32);
        }
    }
    
        fn grow_buffer(&mut self, needed: i32) {
        assert_eq!(self.base.buffer_start_(), self.base.buffer_.start());
    
        let old_size = self.base.buffer_.size();
        let new_size = std::cmp::min(2 * old_size, old_size + 1 * 1024 * 1024);
        let space = self.buffer_space() + (new_size - old_size);
        let new_size = if space < needed {
            new_size + (needed - space)
        } else {
            new_size
        };
    
        if new_size > Self::K_MAXIMAL_BUFFER_SIZE {
           // unsafe{V8::FatalProcessOutOfMemory(crate::ptr::null_mut(), "Assembler::GrowBuffer")};
        }
    
        let mut new_buffer = self.base.buffer_.grow(new_size);
    
        assert_eq!(new_size, new_buffer.size());
        let new_start = new_buffer.start();
        let pc_delta = new_start - self.base.buffer_start_();
        let rc_delta = (new_start + new_size) - (self.base.buffer_start_() + old_size);
        let reloc_size = (self.base.buffer_start_() + old_size) - self.reloc_info_writer.pos();
        unsafe {
            std::ptr::copy_nonoverlapping(self.base.buffer_start_(), new_start, self.base.pc_offset() as usize);
            std::ptr::copy_nonoverlapping(self.reloc_info_writer.pos(), self.reloc_info_writer.pos() + rc_delta, reloc_size as usize);
        }
        self.base.buffer_ = new_buffer;
        self.base.buffer_start_ = new_start;
        self.base.pc_ += pc_delta;
        self.reloc_info_writer.reposition(self.reloc_info_writer.pos() + rc_delta, self.reloc_info_writer.last_pc() + pc_delta);
        }
 
        fn write_code_comments(&mut self) -> i32 {
       0
    }
    fn emit_constant_pool(&mut self) -> i32 {
       0
    }
   
       fn buffer_space(&self) -> i32 {
        self.reloc_info_writer.pos() - self.base.pc_offset() as i32
    }
  
    fn is_constant_pool_available(&self) -> bool {
         true
     }
        /// A constant defined to avoid the need for reimplementing it, just a simple getter
     const K_GAP: i32 = 32;
     /// A constant defined to avoid the need for reimplementing it, just a simple getter
     const K_MAXIMAL_BUFFER_SIZE: i32 = 512 * 1024 * 1024;
    
}

struct PatchingAssembler {
    assembler: Assembler
}
impl PatchingAssembler {
    fn new(options: AssemblerOptions, address: usize, instructions: i32) -> Self {
        PatchingAssembler {
            assembler: Assembler::new(options, Some(Box::new(AssemblerBuffer::with_external_buffer(address as *mut u8, instructions as usize * AssemblerBase::kInstrSize + Assembler::K_GAP as usize ))))
        }
    }
}
#[derive(Debug)]
pub struct Trampoline{
    next_slot_:i32,
    free_slot_count_:i32
}
impl Trampoline {
    pub fn new() -> Self{
       Trampoline {
            next_slot_:0,
            free_slot_count_:0
        }
    }
}

} // end of module

