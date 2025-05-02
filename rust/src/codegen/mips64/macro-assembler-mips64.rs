// Note: This is a partial conversion due to the complexity and size of the
// original C++ file. Some parts may be stubbed out, and further work would be
// needed for a complete, functional Rust equivalent. Additionally, many V8-specific
// types are not defined here and would need to be implemented or mocked.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

//use std::os::raw::*;
//use libc::*;

// Placeholder types and constants.  These need to be replaced with
// actual Rust definitions.
type RegList = u64;
type DoubleRegList = u64;
type Register = u32;
type FPURegister = u32;
type MSARegister = u32;
type Operand = i64;
type Condition = u32;
type FPUCondition = u32;
type MSABranchCondition = u32;
type BranchDelaySlot = u32;
type Address = usize;
type RootIndex = u32;
type IsolateFieldId = u32;
type MSASize = u32;
type MSADataType = u32;
type MSABranchDF = u32;
type SecondaryField = u32;
type Builtins = u32;
type RAStatus = u32;
type SaveFPRegsMode = u32;
type SmiCheck = u32;
type StubCallMode = u32;
type LiFlags = u32;
type AbortReason = u32;
type FPURoundingMode = u32;

const kPointerSize: i32 = 8;
const kDoubleSize: i32 = 8;
const kSimd128Size: i32 = 16;
const kHeapObjectTag: i64 = 1; // Dummy value
const kZapValue: i64 = 2;      // Dummy value
const kImm16Mask: i64 = 0xFFFF;
const kUpper16MaskOf64: i64 = 0xFFFF00000000;
const kHigher16MaskOf64: i64 = 0xFFFF000000000000;
const kLuiShift: i32 = 16;
const kRootRegister: Register = 28; // Example value
const kJSCallerSaved: RegList = 0xFF; // Example value.  Need real bitmask.
const kCallerSavedFPU: DoubleRegList = 0xFF;
const zero_reg: Register = 0;
const at: Register = 1;
const t8: Register = 8;
const t9: Register = 9;
const s6: Register = 22;
const kNumRegisters: i32 = 32;
const kDoubleCompareReg: FPURegister = 0;
const kScratchReg: Register = 10;
const kFCSROverflowCauseMask: u32 = 0x1;
const kFCSRUnderflowCauseMask: u32 = 0x2;
const kFCSRInvalidOpCauseMask: u32 = 0x4;
const OPTIMIZE_SIZE: LiFlags = 0;
const ADDRESS_LOAD: LiFlags = 1;
const CONSTANT_SIZE: LiFlags = 2;
const PROTECT: BranchDelaySlot = 0;
const D: SecondaryField = 1;
const W: SecondaryField = 2;
const L: SecondaryField = 3;
const SmiCheck_kInline: SmiCheck = 1;
const SmiCheck_kOmit: SmiCheck = 2;
const kArchVariant_kMips64r6: i32 = 0;
const kArchVariant_kMips64r2: i32 = 1;
const kArchVariant: i32 = kArchVariant_kMips64r6;
const eq: Condition = 0;
const ne: Condition = 1;
const lt: Condition = 2;
const gt: Condition = 3;
const le: Condition = 4;
const ge: Condition = 5;
const lo: Condition = 6;
const hs: Condition = 7;
const hi: Condition = 8;
const ls: Condition = 9;
const ULT: FPUCondition = 0;
const UN: FPUCondition = 1;
const ULE: FPUCondition = 2;
const mode_floor: FPURoundingMode = 0;
const mode_ceil: FPURoundingMode = 1;
const mode_trunc: FPURoundingMode = 2;
const mode_round: FPURoundingMode = 3;
const kRoundToNearest: FPURoundingMode = 4;
const all_not_zero: MSABranchCondition = 0;
const one_elem_not_zero: MSABranchCondition = 1;
const one_elem_zero: MSABranchCondition = 2;
const all_zero: MSABranchCondition = 3;
const RootIndex_kBuiltinsExit: RootIndex = 1;
const ExternalReferenceTable = u32;
const MSACSR: u32 = 0;
const kRoundDown: i32 = 0;

macro_rules! DCHECK {
    ($x:expr) => {
        if !$x {
            panic!("DCHECK failed: {}", stringify!($x));
        }
    };
}

