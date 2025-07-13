// Converted from V8 C++ source files:
// Header: register-x64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_x64 {
    use std::sync::Arc;

    // Placeholder for Isolate
    pub struct Isolate {}

    // Placeholder for Value
    pub trait Value {}

    // Placeholder for Local
    pub struct Local<'a, T: ?Sized + 'a> {
        _phantom: std::marker::PhantomData<&'a T>,
    }

    // Placeholder for Representation
    #[derive(Clone, Copy)]
    pub struct Representation {}

    // Placeholder for MachineType
    #[derive(Clone, Copy)]
    pub struct MachineType {}

    // Placeholder for MemoryRepresentation
    #[derive(Clone, Copy)]
    pub struct MemoryRepresentation {}

    // Placeholder for V
    #[derive(Clone, Copy)]
    pub struct V<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    // Placeholder for WordPtr
    #[derive(Clone, Copy)]
    pub struct WordPtr {}

    // Placeholder for SourceRange
    #[derive(Clone, Copy)]
    pub struct SourceRange {}

    // Placeholder for ZoneVector
    pub struct ZoneVector<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    // Placeholder for OpIndex
    #[derive(Clone, Copy)]
    pub struct OpIndex {}

    // Placeholder for ArchOpcode
    #[derive(Clone, Copy)]
    pub struct ArchOpcode {}

    // Placeholder for Macro
    pub struct Macro {}

    // Placeholder for SaveOptions
    pub struct SaveOptions {}

    // Placeholder for Cancelable
    pub struct Cancelable {}

    // Placeholder for DwVfpRegister
    pub struct DwVfpRegister {}

    // Placeholder for InstructionOperand
    pub struct InstructionOperand {}

    // Placeholder for RegisterArray
    pub struct RegisterArray {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum RegisterCode {
        kRegCode_rax,
        kRegCode_rcx,
        kRegCode_rdx,
        kRegCode_rbx,
        kRegCode_rsp,
        kRegCode_rbp,
        kRegCode_rsi,
        kRegCode_rdi,
        kRegCode_r8,
        kRegCode_r9,
        kRegCode_r10,
        kRegCode_r11,
        kRegCode_r12,
        kRegCode_r13,
        kRegCode_r14,
        kRegCode_r15,
        kRegAfterLast,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub const fn from_code(code: RegisterCode) -> Self {
            Register { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_byte_register(&self) -> bool {
            self.code <= 3
        }
        // Return the high bit of the register code as a 0 or 1.  Used often
        // when constructing the REX prefix byte.
        pub const fn high_bit(&self) -> i32 {
            self.code >> 3
        }
        // Return the 3 low bits of the register code.  Used when encoding registers
        // in modR/M, SIB, and opcode bytes.
        pub const fn low_bits(&self) -> i32 {
            self.code & 0x7
        }

        pub const fn no_reg() -> Self {
            Register {
                code: RegisterCode::kRegAfterLast as i32,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct TaggedRegister {
        reg_: Register,
    }

    impl TaggedRegister {
        pub fn new(reg: Register) -> Self {
            TaggedRegister { reg_: reg }
        }
        pub fn reg(&self) -> Register {
            self.reg_
        }
    }

    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! declare_register {
        ($name:ident, $reg_code:ident) => {
            pub const $name: Register = Register::from_code(RegisterCode::$reg_code);
        };
    }

    declare_register!(rax, kRegCode_rax);
    declare_register!(rcx, kRegCode_rcx);
    declare_register!(rdx, kRegCode_rdx);
    declare_register!(rbx, kRegCode_rbx);
    declare_register!(rsp, kRegCode_rsp);
    declare_register!(rbp, kRegCode_rbp);
    declare_register!(rsi, kRegCode_rsi);
    declare_register!(rdi, kRegCode_rdi);
    declare_register!(r8, kRegCode_r8);
    declare_register!(r9, kRegCode_r9);
    declare_register!(r10, kRegCode_r10);
    declare_register!(r11, kRegCode_r11);
    declare_register!(r12, kRegCode_r12);
    declare_register!(r13, kRegCode_r13);
    declare_register!(r14, kRegCode_r14);
    declare_register!(r15, kRegCode_r15);

    pub const no_reg: Register = Register::no_reg();

    pub const K_NUM_REGS: i32 = 16;

    #[cfg(target_os = "windows")]
    pub const K_CARG_REGS: [Register; 4] = [rcx, rdx, r8, r9];

    #[cfg(not(target_os = "windows"))]
    pub const K_CARG_REGS: [Register; 6] = [rdi, rsi, rdx, rcx, r8, r9];

    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_CARG_REGS.len();

    macro_rules! define_register_names {
        ($reg_type:ident, $reg_macro:ident) => {
            impl $reg_type {
                pub fn RegisterName(&self) -> &'static str {
                    match self.code() {
                        0 => stringify!($reg_type).into(),
                        1 => stringify!($reg_type).into(),
                        2 => stringify!($reg_type).into(),
                        3 => stringify!($reg_type).into(),
                        4 => stringify!($reg_type).into(),
                        5 => stringify!($reg_type).into(),
                        6 => stringify!($reg_type).into(),
                        7 => stringify!($reg_type).into(),
                        8 => stringify!($reg_type).into(),
                        9 => stringify!($reg_type).into(),
                        10 => stringify!($reg_type).into(),
                        11 => stringify!($reg_type).into(),
                        12 => stringify!($reg_type).into(),
                        13 => stringify!($reg_type).into(),
                        14 => stringify!($reg_type).into(),
                        15 => stringify!($reg_type).into(),
                        _ => "invalid".into(),
                    }
                }
            }
        };
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum DoubleRegisterCode {
        kDoubleCode_xmm0,
        kDoubleCode_xmm1,
        kDoubleCode_xmm2,
        kDoubleCode_xmm3,
        kDoubleCode_xmm4,
        kDoubleCode_xmm5,
        kDoubleCode_xmm6,
        kDoubleCode_xmm7,
        kDoubleCode_xmm8,
        kDoubleCode_xmm9,
        kDoubleCode_xmm10,
        kDoubleCode_xmm11,
        kDoubleCode_xmm12,
        kDoubleCode_xmm13,
        kDoubleCode_xmm14,
        kDoubleCode_xmm15,
        kDoubleAfterLast,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct XMMRegister {
        code: i32,
    }

    impl XMMRegister {
        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            XMMRegister { code: code as i32 }
        }
        pub const fn code(&self) -> i32 {
            self.code
        }
        // Return the high bit of the register code as a 0 or 1.  Used often
        // when constructing the REX prefix byte.
        pub const fn high_bit(&self) -> i32 {
            self.code >> 3
        }
        // Return the 3 low bits of the register code.  Used when encoding registers
        // in modR/M, SIB, and opcode bytes.
        pub const fn low_bits(&self) -> i32 {
            self.code & 0x7
        }

        pub const fn no_reg() -> Self {
            XMMRegister {
                code: DoubleRegisterCode::kDoubleAfterLast as i32,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum YMMRegisterCode {
        kYMMCode_ymm0,
        kYMMCode_ymm1,
        kYMMCode_ymm2,
        kYMMCode_ymm3,
        kYMMCode_ymm4,
        kYMMCode_ymm5,
        kYMMCode_ymm6,
        kYMMCode_ymm7,
        kYMMCode_ymm8,
        kYMMCode_ymm9,
        kYMMCode_ymm10,
        kYMMCode_ymm11,
        kYMMCode_ymm12,
        kYMMCode_ymm13,
        kYMMCode_ymm14,
        kYMMCode_ymm15,
        kYMMAfterLast,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct YMMRegister {
        code: i32,
    }

    impl YMMRegister {
        pub const fn from_code(code: YMMRegisterCode) -> Self {
            YMMRegister { code: code as i32 }
        }

        pub const fn from_xmm(xmm: XMMRegister) -> Self {
            YMMRegister { code: xmm.code() }
        }
        pub const fn code(&self) -> i32 {
            self.code
        }
    }

    pub type FloatRegister = XMMRegister;
    pub type DoubleRegister = XMMRegister;
    pub type Simd128Register = XMMRegister;
    pub type Simd256Register = YMMRegister;

    macro_rules! declare_double_register {
        ($name:ident, $reg_code:ident) => {
            pub const $name: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCode::$reg_code);
        };
    }

    declare_double_register!(xmm0, kDoubleCode_xmm0);
    declare_double_register!(xmm1, kDoubleCode_xmm1);
    declare_double_register!(xmm2, kDoubleCode_xmm2);
    declare_double_register!(xmm3, kDoubleCode_xmm3);
    declare_double_register!(xmm4, kDoubleCode_xmm4);
    declare_double_register!(xmm5, kDoubleCode_xmm5);
    declare_double_register!(xmm6, kDoubleCode_xmm6);
    declare_double_register!(xmm7, kDoubleCode_xmm7);
    declare_double_register!(xmm8, kDoubleCode_xmm8);
    declare_double_register!(xmm9, kDoubleCode_xmm9);
    declare_double_register!(xmm10, kDoubleCode_xmm10);
    declare_double_register!(xmm11, kDoubleCode_xmm11);
    declare_double_register!(xmm12, kDoubleCode_xmm12);
    declare_double_register!(xmm13, kDoubleCode_xmm13);
    declare_double_register!(xmm14, kDoubleCode_xmm14);
    declare_double_register!(xmm15, kDoubleCode_xmm15);

    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    macro_rules! declare_ymm_register {
        ($name:ident, $reg_code:ident) => {
            pub const $name: YMMRegister = YMMRegister::from_code(YMMRegisterCode::$reg_code);
        };
    }

    declare_ymm_register!(ymm0, kYMMCode_ymm0);
    declare_ymm_register!(ymm1, kYMMCode_ymm1);
    declare_ymm_register!(ymm2, kYMMCode_ymm2);
    declare_ymm_register!(ymm3, kYMMCode_ymm3);
    declare_ymm_register!(ymm4, kYMMCode_ymm4);
    declare_ymm_register!(ymm5, kYMMCode_ymm5);
    declare_ymm_register!(ymm6, kYMMCode_ymm6);
    declare_ymm_register!(ymm7, kYMMCode_ymm7);
    declare_ymm_register!(ymm8, kYMMCode_ymm8);
    declare_ymm_register!(ymm9, kYMMCode_ymm9);
    declare_ymm_register!(ymm10, kYMMCode_ymm10);
    declare_ymm_register!(ymm11, kYMMCode_ymm11);
    declare_ymm_register!(ymm12, kYMMCode_ymm12);
    declare_ymm_register!(ymm13, kYMMCode_ymm13);
    declare_ymm_register!(ymm14, kYMMCode_ymm14);
    declare_ymm_register!(ymm15, kYMMCode_ymm15);

    define_register_names!(Register, GENERAL_REGISTERS);
    define_register_names!(XMMRegister, DOUBLE_REGISTERS);
    define_register_names!(YMMRegister, YMM_REGISTERS);

    pub const K_STACK_POINTER_REGISTER: Register = rsp;
    pub const K_RETURN_REGISTER_0: Register = rax;
    pub const K_RETURN_REGISTER_1: Register = rdx;
    pub const K_RETURN_REGISTER_2: Register = r8;
    pub const K_JSFUNCTION_REGISTER: Register = rdi;
    pub const K_CONTEXT_REGISTER: Register = rsi;
    pub const K_ALLOCATE_SIZE_REGISTER: Register = rdx;
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = rax;
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = r9;
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = r12;
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = r15;

    pub const K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER: Register = rax;
    pub const K_JAVASCRIPT_CALL_CODE_START_REGISTER: Register = rcx;
    pub const K_JAVASCRIPT_CALL_TARGET_REGISTER: Register = K_JSFUNCTION_REGISTER;
    pub const K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER: Register = rdx;
    pub const K_JAVASCRIPT_CALL_EXTRA_ARG_1_REGISTER: Register = rbx;
    pub const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = r15;

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = rbx;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = rax;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = r15;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = rsi;
    pub const K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER: Register = r10;

    // Default scratch register used by MacroAssembler (and other code that needs
    // a spare register). The register isn't callee save, and not used by the
    // function calling convention.
    pub const K_SCRATCH_REGISTER: Register = r10;
    pub const K_SCRATCH_DOUBLE_REG: XMMRegister = xmm15;
    pub const K_SCRATCH_SIMD256_REG: YMMRegister = ymm15;
    pub const K_ROOT_REGISTER: Register = r13; // callee save

    #[cfg(feature = "v8_compress_pointers")]
    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = r14; // callee save
    #[cfg(not(feature = "v8_compress_pointers"))]
    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = no_reg;

    pub const K_FP_RETURN_REGISTER_0: XMMRegister = xmm0;

    // Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum AliasingKind {
        kNoAlias,
        kAlias,
        kMaybeAlias,
        kOverlap,
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kOverlap;
    pub const K_SIMD_MASK_REGISTERS: bool = false;
}
