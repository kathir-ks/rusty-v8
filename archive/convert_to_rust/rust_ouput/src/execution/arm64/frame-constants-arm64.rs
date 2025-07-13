// Converted from V8 C++ source files:
// Header: frame-constants-arm64.h
// Implementation: frame-constants-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arm64 {
pub mod frame_constants_arm64 {
use crate::codegen::register::Register;
use crate::execution::frame_constants::AllStatic;
use crate::execution::frame_constants::TypedFrameConstants;

// The layout of an EntryFrame is as follows:
//
//         BOTTOM OF THE STACK   HIGHEST ADDRESS
//  slot      Entry frame
//       +---------------------+-----------------------
// -19   | saved register d15  |
// ...   |        ...          |
// -12   | saved register d8   |
//       |- - - - - - - - - - -|
// -11   | saved register x28  |
// ...   |        ...          |
//  -2   | saved register x19  |
//       |- - - - - - - - - - -|
//  -1   |   saved lr (x30)    |
//       |- - - - - - - - - - -|
//   0   |   saved fp (x29)    |  <-- frame ptr
//       |- - - - - - - - - - -|
//   1   | stack frame marker  |
//       |      (ENTRY)        |
//       |- - - - - - - - - - -|
//   2   | stack frame marker  |
//       |        (0)          |
//       |- - - - - - - - - - -|
//   3   |     C entry FP      |
//       |- - - - - - - - - - -|
//   4   |   JS entry frame    |
//       |       marker        |
//       |- - - - - - - - - - -|
//   5   |  fast api call fp   |
//       |- - - - - - - - - - -|
//   6   |  fast api call pc   |  <-- stack ptr
//  -----+---------------------+-----------------------
//          TOP OF THE STACK     LOWEST ADDRESS
//
pub struct EntryFrameConstants {}

impl EntryFrameConstants {
  // This is the offset to where JSEntry pushes the current value of
  // Isolate::c_entry_fp onto the stack.
  pub const kNextExitFrameFPOffset: i32 = -3 * kSystemPointerSize;
  // The offsets for storing the FP and PC of fast API calls.
  pub const kNextFastCallFrameFPOffset: i32 = -5 * kSystemPointerSize;
  pub const kNextFastCallFramePCOffset: i32 = -6 * kSystemPointerSize;

  pub const kFixedFrameSize: i32 = 6 * kSystemPointerSize;

  // The following constants are defined so we can static-assert their values
  // near the relevant JSEntry assembly code, not because they're actually very
  // useful.
  pub const kCalleeSavedRegisterBytesPushedBeforeFpLrPair: i32 =
    18 * kSystemPointerSize;
  pub const kCalleeSavedRegisterBytesPushedAfterFpLrPair: i32 = 0;
  pub const kOffsetToCalleeSavedRegisters: i32 = 0;

  // These offsets refer to the immediate caller (a native frame), not to the
  // previous JS exit frame like kCallerFPOffset above.
  pub const kDirectCallerFPOffset: i32 =
    Self::kCalleeSavedRegisterBytesPushedAfterFpLrPair +
    Self::kOffsetToCalleeSavedRegisters;
  pub const kDirectCallerPCOffset: i32 =
    Self::kDirectCallerFPOffset + 1 * kSystemPointerSize;
  pub const kDirectCallerSPOffset: i32 =
    Self::kDirectCallerPCOffset + 1 * kSystemPointerSize +
    Self::kCalleeSavedRegisterBytesPushedBeforeFpLrPair;
}

impl AllStatic for EntryFrameConstants {}

pub struct WasmLiftoffSetupFrameConstants {}

impl WasmLiftoffSetupFrameConstants {
  // Number of gp parameters, without the instance.
  pub const kNumberOfSavedGpParamRegs: i32 = 6;
  pub const kNumberOfSavedFpParamRegs: i32 = 8;

