// Converted from V8 C++ source files:
// Header: variables.h
// Implementation: variables.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ast::{AstRawString, ZoneObject};
use crate::compiler::turboshaft::operations::List;
use crate::init::bootstrapper::Scope;
use crate::strings::uri::V8;

pub enum VariableMode {
    kVar,
    kLet,
    kConst,
    kDynamic,
    kDynamicGlobal,
    kDynamicLocal,
    kSloppyVar,
    kModuleVar,
    kImport,
    kUsing,
    kAwaitUsing,
}

pub enum VariableKind {
    NORMAL_VARIABLE,
    THIS_VARIABLE,
    ARGUMENTS_VARIABLE,
    SLOPPY_FUNCTION_NAME_VARIABLE,
    PARAMETER_VARIABLE,
    SLOPPY_BLOCK_FUNCTION_VARIABLE,
}

pub enum InitializationFlag {
    kCreatedInitialized,
    kNeedsInitialization,
}

pub enum MaybeAssignedFlag {
    kNotAssigned,
    kMaybeAssigned,
}

pub enum IsStaticFlag {
    kNotStatic,
    kStatic,
}

pub enum VariableLocation {
    UNALLOCATED,
    PARAMETER,
    LOCAL,
    CONTEXT,
    LOOKUP,
    MODULE,
    REPL_GLOBAL,
}

const kNoSourcePosition: i32 = -1;

const kUncacheableHoleCheckBitmapIndex: u8 = 0;

pub struct Variable {
    scope_: *mut Scope,
    name_: *const AstRawString,
    local_if_not_shadowed_: *mut Variable,
    next_: *mut Variable,
    index_: i32,
    initializer_position_: i32,
    bit_field_: u16,
    hole_check_analysis_bit_field_: u16,
}

impl Variable {
    pub fn new(
        scope: *mut Scope,
        name: *const AstRawString,
        mode: VariableMode,
        kind: VariableKind,
        initialization_flag: InitializationFlag,
        maybe_assigned_flag: MaybeAssignedFlag,
        is_static_flag: IsStaticFlag,
    ) -> Variable {
        let maybe_assigned_flag_field = Self::MaybeAssignedFlagField::encode(maybe_assigned_flag as i32);
        let initialization_flag_field = Self::InitializationFlagField::encode(initialization_flag as i32);
        let variable_mode_field = Self::VariableModeField::encode(mode as i32);
        let is_used_field = Self::IsUsedField::encode(false);
        let force_context_allocation_bit = Self::ForceContextAllocationBit::encode(false);
        let location_field = Self::LocationField::encode(VariableLocation::UNALLOCATED as i32);
        let variable_kind_field = Self::VariableKindField::encode(kind as i32);
        let is_static_flag_field = Self::IsStaticFlagField::encode(is_static_flag as i32);

        let bit_field = (maybe_assigned_flag_field |
                     initialization_flag_field |
                     variable_mode_field |
                     is_used_field |
                     force_context_allocation_bit |
                     location_field |
                     variable_kind_field |
                     is_static_flag_field) as u16;

        let hole_check_bitmap_index_field = Self::HoleCheckBitmapIndexField::encode(kUncacheableHoleCheckBitmapIndex as i32);
        let force_hole_initialization_flag_field = Self::ForceHoleInitializationFlagField::encode(ForceHoleInitializationFlag::kHoleInitializationNotForced as i32);

        let hole_check_analysis_bit_field = (hole_check_bitmap_index_field |
                                            force_hole_initialization_flag_field) as u16;

        Variable {
            scope_: scope,
            name_: name,
            local_if_not_shadowed_: std::ptr::null_mut(),
            next_: std::ptr::null_mut(),
            index_: -1,
            initializer_position_: kNoSourcePosition,
            bit_field_: bit_field,
            hole_check_analysis_bit_field_: hole_check_analysis_bit_field,
        }
    }

    pub fn from_other(other: &Variable) -> Variable {
        Variable {
            scope_: other.scope_,
            name_: other.name_,
            local_if_not_shadowed_: std::ptr::null_mut(),
            next_: std::ptr::null_mut(),
            index_: other.index_,
            initializer_position_: other.initializer_position_,
            bit_field_: other.bit_field_,
            hole_check_analysis_bit_field_: other.hole_check_analysis_bit_field_,
        }
    }

