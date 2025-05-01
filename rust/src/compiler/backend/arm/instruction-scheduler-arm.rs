pub mod instruction_scheduler {
    /// Represents the architecture-specific opcode for ARM instructions.
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum ArchOpcode {
        ArmAdd,
        ArmAnd,
        ArmBic,
        ArmClz,
        ArmCmp,
        ArmCmn,
        ArmTst,
        ArmTeq,
        ArmOrr,
        ArmEor,
        ArmSub,
        ArmRsb,
        ArmMul,
        ArmMla,
        ArmMls,
        ArmSmmul,
        ArmSmull,
        ArmSmmla,
        ArmUmull,
        ArmSdiv,
        ArmUdiv,
        ArmMov,
        ArmMvn,
        ArmBfc,
        ArmUbfx,
        ArmSbfx,
        ArmSxtb,
        ArmSxth,
        ArmSxtab,
        ArmSxtah,
        ArmUxtb,
        ArmUxth,
        ArmUxtab,
        ArmUxtah,
        ArmRbit,
        ArmRev,
        ArmAddPair,
        ArmSubPair,
        ArmMulPair,
        ArmLslPair,
        ArmLsrPair,
        ArmAsrPair,
        ArmVcmpF32,
        ArmVaddF32,
        ArmVsubF32,
        ArmVmulF32,
        ArmVmlaF32,
        ArmVmlsF32,
        ArmVdivF32,
        ArmVabsF32,
        ArmVnegF32,
        ArmVsqrtF32,
        ArmVcmpF64,
        ArmVaddF64,
        ArmVsubF64,
        ArmVmulF64,
        ArmVmlaF64,
        ArmVmlsF64,
        ArmVdivF64,
        ArmVmodF64,
        ArmVabsF64,
        ArmVnegF64,
        ArmVsqrtF64,
        ArmVmullLow,
        ArmVmullHigh,
        ArmVrintmF32,
        ArmVrintmF64,
        ArmVrintpF32,
        ArmVrintpF64,
        ArmVrintzF32,
        ArmVrintzF64,
        ArmVrintaF64,
        ArmVrintnF32,
        ArmVrintnF64,
        ArmVcvtF32F64,
        ArmVcvtF64F32,
        ArmVcvtF32S32,
        ArmVcvtF32U32,
        ArmVcvtF64S32,
        ArmVcvtF64U32,
        ArmVcvtS32F32,
        ArmVcvtU32F32,
        ArmVcvtS32F64,
        ArmVcvtU32F64,
        ArmVmovU32F32,
        ArmVmovF32U32,
        ArmVmovLowU32F64,
        ArmVmovLowF64U32,
        ArmVmovHighU32F64,
        ArmVmovHighF64U32,
        ArmVmovF64U32U32,
        ArmVmovU32U32F64,
        ArmVcnt,
        ArmVpadal,
        ArmVpaddl,
        ArmFloat32Max,
        ArmFloat64Max,
        ArmFloat32Min,
        ArmFloat64Min,
        ArmFloat64SilenceNaN,
        ArmF64x2Splat,
        ArmF64x2ExtractLane,
        ArmF64x2ReplaceLane,
        ArmF64x2Abs,
        ArmF64x2Neg,
        ArmF64x2Sqrt,
        ArmF64x2Add,
        ArmF64x2Sub,
        ArmF64x2Mul,
        ArmF64x2Div,
        ArmF64x2Min,
        ArmF64x2Max,
        ArmF64x2Eq,
        ArmF64x2Ne,
        ArmF64x2Lt,
        ArmF64x2Le,
        ArmF64x2Qfma,
        ArmF64x2Qfms,
        ArmF64x2Pmin,
        ArmF64x2Pmax,
        ArmF64x2Ceil,
        ArmF64x2Floor,
        ArmF64x2Trunc,
        ArmF64x2NearestInt,
        ArmF64x2ConvertLowI32x4S,
        ArmF64x2ConvertLowI32x4U,
        ArmF64x2PromoteLowF32x4,
        ArmF32x4Splat,
        ArmF32x4ExtractLane,
        ArmF32x4ReplaceLane,
        ArmF32x4SConvertI32x4,
        ArmF32x4UConvertI32x4,
        ArmF32x4Abs,
        ArmF32x4Neg,
        ArmF32x4Sqrt,
        ArmF32x4Add,
        ArmF32x4Sub,
        ArmF32x4Mul,
        ArmF32x4Div,
        ArmF32x4Min,
        ArmF32x4Max,
        ArmF32x4Eq,
        ArmF32x4Ne,
        ArmF32x4Lt,
        ArmF32x4Le,
        ArmF32x4Qfma,
        ArmF32x4Qfms,
        ArmF32x4Pmin,
        ArmF32x4Pmax,
        ArmF32x4DemoteF64x2Zero,
        ArmI64x2SplatI32Pair,
        ArmI64x2ReplaceLaneI32Pair,
        ArmI64x2Abs,
        ArmI64x2Neg,
        ArmI64x2Shl,
        ArmI64x2ShrS,
        ArmI64x2Add,
        ArmI64x2Sub,
        ArmI64x2Mul,
        ArmI64x2ShrU,
        ArmI64x2BitMask,
        ArmI64x2Eq,
        ArmI64x2Ne,
        ArmI64x2GtS,
        ArmI64x2GeS,
        ArmI64x2SConvertI32x4Low,
        ArmI64x2SConvertI32x4High,
        ArmI64x2UConvertI32x4Low,
        ArmI64x2UConvertI32x4High,
        ArmI32x4Splat,
        ArmI32x4ExtractLane,
        ArmI32x4ReplaceLane,
        ArmI32x4SConvertF32x4,
        ArmI32x4SConvertI16x8Low,
        ArmI32x4SConvertI16x8High,
        ArmI32x4Neg,
        ArmI32x4Shl,
        ArmI32x4ShrS,
        ArmI32x4Add,
        ArmI32x4Sub,
        ArmI32x4Mul,
        ArmI32x4MinS,
        ArmI32x4MaxS,
        ArmI32x4Eq,
        ArmI32x4Ne,
        ArmI32x4GtS,
        ArmI32x4GeS,
        ArmI32x4UConvertF32x4,
        ArmI32x4UConvertI16x8Low,
        ArmI32x4UConvertI16x8High,
        ArmI32x4ShrU,
        ArmI32x4MinU,
        ArmI32x4MaxU,
        ArmI32x4GtU,
        ArmI32x4GeU,
        ArmI32x4Abs,
        ArmI32x4BitMask,
        ArmI32x4DotI16x8S,
        ArmI16x8DotI8x16S,
        ArmI32x4DotI8x16AddS,
        ArmI32x4TruncSatF64x2SZero,
        ArmI32x4TruncSatF64x2UZero,
        ArmI16x8Splat,
        ArmI16x8ExtractLaneS,
        ArmI16x8ReplaceLane,
        ArmI16x8SConvertI8x16Low,
        ArmI16x8SConvertI8x16High,
        ArmI16x8Neg,
        ArmI16x8Shl,
        ArmI16x8ShrS,
        ArmI16x8SConvertI32x4,
        ArmI16x8Add,
        ArmI16x8AddSatS,
        ArmI16x8Sub,
        ArmI16x8SubSatS,
        ArmI16x8Mul,
        ArmI16x8MinS,
        ArmI16x8MaxS,
        ArmI16x8Eq,
        ArmI16x8Ne,
        ArmI16x8GtS,
        ArmI16x8GeS,
        ArmI16x8ExtractLaneU,
        ArmI16x8UConvertI8x16Low,
        ArmI16x8UConvertI8x16High,
        ArmI16x8ShrU,
        ArmI16x8UConvertI32x4,
        ArmI16x8AddSatU,
        ArmI16x8SubSatU,
        ArmI16x8MinU,
        ArmI16x8MaxU,
        ArmI16x8GtU,
        ArmI16x8GeU,
        ArmI16x8RoundingAverageU,
        ArmI16x8Abs,
        ArmI16x8BitMask,
        ArmI16x8Q15MulRSatS,
        ArmI8x16Splat,
        ArmI8x16ExtractLaneS,
        ArmI8x16ReplaceLane,
        ArmI8x16Neg,
        ArmI8x16Shl,
        ArmI8x16ShrS,
        ArmI8x16SConvertI16x8,
        ArmI8x16Add,
        ArmI8x16AddSatS,
        ArmI8x16Sub,
        ArmI8x16SubSatS,
        ArmI8x16MinS,
        ArmI8x16MaxS,
        ArmI8x16Eq,
        ArmI8x16Ne,
        ArmI8x16GtS,
        ArmI8x16GeS,
        ArmI8x16ExtractLaneU,
        ArmI8x16UConvertI16x8,
        ArmI8x16AddSatU,
        ArmI8x16SubSatU,
        ArmI8x16ShrU,
        ArmI8x16MinU,
        ArmI8x16MaxU,
        ArmI8x16GtU,
        ArmI8x16GeU,
        ArmI8x16RoundingAverageU,
        ArmI8x16Abs,
        ArmI8x16BitMask,
        ArmS128Const,
        ArmS128Zero,
        ArmS128AllOnes,
        ArmS128Dup,
        ArmS128And,
        ArmS128Or,
        ArmS128Xor,
        ArmS128Not,
        ArmS128Select,
        ArmS128AndNot,
        ArmS32x4ZipLeft,
        ArmS32x4ZipRight,
        ArmS32x4UnzipLeft,
        ArmS32x4UnzipRight,
        ArmS32x4TransposeLeft,
        ArmS32x4TransposeRight,
        ArmS32x4Shuffle,
        ArmS16x8ZipLeft,
        ArmS16x8ZipRight,
        ArmS16x8UnzipLeft,
        ArmS16x8UnzipRight,
        ArmS16x8TransposeLeft,
        ArmS16x8TransposeRight,
        ArmS8x16ZipLeft,
        ArmS8x16ZipRight,
        ArmS8x16UnzipLeft,
        ArmS8x16UnzipRight,
        ArmS8x16TransposeLeft,
        ArmS8x16TransposeRight,
        ArmS8x16Concat,
        ArmI8x16Swizzle,
        ArmI8x16Shuffle,
        ArmS32x2Reverse,
        ArmS16x4Reverse,
        ArmS16x2Reverse,
        ArmS8x8Reverse,
        ArmS8x4Reverse,
        ArmS8x2Reverse,
        ArmI64x2AllTrue,
        ArmI32x4AllTrue,
        ArmI16x8AllTrue,
        ArmV128AnyTrue,
        ArmI8x16AllTrue,
        ArmVldrF32,
        ArmVldrF64,
        ArmVld1F64,
        ArmVld1S128,
        ArmLdrb,
        ArmLdrsb,
        ArmLdrh,
        ArmLdrsh,
        ArmLdr,
        ArmPeek,
        ArmWord32AtomicPairLoad,
        ArmS128Load8Splat,
        ArmS128Load16Splat,
        ArmS128Load32Splat,
        ArmS128Load64Splat,
        ArmS128Load8x8S,
        ArmS128Load8x8U,
        ArmS128Load16x4S,
        ArmS128Load16x4U,
        ArmS128Load32x2S,
        ArmS128Load32x2U,
        ArmS128Load32Zero,
        ArmS128Load64Zero,
        ArmS128LoadLaneLow,
        ArmS128LoadLaneHigh,
        ArmVstrF32,
        ArmVstrF64,
        ArmVst1F64,
        ArmVst1S128,
        ArmStrb,
        ArmStrh,
        ArmStr,
        ArmPush,
        ArmPoke,
        ArmDmbIsh,
        ArmDsbIsb,
        ArmWord32AtomicPairStore,
        ArmWord32AtomicPairAdd,
        ArmWord32AtomicPairSub,
        ArmWord32AtomicPairAnd,
        ArmWord32AtomicPairOr,
        ArmWord32AtomicPairXor,
        ArmWord32AtomicPairExchange,
        ArmWord32AtomicPairCompareExchange,
        ArmS128StoreLaneLow,
        ArmS128StoreLaneHigh,
        // TODO: Add the rest of the opcodes from COMMON_ARCH_OPCODE_LIST
    }

    /// Represents an instruction.  The actual definition would be more involved.
    #[derive(Debug)]
    pub struct Instruction {
        arch_opcode: ArchOpcode,
    }

    impl Instruction {
        pub fn new(arch_opcode: ArchOpcode) -> Self {
            Instruction { arch_opcode }
        }

        pub fn arch_opcode(&self) -> ArchOpcode {
            self.arch_opcode
        }
    }

    pub const K_NO_OPCODE_FLAGS: i32 = 0;
    pub const K_IS_LOAD_OPERATION: i32 = 1 << 0;
    pub const K_HAS_SIDE_EFFECT: i32 = 1 << 1;

    /// The instruction scheduler.
    pub struct InstructionScheduler {}

    impl InstructionScheduler {
        /// Checks if the scheduler is supported.
        pub fn scheduler_supported() -> bool {
            true
        }

        /// Gets the target instruction flags for the given instruction.
        pub fn get_target_instruction_flags(&self, instr: &Instruction) -> i32 {
            match instr.arch_opcode() {
                ArchOpcode::ArmAdd
                | ArchOpcode::ArmAnd
                | ArchOpcode::ArmBic
                | ArchOpcode::ArmClz
                | ArchOpcode::ArmCmp
                | ArchOpcode::ArmCmn
                | ArchOpcode::ArmTst
                | ArchOpcode::ArmTeq
                | ArchOpcode::ArmOrr
                | ArchOpcode::ArmEor
                | ArchOpcode::ArmSub
                | ArchOpcode::ArmRsb
                | ArchOpcode::ArmMul
                | ArchOpcode::ArmMla
                | ArchOpcode::ArmMls
                | ArchOpcode::ArmSmmul
                | ArchOpcode::ArmSmull
                | ArchOpcode::ArmSmmla
                | ArchOpcode::ArmUmull
                | ArchOpcode::ArmSdiv
                | ArchOpcode::ArmUdiv
                | ArchOpcode::ArmMov
                | ArchOpcode::ArmMvn
                | ArchOpcode::ArmBfc
                | ArchOpcode::ArmUbfx
                | ArchOpcode::ArmSbfx
                | ArchOpcode::ArmSxtb
                | ArchOpcode::ArmSxth
                | ArchOpcode::ArmSxtab
                | ArchOpcode::ArmSxtah
                | ArchOpcode::ArmUxtb
                | ArchOpcode::ArmUxth
                | ArchOpcode::ArmUxtab
                | ArchOpcode::ArmUxtah
                | ArchOpcode::ArmRbit
                | ArchOpcode::ArmRev
                | ArchOpcode::ArmAddPair
                | ArchOpcode::ArmSubPair
                | ArchOpcode::ArmMulPair
                | ArchOpcode::ArmLslPair
                | ArchOpcode::ArmLsrPair
                | ArchOpcode::ArmAsrPair
                | ArchOpcode::ArmVcmpF32
                | ArchOpcode::ArmVaddF32
                | ArchOpcode::ArmVsubF32
                | ArchOpcode::ArmVmulF32
                | ArchOpcode::ArmVmlaF32
                | ArchOpcode::ArmVmlsF32
                | ArchOpcode::ArmVdivF32
                | ArchOpcode::ArmVabsF32
                | ArchOpcode::ArmVnegF32
                | ArchOpcode::ArmVsqrtF32
                | ArchOpcode::ArmVcmpF64
                | ArchOpcode::ArmVaddF64
                | ArchOpcode::ArmVsubF64
                | ArchOpcode::ArmVmulF64
                | ArchOpcode::ArmVmlaF64
                | ArchOpcode::ArmVmlsF64
                | ArchOpcode::ArmVdivF64
                | ArchOpcode::ArmVmodF64
                | ArchOpcode::ArmVabsF64
                | ArchOpcode::ArmVnegF64
                | ArchOpcode::ArmVsqrtF64
                | ArchOpcode::ArmVmullLow
                | ArchOpcode::ArmVmullHigh
                | ArchOpcode::ArmVrintmF32
                | ArchOpcode::ArmVrintmF64
                | ArchOpcode::ArmVrintpF32
                | ArchOpcode::ArmVrintpF64
                | ArchOpcode::ArmVrintzF32
                | ArchOpcode::ArmVrintzF64
                | ArchOpcode::ArmVrintaF64
                | ArchOpcode::ArmVrintnF32
                | ArchOpcode::ArmVrintnF64
                | ArchOpcode::ArmVcvtF32F64
                | ArchOpcode::ArmVcvtF64F32
                | ArchOpcode::ArmVcvtF32S32
                | ArchOpcode::ArmVcvtF32U32
                | ArchOpcode::ArmVcvtF64S32
                | ArchOpcode::ArmVcvtF64U32
                | ArchOpcode::ArmVcvtS32F32
                | ArchOpcode::ArmVcvtU32F32
                | ArchOpcode::ArmVcvtS32F64
                | ArchOpcode::ArmVcvtU32F64
                | ArchOpcode::ArmVmovU32F32
                | ArchOpcode::ArmVmovF32U32
                | ArchOpcode::ArmVmovLowU32F64
                | ArchOpcode::ArmVmovLowF64U32
                | ArchOpcode::ArmVmovHighU32F64
                | ArchOpcode::ArmVmovHighF64U32
                | ArchOpcode::ArmVmovF64U32U32
                | ArchOpcode::ArmVmovU32U32F64
                | ArchOpcode::ArmVcnt
                | ArchOpcode::ArmVpadal
                | ArchOpcode::ArmVpaddl
                | ArchOpcode::ArmFloat32Max
                | ArchOpcode::ArmFloat64Max
                | ArchOpcode::ArmFloat32Min
                | ArchOpcode::ArmFloat64Min
                | ArchOpcode::ArmFloat64SilenceNaN
                | ArchOpcode::ArmF64x2Splat
                | ArchOpcode::ArmF64x2ExtractLane
                | ArchOpcode::ArmF64x2ReplaceLane
                | ArchOpcode::ArmF64x2Abs
                | ArchOpcode::ArmF64x2Neg
                | ArchOpcode::ArmF64x2Sqrt
                | ArchOpcode::ArmF64x2Add
                | ArchOpcode::ArmF64x2Sub
                | ArchOpcode::ArmF64x2Mul
                | ArchOpcode::ArmF64x2Div
                | ArchOpcode::ArmF64x2Min
                | ArchOpcode::ArmF64x2Max
                | ArchOpcode::ArmF64x2Eq
                | ArchOpcode::ArmF64x2Ne
                | ArchOpcode::ArmF64x2Lt
                | ArchOpcode::ArmF64x2Le
                | ArchOpcode::ArmF64x2Qfma
                | ArchOpcode::ArmF64x2Qfms
                | ArchOpcode::ArmF64x2Pmin
                | ArchOpcode::ArmF64x2Pmax
                | ArchOpcode::ArmF64x2Ceil
                | ArchOpcode::ArmF64x2Floor
                | ArchOpcode::ArmF64x2Trunc
                | ArchOpcode::ArmF64x2NearestInt
                | ArchOpcode::ArmF64x2ConvertLowI32x4S
                | ArchOpcode::ArmF64x2ConvertLowI32x4U
                | ArchOpcode::ArmF64x2PromoteLowF32x4
                | ArchOpcode::ArmF32x4Splat
                | ArchOpcode::ArmF32x4ExtractLane
                | ArchOpcode::ArmF32x4ReplaceLane
                | ArchOpcode::ArmF32x4SConvertI32x4
                | ArchOpcode::ArmF32x4UConvertI32x4
                | ArchOpcode::ArmF32x4Abs
                | ArchOpcode::ArmF32x4Neg
                | ArchOpcode::ArmF32x4Sqrt
                | ArchOpcode::ArmF32x4Add
                | ArchOpcode::ArmF32x4Sub
                | ArchOpcode::ArmF32x4Mul
                | ArchOpcode::ArmF32x4Div
                | ArchOpcode::ArmF32x4Min
                | ArchOpcode::ArmF32x4Max
                | ArchOpcode::ArmF32x4Eq
                | ArchOpcode::ArmF32x4Ne
                | ArchOpcode::ArmF32x4Lt
                | ArchOpcode::ArmF32x4Le
                | ArchOpcode::ArmF32x4Qfma
                | ArchOpcode::ArmF32x4Qfms
                | ArchOpcode::ArmF32x4Pmin
                | ArchOpcode::ArmF32x4Pmax
                | ArchOpcode::ArmF32x4DemoteF64x2Zero
                | ArchOpcode::ArmI64x2SplatI32Pair
                | ArchOpcode::ArmI64x2ReplaceLaneI32Pair
                | ArchOpcode::ArmI64x2Abs
                | ArchOpcode::ArmI64x2Neg
                | ArchOpcode::ArmI64x2Shl
                | ArchOpcode::ArmI64x2ShrS
                | ArchOpcode::ArmI64x2Add
                | ArchOpcode::ArmI64x2Sub
                | ArchOpcode::ArmI64x2Mul
                | ArchOpcode::ArmI64x2ShrU
                | ArchOpcode::ArmI64x2BitMask
                | ArchOpcode::ArmI64x2Eq
                | ArchOpcode::ArmI64x2Ne
                | ArchOpcode::ArmI64x2GtS
                | ArchOpcode::ArmI64x2GeS
                | ArchOpcode::ArmI64x2SConvertI32x4Low
                | ArchOpcode::ArmI64x2SConvertI32x4High
                | ArchOpcode::ArmI64x2UConvertI32x4Low
                | ArchOpcode::ArmI64x2UConvertI32x4High
                | ArchOpcode::ArmI32x4Splat
                | ArchOpcode::ArmI32x4ExtractLane
                | ArchOpcode::ArmI32x4ReplaceLane
                | ArchOpcode::ArmI32x4SConvertF32x4
                | ArchOpcode::ArmI32x4SConvertI16x8Low
                | ArchOpcode::ArmI32x4SConvertI16x8High
                | ArchOpcode::ArmI32x4Neg
                | ArchOpcode::ArmI32x4Shl
                | ArchOpcode::ArmI32x4ShrS
                | ArchOpcode::ArmI32x4Add
                | ArchOpcode::ArmI32x4Sub
                | ArchOpcode::ArmI32x4Mul
                | ArchOpcode::ArmI32x4MinS
                | ArchOpcode::ArmI32x4MaxS
                | ArchOpcode::ArmI32x4Eq
                | ArchOpcode::ArmI32x4Ne
                | ArchOpcode::ArmI32x4GtS
                | ArchOpcode::ArmI32x4GeS
                | ArchOpcode::ArmI32x4UConvertF32x4
                | ArchOpcode::ArmI32x4UConvertI16x8Low
                | ArchOpcode::ArmI32x4UConvertI16x8High
                | ArchOpcode::ArmI32x4ShrU
                | ArchOpcode::ArmI32x4MinU
                | ArchOpcode::ArmI32x4MaxU
                | ArchOpcode::ArmI32x4GtU
                | ArchOpcode::ArmI32x4GeU
                | ArchOpcode::ArmI32x4Abs
                | ArchOpcode::ArmI32x4BitMask
                | ArchOpcode::ArmI32x4DotI16x8S
                | ArchOpcode::ArmI16x8DotI8x16S
                | ArchOpcode::ArmI32x4DotI8x16AddS
                | ArchOpcode::ArmI32x4TruncSatF64x2SZero
                | ArchOpcode::ArmI32x4TruncSatF64x2UZero
                | ArchOpcode::ArmI16x8Splat
                | ArchOpcode::ArmI16x8ExtractLaneS
                | ArchOpcode::ArmI16x8ReplaceLane
                | ArchOpcode::ArmI16x8SConvertI8x16Low
                | ArchOpcode::ArmI16x8SConvertI8x16High
                | ArchOpcode::ArmI16x8Neg
                | ArchOpcode::ArmI16x8Shl
                | ArchOpcode::ArmI16x8ShrS
                | ArchOpcode::ArmI16x8SConvertI32x4
                | ArchOpcode::ArmI16x8Add
                | ArchOpcode::ArmI16x8AddSatS
                | ArchOpcode::ArmI16x8Sub
                | ArchOpcode::ArmI16x8SubSatS
                | ArchOpcode::ArmI16x8Mul
                | ArchOpcode::ArmI16x8MinS
                | ArchOpcode::ArmI16x8MaxS
                | ArchOpcode::ArmI16x8Eq
                | ArchOpcode::ArmI16x8Ne
                | ArchOpcode::ArmI16x8GtS
                | ArchOpcode::ArmI16x8GeS
                | ArchOpcode::ArmI16x8ExtractLaneU
                | ArchOpcode::ArmI16x8UConvertI8x16Low
                | ArchOpcode::ArmI16x8UConvertI8x16High
                | ArchOpcode::ArmI16x8ShrU
                | ArchOpcode::ArmI16x8UConvertI32x4
                | ArchOpcode::ArmI16x8AddSatU
                | ArchOpcode::ArmI16x8SubSatU
                | ArchOpcode::ArmI16x8MinU
                | ArchOpcode::ArmI16x8MaxU
                | ArchOpcode::ArmI16x8GtU
                | ArchOpcode::ArmI16x8GeU
                | ArchOpcode::ArmI16x8RoundingAverageU
                | ArchOpcode::ArmI16x8Abs
                | ArchOpcode::ArmI16x8BitMask
                | ArchOpcode::ArmI16x8Q15MulRSatS
                | ArchOpcode::ArmI8x16Splat
                | ArchOpcode::ArmI8x16ExtractLaneS
                | ArchOpcode::ArmI8x16ReplaceLane
                | ArchOpcode::ArmI8x16Neg
                | ArchOpcode::ArmI8x16Shl
                | ArchOpcode::ArmI8x16ShrS
                | ArchOpcode::ArmI8x16SConvertI16x8
                | ArchOpcode::ArmI8x16Add
                | ArchOpcode::ArmI8x16AddSatS
                | ArchOpcode::ArmI8x16Sub
                | ArchOpcode::ArmI8x16SubSatS
                | ArchOpcode::ArmI8x16MinS
                | ArchOpcode::ArmI8x16MaxS
                | ArchOpcode::ArmI8x16Eq
                | ArchOpcode::ArmI8x16Ne
                | ArchOpcode::ArmI8x16GtS
                | ArchOpcode::ArmI8x16GeS
                | ArchOpcode::ArmI8x16ExtractLaneU
                | ArchOpcode::ArmI8x16UConvertI16x8
                | ArchOpcode::ArmI8x16AddSatU
                | ArchOpcode::ArmI8x16SubSatU
                | ArchOpcode::ArmI8x16ShrU
                | ArchOpcode::ArmI8x16MinU
                | ArchOpcode::ArmI8x16MaxU
                | ArchOpcode::ArmI8x16GtU
                | ArchOpcode::ArmI8x16GeU
                | ArchOpcode::ArmI8x16RoundingAverageU
                | ArchOpcode::ArmI8x16Abs
                | ArchOpcode::ArmI8x16BitMask
                | ArchOpcode::ArmS128Const
                | ArchOpcode::ArmS128Zero
                | ArchOpcode::ArmS128AllOnes
                | ArchOpcode::ArmS128Dup
                | ArchOpcode::ArmS128And
                | ArchOpcode::ArmS128Or
                | ArchOpcode::ArmS128Xor
                | ArchOpcode::ArmS128Not
                | ArchOpcode::ArmS128Select
                | ArchOpcode::ArmS128AndNot
                | ArchOpcode::ArmS32x4ZipLeft
                | ArchOpcode::ArmS32x4ZipRight
                | ArchOpcode::ArmS32x4UnzipLeft
                | ArchOpcode::ArmS32x4UnzipRight
                | ArchOpcode::ArmS32x4TransposeLeft
                | ArchOpcode::ArmS32x4TransposeRight
                | ArchOpcode::ArmS32x4Shuffle
                | ArchOpcode::ArmS16x8ZipLeft
                | ArchOpcode::ArmS16x8ZipRight
                | ArchOpcode::ArmS16x8UnzipLeft