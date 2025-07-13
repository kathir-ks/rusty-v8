// Converted from V8 C++ source files:
// Header: ast-function-literal-id-reindexer.h
// Implementation: ast-function-literal-id-reindexer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

//use crate::ast::ast::{Expression, FunctionLiteral, Call, ClassLiteral, ClassLiteralProperty};
//use crate::ast::ast_traversal_visitor::AstTraversalVisitor;

// Dummy implementations for types from other files
pub struct Expression {}
pub struct FunctionLiteral {
    id: i32,
}
impl FunctionLiteral {
    pub fn function_literal_id(&self) -> i32 {
        self.id
    }
    pub fn set_function_literal_id(&mut self, id: i32) {
        self.id = id;
    }
}
pub struct Call {
    possibly_eval: bool,
}
impl Call {
    pub fn is_possibly_eval(&self) -> bool {
        self.possibly_eval
    }
    pub fn adjust_eval_scope_info_index(&mut self, _delta: i32) {}
}
pub struct ClassLiteral {}
impl ClassLiteral {
    pub fn extends(&self) -> *const Expression {
        std::ptr::null()
    }
    pub fn constructor(&self) -> *const Expression {
        std::ptr::null()
    }
    pub fn static_initializer(&self) -> *const Expression {
        std::ptr::null()
    }
    pub fn instance_members_initializer_function(&self) -> *const Expression {
        std::ptr::null()
    }
    pub fn private_members(&self) -> *const ZonePtrList<ClassLiteralProperty> {
        std::ptr::null()
    }
    pub fn public_members(&self) -> *const ZonePtrList<ClassLiteralProperty> {
        std::ptr::null()
    }
}
pub struct ClassLiteralProperty {}
impl ClassLiteralProperty {
    pub fn kind(&self) -> ClassLiteralPropertyKind {
        ClassLiteralPropertyKind::FIELD
    }
    pub fn value(&self) -> *mut Expression {
        std::ptr::null_mut()
    }
    pub fn key(&self) -> *mut Expression {
        std::ptr::null_mut()
    }

    pub fn is_computed_name(&self) -> bool {
        false
    }
}

pub enum ClassLiteralPropertyKind {
    FIELD,
    METHOD,
}

pub struct ZonePtrList<T> {
    length: usize,
}

impl<T> ZonePtrList<T> {
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn at(&self, _i: usize) -> *mut T {
        std::ptr::null_mut()
    }
}
// Dummy implementations for types from other files
pub trait AstTraversalVisitorTrait {
    fn visit_function_literal(&mut self, lit: *mut FunctionLiteral);
    fn visit_class_literal(&mut self, lit: *mut ClassLiteral);
    fn visit_call(&mut self, lit: *mut Call);
    fn visit(&mut self, expr: *mut Expression);
    fn stack_limit(&self) -> usize;
}

struct AstTraversalVisitorData {
    stack_limit: usize,
}

impl AstTraversalVisitorData {
    fn new(stack_limit: usize) -> Self {
        AstTraversalVisitorData { stack_limit }
    }
}

pub struct AstTraversalVisitor<T>
where
    T: AstTraversalVisitorTrait,
{
    data: AstTraversalVisitorData,
    visitor: RefCell<T>,
}

impl<T> AstTraversalVisitor<T>
where
    T: AstTraversalVisitorTrait,
{
    pub fn new(stack_limit: usize, visitor: T) -> Self {
        AstTraversalVisitor {
            data: AstTraversalVisitorData::new(stack_limit),
            visitor: RefCell::new(visitor),
        }
    }

    fn visit(&self, expr: *mut Expression) {
        self.visitor.borrow_mut().visit(expr);
    }

    fn visit_function_literal(&self, lit: *mut FunctionLiteral) {
        self.visitor.borrow_mut().visit_function_literal(lit);
    }

    fn visit_class_literal(&self, lit: *mut ClassLiteral) {
        self.visitor.borrow_mut().visit_class_literal(lit);
    }

    fn visit_call(&self, expr: *mut Call) {
        self.visitor.borrow_mut().visit_call(expr);
    }

    fn stack_limit(&self) -> usize {
        self.data.stack_limit
    }
}

// Changes the ID of all FunctionLiterals in the given Expression by adding the
// given delta.
pub struct AstFunctionLiteralIdReindexer {
    delta_: i32,
    stack_limit_: usize,
    visited_: RefCell<HashSet<*mut FunctionLiteral>>,
}

