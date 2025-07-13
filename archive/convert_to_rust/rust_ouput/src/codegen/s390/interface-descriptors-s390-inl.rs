// Converted from V8 C++ source files:
// Header: interface-descriptors-s390-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::marker::PhantomData;

//use crate::codegen::interface_descriptors::*;
//use crate::execution::frames::*;

// Mocked dependencies
pub struct RegisterArray(());
pub struct DoubleRegisterArray(());
pub struct RegList(());
pub struct CallInterfaceDescriptorData(());
pub enum Register {
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    kReturnRegister0,
    kReturnRegister1,
    kReturnRegister2,
    kContextRegister,
    kInterpreterAccumulatorRegister,
    kInterpreterBytecodeOffsetRegister,
    kInterpreterBytecodeArrayRegister,
    kInterpreterDispatchTableRegister,
}

pub enum DoubleRegister {
    d1,
    d2,
    d3,
    d4,
    d5,
    d6,
    d7,
    kFPReturnRegister0,
    no_dreg,
}
pub enum ConvertReceiverMode {}
pub enum Builtin {}
pub struct OpIndex {}
pub struct Node {}
pub struct Context {}
pub struct Tagged<T> {
    phantom: PhantomData<T>,
}
pub struct JsonObject {}
pub struct Expression {}
pub struct RegExpNodeInfo {}
pub struct MSAControlRegister {}

const kMaxBuiltinRegisterParams: usize = 5;

const fn RegisterArray(r1: Register, r2: Register, r3: Register, r4: Register, r5: Register) -> RegisterArray {
    RegisterArray(())
}

const fn DoubleRegisterArray(d1: DoubleRegister, d2: DoubleRegister, d3: DoubleRegister, d4: DoubleRegister, d5: DoubleRegister, d6: DoubleRegister, d7: DoubleRegister) -> DoubleRegisterArray {
    DoubleRegisterArray(())
}

const fn RegisterArray_ret(r1: Register, r2: Register, r3: Register) -> RegisterArray {
    RegisterArray(())
}

const fn DoubleRegisterArray_ret(r1: DoubleRegister, r2: DoubleRegister, r3: DoubleRegister) -> DoubleRegisterArray {
    DoubleRegisterArray(())
}

pub struct CallInterfaceDescriptor {}
impl CallInterfaceDescriptor {
    pub const fn DefaultRegisterArray() -> RegisterArray {
        RegisterArray(Register::r2, Register::r3, Register::r4, Register::r5, Register::r6)
    }

    pub const fn DefaultDoubleRegisterArray() -> DoubleRegisterArray {
        DoubleRegisterArray(DoubleRegister::d1, DoubleRegister::d2, DoubleRegister::d3, DoubleRegister::d4, DoubleRegister::d5, DoubleRegister::d6, DoubleRegister::d7)
    }

    pub const fn DefaultReturnRegisterArray() -> RegisterArray {
        RegisterArray_ret(Register::kReturnRegister0, Register::kReturnRegister1, Register::kReturnRegister2)
    }

    pub const fn DefaultReturnDoubleRegisterArray() -> DoubleRegisterArray {
        DoubleRegisterArray_ret(DoubleRegister::kFPReturnRegister0, DoubleRegister::no_dreg, DoubleRegister::no_dreg)
    }
}

pub struct StaticCallInterfaceDescriptor<T> {
    phantom: PhantomData<T>,
}

impl<DerivedDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    #[cfg(debug_assertions)]
    pub fn VerifyArgumentRegisterCount(_data: &CallInterfaceDescriptorData, argc: i32) {
        if argc >= 1 { println!("DCHECK(allocatable_regs.has(r2))"); }
        if argc >= 2 { println!("DCHECK(allocatable_regs.has(r3))"); }
        if argc >= 3 { println!("DCHECK(allocatable_regs.has(r4))"); }
        if argc >= 4 { println!("DCHECK(allocatable_regs.has(r5))"); }
        if argc >= 5 { println!("DCHECK(allocatable_regs.has(r6))"); }
        if argc >= 6 { println!("DCHECK(allocatable_regs.has(r7))"); }
        if argc >= 7 { println!("DCHECK(allocatable_regs.has(r8))"); }
        if argc >= 8 { println!("DCHECK(allocatable_regs.has(r9))"); }
        // Additional arguments are passed on the stack.
    }
}

pub struct WriteBarrierDescriptor {}
impl WriteBarrierDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r7, Register::r6, Register::r4, Register::r2, Register::r5, Register::kContextRegister)
    }
}

