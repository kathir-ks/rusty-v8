// Converted from V8 C++ source files:
// Header: interface-descriptors.h
// Implementation: interface-descriptors.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interface_descriptors {
    use std::mem::MaybeUninit;
    use std::ptr;
    use std::sync::Once;

    use crate::base::logging::DCHECK_EQ;
    use crate::codegen::machine_type::MachineType;
    use crate::codegen::register::{DoubleRegister, Register};
    use crate::common::globals::FLAG_concurrent_sparkplug;
    use crate::execution::isolate::Isolate;

    #[macro_export]
    macro_rules! TORQUE_BUILTIN_LIST_TFC {
        ($V:ident) => {
            BUILTIN_LIST_FROM_TORQUE!(
                IGNORE_BUILTIN,
                IGNORE_BUILTIN,
                $V,
                IGNORE_BUILTIN,
                IGNORE_BUILTIN,
                IGNORE_BUILTIN
            );
        };
    }

    #[macro_export]
    macro_rules! INTERFACE_DESCRIPTOR_LIST {
        ($V:ident) => {
            $V!(Abort) ;
            $V!(Allocate) ;
            $V!(CallApiCallbackGeneric) ;
            $V!(CallApiCallbackOptimized) ;
            $V!(ApiGetter) ;
            $V!(ArrayConstructor) ;
            $V!(ArrayNArgumentsConstructor) ;
            $V!(ArrayNoArgumentConstructor) ;
            $V!(ArraySingleArgumentConstructor) ;
            $V!(AsyncFunctionStackParameter) ;
            $V!(BaselineLeaveFrame) ;
            $V!(BaselineOutOfLinePrologue) ;
            $V!(BigIntToI32Pair) ;
            $V!(BigIntToI64) ;
            $V!(BinaryOp) ;
            $V!(BinaryOp_Baseline) ;
            $V!(BinaryOp_WithFeedback) ;
            $V!(BinarySmiOp_Baseline) ;
            $V!(CallForwardVarargs) ;
            $V!(CallFunctionTemplate) ;
            $V!(CallFunctionTemplateGeneric) ;
            $V!(CallTrampoline) ;
            $V!(CallTrampoline_Baseline) ;
            $V!(CallTrampoline_Baseline_Compact) ;
            $V!(CallTrampoline_WithFeedback) ;
            $V!(CallVarargs) ;
            $V!(CallWithArrayLike) ;
            $V!(CallWithArrayLike_WithFeedback) ;
            $V!(CallWithSpread) ;
            $V!(CallWithSpread_Baseline) ;
            $V!(CallWithSpread_WithFeedback) ;
            $V!(CCall) ;
            $V!(CEntryDummy) ;
            $V!(CEntry1ArgvOnStack) ;
            $V!(CloneObjectBaseline) ;
            $V!(CloneObjectWithVector) ;
            $V!(Compare) ;
            $V!(CompareNoContext) ;
            $V!(StringEqual) ;
            $V!(Compare_Baseline) ;
            $V!(Compare_WithFeedback) ;
            $V!(Construct_Baseline) ;
            $V!(ConstructForwardVarargs) ;
            $V!(ConstructForwardAllArgs) ;
            $V!(ConstructForwardAllArgs_Baseline) ;
            $V!(ConstructForwardAllArgs_WithFeedback) ;
            $V!(ConstructStub) ;
            $V!(ConstructVarargs) ;
            $V!(ConstructWithArrayLike) ;
            $V!(Construct_WithFeedback) ;
            $V!(ConstructWithSpread) ;
            $V!(ConstructWithSpread_Baseline) ;
            $V!(ConstructWithSpread_WithFeedback) ;
            $V!(ContextOnly) ;
            $V!(CopyDataPropertiesWithExcludedProperties) ;
            $V!(CopyDataPropertiesWithExcludedPropertiesOnStack) ;
            $V!(CppBuiltinAdaptor) ;
            $V!(CreateFromSlowBoilerplateHelper) ;
            $V!(DefineKeyedOwn) ;
            $V!(DefineKeyedOwnBaseline) ;
            $V!(DefineKeyedOwnWithVector) ;
            $V!(FastNewObject) ;
            $V!(FindNonDefaultConstructorOrConstruct) ;
            $V!(ForInPrepare) ;
            $V!(GetIteratorStackParameter) ;
            $V!(GetProperty) ;
            $V!(GrowArrayElements) ;
            $V!(I32PairToBigInt) ;
            $V!(I64ToBigInt) ;
            $V!(InterpreterCEntry1) ;
            $V!(InterpreterCEntry2) ;
            $V!(InterpreterDispatch) ;
            $V!(InterpreterPushArgsThenCall) ;
            $V!(InterpreterPushArgsThenConstruct) ;
            $V!(JSTrampoline) ;
            $V!(KeyedHasICBaseline) ;
            $V!(KeyedHasICWithVector) ;
            $V!(KeyedLoad) ;
            $V!(KeyedLoadBaseline) ;
            $V!(EnumeratedKeyedLoadBaseline) ;
            $V!(KeyedLoadWithVector) ;
            $V!(EnumeratedKeyedLoad) ;
            $V!(Load) ;
            $V!(LoadBaseline) ;
            $V!(LoadGlobal) ;
            $V!(LoadGlobalBaseline) ;
            $V!(LoadGlobalNoFeedback) ;
            $V!(LoadGlobalWithVector) ;
            $V!(LoadNoFeedback) ;
            $V!(LoadWithReceiverAndVector) ;
            $V!(LoadWithReceiverBaseline) ;
            $V!(LoadWithVector) ;
            $V!(LookupWithVector) ;
            $V!(LookupTrampoline) ;
            $V!(LookupBaseline) ;
            $V!(MaglevOptimizeCodeOrTailCallOptimizedCodeSlot) ;
            $V!(NewHeapNumber) ;
            $V!(NoContext) ;
            $V!(OnStackReplacement) ;
            $V!(RegExpTrampoline) ;
            $V!(RestartFrameTrampoline) ;
            $V!(ResumeGenerator) ;
            $V!(ResumeGeneratorBaseline) ;
            $V!(RunMicrotasks) ;
            $V!(RunMicrotasksEntry) ;
            $V!(SingleParameterOnStack) ;
            $V!(Store) ;
            $V!(StoreNoFeedback) ;
            $V!(StoreBaseline) ;
            $V!(StoreGlobal) ;
            $V!(StoreGlobalBaseline) ;
            $V!(StoreGlobalWithVector) ;
            $V!(StoreTransition) ;
            $V!(StoreWithVector) ;
            $V!(StringAtAsString) ;
            $V!(StringSubstring) ;
            $V!(SuspendGeneratorBaseline) ;
            $V!(TypeConversion) ;
            $V!(TypeConversion_Baseline) ;
            $V!(TypeConversionNoContext) ;
            $V!(Typeof) ;
            $V!(UnaryOp_Baseline) ;
            $V!(UnaryOp_WithFeedback) ;
            $V!(Void) ;
            $V!(WasmDummy) ;
            $V!(WasmFloat32ToNumber) ;
            $V!(WasmFloat64ToTagged) ;
            $V!(WasmJSToWasmWrapper) ;
            $V!(WasmToJSWrapper) ;
            $V!(WasmSuspend) ;
            $V!(WasmHandleStackOverflow) ;
            $V!(WriteBarrier) ;
            $V!(IndirectPointerWriteBarrier) ;
            IF_TSAN!($V, TSANLoad) ;
            IF_TSAN!($V, TSANStore) ;
            BUILTIN_LIST_TFS!($V) ;
            TORQUE_BUILTIN_LIST_TFC!($V) ;
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StackArgumentOrder {
        kDefault,
        kJS,
    }

    bitflags::bitflags! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct Flag: u32 {
            const kNoFlags = 0u32;
            const kNoContext = 1u32 << 0;
            const kNoStackScan = 1u32 << 1;
            const kAllowVarArgs = 1u32 << 2;
            const kCalleeSaveRegisters = 1u32 << 3;
        }
    }

    #[derive(Debug)]
    pub struct CallInterfaceDescriptorData {
        flags_: Flag,
        tag_: CodeEntrypointTag,
        return_count_: i32,
        param_count_: i32,
        register_param_count_: i32,
        register_params_: *const Register,
        double_register_params_: *const DoubleRegister,
        register_returns_: *const Register,
        double_register_returns_: *const DoubleRegister,
        machine_types_: *mut MachineType,
        allocatable_registers_: RegList,
    }

    impl Default for CallInterfaceDescriptorData {
        fn default() -> Self {
            CallInterfaceDescriptorData {
                flags_: Flag::kNoFlags,
                tag_: CodeEntrypointTag::kDefaultCodeEntrypointTag,
                return_count_: -1,
                param_count_: -1,
                register_param_count_: -1,
                register_params_: ptr::null(),
                double_register_params_: ptr::null(),
                register_returns_: ptr::null(),
                double_register_returns_: ptr::null(),
                machine_types_: ptr::null_mut(),
                allocatable_registers_: RegList::default(),
            }
        }
    }

    impl CallInterfaceDescriptorData {
        const kUninitializedCount: i32 = -1;

        pub fn new() -> Self {
            Self::default()
        }

        pub fn initialize_registers(
            &mut self,
            flags: Flag,
            tag: CodeEntrypointTag,
            return_count: i32,
            parameter_count: i32,
            stack_order: StackArgumentOrder,
            register_parameter_count: i32,
            registers: *const Register,
            double_registers: *const DoubleRegister,
            return_registers: *const Register,
            return_double_registers: *const DoubleRegister,
        ) {
            if self.is_initialized_types() {
                panic!("Types already initialized");
            }

            #[cfg(debug_assertions)]
            {
                CheckRegisterConfiguration(register_parameter_count, registers, double_registers);
                CheckRegisterConfiguration(return_count, return_registers, return_double_registers);
            }

            self.flags_ = flags;
            self.tag_ = tag;
            self.stack_order_ = stack_order;
            self.return_count_ = return_count;
            self.param_count_ = parameter_count;
            self.register_param_count_ = register_parameter_count;
            self.register_params_ = registers;
            self.double_register_params_ = double_registers;
            self.register_returns_ = return_registers;
            self.double_register_returns_ = return_double_registers;
        }

        pub fn initialize_types(&mut self, machine_types: *const MachineType, machine_types_length: i32) {
            if !self.is_initialized_registers() {
                panic!("Registers not initialized");
            }

            let types_length = self.return_count_ + self.param_count_;

            if machine_types == ptr::null() {
                unsafe {
                    self.machine_types_ =
                        allocate_array(types_length as usize, MachineType::AnyTagged());
                }
            } else {
                if machine_types_length != types_length {
                    panic!("Invalid machine types length");
                }
                unsafe {
                    self.machine_types_ = allocate_array(types_length as usize);
                    for i in 0..types_length {
                        let machine_type = machine_types.add(i as usize).read();
                        self.machine_types_.add(i as usize).write(machine_type);
                    }
                }
            }

            if !(self.flags_ & Flag::kNoStackScan).contains() && !self.all_stack_parameters_are_tagged() {
                panic!("Stack parameters are not all tagged");
            }
        }

        pub fn reset(&mut self) {
            if !self.machine_types_.is_null() {
                unsafe { deallocate_array(self.machine_types_) };
                self.machine_types_ = ptr::null_mut();
            }
            self.register_params_ = ptr::null();
            self.double_register_params_ = ptr::null();
            self.register_returns_ = ptr::null();
            self.double_register_returns_ = ptr::null();
        }

        pub fn is_initialized(&self) -> bool {
            self.is_initialized_registers() && self.is_initialized_types()
        }

        pub fn flags(&self) -> Flag {
            self.flags_
        }
        pub fn tag(&self) -> CodeEntrypointTag {
            self.tag_
        }
        pub fn return_count(&self) -> i32 {
            self.return_count_
        }
        pub fn param_count(&self) -> i32 {
            self.param_count_
        }
        pub fn register_param_count(&self) -> i32 {
            self.register_param_count_
        }
        pub fn register_param(&self, index: i32) -> Register {
            if index >= self.register_param_count_ {
                panic!("Register parameter index out of bounds");
            }
            unsafe { *self.register_params_.add(index as usize) }
        }
        pub fn double_register_param(&self, index: i32) -> DoubleRegister {
            if index >= self.register_param_count_ {
                panic!("Double register parameter index out of bounds");
            }
            unsafe { *self.double_register_params_.add(index as usize) }
        }
        pub fn register_return(&self, index: i32) -> Register {
            if index >= self.return_count_ {
                panic!("Register return index out of bounds");
            }
            unsafe { *self.register_returns_.add(index as usize) }
        }
        pub fn double_register_return(&self, index: i32) -> DoubleRegister {
            if index >= self.return_count_ {
                panic!("Double register return index out of bounds");
            }
            unsafe { *self.double_register_returns_.add(index as usize) }
        }
        pub fn return_type(&self, index: i32) -> MachineType {
            if index >= self.return_count_ {
                panic!("Return type index out of bounds");
            }
            unsafe { *self.machine_types_.add(index as usize) }
        }
        pub fn param_type(&self, index: i32) -> MachineType {
            if index >= self.param_count_ {
                panic!("Parameter type index out of bounds");
            }
            unsafe { *self.machine_types_.add((self.return_count_ + index) as usize) }
        }
        pub fn stack_order(&self) -> StackArgumentOrder {
            self.stack_order_
        }

        pub fn restrict_allocatable_registers(&mut self, registers: *const Register, num: usize) {
            if !self.allocatable_registers_.is_empty() {
                panic!("Allocatable registers already restricted");
            }
            for i in 0..num {
                unsafe {
                    self.allocatable_registers_.set(*registers.add(i));
                }
            }
            if self.allocatable_registers_.is_empty() {
                panic!("Allocatable registers cannot be empty");
            }
        }

        pub fn allocatable_registers(&self) -> RegList {
            self.allocatable_registers_
        }
    }

    impl CallInterfaceDescriptorData {
        fn is_initialized_registers(&self) -> bool {
            let initialized = self.return_count_ != Self::kUninitializedCount
                && self.param_count_ != Self::kUninitializedCount
                && (self.register_param_count_ == 0 || !self.register_params_.is_null());
            initialized
        }

        fn is_initialized_types(&self) -> bool {
            let initialized = !self.machine_types_.is_null();
            initialized
        }

        #[cfg(debug_assertions)]
        fn all_stack_parameters_are_tagged(&self) -> bool {
            if !self.is_initialized() {
                panic!("Not initialized");
            }
            let types_length = self.return_count_ + self.param_count_;
            let first_stack_param = self.return_count_ + self.register_param_count_;
            for i in first_stack_param..types_length {
                unsafe {
                    if !(*self.machine_types_.add(i as usize)).is_tagged() {
                        return false;
                    }
                }
            }
            true
        }
    }

    struct CallDescriptorsData {
        call_descriptor_data: [CallInterfaceDescriptorData; NUMBER_OF_DESCRIPTORS],
        initialized: Once,
    }

    impl CallDescriptorsData {
        const fn new() -> Self {
            CallDescriptorsData {
                call_descriptor_data: [CallInterfaceDescriptorData::default(); NUMBER_OF_DESCRIPTORS],
                initialized: Once::new(),
            }
        }
    }

    static CALL_DESCRIPTORS_DATA: CallDescriptorsData = CallDescriptorsData::new();

    pub struct CallDescriptors;

    impl CallDescriptors {
        pub const NUMBER_OF_DESCRIPTORS: usize = {
            #[allow(non_camel_case_types)]
            enum NumberOfDescriptors {
                NUMBER_OF_DESCRIPTORS,
            }
            NumberOfDescriptors::NUMBER_OF_DESCRIPTORS as usize
        };

        pub fn initialize_once_per_process() {
            CALL_DESCRIPTORS_DATA.initialized.call_once(|| {
                macro_rules! interface_descriptor {
                    ($name:ident, $($args:tt)*) => {
                        $crate::codegen::interface_descriptors::$name##Descriptor::new().initialize(Self::call_descriptor_data_mut(CallDescriptors::Key::$name));
                    };
                }
                INTERFACE_DESCRIPTOR_LIST!(interface_descriptor);

                assert!(ContextOnlyDescriptor::new().has_context_parameter());
                assert!(!NoContextDescriptor::new().has_context_parameter());
                assert!(!AllocateDescriptor::new().has_context_parameter());
                assert!(!AbortDescriptor::new().has_context_parameter());
                assert!(!WasmFloat32ToNumberDescriptor::new().has_context_parameter());
                assert!(!WasmFloat64ToTaggedDescriptor::new().has_context_parameter());
            });
        }

        pub fn tear_down() {
            for data in &mut CALL_DESCRIPTORS_DATA.call_descriptor_data {
                data.reset();
            }
        }

        pub fn call_descriptor_data(key: Key) -> &'static CallInterfaceDescriptorData {
            unsafe { &CALL_DESCRIPTORS_DATA.call_descriptor_data[key as usize] }
        }

        pub fn call_descriptor_data_mut(key: Key) -> &'static mut CallInterfaceDescriptorData {
            unsafe { &mut CALL_DESCRIPTORS_DATA.call_descriptor_data[key as usize] }
        }

        pub fn get_key(data: &CallInterfaceDescriptorData) -> Key {
            let ptr = data as *const CallInterfaceDescriptorData;
            let base = CALL_DESCRIPTORS_DATA.call_descriptor_data.as_ptr();
            let index = unsafe { ptr.offset_from(base) };
            if index < 0 || index >= Self::NUMBER_OF_DESCRIPTORS as isize {
                panic!("Invalid data pointer");
            }
            Key::from_usize(index as usize).unwrap()
        }
    }

    #[repr(usize)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Key {
        Abort,
        Allocate,
        CallApiCallbackGeneric,
        CallApiCallbackOptimized,
        ApiGetter,
        ArrayConstructor,
        ArrayNArgumentsConstructor,
        ArrayNoArgumentConstructor,
        ArraySingleArgumentConstructor,
        AsyncFunctionStackParameter,
        BaselineLeaveFrame,
        BaselineOutOfLinePrologue,
        BigIntToI32Pair,
        BigIntToI64,
        BinaryOp,
        BinaryOp_Baseline,
        BinaryOp_WithFeedback,
        BinarySmiOp_Baseline,
        CallForwardVarargs,
        CallFunctionTemplate,
        CallFunctionTemplateGeneric,
        CallTrampoline,
        CallTrampoline_Baseline,
        CallTrampoline_Baseline_Compact,
        CallTrampoline_WithFeedback,
        CallVarargs,
        CallWithArrayLike,
        CallWithArrayLike_WithFeedback,
        CallWithSpread,
        CallWithSpread_Baseline,
        CallWithSpread_WithFeedback,
        CCall,
        CEntryDummy,
        CEntry1ArgvOnStack,
        CloneObjectBaseline,
        CloneObjectWithVector,
        Compare,
        CompareNoContext,
        StringEqual,
        Compare_Baseline,
        Compare_WithFeedback,
        Construct_Baseline,
        ConstructForwardVarargs,
        ConstructForwardAllArgs,
        ConstructForwardAllArgs_Baseline,
        ConstructForwardAllArgs_WithFeedback,
        ConstructStub,
        ConstructVarargs,
        ConstructWithArrayLike,
        Construct_WithFeedback,
        ConstructWithSpread,
        ConstructWithSpread_Baseline,
        ConstructWithSpread_WithFeedback,
        ContextOnly,
        CopyDataPropertiesWithExcludedProperties,
        CopyDataPropertiesWithExcludedPropertiesOnStack,
        CppBuiltinAdaptor,
        CreateFromSlowBoilerplateHelper,
        DefineKeyedOwn,
        DefineKeyedOwnBaseline,
        DefineKeyedOwnWithVector,
        FastNewObject,
        FindNonDefaultConstructorOrConstruct,
        ForInPrepare,
        GetIteratorStackParameter,
        GetProperty,
        GrowArrayElements,
        I32PairToBigInt,
        I64ToBigInt,
        InterpreterCEntry1,
        InterpreterCEntry2,
        InterpreterDispatch,
        InterpreterPushArgsThenCall,
        InterpreterPushArgsThenConstruct,
        JSTrampoline,
        KeyedHasICBaseline,
        KeyedHasICWithVector,
        KeyedLoad,
        KeyedLoadBaseline,
        EnumeratedKeyedLoadBaseline,
        KeyedLoadWithVector,
        EnumeratedKeyedLoad,
        Load,
        LoadBaseline,
        LoadGlobal,
        LoadGlobalBaseline,
        LoadGlobalNoFeedback,
        LoadGlobalWithVector,
        LoadNoFeedback,
        LoadWithReceiverAndVector,
        LoadWithReceiverBaseline,
        LoadWithVector,
        LookupWithVector,
        LookupTrampoline,
        LookupBaseline,
        MaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
        NewHeapNumber,
        NoContext,
        OnStackReplacement,
        RegExpTrampoline,
        RestartFrameTrampoline,
        ResumeGenerator,
        ResumeGeneratorBaseline,
        RunMicrotasks,
        RunMicrotasksEntry,
        SingleParameterOnStack,
        Store,
        StoreNoFeedback,
        StoreBaseline,
        StoreGlobal,
        StoreGlobalBaseline,
        StoreGlobalWithVector,
        StoreTransition,
        StoreWithVector,
        StringAtAsString,
        StringSubstring,
        SuspendGeneratorBaseline,
        TypeConversion,
        TypeConversion_Baseline,
        TypeConversionNoContext,
        Typeof,
        UnaryOp_Baseline,
        UnaryOp_WithFeedback,
        Void,
        WasmDummy,
        WasmFloat32ToNumber,
        WasmFloat64ToTagged,
        WasmJSToWasmWrapper,
        WasmToJSWrapper,
        WasmSuspend,
        WasmHandleStackOverflow,
        WriteBarrier,
        IndirectPointerWriteBarrier,
        TSANLoad,
        TSANStore,
        TurboshaftFloat32,
        TurboshaftFloat64,
        NUMBER_OF_DESCRIPTORS,
    }

    impl Key {
        fn from_usize(value: usize) -> Option<Self> {
            if value < Self::NUMBER_OF_DESCRIPTORS {
                Some(unsafe { std::mem::transmute(value) })
            } else {
                None
            }
        }
    }

    const kMaxBuiltinRegisterParams: i32 = 5;
    const kMaxTFSBuiltinRegisterParams: i32 = {
        #[cfg(target_arch = "ia32")]
        {
            3
        }
        #[cfg(not(target_arch = "ia32"))]
        {
            kMaxBuiltinRegisterParams
        }
    };
    const kJSBuiltinRegisterParams: i32 = 4;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct CallInterfaceDescriptor {
        data_: *const CallInterfaceDescriptorData,
    }

    impl Default for CallInterfaceDescriptor {
        fn default() -> Self {
            CallInterfaceDescriptor { data_: ptr::null() }
        }
    }

    impl CallInterfaceDescriptor {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn from_key(key: Key) -> Self {
            CallInterfaceDescriptor {
                data_: CallDescriptors::call_descriptor_data(key),
            }
        }

        pub fn flags(&self) -> Flag {
            unsafe { (*self.data_).flags() }
        }

        pub fn tag(&self) -> CodeEntrypointTag {
            unsafe { (*self.data_).tag() }
        }

        pub fn has_context_parameter(&self) -> bool {
            !(self.flags() & Flag::kNoContext).contains()
        }

        pub fn allow_var_args(&self) -> bool {
            (self.flags() & Flag::kAllowVarArgs).contains()
        }

        pub fn callee_save_registers(&self) -> bool {
            (self.flags() & Flag::kCalleeSaveRegisters).contains()
        }

        pub fn get_return_count(&self) -> i32 {
            unsafe { (*self.data_).return_count() }
        }

        pub fn get_return_type(&self, index: i32) -> MachineType {
            unsafe { (*self.data_).return_type(index) }
        }

        pub fn get_parameter_count(&self) -> i32 {
            unsafe { (*self.data_).param_count() }
        }

        pub fn get_register_parameter_count(&self) -> i32 {
            unsafe { (*self.data_).register_param_count() }
        }

        pub fn get_stack_parameter_count(&self) -> i32 {
            self.get_parameter_count() - self.get_register_parameter_count()
        }

        pub fn get_register_parameter(&self, index: i32) -> Register {
            unsafe { (*self.data_).register_param(index) }
        }

        pub fn get_double_register_parameter(&self, index: i32) -> DoubleRegister {
            unsafe { (*self.data_).double_register_param(index) }
        }

        pub fn get_register_return(&self, index: i32) -> Register {
            unsafe { (*self.data_).register_return(index) }
        }

        pub fn get_double_register_return(&self, index: i32) -> DoubleRegister {
            unsafe { (*self.data_).double_register_return(index) }
        }

        pub fn get_parameter_type(&self, index: i32) -> MachineType {
            unsafe { (*self.data_).param_type(index) }
        }

        pub fn allocatable_registers(&self) -> RegList {
            unsafe { (*self.data_).allocatable_registers() }
        }

        pub fn get_stack_argument_order(&self) -> StackArgumentOrder {
            unsafe { (*self.data_).stack_order() }
        }

        pub const fn context_register() -> Register {
            Register::kContextRegister
        }

        pub fn debug_name(&self) -> &'static str {
            let key = CallDescriptors::get_key(unsafe { &*self.data_ });
            match key {
                Key::Abort => "Abort Descriptor",
                Key::Allocate => "Allocate Descriptor",
                Key::CallApiCallbackGeneric => "CallApiCallbackGeneric Descriptor",
                Key::CallApiCallbackOptimized => "CallApiCallbackOptimized Descriptor",
                Key::ApiGetter => "ApiGetter Descriptor",
                Key::ArrayConstructor => "ArrayConstructor Descriptor",
                Key::ArrayNArgumentsConstructor => "ArrayNArgumentsConstructor Descriptor",
                Key::ArrayNoArgumentConstructor => "ArrayNoArgumentConstructor Descriptor",
                Key::ArraySingleArgumentConstructor => "ArraySingleArgumentConstructor Descriptor",
                Key::AsyncFunctionStackParameter => "AsyncFunctionStackParameter Descriptor",
                Key::BaselineLeaveFrame => "BaselineLeaveFrame Descriptor",
                Key::BaselineOutOfLinePrologue => "BaselineOutOfLinePrologue Descriptor",
                Key::BigIntToI32Pair => "BigIntToI32Pair Descriptor",
                Key::BigIntToI64 => "BigIntToI64 Descriptor",
                Key::BinaryOp => "BinaryOp Descriptor",
                Key::BinaryOp_Baseline => "BinaryOp_Baseline Descriptor",
                Key::BinaryOp_WithFeedback => "BinaryOp_WithFeedback Descriptor",
                Key::BinarySmiOp_Baseline => "BinarySmiOp_Baseline Descriptor",
                Key::CallForwardVarargs => "CallForwardVarargs Descriptor",
                Key::CallFunctionTemplate => "CallFunctionTemplate Descriptor",
                Key::CallFunctionTemplateGeneric => "CallFunctionTemplateGeneric Descriptor",
                Key::CallTrampoline => "CallTrampoline Descriptor",
                Key::CallTrampoline_Baseline => "CallTrampoline_Baseline Descriptor",
                Key::CallTrampoline_Baseline_Compact => "CallTrampoline_Baseline_Compact Descriptor",
                Key::CallTrampoline_WithFeedback => "CallTrampoline_WithFeedback Descriptor",
                Key::CallVarargs => "CallVarargs Descriptor",
                Key::CallWithArrayLike => "CallWithArrayLike Descriptor",
                Key::CallWithArrayLike_WithFeedback => "CallWithArrayLike_WithFeedback Descriptor",
                Key::CallWithSpread => "CallWithSpread Descriptor",
                Key::CallWithSpread_Baseline => "CallWithSpread_Baseline Descriptor",
                Key::CallWithSpread_WithFeedback => "CallWithSpread_WithFeedback Descriptor",
                Key::CCall => "CCall Descriptor",
                Key::CEntryDummy => "CEntryDummy Descriptor",
                Key::CEntry1ArgvOnStack => "CEntry1ArgvOnStack Descriptor",
                Key::CloneObjectBaseline => "CloneObjectBaseline Descriptor",
                Key::CloneObjectWithVector => "CloneObjectWithVector Descriptor",
                Key::Compare => "Compare Descriptor",
                Key::CompareNoContext => "CompareNoContext Descriptor",
                Key::StringEqual => "StringEqual Descriptor",
                Key::Compare_Baseline => "Compare_Baseline Descriptor",
                Key::Compare_WithFeedback => "Compare_WithFeedback Descriptor",
                Key::Construct_Baseline => "Construct_Baseline Descriptor",
                Key::ConstructForwardVarargs => "ConstructForwardVarargs Descriptor",
                Key::ConstructForwardAllArgs => "ConstructForwardAllArgs Descriptor",
                Key::ConstructForwardAllArgs_Baseline => "ConstructForwardAllArgs_Baseline Descriptor",
                Key::ConstructForwardAllArgs_WithFeedback => "ConstructForwardAllArgs_WithFeedback Descriptor",
                Key::ConstructStub => "ConstructStub Descriptor",
                Key::ConstructVarargs => "ConstructVarargs Descriptor",
                Key::ConstructWithArrayLike => "ConstructWithArrayLike Descriptor",
                Key::Construct_WithFeedback => "Construct_WithFeedback Descriptor",
                Key::ConstructWithSpread => "ConstructWithSpread Descriptor",
                Key::ConstructWithSpread_Baseline => "ConstructWithSpread_Baseline Descriptor",
                Key::ConstructWithSpread_WithFeedback => "ConstructWithSpread_WithFeedback Descriptor",
                Key::ContextOnly => "ContextOnly Descriptor",
                Key::CopyDataPropertiesWithExcludedProperties => "CopyDataPropertiesWithExcludedProperties Descriptor",
                Key::CopyDataPropertiesWithExcludedPropertiesOnStack => "CopyDataPropertiesWithExcludedPropertiesOnStack Descriptor",
                Key::CppBuiltinAdaptor => "CppBuiltinAdaptor Descriptor",
                Key::CreateFromSlowBoilerplateHelper => "CreateFromSlowBoilerplateHelper Descriptor",
                Key::DefineKeyedOwn => "DefineKeyedOwn Descriptor",
                Key::DefineKeyedOwnBaseline => "DefineKeyedOwnBaseline Descriptor",
                Key::DefineKeyedOwnWithVector => "DefineKeyedOwnWithVector Descriptor",
                Key::FastNewObject => "FastNewObject Descriptor",
                Key::FindNonDefaultConstructorOrConstruct => "FindNonDefaultConstructorOrConstruct Descriptor",
                Key::ForInPrepare => "ForInPrepare Descriptor",
                Key::GetIteratorStackParameter => "GetIteratorStackParameter Descriptor",
                Key::GetProperty => "GetProperty Descriptor",
                Key::GrowArrayElements => "GrowArrayElements Descriptor",
                Key::I32PairToBigInt => "I32PairToBigInt Descriptor",
                Key::I64ToBigInt => "I64ToBigInt Descriptor",
                Key::InterpreterCEntry1 => "InterpreterCEntry1 Descriptor",
                Key::InterpreterCEntry2 => "InterpreterCEntry2 Descriptor",
                Key::InterpreterDispatch => "InterpreterDispatch Descriptor",
                Key::InterpreterPushArgsThenCall => "InterpreterPushArgsThenCall Descriptor",
                Key::InterpreterPushArgsThenConstruct => "InterpreterPushArgsThenConstruct Descriptor",
                Key::JSTrampoline => "JSTrampoline Descriptor",
                Key::KeyedHasICBaseline => "KeyedHasICBaseline Descriptor",
                Key::KeyedHasICWithVector => "KeyedHasICWithVector Descriptor",
                Key::KeyedLoad => "KeyedLoad Descriptor",
                Key::KeyedLoadBaseline => "KeyedLoadBaseline Descriptor",
                Key::EnumeratedKeyedLoadBaseline => "EnumeratedKeyedLoadBaseline Descriptor",
                Key::KeyedLoadWithVector => "KeyedLoadWithVector Descriptor",
                Key::EnumeratedKeyedLoad => "EnumeratedKeyedLoad Descriptor",
                Key::Load => "Load Descriptor",
                Key::LoadBaseline => "LoadBaseline Descriptor",
                Key::LoadGlobal => "LoadGlobal Descriptor",
                Key::LoadGlobalBaseline => "LoadGlobalBaseline Descriptor",
                Key::LoadGlobalNoFeedback => "LoadGlobalNoFeedback Descriptor",
                Key::LoadGlobalWithVector => "LoadGlobalWithVector Descriptor",
                Key::LoadNoFeedback => "LoadNoFeedback Descriptor",
                Key::LoadWithReceiverAnd
