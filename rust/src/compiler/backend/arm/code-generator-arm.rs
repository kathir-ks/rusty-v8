// TODO: Replace with appropriate Rust crates for V8 functionality

// Mock implementations for V8 types and functions
mod v8 {
    pub mod base {
        pub mod numbers {
            pub struct Double {
                value: f64,
            }

            impl Double {
                pub fn value(&self) -> f64 {
                    self.value
                }
            }
        }
    }

    pub mod codegen {
        pub mod arm {
            pub mod assembler_arm {
                // Mock AssemblerArm
                pub struct AssemblerArm;

                impl AssemblerArm {
                    pub fn new() -> Self {
                        AssemblerArm
                    }

                    pub fn push(&mut self, register: Register) {}
                    pub fn Push(&mut self, reg1: Register, reg2: Register) {}
                    pub fn Push(&mut self, reg1: Register, reg2: Register, reg3: Register) {}
                    pub fn allocate_stack_space(&mut self, size: i32) {}
                    pub fn add(&mut self, dest: Register, src: Register, operand: Operand) {}
                    pub fn sub(&mut self, dest: Register, src: Register, operand: Operand) {}
                    pub fn ldr(&mut self, dest: Register, mem_operand: MemOperand) {}
                    pub fn str(&mut self, src: Register, mem_operand: MemOperand) {}
                    pub fn b(&mut self, condition: Condition, label: &Label) {}
                    pub fn bind(&mut self, label: &Label) {}
                    pub fn LeaveFrame(&mut self, frame_type: super::super::macro_assembler::StackFrame) {}
                    pub fn ldm(&mut self, addressing_mode: AddressingMode, base: Register, registers: std::vec::Vec<Register>) {}
                    pub fn cmp(&mut self, reg1: Register, reg2: Register) {}
                    pub fn Assert(&mut self, cond: Condition, reason: AbortReason) {}
                    pub fn Call(&mut self, code: i32, reloc_info: RelocInfo) {}
                    pub fn call_code_object(&mut self, reg: Register) {}
                    pub fn dmb(&mut self, is_domain_share: ISH) {}
                    pub fn PushCallerSaved(&mut self, fp_mode: SaveFPRegsMode, kReturnRegister0: Register) -> i32 { 0 }
                    pub fn PopCallerSaved(&mut self, fp_mode: SaveFPRegsMode, kReturnRegister0: Register) -> i32 { 0 }

                    // NEON instructions
                    pub fn vld1(&mut self, neon_8: NeonType, list_operand: NeonListOperand, mem_operand: NeonMemOperand) {}
                    pub fn vst1(&mut self, neon_8: NeonType, list_operand: NeonListOperand, mem_operand: NeonMemOperand) {}
                    pub fn vmov(&mut self, reg1: SwVfpRegister, reg2: Register) {}
                    pub fn vmov(&mut self, reg: Register, value: i32) {}
                    pub fn vsub(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}
                    pub fn vadd(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}
                    pub fn vmul(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}
                    pub fn vdiv(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}
                    pub fn vsqrt(&mut self, reg_dst: SwVfpRegister, reg_src: SwVfpRegister) {}
                    pub fn vneg(&mut self, reg_dst: SwVfpRegister, reg_src: SwVfpRegister) {}
                    pub fn vabs(&mut self, reg_dst: SwVfpRegister, reg_src: SwVfpRegister) {}
                    pub fn vcmp(&mut self, reg1: Register, reg2: Register) {}
                    pub fn vmla(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}
                    pub fn vmls(&mut self, reg_dst: SwVfpRegister, reg_src1: SwVfpRegister, reg_src2: SwVfpRegister) {}

                    pub fn vmov(&mut self, reg_dst: DwVfpRegister, reg_src1: DwVfpRegister, reg_src2: DwVfpRegister) {}