    pub fn scope(&self) -> *mut Scope {
        self.scope_
    }

    pub fn set_scope(&mut self, scope: *mut Scope) {
        self.scope_ = scope;
    }

    pub fn name(&self) -> String {
       "name".to_string()
    }

    pub fn raw_name(&self) -> *const AstRawString {
        self.name_
    }

    pub fn mode(&self) -> VariableMode {
        let mode_int = Self::VariableModeField::decode(self.bit_field_ as i32);
        match mode_int {
            0 => VariableMode::kVar,
            1 => VariableMode::kLet,
            2 => VariableMode::kConst,
            3 => VariableMode::kDynamic,
            4 => VariableMode::kDynamicGlobal,
            5 => VariableMode::kDynamicLocal,
            6 => VariableMode::kSloppyVar,
            7 => VariableMode::kModuleVar,
            8 => VariableMode::kImport,
            9 => VariableMode::kUsing,
            10 => VariableMode::kAwaitUsing,
            _ => VariableMode::kVar,
        }
    }

    pub fn set_mode(&mut self, mode: VariableMode) {
        self.bit_field_ = Self::VariableModeField::update(self.bit_field_ as i32, mode as i32) as u16;
    }

    pub fn set_is_static_flag(&mut self, is_static_flag: IsStaticFlag) {
        self.bit_field_ = Self::IsStaticFlagField::update(self.bit_field_ as i32, is_static_flag as i32) as u16;
    }

    pub fn is_static_flag(&self) -> IsStaticFlag {
        let flag_int = Self::IsStaticFlagField::decode(self.bit_field_ as i32);
        match flag_int {
            0 => IsStaticFlag::kNotStatic,
            1 => IsStaticFlag::kStatic,
            _ => IsStaticFlag::kNotStatic,
        }
    }

    pub fn is_static(&self) -> bool {
        self.is_static_flag() == IsStaticFlag::kStatic
    }

    pub fn has_forced_context_allocation(&self) -> bool {
        Self::ForceContextAllocationBit::decode(self.bit_field_ as i32)
    }

    pub fn force_context_allocation(&mut self) {
        if !self.is_unallocated() && !self.is_context_slot() && !self.is_lookup_slot() && self.location() != VariableLocation::MODULE {
            return;
        }
        self.bit_field_ = Self::ForceContextAllocationBit::update(self.bit_field_ as i32, true) as u16;
    }

    pub fn is_used(&mut self) -> bool {
        Self::IsUsedField::decode(self.bit_field_ as i32)
    }

    pub fn set_is_used(&mut self) {
        self.bit_field_ = Self::IsUsedField::update(self.bit_field_ as i32, true) as u16;
    }

    pub fn maybe_assigned(&self) -> MaybeAssignedFlag {
        let flag_int = Self::MaybeAssignedFlagField::decode(self.bit_field_ as i32);
        match flag_int {
            0 => MaybeAssignedFlag::kNotAssigned,
            1 => MaybeAssignedFlag::kMaybeAssigned,
            _ => MaybeAssignedFlag::kNotAssigned,
        }
    }

    pub fn clear_maybe_assigned(&mut self) {
        self.bit_field_ = Self::MaybeAssignedFlagField::update(self.bit_field_ as i32, MaybeAssignedFlag::kNotAssigned as i32) as u16;
    }

   pub fn set_maybe_assigned(&mut self) {
        if self.is_immutable_lexical_variable_mode(self.mode()) {
            return;
        }
        if self.has_local_if_not_shadowed() {
            if self.maybe_assigned() == MaybeAssignedFlag::kNotAssigned {
               unsafe { (*self.local_if_not_shadowed_).SetMaybeAssigned(); }
            }
        }
        self.set_maybe_assigned_internal();
    }

    fn set_maybe_assigned_internal(&mut self) {
        self.bit_field_ = Self::MaybeAssignedFlagField::update(self.bit_field_ as i32, MaybeAssignedFlag::kMaybeAssigned as i32) as u16;
    }

    pub fn requires_brand_check(&self) -> bool {
        self.is_private_method_or_accessor_variable_mode(self.mode())
    }

    pub fn initializer_position(&self) -> i32 {
        self.initializer_position_
    }

