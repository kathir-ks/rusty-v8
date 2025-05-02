// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::alloc::Allocator;
use std::cell::Cell;
use std::rc::Rc;

/// A label representing a loop header in a bytecode array. It is bound before
/// the jump is seen, so its position is always known by the time the jump is
/// reached.
#[derive(Debug, Default)]
pub struct BytecodeLoopHeader {
    offset_: Cell<usize>,
}

impl BytecodeLoopHeader {
    pub fn new() -> Self {
        BytecodeLoopHeader {
            offset_: Cell::new(Self::kInvalidOffset),
        }
    }

    pub fn offset(&self) -> usize {
        debug_assert_ne!(self.offset_.get(), Self::kInvalidOffset);
        self.offset_.get()
    }

    const kInvalidOffset: usize = usize::MAX;

    fn bind_to(&self, offset: usize) {
        debug_assert_ne!(offset, Self::kInvalidOffset);
        debug_assert_eq!(self.offset_.get(), Self::kInvalidOffset);
        self.offset_.set(offset);
    }
}

/// A label representing a forward branch target in a bytecode array. When a
/// label is bound, it represents a known position in the bytecode array. A label
/// can only have at most one referrer jump.
#[derive(Debug, Default)]
pub struct BytecodeLabel {
    bound_: Cell<bool>,
    jump_offset_: Cell<usize>,
}

impl BytecodeLabel {
    pub fn new() -> Self {
        BytecodeLabel {
            bound_: Cell::new(false),
            jump_offset_: Cell::new(Self::kInvalidOffset),
        }
    }

    pub fn is_bound(&self) -> bool {
        self.bound_.get()
    }

    pub fn jump_offset(&self) -> usize {
        debug_assert_ne!(self.jump_offset_.get(), Self::kInvalidOffset);
        self.jump_offset_.get()
    }

    pub fn has_referrer_jump(&self) -> bool {
        self.jump_offset_.get() != Self::kInvalidOffset
    }

    const kInvalidOffset: usize = usize::MAX;

    fn bind(&self) {
        debug_assert!(!self.bound_.get());
        self.bound_.set(true);
    }

    fn set_referrer(&self, offset: usize) {
        debug_assert!(!self.bound_.get());
        debug_assert_ne!(offset, Self::kInvalidOffset);
        debug_assert_eq!(self.jump_offset_.get(), Self::kInvalidOffset);
        self.jump_offset_.set(offset);
    }
}

trait BytecodeArrayBuilder {
    fn current_offset(&self) -> usize;
}

/// Class representing a branch target of multiple jumps.
pub struct BytecodeLabels {
    labels_: Vec<Rc<BytecodeLabel>>,
    is_bound_: Cell<bool>,
    //zone: Zone, //Needs to be implemented
}

impl BytecodeLabels {
    pub fn new() -> Self {
        BytecodeLabels {
            labels_: Vec::new(),
            is_bound_: Cell::new(false),
            //zone: Zone::new(),
        }
    }

    pub fn new_label(&mut self) -> Rc<BytecodeLabel> {
        let label = Rc::new(BytecodeLabel::new());
        self.labels_.push(label.clone());
        label
    }

    pub fn bind<T: BytecodeArrayBuilder>(&self, builder: &T) {
        for label in &self.labels_ {
            label.bind();
            // TODO: Add logic for backpatching using builder.current_offset() if needed.
        }
        self.is_bound_.set(true);
    }

    pub fn is_bound(&self) -> bool {
        if self.is_bound_.get() {
            debug_assert!(self.labels_.iter().all(|l| !l.has_referrer_jump() || l.is_bound()));
        }
        self.is_bound_.get()
    }

    pub fn is_empty(&self) -> bool {
        self.labels_.is_empty()
    }
}