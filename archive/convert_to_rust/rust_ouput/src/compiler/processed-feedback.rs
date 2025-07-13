// Converted from V8 C++ source files:
// Header: processed-feedback.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_refs {
    // Define a dummy type for now.  Need to link to the correct definition.
    pub struct HeapObjectRef {}
    pub struct ContextRef {}
    pub struct PropertyCellRef {}
    pub struct OptionalObjectRef {}
    pub struct JSHeapBroker {}
    pub struct MapRef {}
    pub struct NameRef {}
    pub struct FunctionTemplateInfoRef {}
    pub struct OptionalHeapObjectRef {}
    pub struct JSObjectRef {}
    pub struct OptionalJSObjectRef {}
    pub struct AllocationSiteRef {}
    pub struct RegExpBoilerplateDescriptionRef {}
    pub struct JSArrayRef {}

    // Implement methods if needed, even if they are empty
    impl HeapObjectRef {
        pub fn new() -> Self {
            HeapObjectRef {}
        }
    }
    impl ContextRef {
         pub fn new() -> Self {
            ContextRef {}
        }
    }
    impl PropertyCellRef {
        pub fn new() -> Self {
            PropertyCellRef {}
        }
    }
    impl OptionalObjectRef {
        pub fn new() -> Self {
            OptionalObjectRef {}
        }
    }
    impl JSHeapBroker {
         pub fn new() -> Self {
            JSHeapBroker {}
        }
    }
    impl MapRef {
         pub fn new() -> Self {
            MapRef {}
        }
    }
    impl NameRef {
         pub fn new() -> Self {
            NameRef {}
        }
    }
    impl FunctionTemplateInfoRef {
         pub fn new() -> Self {
            FunctionTemplateInfoRef {}
        }
    }
    impl OptionalHeapObjectRef {
        pub fn new() -> Self {
            OptionalHeapObjectRef {}
        }
    }
     impl JSObjectRef {
         pub fn new() -> Self {
            JSObjectRef {}
        }
    }
    impl OptionalJSObjectRef {
         pub fn new() -> Self {
            OptionalJSObjectRef {}
        }
    }
    impl AllocationSiteRef {
         pub fn new() -> Self {
            AllocationSiteRef {}
        }
    }
    impl RegExpBoilerplateDescriptionRef {
         pub fn new() -> Self {
            RegExpBoilerplateDescriptionRef {}
        }
    }
    impl JSArrayRef {
         pub fn new() -> Self {
            JSArrayRef {}
        }
    }
}

pub mod processed_feedback {
    use super::heap_refs::*;
    use std::fmt;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Kind {
        kInsufficient,
        kBinaryOperation,
        kCall,
        kCompareOperation,
        kElementAccess,
        kForIn,
        kGlobalAccess,
        kInstanceOf,
        kTypeOf,
        kLiteral,
        kMegaDOMPropertyAccess,
        kNamedAccess,
        kRegExpLiteral,
        kTemplateObject,
    }

    impl fmt::Display for Kind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FeedbackSlotKind {
        kBinaryOp,
        kTypeOf,
        kCompareOp,
        kForIn,
        kInstanceOf,
        kLiteral,
    }

    #[derive(Debug, Clone)]
    pub struct ProcessedFeedback {
        kind_: Kind,
        slot_kind_: FeedbackSlotKind,
    }

    impl ProcessedFeedback {
        pub fn kind(&self) -> Kind {
            self.kind_
        }

        pub fn slot_kind(&self) -> FeedbackSlotKind {
            self.slot_kind_
        }

        pub fn is_insufficient(&self) -> bool {
            self.kind() == Kind::kInsufficient
        }