macro_rules! CHECK {
  ($x:expr) => {
      if !$x {
          panic!("CHECK failed: {}", stringify!($x));
      }
  };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! ASM_CODE_COMMENT {
    ($masm:ident) => {
        // Placeholder - real implementation would insert a comment into the
        // assembly stream.
        println!("ASM_CODE_COMMENT");
    };
}

mod base {
    pub mod bits {
        pub fn IsPowerOfTwo(x: i64) -> bool {
            (x > 0) && ((x & (x - 1)) == 0)
        }

        pub fn CountTrailingZeros64(x: i64) -> i32 {
            if x == 0 {
              64
            } else {
              x.trailing_zeros() as i32
            }
        }
    }

  pub mod Double {
    pub const kSignMask: i64 = 0x8000000000000000;
  }

  pub fn bit_cast<T, U>(x: T) -> U {
      unsafe { std::mem::transmute_copy(&x) }
  }
}

mod internal {
    pub mod logging {
        pub mod counters {
            pub struct Counters {}

            impl Counters {
                pub fn new() -> Counters {
                    Counters {}
                }
            }
        }
    }

    pub mod builtins {
      pub fn RecordWrite(fp_mode: SaveFPRegsMode) -> Builtins {
        0
      }

      pub fn EphemeronKeyBarrier(fp_mode: SaveFPRegsMode) -> Builtins {
        1
      }

    }

    pub mod wasm {
      pub mod WasmCode {
        pub fn GetRecordWriteBuiltin(fp_mode: SaveFPRegsMode) -> i32 {
          0
        }
      }
    }

    pub mod objects {
        pub mod heap_number {
            pub const kExponentShift: u16 = 52;
            pub const kExponentBits: u16 = 11;
            pub const kMantissaBits: i32 = 52;
            pub const kExponentBias: i32 = 1023;
        }
    }

    pub mod heap {
        pub mod mutable_page_metadata {
            pub const kPointersToHereAreInterestingMask: u32 = 0x1;
            pub const kPointersFromHereAreInterestingMask: u32 = 0x2;
        }
    }

    pub mod execution {
        pub mod frames_inl {
            pub struct StackFrame {}
        }
    }

    pub mod debug {
        pub mod debug {
            pub fn set_break_point() {}
        }
    }

    pub mod init {
        pub mod bootstrapper {
            pub fn CreateIsolate() {}
        }
    }

    pub mod codegen {
        pub mod interface_descriptors_inl {
            pub struct CallInterfaceDescriptor {}
        }

        pub mod code_factory {
            pub struct CodeFactory {}
        }

        pub mod register_configuration {
            pub struct RegisterConfiguration {}
        }

        pub mod macro_assembler {
            pub struct UseScratchRegisterScope<'a> {
                masm: &'a mut MacroAssembler,
                available: bool,
            }

            impl<'a> UseScratchRegisterScope<'a> {
                pub fn new(masm: &'a mut MacroAssembler) -> Self {
                    UseScratchRegisterScope { masm, available: true }
                }

                pub fn Acquire(&mut self) -> Register {
                    self.available = false;
                    10 // kScratchReg
                }

                pub fn hasAvailable(&self) -> bool {
                  self.available
                }
            }

            impl Drop for UseScratchRegisterScope<'_> {
                fn drop(&mut self) {
                    // Restore any state if needed
                }
            }

            pub struct BlockTrampolinePoolScope<'a> {
              masm: &'a mut MacroAssembler,
            }

            impl<'a> BlockTrampolinePoolScope<'a> {
              pub fn new(masm: &'a mut MacroAssembler) -> Self {
                BlockTrampolinePoolScope {masm}
              }
            }

            impl Drop for BlockTrampolinePoolScope<'_> {
              fn drop(&mut self) {
                // Restore any state if needed
              }
            }

        }

        pub mod mips64 {
            pub mod macro_assembler_mips64 {
                // This is just a placeholder.  The actual implementation
                // details would be platform-specific and complex.
                pub struct MacroAssembler {}

                impl MacroAssembler {
                    pub fn new() -> MacroAssembler {
                        MacroAssembler {}
                    }
                }
            }
        }
    }
}

