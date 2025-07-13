// Converted from V8 C++ source files:
// Header: bytecode-node.h
// Implementation: bytecode-node.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interpreter {
use std::fmt;

use crate::interpreter::bytecode_source_info::BytecodeSourceInfo;
use crate::interpreter::bytecodes::{Bytecode, Bytecodes};
use crate::compiler::bytecode_analysis::{OperandType};
use crate::compiler::turboshaft::builtin_compiler::{ImplicitRegisterUse, OperandScale};
//use crate::base::bits::RoundUp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BytecodeNode {
  bytecode_: Bytecode,
  operands_: [u32; Bytecodes::K_MAX_OPERANDS],
  operand_count_: i32,
  operand_scale_: OperandScale,
  source_info_: BytecodeSourceInfo,
}

impl BytecodeNode {
  #[inline]
  pub fn new(bytecode: Bytecode, source_info: BytecodeSourceInfo) -> Self {
    let operand_count = 0;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node
  }

  #[inline]
  pub fn new_with_operand0(
    bytecode: Bytecode,
    operand0: u32,
    source_info: BytecodeSourceInfo,
  ) -> Self {
    let operand_count = 1;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node.set_operand(0, operand0);
    node
  }

  #[inline]
  pub fn new_with_operand0_operand1(
    bytecode: Bytecode,
    operand0: u32,
    operand1: u32,
    source_info: BytecodeSourceInfo,
  ) -> Self {
    let operand_count = 2;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node.set_operand(0, operand0);
    node.set_operand(1, operand1);
    node
  }

  #[inline]
  pub fn new_with_operand0_operand1_operand2(
    bytecode: Bytecode,
    operand0: u32,
    operand1: u32,
    operand2: u32,
    source_info: BytecodeSourceInfo,
  ) -> Self {
    let operand_count = 3;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node.set_operand(0, operand0);
    node.set_operand(1, operand1);
    node.set_operand(2, operand2);
    node
  }

  #[inline]
  pub fn new_with_operand0_operand1_operand2_operand3(
    bytecode: Bytecode,
    operand0: u32,
    operand1: u32,
    operand2: u32,
    operand3: u32,
    source_info: BytecodeSourceInfo,
  ) -> Self {
    let operand_count = 4;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node.set_operand(0, operand0);
    node.set_operand(1, operand1);
    node.set_operand(2, operand2);
    node.set_operand(3, operand3);
    node
  }

  #[inline]
  pub fn new_with_operand0_operand1_operand2_operand3_operand4(
    bytecode: Bytecode,
    operand0: u32,
    operand1: u32,
    operand2: u32,
    operand3: u32,
    operand4: u32,
    source_info: BytecodeSourceInfo,
  ) -> Self {
    let operand_count = 5;
    let mut node = BytecodeNode {
      bytecode_: bytecode,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: operand_count,
      operand_scale_: OperandScale::kSingle,
      source_info_: source_info,
    };
    assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
    node.set_operand(0, operand0);
    node.set_operand(1, operand1);
    node.set_operand(2, operand2);
    node.set_operand(3, operand3);
    node.set_operand(4, operand4);
    node
  }

  #[inline]
  pub fn print(&self, os: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    #[cfg(debug_assertions)]
    {
      let mut saved_state = os.debug_struct("BytecodeNode");
      saved_state.field("bytecode", &Bytecodes::to_string(self.bytecode_));

      for i in 0..self.operand_count() {
        saved_state.field(
          &format!("operand_{}", i),
          &format!("{:08x}", self.operands_[i as usize]),
        );
      }

      if self.source_info_.is_valid() {
        saved_state.field("source_info", &self.source_info_);
      }

      saved_state.finish()
    }
    #[cfg(not(debug_assertions))]
    {
      write!(os, "{:p}", self)
    }
  }

  pub fn bytecode(&self) -> Bytecode {
    self.bytecode_
  }

