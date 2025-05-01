// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/operation-typer.h
// Skipped creating a header file as the struct definition and impl block provide the same functionality in Rust

use std::{
    f64,
    f64::NAN,
    cmp::{min, max},
    assert_eq,
    // rc::Rc,
};

// use crate::compiler::common_operator::Operator; // Assuming this exists
// use crate::compiler::js_heap_broker::JSHeapBroker; // Assuming this exists
// use crate::compiler::turbofan_types::Type; // Assuming this exists
// use crate::compiler::type_cache::TypeCache; // Assuming this exists
// use crate::objects::oddball::Oddball; // Assuming this exists
// Assuming these exists in other modules, and are imported here

// Mock implementations for the dependencies
mod common_operator {
    pub struct Operator {}
}

mod js_heap_broker {
    pub struct JSHeapBroker {}

    impl JSHeapBroker {
        pub fn empty_string(&self) -> i32 { 0 }
        pub fn NaN_string(&self) -> i32 { 1 }
        pub fn zero_string(&self) -> i32 { 2 }
        pub fn false_value(&self) -> i32 { 3 }
        pub fn true_value(&self) -> i32 { 4 }
    }
}

mod turbofan_types {
    use std::{f64, ops::BitOr};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Type {
        min: f64,
        max: f64,
        is_nan: bool,
        is_minus_zero: bool,
    }

    impl Type {
        pub fn new(min: f64, max: f64, is_nan: bool, is_minus_zero: bool) -> Self {
            Type { min, max, is_nan, is_minus_zero }
        }
        pub fn constant(value: f64, _zone: &Zone) -> Self {
            Type::new(value, value, value.is_nan(), value == 0.0 && value.is_sign_negative())
        }

        pub fn constant_broker(_broker: &js_heap_broker::JSHeapBroker, _value: i32, _zone: &Zone) -> Self {
            Type::any() // Placeholder since we don't have the actual broker logic
        }

        pub fn minus_zero_or_nan() -> Self {
            Type { min: 0.0, max: 0.0, is_nan: true, is_minus_zero: true }
        }

        pub fn range(min: f64, max: f64, _zone: &Zone) -> Self {
            Type::new(min, max, false, false)
        }

        pub fn union(self, other: Type, _zone: &Zone) -> Self {
            Type {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
                is_nan: self.is_nan || other.is_nan,
                is_minus_zero: self.is_minus_zero || other.is_minus_zero,
            }
        }

        pub fn intersect(self, other: Type, _zone: &Zone) -> Self {
            Type {
                min: self.min.max(other.min),
                max: self.max.min(other.max),
                is_nan: self.is_nan && other.is_nan,
                is_minus_zero: self.is_minus_zero && other.is_minus_zero,
            }
        }

        pub fn is(&self, other: Type) -> bool {
            self.min >= other.min && self.max <= other.max &&
            (!other.is_nan || self.is_nan) &&
            (!other.is_minus_zero || self.is_minus_zero)
        }

        pub fn maybe(&self, other: Type) -> bool {
            self.min <= other.max && self.max >= other.min || other.is_nan || other.is_minus_zero
        }

        pub fn min(&self) -> f64 {
            self.min
        }

        pub fn max(&self) -> f64 {
            self.max
        }

        pub fn is_range(&self) -> bool {
            !self.is_nan && !self.is_minus_zero
        }

        pub fn is_integer(&self) -> bool {
            self.min.floor() == self.min && self.max.ceil() == self.max
        }

        pub fn is_none(&self) -> bool {
            self.min > self.max
        }

        pub fn is_singleton(&self) -> bool {
            self.min == self.max
        }

        pub fn is_unique(&self) -> bool {
            true // Placeholder.  The real implementation depends on heap object properties.
        }

        pub fn is_number(&self) -> bool {
            true // Placeholder
        }

        pub fn is_bigint(&self) -> bool {
            false // Placeholder
        }

        pub fn is_boolean(&self) -> bool {
            false
        }

        pub fn is_string(&self) -> bool {
            false
        }

        pub fn is_receiver(&self) -> bool {
            false
        }

        pub fn is_primitive(&self) -> bool {
            true
        }

        pub fn is_plain_primitive(&self) -> bool {
            true
        }

        pub fn is_ordered_number(&self) -> bool {
            true
        }

        pub fn is_machine(&self) -> bool {
            false
        }

        pub fn is_hole(&self) -> bool {
            false
        }