use internal::codegen::mips64::macro_assembler_mips64::MacroAssembler;
use internal::codegen::macro_assembler::*;

struct MemOperand {
  base: Register,
  offset: i32,
  rmode: i32,
  heap_number_request: i32,
}

impl MemOperand {
  fn new(base: Register, offset: i32) -> MemOperand {
      MemOperand { base, offset, rmode: 0, heap_number_request: 0 }
  }

  fn rm(&self) -> Register {
      self.base
  }

  fn offset(&self) -> i32 {
      self.offset
  }

  fn code(&self) -> i32 {
    self.base as i32
  }
}

struct Options {
  isolate_independent_code: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            isolate_independent_code: false,
        }
    }
}

impl MacroAssembler {
    fn RequiredStackSizeForCallerSaved(
        &self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        let mut bytes = 0;
        let exclusions = [exclusion1, exclusion2, exclusion3];
        // Note: The RegList subtraction logic isn't implemented here.  This
        // is a placeholder.
        let list: RegList = 0xFF; // Dummy value.
        bytes += list.count() * kPointerSize;

        if fp_mode == 1 {
            // SaveFPRegsMode::kSave
            bytes += kCallerSavedFPU.count() * kDoubleSize;
        }

        bytes
    }

    fn PushCallerSaved(
        &mut self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        ASM_CODE_COMMENT!(self);
        let mut bytes = 0;
        let exclusions = [exclusion1, exclusion2, exclusion3];
        // Note: The RegList subtraction logic isn't implemented here.  This
        // is a placeholder.
        let list: RegList = 0xFF; // Dummy value
        self.MultiPush(list);
        bytes += list.count() * kPointerSize;

        if fp_mode == 1 {
          // SaveFPRegsMode::kSave
            self.MultiPushFPU(kCallerSavedFPU);
            bytes += kCallerSavedFPU.count() * kDoubleSize;
        }

        bytes
    }

    fn PopCallerSaved(
        &mut self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        ASM_CODE_COMMENT!(self);
        let mut bytes = 0;
        if fp_mode == 1 {
            self.MultiPopFPU(kCallerSavedFPU);
            bytes += kCallerSavedFPU.count() * kDoubleSize;
        }

        let exclusions = [exclusion1, exclusion2, exclusion3];
        // Note: The RegList subtraction logic isn't implemented here.  This
        // is a placeholder.
        let list: RegList = 0xFF; // Dummy value
        self.MultiPop(list);
        bytes += list.count() * kPointerSize;

        bytes
    }

    fn LoadRoot(&mut self, destination: Register, index: RootIndex) {
        self.Ld(
            destination,
            MemOperand::new(s6, self.RootRegisterOffsetForRootIndex(index)),
        );
    }

    fn LoadRootCond(
        &mut self,
        destination: Register,
        index: RootIndex,
        cond: Condition,
        src1: Register,
        src2: Operand,
    ) {
        self.Branch(2, self.NegateCondition(cond), src1, src2);
        self.Ld(
            destination,
            MemOperand::new(s6, self.RootRegisterOffsetForRootIndex(index)),
        );
    }

    fn PushCommonFrame(&mut self, marker_reg: Register) {
        if marker_reg != 0 {
            //marker_reg.is_valid()
            self.Push(4, 5, marker_reg); // ra, fp
            self.Daddu(5, 6, 8i64); // fp, sp, kPointerSize
        } else {
            self.Push(4, 5, zero_reg); // ra, fp
            self.mov(5, 6); // fp, sp
        }
    }

    fn PushStandardFrame(&mut self, function_reg: Register) {
        let offset = -8; //StandardFrameConstants::kContextOffset;
        if function_reg != 0 {
            //function_reg.is_valid()
            self.Push(
                4, // ra
                5, // fp
                7, // cp
                function_reg,
                9,
            ); // kJavaScriptCallArgCountRegister
             //   offset += 2 * kPointerSize;
        } else {
            self.Push(
                4, // ra
                5, // fp
                7, // cp
                9,
            ); // kJavaScriptCallArgCountRegister
            //  offset += kPointerSize;
        }
        self.Daddu(
            5,
            6,
            offset,
        );
    }

