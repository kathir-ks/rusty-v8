// Converted from V8 C++ source files:
// Header: prettyprinter.h
// Implementation: prettyprinter.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod prettyprinter {
    use std::fmt::Write;
    use std::ptr;

    use crate::ast::ast::*;
    use crate::base::compiler_specific::*;
    use crate::execution::isolate::Isolate;
    use crate::objects::function_kind::FunctionKind;
    use crate::strings::string_builder::*;

    pub struct CallPrinter {
        isolate: *mut Isolate,
        num_prints: i32,
        builder: IncrementalStringBuilder,
        position: i32,
        found: bool,
        done: bool,
        is_user_js: bool,
        is_iterator_error: bool,
        is_async_iterator_error: bool,
        is_call_error: bool,
        error_in_spread_args: SpreadErrorInArgsHint,
        destructuring_prop: *mut ObjectLiteralProperty,
        destructuring_assignment: *mut Assignment,
        spread_arg: *mut Expression,
        function_kind: FunctionKind,
        // DEFINE_AST_VISITOR_SUBCLASS_MEMBERS
    }

    impl CallPrinter {
        pub fn new(
            isolate: *mut Isolate,
            is_user_js: bool,
            error_in_spread_args: SpreadErrorInArgsHint,
        ) -> CallPrinter {
            CallPrinter {
                isolate,
                num_prints: 0,
                builder: IncrementalStringBuilder::new(isolate),
                position: 0,
                found: false,
                done: false,
                is_user_js,
                is_iterator_error: false,
                is_async_iterator_error: false,
                is_call_error: false,
                error_in_spread_args,
                destructuring_prop: ptr::null_mut(),
                destructuring_assignment: ptr::null_mut(),
                spread_arg: ptr::null_mut(),
                function_kind: FunctionKind::kNormalFunction,
                // InitializeAstVisitor(isolate);
            }
        }

        //~CallPrinter() = default;
        pub fn get_error_hint(&self) -> ErrorHint {
            if self.is_call_error {
                if self.is_iterator_error {
                    return ErrorHint::kCallAndNormalIterator;
                }
                if self.is_async_iterator_error {
                    return ErrorHint::kCallAndAsyncIterator;
                }
            } else {
                if self.is_iterator_error {
                    return ErrorHint::kNormalIterator;
                }
                if self.is_async_iterator_error {
                    return ErrorHint::kAsyncIterator;
                }
            }
            ErrorHint::kNone
        }

        pub fn spread_arg(&self) -> *mut Expression {
            self.spread_arg
        }

        pub fn destructuring_prop(&self) -> *mut ObjectLiteralProperty {
            self.destructuring_prop
        }

        pub fn destructuring_assignment(&self) -> *mut Assignment {
            self.destructuring_assignment
        }

        pub fn print(&mut self, program: *mut FunctionLiteral, position: i32) -> Result<String, String> {
            self.num_prints = 0;
            self.position = position;
            self.find(program as *mut AstNode, false);
            let handle = self.builder.finish();
            match handle {
                Ok(s) => Ok(s),
                Err(e) => Err(format!("String Builder failed {:?}", e)),
            }
        }

        fn find(&mut self, node: *mut AstNode, print: bool) {
            if self.found {
                if print {
                    let prev_num_prints = self.num_prints;
                    self.visit(node);
                    if prev_num_prints != self.num_prints {
                        return;
                    }
                }
                self.print_str("(intermediate value)");
            } else {
                self.visit(node);
            }
        }

        fn print_char(&mut self, c: char) {
            if !self.found || self.done {
                return;
            }
            self.num_prints += 1;
            self.builder.append_character(c);
        }

        fn print_str(&mut self, str: &str) {
            if !self.found || self.done {
                return;
            }
            self.num_prints += 1;
            self.builder.append_cstring(str);
        }

        fn print_string(&mut self, str: String) {
            if !self.found || self.done {
                return;
            }
            self.num_prints += 1;
            self.builder.append_string(str);
        }

        fn find_statements(&mut self, statements: *const ZonePtrList<Statement>) {
            if statements.is_null() {
                return;
            }
            unsafe {
                for i in 0..(*statements).length() {
                    let statement = (*statements).at(i);
                    self.find(statement as *mut AstNode, false);
                }
            }
        }

        fn find_arguments(&mut self, arguments: *const ZonePtrList<Expression>) {
            if self.found {
                return;
            }
            if arguments.is_null() {
                return;
            }
            unsafe {
                for i in 0..(*arguments).length() {
                    let argument = (*arguments).at(i);
                    self.find(argument as *mut AstNode, false);
                }
            }
        }

        fn print_literal_direct_handle(&mut self, value: String, quote: bool) {
           if quote {
                self.print_char('"');
            }
            self.print_str(&value);
            if quote {
                self.print_char('"');
            }
        }

        fn print_literal_ast_raw_string(&mut self, value: *const AstRawString, quote: bool) {
            unsafe {
                self.print_literal_direct_handle((*value).string(), quote);
            }
        }

        // Individual nodes
        fn visit(&mut self, node: *mut AstNode) {
            if node.is_null() {
                return;
            }
            unsafe {
                match (*node).node_type() {
                    AstNodeType::kBlock => self.visit_block(node as *mut Block),
                    AstNodeType::kVariableDeclaration => self.visit_variable_declaration(node as *mut VariableDeclaration),
                    AstNodeType::kFunctionDeclaration => self.visit_function_declaration(node as *mut FunctionDeclaration),
                    AstNodeType::kExpressionStatement => self.visit_expression_statement(node as *mut ExpressionStatement),
                    AstNodeType::kEmptyStatement => self.visit_empty_statement(node as *mut EmptyStatement),
                    AstNodeType::kSloppyBlockFunctionStatement => self.visit_sloppy_block_function_statement(node as *mut SloppyBlockFunctionStatement),
                    AstNodeType::kIfStatement => self.visit_if_statement(node as *mut IfStatement),
                    AstNodeType::kContinueStatement => self.visit_continue_statement(node as *mut ContinueStatement),
                    AstNodeType::kBreakStatement => self.visit_break_statement(node as *mut BreakStatement),
                    AstNodeType::kReturnStatement => self.visit_return_statement(node as *mut ReturnStatement),
                    AstNodeType::kWithStatement => self.visit_with_statement(node as *mut WithStatement),
                    AstNodeType::kSwitchStatement => self.visit_switch_statement(node as *mut SwitchStatement),
                    AstNodeType::kDoWhileStatement => self.visit_do_while_statement(node as *mut DoWhileStatement),
                    AstNodeType::kWhileStatement => self.visit_while_statement(node as *mut WhileStatement),
                    AstNodeType::kForStatement => self.visit_for_statement(node as *mut ForStatement),
                    AstNodeType::kForInStatement => self.visit_for_in_statement(node as *mut ForInStatement),
                    AstNodeType::kForOfStatement => self.visit_for_of_statement(node as *mut ForOfStatement),
                    AstNodeType::kTryCatchStatement => self.visit_try_catch_statement(node as *mut TryCatchStatement),
                    AstNodeType::kTryFinallyStatement => self.visit_try_finally_statement(node as *mut TryFinallyStatement),
                    AstNodeType::kDebuggerStatement => self.visit_debugger_statement(node as *mut DebuggerStatement),
                    AstNodeType::kFunctionLiteral => self.visit_function_literal(node as *mut FunctionLiteral),
                    AstNodeType::kClassLiteral => self.visit_class_literal(node as *mut ClassLiteral),
                    AstNodeType::kInitializeClassMembersStatement => self.visit_initialize_class_members_statement(node as *mut InitializeClassMembersStatement),
                    AstNodeType::kInitializeClassStaticElementsStatement => self.visit_initialize_class_static_elements_statement(node as *mut InitializeClassStaticElementsStatement),
                    AstNodeType::kAutoAccessorGetterBody => self.visit_auto_accessor_getter_body(node as *mut AutoAccessorGetterBody),
                    AstNodeType::kAutoAccessorSetterBody => self.visit_auto_accessor_setter_body(node as *mut AutoAccessorSetterBody),
                    AstNodeType::kNativeFunctionLiteral => self.visit_native_function_literal(node as *mut NativeFunctionLiteral),
                    AstNodeType::kConditionalChain => self.visit_conditional_chain(node as *mut ConditionalChain),
                    AstNodeType::kConditional => self.visit_conditional(node as *mut Conditional),
                    AstNodeType::kLiteral => self.visit_literal(node as *mut Literal),
                    AstNodeType::kRegExpLiteral => self.visit_reg_exp_literal(node as *mut RegExpLiteral),
                    AstNodeType::kObjectLiteral => self.visit_object_literal(node as *mut ObjectLiteral),
                    AstNodeType::kArrayLiteral => self.visit_array_literal(node as *mut ArrayLiteral),
                    AstNodeType::kVariableProxy => self.visit_variable_proxy(node as *mut VariableProxy),
                    AstNodeType::kAssignment => self.visit_assignment(node as *mut Assignment),
                    AstNodeType::kCompoundAssignment => self.visit_compound_assignment(node as *mut CompoundAssignment),
                    AstNodeType::kYield => self.visit_yield(node as *mut Yield),
                    AstNodeType::kYieldStar => self.visit_yield_star(node as *mut YieldStar),
                    AstNodeType::kAwait => self.visit_await(node as *mut Await),
                    AstNodeType::kThrow => self.visit_throw(node as *mut Throw),
                    AstNodeType::kOptionalChain => self.visit_optional_chain(node as *mut OptionalChain),
                    AstNodeType::kProperty => self.visit_property(node as *mut Property),
                    AstNodeType::kCall => self.visit_call(node as *mut Call),
                    AstNodeType::kCallNew => self.visit_call_new(node as *mut CallNew),
                    AstNodeType::kCallRuntime => self.visit_call_runtime(node as *mut CallRuntime),
                    AstNodeType::kSuperCallForwardArgs => self.visit_super_call_forward_args(node as *mut SuperCallForwardArgs),
                    AstNodeType::kUnaryOperation => self.visit_unary_operation(node as *mut UnaryOperation),
                    AstNodeType::kCountOperation => self.visit_count_operation(node as *mut CountOperation),
                    AstNodeType::kBinaryOperation => self.visit_binary_operation(node as *mut BinaryOperation),
                    AstNodeType::kNaryOperation => self.visit_nary_operation(node as *mut NaryOperation),
                    AstNodeType::kCompareOperation => self.visit_compare_operation(node as *mut CompareOperation),
                    AstNodeType::kSpread => self.visit_spread(node as *mut Spread),
                    AstNodeType::kEmptyParentheses => self.visit_empty_parentheses(node as *mut EmptyParentheses),
                    AstNodeType::kGetTemplateObject => self.visit_get_template_object(node as *mut GetTemplateObject),
                    AstNodeType::kTemplateLiteral => self.visit_template_literal(node as *mut TemplateLiteral),
                    AstNodeType::kImportCallExpression => self.visit_import_call_expression(node as *mut ImportCallExpression),
                    AstNodeType::kThisExpression => self.visit_this_expression(node as *mut ThisExpression),
                    AstNodeType::kSuperPropertyReference => self.visit_super_property_reference(node as *mut SuperPropertyReference),
                    AstNodeType::kSuperCallReference => self.visit_super_call_reference(node as *mut SuperCallReference),
                    _ => println!("Unknown AstNodeType {:?}", (*node).node_type()),
                }
            }
        }

        fn visit_block(&mut self, node: *mut Block) {
            self.find_statements(unsafe { (*node).statements() });
        }

        fn visit_variable_declaration(&mut self, _node: *mut VariableDeclaration) {}

        fn visit_function_declaration(&mut self, _node: *mut FunctionDeclaration) {}

        fn visit_expression_statement(&mut self, node: *mut ExpressionStatement) {
            unsafe {
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_empty_statement(&mut self, _node: *mut EmptyStatement) {}

        fn visit_sloppy_block_function_statement(
            &mut self,
            node: *mut SloppyBlockFunctionStatement,
        ) {
            unsafe {
                self.find((*node).statement() as *mut AstNode, false);
            }
        }

        fn visit_if_statement(&mut self, node: *mut IfStatement) {
            unsafe {
                self.find((*node).condition() as *mut AstNode, false);
                self.find((*node).then_statement() as *mut AstNode, false);
                if (*node).HasElseStatement() {
                    self.find((*node).else_statement() as *mut AstNode, false);
                }
            }
        }

        fn visit_continue_statement(&mut self, _node: *mut ContinueStatement) {}

        fn visit_break_statement(&mut self, _node: *mut BreakStatement) {}

        fn visit_return_statement(&mut self, node: *mut ReturnStatement) {
            unsafe {
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_with_statement(&mut self, node: *mut WithStatement) {
            unsafe {
                self.find((*node).expression() as *mut AstNode, false);
                self.find((*node).statement() as *mut AstNode, false);
            }
        }

        fn visit_switch_statement(&mut self, node: *mut SwitchStatement) {
            unsafe {
                self.find((*node).tag() as *mut AstNode, false);
                let cases = (*node).cases();
                if !cases.is_null() {
                    for i in 0..(*cases).length() {
                        let clause = (*cases).at(i);
                        if !(*clause).is_default() {
                            self.find((*clause).label() as *mut AstNode, false);
                        }
                        self.find_statements((*clause).statements());
                    }
                }
            }
        }

        fn visit_do_while_statement(&mut self, node: *mut DoWhileStatement) {
            unsafe {
                self.find((*node).body() as *mut AstNode, false);
                self.find((*node).cond() as *mut AstNode, false);
            }
        }

        fn visit_while_statement(&mut self, node: *mut WhileStatement) {
            unsafe {
                self.find((*node).cond() as *mut AstNode, false);
                self.find((*node).body() as *mut AstNode, false);
            }
        }

        fn visit_for_statement(&mut self, node: *mut ForStatement) {
            unsafe {
                if !(*node).init().is_null() {
                    self.find((*node).init() as *mut AstNode, false);
                }
                if !(*node).cond().is_null() {
                    self.find((*node).cond() as *mut AstNode, false);
                }
                if !(*node).next().is_null() {
                    self.find((*node).next() as *mut AstNode, false);
                }
                self.find((*node).body() as *mut AstNode, false);
            }
        }

        fn visit_for_in_statement(&mut self, node: *mut ForInStatement) {
            unsafe {
                self.find((*node).each() as *mut AstNode, false);
                self.find((*node).subject() as *mut AstNode, false);
                self.find((*node).body() as *mut AstNode, false);
            }
        }

        fn visit_for_of_statement(&mut self, node: *mut ForOfStatement) {
            unsafe {
                self.find((*node).each() as *mut AstNode, false);

                let was_found = if (*node).subject().position() == self.position {
                    self.is_async_iterator_error = (*node).type_() == IteratorType::kAsync;
                    self.is_iterator_error = !self.is_async_iterator_error;
                    let was_found = !self.found;
                    if was_found {
                        self.found = true;
                    }
                    was_found
                } else {
                    false
                };

                self.find((*node).subject() as *mut AstNode, true);

                if was_found {
                    self.done = true;
                    self.found = false;
                }

                self.find((*node).body() as *mut AstNode, false);
            }
        }

        fn visit_try_catch_statement(&mut self, node: *mut TryCatchStatement) {
            unsafe {
                self.find((*node).try_block() as *mut AstNode, false);
                self.find((*node).catch_block() as *mut AstNode, false);
            }
        }

        fn visit_try_finally_statement(&mut self, node: *mut TryFinallyStatement) {
            unsafe {
                self.find((*node).try_block() as *mut AstNode, false);
                self.find((*node).finally_block() as *mut AstNode, false);
            }
        }

        fn visit_debugger_statement(&mut self, _node: *mut DebuggerStatement) {}

        fn visit_function_literal(&mut self, node: *mut FunctionLiteral) {
            let last_function_kind = self.function_kind;
            unsafe {
                self.function_kind = (*node).kind();
                self.find_statements((*node).body());
                self.function_kind = last_function_kind;
            }
        }

        fn visit_class_literal(&mut self, node: *mut ClassLiteral) {
            unsafe {
                if !(*node).extends().is_null() {
                    self.find((*node).extends() as *mut AstNode, false);
                }

                let public_members = (*node).public_members();
                if !public_members.is_null() {
                    for i in 0..(*public_members).length() {
                        self.find((*public_members).at(i).value() as *mut AstNode, false);
                    }
                }

                let private_members = (*node).private_members();
                if !private_members.is_null() {
                    for i in 0..(*private_members).length() {
                        self.find((*private_members).at(i).value() as *mut AstNode, false);
                    }
                }
            }
        }

        fn visit_initialize_class_members_statement(
            &mut self,
            node: *mut InitializeClassMembersStatement,
        ) {
            unsafe {
                let fields = (*node).fields();
                if !fields.is_null() {
                    for i in 0..(*fields).length() {
                        self.find((*fields).at(i).value() as *mut AstNode, false);
                    }
                }
            }
        }

        fn visit_initialize_class_static_elements_statement(
            &mut self,
            node: *mut InitializeClassStaticElementsStatement,
        ) {
            unsafe {
                let elements = (*node).elements();
                if !elements.is_null() {
                    for i in 0..(*elements).length() {
                        let element = (*elements).at(i);
                        if (*element).kind() == ClassLiteral::StaticElement::PROPERTY {
                            self.find((*element).property().value() as *mut AstNode, false);
                        } else {
                            self.find((*element).static_block() as *mut AstNode, false);
                        }
                    }
                }
            }
        }

        fn visit_auto_accessor_getter_body(&mut self, _node: *mut AutoAccessorGetterBody) {}

        fn visit_auto_accessor_setter_body(&mut self, _node: *mut AutoAccessorSetterBody) {}

        fn visit_native_function_literal(&mut self, _node: *mut NativeFunctionLiteral) {}

        fn visit_conditional_chain(&mut self, node: *mut ConditionalChain) {
            unsafe {
                for i in 0..(*node).conditional_chain_length() {
                    self.find((*node).condition_at(i) as *mut AstNode, false);
                    self.find((*node).then_expression_at(i) as *mut AstNode, false);
                }
                self.find((*node).else_expression() as *mut AstNode, false);
            }
        }

        fn visit_conditional(&mut self, node: *mut Conditional) {
            unsafe {
                self.find((*node).condition() as *mut AstNode, false);
                self.find((*node).then_expression() as *mut AstNode, false);
                self.find((*node).else_expression() as *mut AstNode, false);
            }
        }

        fn visit_literal(&mut self, node: *mut Literal) {
             unsafe {
                let value = (*node).build_value(self.isolate);
                self.print_literal_direct_handle(value, true);
            }
        }

        fn visit_reg_exp_literal(&mut self, node: *mut RegExpLiteral) {
            self.print_char('/');
            unsafe {
                self.print_literal_ast_raw_string((*node).pattern(), false);
                self.print_char('/');
                // #define V(Lower, Camel, LowerCamel, Char, Bit) \
                // if (node->flags() & RegExp::k##Camel) Print(Char);
                // REGEXP_FLAG_LIST(V)
                // #undef V
            }
        }

        fn visit_object_literal(&mut self, node: *mut ObjectLiteral) {
            self.print_char('{');
            unsafe {
                let properties = (*node).properties();
                if !properties.is_null() {
                    for i in 0..(*properties).length() {
                        self.find((*properties).at(i).value() as *mut AstNode, false);
                    }
                }
            }
            self.print_char('}');
        }

        fn visit_array_literal(&mut self, node: *mut ArrayLiteral) {
            self.print_char('[');
            unsafe {
                let values = (*node).values();
                if !values.is_null() {
                    for i in 0..(*values).length() {
                        if i != 0 {
                            self.print_char(',');
                        }
                        let subexpr = (*values).at(i);
                        let spread = subexpr.cast::<Spread>();
                        if !spread.is_null() && !self.found && self.position == (*spread).expression().position() {
                            self.found = true;
                            self.is_iterator_error = true;
                            self.find((*spread).expression() as *mut AstNode, true);
                            self.done = true;
                            return;
                        }
                        self.find(subexpr as *mut AstNode, true);
                    }
                }
            }
            self.print_char(']');
        }

        fn visit_variable_proxy(&mut self, node: *mut VariableProxy) {
             unsafe {
                if self.is_user_js {
                    self.print_literal_ast_raw_string((*node).name(), false);
                } else {
                    self.print_str("(var)");
                }
            }
        }

        fn visit_assignment(&mut self, node: *mut Assignment) {
           unsafe {
                let was_found = if (*node).target().is_object_literal() {
                    let target = (*node).target().cast::<ObjectLiteral>();
                    if target.is_null() {
                        false
                    } else if (*target).position() == self.position {
                        let was_found = !self.found;
                        self.found = true;
                        self.destructuring_assignment = node;
                        was_found
                    } else {
                        let mut was_found = false;
                        let properties = (*target).properties();
                        if !properties.is_null() {
                            for i in 0..(*properties).length() {
                                let prop = (*properties).at(i);
                                if (*prop).value().position() == self.position {
                                    was_found = !self.found;
                                    self.found = true;
                                    self.destructuring_prop = prop;
                                    self.destructuring_assignment = node;
                                    break;
                                }
                            }
                        }
                        was_found
                    }
                } else {
                    false
                };

                if !was_found {
                    if self.found {
                        self.find((*node).target() as *mut AstNode, true);
                        return;
                    }
                    self.find((*node).target() as *mut AstNode, false);
                    if (*node).target().is_array_literal() {
                        if (*node).value().position() == self.position {
                            self.is_iterator_error = true;
                        }
                        self.find((*node).value() as *mut AstNode, true);
                    } else {
                        self.find((*node).value() as *mut AstNode, false);
                    }
                } else {
                    self.find((*node).value() as *mut AstNode, true);
                }

                if was_found {
                    self.done = true;
                    self.found = false;
                }
            }
        }

        fn visit_compound_assignment(&mut self, node: *mut CompoundAssignment) {
            self.visit_assignment(node as *mut Assignment);
        }

        fn visit_yield(&mut self, node: *mut Yield) {
             unsafe {
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_yield_star(&mut self, node: *mut YieldStar) {
              unsafe {
                if !self.found && self.position == (*node).expression().position() {
                    self.found = true;
                    if is_async_function(self.function_kind) {
                        self.is_async_iterator_error = true;
                    } else {
                        self.is_iterator_error = true;
                    }
                    self.print_str("yield* ");
                }
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_await(&mut self, node: *mut Await) {
            unsafe {
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_throw(&mut self, node: *mut Throw) {
             unsafe {
                self.find((*node).exception() as *mut AstNode, false);
            }
        }

        fn visit_optional_chain(&mut self, node: *mut OptionalChain) {
           unsafe {
                self.find((*node).expression() as *mut AstNode, false);
            }
        }

        fn visit_property(&mut self, node: *mut Property) {
            unsafe {
                let key = (*node).key();
                let literal = key.cast::<Literal>();
                if !literal.is_null() {
                    // if (IsInternalizedString(*literal->BuildValue(isolate_))) {
                     let value = (*literal).build_value(self.isolate);
                     let key_str = value;
                       self.find((*node).obj() as *mut AstNode, true);

                         if (*node).is_optional_chain_link() {
                           self.print_char('?');
                         }
                       self.print_char('.');

                     self.print_literal_direct_handle(key_str, false);
                  //  }
                } else {
                    self.find((*node).obj() as *mut AstNode, true);
                    if (*node).is_optional_chain_link() {
                         self.print_str("?.");
                      }
                    self.print_char('[');
                    self.find(key as *mut AstNode, true);
                    self.print_char(']');
                }
            }
        }

        fn visit_call(&mut self, node: *mut Call) {
           unsafe {
                let was_found = if (*node).position() == self.position {
                    if self.error_in_spread_args == SpreadErrorInArgsHint::kErrorInArgs && !(*node).arguments().is_null() && (*(*node).arguments()).length() > 0
                    {
                        if let Some(last_arg) = (*(*node).arguments()).last() {
                            let spread = last_arg.cast::<Spread>();
                            if !spread.is_null() {
                                self.found = true;
                                self.spread_arg = (*spread).expression();
                                self.find(self.spread_arg as *mut AstNode, true);
                                self.done = true;
                                self.found = false;
                                return;
                            }
                        }
                    }
                    self.is_call_error = true;
                    !self.found
                } else {
                    false
                };

                if was_found {
                    if !self.is_user_js && (*node).expression().is_variable_proxy() {
                        self.done = true;
                        return;
                    }
                    self.found = true;
                }

                self.find((*node).expression() as *mut AstNode, true);
                if !was_found && !self.is_iterator_error {
                    self.print_str("(...)");
                }

                self.find_arguments((*node).arguments());
                if was_found {
                    self.done = true;
                    self.found = false;
                }
            }
        }

        fn visit_call_new(&mut self, node: *mut CallNew) {
           unsafe {
                let was_found = if (*node).position() == self.position {
                    if self.error_in_spread_args == SpreadErrorInArgsHint::kErrorInArgs && !(*node).arguments().is_null() && (*(*node).arguments()).length() > 0
                    {
                        if let Some(last_arg) = (*(*node).arguments()).last() {
                            let spread = last_arg.cast::<Spread>();
                            if !spread.is_null() {
                                self.found = true;
                                self.spread_arg = (*spread).expression();
                                self.find(self.spread_arg as *mut AstNode, true);
                                self.done = true;
                                self.found = false;
                                return;
                            }
                        }
                    }
                    self.is_call_error = true;
                    !self.found
                } else {
                    false
                };

                if was_found {
                    if !self.is_user_js && (*node).expression().is_variable_proxy() {
                        self.done = true;
                        return;
                    }
                    self.found = true;
                }

                self.find((*node).expression() as *mut AstNode, was_found || self.is_iterator_error);
                self.find_arguments((*node).arguments());
                if was_found {
                    self.done = true;
                    self.found = false;
                }
            }
        }

        fn visit_call_runtime(&mut self, node: *mut CallRuntime) {
            unsafe {
                self.find_arguments((*node).arguments());
            }
        }

        fn visit_super_call_forward_args(&mut self, node: *mut SuperCallForwardArgs) {
            unsafe {
                self.find((*node).expression() as *mut AstNode, true);
            }
            self.print_str("(...forwarded args...)");
        }

        fn visit_unary_operation(&mut self, node: *mut UnaryOperation) {
             unsafe {
                let op = (*node).op();
                let needs_space =
                    op == Token::Value::kDelete || op == Token::Value::kTypeOf || op == Token::Value::kVoid;
                self.print_char('(');
                self.print_str(Token::string(op));
                if needs_space {
                    self.print_char(' ');
                }
                self.find((*node).expression() as *mut AstNode, true);
                self.print_char(')');
            }
        }

        fn visit_count_operation(&mut self, node: *mut CountOperation) {
            unsafe {
                self.print_char('(');
                if (*node).is_prefix() {
                    self.print_str(Token::string((*node).op()));
                }
                self.find((*node).expression() as *mut AstNode, true);
                if (*node).is_postfix() {
                    self.print_str(Token::string((*node).op()));
                }
                self.print_char(')');
            }
        }

        fn visit_binary_operation(&mut self, node: *mut BinaryOperation) {
           unsafe {
                self.print_char('(');
                self.find((*node).left() as *mut AstNode, true);
                self.print_char(' ');
                self.print_str(Token::string((*node).op()));
                self.print_char(' ');
                self.find((*node).right() as *mut AstNode, true);
                self.print_char(')');
            }
        }

        
