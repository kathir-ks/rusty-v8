// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/regexp/regexp-ast.h

use std::{
    cmp::{max, min},
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXor},
    vec,
};

// TODO: Replace with actual implementations/crates.
mod base {
    pub type uc32 = u32;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Vector<T> {
        data: *const T,
        length: usize,
        _phantom: PhantomData<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: *const T, length: usize) -> Self {
            Vector {
                data,
                length,
                _phantom: PhantomData,
            }
        }

        pub fn len(&self) -> usize {
            self.length
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index < self.length {
                unsafe { Some(&*self.data.add(index)) }
            } else {
                None
            }
        }
    }

    impl<T> std::ops::Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.length {
                panic!("Index out of bounds");
            }
            unsafe { &*self.data.add(index) }
        }
    }
}

// mod unicode; // Placeholder for the unicode crate/module

mod zone {
    use std::{
        cell::RefCell,
        collections::LinkedList,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    pub struct Zone {
        // TODO: Implement zone-based memory management.
        //  This is a simplified version that just uses a linked list to track allocated objects.
        allocated: RefCell<LinkedList<Rc<dyn std::any::Any>>>,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {
                allocated: RefCell::new(LinkedList::new()),
            }
        }

        pub fn alloc<T>(&self, value: T) -> Rc<T>
        where
            T: 'static,
        {
            let rc = Rc::new(value);
            self.allocated.borrow_mut().push_back(rc.clone());
            // Need to cast Rc<T> to Rc<dyn Any> to store in the list
            let any_rc: Rc<dyn std::any::Any> = rc.clone();
            let rc_downcasted = any_rc.downcast::<T>().unwrap();
            rc_downcasted
        }

        pub fn new_list<T>(&self, capacity: usize) -> ZoneList<T> {
            ZoneList::new(capacity, self)
        }
    }

    #[derive(Debug, Clone)]
    pub struct ZoneList<T> {
        elements: Rc<RefCell<Vec<T>>>,
        zone: *const Zone,
    }

    impl<T> ZoneList<T> {
        pub fn new(capacity: usize, zone: &Zone) -> Self {
            ZoneList {
                elements: Rc::new(RefCell::new(Vec::with_capacity(capacity))),
                zone,
            }
        }

        pub fn add(&self, element: T, _zone: &Zone) {
            self.elements.borrow_mut().push(element);
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.elements.borrow().get(index)
        }

        pub fn len(&self) -> usize {
            self.elements.borrow().len()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.borrow().is_empty()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.elements.borrow().iter()
        }
    }

    impl<'a, T> IntoIterator for &'a ZoneList<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    // Mocked ZoneObject
    pub trait ZoneObject {}
}

//mod isolate; // Placeholder for isolate related code

pub mod regexp_flags {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RegExpFlags {
        pub global: bool,
        pub ignore_case: bool,
        pub multiline: bool,
        pub dot_all: bool,
        pub unicode: bool,
        pub sticky: bool,
        pub has_indices: bool,
    }

    impl RegExpFlags {
        pub fn new(
            global: bool,
            ignore_case: bool,
            multiline: bool,
            dot_all: bool,
            unicode: bool,
            sticky: bool,
            has_indices: bool,
        ) -> Self {
            Self {
                global,
                ignore_case,
                multiline,
                dot_all,
                unicode,
                sticky,
                has_indices,
            }
        }
    }
}

use base::Vector;
use regexp_flags::RegExpFlags;
use zone::{Zone, ZoneList, ZoneObject};

//use isolate::Isolate;

//use unicode::UnicodeSet;

const K_MAX_INT: i32 = i32::MAX;

macro_rules! for_each_reg_exp_tree_type {
    ($visit:ident) => {
        $visit!(Disjunction);
        $visit!(Alternative);
        $visit!(Assertion);
        $visit!(ClassRanges);
        $visit!(ClassSetOperand);
        $visit!(ClassSetExpression);
        $visit!(Atom);
        $visit!(Quantifier);
        $visit!(Capture);
        $visit!(Group);
        $visit!(Lookaround);
        $visit!(BackReference);
        $visit!(Empty);
        $visit!(Text);
    };
}