    fn RecordWriteField(
        &mut self,
        object: Register,
        offset: i32,
        value: Register,
        dst: Register,
        ra_status: RAStatus,
        save_fp: SaveFPRegsMode,
        smi_check: SmiCheck,
    ) {
        ASM_CODE_COMMENT!(self);
        DCHECK!(value != dst && value != t8 && value != object && dst != t8 && dst != object && t8 != object);

        let mut done = 0;
        if smi_check == SmiCheck_kInline {
            self.JumpIfSmi(value, &mut done);
        }

        DCHECK!(offset % kPointerSize == 0);

        self.Daddu(dst, object, offset as i64 - kHeapObjectTag);
        if true { //v8_flags.slow_debug_code
            let mut ok = 1;
            self.And(t8, dst, kPointerSize as i64 - 1);
            self.Branch(&mut ok, eq, t8, 0i64);
            self.stop();
            //self.bind(&mut ok);
        }

        self.RecordWrite(object, dst, value, ra_status, save_fp, SmiCheck_kOmit);

       // self.bind(&mut done);

        if true { //v8_flags.slow_debug_code
            self.li(value, base::bit_cast::<i64, i64>(kZapValue + 4));
            self.li(dst, base::bit_cast::<i64, i64>(kZapValue + 8));
        }
    }

    fn MaybeSaveRegisters(&mut self, registers: RegList) {
        if registers == 0 {
            return;
        }
        self.MultiPush(registers);
    }

    fn MaybeRestoreRegisters(&mut self, registers: RegList) {
        if registers == 0 {
            return;
        }
        self.MultiPop(registers);
    }

    fn CallEphemeronKeyBarrier(
        &mut self,
        object: Register,
        slot_address: Register,
        fp_mode: SaveFPRegsMode,
    ) {
        ASM_CODE_COMMENT!(self);
        DCHECK!(object != slot_address);
        let registers = self.WriteBarrierDescriptorComputeSavedRegisters(object, slot_address);
        self.MaybeSaveRegisters(registers);

        let object_parameter = self.WriteBarrierDescriptorObjectRegister();
        let slot_address_parameter = self.WriteBarrierDescriptorSlotAddressRegister();

        self.Push(object, slot_address, 0); // dummy 0
        self.Pop(slot_address_parameter);
        self.Pop(object_parameter);

        self.CallBuiltin(internal::builtins::EphemeronKeyBarrier(fp_mode));
        self.MaybeRestoreRegisters(registers);
    }

    fn CallRecordWriteStubSaveRegisters(
        &mut self,
        object: Register,
        slot_address: Register,
        fp_mode: SaveFPRegsMode,
        mode: StubCallMode,
    ) {
        DCHECK!(object != slot_address);
        let registers = self.WriteBarrierDescriptorComputeSavedRegisters(object, slot_address);
        self.MaybeSaveRegisters(registers);

        let object_parameter = self.WriteBarrierDescriptorObjectRegister();
        let slot_address_parameter = self.WriteBarrierDescriptorSlotAddressRegister();

        self.Push(object, slot_address, 0); // dummy 0
        self.Pop(slot_address_parameter);
        self.Pop(object_parameter);

        self.CallRecordWriteStub(object_parameter, slot_address_parameter, fp_mode, mode);

        self.MaybeRestoreRegisters(registers);
    }

    fn CallRecordWriteStub(
        &mut self,
        object: Register,
        slot_address: Register,
        fp_mode: SaveFPRegsMode,
        mode: StubCallMode,
    ) {
        DCHECK!(
            self.WriteBarrierDescriptorObjectRegister() == object
                && self.WriteBarrierDescriptorSlotAddressRegister() == slot_address
        );

        // #[cfg(V8_ENABLE_WEBASSEMBLY)]
        if mode == 1 {
          // StubCallMode::kCallWasmRuntimeStub
            let wasm_target =
                0;//wasm::WasmCode::GetRecordWriteBuiltin(fp_mode) as Address; // TODO: cast
            self.Call(wasm_target, 0); //RelocInfo::WASM_STUB_CALL
        } else {
            self.CallBuiltin(internal::builtins::RecordWrite(fp_mode));
        }
    }

