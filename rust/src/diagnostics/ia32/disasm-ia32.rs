// TODO: Reimplement functionality from "src/base/compiler-specific.h"
// TODO: Reimplement functionality from "src/base/strings.h"
// TODO: Reimplement functionality from "src/codegen/ia32/fma-instr.h"
// TODO: Reimplement functionality from "src/codegen/ia32/sse-instr.h"
// TODO: Reimplement functionality from "src/diagnostics/disasm.h"

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

//use std::ffi::CString;
//use std::os::raw::c_char;

const V8_TARGET_ARCH_IA32: bool = true;

#[cfg(all(target_arch = "x86", V8_TARGET_ARCH_IA32))]
pub mod disasm_ia32 {
    use std::fmt;
    use std::mem::transmute;
    //use std::os::raw::c_char;

    #[derive(Clone, Copy)]
    enum OperandOrder {
        UNSET_OP_ORDER = 0,
        REG_OPER_OP_ORDER,
        OPER_REG_OP_ORDER,
    }

    struct ByteMnemonic {
        b: i32, // -1 terminates, otherwise must be in range (0..255)
        mnem: &'static str,
        op_order_: OperandOrder,
    }

    const TWO_OPERANDS_INSTR: [ByteMnemonic; 17] = [
        ByteMnemonic {b: 0x01, mnem: "add", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x03, mnem: "add", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x09, mnem: "or", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x0B, mnem: "or", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x13, mnem: "adc", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x1B, mnem: "sbb", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x21, mnem: "and", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x23, mnem: "and", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x29, mnem: "sub", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x2A, mnem: "subb", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x2B, mnem: "sub", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x31, mnem: "xor", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x33, mnem: "xor", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x38, mnem: "cmpb", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x39, mnem: "cmp", op_order_: OperandOrder::OPER_REG_OP_ORDER},
        ByteMnemonic {b: 0x3A, mnem: "cmpb", op_order_: OperandOrder::REG_OPER_OP_ORDER},
        ByteMnemonic {b: 0x3B, mnem: "cmp", op_order_: OperandOrder::REG_OPER_OP_ORDER},
    ];

    const TWO_OPERANDS_INSTR_END: ByteMnemonic = ByteMnemonic {b: -1, mnem: "", op_order_: OperandOrder::UNSET_OP_ORDER};

    const ZERO_OPERANDS_INSTR: [ByteMnemonic; 14] = [
        ByteMnemonic {b: 0xC3, mnem: "ret", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xC9, mnem: "leave", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x90, mnem: "nop", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xF4, mnem: "hlt", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xCC, mnem: "int3", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x60, mnem: "pushad", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x61, mnem: "popad", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x9C, mnem: "pushfd", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x9D, mnem: "popfd", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x9E, mnem: "sahf", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x99, mnem: "cdq", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x9B, mnem: "fwait", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xFC, mnem: "cld", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xAB, mnem: "stos", op_order_: OperandOrder::UNSET_OP_ORDER},
    ];

    const ZERO_OPERANDS_INSTR_END: ByteMnemonic = ByteMnemonic {b: -1, mnem: "", op_order_: OperandOrder::UNSET_OP_ORDER};

    const CALL_JUMP_INSTR: [ByteMnemonic; 2] = [
        ByteMnemonic {b: 0xE8, mnem: "call", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0xE9, mnem: "jmp", op_order_: OperandOrder::UNSET_OP_ORDER},
    ];

    const CALL_JUMP_INSTR_END: ByteMnemonic = ByteMnemonic {b: -1, mnem: "", op_order_: OperandOrder::UNSET_OP_ORDER};

    const SHORT_IMMEDIATE_INSTR: [ByteMnemonic; 7] = [
        ByteMnemonic {b: 0x05, mnem: "add", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x0D, mnem: "or", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x15, mnem: "adc", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x25, mnem: "and", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x2D, mnem: "sub", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x35, mnem: "xor", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x3D, mnem: "cmp", op_order_: OperandOrder::UNSET_OP_ORDER},
    ];

