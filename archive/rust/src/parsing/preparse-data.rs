// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod preparse_data {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::ast::scopes::*;
    use crate::ast::variables::*;
    use crate::base::logging::*;
    //use crate::handles::handles::*; // Assuming handles are managed by the VM
    //use crate::objects::objects_inl::*; // Assuming objects are managed by the VM
    //use crate::objects::shared_function_info::*; // Assuming objects are managed by the VM
    use crate::parsing::parser::*;
    use crate::parsing::preparse_data_impl::*;
    use crate::parsing::preparser::*;
    //use crate::roots::roots::*; // Assuming roots are managed by the VM
    use crate::zone::zone_list_inl::*;
    use crate::zone::zone_utils::*;

    mod base {
        pub mod logging {
            #[macro_export]
            macro_rules! CHECK {
                ($cond:expr) => {
                    if !$cond {
                        panic!("Check failed: {}", stringify!($cond));
                    }
                };
            }
            #[macro_export]
            macro_rules! CHECK_EQ {
                ($left:expr, $right:expr) => {
                    if $left != $right {
                        panic!("Check failed: {} == {}", stringify!($left), stringify!($right));
                    }
                };
            }

            #[macro_export]
            macro_rules! CHECK_NE {
                ($left:expr, $right:expr) => {
                    if $left == $right {
                        panic!("Check failed: {} != {}", stringify!($left), stringify!($right));
                    }
                };
            }

            #[macro_export]
            macro_rules! CHECK_GE {
                ($left:expr, $right:expr) => {
                    if $left < $right {
                        panic!("Check failed: {} >= {}", stringify!($left), stringify!($right));
                    }
                };
            }
            #[macro_export]
            macro_rules! CHECK_LE {
                ($left:expr, $right:expr) => {
                    if $left > $right {
                        panic!("Check failed: {} <= {}", stringify!($left), stringify!($right));
                    }
                };
            }
        }

        pub mod bit_field {
            #[derive(Debug, Copy, Clone)]
            pub struct BitField8<T, const START: usize, const LENGTH: usize> {
                _phantom: std::marker::PhantomData<T>,
            }

            impl<T: Into<u8> + Copy, const START: usize, const LENGTH: usize> BitField8<T, START, LENGTH> {
                const NUM_VALUES: u8 = (1 << LENGTH) - 1;

                pub fn encode(value: T) -> u32 {
                    (value.into() as u32) << START
                }

                pub fn decode(encoded: u32) -> u8 {
                    ((encoded >> START) & Self::NUM_VALUES as u32) as u8
                }

                pub fn next<U, const NEXT_START: usize>(self) -> BitField8<U, NEXT_START, LENGTH> {
                    BitField8::<U, NEXT_START, LENGTH> { _phantom: std::marker::PhantomData }
                }
            }
        }
    }
    mod ast {
        pub mod scopes {
            use std::cell::RefCell;
            use std::rc::Rc;
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum ScopeType {
                FunctionScope,
                // Add other scope types as needed
            }

            #[derive(Debug)]
            pub struct Scope {
                scope_type_: ScopeType,
                inner_scope_calls_eval_: bool,
                inner_scope_: Option<Rc<RefCell<Scope>>>,
                sibling_: Option<Rc<RefCell<Scope>>>,
                locals_: Vec<*mut super::variables::Variable>, //Vec<Rc<RefCell<super::variables::Variable>>>,
                                                                 // Add other scope properties as needed
            }

            impl Scope {
                pub fn new(scope_type: ScopeType) -> Scope {
                    Scope {
                        scope_type_: scope_type,
                        inner_scope_calls_eval_: false,
                        inner_scope_: None,
                        sibling_: None,
                        locals_: Vec::new(),
                    }
                }

                pub fn scope_type(&self) -> ScopeType {
                    self.scope_type_
                }

                pub fn is_declaration_scope(&self) -> bool {
                    self.scope_type_ == ScopeType::FunctionScope
                }

                pub fn as_declaration_scope(&self) -> &DeclarationScope {
                    // TODO: Implement proper casting / downcasting
                    unsafe { std::mem::transmute(self) }
                }

                pub fn inner_scope_calls_eval(&self) -> bool {
                    self.inner_scope_calls_eval_
                }

                pub fn record_eval_call(&mut self) {
                    self.inner_scope_calls_eval_ = true;
                }

                pub fn record_inner_scope_eval_call(&mut self) {
                    self.inner_scope_calls_eval_ = true;
                }

                pub fn inner_scope(&self) -> Option<Rc<RefCell<Scope>>> {
                    self.inner_scope_.clone()
                }

                pub fn sibling(&self) -> Option<Rc<RefCell<Scope>>> {
                    self.sibling_.clone()
                }

                pub fn locals(&self) -> &Vec<*mut super::variables::Variable> {
                    &self.locals_
                }

                pub fn is_function_scope(&self) -> bool {
                    self.scope_type_ == ScopeType::FunctionScope
                }

                pub fn is_hidden(&self) -> bool {
                    false // Placeholder, implement as needed
                }
            }

            #[derive(Debug)]
            pub struct DeclarationScope {
                base: Scope,
                sloppy_eval_can_extend_vars_: bool,
                function_kind_: FunctionKind,
                num_parameters_: usize,
                start_position_: i32,
                end_position_: i32,
                function_var_: Option<*mut super::variables::Variable>, //Option<Rc<RefCell<super::variables::Variable>>>,
                preparse_data_builder_: Option<*mut super::super::preparse_data::PreparseDataBuilder>,
                needs_private_name_context_chain_recalc_: bool,
                declarations_: Vec<Declaration>,
                // Add other declaration scope properties as needed
            }

            impl DeclarationScope {
                pub fn new(function_kind: FunctionKind) -> DeclarationScope {
                    DeclarationScope {
                        base: Scope::new(ScopeType::FunctionScope),
                        sloppy_eval_can_extend_vars_: false,
                        function_kind_: function_kind,
                        num_parameters_: 0,
                        start_position_: 0,
                        end_position_: 0,
                        function_var_: None,
                        preparse_data_builder_: None,
                        needs_private_name_context_chain_recalc_: false,
                        declarations_: Vec::new(),
                    }
                }

                pub fn sloppy_eval_can_extend_vars(&self) -> bool {
                    self.sloppy_eval_can_extend_vars_
                }

                pub fn function_kind(&self) -> FunctionKind {
                    self.function_kind_
                }

                pub fn num_parameters(&self) -> usize {
                    self.num_parameters_
                }

                pub fn start_position(&self) -> i32 {
                    self.start_position_
                }

                pub fn end_position(&self) -> i32 {
                    self.end_position_
                }

                pub fn function_var(&self) -> Option<*mut super::variables::Variable> {
                    self.function_var_
                }

                pub fn set_preparse_data_builder(&mut self, builder: *mut super::super::preparse_data::PreparseDataBuilder) {
                    self.preparse_data_builder_ = Some(builder);
                }

                pub fn needs_private_name_context_chain_recalc(&self) -> bool {
                    self.needs_private_name_context_chain_recalc_
                }

                pub fn record_needs_private_name_context_chain_recalc(&mut self) {
                    self.needs_private_name_context_chain_recalc_ = true;
                }

                pub fn set_num_parameters(&mut self, num_parameters: usize) {
                    self.num_parameters_ = num_parameters;
                }
                pub fn set_start_position(&mut self, start_position: i32) {
                    self.start_position_ = start_position;
                }

                pub fn set_end_position(&mut self, end_position: i32) {
                    self.end_position_ = end_position;
                }
                pub fn declarations(&mut self) -> &mut Vec<Declaration> {
                    &mut self.declarations_
                }
            }

            impl std::ops::Deref for DeclarationScope {
                type Target = Scope;

                fn deref(&self) -> &Self::Target {
                    &self.base
                }
            }

            impl std::ops::DerefMut for DeclarationScope {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.base
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum FunctionKind {
                NormalFunction,
                // Add other function kinds as needed
            }

            #[derive(Debug)]
            pub struct ClassScope {
                base: Scope,
                should_save_class_variable_index_: bool,
                class_variable_: Option<*mut super::variables::Variable>, //Option<Rc<RefCell<super::variables::Variable>>>,
                is_anonymous_class_: bool,
            }

            impl ClassScope {
                pub fn new() -> ClassScope {
                    ClassScope {
                        base: Scope::new(ScopeType::FunctionScope), // Placeholder scope type
                        should_save_class_variable_index_: false,
                        class_variable_: None,
                        is_anonymous_class_: false,
                    }
                }

                pub fn should_save_class_variable_index(&self) -> bool {
                    self.should_save_class_variable_index_
                }

                pub fn set_should_save_class_variable_index(&mut self) {
                    self.should_save_class_variable_index_ = true;
                }

                pub fn class_variable(&self) -> Option<*mut super::variables::Variable> {
                    self.class_variable_
                }

                pub fn is_anonymous_class(&self) -> bool {
                    self.is_anonymous_class_
                }

                pub fn declare_class_variable(
                    &mut self,
                    ast_value_factory: *mut super::super::parser::AstValueFactory,
                    name: *mut super::super::parser::AstRawString,
                    pos: i32,
                ) -> *mut super::variables::Variable {
                    // TODO: Implement variable declaration
                    let var = unsafe {
                        let factory = super::super::parser::AstNodeFactory { ast_value_factory_: ast_value_factory, zone_: std::ptr::null_mut() };
                        let var = super::variables::Variable::new(name, VariableMode::VAR, InitializationFlag::kCreatedInitialized);
                        self.class_variable_ = Some(std::mem::transmute(var));
                        std::mem::transmute(var)
                    };
                    var
                }
            }

            impl std::ops::Deref for ClassScope {
                type Target = Scope;

                fn deref(&self) -> &Self::Target {
                    &self.base
                }
            }

            impl std::ops::DerefMut for ClassScope {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.base
                }
            }
            #[derive(Debug)]
            pub struct Declaration {
                var_: Option<*mut super::variables::Variable>, //Option<Rc<RefCell<super::variables::Variable>>>,
            }

            impl Declaration {
                pub fn new() -> Declaration {
                    Declaration { var_: None }
                }

                pub fn set_var(&mut self, var: *mut super::variables::Variable) {
                    self.var_ = Some(var);
                }
            }
        }
        pub mod variables {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum VariableMode {
                VAR,
                // Add other variable modes as needed
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum InitializationFlag {
                kCreatedInitialized,
                // Add other initialization flags as needed
            }

            #[derive(Debug)]
            pub struct Variable {
                mode_: VariableMode,
                maybe_assigned_: MaybeAssigned,
                name_: *mut super::super::parser::AstRawString,
                has_forced_context_allocation_: bool,
                initialization_flag_: InitializationFlag,
                is_used_: bool,
                // Add other variable properties as needed
            }

            impl Variable {
                pub fn new(name: *mut super::super::parser::AstRawString, mode: VariableMode, initialization_flag: InitializationFlag) -> Variable {
                    Variable {
                        mode_: mode,
                        maybe_assigned_: MaybeAssigned::kMaybeAssigned,
                        name_: name,
                        has_forced_context_allocation_: false,
                        initialization_flag_: initialization_flag,
                        is_used_: false,
                    }
                }

                pub fn mode(&self) -> VariableMode {
                    self.mode_
                }

                pub fn maybe_assigned(&self) -> MaybeAssigned {
                    self.maybe_assigned_
                }

                pub fn raw_name(&self) -> *mut super::super::parser::AstRawString {
                    self.name_
                }

                pub fn has_forced_context_allocation(&self) -> bool {
                    self.has_forced_context_allocation_
                }

                pub fn set_is_used(&mut self) {
                    self.is_used_ = true;
                }

                pub fn set_maybe_assigned(&mut self) {
                    self.maybe_assigned_ = MaybeAssigned::kMaybeAssigned;
                }

                pub fn force_context_allocation(&mut self) {
                    self.has_forced_context_allocation_ = true;
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum MaybeAssigned {
                kMaybeAssigned,
                // Add other maybe assigned values as needed
            }
        }
    }
    mod objects {
        // Placeholder for objects module
    }
    mod handles {
        // Placeholder for handles module
    }
    mod roots {
        // Placeholder for roots module
    }
    mod zone {
        // Placeholder for zone module
    }
    mod parsing {
        pub mod preparser {
            // Placeholder for preparser module
        }
        pub mod parser {
            use std::cell::RefCell;
            use std::rc::Rc;
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum LanguageMode {
                SmiOnly,
                // Add other language modes as needed
            }

            #[derive(Debug)]
            pub struct AstValueFactory {
                // Add fields as needed
            }

            impl AstValueFactory {
                pub fn empty_string(&self) -> *mut AstRawString {
                    std::ptr::null_mut() // Placeholder
                }
            }

            #[derive(Debug)]
            pub struct AstRawString {
                is_one_byte_: bool,
                length_: usize,
                data_: Vec<u8>,
            }

            impl AstRawString {
                pub fn new(is_one_byte: bool, length: usize, data: Vec<u8>) -> AstRawString {
                    AstRawString {
                        is_one_byte_: is_one_byte,
                        length_: length,
                        data_: data,
                    }
                }
                pub fn is_one_byte(&self) -> bool {
                    self.is_one_byte_
                }
                pub fn length(&self) -> usize {
                    self.length_
                }

                pub fn raw_data(&self) -> &Vec<u8> {
                    &self.data_
                }
            }

            #[derive(Debug)]
            pub struct AstNodeFactory {
                pub ast_value_factory_: *mut AstValueFactory,
                pub zone_: *mut super::super::zone::Zone,
            }

            impl AstNodeFactory {
                pub fn new_variable_declaration(&self, pos: i32) -> *mut super::super::ast::scopes::Declaration {
                    let declaration = unsafe {
                        let decl = super::super::ast::scopes::Declaration::new();
                        std::mem::transmute(decl)
                    };
                    declaration
                }
            }

            #[derive(Debug)]
            pub struct Parser {
                preparse_data_buffer_: RefCell<Vec<u8>>,
                factory_: RefCell<Factory>,
                preparse_data_builder_: RefCell<Option<*mut super::PreparseDataBuilder>>,
            }

            impl Parser {
                pub fn new() -> Parser {
                    Parser {
                        preparse_data_buffer_: RefCell::new(Vec::new()),
                        factory_: RefCell::new(Factory::new()),
                        preparse_data_builder_: RefCell::new(None),
                    }
                }
                pub fn preparse_data_buffer(&self) -> &RefCell<Vec<u8>> {
                    &self.preparse_data_buffer_
                }

                pub fn factory(&self) -> &RefCell<Factory> {
                    &self.factory_
                }

                pub fn set_preparse_data_builder(&self, builder: *mut super::PreparseDataBuilder) {
                    *self.preparse_data_builder_.borrow_mut() = Some(builder);
                }

                pub fn preparse_data_builder(&self) -> Option<*mut super::PreparseDataBuilder> {
                    *self.preparse_data_builder_.borrow()
                }
                pub fn main_zone(&self) -> *mut super::zone::Zone {
                    std::ptr::null_mut() // Placeholder
                }
            }

            #[derive(Debug)]
            pub struct Factory {
                zone_: RefCell<super::super::zone::Zone>,
                ast_value_factory_: RefCell<AstValueFactory>,
            }

            impl Factory {
                pub fn new() -> Factory {
                    Factory {
                        zone_: RefCell::new(super::super::zone::Zone::new()),
                        ast_value_factory_: RefCell::new(AstValueFactory {}),
                    }
                }

                pub fn zone(&self) -> &RefCell<super::super::zone::Zone> {
                    &self.zone_
                }
            }
        }
        pub mod preparse_data_impl {
            // Implementations for PreparseByteDataConstants, etc.
        }
    }
    mod base_consumed_preparse_data {
        // Implementations for BaseConsumedPreparseData
    }
    mod zone {
        #[derive(Debug)]
        pub struct Zone {}

        impl Zone {
            pub fn new() -> Zone {
                Zone {}
            }

            pub fn allocate_array<T, U>(&self, size: usize) -> *mut T {
                let layout = std::alloc::Layout::array::<T>(size).unwrap();
                let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
                ptr
            }

            pub fn new_in_zone<T>(&self) -> *mut T {
                let layout = std::alloc::Layout::new::<T>();
                let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
                ptr
            }
        }
    }

    const kNoSourcePosition: i32 = -1;

    fn IsDefaultConstructor(function_kind: ast::scopes::FunctionKind) -> bool {
        function_kind == ast::scopes::FunctionKind::NormalFunction // Placeholder implementation
    }

    fn IsSerializableVariableMode(mode: ast::variables::VariableMode) -> bool {
        mode == ast::variables::VariableMode::VAR // Placeholder implementation
    }

    // Forward declarations of types (needed because of circular dependencies)
    pub struct PreparseDataBuilder {
        parent_: *mut PreparseDataBuilder,
        byte_data_: ByteData,
        children_buffer_: ZoneVector<*mut PreparseDataBuilder>,
        function_scope_: *mut ast::scopes::DeclarationScope,
        function_length_: i32,
        num_inner_functions_: i32,
        num_inner_with_data_: i32,
        bailed_out_: bool,
        has_data_: bool,
        children_: Vec<*mut PreparseDataBuilder>,
        finalized_children_: bool,
    }

    impl PreparseDataBuilder {
        pub fn new(
            zone: *mut zone::Zone,
            parent_builder: *mut PreparseDataBuilder,
            children_buffer: *mut ZoneVector<*mut PreparseDataBuilder>,
        ) -> PreparseDataBuilder {
            PreparseDataBuilder {
                parent_: parent_builder,
                byte_data_: ByteData::new(),
                children_buffer_: unsafe { ZoneVector::from_raw(children_buffer) },
                function_scope_: std::ptr::null_mut(),
                function_length_: -1,
                num_inner_functions_: 0,
                num_inner_with_data_: 0,
                bailed_out_: false,
                has_data_: false,
                children_: Vec::new(),
                finalized_children_: false,
            }
        }

        pub struct DataGatheringScope<'a> {
            preparser_: &'a mut Preparser,
            builder_: *mut PreparseDataBuilder,
        }

        impl<'a> DataGatheringScope<'a> {
            pub fn new(preparser: &'a mut Preparser) -> Self {
                DataGatheringScope {
                    preparser_: preparser,
                    builder_: std::ptr::null_mut(),
                }
            }

            pub fn start(&mut self, function_scope: *mut ast::scopes::DeclarationScope) {
                let main_zone = self.preparser_.main_zone();
                unsafe {
                    let builder = (*main_zone).new_in_zone::<PreparseDataBuilder>();

                    *builder = PreparseDataBuilder::new(
                        main_zone,
                        self.preparser_.preparse_data_builder(),
                        self.preparser_.preparse_data_builder_buffer(),
                    );

                    self.preparser_.set_preparse_data_builder(builder);
                    (*function_scope).set_preparse_data_builder(builder);
                    self.builder_ = builder;
                }
            }

            pub fn close(&mut self) {
                unsafe {
                    let parent = (*self.builder_).parent_;
                    self.preparser_.set_preparse_data_builder(parent);
                    (*self.builder_).finalize_children(self.preparser_.main_zone());

                    if parent == std::ptr::null_mut() {
                        return;
                    }
                    if !(*self.builder_).has_data_for_parent() {
                        return;
                    }
                    (*parent).add_child(self.builder_);
                }
            }

            pub fn set_skippable_function(
                &mut self,
                function_scope: *mut ast::scopes::DeclarationScope,
                function_length: i32,
                num_inner_functions: i32,
            ) {
                unsafe {
                    CHECK!(
                        (*self.builder_).function_scope_ == std::ptr::null_mut()
                    );
                    (*self.builder_).function_scope_ = function_scope;
                    CHECK_EQ!((*self.builder_).num_inner_functions_, 0);
                    (*self.builder_).function_length_ = function_length;
                    (*self.builder_).num_inner_functions_ = num_inner_functions;
                    (*(*self.builder_).parent_).has_data_ = true;
                }
            }
        }

        fn has_inner_functions(&self) -> bool {
            !self.children_.is_empty()
        }

        fn has_data(&self) -> bool {
            !self.bailed_out_ && self.has_data_
        }

        fn has_data_for_parent(&self) -> bool {
            self.has_data() || self.function_scope_ != std::ptr::null_mut()
        }

        fn add_child(&mut self, child: *mut PreparseDataBuilder) {
            CHECK!(!self.finalized_children_);
            self.children_buffer_.push(child);
        }

        fn finalize_children(&mut self, zone: *mut zone::Zone) {
            CHECK!(!self.finalized_children_);
            let children = self.children_buffer_.to_vec();
            self.children_buffer_.rewind();
            self.children_ = children;
            self.finalized_children_ = true;
        }

        fn scope_needs_data(scope: *mut ast::scopes::Scope) -> bool {
            unsafe {
                if (*scope).is_function_scope() {
                    return !IsDefaultConstructor((*scope).as_declaration_scope().function_kind());
                }
                if !(*scope).is_hidden() {
                    for var in (*scope).locals().iter() {
                        if IsSerializableVariableMode((**var).mode()) {
                            return true;
                        }
                    }
                }
                let mut inner = (*scope).inner_scope();
                while inner.is_some() {
                    if Self::scope_needs_data(inner.unwrap().borrow_mut().as_mut() as *mut ast::scopes::Scope) {
                        return true;
                    }
                    inner = inner.unwrap().borrow().sibling().clone();
                }
                false
            }
        }

        fn save_data_for_skippable_function(&mut self, builder: *mut PreparseDataBuilder) -> bool {
            unsafe {
                let function_scope = (*builder).function_scope_;
                self.byte_data_.write_varint32((*function_scope).start_position() as u32);
                self.byte_data_.write_varint32((*function_scope).end_position() as u32);

                let has_data = (*builder).has_data();
                let length_equals_parameters =
                    (*function_scope).num_parameters() as i32 == (*builder).function_length_;
                let has_data_and_num_parameters =
                    HasDataField::encode(has_data) |
                    LengthEqualsParametersField::encode(length_equals_parameters) |
                    NumberOfParametersField::encode((*function_scope).num_parameters() as u32);
                self.byte_data_.write_varint32(has_data_and_num_parameters);
                if !length_equals_parameters {
                    self.byte_data_.write_varint32((*builder).function_length_ as u32);
                }
                self.byte_data_.write_varint32((*builder).num_inner_functions_ as u32);

                let language_mode = parser::LanguageMode::SmiOnly; // Placeholder for now, get from function_scope
                let language_and_super =
                    LanguageField::encode(language_mode) |
                    UsesSuperField::encode((*function_scope).needs_private_name_context_chain_recalc());
                self.byte_data_.write_quarter(language_and_super);
                has_data
            }
        }

        fn save_scope_allocation_data(&mut self, scope: *mut ast::scopes::DeclarationScope, parser: *mut parser::Parser) {
            if !self.has_data_ {
                return;
            }
            CHECK!(self.has_inner_functions());
            unsafe {
                let preparse_data_buffer = (*parser).preparse_data_buffer();
                self.byte_data_.start(&(*preparse_data_buffer.borrow_mut()));

                self.byte_data_.reserve(4);
                self.byte_data_.write_uint32(0);

                self.byte_data_.reserve(self.children_.len() * kSkippableFunctionMaxDataSize as usize);
                CHECK!(self.finalized_children_);

                for &builder in &self.children_ {
                    if self.save_data_for_skippable_function(builder) {
                        self.num_inner_with_data_ += 1;
                    }
                }
                if !self.bailed_out_ {
                    let current_length = self.byte_data_.length();
                    self.byte_data_.save_current_size_at_first_uint32();
                    CHECK_GE!(self.byte_data_.length(), 4);
                    CHECK_LE!(self.byte_data_.length(), u32::MAX as i32);

                    self.byte_data_.reserve(12);
                    self.byte_data_.write_uint32(ByteData::kMagicValue);
                    self.byte_data_.write_uint32((*scope).start_position() as u32);
                    self.byte_data_.write_uint32((*scope).end_position() as u32);

                    if Self::scope_needs_data(scope as *mut ast::scopes::Scope) {
                        self.save_data_for_scope(scope as *mut ast::scopes::Scope);
                    }
                }
                self.byte_data_.finalize((*(*parser).factory().borrow()).zone().borrow_mut().as_mut() as *mut zone::Zone);
            }
        }

        fn save_data_for_scope(&mut self, scope: *mut ast::scopes::Scope) {
            unsafe {
                CHECK_NE!((*scope).end_position(), kNoSourcePosition);
                CHECK!(Self::scope_needs_data(scope));

                self.byte_data_.reserve(1);
                self.byte_data_.write_uint8((*scope).scope_type() as u8);

                let scope_data_flags =
                    ScopeSloppyEvalCanExtendVarsBit::encode(
                        (*scope).is_declaration_scope() &&
                        (*scope).as_declaration_scope().sloppy_eval_can_extend_vars()
                    ) |
                    InnerScopeCallsEvalField::encode((*scope).inner_scope_calls_eval()) |
                    NeedsPrivateNameContextChainRecalcField::encode(
                        (*scope).is_function_scope() &&
                        (*scope).as_declaration_scope().needs_private_name_context_chain_recalc()
                    ) |
                    ShouldSaveClassVariableIndexField::encode(
                        (*scope).is_class_scope() &&
                        (*scope).as_class_scope().should_save_class_variable_index()
                    );
                self.byte_data_.reserve(1);
                self.byte_data_.write_uint8(scope_data_flags as u8);

                if (*scope).is_function_scope() {
                    let function = (*scope).as_declaration_scope().function_var();
                    if function.is_some() {
                        self.save_data_for_variable(function.unwrap());
                    }
                }

                for var in (*scope).locals() {
                    if IsSerializableVariableMode((**var).mode()) {
                        self.save_data_for_variable(*var);
                    }
                }

                self.save_data_for_inner_scopes(scope);
            }
        }

        fn save_data_for_variable(&mut self, var: *mut ast::variables::Variable) {
            unsafe {
                let name = (*var).raw_name();
                self.byte_data_.reserve(4 + ((*name).length() + 1) * 1);
                self.byte_data_.write_uint8((*name).is_one_byte() as u8);
                self.byte_data_.write_uint32((*name).length() as u32);

                for i in 0..(*name).length() {
                    self.byte_data_.write_uint8((*name).raw_data()[i]);
                }
                let variable_data =
                    VariableMaybeAssignedField::encode(
                        if (*var).maybe_assigned() == ast::variables::MaybeAssigned::kMaybeAssigned {
                            true
                        } else {
                            false
                        }
                    ) |
                    VariableContextAllocatedField::encode((*var).has_forced_context_allocation());
                self.byte_data_.reserve(1);
                self.byte_data_.write_quarter(variable_data);
            }
        }

        fn save_data_for_inner_scopes(&mut self, scope: *mut ast::scopes::Scope) {
            unsafe {
                let mut inner = (*scope).inner_scope();
                while inner.is_some() {
                    let inner_scope = inner.unwrap().borrow_mut();
                    if inner_scope.is_skippable_function_scope() {
                        CHECK!(inner_scope.as_declaration_scope().preparse_data_builder_.is_some());
                    } else if Self::scope_needs_data(inner_scope.as_mut() as *mut ast::scopes::Scope) {
                        self.save_data_for_scope(inner_scope.as_mut() as *mut ast::scopes::Scope);
                    }
                    inner = inner_scope.sibling().clone();
                }
            }
        }

        fn this_or_parent_bailed_out(&self) -> bool {
            self.bailed_out_
        }
        // Placeholder for Serialize methods
    }

    #[derive(Default)]
    pub struct ByteData {
        byte_data_: Vec<u8>,
        index_: i32,
        zone_byte_data_: base::vector::Vector<u8>,
        free_