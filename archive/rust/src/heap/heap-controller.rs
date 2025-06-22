// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/heap-controller.rs

use std::cmp::{max, min};
use std::marker::PhantomData;
use std::optional::Option;

// Placeholder for Isolate, Spaces, PageMetadata, TraceEvent, and v8_flags
// Since we don't have the definitions for these, we'll use simple placeholders.

// pub struct Isolate {}

// impl Isolate {
//     pub fn from_heap(_heap: &Heap) -> &mut Self {
//         todo!()
//     }
//     pub fn print_with_timestamp(&mut self, _format: &str, _args: ...) {
//         // Implementation for printing with timestamp, using format! and println!
//         // Placeholder for varargs
//         todo!()
//     }
// }

// pub struct Heap {}

// impl Heap {
//     pub enum HeapGrowingMode {
//         kConservative,
//         kSlow,
//         kMinimal,
//         kDefault,
//     }
// }

// pub mod v8_flags {
//     pub static heap_growing_percent: f64 = 0.0;
//     pub static trace_gc_verbose: bool = false;
// }

// pub mod PageMetadata {
//   pub const kPageSize: usize = 4096; // Example value, replace with actual
// }

const KB: usize = 1024;
const MB: usize = 1024 * KB;

pub trait MemoryControllerTrait {
    const K_NAME: &'static str;
    const K_MIN_GROWING_FACTOR: f64;
    const K_MAX_GROWING_FACTOR: f64;
    const K_CONSERVATIVE_GROWING_FACTOR: f64;
    const K_MIN_SIZE: usize;
    const K_MAX_SIZE: usize;
    const K_TARGET_MUTATOR_UTILIZATION: f64;
}

pub struct MemoryController<Trait: MemoryControllerTrait> {
    _trait: PhantomData<Trait>,
}

impl<Trait: MemoryControllerTrait> MemoryController<Trait> {
    /// Calculates the growing factor for the heap.
    pub fn growing_factor(
        _heap: &(), //Heap
        max_heap_size: usize,
        gc_speed: Option<f64>,
        mutator_speed: f64,
        growing_mode: i32, //Heap::HeapGrowingMode,
    ) -> f64 {
        let max_factor = Self::max_growing_factor(max_heap_size);
        let mut factor = Self::dynamic_growing_factor(gc_speed, mutator_speed, max_factor);
        match growing_mode {
            0 => { //Heap::HeapGrowingMode::kConservative =>
                factor = min(factor, Trait::K_CONSERVATIVE_GROWING_FACTOR);
            }
            1 => { //Heap::HeapGrowingMode::kSlow =>
                factor = min(factor, Trait::K_CONSERVATIVE_GROWING_FACTOR);
            }
            2 => { //Heap::HeapGrowingMode::kMinimal =>
                factor = Trait::K_MIN_GROWING_FACTOR;
            }
            3 => {} //Heap::HeapGrowingMode::kDefault => {}
            _ => {}
        }

        // if v8_flags::heap_growing_percent > 0.0 {
        //     factor = 1.0 + v8_flags::heap_growing_percent / 100.0;
        // }

        // if v8_flags::trace_gc_verbose {
        //     //Isolate::from_heap(heap).print_with_timestamp(
        //     //    format!("[{}] factor {:.1} based on mu={:.3}, speed_ratio={:.f} (gc={:.f}, mutator={:.f})\n",
        //     //            Trait::K_NAME, factor, Trait::K_TARGET_MUTATOR_UTILIZATION,
        //     //            gc_speed.unwrap_or(0.0) / mutator_speed, gc_speed.unwrap_or(0.0),
        //     //            mutator_speed).as_str()
        //     //);
        // }
        factor
    }

    /// Calculates the maximum growing factor based on the maximum heap size.
    fn max_growing_factor(max_heap_size: usize) -> f64 {
        const K_MIN_SMALL_FACTOR: f64 = 1.3;
        const K_MAX_SMALL_FACTOR: f64 = 2.0;
        const K_HIGH_FACTOR: f64 = 4.0;

        // If we are on a device with lots of memory, we allow a high heap
        // growing factor.
        if max_heap_size >= Trait::K_MAX_SIZE {
            return K_HIGH_FACTOR;
        }

        let max_size = max(max_heap_size, Trait::K_MIN_SIZE);

        // On smaller devices we linearly scale the factor: C+(D-C)*(X-A)/(B-A)
        let factor = K_MIN_SMALL_FACTOR
            + (K_MAX_SMALL_FACTOR - K_MIN_SMALL_FACTOR)
                * ((max_size - Trait::K_MIN_SIZE) as f64)
                / ((Trait::K_MAX_SIZE - Trait::K_MIN_SIZE) as f64);
        factor
    }

