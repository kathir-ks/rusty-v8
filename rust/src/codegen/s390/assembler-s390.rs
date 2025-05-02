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
// Copyright 2014 the V8 project authors. All rights reserved.

// A light-weight S390 Assembler
// Generates user mode instructions for z/Architecture

pub mod s390 {
    use std::mem;
    //use elf; // Requires external crate. Need to decide on one and add as a dependency.
    use std::fs::File;
    use std::io::Read;
    use std::os::unix::io::AsRawFd;
    use std::ptr::null_mut;

    use crate::base::platform::platform::*;
    use crate::codegen::assembler::*;
    use crate::codegen::external_reference::*;
    use crate::codegen::label::*;
    use crate::codegen::s390::constants_s390::*;
    use crate::codegen::s390::register_s390::*;
    use crate::objects::smi::*;

    // Placeholder for LocalIsolate, CodeDesc, SafepointTableBuilderBase
    pub struct LocalIsolate {}
    pub struct CodeDesc {}
    pub struct SafepointTableBuilderBase {}
    pub struct AssemblerOptions {}
    pub struct AssemblerBuffer {}
    pub struct MaybeAssemblerZone {}
    pub struct WritableJitAllocation {}
    pub struct Instruction {}

    impl Instruction {
        pub fn InstructionBits(pc: *const u8) -> SixByteInstr {
            //Implement
            SixByteInstr(0)
        }

        pub fn SetInstructionBits<T>(pc: *mut u8, instr: T) {
            //Implement
        }

