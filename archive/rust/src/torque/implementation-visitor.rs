// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Placeholder types for V8 internal dependencies.
// Replace with actual implementations if available.
pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! DCHECK {
            ($condition:expr) => {
                if !($condition) {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
            ($condition:expr, $($arg:tt)*) => {
                if !($condition) {
                    panic!("DCHECK failed: {}: {}", stringify!($condition), format_args!($($arg)*));
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_EQ {
            ($left:expr, $right:expr) => {
                if $left != $right {
                    panic!("DCHECK_EQ failed: {} != {}", $left, $right);
                }
            };
            ($left:expr, $right:expr, $($arg:tt)*) => {
                if $left != $right {
                    panic!("DCHECK_EQ failed: {} != {}: {}", $left, $right, format_args!($($arg)*));
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_LE {
            ($left:expr, $right:expr) => {
                if $left > $right {
                    panic!("DCHECK_LE failed: {} > {}", $left, $right);
                }
            };
            ($left:expr, $right:expr, $($arg:tt)*) => {
                if $left > $right {
                    panic!("DCHECK_LE failed: {} > {}: {}", $left, $right, format_args!($($arg)*));
                }
            };
        }

        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE");
            };
        }

        #[macro_export]
        macro_rules! DCHECK_IMPLIES {
            ($condition:expr, $implication:expr) => {
                if $condition && !$implication {
                    panic!("DCHECK_IMPLIES failed: {} implies {}", stringify!($condition), stringify!($implication));
                }
            };
        }
    }
}

pub mod torque {
    pub mod ast {
        #[derive(Debug, Clone)]
        pub struct Identifier {
            pub value: String,
            pub pos: SourcePosition,
        }

        #[derive(Debug, Clone)]
        pub struct NameAndExpression {
            pub name: *mut Identifier, // *mut is a temporary solution. consider a managed pointer.
            pub expression: *mut Expression,  // *mut is a temporary solution. consider a managed pointer.
        }

        #[derive(Debug, Clone)]
        pub struct Expression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct StructExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct CallExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct CallMethodExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct IntrinsicCallExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct TailCallStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ConditionalExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct LogicalOrExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct LogicalAndExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct IncrementDecrementExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct AssignmentExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct StringLiteralExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct FloatingPointLiteralExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct IntegerLiteralExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct AssumeTypeImpossibleExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct TryLabelExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct StatementExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct NewExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct SpreadExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct LocationExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct FieldAccessExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct DereferenceExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ElementAccessExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct IdentifierExpression; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ReturnStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct GotoStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct IfStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct WhileStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct BreakStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ContinueStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ForLoopStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct VarDeclarationStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct BlockStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ExpressionStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct DebugStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct AssertStatement; // Placeholder

        #[derive(Debug, Clone)]
        pub struct Statement; // Placeholder
    }
    pub mod cfg {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct BottomOffset(pub usize);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct StackRange {
            pub begin: BottomOffset,
            pub end: BottomOffset,
        }

        impl StackRange {
            pub fn Size(&self) -> usize {
                self.end.0 - self.begin.0
            }
        }
        #[derive(Debug, Clone)]
        pub struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Stack<T> {
            pub fn new() -> Self {
                Stack { items: Vec::new() }
            }

            pub fn push(&mut self, item: T) {
                self.items.push(item);
            }

            pub fn pop(&mut self) -> Option<T> {
                self.items.pop()
            }

            pub fn len(&self) -> usize {
                self.items.len()
            }

            pub fn is_empty(&self) -> bool {
                self.items.is_empty()
            }

            pub fn AboveTop(&self) -> BottomOffset {
                BottomOffset(self.items.len())
            }
        }

        #[derive(Debug, Clone)]
        pub struct Block; // Placeholder

        #[derive(Debug, Clone)]
        pub struct CfgAssembler {
            current_stack: Stack<String>,
            current_block_is_complete: bool,
        }

        impl CfgAssembler {
            pub fn new() -> Self {
                CfgAssembler {
                    current_stack: Stack::new(),
                    current_block_is_complete: false,
                }
            }
            pub fn CurrentStack(&mut self) -> &mut Stack<String> {
                &mut self.current_stack
            }
            pub fn CurrentBlockIsComplete(&self) -> bool {
                self.current_block_is_complete
            }

            pub fn DropTo(&mut self, offset: BottomOffset) {
                while self.current_stack.AboveTop() != offset {
                    self.current_stack.pop();
                }
            }
            pub fn DeleteRange(&mut self, range: StackRange) {
                //Not implemented
            }

            pub fn TopRange(&mut self, size: usize) -> StackRange {
                let end = self.current_stack.AboveTop();
                let begin = BottomOffset(end.0 - size);
                StackRange { begin, end }
            }

            pub fn Push<T>(&mut self, value: T) {
                //Placeholder implementation
                self.current_stack.push("placeholder".to_string());
            }
        }
    }
    pub mod cpp_builder {
        #[derive(Debug, Clone)]
        pub struct Function;  // Placeholder

        #[derive(Debug, Clone)]
        pub struct Class;  // Placeholder
    }
    pub mod declarations {
        use super::ast::Identifier;
        use super::types::{Signature, Type, AggregateType};
        use super::Locality;
        use super::{VisitResult};
        #[derive(Debug, Clone)]
        pub struct Callable {
            pub name: String,
        }

        #[derive(Debug, Clone)]
        pub struct Macro {
            pub name: String,
            pub signature: Signature,
        }

        #[derive(Debug, Clone)]
        pub struct ExternMacro; // Placeholder

        #[derive(Debug, Clone)]
        pub struct TorqueMacro; // Placeholder

        #[derive(Debug, Clone)]
        pub struct Method {
            pub receiver_type: *const AggregateType,
            pub signature: Signature,
        } // Placeholder

        #[derive(Debug, Clone)]
        pub struct Builtin; // Placeholder

        #[derive(Debug, Clone)]
        pub struct NamespaceConstant; // Placeholder

        #[derive(Debug, Clone)]
        pub struct TypeAlias; // Placeholder

        #[derive(Debug, Clone)]
        pub struct Namespace; // Placeholder

        #[derive(Debug, Clone)]
        pub struct QualifiedName {
            pub name: String,
        }

        impl QualifiedName {
            pub fn new(name: String) -> Self {
                QualifiedName { name }
            }
        }

        #[derive(Debug, Clone)]
        pub struct GenericCallable; // Placeholder
    }
    pub mod global_context {
        use std::cell::RefCell;
        use std::io::Stdout;

        #[derive(Debug, Clone)]
        pub struct PerFileStreams {
            pub csa_ccfile: Stdout,  // Replace with appropriate stream type
            pub csa_headerfile: Stdout, // Replace with appropriate stream type
            pub class_definition_inline_headerfile_macro_definitions: Stdout, // Replace with appropriate stream type
            pub class_definition_inline_headerfile_macro_declarations: Stdout, // Replace with appropriate stream type
        }

        thread_local! {
            static CURRENT_FILE_STREAMS: RefCell<Option<PerFileStreams>> = RefCell::new(None);
        }

        impl PerFileStreams {
            pub fn set_current(streams: PerFileStreams) {
                CURRENT_FILE_STREAMS.with(|f| {
                    *f.borrow_mut() = Some(streams);
                });
            }

            pub fn clear_current() {
                CURRENT_FILE_STREAMS.with(|f| {
                    *f.borrow_mut() = None;
                });
            }

            pub fn Get() -> Option<&'static RefCell<Option<PerFileStreams>>> {
                Some(&CURRENT_FILE_STREAMS)
            }
        }
    }
    pub mod type_oracle {
        use super::types::Type;

        #[derive(Debug, Clone)]
        pub struct TypeOracle;

        impl TypeOracle {
            pub fn MatchReferenceGeneric(ty: *const Type) -> Option<*const Type> {
                // Placeholder
                None
            }
            pub fn MatchReferenceGeneric_with_is_const(ty: *const Type, is_const: &mut bool) -> Option<*const Type> {
                // Placeholder
                None
            }
            pub fn GetConstSliceGeneric() -> *const Type {
                //Placeholder
                std::ptr::null()
            }
            pub fn GetMutableSliceGeneric() -> *const Type {
                //Placeholder
                std::ptr::null()
            }
        }
    }
    pub mod types {
        use std::collections::HashMap;
        use std::fmt;
        use std::ops::Deref;
        use std::rc::Rc;

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Type {
            // Placeholder fields
            pub name: String,
        }

        impl Type {
            pub fn MatchUnaryGeneric(ty: *const Type, generic_type: *const Type) -> Option<*const Type> {
                // Placeholder implementation
                None
            }
            pub fn StructSupertype(&self) -> bool {
                false //Placeholder
            }
        }
        #[derive(Debug, Clone)]
        pub struct Signature; // Placeholder

        #[derive(Debug, Clone)]
        pub struct AggregateType; // Placeholder

        #[derive(Debug, Clone)]
        pub struct ClassType; // Placeholder

        impl ClassType {
            pub fn fields(&self) -> Vec<Field> {
                //Placeholder
                Vec::new()
            }
        }
    }
    pub mod utils {
        use std::fmt;
        use std::fs;
        use std::io::{self, Write};
        use std::path::Path;
        use std::sync::{Arc, Mutex};

        #[derive(Debug, Clone)]
        pub struct SourcePosition; // Placeholder

        impl SourcePosition {
            pub fn new() -> Self {
                SourcePosition {}
            }
        }

        #[derive(Debug, Clone)]
        pub struct CurrentSourcePosition; // Placeholder

        impl CurrentSourcePosition {
            pub fn Get() -> SourcePosition {
                SourcePosition::new()
            }
        }

        // Placeholder for error reporting
        #[derive(Debug, Clone)]
        pub struct Error {
            message: String,
            position: Option<SourcePosition>,
        }

        impl Error {
            pub fn new(message: String) -> Self {
                Error {
                    message,
                    position: None,
                }
            }
            pub fn Position(mut self, position: SourcePosition) -> Self {
                self.position = Some(position);
                self
            }

            pub fn Throw(&self) -> ! {
                // Placeholder
                panic!("{}", self.message);
            }
        }

        pub fn StartsWithSingleUnderscore(name: &str) -> bool {
            name.starts_with("_") && name[1..].chars().next().map_or(false, |c| c != '_')
        }
        #[derive(Debug, Clone)]
        pub struct Lint {
            message: String,
            position: Option<SourcePosition>,
        }

        impl Lint {
            pub fn new(message: String) -> Self {
                Lint {
                    message,
                    position: None,
                }
            }
            pub fn Position(mut self, position: SourcePosition) -> Self {
                self.position = Some(position);
                self
            }
        }
        // Placeholder for Lint reporting
        pub fn Lint(s1: String, s2: String, s3: String, s4: String) -> Lint {
            // Placeholder
            Lint::new(format!("{}{}{}{}", s1, s2, s3, s4))
        }
        pub fn ReportError(s1: String, s2: usize, s3: String, s4: String, s5: usize) {
            // Placeholder
            println!("{}{}{}{}{}", s1, s2, s3, s4, s5);
        }
        pub fn ReportErrorIfDifferent(path: &str, content: &str) -> io::Result<()> {
            let existing_content = fs::read_to_string(path).unwrap_or_default();
            if existing_content != content {
                fs::write(path, content)?;
            }
            Ok(())
        }
        pub fn ReplaceFileContentsIfDifferent(file: String, content: String) {
            let path = Path::new(&file);
            let existing_content = fs::read_to_string(path).unwrap_or_default();

            if existing_content != content {
                fs::write(path, content).unwrap();
            }
        }
    }

    use crate::base::macros::{DCHECK, DCHECK_EQ, DCHECK_IMPLIES, DCHECK_LE, UNREACHABLE};
    use ast::*;
    use cfg::*;
    use cpp_builder::*;
    use declarations::*;
    use global_context::*;
    use std::{cell::RefCell, collections::HashMap, optional::Option, ptr, rc::Rc};
    use type_oracle::*;
    use types::*;
    use utils::*;

    pub type VisitResultVector = Vec<VisitResult>;
    pub type NameVector = Vec<String>;
    pub type TypeVector = Vec<*const Type>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FieldSynchronization {
        kNone,
    }

    #[derive(Debug, Clone)]
    pub struct BitField {
        pub name_and_type: NameAndType,
    }

    #[derive(Debug, Clone)]
    pub struct NameAndType {
        pub name: String,
        pub type_: *const Type,
    }

    #[derive(Debug, Clone)]
    pub struct Field {
        pub name_and_type: NameAndType,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VisitResult {
        type_: *const Type,
        stack_range: Option<StackRange>,
    }

    impl VisitResult {
        pub fn new(type_: *const Type, stack_range: Option<StackRange>) -> Self {
            VisitResult { type_, stack_range }
        }
        pub fn IsOnStack(&self) -> bool {
            self.stack_range.is_some()
        }
        pub fn type_(&self) -> *const Type {
            self.type_
        }

        pub fn stack_range(&self) -> StackRange {
            self.stack_range.unwrap()
        }
    }

    // Implement PartialEq for VisitResult, considering pointer equality for types.
    impl PartialEq for LocationReference {
        fn eq(&self, other: &Self) -> bool {
            // Compare based on which variant is active.  This example only considers
            // the `temporary_` variant.  You'll need to expand this to cover all variants.
            if self.temporary_.is_some() && other.temporary_.is_some() {
                self.temporary_.as_ref().unwrap() == other.temporary_.as_ref().unwrap() &&
                    self.temporary_description_ == other.temporary_description_
            } else if self.variable_.is_some() && other.variable_.is_some() {
                self.variable_.as_ref().unwrap() == other.variable_.as_ref().unwrap() &&
                    self.binding_ == other.binding_
            }
            else if self.heap_reference_.is_some() && other.heap_reference_.is_some() {
                self.heap_reference_.as_ref().unwrap() == other.heap_reference_.as_ref().unwrap() &&
                    self.heap_reference_synchronization_ == other.heap_reference_synchronization_
            }
            else if self.heap_slice_.is_some() && other.heap_slice_.is_some() {
                self.heap_slice_.as_ref().unwrap() == other.heap_slice_.as_ref().unwrap()
            }
            else {
                false // Not equal if different variants are active.
            }
        }
    }

    /// Represents an l-value. Can represent assignable stack ranges,
    /// unassignable temporaries, or field/element access expressions.
    #[derive(Debug, Clone)]
    pub struct LocationReference {
        variable_: Option<VisitResult>,
        temporary_: Option<VisitResult>,
        temporary_description_: Option<String>,
        heap_reference_: Option<VisitResult>,
        heap_reference_synchronization_: FieldSynchronization,
        heap_slice_: Option<VisitResult>,
        eval_function_: Option<String>,
        assign_function_: Option<String>,
        call_arguments_: VisitResultVector,
        binding_: Option<*mut Binding<LocalValue>>,

        // The location of the bitfield struct that contains this bitfield, if this
        // reference is a bitfield access. Uses a shared_ptr so that LocationReference
        // is copyable, allowing us to set this field equal to a copy of a
        // stack-allocated LocationReference.
        bit_field_struct_: Option<Arc<LocationReference>>,
        bit_field_: Option<BitField>,
    }

    impl LocationReference {
        /// An assignable stack range.
        pub fn VariableAccess(variable: VisitResult, binding: Option<*mut Binding<LocalValue>>) -> Self {
            DCHECK!(variable.IsOnStack());
            LocationReference {
                variable_: Some(variable),
                temporary_: None,
                temporary_description_: None,
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: None,
                eval_function_: None,
                assign_function_: None,
                call_arguments_: Vec::new(),
                binding_: binding,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        /// An unassignable value. {description} is only used for error messages.
        pub fn Temporary(temporary: VisitResult, description: String) -> Self {
            LocationReference {
                variable_: None,
                temporary_: Some(temporary),
                temporary_description_: Some(description),
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: None,
                eval_function_: None,
                assign_function_: None,
                call_arguments_: Vec::new(),
                binding_: None,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        /// A heap reference, that is, a tagged value and an offset to encode an inner
        /// pointer.
        pub fn HeapReference(heap_reference: VisitResult, synchronization: FieldSynchronization) -> Self {
            DCHECK!(TypeOracle::MatchReferenceGeneric(heap_reference.type_()).is_some());
            LocationReference {
                variable_: None,
                temporary_: None,
                temporary_description_: None,
                heap_reference_: Some(heap_reference),
                heap_reference_synchronization_: synchronization,
                heap_slice_: None,
                eval_function_: None,
                assign_function_: None,
                call_arguments_: Vec::new(),
                binding_: None,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        /// A reference to an array on the heap. That is, a tagged value, an offset to
        /// encode an inner pointer, and the number of elements.
        pub fn HeapSlice(heap_slice: VisitResult) -> Self {
            let is_match = Type::MatchUnaryGeneric(heap_slice.type_(), TypeOracle::GetConstSliceGeneric()).is_some()
                || Type::MatchUnaryGeneric(heap_slice.type_(), TypeOracle::GetMutableSliceGeneric()).is_some();
            DCHECK!(is_match);

            LocationReference {
                variable_: None,
                temporary_: None,
                temporary_description_: None,
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: Some(heap_slice),
                eval_function_: None,
                assign_function_: None,
                call_arguments_: Vec::new(),
                binding_: None,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        pub fn ArrayAccess(base: VisitResult, offset: VisitResult) -> Self {
            LocationReference {
                variable_: None,
                temporary_: None,
                temporary_description_: None,
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: None,
                eval_function_: Some("[]".to_string()),
                assign_function_: Some("[]=".to_string()),
                call_arguments_: vec![base, offset],
                binding_: None,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        pub fn FieldAccess(object: VisitResult, fieldname: String) -> Self {
            LocationReference {
                variable_: None,
                temporary_: None,
                temporary_description_: None,
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: None,
                eval_function_: Some(".".to_string() + &fieldname),
                assign_function_: Some(".".to_string() + &fieldname + "="),
                call_arguments_: vec![object],
                binding_: None,
                bit_field_struct_: None,
                bit_field_: None,
            }
        }

        pub fn BitFieldAccess(object: &LocationReference, field: BitField) -> Self {
            LocationReference {
                variable_: None,
                temporary_: None,
                temporary_description_: None,
                heap_reference_: None,
                heap_reference_synchronization_: FieldSynchronization::kNone,
                heap_slice_: None,
                eval_function_: None,
                assign_function_: None,
                call_arguments_: Vec::new(),
                binding_: None,
                bit_field_struct_: Some(Arc::new(object.clone())),
                bit_field_: Some(field),
            }
        }

        pub fn IsConst(&self) -> bool {
            if self.IsHeapReference() {
                let mut is_const = false;
                let success = TypeOracle::MatchReferenceGeneric_with_is_const(self.heap_reference().type_(), &mut is_const).is_some();
                DCHECK!(success);
                return is_const;
            }
            self.IsTemporary()
        }

        pub fn IsVariableAccess(&self) -> bool {
            self.variable_.is_some()
        }
        pub fn variable(&self) -> &VisitResult {
            DCHECK!(self.IsVariableAccess());
            self.variable_.as_ref().unwrap()
        }
        pub fn IsTemporary(&self) -> bool {
            self.temporary_.is_some()
        }
        pub fn temporary(&self) -> &VisitResult {
            DCHECK!(self.IsTemporary());
            self.temporary_.as_ref().unwrap()
        }
        pub fn IsHeapReference(&self) -> bool {
            self.heap_reference_.is_some()
        }
        pub fn heap_reference(&self) -> &VisitResult {
            DCHECK!(self.IsHeapReference());
            self.heap_reference_.as_ref().unwrap()
        }
        pub fn heap_reference_synchronization(&self) -> FieldSynchronization {
            DCHECK!(self.IsHeapReference());
            self.heap_reference_synchronization_
        }
        pub fn IsHeapSlice(&self) -> bool {
            self.heap_slice_.is_some()
        }
        pub fn heap_slice(&self) -> &VisitResult {
            DCHECK!(self.IsHeapSlice());
            self.heap_slice_.as_ref().unwrap()
        }
        pub fn IsBitFieldAccess(&self) -> bool {
            let is_bitfield_access = self.bit_field_struct_.is_some();
            DCHECK_EQ!(is_bitfield_access, self.bit_field_.is_some());
            is_bitfield_access
        }
        pub fn bit_field_struct_location(&self) -> &LocationReference {
            DCHECK!(self.IsBitFieldAccess());
            self.bit_field_struct_.as_ref().unwrap()
        }
        pub fn bit_field(&self) -> &BitField {
            DCHECK!(self.IsBitFieldAccess());
            self.bit_field_.as_ref().unwrap()
        }

        pub fn ReferencedType(&self) -> Option<*const Type> {
            if self.IsHeapReference() {
                return TypeOracle::MatchReferenceGeneric(self.heap_reference().type_());
            }
            if self.IsHeapSlice() {
                if let Some(type_) = Type::MatchUnaryGeneric(
                    self.heap_slice().type_(),
                    TypeOracle::GetMutableSliceGeneric(),
                ) {
                    return Some(type_);
                }
                return Type::MatchUnaryGeneric(
                    self.heap_slice().type_(),
                    TypeOracle::GetConstSliceGeneric(),
                );
            }
            if self.IsBitFieldAccess() {
                return Some(self.bit_field().name_and_type.type_);
            }
            if self.IsVariableAccess() || self.IsHeapSlice() || self.IsTemporary() {
                return Some(self.GetVisitResult().type_());
            }
            None
        }

        pub fn GetVisitResult(&self) -> &VisitResult {
            if self.IsVariableAccess() {
                return self.variable();
            }
            if self.IsHeapSlice() {
                return self.heap_slice();
            }
            DCHECK!(self.IsTemporary());
            self.temporary()
        }

        // For error reporting.
        pub fn temporary_description(&self) -> &String {
            DCHECK!(self.IsTemporary());
            self.temporary_description_.as_ref().unwrap()
        }

        pub fn IsCallAccess(&self) -> bool {
            let is_call_access = self.eval_function_.is_some();
            DCHECK_EQ!(is_call_access, self.assign_function_.is_some());
            is_call_access
        }
        pub fn call_arguments(&self) -> &VisitResultVector {
            DCHECK!(self.IsCallAccess());
            &self.call_arguments_
        }
        pub fn eval_function(&self) -> &String {
            DCHECK!(self.IsCallAccess());
            self.eval_function_.as_ref().unwrap()
        }
        pub fn assign_function(&self) -> &String {
            DCHECK!(self.IsCallAccess());
            self.assign_function_.as_ref().unwrap()
        }
        pub fn binding(&self) -> Option<*mut Binding<LocalValue>> {
            DCHECK!(self.IsVariableAccess());
            self.binding_
        }
    }

    #[derive(Debug, Clone)]
    pub struct InitializerResults {
        pub names: Vec<*mut Identifier>, // *mut is a temporary solution. consider a managed pointer.
        pub field_value_map: HashMap<String, VisitResult>,
    }

    #[derive(Debug, Clone)]
    pub struct LayoutForInitialization {
        pub array_lengths: HashMap<String, VisitResult>,
        pub offsets: HashMap<String, VisitResult>,
        pub size: VisitResult,
    }

    pub static mut next_unique_binding_index: u64 = 0;

    /// Manages bindings for a specific type T.
    #[derive(Debug)]
    pub struct BindingsManager<T> {
        current_bindings_: HashMap<String, Option<*mut Binding<T>>>,
    }

    impl<T> BindingsManager<T> {
        pub fn new() -> Self {
            BindingsManager {
                current_bindings_: HashMap::new(),
            }
        }

        pub fn TryLookup(&self, name: &str) -> Option<*mut Binding<T>> {
            if StartsWithSingleUnderscore(name) {
                Error(format!("Trying to reference '{}' which is marked as unused.", name)).Throw();
            }
            match self.current_bindings_.get(name) {
                Some(binding) => {
                    if let Some(b) = binding {
                        unsafe {
                            (**b).SetUsed();
                        }
                        Some(*b)
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }

    /// Represents a binding between a name and a value of type T.
    #[derive(Debug)]
    pub struct Binding<T> {
        data: T,
        manager_: *mut BindingsManager<T>,
        name_: String,
        previous_binding_: Option<*mut Binding<T>>,
        declaration_position_: SourcePosition,
        used_: bool,
        written_: bool,
        unique_index_: u64,
    }

    impl<T> Binding<T> {
        pub fn new<Args>(manager: *mut BindingsManager<T>, name: String, data: Args) -> Self
            where
                T: From<Args>,
        {
            unsafe {
                let mut binding = Binding {
                    data: T::from(data),
                    manager_: manager,
                    name_: name.clone(),
                    previous_binding_: None,
                    declaration_position_: CurrentSourcePosition::Get(),
                    used_: false,
                    written_: false,
                    unique_index_: {
                        next_unique_binding_index += 1;
                        next_unique_binding_index - 1
                    },
                };

                let previous_binding = (*manager).current_bindings_.insert(name.clone(), None);
                binding.previous_binding_ = previous_binding.flatten();

                Binding {
                    data: binding.data,
                    manager_: binding.manager_,
                    name_: binding.name_.clone(),
                    previous_binding_: binding.previous_binding_,
                    declaration_position_: binding.declaration_position_.clone(),
                    used_: binding.used_,
                    written_: binding.written_,
                    unique_index_: binding.unique_index_,
                }
            }
        }

        pub fn new_with_identifier<Args>(
            manager: *mut BindingsManager<T>,
            name: *mut Identifier,
            data: Args,
        ) -> Self
            where
                T: From<Args>,
        {
            unsafe {
                Binding::new(manager, (*name).value.clone(), data)
            }
        }

        pub fn BindingTypeString(&self) -> String {
            // Placeholder implementation, override