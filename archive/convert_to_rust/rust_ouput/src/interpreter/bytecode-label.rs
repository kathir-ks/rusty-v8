// Converted from V8 C++ source files:
// Header: bytecode-label.h
// Implementation: bytecode-label.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct BytecodeArrayBuilder {}

#[derive(Debug, PartialEq)]
pub enum BytecodeLabelError {
    AlreadyBound,
    InvalidOffset,
    ReferrerAlreadySet,
}

// A label representing a loop header in a bytecode array. It is bound before
// the jump is seen, so its position is always known by the time the jump is
// reached.
#[derive(Debug)]
pub struct BytecodeLoopHeader {
    offset_: Option<usize>,
}

impl BytecodeLoopHeader {
    pub fn new() -> Self {
        BytecodeLoopHeader { offset_: None }
    }

    pub fn offset(&self) -> Result<usize, BytecodeLabelError> {
        self.offset_.ok_or(BytecodeLabelError::InvalidOffset)
    }

    fn bind_to(&mut self, offset: usize) -> Result<(), BytecodeLabelError> {
        if self.offset_.is_some() {
            return Err(BytecodeLabelError::AlreadyBound);
        }
        self.offset_ = Some(offset);
        Ok(())
    }
}

// A label representing a forward branch target in a bytecode array. When a
// label is bound, it represents a known position in the bytecode array. A label
// can only have at most one referrer jump.
#[derive(Debug)]
pub struct BytecodeLabel {
    bound_: bool,
    jump_offset_: Option<usize>,
}

impl BytecodeLabel {
    pub fn new() -> Self {
        BytecodeLabel {
            bound_: false,
            jump_offset_: None,
        }
    }

    pub fn is_bound(&self) -> bool {
        self.bound_
    }

    pub fn jump_offset(&self) -> Result<usize, BytecodeLabelError> {
        self.jump_offset_.ok_or(BytecodeLabelError::InvalidOffset)
    }

    pub fn has_referrer_jump(&self) -> bool {
        self.jump_offset_.is_some()
    }

    fn bind(&mut self) -> Result<(), BytecodeLabelError> {
        if self.bound_ {
            return Err(BytecodeLabelError::AlreadyBound);
        }
        self.bound_ = true;
        Ok(())
    }

    fn set_referrer(&mut self, offset: usize) -> Result<(), BytecodeLabelError> {
        if self.bound_ {
            return Err(BytecodeLabelError::AlreadyBound);
        }
        if self.jump_offset_.is_some() {
            return Err(BytecodeLabelError::ReferrerAlreadySet);
        }
        self.jump_offset_ = Some(offset);
        Ok(())
    }
}

use std::cell::RefCell;
use std::rc::Rc;

// Class representing a branch target of multiple jumps.
pub struct BytecodeLabels {
    labels_: Vec<Rc<RefCell<BytecodeLabel>>>,
    is_bound_: bool,
}

impl BytecodeLabels {
    pub fn new() -> Self {
        BytecodeLabels {
            labels_: Vec::new(),
            is_bound_: false,
        }
    }

    pub fn create_label(&mut self) -> Rc<RefCell<BytecodeLabel>> {
        if self.is_bound_ {
            panic!("Cannot create new labels after binding.");
        }
        let label = Rc::new(RefCell::new(BytecodeLabel::new()));
        self.labels_.push(label.clone());
        label
    }

    pub fn bind(&mut self, builder: &mut BytecodeArrayBuilder) {
        if self.is_bound_ {
            return;
        }

        self.is_bound_ = true;
        for label in &self.labels_ {
            // Assuming BytecodeArrayBuilder has a bind method
            bind_label(builder, label);
        }
    }

    pub fn is_bound(&self) -> bool {
        if self.is_bound_ {
            for label in &self.labels_ {
                let label_ref = label.borrow();
                if label_ref.has_referrer_jump() && !label_ref.is_bound() {
                    return false;
                }
            }
        }
        self.is_bound_
    }

    pub fn is_empty(&self) -> bool {
        self.labels_.is_empty()
    }
}

fn bind_label(_builder: &mut BytecodeArrayBuilder, label: &Rc<RefCell<BytecodeLabel>>) {
    let mut mutable_label = label.borrow_mut();
    mutable_label.bind().unwrap();
}