        pub fn is_non_internal(&self) -> bool {
            true
        }

        pub fn is_undetectable(&self) -> bool {
            false
        }

        pub fn any() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn number() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, false)
        }

        pub fn bigint() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn boolean() -> Self {
            Type::new(0.0, 1.0, false, false)
        }

        pub fn string() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn receiver() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn primitive() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn signed32() -> Self {
            Type::new(-2147483648.0, 2147483647.0, false, false)
        }

        pub fn unsigned32() -> Self {
            Type::new(0.0, 4294967295.0, false, false)
        }

        pub fn plain_number() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn machine() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn undetectable() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn null() -> Self {
            Type::new(0.0, 0.0, false, false) // Placeholder
        }

        pub fn undefined() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn symbol() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn non_bigint() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn non_number() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn string_wrapper_or_other_object() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn signed_bigint64() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn unsigned_bigint63() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn negative32() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn unsigned31() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false)
        }

        pub fn non_internal() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true)
        }

        pub fn hole() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn positive_safe_integer() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn additive_safe_integer() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn safe_integer_or_minus_zero() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

        pub fn uint8() -> Self {
            Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false) // Placeholder
        }

    }

    impl BitOr for Type {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            self.union(other, &Zone{})
        }
    }
}

mod type_cache {
    use crate::turbofan_types::Type;
    use super::Zone;

    pub struct TypeCache {
        pub k_zeroish: Type,
        pub k_singleton_zero: Type,
        pub k_singleton_one: Type,
        pub k_integer: Type,
        pub k_integer_or_minus_zero_or_nan: Type,
        pub k_zero_or_minus_zero: Type,
        pub k_minus_one_or_zero: Type,
        pub k_zero_or_one: Type,
        pub k_singleton_minus_one: Type,
        pub k_minus_zero: Type,
        pub k_zero_to_thirty_two: Type,
        pub k_uint8: Type,
        pub k_positive_safe_integer: Type,
        pub k_additive_safe_integer: Type,
        pub k_safe_integer_or_minus_zero: Type
    }

    impl TypeCache {
        pub fn get() -> Self {
            let zone = Zone::new();
            TypeCache {
                k_zeroish: Type::new(0.0, 0.0, true, true),
                k_singleton_zero: Type::constant(0.0, &zone),
                k_singleton_one: Type::constant(1.0, &zone),
                k_integer: Type::new(f64::NEG_INFINITY, f64::INFINITY, false, false), // Placeholder
                k_integer_or_minus_zero_or_nan: Type::new(f64::NEG_INFINITY, f64::INFINITY, true, true),
                k_zero_or_minus_zero: Type::new(0.0, 0.0, false, true),
                k_minus_one_or_zero: Type::new(-1.0, 0.0, false, false),
                k_zero_or_one: Type::new(0.0, 1.0, false, false),
                k_singleton_minus_one: Type::constant(-1.0, &zone),
                k_minus_zero: Type::new(0.0, 0.0, false, true),
                k_zero_to_thirty_two: Type::new(0.0, 32.0, false, false),
                k_uint8: Type::new(0.0, 255.0, false, false),
                k_positive_safe_integer: Type::new(0.0, 9007199254740991.0, false, false),
                k_additive_safe_integer: Type::new(-9007199254740991.0, 9007199254740991.0, false, false),
                k_safe_integer_or_minus_zero: Type::new(f64::NEG_INFINITY, f64::INFINITY, false, true),
            }
        }
    }
}

mod objects {
    pub mod oddball {
        pub struct Oddball {}
    }
}

#[derive(Debug)]
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

const V8_INFINITY: f64 = f64::INFINITY;
const K_MAX_INT: f64 = 2147483647.0;
const K_MIN_INT: f64 = -2147483648.0;
const K_MAX_UINT32: f64 = 4294967295.0;

pub struct OperationTyper {
    zone_: Zone,
    cache_: type_cache::TypeCache,
    infinity_: turbofan_types::Type,
    minus_infinity_: turbofan_types::Type,
    singleton_empty_string_: turbofan_types::Type,
    singleton_nan_string_: turbofan_types::Type,
    singleton_zero_string_: turbofan_types::Type,
    singleton_false_: turbofan_types::Type,
    singleton_true_: turbofan_types::Type,
    signed32ish_: turbofan_types::Type,
    unsigned32ish_: turbofan_types::Type,
    falsish_: turbofan_types::Type,
    truish_: turbofan_types::Type,
}