  pub fn operand(&self, i: i32) -> u32 {
    assert!(i < self.operand_count());
    self.operands_[i as usize]
  }

  pub fn operands(&self) -> &[u32; Bytecodes::K_MAX_OPERANDS] {
    &self.operands_
  }

  pub fn update_operand0(&mut self, operand0: u32) {
    self.set_operand(0, operand0);
  }

  pub fn operand_count(&self) -> i32 {
    self.operand_count_
  }

  pub fn operand_scale(&self) -> OperandScale {
    self.operand_scale_
  }

  pub fn source_info(&self) -> &BytecodeSourceInfo {
    &self.source_info_
  }

  pub fn set_source_info(&mut self, source_info: BytecodeSourceInfo) {
    self.source_info_ = source_info;
  }
  
  fn scale_for_operand<const T: OperandType>(&self, operand: u32) -> OperandScale {
      if operand == T as u32 {
          return OperandScale::kSingle;
      }
      
      OperandScale::kSingle
  }

  fn update_scale_for_operand(&mut self, operand_index: i32, operand: u32) {
    if Bytecodes::operand_is_scalable_signed_byte(self.bytecode(), operand_index) {
      self.operand_scale_ =
        std::cmp::max(self.operand_scale_, Bytecodes::scale_for_signed_operand(operand));
    } else if Bytecodes::operand_is_scalable_unsigned_byte(self.bytecode(), operand_index) {
      self.operand_scale_ =
        std::cmp::max(self.operand_scale_, Bytecodes::scale_for_unsigned_operand(operand));
    }
  }

  fn set_operand(&mut self, operand_index: i32, operand: u32) {
    self.operands_[operand_index as usize] = operand;
    self.update_scale_for_operand(operand_index, operand);
  }
  
    #[inline]
  pub fn create_constant(source_info: BytecodeSourceInfo, operand0: u32) -> Self {
    Self::create::<{ Bytecode::kLdarSmi } , { ImplicitRegisterUse::kNone }, { OperandType::kIdx }>(source_info, operand0)
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse>() -> Self {
    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [0; Bytecodes::K_MAX_OPERANDS],
      operand_count_: 0,
      operand_scale_: OperandScale::kSingle,
      source_info_: BytecodeSourceInfo::new(0,0),
    }
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
    OperandType operand0_type,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32) -> Self {
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0) as i32, OPERAND0_TYPE as i32);
    let mut scale = OperandScale::kSingle;
    if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand0);
    }
    
    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [operand0, 0, 0, 0, 0],
      operand_count_: 1,
      operand_scale_: scale,
      source_info_: source_info,
    }
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
    OperandType operand0_type,
    OperandType operand1_type,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32) -> Self {
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0) as i32, OPERAND0_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1) as i32, OPERAND1_TYPE as i32);
    let mut scale = OperandScale::kSingle;
    if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand0);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand1);
    }
    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [operand0, operand1, 0, 0, 0],
      operand_count_: 2,
      operand_scale_: scale,
      source_info_: source_info,
    }
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
    OperandType operand0_type,
    OperandType operand1_type,
    OperandType operand2_type,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32) -> Self {
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0) as i32, OPERAND0_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1) as i32, OPERAND1_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2) as i32, OPERAND2_TYPE as i32);
    let mut scale = OperandScale::kSingle;
    if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand0);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand1);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand2);
    }
    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [operand0, operand1, operand2, 0, 0],
      operand_count_: 3,
      operand_scale_: scale,
      source_info_: source_info,
    }
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
    OperandType operand0_type,
    OperandType operand1_type,
    OperandType operand2_type,
    OperandType operand3_type,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType, const OPERAND3_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32, operand3: u32) -> Self {
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0) as i32, OPERAND0_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1) as i32, OPERAND1_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2) as i32, OPERAND2_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 3) as i32, OPERAND3_TYPE as i32);
    let mut scale = OperandScale::kSingle;
    if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand0);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand1);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand2);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand3);
    }

    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [operand0, operand1, operand2, operand3, 0],
      operand_count_: 4,
      operand_scale_: scale,
      source_info_: source_info,
    }
  }

  template! {
    Bytecode bytecode,
    ImplicitRegisterUse implicit_register_use,
    OperandType operand0_type,
    OperandType operand1_type,
    OperandType operand2_type,
    OperandType operand3_type,
    OperandType operand4_type,
  }
  #[inline]
  pub fn create<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType, const OPERAND3_TYPE: OperandType, const OPERAND4_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32, operand3: u32, operand4: u32) -> Self {
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0) as i32, OPERAND0_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1) as i32, OPERAND1_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2) as i32, OPERAND2_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 3) as i32, OPERAND3_TYPE as i32);
    assert_eq!(Bytecodes::get_operand_type(BYTECODE, 4) as i32, OPERAND4_TYPE as i32);
    let mut scale = OperandScale::kSingle;
    if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand0);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand1);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand2);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand3);
    }
        if BYTECODE == Bytecode::kLdarSmi {
          scale = Self::scale_for_operand::<{OperandType::kIdx}>(operand4);
    }

    BytecodeNode {
      bytecode_: BYTECODE,
      operands_: [operand0, operand1, operand2, operand3, operand4],
      operand_count_: 5,
      operand_scale_: scale,
      source_info_: source_info,
    }
  }

}

