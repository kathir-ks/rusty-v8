// Converted from V8 C++ source files:
// Header: deoptimized-frame-info.h
// Implementation: deoptimized-frame-info.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/deoptimizer/deoptimized-frame-info.h
use std::rc::Rc;
use std::cell::RefCell;

//use crate::deoptimizer::translated_state::TranslatedState;
//use crate::deoptimizer::translated_state::TranslatedStateIterator;
//use crate::execution::isolate::Isolate;

pub struct DeoptimizedFrameInfo {
  context_: Handle<Object>,
  parameters_: Vec<Handle<Object>>,
  expression_stack_: Vec<Handle<Object>>,
}

impl DeoptimizedFrameInfo {
  pub fn new(state: &mut TranslatedState, frame_it: TranslatedStateIterator, isolate: &mut Isolate) -> Self {
    let parameter_count = frame_it.shared_info().internal_formal_parameter_count_without_receiver();
    let mut stack_it = frame_it.begin();

    // Get the function. Note that this might materialize the function.
    // In case the debugger mutates this value, we should deoptimize
    // the function and remember the value in the materialized value store.
    assert_eq!(parameter_count,
              stack_it.value().shared().internal_formal_parameter_count_without_receiver());

    stack_it.next(); // Skip the function.
    stack_it.next(); // Skip the receiver.

    //DCHECK_EQ(TranslatedFrame::kUnoptimizedFunction, frame_it.kind());

    let mut parameters_: Vec<Handle<Object>> = Vec::new();
    for i in 0..parameter_count {
      let parameter = get_value_for_debugger(stack_it.clone(), isolate);
      parameters_.push(parameter);
      stack_it.next();
    }

    // Get the context.
    let context_ = get_value_for_debugger(stack_it.clone(), isolate);
    stack_it.next();

    // Get the expression stack.
    //DCHECK_EQ(TranslatedFrame::kUnoptimizedFunction, frame_it.kind());
    let stack_height = frame_it.height() as usize;  // Accumulator *not* included.

    let mut expression_stack_: Vec<Handle<Object>> = Vec::new();
    for i in 0..stack_height {
      let expression = get_value_for_debugger(stack_it.clone(), isolate);
      expression_stack_.push(expression);
      stack_it.next();
    }

    //DCHECK_EQ(TranslatedFrame::kUnoptimizedFunction, frame_it.kind());
    stack_it.next();  // Skip the accumulator.

    //CHECK(stack_it == frame_it.end());
    Self {
      context_: context_,
      parameters_: parameters_,
      expression_stack_: expression_stack_,
    }
  }

  pub fn get_context(&self) -> Handle<Object> {
    self.context_.clone()
  }

  pub fn get_parameter(&self, index: usize) -> Handle<Object> {
    assert!(0 <= index as i32 && index < self.parameters_count() as i32);
    self.parameters_[index].clone()
  }

  pub fn get_expression(&self, index: usize) -> Handle<Object> {
    assert!(0 <= index as i32 && index < self.expression_count() as i32);
    self.expression_stack_[index].clone()
  }

  fn parameters_count(&self) -> usize {
    self.parameters_.len()
  }

  fn expression_count(&self) -> usize {
    self.expression_stack_.len()
  }

  fn set_parameter(&mut self, index: usize, obj: Handle<Object>) {
    assert!(0 <= index as i32 && index < self.parameters_count() as i32);
    self.parameters_[index] = obj;
  }

  fn set_expression(&mut self, index: usize, obj: Handle<Object>) {
    assert!(0 <= index as i32 && index < self.expression_count() as i32);
    self.expression_stack_[index] = obj;
  }
}

fn get_value_for_debugger(it: TranslatedStateIterator, isolate: &mut Isolate) -> Handle<Object> {
  if it.get_raw_value() == isolate.read_only_roots().arguments_marker() &&
     !it.is_materializable_by_debugger() {
    return isolate.factory().optimized_out();
  }
  return it.get_value();
}

// Dummy structs and enums
#[derive(Clone, Debug)]
pub struct Handle<T> {
  value: Rc<T>,
}
impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { value: Rc::new(value) }
    }
}

pub struct TranslatedState {}
impl TranslatedState {
    pub fn new() -> Self {
        TranslatedState {}
    }
}

#[derive(Clone, Debug)]
pub struct TranslatedStateIterator {
    index: i32,
}
impl TranslatedStateIterator {
    pub fn new() -> Self {
        TranslatedStateIterator { index: 0 }
    }
    pub fn next(&mut self){
        self.index += 1;
    }
    pub fn shared_info(&self) -> SharedInfo {
      SharedInfo {}
    }
    pub fn value(&self) -> JsFunction {
      JsFunction {}
    }
    pub fn begin(&self) -> TranslatedStateIterator {
      TranslatedStateIterator {}
    }
    pub fn get_raw_value(&self) -> i32 {
      0
    }
    pub fn is_materializable_by_debugger(&self) -> bool {
      false
    }
    pub fn get_value(&self) -> Handle<Object> {
      Handle::new(Object{})
    }
    pub fn height(&self) -> u32 {
        0
    }
    pub fn end(&self) -> TranslatedStateIterator {
      TranslatedStateIterator {}
    }
}

pub struct Isolate {
  factory_: Factory,
  read_only_roots_: ReadOnlyRoots,
}

impl Isolate {
    pub fn new() -> Self {
        Isolate { factory_: Factory{}, read_only_roots_: ReadOnlyRoots{} }
    }
    pub fn factory(&mut self) -> &mut Factory {
        &mut self.factory_
    }
    pub fn read_only_roots(&self) -> &ReadOnlyRoots {
        &self.read_only_roots_
    }
}

pub struct Factory {}
impl Factory {
    pub fn optimized_out(&mut self) -> Handle<Object> {
        Handle::new(Object{})
    }
}

pub struct ReadOnlyRoots {}
impl ReadOnlyRoots {
    pub fn arguments_marker(&self) -> i32 {
        0
    }
}

#[derive(Clone, Debug)]
pub struct Object {}
#[derive(Clone, Debug)]
pub struct SharedInfo {
    internal_formal_parameter_count_without_receiver: i32,
}
impl SharedInfo {
    pub fn internal_formal_parameter_count_without_receiver(&self) -> i32 {
        self.internal_formal_parameter_count_without_receiver
    }
}

#[derive(Clone, Debug)]
pub struct JsFunction {}
impl JsFunction {
    pub fn shared(&self) -> SharedInfo {
        SharedInfo {
            internal_formal_parameter_count_without_receiver: 0,
        }
    }
}
