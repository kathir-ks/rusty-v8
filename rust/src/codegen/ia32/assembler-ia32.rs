// Copyright (c) 1994-2006 Sun Microsystems Inc.
// All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistribution in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// - Neither the name of Sun Microsystems or the names of contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// The original source code covered by the above license above has been
// modified significantly by Google Inc.
// Copyright 2011 the V8 project authors. All rights reserved.

// A light-weight IA32 Assembler.

pub mod assembler_ia32 {
    use std::mem;
    use std::ptr;

    use crate::codegen::assembler::*;
    use crate::codegen::ia32::constants_ia32::*;
    //use crate::codegen::ia32::fma_instr::*; // FMA instructions not fully implemented
    use crate::codegen::ia32::register_ia32::*;
    use crate::codegen::ia32::sse_instr::*;
    use crate::codegen::label::*;
    //use crate::execution::isolate::*; // Isolate struct not directly translated
    //use crate::objects::smi::*; // Tagged<Smi> not directly translated
    use crate::utils::utils::*;
    use bitfield::bitfield;

    //Placeholder for Isolate, Smi, HeapObject, ExternalReference
    pub type Isolate = ();
    pub type Smi = i32;
    pub type HeapObject = u64;
    pub type ExternalReference = u64;
    pub type Code = u64;
    pub type LocalIsolate = ();
    pub type SafepointTableBuilder = ();
    pub type WritableJitAllocation = ();
    pub type CodeDesc = ();
    pub type AssemblerOptions = ();
    pub type MaybeAssemblerZone = ();
    pub type AssemblerBuffer = ();
    pub type DeoptimizeReason = i32;
    pub type SourcePosition = i32;
    pub type ICacheFlushMode = i32;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub enum Condition {
        Overflow = 0,
        NoOverflow = 1,
        Below = 2,
        AboveEqual = 3,
        Equal = 4,
        NotEqual = 5,
        BelowEqual = 6,
        Above = 7,
        Negative = 8,
        Positive = 9,
        ParityEven = 10,
        ParityOdd = 11,
        Less = 12,
        GreaterEqual = 13,
        LessEqual = 14,
        Greater = 15,

        // aliases
        Carry = Below,
        NotCarry = AboveEqual,
        Zero = Equal,
        NotZero = NotEqual,
        Sign = Negative,
        NotSign = Positive,

        // Unified cross-platform condition names/aliases.
        KEqual = Equal,
        KNotEqual = NotEqual,
        KLessThan = Less,
        KGreaterThan = Greater,
        KLessThanEqual = LessEqual,
        KGreaterThanEqual = GreaterEqual,
        KUnsignedLessThan = Below,
        KUnsignedGreaterThan = Above,
        KUnsignedLessThanEqual = BelowEqual,
        KUnsignedGreaterThanEqual = AboveEqual,
        KOverflow = Overflow,
        KNoOverflow = NoOverflow,
        KZero = Equal,
        KNotZero = NotEqual,
    }