    /// Calculates the dynamic growing factor based on GC speed and mutator speed.
    fn dynamic_growing_factor(
        gc_speed: Option<f64>,
        mutator_speed: f64,
        max_factor: f64,
    ) -> f64 {
        if gc_speed.is_none() || mutator_speed == 0.0 {
            return max_factor;
        }

        let speed_ratio = gc_speed.unwrap() / mutator_speed;

        let a = speed_ratio * (1.0 - Trait::K_TARGET_MUTATOR_UTILIZATION);
        let b = speed_ratio * (1.0 - Trait::K_TARGET_MUTATOR_UTILIZATION)
            - Trait::K_TARGET_MUTATOR_UTILIZATION;

        // The factor is a / b, but we need to check for small b first.
        let factor = if a < b * max_factor {
            a / b
        } else {
            max_factor
        };
        max(factor, Trait::K_MIN_GROWING_FACTOR)
    }

    fn minimum_allocation_limit_growing_step(growing_mode: i32) -> usize { //Heap::HeapGrowingMode
      const K_REGULAR_ALLOCATION_LIMIT_GROWING_STEP: usize = 8;
      const K_LOW_MEMORY_ALLOCATION_LIMIT_GROWING_STEP: usize = 2;
      let limit = if 4096 > MB { 4096 } else { MB }; //PageMetadata::kPageSize

      return limit * (if growing_mode == 0 { //Heap::HeapGrowingMode::kConservative
                          K_LOW_MEMORY_ALLOCATION_LIMIT_GROWING_STEP
                      } else {
                          K_REGULAR_ALLOCATION_LIMIT_GROWING_STEP
                      });
    }

    fn bound_allocation_limit(
        _heap: &(), //Heap,
        current_size: usize,
        limit: u64,
        min_size: usize,
        max_size: usize,
        new_space_capacity: usize,
        growing_mode: i32, //Heap::HeapGrowingMode,
    ) -> usize {
        let mut limit = max(limit, (current_size as u64) + Self::minimum_allocation_limit_growing_step(growing_mode) as u64) + new_space_capacity as u64;
        let halfway_to_the_max = ((current_size as u64) + max_size as u64) / 2;
        let limit_or_halfway = min(limit, halfway_to_the_max);
        let result = max(limit_or_halfway, min_size as u64) as usize;

        //if v8_flags::trace_gc_verbose {
        //    //Isolate::from_heap(heap).print_with_timestamp(
        //    //    format!("[{}] Limit: old size: {} KB, new limit: {} KB\n", Trait::K_NAME, current_size / KB, result / KB).as_str()
        //    //);
        //}

        result
    }
}

pub struct V8HeapTrait;

impl MemoryControllerTrait for V8HeapTrait {
    const K_NAME: &'static str = "V8Heap";
    const K_MIN_GROWING_FACTOR: f64 = 1.05;
    const K_MAX_GROWING_FACTOR: f64 = 1.4;
    const K_CONSERVATIVE_GROWING_FACTOR: f64 = 1.1;
    const K_MIN_SIZE: usize = 16 * MB;
    const K_MAX_SIZE: usize = 2048 * MB;
    const K_TARGET_MUTATOR_UTILIZATION: f64 = 0.92;
}

pub struct GlobalMemoryTrait;

impl MemoryControllerTrait for GlobalMemoryTrait {
    const K_NAME: &'static str = "GlobalMemory";
    const K_MIN_GROWING_FACTOR: f64 = 1.05;
    const K_MAX_GROWING_FACTOR: f64 = 1.2;
    const K_CONSERVATIVE_GROWING_FACTOR: f64 = 1.1;
    const K_MIN_SIZE: usize = 16 * MB;
    const K_MAX_SIZE: usize = 2048 * MB;
    const K_TARGET_MUTATOR_UTILIZATION: f64 = 0.92;
}