// Converted from V8 C++ source files:
// Header: interface-descriptors-ppc-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interface_descriptors_ppc_inl {
use crate::execution::frames::Context;
use crate::v8::internal::Builtin;
use crate::v8::internal::ConvertReceiverMode;
use crate::codegen::ppc::register_ppc::Register;
use crate::codegen::ppc::reglist_ppc::RegList;

pub struct CallInterfaceDescriptorData {}
pub struct DoubleRegisterArray(pub Register, pub Register, pub Register, pub Register, pub Register, pub Register, pub Register);
pub struct RegisterArray(pub Register, pub Register, pub Register, pub Register, pub Register);
impl RegisterArray {
    pub fn size(&self) -> usize {
        5 // Assuming 5 registers based on the struct definition
    }
}
pub struct FPUControlRegister {}
pub struct OpIndex {}
pub struct InstructionOperand {}
pub struct Tagged<T>(T);
pub struct Node {}
pub struct RegExpNodeInfo {}
pub struct JsonObject {}
pub struct Expression {}

    const kMaxBuiltinRegisterParams: usize = 5;
    const kReturnRegister0: Register = Register { code: 3 };
    const kReturnRegister1: Register = Register { code: 4 };
    const kReturnRegister2: Register = Register { code: 5 };
    const kFPReturnRegister0: Register = Register { code: 6 };
    const r3: Register = Register { code: 7 };
    const r4: Register = Register { code: 8 };
    const r5: Register = Register { code: 9 };
    const r6: Register = Register { code: 10 };
    const r7: Register = Register { code: 11 };
    const r8: Register = Register { code: 12 };
    const r9: Register = Register { code: 13 };
    const r10: Register = Register { code: 14 };
    const r14: Register = Register { code: 15 };
    const d1: Register = Register { code: 16 };
    const d2: Register = Register { code: 17 };
    const d3: Register = Register { code: 18 };
    const d4: Register = Register { code: 19 };
    const d5: Register = Register { code: 20 };
    const d6: Register = Register { code: 21 };
    const d7: Register = Register { code: 22 };
    const kContextRegister: Register = Register { code: 23 };
    const kInterpreterAccumulatorRegister: Register = Register { code: 24 };
    const kInterpreterBytecodeOffsetRegister: Register = Register { code: 25 };
    const kInterpreterBytecodeArrayRegister: Register = Register { code: 26 };
    const kInterpreterDispatchTableRegister: Register = Register { code: 27 };
    const no_dreg: Register = Register {code: 0};

    impl DoubleRegisterArray {
        pub fn new(r0: Register, r1: Register, r2: Register, r3: Register, r4: Register, r5: Register, r6: Register) -> Self {
            DoubleRegisterArray(r0, r1, r2, r3, r4, r5, r6)
        }
    }

    impl RegisterArray {
        pub fn new(r0: Register, r1: Register, r2: Register, r3: Register, r4: Register) -> Self {
            RegisterArray(r0, r1, r2, r3, r4)
        }
    }

    pub struct CallInterfaceDescriptor {}

    impl CallInterfaceDescriptor {
        pub const fn DefaultRegisterArray() -> RegisterArray {
            let registers = RegisterArray::new(r3, r4, r5, r6, r7);
            assert_eq!(registers.size(), kMaxBuiltinRegisterParams);
            registers
        }

        pub const fn DefaultDoubleRegisterArray() -> DoubleRegisterArray {
            let registers = DoubleRegisterArray::new(d1, d2, d3, d4, d5, d6, d7);
            registers
        }

        pub const fn DefaultReturnRegisterArray() -> RegisterArray {
            let registers = RegisterArray::new(kReturnRegister0, kReturnRegister1, kReturnRegister2);
            registers
        }

        pub const fn DefaultReturnDoubleRegisterArray() -> DoubleRegisterArray {
            let registers = DoubleRegisterArray::new(kFPReturnRegister0, no_dreg, no_dreg, no_dreg, no_dreg, no_dreg, no_dreg);
            registers
        }
    }

    pub struct StaticCallInterfaceDescriptor<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> StaticCallInterfaceDescriptor<T> {
        #[cfg(debug_assertions)]
        pub fn VerifyArgumentRegisterCount(data: &CallInterfaceDescriptorData, argc: i32) {
            let allocatable_regs = RegList { code: 0 };//data.allocatable_registers(); //cannot access data in rust
            if argc >= 1 {
                assert!(allocatable_regs.has(r3));
            }
            if argc >= 2 {
                assert!(allocatable_regs.has(r4));
            }
            if argc >= 3 {
                assert!(allocatable_regs.has(r5));
            }
            if argc >= 4 {
                assert!(allocatable_regs.has(r6));
            }
            if argc >= 5 {
                assert!(allocatable_regs.has(r7));
            }
            if argc >= 6 {
                assert!(allocatable_regs.has(r8));
            }
            if argc >= 7 {
                assert!(allocatable_regs.has(r9));
            }
            if argc >= 8 {
                assert!(allocatable_regs.has(r10));
            }
        }
    }

    pub struct WriteBarrierDescriptor {}

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r8, r7, r5, r3, r6, kContextRegister)
        }
    }

    pub struct LoadDescriptor {}

    impl LoadDescriptor {
        pub const fn ReceiverRegister() -> Register {
            r4
        }
        pub const fn NameRegister() -> Register {
            r5
        }
        pub const fn SlotRegister() -> Register {
            r3
        }
    }

    pub struct LoadWithVectorDescriptor {}

    impl LoadWithVectorDescriptor {
        pub const fn VectorRegister() -> Register {
            r6
        }
    }

    pub struct KeyedLoadBaselineDescriptor {}

    impl KeyedLoadBaselineDescriptor {
        pub const fn ReceiverRegister() -> Register {
            r4
        }
        pub const fn NameRegister() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn SlotRegister() -> Register {
            r5
        }
    }

    pub struct KeyedLoadWithVectorDescriptor {}

    impl KeyedLoadWithVectorDescriptor {
        pub const fn VectorRegister() -> Register {
            r6
        }
    }

    pub struct EnumeratedKeyedLoadBaselineDescriptor {}

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn EnumIndexRegister() -> Register {
            r7
        }

        pub const fn CacheTypeRegister() -> Register {
            r8
        }

        pub const fn SlotRegister() -> Register {
            r5
        }
    }

    pub struct KeyedHasICBaselineDescriptor {}

    impl KeyedHasICBaselineDescriptor {
        pub const fn ReceiverRegister() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn NameRegister() -> Register {
            r4
        }
        pub const fn SlotRegister() -> Register {
            r5
        }
    }

    pub struct KeyedHasICWithVectorDescriptor {}

    impl KeyedHasICWithVectorDescriptor {
        pub const fn VectorRegister() -> Register {
            r6
        }
    }

    pub struct LoadWithReceiverAndVectorDescriptor {}

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn LookupStartObjectRegister() -> Register {
            r7
        }
    }

    pub struct StoreDescriptor {}

    impl StoreDescriptor {
        pub const fn ReceiverRegister() -> Register {
            r4
        }
        pub const fn NameRegister() -> Register {
            r5
        }
        pub const fn ValueRegister() -> Register {
            r3
        }
        pub const fn SlotRegister() -> Register {
            r7
        }
    }

    pub struct StoreWithVectorDescriptor {}

    impl StoreWithVectorDescriptor {
        pub const fn VectorRegister() -> Register {
            r6
        }
    }

    pub struct DefineKeyedOwnDescriptor {}

    impl DefineKeyedOwnDescriptor {
        pub const fn FlagsRegister() -> Register {
            r8
        }
    }

    pub struct StoreTransitionDescriptor {}

    impl StoreTransitionDescriptor {
        pub const fn MapRegister() -> Register {
            r8
        }
    }

    pub struct ApiGetterDescriptor {}

    impl ApiGetterDescriptor {
        pub const fn HolderRegister() -> Register {
            r3
        }
        pub const fn CallbackRegister() -> Register {
            r6
        }
    }

    pub struct GrowArrayElementsDescriptor {}

    impl GrowArrayElementsDescriptor {
        pub const fn ObjectRegister() -> Register {
            r3
        }
        pub const fn KeyRegister() -> Register {
            r6
        }
    }

    pub struct BaselineLeaveFrameDescriptor {}

    impl BaselineLeaveFrameDescriptor {
        pub const fn ParamsSizeRegister() -> Register {
            r6
        }
        pub const fn WeightRegister() -> Register {
            r7
        }
    }

    pub struct TypeConversionDescriptor {}

    impl TypeConversionDescriptor {
        pub const fn ArgumentRegister() -> Register {
            r3
        }
    }

    pub struct TypeofDescriptor {}

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r3, r3, r3, r3) //Dummy regs
        }
    }

    pub struct CallTrampolineDescriptor {}

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r3, r3, r3) //Dummy regs
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r3, r3, r3) //Dummy regs
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r5, r3, r3) //Dummy regs
        }
    }

    pub struct CallVarargsDescriptor {}

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r7, r5, r3) //Dummy regs
        }
    }

    pub struct CallForwardVarargsDescriptor {}

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r5, r3, r3) //Dummy regs
        }
    }

    pub struct CallFunctionTemplateDescriptor {}

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r5, r3, r3, r3) //Dummy regs
        }
    }

    pub struct CallFunctionTemplateGenericDescriptor {}

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r5, r6, r3, r3) //Dummy regs
        }
    }

    pub struct CallWithSpreadDescriptor {}

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r5, r3, r3) //Dummy regs
        }
    }

    pub struct CallWithArrayLikeDescriptor {}

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r5, r3, r3, r3) //Dummy regs
        }
    }

    pub struct ConstructVarargsDescriptor {}

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r3, r7, r5) //Dummy regs
        }
    }

    pub struct ConstructForwardVarargsDescriptor {}

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r3, r5, r3) //Dummy regs
        }
    }

    pub struct ConstructWithSpreadDescriptor {}

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r3, r5, r3) //Dummy regs
        }
    }

    pub struct ConstructWithArrayLikeDescriptor {}

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r5, r3, r3) //Dummy regs
        }
    }

    pub struct ConstructStubDescriptor {}

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r3, r3, r3) //Dummy regs
        }
    }

    pub struct AbortDescriptor {}

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r4, r4, r4, r4) //Dummy regs
        }
    }

    pub struct CompareDescriptor {}

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r4, r4, r4) //Dummy regs
        }
    }

    pub struct Compare_BaselineDescriptor {}

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r5, r4, r4) //Dummy regs
        }
    }

    pub struct BinaryOpDescriptor {}

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r4, r4, r4) //Dummy regs
        }
    }

    pub struct BinaryOp_BaselineDescriptor {}

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r3, r5, r4, r4) //Dummy regs
        }
    }

    pub struct BinarySmiOp_BaselineDescriptor {}

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r4, r5, r4, r4) //Dummy regs
        }
    }

    pub struct CallApiCallbackOptimizedDescriptor {}

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn ApiFunctionAddressRegister() -> Register {
            r4
        }
        pub const fn ActualArgumentsCountRegister() -> Register {
            r5
        }
        pub const fn FunctionTemplateInfoRegister() -> Register {
            r6
        }
    }

    pub struct CallApiCallbackGenericDescriptor {}

    impl CallApiCallbackGenericDescriptor {
        pub const fn ActualArgumentsCountRegister() -> Register {
            r5
        }
        pub const fn TopmostScriptHavingContextRegister() -> Register {
            r4
        }
        pub const fn FunctionTemplateInfoRegister() -> Register {
            r6
        }
    }

    pub struct InterpreterDispatchDescriptor {}

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(
                kInterpreterAccumulatorRegister,
                kInterpreterBytecodeOffsetRegister,
                kInterpreterBytecodeArrayRegister,
                kInterpreterDispatchTableRegister,
                kInterpreterDispatchTableRegister,
            )
        }
    }

    pub struct InterpreterPushArgsThenCallDescriptor {}

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r5, r4, r3, r3) //Dummy regs
        }
    }

    pub struct InterpreterPushArgsThenConstructDescriptor {}

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r7, r4, r6, r5) //Dummy regs
        }
    }

    pub struct ConstructForwardAllArgsDescriptor {}

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r4, r6, r4, r4, r4) //Dummy regs
        }
    }

    pub struct ResumeGeneratorDescriptor {}

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r4, r4, r4, r4) //Dummy regs
        }
    }

    pub struct RunMicrotasksEntryDescriptor {}

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r3, r4, r4, r4, r4) //Dummy regs
        }
    }

    pub struct WasmJSToWasmWrapperDescriptor {}

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray::new(r14,r14,r14,r14,r14)
        }
    }

    impl RegList {
        pub fn has(&self, reg: Register) -> bool {
            self.code() & (1 << reg.code) != 0
        }
    }
}