pub struct LoadDescriptor {}
impl LoadDescriptor {
    pub const fn ReceiverRegister() -> Register { Register::r3 }
    pub const fn NameRegister() -> Register { Register::r4 }
    pub const fn SlotRegister() -> Register { Register::r2 }
}

pub struct LoadWithVectorDescriptor {}
impl LoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register { Register::r5 }
}

pub struct KeyedLoadBaselineDescriptor {}
impl KeyedLoadBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register { Register::r3 }
    pub const fn NameRegister() -> Register { Register::kInterpreterAccumulatorRegister }
    pub const fn SlotRegister() -> Register { Register::r4 }
}

pub struct KeyedLoadWithVectorDescriptor {}
impl KeyedLoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register { Register::r5 }
}

pub struct EnumeratedKeyedLoadBaselineDescriptor {}
impl EnumeratedKeyedLoadBaselineDescriptor {
    pub const fn EnumIndexRegister() -> Register { Register::r6 }
    pub const fn CacheTypeRegister() -> Register { Register::r7 }
    pub const fn SlotRegister() -> Register { Register::r4 }
}

pub struct KeyedHasICBaselineDescriptor {}
impl KeyedHasICBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register { Register::kInterpreterAccumulatorRegister }
    pub const fn NameRegister() -> Register { Register::r3 }
    pub const fn SlotRegister() -> Register { Register::r4 }
}

pub struct KeyedHasICWithVectorDescriptor {}
impl KeyedHasICWithVectorDescriptor {
    pub const fn VectorRegister() -> Register { Register::r5 }
}

pub struct LoadWithReceiverAndVectorDescriptor {}
impl LoadWithReceiverAndVectorDescriptor {
    pub const fn LookupStartObjectRegister() -> Register { Register::r6 }
}

pub struct StoreDescriptor {}
impl StoreDescriptor {
    pub const fn ReceiverRegister() -> Register { Register::r3 }
    pub const fn NameRegister() -> Register { Register::r4 }
    pub const fn ValueRegister() -> Register { Register::r2 }
    pub const fn SlotRegister() -> Register { Register::r6 }
}

pub struct StoreWithVectorDescriptor {}
impl StoreWithVectorDescriptor {
    pub const fn VectorRegister() -> Register { Register::r5 }
}

pub struct DefineKeyedOwnDescriptor {}
impl DefineKeyedOwnDescriptor {
    pub const fn FlagsRegister() -> Register { Register::r7 }
}

pub struct StoreTransitionDescriptor {}
impl StoreTransitionDescriptor {
    pub const fn MapRegister() -> Register { Register::r7 }
}

pub struct ApiGetterDescriptor {}
impl ApiGetterDescriptor {
    pub const fn HolderRegister() -> Register { Register::r2 }
    pub const fn CallbackRegister() -> Register { Register::r5 }
}

pub struct GrowArrayElementsDescriptor {}
impl GrowArrayElementsDescriptor {
    pub const fn ObjectRegister() -> Register { Register::r2 }
    pub const fn KeyRegister() -> Register { Register::r5 }
}

pub struct BaselineLeaveFrameDescriptor {}
impl BaselineLeaveFrameDescriptor {
    pub const fn ParamsSizeRegister() -> Register {
        Register::r5
    }
    pub const fn WeightRegister() -> Register {
        Register::r6
    }
}

pub struct TypeConversionDescriptor {}
impl TypeConversionDescriptor {
    pub const fn ArgumentRegister() -> Register { Register::r2 }
}

pub struct TypeofDescriptor {}
impl TypeofDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r2, Register::r2, Register::r2, Register::r2, Register::r2) }
}

pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {}
impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
    pub const fn FlagsRegister() -> Register { Register::r4 }
    pub const fn FeedbackVectorRegister() -> Register { Register::r7 }
    pub const fn TemporaryRegister() -> Register { Register::r6 }
}

pub struct CallTrampolineDescriptor {}
impl CallTrampolineDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r2, Register::r2, Register::r2)
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}
impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r2, Register::r2, Register::r2)
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}
impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r4, Register::r2, Register::r2)
    }
}

pub struct CallVarargsDescriptor {}
impl CallVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r6, Register::r4, Register::r2)
    }
}

pub struct CallForwardVarargsDescriptor {}
impl CallForwardVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r4, Register::r2, Register::r2)
    }
}

pub struct CallFunctionTemplateDescriptor {}
impl CallFunctionTemplateDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r4, Register::r2, Register::r2, Register::r2)
    }
}

pub struct CallFunctionTemplateGenericDescriptor {}
impl CallFunctionTemplateGenericDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r4, Register::r5, Register::r2, Register::r2)
    }
}

