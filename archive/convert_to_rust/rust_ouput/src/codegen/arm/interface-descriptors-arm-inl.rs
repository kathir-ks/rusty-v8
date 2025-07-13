// Converted from V8 C++ source files:
// Header: interface-descriptors-arm-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::array;

//use crate::codegen::interface_descriptors::*;
//use crate::execution::frames::*;

//use crate::codegen::arm::register::*; // Assuming register definitions are in this file
//use crate::codegen::interface_descriptors::*;

// Placeholder structs/enums - replace with actual definitions
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Register {
    pub code: u32,
}

impl Register {
    pub fn from_code(code: u32) -> Self {
        Register { code }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DoubleRegister {
    pub code: u32,
}

impl DoubleRegister {
    pub fn from_code(code: u32) -> Self {
        DoubleRegister { code }
    }
}

pub struct RegisterArray(pub [Register; 5]);
pub struct DoubleRegisterArray(pub [DoubleRegister; 7]);

pub const kMaxBuiltinRegisterParams: usize = 5;

// Placeholder register constants - replace with actual values
pub const r0: Register = Register { code: 0 };
pub const r1: Register = Register { code: 1 };
pub const r2: Register = Register { code: 2 };
pub const r3: Register = Register { code: 3 };
pub const r4: Register = Register { code: 4 };
pub const r5: Register = Register { code: 5 };
pub const r6: Register = Register { code: 6 };
pub const r7: Register = Register { code: 7 };

pub const d0: DoubleRegister = DoubleRegister { code: 0 };
pub const d1: DoubleRegister = DoubleRegister { code: 1 };
pub const d2: DoubleRegister = DoubleRegister { code: 2 };
pub const d3: DoubleRegister = DoubleRegister { code: 3 };
pub const d4: DoubleRegister = DoubleRegister { code: 4 };
pub const d5: DoubleRegister = DoubleRegister { code: 5 };
pub const d6: DoubleRegister = DoubleRegister { code: 6 };

pub const kReturnRegister0: Register = Register { code: 0 };
pub const kReturnRegister1: Register = Register { code: 1 };
pub const kReturnRegister2: Register = Register { code: 2 };
pub const kFPReturnRegister0: DoubleRegister = DoubleRegister { code: 0 };
pub const kContextRegister: Register = Register{code: 15};
pub const kInterpreterAccumulatorRegister: Register = Register{code: 10};
pub const kInterpreterBytecodeOffsetRegister: Register = Register{code: 11};
pub const kInterpreterBytecodeArrayRegister: Register = Register{code: 12};
pub const kInterpreterDispatchTableRegister: Register = Register{code: 13};

pub const no_dreg: DoubleRegister = DoubleRegister { code: 99 }; // some invalid value

pub struct CallInterfaceDescriptor {}

impl CallInterfaceDescriptor {
    pub const fn DefaultRegisterArray() -> RegisterArray {
        RegisterArray([r0, r1, r2, r3, r4])
    }

    pub const fn DefaultDoubleRegisterArray() -> array::[DoubleRegister; 7] {
        [d0, d1, d2, d3, d4, d5, d6]
    }

    pub const fn DefaultReturnRegisterArray() -> RegisterArray {
        RegisterArray([kReturnRegister0, kReturnRegister1, kReturnRegister2])
    }

    pub const fn DefaultReturnDoubleRegisterArray() -> array::[DoubleRegister; 3] {
        [kFPReturnRegister0, no_dreg, no_dreg]
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RegList {
    pub mask: u32,
}

impl RegList {
    pub fn has(&self, reg: Register) -> bool {
        (self.mask & (1 << reg.code)) != 0
    }
}

pub struct CallInterfaceDescriptorData {
    allocatable_registers_: RegList
}

impl CallInterfaceDescriptorData {
    pub fn allocatable_registers(&self) -> RegList {
        self.allocatable_registers_
    }
}

pub struct StaticCallInterfaceDescriptor<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<DerivedDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    #[cfg(debug_assertions)]
    pub fn VerifyArgumentRegisterCount(data: &mut CallInterfaceDescriptorData, argc: i32) {
        let allocatable_regs = data.allocatable_registers();
        if argc >= 1 {
            assert!(allocatable_regs.has(r0));
        }
        if argc >= 2 {
            assert!(allocatable_regs.has(r1));
        }
        if argc >= 3 {
            assert!(allocatable_regs.has(r2));
        }
        if argc >= 4 {
            assert!(allocatable_regs.has(r3));
        }
        if argc >= 5 {
            assert!(allocatable_regs.has(r4));
        }
        if argc >= 6 {
            assert!(allocatable_regs.has(r5));
        }
        if argc >= 7 {
            assert!(allocatable_regs.has(r6));
        }
        if argc >= 8 {
            assert!(allocatable_regs.has(r7));
        }
        // Additional arguments are passed on the stack.
    }
}

pub struct WriteBarrierDescriptor {}

impl WriteBarrierDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r1, r5, r4, r2, r0, r3, kContextRegister])
    }
}

pub struct LoadDescriptor {}