        pub fn as_binary_operation(&self) -> &BinaryOperationFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kBinaryOperation, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const BinaryOperationFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_type_of(&self) -> &TypeOfOpFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kTypeOf, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const TypeOfOpFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_call(&self) -> &CallFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kCall, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const CallFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_compare_operation(&self) -> &CompareOperationFeedback {
             match self {
                ProcessedFeedback { kind_: Kind::kCompareOperation, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const CompareOperationFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_element_access(&self) -> &ElementAccessFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kElementAccess, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const ElementAccessFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_for_in(&self) -> &ForInFeedback {
             match self {
                ProcessedFeedback { kind_: Kind::kForIn, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const ForInFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_global_access(&self) -> &GlobalAccessFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kGlobalAccess, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const GlobalAccessFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_instance_of(&self) -> &InstanceOfFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kInstanceOf, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const InstanceOfFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_named_access(&self) -> &NamedAccessFeedback {
             match self {
                ProcessedFeedback { kind_: Kind::kNamedAccess, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const NamedAccessFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_mega_dom_property_access(&self) -> &MegaDOMPropertyAccessFeedback {
             match self {
                ProcessedFeedback { kind_: Kind::kMegaDOMPropertyAccess, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const MegaDOMPropertyAccessFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_literal(&self) -> &LiteralFeedback {
             match self {
                ProcessedFeedback { kind_: Kind::kLiteral, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const LiteralFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_reg_exp_literal(&self) -> &RegExpLiteralFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kRegExpLiteral, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const RegExpLiteralFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn as_template_object(&self) -> &TemplateObjectFeedback {
            match self {
                ProcessedFeedback { kind_: Kind::kTemplateObject, .. } => {
                    unsafe { &*(self as *const ProcessedFeedback as *const TemplateObjectFeedback) }
                },
                _ => panic!("Incorrect cast"),
            }
        }

        pub fn new(kind: Kind, slot_kind: FeedbackSlotKind) -> Self {
            ProcessedFeedback {
                kind_: kind,
                slot_kind_: slot_kind,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct InsufficientFeedback {
        base: ProcessedFeedback,
    }

    impl InsufficientFeedback {
        pub fn new(slot_kind: FeedbackSlotKind) -> Self {
            InsufficientFeedback {
                base: ProcessedFeedback::new(Kind::kInsufficient, slot_kind),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct GlobalAccessFeedback {
        base: ProcessedFeedback,
        cell_or_context_: OptionalObjectRef,
        index_and_immutable_: i32,
    }

    impl GlobalAccessFeedback {
        pub fn new(cell: PropertyCellRef, slot_kind: FeedbackSlotKind) -> Self {
            GlobalAccessFeedback {
                base: ProcessedFeedback::new(Kind::kGlobalAccess, slot_kind),
                cell_or_context_: OptionalObjectRef::new(),
                index_and_immutable_: 0,
            }
        }

        pub fn new_script_context(script_context: ContextRef, slot_index: i32, immutable: bool, slot_kind: FeedbackSlotKind) -> Self {
            GlobalAccessFeedback {
                base: ProcessedFeedback::new(Kind::kGlobalAccess, slot_kind),
                cell_or_context_: OptionalObjectRef::new(),
                index_and_immutable_: 0,
            }
        }

        pub fn new_megamorphic(slot_kind: FeedbackSlotKind) -> Self {
            GlobalAccessFeedback {
                base: ProcessedFeedback::new(Kind::kGlobalAccess, slot_kind),
                cell_or_context_: OptionalObjectRef::new(),
                index_and_immutable_: 0,
            }
        }

        pub fn is_megamorphic(&self) -> bool {
            false
        }

        pub fn is_property_cell(&self) -> bool {
            false
        }

        pub fn property_cell(&self) -> PropertyCellRef {
            PropertyCellRef::new()
        }

        pub fn is_script_context_slot(&self) -> bool {
            false
        }

        pub fn script_context(&self) -> ContextRef {
            ContextRef::new()
        }

        pub fn slot_index(&self) -> i32 {
            0
        }

        pub fn immutable(&self) -> bool {
            false
        }

        pub fn get_constant_hint(&self, _broker: &JSHeapBroker) -> OptionalObjectRef {
            OptionalObjectRef::new()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AccessMode {
        kLoad,
        kStore,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KeyedAccessLoadMode {
        kGeneric,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KeyedAccessStoreMode {
        kGeneric,
    }

    #[derive(Debug, Clone)]
    pub struct KeyedAccessMode {
        access_mode_: AccessMode,
        load_store_mode_: LoadStoreMode,
    }

    #[derive(Debug, Clone, Copy)]
    union LoadStoreMode {
        load_mode: KeyedAccessLoadMode,
        store_mode: KeyedAccessStoreMode,
    }

    impl KeyedAccessMode {
        pub fn from_nexus(_nexus: &FeedbackNexus) -> Self {
            KeyedAccessMode {
                access_mode_: AccessMode::kLoad,
                load_store_mode_: LoadStoreMode { load_mode: KeyedAccessLoadMode::kGeneric },
            }
        }

        pub fn access_mode(&self) -> AccessMode {
            self.access_mode_
        }

        pub fn is_load(&self) -> bool {
            self.access_mode_ == AccessMode::kLoad
        }

        pub fn is_store(&self) -> bool {
            self.access_mode_ == AccessMode::kStore
        }

        pub fn load_mode(&self) -> KeyedAccessLoadMode {
            unsafe { self.load_store_mode_.load_mode }
        }

        pub fn store_mode(&self) -> KeyedAccessStoreMode {
            unsafe { self.load_store_mode_.store_mode }
        }

        fn new_load(access_mode: AccessMode, load_mode: KeyedAccessLoadMode) -> Self {
            KeyedAccessMode {
                access_mode_: access_mode,
                load_store_mode_: LoadStoreMode { load_mode: load_mode },
            }
        }

        fn new_store(access_mode: AccessMode, store_mode: KeyedAccessStoreMode) -> Self {
            KeyedAccessMode {
                access_mode_: access_mode,
                load_store_mode_: LoadStoreMode { store_mode: store_mode },
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ElementAccessFeedback {
        base: ProcessedFeedback,
        keyed_mode_: KeyedAccessMode,
        transition_groups_: Vec<TransitionGroup>,
    }

    pub type TransitionGroup = Vec<MapRef>;

    impl ElementAccessFeedback {
        pub fn new(_zone: &Zone, keyed_mode: &KeyedAccessMode, slot_kind: FeedbackSlotKind) -> Self {
            ElementAccessFeedback {
                base: ProcessedFeedback::new(Kind::kElementAccess, slot_kind),
                keyed_mode_: keyed_mode.clone(),
                transition_groups_: Vec::new(),
            }
        }

        pub fn keyed_mode(&self) -> KeyedAccessMode {
            self.keyed_mode_.clone()
        }

        pub fn transition_groups(&self) -> &Vec<TransitionGroup> {
            &self.transition_groups_
        }

        pub fn has_only_string_maps(&self, _broker: &JSHeapBroker) -> bool {
            false
        }

        pub fn add_group(&mut self, group: TransitionGroup) {
            self.transition_groups_.push(group);
        }

        pub fn refine(
            &self,
            _broker: &JSHeapBroker,
            _inferred_maps: &Vec<MapRef>,
        ) -> &ElementAccessFeedback {
            self
        }

        pub fn refine_ref_set(
            &self,
            _broker: &JSHeapBroker,
            _inferred_maps: &ZoneRefSet<Map>,
            _always_keep_group_target: bool,
        ) -> &ElementAccessFeedback {
            self
        }

        pub fn refine_named(
            &self,
            _broker: &JSHeapBroker,
            _name: NameRef,
        ) -> &NamedAccessFeedback {
            unsafe { &*(self as *const ElementAccessFeedback as *const NamedAccessFeedback) }
        }
    }

    #[derive(Debug, Clone)]
    pub struct NamedAccessFeedback {
        base: ProcessedFeedback,
        name_: NameRef,
        maps_: Vec<MapRef>,
        has_deprecated_map_without_migration_target_: bool,
    }

    impl NamedAccessFeedback {
        pub fn new(
            name: NameRef,
            maps: Vec<MapRef>,
            slot_kind: FeedbackSlotKind,
            has_deprecated_map_without_migration_target: bool,
        ) -> Self {
            NamedAccessFeedback {
                base: ProcessedFeedback::new(Kind::kNamedAccess, slot_kind),
                name_: name,
                maps_: maps,
                has_deprecated_map_without_migration_target_: has_deprecated_map_without_migration_target,
            }
        }

        pub fn name(&self) -> NameRef {
            self.name_
        }

        pub fn maps(&self) -> &Vec<MapRef> {
            &self.maps_
        }

        pub fn has_deprecated_map_without_migration_target(&self) -> bool {
            self.has_deprecated_map_without_migration_target_
        }
    }

    #[derive(Debug, Clone)]
    pub struct MegaDOMPropertyAccessFeedback {
        base: ProcessedFeedback,
        info_: FunctionTemplateInfoRef,
    }

    impl MegaDOMPropertyAccessFeedback {
        pub fn new(info_ref: FunctionTemplateInfoRef, slot_kind: FeedbackSlotKind) -> Self {
            MegaDOMPropertyAccessFeedback {
                base: ProcessedFeedback::new(Kind::kMegaDOMPropertyAccess, slot_kind),
                info_: info_ref,
            }
        }

        pub fn info(&self) -> FunctionTemplateInfoRef {
            self.info_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SpeculationMode {
        kEager,
        kAllowSpeculation,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CallFeedbackContent {
        kGeneric,
    }

    #[derive(Debug, Clone)]
    pub struct CallFeedback {
        base: ProcessedFeedback,
        target_: OptionalHeapObjectRef,
        frequency_: f32,
        mode_: SpeculationMode,
        content_: CallFeedbackContent,
    }

    impl CallFeedback {
        pub fn new(
            target: OptionalHeapObjectRef,
            frequency: f32,
            mode: SpeculationMode,
            call_feedback_content: CallFeedbackContent,
            slot_kind: FeedbackSlotKind,
        ) -> Self {
            CallFeedback {
                base: ProcessedFeedback::new(Kind::kCall, slot_kind),
                target_: target,
                frequency_: frequency,
                mode_: mode,
                content_: call_feedback_content,
            }
        }

        pub fn target(&self) -> OptionalHeapObjectRef {
            self.target_
        }

        pub fn frequency(&self) -> f32 {
            self.frequency_
        }

        pub fn speculation_mode(&self) -> SpeculationMode {
            self.mode_
        }

        pub fn call_feedback_content(&self) -> CallFeedbackContent {
            self.content_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BinaryOperationHint {}

    #[derive(Debug, Clone)]
    pub struct BinaryOperationFeedback {
        base: ProcessedFeedback,
        value_: BinaryOperationHint,
    }

    impl BinaryOperationFeedback {
        pub fn new(value: BinaryOperationHint, slot_kind: FeedbackSlotKind) -> Self {
             BinaryOperationFeedback {
                base: ProcessedFeedback::new(Kind::kBinaryOperation, slot_kind),
                value_: value,
             }
        }

        pub fn value(&self) -> BinaryOperationHint {
            self.value_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CompareOperationHint {}

    #[derive(Debug, Clone)]
    pub struct CompareOperationFeedback {
        base: ProcessedFeedback,
        value_: CompareOperationHint,
    }

    impl CompareOperationFeedback {
        pub fn new(value: CompareOperationHint, slot_kind: FeedbackSlotKind) -> Self {
            CompareOperationFeedback {
                base: ProcessedFeedback::new(Kind::kCompareOperation, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> CompareOperationHint {
            self.value_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ForInHint {}

    #[derive(Debug, Clone)]
    pub struct ForInFeedback {
        base: ProcessedFeedback,
        value_: ForInHint,
    }

    impl ForInFeedback {
        pub fn new(value: ForInHint, slot_kind: FeedbackSlotKind) -> Self {
            ForInFeedback {
                base: ProcessedFeedback::new(Kind::kForIn, slot_kind),
                value_: value,
            }
        }
        pub fn value(&self) -> ForInHint {
            self.value_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TypeOfFeedback {
        kGeneric,
    }

    impl TypeOfFeedback {
        pub enum Result {
            kString,
        }
    }

    #[derive(Debug, Clone)]
    pub struct TypeOfOpFeedback {
        base: ProcessedFeedback,
        value_: TypeOfFeedback::Result,
    }

    impl TypeOfOpFeedback {
        pub fn new(value: TypeOfFeedback::Result, slot_kind: FeedbackSlotKind) -> Self {
            TypeOfOpFeedback {
                base: ProcessedFeedback::new(Kind::kTypeOf, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> TypeOfFeedback::Result {
            self.value_
        }
    }

    #[derive(Debug, Clone)]
    pub struct InstanceOfFeedback {
        base: ProcessedFeedback,
        value_: OptionalJSObjectRef,
    }

    impl InstanceOfFeedback {
        pub fn new(value: OptionalJSObjectRef, slot_kind: FeedbackSlotKind) -> Self {
            InstanceOfFeedback {
                base: ProcessedFeedback::new(Kind::kInstanceOf, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> OptionalJSObjectRef {
            self.value_
        }
    }

    #[derive(Debug, Clone)]
    pub struct LiteralFeedback {
        base: ProcessedFeedback,
        value_: AllocationSiteRef,
    }

    impl LiteralFeedback {
        pub fn new(value: AllocationSiteRef, slot_kind: FeedbackSlotKind) -> Self {
            LiteralFeedback {
                base: ProcessedFeedback::new(Kind::kLiteral, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> AllocationSiteRef {
            self.value_
        }
    }

    #[derive(Debug, Clone)]
    pub struct RegExpLiteralFeedback {
        base: ProcessedFeedback,
        value_: RegExpBoilerplateDescriptionRef,
    }

    impl RegExpLiteralFeedback {
        pub fn new(value: RegExpBoilerplateDescriptionRef, slot_kind: FeedbackSlotKind) -> Self {
            RegExpLiteralFeedback {
                base: ProcessedFeedback::new(Kind::kRegExpLiteral, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> RegExpBoilerplateDescriptionRef {
            self.value_
        }
    }

    #[derive(Debug, Clone)]
    pub struct TemplateObjectFeedback {
        base: ProcessedFeedback,
        value_: JSArrayRef,
    }

    impl TemplateObjectFeedback {
        pub fn new(value: JSArrayRef, slot_kind: FeedbackSlotKind) -> Self {
            TemplateObjectFeedback {
                base: ProcessedFeedback::new(Kind::kTemplateObject, slot_kind),
                value_: value,
            }
        }

        pub fn value(&self) -> JSArrayRef {
            self.value_
        }
    }

    pub struct Zone {}
    pub struct FeedbackNexus {}
    pub struct ZoneRefSet<T> {}
    pub struct Map {}
    pub struct ZoneVector<T> {
        data: Vec<T>,
    }

    impl<T> ZoneVector<T> {
        pub fn new() -> Self {
            ZoneVector { data: Vec::new() }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
    }
}