    // Returns the equivalent of !cc.
    #[inline]
    pub fn negate_condition(cc: Condition) -> Condition {
        unsafe { mem::transmute((cc as i32) ^ 1) }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub enum RoundingMode {
        RoundToNearest = 0x0,
        RoundDown = 0x1,
        RoundUp = 0x2,
        RoundToZero = 0x3,
    }

    // -----------------------------------------------------------------------------
    // Machine instruction Immediates

    #[derive(Debug, Copy, Clone)]
    pub struct Immediate {
        value: Value,
        is_heap_number_request_: bool,
        rmode_: RelocInfoMode,
    }

    impl Immediate {
        // Calls where x is an Address (uintptr_t) resolve to this overload.
        #[inline]
        pub fn new_int(x: i32, rmode: RelocInfoMode) -> Self {
            Immediate {
                value: Value { immediate: x },
                is_heap_number_request_: false,
                rmode_: rmode,
            }
        }

        #[inline]
        pub fn new_ext_ref(_ext: ExternalReference) -> Self {
            //Immediate::new_int(ext as i32, RelocInfoMode::ExternalReference)
            unimplemented!()
        }

        #[inline]
        pub fn new_heap_object(_handle: HeapObject) -> Self {
             //Immediate::new_int(handle as i32, RelocInfoMode::FullEmbeddedObject)
             unimplemented!()
        }

        #[inline]
        pub fn new_smi(_value: Smi) -> Self {
            //Immediate::new_int(value as i32, RelocInfoMode::NoInfo)
            unimplemented!()
        }

        pub fn embedded_number(_number: f64) -> Self {
            // Smi or HeapNumber.
            unimplemented!()
        }

        pub fn code_relative_offset(label: *mut Label) -> Self {
            Immediate::new_label(label)
        }

        pub fn is_heap_number_request(&self) -> bool {
            if self.is_heap_number_request_ {
                assert!(
                    self.rmode_ == RelocInfoMode::FullEmbeddedObject
                        || self.rmode_ == RelocInfoMode::CodeTarget
                );
            }
            self.is_heap_number_request_
        }

        pub fn heap_number_request(&self) -> HeapNumberRequest {
            assert!(self.is_heap_number_request());
            self.value.heap_number_request
        }

        pub fn immediate(&self) -> i32 {
            assert!(!self.is_heap_number_request());
            self.value.immediate
        }

        pub fn is_embedded_object(&self) -> bool {
            !self.is_heap_number_request() && self.rmode() == RelocInfoMode::FullEmbeddedObject
        }

        pub fn embedded_object(&self) -> HeapObject {
            //Handle<HeapObject>(reinterpret_cast<Address*>(immediate()))
            unimplemented!()
        }

        pub fn is_external_reference(&self) -> bool {
            self.rmode() == RelocInfoMode::ExternalReference
        }

        pub fn external_reference(&self) -> ExternalReference {
            assert!(self.is_external_reference());
            //base::bit_cast<ExternalReference>(immediate())
            unimplemented!()
        }

        pub fn is_zero(&self) -> bool {
            RelocInfoMode::is_no_info(self.rmode_) && self.immediate() == 0
        }

        pub fn is_int8(&self) -> bool {
            RelocInfoMode::is_no_info(self.rmode_) && is_int8(self.immediate())
        }

        pub fn is_uint8(&self) -> bool {
            RelocInfoMode::is_no_info(self.rmode_) && is_uint8(self.immediate())
        }

        pub fn is_int16(&self) -> bool {
            RelocInfoMode::is_no_info(self.rmode_) && is_int16(self.immediate())
        }

        pub fn is_uint16(&self) -> bool {
            RelocInfoMode::is_no_info(self.rmode_) && is_uint16(self.immediate())
        }

        pub fn rmode(&self) -> RelocInfoMode {
            self.rmode_
        }

        fn new_label(value: *mut Label) -> Self {
            Immediate {
                value: Value {
                    immediate: value as i32,
                },
                is_heap_number_request_: false,
                rmode_: RelocInfoMode::InternalReference,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    union Value {
        heap_number_request: HeapNumberRequest,
        immediate: i32,
    }

    impl Value {
        const fn new() -> Self {
            Value { immediate: 0 }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct HeapNumberRequest {}

    // -----------------------------------------------------------------------------
    // Machine instruction Operands

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub enum ScaleFactor {
        Times1 = 0,
        Times2 = 1,
        Times4 = 2,
        Times8 = 3,
        TimesIntSize = Times4,

        TimesHalfSystemPointerSize = Times2,
        TimesSystemPointerSize = Times4,

        TimesTaggedSize = Times4,
    }

    const K_SYSTEM_POINTER_SIZE: usize = 4;

    #[derive(Copy, Clone)]
    pub struct Operand {
        buf_: [u8; 6],
        len_: u8,
        rmode_: RelocInfoMode,
    }

    impl Operand {
        // reg
        #[inline]
        pub fn new_reg(reg: Register) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(3, reg);
            operand
        }

        // XMM reg
        #[inline]
        pub fn new_xmm_reg(xmm_reg: XMMRegister) -> Self {
            let reg = Register::from_code(xmm_reg.code());
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(3, reg);
            operand
        }

        // [disp/r]
        #[inline]
        pub fn new_disp_rmode(disp: i32, rmode: RelocInfoMode) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, EBP);
            operand.set_dispr(disp, rmode);
            operand
        }

        // [disp/r]
        #[inline]
        pub fn new_imm(imm: Immediate) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, EBP);
            operand.set_dispr(imm.immediate(), imm.rmode_);
            operand
        }

        // [base + disp/r]
        pub fn new_base_disp_rmode(base: Register, disp: i32, rmode: RelocInfoMode) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, base);
            operand.set_dispr(disp, rmode);
            operand
        }