    pub fn set_initializer_position(&mut self, pos: i32) {
        self.initializer_position_ = pos;
    }

    pub fn is_unallocated(&self) -> bool {
        self.location() == VariableLocation::UNALLOCATED
    }

    pub fn is_parameter(&self) -> bool {
        self.location() == VariableLocation::PARAMETER
    }

    pub fn is_stack_local(&self) -> bool {
        self.location() == VariableLocation::LOCAL
    }

    pub fn is_stack_allocated(&self) -> bool {
        self.is_parameter() || self.is_stack_local()
    }

    pub fn is_context_slot(&self) -> bool {
        self.location() == VariableLocation::CONTEXT
    }

    pub fn is_lookup_slot(&self) -> bool {
        self.location() == VariableLocation::LOOKUP
    }

    pub fn is_global_object_property(&self) -> bool {
        (self.is_dynamic_variable_mode(self.mode()) || self.mode() == VariableMode::kVar)
            && self.scope_ != std::ptr::null_mut()
    }

    pub fn is_repl_global(&self) -> bool {
        false
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic_variable_mode(self.mode())
    }

    pub fn initialization_flag(&self) -> InitializationFlag {
        let flag_int = Self::InitializationFlagField::decode(self.bit_field_ as i32);
        match flag_int {
            0 => InitializationFlag::kCreatedInitialized,
            1 => InitializationFlag::kNeedsInitialization,
            _ => InitializationFlag::kCreatedInitialized,
        }
    }

    pub fn binding_needs_init(&self) -> bool {
       if self.is_hole_initialization_forced() {
            return true;
        }
        if self.is_stack_allocated() {
            return false;
        }
        self.initialization_flag() == InitializationFlag::kNeedsInitialization
    }

    pub fn force_hole_initialization_flag_field(&self) -> ForceHoleInitializationFlag {
        let flag_int = Self::ForceHoleInitializationFlagField::decode(self.hole_check_analysis_bit_field_ as i32);
          match flag_int {
            0 => ForceHoleInitializationFlag::kHoleInitializationNotForced,
            1 => ForceHoleInitializationFlag::kHasHoleCheckUseInDifferentClosureScope,
            2 => ForceHoleInitializationFlag::kHasHoleCheckUseInSameClosureScope,
            3 => ForceHoleInitializationFlag::kHasHoleCheckUseInUnknownScope,
            _ => ForceHoleInitializationFlag::kHoleInitializationNotForced,
        }
    }

    pub fn is_hole_initialization_forced(&self) -> bool {
        self.force_hole_initialization_flag_field() != ForceHoleInitializationFlag::kHoleInitializationNotForced
    }

    pub fn has_hole_check_use_in_same_closure_scope(&self) -> bool {
        self.force_hole_initialization_flag_field() as i32 & ForceHoleInitializationFlag::kHasHoleCheckUseInSameClosureScope as i32 != 0
    }

    pub fn force_hole_initialization(&mut self, flag: ForceHoleInitializationFlag) {
        if self.initialization_flag() != InitializationFlag::kNeedsInitialization {
            return;
        }

        self.hole_check_analysis_bit_field_ |= Self::ForceHoleInitializationFlagField::encode(flag as i32) as u16;
    }

    pub fn reset_hole_check_bitmap_index(&mut self) {
        self.hole_check_analysis_bit_field_ = Self::HoleCheckBitmapIndexField::update(
            self.hole_check_analysis_bit_field_ as i32,
            kUncacheableHoleCheckBitmapIndex as i32,
        ) as u16;
    }

   pub fn remember_hole_check_in_bitmap(
        &mut self,
        bitmap: &mut u64,
        list: &mut Vec<*mut Variable>,
    ) {
        let index = self.HoleCheckBitmapIndex();
        if index == kUncacheableHoleCheckBitmapIndex {
            let mut index = list.len() as u8 + 1;
            if index == Self::kHoleCheckBitmapBits as u8 {
                return;
            }
            self.assign_hole_check_bitmap_index(list, index);
        }
        *bitmap |= 1 << self.HoleCheckBitmapIndex();
    }

    pub fn has_remembered_hole_check(&self, bitmap: u64) -> bool {
        let index = self.HoleCheckBitmapIndex();
        let result = bitmap & (1 << index) != 0;
        !result
    }