    const SHORT_IMMEDIATE_INSTR_END: ByteMnemonic = ByteMnemonic {b: -1, mnem: "", op_order_: OperandOrder::UNSET_OP_ORDER};

    // Generally we don't want to generate these because they are subject to partial
    // register stalls.  They are included for completeness and because the cmp
    // variant is used by the RecordWrite stub.  Because it does not update the
    // register it is not subject to partial register stalls.
    const BYTE_IMMEDIATE_INSTR: [ByteMnemonic; 4] = [
        ByteMnemonic {b: 0x0C, mnem: "or", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x24, mnem: "and", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x34, mnem: "xor", op_order_: OperandOrder::UNSET_OP_ORDER},
        ByteMnemonic {b: 0x3C, mnem: "cmp", op_order_: OperandOrder::UNSET_OP_ORDER},
    ];

    const BYTE_IMMEDIATE_INSTR_END: ByteMnemonic = ByteMnemonic {b: -1, mnem: "", op_order_: OperandOrder::UNSET_OP_ORDER};

    const JUMP_CONDITIONAL_MNEM: [&'static str; 16] = [
        /*0*/ "jo", "jno", "jc", "jnc",
        /*4*/ "jz", "jnz", "jna", "ja",
        /*8*/ "js", "jns", "jpe", "jpo",
        /*12*/ "jl", "jnl", "jng", "jg",
    ];

    const SET_CONDITIONAL_MNEM: [&'static str; 16] = [
        /*0*/ "seto", "setno", "setc", "setnc",
        /*4*/ "setz", "setnz", "setna", "seta",
        /*8*/ "sets", "setns", "setpe", "setpo",
        /*12*/ "setl", "setnl", "setng", "setg",
    ];

    const CONDITIONAL_MOVE_MNEM: [&'static str; 16] = [
        /*0*/ "cmovo", "cmovno", "cmovc", "cmovnc",
        /*4*/ "cmovz", "cmovnz", "cmovna", "cmova",
        /*8*/ "cmovs", "cmovns", "cmovpe", "cmovpo",
        /*12*/ "cmovl", "cmovnl", "cmovng", "cmovg",
    ];

    const CMP_PSEUDO_OP: [&'static str; 16] = [
        "eq", "lt", "le", "unord", "neq", "nlt", "nle", "ord",
        "eq_uq", "nge", "ngt", "false", "neq_oq", "ge", "gt", "true",
    ];

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum InstructionType {
        NO_INSTR,
        ZERO_OPERANDS_INSTR,
        TWO_OPERANDS_INSTR,
        JUMP_CONDITIONAL_SHORT_INSTR,
        REGISTER_INSTR,
        MOVE_REG_INSTR,
        CALL_JUMP_INSTR,
        SHORT_IMMEDIATE_INSTR,
        BYTE_IMMEDIATE_INSTR,
    }

    struct InstructionDesc {
        mnem: &'static str,
        type_: InstructionType,
        op_order_: OperandOrder,
    }

    struct InstructionTable {
        instructions_: [InstructionDesc; 256],
    }

    impl InstructionTable {
        fn new() -> InstructionTable {
            let mut table = InstructionTable {
                instructions_: [InstructionDesc {
                    mnem: "",
                    type_: InstructionType::NO_INSTR,
                    op_order_: OperandOrder::UNSET_OP_ORDER,
                }; 256],
            };
            table.clear();
            table.init();
            table
        }

        fn get(&self, x: u8) -> &InstructionDesc {
            &self.instructions_[x as usize]
        }

        fn get_instance() -> &'static InstructionTable {
            static TABLE: InstructionTable = InstructionTable::new();
            &TABLE
        }

        fn clear(&mut self) {
            for i in 0..256 {
                self.instructions_[i].mnem = "";
                self.instructions_[i].type_ = InstructionType::NO_INSTR;
                self.instructions_[i].op_order_ = OperandOrder::UNSET_OP_ORDER;
            }
        }