    fn RecordWrite(
        &mut self,
        object: Register,
        address: Register,
        value: Register,
        ra_status: RAStatus,
        fp_mode: SaveFPRegsMode,
        smi_check: SmiCheck,
    ) {
        DCHECK!(object != address && object != value && object != t8 && address != value && address != t8 && value != t8);

        if true { //v8_flags.slow_debug_code
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            DCHECK!(object != value && object != scratch && value != scratch);
            self.Ld(scratch, MemOperand::new(address, 0));
            // self.Assert(eq, AbortReason::kWrongAddressOrValuePassedToRecordWrite, scratch, value);
        }

        if false {
            // v8_flags.disable_write_barriers
            return;
        }

        let mut done = 0;
        if smi_check == SmiCheck_kInline {
            DCHECK!(0 == 0); // kSmiTag == 0
            self.JumpIfSmi(value, &mut done);
        }

        self.CheckPageFlag(
            value,
            value, // Used as scratch.
            internal::heap::mutable_page_metadata::kPointersToHereAreInterestingMask,
            eq,
            &mut done,
        );
        self.CheckPageFlag(
            object,
            value, // Used as scratch.
            internal::heap::mutable_page_metadata::kPointersFromHereAreInterestingMask,
            eq,
            &mut done,
        );

        if ra_status == 0 { //kRAHasNotBeenSaved
            self.push(4); // ra
        }

        let slot_address = self.WriteBarrierDescriptorSlotAddressRegister();
        DCHECK!(object != slot_address && slot_address != value);
        self.mov(slot_address, address);
        self.CallRecordWriteStub(object, slot_address, fp_mode, 0); // dummy 0

        if ra_status == 0 { //kRAHasNotBeenSaved
            self.pop(4); // ra
        }

        // self.bind(&mut done);

        if true { //v8_flags.slow_debug_code
            self.li(address, base::bit_cast::<i64, i64>(kZapValue + 12));
            self.li(value, base::bit_cast::<i64, i64>(kZapValue + 16));
            self.li(slot_address, base::bit_cast::<i64, i64>(kZapValue + 20));
        }
    }

