// Converted from V8 C++ source files:
// Header: interface-descriptors-arm64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
//use crate::codegen::interface_descriptors::*;
//use crate::execution::frames::*;
use std::marker::PhantomData;

use crate::base::template_utils::StaticAssert;
use crate::codegen::reglist::RegList;

pub struct CallInterfaceDescriptor {}

impl CallInterfaceDescriptor {
    pub fn DefaultRegisterArray() -> RegisterArray {
        let registers = RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 1 }),
            x2: Some(Register { code: 2 }),
            x3: Some(Register { code: 3 }),
            x4: Some(Register { code: 4 }),
            x5: None,
            x6: None,
            x7: None,
        };
        // Ensure the size is correct at compile time.
        let _ = StaticAssert::<{ 5 == kMaxBuiltinRegisterParams }>::ok();
        registers
    }

    pub fn DefaultDoubleRegisterArray() -> DoubleRegisterArray {
        DoubleRegisterArray {
            d0: Some(Register { code: 0 }),
            d1: Some(Register { code: 1 }),
            d2: Some(Register { code: 2 }),
            d3: Some(Register { code: 3 }),
            d4: Some(Register { code: 4 }),
            d5: Some(Register { code: 5 }),
            d6: Some(Register { code: 6 }),
            d7: None,
        }
    }

    pub fn DefaultReturnRegisterArray() -> RegisterArray {
        RegisterArray {
            x0: Some(kReturnRegister0),
            x1: Some(kReturnRegister1),
            x2: Some(kReturnRegister2),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }

    pub fn DefaultReturnDoubleRegisterArray() -> DoubleRegisterArray {
        DoubleRegisterArray {
            d0: Some(kFPReturnRegister0),
            d1: Some(no_dreg),
            d2: Some(no_dreg),
            d3: None,
            d4: None,
            d5: None,
            d6: None,
            d7: None,
        }
    }
}

const kMaxBuiltinRegisterParams: usize = 5;

#[derive(Clone, Copy)]
pub struct Register {
    code: i8,
}

#[derive(Clone, Copy)]
pub struct FPRegister {
    code: i8,
}

#[derive(Clone, Copy)]
pub struct RegisterArray {
    x0: Option<Register>,
    x1: Option<Register>,
    x2: Option<Register>,
    x3: Option<Register>,
    x4: Option<Register>,
    x5: Option<Register>,
    x6: Option<Register>,
    x7: Option<Register>,
}

#[derive(Clone, Copy)]
pub struct DoubleRegisterArray {
    d0: Option<Register>,
    d1: Option<Register>,
    d2: Option<Register>,
    d3: Option<Register>,
    d4: Option<Register>,
    d5: Option<Register>,
    d6: Option<Register>,
    d7: Option<Register>,
}

const kReturnRegister0: Register = Register { code: 0 };
const kReturnRegister1: Register = Register { code: 1 };
const kReturnRegister2: Register = Register { code: 2 };
const kFPReturnRegister0: Register = Register { code: 0 };
const no_dreg: Register = Register { code: -1 };

#[cfg(debug_assertions)]
pub struct StaticCallInterfaceDescriptor<DerivedDescriptor> {
    _phantom: PhantomData<DerivedDescriptor>,
}

#[cfg(debug_assertions)]
impl<DerivedDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    pub fn VerifyArgumentRegisterCount(data: &mut CallInterfaceDescriptorData, argc: i32) {
        let allocatable_regs = data.allocatable_registers();
        if argc >= 1 {
            assert!(allocatable_regs.has(Register { code: 0 }));
        }
        if argc >= 2 {
            assert!(allocatable_regs.has(Register { code: 1 }));
        }
        if argc >= 3 {
            assert!(allocatable_regs.has(Register { code: 2 }));
        }
        if argc >= 4 {
            assert!(allocatable_regs.has(Register { code: 3 }));
        }
        if argc >= 5 {
            assert!(allocatable_regs.has(Register { code: 4 }));
        }
        if argc >= 6 {
            assert!(allocatable_regs.has(Register { code: 5 }));
        }
        if argc >= 7 {
            assert!(allocatable_regs.has(Register { code: 6 }));
        }
        if argc >= 8 {
            assert!(allocatable_regs.has(Register { code: 7 }));
        }
    }
}

pub struct CallInterfaceDescriptorData {
    registers: [Register; 8],
}

impl CallInterfaceDescriptorData {
    pub fn allocatable_registers(&self) -> RegList {
        let mut reg_list = RegList {
            mask_: 0,
        };

        for reg in &self.registers {
            reg_list.set(*reg);
        }
        reg_list
    }
}

