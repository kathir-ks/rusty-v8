// Converted from V8 C++ source files:
// Header: interface-descriptors-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interface_descriptors_inl {
use std::cmp::min;
use std::mem;

use crate::base::logging::DCHECK_EQ;
use crate::codegen::interface_descriptors::*;
use crate::codegen::register::*;

#[cfg(v8_enable_webassembly)]
use crate::wasm::wasm_linkage::*;

#[cfg(v8_target_arch_x64)]
use crate::codegen::x64::interface_descriptors_x64_inl::*;
#[cfg(v8_target_arch_arm64)]
use crate::codegen::arm64::interface_descriptors_arm64_inl::*;
#[cfg(v8_target_arch_ia32)]
use crate::codegen::ia32::interface_descriptors_ia32_inl::*;
#[cfg(v8_target_arch_arm)]
use crate::codegen::arm::interface_descriptors_arm_inl::*;
#[cfg(v8_target_arch_ppc64)]
use crate::codegen::ppc::interface_descriptors_ppc_inl::*;
#[cfg(v8_target_arch_s390x)]
use crate::codegen::s390::interface_descriptors_s390_inl::*;
#[cfg(v8_target_arch_mips64)]
use crate::codegen::mips64::interface_descriptors_mips64_inl::*;
#[cfg(v8_target_arch_loong64)]
use crate::codegen::loong64::interface_descriptors_loong64_inl::*;
#[cfg(any(v8_target_arch_riscv32, v8_target_arch_riscv64))]
use crate::codegen::riscv::interface_descriptors_riscv_inl::*;

impl CallInterfaceDescriptor {
    pub const fn default_js_register_array() -> [Register; kJSBuiltinRegisterParams] {
        register_array(
            kJavaScriptCallTargetRegister,
            kJavaScriptCallNewTargetRegister,
            kJavaScriptCallArgCountRegister,
            kJavaScriptCallExtraArg1Register,
        )
    }
}

impl<DerivedDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    pub const fn registers() -> [Register; kMaxBuiltinRegisterParams] {
        CallInterfaceDescriptor::default_register_array()
    }

    pub const fn double_registers() -> [DoubleRegister; kMaxBuiltinRegisterParams] {
        CallInterfaceDescriptor::default_double_register_array()
    }

    pub const fn return_registers() -> [Register; kMaxReturns] {
        CallInterfaceDescriptor::default_return_register_array()
    }

    pub const fn return_double_registers() -> [DoubleRegister; kMaxReturns] {
        CallInterfaceDescriptor::default_return_double_register_array()
    }
}

impl<DerivedDescriptor> StaticJSCallInterfaceDescriptor<DerivedDescriptor> {
    pub const fn registers() -> [Register; kJSBuiltinRegisterParams] {
        CallInterfaceDescriptor::default_js_register_array()
    }
}

impl JSTrampolineDescriptor {
    pub const fn registers() -> [Register; kJSBuiltinRegisterParams] {
        register_array(
            kJavaScriptCallTargetRegister,
            kJavaScriptCallNewTargetRegister,
            kJavaScriptCallArgCountRegister,
            kJavaScriptCallDispatchHandleRegister,
        )
    }
}

impl CompareNoContextDescriptor {
    pub const fn registers() -> [Register; kMaxBuiltinRegisterParams] {
        CompareDescriptor::registers()
    }
}

impl<DerivedDescriptor: StaticInterfaceDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    pub fn initialize(data: &mut CallInterfaceDescriptorData) {
        const REGISTERS: [Register; kMaxBuiltinRegisterParams] =
            <DerivedDescriptor>::registers();
        const DOUBLE_REGISTERS: [DoubleRegister; kMaxBuiltinRegisterParams] =
            <DerivedDescriptor>::double_registers();
        const RETURN_REGISTERS: [Register; kMaxReturns] =
            <DerivedDescriptor>::return_registers();
        const RETURN_DOUBLE_REGISTERS: [DoubleRegister; kMaxReturns] =
            <DerivedDescriptor>::return_double_registers();
        DCHECK_EQ(data as *mut _, DerivedDescriptor::data(data) as *mut _);
        assert!(!data.is_initialized());