                    pub fn mov(&mut self, output_register: Register, input_register: Register) {}
                    pub fn mov(&mut self, output_register: Register, input_operand: Operand, sbit: SBit) {}
                    pub fn mvn(&mut self, output_register: Register, input_operand: Operand, sbit: SBit) {}
                    pub fn orr(&mut self, output_register: Register, input_register: Register, input_operand: Operand, sbit: SBit) {}
                    pub fn eor(&mut self, output_register: Register, input_register: Register, input_operand: Operand, sbit: SBit) {}
                    pub fn bic(&mut self, output_register: Register, input_register: Register, input_operand: Operand, sbit: SBit) {}
                    pub fn mul(&mut self, output_register: Register, input_register: Register, input_register2: Register, sbit: SBit) {}
                    pub fn mla(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_register3: Register, sbit: SBit) {}
                    pub fn mls(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_register3: Register) {}
                    pub fn smull(&mut self, output_register: Register, output_register2: Register, input_register: Register, input_register2: Register) {}
                    pub fn umull(&mut self, output_register: Register, output_register2: Register, input_register: Register, input_register2: Register, sbit: SBit) {}
                    pub fn sdiv(&mut self, output_register: Register, input_register: Register, input_register2: Register) {}
                    pub fn udiv(&mut self, output_register: Register, input_register: Register, input_register2: Register) {}

                    pub fn bfc(&mut self, output_register: Register, input_int8: i8, input_int82: i8) {}
                    pub fn ubfx(&mut self, output_register: Register, input_register: Register, input_int8: i8, input_int82: i8) {}
                    pub fn sbfx(&mut self, output_register: Register, input_register: Register, input_int8: i8, input_int82: i8) {}
                    pub fn sxtb(&mut self, output_register: Register, input_register: Register, input_int32: i32) {}
                    pub fn sxth(&mut self, output_register: Register, input_register: Register, input_int32: i32) {}
                    pub fn sxtab(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_int32: i32) {}
                    pub fn sxtah(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_int32: i32) {}
                    pub fn uxtb(&mut self, output_register: Register, input_register: Register, input_int32: i32) {}
                    pub fn uxth(&mut self, output_register: Register, input_register: Register, input_int32: i32) {}
                    pub fn uxtab(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_int32: i32) {}
                    pub fn uxtah(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_int32: i32) {}

                    pub fn rbit(&mut self, output_register: Register, input_register: Register) {}
                    pub fn rev(&mut self, output_register: Register, input_register: Register) {}
                    pub fn clz(&mut self, output_register: Register, input_register: Register) {}
                    pub fn cmn(&mut self, input_register: Register, input_operand: Operand) {}
                    pub fn tst(&mut self, input_register: Register, input_operand: Operand) {}
                    pub fn teq(&mut self, input_register: Register, input_operand: Operand) {}
                    pub fn smmul(&mut self, output_register: Register, input_register: Register, input_register2: Register) {}
                    pub fn smmla(&mut self, output_register: Register, input_register: Register, input_register2: Register, input_register3: Register) {}

                    // 64bit int
                    pub fn ldrexd(&mut self, reg1: Register, reg2: Register, mem: Register) {}
                    pub fn strexd(&mut self, reg_scratch: Register, reg1: Register, reg2: Register, mem: Register) {}

                    // jump
                    pub fn Jump(&mut self, code: i32, rmode: RelocInfo) {}
                    pub fn JumpCodeObject(&mut self, reg: Register) {}
                    pub fn Jump(&mut self, reg: Register) {}

                    // debug
                    pub fn DebugBreak(&mut self) {}

                    // comment
                    pub fn RecordComment(&mut self, comment: *const char, source_location: SourceLocation) {}

                    // BailoutIfDeoptimized
                    pub fn BailoutIfDeoptimized(&mut self) {}

                    pub fn PrepareCallCFunction(&mut self, i: i32) {}
                    pub fn MovToFloatParameters(&mut self, input_double_register: DwVfpRegister, input_double_register2: DwVfpRegister) {}
                    pub fn CallCFunction(&mut self, ieee754_pow_function: ExternalReference, i: i32, i2: i32) -> i32 { 0 }
                    pub fn MovFromFloatResult(&mut self, output_double_register: DwVfpRegister) {}
                    pub fn MovToFloatParameter(&mut self, input_double_register: DwVfpRegister) {}
                    pub fn CallJSFunction(&mut self, func: Register, num_arguments: u32) {}
                    pub fn CallBuiltinByIndex(&mut self, builtin_index: Register, target: Register) {}
                    pub fn CallBuiltin(&mut self, abort_csadcheck: Builtin) {}
                    pub fn ComputeCodeStartAddress(&mut self, scratch: Register) {}
                    pub fn stop(&mut self) {}