    pub fn throw_on_const_assignment(&self, language_mode: i32) -> bool {
        self.kind() != VariableKind::SLOPPY_FUNCTION_NAME_VARIABLE || language_mode != 0
    }

    pub fn is_this(&self) -> bool {
        self.kind() == VariableKind::THIS_VARIABLE
    }

    pub fn is_sloppy_function_name(&self) -> bool {
        self.kind() == VariableKind::SLOPPY_FUNCTION_NAME_VARIABLE
    }

    pub fn is_parameter(&self) -> bool {
        self.kind() == VariableKind::PARAMETER_VARIABLE
    }

    pub fn is_sloppy_block_function(&self) -> bool {
        self.kind() == VariableKind::SLOPPY_BLOCK_FUNCTION_VARIABLE
    }

    pub fn local_if_not_shadowed(&self) -> *mut Variable {
        self.local_if_not_shadowed_
    }

    pub fn has_local_if_not_shadowed(&self) -> bool {
        self.local_if_not_shadowed_ != std::ptr::null_mut()
    }

    pub fn set_local_if_not_shadowed(&mut self, local: *mut Variable) {
        self.local_if_not_shadowed_ = local;
    }

    pub fn location(&self) -> VariableLocation {
        let location_int = Self::LocationField::decode(self.bit_field_ as i32);
        match location_int {
            0 => VariableLocation::UNALLOCATED,
            1 => VariableLocation::PARAMETER,
            2 => VariableLocation::LOCAL,
            3 => VariableLocation::CONTEXT,
            4 => VariableLocation::LOOKUP,
            5 => VariableLocation::MODULE,
            6 => VariableLocation::REPL_GLOBAL,
            _ => VariableLocation::UNALLOCATED,
        }
    }

    pub fn kind(&self) -> VariableKind {
        let kind_int = Self::VariableKindField::decode(self.bit_field_ as i32);
        match kind_int {
            0 => VariableKind::NORMAL_VARIABLE,
            1 => VariableKind::THIS_VARIABLE,
            2 => VariableKind::ARGUMENTS_VARIABLE,
            3 => VariableKind::SLOPPY_FUNCTION_NAME_VARIABLE,
            4 => VariableKind::PARAMETER_VARIABLE,
            5 => VariableKind::SLOPPY_BLOCK_FUNCTION_VARIABLE,
            _ => VariableKind::NORMAL_VARIABLE,
        }
    }

    pub fn index(&self) -> i32 {
        self.index_
    }

    pub fn is_receiver(&self) -> bool {
       self.is_parameter() && self.index_ == -1
    }

    pub fn is_export(&self) -> bool {
        self.location() == VariableLocation::MODULE && self.index() != 0 && self.index() > 0
    }

    pub fn allocate_to(&mut self, location: VariableLocation, index: i32) {
        if !self.is_unallocated() && !(self.location() == location && self.index() == index) {
            return;
        }
        self.bit_field_ = Self::LocationField::update(self.bit_field_ as i32, location as i32) as u16;
        self.index_ = index;
    }

    pub fn make_parameter_non_simple(&mut self) {
        if !self.is_parameter() {
            return;
        }
        self.bit_field_ = Self::VariableModeField::update(self.bit_field_ as i32, VariableMode::kLet as i32) as u16;
        self.bit_field_ = Self::InitializationFlagField::update(self.bit_field_ as i32, InitializationFlag::kNeedsInitialization as i32) as u16;
    }

    pub fn default_initialization_flag(mode: VariableMode) -> InitializationFlag {
         match mode {
            VariableMode::kVar => InitializationFlag::kCreatedInitialized,
            _ => InitializationFlag::kNeedsInitialization,
        }
    }

    pub fn rewrite_location_for_repl(&mut self) {
        if self.mode() == VariableMode::kLet || self.mode() == VariableMode::kConst ||
           self.mode() == VariableMode::kUsing || self.mode() == VariableMode::kAwaitUsing {
            if self.location() != VariableLocation::CONTEXT {
                return;
            }
            self.bit_field_ = Self::LocationField::update(self.bit_field_ as i32, VariableLocation::REPL_GLOBAL as i32) as u16;
        }
    }