  // On arm, spilled registers are implicitly sorted backwards by number.
  // We spill:
  //   x0, x2, x3, x4, x5, x6: param1, param2, ..., param6
  // in the following FP-relative order: [x6, x5, x4, x3, x2, x0].
  // The instance slot is in position '0', the first spill slot is at '1'.
  pub const kInstanceSpillOffset: i32 = TYPED_FRAME_PUSHED_VALUE_OFFSET(0);

  pub const kParameterSpillsOffset: [i32; 6] = [
    TYPED_FRAME_PUSHED_VALUE_OFFSET(6),
    TYPED_FRAME_PUSHED_VALUE_OFFSET(5),
    TYPED_FRAME_PUSHED_VALUE_OFFSET(4),
    TYPED_FRAME_PUSHED_VALUE_OFFSET(3),
    TYPED_FRAME_PUSHED_VALUE_OFFSET(2),
    TYPED_FRAME_PUSHED_VALUE_OFFSET(1),
  ];

  // SP-relative.
  pub const kWasmInstanceDataOffset: i32 = 2 * kSystemPointerSize;
  pub const kDeclaredFunctionIndexOffset: i32 = 1 * kSystemPointerSize;
  pub const kNativeModuleOffset: i32 = 0;
}

impl TypedFrameConstants for WasmLiftoffSetupFrameConstants {}

pub struct WasmLiftoffFrameConstants {}

impl WasmLiftoffFrameConstants {
  pub const kFeedbackVectorOffset: i32 = 3 * kSystemPointerSize;
  pub const kInstanceDataOffset: i32 = 2 * kSystemPointerSize;
}

impl TypedFrameConstants for WasmLiftoffFrameConstants {}

// Frame constructed by the {WasmDebugBreak} builtin.
// After pushing the frame type marker, the builtin pushes all Liftoff cache
// registers (see liftoff-assembler-defs.h).
pub struct WasmDebugBreakFrameConstants {}

impl WasmDebugBreakFrameConstants {
  // x16: ip0, x17: ip1, x18: platform register, x26: root, x28: base, x29: fp,
  // x30: lr, x31: xzr.
  // static constexpr RegList kPushedGpRegs = {
  //     x0,  x1,  x2,  x3,  x4,  x5,  x6,  x7,  x8,  x9,  x10, x11,
  //     x12, x13, x14, x15, x19, x20, x21, x22, x23, x24, x25, x27};
  //
  // // We push FpRegs as 128-bit SIMD registers, so 16-byte frame alignment
  // // is guaranteed regardless of register count.
  // static constexpr DoubleRegList kPushedFpRegs = {
  //     d0,  d1,  d2,  d3,  d4,  d5,  d6,  d7,  d8,  d9,  d10, d11, d12, d13,
  //     d14, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27};

  // Assuming RegList and DoubleRegList are bitsets
  pub const kPushedGpRegs: u32 = 0b11111111111111110000011111111;
  pub const kPushedFpRegs: u64 = 0xFFFFFFFFFFFFFFF; // Example value, adjust as needed
  
  pub const kNumPushedGpRegisters: i32 = Self::count_set_bits(Self::kPushedGpRegs);
  //static_assert(kNumPushedGpRegisters % 2 == 0,"stack frames need to be 16-byte aligned");
  
  pub const kNumPushedFpRegisters: i32 = Self::count_set_bits(Self::kPushedFpRegs as u32);

  pub const kLastPushedGpRegisterOffset: i32 =
    // Header is padded to 16 byte (see {MacroAssembler::EnterFrame}).
    -round_up(TypedFrameConstants::kFixedFrameSizeFromFp, 16) -
    kSystemPointerSize * Self::kNumPushedGpRegisters;
  pub const kLastPushedFpRegisterOffset: i32 =
    Self::kLastPushedGpRegisterOffset - kSimd128Size * Self::kNumPushedFpRegisters;