pub trait RegExpVisitor {
    fn visit_disjunction(&mut self, node: &RegExpDisjunction, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_alternative(&mut self, node: &RegExpAlternative, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_assertion(&mut self, node: &RegExpAssertion, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_class_ranges(&mut self, node: &RegExpClassRanges, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_class_set_operand(&mut self, node: &RegExpClassSetOperand, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_class_set_expression(&mut self, node: &RegExpClassSetExpression, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_atom(&mut self, node: &RegExpAtom, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_quantifier(&mut self, node: &RegExpQuantifier, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_capture(&mut self, node: &RegExpCapture, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_group(&mut self, node: &RegExpGroup, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_lookaround(&mut self, node: &RegExpLookaround, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_back_reference(&mut self, node: &RegExpBackReference, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_empty(&mut self, node: &RegExpEmpty, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn visit_text(&mut self, node: &RegExpText, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

trait RegExpNode {}

trait RegExpCompiler {}

trait RegExpTreeTrait: ZoneObject {
    fn accept(&self, visitor: &mut dyn RegExpVisitor, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn to_node(&self, compiler: &mut dyn RegExpCompiler, on_success: &mut dyn RegExpNode) -> Box<dyn RegExpNode>;
    fn is_text_element(&self) -> bool {
        false
    }
    fn is_anchored_at_start(&self) -> bool {
        false
    }
    fn is_anchored_at_end(&self) -> bool {
        false
    }
    fn min_match(&self) -> i32;
    fn max_match(&self) -> i32;
    fn capture_registers(&self) -> Interval {
        Interval::empty()
    }
    fn append_to_text(&self, text: &mut RegExpText, zone: &Zone) {}
    fn print(&self, os: &mut std::fmt::Write, zone: &Zone) -> std::fmt::Result {
        write!(os, "RegExpTree")
    }
    fn as_disjunction(&self) -> Option<&RegExpDisjunction> {
        None
    }
    fn is_disjunction(&self) -> bool {
        false
    }
    fn as_alternative(&self) -> Option<&RegExpAlternative> {
        None
    }
    fn is_alternative(&self) -> bool {
        false
    }
    fn as_assertion(&self) -> Option<&RegExpAssertion> {
        None
    }
    fn is_assertion(&self) -> bool {
        false
    }
    fn as_class_ranges(&self) -> Option<&RegExpClassRanges> {
        None
    }
    fn is_class_ranges(&self) -> bool {
        false
    }
    fn as_class_set_operand(&self) -> Option<&RegExpClassSetOperand> {
        None
    }
    fn is_class_set_operand(&self) -> bool {
        false
    }
    fn as_class_set_expression(&self) -> Option<&RegExpClassSetExpression> {
        None
    }
    fn is_class_set_expression(&self) -> bool {
        false
    }
    fn as_atom(&self) -> Option<&RegExpAtom> {
        None
    }
    fn is_atom(&self) -> bool {
        false
    }
    fn as_quantifier(&self) -> Option<&RegExpQuantifier> {
        None
    }
    fn is_quantifier(&self) -> bool {
        false
    }
    fn as_capture(&self) -> Option<&RegExpCapture> {
        None
    }
    fn is_capture(&self) -> bool {
        false
    }
    fn as_group(&self) -> Option<&RegExpGroup> {
        None
    }
    fn is_group(&self) -> bool {
        false
    }
    fn as_lookaround(&self) -> Option<&RegExpLookaround> {
        None
    }
    fn is_lookaround(&self) -> bool {
        false
    }
    fn as_back_reference(&self) -> Option<&RegExpBackReference> {
        None
    }
    fn is_back_reference(&self) -> bool {
        false
    }
    fn as_empty(&self) -> Option<&RegExpEmpty> {
        None
    }
    fn is_empty(&self) -> bool {
        false
    }
    fn as_text(&self) -> Option<&RegExpText> {
        None
    }
    fn is_text(&self) -> bool {
        false
    }
}

/// A simple closed interval.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Interval {
    from_: i32,
    to_: i32,
}

impl Interval {
    const K_NONE: i32 = -1;

    /// Creates an empty interval.
    pub fn new() -> Self {
        Interval {
            from_: Self::K_NONE,
            to_: Self::K_NONE - 1, // '- 1' for branchless size().
        }
    }

    /// Creates an interval with the given bounds.
    pub fn from(from: i32, to: i32) -> Self {
        Interval { from_: from, to_: to }
    }

    /// Returns the union of this interval with another.
    pub fn union(self, that: Interval) -> Self {
        if that.from_ == Self::K_NONE {
            return self;
        }
        if self.from_ == Self::K_NONE {
            return that;
        }
        Interval {
            from_: min(self.from_, that.from_),
            to_: max(self.to_, that.to_),
        }
    }

    /// Creates an empty interval.
    pub fn empty() -> Self {
        Interval::new()
    }

    /// Returns whether this interval contains the given value.
    pub fn contains(self, value: i32) -> bool {
        (self.from_ <= value) && (value <= self.to_)
    }

    /// Returns whether this interval is empty.
    pub fn is_empty(self) -> bool {
        self.from_ == Self::K_NONE
    }

    /// Returns the lower bound of this interval.
    pub fn from(self) -> i32 {
        self.from_
    }

    /// Returns the upper bound of this interval.
    pub fn to(self) -> i32 {
        self.to_
    }

    /// Returns the size of this interval.
    pub fn size(self) -> i32 {
        self.to_ - self.from_ + 1
    }
}

/// Named standard character sets.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StandardCharacterSet {
    /// Like /\s/.
    Whitespace = 's' as isize,
    /// Like /\S/.
    NotWhitespace = 'S' as isize,
    /// Like /\w/.
    Word = 'w' as isize,
    /// Like /\W/.
    NotWord = 'W' as isize,
    /// Like /\d/.
    Digit = 'd' as isize,
    /// Like /\D/.
    NotDigit = 'D' as isize,
    /// The inverse of /./.
    LineTerminator = 'n' as isize,
    /// Like /./.
    NotLineTerminator = '.' as isize,
    /// Matches every character, like /./s.
    Everything = '*' as isize,
}

/// Represents code points (with values up to 0x10FFFF) in the range from from_
/// to to_, both ends are inclusive.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CharacterRange {
    from_: base::uc32,
    to_: base::uc32,
}

impl CharacterRange {
    const K_MAX_CODE_POINT: base::uc32 = 0x10ffff;

    pub fn new() -> Self {
        CharacterRange { from_: 0, to_: 0 }
    }

    /// Creates a character range containing only the given value.
    pub fn singleton(value: base::uc32) -> Self {
        CharacterRange {
            from_: value,
            to_: value,
        }
    }

    /// Creates a character range with the given bounds.
    pub fn range(from: base::uc32, to: base::uc32) -> Self {
        assert!(from <= to);
        assert!(to <= Self::K_MAX_CODE_POINT);
        CharacterRange {
            from_: from,
            to_: to,
        }
    }

    /// Creates a character range matching everything.
    pub fn everything() -> Self {
        CharacterRange {
            from_: 0,
            to_: Self::K_MAX_CODE_POINT,
        }
    }

    /// Creates a list containing only the given character range.
    pub fn list(zone: &Zone, range: CharacterRange) -> ZoneList<CharacterRange> {
        let mut list = zone.new_list::<CharacterRange>(1);
        list.add(range, zone);
        list
    }

    // Add class escapes. Add case equivalent closure for \w and \W if necessary.
    pub fn add_class_escape(
        standard_character_set: StandardCharacterSet,
        ranges: &mut ZoneList<CharacterRange>,
        add_unicode_case_equivalents: bool,
        zone: &Zone,
    ) {
        // V8_EXPORT_PRIVATE static void AddClassEscape(
        //     StandardCharacterSet standard_character_set,
        //     ZoneList<CharacterRange>* ranges, bool add_unicode_case_equivalents,
        //     Zone* zone);
        todo!()
    }

    // Add case equivalents to ranges. Only used for /i, not for /ui or /vi, as
    // the semantics for unicode mode are slightly different.
    // See https://tc39.es/ecma262/#sec-runtime-semantics-canonicalize-ch Note 4.
    pub fn add_case_equivalents(
        //isolate: &mut Isolate,
        zone: &Zone,
        ranges: &mut ZoneList<CharacterRange>,
        is_one_byte: bool,
    ) {
        // V8_EXPORT_PRIVATE static void AddCaseEquivalents(
        //     Isolate* isolate, Zone* zone, ZoneList<CharacterRange>* ranges,
        //     bool is_one_byte);
        todo!()
    }
    // Add case equivalent code points to ranges. Only used for /ui and /vi, not
    // for /i, as the semantics for non-unicode mode are slightly different.
    // See https://tc39.es/ecma262/#sec-runtime-semantics-canonicalize-ch Note 4.
    pub fn add_unicode_case_equivalents(ranges: &mut ZoneList<CharacterRange>, zone: &Zone) {
        todo!()
    }

    /// Returns whether this range contains the given code point.
    pub fn contains(self, i: base::uc32) -> bool {
        self.from_ <= i && i <= self.to_
    }

    /// Returns the lower bound of this range.
    pub fn from(self) -> base::uc32 {
        self.from_
    }

    /// Returns the upper bound of this range.
    pub fn to(self) -> base::uc32 {
        self.to_
    }

    /// Returns whether this range matches everything up to the given maximum value.
    pub fn is_everything(self, max: base::uc32) -> bool {
        self.from_ == 0 && self.to_ >= max
    }

    /// Returns whether this range contains only a single code point.
    pub fn is_singleton(self) -> bool {
        self.from_ == self.to_
    }

    // Whether a range list is in canonical form: Ranges ordered by from value,
    // and ranges non-overlapping and non-adjacent.
    pub fn is_canonical(ranges: &ZoneList<CharacterRange>) -> bool {
        //V8_EXPORT_PRIVATE static bool IsCanonical(
        //    const ZoneList<CharacterRange>* ranges);
        todo!()
    }
    // Convert range list to canonical form. The characters covered by the ranges
    // will still be the same, but no character is in more than one range, and
    // adjacent ranges are merged. The resulting list may be shorter than the
    // original, but cannot be longer.
    pub fn canonicalize(ranges: &mut ZoneList<CharacterRange>) {
        todo!()
    }
    // Negate the contents of a character range in canonical form.
    pub fn negate(src: &ZoneList<CharacterRange>, dst: &mut ZoneList<CharacterRange>, zone: &Zone) {
        todo!()
    }
    // Intersect the contents of two character ranges in canonical form.
    pub fn intersect(
        lhs: &ZoneList<CharacterRange>,
        rhs: &ZoneList<CharacterRange>,
        dst: &mut ZoneList<CharacterRange>,
        zone: &Zone,
    ) {
        todo!()
    }
    // Subtract the contents of |to_remove| from the contents of |src|.
    pub fn subtract(
        src: &ZoneList<CharacterRange>,
        to_remove: &ZoneList<CharacterRange>,
        dst: &mut ZoneList<CharacterRange>,
        zone: &Zone,
    ) {
        todo!()
    }
    // Remove all ranges outside the one-byte range.
    pub fn clamp_to_one_byte(ranges: &mut ZoneList<CharacterRange>) {
        todo!()
    }
    // Checks if two ranges (both need to be canonical) are equal.
    pub fn equals(lhs: &ZoneList<CharacterRange>, rhs: &ZoneList<CharacterRange>) -> bool {
        todo!()
    }
}

pub trait RegExpTree: RegExpTreeTrait {
    const K_INFINITY: i32 = K_MAX_INT;
    fn accept(&self, visitor: &mut dyn RegExpVisitor, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn to_node(&self, compiler: &mut dyn RegExpCompiler, on_success: &mut dyn RegExpNode) -> Box<dyn RegExpNode>;
    fn is_text_element(&self) -> bool {
        false
    }
    fn is_anchored_at_start(&self) -> bool {
        false
    }
    fn is_anchored_at_end(&self) -> bool {
        false
    }
    fn min_match(&self) -> i32;
    fn max_match(&self) -> i32;
    fn capture_registers(&self) -> Interval {
        Interval::empty()
    }
    fn append_to_text(&self, text: &mut RegExpText, zone: &Zone) {}
    fn print(&self, os: &mut std::fmt::Write, zone: &Zone) -> std::fmt::Result {
        write!(os, "RegExpTree")
    }
    fn as_disjunction(&self) -> Option<&RegExpDisjunction> {
        None
    }
    fn is_disjunction(&self) -> bool {
        false
    }
    fn as_alternative(&self) -> Option<&RegExpAlternative> {
        None
    }
    fn is_alternative(&self) -> bool {
        false
    }
    fn as_assertion(&self) -> Option<&RegExpAssertion> {
        None
    }
    fn is_assertion(&self) -> bool {
        false
    }
    fn as_class_ranges(&self) -> Option<&RegExpClassRanges> {
        None
    }
    fn is_class_ranges(&self) -> bool {
        false
    }
    fn as_class_set_operand(&self) -> Option<&RegExpClassSetOperand> {
        None
    }
    fn is_class_set_operand(&self) -> bool {
        false
    }
    fn as_class_set_expression(&self) -> Option<&RegExpClassSetExpression> {
        None
    }
    fn is_class_set_expression(&self) -> bool {
        false
    }
    fn as_atom(&self) -> Option<&RegExpAtom> {
        None
    }
    fn is_atom(&self) -> bool {
        false
    }
    fn as_quantifier(&self) -> Option<&RegExpQuantifier> {
        None
    }
    fn is_quantifier(&self) -> bool {
        false
    }
    fn as_capture(&self) -> Option<&RegExpCapture> {
        None
    }
    fn is_capture(&self) -> bool {
        false
    }
    fn as_group(&self) -> Option<&RegExpGroup> {
        None
    }
    fn is_group(&self) -> bool {
        false
    }
    fn as_lookaround(&self) -> Option<&RegExpLookaround> {
        None
    }
    fn is_lookaround(&self) -> bool {
        false
    }
    fn as_back_reference(&self) -> Option<&RegExpBackReference> {
        None
    }
    fn is_back_reference(&self) -> bool {
        false
    }
    fn as_empty(&self) -> Option<&RegExpEmpty> {
        None
    }
    fn is_empty(&self) -> bool {
        false
    }
    fn as_text(&self) -> Option<&RegExpText> {
        None
    }
    fn is_text(&self) -> bool {
        false
    }
}

/// Represents a disjunction of regular expressions.
pub struct RegExpDisjunction {
    alternatives_: ZoneList<Box<dyn RegExpTree>>,
    min_match_: i32,
    max_match_: i32,
}

impl RegExpDisjunction {
    /// Creates a new disjunction with the given alternatives.
    pub fn new(alternatives: ZoneList<Box<dyn RegExpTree>>) -> Self {
        let mut min_match = K_MAX_INT;
        let mut max_match = 0;
        for alternative in &alternatives {
            min_match = min(min_match, alternative.min_match());
            max_match = max(max_match, alternative.max_match());
        }
        RegExpDisjunction {
            alternatives_: alternatives,
            min_match_: min_match,
            max_match_: max_match,
        }
    }

    fn sort_consecutive_atoms(&mut self, _compiler: &mut dyn RegExpCompiler) -> bool {
        todo!()
    }
    fn rationalize_consecutive_atoms(&mut self, _compiler: &mut dyn RegExpCompiler) {
        todo!()
    }
    fn fix_single_character_disjunctions(&mut self, _compiler: &mut dyn RegExpCompiler) {
        todo!()
    }

    /// Returns the list of alternatives in this disjunction.
    pub fn alternatives(&self) -> &ZoneList<Box<dyn RegExpTree>> {
        &self.alternatives_
    }
}

impl ZoneObject for RegExpDisjunction {}

impl RegExpTree for RegExpDisjunction {
    fn accept(&self, visitor: &mut dyn RegExpVisitor, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        visitor.visit_disjunction(self, data)
    }

    fn to_node(&self, _compiler: &mut dyn RegExpCompiler, _on_success: &mut dyn RegExpNode) -> Box<dyn RegExpNode> {
        todo!()
    }

    fn capture_registers(&self) -> Interval {
        let mut interval = Interval::empty();
        for alternative in &self.alternatives_ {
            interval = interval.union(alternative.capture_registers());
        }
        interval
    }

    fn is_anchored_at_start(&self) -> bool {
        self.alternatives_
            .iter()
            .all(|alternative| alternative.is_anchored_at_start())
    }

    fn is_anchored_at_end(&self) -> bool {
        self.alternatives_
            .iter()
            .all(|alternative| alternative.is_anchored_at_end())
    }

    fn min_match(&self) -> i32 {
        self.min_match_
    }

    fn max_match(&self) -> i32 {
        self.max_match_
    }

    fn as_disjunction(&self) -> Option<&RegExpDisjunction> {
        Some(self)
    }

    fn is_disjunction(&self) -> bool {
        true
    }
}

/// Represents an alternative in a regular expression.
pub struct RegExpAlternative {
    nodes_: ZoneList<Box<dyn RegExpTree>>,
    min_match_: i32,
    max_match_: i32,
}

impl RegExpAlternative {
    /// Creates a new alternative with the given nodes.
    pub fn new(nodes: ZoneList<Box<dyn RegExpTree>>) -> Self {
        let mut min_match = 0;
        let mut max_match = 0;
        for node in &nodes {
            min_match += node.min_match();
            if max_match != K_MAX_INT {
                if node.max_match() == K_MAX_INT {
                    max_match = K_MAX_INT;
                } else {
                    max_match += node.max_match();
                }
            }
            if max_match > K_MAX_INT {
                max_match = K_MAX_INT
            }
        }
        RegExpAlternative {
            nodes_: nodes,
            min_match_: min_match,
            max_match_: max_match,
        }
    }

    /// Returns the list of nodes in this alternative.
    pub fn nodes(&self) -> &ZoneList<Box<dyn RegExpTree>> {
        &self.nodes_
    }
}

impl ZoneObject for RegExpAlternative {}

impl RegExpTree for RegExpAlternative {
    fn accept(&self, visitor: &mut dyn RegExpVisitor, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        visitor.visit_alternative(self, data)
    }

    fn to_node(&self, _compiler: &mut dyn RegExpCompiler, _on_success: &mut dyn RegExpNode) -> Box<dyn RegExpNode> {
        todo!()
    }

    fn capture_registers(&self) -> Interval {
        let mut interval = Interval::empty();
        for node in &self.nodes_ {
            interval = interval.union(node.capture_registers());
        }
        interval
    }

    fn is_anchored_at_start(&self) -> bool {
        self.nodes_
            .get(0)
            .map_or(false, |node| node.is_anchored_at_start())
    }

    fn is_anchored_at_end(&self) -> bool {
        self.nodes_
            .get(self.nodes_.len().wrapping_sub(1))
            .map_or(false, |node| node.is_anchored_at_end())
    }

    fn min_match(&self) -> i32 {
        self.min_match_
    }

    fn max_match(&self) -> i32 {
        self.max_match_
    }

    fn as_alternative(&self) -> Option<&RegExpAlternative> {
        Some(self)
    }

    fn is_alternative(&self) -> bool {
        true
    }
}

/// Represents an assertion in a regular expression.
pub struct RegExpAssertion {
    assertion_type_: Type,
}

impl RegExpAssertion {
    /// Creates a new assertion with the given type.
    pub fn new(assertion_type: Type) -> Self {
        RegExpAssertion {
            assertion_type_: assertion_type,
        }
    }

    /// Returns the type of this assertion.
    pub fn assertion_type(&self) -> Type {
        self.assertion_type_
    }
}

impl ZoneObject for RegExpAssertion {}

impl RegExpTree for RegExpAssertion {
    fn accept(&self, visitor: &mut dyn RegExpVisitor, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        visitor.visit_assertion(self, data)
    }

    fn to_node(&self, _compiler: &mut dyn RegExpCompiler, _on_success: &mut dyn RegExpNode) -> Box<dyn RegExpNode> {
        todo!()
    }

    fn is_anchored_at_start(&self) -> bool {
        self.assertion_type_ == Type::START_OF_LINE || self.assertion_type_ == Type::START_OF_INPUT
    }

    fn is_anchored_at_end(&self) -> bool {
        self.assertion_type_ == Type::END_OF_LINE || self.assertion_type_ == Type::END_OF_INPUT
    }

    fn min_match(&self) -> i32 {
        0
    }

    fn max_match(&self) -> i32 {
        0
    }

    fn as_assertion(&self) -> Option<&RegExpAssertion> {
        Some(self)
    }

    fn is_assertion(&self) -> bool {
        true
    }
}

/// The type of an assertion.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    /// Matches the start of a line.
    START_OF_LINE = 0,
    /// Matches the start of the input.
    START_OF_INPUT = 1,
    /// Matches the end of a line.
    END_OF_LINE = 2,
    /// Matches the end of the input.
    END_OF_INPUT = 3,
    /// Matches a word boundary.
    BOUNDARY = 4,
    /// Matches a non-word boundary.
    NON_BOUNDARY = 5,
}

/// Represents a character set.
pub struct CharacterSet {
    ranges_: Option<ZoneList<CharacterRange>>,
    standard_set_type_: Option<StandardCharacterSet>,
}

impl CharacterSet {
    /// Creates a new character set with the given standard set type.
    pub fn new_standard(standard_set_type: StandardCharacterSet) -> Self {
        CharacterSet {
            ranges_: None,
            standard_set_type_: Some(standard_set_type),
        }
    }

    /// Creates a new character set with