        fn init(&mut self) {
            self.copy_table(&TWO_OPERANDS_INSTR, InstructionType::TWO_OPERANDS_INSTR);
            self.copy_table(&ZERO_OPERANDS_INSTR, InstructionType::ZERO_OPERANDS_INSTR);
            self.copy_table(&CALL_JUMP_INSTR, InstructionType::CALL_JUMP_INSTR);
            self.copy_table(&SHORT_IMMEDIATE_INSTR, InstructionType::SHORT_IMMEDIATE_INSTR);
            self.copy_table(&BYTE_IMMEDIATE_INSTR, InstructionType::BYTE_IMMEDIATE_INSTR);
            self.add_jump_conditional_short();
            self.set_table_range(InstructionType::REGISTER_INSTR, 0x40, 0x47, "inc");
            self.set_table_range(InstructionType::REGISTER_INSTR, 0x48, 0x4F, "dec");
            self.set_table_range(InstructionType::REGISTER_INSTR, 0x50, 0x57, "push");
            self.set_table_range(InstructionType::REGISTER_INSTR, 0x58, 0x5F, "pop");
            self.set_table_range(InstructionType::REGISTER_INSTR, 0x91, 0x97, "xchg eax,"); // 0x90 is nop.
            self.set_table_range(InstructionType::MOVE_REG_INSTR, 0xB8, 0xBF, "mov");
        }

        fn copy_table(&mut self, bm: &[ByteMnemonic], type_: InstructionType) {
            for i in 0..bm.len() {
                if bm[i].b < 0 {
                    break;
                }

                let b = bm[i].b as usize;
                let id = &mut self.instructions_[b];
                id.mnem = bm[i].mnem;
                id.op_order_ = bm[i].op_order_;
                if id.type_ != InstructionType::NO_INSTR {
                    panic!("Information already entered.");
                }
                id.type_ = type_;
            }
        }

        fn set_table_range(&mut self, type_: InstructionType, start: u8, end: u8, mnem: &'static str) {
            for b in start..=end {
                let id = &mut self.instructions_[b as usize];
                if id.type_ != InstructionType::NO_INSTR {
                    panic!("Information already entered.");
                }
                id.mnem = mnem;
                id.type_ = type_;
            }
        }