    fn HoleCheckBitmapIndex(&self) -> u8 {
        Self::HoleCheckBitmapIndexField::decode(self.hole_check_analysis_bit_field_ as i32) as u8
    }

    fn assign_hole_check_bitmap_index(
        &mut self,
        list: &mut Vec<*mut Variable>,
        next_index: u8,
    ) {
        self.hole_check_analysis_bit_field_ = Self::HoleCheckBitmapIndexField::update(
            self.hole_check_analysis_bit_field_ as i32,
            next_index as i32,
        ) as u16;
        list.push(self);
    }

    fn is_dynamic_variable_mode(&self, mode: VariableMode) -> bool {
         match mode {
            VariableMode::kDynamic | VariableMode::kDynamicGlobal | VariableMode::kDynamicLocal => true,
            _ => false,
        }
    }

   fn is_immutable_lexical_variable_mode(&self, mode: VariableMode) -> bool {
         match mode {
            VariableMode::kConst | VariableMode::kImport => true,
            _ => false,
        }
    }

    fn is_private_method_or_accessor_variable_mode(&self, mode: VariableMode) -> bool {
        false
    }
}

impl Variable {
    struct VariableModeField {}
    impl VariableModeField {
        const OFFSET: i32 = 0;
        const WIDTH: i32 = 4;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct VariableKindField {}
    impl VariableKindField {
        const OFFSET: i32 = 4;
        const WIDTH: i32 = 3;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct LocationField {}
    impl LocationField {
        const OFFSET: i32 = 7;
        const WIDTH: i32 = 3;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct ForceContextAllocationBit {}
    impl ForceContextAllocationBit {
        const OFFSET: i32 = 10;
        const WIDTH: i32 = 1;
        fn encode(value: bool) -> i32 {
            (if value { 1 } else { 0 }) << Self::OFFSET
        }
        fn decode(bits: i32) -> bool {
            ((bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)) != 0
        }
        fn update(bits: i32, value: bool) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct IsUsedField {}
    impl IsUsedField {
        const OFFSET: i32 = 11;
        const WIDTH: i32 = 1;
        fn encode(value: bool) -> i32 {
            (if value { 1 } else { 0 }) << Self::OFFSET
        }
        fn decode(bits: i32) -> bool {
            ((bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)) != 0
        }
        fn update(bits: i32, value: bool) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct InitializationFlagField {}
    impl InitializationFlagField {
        const OFFSET: i32 = 12;
        const WIDTH: i32 = 1;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
        struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct MaybeAssignedFlagField {}
    impl MaybeAssignedFlagField {
        const OFFSET: i32 = 13;
        const WIDTH: i32 = 1;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
         struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct IsStaticFlagField {}
    impl IsStaticFlagField {
        const OFFSET: i32 = 14;
        const WIDTH: i32 = 1;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
         struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct HoleCheckBitmapIndexField {}
    impl HoleCheckBitmapIndexField {
        const OFFSET: i32 = 0;
        const WIDTH: i32 = 8;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
         struct Next {}
        impl Next {
            fn encode<T>(value: T) -> T {
                value
            }
        }
    }

    struct ForceHoleInitializationFlagField {}
    impl ForceHoleInitializationFlagField {
        const OFFSET: i32 = 8;
        const WIDTH: i32 = 2;
        fn encode(value: i32) -> i32 {
            (value & ((1 << Self::WIDTH) - 1)) << Self::OFFSET
        }
        fn decode(bits: i32) -> i32 {
            (bits >> Self::OFFSET) & ((1 << Self::WIDTH) - 1)
        }
        fn update(bits: i32, value: i32) -> i32 {
            let mask: i32 = ((1 << Self::WIDTH) - 1) << Self::OFFSET;
            (bits & !mask) | Self::encode(value)
        }
    }

    const kHoleCheckBitmapBits: usize = 64;
}

pub enum ForceHoleInitializationFlag {
    kHoleInitializationNotForced = 0,
    kHasHoleCheckUseInDifferentClosureScope = 1 << 0,
    kHasHoleCheckUseInSameClosureScope = 1 << 1,
    kHasHoleCheckUseInUnknownScope = kHasHoleCheckUseInDifferentClosureScope as isize |
                                     kHasHoleCheckUseInSameClosureScope as isize
                                         as isize,
}
