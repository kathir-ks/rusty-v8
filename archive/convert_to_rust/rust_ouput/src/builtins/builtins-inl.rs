// Converted from V8 C++ source files:
// Header: builtins-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins_inl {
    use crate::builtins::builtins::Builtin;
    use crate::execution::isolate::Isolate;
    use crate::codegen::loong64::assembler_loong64::ConvertReceiverMode;
    use crate::interpreter::interpreter_generator::TypeofMode;

    pub enum SaveFPRegsMode {
        kIgnore,
        kSave,
    }

    const kDontAdaptArgumentsSentinel: i32 = -1;

    // Define a JSParameterCount macro equivalent (as it seems to be used to wrap numbers)
    macro_rules! JSParameterCount {
        ($x:expr) => {
            $x
        };
    }

    pub enum ToPrimitiveHint {
        kDefault,
        kNumber,
        kString,
    }

    pub enum OrdinaryToPrimitiveHint {
        kNumber,
        kString,
    }

    pub enum StringAddFlags {
        STRING_ADD_CHECK_NONE,
        STRING_ADD_CONVERT_LEFT,
        STRING_ADD_CONVERT_RIGHT,
    }

    pub enum ArgvMode {
        kStack,
        kRegister,
    }

    pub enum InterpreterPushArgsMode {
        kArrayFunction,
        kWithFinalSpread,
        kOther,
    }

    // Placeholder for UNREACHABLE macro
    macro_rules! UNREACHABLE {
        () => {
            panic!("Unreachable code reached!");
        };
    }

    // Placeholder for CHECK macro
    macro_rules! CHECK {
        ($x:expr) => {
            if !$x {
                panic!("Check failed: {}", stringify!($x));
            }
        };
    }

    // static
    pub fn record_write(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kRecordWriteIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kRecordWriteSaveFP,
        }
    }

    // static
    pub fn indirect_pointer_barrier(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kIndirectPointerBarrierIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kIndirectPointerBarrierSaveFP,
        }
    }

    // static
    pub fn ephemeron_key_barrier(fp_mode: SaveFPRegsMode) -> Builtin {
        match fp_mode {
            SaveFPRegsMode::kIgnore => Builtin::kEphemeronKeyBarrierIgnoreFP,
            SaveFPRegsMode::kSave => Builtin::kEphemeronKeyBarrierSaveFP,
        }
    }

    // static
    pub fn adaptor_with_builtin_exit_frame(formal_parameter_count: i32) -> Builtin {
        match formal_parameter_count {
            kDontAdaptArgumentsSentinel => Builtin::kAdaptorWithBuiltinExitFrame0,
            JSParameterCount!(0) => Builtin::kAdaptorWithBuiltinExitFrame0,
            JSParameterCount!(1) => Builtin::kAdaptorWithBuiltinExitFrame1,
            JSParameterCount!(2) => Builtin::kAdaptorWithBuiltinExitFrame2,
            JSParameterCount!(3) => Builtin::kAdaptorWithBuiltinExitFrame3,
            JSParameterCount!(4) => Builtin::kAdaptorWithBuiltinExitFrame4,
            JSParameterCount!(5) => Builtin::kAdaptorWithBuiltinExitFrame5,
            _ => UNREACHABLE!(),
        }
    }

    // static
    pub fn call_function(mode: ConvertReceiverMode) -> Builtin {
        match mode {
            ConvertReceiverMode::kNullOrUndefined => Builtin::kCallFunction_ReceiverIsNullOrUndefined,
            ConvertReceiverMode::kNotNullOrUndefined => Builtin::kCallFunction_ReceiverIsNotNullOrUndefined,
            ConvertReceiverMode::kAny => Builtin::kCallFunction_ReceiverIsAny,
        }
    }

    // static
    pub fn call(mode: ConvertReceiverMode) -> Builtin {
        match mode {
            ConvertReceiverMode::kNullOrUndefined => Builtin::kCall_ReceiverIsNullOrUndefined,
            ConvertReceiverMode::kNotNullOrUndefined => Builtin::kCall_ReceiverIsNotNullOrUndefined,
            ConvertReceiverMode::kAny => Builtin::kCall_ReceiverIsAny,
        }
    }

    // static
    pub fn is_any_call(builtin: Builtin) -> bool {
        match builtin {
            Builtin::kCallFunction_ReceiverIsNullOrUndefined |
            Builtin::kCallFunction_ReceiverIsNotNullOrUndefined |
            Builtin::kCallFunction_ReceiverIsAny |
            Builtin::kCall_ReceiverIsNullOrUndefined |
            Builtin::kCall_ReceiverIsNotNullOrUndefined |
            Builtin::kCall_ReceiverIsAny => true,
            _ => false,
        }
    }

    // static
    pub fn non_primitive_to_primitive(hint: ToPrimitiveHint) -> Builtin {
        match hint {
            ToPrimitiveHint::kDefault => Builtin::kNonPrimitiveToPrimitive_Default,
            ToPrimitiveHint::kNumber => Builtin::kNonPrimitiveToPrimitive_Number,
            ToPrimitiveHint::kString => Builtin::kNonPrimitiveToPrimitive_String,
        }
    }

    // static
    pub fn ordinary_to_primitive(hint: OrdinaryToPrimitiveHint) -> Builtin {
        match hint {
            OrdinaryToPrimitiveHint::kNumber => Builtin::kOrdinaryToPrimitive_Number,
            OrdinaryToPrimitiveHint::kString => Builtin::kOrdinaryToPrimitive_String,
        }
    }

    // static
    pub fn string_add(flags: StringAddFlags) -> Builtin {
        match flags {
            StringAddFlags::STRING_ADD_CHECK_NONE => Builtin::kStringAdd_CheckNone,
            StringAddFlags::STRING_ADD_CONVERT_LEFT => Builtin::kStringAddConvertLeft,
            StringAddFlags::STRING_ADD_CONVERT_RIGHT => Builtin::kStringAddConvertRight,
        }
    }

    // static
    pub fn load_global_ic(typeof_mode: TypeofMode) -> Builtin {
        match typeof_mode {
            TypeofMode::kNotInside => Builtin::kLoadGlobalICTrampoline,
            _ => Builtin::kLoadGlobalICInsideTypeofTrampoline, // Corrected logic
        }
    }

    // static
    pub fn load_global_ic_in_optimized_code(typeof_mode: TypeofMode) -> Builtin {
        match typeof_mode {
            TypeofMode::kNotInside => Builtin::kLoadGlobalIC,
            _ => Builtin::kLoadGlobalICInsideTypeof, // Corrected logic
        }
    }

    // static
    pub fn c_entry(result_size: i32, argv_mode: ArgvMode, builtin_exit_frame: bool, switch_to_central_stack: bool) -> Builtin {
        // Aliases for readability below.
        let rs = result_size;
        let am = argv_mode;
        let be = builtin_exit_frame;

        if switch_to_central_stack {
            assert_eq!(result_size, 1);
            assert_eq!(match argv_mode{ ArgvMode::kStack => true, _ => false}, true); //equivalent to ArgvMode::kStack
            assert_eq!(builtin_exit_frame, false);
            return Builtin::kWasmCEntry;
        }

        if rs == 1 && match am{ ArgvMode::kStack => true, _ => false} && !be {
            return Builtin::kCEntry_Return1_ArgvOnStack_NoBuiltinExit;
        } else if rs == 1 && match am{ ArgvMode::kStack => true, _ => false} && be {
            return Builtin::kCEntry_Return1_ArgvOnStack_BuiltinExit;
        } else if rs == 1 && match am{ ArgvMode::kRegister => true, _ => false} && !be {
            return Builtin::kCEntry_Return1_ArgvInRegister_NoBuiltinExit;
        } else if rs == 2 && match am{ ArgvMode::kStack => true, _ => false} && !be {
            return Builtin::kCEntry_Return2_ArgvOnStack_NoBuiltinExit;
        } else if rs == 2 && match am{ ArgvMode::kStack => true, _ => false} && be {
            return Builtin::kCEntry_Return2_ArgvOnStack_BuiltinExit;
        } else if rs == 2 && match am{ ArgvMode::kRegister => true, _ => false} && !be {
            return Builtin::kCEntry_Return2_ArgvInRegister_NoBuiltinExit;
        }

        UNREACHABLE!();
    }

    // static
    pub fn runtime_c_entry(result_size: i32, switch_to_central_stack: bool) -> Builtin {
        c_entry(result_size, ArgvMode::kStack, false, switch_to_central_stack)
    }

    // static
    pub fn interpreter_c_entry(result_size: i32) -> Builtin {
        c_entry(result_size, ArgvMode::kRegister, false, false)
    }

    // static
    pub fn interpreter_push_args_then_call(receiver_mode: ConvertReceiverMode, mode: InterpreterPushArgsMode) -> Builtin {
        match mode {
            InterpreterPushArgsMode::kArrayFunction => {
                // There is no special-case handling of calls to Array. They will all go
                // through the kOther case below.
                UNREACHABLE!();
            }
            InterpreterPushArgsMode::kWithFinalSpread => Builtin::kInterpreterPushArgsThenCallWithFinalSpread,
            InterpreterPushArgsMode::kOther => {
                match receiver_mode {
                    ConvertReceiverMode::kNullOrUndefined => Builtin::kInterpreterPushUndefinedAndArgsThenCall,
                    ConvertReceiverMode::kNotNullOrUndefined | ConvertReceiverMode::kAny => Builtin::kInterpreterPushArgsThenCall,
                }
            }
        }
    }

    // static
    pub fn interpreter_push_args_then_construct(mode: InterpreterPushArgsMode) -> Builtin {
        match mode {
            InterpreterPushArgsMode::kArrayFunction => Builtin::kInterpreterPushArgsThenConstructArrayFunction,
            InterpreterPushArgsMode::kWithFinalSpread => Builtin::kInterpreterPushArgsThenConstructWithFinalSpread,
            InterpreterPushArgsMode::kOther => Builtin::kInterpreterPushArgsThenConstruct,
        }
    }

    // static
    pub fn entry_of(builtin: Builtin, isolate: &mut Isolate) -> Address {
        isolate.builtin_entry_table()[Builtin::to_int(builtin) as usize]
    }

    // Define a type alias for addresses, assuming it's a raw pointer.
    pub type Address = *const u8;

    // static
    pub fn is_js_entry_variant(builtin: Builtin) -> bool {
        match builtin {
            Builtin::kJSEntry | Builtin::kJSConstructEntry | Builtin::kJSRunMicrotasksEntry => true,
            _ => false,
        }
    }

    // static
    pub fn get_formal_parameter_count(builtin: Builtin) -> i32 {
        fn has_js_linkage(_builtin: Builtin) -> bool {
            true
        }

        CHECK!(has_js_linkage(builtin));

        fn get_stack_parameter_count(_builtin: Builtin) -> i32 {
            0
        }

        fn kind_of(_builtin: Builtin) -> i32 {
            0
        }

        const TSJ: i32 = 1;
        const TFJ: i32 = 2;
        const ASM: i32 = 3;
        const TFC: i32 = 4;
        const CPP: i32 = 5;

        if kind_of(builtin) == TSJ || kind_of(builtin) == TFJ {
            return get_stack_parameter_count(builtin);
        } else if kind_of(builtin) == ASM || kind_of(builtin) == TFC {
            // At the moment, all ASM builtins are varargs builtins. This is verified
            // in CheckFormalParameterCount.
            return kDontAdaptArgumentsSentinel;
        } else if kind_of(builtin) == CPP {
            // Define a mock BUILTIN_LIST_C macro for this example.
            macro_rules! CPP_BUILTIN {
                ($Name:ident, $Argc:expr) => {
                   if builtin == Builtin::k##$Name {
                        return $Argc;
                   }
                };
            }
            macro_rules! BUILTIN_LIST_C {
                ($macro:ident) => {
                    $macro!(Abort, 1);
                    $macro!(ArgumentsAdaptor, kDontAdaptArgumentsSentinel); // Example
                };
            }
            BUILTIN_LIST_C!(CPP_BUILTIN);
            return 0;
        } else {
            UNREACHABLE!();
        }
    }
}