impl RegList {
    fn set(&mut self, reg: Register) {
        if reg.code >= 0 && reg.code < 64 {
            self.mask_ |= 1 << reg.code;
        }
    }

    fn has(&self, reg: Register) -> bool {
        if reg.code >= 0 && reg.code < 64 {
            (self.mask_ & (1 << reg.code)) != 0
        } else {
            false
        }
    }
}

pub struct WriteBarrierDescriptor {}

impl WriteBarrierDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 3 }),
            x1: Some(Register { code: 1 }),
            x2: Some(Register { code: 4 }),
            x3: Some(Register { code: 5 }),
            x4: Some(Register { code: 2 }),
            x5: Some(Register { code: 0 }),
            x6: Some(Register { code: 6 }),
            x7: Some(Register { code: 7 }),
        }
    }
}

pub struct LoadDescriptor {}

impl LoadDescriptor {
    pub const fn ReceiverRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn NameRegister() -> Register {
        Register { code: 2 }
    }
    pub const fn SlotRegister() -> Register {
        Register { code: 0 }
    }
}

pub struct LoadWithVectorDescriptor {}

impl LoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct KeyedLoadBaselineDescriptor {}

impl KeyedLoadBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn NameRegister() -> Register {
        kInterpreterAccumulatorRegister
    }
    pub const fn SlotRegister() -> Register {
        Register { code: 2 }
    }
}

pub struct KeyedLoadWithVectorDescriptor {}

impl KeyedLoadWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct EnumeratedKeyedLoadBaselineDescriptor {}

impl EnumeratedKeyedLoadBaselineDescriptor {
    pub const fn EnumIndexRegister() -> Register {
        Register { code: 4 }
    }
    pub const fn CacheTypeRegister() -> Register {
        Register { code: 5 }
    }
    pub const fn SlotRegister() -> Register {
        Register { code: 2 }
    }
}

pub struct KeyedHasICBaselineDescriptor {}

impl KeyedHasICBaselineDescriptor {
    pub const fn ReceiverRegister() -> Register {
        kInterpreterAccumulatorRegister
    }
    pub const fn NameRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn SlotRegister() -> Register {
        Register { code: 2 }
    }
}

pub struct KeyedHasICWithVectorDescriptor {}

impl KeyedHasICWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct LoadWithReceiverAndVectorDescriptor {}

impl LoadWithReceiverAndVectorDescriptor {
    pub const fn LookupStartObjectRegister() -> Register {
        Register { code: 4 }
    }
}

pub struct StoreDescriptor {}

impl StoreDescriptor {
    pub const fn ReceiverRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn NameRegister() -> Register {
        Register { code: 2 }
    }
    pub const fn ValueRegister() -> Register {
        Register { code: 0 }
    }
    pub const fn SlotRegister() -> Register {
        Register { code: 4 }
    }
}

pub struct StoreWithVectorDescriptor {}

impl StoreWithVectorDescriptor {
    pub const fn VectorRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct DefineKeyedOwnDescriptor {}

impl DefineKeyedOwnDescriptor {
    pub const fn FlagsRegister() -> Register {
        Register { code: 5 }
    }
}

pub struct StoreTransitionDescriptor {}

impl StoreTransitionDescriptor {
    pub const fn MapRegister() -> Register {
        Register { code: 5 }
    }
}

pub struct ApiGetterDescriptor {}

impl ApiGetterDescriptor {
    pub const fn HolderRegister() -> Register {
        Register { code: 0 }
    }
    pub const fn CallbackRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct GrowArrayElementsDescriptor {}

impl GrowArrayElementsDescriptor {
    pub const fn ObjectRegister() -> Register {
        Register { code: 0 }
    }
    pub const fn KeyRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct BaselineLeaveFrameDescriptor {}

impl BaselineLeaveFrameDescriptor {
    pub const fn ParamsSizeRegister() -> Register {
        Register { code: 3 }
    }
    pub const fn WeightRegister() -> Register {
        Register { code: 4 }
    }
}

pub struct TypeConversionDescriptor {}

impl TypeConversionDescriptor {
    pub const fn ArgumentRegister() -> Register {
        Register { code: 0 }
    }
}

pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {}

impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
    pub const fn FlagsRegister() -> Register {
        Register { code: 8 }
    }
    pub const fn FeedbackVectorRegister() -> Register {
        Register { code: 9 }
    }
    pub const fn TemporaryRegister() -> Register {
        Register { code: 5 }
    }
}

pub struct TypeofDescriptor {}

impl TypeofDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: None,
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallTrampolineDescriptor {}

impl CallTrampolineDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}

impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}

impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallVarargsDescriptor {}

impl CallVarargsDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 3 }),
            x3: Some(Register { code: 4 }),
            x4: Some(Register { code: 2 }),
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallForwardVarargsDescriptor {}

impl CallForwardVarargsDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallFunctionTemplateDescriptor {}

impl CallFunctionTemplateDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 2 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallFunctionTemplateGenericDescriptor {}

impl CallFunctionTemplateGenericDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 2 }),
            x2: Some(Register { code: 3 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallWithSpreadDescriptor {}

impl CallWithSpreadDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallWithArrayLikeDescriptor {}

impl CallWithArrayLikeDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 2 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructVarargsDescriptor {}

impl ConstructVarargsDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: Some(Register { code: 0 }),
            x3: Some(Register { code: 4 }),
            x4: Some(Register { code: 2 }),
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructForwardVarargsDescriptor {}

impl ConstructForwardVarargsDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: Some(Register { code: 0 }),
            x3: Some(Register { code: 2 }),
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructWithSpreadDescriptor {}

impl ConstructWithSpreadDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: Some(Register { code: 0 }),
            x3: Some(Register { code: 2 }),
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructWithArrayLikeDescriptor {}

impl ConstructWithArrayLikeDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructStubDescriptor {}

impl ConstructStubDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: Some(Register { code: 0 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct AbortDescriptor {}

impl AbortDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: None,
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CompareDescriptor {}

impl CompareDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct Compare_BaselineDescriptor {}

impl Compare_BaselineDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct BinaryOpDescriptor {}

impl BinaryOpDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct BinaryOp_BaselineDescriptor {}

impl BinaryOp_BaselineDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 0 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct BinarySmiOp_BaselineDescriptor {}

impl BinarySmiOp_BaselineDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 1 }),
            x2: Some(Register { code: 2 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct CallApiCallbackOptimizedDescriptor {}

impl CallApiCallbackOptimizedDescriptor {
    pub const fn ApiFunctionAddressRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn ActualArgumentsCountRegister() -> Register {
        Register { code: 2 }
    }
    pub const fn FunctionTemplateInfoRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct CallApiCallbackGenericDescriptor {}

impl CallApiCallbackGenericDescriptor {
    pub const fn ActualArgumentsCountRegister() -> Register {
        Register { code: 2 }
    }
    pub const fn TopmostScriptHavingContextRegister() -> Register {
        Register { code: 1 }
    }
    pub const fn FunctionTemplateInfoRegister() -> Register {
        Register { code: 3 }
    }
}

pub struct InterpreterDispatchDescriptor {}

impl InterpreterDispatchDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(kInterpreterAccumulatorRegister),
            x1: Some(kInterpreterBytecodeOffsetRegister),
            x2: Some(kInterpreterBytecodeArrayRegister),
            x3: Some(kInterpreterDispatchTableRegister),
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct InterpreterPushArgsThenCallDescriptor {}

impl InterpreterPushArgsThenCallDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 2 }),
            x2: Some(Register { code: 1 }),
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct InterpreterPushArgsThenConstructDescriptor {}

impl InterpreterPushArgsThenConstructDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 4 }),
            x2: Some(Register { code: 1 }),
            x3: Some(Register { code: 3 }),
            x4: Some(Register { code: 2 }),
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ConstructForwardAllArgsDescriptor {}

impl ConstructForwardAllArgsDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 1 }),
            x1: Some(Register { code: 3 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct ResumeGeneratorDescriptor {}

impl ResumeGeneratorDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 1 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct RunMicrotasksEntryDescriptor {}

impl RunMicrotasksEntryDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 0 }),
            x1: Some(Register { code: 1 }),
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

pub struct WasmJSToWasmWrapperDescriptor {}

impl WasmJSToWasmWrapperDescriptor {
    pub fn registers() -> RegisterArray {
        RegisterArray {
            x0: Some(Register { code: 8 }),
            x1: None,
            x2: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
        }
    }
}

const kInterpreterAccumulatorRegister: Register = Register { code: 100 };
const kInterpreterBytecodeOffsetRegister: Register = Register { code: 101 };
const kInterpreterBytecodeArrayRegister: Register = Register { code: 102 };
const kInterpreterDispatchTableRegister: Register = Register { code: 103 };
