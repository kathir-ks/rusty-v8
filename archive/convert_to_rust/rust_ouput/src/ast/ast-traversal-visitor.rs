// Converted from V8 C++ source files:
// Header: ast-traversal-visitor.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ast_traversal_visitor {
    use crate::ast::ast::*;
    use crate::ast::scopes::*;
    use crate::execution::isolate::*;

    pub trait AstTraversalVisitorSubclass {
        fn visit_node(&mut self, node: *mut AstNode) -> bool;
        fn visit_expression(&mut self, node: *mut Expression) -> bool;
    }

    pub struct AstTraversalVisitor<Subclass: AstTraversalVisitorSubclass> {
        root_: *mut AstNode,
        depth_: i32,
        isolate: *mut Isolate, // Placeholder.  Replace with proper Isolate type if available
        impl_: Subclass,
        stack_limit: Option<usize>,
    }

    impl<Subclass: AstTraversalVisitorSubclass> AstTraversalVisitor<Subclass> {
        pub fn new(isolate: *mut Isolate, root: *mut AstNode, impl_: Subclass) -> Self {
            AstTraversalVisitor {
                root_: root,
                depth_: 0,
                isolate: isolate,
                impl_: impl_,
                stack_limit: None,
            }
        }

        pub fn new_with_stack_limit(stack_limit: usize, root: *mut AstNode, impl_: Subclass) -> Self {
            AstTraversalVisitor {
                root_: root,
                depth_: 0,
                isolate: std::ptr::null_mut(), // Assuming isolate is not needed when stack_limit is provided
                impl_: impl_,
                stack_limit: Some(stack_limit),
            }
        }
        
        fn initialize_ast_visitor(&mut self, isolate: *mut Isolate) {
            // Placeholder for actual initialization logic
            self.isolate = isolate;
        }

        fn initialize_ast_visitor_with_stack_limit(&mut self, stack_limit: usize) {
            // Placeholder for actual initialization logic
            self.stack_limit = Some(stack_limit);
        }

        pub fn run(&mut self) {
            if self.root_.is_null() {
                return;
            }
            self.visit(self.root_);
        }

        pub fn visit_node(&mut self, node: *mut AstNode) -> bool {
            self.impl_.visit_node(node)
        }

        pub fn visit_expression(&mut self, node: *mut Expression) -> bool {
            self.impl_.visit_expression(node)
        }

        pub fn visit_declarations(&mut self, declarations: *mut DeclarationList) {
            if declarations.is_null() {
                return;
            }
            let decls = unsafe { &*declarations };
            for decl in &decls.list {
                self.visit(*decl);
            }
        }

        pub fn visit_statements(&mut self, statements: *const ZonePtrList<Statement>) {
            if statements.is_null() {
                return;
            }

            let stmts = unsafe { &*statements };
            for i in 0..stmts.length() {
                let stmt = stmts.at(i);
                self.visit(stmt);
            }
        }

        pub fn visit_variable_declaration(&mut self, decl: *mut VariableDeclaration) {
            unsafe {
                if !self.impl_.visit_node(decl as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_function_declaration(&mut self, decl: *mut FunctionDeclaration) {
            unsafe {
                if !self.impl_.visit_node(decl as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*decl).fun() });
        }

        pub fn visit_block(&mut self, stmt: *mut Block) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }

            let block = unsafe { &*stmt };

            if !block.scope().is_null() {
                self.depth_ += 1;
                self.visit_declarations(unsafe { (*block.scope()).declarations() });
                self.depth_ -= 1;
            }

            self.visit_statements(block.statements());
        }

        pub fn visit_expression_statement(&mut self, stmt: *mut ExpressionStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).expression() });
        }

        pub fn visit_empty_statement(&mut self, _stmt: *mut EmptyStatement) {}

        pub fn visit_sloppy_block_function_statement(&mut self, stmt: *mut SloppyBlockFunctionStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).statement() });
        }

        pub fn visit_if_statement(&mut self, stmt: *mut IfStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).condition() });
            self.visit(unsafe { (*stmt).then_statement() });
            self.visit(unsafe { (*stmt).else_statement() });
        }

        pub fn visit_continue_statement(&mut self, stmt: *mut ContinueStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_break_statement(&mut self, stmt: *mut BreakStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_return_statement(&mut self, stmt: *mut ReturnStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).expression() });
        }

        pub fn visit_with_statement(&mut self, stmt: *mut WithStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).expression() });
            self.visit(unsafe { (*stmt).statement() });
        }

        pub fn visit_switch_statement(&mut self, stmt: *mut SwitchStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).tag() });

            let clauses = unsafe { (*stmt).cases() };
            for i in 0..unsafe { (*clauses).length() } {
                let clause = unsafe { (*clauses).at(i) };
                if !unsafe { (*clause).is_default() } {
                    let label = unsafe { (*clause).label() };
                    self.visit(label);
                }
                let stmts = unsafe { (*clause).statements() };
                self.visit_statements(stmts);
            }
        }

        pub fn visit_do_while_statement(&mut self, stmt: *mut DoWhileStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).body() });
            self.visit(unsafe { (*stmt).cond() });
        }

        pub fn visit_while_statement(&mut self, stmt: *mut WhileStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).cond() });
            self.visit(unsafe { (*stmt).body() });
        }

        pub fn visit_for_statement(&mut self, stmt: *mut ForStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            if !unsafe { (*stmt).init() }.is_null() {
                self.visit(unsafe { (*stmt).init() });
            }
            if !unsafe { (*stmt).cond() }.is_null() {
                self.visit(unsafe { (*stmt).cond() });
            }
            if !unsafe { (*stmt).next() }.is_null() {
                self.visit(unsafe { (*stmt).next() });
            }
            self.visit(unsafe { (*stmt).body() });
        }

        pub fn visit_for_in_statement(&mut self, stmt: *mut ForInStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).each() });
            self.visit(unsafe { (*stmt).subject() });
            self.visit(unsafe { (*stmt).body() });
        }

        pub fn visit_for_of_statement(&mut self, stmt: *mut ForOfStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).each() });
            self.visit(unsafe { (*stmt).subject() });
            self.visit(unsafe { (*stmt).body() });
        }

        pub fn visit_try_catch_statement(&mut self, stmt: *mut TryCatchStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).try_block() });
            self.visit(unsafe { (*stmt).catch_block() });
        }

        pub fn visit_try_finally_statement(&mut self, stmt: *mut TryFinallyStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
            self.visit(unsafe { (*stmt).try_block() });
            self.visit(unsafe { (*stmt).finally_block() });
        }

        pub fn visit_debugger_statement(&mut self, stmt: *mut DebuggerStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_function_literal(&mut self, expr: *mut FunctionLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }

            let scope = unsafe { (*expr).scope() };
            self.depth_ += 1;
            self.visit_declarations(unsafe { (*scope).declarations() });
            self.depth_ -= 1;

            if unsafe { (*expr).scope() }.was_lazily_parsed() {
                return;
            }

            self.depth_ += 1;
            self.visit_statements(unsafe { (*expr).body() });
            self.depth_ -= 1;
        }

        pub fn visit_native_function_literal(&mut self, expr: *mut NativeFunctionLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_conditional_chain(&mut self, expr: *mut ConditionalChain) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            for i in 0..unsafe { (*expr).conditional_chain_length() } {
                self.depth_ += 1;
                self.visit(unsafe { (*expr).condition_at(i) });
                self.visit(unsafe { (*expr).then_expression_at(i) });
                self.depth_ -= 1;
            }
            self.visit(unsafe { (*expr).else_expression() });
        }

        pub fn visit_conditional(&mut self, expr: *mut Conditional) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).condition() });
            self.visit(unsafe { (*expr).then_expression() });
            self.visit(unsafe { (*expr).else_expression() });
            self.depth_ -= 1;
        }

        pub fn visit_variable_proxy(&mut self, expr: *mut VariableProxy) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_literal(&mut self, expr: *mut Literal) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_reg_exp_literal(&mut self, expr: *mut RegExpLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_object_literal(&mut self, expr: *mut ObjectLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            let props = unsafe { (*expr).properties() };
            for i in 0..unsafe { (*props).length() } {
                let prop = unsafe { (*props).at(i) };
                self.depth_ += 1;
                self.visit(unsafe { (*prop).key() });
                self.visit(unsafe { (*prop).value() });
                self.depth_ -= 1;
            }
        }

        pub fn visit_array_literal(&mut self, expr: *mut ArrayLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            let values = unsafe { (*expr).values() };
            for i in 0..unsafe { (*values).length() } {
                let value = unsafe { (*values).at(i) };
                self.depth_ += 1;
                self.visit(value);
                self.depth_ -= 1;
            }
        }

        pub fn visit_assignment(&mut self, expr: *mut Assignment) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).target() });
            self.visit(unsafe { (*expr).value() });
            self.depth_ -= 1;
        }

        pub fn visit_compound_assignment(&mut self, expr: *mut CompoundAssignment) {
            self.visit_assignment(expr);
        }

        pub fn visit_yield(&mut self, expr: *mut Yield) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_yield_star(&mut self, expr: *mut YieldStar) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_await(&mut self, expr: *mut Await) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_throw(&mut self, expr: *mut Throw) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).exception() });
            self.depth_ -= 1;
        }

        pub fn visit_optional_chain(&mut self, expr: *mut OptionalChain) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_property(&mut self, expr: *mut Property) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).obj() });
            self.visit(unsafe { (*expr).key() });
            self.depth_ -= 1;
        }

        pub fn visit_call(&mut self, expr: *mut Call) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            let args = unsafe { (*expr).arguments() };
            for i in 0..unsafe { (*args).length() } {
                let arg = unsafe { (*args).at(i) };
                self.visit(arg);
            }
            self.depth_ -= 1;
        }

        pub fn visit_call_new(&mut self, expr: *mut CallNew) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            let args = unsafe { (*expr).arguments() };
            for i in 0..unsafe { (*args).length() } {
                let arg = unsafe { (*args).at(i) };
                self.visit(arg);
            }
            self.depth_ -= 1;
        }

        pub fn visit_call_runtime(&mut self, expr: *mut CallRuntime) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            let args = unsafe { (*expr).arguments() };
            for i in 0..unsafe { (*args).length() } {
                let arg = unsafe { (*args).at(i) };
                self.depth_ += 1;
                self.visit(arg);
                self.depth_ -= 1;
            }
        }

        pub fn visit_unary_operation(&mut self, expr: *mut UnaryOperation) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_count_operation(&mut self, expr: *mut CountOperation) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_binary_operation(&mut self, expr: *mut BinaryOperation) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).left() });
            self.visit(unsafe { (*expr).right() });
            self.depth_ -= 1;
        }

        pub fn visit_nary_operation(&mut self, expr: *mut NaryOperation) {
            unsafe {
                if (!self.impl_.visit_node(expr as *mut AstNode)) {
                    return;
                }
                if (!self.impl_.visit_expression(expr as *mut Expression)) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).first() });
            for i in 0..unsafe { (*expr).subsequent_length() } {
                self.visit(unsafe { (*expr).subsequent(i) });
            }
            self.depth_ -= 1;
        }

        pub fn visit_compare_operation(&mut self, expr: *mut CompareOperation) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).left() });
            self.visit(unsafe { (*expr).right() });
            self.depth_ -= 1;
        }

        pub fn visit_this_expression(&mut self, expr: *mut ThisExpression) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_class_literal(&mut self, expr: *mut ClassLiteral) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }

            if !unsafe { (*expr).extends() }.is_null() {
                self.depth_ += 1;
                self.visit(unsafe { (*expr).extends() });
                self.depth_ -= 1;
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).constructor() });
            self.depth_ -= 1;

            if !unsafe { (*expr).static_initializer() }.is_null() {
                self.depth_ += 1;
                self.visit(unsafe { (*expr).static_initializer() });
                self.depth_ -= 1;
            }

            if !unsafe { (*expr).instance_members_initializer_function() }.is_null() {
                self.depth_ += 1;
                self.visit(unsafe { (*expr).instance_members_initializer_function() });
                self.depth_ -= 1;
            }

            let private_members = unsafe { (*expr).private_members() };
            for i in 0..unsafe { (*private_members).length() } {
                let prop = unsafe { (*private_members).at(i) };
                self.depth_ += 1;
                self.visit(unsafe { (*prop).value() });
                self.depth_ -= 1;
            }

            let props = unsafe { (*expr).public_members() };
            for i in 0..unsafe { (*props).length() } {
                let prop = unsafe { (*props).at(i) };
                if !unsafe { (*prop).key() }.is_literal() {
                    self.depth_ += 1;
                    self.visit(unsafe { (*prop).key() });
                    self.depth_ -= 1;
                }
                self.depth_ += 1;
                self.visit(unsafe { (*prop).value() });
                self.depth_ -= 1;
            }
        }

        pub fn visit_initialize_class_members_statement(&mut self, stmt: *mut InitializeClassMembersStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }

            let props = unsafe { (*stmt).fields() };
            for i in 0..unsafe { (*props).length() } {
                let prop = unsafe { (*props).at(i) };
                if !unsafe { (*prop).key() }.is_literal() {
                    self.visit(unsafe { (*prop).key() });
                }
                self.visit(unsafe { (*prop).value() });
            }
        }

        pub fn visit_initialize_class_static_elements_statement(&mut self, stmt: *mut InitializeClassStaticElementsStatement) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }

            let elements = unsafe { (*stmt).elements() };
            for i in 0..unsafe { (*elements).length() } {
                let element = unsafe { (*elements).at(i) };
                match unsafe { (*element).kind() } {
                    ClassLiteralStaticElementKind::PROPERTY => {
                        let prop = unsafe { (*element).property() };
                        if !unsafe { (*prop).key() }.is_literal() {
                            self.visit(unsafe { (*prop).key() });
                        }
                        self.visit(unsafe { (*prop).value() });
                    }
                    ClassLiteralStaticElementKind::STATIC_BLOCK => {
                        self.visit(unsafe { (*element).static_block() });
                    }
                }
            }
        }

        pub fn visit_auto_accessor_getter_body(&mut self, stmt: *mut AutoAccessorGetterBody) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_auto_accessor_setter_body(&mut self, stmt: *mut AutoAccessorSetterBody) {
            unsafe {
                if !self.impl_.visit_node(stmt as *mut AstNode) {
                    return;
                }
            }
        }

        pub fn visit_spread(&mut self, expr: *mut Spread) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        pub fn visit_empty_parentheses(&mut self, expr: *mut EmptyParentheses) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_get_template_object(&mut self, expr: *mut GetTemplateObject) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_template_literal(&mut self, expr: *mut TemplateLiteral) {
            unsafe {
                if (!self.impl_.visit_node(expr as *mut AstNode)) {
                    return;
                }
                if (!self.impl_.visit_expression(expr as *mut Expression)) {
                    return;
                }
            }
            let substitutions = unsafe { (*expr).substitutions() };
            for sub in unsafe { &*substitutions }.iter() {
                self.depth_ += 1;
                self.visit(*sub);
                self.depth_ -= 1;
            }
        }

        pub fn visit_import_call_expression(&mut self, expr: *mut ImportCallExpression) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).specifier() });
            if !unsafe { (*expr).import_options() }.is_null() {
                self.visit(unsafe { (*expr).import_options() });
            }
            self.depth_ -= 1;
        }

        pub fn visit_super_property_reference(&mut self, expr: *mut SuperPropertyReference) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
        }

        pub fn visit_super_call_reference(&mut self, expr: *mut SuperCallReference) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit_variable_proxy(unsafe { (*expr).new_target_var() });
            self.visit_variable_proxy(unsafe { (*expr).this_function_var() });
            self.depth_ -= 1;
        }

        pub fn visit_super_call_forward_args(&mut self, expr: *mut SuperCallForwardArgs) {
            unsafe {
                if !self.impl_.visit_node(expr as *mut AstNode) {
                    return;
                }
                if !self.impl_.visit_expression(expr as *mut Expression) {
                    return;
                }
            }
            self.depth_ += 1;
            self.visit(unsafe { (*expr).expression() });
            self.depth_ -= 1;
        }

        fn visit(&mut self, node: *mut AstNode) {
            if node.is_null() {
                return;
            }
            unsafe {
                match (*node).node_type() {
                    AstNodeEnum::kVariableDeclaration => self.visit_variable_declaration(node as *mut VariableDeclaration),
                    AstNodeEnum::kFunctionDeclaration => self.visit_function_declaration(node as *mut FunctionDeclaration),
                    AstNodeEnum::kBlock => self.visit_block(node as *mut Block),
                    AstNodeEnum::kExpressionStatement => self.visit_expression_statement(node as *mut ExpressionStatement),
                    AstNodeEnum::kEmptyStatement => self.visit_empty_statement(node as *mut EmptyStatement),
                    AstNodeEnum::kSloppyBlockFunctionStatement => self.visit_sloppy_block_function_statement(node as *mut SloppyBlockFunctionStatement),
                    AstNodeEnum::kIfStatement => self.visit_if_statement(node as *mut IfStatement),
                    AstNodeEnum::kContinueStatement => self.visit_continue_statement(node as *mut ContinueStatement),
                    AstNodeEnum::kBreakStatement => self.visit_break_statement(node as *mut BreakStatement),
                    AstNodeEnum::kReturnStatement => self.visit_return_statement(node as *mut ReturnStatement),
                    AstNodeEnum::kWithStatement => self.visit_with_statement(node as