  // Offsets are fp-relative.
  pub fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
    if (Self::kPushedGpRegs & (1 << reg_code)) == 0 {
      panic!("DCHECK_NE(0, kPushedGpRegs.bits() & (1 << reg_code)) failed");
    }
    let lower_regs: u32 = Self::kPushedGpRegs & ((1u32 << reg_code) - 1);
    Self::kLastPushedGpRegisterOffset +
      Self::count_set_bits(lower_regs) * kSystemPointerSize
  }

  pub fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
    if (Self::kPushedFpRegs & (1 << reg_code)) == 0 {
       panic!("DCHECK_NE(0, kPushedFpRegs.bits() & (1 << reg_code)) failed");
    }
    let lower_regs: u32 = (Self::kPushedFpRegs & ((1u64 << reg_code) - 1)) as u32;
    Self::kLastPushedFpRegisterOffset +
      Self::count_set_bits(lower_regs) * kSimd128Size
  }
  
  fn count_set_bits(bits: u32) -> i32 {
      bits.count_ones() as i32
  }
}

impl TypedFrameConstants for WasmDebugBreakFrameConstants {}

// Placeholder for architecture-specific register definitions.
const fp: Register = Register { code: 29 };
const cp: Register = Register { code: 18 };

// Placeholder for architecture-specific constants.
const kSystemPointerSize: i32 = 8;
const kSimd128Size: i32 = 16;

// Placeholder for architecture-specific constants.
fn round_up(value: i32, alignment: i32) -> i32 {
  (value + alignment - 1) / alignment * alignment
}

// Placeholder for architecture-specific macros.
macro_rules! TYPED_FRAME_PUSHED_VALUE_OFFSET {
    ($slot_index:expr) => {
        (($slot_index + 1) * kSystemPointerSize)
    };
}
const TYPED_FRAME_PUSHED_VALUE_OFFSET : fn(i32) -> i32 = |slot_index| (slot_index + 1) * kSystemPointerSize;
}  // namespace frame_constants_arm64
} //namespace arm64

use crate::codegen::register::Register;

pub mod internal {
    use super::arm64::frame_constants_arm64::*;

    pub struct JavaScriptFrame {}

    impl JavaScriptFrame {
        pub fn fp_register() -> Register {
            fp
        }
        pub fn context_register() -> Register {
            cp
        }
        pub fn constant_pool_pointer_register() -> Register {
            panic!("UNREACHABLE");
        }
    }

    pub struct UnoptimizedFrameConstants {}

    impl UnoptimizedFrameConstants {
        pub fn register_stack_slot_count(register_count: i32) -> i32 {
            //static_assert(InterpreterFrameConstants::kFixedFrameSize % 16 == 0);
            // Round up to a multiple of two, to make the frame a multiple of 16 bytes.
            round_up(register_count, 2)
        }
    }

    pub struct BuiltinContinuationFrameConstants {}

    impl BuiltinContinuationFrameConstants {
        pub const kFixedSlotCount: i32 = 4; // Example value
        pub fn padding_slot_count(register_count: i32) -> i32 {
            // Round the total slot count up to a multiple of two, to make the frame a
            // multiple of 16 bytes.
            let slot_count = Self::kFixedSlotCount + register_count;
            let rounded_slot_count = round_up(slot_count, 2);
            rounded_slot_count - slot_count
        }
    }

    pub struct StandardFrameConstants {}
    impl StandardFrameConstants {
        pub const kFixedSlotCountFromFp: i32 = 5; // example
    }

    pub struct MaglevFrame {}

    impl MaglevFrame {
        // static
        pub fn stack_guard_frame_size(register_input_count: i32) -> i64 {
            // Include any paddings from kFixedFrameSizeFromFp, an extra slot + padding
            // for the single argument into StackGuardWithGap and finally padded register
            // input count.
            let slot_count = round_up(StandardFrameConstants::kFixedSlotCountFromFp, 2) +
                                 2 /* argument */ + round_up(register_input_count, 2);
            (slot_count * kSystemPointerSize) as i64
        }
    }
}
