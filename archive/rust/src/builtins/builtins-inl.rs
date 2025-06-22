// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a partial translation, as it only covers the inline definitions
// and omits the declarations from "src/builtins/builtins.h" and
// "src/execution/isolate.h" which are not provided. A complete translation
// would require those definitions as well.  The `Builtin` enum and other
// types like `SaveFPRegsMode`, `ConvertReceiverMode`, `ToPrimitiveHint`,
// `OrdinaryToPrimitiveHint`, `StringAddFlags`, `TypeofMode`, `ArgvMode`,
// `InterpreterPushArgsMode` are assumed to be defined elsewhere. Also, the
// macros like `UNREACHABLE()`, `DCHECK_EQ`, `CHECK`, `JSParameterCount` are
// not translated as they depend heavily on the surrounding V8 context and
// would be difficult to replicate faithfully without more information.
// The values of the enum variants of Builtin are also needed to completely translate.

// Assuming definitions for types and enums from the original C++ code.
// These are placeholders and need to be replaced with actual Rust definitions
// derived from the corresponding C++ headers.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SaveFPRegsMode {
    kIgnore,
    kSave,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ConvertReceiverMode {
    kNullOrUndefined,
    kNotNullOrUndefined,
    kAny,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ToPrimitiveHint {
    kDefault,
    kNumber,
    kString,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OrdinaryToPrimitiveHint {
    kNumber,
    kString,
}

type StringAddFlags = i32; // Placeholder
const STRING_ADD_CHECK_NONE: StringAddFlags = 0; // Placeholder
const STRING_ADD_CONVERT_LEFT: StringAddFlags = 1; // Placeholder
const STRING_ADD_CONVERT_RIGHT: StringAddFlags = 2; // Placeholder

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TypeofMode {
    kNotInside,
    kInside,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ArgvMode {
    kStack,
    kRegister,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InterpreterPushArgsMode {
    kArrayFunction,
    kWithFinalSpread,
    kOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Builtin {
    kRecordWriteIgnoreFP,
    kRecordWriteSaveFP,
    kIndirectPointerBarrierIgnoreFP,
    kIndirectPointerBarrierSaveFP,
    kEphemeronKeyBarrierIgnoreFP,
    kEphemeronKeyBarrierSaveFP,
    kAdaptorWithBuiltinExitFrame0,
    kAdaptorWithBuiltinExitFrame1,
    kAdaptorWithBuiltinExitFrame2,
    kAdaptorWithBuiltinExitFrame3,
    kAdaptorWithBuiltinExitFrame4,
    kAdaptorWithBuiltinExitFrame5,
    kCallFunction_ReceiverIsNullOrUndefined,
    kCallFunction_ReceiverIsNotNullOrUndefined,
    kCallFunction_ReceiverIsAny,
    kCall_ReceiverIsNullOrUndefined,
    kCall_ReceiverIsNotNullOrUndefined,
    kCall_ReceiverIsAny,
    kNonPrimitiveToPrimitive_Default,
    kNonPrimitiveToPrimitive_Number,
    kNonPrimitiveToPrimitive_String,
    kOrdinaryToPrimitive_Number,
    kOrdinaryToPrimitive_String,
    kStringAdd_CheckNone,
    kStringAddConvertLeft,
    kStringAddConvertRight,
    kLoadGlobalICTrampoline,
    kLoadGlobalICInsideTypeofTrampoline,
    kLoadGlobalIC,
    kLoadGlobalICInsideTypeof,
    kWasmCEntry,
    kCEntry_Return1_ArgvOnStack_NoBuiltinExit,
    kCEntry_Return1_ArgvOnStack_BuiltinExit,
    kCEntry_Return1_ArgvInRegister_NoBuiltinExit,
    kCEntry_Return2_ArgvOnStack_NoBuiltinExit,
    kCEntry_Return2_ArgvOnStack_BuiltinExit,
    kCEntry_Return2_ArgvInRegister_NoBuiltinExit,
    kInterpreterPushArgsThenCallWithFinalSpread,
    kInterpreterPushUndefinedAndArgsThenCall,
    kInterpreterPushArgsThenCall,
    kInterpreterPushArgsThenConstructArrayFunction,
    kInterpreterPushArgsThenConstructWithFinalSpread,
    kInterpreterPushArgsThenConstruct,
    kJSEntry,
    kJSConstructEntry,
    kJSRunMicrotasksEntry,
    // Add other variants as needed
}

// Placeholder for types used in original C++ code but not defined here.
struct Isolate;

const kDontAdaptArgumentsSentinel: i32 = -1; // Placeholder

// Mimic the JSParameterCount macro.
macro_rules! JSParameterCount {
    ($x:expr) => {
        $x
    };
}

// Implement Builtins as a struct with static methods
struct Builtins;

impl Builtins {
    /// Returns the Builtin for RecordWrite based on SaveFPRegsMode.
    const fn RecordWrite(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kRecordWriteIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kRecordWriteSaveFP,
        }
    }

    /// Returns the Builtin for IndirectPointerBarrier based on SaveFPRegsMode.
    const fn IndirectPointerBarrier(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kIndirectPointerBarrierIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kIndirectPointerBarrierSaveFP,
        }
    }

    /// Returns the Builtin for EphemeronKeyBarrier based on SaveFPRegsMode.
    const fn EphemeronKeyBarrier(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kEphemeronKeyBarrierIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kEphemeronKeyBarrierSaveFP,
        }
    }

    /// Returns the Builtin for AdaptorWithBuiltinExitFrame based on the
    /// formal parameter count.
    fn AdaptorWithBuiltinExitFrame(formal_parameter_count: i32) -> Builtin {
        match formal_parameter_count {
            kDontAdaptArgumentsSentinel => Builtin::kAdaptorWithBuiltinExitFrame0,
            JSParameterCount!(0) => Builtin::kAdaptorWithBuiltinExitFrame0,
            JSParameterCount!(1) => Builtin::kAdaptorWithBuiltinExitFrame1,
            JSParameterCount!(2) => Builtin::kAdaptorWithBuiltinExitFrame2,
            JSParameterCount!(3) => Builtin::kAdaptorWithBuiltinExitFrame3,
            JSParameterCount!(4) => Builtin::kAdaptorWithBuiltinExitFrame4,
            JSParameterCount!(5) => Builtin::kAdaptorWithBuiltinExitFrame5,
            _ => {
                // Mimic UNREACHABLE. In real code, panic or return a Result.
                panic!("Unreachable code in AdaptorWithBuiltinExitFrame");
            }
        }
    }

    /// Returns the Builtin for CallFunction based on ConvertReceiverMode.
    const fn CallFunction(mode: ConvertReceiverMode) -> Builtin {
        match mode {
            ConvertReceiverMode::kNullOrUndefined => {
                Builtin::kCallFunction_ReceiverIsNullOrUndefined
            }
            ConvertReceiverMode::kNotNullOrUndefined => {
                Builtin::kCallFunction_ReceiverIsNotNullOrUndefined
            }
            ConvertReceiverMode::kAny => Builtin::kCallFunction_ReceiverIsAny,
        }
    }

    /// Returns the Builtin for Call based on ConvertReceiverMode.
    const fn Call(mode: ConvertReceiverMode) -> Builtin {
        match mode {
            ConvertReceiverMode::kNullOrUndefined => Builtin::kCall_ReceiverIsNullOrUndefined,
            ConvertReceiverMode::kNotNullOrUndefined => {
                Builtin::kCall_ReceiverIsNotNullOrUndefined
            }
            ConvertReceiverMode::kAny => Builtin::kCall_ReceiverIsAny,
        }
    }

    /// Checks if the given Builtin is one of the Call variants.
    const fn IsAnyCall(builtin: Builtin) -> bool {
        match builtin {
            Builtin::kCallFunction_ReceiverIsNullOrUndefined
            | Builtin::kCallFunction_ReceiverIsNotNullOrUndefined
            | Builtin::kCallFunction_ReceiverIsAny
            | Builtin::kCall_ReceiverIsNullOrUndefined
            | Builtin::kCall_ReceiverIsNotNullOrUndefined
            | Builtin::kCall_ReceiverIsAny => true,
            _ => false,
        }
    }

    /// Returns the Builtin for NonPrimitiveToPrimitive based on ToPrimitiveHint.
    const fn NonPrimitiveToPrimitive(hint: ToPrimitiveHint) -> Builtin {
        match hint {
            ToPrimitiveHint::kDefault => Builtin::kNonPrimitiveToPrimitive_Default,
            ToPrimitiveHint::kNumber => Builtin::kNonPrimitiveToPrimitive_Number,
            ToPrimitiveHint::kString => Builtin::kNonPrimitiveToPrimitive_String,
        }
    }

    /// Returns the Builtin for OrdinaryToPrimitive based on OrdinaryToPrimitiveHint.
    const fn OrdinaryToPrimitive(hint: OrdinaryToPrimitiveHint) -> Builtin {
        match hint {
            OrdinaryToPrimitiveHint::kNumber => Builtin::kOrdinaryToPrimitive_Number,
            OrdinaryToPrimitiveHint::kString => Builtin::kOrdinaryToPrimitive_String,
        }
    }

    /// Returns the Builtin for StringAdd based on StringAddFlags.
    const fn StringAdd(flags: StringAddFlags) -> Builtin {
        match flags {
            STRING_ADD_CHECK_NONE => Builtin::kStringAdd_CheckNone,
            STRING_ADD_CONVERT_LEFT => Builtin::kStringAddConvertLeft,
            STRING_ADD_CONVERT_RIGHT => Builtin::kStringAddConvertRight,
            _ => {
                // Mimic UNREACHABLE. In real code, panic or return a Result.
                panic!("Unreachable code in StringAdd");
            }
        }
    }

    /// Returns the Builtin for LoadGlobalIC based on TypeofMode.
    const fn LoadGlobalIC(typeof_mode: TypeofMode) -> Builtin {
        if typeof_mode == TypeofMode::kNotInside {
            Builtin::kLoadGlobalICTrampoline
        } else {
            Builtin::kLoadGlobalICInsideTypeofTrampoline
        }
    }

    /// Returns the Builtin for LoadGlobalICInOptimizedCode based on TypeofMode.
    const fn LoadGlobalICInOptimizedCode(typeof_mode: TypeofMode) -> Builtin {
        if typeof_mode == TypeofMode::kNotInside {
            Builtin::kLoadGlobalIC
        } else {
            Builtin::kLoadGlobalICInsideTypeof
        }
    }

    /// Returns the Builtin for CEntry based on various parameters.
    fn CEntry(
        result_size: i32,
        argv_mode: ArgvMode,
        builtin_exit_frame: bool,
        switch_to_central_stack: bool,
    ) -> Builtin {
        // Aliases for readability below.
        let rs = result_size;
        let am = argv_mode;
        let be = builtin_exit_frame;

        if switch_to_central_stack {
            // Mimic DCHECK_EQ
            if result_size != 1 || argv_mode != ArgvMode::kStack || builtin_exit_frame != false {
                panic!("DCHECK_EQ failed in CEntry");
            }
            return Builtin::kWasmCEntry;
        }

        if rs == 1 && am == ArgvMode::kStack && !be {
            Builtin::kCEntry_Return1_ArgvOnStack_NoBuiltinExit
        } else if rs == 1 && am == ArgvMode::kStack && be {
            Builtin::kCEntry_Return1_ArgvOnStack_BuiltinExit
        } else if rs == 1 && am == ArgvMode::kRegister && !be {
            Builtin::kCEntry_Return1_ArgvInRegister_NoBuiltinExit
        } else if rs == 2 && am == ArgvMode::kStack && !be {
            Builtin::kCEntry_Return2_ArgvOnStack_NoBuiltinExit
        } else if rs == 2 && am == ArgvMode::kStack && be {
            Builtin::kCEntry_Return2_ArgvOnStack_BuiltinExit
        } else if rs == 2 && am == ArgvMode::kRegister && !be {
            Builtin::kCEntry_Return2_ArgvInRegister_NoBuiltinExit
        } else {
            // Mimic UNREACHABLE. In real code, panic or return a Result.
            panic!("Unreachable code in CEntry");
        }
    }

    /// Returns the Builtin for RuntimeCEntry based on result_size and
    /// switch_to_central_stack.
    fn RuntimeCEntry(result_size: i32, switch_to_central_stack: bool) -> Builtin {
        Builtins::CEntry(result_size, ArgvMode::kStack, false, switch_to_central_stack)
    }

    /// Returns the Builtin for InterpreterCEntry based on result_size.
    fn InterpreterCEntry(result_size: i32) -> Builtin {
        Builtins::CEntry(result_size, ArgvMode::kRegister, false, false)
    }

    /// Returns the Builtin for InterpreterPushArgsThenCall based on
    /// receiver_mode and mode.
    fn InterpreterPushArgsThenCall(
        receiver_mode: ConvertReceiverMode,
        mode: InterpreterPushArgsMode,
    ) -> Builtin {
        match mode {
            InterpreterPushArgsMode::kArrayFunction => {
                // Mimic UNREACHABLE. In real code, panic or return a Result.
                panic!("Unreachable code in InterpreterPushArgsThenCall");
            }
            InterpreterPushArgsMode::kWithFinalSpread => {
                Builtin::kInterpreterPushArgsThenCallWithFinalSpread
            }
            InterpreterPushArgsMode::kOther => match receiver_mode {
                ConvertReceiverMode::kNullOrUndefined => {
                    Builtin::kInterpreterPushUndefinedAndArgsThenCall
                }
                ConvertReceiverMode::kNotNullOrUndefined | ConvertReceiverMode::kAny => {
                    Builtin::kInterpreterPushArgsThenCall
                }
            },
        }
    }

    /// Returns the Builtin for InterpreterPushArgsThenConstruct based on mode.
    fn InterpreterPushArgsThenConstruct(mode: InterpreterPushArgsMode) -> Builtin {
        match mode {
            InterpreterPushArgsMode::kArrayFunction => {
                Builtin::kInterpreterPushArgsThenConstructArrayFunction
            }
            InterpreterPushArgsMode::kWithFinalSpread => {
                Builtin::kInterpreterPushArgsThenConstructWithFinalSpread
            }
            InterpreterPushArgsMode::kOther => Builtin::kInterpreterPushArgsThenConstruct,
        }
    }

    /// Returns the entry address of a Builtin in the isolate.  This requires
    /// access to the Isolate's internal state, which is not available here.
    /// Returns None to indicate the entry cannot be retrieved.
    fn EntryOf(_builtin: Builtin, _isolate: &Isolate) -> Option<usize> {
        // TODO(you): implement the logic to retrieve the entry address
        // from the isolate. This requires access to isolate.builtin_entry_table()
        // which is not available without the Isolate definition.
        // let table = isolate.builtin_entry_table();
        // return table[Builtins::ToInt(builtin)];
        None // Indicate the entry point could not be retrieved.
    }

    /// Checks if the given Builtin is a JSEntry variant.
    const fn IsJSEntryVariant(builtin: Builtin) -> bool {
        match builtin {
            Builtin::kJSEntry | Builtin::kJSConstructEntry | Builtin::kJSRunMicrotasksEntry => {
                true
            }
            _ => false,
        }
    }

    // The following functions are unimplemented because they require access to
    // other parts of the V8 codebase that are not provided.
    // GetFormalParameterCount, HasJSLinkage, KindOf, GetStackParameterCount,
    // ToInt.  These would require further definitions and dependencies to be
    // fully implemented.

    // Placeholder function
    fn GetFormalParameterCount(_builtin: Builtin) -> i32 {
      // Mimic UNREACHABLE. In real code, panic or return a Result.
      panic!("Unreachable code in GetFormalParameterCount");
    }

    // Placeholder function
    const fn HasJSLinkage(_builtin: Builtin) -> bool {
      // Mimic UNREACHABLE. In real code, panic or return a Result.
      panic!("Unreachable code in HasJSLinkage");
    }

    // Placeholder function
    fn KindOf(_builtin: Builtin) -> i32 {
      // Mimic UNREACHABLE. In real code, panic or return a Result.
      panic!("Unreachable code in KindOf");
    }

    // Placeholder function
    fn GetStackParameterCount(_builtin: Builtin) -> i32 {
      // Mimic UNREACHABLE. In real code, panic or return a Result.
      panic!("Unreachable code in GetStackParameterCount");
    }

    // Placeholder function
    fn ToInt(_builtin: Builtin) -> i32 {
      // Mimic UNREACHABLE. In real code, panic or return a Result.
      panic!("Unreachable code in ToInt");
    }

}