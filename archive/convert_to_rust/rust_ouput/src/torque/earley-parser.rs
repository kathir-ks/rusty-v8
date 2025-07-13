// Converted from V8 C++ source files:
// Header: earley-parser.h
// Implementation: earley-parser.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub fn hash_combine<T: std::hash::Hash>(seed: usize, value: T) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut hasher);
        value.hash(&mut hasher);
        hasher.finish() as usize
    }
}

pub mod ast {
    // Placeholder for ast
    pub struct AstNode {}
}

pub mod utils {
    // Placeholder for utils
    pub struct Location {}
}

pub mod source_positions {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LineAndColumn {
        pub offset: usize,
        pub line: usize,
        pub column: usize,
        pub source: usize,
    }

    impl LineAndColumn {
        pub fn Invalid() -> Self {
            LineAndColumn {
                offset: 0,
                line: 0,
                column: 0,
                source: 0,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SourcePosition {
        pub source: usize,
        pub start: LineAndColumn,
        pub end: LineAndColumn,
    }

    impl SourcePosition {
        pub fn new(source: usize, start: LineAndColumn, end: LineAndColumn) -> Self {
            SourcePosition { source, start, end }
        }
    }

    pub struct CurrentSourcePosition {}

    impl CurrentSourcePosition {
        thread_local! {
            static POSITION: std::cell::RefCell<SourcePosition> = std::cell::RefCell::new(SourcePosition { source: 0, start: LineAndColumn::Invalid(), end: LineAndColumn::Invalid() });
        }

        pub fn get() -> SourcePosition {
            CurrentSourcePosition::POSITION.with(|p| *p.borrow())
        }

        pub fn set(pos: SourcePosition) {
            CurrentSourcePosition::POSITION.with(|p| *p.borrow_mut() = pos);
        }

        pub struct Scope {
            old_position: SourcePosition,
        }

        impl Scope {
            pub fn new(pos: SourcePosition) -> Self {
                let old_position = CurrentSourcePosition::get();
                CurrentSourcePosition::set(pos);
                Scope { old_position }
            }
        }

        impl Drop for Scope {
            fn drop(&mut self) {
                CurrentSourcePosition::set(self.old_position);
            }
        }
        
        pub fn Get() -> &'static std::thread::LocalKey<std::cell::RefCell<SourcePosition>> {
            &CurrentSourcePosition::POSITION
        }
    }

    pub struct CurrentSourceFile {}

    impl CurrentSourceFile {
        pub fn Get() -> usize {
            0 // Placeholder
        }
    }
}

use source_positions::*;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub struct V8_EXPORT_PRIVATE {}

macro_rules! USE {
    ($x:expr) => {
        let _ = $x;
    };
}

fn ReportError(message: String) -> ! {
    panic!("{}", message);
}

fn StringLiteralQuote(s: String) -> String {
    format!("\"{}\"", s)
}

pub struct Symbol;

pub struct Item;

pub mod earley_parser {
    use super::*;
    use std::cell::RefCell;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct ParseResultHolderBase {
        type_id_: ParseResultTypeId,
    }

    impl ParseResultHolderBase {
        pub fn new(type_id: ParseResultTypeId) -> Self {
            ParseResultHolderBase { type_id_: type_id }
        }

        pub fn type_id(&self) -> ParseResultTypeId {
            self.type_id_
        }

        pub fn into_any(self) -> Rc<dyn std::any::Any> {
            Rc::new(self)
        }

