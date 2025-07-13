// Converted from V8 C++ source files:
// Header: struct-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

use crate::heap::heap_write_barrier_inl::HeapWriteBarrier;
use crate::objects::objects_inl::*;
use crate::objects::oddball::Oddball;
use crate::roots::roots_inl::*;
use crate::V8;

//use crate::torque_generated::src::objects::struct_tq_inl::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessorComponent {
    ACCESSOR_GETTER,
    ACCESSOR_SETTER,
}

pub struct Struct {
    dummy: i32,
    phantom: std::marker::PhantomData<*const ()>,
}

impl Struct {
    pub fn cast(obj: Tagged<Object>) -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}

pub struct Tuple2 {
    dummy: i32,
    phantom: std::marker::PhantomData<*const ()>,
}

impl Tuple2 {
    pub fn cast(obj: Tagged<Object>) -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}

pub struct AccessorPair {
    getter: Tagged<Object>,
    setter: Tagged<Object>,
}

impl AccessorPair {
    pub fn cast(obj: Tagged<Object>) -> Self {
        Self {
            getter: Tagged {
                _object: obj._object,
            },
            setter: Tagged {
                _object: obj._object,
            },
        }
    }
}

impl AccessorPair {
    pub fn new() -> Self {
        AccessorPair {
            getter: Tagged { _object: 0 },
            setter: Tagged { _object: 0 },
        }
    }

    pub fn get(&self, component: AccessorComponent) -> Tagged<Object> {
        match component {
            AccessorComponent::ACCESSOR_GETTER => self.getter,
            AccessorComponent::ACCESSOR_SETTER => self.setter,
        }
    }

    pub fn set(&mut self, component: AccessorComponent, value: Tagged<Object>) {
        match component {
            AccessorComponent::ACCESSOR_GETTER => self.set_getter(value),
            AccessorComponent::ACCESSOR_SETTER => self.set_setter(value),
        }
    }

    pub fn set(&mut self, component: AccessorComponent, value: Tagged<Object>, tag: ReleaseStoreTag) {
        match component {
            AccessorComponent::ACCESSOR_GETTER => self.set_getter(value),
            AccessorComponent::ACCESSOR_SETTER => self.set_setter(value),
        }
    }

    pub fn getter(&self) -> Tagged<Object> {
        self.getter
    }

    pub fn setter(&self) -> Tagged<Object> {
        self.setter
    }

    pub fn set_getter(&mut self, value: Tagged<Object>) {
        self.getter = value;
    }

    pub fn set_setter(&mut self, value: Tagged<Object>) {
        self.setter = value;
    }

    pub fn SetComponents(&mut self, getter: Tagged<Object>, setter: Tagged<Object>) {
        if !self.IsNull(getter) {
            self.set_getter(getter);
        }
        if !self.IsNull(setter) {
            self.set_setter(setter);
        }
    }

    fn IsNull(&self, obj: Tagged<Object>) -> bool {
        obj._object == 0
    }

    pub fn Equals(&self, getter_value: Tagged<Object>, setter_value: Tagged<Object>) -> bool {
        (self.getter() == getter_value) && (self.setter() == setter_value)
    }
}

pub struct ClassPositions {
    dummy: i32,
    phantom: std::marker::PhantomData<*const ()>,
}

impl ClassPositions {
    pub fn cast(obj: Tagged<Object>) -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}
