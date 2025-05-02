pub mod register_ppc {
    use std::marker::Copy;
    use std::ops::{Deref, DerefMut};

    // Macros for register definitions.
    macro_rules! define_registers {
        ($vis:vis $name:ident, $($reg:ident),*) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            $vis enum $name {
                $($reg,)*
                AfterLast,
            }

            impl $name {
                pub const fn from_code(code: usize) -> Self {
                    match code {
                        $(x if x == $name::$reg as usize => $name::$reg,)*
                        _ => panic!("Invalid register code"),
                    }
                }

                pub const fn to_code(self) -> usize {
                    self as usize
                }

                pub const fn no_reg() -> Self {
                   // This needs a const way to get a last.  For now AfterLast - 1;
                   // Once const enum is stable, use it.
                   let code = $name::AfterLast as usize;
                   if code == 0 {
                       panic!("AfterLast cannot be 0");
                   } else {
                       unsafe {
                         std::mem::transmute(code -1)
                       }
                   }
                }

                pub const fn register_name(self) -> &'static str {
                    match self {
                        $($name::$reg => stringify!($reg),)*
                        $name::AfterLast => "AfterLast",
                    }
                }
            }
        };
    }

    // Base register struct
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct RegisterBase<T, const N: usize> {
        code: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const N: usize> RegisterBase<T, N> {
        const fn new(code: usize) -> Self {
            Self {
                code,
                _phantom: std::marker::PhantomData,
            }
        }

        pub const fn code(self) -> usize {
            self.code
        }

        pub const fn from_code(code: usize) -> Self {
          Self {
              code: code,
              _phantom: std::marker::PhantomData,
          }
        }

        pub const fn no_reg() -> Self {
            Self {
                code: N - 1,
                _phantom: std::marker::PhantomData,
            }
        }
    }
    unsafe impl<T, const N: usize> Send for RegisterBase<T, N> {}
    unsafe impl<T, const N: usize> Sync for RegisterBase<T, N> {}

    // General Registers
    define_registers!(pub GeneralRegister, r0, sp, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, ip, r13, r14, r15, r16, r17, r18, r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, fp);

    pub type Register = RegisterBase<GeneralRegister, {GeneralRegister::AfterLast as usize}>;

    impl Register {
        pub const MANTISSA_OFFSET: i32 = if cfg!(target_endian = "little") {
            0
        } else {
            4
        };
        pub const EXPONENT_OFFSET: i32 = if cfg!(target_endian = "little") {
            4
        } else {
            0
        };
    }

    // Double Registers
    define_registers!(pub DoubleRegisterEnum, d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31);

    pub type DoubleRegister = RegisterBase<DoubleRegisterEnum, {DoubleRegisterEnum::AfterLast as usize}>;

    impl DoubleRegister {
        pub const SIZE_IN_BYTES: usize = 8;

        pub fn supported_register_count() -> usize {
            32 // TODO: Determine this dynamically based on CPU features.
        }

        pub fn to_simd(self) -> Simd128Register {
            let reg_code = self.code();
            assert!(reg_code >= 0 && reg_code < Simd128RegisterEnum::AfterLast as usize);
            Simd128Register::from_code(reg_code)
        }
    }

    // Float Register alias
    pub type FloatRegister = DoubleRegister;

    // SIMD128 Registers
    define_registers!(pub Simd128RegisterEnum, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31);

    pub type Simd128Register = RegisterBase<Simd128RegisterEnum, {Simd128RegisterEnum::AfterLast as usize}>;

    // C Registers
    define_registers!(pub CRegisterEnum, cr0, cr1, cr2, cr3, cr4, cr5, cr6, cr7, cr8, cr9, cr10, cr11, cr12, cr15);

    pub type CRegister = RegisterBase<CRegisterEnum, {CRegisterEnum::AfterLast as usize}>;

    // Constants
    pub const K_NUM_REQUIRED_STACK_FRAME_SLOTS: i32 = if cfg!(all(target_arch = "powerpc64", any(target_endian = "little", target_os = "linux"))) {
        12
    } else {
        14
    };
    pub const K_STACK_FRAME_LR_SLOT: i32 = 2;
    pub const K_STACK_FRAME_EXTRA_PARAM_SLOT: i32 = if cfg!(all(target_arch = "powerpc64", any(target_endian = "little", target_os = "linux"))) {
        12
    } else {
        14
    };

    // Aliases
    pub const K_CONSTANT_POOL_REGISTER: Register = Register::from_code(GeneralRegister::r28 as usize);
    pub const K_ROOT_REGISTER: Register = Register::from_code(GeneralRegister::r29 as usize);
    pub const CP: Register = Register::from_code(GeneralRegister::r30 as usize);

    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = if cfg!(feature = "compress_pointers") {
        Register::no_reg() //Disabled for now as there's no proper conditional compilation
    } else {
        Register::from_code(GeneralRegister::r27 as usize)
    };

    pub const K_C_ARG_REGS: [Register; 8] = [
        Register::from_code(GeneralRegister::r3 as usize),
        Register::from_code(GeneralRegister::r4 as usize),
        Register::from_code(GeneralRegister::r5 as usize),
        Register::from_code(GeneralRegister::r6 as usize),
        Register::from_code(GeneralRegister::r7 as usize),
        Register::from_code(GeneralRegister::r8 as usize),
        Register::from_code(GeneralRegister::r9 as usize),
        Register::from_code(GeneralRegister::r10 as usize),
    ];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();

    pub const fn argument_padding_slots(argument_count: i32) -> i32 {
        0
    }

    #[derive(PartialEq, Eq)]
    pub enum AliasingKind {
        kIndependent,
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kIndependent;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    pub const K_FIRST_CALLEE_SAVED_DOUBLE_REG: DoubleRegister = DoubleRegister::from_code(DoubleRegisterEnum::d14 as usize);
    pub const K_LAST_CALLEE_SAVED_DOUBLE_REG: DoubleRegister = DoubleRegister::from_code(DoubleRegisterEnum::d31 as usize);
    pub const K_DOUBLE_REG_ZERO: DoubleRegister = DoubleRegister::from_code(DoubleRegisterEnum::d14 as usize);
    pub const K_SCRATCH_DOUBLE_REG: DoubleRegister = DoubleRegister::from_code(DoubleRegisterEnum::d13 as usize);
    pub const K_SIMD128_REG_ZERO: Simd128Register = Simd128Register::from_code(Simd128RegisterEnum::v14 as usize);
    pub const K_SCRATCH_SIMD128_REG: Simd128Register = Simd128Register::from_code(Simd128RegisterEnum::v13 as usize);
    pub const K_SCRATCH_SIMD128_REG2: Simd128Register = Simd128Register::from_code(Simd128RegisterEnum::v15 as usize);

    pub const K_RETURN_REGISTER0: Register = Register::from_code(GeneralRegister::r3 as usize);
    pub const K_RETURN_REGISTER1: Register = Register::from_code(GeneralRegister::r4 as usize);
    pub const K_RETURN_REGISTER2: Register = Register::from_code(GeneralRegister::r5 as usize);
    pub const K_JS_FUNCTION_REGISTER: Register = Register::from_code(GeneralRegister::r4 as usize);
    pub const K_CONTEXT_REGISTER: Register = Register::from_code(GeneralRegister::r30 as usize);
    pub const K_ALLOCATE_SIZE_REGISTER: Register = Register::from_code(GeneralRegister::r4 as usize);
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = Register::from_code(GeneralRegister::r3 as usize);
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = Register::from_code(GeneralRegister::r15 as usize);
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = Register::from_code(GeneralRegister::r16 as usize);
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = Register::from_code(GeneralRegister::r17 as usize);

    pub const K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER: Register = Register::from_code(GeneralRegister::r3 as usize);
    pub const K_JAVASCRIPT_CALL_CODE_START_REGISTER: Register = Register::from_code(GeneralRegister::r5 as usize);
    pub const K_JAVASCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
    pub const K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER: Register = Register::from_code(GeneralRegister::r6 as usize);
    pub const K_JAVASCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = Register::from_code(GeneralRegister::r5 as usize);
    pub const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = Register::no_reg();

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = Register::from_code(GeneralRegister::r4 as usize);
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = Register::from_code(GeneralRegister::r3 as usize);
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = Register::from_code(GeneralRegister::r5 as usize);
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = Register::from_code(GeneralRegister::r10 as usize);
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = Register::from_code(GeneralRegister::r15 as usize);

    pub const K_FP_RETURN_REGISTER0: DoubleRegister = DoubleRegister::from_code(DoubleRegisterEnum::d1 as usize);

    // Assign |source| value to |no_reg| and return the |source|'s previous value.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    pub fn to_register(num: i32) -> Register {
        Register::from_code(num as usize) // TODO: Implement
    }

}