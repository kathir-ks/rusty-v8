// TODO: Add appropriate Rust crates
// extern crate ...;

mod disasm {
    use std::fmt;

    /// Represents the order of operands in an instruction.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperandType {
        UNSET_OP_ORDER = 0,
        REG_OPER_OP_ORDER = 1,
        OPER_REG_OP_ORDER = 2,
        BYTE_SIZE_OPERAND_FLAG = 4,
        BYTE_REG_OPER_OP_ORDER = 5, //REG_OPER_OP_ORDER | BYTE_SIZE_OPERAND_FLAG,
        BYTE_OPER_REG_OP_ORDER = 6, //OPER_REG_OP_ORDER | BYTE_SIZE_OPERAND_FLAG,
        OPER_XMMREG_OP_ORDER = 7,
        XMMREG_OPER_OP_ORDER = 8,
        XMMREG_XMMOPER_OP_ORDER = 9,
        XMMOPER_XMMREG_OP_ORDER = 10,
    }

    /// Represents a byte and its associated mnemonic and operand order.
    #[derive(Debug, Copy, Clone)]
    struct ByteMnemonic {
        b: i32,
        op_order_: OperandType,
        mnem: &'static str,
    }

    const TWO_OPERANDS_INSTR: [ByteMnemonic; 48] = [
        ByteMnemonic { b: 0x00, op_order_: OperandType::BYTE_OPER_REG_OP_ORDER, mnem: "add" },
        ByteMnemonic { b: 0x01, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "add" },
        ByteMnemonic { b: 0x02, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "add" },
        ByteMnemonic { b: 0x03, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "add" },
        ByteMnemonic { b: 0x08, op_order_: OperandType::BYTE_OPER_REG_OP_ORDER, mnem: "or" },
        ByteMnemonic { b: 0x09, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "or" },
        ByteMnemonic { b: 0x0A, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "or" },
        ByteMnemonic { b: 0x0B, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "or" },
        ByteMnemonic { b: 0x10, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "adc" },
        ByteMnemonic { b: 0x11, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "adc" },
        ByteMnemonic { b: 0x12, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "adc" },
        ByteMnemonic { b: 0x13, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "adc" },
        ByteMnemonic { b: 0x18, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "sbb" },
        ByteMnemonic { b: 0x19, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "sbb" },
        ByteMnemonic { b: 0x1A, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "sbb" },
        ByteMnemonic { b: 0x1B, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "sbb" },
        ByteMnemonic { b: 0x20, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "and" },
        ByteMnemonic { b: 0x21, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "and" },
        ByteMnemonic { b: 0x22, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "and" },
        ByteMnemonic { b: 0x23, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "and" },
        ByteMnemonic { b: 0x28, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "sub" },
        ByteMnemonic { b: 0x29, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "sub" },
        ByteMnemonic { b: 0x2A, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "sub" },
        ByteMnemonic { b: 0x2B, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "sub" },
        ByteMnemonic { b: 0x30, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "xor" },
        ByteMnemonic { b: 0x31, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "xor" },
        ByteMnemonic { b: 0x32, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "xor" },
        ByteMnemonic { b: 0x33, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "xor" },
        ByteMnemonic { b: 0x38, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "cmp" },
        ByteMnemonic { b: 0x39, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "cmp" },
        ByteMnemonic { b: 0x3A, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "cmp" },
        ByteMnemonic { b: 0x3B, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "cmp" },
        ByteMnemonic { b: 0x63, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "movsxl" },
        ByteMnemonic { b: 0x84, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "test" },
        ByteMnemonic { b: 0x85, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "test" },
        ByteMnemonic { b: 0x86, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "xchg" },
        ByteMnemonic { b: 0x87, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "xchg" },
        ByteMnemonic { b: 0x88, op_order_: OperandType::BYTE_OPER_REG_OP_ORDER, mnem: "mov" },
        ByteMnemonic { b: 0x89, op_order_: OperandType::OPER_REG_OP_ORDER, mnem: "mov" },
        ByteMnemonic { b: 0x8A, op_order_: OperandType::BYTE_REG_OPER_OP_ORDER, mnem: "mov" },
        ByteMnemonic { b: 0x8B, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "mov" },
        ByteMnemonic { b: 0x8D, op_order_: OperandType::REG_OPER_OP_ORDER, mnem: "lea" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" }, // Termination
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" },
    ];

    const ZERO_OPERANDS_INSTR: [ByteMnemonic; 18] = [
        ByteMnemonic { b: 0xC3, op_order_: OperandType::UNSET_OP_ORDER, mnem: "ret" },
        ByteMnemonic { b: 0xC9, op_order_: OperandType::UNSET_OP_ORDER, mnem: "leave" },
        ByteMnemonic { b: 0xF4, op_order_: OperandType::UNSET_OP_ORDER, mnem: "hlt" },
        ByteMnemonic { b: 0xFC, op_order_: OperandType::UNSET_OP_ORDER, mnem: "cld" },
        ByteMnemonic { b: 0xCC, op_order_: OperandType::UNSET_OP_ORDER, mnem: "int3" },
        ByteMnemonic { b: 0x60, op_order_: OperandType::UNSET_OP_ORDER, mnem: "pushad" },
        ByteMnemonic { b: 0x61, op_order_: OperandType::UNSET_OP_ORDER, mnem: "popad" },
        ByteMnemonic { b: 0x9C, op_order_: OperandType::UNSET_OP_ORDER, mnem: "pushfd" },
        ByteMnemonic { b: 0x9D, op_order_: OperandType::UNSET_OP_ORDER, mnem: "popfd" },
        ByteMnemonic { b: 0x9E, op_order_: OperandType::UNSET_OP_ORDER, mnem: "sahf" },
        ByteMnemonic { b: 0x99, op_order_: OperandType::UNSET_OP_ORDER, mnem: "cdq" },
        ByteMnemonic { b: 0x9B, op_order_: OperandType::UNSET_OP_ORDER, mnem: "fwait" },
        ByteMnemonic { b: 0xAB, op_order_: OperandType::UNSET_OP_ORDER, mnem: "stos" },
        ByteMnemonic { b: 0xA4, op_order_: OperandType::UNSET_OP_ORDER, mnem: "movs" },
        ByteMnemonic { b: 0xA5, op_order_: OperandType::UNSET_OP_ORDER, mnem: "movs" },
        ByteMnemonic { b: 0xA6, op_order_: OperandType::UNSET_OP_ORDER, mnem: "cmps" },
        ByteMnemonic { b: 0xA7, op_order_: OperandType::UNSET_OP_ORDER, mnem: "cmps" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" }, // Termination
    ];

    const CALL_JUMP_INSTR: [ByteMnemonic; 3] = [
        ByteMnemonic { b: 0xE8, op_order_: OperandType::UNSET_OP_ORDER, mnem: "call" },
        ByteMnemonic { b: 0xE9, op_order_: OperandType::UNSET_OP_ORDER, mnem: "jmp" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" }, // Termination
    ];

    const SHORT_IMMEDIATE_INSTR: [ByteMnemonic; 9] = [
        ByteMnemonic { b: 0x05, op_order_: OperandType::UNSET_OP_ORDER, mnem: "add" },
        ByteMnemonic { b: 0x0D, op_order_: OperandType::UNSET_OP_ORDER, mnem: "or" },
        ByteMnemonic { b: 0x15, op_order_: OperandType::UNSET_OP_ORDER, mnem: "adc" },
        ByteMnemonic { b: 0x1D, op_order_: OperandType::UNSET_OP_ORDER, mnem: "sbb" },
        ByteMnemonic { b: 0x25, op_order_: OperandType::UNSET_OP_ORDER, mnem: "and" },
        ByteMnemonic { b: 0x2D, op_order_: OperandType::UNSET_OP_ORDER, mnem: "sub" },
        ByteMnemonic { b: 0x35, op_order_: OperandType::UNSET_OP_ORDER, mnem: "xor" },
        ByteMnemonic { b: 0x3D, op_order_: OperandType::UNSET_OP_ORDER, mnem: "cmp" },
        ByteMnemonic { b: -1, op_order_: OperandType::UNSET_OP_ORDER, mnem: "" }, // Termination
    ];

    const CONDITIONAL_CODE_SUFFIX: [&str; 16] = [
        "o", "no", "c", "nc", "z", "nz", "na", "a", "s", "ns", "pe", "po", "l", "ge", "le", "g",
    ];

    /// Represents the type of an instruction.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum InstructionType {
        NO_INSTR,
        ZERO_OPERANDS_INSTR,
        TWO_OPERANDS_INSTR,
        JUMP_CONDITIONAL_SHORT_INSTR,
        REGISTER_INSTR,
        PUSHPOP_INSTR,
        MOVE_REG_INSTR,
        CALL_JUMP_INSTR,
        SHORT_IMMEDIATE_INSTR,
    }

    /// Represents instruction prefixes.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Prefixes {
        ESCAPE_PREFIX = 0x0F,
        SEGMENT_FS_OVERRIDE_PREFIX = 0x64,
        OPERAND_SIZE_OVERRIDE_PREFIX = 0x66,
        ADDRESS_SIZE_OVERRIDE_PREFIX = 0x67,
        VEX3_PREFIX = 0xC4,
        VEX2_PREFIX = 0xC5,
        LOCK_PREFIX = 0xF0,
        REPNE_PREFIX = 0xF2,
        REP_PREFIX = 0xF3,
        REPEQ_PREFIX = REP_PREFIX,
    }

    /// Represents a description of an instruction.
    #[derive(Debug, Copy, Clone)]
    pub struct InstructionDesc {
        mnem: &'static str,
        type_: InstructionType,
        op_order_: OperandType,
        byte_size_operation: bool,
    }

    /// A table containing instruction descriptions.
    pub struct InstructionTable {
        instructions_: [InstructionDesc; 256],
    }

    impl InstructionTable {
        /// Creates a new `InstructionTable`.
        pub fn new() -> Self {
            let mut table = InstructionTable {
                instructions_: [InstructionDesc {
                    mnem: "(bad)",
                    type_: InstructionType::NO_INSTR,
                    op_order_: OperandType::UNSET_OP_ORDER,
                    byte_size_operation: false,
                }; 256],
            };
            table.init();
            table
        }

        /// Returns the `InstructionDesc` for the given byte.
        pub fn get(&self, x: u8) -> InstructionDesc {
            self.instructions_[x as usize]
        }

        /// Clears the instruction table.
        fn clear(&mut self) {
            for i in 0..256 {
                self.instructions_[i] = InstructionDesc {
                    mnem: "(bad)",
                    type_: InstructionType::NO_INSTR,
                    op_order_: OperandType::UNSET_OP_ORDER,
                    byte_size_operation: false,
                };
            }
        }

        /// Initializes the instruction table.
        fn init(&mut self) {
            self.copy_table(&TWO_OPERANDS_INSTR, InstructionType::TWO_OPERANDS_INSTR);
            self.copy_table(&ZERO_OPERANDS_INSTR, InstructionType::ZERO_OPERANDS_INSTR);
            self.copy_table(&CALL_JUMP_INSTR, InstructionType::CALL_JUMP_INSTR);
            self.copy_table(&SHORT_IMMEDIATE_INSTR, InstructionType::SHORT_IMMEDIATE_INSTR);
            self.add_jump_conditional_short();
            self.set_table_range(InstructionType::PUSHPOP_INSTR, 0x50, 0x57, false, "push");
            self.set_table_range(InstructionType::PUSHPOP_INSTR, 0x58, 0x5F, false, "pop");
            self.set_table_range(InstructionType::MOVE_REG_INSTR, 0xB8, 0xBF, false, "mov");
        }

        /// Copies entries from a `ByteMnemonic` array into the instruction table.
        fn copy_table(&mut self, bm: &[ByteMnemonic], type_: InstructionType) {
            for &bmnem in bm {
                if bmnem.b < 0 {
                    break; // End of table
                }
                let id = &mut self.instructions_[bmnem.b as usize];
                id.mnem = bmnem.mnem;
                let op_order = bmnem.op_order_;
                id.op_order_ = match (op_order as u32 & !OperandType::BYTE_SIZE_OPERAND_FLAG as u32) {
                    0 => OperandType::UNSET_OP_ORDER,
                    1 => OperandType::REG_OPER_OP_ORDER,
                    2 => OperandType::OPER_REG_OP_ORDER,
                    _ => OperandType::UNSET_OP_ORDER,
                };
                assert_eq!(id.type_, InstructionType::NO_INSTR);
                id.type_ = type_;
                id.byte_size_operation = (op_order as u32 & OperandType::BYTE_SIZE_OPERAND_FLAG as u32) != 0;
            }
        }

        /// Sets a range of entries in the instruction table to the given values.
        fn set_table_range(
            &mut self,
            type_: InstructionType,
            start: u8,
            end: u8,
            byte_size: bool,
            mnem: &'static str,
        ) {
            for b in start..=end {
                let id = &mut self.instructions_[b as usize];
                assert_eq!(id.type_, InstructionType::NO_INSTR);
                id.mnem = mnem;
                id.type_ = type_;
                id.byte_size_operation = byte_size;
            }
        }

        /// Adds jump conditional short instructions to the table.
        fn add_jump_conditional_short(&mut self) {
            for b in 0x70..=0x7F {
                let id = &mut self.instructions_[b as usize];
                assert_eq!(id.type_, InstructionType::NO_INSTR);
                id.mnem = ""; // Computed depending on condition code.
                id.type_ = InstructionType::JUMP_CONDITIONAL_SHORT_INSTR;
            }
        }
    }

    // TODO: Implement Lazy Instance
    // lazy_static! {
    //     static ref INSTRUCTION_TABLE: InstructionTable = InstructionTable::new();
    // }
    fn get_instruction_table() -> &'static InstructionTable {
        use std::sync::Once;
        use std::mem::MaybeUninit;

        static mut TABLE: MaybeUninit<InstructionTable> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            unsafe {
                TABLE.as_mut_ptr().write(InstructionTable::new());
            }
        });

        unsafe { TABLE.assume_init_ref() }
    }

    const CMOV_INSTRUCTIONS: [InstructionDesc; 16] = [
        InstructionDesc { mnem: "cmovo", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovno", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovc", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovnc", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovz", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovnz", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovna", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmova", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovs", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovns", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovpe", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovpo", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovl", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovge", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovle", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
        InstructionDesc { mnem: "cmovg", type_: InstructionType::TWO_OPERANDS_INSTR, op_order_: OperandType::REG_OPER_OP_ORDER, byte_size_operation: false },
    ];

    const CMP_PSEUDO_OP: [&str; 16] = [
        "eq", "lt", "le", "unord", "neq", "nlt", "nle", "ord", "eq_uq", "nge", "ngt", "false", "neq_oq", "ge", "gt", "true",
    ];

    mod immediate_helpers {
        use std::mem::transmute;

        /// Reads an 8-bit signed integer from the given data.
        #[inline]
        pub fn imm8(data: &[u8]) -> i8 {
            unsafe { *data.as_ptr() as i8 }
        }

        /// Reads an 8-bit unsigned integer from the given data.
        #[inline]
        pub fn imm8_u(data: &[u8]) -> u8 {
            data[0]
        }

        /// Reads a 16-bit signed integer from the given data.
        #[inline]
        pub fn imm16(data: &[u8]) -> i16 {
            unsafe { transmute::<[u8; 2], i16>([data[0], data[1]]) }
        }

        /// Reads a 16-bit unsigned integer from the given data.
        #[inline]
        pub fn imm16_u(data: &[u8]) -> u16 {
            unsafe { transmute::<[u8; 2], u16>([data[0], data[1]]) }
        }

        /// Reads a 32-bit signed integer from the given data.
        #[inline]
        pub fn imm32(data: &[u8]) -> i32 {
            unsafe { transmute::<[u8; 4], i32>([data[0], data[1], data[2], data[3]]) }
        }

        /// Reads a 32-bit unsigned integer from the given data.
        #[inline]
        pub fn imm32_u(data: &[u8]) -> u32 {
            unsafe { transmute::<[u8; 4], u32>([data[0], data[1], data[2], data[3]]) }
        }

        /// Reads a 64-bit signed integer from the given data.
        #[inline]
        pub fn imm64(data: &[u8]) -> i64 {
            unsafe { transmute::<[u8; 8], i64>([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]) }
        }
    }

    use immediate_helpers::*;

    /// Trait for converting register names and addresses to strings.
    pub trait NameConverter {
        /// Converts an address to a string.
        fn name_of_address(&self, addr: *const u8) -> String;
        /// Converts a constant to a string.
        fn name_of_constant(&self, addr: *const u8) -> String;
        /// Converts a CPU register to a string.
        fn name_of_cpu_register(&self, reg: i32) -> String;
        /// Converts a byte CPU register to a string.
        fn name_of_byte_cpu_register(&self, reg: i32) -> String;
        /// Converts an XMM register to a string.
        fn name_of_xmm_register(&self, reg: i32) -> String;
        /// Converts a name in code to a string.
        fn name_in_code(&self, addr: *const u8) -> String;

        fn root_relative_name(&self, offset: i32) -> Option<String>;
    }

    /// Default name converter that uses simple names.
    pub struct DefaultNameConverter {}

    impl NameConverter for DefaultNameConverter {
        fn name_of_address(&self, addr: *const u8) -> String {
            format!("{:p}", addr)
        }

        fn name_of_constant(&self, addr: *const u8) -> String {
            self.name_of_address(addr)
        }

        fn name_of_cpu_register(&self, reg: i32) -> String {
            CPU_REGS
                .get(reg as usize)
                .map(|&s| s.to_string())
                .unwrap_or_else(|| "noreg".to_string())
        }

        fn name_of_byte_cpu_register(&self, reg: i32) -> String {
            BYTE_CPU_REGS
                .get(reg as usize)
                .map(|&s| s.to_string())
                .unwrap_or_else(|| "noreg".to_string())
        }

        fn name_of_xmm_register(&self, reg: i32) -> String {
            XMM_REGS
                .get(reg as usize)
                .map(|&s| s.to_string())
                .unwrap_or_else(|| "noxmmreg".to_string())
        }

        fn name_in_code(&self, _addr: *const u8) -> String {
            "unknown".to_string()
        }

        fn root_relative_name(&self, _offset: i32) -> Option<String> {
            None
        }
    }

    static CPU_REGS: [&str; 16] = [
        "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
    ];
    static BYTE_CPU_REGS: [&str; 16] = [
        "al", "cl", "dl", "bl", "spl", "bpl", "sil", "dil", "r8l", "r9l", "r10l", "r11l", "r12l", "r13l", "r14l", "r15l",
    ];
    static XMM_REGS: [&str; 16] = [
        "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10", "xmm11", "xmm12", "xmm13", "xmm14", "xmm15",
    ];
    static YMM_REGS: [&str; 16] = [
        "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10", "ymm11", "ymm12", "ymm13", "ymm14", "ymm15",
    ];

    /// Represents the size of an operand.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum OperandSize {
        OPERAND_BYTE_SIZE = 0,
        OPERAND_WORD_SIZE = 1,
        OPERAND_DOUBLEWORD_SIZE = 2,
        OPERAND_QUADWORD_SIZE = 3,
    }

    // TODO: Implement Vector<char> (C++'s v8::base::Vector<char>)
    // For now, using String, but consider alternatives for performance.

    // TODO: Implement EmbeddedVector<char, 128>
    // This is a fixed-size buffer that can be useful for small strings.
    // For now, just using String, but we may need to optimize this.

    /// X64 Disassembler
    pub struct DisassemblerX64<'a> {
        converter_: &'a dyn NameConverter,
        tmp_buffer_: String, //v8::base::EmbeddedVector<char, 128>,
        tmp_buffer_pos_: usize,
        abort_on_unimplemented_: bool,
        rex_: u8,
        operand_size_: u8,
        group_1_prefix_: u8,
        segment_prefix_: u8,
        address_size_prefix_: u8,
        vex_byte0_: u8,
        vex_byte1_: u8,
        vex_byte2_: u8,
        byte_size_operand_: bool,
        instruction_table_: &'static InstructionTable,
    }

    impl<'a> DisassemblerX64<'a> {
        /// Creates a new `DisassemblerX64`.
        pub fn new(
            converter_: &'a dyn NameConverter,
            unimplemented_action: UnimplementedOpcodeAction,
        ) -> Self {
            DisassemblerX64 {
                converter_,
                tmp_buffer_: String::with_capacity(128),
                tmp_buffer_pos_: 0,
                abort_on_unimplemented_: unimplemented_action == UnimplementedOpcodeAction::AbortOnUnimplementedOpcode,
                rex_: 0,
                operand_size_: 0,
                group_1_prefix_: 0,
                segment_prefix_: 0,
                address_size_prefix_: 0,
                vex_byte0_: 0,
                vex_byte1_: 0,
                vex_byte2_: 0,
                byte_size_operand_: false,
                instruction_table_: get_instruction_table(),
            }
        }

        fn set_rex(&mut self, rex: u8) {
            assert_eq!(0x40, rex & 0xF0);
            self.rex_ = rex;
        }

        fn rex(&self) -> bool {
            self.rex_ != 0
        }

        fn rex_b(&self) -> bool {
            (self.rex_ & 0x01) != 0
        }

        fn base_reg(&self, low_bits: i32) -> i32 {
            low_bits | (((self.rex_ & 0x01) as i32) << 3)
        }

        fn rex_x(&self) -> bool {
            (self.rex_ & 0x02) != 0
        }

        fn rex_r(&self) -> bool {
            (self.rex_ & 0x04) != 0
        }

        fn rex_w(&self) -> bool {
            (self.rex_ & 0x08) != 0
        }

        fn vex_w(&self) -> bool {
            assert!(self.vex_byte0_ == Prefixes::VEX3_PREFIX as u8 || self.vex_byte0_ == Prefixes::VEX2_PREFIX as u8);
            if self.vex_byte0_ == Prefixes::VEX3_PREFIX as u8 {
                (self.vex_byte2_ & 0x