        pub fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl Drop for ParseResultHolderBase {
        fn drop(&mut self) {}
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ParseResultHolderBase::TypeId {
        kStdString,
        kBool,
        kInt32,
        kDouble,
        kIntegerLiteral,
        kStdVectorOfString,
        kExpressionPtr,
        kIdentifierPtr,
        kOptionalIdentifierPtr,
        kStatementPtr,
        kDeclarationPtr,
        kTypeExpressionPtr,
        kOptionalTypeExpressionPtr,
        kTryHandlerPtr,
        kNameAndTypeExpression,
        kEnumEntry,
        kStdVectorOfEnumEntry,
        kImplicitParameters,
        kOptionalImplicitParameters,
        kNameAndExpression,
        kAnnotation,
        kVectorOfAnnotation,
        kAnnotationParameter,
        kOptionalAnnotationParameter,
        kClassFieldExpression,
        kStructFieldExpression,
        kBitFieldDeclaration,
        kStdVectorOfNameAndTypeExpression,
        kStdVectorOfNameAndExpression,
        kStdVectorOfClassFieldExpression,
        kStdVectorOfStructFieldExpression,
        kStdVectorOfBitFieldDeclaration,
        kIncrementDecrementOperator,
        kOptionalStdString,
        kStdVectorOfStatementPtr,
        kStdVectorOfDeclarationPtr,
        kStdVectorOfStdVectorOfDeclarationPtr,
        kStdVectorOfExpressionPtr,
        kExpressionWithSource,
        kParameterList,
        kTypeList,
        kOptionalTypeList,
        kLabelAndTypes,
        kStdVectorOfLabelAndTypes,
        kStdVectorOfTryHandlerPtr,
        kOptionalStatementPtr,
        kOptionalExpressionPtr,
        kTypeswitchCase,
        kStdVectorOfTypeswitchCase,
        kStdVectorOfIdentifierPtr,
        kOptionalClassBody,
        kGenericParameter,
        kGenericParameters,
        kJsonValue,
        kJsonMember,
        kStdVectorOfJsonValue,
        kStdVectorOfJsonMember,
    }

    pub type ParseResultTypeId = ParseResultHolderBase::TypeId;

    #[derive(Debug)]
    pub struct ParseResultHolder<T> {
        value_: T,
    }

    impl<T> ParseResultHolder<T> {
        pub fn new(value: T) -> Self {
            ParseResultHolder { value_: value }
        }

        pub fn value(&self) -> &T {
            &self.value_
        }

        pub fn value_mut(&mut self) -> &mut T {
            &mut self.value_
        }
    }

    pub struct ParseResult {
        value_: Rc<dyn std::any::Any>,
        type_id_: ParseResultTypeId,
    }

    impl ParseResult {
        pub fn new<T: 'static>(x: T) -> Self {
            ParseResult {
                value_: Rc::new(x),
                type_id_: Self::type_id::<T>(),
            }
        }
        fn type_id<T: 'static>() -> ParseResultTypeId {
            use ParseResultHolderBase::TypeId::*;
            use std::any::TypeId;
            let id = TypeId::of::<T>();
            if id == TypeId::of::<String>() {
                kStdString
            } else if id == TypeId::of::<bool>() {
                kBool
            } else if id == TypeId::of::<i32>() {
                kInt32
            } else if id == TypeId::of::<f64>() {
                kDouble
            } else if id == TypeId::of::<i64>() {
                kIntegerLiteral
            } else if id == TypeId::of::<Vec<String>>() {
                kStdVectorOfString
            } else if id == TypeId::of::<ExpressionPtr>() {
                kExpressionPtr
            } else if id == TypeId::of::<IdentifierPtr>() {
                kIdentifierPtr
            } else if id == TypeId::of::<Option<IdentifierPtr>>() {
                kOptionalIdentifierPtr
            } else if id == TypeId::of::<StatementPtr>() {
                kStatementPtr
            } else if id == TypeId::of::<DeclarationPtr>() {
                kDeclarationPtr
            } else if id == TypeId::of::<TypeExpressionPtr>() {
                kTypeExpressionPtr
            } else if id == TypeId::of::<Option<TypeExpressionPtr>>() {
                kOptionalTypeExpressionPtr
            } else if id == TypeId::of::<TryHandlerPtr>() {
                kTryHandlerPtr
            } else if id == TypeId::of::<NameAndTypeExpression>() {
                kNameAndTypeExpression
            } else if id == TypeId::of::<EnumEntry>() {
                kEnumEntry
            } else if id == TypeId::of::<Vec<EnumEntry>>() {
                kStdVectorOfEnumEntry
            } else if id == TypeId::of::<ImplicitParameters>() {
                kImplicitParameters
            } else if id == TypeId::of::<Option<ImplicitParameters>>() {
                kOptionalImplicitParameters
            } else if id == TypeId::of::<NameAndExpression>() {
                kNameAndExpression
            } else if id == TypeId::of::<Annotation>() {
                kAnnotation
            } else if id == TypeId::of::<Vec<Annotation>>() {
                kVectorOfAnnotation
            } else if id == TypeId::of::<AnnotationParameter>() {
                kAnnotationParameter
            } else if id == TypeId::of::<Option<AnnotationParameter>>() {
                kOptionalAnnotationParameter
            } else if id == TypeId::of::<ClassFieldExpression>() {
                kClassFieldExpression
            } else if id == TypeId::of::<StructFieldExpression>() {
                kStructFieldExpression
            } else if id == TypeId::of::<BitFieldDeclaration>() {
                kBitFieldDeclaration
            } else if id == TypeId::of::<Vec<NameAndTypeExpression>>() {
                kStdVectorOfNameAndTypeExpression
            } else if id == TypeId::of::<Vec<NameAndExpression>>() {
                kStdVectorOfNameAndExpression
            } else if id == TypeId::of::<Vec<ClassFieldExpression>>() {
                kStdVectorOfClassFieldExpression
            } else if id == TypeId::of::<Vec<StructFieldExpression>>() {
                kStdVectorOfStructFieldExpression
            } else if id == TypeId::of::<Vec<BitFieldDeclaration>>() {
                kStdVectorOfBitFieldDeclaration
            } else if id == TypeId::of::<IncrementDecrementOperator>() {
                kIncrementDecrementOperator
            } else if id == TypeId::of::<Option<String>>() {
                kOptionalStdString
            } else if id == TypeId::of::<Vec<StatementPtr>>() {
                kStdVectorOfStatementPtr
            } else if id == TypeId::of::<Vec<DeclarationPtr>>() {
                kStdVectorOfDeclarationPtr
            } else if id == TypeId::of::<Vec<Vec<DeclarationPtr>>>() {
                kStdVectorOfStdVectorOfDeclarationPtr
            } else if id == TypeId::of::<Vec<ExpressionPtr>>() {
                kStdVectorOfExpressionPtr
            } else if id == TypeId::of::<ExpressionWithSource>() {
                kExpressionWithSource
            } else if id == TypeId::of::<ParameterList>() {
                kParameterList
            } else if id == TypeId::of::<TypeList>() {
                kTypeList
            } else if id == TypeId::of::<Option<TypeList>>() {
                kOptionalTypeList
            } else if id == TypeId::of::<LabelAndTypes>() {
                kLabelAndTypes
            } else if id == TypeId::of::<Vec<LabelAndTypes>>() {
                kStdVectorOfLabelAndTypes
            } else if id == TypeId::of::<Vec<TryHandlerPtr>>() {
                kStdVectorOfTryHandlerPtr
            } else if id == TypeId::of::<Option<StatementPtr>>() {
                kOptionalStatementPtr
            } else if id == TypeId::of::<Option<ExpressionPtr>>() {
                kOptionalExpressionPtr
            } else if id == TypeId::of::<TypeswitchCase>() {
                kTypeswitchCase
            } else if id == TypeId::of::<Vec<TypeswitchCase>>() {
                kStdVectorOfTypeswitchCase
            } else if id == TypeId::of::<Vec<IdentifierPtr>>() {
                kStdVectorOfIdentifierPtr
            } else if id == TypeId::of::<Option<ClassBody>>() {
                kOptionalClassBody
            } else if id == TypeId::of::<GenericParameter>() {
                kGenericParameter
            } else if id == TypeId::of::<GenericParameters>() {
                kGenericParameters
            } else if id == TypeId::of::<JsonValue>() {
                kJsonValue
            } else if id == TypeId::of::<JsonMember>() {
                kJsonMember
            } else if id == TypeId::of::<Vec<JsonValue>>() {
                kStdVectorOfJsonValue
            } else if id == TypeId::of::<Vec<JsonMember>>() {
                kStdVectorOfJsonMember
            } else {
                panic!("unknown type {:?}", id);
            }
        }