impl std::cmp::PartialEq for BytecodeNode {
  fn eq(&self, other: &Self) -> bool {
    if self as *const _ == other as *const _ {
      return true;
    } else if self.bytecode() != other.bytecode() || self.source_info() != other.source_info() {
      return false;
    } else {
      for i in 0..self.operand_count() {
        if self.operand(i) != other.operand(i) {
          return false;
        }
      }
    }
    true
  }
}

impl std::cmp::Eq for BytecodeNode {}

impl fmt::Display for BytecodeNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.print(f)
  }
}
}  // namespace interpreter

pub mod bytecode_source_info {
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BytecodeSourceInfo {
  source_position_: i32,
  statement_position_: i32,
}

impl BytecodeSourceInfo {
  pub fn new(source_position: i32, statement_position: i32) -> Self {
    BytecodeSourceInfo {
      source_position_: source_position,
      statement_position_: statement_position,
    }
  }

  pub fn is_valid(&self) -> bool {
    self.source_position_ != 0 || self.statement_position_ != 0
  }
}

impl fmt::Display for BytecodeSourceInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "source_position: {}, statement_position: {}",
      self.source_position_, self.statement_position_
    )
  }
}
}  // namespace bytecode_source_info

pub mod bytecodes {
use std::fmt;

use crate::compiler::bytecode_analysis::OperandType;
use crate::compiler::turboshaft::builtin_compiler::OperandScale;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Bytecode {
  kLdarSmi,
  kIllegal,
  kGetSuperConstructor,
  kCallRuntime,
  kCallUndefinedReceiver1,
  kCallProperty1,
  kReturn,
  kThrow,
  kForInPrepare,
  kLdaGlobal,
  kStaCurrentContextSlot,
  kDebugger,
  kNop,
  kStar0,
  kStar1,
  kStar2,
  kStar3,
  kStar4,
  kStar5,
  kStar6,
  kStar7,
  kStar8,
  kStar9,
  kStar10,
  kStar11,
  kStar12,
  kStar13,
  kStar14,
  kStar15,
  kLdar,
  kLdaZero,
  kLdaTheHole,
  kLdaUndefined,
  kLdaNull,
  kLdaTrue,
  kLdaFalse,
  kLdaConstant,
  kLdaContextSlot,
  kLdaModuleVariable,
  kLdaGlobalInsideTypeof,
  kLdaReceiver,
  kLdaReceiver0,
  kLdaSmi,
  kLdaHomeObject,
  kStaModuleVariable,
  kStaContextSlot,
  kStaReceiver,
  kStaInArrayLiteral,
  kStaCurrentContextSlot,
  kThrowReferenceErrorIfHole,
  kThrowSuperNotCalled,
  kThrowSuperAlreadyCalled,
  kCallUndefinedReceiver0,
  kCallUndefinedReceiver2,
  kCallUndefinedReceiver3,
  kCallUndefinedReceiver4,
  kCallUndefinedReceiver5,
  kCallUndefinedReceiver6,
  kCallUndefinedReceiver7,
  kCallUndefinedReceiver8,
  kCallUndefinedReceiver9,
  kCallUndefinedReceiver10,
  kCallUndefinedReceiver11,
  kCallUndefinedReceiver12,
  kCallUndefinedReceiver13,
  kCallUndefinedReceiver14,
  kCallUndefinedReceiver15,
  kCallProperty0,
  kCallProperty2,
  kCallProperty3,
  kCallProperty4,
  kCallProperty5,
  kCallProperty6,
  kCallProperty7,
  kCallProperty8,
  kCallProperty9,
  kCallProperty10,
  kCallProperty11,
  kCallProperty12,
  kCallProperty13,
  kCallProperty14,
  kCallProperty15,
  kConstruct0,
  kConstruct1,
  kConstruct2,
  kConstruct3,
  kConstruct4,
  kConstruct5,
  kConstruct6,
  kConstruct7,
  kConstruct8,
  kConstruct9,
  kConstruct10,
  kConstruct11,
  kConstruct12,
  kConstruct13,
  kConstruct14,
  kConstruct15,
  kTestEqual,
  kTestEqualStrict,
  kTestLessThan,
  kTestGreaterThan,
  kTestLessThanOrEqual,
  kTestGreaterThanOrEqual,
  kTestReferenceEqual,
  kTestInstanceOf,
  kTestIn,
  kTestUndefined,
  kTestNull,
  kTestTrue,
  kTestFalse,
  kTestTypeOf,
  kTestTypeOfIsObject,
  kTestTypeOfIsString,
  kTestHole,
  kTestNotHole,
  kToName,
  kToNumber,
  kToString,
  kToBigInt,
  kToObject,
  kIncrementSmi,
  kDecrementSmi,
  kAddSmi,
  kSubSmi,
  kMulSmi,
  kDivSmi,
  kModSmi,
  kBitAndSmi,
  kBitOrSmi,
  kBitXorSmi,
  kShiftLeftSmi,
  kShiftRightSmi,
  kShiftRightLogicalSmi,
  kAdd,
  kSub,
  kMul,
  kDiv,
  kMod,
  kBitAnd,
  kBitOr,
  kBitXor,
  kShiftLeft,
  kShiftRight,
  kShiftRightLogical,
  kAddLoopFeedback,
  kSubLoopFeedback,
  kMulLoopFeedback,
  kDivLoopFeedback,
  kModLoopFeedback,
  kBitAndLoopFeedback,
  kBitOrLoopFeedback,
  kBitXorLoopFeedback,
  kShiftLeftLoopFeedback,
  kShiftRightLoopFeedback,
  kShiftRightLogicalLoopFeedback,
  kExpLoopFeedback,
  kExp,
  kNegate,
  kBitNot,
  kCountBits,
  kToBooleanLogicalNot,
  kLogicalNot,
  kTypeOf,
  kDeletePropertyStrict,
  kDeletePropertySloppy,
  kGetProperty,
  kGetPropertyStrict,
  kSetPropertyStrict,
  kSetPropertySloppy,
  kGetSuperProperty,
  kGetSuperPropertyStrict,
  kSetSuperProperty,
  kLoadAccessorWithReceiver,
  kLoadAccessor,
  kStaKeyedProperty,
  kStaKeyedPropertySloppy,
  kStaKeyedPropertyStrict,
  kDefineKeyedOwnPropertySloppy,
  kDefineKeyedOwnPropertyStrict,
  kLoadKeyedProperty,
  kLoadKeyedPropertySloppy,
  kLoadKeyedPropertyStrict,
  kHasKeyedProperty,
  kCallKeyedProperty0,
  kCallKeyedProperty1,
  kCallKeyedProperty2,
  kCallKeyedProperty3,
  kCallKeyedProperty4,
  kCallKeyedProperty5,
  kCallKeyedProperty6,
  kCallKeyedProperty7,
  kCallKeyedProperty8,
  kCallKeyedProperty9,
  kCallKeyedProperty10,
  kCallKeyedProperty11,
  kCallKeyedProperty12,
  kCallKeyedProperty13,
  kCallKeyedProperty14,
  kCallKeyedProperty15,
  kGetIterator,
  kGetAsyncIterator,
  kNext,
  kNextAsync,
  kCreateArrayLiteral,
  kCreateArrayFromIterable,
  kCreateEmptyArray,
  kCreateRegExpLiteral,
  kCreateObjectLiteral,
  kCreateGeneratorObject,
  kCreateAsyncGeneratorObject,
  kCreateLiteralAnonymousFunction,
  kCreateLiteralAnonymousFunctionContext,
  kCreateFunctionContext,
  kCreateBlockContext,
  kPushContext,
  kPopContext,
  kCall,
  kConstruct,
  kStackCheck,
  kThrowUndefinedIfHole,
  kThrowUndefinedOrNullIfHole,
  kThrowTypeError,
  kReThrow,
  kReturnUndefined,
  kThrowIllegalInvocation,
  kForInNext,
  kForInStep,
  kJumpLoop,
  kJump,
  kJumpIfTrue,
  kJumpIfFalse,
  kJumpIfToBooleanTrue,
  kJumpIfToBooleanFalse,
  kJumpIfUndefined,
  kJumpIfNull,
  kJumpIfNotNull,
  kJumpIfUndefinedOrNull,
  kJumpIfNotUndefined,
  kJumpIfNotHole,
  kJumpIfHole,
  kJumpIfJSReceiver,
  kJumpIfNotJSReceiver,
  kSwitchOnSmiNoFeedback,
  kSwitchOnSmiWithFeedback,
  kSwitchOnString,
  kReturnFromFinally,
  kThrowFromFinally,
  kCreateCatchContext,
  kCreateWithContext,
  kMaybePromoteWeakable,
  kLazyCompileContinuation,
  kSuspendGenerator,
  kResumeGenerator,
  kEnqueueMicrotask,
  kPerformMicrotaskCheckpoint,
  kCreateCatchContext,
  kCreateWithContext,
  kMaybePromoteWeakable,
  kLazyCompileContinuation,
  kSuspendGenerator,
  kResumeGenerator,
  kEnqueueMicrotask,
  kPerformMicrotaskCheckpoint,
}

impl Bytecode {
  pub fn to_string(&self) -> &str {
    match self {
      Bytecode::kLdarSmi => "LdarSmi",
      Bytecode::kIllegal => "Illegal",
      Bytecode::kGetSuperConstructor => "GetSuperConstructor",
      Bytecode::kCallRuntime => "CallRuntime",
      Bytecode::kCallUndefinedReceiver1 => "CallUndefinedReceiver1",
      Bytecode::kCallProperty1 => "CallProperty1",
      Bytecode::kReturn => "Return",
      Bytecode::kThrow => "Throw",
      Bytecode::kForInPrepare => "ForInPrepare",
      Bytecode::kLdaGlobal => "LdaGlobal",
      Bytecode::kStaCurrentContextSlot => "StaCurrentContextSlot",
      Bytecode::kDebugger => "Debugger",
      Bytecode::kNop => "Nop",
      Bytecode::kStar0 => "Star0",
      Bytecode::kStar1 => "Star1",
      Bytecode::kStar2 => "Star2",
      Bytecode::kStar3 => "Star3",
      Bytecode::kStar4 => "Star4",
      Bytecode::kStar5 => "Star5",
      Bytecode::kStar6 => "Star6",
      Bytecode::kStar7 => "Star7",
      Bytecode::kStar8 => "Star8",
      Bytecode::kStar9 => "Star9",
      Bytecode::kStar10 => "Star10",
      Bytecode::kStar11 => "Star11",
      Bytecode::kStar12 => "Star12",
      Bytecode::kStar13 => "Star13",
      Bytecode::kStar14 => "Star14",
      Bytecode::kStar15 => "Star15",
      Bytecode::kLdar => "Ldar",
      Bytecode::kLdaZero => "LdaZero",
      Bytecode::kLdaTheHole => "LdaTheHole",
      Bytecode::kLdaUndefined => "LdaUndefined",
      Bytecode::kLdaNull => "LdaNull",
      Bytecode::kLdaTrue => "LdaTrue",
      Bytecode::kLdaFalse => "LdaFalse",
      Bytecode::kLdaConstant => "LdaConstant",
      Bytecode::kLdaContextSlot => "LdaContextSlot",
      Bytecode::kLdaModuleVariable => "LdaModuleVariable",
      Bytecode::kLdaGlobalInsideTypeof => "LdaGlobalInsideTypeof",
      Bytecode::kLdaReceiver => "LdaReceiver",
      Bytecode::kLdaReceiver0 => "LdaReceiver0",
      Bytecode::kLdaSmi => "LdaSmi",
      Bytecode::kLdaHomeObject => "LdaHomeObject",
      Bytecode::kStaModuleVariable => "StaModuleVariable",
      Bytecode::kStaContextSlot => "StaContextSlot",
      Bytecode::kStaReceiver => "StaReceiver",
      Bytecode::kStaInArrayLiteral => "StaInArrayLiteral",
      Bytecode::kStaCurrentContextSlot => "StaCurrentContextSlot",
      Bytecode::kThrowReferenceErrorIfHole => "ThrowReferenceErrorIfHole",
      Bytecode::kThrowSuperNotCalled => "ThrowSuperNotCalled",
      Bytecode::kThrowSuperAlreadyCalled => "ThrowSuperAlreadyCalled",
      Bytecode::kCallUndefinedReceiver0 => "CallUndefinedReceiver0",
      Bytecode::kCallUndefinedReceiver2 => "CallUndefinedReceiver2",
      Bytecode::kCallUndefinedReceiver3 => "CallUndefinedReceiver3",
      Bytecode::kCallUndefinedReceiver4 => "CallUndefinedReceiver4",
      Bytecode::kCallUndefinedReceiver5 => "CallUndefinedReceiver5",
      Bytecode::kCallUndefinedReceiver6 => "CallUndefinedReceiver6",
      Bytecode::kCallUndefinedReceiver7 => "CallUndefinedReceiver7",
      Bytecode::kCallUndefinedReceiver8 => "CallUndefinedReceiver8",
      Bytecode::kCallUndefinedReceiver9 => "CallUndefinedReceiver9",
      Bytecode::kCallUndefinedReceiver10 => "CallUndefinedReceiver10",
      Bytecode::kCallUndefinedReceiver11 => "CallUndefinedReceiver11",
      Bytecode::kCallUndefinedReceiver12 => "CallUndefinedReceiver12",
      Bytecode::kCallUndefinedReceiver13 => "CallUndefinedReceiver13",
      Bytecode::kCallUndefinedReceiver14 => "CallUndefinedReceiver14",
      Bytecode::kCallUndefinedReceiver15 => "CallUndefinedReceiver15",
      Bytecode::kCallProperty0 => "CallProperty0",