    fn Addu(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
          self.addu(rd, rs, rt as u32);
      } else {
          if self.is_int16(rt) && false { //&& !MustUseReg(rt.rmode()) {
              self.addiu(rd, rs, rt as i32);
          } else {
              let mut temps = UseScratchRegisterScope::new(self);
              let scratch = temps.Acquire();
              DCHECK!(rs != scratch);
              self.li(scratch, rt);
              self.addu(rd, rs, scratch);
          }
      }
    }

    fn Daddu(&mut self, rd: Register, rs: Register, rt: i64) {
        if (rt & 1) != 0 { //rt.is_reg() {
            self.daddu(rd, rs, rt as u32);
        } else {
            if self.is_int16(rt) && true { //!MustUseReg(rt.rmode()) {
                self.daddiu(rd, rs, rt as i32);
            } else {
                let mut temps = UseScratchRegisterScope::new(self);
                let scratch = temps.Acquire();
                DCHECK!(rs != scratch);
                self.li(scratch, rt);
                self.daddu(rd, rs, scratch);
            }
        }
    }

    fn Subu(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        self.subu(rd, rs, rt as u32);
      } else {
        DCHECK!(self.is_int32(rt));
        if self.is_int16(-rt) && true { //!MustUseReg(rt.rmode()) {
          self.addiu(rd, rs, -rt as i32); // No subiu instr, use addiu(x, y, -imm).
        } else {
          let mut temps = UseScratchRegisterScope::new(self);
          let scratch = temps.Acquire();
          DCHECK!(rs != scratch);
          if -rt >> 16 == 0 && true { //!MustUseReg(rt.rmode()) {
            self.li(scratch, -rt);
            self.Addu(rd, rs, scratch);
          } else {
            self.li(scratch, rt);
            self.subu(rd, rs, scratch);
          }
        }
      }
    }

    fn Dsubu(&mut self, rd: Register, rs: Register, rt: i64) {
        if (rt & 1) != 0 { //rt.is_reg() {
            self.dsubu(rd, rs, rt as u32);
        } else if self.is_int16(-rt) && true { //!MustUseReg(rt.rmode()) {
            self.daddiu(rd, rs, -rt as i32); // No dsubiu instr, use daddiu(x, y, -imm).
        } else {
            DCHECK!(rs != at);
            let li_count = self.InstrCountForLi64Bit(rt);
            let li_neg_count = self.InstrCountForLi64Bit(-rt);
            if li_neg_count < li_count && true { //!MustUseReg(rt.rmode()) {
                DCHECK!(rt != i32::min_value() as i64);
                let mut temps = UseScratchRegisterScope::new(self);
                let scratch = temps.Acquire();
                self.li(scratch, -rt);
                self.Daddu(rd, rs, scratch);
            } else {
                let mut temps = UseScratchRegisterScope::new(self);
                let scratch = temps.Acquire();
                self.li(scratch, rt);
                self.dsubu(rd, rs, scratch);
            }
        }
    }

    fn Mul(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        self.mul(rd, rs, rt as u32);
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        self.mul(rd, rs, scratch);
      }
    }

    fn Mulh(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        if kArchVariant != 0 { //kArchVariant != kMips64r6 {
          self.mult(rs, rt as u32);
          self.mfhi(rd);
        } else {
          self.muh(rd, rs, rt as u32);
        }
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        if kArchVariant != 0 { //kArchVariant != kMips64r6 {
          self.mult(rs, scratch);
          self.mfhi(rd);
        } else {
          self.muh(rd, rs, scratch);
        }
      }
    }

    fn Mulhu(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        if kArchVariant != 0 { //kArchVariant != kMips64r6 {
          self.multu(rs, rt as u32);
          self.mfhi(rd);
        } else {
          self.muhu(rd, rs, rt as u32);
        }
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        if kArchVariant != 0 { //kArchVariant != kMips64r6 {
          self.multu(rs, scratch);
          self.mfhi(rd);
        } else {
          self.muhu(rd, rs, scratch);
        }
      }
    }

    fn Dmul(&mut self, rd: Register, rs: Register, rt: Operand) {
        if (rt & 1) != 0 { //rt.is_reg() {
            if kArchVariant == 0 { //kArchVariant == kMips64r6 {
                self.dmul(rd, rs, rt as u32);
            } else {
                self.dmult(rs, rt as u32);
                self.mflo(rd);
            }
        } else {
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            DCHECK!(rs != scratch);
            self.li(scratch, rt);
            if kArchVariant == 0 { //kArchVariant == kMips64r6 {
                self.dmul(rd, rs, scratch);
            } else {
                self.dmult(rs, scratch);
                self.mflo(rd);
            }
        }
    }

    fn Dmulh(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        if kArchVariant == 0 { //kArchVariant == kMips64r6 {
          self.dmuh(rd, rs, rt as u32);
        } else {
          self.dmult(rs, rt as u32);
          self.mfhi(rd);
        }
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        if kArchVariant == 0 { //kArchVariant == kMips64r6 {
          self.dmuh(rd, rs, scratch);
        } else {
          self.dmult(rs, scratch);
          self.mfhi(rd);
        }
      }
    }

    fn Dmulhu(&mut self, rd: Register, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        if kArchVariant == 0 { //kArchVariant == kMips64r6 {
          self.dmuhu(rd, rs, rt as u32);
        } else {
          self.dmultu(rs, rt as u32);
          self.mfhi(rd);
        }
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        if kArchVariant == 0 { //kArchVariant == kMips64r6 {
          self.dmuhu(rd, rs, scratch);
        } else {
          self.dmultu(rs, scratch);
          self.mfhi(rd);
        }
      }
    }

    fn Mult(&mut self, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        self.mult(rs, rt as u32);
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        self.mult(rs, scratch);
      }
    }

    fn Dmult(&mut self, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        self.dmult(rs, rt as u32);
      } else {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.Acquire();
        DCHECK!(rs != scratch);
        self.li(scratch, rt);
        self.dmult(rs, scratch);
      }
    }

    fn Multu(&mut self, rs: Register, rt: Operand) {
      if (rt & 1) != 0 { //rt.is_reg() {
        self.multu(rs, rt as u32);
      } else {
        let mut temps = UseScratchRegisterScope::new