// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ast_source_ranges {
    use std::collections::HashMap;

    /// Specifies a range within the source code. {start} is 0-based and inclusive,
    /// {end} is 0-based and exclusive.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SourceRange {
        pub start: i32,
        pub end: i32,
    }

    impl SourceRange {
        pub const K_NO_SOURCE_POSITION: i32 = -1;
        pub const K_FUNCTION_LITERAL_SOURCE_POSITION: i32 = -2;

        pub fn new(start: i32, end: i32) -> Self {
            SourceRange { start, end }
        }

        pub fn empty() -> Self {
            SourceRange {
                start: Self::K_NO_SOURCE_POSITION,
                end: Self::K_NO_SOURCE_POSITION,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.start == Self::K_NO_SOURCE_POSITION
        }

        pub fn open_ended(start: i32) -> Self {
            SourceRange {
                start,
                end: Self::K_NO_SOURCE_POSITION,
            }
        }

        pub fn continuation_of(that: &SourceRange, end: i32) -> Self {
            if that.is_empty() {
                Self::empty()
            } else {
                SourceRange {
                    start: that.end,
                    end,
                }
            }
        }

        pub fn function_literal_marker_range() -> Self {
            SourceRange {
                start: Self::K_FUNCTION_LITERAL_SOURCE_POSITION,
                end: Self::K_FUNCTION_LITERAL_SOURCE_POSITION,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SourceRangeKind {
        Body,
        Catch,
        Continuation,
        Else,
        Finally,
        Right,
        Then,
    }

    pub trait AstNodeSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange;
        fn has_range(&self, kind: SourceRangeKind) -> bool;
        fn remove_continuation_range(&mut self) {}
    }

    #[derive(Debug)]
    pub struct BinaryOperationSourceRanges {
        right_range: SourceRange,
    }

    impl BinaryOperationSourceRanges {
        pub fn new(right_range: SourceRange) -> Self {
            BinaryOperationSourceRanges { right_range }
        }
    }

    impl AstNodeSourceRanges for BinaryOperationSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.right_range
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Right
        }
    }

    #[derive(Debug)]
    pub struct ContinuationSourceRanges {
        continuation_position: i32,
    }

    impl ContinuationSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            ContinuationSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for ContinuationSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::Continuation));
            self.continuation_position = SourceRange::K_NO_SOURCE_POSITION;
        }
    }

    #[derive(Debug)]
    pub struct BlockSourceRanges {
        continuation_position: i32,
    }

    impl BlockSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            BlockSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for BlockSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            let mut continuation_ranges = ContinuationSourceRanges::new(self.continuation_position);
            continuation_ranges.remove_continuation_range();
            self.continuation_position = continuation_ranges.continuation_position;
        }
    }

    #[derive(Debug)]
    pub struct CaseClauseSourceRanges {
        body_range: SourceRange,
    }

    impl CaseClauseSourceRanges {
        pub fn new(body_range: SourceRange) -> Self {
            CaseClauseSourceRanges { body_range }
        }
    }

    impl AstNodeSourceRanges for CaseClauseSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.body_range
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Body
        }
    }

    #[derive(Debug)]
    pub struct ConditionalChainSourceRanges {
        then_ranges: Vec<SourceRange>,
        else_ranges: Vec<SourceRange>,
    }

    impl ConditionalChainSourceRanges {
        pub fn new() -> Self {
            ConditionalChainSourceRanges {
                then_ranges: Vec::new(),
                else_ranges: Vec::new(),
            }
        }

        pub fn get_range_at_index(&self, kind: SourceRangeKind, index: usize) -> SourceRange {
            match kind {
                SourceRangeKind::Then => {
                    assert!(index < self.then_ranges.len());
                    self.then_ranges[index]
                }
                SourceRangeKind::Else => {
                    assert!(index < self.else_ranges.len());
                    self.else_ranges[index]
                }
                _ => panic!("Unexpected SourceRangeKind"),
            }
        }

        pub fn add_then_range(&mut self, range: SourceRange) {
            self.then_ranges.push(range);
        }

        pub fn add_else_range(&mut self, else_range: SourceRange) {
            self.else_ranges.push(else_range);
        }

        pub fn range_count(&self) -> usize {
            self.then_ranges.len()
        }
    }

    impl AstNodeSourceRanges for ConditionalChainSourceRanges {
        fn get_range(&self, _kind: SourceRangeKind) -> SourceRange {
            panic!("UNREACHABLE");
        }
        fn has_range(&self, _kind: SourceRangeKind) -> bool {
            false
        }
    }

    #[derive(Debug)]
    pub struct ConditionalSourceRanges {
        then_range: SourceRange,
        else_range: SourceRange,
    }

    impl ConditionalSourceRanges {
        pub fn new(then_range: SourceRange, else_range: SourceRange) -> Self {
            ConditionalSourceRanges {
                then_range,
                else_range,
            }
        }
    }

    impl AstNodeSourceRanges for ConditionalSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::Then => self.then_range,
                SourceRangeKind::Else => self.else_range,
                _ => panic!("UNREACHABLE"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Then || kind == SourceRangeKind::Else
        }
    }

    #[derive(Debug)]
    pub struct FunctionLiteralSourceRanges {}

    impl FunctionLiteralSourceRanges {
        pub fn new() -> Self {
            FunctionLiteralSourceRanges {}
        }
    }

    impl AstNodeSourceRanges for FunctionLiteralSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::function_literal_marker_range()
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Body
        }
    }

    #[derive(Debug)]
    pub struct IfStatementSourceRanges {
        then_range: SourceRange,
        else_range: SourceRange,
        has_continuation: bool,
    }

    impl IfStatementSourceRanges {
        pub fn new(then_range: SourceRange, else_range: SourceRange) -> Self {
            IfStatementSourceRanges {
                then_range,
                else_range,
                has_continuation: true,
            }
        }
    }

    impl AstNodeSourceRanges for IfStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::Else => self.else_range,
                SourceRangeKind::Then => self.then_range,
                SourceRangeKind::Continuation => {
                    if !self.has_continuation {
                        return SourceRange::empty();
                    }
                    let trailing_range = if self.else_range.is_empty() {
                        self.then_range
                    } else {
                        self.else_range
                    };
                    SourceRange::continuation_of(&trailing_range, SourceRange::K_NO_SOURCE_POSITION)
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Then
                || kind == SourceRangeKind::Else
                || kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::Continuation));
            self.has_continuation = false;
        }
    }

    #[derive(Debug)]
    pub struct IterationStatementSourceRanges {
        body_range: SourceRange,
        has_continuation: bool,
    }

    impl IterationStatementSourceRanges {
        pub fn new(body_range: SourceRange) -> Self {
            IterationStatementSourceRanges {
                body_range,
                has_continuation: true,
            }
        }
    }

    impl AstNodeSourceRanges for IterationStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::Body => self.body_range,
                SourceRangeKind::Continuation => {
                    if !self.has_continuation {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.body_range, SourceRange::K_NO_SOURCE_POSITION)
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Body || kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::Continuation));
            self.has_continuation = false;
        }
    }

    #[derive(Debug)]
    pub struct JumpStatementSourceRanges {
        continuation_position: i32,
    }

    impl JumpStatementSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            JumpStatementSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for JumpStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            let mut continuation_ranges = ContinuationSourceRanges::new(self.continuation_position);
            continuation_ranges.remove_continuation_range();
            self.continuation_position = continuation_ranges.continuation_position;
        }
    }

    #[derive(Debug)]
    pub struct NaryOperationSourceRanges {
        ranges: Vec<SourceRange>,
    }

    impl NaryOperationSourceRanges {
        pub fn new(range: SourceRange) -> Self {
            NaryOperationSourceRanges {
                ranges: vec![range],
            }
        }

        pub fn get_range_at_index(&self, index: usize) -> SourceRange {
            assert!(index < self.ranges.len());
            self.ranges[index]
        }

        pub fn add_range(&mut self, range: SourceRange) {
            self.ranges.push(range);
        }
        pub fn range_count(&self) -> usize {
            self.ranges.len()
        }
    }

    impl AstNodeSourceRanges for NaryOperationSourceRanges {
        fn get_range(&self, _kind: SourceRangeKind) -> SourceRange {
            panic!("UNREACHABLE");
        }
        fn has_range(&self, _kind: SourceRangeKind) -> bool {
            false
        }
    }

    #[derive(Debug)]
    pub struct ExpressionSourceRanges {
        right_range: SourceRange,
    }

    impl ExpressionSourceRanges {
        pub fn new(right_range: SourceRange) -> Self {
            ExpressionSourceRanges { right_range }
        }
    }

    impl AstNodeSourceRanges for ExpressionSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            self.right_range
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Right
        }
    }

    #[derive(Debug)]
    pub struct SuspendSourceRanges {
        continuation_position: i32,
    }

    impl SuspendSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            SuspendSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for SuspendSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            let mut continuation_ranges = ContinuationSourceRanges::new(self.continuation_position);
            continuation_ranges.remove_continuation_range();
            self.continuation_position = continuation_ranges.continuation_position;
        }
    }

    #[derive(Debug)]
    pub struct SwitchStatementSourceRanges {
        continuation_position: i32,
    }

    impl SwitchStatementSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            SwitchStatementSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for SwitchStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            let mut continuation_ranges = ContinuationSourceRanges::new(self.continuation_position);
            continuation_ranges.remove_continuation_range();
            self.continuation_position = continuation_ranges.continuation_position;
        }
    }

    #[derive(Debug)]
    pub struct ThrowSourceRanges {
        continuation_position: i32,
    }

    impl ThrowSourceRanges {
        pub fn new(continuation_position: i32) -> Self {
            ThrowSourceRanges {
                continuation_position,
            }
        }
    }

    impl AstNodeSourceRanges for ThrowSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            SourceRange::open_ended(self.continuation_position)
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            let mut continuation_ranges = ContinuationSourceRanges::new(self.continuation_position);
            continuation_ranges.remove_continuation_range();
            self.continuation_position = continuation_ranges.continuation_position;
        }
    }

    #[derive(Debug)]
    pub struct TryCatchStatementSourceRanges {
        catch_range: SourceRange,
        has_continuation: bool,
    }

    impl TryCatchStatementSourceRanges {
        pub fn new(catch_range: SourceRange) -> Self {
            TryCatchStatementSourceRanges {
                catch_range,
                has_continuation: true,
            }
        }
    }

    impl AstNodeSourceRanges for TryCatchStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::Catch => self.catch_range,
                SourceRangeKind::Continuation => {
                    if !self.has_continuation {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.catch_range, SourceRange::K_NO_SOURCE_POSITION)
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Catch || kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::Continuation));
            self.has_continuation = false;
        }
    }

    #[derive(Debug)]
    pub struct TryFinallyStatementSourceRanges {
        finally_range: SourceRange,
        has_continuation: bool,
    }

    impl TryFinallyStatementSourceRanges {
        pub fn new(finally_range: SourceRange) -> Self {
            TryFinallyStatementSourceRanges {
                finally_range,
                has_continuation: true,
            }
        }
    }

    impl AstNodeSourceRanges for TryFinallyStatementSourceRanges {
        fn get_range(&self, kind: SourceRangeKind) -> SourceRange {
            assert!(self.has_range(kind));
            match kind {
                SourceRangeKind::Finally => self.finally_range,
                SourceRangeKind::Continuation => {
                    if !self.has_continuation {
                        return SourceRange::empty();
                    }
                    SourceRange::continuation_of(&self.finally_range, SourceRange::K_NO_SOURCE_POSITION)
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        fn has_range(&self, kind: SourceRangeKind) -> bool {
            kind == SourceRangeKind::Finally || kind == SourceRangeKind::Continuation
        }

        fn remove_continuation_range(&mut self) {
            assert!(self.has_range(SourceRangeKind::Continuation));
            self.has_continuation = false;
        }
    }

    // Dummy ZoneObject type, replace with actual type if needed
    pub trait ZoneObject {}
    impl<T> ZoneObject for T {}

    /// Maps ast node pointers to associated source ranges. The parser creates these
    /// mappings and the bytecode generator consumes them.
    #[derive(Debug)]
    pub struct SourceRangeMap<'a> {
        map: HashMap<&'a dyn ZoneObject, Box<dyn AstNodeSourceRanges>>,
    }

    impl<'a> SourceRangeMap<'a> {
        pub fn new() -> Self {
            SourceRangeMap {
                map: HashMap::new(),
            }
        }

        pub fn find(&self, node: &'a dyn ZoneObject) -> Option<&dyn AstNodeSourceRanges> {
            self.map.get(node).map(|boxed_trait_object| boxed_trait_object.as_ref())
        }

        pub fn insert_binary_operation(
            &mut self,
            node: &'a dyn ZoneObject,
            ranges: BinaryOperationSourceRanges,
        ) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_block(&mut self, node: &'a dyn ZoneObject, ranges: BlockSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_case_clause(&mut self, node: &'a dyn ZoneObject, ranges: CaseClauseSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_conditional_chain(&mut self, node: &'a dyn ZoneObject, ranges: ConditionalChainSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_conditional(&mut self, node: &'a dyn ZoneObject, ranges: ConditionalSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_expression(&mut self, node: &'a dyn ZoneObject, ranges: ExpressionSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_function_literal(&mut self, node: &'a dyn ZoneObject, ranges: FunctionLiteralSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_if_statement(&mut self, node: &'a dyn ZoneObject, ranges: IfStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_iteration_statement(&mut self, node: &'a dyn ZoneObject, ranges: IterationStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_jump_statement(&mut self, node: &'a dyn ZoneObject, ranges: JumpStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_nary_operation(&mut self, node: &'a dyn ZoneObject, ranges: NaryOperationSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_suspend(&mut self, node: &'a dyn ZoneObject, ranges: SuspendSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_switch_statement(&mut self, node: &'a dyn ZoneObject, ranges: SwitchStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_throw(&mut self, node: &'a dyn ZoneObject, ranges: ThrowSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_try_catch_statement(&mut self, node: &'a dyn ZoneObject, ranges: TryCatchStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }

        pub fn insert_try_finally_statement(&mut self, node: &'a dyn ZoneObject, ranges: TryFinallyStatementSourceRanges) {
            self.map.insert(node, Box::new(ranges));
        }
    }
}