impl AstFunctionLiteralIdReindexer {
    pub fn new(stack_limit: usize, delta: i32) -> Self {
        AstFunctionLiteralIdReindexer {
            delta_: delta,
            stack_limit_: stack_limit,
            visited_: RefCell::new(HashSet::new()),
        }
    }

    pub fn reindex(&self, pattern: *mut Expression) {
        self.visited_.borrow_mut().clear();
        self.visit(pattern);
        self.check_visited(pattern);
    }

    fn check_visited(&self, expr: *mut Expression) {
        AstFunctionLiteralIdReindexChecker::new(self.stack_limit(), &self.visited_.borrow()).visit(expr);
    }
}

impl AstTraversalVisitorTrait for AstFunctionLiteralIdReindexer {
    fn visit_function_literal(&mut self, lit: *mut FunctionLiteral) {
        unsafe {
            if !self.visited_.borrow_mut().insert(lit) {
                println!("Already visited");
                return;
            }
            let lit_ref = &mut *lit;
            let new_id = lit_ref.function_literal_id() + self.delta_;
            lit_ref.set_function_literal_id(new_id);
        }
    }

    fn visit_class_literal(&mut self, expr: *mut ClassLiteral) {
        unsafe {
            let expr_ref = &*expr;

            if expr_ref.extends() != std::ptr::null() {
                self.visit(expr_ref.extends() as *mut Expression);
            }
            if expr_ref.constructor() != std::ptr::null() {
                self.visit(expr_ref.constructor() as *mut Expression);
            }
            if expr_ref.static_initializer() != std::ptr::null() {
                self.visit(expr_ref.static_initializer() as *mut Expression);
            }
            if expr_ref.instance_members_initializer_function() != std::ptr::null() {
                self.visit(expr_ref.instance_members_initializer_function() as *mut Expression);
            }

            let private_members = expr_ref.private_members();
            if private_members != std::ptr::null() {
                let private_members_ref = &*private_members;
                for i in 0..private_members_ref.length() {
                    let prop = private_members_ref.at(i);
                    if (&*prop).kind() == ClassLiteralPropertyKind::FIELD {
                        self.check_visited((&*prop).value());
                    } else {
                        self.visit((&*prop).value());
                    }
                }
            }

            let props = expr_ref.public_members();
            if props != std::ptr::null() {
                let props_ref = &*props;
                for i in 0..props_ref.length() {
                    let prop = props_ref.at(i);

                    if (&*prop).is_computed_name() && (&*prop).kind() == ClassLiteralPropertyKind::FIELD {
                        self.check_visited((&*prop).key());
                        self.check_visited((&*prop).value());
                    } else {
                        self.visit((&*prop).key());
                        self.visit((&*prop).value());
                    }
                }
            }
        }
    }

    fn visit_call(&mut self, expr: *mut Call) {
        unsafe {
            let expr_ref = &mut *expr;
            if expr_ref.is_possibly_eval() {
                expr_ref.adjust_eval_scope_info_index(self.delta_);
            }
        }
    }

    fn visit(&mut self, expr: *mut Expression) {}
    fn stack_limit(&self) -> usize {
        self.stack_limit_
    }
}

struct AstFunctionLiteralIdReindexChecker<'a> {
    stack_limit_: usize,
    visited_: &'a RefCell<HashSet<*mut FunctionLiteral>>,
}

impl<'a> AstFunctionLiteralIdReindexChecker<'a> {
    fn new(stack_limit: usize, visited: &'a RefCell<HashSet<*mut FunctionLiteral>>) -> Self {
        AstFunctionLiteralIdReindexChecker {
            stack_limit_: stack_limit,
            visited_: visited,
        }
    }

    fn visit(&self, expr: *mut Expression) {}
}

impl<'a> AstTraversalVisitorTrait for AstFunctionLiteralIdReindexChecker<'a> {
    fn visit_function_literal(&mut self, lit: *mut FunctionLiteral) {
        let visited = self.visited_.borrow();
        if !visited.contains(&lit) {
           println!("Unvisited function literal found.");
        }
    }

    fn visit_class_literal(&mut self, lit: *mut ClassLiteral) {}
    fn visit_call(&mut self, lit: *mut Call) {}
    fn visit(&mut self, expr: *mut Expression) {}
    fn stack_limit(&self) -> usize {
        self.stack_limit_
    }
}