        pub fn InstructionLength(pc: *const u8) -> i32 {
            //Implement
            0
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Operand {
        rmode_: RelocInfoMode,
        value_: Value,
        is_heap_number_request_: bool,
        rm_: Register,
    }

    impl Operand {
        pub fn new(immediate: i64, rmode: RelocInfoMode) -> Self {
            Operand {
                rmode_: rmode,
                value_: Value { immediate: immediate },
                is_heap_number_request_: false,
                rm_: no_reg,
            }
        }

        pub fn Zero() -> Self {
            Operand::new(0, RelocInfoMode::NO_INFO)
        }

        pub fn from_external_reference(f: &ExternalReference) -> Self {
            Operand {
                rmode_: RelocInfoMode::EXTERNAL_REFERENCE,
                value_: Value {
                    immediate: f.address() as i64,
                },
                is_heap_number_request_: false,
                rm_: no_reg,
            }
        }

        pub fn from_heap_object_handle(handle: &HeapObjectHandle) -> Self {
            //Implement
            Operand::Zero()
        }

        pub fn from_tagged_smi(value: TaggedSmi) -> Self {
            Operand {
                rmode_: RelocInfoMode::NO_INFO,
                value_: Value {
                    immediate: value.ptr() as i64,
                },
                is_heap_number_request_: false,
                rm_: no_reg,
            }
        }

        pub fn from_register(rm: Register) -> Self {
            Operand {
                rmode_: RelocInfoMode::NO_INFO,
                value_: Value { immediate: 0 },
                is_heap_number_request_: false,
                rm_: rm,
            }
        }

        pub fn EmbeddedNumber(value: f64) -> Self {
            //Implement
            Operand::Zero()
        }

        pub fn is_reg(&self) -> bool {
            self.rm_.is_valid()
        }

        pub fn must_output_reloc_info(&self, assembler: &Assembler) -> bool {
            //Implement
            false
        }

        pub fn immediate(&self) -> i64 {
            assert!(!self.rm_.is_valid());
            assert!(!self.is_heap_number_request());
            self.value_.immediate
        }

        pub fn heap_number_request(&self) -> HeapNumberRequest {
            assert!(self.is_heap_number_request());
            self.value_.heap_number_request
        }

        pub fn setBits(&mut self, n: i32) {
            let mask = (1u32 << n) - 1;
            self.value_.immediate =
                ((self.value_.immediate as u32) & mask as u32) as i64;
        }

        pub fn rm(&self) -> Register {
            self.rm_
        }

        pub fn is_heap_number_request(&self) -> bool {
            if self.is_heap_number_request_ {
                assert!(!self.rm_.is_valid());
                assert!(
                    self.rmode_ == RelocInfoMode::FULL_EMBEDDED_OBJECT
                        || self.rmode_ == RelocInfoMode::CODE_TARGET
                );
            }
            self.is_heap_number_request_
        }

        pub fn rmode(&self) -> RelocInfoMode {
            self.rmode_
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    union Value {
        heap_number_request: HeapNumberRequest,
        immediate: i64,
    }

    impl Value {
        const fn new() -> Self {
            Value { immediate: 0 }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct HeapNumberRequest {}

    pub type Disp = i32;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MemOperand {
        baseRegister: Register,
        indexRegister: Register,
        offset_: i32,
    }

    impl MemOperand {
        pub fn new(rx: Register, offset: Disp) -> Self {
            MemOperand {
                baseRegister: rx,
                indexRegister: no_reg,
                offset_: offset,
            }
        }

        pub fn new_indexed(rx: Register, rb: Register, offset: Disp) -> Self {
            MemOperand {
                baseRegister: rb,
                indexRegister: rx,
                offset_: offset,
            }
        }

        pub fn offset(&self) -> i32 {
            self.offset_
        }
        pub fn getDisplacement(&self) -> u32 {
            self.offset() as u32
        }

        pub fn rb(&self) -> Register {
            assert!(self.baseRegister != no_reg);
            self.baseRegister
        }

        pub fn getBaseRegister(&self) -> Register {
            self.rb()
        }

        pub fn rx(&self) -> Register {
            assert!(self.indexRegister != no_reg);
            self.indexRegister
        }
        pub fn getIndexRegister(&self) -> Register {
            self.rx()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct DeferredRelocInfo {
        position_: i32,
        rmode_: RelocInfoMode,
        data_: i64,
    }

    impl DeferredRelocInfo {
        pub fn new() -> Self {
            DeferredRelocInfo {
                position_: 0,
                rmode_: RelocInfoMode::NO_INFO,
                data_: 0,
            }
        }
        pub fn new_with_params(position: i32, rmode: RelocInfoMode, data: i64) -> Self {
            DeferredRelocInfo {
                position_: position,
                rmode_: rmode,
                data_: data,
            }
        }

        pub fn position(&self) -> i32 {
            self.position_
        }
        pub fn rmode(&self) -> RelocInfoMode {
            self.rmode_
        }
        pub fn data(&self) -> i64 {
            self.data_
        }
    }

    #[derive(Debug)]
    pub struct Assembler {
        base: AssemblerBase,
        reloc_info_writer: RelocInfoWriter,
        relocations_: Vec<DeferredRelocInfo>,
        scratch_register_list_: RegList,
        scratch_double_register_list_: DoubleRegList,
        last_bound_pos_: i32,
    }

    impl Assembler {
        const K_NO_HANDLER_TABLE: i32 = 0;
        const K_NO_SAFEPOINT_TABLE: *mut SafepointTableBuilderBase =
            std::ptr::null_mut();
        const K_GAP: i32 = 32;
        const K_MAXIMAL_BUFFER_SIZE: i32 = 512 * 1024 * 1024; //512 * MB;
        const K_MAX_RELOC_SIZE: i32 = RelocInfoWriter::K_MAX_SIZE;

        pub fn new(options: &AssemblerOptions, buffer: Option<Box<AssemblerBuffer>>) -> Self {
            let mut assembler = Assembler {
                base: AssemblerBase::new(buffer),
                reloc_info_writer: RelocInfoWriter::new(),
                relocations_: Vec::new(),
                scratch_register_list_: Self::DefaultTmpList(),
                scratch_double_register_list_: Self::DefaultFPTmpList(),
                last_bound_pos_: 0,
            };
            assert!(AssemblerBase::K_MINIMAL_BUFFER_SIZE >= 2 * Self::K_GAP);

            assembler
        }

        pub fn new_with_zone(
            _zone: &MaybeAssemblerZone,
            options: &AssemblerOptions,
            buffer: Option<Box<AssemblerBuffer>>,
        ) -> Self {
            Assembler::new(options, buffer)
        }

        pub fn DefaultTmpList() -> RegList {
            RegList::new()
            //Implement DefaultTmpList
        }

        pub fn DefaultFPTmpList() -> DoubleRegList {
            DoubleRegList::new()
            //Implement DefaultFPTmpList
        }

        pub fn GetCode(
            &mut self,
            isolate: *mut LocalIsolate,
            desc: *mut CodeDesc,
            safepoint_table_builder: *mut SafepointTableBuilderBase,
            handler_table_offset: i32,
        ) {
            //Implement GetCode
        }

        pub fn GetCode_isolate(&mut self, isolate: *mut LocalIsolate, desc: *mut CodeDesc) {
            self.GetCode(
                isolate,
                desc,
                Self::K_NO_SAFEPOINT_TABLE,
                Self::K_NO_HANDLER_TABLE,
            )
        }

        pub fn MaybeEmitOutOfLineConstantPool(&mut self) {}

        pub fn bind(&mut self, l: &mut Label) {
            //Implement bind
            self.bind_to(l, self.pc_offset());
        }

        pub fn link(&mut self, l: &mut Label) -> i32 {
            //Implement link
            if l.is_linked() {
                return l.pos();
            }
            if l.is_bound() {
                return l.pos();
            }
            self.next(l);
            l.pos()
        }

        pub fn branch_offset(&mut self, l: &mut Label) -> i32 {
            self.link(l) - self.pc_offset()
        }

        pub fn load_label_offset(&mut self, r1: Register, l: &mut Label) {
            //Implement load_label_offset
        }

        pub fn target_address_at(pc: Address, constant_pool: Address) -> Address {
            //Implement target_address_at
            pc
        }

        pub fn target_compressed_address_at(pc: Address, constant_pool: Address) -> Tagged_t {
            //Implement
            Tagged_t(0)
        }

        pub fn set_target_address_at(
            pc: Address,
            constant_pool: Address,
            target: Address,
            jit_allocation: *mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            //Implement set_target_address_at
        }

        pub fn set_target_compressed_address_at(
            pc: Address,
            constant_pool: Address,
            target: Tagged_t,
            jit_allocation: *mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            //Implement
        }

        pub fn code_target_object_handle_at(&self, pc: Address) -> Handle<Object> {
            //Implement
            Handle {
                location_: std::ptr::null_mut(),
            }
        }

        pub fn compressed_embedded_object_handle_at(
            &self,
            pc: Address,
            constant_pool: Address,
        ) -> Handle<HeapObject> {
            //Implement
            Handle {
                location_: std::ptr::null_mut(),
            }
        }

        pub fn deserialization_special_target_size(instruction_payload: Address) -> i32 {
            0
        }

        pub fn deserialization_set_target_internal_reference_at(
            pc: Address,
            target: Address,
            jit_allocation: &mut WritableJitAllocation,
            mode: RelocInfoMode,
        ) {
            //Implement
        }

        pub fn uint32_constant_at(pc: Address, constant_pool: Address) -> u32 {
            //Implement
            0
        }

        pub fn set_uint32_constant_at(
            pc: Address,
            constant_pool: Address,
            new_constant: u32,
            jit_allocation: *mut WritableJitAllocation,
            icache_flush_mode: ICacheFlushMode,
        ) {
            //Implement
        }

        pub fn GetScratchRegisterList(&mut self) -> &mut RegList {
            &mut self.scratch_register_list_
        }

        pub fn GetScratchDoubleRegisterList(&mut self) -> &mut DoubleRegList {
            &mut self.scratch_double_register_list_
        }

        pub fn getfield<T: Copy, const SIZE: usize, const LO: usize, const HI: usize>(
            value: T,
        ) -> T {
            assert!(LO < HI);
            assert!(SIZE > 0);
            let mask = HI - LO;
            let shift = SIZE * 8 - HI;
            let mask_value = if mask == 32 {
                0xffffffff
            } else {
                (1 << mask) - 1
            };
            //Rust cant do bitwise ops on generic types so have to coerce to and from u32
            let v = unsafe { mem::transmute::<T, u32>(value) };
            let result = (v & mask_value) << shift;

            unsafe { mem::transmute::<u32, T>(result) }
        }

        fn ril_format(&mut self, opcode: Opcode, f1: i32, f2: i64) {
            let op1 = (opcode as u32) >> 4;
            let op2 = (opcode as u32) & 0xf;
            let val: u64 = self.getfield::<u64, 6, 0, 8>(op1 as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(op2 as u64)
                | self.getfield::<u64, 6, 16, 48>(f2 as u64);

            self.emit6bytes(val);
        }

        fn rr_format(&mut self, opcode: Opcode, f1: i32, f2: i32) {
            let val: u16 = self.getfield::<u16, 2, 0, 8>(opcode as u16)
                | self.getfield::<u16, 2, 8, 12>(f1 as u16)
                | self.getfield::<u16, 2, 12, 16>(f2 as u16);

            self.emit2bytes(val);
        }

        fn rrd_format(&mut self, opcode: Opcode, f1: i32, f2: i32, f3: i32) {
            let val: u32 = self.getfield::<u32, 4, 0, 16>(opcode as u32)
                | self.getfield::<u32, 4, 16, 20>(f1 as u32)
                | self.getfield::<u32, 4, 24, 28>(f2 as u32)
                | self.getfield::<u32, 4, 28, 32>(f3 as u32);
            self.emit4bytes(val);
        }

        fn rre_format(&mut self, opcode: Opcode, f1: i32, f2: i32) {
            let val: u32 = self.getfield::<u32, 4, 0, 16>(opcode as u32)
                | self.getfield::<u32, 4, 24, 28>(f1 as u32)
                | self.getfield::<u32, 4, 28, 32>(f2 as u32);
            self.emit4bytes(val);
        }

        fn rx_format(&mut self, opcode: Opcode, f1: i32, f2: i32, f3: i32, f4: i32) {
            assert!(is_uint8(opcode as i32));
            assert!(is_uint12(f4));

            let val: u32 = self.getfield::<u32, 4, 0, 8>(opcode as u32)
                | self.getfield::<u32, 4, 8, 12>(f1 as u32)
                | self.getfield::<u32, 4, 12, 16>(f2 as u32)
                | self.getfield::<u32, 4, 16, 20>(f3 as u32)
                | self.getfield::<u32, 4, 20, 32>(f4 as u32);
            self.emit4bytes(val);
        }

        fn rxy_format(&mut self, opcode: Opcode, f1: i32, f2: i32, f3: i32, f4: i32) {
            assert!(is_uint16(opcode as i32));
            assert!(is_int20(f4));

            let val: u64 = self.getfield::<u64, 6, 0, 8>((opcode as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(f2 as u64)
                | self.getfield::<u64, 6, 16, 20>(f3 as u64)
                | self.getfield::<u64, 6, 20, 32>((f4 & 0x0fff) as u64)
                | self.getfield::<u64, 6, 32, 40>((f4 >> 12) as u64)
                | self.getfield::<u64, 6, 40, 48>((opcode as u32 & 0x00ff) as u64);

            self.emit6bytes(val);
        }

        fn rsy_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32) {
            assert!(is_int20(f4));
            assert!(is_uint16(op as i32));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(f2 as u64)
                | self.getfield::<u64, 6, 16, 20>(f3 as u64)
                | self.getfield::<u64, 6, 20, 32>((f4 & 0x0fff) as u64)
                | self.getfield::<u64, 6, 32, 40>((f4 >> 12) as u64)
                | self.getfield::<u64, 6, 40, 48>((op as u32 & 0xff) as u64);
            self.emit6bytes(code);
        }

        fn rs_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32) {
            let code: u32 = self.getfield::<u32, 4, 0, 8>(op as u32)
                | self.getfield::<u32, 4, 8, 12>(f1 as u32)
                | self.getfield::<u32, 4, 12, 16>(f2 as u32)
                | self.getfield::<u32, 4, 16, 20>(f3 as u32)
                | self.getfield::<u32, 4, 20, 32>(f4 as u32);
            self.emit4bytes(code);
        }

        fn rxe_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32, f5: i32) {
            assert!(is_uint12(f4));
            assert!(is_uint16(op as i32));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(f2 as u64)
                | self.getfield::<u64, 6, 16, 20>(f3 as u64)
                | self.getfield::<u64, 6, 20, 32>((f4 & 0x0fff) as u64)
                | self.getfield::<u64, 6, 32, 36>(f5 as u64)
                | self.getfield::<u64, 6, 40, 48>((op as u32 & 0xff) as u64);

            self.emit6bytes(code);
        }

        fn ri_format(&mut self, opcode: Opcode, f1: i32, f2: i32) {
            let op1 = (opcode as u32) >> 4;
            let op2 = (opcode as u32) & 0xf;

            let val: u32 = self.getfield::<u32, 4, 0, 8>(op1 as u32)
                | self.getfield::<u32, 4, 8, 12>(f1 as u32)
                | self.getfield::<u32, 4, 12, 16>(op2 as u32)
                | self.getfield::<u32, 4, 16, 32>(f2 as u32);
            self.emit4bytes(val);
        }

        fn rrf_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32) {
            let code: u32 = self.getfield::<u32, 4, 0, 16>(op as u32)
                | self.getfield::<u32, 4, 16, 20>(f1 as u32)
                | self.getfield::<u32, 4, 20, 24>(f2 as u32)
                | self.getfield::<u32, 4, 24, 28>(f3 as u32)
                | self.getfield::<u32, 4, 28, 32>(f4 as u32);
            self.emit4bytes(code);
        }

        fn rsi_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32) {
            assert!(is_uint8(op as i32));
            assert!(is_uint16(f3) || is_int16(f3));

            let code: u32 = self.getfield::<u32, 4, 0, 8>(op as u32)
                | self.getfield::<u32, 4, 8, 12>(f1 as u32)
                | self.getfield::<u32, 4, 12, 16>(f2 as u32)
                | self.getfield::<u32, 4, 16, 32>(f3 as u32);
            self.emit4bytes(code);
        }

        fn rsl_format(&mut self, op: Opcode, f1: u16, f2: i32, f3: i32, f4: i32, f5: i32) {
            assert!(is_uint16(op as i32));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 16>(f1 as u64)
                | self.getfield::<u64, 6, 16, 20>(f2 as u64)
                | self.getfield::<u64, 6, 20, 32>(f3 as u64)
                | self.getfield::<u64, 6, 32, 36>(f4 as u64)
                | self.getfield::<u64, 6, 36, 40>(f5 as u64)
                | self.getfield::<u64, 6, 40, 48>((op as u32 & 0x00FF) as u64);
            self.emit6bytes(code);
        }

        fn s_format(&mut self, op: Opcode, f1: i32, f2: i32) {
            assert_ne!((op as u32 & 0xff00), 0);
            assert!(is_uint12(f2));

            let code: u32 = self.getfield::<u32, 4, 0, 16>(op as u32)
                | self.getfield::<u32, 4, 16, 20>(f1 as u32)
                | self.getfield::<u32, 4, 20, 32>(f2 as u32);
            self.emit4bytes(code);
        }

        fn si_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32) {
            let code: u32 = self.getfield::<u32, 4, 0, 8>(op as u32)
                | self.getfield::<u32, 4, 8, 16>(f1 as u32)
                | self.getfield::<u32, 4, 16, 20>(f2 as u32)
                | self.getfield::<u32, 4, 20, 32>(f3 as u32);
            self.emit4bytes(code);
        }

        fn siy_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32) {
            assert!(is_uint20(f3) || is_int20(f3));
            assert!(is_uint16(op as i32));
            assert!(is_uint8(f1) || is_int8(f1));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 16>(f1 as u64)
                | self.getfield::<u64, 6, 16, 20>(f2 as u64)
                | self.getfield::<u64, 6, 20, 32>(f3 as u64)
                | self.getfield::<u64, 6, 32, 40>((f3 >> 12) as u64)
                | self.getfield::<u64, 6, 40, 48>((op as u32 & 0x00FF) as u64);

            self.emit6bytes(code);
        }

        fn rrs_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32, f5: i32) {
            assert!(is_uint12(f4));
            assert!(is_uint16(op as i32));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(f2 as u64)
                | self.getfield::<u64, 6, 16, 20>(f3 as u64)
                | self.getfield::<u64, 6, 20, 32>(f4 as u64)
                | self.getfield::<u64, 6, 32, 36>(f5 as u64)
                | self.getfield::<u64, 6, 40, 48>((op as u32 & 0x00FF) as u64);
            self.emit6bytes(code);
        }

        fn ris_format(&mut self, op: Opcode, f1: i32, f2: i32, f3: i32, f4: i32, f5: i32) {
            assert!(is_uint12(f3));
            assert!(is_uint16(op as i32));
            assert!(is_uint8(f5));

            let code: u64 = self.getfield::<u64, 6, 0, 8>((op as u32 >> 8) as u64)
                | self.getfield::<u64, 6, 8, 12>(f1 as u64)
                | self.getfield::<u64, 6, 12, 16>(f2 as u64)
                | self.getfield::<u64, 6, 16, 20>(f3 as u64)
                | self.