impl OperationTyper {
    pub fn new(broker: &js_heap_broker::JSHeapBroker, zone: Zone) -> Self {
        let cache_ = type_cache::TypeCache::get();
        let infinity_ = turbofan_types::Type::constant(V8_INFINITY, &zone);
        let minus_infinity_ = turbofan_types::Type::constant(-V8_INFINITY, &zone);
        let truncating_to_zero = turbofan_types::Type::minus_zero_or_nan();
        debug_assert!(!truncating_to_zero.maybe(turbofan_types::Type::signed32()));

        let singleton_empty_string_ =
            turbofan_types::Type::constant_broker(broker, broker.empty_string(), &zone);
        let singleton_nan_string_ = turbofan_types::Type::constant_broker(broker, broker.NaN_string(), &zone);
        let singleton_zero_string_ = turbofan_types::Type::constant_broker(broker, broker.zero_string(), &zone);
        let singleton_false_ = turbofan_types::Type::constant_broker(broker, broker.false_value(), &zone);
        let singleton_true_ = turbofan_types::Type::constant_broker(broker, broker.true_value(), &zone);
        let signed32ish_ = turbofan_types::Type::union(turbofan_types::Type::signed32(), truncating_to_zero, &zone);
        let unsigned32ish_ = turbofan_types::Type::union(turbofan_types::Type::unsigned32(), truncating_to_zero, &zone);

        let falsish_ = turbofan_types::Type::union(
            turbofan_types::Type::undetectable(),
            turbofan_types::Type::union(turbofan_types::Type::union(singleton_false_, cache_.k_zeroish, &zone),
                        singleton_empty_string_, &zone),
            &zone);
        let truish_ = turbofan_types::Type::union(
            singleton_true_,
            turbofan_types::Type::union(turbofan_types::Type::receiver(), turbofan_types::Type::symbol(), &zone), &zone);
        OperationTyper {
            zone_: zone,
            cache_: cache_,
            infinity_: infinity_,
            minus_infinity_: minus_infinity_,
            singleton_empty_string_: singleton_empty_string_,
            singleton_nan_string_: singleton_nan_string_,
            singleton_zero_string_: singleton_zero_string_,
            singleton_false_: singleton_false_,
            singleton_true_: singleton_true_,
            signed32ish_: signed32ish_,
            unsigned32ish_: unsigned32ish_,
            falsish_: falsish_,
            truish_: truish_,
        }
    }

    fn zone(&self) -> &Zone {
        &self.zone_
    }

    pub fn merge(&self, left: turbofan_types::Type, right: turbofan_types::Type) -> turbofan_types::Type {
        turbofan_types::Type::union(left, right, self.zone())
    }

    pub fn weaken_range(&self, previous_range: turbofan_types::Type, current_range: turbofan_types::Type) -> turbofan_types::Type {
        const WEAKEN_MIN_LIMITS: [f64; 22] = [0.0,
                                            -1073741824.0,
                                            -2147483648.0,
                                            -4294967296.0,
                                            -8589934592.0,
                                            -17179869184.0,
                                            -34359738368.0,
                                            -68719476736.0,
                                            -137438953472.0,
                                            -274877906944.0,
                                            -549755813888.0,
                                            -1099511627776.0,
                                            -2199023255552.0,
                                            -4398046511104.0,
                                            -8796093022208.0,
                                            -17592186044416.0,
                                            -35184372088832.0,
                                            -70368744177664.0,
                                            -140737488355328.0,
                                            -281474976710656.0,
                                            -562949953421312.0];
        const WEAKEN_MAX_LIMITS: [f64; 22] = [0.0,
                                            1073741823.0,
                                            2147483647.0,
                                            4294967295.0,
                                            8589934591.0,
                                            17179869183.0,
                                            34359738367.0,
                                            68719476735.0,
                                            137438953471.0,
                                            274877906943.0,
                                            549755813887.0,
                                            1099511627775.0,
                                            2199023255551.0,
                                            4398046511103.0,
                                            8796093022207.0,
                                            17592186044415.0,
                                            35184372088831.0,
                                            70368744177663.0,
                                            140737488355327.0,
                                            281474976710655.0,
                                            562949953421311.0];
        assert_eq!(WEAKEN_MIN_LIMITS.len(), WEAKEN_MAX_LIMITS.len());

        let current_min = current_range.min();
        let mut new_min = current_min;
        if current_min != previous_range.min() {
            new_min = -V8_INFINITY;
            for &min_limit in &WEAKEN_MIN_LIMITS {
                if min_limit <= current_min {
                    new_min = min_limit;
                    break;
                }
            }
        }

        let current_max = current_range.max();
        let mut new_max = current_max;
        if current_max != previous_range.max() {
            new_max = V8_INFINITY;
            for &max_limit in &WEAKEN_MAX_LIMITS {
                if max_limit >= current_max {
                    new_max = max_limit;
                    break;
                }
            }
        }

        turbofan_types::Type::range(new_min, new_max, self.zone())
    }

