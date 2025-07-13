// Converted from V8 C++ source files:
// Header: parameter-difference.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::option::Option;
use std::vec::Vec;

//use crate::torque::types::Type; // Assuming this is where Type is defined
use crate::torque::types::*;

#[derive(Debug, PartialEq)]
pub enum ParameterDifferenceError {
    SizeMismatch,
    Unreachable,
}

pub struct ParameterDifference {
    difference_: Vec<Option<Type>>,
}

impl ParameterDifference {
    pub fn new(to: &TypeVector, from: &TypeVector) -> Self {
        assert_eq!(to.len(), from.len());
        let mut parameter_difference = ParameterDifference {
            difference_: Vec::new(),
        };
        for i in 0..to.len() {
            parameter_difference.add_parameter(&to[i], &from[i]);
        }
        parameter_difference
    }

    pub fn strictly_better_than(&self, other: &ParameterDifference) -> bool {
        assert_eq!(self.difference_.len(), other.difference_.len());
        let mut better_parameter_found = false;
        for i in 0..self.difference_.len() {
            let a = &self.difference_[i];
            let b = &other.difference_[i];
            if a == b {
                continue;
            } else if let (Some(a_type), Some(b_type)) = (a, b) {
                if a_type.is_subtype_of(b_type) {
                    assert!(!b_type.is_subtype_of(a_type));
                    better_parameter_found = true;
                } else {
                  return false;
                }
            } else if a.is_some() && b.is_none() {
                better_parameter_found = true;
            } else {
                return false;
            }
        }
        better_parameter_found
    }

    fn add_parameter(&mut self, to: &Type, from: &Type) {
        if from.is_subtype_of(to) {
            self.difference_.push(Some(to.clone()));
        } else if is_assignable_from(to, from) {
            self.difference_.push(None);
        } else {
          panic!("Unreachable");
        }
    }
}
