// Copyright (c) 1994-2006 Sun Microsystems Inc.
// All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistribution in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the
// distribution.
//
// - Neither the name of Sun Microsystems or the names of contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.

// The original source code covered by the above license above has been
// modified significantly by Google Inc.
// Copyright 2012 the V8 project authors. All rights reserved.

// A light-weight ARM Assembler
// Generates user mode instructions for the ARM architecture up to version 5

pub mod assembler_arm {
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Not};

    //use crate::base::numbers::double::Double; // Assuming Double is in this module.
    //use crate::base::small_vector::SmallVector; // Assuming SmallVector is in this module.
    //use crate::codegen::arm::constants_arm::*; // Assuming constants_arm is in this module.
    //use crate::codegen::arm::register_arm::*; // Assuming register_arm is in this module.
    //use crate::codegen::assembler::*; // Assuming assembler is in this module.
    //use crate::codegen::constant_pool::*; // Assuming constant_pool is in this module.
    //use crate::codegen::machine_type::*; // Assuming machine_type is in this module.
    //use crate::utils::boxed_float::*; // Assuming boxed_float is in this module.

    // Dummy imports and type definitions to allow compilation
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RelocInfoMode {
        NO_INFO,
        FULL_EMBEDDED_OBJECT,
        CODE_TARGET,
        INTERNAL_REFERENCE,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ShiftOp {
        LSL,
        ASR,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ExternalReference {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct HeapObject {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Smi {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register(u16);
    impl Register {
        pub const no_reg: Register = Register(0xffff);
        pub fn is_valid(self) -> bool { self != Self::no_reg }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct HeapNumberRequest {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AddrMode {
        Offset,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BlockAddrMode {
        ia_w,
        db_w
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LFlag {
        Short
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SwVfpRegister(u16);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DwVfpRegister(u16);
    impl DwVfpRegister {
        pub fn code(&self) -> u16 { self.0 }
        pub fn is_valid(&self) -> bool { self.0 != 0xffff }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QwNeonRegister(u16);
    impl QwNeonRegister {
        pub fn low(&self) -> DwVfpRegister { DwVfpRegister(self.0) }
        pub fn high(&self) -> DwVfpRegister { DwVfpRegister(self.0 + 1) }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct LowDwVfpRegister(u16);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum NeonSize {
        Byte,
        Word,
        Dword,
        Qword
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum NeonDataType {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Condition {
        al,
        eq,
        ne,
        ge,
        lt,
        gt,
        le,
        vs,
        vc,
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SBit {
        LeaveCC
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SRegister {}
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SRegisterFieldMask {}
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BarrierOption {}
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CRegister {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum DeoptimizeReason {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SourcePosition {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Instr {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ICacheFlushMode {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AssemblerOptions {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MaybeAssemblerZone {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct AssemblerBuffer {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CodeDesc {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct LocalIsolate {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Isolate {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct WritableJitAllocation {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum VFPConversionMode {
        kDefaultRoundToZero
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MachineRepresentation {}

    const kSmiTag: usize = 0;
    const kSmiTagSize: i32 = 1;
    const kPointerSizeLog2: i32 = 2;
    const kDoubleSizeLog2: i32 = 3;
    const kPointerSize: i32 = 4;

    const kMaxInt: i32 = i32::MAX;

    const kDefaultStopCode: i32 = 0;
    const kInstrSize: i32 = 4;
    const FLUSH_ICACHE_IF_NEEDED: ICacheFlushMode = ICacheFlushMode {};

    fn is_uint12(x: i32) -> bool {
        x >= 0 && x < 4096
    }

    /// Coprocessor number
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Coprocessor {
        p0 = 0,
        p1 = 1,
        p2 = 2,
        p3 = 3,
        p4 = 4,
        p5 = 5,
        p6 = 6,
        p7 = 7,
        p8 = 8,
        p9 = 9,
        p10 = 10,
        p11 = 11,
        p12 = 12,
        p13 = 13,
        p14 = 14,
        p15 = 15,
    }

    /// Class Operand represents a shifter operand in data processing instructions
    #[derive(Debug, Copy, Clone)]
    pub struct Operand {
        rm_: Register,
        rs_: Register,
        shift_op_: ShiftOp,
        shift_imm_: i32,
        value_: Value,
        is_heap_number_request_: bool,
        rmode_: RelocInfoMode,
    }

    impl Operand {
        /// immediate
        #[inline]
        pub fn new_immediate(immediate: i32, rmode: RelocInfoMode) -> Self {
            Operand {
                rm_: Register::no_reg,
                rs_: Register::no_reg,
                shift_op_: ShiftOp::LSL,
                shift_imm_: 0,
                value_: Value { immediate },
                is_heap_number_request_: false,
                rmode_: rmode,
            }
        }

        #[inline]
        pub fn zero() -> Self {
            Operand::new_immediate(0, RelocInfoMode::NO_INFO)
        }

        #[inline]
        pub fn new_external_reference(f: &ExternalReference) -> Self {
            todo!()
        }

        pub fn new_handle(handle: &HeapObject) -> Self {
            todo!()
        }

        #[inline]
        pub fn new_tagged(value: &Smi) -> Self {
            todo!()
        }

        /// rm
        #[inline]
        pub fn new_register(rm: Register) -> Self {
            Operand {
                rm_: rm,
                rs_: Register::no_reg,
                shift_op_: ShiftOp::LSL,
                shift_imm_: 0,
                value_: Value { immediate: 0 },
                is_heap_number_request_: false,
                rmode_: RelocInfoMode::NO_INFO,
            }
        }

        // rm <shift_op> shift_imm
        pub fn new_shifted_register(rm: Register, shift_op: ShiftOp, shift_imm: i32) -> Self {
            Operand {
                rm_: rm,
                rs_: Register::no_reg,
                shift_op_: shift_op,
                shift_imm_: shift_imm,
                value_: Value { immediate: 0 },
                is_heap_number_request_: false,
                rmode_: RelocInfoMode::NO_INFO,
            }
        }
        #[inline]
        pub fn smi_untag(rm: Register) -> Self {
            Operand::new_shifted_register(rm, ShiftOp::ASR, kSmiTagSize)
        }
        #[inline]
        pub fn pointer_offset_from_smi_key(key: Register) -> Self {
            assert!(kSmiTag == 0 && kSmiTagSize < kPointerSizeLog2);
            Operand::new_shifted_register(key, ShiftOp::LSL, kPointerSizeLog2 - kSmiTagSize)
        }
        #[inline]
        pub fn double_offset_from_smi_key(key: Register) -> Self {
            assert!(kSmiTag == 0 && kSmiTagSize < kDoubleSizeLog2);
            Operand::new_shifted_register(key, ShiftOp::LSL, kDoubleSizeLog2 - kSmiTagSize)
        }

        // rm <shift_op> rs
        pub fn new_shifted_register_with_reg(rm: Register, shift_op: ShiftOp, rs: Register) -> Self {
            Operand {
                rm_: rm,
                rs_: rs,
                shift_op_: shift_op,
                shift_imm_: 0,
                value_: Value { immediate: 0 },
                is_heap_number_request_: false,
                rmode_: RelocInfoMode::NO_INFO,
            }
        }

        pub fn embedded_number(number: f64) -> Self {
            todo!()
        }

        // Return true if this is a register operand.
        pub fn is_register(&self) -> bool {
            self.rm_.is_valid() && self.rs_ == Register::no_reg && self.shift_op_ == ShiftOp::LSL &&
                self.shift_imm_ == 0
        }
        // Return true if this is a register operand shifted with an immediate.
        pub fn is_immediate_shifted_register(&self) -> bool {
            self.rm_.is_valid() && !self.rs_.is_valid()
        }
        // Return true if this is a register operand shifted with a register.
        pub fn is_register_shifted_register(&self) -> bool {
            self.rm_.is_valid() && self.rs_.is_valid()
        }

        // Return the number of actual instructions required to implement the given
        // instruction for this particular operand. This can be a single instruction,
        // if no load into a scratch register is necessary, or anything between 2 and
        // 4 instructions when we need to load from the constant pool (depending upon
        // whether the constant pool entry is in the small or extended section). If
        // the instruction this operand is used for is a MOV or MVN instruction the
        // actual instruction to use is required for this calculation. For other
        // instructions instr is ignored.
        //
        // The value returned is only valid as long as no entries are added to the
        // constant pool between this call and the actual instruction being emitted.
        pub fn instructions_required(&self, assembler: &Assembler, instr: i32) -> i32 {
            todo!()
        }
        pub fn must_output_reloc_info(&self, assembler: &Assembler) -> bool {
            todo!()
        }

        #[inline]
        pub fn immediate(&self) -> i32 {
            assert!(self.is_immediate());
            assert!(!self.is_heap_number_request());
            self.value_.immediate
        }
        pub fn is_immediate(&self) -> bool { !self.rm_.is_valid() }

        pub fn heap_number_request(&self) -> HeapNumberRequest {
            assert!(self.is_heap_number_request());
            self.value_.heap_number_request
        }
        pub fn is_heap_number_request(&self) -> bool {
            if self.is_heap_number_request_ {
              assert!(self.is_immediate());
              assert!(self.rmode_ == RelocInfoMode::FULL_EMBEDDED_OBJECT ||
                      self.rmode_ == RelocInfoMode::CODE_TARGET);
            }
            self.is_heap_number_request_
        }

        pub fn rm(&self) -> Register { self.rm_ }
        pub fn rs(&self) -> Register { self.rs_ }
        pub fn shift_op(&self) -> ShiftOp { self.shift_op_ }
    }

    #[derive(Debug, Copy, Clone)]
    union Value {
        heap_number_request: HeapNumberRequest,
        immediate: i32,
    }

    /// Class MemOperand represents a memory operand in load and store instructions
    #[derive(Debug, Copy, Clone)]
    pub struct MemOperand {
        rn_: Register,
        rm_: Register,
        offset_: i32,
        shift_op_: ShiftOp,
        shift_imm_: i32,
        am_: AddrMode,
    }

    impl MemOperand {
        // [rn +/- offset]      Offset/NegOffset
        // [rn +/- offset]!     PreIndex/NegPreIndex
        // [rn], +/- offset     PostIndex/NegPostIndex
        // offset is any signed 32-bit value; offset is first loaded to a scratch
        // register if it does not fit the addressing mode (12-bit unsigned and sign
        // bit)
        pub fn new_offset(rn: Register, offset: i32, am: AddrMode) -> Self {
            MemOperand {
                rn_: rn,
                rm_: Register::no_reg,
                offset_: offset,
                shift_op_: ShiftOp::LSL,
                shift_imm_: 0,
                am_: am,
            }
        }

        // [rn +/- rm]          Offset/NegOffset
        // [rn +/- rm]!         PreIndex/NegPreIndex
        // [rn], +/- rm         PostIndex/NegPostIndex
        pub fn new_register_offset(rn: Register, rm: Register, am: AddrMode) -> Self {
            MemOperand {
                rn_: rn,
                rm_: rm,
                offset_: 0,
                shift_op_: ShiftOp::LSL,
                shift_imm_: 0,
                am_: am,
            }
        }

        // [rn +/- rm <shift_op> shift_imm]      Offset/NegOffset
        // [rn +/- rm <shift_op> shift_imm]!     PreIndex/NegPreIndex
        // [rn], +/- rm <shift_op> shift_imm     PostIndex/NegPostIndex
        pub fn new_shifted_register_offset(rn: Register, rm: Register, shift_op: ShiftOp, shift_imm: i32, am: AddrMode) -> Self {
            MemOperand {
                rn_: rn,
                rm_: rm,
                offset_: 0,
                shift_op_: shift_op,
                shift_imm_: shift_imm,
                am_: am,
            }
        }

        #[inline]
        pub fn pointer_address_from_smi_key(array: Register, key: Register, am: AddrMode) -> Self {
            assert!(kSmiTag == 0 && kSmiTagSize < kPointerSizeLog2);
            MemOperand::new_shifted_register_offset(array, key, ShiftOp::LSL, kPointerSizeLog2 - kSmiTagSize, am)
        }

        pub fn is_immediate_offset(&self) -> bool { self.rm_ == Register::no_reg }

        pub fn set_offset(&mut self, offset: i32) {
            assert!(self.is_immediate_offset());
            self.offset_ = offset;
        }

        pub fn offset(&self) -> i32 {
            assert!(self.is_immediate_offset());
            self.offset_
        }

        pub fn rn(&self) -> Register { self.rn_ }
        pub fn rm(&self) -> Register { self.rm_ }
        pub fn am(&self) -> AddrMode { self.am_ }

        pub fn offset_is_uint12_encodable(&self) -> bool {
            if self.offset_ >= 0 {
                is_uint12(self.offset_)
            } else {
                is_uint12(-self.offset_)
            }
        }
    }

    /// Class NeonMemOperand represents a memory operand in load and
    /// store NEON instructions
    #[derive(Debug, Copy, Clone)]
    pub struct NeonMemOperand {
        rn_: Register, // base
        rm_: Register, // register increment
        align_: i32,
    }

    impl NeonMemOperand {
        // [rn {:align}]       Offset
        // [rn {:align}]!      PostIndex
        pub fn new_offset(rn: Register, am: AddrMode, align: i32) -> Self {
            let mut operand = NeonMemOperand {
                rn_: rn,
                rm_: Register::no_reg,
                align_: 0,
            };
            operand.set_alignment(align);
            operand
        }

        // [rn {:align}], rm   PostIndex
        pub fn new_post_index(rn: Register, rm: Register, align: i32) -> Self {
            let mut operand = NeonMemOperand {
                rn_: rn,
                rm_: rm,
                align_: 0,
            };
            operand.set_alignment(align);
            operand
        }

        pub fn rn(&self) -> Register { self.rn_ }
        pub fn rm(&self) -> Register { self.rm_ }
        pub fn align(&self) -> i32 { self.align_ }

        fn set_alignment(&mut self, align: i32) {
            self.align_ = align;
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct NeonListOperand {
        base_: DwVfpRegister,
        register_count_: i32,
    }

    impl NeonListOperand {
        pub fn new(base: DwVfpRegister, register_count: i32) -> Self {
            NeonListOperand {
                base_: base,
                register_count_: register_count,
            }
        }

        pub fn new_q(q_reg: QwNeonRegister) -> Self {
            NeonListOperand {
                base_: q_reg.low(),
                register_count_: 2,
            }
        }
        pub fn base(&self) -> DwVfpRegister { self.base_ }
        pub fn register_count(&self) -> i32 { self.register_count_ }
        pub fn length(&self) -> i32 { self.register_count_ - 1 }
        pub fn list_type(&self) -> NeonListType {
            match self.register_count_ {
                1 => NeonListType::nlt_1,
                2 => NeonListType::nlt_2,
                3 => NeonListType::nlt_3,
                4 => NeonListType::nlt_4,
                _ => panic!("UNREACHABLE"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum NeonListType {
        nlt_1,
        nlt_2,
        nlt_3,
        nlt_4,
    }

    pub struct Assembler {
        assembler_base: AssemblerBase,
        reloc_info_writer: RelocInfoWriter,
        pending_32_bit_constants_: Vec<ConstantPoolEntry>,
        scratch_register_list_: RegList,
        scratch_vfp_register_list_: VfpRegList,
        constant_pool_deadline_: i32,
        const_pool_blocked_nesting_: i32,
        no_const_pool_before_: i32,
        first_const_pool_32_use_: i32,
        last_bound_pos_: i32,
    }

    impl Assembler {
        /// Create an assembler. Instructions and relocation information are emitted
        /// into a buffer, with the instructions starting from the beginning and the
        /// relocation information starting from the end of the buffer. See CodeDesc
        /// for a detailed comment on the layout (globals.h).
        ///
        /// If the provided buffer is None, the assembler allocates and grows its
        /// own buffer. Otherwise it takes ownership of the provided buffer.
        pub fn new(options: &AssemblerOptions, buffer: Option<AssemblerBuffer>) -> Self {
            Assembler {
                assembler_base: AssemblerBase::new(),
                reloc_info_writer: RelocInfoWriter::new(),
                pending_32_bit_constants_: Vec::new(),
                scratch_register_list_: Self::default_tmp_list(),
                scratch_vfp_register_list_: Self::default_fptmp_list(),
                constant_pool_deadline_: kMaxInt,
                const_pool_blocked_nesting_: 0,
                no_const_pool_before_: 0,
                first_const_pool_32_use_: -1,
                last_bound_pos_: 0,
            }
        }
        // For compatibility with assemblers that require a zone.
        pub fn new_with_zone(zone: &MaybeAssemblerZone, options: &AssemblerOptions, buffer: Option<AssemblerBuffer>) -> Self {
            Self::new(options, buffer)
        }

        fn default_tmp_list() -> RegList {
            RegList(0) // Replace with the actual initialization logic.
        }

        fn default_fptmp_list() -> VfpRegList {
            VfpRegList(0) // Replace with the actual initialization logic.
        }

        pub fn aborted_code_generation(&mut self) {
            self.pending_32_bit_constants_.clear();
            self.first_const_pool_32_use_ = -1;
            self.constant_pool_deadline_ = kMaxInt;
        }

        // GetCode emits any pending (non-emitted) code and fills the descriptor desc.
        pub const K_NO_HANDLER_TABLE: i32 = 0;
        pub const K_NO_SAFEPOINT_TABLE: *const SafepointTableBuilderBase = std::ptr::null();
        pub fn get_code(&mut self, isolate: &mut LocalIsolate, desc: &mut CodeDesc, safepoint_table_builder: *mut SafepointTableBuilderBase, handler_table_offset: i32) {
            todo!()
        }

        // Convenience wrapper for allocating with an Isolate.
        pub fn get_code_isolate(&mut self, isolate: &mut Isolate, desc: &mut CodeDesc) {
            todo!()
        }
        // Convenience wrapper for code without safepoint or handler tables.
        pub fn get_code_isolate_local(&mut self, isolate: &mut LocalIsolate, desc: &mut CodeDesc) {
           self.get_code(isolate, desc, Self::K_NO_SAFEPOINT_TABLE as *mut SafepointTableBuilderBase, Self::K_NO_HANDLER_TABLE);
        }

        // Label operations & relative jumps (PPUM Appendix D)
        //
        // Takes a branch opcode (cc) and a label (L) and generates
        // either a backward branch or a forward branch and links it
        // to the label fixup chain. Usage:
        //
        // Label L;    // unbound label
        // j(cc, &L);  // forward branch to unbound label
        // bind(&L);   // bind label to the current pc
        // j(cc, &L);  // backward branch to bound label
        // bind(&L);   // illegal: a label may be bound only once
        //
        // Note: The same Label can be used for forward and backward branches
        // but it may be bound only once.

        pub fn bind(&mut self, l: &mut Label) {
            todo!()
        }  // binds an unbound label L to the current code position

        // Returns the branch offset to the given label from the current code position
        // Links the label to the current position if it is still unbound
        // Manages the jump elimination optimization if the second parameter is true.
        pub fn branch_offset(&mut self, l: &mut Label) -> i32 {
            todo!()
        }

        // Returns true if the given pc address is the start of a constant pool load
        // instruction sequence.
        #[inline]
        pub fn is_constant_pool_load(pc: *const u8) -> bool {
            todo!()
        }

        // Return the address in the constant pool of the code target address used by
        // the branch/call instruction at pc, or the object in a mov.
        #[inline]
        pub fn constant_pool_entry_address(pc: *const u8, constant_pool: *const u8) -> *const u8 {
            todo!()
        }

        // Read/Modify the code target address in the branch/call instruction at pc.
        // The isolate argument is unused (and may be nullptr) when skipping flushing.
        #[inline]
        pub fn target_address_at(pc: *const u8, constant_pool: *const u8) -> *const u8 {
            todo!()
        }
        #[inline]
        pub fn set_target_address_at(
            pc: *mut u8,
            constant_pool: *const u8,
            target: *const u8,
            jit_allocation: &mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            todo!()
        }

        // Get the size of the special target encoded at 'location'.
        #[inline]
        pub fn deserialization_special_target_size(location: *const u8) -> i32 {
            todo!()
        }

        // This sets the internal reference at the pc.
        #[inline]
        pub fn deserialization_set_target_internal_reference_at(
            pc: *mut u8,
            target: *const u8,
            jit_allocation: &mut WritableJitAllocation,
            mode: RelocInfoMode,
        ) {
            todo!()
        }

        // Read/modify the uint32 constant used at pc.
        pub fn uint32_constant_at(pc: *const u8, constant_pool: *const u8) -> u32 {
            todo!()
        }
        pub fn set_uint32_constant_at(
            pc: *mut u8,
            constant_pool: *const u8,
            new_constant: u32,
            jit_allocation: &mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            todo!()
        }

        // Here we are patching the address in the constant pool, not the actual call
        // instruction.  The address in the constant pool is the same size as a
        // pointer.
        pub const K_SPECIAL_TARGET_SIZE: i32 = kPointerSize;

        pub fn get_scratch_register_list(&mut self) -> &mut RegList {
            &mut self.scratch_register_list_
        }
        pub fn get_scratch_vfp_register_list(&mut self) -> &mut VfpRegList {
            &mut self.scratch_vfp_register_list_
        }

        // ---------------------------------------------------------------------------
        // InstructionStream generation

        // Insert the smallest number of nop instructions
        // possible to align the pc offset to a multiple
        // of m. m must be a power of 2 (>= 4).
        pub fn align(&mut self, m: i32) {
            todo!()
        }
        // Insert the smallest number of zero bytes possible to align the pc offset
        // to a mulitple of m. m must be a power of 2 (>= 2).
        pub fn data_align(&mut self, m: i32) {
            todo!()
        }
        // Aligns code to something that's optimal for a jump target for the platform.
        pub fn code_target_align(&mut self) {
            todo!()
        }
        pub fn loop_header_align(&mut self) {
            self.code_target_align();
        }

        // Branch instructions
        pub fn b(&mut self, branch_offset: i32, cond: Condition, rmode: RelocInfoMode) {
            todo!()
        }
        pub fn bl(&mut self, branch_offset: i32, cond: Condition, rmode: RelocInfoMode) {
            todo!()
        }
        pub fn blx_offset(&mut self, branch_offset: i32) {
            todo!()
        }  // v5 and above
        pub fn blx_register(&mut self, target: Register, cond: Condition) {
            todo!()
        }  // v5 and above
        pub fn bx(&mut self, target: Register, cond: Condition) {
            todo!()
        }   // v5 and above, plus v4t

        // Convenience branch instructions using labels
        pub fn b_label(&mut self, l: &mut Label, cond: Condition) {
            todo!()
        }
        pub fn b_cond_label(&mut self, cond: Condition, l: &mut Label) {
            self.b_label(l, cond);
        }
        pub fn bl_label(&mut self, l: &mut Label, cond: Condition) {
            todo!()
        }
        pub fn bl_cond_label(&mut self, cond: Condition, l: &mut Label) {
            self.bl_label(l, cond);
        }
        pub fn blx_label(&mut self, l: &mut Label) {
            todo!()
        }  // v5 and above

        // Data-processing instructions

        pub fn and_(&mut self, dst: Register, src1: Register, src2: &Operand, s: SBit, cond: Condition) {
            todo!()
        }
        pub fn and_reg(&mut self, dst: Register, src1: Register, src2: Register, s: SBit, cond: Condition) {
            todo!()
        }

        pub fn eor(&mut self, dst: Register, src1: Register, src2: &Operand, s: SBit, cond: Condition) {
            todo!()
        }
        pub fn eor_reg(&mut self, dst: Register, src1: Register, src2: Register, s: SBit, cond: Condition) {
            todo!()
        }

        pub fn sub(&mut self, dst: Register, src1: Register, src2: &Operand, s: SBit, cond: Condition) {
            todo!()
        }
        pub fn sub_reg(&mut self, dst: Register, src1: Register, src2: Register, s: SBit, cond: Condition) {
            todo!()
        }

        pub fn rsb(&mut self, dst: Register, src1: Register, src2: &Operand, s: SBit, cond: Condition) {
            todo!()
        }

        pub fn add(&mut self, dst: Register, src1: Register, src2: &Operand, s: SBit, cond: Condition) {
            todo!()
        }
        pub fn add_reg(&mut self, dst: Register,