    pub fn rangify(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.is_range() { return type_; }
        if !type_.is_integer() {
            return type_;
        }
        turbofan_types::Type::range(type_.min(), type_.max(), self.zone())
    }

    fn array_min(&self, a: &[f64]) -> f64 {
        debug_assert!(!a.is_empty());
        let mut x = V8_INFINITY;
        for &val in a {
            if !val.is_nan() {
                x = min(val, x);
            }
        }
        debug_assert!(!x.is_nan());
        if x == 0.0 { 0.0 } else { x }
    }

    fn array_max(&self, a: &[f64]) -> f64 {
        debug_assert!(!a.is_empty());
        let mut x = -V8_INFINITY;
        for &val in a {
            if !val.is_nan() {
                x = max(val, x);
            }
        }
        debug_assert!(!x.is_nan());
        if x == 0.0 { 0.0 } else { x }
    }

    pub fn add_ranger(&self, lhs_min: f64, lhs_max: f64, rhs_min: f64, rhs_max: f64) -> turbofan_types::Type {
        let mut results = [0.0; 4];
        results[0] = lhs_min + rhs_min;
        results[1] = lhs_min + rhs_max;
        results[2] = lhs_max + rhs_min;
        results[3] = lhs_max + rhs_max;

        let mut nans = 0;
        for i in 0..4 {
            if results[i].is_nan() { nans += 1; }
        }

        if nans == 4 { return turbofan_types::Type::constant(NAN, &self.zone_) }
        let mut type_ = turbofan_types::Type::range(self.array_min(&results), self.array_max(&results), self.zone());
        if nans > 0 { type_ = turbofan_types::Type::union(type_, turbofan_types::Type::constant(NAN, &self.zone_), self.zone()); }

        type_
    }

    pub fn subtract_ranger(&self, lhs_min: f64, lhs_max: f64, rhs_min: f64, rhs_max: f64) -> turbofan_types::Type {
        let mut results = [0.0; 4];
        results[0] = lhs_min - rhs_min;
        results[1] = lhs_min - rhs_max;
        results[2] = lhs_max - rhs_min;
        results[3] = lhs_max - rhs_max;

        let mut nans = 0;
        for i in 0..4 {
            if results[i].is_nan() { nans += 1; }
        }

        if nans == 4 { return turbofan_types::Type::constant(NAN, &self.zone_) }
        let type_ = turbofan_types::Type::range(self.array_min(&results), self.array_max(&results), self.zone());
        if nans == 0 { type_ } else { turbofan_types::Type::union(type_, turbofan_types::Type::constant(NAN, &self.zone_), self.zone()) }
    }

    pub fn multiply_ranger(&self, lhs_min: f64, lhs_max: f64, rhs_min: f64, rhs_max: f64) -> turbofan_types::Type {
        let mut results = [0.0; 4];
        results[0] = lhs_min * rhs_min;
        results[1] = lhs_min * rhs_max;
        results[2] = lhs_max * rhs_min;
        results[3] = lhs_max * rhs_max;

        for i in 0..4 {
            if results[i].is_nan() {
                return self.cache_.k_integer_or_minus_zero_or_nan;
            }
        }
        let min_val = self.array_min(&results);
        let max_val = self.array_max(&results);
        let mut type_ = turbofan_types::Type::range(min_val, max_val, self.zone());
        if min_val <= 0.0 && 0.0 <= max_val && (lhs_min < 0.0 || rhs_min < 0.0) {
            type_ = turbofan_types::Type::union(type_, self.cache_.k_minus_zero, self.zone());
        }
        if ((lhs_min == -V8_INFINITY || lhs_max == V8_INFINITY) &&
            (rhs_min <= 0.0 && 0.0 <= rhs_max)) ||
           ((rhs_min == -V8_INFINITY || rhs_max == V8_INFINITY) &&
            (lhs_min <= 0.0 && 0.0 <= lhs_max)) {
            type_ = turbofan_types::Type::union(type_, turbofan_types::Type::constant(NAN, &self.zone_), self.zone());
        }
        type_
    }