        if <DerivedDescriptor>::kRestrictAllocatableRegisters {
            data.restrict_allocatable_registers(
                REGISTERS.as_ptr(),
                REGISTERS.len() as i32,
            );
        } else {
            assert!(!<DerivedDescriptor>::kCalleeSaveRegisters);
        }

        assert!(REGISTERS.len() >= Self::get_register_parameter_count());
        assert!(DOUBLE_REGISTERS.len() >= Self::get_register_parameter_count());
        assert!(RETURN_REGISTERS.len() >= <DerivedDescriptor>::kReturnCount);
        assert!(RETURN_DOUBLE_REGISTERS.len() >= <DerivedDescriptor>::kReturnCount);

        data.initialize_registers(
            <DerivedDescriptor>::flags(),
            <DerivedDescriptor>::kEntrypointTag,
            <DerivedDescriptor>::kReturnCount,
            Self::get_parameter_count(),
            <DerivedDescriptor>::kStackArgumentOrder,
            Self::get_register_parameter_count(),
            REGISTERS.as_ptr(),
            DOUBLE_REGISTERS.as_ptr(),
            RETURN_REGISTERS.as_ptr(),
            RETURN_DOUBLE_REGISTERS.as_ptr(),
        );

        <DerivedDescriptor>::initialize_types(data);
        assert!(data.is_initialized());
        assert!(StaticCallInterfaceDescriptor::<DerivedDescriptor>::check_floating_point_parameters(data));

        #[cfg(debug_assertions)]
        <DerivedDescriptor>::verify(data);
    }
}

impl<DerivedDescriptor: StaticInterfaceDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    pub const fn get_return_count() -> i32 {
        assert!(<DerivedDescriptor>::kReturnCount >= 0);
        <DerivedDescriptor>::kReturnCount
    }

    pub const fn get_parameter_count() -> i32 {
        assert!(<DerivedDescriptor>::kParameterCount >= 0);
        <DerivedDescriptor>::kParameterCount
    }
}

pub mod detail {
    use crate::codegen::interface_descriptors::EmptyRegisterArray;
    use crate::codegen::register::Register;
    use std::array;
    use std::marker::PhantomData;

    pub struct IsRegisterArray<T> {
        phantom: PhantomData<T>,
    }

    impl<T> IsRegisterArray<T> {
        pub const VALUE: bool = false;
    }

    impl<const N: usize> IsRegisterArray<[Register; N]> {
        pub const VALUE: bool = true;
    }

    impl IsRegisterArray<EmptyRegisterArray> {
        pub const VALUE: bool = true;
    }

    pub struct FirstInvalidRegisterHelper<const N: usize, const Index: usize> {}

    impl<const N: usize, const Index: usize> FirstInvalidRegisterHelper<N, Index> {
        pub const fn call(regs: [Register; N]) -> usize {
            if !regs[Index].is_valid() {
                assert_eq!(
                    FirstInvalidRegisterHelper::<N, { Index + 1 }>::call(regs),
                    Index + 1
                );
                return Index;
            }
            FirstInvalidRegisterHelper::<N, { Index + 1 }>::call(regs)
        }
    }

    impl<const N: usize> FirstInvalidRegisterHelper<N, N> {
        pub const fn call(regs: [Register; N]) -> usize {
            N
        }
    }

    pub const fn first_invalid_register<const N: usize>(regs: [Register; N]) -> usize {
        FirstInvalidRegisterHelper::<N, 0>::call(regs)
    }

    pub const fn first_invalid_register_empty(_regs: EmptyRegisterArray) -> usize {
        0
    }
}

impl<DerivedDescriptor: StaticInterfaceDescriptor> StaticCallInterfaceDescriptor<DerivedDescriptor> {
    pub const fn get_register_parameter_count() -> i32 {
        const CHECK: bool = detail::IsRegisterArray::<
            <DerivedDescriptor as StaticInterfaceDescriptor>::RegistersType,
        >::VALUE;
        assert!(CHECK);

        let num_named_params = <DerivedDescriptor>::get_parameter_count();
        let registers = <DerivedDescriptor>::registers();
        let num_valid_registers = detail::first_invalid_register(registers);

        min(
            min(
                num_named_params,
                num_valid_registers as i32,
            ),
            <DerivedDescriptor>::kMaxRegisterParams,
        )
    }

