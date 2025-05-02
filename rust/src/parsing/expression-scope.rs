// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod expression_scope {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub use crate::ast::scopes::*;
    pub use crate::common::message_template::*;
    pub use crate::objects::function_kind::*;
    pub use crate::parsing::scanner::*;
    // use crate::zone::zone::*; // For ScopedPtrList.  Rust doesn't use Zones

    pub type AstRawString = String; // Placeholder:  Assuming String can act as AstRawString

    pub const K_NO_SOURCE_POSITION: i32 = -1; // Constant instead of macro

    pub struct ScopedList<T> {
        items: RefCell<Vec<T>>,
    }

    impl<T> ScopedList<T> {
        pub fn new() -> Self {
            ScopedList { items: RefCell::new(Vec::new()) }
        }

        pub fn add(&self, item: T) {
            self.items.borrow_mut().push(item);
        }

        pub fn length(&self) -> usize {
            self.items.borrow().len()
        }

        pub fn at(&self, index: usize) -> std::cell::Ref<'_, T> {
            std::cell::Ref::map(self.items.borrow(), |v| &v[index])
        }

        pub fn merge_into(&self, other: &ScopedList<T>)
        where
            T: Clone,
        {
            let mut other_items = other.items.borrow_mut();
            for item in self.items.borrow().iter() {
                other_items.push(item.clone());
            }
        }
    }

    pub trait ExpressionTypes {
        type Impl: ParserBase;  // Define the Parser type
        type Expression;
    }

    pub trait ParserBase {
        fn new_raw_variable(&mut self, name: &AstRawString, pos: i32) -> Rc<RefCell<VariableProxy>>;
        fn declare_variable(
            &mut self,
            name: &AstRawString,
            kind: VariableKind,
            mode: VariableMode,
            initialization_flag: VariableInitializationFlag,
            scope: &mut Scope,
            was_added: &mut bool,
            pos: i32,
        ) -> Rc<RefCell<Variable>>;
        fn new_function_scope(&mut self, kind: FunctionKind) -> Rc<RefCell<DeclarationScope>>;
        fn declare_and_bind_variable(
            &mut self,
            proxy: &Rc<RefCell<VariableProxy>>,
            kind: VariableKind,
            mode: VariableMode,
            result: &Rc<RefCell<DeclarationScope>>,
            was_added: &mut bool,
            initializer_position: i32,
        );
        fn report_message_at(&self, loc: &ScannerLocation, message: MessageTemplate);
        fn report_message(&self, message: MessageTemplate);
        fn scope(&mut self) -> &mut Scope;
        fn language_mode(&self) -> LanguageMode;
        fn loop_nesting_depth(&self) -> i32;
        fn is_let(&self, name: &AstRawString) -> bool;
        fn rewrite_invalid_reference_expression<T>(
            &mut self,
            expression: T,
            beg_pos: i32,
            end_pos: i32,
            message: MessageTemplate,
            early_error: bool,
        ) -> T;
        fn is_assignable_identifier<T>(&self, expression: T) -> bool;
        fn variable_buffer(&self) -> &ScopedList<(Rc<RefCell<VariableProxy>>, i32)>;
        fn has_error(&self) -> bool;

        // Add any other methods needed by the Rust translation from the C++ Parser
    }

    /// ExpressionScope is used in a stack fashion, and is used to specialize
    /// expression parsing for the task at hand.
    pub struct ExpressionScope<'a, Types: ExpressionTypes> {
        parser: &'a mut Types::Impl,
        parent: Option<&'a mut ExpressionScope<'a, Types>>,
        type_: ScopeType,
        has_possible_parameter_in_scope_chain_: bool,
        has_possible_arrow_parameter_in_scope_chain_: bool,
    }

    impl<'a, Types: ExpressionTypes> ExpressionScope<'a, Types> {
        pub fn new(parser: &'a mut Types::Impl, type_: ScopeType) -> Self {
            let parent = std::mem::replace(&mut parser.expression_scope(), None);

            let has_possible_parameter_in_scope_chain_ =
                type_.can_be_parameter_declaration() ||
                (parent.as_ref().map_or(false, |p| p.has_possible_parameter_in_scope_chain()));

            let has_possible_arrow_parameter_in_scope_chain_ =
                type_.can_be_arrow_parameter_declaration() ||
                (parent.as_ref().map_or(false, |p| p.has_possible_arrow_parameter_in_scope_chain()));

            let mut scope = ExpressionScope {
                parser,
                parent,
                type_,
                has_possible_parameter_in_scope_chain_,
                has_possible_arrow_parameter_in_scope_chain_,
            };
            scope.parser.expression_scope() = Some(&mut scope);
            scope
        }

        pub fn drop(self) {
            let parent = self.parent;
            let current_scope = self.parser.expression_scope().take();

            if let Some(mut parent_scope) = parent {
                self.parser.expression_scope() = Some(parent_scope);
            } else {
                // if !std::ptr::eq(current_scope.unwrap() as *mut _, &self as *const _ as *mut _) {
                //     println!("Parser's expression scope is not the current scope.");
                //     panic!("ExpressionScope mismatch during drop");
                // }
                self.parser.expression_scope() = None;
            }
        }

        fn parser(&self) -> &Types::Impl {
            self.parser
        }

        fn parent(&self) -> Option<&ExpressionScope<'a, Types>> {
            self.parent.map(|x| &*x)
        }

        fn report(&self, loc: &ScannerLocation, message: MessageTemplate) {
            self.parser.report_message_at(loc, message);
        }

        pub fn new_variable(
            &mut self,
            name: &AstRawString,
            pos: i32,
        ) -> Rc<RefCell<VariableProxy>> {
            let result = self.parser.new_raw_variable(name, pos);
            if self.can_be_expression() {
                self.as_expression_parsing_scope().track_variable(result.clone());
            } else {
                let var = self.declare(name, pos);
                if self.is_var_declaration() {
                    let mut passed_through_with = false;
                    let mut scope = self.parser.scope();
                    loop {
                        if scope.is_declaration_scope() {
                            break;
                        }
                        if scope.is_with_scope() {
                            passed_through_with = true;
                        } else if scope.is_catch_scope() {
                            if let Some(masking_var) = scope.lookup_local(name) {
                                result.borrow_mut().set_is_assigned();
                                if passed_through_with {
                                    break;
                                }
                                result.borrow_mut().bind_to(masking_var.clone());
                                masking_var.borrow_mut().set_maybe_assigned();
                                return result;
                            }
                        }
                        scope = scope.outer_scope();
                    }
                    if passed_through_with {
                        self.parser.scope().add_unresolved(result.clone());
                        return result;
                    }
                    result.borrow_mut().bind_to(var.clone());
                }
            }
            result
        }

        pub fn merge_variable_list(
            &mut self,
            variable_list: &mut ScopedList<(Rc<RefCell<VariableProxy>>, i32)>,
        ) {
            if !self.can_be_expression() {
                return;
            }
            if !self.can_be_declaration() {
                for proxy_initializer_pair in self.parser.variable_buffer().items.borrow().iter() {
                    let proxy = proxy_initializer_pair.0.clone();
                    self.parser.scope().add_unresolved(proxy);
                }
            }
            variable_list.merge_into(self.as_expression_parsing_scope().variable_list());
        }

        pub fn declare(&mut self, name: &AstRawString, pos: i32) -> Rc<RefCell<Variable>> {
            if self.type_ == ScopeType::ParameterDeclaration {
                self.as_parameter_declaration_parsing_scope().declare(name, pos)
            } else {
                self.as_variable_declaration_parsing_scope().declare(name, pos)
            }
        }

        pub fn mark_identifier_as_assigned(&mut self) {
            if !self.can_be_expression() {
                return;
            }
            self.as_expression_parsing_scope().mark_identifier_as_assigned();
        }

        pub fn validate_as_pattern(&mut self, expression: Types::Expression, begin: i32, end: i32) {
            if !self.can_be_expression() {
                return;
            }
            self.as_expression_parsing_scope().validate_pattern(expression, begin, end);
            self.as_expression_parsing_scope().clear_expression_error();
        }

        pub fn validate_as_expression(&mut self) {
            if !self.can_be_expression() {
                return;
            }
            self.as_expression_parsing_scope().validate_expression();
            self.as_expression_parsing_scope().clear_pattern_error();
        }

        pub fn record_async_arrow_parameters_error(
            &mut self,
            loc: &ScannerLocation,
            message: MessageTemplate,
        ) {
            if !self.can_be_expression() {
                return;
            }
            self.as_expression_parsing_scope()
                .record_async_arrow_parameters_error(loc, message);
        }

        pub fn record_parameter_initializer_error(
            &mut self,
            loc: &ScannerLocation,
            message: MessageTemplate,
        ) {
            let mut scope = self;
            loop {
                if scope.is_certainly_parameter_declaration() {
                    break;
                }
                if !scope.has_possible_parameter_in_scope_chain() {
                    return;
                }
                if scope.can_be_parameter_declaration() {
                    scope
                        .as_arrow_head_parsing_scope()
                        .record_declaration_error(loc, message);
                }
                scope = match &mut scope.parent {
                    Some(parent) => parent,
                    None => return,
                };
            }
            self.report(loc, message);
        }

        pub fn record_this_use(&mut self) {
            let mut scope = self;
            loop {
                if scope.is_arrow_head_parsing_scope() {
                    scope.as_arrow_head_parsing_scope().record_this_use();
                }
                scope = match &mut scope.parent {
                    Some(parent) => parent,
                    None => break,
                };
            }
        }

        pub fn record_pattern_error(&mut self, loc: &ScannerLocation, message: MessageTemplate) {
            if self.is_certainly_pattern() {
                self.report(loc, message);
            } else {
                self.as_expression_parsing_scope().record_pattern_error(loc, message);
            }
        }

        pub fn record_strict_mode_parameter_error(
            &mut self,
            loc: &ScannerLocation,
            message: MessageTemplate,
        ) {
            if !self.can_be_parameter_declaration() {
                return;
            }
            if self.is_certainly_parameter_declaration() {
                if is_strict(self.parser.language_mode()) {
                    self.report(loc, message);
                } else {
                    // self.parser.parameters_.set_strict_parameter_error(loc, message);  // how to express parameters_?
                    todo!()
                }
            } else {
                // self.parser.next_arrow_function_info_.strict_parameter_error_location = loc;
                // self.parser.next_arrow_function_info_.strict_parameter_error_message = message;
                todo!()
            }
        }

        pub fn record_declaration_error(&mut self, loc: &ScannerLocation, message: MessageTemplate) {
            if !self.can_be_declaration() {
                return;
            }
            if self.is_certainly_declaration() {
                self.report(loc, message);
            } else {
                self.as_arrow_head_parsing_scope().record_declaration_error(loc, message);
            }
        }

        pub fn record_expression_error(&mut self, loc: &ScannerLocation, message: MessageTemplate) {
            if !self.can_be_expression() {
                return;
            }
            self.as_expression_parsing_scope().record_expression_error(loc, message);
        }

        pub fn record_non_simple_parameter(&mut self) {
            if !self.is_arrow_head_parsing_scope() {
                return;
            }
            self.as_arrow_head_parsing_scope().record_non_simple_parameter();
        }

        pub fn is_certainly_declaration(&self) -> bool {
            self.type_.is_certainly_declaration()
        }

        pub fn set_initializers(&mut self, variable_index: i32, peek_position: i32) -> i32 {
            if self.can_be_expression() {
                self.as_expression_parsing_scope().set_initializers(variable_index, peek_position)
            } else {
                variable_index
            }
        }

        pub fn has_possible_arrow_parameter_in_scope_chain(&self) -> bool {
            self.has_possible_arrow_parameter_in_scope_chain_
        }

        fn as_expression_parsing_scope(&mut self) -> &mut ExpressionParsingScope<'a, Types> {
            if !self.can_be_expression() {
                panic!("Scope is not an ExpressionParsingScope");
            }
            unsafe {
                std::mem::transmute::<
                    &mut ExpressionScope<'a, Types>,
                    &mut ExpressionParsingScope<'a, Types>,
                >(self)
            }
        }

        fn as_arrow_head_parsing_scope(&mut self) -> &mut ArrowHeadParsingScope<'a, Types> {
            if !self.is_arrow_head_parsing_scope() {
                panic!("Scope is not an ArrowHeadParsingScope");
            }
            unsafe {
                std::mem::transmute::<
                    &mut ExpressionScope<'a, Types>,
                    &mut ArrowHeadParsingScope<'a, Types>,
                >(self)
            }
        }

        fn as_parameter_declaration_parsing_scope(
            &mut self,
        ) -> &mut ParameterDeclarationParsingScope<'a, Types> {
            if !self.is_certainly_parameter_declaration() {
                panic!("Scope is not a ParameterDeclarationParsingScope");
            }
            unsafe {
                std::mem::transmute::<
                    &mut ExpressionScope<'a, Types>,
                    &mut ParameterDeclarationParsingScope<'a, Types>,
                >(self)
            }
        }

        fn as_variable_declaration_parsing_scope(
            &mut self,
        ) -> &mut VariableDeclarationParsingScope<'a, Types> {
            if !self.is_variable_declaration() {
                panic!("Scope is not a VariableDeclarationParsingScope");
            }
            unsafe {
                std::mem::transmute::<
                    &mut ExpressionScope<'a, Types>,
                    &mut VariableDeclarationParsingScope<'a, Types>,
                >(self)
            }
        }

        fn can_be_expression(&self) -> bool {
            self.type_.can_be_expression()
        }

        fn can_be_declaration(&self) -> bool {
            self.type_.can_be_declaration()
        }

        fn is_variable_declaration(&self) -> bool {
            self.type_.is_variable_declaration()
        }

        fn is_lexical_declaration(&self) -> bool {
            self.type_ == ScopeType::LexicalDeclaration
        }

        fn is_async_arrow_head_parsing_scope(&self) -> bool {
            self.type_ == ScopeType::MaybeAsyncArrowParameterDeclaration
        }

        fn is_var_declaration(&self) -> bool {
            self.type_ == ScopeType::VarDeclaration
        }

        fn is_arrow_head_parsing_scope(&self) -> bool {
            self.type_.is_arrow_head_parsing_scope()
        }

        fn is_certainly_pattern(&self) -> bool {
            self.is_certainly_declaration()
        }

        fn can_be_parameter_declaration(&self) -> bool {
            self.type_.can_be_parameter_declaration()
        }

        fn can_be_arrow_parameter_declaration(&self) -> bool {
            self.type_.can_be_arrow_parameter_declaration()
        }

        fn is_certainly_parameter_declaration(&self) -> bool {
            self.type_ == ScopeType::ParameterDeclaration
        }

        fn has_possible_parameter_in_scope_chain(&self) -> bool {
            self.has_possible_parameter_in_scope_chain_
        }
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum ScopeType {
        Expression,
        MaybeArrowParameterDeclaration,
        MaybeAsyncArrowParameterDeclaration,
        ParameterDeclaration,
        VarDeclaration,
        LexicalDeclaration,
    }

    impl ScopeType {
        fn can_be_expression(&self) -> bool {
            use ScopeType::*;
            matches!(
                self,
                Expression | MaybeArrowParameterDeclaration | MaybeAsyncArrowParameterDeclaration
            )
        }

        fn can_be_declaration(&self) -> bool {
            use ScopeType::*;
            matches!(
                self,
                MaybeArrowParameterDeclaration
                    | MaybeAsyncArrowParameterDeclaration
                    | ParameterDeclaration
                    | VarDeclaration
                    | LexicalDeclaration
            )
        }

        fn is_arrow_head_parsing_scope(&self) -> bool {
            use ScopeType::*;
            matches!(self, MaybeArrowParameterDeclaration | MaybeAsyncArrowParameterDeclaration)
        }

        fn can_be_parameter_declaration(&self) -> bool {
            use ScopeType::*;
            matches!(self, MaybeArrowParameterDeclaration | MaybeAsyncArrowParameterDeclaration | ParameterDeclaration)
        }

        fn can_be_arrow_parameter_declaration(&self) -> bool {
            use ScopeType::*;
            matches!(self, MaybeArrowParameterDeclaration | MaybeAsyncArrowParameterDeclaration)
        }

        fn is_variable_declaration(&self) -> bool {
            use ScopeType::*;
            matches!(self, VarDeclaration | LexicalDeclaration)
        }

        fn is_certainly_declaration(&self) -> bool {
            use ScopeType::*;
            matches!(self, ParameterDeclaration | VarDeclaration | LexicalDeclaration)
        }
    }

    /// Used to unambiguously parse var, let, const declarations.
    pub struct VariableDeclarationParsingScope<'a, Types: ExpressionTypes> {
        base: ExpressionScope<'a, Types>,
        mode_: VariableMode,
        names_: Option<ScopedList<AstRawString>>, // Use Option to represent nullable ZonePtrList
    }

    impl<'a, Types: ExpressionTypes> VariableDeclarationParsingScope<'a, Types> {
        pub fn new(
            parser: &'a mut Types::Impl,
            mode: VariableMode,
            names: Option<ScopedList<AstRawString>>,
        ) -> Self {
            let scope_type = if is_lexical_variable_mode(mode) {
                ScopeType::LexicalDeclaration
            } else {
                ScopeType::VarDeclaration
            };
            VariableDeclarationParsingScope {
                base: ExpressionScope::new(parser, scope_type),
                mode_: mode,
                names_: names,
            }
        }

        pub fn declare(&mut self, name: &AstRawString, pos: i32) -> Rc<RefCell<Variable>> {
            let kind = VariableKind::NORMAL_VARIABLE;
            let mut was_added = false;
            let var = self.base.parser().declare_variable(
                name,
                kind,
                self.mode_,
                Variable::default_initialization_flag(self.mode_),
                self.base.parser().scope(),
                &mut was_added,
                pos,
            );
            if was_added && self.base.parser().scope().num_var() > K_MAX_NUM_FUNCTION_LOCALS {
                self.base.parser().report_message(MessageTemplate::kTooManyVariables);
            }

            if let Some(names) = &self.names_ {
                names.add(name.clone());
            }

            if self.base.is_lexical_declaration() {
                if self.base.parser().is_let(name) {
                    self.base.parser().report_message_at(
                        &ScannerLocation {
                            beg_pos: pos,
                            end_pos: pos + name.len() as i32,
                        },
                        MessageTemplate::kLetInLexicalBinding,
                    );
                }
            } else {
                if self.base.parser().loop_nesting_depth() > 0 {
                    var.borrow_mut().set_maybe_assigned();
                }
            }
            var
        }
    }

    const K_MAX_NUM_FUNCTION_LOCALS: i32 = (1 << 23) - 1;

    ///Represents the parameter declarations of functions.
    pub struct ParameterDeclarationParsingScope<'a, Types: ExpressionTypes> {
        base: ExpressionScope<'a, Types>,
        duplicate_loc_: ScannerLocation,
    }

    impl<'a, Types: ExpressionTypes> ParameterDeclarationParsingScope<'a, Types> {
        pub fn new(parser: &'a mut Types::Impl) -> Self {
            ParameterDeclarationParsingScope {
                base: ExpressionScope::new(parser, ScopeType::ParameterDeclaration),
                duplicate_loc_: ScannerLocation::invalid(),
            }
        }

        pub fn declare(&mut self, name: &AstRawString, pos: i32) -> Rc<RefCell<Variable>> {
            let kind = VariableKind::PARAMETER_VARIABLE;
            let mode = VariableMode::kVar;
            let mut was_added = false;
            let var = self.base.parser().declare_variable(
                name,
                kind,
                mode,
                Variable::default_initialization_flag(mode),
                self.base.parser().scope(),
                &mut was_added,
                pos,
            );
            if !self.has_duplicate() && !was_added {
                self.duplicate_loc_ = ScannerLocation {
                    beg_pos: pos,
                    end_pos: pos + name.len() as i32,
                };
            }
            var
        }

        pub fn has_duplicate(&self) -> bool {
            self.duplicate_loc_.is_valid()
        }

        pub fn duplicate_location(&self) -> &ScannerLocation {
            &self.duplicate_loc_
        }
    }

    /// Parsing expressions is always ambiguous between at least left-hand-side and
    /// right-hand-side of assignments.
    pub struct ExpressionParsingScope<'a, Types: ExpressionTypes> {
        base: ExpressionScope<'a, Types>,
        variable_list_: ScopedList<(Rc<RefCell<VariableProxy>>, i32)>,
        messages_: [MessageTemplate; K_NUMBER_OF_ERRORS as usize],
        locations_: [ScannerLocation; K_NUMBER_OF_ERRORS as usize],
        has_async_arrow_in_scope_chain_: bool,
        verified_: bool, // Debug Only
    }

    const K_NUMBER_OF_ERRORS: i32 = 2;

    impl<'a, Types: ExpressionTypes> ExpressionParsingScope<'a, Types> {
        pub fn new(parser: &'a mut Types::Impl, type_: ScopeType) -> Self {
            let has_async_arrow_in_scope_chain_ =
                type_ == ScopeType::MaybeAsyncArrowParameterDeclaration ||
                (parser.expression_scope().is_some() && parser.expression_scope().as_ref().unwrap().can_be_expression() &&
                 parser.expression_scope().as_ref().unwrap().as_expression_parsing_scope().has_async_arrow_in_scope_chain_);

            let mut scope = ExpressionParsingScope {
                base: ExpressionScope::new(parser, type_),
                variable_list_: ScopedList::new(),
                messages_: [MessageTemplate::kNone; K_NUMBER_OF_ERRORS as usize],
                locations_: [ScannerLocation::invalid(); K_NUMBER_OF_ERRORS as usize],
                has_async_arrow_in_scope_chain_,
                verified_: false,
            };

            scope.clear(ErrorNumber::ExpressionIndex as usize);
            scope.clear(ErrorNumber::PatternIndex as usize);
            scope
        }

        pub fn record_async_arrow_parameters_error(
            &mut self,
            loc: &ScannerLocation,
            message: MessageTemplate,
        ) {
            let mut scope = &mut self.base;
            loop {
                if !self.has_async_arrow_in_scope_chain_ {
                    break;
                }

                if scope.type_ == ScopeType::MaybeAsyncArrowParameterDeclaration {
                    scope
                        .as_arrow_head_parsing_scope()
                        .record_declaration_error(loc, message);
                }

                scope = match &mut scope.parent {
                    Some(parent) => parent,
                    None => break,
                };
            }
        }

        pub fn validate_and_rewrite_reference(
            &mut self,
            expression: Types::Expression,
            beg_pos: i32,
            end_pos: i32,
        ) -> Types::Expression {
            if self.base.parser().is_assignable_identifier(expression) {
                self.mark_identifier_as_assigned();
                self.mark_verified();
                return expression;
            } else {
                self.validate_expression();
                return expression;
            }
            self.mark_verified();
            let early_error = false;
            self.base.parser().rewrite_invalid_reference_expression(
                expression,
                beg_pos,
                end_pos,
                MessageTemplate::kInvalidLhsInFor,
                early_error,
            )
        }

        pub fn record_expression_error(&mut self, loc: &ScannerLocation, message: MessageTemplate) {
            self.record(ErrorNumber::ExpressionIndex as usize, loc, message);
        }

        pub fn record_pattern_error(&mut self, loc: &ScannerLocation, message: MessageTemplate) {
            self.record(ErrorNumber::PatternIndex as usize, loc, message);
        }

        pub fn validate_expression(&mut self) {
            self.validate(ErrorNumber::ExpressionIndex as usize);
        }

        pub fn validate_pattern(&mut self, expression: Types::Expression, begin: i32, end: i32) {
            self.validate(ErrorNumber::PatternIndex as usize);
            if false { //expression.is_parenthesized() {
                self.base.report(
                    &ScannerLocation {
                        beg_pos: begin,
                        end_pos: end,
                    },
                    MessageTemplate::kInvalidDestructuringTarget,
                );
            }

            for variable_initializer_pair in self.variable_list_.items.borrow().iter() {
                variable_initializer_pair.0.borrow_mut().set_is_assigned();
            }
        }

        pub fn clear_expression_error(&mut self) {
            self.clear(ErrorNumber::ExpressionIndex as usize);
        }

        pub fn clear_pattern_error(&mut self) {
            self.clear(ErrorNumber::PatternIndex as usize);
        }

        pub fn track_variable(&mut self, variable: Rc<RefCell<VariableProxy>>) {
            if !self.base.can_be_declaration() {
                self.base.parser().scope().add_unresolved(variable.clone());
            }
            self.variable_list_.add((variable, K_NO_SOURCE_POSITION));
        }

        pub fn mark_identifier_as_assigned(&mut self) {
            if self.variable_list_.length() == 0 {
                return;
            }
            self.variable_list_.at(self.variable_list_.length() - 1).0.borrow_mut().set_is_assigned();
        }

        pub fn set_initializers(&mut self, first_variable_index: i32, position: i32) -> i32 {
            let len = self.variable_list_.length();
            if len == 0 {
                return 0;
            }

            let end = len - 1;
            for i in (first_variable_index..=end).rev() {
                if self.variable_list_.at(i as usize).1 == K_NO_SOURCE_POSITION {
                    // self.variable_list_.at(i as usize).1 = position;
                }
            }
            end as i32
        }

        pub fn variable_list(&mut self) -> &mut ScopedList<(Rc<RefCell<VariableProxy>>, i32)> {
            &mut self.variable_list_
        }

        fn clear(&mut self, index: usize) {
            self.messages_[index] = MessageTemplate::kNone;
            self.locations_[index] = ScannerLocation::invalid();
        }

        fn is_valid(&self, index: usize) -> bool {
            !self.locations_[index].is_valid()
        }

        fn record(&mut self, index: usize, loc: &ScannerLocation, message: MessageTemplate) {
            if !self.is_valid(index) {
                return;
            }
            self.messages_[index] = message;
            self.locations_[index] = *loc;
        }

        fn validate(&mut self, index: usize) {
            if !self.is_valid(index) {
                self.report(index);
            }
            self.mark_verified();
        }

        fn report(&self, index: usize) {
            self.base.report(&self.locations_[index], self.messages_[index]);
        }

        fn mark_verified(&mut self) {
            self.verified_ = true;
        }
    }

    impl<'a, Types: ExpressionTypes> Drop for ExpressionParsingScope<'a, Types> {
        fn drop(&mut self) {
            if !self.has_error() || self.verified_ {
                // Do nothing
            }
        }
    }

    #[allow(dead_code)]
    enum ErrorNumber {
        ExpressionIndex = 0,
        PatternIndex = 1,
    }

    /// This class is used to parse multiple ambiguous expressions and declarations
    /// in the same scope.
    pub struct AccumulationScope<'a, Types: ExpressionTypes> {
        scope_: Option<&'a mut ExpressionParsingScope<'a, Types>>,
        messages_: [MessageTemplate; ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
        locations_: [ScannerLocation; ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
    }

    impl<'a, Types: ExpressionTypes> AccumulationScope<'a, Types> {
        pub fn new(scope: &'a mut ExpressionScope<'a, Types>) -> Self {
            if !scope.can_be_expression() {
                return AccumulationScope {
                    scope_: None,
                    messages_: [MessageTemplate::kNone; ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
                    locations_: [ScannerLocation::invalid(); ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
                };
            }
            let scope_ = scope.as_expression_parsing_scope();
            let mut accumulation_scope = AccumulationScope {
                scope_: Some(scope_),
                messages_: [MessageTemplate::kNone; ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
                locations_: [ScannerLocation::invalid(); ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS as usize],
            };

            for i in 0..ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS {
                accumulation_scope.copy(i as usize);
                scope_.clear(i as usize);
            }
            accumulation_scope
        }

        pub fn accumulate(&mut self) {
            if self.scope_.is_none() {
                return;
            }
            let scope_ = self.scope_.as_mut().unwrap();
            for i in 0..ExpressionParsingScope::<Types>::K_NUMBER_OF_ERRORS {
                if !self.locations_[i as usize].is_valid() {
                    self.copy(i as usize);
                }
                scope_.clear(i as usize);
            }
        }

        pub fn validate_expression(&mut self) {
            if self.scope_.is_none() {
                return;
            }
            let scope_ = self.scope_.as_mut().unwrap();
            scope_.validate_expression();
            scope_.clear(ErrorNumber::PatternIndex as usize);
        }
    }

    impl<'a, Types: ExpressionTypes> Drop for AccumulationScope<'a, Types> {
        fn drop(&mut self) {
            if self.scope_.is_none() {
                return;
            }

            self.accumulate();
            let scope_ = self.