    pub fn convert_receiver(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.is_receiver() { return type_; }
        let maybe_primitive = type_.maybe(turbofan_types::Type::primitive());
        let mut type_ = turbofan_types::Type::intersect(type_, turbofan_types::Type::receiver(), self.zone());
        if maybe_primitive {
            type_ = turbofan_types::Type::union(type_, turbofan_types::Type::string_wrapper_or_other_object(), self.zone());
        }
        type_
    }

    pub fn to_number(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.is_number() { return type_; }

        if type_.maybe(turbofan_types::Type::string() | turbofan_types::Type::receiver()) {
            return turbofan_types::Type::number();
        }

        let mut type_ = turbofan_types::Type::intersect(type_, turbofan_types::Type::plain_primitive(), self.zone());

        debug_assert!(type_.is(turbofan_types::Type::number()));

        if type_.maybe(turbofan_types::Type::null()) {
            type_ = turbofan_types::Type::union(type_, self.cache_.k_singleton_zero, self.zone());
        }
        if type_.maybe(turbofan_types::Type::undefined()) {
            type_ = turbofan_types::Type::union(type_, turbofan_types::Type::constant(NAN, &self.zone_), self.zone());
        }
        if type_.maybe(self.singleton_false_ ) {
            type_ = turbofan_types::Type::union(type_, self.cache_.k_singleton_zero, self.zone());
        }
        if type_.maybe(self.singleton_true_ ) {
            type_ = turbofan_types::Type::union(type_, self.cache_.k_singleton_one, self.zone());
        }
        turbofan_types::Type::intersect(type_, turbofan_types::Type::number(), self.zone())
    }

    pub fn to_number_convert_big_int(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        let maybe_bigint = type_.maybe(turbofan_types::Type::bigint()) || type_.maybe(turbofan_types::Type::receiver());
        let mut type_ = self.to_number(turbofan_types::Type::intersect(type_, turbofan_types::Type::non_bigint(), self.zone()));
        if maybe_bigint {
            type_ = turbofan_types::Type::union(type_, self.cache_.k_integer, self.zone());
        }
        type_
    }

    pub fn to_big_int(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.is_bigint() {
            return type_;
        }

        turbofan_types::Type::bigint()
    }

    pub fn to_big_int_convert_number(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.is(turbofan_types::Type::unsigned32())) {
            return turbofan_types::Type::unsigned_bigint63();
        } else if type_.is(turbofan_types::Type::signed32()) {
            return turbofan_types::Type::signed_bigint64();
        }

        let maybe_number =
            type_.maybe(turbofan_types::Type::number()) || type_.maybe(turbofan_types::Type::receiver());
        let mut type_ = self.to_big_int(turbofan_types::Type::intersect(type_, turbofan_types::Type::non_number(), self.zone()));
        if maybe_number {
            type_ = turbofan_types::Type::union(type_, turbofan_types::Type::bigint(), self.zone());
        }
        type_
    }

    pub fn to_numeric(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        if type_.maybe(turbofan_types::Type::receiver())) {
            type_ = turbofan_types::Type::union(type_, turbofan_types::Type::bigint(), self.zone());
        }
        turbofan_types::Type::union(self.to_number(turbofan_types::Type::intersect(type_, turbofan_types::Type::non_bigint(), self.zone())),
                             turbofan_types::Type::intersect(type_, turbofan_types::Type::bigint(), self.zone()), self.zone())
    }

    pub fn number_abs(&self, type_: turbofan_types::Type) -> turbofan_types::Type {
        debug_assert!(type_.is_number());
        if type_.is_none() { return type_; }

        let maybe_nan = type_.maybe(turbofan_types::Type::constant(NAN, &self.zone_));
        let maybe_minuszero = type_.maybe(self.cache_.k_minus_zero);

        let mut type_ = turbofan_types::Type::intersect(type_, turbofan_types::Type::plain_number(), self.zone());
        if !type_.is_none() {
            let max_val = type_.max();
            let min_val = type_.min();
            if min_val < 0.0 {
                if type_.is(self.cache_.k_integer) {
                    type_ =
                        turbofan_types::Type::range(0.0, f64::max(min_val.abs(), max_val.abs()), self.zone());
                } else {
                    type_ = turbofan_types::Type::plain_number();