    pub const fn get_stack_parameter_count() -> i32 {
        <DerivedDescriptor>::get_parameter_count() - Self::get_register_parameter_count()
    }

    pub const fn get_register_parameter(i: i32) -> Register {
        assert!(!Self::is_floating_point(
            Self::get_parameter_type(i).representation(),
        ));
        let registers = <DerivedDescriptor>::registers();
        registers[i as usize]
    }

    pub const fn get_stack_parameter_index(i: i32) -> i32 {
        i - Self::get_register_parameter_count()
    }

    pub const fn get_parameter_type(i: i32) -> MachineType {
        if <DerivedDescriptor>::kCustomMachineTypes {
            if <DerivedDescriptor>::allow_var_args() && i >= <DerivedDescriptor>::get_parameter_count() {
                return MachineType::AnyTagged();
            }
            assert!(i < <DerivedDescriptor>::get_parameter_count());
            let machine_types = <DerivedDescriptor>::machine_types();
            machine_types[<DerivedDescriptor>::kReturnCount as usize + i as usize]
        } else {
             MachineType::AnyTagged()
        }
    }

    pub const fn is_floating_point(representation: Representation) -> bool {
        match representation {
            Representation::Double => true,
            Representation::Float32 => true,
            _ => false,
        }
    }

    pub const fn get_double_register_parameter(i: i32) -> DoubleRegister {
        assert!(Self::is_floating_point(
            Self::get_parameter_type(i).representation(),
        ));
        let double_registers = <DerivedDescriptor>::double_registers();
        DoubleRegister::from_code(double_registers[i as usize].code())
    }
}

impl FastNewObjectDescriptor {
    pub const fn target_register() -> Register {
        kJSFunctionRegister
    }

    pub const fn new_target_register() -> Register {
        kJavaScriptCallNewTargetRegister
    }
}

impl WriteBarrierDescriptor {
    pub const fn object_register() -> Register {
        registers()[kObject as usize]
    }

    pub const fn slot_address_register() -> Register {
        registers()[kSlotAddress as usize]
    }

    pub const fn value_register() -> Register {
        registers()[kSlotAddress as usize + 1]
    }

    pub fn compute_saved_registers(object: Register, slot_address: Register) -> RegList {
        assert!(object != slot_address);
        let mut saved_registers = RegList::default();

        #[cfg(v8_target_arch_x64)]
        {
            if object != Self::object_register() {
                saved_registers.set(Self::object_register());
            }
            if slot_address != no_reg && slot_address != Self::slot_address_register() {
                saved_registers.set(Self::slot_address_register());
            }
        }

        #[cfg(any(
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_loong64,
            v8_target_arch_mips64
        ))]
        {
            if object != Self::object_register() {
                saved_registers.set(Self::object_register());
            }
            saved_registers.set(Self::slot_address_register());
        }

        #[cfg(not(any(
            v8_target_arch_x64,
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_loong64,
            v8_target_arch_mips64
        )))]
        {
            let allocated_registers = registers();
            for i in 0..allocated_registers.len() {
                saved_registers.set(allocated_registers[i]);
            }
        }

        saved_registers
    }
}

impl IndirectPointerWriteBarrierDescriptor {
    pub const fn registers() -> [Register; kMaxBuiltinRegisterParams] {
        WriteBarrierDescriptor::registers()
    }

    pub const fn object_register() -> Register {
        registers()[kObject as usize]
    }

    pub const fn slot_address_register() -> Register {
        registers()[kSlotAddress as usize]
    }

    pub const fn indirect_pointer_tag_register() -> Register {
        registers()[kIndirectPointerTag as usize]
    }

