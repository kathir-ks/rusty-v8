// Converted from V8 C++ source files:
// Header: api-callbacks.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::any::Any;

use crate::v8::internal::SideEffectType;
use crate::PropertyAttributes;

pub struct AccessorInfo {
    maybe_redirected_getter: usize,
    setter: usize,
    replace_on_access: bool,
    is_sloppy: bool,
    getter_side_effect_type: SideEffectType,
    setter_side_effect_type: SideEffectType,
    initial_property_attributes: PropertyAttributes,
    flags: u32, 
}

impl AccessorInfo {
    pub fn getter(&self) -> usize {
        self.maybe_redirected_getter
    }

    pub fn set_getter(&mut self, value: usize) {
        self.maybe_redirected_getter = value;
    }

    pub fn init_getter_redirection(&mut self, _isolate: IsolateForSandbox) {
        // Implementation detail, can be left empty or provide a default behavior
    }

    pub fn remove_getter_redirection(&mut self, _isolate: IsolateForSandbox) {
        // Implementation detail, can be left empty or provide a default behavior
    }

    pub fn has_getter(&self, _isolate: *mut Isolate) -> bool {
        self.maybe_redirected_getter != 0
    }

    pub fn setter(&self) -> usize {
        self.setter
    }

    pub fn set_setter(&mut self, value: usize) {
        self.setter = value;
    }

    pub fn has_setter(&self, _isolate: *mut Isolate) -> bool {
        self.setter != 0
    }

    pub fn replace_on_access(&self) -> bool {
        self.replace_on_access
    }

    pub fn set_replace_on_access(&mut self, value: bool) {
        self.replace_on_access = value;
    }

    pub fn is_sloppy(&self) -> bool {
        self.is_sloppy
    }

    pub fn set_is_sloppy(&mut self, value: bool) {
        self.is_sloppy = value;
    }

    pub fn getter_side_effect_type(&self) -> SideEffectType {
        self.getter_side_effect_type
    }

    pub fn set_getter_side_effect_type(&mut self, type_: SideEffectType) {
        self.getter_side_effect_type = type_;
    }

    pub fn setter_side_effect_type(&self) -> SideEffectType {
        self.setter_side_effect_type
    }

    pub fn set_setter_side_effect_type(&mut self, type_: SideEffectType) {
        self.setter_side_effect_type = type_;
    }

    pub fn initial_property_attributes(&self) -> PropertyAttributes {
        self.initial_property_attributes
    }

    pub fn set_initial_property_attributes(&mut self, attributes: PropertyAttributes) {
        self.initial_property_attributes = attributes;
    }

    pub fn is_compatible_receiver_map(
        _info: &AccessorInfo,
        _map: &Map,
    ) -> bool {
        true
    }

    pub fn is_compatible_receiver(&self, _receiver: Object) -> bool {
        true
    }

    pub fn append_unique(
        _isolate: *mut Isolate,
        _descriptors: Object,
        _array: FixedArray,
        valid_descriptors: i32,
    ) -> i32 {
        valid_descriptors
    }

    pub fn clear_padding(&mut self) {}
}

impl std::fmt::Debug for AccessorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessorInfo")
            .field("getter", &self.getter())
            .field("setter", &self.setter())
            .field("replace_on_access", &self.replace_on_access())
            .field("is_sloppy", &self.is_sloppy())
            .field("getter_side_effect_type", &self.getter_side_effect_type())
            .field("setter_side_effect_type", &self.setter_side_effect_type())
            .field("initial_property_attributes", &self.initial_property_attributes())
            .finish()
    }
}

pub struct AccessCheckInfo {
    dummy: i32,
}

impl AccessCheckInfo {
    pub fn get(_isolate: *mut Isolate, _receiver: JSObject) -> Self {
        AccessCheckInfo { dummy: 0 }
    }
}

pub struct InterceptorInfo {
    can_intercept_symbols: bool,
    non_masking: bool,
    is_named: bool,
    has_no_side_effect: bool,
    has_new_callbacks_signature: bool,
    flags: u32,
}

impl InterceptorInfo {
    pub fn can_intercept_symbols(&self) -> bool {
        self.can_intercept_symbols
    }

    pub fn set_can_intercept_symbols(&mut self, value: bool) {
        self.can_intercept_symbols = value;
    }

    pub fn non_masking(&self) -> bool {
        self.non_masking
    }

    pub fn set_non_masking(&mut self, value: bool) {
        self.non_masking = value;
    }

    pub fn is_named(&self) -> bool {
        self.is_named
    }

    pub fn set_is_named(&mut self, value: bool) {
        self.is_named = value;
    }

    pub fn has_no_side_effect(&self) -> bool {
        self.has_no_side_effect
    }

    pub fn set_has_no_side_effect(&mut self, value: bool) {
        self.has_no_side_effect = value;
    }

    pub fn has_new_callbacks_signature(&self) -> bool {
        self.has_new_callbacks_signature
    }

    pub fn set_has_new_callbacks_signature(&mut self, value: bool) {
        self.has_new_callbacks_signature = value;
    }
}

pub struct IsolateForSandbox {}
pub struct Isolate {}
pub struct Map {}
pub struct JSObject {}
pub struct Object {}
pub struct FixedArray {}