                    pub fn VFPCompareAndSetFlags(&mut self, input_float_register: SwVfpRegister, input_float_register2: SwVfpRegister) {}
                    pub fn VFPCompareAndSetFlags(&mut self, input_double_register: DwVfpRegister, input_double: f64) {}
                    pub fn vcvt_f32_f64(&mut self, output_float_register: SwVfpRegister, input_double_register: DwVfpRegister) {}
                    pub fn vcvt_f64_f32(&mut self, output_double_register: DwVfpRegister, input_float_register: SwVfpRegister) {}
                    pub fn vmov(&mut self, scratch: SwVfpRegister, input_register: Register) {}
                    pub fn vcvt_f32_s32(&mut self, output_float_register: SwVfpRegister, scratch: SwVfpRegister) {}
                    pub fn vcvt_f32_u32(&mut self, output_float_register: SwVfpRegister, scratch: SwVfpRegister) {}
                    pub fn vcvt_f64_s32(&mut self, output_double_register: DwVfpRegister, scratch: SwVfpRegister) {}
                    pub fn vcvt_f64_u32(&mut self, output_double_register: DwVfpRegister, scratch: SwVfpRegister) {}
                    pub fn vcvt_s32_f32(&mut self, scratch: SwVfpRegister, input_float_register: SwVfpRegister) {}
                    pub fn vcvt_u32_f32(&mut self, scratch: SwVfpRegister, input_float_register: SwVfpRegister) {}
                    pub fn vcvt_s32_f64(&mut self, scratch: SwVfpRegister, input_double_register: DwVfpRegister) {}
                    pub fn vcvt_u32_f64(&mut self, scratch: SwVfpRegister, input_double_register: DwVfpRegister) {}
                    pub fn VmovLow(&mut self, output_register: Register, input_double_register: DwVfpRegister) {}
                    pub fn VmovLow(&mut self, output_double_register: DwVfpRegister, input_register: Register) {}
                    pub fn VmovHigh(&mut self, output_register: Register, input_double_register: DwVfpRegister) {}
                    pub fn VmovHigh(&mut self, output_double_register: DwVfpRegister, input_register: Register) {}
                    pub fn vldrb(&mut self, output_register: Register, input_offset: MemOperand) {}
                    pub fn vldrsb(&mut self, output_register: Register, input_offset: MemOperand) {}
                    pub fn vstrb(&mut self, output_register: Register, input_offset: MemOperand) {}
                    pub fn vldrh(&mut self, output_register: Register, input_offset: MemOperand) {}
                    pub fn vldrsh(&mut self, output_register: Register, input_offset: MemOperand) {}
                    pub fn vstrh(&mut self, output_register: Register, input_offset: MemOperand) {}

                    // float max/min
                    pub fn FloatMax(&mut self, result: SwVfpRegister, left: SwVfpRegister, right: SwVfpRegister, entry: &Label) {}
                    pub fn FloatMin(&mut self, result: SwVfpRegister, left: SwVfpRegister, right: SwVfpRegister, entry: &Label) {}
                    pub fn FloatMax(&mut self, result: DwVfpRegister, left: DwVfpRegister, right: DwVfpRegister, entry: &Label) {}
                    pub fn FloatMin(&mut self, result: DwVfpRegister, left: DwVfpRegister, right: DwVfpRegister, entry: &Label) {}
                    pub fn VFPCanonicalizeNaN(&mut self, result: DwVfpRegister, value: DwVfpRegister) {}
                }
            }

            pub mod constants_arm {
                use super::register_arm::Register;

                pub const kRootRegister: Register = Register { code: 10 }; // Mock value
                pub const kJavaScriptCallCodeStartRegister: Register = Register { code: 11 }; // Mock
            }

            pub mod register_arm {
                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct Register {
                    pub code: i32,
                }

                impl Register {
                    pub fn from_code(code: i32) -> Self {
                        Register { code }
                    }
                }

                // Define common registers
                pub const r0: Register = Register { code: 0 };
                pub const r1: Register = Register { code: 1 };
                pub const r2: Register = Register { code: 2 };
                pub const r3: Register = Register { code: 3 };
                pub const r4: Register = Register { code: 4 };
                pub const r5: Register = Register { code: 5 };
                pub const r6: Register = Register { code: 6 };
                pub const r7: Register = Register { code: 7 };
                pub const r8: Register = Register { code: 8 };
                pub const r9: Register = Register { code: 9 };
                pub const r10: Register = Register { code: 10 };
                pub const r11: Register = Register { code: 11 };
                pub const r12: Register = Register { code: 12 };
                pub const fp: Register = Register { code: 11 };
                pub const ip: Register = Register { code: 12 };
                pub const sp: Register = Register { code: 13 };
                pub const lr: Register = Register { code: 14 };
                pub const pc: Register = Register { code: 15 };
                pub const kReturnRegister0: Register = r0;
            }
        }

