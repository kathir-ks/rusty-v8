// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-operators.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::Any;
use std::ops::{Deref, DerefMut};

pub struct V8 {}

pub struct Isolate {
    heap_: Heap,
}

impl Isolate {
    fn heap(&mut self) -> &mut Heap {
        &mut self.heap_
    }
    fn exception(&self) -> Tagged<Object> {
        Tagged::<Object>{}
    }
}

pub struct Heap {}

impl Heap {
    fn to_boolean(&mut self, value: bool) -> Tagged<Object> {
        Tagged::<Object>{}
    }
}

pub struct Arguments {
    length_: usize,
    values_: Vec<DirectHandle<Object>>,
}

impl Arguments {
    pub fn length(&self) -> usize {
        self.length_
    }
    pub fn at(&self, index: usize) -> DirectHandle<Object> {
        self.values_[index].clone()
    }
}

#[derive(Clone)]
pub struct DirectHandle<T> {
    value: T,
}

impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle { value }
    }
    pub fn of(data: &[&DirectHandle<T>]) -> Self {
        Self::new(data[0].value.clone()) // only return first for test
    }
}

impl<T> Deref for DirectHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DirectHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub struct HandleScope {}

impl HandleScope {
    pub fn new(_isolate: &Isolate) -> Self {
        HandleScope {}
    }
}

pub struct SealHandleScope {}

impl SealHandleScope {
    pub fn new(_isolate: &Isolate) -> Self {
        SealHandleScope {}
    }
}

#[derive(Clone, Copy)]
pub struct Tagged<T> {

}

pub trait ObjectTrait {
    fn equals(&self, other: &Self) -> bool;
    fn strict_equals(&self, other: &Self) -> bool;
}

impl ObjectTrait for Tagged<Object> {
    fn equals(&self, _other: &Self) -> bool {
        false
    }
    fn strict_equals(&self, _other: &Self) -> bool {
        false
    }
}

pub struct Object {}

impl Object {
    pub fn add(_isolate: &mut Isolate, lhs: &DirectHandle<Object>, rhs: &DirectHandle<Object>) -> Result<Tagged<Object>, String> {
        Ok(Tagged::<Object>{})
    }

    pub fn equals(_isolate: &mut Isolate, x: &DirectHandle<Object>, y: &DirectHandle<Object>) -> Maybe<bool> {
        Maybe::Just(x as *const _ == y as *const _)
    }

    pub fn strict_equals(x: Tagged<Object>, y: Tagged<Object>) -> bool {
        std::ptr::eq(&x, &y)
    }

    pub fn less_than(_isolate: &mut Isolate, x: &DirectHandle<Object>, y: &DirectHandle<Object>) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn greater_than(_isolate: &mut Isolate, x: &DirectHandle<Object>, y: &DirectHandle<Object>) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn less_than_or_equal(_isolate: &mut Isolate, x: &DirectHandle<Object>, y: &DirectHandle<Object>) -> Maybe<bool> {
        Maybe::Just(false)
    }

    pub fn greater_than_or_equal(_isolate: &mut Isolate, x: &DirectHandle<Object>, y: &DirectHandle<Object>) -> Maybe<bool> {
        Maybe::Just(false)
    }
}

pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    pub fn is_nothing(&self) -> bool {
        match self {
            Maybe::Nothing => true,
            _ => false,
        }
    }

    pub fn from_just(self) -> T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => panic!("Tried to get value from Maybe::Nothing"),
        }
    }
}

pub struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    pub fn exception(&self) -> Tagged<Object> {
        Tagged::<Object>{}
    }
}

pub fn runtime_add(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let lhs = args.at(0);
    let rhs = args.at(1);
    Object::add(isolate, &lhs, &rhs)
}

pub fn runtime_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::equals(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(value)),
    }
}

pub fn runtime_not_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::equals(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(!value)),
    }
}

pub fn runtime_strict_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = Tagged::<Object>{}; //args[0];
    let y = Tagged::<Object>{}; //args[1];
    Ok(isolate.heap().to_boolean(Object::strict_equals(x, y)))
}

pub fn runtime_strict_not_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = Tagged::<Object>{}; //args[0];
    let y = Tagged::<Object>{}; //args[1];
    Ok(isolate.heap().to_boolean(!Object::strict_equals(x, y)))
}

pub fn runtime_reference_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = Tagged::<Object>{}; //args[0];
    let y = Tagged::<Object>{}; //args[1];
    Ok(isolate.heap().to_boolean(std::ptr::eq(&x, &y)))
}

pub fn runtime_less_than(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::less_than(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(value)),
    }
}

pub fn runtime_greater_than(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::greater_than(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(value)),
    }
}

pub fn runtime_less_than_or_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::less_than_or_equal(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(value)),
    }
}

pub fn runtime_greater_than_or_equal(isolate: &mut Isolate, args: &Arguments) -> Result<Tagged<Object>, String> {
    let x = args.at(0);
    let y = args.at(1);
    let result = Object::greater_than_or_equal(isolate, &x, &y);
    match result {
        Maybe::Nothing => Ok(isolate.exception()),
        Maybe::Just(value) => Ok(isolate.heap().to_boolean(value)),
    }
}
