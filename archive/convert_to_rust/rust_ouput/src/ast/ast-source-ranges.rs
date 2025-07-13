// Converted from V8 C++ source files:
// Header: ast-source-ranges.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ast_source_ranges {
    use std::collections::HashMap;
    use crate::ast::ast::*;
    use crate::zone::zone_containers::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SourceRange {
        pub start: i32,
        pub end: i32,
    }

    impl SourceRange {
        pub fn new(start: i32, end: i32) -> Self {
            SourceRange { start, end }
        }

        pub fn empty() -> Self {
            SourceRange { start: kNoSourcePosition, end: kNoSourcePosition }
        }

        pub fn is_empty(&self) -> bool {
            self.start == kNoSourcePosition
        }

        pub fn open_ended(start: i32) -> Self {
            SourceRange { start, end: kNoSourcePosition }
        }

        pub fn continuation_of(that: &SourceRange, end: i32) -> Self {
            if that.is_empty() {
                SourceRange::empty()
            } else {
                SourceRange { start: that.end, end }
            }
        }

        pub const kFunctionLiteralSourcePosition: i32 = -2;

        pub fn function_literal_marker_range() -> Self {
            SourceRange { start: Self::kFunctionLiteralSourcePosition, end: Self::kFunctionLiteralSourcePosition }
        }
    }

    impl Default for SourceRange {
        fn default() -> Self {
            SourceRange::new(kNoSourcePosition, kNoSourcePosition)
        }
    }

    pub const kNoSourcePosition: i32 = -1;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SourceRangeKind {
        kBody,
        kCatch,
        kContinuation,
        kElse,
        kFinally,
        kRight,
        kThen,
    }

    pub trait AstNodeSourceRangesMethods {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange;
        fn has_range(&self, kind: SourceRangeKind) -> bool;
        fn remove_continuation_range(&mut self);
    }

    pub struct AstNodeSourceRanges {
       // No fields, acting as a base struct.
    }

    impl AstNodeSourceRanges {
        pub fn new() -> Self {
            AstNodeSourceRanges {}
        }
    }
    impl AstNodeSourceRangesMethods for AstNodeSourceRanges{
        fn get_range(&self, _kind: SourceRangeKind) -> SourceRange{
           SourceRange::default()
        }
        fn has_range(&self, _kind: SourceRangeKind) -> bool{
            false
        }
        fn remove_continuation_range(&mut self){}
    }

    pub struct BinaryOperationSourceRanges {
        right_range_: SourceRange,
    }

    impl BinaryOperationSourceRanges {
        pub fn new(right_range: SourceRange) -> Self {
            BinaryOperationSourceRanges { right_range_: right_range }
        }
    }
    impl AstNodeSourceRangesMethods for BinaryOperationSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.right_range_
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kRight
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct ContinuationSourceRanges {
        continuation_position_: i32,
    }

    impl ContinuationSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            ContinuationSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for ContinuationSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct BlockSourceRanges {
        continuation_position_: i32,
    }

    impl BlockSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            BlockSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for BlockSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct CaseClauseSourceRanges {
        body_range_: SourceRange,
    }

    impl CaseClauseSourceRanges {
        pub fn new(body_range: SourceRange) -> Self {
            CaseClauseSourceRanges { body_range_: body_range }
        }
    }
    impl AstNodeSourceRangesMethods for CaseClauseSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.body_range_
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kBody
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct ConditionalChainSourceRanges {
        then_ranges_: Vec<SourceRange>,
        else_ranges_: Vec<SourceRange>,
    }

    impl ConditionalChainSourceRanges {
        pub fn new() -> Self {
            ConditionalChainSourceRanges {
                then_ranges_: Vec::new(),
                else_ranges_: Vec::new(),
            }
        }

        pub fn get_range_at_index(&self, kind: SourceRangeKind, index: usize) -> SourceRange {
            if kind == SourceRangeKind::kThen {
                assert!(index < self.then_ranges_.len());
                self.then_ranges_[index]
            } else {
                assert_eq!(kind, SourceRangeKind::kElse);
                assert!(index < self.else_ranges_.len());
                self.else_ranges_[index]
            }
        }

        pub fn add_then_range(&mut self, range: SourceRange) {
            self.then_ranges_.push(range);
        }

        pub fn add_else_range(&mut self, else_range: SourceRange) {
            self.else_ranges_.push(else_range);
        }

        pub fn range_count(&self) -> usize {
            self.then_ranges_.len()
        }
    }
    impl AstNodeSourceRangesMethods for ConditionalChainSourceRanges{
        fn get_range(&self, _kind: SourceRangeKind) -> SourceRange {
            panic!("Unreachable");
        }

        fn has_range(&self, _kind: SourceRangeKind) -> bool {
            false
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct ConditionalSourceRanges {
        then_range_: SourceRange,
        else_range_: SourceRange,
    }

    impl ConditionalSourceRanges {
        pub fn new(then_range: SourceRange, else_range: SourceRange) -> Self {
            ConditionalSourceRanges { then_range_: then_range, else_range_: else_range }
        }
    }
    impl AstNodeSourceRangesMethods for ConditionalSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::kThen => self.then_range_,
                SourceRangeKind::kElse => self.else_range_,
                _ => panic!("Unreachable"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kThen || kind == SourceRangeKind::kElse
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct FunctionLiteralSourceRanges;

    impl FunctionLiteralSourceRanges {
        pub fn new() -> Self {
            FunctionLiteralSourceRanges {}
        }
    }
    impl AstNodeSourceRangesMethods for FunctionLiteralSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::function_literal_marker_range()
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kBody
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct IfStatementSourceRanges {
        then_range_: SourceRange,
        else_range_: SourceRange,
        has_continuation_: bool,
    }

    impl IfStatementSourceRanges {
        pub fn new(then_range: SourceRange, else_range: SourceRange) -> Self {
            IfStatementSourceRanges {
                then_range_: then_range,
                else_range_: else_range,
                has_continuation_: true,
            }
        }
    }
    impl AstNodeSourceRangesMethods for IfStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::kElse => self.else_range_,
                SourceRangeKind::kThen => self.then_range_,
                SourceRangeKind::kContinuation => {
                    if !self.has_continuation_ {
                        return SourceRange::empty();
                    }
                    let trailing_range = if self.else_range_.is_empty() {
                        self.then_range_
                    } else {
                        self.else_range_
                    };
                    SourceRange::continuation_of(&trailing_range, kNoSourcePosition)
                }
                _ => panic!("Unreachable"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kThen || kind == SourceRangeKind::kElse || kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.has_continuation_ = false;
        }
    }

    pub struct IterationStatementSourceRanges {
        body_range_: SourceRange,
        has_continuation_: bool,
    }

    impl IterationStatementSourceRanges {
        pub fn new(body_range: SourceRange) -> Self {
            IterationStatementSourceRanges {
                body_range_: body_range,
                has_continuation_: true,
            }
        }
    }
    impl AstNodeSourceRangesMethods for IterationStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::kBody => self.body_range_,
                SourceRangeKind::kContinuation => {
                    if !self.has_continuation_ {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.body_range_, kNoSourcePosition)
                }
                _ => panic!("Unreachable"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kBody || kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.has_continuation_ = false;
        }
    }

    pub struct JumpStatementSourceRanges {
        continuation_position_: i32,
    }

    impl JumpStatementSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            JumpStatementSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for JumpStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct NaryOperationSourceRanges {
        ranges_: Vec<SourceRange>,
    }

    impl NaryOperationSourceRanges {
        pub fn new(range: SourceRange) -> Self {
            NaryOperationSourceRanges {
                ranges_: vec![range],
            }
        }

        pub fn get_range_at_index(&self, index: usize) -> SourceRange {
            assert!(index < self.ranges_.len());
            self.ranges_[index]
        }

        pub fn add_range(&mut self, range: SourceRange) {
            self.ranges_.push(range);
        }

        pub fn range_count(&self) -> usize {
            self.ranges_.len()
        }
    }
    impl AstNodeSourceRangesMethods for NaryOperationSourceRanges{
        fn get_range(&self, _kind: SourceRangeKind) -> SourceRange {
            panic!("Unreachable");
        }

        fn has_range(&self, _kind: SourceRangeKind) -> bool {
            false
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct ExpressionSourceRanges {
        right_range_: SourceRange,
    }

    impl ExpressionSourceRanges {
        pub fn new(right_range: SourceRange) -> Self {
            ExpressionSourceRanges { right_range_: right_range }
        }
    }
    impl AstNodeSourceRangesMethods for ExpressionSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.right_range_
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kRight
        }
         fn remove_continuation_range(&mut self){}
    }

    pub struct SuspendSourceRanges {
        continuation_position_: i32,
    }

    impl SuspendSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            SuspendSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for SuspendSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct SwitchStatementSourceRanges {
        continuation_position_: i32,
    }

    impl SwitchStatementSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            SwitchStatementSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for SwitchStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct ThrowSourceRanges {
        continuation_position_: i32,
    }

    impl ThrowSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            ThrowSourceRanges { continuation_position_: continuation_position }
        }
    }
    impl AstNodeSourceRangesMethods for ThrowSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position_)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.continuation_position_ = kNoSourcePosition;
        }
    }

    pub struct TryCatchStatementSourceRanges {
        catch_range_: SourceRange,
        has_continuation_: bool,
    }

    impl TryCatchStatementSourceRanges {
        pub fn new(catch_range: SourceRange) -> Self {
            TryCatchStatementSourceRanges {
                catch_range_: catch_range,
                has_continuation_: true,
            }
        }
    }
    impl AstNodeSourceRangesMethods for TryCatchStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::kCatch => self.catch_range_,
                SourceRangeKind::kContinuation => {
                    if !self.has_continuation_ {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.catch_range_, kNoSourcePosition)
                }
                _ => panic!("Unreachable"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kCatch || kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.has_continuation_ = false;
        }
    }

    pub struct TryFinallyStatementSourceRanges {
        finally_range_: SourceRange,
        has_continuation_: bool,
    }

    impl TryFinallyStatementSourceRanges {
        pub fn new(finally_range: SourceRange) -> Self {
            TryFinallyStatementSourceRanges {
                finally_range_: finally_range,
                has_continuation_: true,
            }
        }
    }
    impl AstNodeSourceRangesMethods for TryFinallyStatementSourceRanges{
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::kFinally => self.finally_range_,
                SourceRangeKind::kContinuation => {
                    if !self.has_continuation_ {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.finally_range_, kNoSourcePosition)
                }
                _ => panic!("Unreachable"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::kFinally || kind == SourceRangeKind::kContinuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::kContinuation));
            self.has_continuation_ = false;
        }
    }

    pub struct SourceRangeMap {
        map_: HashMap<*const ZoneObject, Box<dyn AstNodeSourceRangesMethods>>,
    }

    impl SourceRangeMap {
        pub fn new() -> Self {
            SourceRangeMap { map_: HashMap::new() }
        }

        pub fn find(&self, node: *const ZoneObject) -> Option<&dyn AstNodeSourceRangesMethods> {
            self.map_.get(&node).map(|boxed_trait| boxed_trait.as_ref())
        }

        pub fn insert_binary_operation(&mut self, node: *mut BinaryOperation, ranges: BinaryOperationSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_block(&mut self, node: *mut Block, ranges: BlockSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_case_clause(&mut self, node: *mut CaseClause, ranges: CaseClauseSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_conditional_chain(&mut self, node: *mut ConditionalChain, ranges: ConditionalChainSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_conditional(&mut self, node: *mut Conditional, ranges: ConditionalSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_expression(&mut self, node: *mut Expression, ranges: ExpressionSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_function_literal(&mut self, node: *mut FunctionLiteral, ranges: FunctionLiteralSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_if_statement(&mut self, node: *mut IfStatement, ranges: IfStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_iteration_statement(&mut self, node: *mut IterationStatement, ranges: IterationStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_jump_statement(&mut self, node: *mut JumpStatement, ranges: JumpStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_nary_operation(&mut self, node: *mut NaryOperation, ranges: NaryOperationSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_suspend(&mut self, node: *mut Suspend, ranges: SuspendSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_switch_statement(&mut self, node: *mut SwitchStatement, ranges: SwitchStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_throw(&mut self, node: *mut Throw, ranges: ThrowSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_try_catch_statement(&mut self, node: *mut TryCatchStatement, ranges: TryCatchStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }

        pub fn insert_try_finally_statement(&mut self, node: *mut TryFinallyStatement, ranges: TryFinallyStatementSourceRanges) {
            assert!(!node.is_null());
            self.map_.insert(node as *const ZoneObject, Box::new(ranges));
        }
    }
}