pub struct CallWithSpreadDescriptor {}
impl CallWithSpreadDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r2, Register::r4, Register::r2, Register::r2)
    }
}

pub struct CallWithArrayLikeDescriptor {}
impl CallWithArrayLikeDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r4, Register::r2, Register::r2, Register::r2)
    }
}

pub struct ConstructVarargsDescriptor {}
impl ConstructVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r5, Register::r2, Register::r6, Register::r4)
    }
}

pub struct ConstructForwardVarargsDescriptor {}
impl ConstructForwardVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r5, Register::r2, Register::r4, Register::r2)
    }
}

pub struct ConstructWithSpreadDescriptor {}
impl ConstructWithSpreadDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r5, Register::r2, Register::r4, Register::r2)
    }
}

pub struct ConstructWithArrayLikeDescriptor {}
impl ConstructWithArrayLikeDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r5, Register::r4, Register::r2, Register::r2)
    }
}

pub struct ConstructStubDescriptor {}
impl ConstructStubDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r3, Register::r5, Register::r2, Register::r2, Register::r2)
    }
}

pub struct AbortDescriptor {}
impl AbortDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r3, Register::r2, Register::r2, Register::r2, Register::r2) }
}

pub struct CompareDescriptor {}
impl CompareDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r3, Register::r2, Register::r2, Register::r2, Register::r2) }
}

pub struct Compare_BaselineDescriptor {}
impl Compare_BaselineDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r3, Register::r2, Register::r4, Register::r2, Register::r2) }
}

pub struct BinaryOpDescriptor {}
impl BinaryOpDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r3, Register::r2, Register::r2, Register::r2, Register::r2) }
}

pub struct BinaryOp_BaselineDescriptor {}
impl BinaryOp_BaselineDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r3, Register::r2, Register::r4, Register::r2, Register::r2) }
}

pub struct BinarySmiOp_BaselineDescriptor {}
impl BinarySmiOp_BaselineDescriptor {
    pub const fn registers() -> RegisterArray { RegisterArray(Register::r2, Register::r3, Register::r4, Register::r2, Register::r2) }
}

pub struct CallApiCallbackOptimizedDescriptor {}
impl CallApiCallbackOptimizedDescriptor {
    pub const fn ApiFunctionAddressRegister() -> Register { Register::r3 }
    pub const fn ActualArgumentsCountRegister() -> Register { Register::r4 }
    pub const fn FunctionTemplateInfoRegister() -> Register { Register::r5 }
}

pub struct CallApiCallbackGenericDescriptor {}
impl CallApiCallbackGenericDescriptor {
    pub const fn ActualArgumentsCountRegister() -> Register { Register::r4 }
    pub const fn TopmostScriptHavingContextRegister() -> Register { Register::r3 }
    pub const fn FunctionTemplateInfoRegister() -> Register { Register::r5 }
}

pub struct InterpreterDispatchDescriptor {}
impl InterpreterDispatchDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(
            Register::kInterpreterAccumulatorRegister,
            Register::kInterpreterBytecodeOffsetRegister,
            Register::kInterpreterBytecodeArrayRegister,
            Register::kInterpreterDispatchTableRegister,
            Register::r2,
        )
    }
}

pub struct InterpreterPushArgsThenCallDescriptor {}
impl InterpreterPushArgsThenCallDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(
            Register::r2, // argument count
            Register::r4, // address of first argument
            Register::r3, // the target callable to be call
            Register::r2,
            Register::r2,
        )
    }
}

pub struct InterpreterPushArgsThenConstructDescriptor {}
impl InterpreterPushArgsThenConstructDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(
            Register::r2, // argument count
            Register::r6, // address of the first argument
            Register::r3, // constructor to call
            Register::r5, // new target
            Register::r4, // allocation site feedback if available, undefined otherwise
        )
    }
}

pub struct ConstructForwardAllArgsDescriptor {}
impl ConstructForwardAllArgsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(
            Register::r3, // constructor to call
            Register::r5, // new target
            Register::r2,
            Register::r2,
            Register::r2,
        )
    }
}

pub struct ResumeGeneratorDescriptor {}
impl ResumeGeneratorDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(
            Register::r2, // the value to pass to the generator
            Register::r3, // the JSGeneratorObject to resume
            Register::r2,
            Register::r2,
            Register::r2,
        )
    }
}

pub struct RunMicrotasksEntryDescriptor {}
impl RunMicrotasksEntryDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r2, Register::r3, Register::r2, Register::r2, Register::r2)
    }
}

pub struct WasmJSToWasmWrapperDescriptor {}
impl WasmJSToWasmWrapperDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray(Register::r8, Register::r2, Register::r2, Register::r2, Register::r2)
    }
}