impl LoadDescriptor {
    pub const fn ReceiverRegister() -> Register {
        r1
    }
    pub const fn NameRegister() -> Register {
        r2
    }
    pub const fn SlotRegister() -> Register {
        r0
    }
}

pub struct LoadWithVectorDescriptor {}

impl LoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        r3
    }
}

pub struct KeyedLoadBaselineDescriptor {}

impl KeyedLoadBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register {
        r1
    }
    pub const fn NameRegister() -> Register {
        kInterpreterAccumulatorRegister
    }
    pub const fn SlotRegister() -> Register {
        r2
    }
}

pub struct KeyedLoadWithVectorDescriptor {}

impl KeyedLoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        r3
    }
}

pub struct EnumeratedKeyedLoadBaselineDescriptor {}

impl EnumeratedKeyedLoadBaselineDescriptor {
    pub const fn EnumIndexRegister() -> Register {
        r4
    }
    pub const fn CacheTypeRegister() -> Register {
        r5
    }
    pub const fn SlotRegister() -> Register {
        r2
    }
}

pub struct KeyedHasICBaselineDescriptor {}

impl KeyedHasICBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register {
        kInterpreterAccumulatorRegister
    }
    pub const fn NameRegister() -> Register {
        r1
    }
    pub const fn SlotRegister() -> Register {
        r2
    }
}

pub struct KeyedHasICWithVectorDescriptor {}

impl KeyedHasICWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        r3
    }
}

pub struct LoadWithReceiverAndVectorDescriptor {}

impl LoadWithReceiverAndVectorDescriptor {
    pub const fn LookupStartObjectRegister() -> Register {
        r4
    }
}

pub struct StoreDescriptor {}

impl StoreDescriptor {
    pub const fn ReceiverRegister() -> Register {
        r1
    }
    pub const fn NameRegister() -> Register {
        r2
    }
    pub const fn ValueRegister() -> Register {
        r0
    }
    pub const fn SlotRegister() -> Register {
        r4
    }
}

pub struct StoreWithVectorDescriptor {}

impl StoreWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        r3
    }
}

pub struct DefineKeyedOwnDescriptor {}

impl DefineKeyedOwnDescriptor {
    pub const fn FlagsRegister() -> Register {
        r5
    }
}

pub struct StoreTransitionDescriptor {}

impl StoreTransitionDescriptor {
    pub const fn MapRegister() -> Register {
        r5
    }
}

pub struct ApiGetterDescriptor {}

impl ApiGetterDescriptor {
    pub const fn HolderRegister() -> Register {
        r0
    }
    pub const fn CallbackRegister() -> Register {
        r3
    }
}

pub struct GrowArrayElementsDescriptor {}

impl GrowArrayElementsDescriptor {
    pub const fn ObjectRegister() -> Register {
        r0
    }
    pub const fn KeyRegister() -> Register {
        r3
    }
}

pub struct BaselineLeaveFrameDescriptor {}

impl BaselineLeaveFrameDescriptor {
    pub const fn ParamsSizeRegister() -> Register {
        r3
    }
    pub const fn WeightRegister() -> Register {
        r4
    }
}

pub struct TypeConversionDescriptor {}

impl TypeConversionDescriptor {
    pub const fn ArgumentRegister() -> Register {
        r0
    }
}

pub struct TypeofDescriptor {}

impl TypeofDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r0])
    }
}

pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {}

impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
    pub const fn FlagsRegister() -> Register {
        r2
    }
    pub const fn FeedbackVectorRegister() -> Register {
        r5
    }
    pub const fn TemporaryRegister() -> Register {
        r4
    }
}

pub struct CallTrampolineDescriptor {}

impl CallTrampolineDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments
        // r1 : the target to call
        RegisterArray([r1, r0])
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}

impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : the source
        // r1 : the excluded property count
        RegisterArray([r1, r0])
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}

impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : the source
        // r1 : the excluded property count
        // r2 : the excluded property base
        RegisterArray([r1, r0, r2])
    }
}

pub struct CallVarargsDescriptor {}

impl CallVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments (on the stack)
        // r1 : the target to call
        // r4 : arguments list length (untagged)
        // r2 : arguments list (FixedArray)
        RegisterArray([r1, r0, r4, r2])
    }
}

pub struct CallForwardVarargsDescriptor {}

impl CallForwardVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments
        // r2 : start index (to support rest parameters)
        // r1 : the target to call
        RegisterArray([r1, r0, r2])
    }
}

pub struct CallFunctionTemplateDescriptor {}

impl CallFunctionTemplateDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1 : function template info
        // r2 : number of arguments (on the stack)
        RegisterArray([r1, r2])
    }
}

pub struct CallFunctionTemplateGenericDescriptor {}

impl CallFunctionTemplateGenericDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1 : function template info
        // r2 : number of arguments (on the stack)
        // r3 : topmost script-having context
        RegisterArray([r1, r2, r3])
    }
}

pub struct CallWithSpreadDescriptor {}