        pub fn cast<T: 'static>(&self) -> &T {
            if let Some(value) = self.value_.downcast_ref::<T>() {
                value
            } else {
                panic!("Wrong type when casting ParseResult")
            }
        }

        pub fn cast_mut<T: 'static>(&mut self) -> &mut T {
            if let Some(value) = Rc::get_mut(&mut self.value_).and_then(|rc| rc.downcast_mut::<T>()) {
                value
            } else {
                panic!("Wrong type when casting ParseResult")
            }
        }
    }

    // Implement all the types we use.
    #[derive(Debug)]
    pub struct ExpressionPtr {}
    #[derive(Debug)]
    pub struct IdentifierPtr {}
    #[derive(Debug)]
    pub struct StatementPtr {}
    #[derive(Debug)]
    pub struct DeclarationPtr {}
    #[derive(Debug)]
    pub struct TypeExpressionPtr {}
    #[derive(Debug)]
    pub struct TryHandlerPtr {}
    #[derive(Debug)]
    pub struct NameAndTypeExpression {}
    #[derive(Debug)]
    pub struct EnumEntry {}
    #[derive(Debug)]
    pub struct ImplicitParameters {}
    #[derive(Debug)]
    pub struct NameAndExpression {}
    #[derive(Debug)]
    pub struct Annotation {}
    #[derive(Debug)]
    pub struct AnnotationParameter {}
    #[derive(Debug)]
    pub struct ClassFieldExpression {}
    #[derive(Debug)]
    pub struct StructFieldExpression {}
    #[derive(Debug)]
    pub struct BitFieldDeclaration {}
    #[derive(Debug)]
    pub struct IncrementDecrementOperator {}
    #[derive(Debug)]
    pub struct ParameterList {}
    #[derive(Debug)]
    pub struct TypeList {}
    #[derive(Debug)]
    pub struct LabelAndTypes {}
    #[derive(Debug)]
    pub struct TypeswitchCase {}
    #[derive(Debug)]
    pub struct ClassBody {}
    #[derive(Debug)]
    pub struct GenericParameter {}
    #[derive(Debug)]
    pub struct GenericParameters {}
    #[derive(Debug)]
    pub struct JsonValue {}
    #[derive(Debug)]
    pub struct JsonMember {}