        pub mod macro_assembler {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum StackFrame {
                MANUAL,
                NO_FRAME_TYPE,
            }
        }
    }

    pub mod assembler_inl {
    }

    pub mod interface_descriptors_inl {
    }

    pub mod machine_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MachineRepresentation {
            kInteger32,
            kInteger64,
            kFloat32,
            kFloat64,
            kSimd128,
            kWord32,
            kWord64,
            kBit,
            kTagged,
            kCompressed,
            kNone,
        }
    }

    pub mod macro_assembler {
        // Mock MacroAssembler
        pub struct MacroAssembler;

        impl MacroAssembler {
            pub fn new() -> Self {
                MacroAssembler
            }

            pub fn AllocateStackSpace(&mut self, size: i32) {}
        }
    }

    pub mod optimized_compilation_info {
    }

    pub mod common {
        pub mod globals {
            pub const kSystemPointerSize: i32 = 4;
        }
    }

    pub mod compiler {
        pub mod backend {
            pub mod code_generator_impl {
            }

            pub mod code_generator {
                use super::super::super::codegen::arm::register_arm::Register;
                // Mock CodeGenerator
                pub struct CodeGenerator;

                impl CodeGenerator {
                    pub fn AssembleDeconstructFrame(&mut self) {}
                }
            }

            pub mod gap_resolver {
            }

            pub mod instruction_codes {
            }
        }
    }

    pub mod heap {
        pub mod mutable_page_metadata {
        }
    }

    pub mod utils {
        pub mod boxed_float {
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RelocInfo {
        CODE_TARGET,
        // Add other RelocInfo types as needed
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ExternalReference {
        // Add fields as needed
    }

    impl ExternalReference {
        pub fn ieee754_pow_function() -> Self {
            ExternalReference {}
        }

        pub fn mod_two_doubles_operation() -> Self {
            ExternalReference {}
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Builtin {
        kAbortCSADcheck,
        // Add other Builtin types as needed
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AbortReason {
        kWrongFunctionCodeStart,
        kOperandIsCleared,
        kWrongFunctionContext,
        // Add other AbortReason types as needed
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CallDescriptor {
        // Add fields as needed
        kFixedTargetRegister,
    }

    impl CallDescriptor {
        pub fn kFixedTargetRegister() -> Self {
            CallDescriptor::kFixedTargetRegister
        }
    }
}

mod flags {
    pub mod v8_flags {
        pub static debug_code: bool = true;
    }
}

mod wasm {
    pub mod wasm_linkage {
    }
    pub mod wasm_objects {
    }
}

use v8::base::numbers::Double;
use v8::codegen::arm::assembler_arm::AssemblerArm;
use v8::codegen::arm::assembler_arm::Condition;
use v8::codegen::arm::constants_arm::*;
use v8::codegen::arm::register_arm::*;
use v8::codegen::assembler_inl::*;
use v8::codegen::interface_descriptors_inl::*;
use v8::codegen::machine_type::*;
use v8::codegen::macro_assembler::*;
use v8::common::globals::*;
use v8::compiler::backend::code_generator_impl::*;
use v8::compiler::backend::code_generator::*;
use v8::compiler::backend::instruction_codes::*;
use v8::heap::mutable_page_metadata::*;
use v8::utils::boxed_float::*;
use v8::*;

// Mock implementations for assembler types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    kMode_None,
    kMode_Offset_RI,
    kMode_Offset_RR,
    kMode_Root,
    kMode_Operand2_I,
    kMode_Operand2_R,
    kMode_Operand2_R_ASR_I,
    kMode_Operand2_R_ASR_R,
    kMode_Operand2_R_LSL_I,
    kMode_Operand2_R_LSL_R,
    kMode_Operand2_R_LSR_I,
    kMode_Operand2_R_LSR_R,
    kMode_Operand2_R_ROR_I,
    kMode_Operand2_R_ROR_R,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Operand {
    value: i32,
    rmode: RelocInfo,
}

impl Operand {
    pub fn new(value: i32, rmode: RelocInfo) -> Self {
        Operand { value, rmode }
    }

    pub fn EmbeddedNumber(value: f64) -> Self {
        Operand { value: 0, rmode: RelocInfo::CODE_TARGET } // Mock
    }

    pub fn ToExternalReference(external_reference: ExternalReference) -> Self {
        Operand { value: 0, rmode: RelocInfo::CODE_TARGET } // Mock
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemOperand {
    base: Register,
    offset: i32,
}

impl MemOperand {
    pub fn new(base: Register, offset: i32) -> Self {
        MemOperand { base, offset }
    }

    pub fn new_root(k_root_register: Register, input_int32: i32) -> Self {
        MemOperand { base: k_root_register, offset: input_int32 }
    }
    
    pub fn new_with_registers(reg1:Register, reg2:Register) -> Self {
        MemOperand { base: reg1, offset: reg2.code }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NeonMemOperand {
    reg: Register,
}

impl NeonMemOperand {
    pub fn new(reg: Register) -> Self {
        NeonMemOperand { reg }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ISH {
    ISH
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NeonListOperand {
    reg: DwVfpRegister,
}

impl NeonListOperand {
    pub fn new(reg: DwVfpRegister) -> Self {
        NeonListOperand { reg }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NeonType {
    Neon8,
    Neon32,
    Neon64
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NeonDataType {
    NeonS32,
    NeonS16,
    NeonS8,
    NeonU32,
    NeonU16,
    NeonU8
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Neon8;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Neon32;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Neon64;

// Mock implementations for register types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SwVfpRegister {
    code: i32,
}

impl SwVfpRegister {
    pub fn from_code(code: i32) -> Self {
        SwVfpRegister { code }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DwVfpRegister {
    code: i32,
}

impl DwVfpRegister {
    pub fn from_code(code: i32) -> Self {
        DwVfpRegister { code }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QwNeonRegister {
    code: i32,
}

impl QwNeonRegister {
    pub fn from_code(code: i32) -> Self {
        QwNeonRegister { code }
    }

    pub fn low(&self) -> SwVfpRegister {
        SwVfpRegister::from_code(self.code)
    }

    pub fn high(&self) -> SwVfpRegister {
        SwVfpRegister::from_code(self.code + 1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Simd128Register {
    code: i32,
}

impl Simd128Register {
    pub fn new(code: i32) -> Self {
        Simd128Register { code }
    }

    pub fn low(&self) -> DwVfpRegister {
        DwVfpRegister::from_code(self.code * 2)
    }

    pub fn high(&self) -> DwVfpRegister {
        DwVfpRegister::from_code(self.code * 2 + 1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagsCondition {
    kEqual,
    kNotEqual,
    kSignedLessThan,
    kSignedGreaterThanOrEqual,
    kSignedLessThanOrEqual,
    kSignedGreaterThan,
    kUnsignedLessThan,
    kUnsignedGreaterThanOrEqual,
    kUnsignedLessThanOrEqual,
    kUnsignedGreaterThan,
    kFloatLessThanOrUnordered,
    kFloatGreaterThanOrEqual,
    kFloatLessThanOrEqual,
    kFloatGreaterThanOrUnordered,
    kFloatLessThan,
    kFloatGreaterThanOrEqualOrUnordered,
    kFloatLessThanOrEqualOrUnordered,
    kFloatGreaterThan,
    kOverflow,
    kNotOverflow,
    kPositiveOrZero,
    kNegative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RecordWriteMode {
    kNoOp,
    kValueIsPointer,
    kValueIsEphemeronKey,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StubCallMode {
    kCallWasmRuntimeStub
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtomicMemoryOrder {
    kSeqCst
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SaveFPRegsMode {
    kIgnore,
    kSave
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnwindingInfoWriter {
}

impl UnwindingInfoWriter {
    pub fn MarkFrameDeconstructed(&mut self, pc_offset: i32) {}
    pub fn MarkBlockWillExit(&mut self) {}
    pub fn MarkLinkRegisterOnTopOfStack(&mut self, pc_offset: i32) {}
    pub fn MarkPopLinkRegisterFromTopOfStack(&mut self, pc_offset: i32) {}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourceLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SBit {
    SetCC,
    LeaveCC,
}

// Adds Arm-specific methods to convert InstructionOperands.
struct ArmOperandConverter<'a> {
    gen: &'a CodeGenerator,
    instr_: &'a Instruction,
}

impl<'a> ArmOperandConverter<'a> {
    fn new(gen: &'a CodeGenerator, instr: &'a Instruction) -> Self {
        ArmOperandConverter { gen, instr_ : instr }
    }

    fn OutputSBit(&self) -> SBit {
        match self.instr_.flags_mode() {
            kFlags_branch | kFlags_conditional_branch | kFlags_deoptimize | kFlags_set
            | kFlags_conditional_set | kFlags_trap | kFlags_select => SBit::SetCC,
            kFlags_none => SBit::LeaveCC,
            _ => unreachable!(),
        }
    }

    fn InputImmediate(&self, index: usize) -> Operand {
        self.ToImmediate(self.instr_.InputAt(index))
    }

    fn InputOperand2(&self, first_index: usize) -> Operand {
        let index = first_index;
        match AddressingModeField::decode(self.instr_.opcode()) {
            AddressingMode::kMode_None | AddressingMode::kMode_Offset_RI | AddressingMode::kMode_Offset_RR | AddressingMode::kMode_Root => {
            }
            AddressingMode::kMode_Operand2_I => {
                return self.InputImmediate(index + 0);
            }
            AddressingMode::kMode_Operand2_R => {
                return Operand::new(self.InputRegister(index + 0).code, RelocInfo::CODE_TARGET);
            }
            AddressingMode::kMode_Operand2_R_ASR_I => {
                //TODO: return Operand::new(self.InputRegister(index + 0), ASR, self.InputInt5(index + 1));
            }
            AddressingMode::kMode_Operand2_R_ASR_R => {
                //TODO: return Operand::new(self.InputRegister(index + 0), ASR, self.InputRegister(index + 1));
            }
            AddressingMode::kMode_Operand2_R_LSL_I => {
                //TODO: return Operand::new(self.InputRegister(index + 0), LSL, self.InputInt5(index + 1));
            }
            AddressingMode::kMode_Operand2_R_LSL_R => {
                //TODO: return Operand::new(self.InputRegister(index + 0), LSL, self.InputRegister(index + 1));
            }
            AddressingMode::kMode_Operand2_R_LSR_I => {
                //TODO: return Operand::new(self.InputRegister(index + 0), LSR, self.InputInt5(index + 1));
            }
            AddressingMode::kMode_Operand2_R_LSR_R => {
                //TODO: return Operand::new(self.InputRegister(index + 0), LSR, self.InputRegister(index + 1));
            }
            AddressingMode::kMode_Operand2_R_ROR_I => {
                //TODO: return Operand::new(self.InputRegister(index + 0), ROR, self.InputInt5(index + 1));
            }
            AddressingMode::kMode_Operand2_R_ROR_R => {
                //TODO: return Operand::new(self.InputRegister(index + 0), ROR, self.InputRegister(index + 1));
            }
        }
        unreachable!();
    }

    fn InputOffset(&self, first_index: &mut usize) -> MemOperand {
        let index = *first_index;
        match AddressingModeField::decode(self.instr_.opcode()) {
            AddressingMode::kMode_None | AddressingMode::kMode_Operand2_I | AddressingMode::kMode_Operand2_R
            | AddressingMode::kMode_Operand2_R_ASR_I | AddressingMode::kMode_Operand2_R_ASR_R | AddressingMode::kMode_Operand2_R_LSL_R
            | AddressingMode::kMode_Operand2_R_LSR_I | AddressingMode::kMode_Operand2_R_LSR_R | AddressingMode::kMode_Operand2_R_ROR_I
            | AddressingMode::kMode_Operand2_R_ROR_R => {
            }
            AddressingMode::kMode_Operand2_R_LSL_I => {
                *first_index += 3;
                //TODO: return MemOperand::new(self.InputRegister(index + 0), self.InputRegister(index + 1), LSL, self.InputInt32(index + 2));
            }
            AddressingMode::kMode_Offset_RI => {
                *first_index += 2;
                return MemOperand::new(self.InputRegister(index + 0), self.InputInt32(index + 1));
            }
            AddressingMode::kMode_Offset_RR => {
                *first_index += 2;
                return MemOperand::new_with_registers(self.InputRegister(index + 0), self.InputRegister(index + 1));
            }
            AddressingMode::kMode_Root => {
                *first_index += 1;
                return MemOperand::new_root(kRootRegister, self.InputInt32(index));
            }
        }
        unreachable!();
    }

    fn InputOffset_idx(&self, first_index: usize) -> MemOperand {
        let mut idx = first_index;
        self.InputOffset(&mut idx)
    }

    fn ToImmediate(&self, operand: *const InstructionOperand) -> Operand {
        unsafe {
            let constant = self.ToConstant(operand.as_ref().unwrap());
            match constant.type_ {
                ConstantType::kInt32 => {
                    return Operand::new(constant.to_i32(), constant.rmode);
                }
                ConstantType::kFloat32 => {
                    