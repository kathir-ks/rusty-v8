// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod js_operator {
    use std::{
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        mem::size_of,
        num::NonZeroU32,
    };

    //use crate::base::compiler_specific; // Assuming this is not needed for the header
    //use crate::codegen::tnode; // Assuming this is not needed for the header
    //use crate::compiler::common_operator; // Assuming this is not needed for the header
    //use crate::compiler::feedback_source; // Assuming this is not needed for the header
    //use crate::compiler::globals; // Assuming this is not needed for the header
    //use crate::compiler::node_properties; // Assuming this is not needed for the header
    //use crate::compiler::node; // Assuming this is not needed for the header
    //use crate::compiler::opcodes; // Assuming this is not needed for the header
    //use crate::compiler::operator_properties; // Assuming this is not needed for the header
    //use crate::objects::feedback_cell; // Assuming this is not needed for the header
    //use crate::objects::oddball; // Assuming this is not needed for the header
    //use crate::runtime::runtime; // Assuming this is not needed for the header

    //#[cfg(debug_assertions)]
    //use crate::wasm::canonical_types; // Assuming this is not needed for the header

    // These are placeholders - replace with actual Rust types
    pub type Object = u32; // Placeholder
    pub type HeapObject = u32; // Placeholder
    pub type Smi = i32; // Placeholder
    pub type FeedbackCell = u32; // Placeholder
    pub type SharedFunctionInfoRef = u32; // Placeholder
    pub type CodeRef = u32; // Placeholder
    pub type AllocationType = u8; // Placeholder
    pub type LanguageMode = u8;
    pub type NameRef = u32; // Placeholder
    pub type FeedbackSource = u32; // Placeholder
    pub type OptionalAllocationSiteRef = u32; // Placeholder
    pub type IterationKind = u8;
    pub type MapRef = u32; // Placeholder
    pub type TemplateObjectDescriptionRef = u32; // Placeholder
    pub type HeapObjectRef = u32; // Placeholder
    pub type ArrayBoilerplateDescriptionRef = u32; // Placeholder
    pub type ObjectBoilerplateDescriptionRef = u32; // Placeholder
    pub type TypeofMode = u8;
    pub type ScopeInfoRef = u32; // Placeholder
    pub type ScopeType = u8; // Placeholder
    pub type ConvertReceiverMode = u8;
    pub type SpeculationMode = u8;
    pub type CallFeedbackRelation = u8;
    pub type CreateArgumentsType = u8;
    pub type CollectionKind = u8;
    pub type StackCheckKind = u8;

    // wasm types
    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub mod wasm {
        pub type ValueType = u8;
        pub struct WasmModule {}
        pub struct CanonicalSig {}
        pub struct NativeModule {}
        pub fn GetTypeCanonicalizer() {}
    }

    pub struct AllStatic; // Placeholder for AllStatic class

    impl AllStatic {
        /// Predicates
        pub const fn is_unary_with_feedback(opcode: IrOpcode) -> bool {
            match opcode {
                IrOpcode::kBitwiseNot
                | IrOpcode::kDecrement
                | IrOpcode::kIncrement
                | IrOpcode::kNegate => true,
                _ => false,
            }
        }

        pub const fn is_binary_with_feedback(opcode: IrOpcode) -> bool {
            match opcode {
                IrOpcode::kEqual
                | IrOpcode::kStrictEqual
                | IrOpcode::kLessThan
                | IrOpcode::kGreaterThan
                | IrOpcode::kLessThanOrEqual
                | IrOpcode::kGreaterThanOrEqual
                | IrOpcode::kBitwiseOr
                | IrOpcode::kBitwiseXor
                | IrOpcode::kBitwiseAnd
                | IrOpcode::kShiftLeft
                | IrOpcode::kShiftRight
                | IrOpcode::kShiftRightLogical
                | IrOpcode::kAdd
                | IrOpcode::kSubtract
                | IrOpcode::kMultiply
                | IrOpcode::kDivide
                | IrOpcode::kModulus
                | IrOpcode::kExponentiate
                | IrOpcode::kJSInstanceOf => true,
                _ => false,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct CallFrequency {
        value_: f32,
    }

    impl CallFrequency {
        pub fn new() -> Self {
            CallFrequency {
                value_: f32::NAN,
            }
        }

        pub fn with_value(value: f32) -> Self {
            assert!(!value.is_nan());
            CallFrequency { value_: value }
        }

        pub fn is_known(&self) -> bool {
            !self.is_unknown()
        }

        pub fn is_unknown(&self) -> bool {
            self.value_.is_nan()
        }

        pub fn value(&self) -> f32 {
            assert!(self.is_known());
            self.value_
        }

        pub const K_NO_FEEDBACK_CALL_FREQUENCY: f32 = -1.0;
    }

    impl Eq for CallFrequency {}

    impl Hash for CallFrequency {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value_.to_bits().hash(state);
        }
    }

    impl fmt::Display for CallFrequency {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CallFrequency({})", self.value_)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ConstructForwardVarargsParameters {
        bit_field_: u32,
    }

    impl ConstructForwardVarargsParameters {
        pub fn new(arity: usize, start_index: u32) -> Self {
            ConstructForwardVarargsParameters {
                bit_field_: ArityField::encode(arity) | StartIndexField::encode(start_index),
            }
        }

        pub fn arity(&self) -> usize {
            ArityField::decode(self.bit_field_)
        }

        pub fn start_index(&self) -> u32 {
            StartIndexField::decode(self.bit_field_)
        }
    }

    impl fmt::Display for ConstructForwardVarargsParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "ConstructForwardVarargsParameters(arity={}, start_index={})",
                self.arity(),
                self.start_index()
            )
        }
    }

    mod ArityField {
        use std::mem::size_of;
        const OFFSET: usize = 0;
        const SIZE: usize = 16;
        pub fn encode(value: usize) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> usize {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as usize
        }
    }

    mod StartIndexField {
        use std::mem::size_of;
        const OFFSET: usize = 16;
        const SIZE: usize = 16;
        pub fn encode(value: u32) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> u32 {
            (bit_field >> OFFSET) & ((1 << SIZE) - 1) as u32
        }
    }

    pub fn construct_forward_varargs_parameters_of(
        op: *const Operator,
    ) -> &'static ConstructForwardVarargsParameters {
        unsafe { &(*(op as *const Operator)).parameter.construct_forward_varargs }
    }

    // Defines the arity (parameters plus the target and new target) and the
    // feedback for a JavaScript constructor call. This is used as a parameter by
    // JSConstruct, JSConstructWithArrayLike, and JSConstructWithSpread operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ConstructParameters {
        arity_: u32,
        frequency_: CallFrequency,
        feedback_: FeedbackSource,
    }

    impl ConstructParameters {
        // A separate declaration to get around circular declaration dependencies.
        // Checked to equal JSConstructNode::kExtraInputCount below.
        pub const K_EXTRA_CONSTRUCT_INPUT_COUNT: i32 = 3;

        pub fn new(arity: u32, frequency: CallFrequency, feedback: FeedbackSource) -> Self {
            assert!(arity >= ConstructParameters::K_EXTRA_CONSTRUCT_INPUT_COUNT as u32);
            // TODO: Add is_int32 check here
            ConstructParameters {
                arity_: arity,
                frequency_: frequency,
                feedback_: feedback,
            }
        }

        // TODO(jgruber): Consider removing `arity()` and just storing the arity
        // without extra args in ConstructParameters. Every spot that creates
        // ConstructParameters artifically adds the extra args. Every spot that uses
        // ConstructParameters artificially subtracts the extra args.
        // We keep them for now for consistency with other spots
        // that expect `arity()` to include extra args.
        pub fn arity(&self) -> u32 {
            self.arity_
        }

        pub fn arity_without_implicit_args(&self) -> i32 {
            (self.arity_ as i32) - ConstructParameters::K_EXTRA_CONSTRUCT_INPUT_COUNT
        }

        pub fn frequency(&self) -> CallFrequency {
            self.frequency_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for ConstructParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "ConstructParameters(arity={}, frequency={}, feedback={})",
                self.arity(),
                self.frequency(),
                self.feedback()
            )
        }
    }

    pub fn construct_parameters_of(op: *const Operator) -> &'static ConstructParameters {
        unsafe { &(*(op as *const Operator)).parameter.construct }
    }

    // Defines the flags for a JavaScript call forwarding parameters. This
    // is used as parameter by JSCallForwardVarargs operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CallForwardVarargsParameters {
        bit_field_: u32,
    }

    impl CallForwardVarargsParameters {
        pub fn new(arity: usize, start_index: u32) -> Self {
            CallForwardVarargsParameters {
                bit_field_: ArityField2::encode(arity) | StartIndexField2::encode(start_index),
            }
        }

        pub fn arity(&self) -> usize {
            ArityField2::decode(self.bit_field_)
        }

        pub fn start_index(&self) -> u32 {
            StartIndexField2::decode(self.bit_field_)
        }
    }

    impl fmt::Display for CallForwardVarargsParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CallForwardVarargsParameters(arity={}, start_index={})",
                self.arity(),
                self.start_index()
            )
        }
    }

    mod ArityField2 {
        use std::mem::size_of;
        const OFFSET: usize = 0;
        const SIZE: usize = 15;
        pub fn encode(value: usize) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> usize {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as usize
        }
    }

    mod StartIndexField2 {
        use std::mem::size_of;
        const OFFSET: usize = 15;
        const SIZE: usize = 15;
        pub fn encode(value: u32) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> u32 {
            (bit_field >> OFFSET) & ((1 << SIZE) - 1) as u32
        }
    }

    pub fn call_forward_varargs_parameters_of(
        op: *const Operator,
    ) -> &'static CallForwardVarargsParameters {
        unsafe { &(*(op as *const Operator)).parameter.call_forward_varargs }
    }

    // Defines the arity (parameters plus the target and receiver) and the call
    // flags for a JavaScript function call. This is used as a parameter by JSCall,
    // JSCallWithArrayLike and JSCallWithSpread operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CallParameters {
        bit_field_: u32,
        frequency_: CallFrequency,
        feedback_: FeedbackSource,
    }

    impl CallParameters {
        // A separate declaration to get around circular declaration dependencies.
        // Checked to equal JSCallNode::kExtraInputCount below.
        pub const K_EXTRA_CALL_INPUT_COUNT: i32 = 3;

        pub fn new(
            arity: usize,
            frequency: CallFrequency,
            feedback: FeedbackSource,
            convert_mode: ConvertReceiverMode,
            speculation_mode: SpeculationMode,
            feedback_relation: CallFeedbackRelation,
        ) -> Self {
            // CallFeedbackRelation is ignored if the feedback slot is invalid.
            // TODO: Add checks for feedback validity
            assert!(arity >= CallParameters::K_EXTRA_CALL_INPUT_COUNT as usize);
            // TODO: Add is_int32 check here
            let bit_field_ = ArityField3::encode(arity)
                | CallFeedbackRelationField::encode(feedback_relation)
                | SpeculationModeField::encode(speculation_mode)
                | ConvertReceiverModeField::encode(convert_mode);

            CallParameters {
                bit_field_: bit_field_,
                frequency_: frequency,
                feedback_: feedback,
            }
        }

        // TODO(jgruber): Consider removing `arity()` and just storing the arity
        // without extra args in CallParameters.
        pub fn arity(&self) -> usize {
            ArityField3::decode(self.bit_field_)
        }

        pub fn arity_without_implicit_args(&self) -> i32 {
            (self.arity() as i32) - CallParameters::K_EXTRA_CALL_INPUT_COUNT
        }

        pub fn frequency(&self) -> CallFrequency {
            self.frequency_
        }

        pub fn convert_mode(&self) -> ConvertReceiverMode {
            ConvertReceiverModeField::decode(self.bit_field_)
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }

        pub fn speculation_mode(&self) -> SpeculationMode {
            SpeculationModeField::decode(self.bit_field_)
        }

        pub fn feedback_relation(&self) -> CallFeedbackRelation {
            CallFeedbackRelationField::decode(self.bit_field_)
        }
    }

    impl fmt::Display for CallParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CallParameters(arity={}, frequency={}, feedback={}, convert_mode={}, speculation_mode={}, feedback_relation={})",
                self.arity(),
                self.frequency(),
                self.feedback(),
                self.convert_mode(),
                self.speculation_mode(),
                self.feedback_relation()
            )
        }
    }

    mod ArityField3 {
        use std::mem::size_of;
        const OFFSET: usize = 0;
        const SIZE: usize = 27;
        pub fn encode(value: usize) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> usize {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as usize
        }
    }

    mod CallFeedbackRelationField {
        const OFFSET: usize = 27;
        const SIZE: usize = 2;
        pub fn encode(value: CallFeedbackRelation) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> CallFeedbackRelation {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as CallFeedbackRelation
        }
    }

    mod SpeculationModeField {
        const OFFSET: usize = 29;
        const SIZE: usize = 1;
        pub fn encode(value: SpeculationMode) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> SpeculationMode {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as SpeculationMode
        }
    }

    mod ConvertReceiverModeField {
        const OFFSET: usize = 30;
        const SIZE: usize = 2;
        pub fn encode(value: ConvertReceiverMode) -> u32 {
            (value as u32) << OFFSET
        }
        pub fn decode(bit_field: u32) -> ConvertReceiverMode {
            ((bit_field >> OFFSET) & ((1 << SIZE) - 1)) as ConvertReceiverMode
        }
    }

    pub fn call_parameters_of(op: *const Operator) -> &'static CallParameters {
        unsafe { &(*(op as *const Operator)).parameter.call }
    }

    // Defines the arity and the ID for a runtime function call. This is used as a
    // parameter by JSCallRuntime operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CallRuntimeParameters {
        id_: RuntimeFunctionId,
        arity_: usize,
    }

    impl CallRuntimeParameters {
        pub fn new(id: RuntimeFunctionId, arity: usize) -> Self {
            CallRuntimeParameters { id_: id, arity_: arity }
        }

        pub fn id(&self) -> RuntimeFunctionId {
            self.id_
        }

        pub fn arity(&self) -> usize {
            self.arity_
        }
    }

    impl fmt::Display for CallRuntimeParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CallRuntimeParameters(id={:?}, arity={})",
                self.id(),
                self.arity()
            )
        }
    }

    pub fn call_runtime_parameters_of(op: *const Operator) -> &'static CallRuntimeParameters {
        unsafe { &(*(op as *const Operator)).parameter.call_runtime }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ContextAccess {
        immutable_: bool,
        depth_: u16,
        index_: u32,
    }

    impl ContextAccess {
        pub fn new(depth: usize, index: usize, immutable: bool) -> Self {
            ContextAccess {
                immutable_: immutable,
                depth_: depth as u16,
                index_: index as u32,
            }
        }

        pub fn depth(&self) -> usize {
            self.depth_ as usize
        }

        pub fn index(&self) -> usize {
            self.index_ as usize
        }

        pub fn immutable(&self) -> bool {
            self.immutable_
        }
    }

    impl fmt::Display for ContextAccess {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "ContextAccess(depth={}, index={}, immutable={})",
                self.depth(),
                self.index(),
                self.immutable()
            )
        }
    }

    pub fn context_access_of(op: *const Operator) -> &'static ContextAccess {
        unsafe { &(*(op as *const Operator)).parameter.context_access }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CreateFunctionContextParameters {
        scope_info_: ScopeInfoRef,
        slot_count_: i32,
        scope_type_: ScopeType,
    }

    impl CreateFunctionContextParameters {
        pub fn new(scope_info: ScopeInfoRef, slot_count: i32, scope_type: ScopeType) -> Self {
            CreateFunctionContextParameters {
                scope_info_: scope_info,
                slot_count_: slot_count,
                scope_type_: scope_type,
            }
        }

        pub fn scope_info(&self) -> ScopeInfoRef {
            self.scope_info_
        }

        pub fn slot_count(&self) -> i32 {
            self.slot_count_
        }

        pub fn scope_type(&self) -> ScopeType {
            self.scope_type_
        }
    }

    impl fmt::Display for CreateFunctionContextParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CreateFunctionContextParameters(scope_info={}, slot_count={}, scope_type={})",
                self.scope_info(),
                self.slot_count(),
                self.scope_type()
            )
        }
    }

    pub fn create_function_context_parameters_of(
        op: *const Operator,
    ) -> &'static CreateFunctionContextParameters {
        unsafe { &(*(op as *const Operator)).parameter.create_function_context }
    }

    // Defines parameters for JSDefineNamedOwnProperty operator.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DefineNamedOwnPropertyParameters {
        name_: NameRef,
        feedback_: FeedbackSource,
    }

    impl DefineNamedOwnPropertyParameters {
        pub fn new(name: NameRef, feedback: FeedbackSource) -> Self {
            DefineNamedOwnPropertyParameters {
                name_: name,
                feedback_: feedback,
            }
        }

        pub fn name(&self) -> NameRef {
            self.name_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for DefineNamedOwnPropertyParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "DefineNamedOwnPropertyParameters(name={}, feedback={})",
                self.name(),
                self.feedback()
            )
        }
    }

    pub fn define_named_own_property_parameters_of(
        op: *const Operator,
    ) -> &'static DefineNamedOwnPropertyParameters {
        unsafe { &(*(op as *const Operator)).parameter.define_named_own_property }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FeedbackParameter {
        feedback_: FeedbackSource,
    }

    impl FeedbackParameter {
        pub fn new(feedback: FeedbackSource) -> Self {
            FeedbackParameter { feedback_: feedback }
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for FeedbackParameter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "FeedbackParameter(feedback={})", self.feedback())
        }
    }

    pub fn feedback_parameter_of(op: *const Operator) -> &'static FeedbackParameter {
        unsafe { &(*(op as *const Operator)).parameter.feedback }
    }

    // Defines the property of an object for a named access. This is
    // used as a parameter by the JSLoadNamed and JSSetNamedProperty operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct NamedAccess {
        name_: NameRef,
        feedback_: FeedbackSource,
        language_mode_: LanguageMode,
    }

    impl NamedAccess {
        pub fn new(language_mode: LanguageMode, name: NameRef, feedback: FeedbackSource) -> Self {
            NamedAccess {
                name_: name,
                feedback_: feedback,
                language_mode_: language_mode,
            }
        }

        pub fn name(&self) -> NameRef {
            self.name_
        }

        pub fn language_mode(&self) -> LanguageMode {
            self.language_mode_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for NamedAccess {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "NamedAccess(language_mode={}, name={}, feedback={})",
                self.language_mode(),
                self.name(),
                self.feedback()
            )
        }
    }

    pub fn named_access_of(op: *const Operator) -> &'static NamedAccess {
        unsafe { &(*(op as *const Operator)).parameter.named_access }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct LoadGlobalParameters {
        name_: NameRef,
        feedback_: FeedbackSource,
        typeof_mode_: TypeofMode,
    }

    impl LoadGlobalParameters {
        pub fn new(name: NameRef, feedback: FeedbackSource, typeof_mode: TypeofMode) -> Self {
            LoadGlobalParameters {
                name_: name,
                feedback_: feedback,
                typeof_mode_: typeof_mode,
            }
        }

        pub fn name(&self) -> NameRef {
            self.name_
        }

        pub fn typeof_mode(&self) -> TypeofMode {
            self.typeof_mode_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for LoadGlobalParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "LoadGlobalParameters(name={}, feedback={}, typeof_mode={})",
                self.name(),
                self.feedback(),
                self.typeof_mode()
            )
        }
    }

    pub fn load_global_parameters_of(op: *const Operator) -> &'static LoadGlobalParameters {
        unsafe { &(*(op as *const Operator)).parameter.load_global }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct StoreGlobalParameters {
        language_mode_: LanguageMode,
        name_: NameRef,
        feedback_: FeedbackSource,
    }

    impl StoreGlobalParameters {
        pub fn new(language_mode: LanguageMode, feedback: FeedbackSource, name: NameRef) -> Self {
            StoreGlobalParameters {
                language_mode_: language_mode,
                name_: name,
                feedback_: feedback,
            }
        }

        pub fn language_mode(&self) -> LanguageMode {
            self.language_mode_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }

        pub fn name(&self) -> NameRef {
            self.name_
        }
    }

    impl fmt::Display for StoreGlobalParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "StoreGlobalParameters(language_mode={}, feedback={}, name={})",
                self.language_mode(),
                self.feedback(),
                self.name()
            )
        }
    }

    pub fn store_global_parameters_of(op: *const Operator) -> &'static StoreGlobalParameters {
        unsafe { &(*(op as *const Operator)).parameter.store_global }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PropertyAccess {
        feedback_: FeedbackSource,
        language_mode_: LanguageMode,
    }

    impl PropertyAccess {
        pub fn new(language_mode: LanguageMode, feedback: FeedbackSource) -> Self {
            PropertyAccess {
                feedback_: feedback,
                language_mode_: language_mode,
            }
        }

        pub fn language_mode(&self) -> LanguageMode {
            self.language_mode_
        }

        pub fn feedback(&self) -> FeedbackSource {
            self.feedback_
        }
    }

    impl fmt::Display for PropertyAccess {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "PropertyAccess(language_mode={}, feedback={})",
                self.language_mode(),
                self.feedback()
            )
        }
    }

    pub fn property_access_of(op: *const Operator) -> &'static PropertyAccess {
        unsafe { &(*(op as *const Operator)).parameter.property_access }
    }

    pub fn create_arguments_type_of(op: *const Operator) -> &'static CreateArgumentsType {
        unsafe { &(*(op as *const Operator)).parameter.create_arguments_type }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CreateArrayParameters {
        arity_: usize,
        site_: OptionalAllocationSiteRef,
    }

    impl CreateArrayParameters {
        pub fn new(arity: usize, site: OptionalAllocationSiteRef) -> Self {
            CreateArrayParameters {
                arity_: arity,
                site_: site,
            }
        }

        pub fn arity(&self) -> usize {
            self.arity_
        }

        pub fn site(&self) -> OptionalAllocationSiteRef {
            self.site_
        }
    }

    impl fmt::Display for CreateArrayParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CreateArrayParameters(arity={}, site={})", self.arity(), self.site())
        }
    }

    pub fn create_array_parameters_of(op: *const Operator) -> &'static CreateArrayParameters {
        unsafe { &(*(op as *const Operator)).parameter.create_array }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CreateArrayIteratorParameters {
        kind_: IterationKind,
    }

    impl CreateArrayIteratorParameters {
        pub fn new(kind: IterationKind) -> Self {
            CreateArrayIteratorParameters { kind_: kind }
        }

        pub fn kind(&self) -> IterationKind {
            self.kind_
        }
    }

    impl fmt::Display for CreateArrayIteratorParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CreateArrayIteratorParameters(kind={})", self.kind())
        }
    }

    pub fn create_array_iterator_parameters_of(
        op: *const Operator,
    ) -> &'static CreateArrayIteratorParameters {
        unsafe { &(*(op as *const Operator)).parameter.create_array_iterator }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CreateCollectionIteratorParameters {
        collection_kind_: CollectionKind,
        iteration_kind_: IterationKind,
    }

    impl CreateCollectionIteratorParameters {
        pub fn new(collection_kind: CollectionKind, iteration_kind: IterationKind) -> Self {
            assert!(
                !(collection_kind == CollectionKind::kSet && iteration_kind == IterationKind::kKeys)
            );

            CreateCollectionIteratorParameters {
                collection_kind_: collection_kind,
                iteration_kind_: iteration_kind,
            }
        }

        pub fn collection_kind(&self) -> CollectionKind {
            self.collection_kind_
        }

        pub fn iteration_kind(&self) -> IterationKind {
            self.iteration_kind_
        }
    }

    impl fmt::Display for CreateCollectionIteratorParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CreateCollectionIteratorParameters(collection_kind={}, iteration_kind={})",
                self.collection_kind(),
                self.iteration_kind()
            )
        }
    }

    pub fn create_collection_iterator_parameters_of(
        op: *const Operator,
    ) -> &'static CreateCollectionIteratorParameters {
        unsafe { &(*(op as *const Operator)).parameter.create_collection_iterator }
    }

    // Defines shared information for the bound function that should be created.
    // This is used as parameter by JSCreateBoundFunction operators.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CreateBoundFunctionParameters {
        arity_: usize,
        map_: MapRef,
    }

    impl CreateBoundFunctionParameters {
        pub fn new(arity: usize, map: MapRef) -> Self {
            CreateBoundFunctionParameters {
                arity_: arity,
                map_: map,
            }
        }

        pub fn arity(&self) -> usize {
            self.arity_
        }

        pub fn map(&self) -> MapRef {
            self.map_
        }
    }

    impl fmt::Display for CreateBoundFunctionParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "CreateBoundFunctionParameters(arity={}, map={})",
                self.arity(),
                self.map()
            )
        }
    }

    pub fn create_bound