    pub fn compute_saved_registers(object: Register, slot_address: Register) -> RegList {
        assert!(object != slot_address);
        let mut saved_registers =
            WriteBarrierDescriptor::compute_saved_registers(object, slot_address);
        saved_registers.set(Self::indirect_pointer_tag_register());
        saved_registers
    }
}

impl ApiGetterDescriptor {
    pub const fn receiver_register() -> Register {
        LoadDescriptor::receiver_register()
    }
}

impl LoadGlobalNoFeedbackDescriptor {
    pub const fn ic_kind_register() -> Register {
        LoadDescriptor::slot_register()
    }
}

impl LoadNoFeedbackDescriptor {
    pub const fn ic_kind_register() -> Register {
        LoadGlobalNoFeedbackDescriptor::ic_kind_register()
    }
}

#[cfg(v8_target_arch_ia32)]
impl LoadGlobalWithVectorDescriptor {
    pub const fn vector_register() -> Register {
        assert!(!LoadWithVectorDescriptor::vector_register().is_valid());
        LoadDescriptor::receiver_register()
    }
}

#[cfg(not(v8_target_arch_ia32))]
impl LoadGlobalWithVectorDescriptor {
    pub const fn vector_register() -> Register {
        LoadWithVectorDescriptor::vector_register()
    }
}

impl LoadDescriptor {
    pub const fn registers() -> [Register; kLoadDescriptorRegisterParams] {
        register_array(Self::receiver_register(), Self::name_register(), Self::slot_register())
    }
}

impl LoadBaselineDescriptor {
    pub const fn registers() -> [Register; kLoadDescriptorRegisterParams] {
        LoadDescriptor::registers()
    }
}

impl LoadGlobalDescriptor {
    pub const fn registers() -> [Register; 2] {
        register_array(
            LoadDescriptor::name_register(),
            LoadDescriptor::slot_register(),
        )
    }
}

impl LoadGlobalBaselineDescriptor {
    pub const fn registers() -> [Register; 2] {
        LoadGlobalDescriptor::registers()
    }
}

impl StoreDescriptor {
    pub const fn registers() -> [Register; kStoreDescriptorRegisterParams] {
        register_array(
            Self::receiver_register(),
            Self::name_register(),
            Self::value_register(),
            Self::slot_register(),
        )
    }
}

impl StoreNoFeedbackDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            StoreDescriptor::receiver_register(),
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
        )
    }
}

impl StoreBaselineDescriptor {
    pub const fn registers() -> [Register; kStoreDescriptorRegisterParams] {
        StoreDescriptor::registers()
    }
}

impl StoreGlobalDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
            StoreDescriptor::slot_register(),
        )
    }
}

impl StoreGlobalBaselineDescriptor {
    pub const fn registers() -> [Register; 3] {
        StoreGlobalDescriptor::registers()
    }
}

impl DefineKeyedOwnDescriptor {
    pub const fn registers() -> [Register; kDefineKeyedOwnRegisterParams] {
        register_array(
            StoreDescriptor::receiver_register(),
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
            Self::flags_register(),
            StoreDescriptor::slot_register(),
        )
    }
}

impl DefineKeyedOwnBaselineDescriptor {
    pub const fn registers() -> [Register; kDefineKeyedOwnRegisterParams] {
        DefineKeyedOwnDescriptor::registers()
    }
}

impl LoadWithReceiverBaselineDescriptor {
    pub const fn registers() -> [Register; 4] {
        register_array(
            LoadDescriptor::receiver_register(),
            LoadWithReceiverAndVectorDescriptor::lookup_start_object_register(),
            LoadDescriptor::name_register(),
            LoadDescriptor::slot_register(),
        )
    }
}