    pub type InputPosition = *const char;

    #[derive(Clone, Copy, Debug)]
    pub struct MatchedInput {
        pub begin: InputPosition,
        pub end: InputPosition,
        pub pos: SourcePosition,
    }

    impl MatchedInput {
        pub fn new(begin: InputPosition, end: InputPosition, pos: SourcePosition) -> Self {
            MatchedInput { begin, end, pos }
        }

        pub fn to_string(&self) -> String {
            unsafe {
                let begin = self.begin as *const u8;
                let end = self.end as *const u8;

                let slice = std::slice::from_raw_parts(begin, end.offset_from(begin) as usize);
                String::from_utf8_lossy(slice).into_owned()
            }
        }
    }

    pub struct ParseResultIterator {
        results_: Vec<ParseResult>,
        i_: usize,
        matched_input_: MatchedInput,
    }

    impl ParseResultIterator {
        pub fn new(results: Vec<ParseResult>, matched_input: MatchedInput) -> Self {
            ParseResultIterator {
                results_: results,
                i_: 0,
                matched_input_: matched_input,
            }
        }

        pub fn next(&mut self) -> ParseResult {
            if self.i_ >= self.results_.len() {
                panic!("Index out of bounds");
            }
            self.i_ += 1;
            self.results_.remove(0)
        }

        pub fn next_as<T: 'static>(&mut self) -> T {
            let result = self.next();
            result.cast::<T>().clone()
        }

        pub fn has_next(&self) -> bool {
            self.i_ < self.results_.len()
        }