        // [disp/r]
        pub fn new_label(label: *mut Label) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, EBP);
            operand.set_dispr(label as i32, RelocInfoMode::InternalReference);
            operand
        }

        // [base + index*scale + disp/r]
        pub fn new_base_index_scale_disp_rmode(
            base: Register,
            index: Register,
            scale: ScaleFactor,
            disp: i32,
            rmode: RelocInfoMode,
        ) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, ESP);
            operand.set_sib(scale, index, base);
            operand.set_dispr(disp, rmode);
            operand
        }

        // [index*scale + disp/r]
        pub fn new_index_scale_disp_rmode(
            index: Register,
            scale: ScaleFactor,
            disp: i32,
            rmode: RelocInfoMode,
        ) -> Self {
            let mut operand = Operand {
                buf_: [0; 6],
                len_: 0,
                rmode_: RelocInfoMode::NoInfo,
            };
            operand.set_modrm(0, ESP);
            operand.set_sib(scale, index, EBP);
            operand.set_dispr(disp, rmode);
            operand
        }

        pub fn jump_table(index: Register, scale: ScaleFactor, table: *mut Label) -> Self {
            Operand::new_index_scale_disp_rmode(
                index,
                scale,
                table as i32,
                RelocInfoMode::InternalReference,
            )
        }

        pub fn for_register_plus_immediate(base: Register, imm: Immediate) -> Self {
            Operand::new_base_disp_rmode(base, imm.immediate(), imm.rmode_)
        }

        // Returns true if this Operand is a wrapper for the specified register.
        pub fn is_reg(&self, reg: Register) -> bool {
            self.is_reg_code(reg.code())
        }
        pub fn is_xmm_reg(&self, reg: XMMRegister) -> bool {
            self.is_reg_code(reg.code())
        }

        // Returns true if this Operand is a wrapper for one register.
        pub fn is_reg_only(&self) -> bool {
            (self.buf_[0] & 0xF8) == 0xC0
        }

        // Asserts that this Operand is a wrapper for one register and returns the
        // register.
        pub fn reg(&self) -> Register {
            assert!(self.is_reg_only());
            Register::from_code((self.buf_[0] & 0x07) as i32)
        }

        pub fn encoded_bytes(&self) -> Vec<u8> {
            self.buf_[..self.len_ as usize].to_vec()
        }
        pub fn rmode(&self) -> RelocInfoMode {
            self.rmode_
        }

        // Set the ModRM byte without an encoded 'reg' register. The
        // register is encoded later as part of the emit_operand operation.
        #[inline]
        fn set_modrm(&mut self, mod_: i32, rm: Register) {
            assert_eq!(mod_ & -4, 0);
            self.buf_[0] = (mod_ << 6 | rm.code()) as u8;
            self.len_ = 1;
        }

        #[inline]
        fn set_sib(&mut self, scale: ScaleFactor, index: Register, base: Register) {
            self.buf_[1] = ((scale as i32) << 6 | (index.code() << 3) | base.code()) as u8;
            self.len_ = 2;
        }

        #[inline]
        fn set_disp8(&mut self, disp: i8) {
            assert!(self.len_ == 1 || self.len_ == 2);
            self.buf_[self.len_ as usize] = disp as u8;
            self.len_ += 1;
        }

        #[inline]
        fn set_dispr(&mut self, disp: i32, rmode: RelocInfoMode) {
            assert!(self.len_ == 1 || self.len_ == 2);
            unsafe {
                let p = self.buf_.as_mut_ptr().add(self.len_ as usize) as *mut i32;
                *p = disp;
            }
            self.len_ += 4;
            self.rmode_ = rmode;
        }

        #[inline]
        fn is_reg_code(&self, reg_code: i32) -> bool {
            ((self.buf_[0] & 0xF8) == 0xC0) // addressing mode is register only.
                && ((self.buf_[0] & 0x07) == reg_code as u8)
                // register codes match.
        }
    }

    impl std::fmt::Debug for Operand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Operand")
                .field("buf_", &self.buf_)
                .field("len_", &self.len_)
                .field("rmode_", &self.rmode_)
                .finish()
        }
    }

    impl std::cmp::PartialEq<XMMRegister> for Operand {
        fn eq(&self, _other: &XMMRegister) -> bool {
            unimplemented!()
        }

        fn ne(&self, other: &XMMRegister) -> bool {
            !(self == other)
        }
    }

    impl std::cmp::Eq for Operand {}

    // -----------------------------------------------------------------------------
    // A Displacement describes the 32bit immediate field of an instruction which
    // may be used together with a Label in order to refer to a yet unknown code
    // position. Displacements stored in the instruction stream are used to describe
    // the instruction and to chain a list of instructions using the same Label.
    // A Displacement contains 2 different fields:
    //
    // next field: position of next displacement in the chain (0 = end of list)
    // type field: instruction type
    //
    // A next value of null (0) indicates the end of a chain (note that there can
    // be no displacement at position zero, because there is always at least one
    // instruction byte before the displacement).
    //
    // Displacement _data field layout
    //
    // |31.....2|1......0|
    // [  next  |  type  |

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Displacement {
        data_: i32,
    }

    impl Displacement {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Type {
            UnconditionalJump,
            CodeRelative,
            Other,
            CodeAbsolute,
        }

        pub fn data(&self) -> i32 {
            self.data_
        }
        pub fn type_(&self) -> Type {
            Self::TypeField::decode(self.data_)
        }
        pub fn next(&self, l: &mut Label) {
            let n = Self::NextField::decode(self.data_);
            if n > 0 {
                l.link_to(n);
            } else {
                l.unuse();
            }
        }
        pub fn link_to(&self, l: &mut Label) {
            self.init(l, self.type_());
        }

        pub fn new_i32(data: i32) -> Self {
            Displacement { data_: data }
        }

        pub fn new_label_type(l: &mut Label, type_: Type) -> Self {
            let mut disp = Displacement { data_: 0 };
            disp.init(l, type_);
            disp
        }

        pub fn print(&self) {
            println!(
                "{} ({:#x}) ",
                match self.type_() {
                    Type::UnconditionalJump => "jmp",
                    _ => "[other]",
                },
                Self::NextField::decode(self.data_)
            );
        }

        fn init(&mut self, l: &mut Label, type_: Type) {
            let next = if l.is_linked() { l.pos() } else { 0 };
            self.data_ = Self::TypeField::encode(type_) | Self::NextField::encode(next);
        }

        bitfield! {
            #[derive(Copy, Clone, PartialEq, Eq)]
            struct TypeField(i32);
            impl Debug;
            Type, set_type: 1, 0;
        }

        bitfield! {
            #[derive(Copy, Clone, PartialEq, Eq)]
            struct NextField(i32);
            impl Debug;
            next, set_next: 31, 2;
        }
    }

    #[derive(Debug)]
    pub struct Assembler {
        assembler_base: AssemblerBase,
        reloc_info_writer: RelocInfoWriter,
        internal_reference_positions_: Vec<i32>,
    }

    impl Assembler {
        const K_GAP: usize = 32;

        pub fn new(_options: &AssemblerOptions, buffer: Option<Box<AssemblerBuffer>>) -> Self {
            // If the provided buffer is nullptr, the assembler allocates and grows its
            // own buffer. Otherwise it takes ownership of the provided buffer.
            let assembler_base = AssemblerBase::new(buffer);

            Assembler {
                assembler_base,
                reloc_info_writer: RelocInfoWriter::new(),
                internal_reference_positions_: Vec::new(),
            }
        }

        // For compatibility with assemblers that require a zone.
        pub fn new_with_zone(
            _zone: &MaybeAssemblerZone,
            options: &AssemblerOptions,
            buffer: Option<Box<AssemblerBuffer>>,
        ) -> Self {
            Assembler::new(options, buffer)
        }

        // GetCode emits any pending (non-emitted) code and fills the descriptor desc.
        pub const K_NO_HANDLER_TABLE: i32 = 0;
        pub const K_NO_SAFEPOINT_TABLE: *mut SafepointTableBuilder = ptr::null_mut();

        pub fn get_code(
            &mut self,
            _isolate: &mut LocalIsolate,
            _desc: &mut CodeDesc,
            _safepoint_table_builder: *mut SafepointTableBuilder,
            _handler_table_offset: i32,
        ) {
            unimplemented!()
        }

        // Convenience wrapper for allocating with an Isolate.
        pub fn get_code_isolate(&mut self, _isolate: &mut Isolate, _desc: &mut CodeDesc) {
            unimplemented!()
        }

        // Convenience wrapper for code without safepoint or handler tables.
        pub fn get_code_local_isolate(&mut self, _isolate: &mut LocalIsolate, _desc: &mut CodeDesc) {
            self.get_code(
                _isolate,
                _desc,
                Assembler::K_NO_SAFEPOINT_TABLE,
                Assembler::K_NO_HANDLER_TABLE,
            );
        }

        pub fn finalize_jump_optimization_info(&mut self) {
            unimplemented!()
        }

        // Unused on this architecture.
        pub fn maybe_emit_out_of_line_constant_pool(&mut self) {}

        // Read/Modify the code target in the branch/call instruction at pc.
        // The isolate argument is unused (and may be nullptr) when skipping flushing.
        #[inline]
        pub fn target_address_at(_pc: Address, _constant_pool: Address) -> Address {
            unimplemented!()
        }

        #[inline]
        pub fn set_target_address_at(
            _pc: Address,
            _constant_pool: Address,
            _target: Address,
            _jit_allocation: &mut WritableJitAllocation,
            _icache_flush_mode: ICacheFlushMode,
        ) {
            unimplemented!()
        }

        // Get the size of the special target encoded at 'instruction_payload'.
        #[inline]
        pub fn deserialization_special_target_size(_instruction_payload: Address) -> i32 {
            unimplemented!()
        }

        // This sets the internal reference at the pc.
        #[inline]
        pub fn deserialization_set_target_internal_reference_at(
            _pc: Address,
            _target: Address,
            _jit_allocation: &mut WritableJitAllocation,
            _mode: RelocInfoMode,
        ) {
            unimplemented!()
        }

        // Read/modify the uint32 constant used at pc.
        pub fn uint32_constant_at(_pc: Address, _constant_pool: Address) -> u32 {
            unimplemented!()
        }

        pub fn set_uint32_constant_at(
            _pc: Address,
            _constant_pool: Address,
            _new_constant: u32,
            _jit_allocation: &mut WritableJitAllocation,
            _icache_flush_mode: ICacheFlushMode,
        ) {
            unimplemented!()
        }

        pub const K_SPECIAL_TARGET_SIZE: usize = K_SYSTEM_POINTER_SIZE;

        // One byte opcode for test al, 0xXX.
        pub const K_TEST_AL_BYTE: u8 = 0xA8;
        // One byte opcode for nop.
        pub const K_NOP_BYTE: u8 = 0x90;

        // One byte opcode for a short unconditional jump.
        pub const K_JMP_SHORT_OPCODE: u8 = 0xEB;
        // One byte prefix for a short conditional jump.
        pub const K_JCC_SHORT_PREFIX: u8 = 0x70;
        pub const K_JNC_SHORT_OPCODE: u8 = Assembler::K_JCC_SHORT_PREFIX | Condition::NotCarry as u8;
        pub const K_JC_SHORT_OPCODE: u8 = Assembler::K_JCC_SHORT_PREFIX | Condition::Carry as u8;
        pub const K_JNZ_SHORT_OPCODE: u8 = Assembler::K_JCC_SHORT_PREFIX | Condition::NotZero as u8;
        pub const K_JZ_SHORT_OPCODE: u8 = Assembler::K_JCC_SHORT_PREFIX | Condition::Zero as u8;

        // ---------------------------------------------------------------------------
        // InstructionStream generation
        //
        // - function names correspond one-to-one to ia32 instruction mnemonics
        // - unless specified otherwise, instructions operate on 32bit operands
        // - instructions on 8bit (byte) operands/registers have a trailing '_b'
        // - instructions on 16bit (word) operands/registers have a trailing '_w'
        // - naming conflicts with C++ keywords are resolved via a trailing '_'

        // NOTE ON INTERFACE: Currently, the interface is not very consistent
        // in the sense that some operations (e.g. mov()) can be called in more
        // the one way to generate the same instruction: The Register argument
        // can in some cases be replaced with an Operand(Register) argument.
        // This should be cleaned up and made more orthogonal. The questions
        // is: should we always use Operands instead of Registers where an
        // Operand is possible, or should we have a Register (overloaded) form
        // instead? We must be careful to make sure that the selected instruction
        // is obvious from the parameters to avoid hard-to-find code generation
        // bugs.

        // Insert the smallest number of nop instructions
        // possible to align the pc offset to a multiple
        // of m. m must be a power of 2.
        pub fn align(&mut self, _m: i32) {
            unimplemented!()
        }
        // Insert the smallest number of zero bytes possible to align the pc offset
        // to a mulitple of m. m must be a power of 2 (>= 2).
        pub fn data_align(&mut self, _m: i32) {
            unimplemented!()
        }
        pub fn nop(&mut self, _bytes: i32) {
            unimplemented!()
        }
        // Aligns code to something that's optimal for a jump target for the platform.
        pub fn code_target_align(&mut self) {
            unimplemented!()
        }
        pub fn loop_header_align(&mut self) {
            self.code_target_align()
        }

        // Stack
        pub fn pushad(&mut self) {
            unimplemented!()
        }
        pub fn popad(&mut self) {
            unimplemented!()
        }

        pub fn pushfd(&mut self) {
            unimplemented!()
        }
        pub fn popfd(&mut self) {
            unimplemented!()
        }

        pub fn push_imm(&mut self, _x: &Immediate) {
            unimplemented!()
        }

        pub fn push_imm32(&mut self, _imm32: i32) {
            unimplemented!()
        }
        pub fn push_reg(&mut self, _src: Register) {
            unimplemented!()
        }
        pub fn push_op(&mut self, _src: Operand) {
            unimplemented!()
        }

        pub fn pop_reg(&mut self, _dst: Register) {
            unimplemented!()
        }
        pub fn pop_op(&mut self, _dst: Operand) {
            unimplemented!()
        }

        pub fn leave(&mut self) {
            unimplemented!()
        }

        // Moves
        pub fn mov_b_reg_reg(&mut self, dst: Register, src: Register) {
            self.mov_b_reg_op(dst, Operand::new_reg(src))
        }

        pub fn mov_b_reg_op(&mut self, _dst: Register, _src: Operand) {
            unimplemented!()
        }
        pub fn mov_b_reg_imm8(&mut self, dst: Register, imm8: i8) {
            self.mov_b_op_imm8(Operand::new_reg(dst), imm8)
        }
        pub fn mov_b_op_imm8(&mut self, dst: Operand, src: i8) {
            self.mov_b_op_imm(dst, Immediate::new_int(src as i32, RelocInfoMode::NoInfo))
        }
        pub fn mov_b_op_imm(&mut self, _dst: Operand, _src: &Immediate) {
            unimplemented!()
        }
        pub fn mov_b_op_reg(&mut self, _dst: Operand, _src: Register) {
            unimplemented!()
        }

        pub fn mov_w_reg_op(&mut self, _dst: Register, _src: Operand) {
            unimplemented!()
        }
        pub fn mov_w_reg_imm16(&mut self, _dst: Register, _src: i16) {
            unimplemented!()
        }
        pub fn mov_w_reg_imm(&mut self, _dst: Register, _src: &Immediate) {
            unimplemented!()
        }
        pub fn mov_w_reg_reg(&mut self, _dst: Register, _src: Register) {
            unimplemented!()
        }

        pub fn mov_reg_imm32(&mut self, _dst: Register, _imm32: i32) {
            unimplemented!()
        }
        pub fn mov_reg_imm(&mut self, _dst: Register, _x: &Immediate) {
            unimplemented!()
        }
        pub fn mov_reg_handle(&mut self, _dst: Register, _handle: HeapObject) {
            unimplemented!()
        }
        pub fn mov_reg_op(&mut self, _dst: Register, _src: Operand) {
            unimplemented!()
        }
        pub fn mov_reg_reg(&mut self, _dst: Register, _src: Register) {
            unimplemented!()
        }
        pub fn mov_op_imm(&mut self, _dst: Operand, _x: &Immediate) {
            unimplemented!()
        }
        pub fn mov_op_handle(&mut self, _dst: Operand, _handle: HeapObject) {
            unimplemented!()
        }
        pub fn mov_op_reg(&mut self, _dst: Operand, _src: Register) {
            unimplemented!()
        }
        pub fn mov_op_address(&mut self, _dst: Operand, _src: Address, _mode: RelocInfoMode) {
            unimplemented!()
        }

        pub fn movsx_b_reg_reg(&mut self, dst: Register, src: Register) {
            self.movsx_b_reg_op(dst, Operand::new_reg(src))
        }

        pub fn movsx_b_reg_op(&mut self, _dst: Register