impl BaselineOutOfLinePrologueDescriptor {
    pub const fn registers() -> [Register; kMaxBaselineOutOfLinePrologueRegisterParams] {
        #[cfg(any(
            v8_target_arch_x64,
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_ppc64,
            v8_target_arch_s390x,
            v8_target_arch_riscv64,
            v8_target_arch_mips64,
            v8_target_arch_loong64,
            v8_target_arch_riscv32
        ))]
        {
            return register_array(
                kContextRegister,
                kJSFunctionRegister,
                kJavaScriptCallArgCountRegister,
                kJavaScriptCallExtraArg1Register,
                kJavaScriptCallNewTargetRegister,
                kInterpreterBytecodeArrayRegister,
            );
        }

        #[cfg(v8_target_arch_ia32)]
        {
            assert_eq!(kJSFunctionRegister, kInterpreterBytecodeArrayRegister);
            return register_array(
                kContextRegister,
                kJSFunctionRegister,
                kJavaScriptCallArgCountRegister,
                kJavaScriptCallExtraArg1Register,
                kJavaScriptCallNewTargetRegister,
                /*kInterpreterBytecodeArrayRegister*/
            );
        }

        #[cfg(not(any(
            v8_target_arch_x64,
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_ppc64,
            v8_target_arch_s390x,
            v8_target_arch_riscv64,
            v8_target_arch_mips64,
            v8_target_arch_loong64,
            v8_target_arch_riscv32,
            v8_target_arch_ia32
        )))]
        {
            return CallInterfaceDescriptor::default_register_array();
        }
    }
}

impl BaselineLeaveFrameDescriptor {
    pub const fn registers() -> [Register; kMaxBaselineLeaveFrameRegisterParams] {
        #[cfg(any(
            v8_target_arch_ia32,
            v8_target_arch_x64,
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_ppc64,
            v8_target_arch_s390x,
            v8_target_arch_riscv64,
            v8_target_arch_mips64,
            v8_target_arch_loong64,
            v8_target_arch_riscv32
        ))]
        {
            return register_array(Self::params_size_register(), Self::weight_register());
        }

        #[cfg(not(any(
            v8_target_arch_ia32,
            v8_target_arch_x64,
            v8_target_arch_arm64,
            v8_target_arch_arm,
            v8_target_arch_ppc64,
            v8_target_arch_s390x,
            v8_target_arch_riscv64,
            v8_target_arch_mips64,
            v8_target_arch_loong64,
            v8_target_arch_riscv32
        )))]
        {
            return CallInterfaceDescriptor::default_register_array();
        }
    }
}

impl OnStackReplacementDescriptor {
    pub const fn registers() -> [Register; kMaxOnStackReplacementParams] {
        #[cfg(v8_target_arch_mips64)]
        {
            return register_array(
                kReturnRegister0,
                kJavaScriptCallArgCountRegister,
                kJavaScriptCallTargetRegister,
                kJavaScriptCallCodeStartRegister,
                kJavaScriptCallNewTargetRegister,
            );
        }

        #[cfg(not(v8_target_arch_mips64))]
        {
            return CallInterfaceDescriptor::default_register_array();
        }
    }

    pub const fn maybe_target_code_register() -> Register {
        Self::registers()[0]
    }

    pub const fn expected_parameter_count_register() -> Register {
        Self::registers()[1]
    }
}

impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
    pub const fn registers() -> [Register; kMaxMaglevOptimizeCodeOrTailCallParams] {
        #[cfg(v8_enable_maglev)]
        {
            return register_array(
                Self::flags_register(),
                Self::feedback_vector_register(),
                Self::temporary_register(),
            );
        }

        #[cfg(not(v8_enable_maglev))]
        {
            return CallInterfaceDescriptor::default_register_array();
        }
    }
}

impl VoidDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl AllocateDescriptor {
    pub const fn registers() -> [Register; 1] {
        register_array(kAllocateSizeRegister)
    }
}

impl CEntry1ArgvOnStackDescriptor {
    pub const fn registers() -> [Register; 2] {
        register_array(
            kRuntimeCallArgCountRegister,
            kRuntimeCallFunctionRegister,
        )
    }
}

impl InterpreterCEntry1Descriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            kRuntimeCallArgCountRegister,
            kRuntimeCallArgvRegister,
            kRuntimeCallFunctionRegister,
        )
    }
}

impl InterpreterCEntry2Descriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            kRuntimeCallArgCountRegister,
            kRuntimeCallArgvRegister,
            kRuntimeCallFunctionRegister,
        )
    }
}

