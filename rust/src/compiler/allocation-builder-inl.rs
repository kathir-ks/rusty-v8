// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/allocation-builder-inl.h

mod access_builder;
mod allocation_builder;
mod heap;
mod objects;
mod common;
mod simplified;
mod jsgraph;

use access_builder::*;
use allocation_builder::*;
use heap::*;
use objects::*;
use common::*;
use simplified::*;
use jsgraph::*;

//use std::mem::size_of;

impl AllocationBuilder {
    pub fn allocate(&mut self, size: i32, allocation: AllocationType, r#type: Type) {
        if size <= 0 {
            panic!("size must be greater than 0");
        }

        if size > self.isolate.heap.max_regular_heap_object_size(allocation) {
            panic!("size exceeds maximum regular heap object size");
        }

        self.effect = self.graph.new_node(
            self.common.begin_region(RegionObservability::kNotObservable),
            self.effect,
        );

        self.allocation = self.graph.new_node(
            self.simplified.allocate(r#type, allocation),
            self.jsgraph.constant_no_hole(size),
            self.effect,
            self.control,
        );

        self.effect = self.allocation;
    }

    pub fn allocate_context(&mut self, variadic_part_length: i32, map: MapRef) {
        if !(FIRST_CONTEXT_TYPE..=LAST_CONTEXT_TYPE).contains(&map.instance_type) {
            panic!("map.instance_type() is not a context type");
        }

        if map.instance_type == NATIVE_CONTEXT_TYPE {
            panic!("map.instance_type() cannot be NATIVE_CONTEXT_TYPE");
        }

        let size = Context::size_for(variadic_part_length);
        self.allocate(size, AllocationType::kYoung, Type::OtherInternal());
        self.store(AccessBuilder::ForMap(), map);
        // static_assert!(Context::kLengthOffset as i32 == offsetof(FixedArray, length_) as i32); // TODO: Implement offsetof
        self.store(
            AccessBuilder::ForFixedArrayLength(),
            self.jsgraph.constant_no_hole(variadic_part_length),
        );
    }

    pub fn can_allocate_array(&self, length: i32, map: MapRef, allocation: AllocationType) -> bool {
        if map.instance_type != FIXED_ARRAY_TYPE && map.instance_type != FIXED_DOUBLE_ARRAY_TYPE {
            panic!("map.instance_type() is not a fixed array type");
        }

        let size = if map.instance_type == FIXED_ARRAY_TYPE {
            FixedArray::size_for(length)
        } else {
            FixedDoubleArray::size_for(length)
        };

        size <= self.isolate.heap.max_regular_heap_object_size(allocation)
    }

    pub fn allocate_array(&mut self, length: i32, map: MapRef, allocation: AllocationType) {
        if !self.can_allocate_array(length, map, allocation) {
            panic!("Cannot allocate array");
        }

        let size = if map.instance_type == FIXED_ARRAY_TYPE {
            FixedArray::size_for(length)
        } else {
            FixedDoubleArray::size_for(length)
        };

        self.allocate(size, allocation, Type::OtherInternal());
        self.store(AccessBuilder::ForMap(), map);
        self.store(
            AccessBuilder::ForFixedArrayLength(),
            self.jsgraph.constant_no_hole(length),
        );
    }

    pub fn can_allocate_sloppy_argument_elements(
        &self,
        length: i32,
        map: MapRef,
        allocation: AllocationType,
    ) -> bool {
        let size = SloppyArgumentsElements::size_for(length);
        size <= self.isolate.heap.max_regular_heap_object_size(allocation)
    }

    pub fn allocate_sloppy_argument_elements(
        &mut self,
        length: i32,
        map: MapRef,
        allocation: AllocationType,
    ) {
        if !self.can_allocate_sloppy_argument_elements(length, map, allocation) {
            panic!("Cannot allocate sloppy argument elements");
        }

        let size = SloppyArgumentsElements::size_for(length);
        self.allocate(size, allocation, Type::OtherInternal());
        self.store(AccessBuilder::ForMap(), map);
        self.store(
            AccessBuilder::ForFixedArrayLength(),
            self.jsgraph.constant_no_hole(length),
        );
    }
}