impl CallWithSpreadDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments (on the stack)
        // r1 : the target to call
        // r2 : the object to spread
        RegisterArray([r1, r0, r2])
    }
}

pub struct CallWithArrayLikeDescriptor {}

impl CallWithArrayLikeDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1 : the target to call
        // r2 : the arguments list
        RegisterArray([r1, r2])
    }
}

pub struct ConstructVarargsDescriptor {}

impl ConstructVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments (on the stack)
        // r1 : the target to call
        // r3 : the new target
        // r4 : arguments list length (untagged)
        // r2 : arguments list (FixedArray)
        RegisterArray([r1, r3, r0, r4, r2])
    }
}

pub struct ConstructForwardVarargsDescriptor {}

impl ConstructForwardVarargsDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments
        // r3 : the new target
        // r2 : start index (to support rest parameters)
        // r1 : the target to call
        RegisterArray([r1, r3, r0, r2])
    }
}

pub struct ConstructWithSpreadDescriptor {}

impl ConstructWithSpreadDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments (on the stack)
        // r1 : the target to call
        // r3 : the new target
        // r2 : the object to spread
        RegisterArray([r1, r3, r0, r2])
    }
}

pub struct ConstructWithArrayLikeDescriptor {}

impl ConstructWithArrayLikeDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1 : the target to call
        // r3 : the new target
        // r2 : the arguments list
        RegisterArray([r1, r3, r2])
    }
}

pub struct ConstructStubDescriptor {}

impl ConstructStubDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0 : number of arguments
        // r1 : the target to call
        // r3 : the new target
        RegisterArray([r1, r3, r0])
    }
}

pub struct AbortDescriptor {}

impl AbortDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r1])
    }
}

pub struct CompareDescriptor {}

impl CompareDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r1, r0])
    }
}

pub struct Compare_BaselineDescriptor {}

impl Compare_BaselineDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1: left operand
        // r0: right operand
        // r2: feedback slot
        RegisterArray([r1, r0, r2])
    }
}

pub struct BinaryOpDescriptor {}

impl BinaryOpDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r1, r0])
    }
}

pub struct BinaryOp_BaselineDescriptor {}

impl BinaryOp_BaselineDescriptor {
    pub const fn registers() -> RegisterArray {
        // r1: left operand
        // r0: right operand
        // r2: feedback slot
        RegisterArray([r1, r0, r2])
    }
}

pub struct BinarySmiOp_BaselineDescriptor {}

impl BinarySmiOp_BaselineDescriptor {
    pub const fn registers() -> RegisterArray {
        // r0: left operand
        // r1: right operand
        // r2: feedback slot
        RegisterArray([r0, r1, r2])
    }
}

pub struct CallApiCallbackOptimizedDescriptor {}

impl CallApiCallbackOptimizedDescriptor {
    pub const fn ApiFunctionAddressRegister() -> Register {
        r1
    }
    pub const fn ActualArgumentsCountRegister() -> Register {
        r2
    }
    pub const fn FunctionTemplateInfoRegister() -> Register {
        r3
    }
}

pub struct CallApiCallbackGenericDescriptor {}

impl CallApiCallbackGenericDescriptor {
    pub const fn ActualArgumentsCountRegister() -> Register {
        r2
    }
    pub const fn TopmostScriptHavingContextRegister() -> Register {
        r1
    }
    pub const fn FunctionTemplateInfoRegister() -> Register {
        r3
    }
}

pub struct InterpreterDispatchDescriptor {}

impl InterpreterDispatchDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([
            kInterpreterAccumulatorRegister,
            kInterpreterBytecodeOffsetRegister,
            kInterpreterBytecodeArrayRegister,
            kInterpreterDispatchTableRegister,
        ])
    }
}

pub struct InterpreterPushArgsThenCallDescriptor {}

impl InterpreterPushArgsThenCallDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([
            r0,   // argument count
            r2,   // address of first argument
            r1,   // the target callable to be call
        ])
    }
}

pub struct InterpreterPushArgsThenConstructDescriptor {}

impl InterpreterPushArgsThenConstructDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([
            r0,   // argument count
            r4,   // address of the first argument
            r1,   // constructor to call
            r3,   // new target
            r2,   // allocation site feedback if available, undefined otherwise
        ])
    }
}

pub struct ConstructForwardAllArgsDescriptor {}

impl ConstructForwardAllArgsDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([
            r1,   // constructor to call
            r3,   // new target
        ])
    }
}

pub struct ResumeGeneratorDescriptor {}

impl ResumeGeneratorDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([
            r0,   // the value to pass to the generator
            r1,   // the JSGeneratorObject to resume
        ])
    }
}

pub struct RunMicrotasksEntryDescriptor {}

impl RunMicrotasksEntryDescriptor {
    pub const fn registers() -> RegisterArray {
        RegisterArray([r0, r1])
    }
}

pub struct WasmJSToWasmWrapperDescriptor {}

impl WasmJSToWasmWrapperDescriptor {
    pub const fn registers() -> RegisterArray {
        // Arbitrarily picked register.
        RegisterArray([r8])
    }
}