        fn add_jump_conditional_short(&mut self) {
            for b in 0x70..=0x7F {
                let id = &mut self.instructions_[b as usize];
                if id.type_ != InstructionType::NO_INSTR {
                    panic!("Information already entered.");
                }
                id.mnem = JUMP_CONDITIONAL_MNEM[(b & 0x0F) as usize];
                id.type_ = InstructionType::JUMP_CONDITIONAL_SHORT_INSTR;
            }
        }
    }

    mod helper_functions {
        pub fn imm8(data: &[u8]) -> i8 {
            data[0] as i8
        }

        pub fn imm8_u(data: &[u8]) -> u8 {
            data[0]
        }

        pub fn imm16(data: &[u8]) -> i16 {
            unsafe { *(data.as_ptr() as *const i16) }
        }

        pub fn imm16_u(data: &[u8]) -> u16 {
            unsafe { *(data.as_ptr() as *const u16) }
        }

        pub fn imm32(data: &[u8]) -> i32 {
            unsafe { *(data.as_ptr() as *const i32) }
        }
    }

    trait NameConverterTrait {
        fn name_of_address(&self, addr: *const u8) -> String;
        fn name_of_constant(&self, addr: *const u8) -> String;
        fn name_of_cpu_register(&self, reg: i32) -> &'static str;
        fn name_of_byte_cpu_register(&self, reg: i32) -> &'static str;
        fn name_of_xmm_register(&self, reg: i32) -> &'static str;
        fn name_in_code(&self, addr: *const u8) -> String;
    }

    struct DefaultNameConverter {}

    impl DefaultNameConverter {
        fn new() -> DefaultNameConverter {
            DefaultNameConverter{}
        }
    }

    impl NameConverterTrait for DefaultNameConverter {
        fn name_of_address(&self, addr: *const u8) -> String {
            format!("{:p}", addr)
        }

        fn name_of_constant(&self, addr: *const u8) -> String {
            self.name_of_address(addr)
        }

        fn name_of_cpu_register(&self, reg: i32) -> &'static str {
            if (0..8).contains(&reg) {
                CPU_REGS[reg as usize]
            } else {
                "noreg"
            }
        }

        fn name_of_byte_cpu_register(&self, reg: i32) -> &'static str {
            if (0..8).contains(&reg) {
                BYTE_CPU_REGS[reg as usize]
            } else {
                "noreg"
            }
        }

        fn name_of_xmm_register(&self, reg: i32) -> &'static str {
            if (0..8).contains(&reg) {
                XMM_REGS[reg as usize]
            } else {
                "noxmmreg"
            }
        }

        fn name_in_code(&self, addr: *const u8) -> String {
            unreachable!() // IA32 does not embed debug strings
        }
    }

    // The IA32 disassembler implementation.
    pub struct DisassemblerIA32<'a> {
        converter_: &'a dyn NameConverterTrait,
        vex_byte0_: u8,     // 0xC4 or 0xC5
        vex_byte1_: u8,
        vex_byte2_: u8,     // only for 3 bytes vex prefix
        instruction_table_: &'static InstructionTable,
        tmp_buffer_: [u8; 128],
        tmp_buffer_pos_: usize,
        unimplemented_opcode_action_: UnimplementedOpcodeAction,
    }

    #[derive(PartialEq)]
    pub enum UnimplementedOpcodeAction {
        AbortOnUnimplementedOpcode,
        Continue,
    }

    impl<'a> DisassemblerIA32<'a> {
        pub fn new(
            converter: &'a dyn NameConverterTrait,
            unimplemented_opcode_action: UnimplementedOpcodeAction,
        ) -> DisassemblerIA32<'a> {
            DisassemblerIA32 {
                converter_: converter,
                vex_byte0_: 0,
                vex_byte1_: 0,
                vex_byte2_: 0,
                instruction_table_: InstructionTable::get_instance(),
                tmp_buffer_: [0; 128],
                tmp_buffer_pos_: 0,
                unimplemented_opcode_action_: unimplemented_opcode_action,
            }
        }

        // Writes one disassembled instruction into 'buffer' (0-terminated).
        // Returns the length of the disassembled machine instruction in bytes.
        pub fn instruction_decode(
            &mut self,
            out_buffer: &mut [u8],
            instruction: *mut u8,
        ) -> i32 {
            unsafe {
                self.tmp_buffer_pos_ = 0; // starting to write as position 0
                let mut data = instruction;
                // Check for hints.
                let mut branch_hint: Option<&str> = None;
                // We use these two prefixes only with branch prediction
                if *data == 0x3E {
                    //ds
                    branch_hint = Some("predicted taken");
                    data = data.offset(1);
                } else if *data == 0x2E {
                    //cs
                    branch_hint = Some("predicted not taken");
                    data = data.offset(1);
                } else if *data == 0xC4 && *data.offset(1) >= 0xC0 {
                    self.vex_byte0_ = *data;
                    self.vex_byte1_ = *data.offset(1);
                    self.vex_byte2_ = *data.offset(2);
                    data = data.offset(3);
                } else if *data == 0xC5 && *data.offset(1) >= 0xC0 {
                    self.vex_byte0_ = *data;
                    self.vex_byte1_ = *data.offset(1);
                    data = data.offset(2);
                } else if *data == 0xF0 {
                    //lock
                    self.append_to_buffer("lock ");
                    data = data.offset(1);
                }

                let mut processed = true; // Will be set to false if the current instruction
                                            // is not in 'instructions' table.
                                            // Decode AVX instructions.
                if self.vex_byte0_ != 0 {
                    data = data.offset(self.avx_instruction(data) as isize);
                } else {
                    let idesc = self.instruction_table_.get(*data);
                    match idesc.type_ {
                        InstructionType::ZERO_OPERANDS_INSTR => {
                            self.append_to_buffer(idesc.mnem);
                            data = data.offset(1);
                        }

                        InstructionType::TWO_OPERANDS_INSTR => {
                            data = data.offset(1);
                            data = data.offset(self.print_operands(idesc.mnem, idesc.op_order_, data) as isize);
                        }

                        InstructionType::JUMP_CONDITIONAL_SHORT_INSTR => {
                            data = data.offset(self.jump_conditional_short(data, branch_hint) as isize);
                        }

                        InstructionType::REGISTER_INSTR => {
                            let reg = (*data & 0x07) as i32;
                            self.append_to_buffer(&format!("{} {}", idesc.mnem, self.converter_.name_of_cpu_register(reg)));
                            data = data.offset(1);
                        }

                        InstructionType::MOVE_REG_INSTR => {
                            let addr = helper_functions::imm32(data.offset(1) as *const u8 as *const [u8; 4]) as *mut u8;
                            let reg = (*data & 0x07) as i32;
                            self.append_to_buffer(&format!(
                                "mov {},{}",
                                self.converter_.name_of_cpu_register(reg),
                                self.converter_.name_of_address(addr as *const u8),
                            ));
                            data = data.offset(5);
                        }

                        InstructionType::CALL_JUMP_INSTR => {
                            let addr = data.offset(helper_functions::imm32(data.offset(1) as *const u8 as *const [u8; 4]) as isize + 5);
                            self.append_to_buffer(&format!(
                                "{} {}",
                                idesc.mnem,
                                self.converter_.name_of_address(addr as *const u8),
                            ));
                            data = data.offset(5);
                        }

                        InstructionType::SHORT_IMMEDIATE_INSTR => {
                            let addr = helper_functions::imm32(data.offset(1) as *const u8 as *const [u8; 4]) as *mut u8;
                            self.append_to_buffer(&format!(
                                "{} eax,{}",
                                idesc.mnem,
                                self.converter_.name_of_address(addr as *const u8),
                            ));
                            data = data.offset(5);
                        }

                        InstructionType::BYTE_IMMEDIATE_INSTR => {
                            self.append_to_buffer(&format!("{} al,0x{:x}", idesc.mnem, *data.offset(1)));
                            data = data.offset(2);
                        }

                        InstructionType::NO_INSTR => {
                            processed = false;
                        }

                        _ => {
                            //This type is not implemented.
                            todo!()
                        }
                    }
                }
                //----------------------------
                if !processed {
                    match *data {
                        0xC2 => {
                            self.append_to_buffer(&format!("ret 0x{:x}", helper_functions::imm16_u(data.offset(1) as *const u8 as *const [u8; 2])));
                            data = data.offset(3);
                        }

                        0x6B => {
                            data = data.offset(1);
                            data = data.offset(self.print_operands("imul", OperandOrder::REG_OPER_OP_ORDER, data) as isize);
                            self.append_to_buffer(&format!(",{}", *data));
                            data = data.offset(1);
                        }

                        0x69 => {
                            data = data.offset(1);
                            data = data.offset(self.print_operands("imul", OperandOrder::REG_OPER_OP_ORDER, data) as isize);
                            self.append_to_buffer(&format!(",{}", helper_functions::imm32(data as *const u8 as *const [u8; 4])));
                            data = data.offset(4);
                        }

                        0xF6 => {
                            data = data.offset(1);
                            let (modrm, regop, rm) = Self::get_modrm(*data);
                            if regop == 0 {
                                //eax
                                self.append_to_buffer("test_b ");
                                data = data.offset(self.print_right_byte_operand(data) as isize);
                                let imm = *data as i32;
                                self.append_to_buffer(&format!(",0x{:x}", imm));
                                data = data.offset(1);
                            } else {
                                self.unimplemented_instruction();
                            }
                        }

                        0x81 | 0x83 => {
                            // 0x81 with sign extension bit set
                            data = data.offset(self.print_immediate_op(data) as isize);
                        }

                        0x0F => {
                            let f0byte = *data.offset(1);
                            let f0mnem = Self::f0mnem(f0byte);
                            let (modrm, regop, rm) = Self::get_modrm(*data.offset(2));
                            // Not every instruction use this, and it is safe to index data+2 as all
                            // instructions are at least 3 bytes with operands.
                            if f0byte == 0x12 {
                                data = data.offset(2);
                                if modrm == 0b11 {
                                    self.append_to_buffer("movhlps ");
                                    self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                    self.append_to_buffer(",");
                                } else {
                                    self.append_to_buffer("movlps ");
                                    self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                    self.append_to_buffer(",");
                                }
                                data = data.offset(self.print_right_xmm_operand(data) as isize);
                            } else if f0byte == 0x13 {
                                data = data.offset(2);
                                self.append_to_buffer("movlps ");
                                data = data.offset(self.print_right_xmm_operand(data) as isize);
                                self.append_to_buffer(",");
                                self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                            } else if f0byte == 0x14 {
                                data = data.offset(2);
                                self.append_to_buffer("unpcklps ");
                                self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                self.append_to_buffer(",");
                                data = data.offset(self.print_right_xmm_operand(data) as isize);
                            } else if f0byte == 0x16 {
                                data = data.offset(2);
                                self.append_to_buffer("movhps ");
                                self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                self.append_to_buffer(",");
                                data = data.offset(self.print_right_xmm_operand(data) as isize);
                            } else if f0byte == 0x17 {
                                data = data.offset(2);
                                self.append_to_buffer("movhps ");
                                data = data.offset(self.print_right_xmm_operand(data) as isize);
                                self.append_to_buffer(",");
                                self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                            } else if f0byte == 0x18 {
                                data = data.offset(2);
                                let suffix: [&'static str; 4] = ["nta", "1", "2", "3"];
                                self.append_to_buffer(&format!("{}{}", f0mnem.unwrap(), suffix[regop as usize & 0x03]));
                                self.append_to_buffer(" ");
                                data = data.offset(self.print_right_operand(data) as isize);
                            } else if f0byte == 0x1F && *data.offset(2) == 0 {
                                self.append_to_buffer("nop"); // 3 byte nop.
                                data = data.offset(3);
                            } else if f0byte == 0x1F && *data.offset(2) == 0x40 && *data.offset(3) == 0 {
                                self.append_to_buffer("nop"); // 4 byte nop.
                                data = data.offset(4);
                            } else if f0byte == 0x1F && *data.offset(2) == 0x44 && *data.offset(3) == 0 && *data.offset(4) == 0 {
                                self.append_to_buffer("nop"); // 5 byte nop.
                                data = data.offset(5);
                            } else if f0byte == 0x1F
                                && *data.offset(2) == 0x80
                                && *data.offset(3) == 0
                                && *data.offset(4) == 0
                                && *data.offset(5) == 0
                                && *data.offset(6) == 0
                            {
                                self.append_to_buffer("nop"); // 7 byte nop.
                                data = data.offset(7);
                            } else if f0byte == 0x1F
                                && *data.offset(2) == 0x84
                                && *data.offset(3) == 0
                                && *data.offset(4) == 0
                                && *data.offset(5) == 0
                                && *data.offset(6) == 0
                                && *data.offset(7) == 0
                            {
                                self.append_to_buffer("nop"); // 8 byte nop.
                                data = data.offset(8);
                            } else if f0byte == 0x0B || f0byte == 0xA2 || f0byte == 0x31 {
                                self.append_to_buffer(f0mnem.unwrap());
                                data = data.offset(2);
                            } else if f0byte == 0x28 {
                                data = data.offset(2);
                                self.append_to_buffer(&format!("movaps {},{}", self.converter_.name_of_xmm_register(regop), self.converter_.name_of_xmm_register(rm)));
                                data = data.offset(1);
                            } else if f0byte == 0x10 || f0byte == 0x11 {
                                data = data.offset(2);
                                // movups xmm, xmm/m128
                                // movups xmm/m128, xmm
                                self.append_to_buffer("movups ");
                                if f0byte == 0x11 {
                                    data = data.offset(self.print_right_xmm_operand(data) as isize);
                                    self.append_to_buffer(",");
                                    self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                } else {
                                    self.append_to_buffer(self.converter_.name_of_xmm_register(regop));
                                    self.append_to_buffer(",");
                                    data = data.offset(self.print_right_xmm_operand(data) as isize);
                                }
                            } else if f0byte == 0x2E {
                                data = data.offset(2);
                                self.append_