        pub fn matched_input(&self) -> &MatchedInput {
            &self.matched_input_
        }
    }

    pub struct LexerResult {
        pub token_symbols: Vec<*mut Symbol>,
        pub token_contents: Vec<MatchedInput>,
    }

    pub type Action =
        fn(child_results: &mut ParseResultIterator) -> Option<ParseResult>;

    pub fn default_action(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if !child_results.has_next() {
            return None;
        }
        Some(child_results.next())
    }

    pub fn as_singleton_vector<T: 'static>(
        action: Action,
    ) -> Action {
        fn closure<T: 'static>(
            child_results: &mut ParseResultIterator,
        ) -> Option<ParseResult> {
            let result = action(child_results);
            match result {
                Some(res) => {
                    let val: T = res.cast::<T>().clone();
                    let vec = vec![val];
                    Some(ParseResult::new(vec))
                }
                None => None,
            }
        }
        closure::<T>
    }

    pub struct Rule {
        right_hand_side_: Vec<*mut Symbol>,
        action_: Action,
        left_hand_side_: *mut Symbol,
    }

    impl Rule {
        pub fn new(right_hand_side: Vec<*mut Symbol>, action: Action) -> Self {
            Rule {
                right_hand_side_: right_hand_side,
                action_: action,
                left_hand_side_: std::ptr::null_mut(),
            }
        }

        pub fn left(&self) -> *mut Symbol {
            assert!(!self.left_hand_side_.is_null());
            self.left_hand_side_
        }

        pub fn right(&self) -> &Vec<*mut Symbol> {
            &self.right_hand_side_
        }

        pub fn set_left_hand_side(&mut self, left_hand_side: *mut Symbol) {
            assert!(self.left_hand_side_.is_null());
            self.left_hand_side_ = left_hand_side;
        }

        pub fn run_action(
            &self,
            completed_item: &Item,
            tokens: &LexerResult,
        ) -> Option<ParseResult> {
            let mut results = Vec::new();
            for child in completed_item.children() {
                if child.is_null() {
                    continue;
                }
                let child_ref: &Item;
                unsafe {
                    child_ref = &*child;
                }
                let symbol_ptr = child_ref.left();
                let symbol: &Symbol;
                unsafe {
                    symbol = &*symbol_ptr;
                }
                let child_result = symbol.run_action(child_ref, tokens);
                if let Some(child_result) = child_result {
                    results.push(child_result);
                }
            }
            let matched_input = completed_item.get_matched_input(tokens);
            let pos_scope = CurrentSourcePosition::Scope::new(matched_input.pos);
            let mut iterator = ParseResultIterator::new(results, matched_input);
            let result = (self.action_)(&mut iterator);
            assert!(!iterator.has_next());
            result
        }
    }

    // A Symbol represents a terminal or a non-terminal of the grammar.
    // It stores the list of rules, which have this symbol as the
    // left-hand side.
    // Terminals have an empty list of rules, they are created by the Lexer
    // instead of from rules.
    // Symbols need to reside at stable memory addresses, because the addresses are
    // used in the parser.

    impl Symbol {
        pub fn new() -> Self {
            Symbol {
                rules_: Vec::new(),
            }
        }

        pub fn is_terminal(&self) -> bool {
            self.rules_.is_empty()
        }
        pub fn rule(&self, index: usize) -> &Rule {
            self.rules_[index].as_ref().unwrap()
        }
        pub fn rule_number(&self) -> usize {
            self.rules_.len()
        }
        pub fn add_rule(&mut self, rule: Rule) {
            let mut boxed_rule = Box::new(rule);
            boxed_rule.set_left_hand_side(self as *mut Symbol);
            self.rules_.push(Some(boxed_rule));
        }
        pub fn run_action(&self, item: &Item, tokens: &LexerResult) -> Option<ParseResult> {
            assert!(item.is_complete());
            assert_eq!(item.left(), self as *const Symbol as *mut Symbol);
            item.rule().run_action(item, tokens)
        }
    }

    impl Symbol {
        pub fn assign(&mut self, rules: Vec<Rule>) -> &mut Self {
            self.rules_.clear();
            for rule in rules {
                self.add_rule(rule);
            }
            self
        }
    }

    #[derive(Clone, Copy)]
    pub struct Item {
        rule_: *const Rule,
        mark_: usize,
        start_: usize,
        pos_: usize,
        prev_: *const Item,
        child_: *const Item,
    }

    impl Item {
        pub fn new(rule: *const Rule, mark: usize, start: usize, pos: usize) -> Self {
            assert!(mark <= unsafe { (*rule).right().len() });
            Item {
                rule_: rule,
                mark_: mark,
                start_: start,
                pos_: pos,
                prev_: std::ptr::null(),
                child_: std::ptr::null(),
            }
        }

        pub fn is_complete(&self) -> bool {
            let rule = unsafe { &*self.rule_ };
            assert!(self.mark_ <= rule.right().len());
            self.mark_ == rule.right().len()
        }

        pub fn next_symbol(&self) -> *mut Symbol {
            assert!(!self.is_complete());
            let rule = unsafe { &*self.rule_ };
            assert!(self.mark_ < rule.right().len());
            rule.right()[self.mark_]
        }

        pub fn advance(&self, new_pos: usize, child: *const Item) -> Self {
            if !child.is_null() {
                let child_ref: &Item;
                unsafe {
                   child_ref = &*child;
                }
                assert!(child_ref.is_complete());
                assert_eq!(self.pos(), child_ref.start());
                assert_eq!(new_pos, child_ref.pos());
                assert_eq!(self.next_symbol(), child_ref.left());
            }
            let rule = unsafe { &*self.rule_ };
            let mut result = Item::new(self.rule_, self.mark_ + 1, self.start_, new_pos);
            result.prev_ = self;
            result.child_ = child;
            result
        }

        pub fn children(&self) -> Vec<*const Item> {
            let mut children = Vec::new();
            let mut current = self;
            while !current.prev_.is_null() {
                children.push(current.child_);
                unsafe {
                    current = &*current.prev_;
                }
            }
            children.reverse();
            let rule = unsafe { &*self.rule_ };
            assert_eq!(children.len(), rule.right().len());
            children
        }

        pub fn split_by_children(&self, tokens: &LexerResult) -> String {
            let right_len;
            unsafe{
                right_len = (&*self.rule_).right().len();
            }
            if right_len == 1 {
                if let Some(child) = self.children().get(0) {
                    if !child.is_null() {
                       let child_ref : &Item;
                        unsafe{
                            child_ref = &**child_ref;
                        }
                       return child_ref.split_by_children(tokens)
                    }
                }
            }
            let mut s = String::new();
            let mut first = true;
            for item_ptr in self.children() {
               if !item_ptr.is_null() {
                    let item: &Item;
                   unsafe{
                        item = &*item_ptr;
                    }
                    if !first {
                        s.push_str("  ");
                    }
                    s.push_str(&item.get_matched_input(tokens).to_string());
                    first = false;
                }
            }
            s
        }

        pub fn check_ambiguity(&self, other: &Item, tokens: &LexerResult) {
            assert!(*self == *other);
            if self.child_ != other.child_ {
                let mut s = String::new();
               let child: &Item;
                unsafe{
                    child = &*self.child_;
                }
               
               s.push_str(&format!("Ambiguous grammer rules for \"{}\":\n   ", child.get_matched_input(tokens).to_string()));
                s.push_str(&child.split_by_children(tokens));
                s.push_str("\nvs\n   ");
                let other_child: &Item;
                unsafe {
                    other_child = &*other.child_;
                }
                s.push_str(&other_child.split_by_children(tokens));
                ReportError(s);
            }
            if self.prev_ != other.prev_ {
                let mut s = String::new();
                s.push_str(&format!(
                    "Ambiguous grammer rules for \"{}\":\n   ",
                    self.get_matched_input(tokens).to_string()
                ));
                s.push_str(&self.split_by_children(tokens));
                s.push_str("  ...\nvs\n   ");
                s.push_str(&other.split_by_children(tokens));
                s.push_str("  ...");
                ReportError(s);
            }
        }

        pub fn get_matched_input(&self, tokens: &LexerResult) -> MatchedInput {
            let start = tokens.token_contents[self.start_];
            let end = if self.start_ == self.pos_ {
                tokens.token_contents[self.start_]
            } else {
                tokens.token_contents[self.pos_ - 1]
            };
            assert_eq!(start.pos.source, end.pos.source);
            let combined = SourcePosition::new(
                start.pos.source,
                start.pos.start,
                end.pos.end,
            );

           unsafe {
                MatchedInput::new(start.begin, end.end, combined)
            }
        }

        pub fn rule(&self) -> &Rule {
            unsafe { &*self.rule_ }
        }
        pub fn left(&self) -> *mut Symbol {
            self.rule().left()
        }
        pub fn right(&self) -> &Vec<*mut Symbol> {
            self.rule().right()
        }
        pub fn pos(&self) -> usize {
            self.pos_
        }
        pub fn start(&self) -> usize {
            self.start_
        }
    }

    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.rule_ == other.rule_
                && self.mark_ == other.mark_
                && self.start_ == other.start_
                && self.pos_ == other.pos_
        }
    }

    impl Eq for Item {}

    impl Hash for Item {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.rule_.hash(state);
            self.mark_.hash(state);
            self.start_.hash(state);
            self.pos_.hash(state);
        }
    }

    pub fn run_earley_algorithm(
        start: *mut Symbol,
        tokens: &LexerResult,
        processed: &mut HashSet<Item>,
    ) -> *const Item {
        let mut worklist = Vec::new();
        let mut future_items = Vec::new();
        let source_position = SourcePosition::new(
            CurrentSourceFile::Get(),
            LineAndColumn::Invalid(),
            LineAndColumn::Invalid(),
        );
        let _source_position_scope = CurrentSourcePosition::Scope::new(source_position);

        let mut waiting: HashMap<(usize, *mut Symbol), HashSet<*const Item>> = HashMap::new();

        let mut debug_trace = Vec::new();

        let mut top_level = Symbol::new();
        let rule = Rule::new(vec![start], default_action);
        top_level.add_rule(rule);

        let top_level_rule = top_level.rule(0) as *const Rule;
        worklist.push(Item::new(top_level_rule, 0, 0, 0));

        let input_length = tokens.token_symbols.len();
        for pos in 0..=input_length {
            while !worklist.is_empty() {
                let item_back = worklist.pop().unwrap();
                let insert_result = processed.insert(item_back);

                let last_token = tokens.token_contents.get(pos).cloned().unwrap();
                CurrentSourcePosition::Get().set(last_token.pos);
                
                if !insert_result {
                   unsafe {
                        (&*item_back).check_ambiguity(&(*item_back), tokens);
                    }
                }
                if !insert_result {
                    continue;
                }

                debug_trace.push(item_back);
               let item: &Item;
                unsafe{
                    item = &*item_back;
                }
                if item.is_complete() {
                    // 'Complete' phase: Advance all items that were waiting to match this
                    // symbol next.
                    let mut parent_set = waiting
                        .remove(&(item.start(), item.left()))
                        .unwrap_or_else(|| HashSet::new());
                    for parent in parent_set {
                       let item_ref: &Item;
                        unsafe {
                            item_ref = &*item_back;
                        }
                       
                        worklist.push(unsafe { (&*parent).advance(pos, item_back) });
                    }
                } else {
                   
                    let next = item.next_symbol();
                    // 'Scan' phase: Check if {next} is the next symbol in the input (this
                    // is never the case if {next} is a non-terminal).
                    if pos < tokens.token_symbols.len() && tokens.token_symbols[pos] == next {
                        future_items.push(item.advance(pos + 1, std::ptr::null()));
                    }
                    // 'Predict' phase: Add items for every rule of the non-terminal.
                    if unsafe {(&*next).is_terminal()} == false {
                        // Remember that this item is waiting for completion with {next}.
                       
                        waiting
                            .entry((pos, next))
                            .or_insert_with(|| HashSet::new())
                            .insert(item_back);
                    }
                   
                    let next_ref : &Symbol;
                    unsafe{
                        next_ref = &*next;
                    }
                    for i in 0..next_ref.rule_number() {
                        let rule = next_ref.rule(i) as *const Rule;
                       
                        let already_completed = processed.get(&Item::new(rule, unsafe { (&*rule).right().len() }, pos, pos));

                        // As discussed in section 3 of
                        //    Aycock, John, and R. Nigel Horspool. "Practical earley
                        //    parsing." The Computer Journal 45.6 (2002): 620-630.
                        // Earley parsing has the following problem with epsilon rules:
                        // When we complete an item that started at the current position
                        // (that is, it matched zero tokens), we might not yet have
                        // predicted all items it can complete with. Thus we check for the
                        // existence of such items here and complete them immediately.
                        if let Some(completed_item) = already_completed {
                           
                            worklist.push(item.advance(pos, completed_item));
                        } else {
                            worklist.push(Item::new(rule, 0, pos, pos));
                        }
                    }
                }
            }
            std::mem::swap(&mut worklist, &mut future_items);
        }

        let final_item = processed.get(&Item::new(top_level_rule, 1, 0, input_length));
        match final_item {
            Some(item) => {
                let children = unsafe { (&*item).