impl FastNewObjectDescriptor {
    pub const fn registers() -> [Register; 2] {
        register_array(Self::target_register(), Self::new_target_register())
    }
}

impl LoadNoFeedbackDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            LoadDescriptor::receiver_register(),
            LoadDescriptor::name_register(),
            Self::ic_kind_register(),
        )
    }
}

impl LoadGlobalNoFeedbackDescriptor {
    pub const fn registers() -> [Register; 2] {
        register_array(
            LoadDescriptor::name_register(),
            Self::ic_kind_register(),
        )
    }
}

impl LoadGlobalWithVectorDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            LoadDescriptor::name_register(),
            LoadDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl LoadWithReceiverAndVectorDescriptor {
    pub const fn registers() -> [Register; 5] {
        register_array(
            LoadDescriptor::receiver_register(),
            Self::lookup_start_object_register(),
            LoadDescriptor::name_register(),
            LoadDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl StoreGlobalWithVectorDescriptor {
    pub const fn registers() -> [Register; 4] {
        register_array(
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
            StoreDescriptor::slot_register(),
            StoreWithVectorDescriptor::vector_register(),
        )
    }
}

impl StoreTransitionDescriptor {
    pub const fn registers() -> [Register; 6] {
        register_array(
            StoreDescriptor::receiver_register(),
            StoreDescriptor::name_register(),
            Self::map_register(),
            StoreDescriptor::value_register(),
            StoreDescriptor::slot_register(),
            StoreWithVectorDescriptor::vector_register(),
        )
    }
}

impl TypeConversionDescriptor {
    pub const fn registers() -> [Register; 1] {
        register_array(Self::argument_register())
    }
}

impl TypeConversionNoContextDescriptor {
    pub const fn registers() -> [Register; 1] {
        register_array(TypeConversionDescriptor::argument_register())
    }
}

impl SingleParameterOnStackDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl AsyncFunctionStackParameterDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl GetIteratorStackParameterDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl LoadWithVectorDescriptor {
    pub const fn registers() -> [Register; 4] {
        register_array(
            LoadDescriptor::receiver_register(),
            LoadDescriptor::name_register(),
            LoadDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl KeyedLoadBaselineDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            Self::receiver_register(),
            Self::name_register(),
            Self::slot_register(),
        )
    }
}

impl EnumeratedKeyedLoadBaselineDescriptor {
    pub const fn registers() -> [Register; 5] {
        register_array(
            KeyedLoadBaselineDescriptor::receiver_register(),
            KeyedLoadBaselineDescriptor::name_register(),
            Self::enum_index_register(),
            Self::cache_type_register(),
            Self::slot_register(),
        )
    }
}

impl EnumeratedKeyedLoadDescriptor {
    pub const fn registers() -> [Register; 6] {
        register_array(
            KeyedLoadBaselineDescriptor::receiver_register(),
            KeyedLoadBaselineDescriptor::name_register(),
            EnumeratedKeyedLoadBaselineDescriptor::enum_index_register(),
            EnumeratedKeyedLoadBaselineDescriptor::cache_type_register(),
            EnumeratedKeyedLoadBaselineDescriptor::slot_register(),
            KeyedLoadWithVectorDescriptor::vector_register(),
        )
    }
}

impl KeyedLoadDescriptor {
    pub const fn registers() -> [Register; 3] {
        KeyedLoadBaselineDescriptor::registers()
    }
}

impl KeyedLoadWithVectorDescriptor {
    pub const fn registers() -> [Register; 4] {
        register_array(
            KeyedLoadBaselineDescriptor::receiver_register(),
            KeyedLoadBaselineDescriptor::name_register(),
            KeyedLoadBaselineDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl KeyedHasICBaselineDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            Self::receiver_register(),
            Self::name_register(),
            Self::slot_register(),
        )
    }
}

impl KeyedHasICWithVectorDescriptor {
    pub const fn registers() -> [Register; 4] {
        register_array(
            KeyedHasICBaselineDescriptor::receiver_register(),
            KeyedHasICBaselineDescriptor::name_register(),
            KeyedHasICBaselineDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl StoreWithVectorDescriptor {
    pub const fn registers() -> [Register; 5] {
        register_array(
            StoreDescriptor::receiver_register(),
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
            StoreDescriptor::slot_register(),
            Self::vector_register(),
        )
    }
}

impl DefineKeyedOwnWithVectorDescriptor {
    pub const fn registers() -> [Register; 5] {
        register_array(
            StoreDescriptor::receiver_register(),
            StoreDescriptor::name_register(),
            StoreDescriptor::value_register(),
            DefineKeyedOwnDescriptor::flags_register(),
            StoreDescriptor::slot_register(),
        )
    }
}

impl CallApiCallbackOptimizedDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            Self::api_function_address_register(),
            Self::actual_arguments_count_register(),
            Self::function_template_info_register(),
        )
    }
}

impl CallApiCallbackGenericDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            Self::actual_arguments_count_register(),
            Self::topmost_script_having_context_register(),
            Self::function_template_info_register(),
        )
    }
}

