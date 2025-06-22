// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

pub mod bytecodes {
    use std::{fmt, string::String};

    use crate::interpreter::bytecode_operands::*;

    /// The list of single-byte Star variants.
    macro_rules! short_star_bytecode_list {
        ($V:ident) => {
            $V!(Star15, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star14, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star13, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star12, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star11, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star10, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star9, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star8, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star7, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star6, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star5, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star4, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star3, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star2, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star1, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
            $V!(Star0, ImplicitRegisterUse::kReadAccumulatorWriteShortStar);
        };
    }

    /// Macro for defining bytecodes with unique handlers.
    macro_rules! bytecode_list_with_unique_handlers_impl {
        ($V:ident, $V_TSA:ident) => {
            // Extended width operands
            $V!(Wide, ImplicitRegisterUse::kNone);
            $V!(ExtraWide, ImplicitRegisterUse::kNone);

            // Debug Breakpoints - one for each possible size of unscaled bytecodes
            // and one for each operand widening prefix bytecode
            $V!(DebugBreakWide, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(DebugBreakExtraWide, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(DebugBreak0, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(DebugBreak1, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg);
            $V!(DebugBreak2, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kReg);
            $V!(DebugBreak3, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg);
            $V!(DebugBreak4, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kReg);
            $V!(DebugBreak5, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kRuntimeId, OperandType::kReg, OperandType::kReg);
            $V!(DebugBreak6, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kRuntimeId, OperandType::kReg, OperandType::kReg, OperandType::kReg);

            // Side-effect-free bytecodes -- carefully ordered for efficient checks
            // - [Loading the accumulator]
            $V!(Ldar, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg);
            $V!(LdaZero, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaSmi, ImplicitRegisterUse::kWriteAccumulator, OperandType::kImm);
            $V!(LdaUndefined, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaNull, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaTheHole, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaTrue, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaFalse, ImplicitRegisterUse::kWriteAccumulator);
            $V!(LdaConstant, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(LdaContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaScriptContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaImmutableContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaCurrentContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(LdaCurrentScriptContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(LdaImmutableCurrentContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);

            // - [Register Loads ]
            $V!(Star, ImplicitRegisterUse::kReadAccumulator, OperandType::kRegOut);
            $V!(Mov, ImplicitRegisterUse::kNone, OperandType::kReg, OperandType::kRegOut);
            $V!(PushContext, ImplicitRegisterUse::kReadAccumulator, OperandType::kRegOut);
            $V!(PopContext, ImplicitRegisterUse::kNone, OperandType::kReg);

            // - [Test Operations ]
            $V!(TestReferenceEqual, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg);
            $V!(TestUndetectable, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(TestNull, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(TestUndefined, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(TestTypeOf, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kFlag8);

            // Globals
            $V!(LdaGlobal, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx);
            $V!(LdaGlobalInsideTypeof, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx);
            $V!(StaGlobal, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kIdx, OperandType::kIdx);

            // Context operations
            $V!(StaContextSlot, ImplicitRegisterUse::kReadAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(StaCurrentContextSlot, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(StaScriptContextSlot, ImplicitRegisterUse::kReadAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(StaCurrentScriptContextSlot, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);

            // Load-Store lookup slots
            $V!(LdaLookupSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(LdaLookupContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaLookupScriptContextSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaLookupGlobalSlot, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaLookupSlotInsideTypeof, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(LdaLookupContextSlotInsideTypeof, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaLookupScriptContextSlotInsideTypeof, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(LdaLookupGlobalSlotInsideTypeof, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kUImm);
            $V!(StaLookupSlot, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx, OperandType::kFlag8);

            // Property loads (LoadIC) operations
            $V!(GetNamedProperty, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kIdx);
            $V!(GetNamedPropertyFromSuper, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kIdx);
            $V!(GetKeyedProperty, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(GetEnumeratedKeyedProperty, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kIdx);

            // Operations on module variables
            $V!(LdaModuleVariable, ImplicitRegisterUse::kWriteAccumulator, OperandType::kImm, OperandType::kUImm);
            $V!(StaModuleVariable, ImplicitRegisterUse::kReadAccumulator, OperandType::kImm, OperandType::kUImm);

            // Propery stores (StoreIC) operations
            $V!(SetNamedProperty, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kIdx);
            $V!(DefineNamedOwnProperty, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kIdx);
            $V!(SetKeyedProperty, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(DefineKeyedOwnProperty, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kFlag8, OperandType::kIdx);
            $V!(StaInArrayLiteral, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(DefineKeyedOwnPropertyInLiteral, ImplicitRegisterUse::kReadAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kFlag8, OperandType::kIdx);

            // Binary Operators
            $V!(Add, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(Sub, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(Mul, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(Div, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(Mod, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(Exp, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(BitwiseOr, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(BitwiseXor, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(BitwiseAnd, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(ShiftLeft, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(ShiftRight, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(ShiftRightLogical, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);

            // Binary operators with immediate operands
            $V!(AddSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(SubSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(MulSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(DivSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(ModSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(ExpSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(BitwiseOrSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(BitwiseXorSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(BitwiseAndSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(ShiftLeftSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(ShiftRightSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);
            $V!(ShiftRightLogicalSmi, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kImm, OperandType::kIdx);

            // Unary Operators
            $V!(Inc, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(Dec, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(Negate, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V_TSA!(BitwiseNot, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(ToBooleanLogicalNot, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(LogicalNot, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(TypeOf, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(DeletePropertyStrict, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg);
            $V!(DeletePropertySloppy, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg);

            // GetSuperConstructor operator
            $V!(GetSuperConstructor, ImplicitRegisterUse::kReadAccumulator, OperandType::kRegOut);
            $V!(FindNonDefaultConstructorOrConstruct, ImplicitRegisterUse::kNone, OperandType::kReg, OperandType::kReg, OperandType::kRegOutPair);

            // Call operations
            $V!(CallAnyReceiver, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(CallProperty, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(CallProperty0, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(CallProperty1, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(CallProperty2, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(CallUndefinedReceiver, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(CallUndefinedReceiver0, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(CallUndefinedReceiver1, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(CallUndefinedReceiver2, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kReg, OperandType::kIdx);
            $V!(CallWithSpread, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(CallRuntime, ImplicitRegisterUse::kWriteAccumulator, OperandType::kRuntimeId, OperandType::kRegList, OperandType::kRegCount);
            $V!(CallRuntimeForPair, ImplicitRegisterUse::kClobberAccumulator, OperandType::kRuntimeId, OperandType::kRegList, OperandType::kRegCount, OperandType::kRegOutPair);
            $V!(CallJSRuntime, ImplicitRegisterUse::kWriteAccumulator, OperandType::kNativeContextIndex, OperandType::kRegList, OperandType::kRegCount);

            // Intrinsics
            $V!(InvokeIntrinsic, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIntrinsicId, OperandType::kRegList, OperandType::kRegCount);

            // Construct operators
            $V!(Construct, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(ConstructWithSpread, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kIdx);
            $V!(ConstructForwardAllArgs, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);

            // Effectful Test Operators
            $V!(TestEqual, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestEqualStrict, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestLessThan, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestGreaterThan, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestLessThanOrEqual, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestGreaterThanOrEqual, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestInstanceOf, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(TestIn, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kReg, OperandType::kIdx);

            // Cast operators
            $V!(ToName, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(ToNumber, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(ToNumeric, ImplicitRegisterUse::kReadWriteAccumulator, OperandType::kIdx);
            $V!(ToObject, ImplicitRegisterUse::kReadAccumulator, OperandType::kRegOut);
            $V!(ToString, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(ToBoolean, ImplicitRegisterUse::kReadWriteAccumulator);

            // Literals
            $V!(CreateRegExpLiteral, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kFlag16);
            $V!(CreateArrayLiteral, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kFlag8);
            $V!(CreateArrayFromIterable, ImplicitRegisterUse::kReadWriteAccumulator);
            $V!(CreateEmptyArrayLiteral, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(CreateObjectLiteral, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kFlag8);
            $V!(CreateEmptyObjectLiteral, ImplicitRegisterUse::kWriteAccumulator);
            $V!(CloneObject, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kFlag8, OperandType::kIdx);

            // Tagged templates
            $V!(GetTemplateObject, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx);

            // Closure allocation
            $V!(CreateClosure, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kIdx, OperandType::kFlag8);

            // Context allocation
            $V!(CreateBlockContext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx);
            $V!(CreateCatchContext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx);
            $V!(CreateFunctionContext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kUImm);
            $V!(CreateEvalContext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kIdx, OperandType::kUImm);
            $V!(CreateWithContext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx);

            // Arguments allocation
            $V!(CreateMappedArguments, ImplicitRegisterUse::kWriteAccumulator);
            $V!(CreateUnmappedArguments, ImplicitRegisterUse::kWriteAccumulator);
            $V!(CreateRestParameter, ImplicitRegisterUse::kWriteAccumulator);

            // Control Flow -- carefully ordered for efficient checks
            // - [Unconditional jumps]
            $V!(JumpLoop, ImplicitRegisterUse::kClobberAccumulator, OperandType::kUImm, OperandType::kImm, OperandType::kIdx);
            // - [Forward jumps]
            $V!(Jump, ImplicitRegisterUse::kNone, OperandType::kUImm);
            // - [Start constant jumps]
            $V!(JumpConstant, ImplicitRegisterUse::kNone, OperandType::kIdx);
            // - [Conditional jumps]
            // - [Conditional constant jumps]
            $V!(JumpIfNullConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfNotNullConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfUndefinedConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfNotUndefinedConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfUndefinedOrNullConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfTrueConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfFalseConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfJSReceiverConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfForInDoneConstant, ImplicitRegisterUse::kNone, OperandType::kIdx, OperandType::kReg, OperandType::kReg);
            // - [Start ToBoolean jumps]
            $V!(JumpIfToBooleanTrueConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(JumpIfToBooleanFalseConstant, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            // - [End constant jumps]
            // - [Conditional immediate jumps]
            $V!(JumpIfToBooleanTrue, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfToBooleanFalse, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            // - [End ToBoolean jumps]
            $V!(JumpIfTrue, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfFalse, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfNull, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfNotNull, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfUndefined, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfNotUndefined, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfUndefinedOrNull, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfJSReceiver, ImplicitRegisterUse::kReadAccumulator, OperandType::kUImm);
            $V!(JumpIfForInDone, ImplicitRegisterUse::kNone, OperandType::kUImm, OperandType::kReg, OperandType::kReg);

            // Smi-table lookup for switch statements
            $V!(SwitchOnSmiNoFeedback, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx, OperandType::kUImm, OperandType::kImm);

            // Complex flow control For..in
            $V!(ForInEnumerate, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg);
            $V!(ForInPrepare, ImplicitRegisterUse::kReadAndClobberAccumulator, OperandType::kRegOutTriple, OperandType::kIdx);
            $V!(ForInNext, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kReg, OperandType::kRegPair, OperandType::kIdx);
            $V!(ForInStep, ImplicitRegisterUse::kNone, OperandType::kRegInOut);

            // Update the pending message
            $V!(SetPendingMessage, ImplicitRegisterUse::kReadWriteAccumulator);

            // Non-local flow control
            $V!(Throw, ImplicitRegisterUse::kReadAccumulator);
            $V!(ReThrow, ImplicitRegisterUse::kReadAccumulator);
            $V!(Return, ImplicitRegisterUse::kReadAccumulator);
            $V!(ThrowReferenceErrorIfHole, ImplicitRegisterUse::kReadAccumulator, OperandType::kIdx);
            $V!(ThrowSuperNotCalledIfHole, ImplicitRegisterUse::kReadAccumulator);
            $V!(ThrowSuperAlreadyCalledIfNotHole, ImplicitRegisterUse::kReadAccumulator);
            $V!(ThrowIfNotSuperConstructor, ImplicitRegisterUse::kNone, OperandType::kReg);

            // Generators
            $V!(SwitchOnGeneratorState, ImplicitRegisterUse::kNone, OperandType::kReg, OperandType::kIdx, OperandType::kUImm);
            $V!(SuspendGenerator, ImplicitRegisterUse::kReadAccumulator, OperandType::kReg, OperandType::kRegList, OperandType::kRegCount, OperandType::kUImm);
            $V!(ResumeGenerator, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kRegOutList, OperandType::kRegCount);

            // Iterator protocol operations
            $V!(GetIterator, ImplicitRegisterUse::kWriteAccumulator, OperandType::kReg, OperandType::kIdx, OperandType::kIdx);

            // Debugger
            $V!(Debugger, ImplicitRegisterUse::kClobberAccumulator);

            // Block Coverage
            $V!(IncBlockCounter, ImplicitRegisterUse::kNone, OperandType::kIdx);

            // Execution Abort (internal error)
            $V!(Abort, ImplicitRegisterUse::kNone, OperandType::kIdx);
        };
    }

    macro_rules! bytecode_list_with_unique_handlers {
        ($V:ident, $V_TSA:ident) => {
            bytecode_list_with_unique_handlers_impl!($V, $V_TSA);
        };
    }

    /// Macro for defining the entire bytecode list.
    macro_rules! bytecode_list {
        ($V:ident, $V_TSA:ident) => {
            bytecode_list_with_unique_handlers!($V, $V_TSA);

            // Special-case Star for common register numbers, to save space
            short_star_bytecode_list!($V);

            // Illegal bytecode
            $V!(Illegal, ImplicitRegisterUse::kNone);
        };
    }

    macro_rules! debug_break_plain_bytecode_list {
        ($V:ident) => {
            $V!(DebugBreak0);
            $V!(DebugBreak1);
            $V!(DebugBreak2);
            $V!(DebugBreak3);
            $V!(DebugBreak4);
            $V!(DebugBreak5);
            $V!(DebugBreak6);
        };
    }

    macro_rules! debug_break_prefix_bytecode_list {
        ($V:ident) => {
            $V!(DebugBreakWide);
            $V!(DebugBreakExtraWide);
        };
    }

    macro_rules! debug_break_bytecode_list {
        ($V:ident) => {
            debug_break_plain_bytecode_list!($V);
            debug_break_prefix_bytecode_list!($V);
        };
    }

    macro_rules! jump_unconditional_immediate_bytecode_list {
        ($V:ident) => {
            $V!(JumpLoop);
            $V!(Jump);
        };
    }

    macro_rules! jump_unconditional_constant_bytecode_list {
        ($V:ident) => {
            $V!(JumpConstant);
        };
    }

    macro_rules! jump_toboolean_conditional_immediate_bytecode_list {
        ($V:ident) => {
            $V!(JumpIfToBooleanTrue);
            $V!(JumpIfToBooleanFalse);
        };
    }

    macro_rules! jump_toboolean_conditional_constant_bytecode_list {
        ($V:ident) => {
            $V!(JumpIfToBooleanTrueConstant);
            $V!(JumpIfToBooleanFalseConstant);
        };
    }

    macro_rules! jump_conditional_immediate_bytecode_list {
        ($V:ident) => {
            jump_toboolean_conditional_immediate_bytecode_list!($V);
            $V!(JumpIfTrue);
            $V!(JumpIfFalse);
            $V!(JumpIfNull);
            $V!(JumpIfNotNull);
            $V!(JumpIfUndefined);
            $V!(JumpIfNotUndefined);
            $V!(JumpIfUndefinedOrNull);
            $V!(JumpIfJSReceiver);
            $V!(JumpIfForInDone);
        };
    }

    macro_rules! jump_conditional_constant_bytecode_list {
        ($V:ident) => {
            jump_toboolean_conditional_constant_bytecode_list!($V);
            $V!(JumpIfNullConstant);
            $V!(JumpIfNotNullConstant);
            $V!(JumpIfUndefinedConstant);
            $V!(JumpIfNotUndefinedConstant);
            $V!(JumpIfUndefinedOrNullConstant);
            $V!(JumpIfTrueConstant);
            $V!(JumpIfFalseConstant);
            $V!(JumpIfJSReceiverConstant);
            $V!(JumpIfForInDoneConstant);
        };
    }

    macro_rules! jump_constant_bytecode_list {
        ($V:ident) => {
            jump_unconditional_constant_bytecode_list!($V);
            jump_conditional_constant_bytecode_list!($V);
        };
    }

    macro_rules! jump_immediate_bytecode_list {
        ($V:ident) => {
            jump_unconditional_immediate_bytecode_list!($V);
            jump_conditional_immediate_bytecode_list!($V);
        };
    }

    macro_rules! jump_to_boolean_bytecode_list {
        ($V:ident) => {
            jump_toboolean_conditional_immediate_bytecode_list!($V);
            jump_toboolean_conditional_constant_bytecode_list!($V);
        };
    }

    macro_rules! jump_unconditional_bytecode_list {
        ($V:ident) => {
            jump_unconditional_immediate_bytecode_list!($V);
            jump_unconditional_constant_bytecode_list!($V);
        };
    }

    macro_rules! jump_conditional_bytecode_list {
        ($V:ident) => {
            jump_conditional_immediate_bytecode_list!($V);
            jump_conditional_constant_bytecode_list!($V);
        };
    }

    macro_rules! jump_forward_bytecode_list {
        ($V:ident) => {
            $V!(Jump);
            $V!(JumpConstant);
            jump_conditional_bytecode_list!($V);
        };
    }

    macro_rules! jump_bytecode_list {
        ($V:ident) => {
            jump_forward_bytecode_list!($V);
            $V!(JumpLoop);
        };
    }

    macro_rules! return_bytecode_list {
        ($V:ident) => {
            $V!(Return);
            $V!(SuspendGenerator);
        };
    }

    macro_rules! unconditional_throw_bytecode_list {
        ($V:ident) => {
            $V!(Throw);
            $V!(ReThrow);
        };
    }

    /// Enumeration of interpreter bytecodes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Bytecode {
        #[allow(dead_code)]
        #[allow(clippy::enum_variant_names)]
        #[allow(non_camel_case_types)]
        #[allow(unused_imports)]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        #[allow(unreachable_patterns)]
        #[allow(unused_parens)]
        #[allow(unused_braces)]
        #[allow(unused_lifetimes)]
        #[allow(unused_unsafe)]
        #[allow(unused_attributes)]
        #[allow(unused_macros)]
        #[allow(unused_imports)]
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        #[allow(unreachable_patterns)]
        #[allow(unused_parens)]
        #[allow(unused_braces)]
        #[allow(unused_lifetimes)]
        #[allow(unused_unsafe)]
        #[allow(unused_attributes)]
        #[allow(unused_macros)]
        #[allow(unused_imports)]