impl ApiGetterDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            Self::receiver_register(),
            Self::holder_register(),
            Self::callback_register(),
        )
    }
}

impl ContextOnlyDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl NoContextDescriptor {
    pub const fn registers() -> [Register; 0] {
        register_array()
    }
}

impl GrowArrayElementsDescriptor {
    pub const fn registers() -> [Register; 2] {
        register_array(
            Self::object_register(),
            Self::key_register(),
        )
    }
}

impl ArrayNArgumentsConstructorDescriptor {
    pub const fn registers() -> [Register; 3] {
        register_array(
            kJavaScriptCallTargetRegister,
            kJavaScriptCallExtraArg1Register,
            kJavaScriptCallArgCountRegister,
        )
    }
}

impl ArrayNoArgumentConstructorDescriptor {
    pub const fn registers() -> [Register; 3] {
        ArrayNArgumentsConstructorDescriptor::registers()
    }
}

impl ArraySingleArgumentConstructorDescriptor {
    pub const fn registers() -> [Register; 3] {
        ArrayNArgumentsConstructorDescriptor::registers()
    }
}

impl RunMicrotasksDescriptor {
    pub const fn microtask_queue_register() -> Register {
        StaticCallInterfaceDescriptor::<Self>::get_register_parameter(0)
    }
}

impl WasmJSToWasmWrapperDescriptor {
    pub const fn wrapper_buffer_register() -> Register {
        registers()[kWasmJSToWasmWrapperRegisterParams::kWrapperBuffer as usize]
    }
}

impl WasmHandleStackOverflowDescriptor {
    pub const fn frame_base_register() -> Register {
        registers()[kWasmHandleStackOverflowRegisterParams::kFrameBase as usize]
    }

    pub const fn gap_register() -> Register {
        registers()[kWasmHandleStackOverflowRegisterParams::kGap as usize]
    }
}

impl WasmToJSWrapperDescriptor {
    pub const fn registers() -> [Register; kWasmToJSWrapperRegisterParams] {
        #[cfg(v8_enable_webassembly)]
        {
            return register_array(wasm::kGpParamRegisters[0]);
        }

        #[cfg(not(v8_enable_webassembly))]
        {
            return EmptyRegisterArray::new();
        }
    }

    pub const fn return_registers() -> [Register; kMaxReturns] {
        #[cfg(v8_enable_webassembly)]
        {
            return register_array(
                wasm::kGpReturnRegisters[0],
                wasm::kGpReturnRegisters[1],
                no_reg,
                no_reg,
            );
        }

        #[cfg(not(v8_enable_webassembly))]
        {
            return CallInterfaceDescriptor::default_register_array();
        }
    }

    pub const fn return_double_registers() -> [DoubleRegister; kMaxReturns] {
        #[cfg(v8_enable_webassembly)]
        {
            return double_register_array(
                no_dreg,
                no_dreg,
                wasm::kFpReturnRegisters[0],
                wasm::kFpReturnRegisters[1],
            );
        }

        #[cfg(not(v8_enable_webassembly))]
        {
            return CallInterfaceDescriptor::default_double_register_array();
        }
    }
}
}
