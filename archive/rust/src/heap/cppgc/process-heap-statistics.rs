// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod process_heap_statistics {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct ProcessHeapStatistics {
        pub total_allocated_object_size_: AtomicUsize,
        pub total_allocated_space_: AtomicUsize,
    }

    impl ProcessHeapStatistics {
        pub const fn new() -> Self {
            Self {
                total_allocated_object_size_: AtomicUsize::new(0),
                total_allocated_space_: AtomicUsize::new(0),
            }
        }
    }
}

pub mod internal {
    use std::sync::atomic::Ordering;
    use crate::process_heap_statistics::ProcessHeapStatistics;

    pub struct ProcessHeapStatisticsUpdater;

    impl ProcessHeapStatisticsUpdater {
        pub struct AllocationObserverImpl {
            object_size_changes_since_last_reset_: usize,
        }

        impl AllocationObserverImpl {
            pub fn new() -> Self {
                AllocationObserverImpl {
                    object_size_changes_since_last_reset_: 0,
                }
            }
            pub fn allocated_object_size_increased(&mut self, bytes: usize) {
                ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
                self.object_size_changes_since_last_reset_ += bytes;
            }

            pub fn allocated_object_size_decreased(&mut self, bytes: usize) {
                ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(bytes);
                self.object_size_changes_since_last_reset_ -= bytes;
            }

            pub fn reset_allocated_object_size(&mut self, bytes: usize) {
                ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(
                    self.object_size_changes_since_last_reset_,
                );
                ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
                self.object_size_changes_since_last_reset_ = bytes;
            }

            pub fn allocated_size_increased(&mut self, bytes: usize) {
                ProcessHeapStatisticsUpdater::increase_total_allocated_space(bytes);
            }

            pub fn allocated_size_decreased(&mut self, bytes: usize) {
                ProcessHeapStatisticsUpdater::decrease_total_allocated_space(bytes);
            }
        }

        pub fn increase_total_allocated_object_size(delta: usize) {
            extern "C" {
                static cppgc_ProcessHeapStatistics: ProcessHeapStatistics;
            }
            unsafe {
                cppgc_ProcessHeapStatistics.total_allocated_object_size_.fetch_add(delta, Ordering::Relaxed);
            }
        }

        pub fn decrease_total_allocated_object_size(delta: usize) {
            extern "C" {
                static cppgc_ProcessHeapStatistics: ProcessHeapStatistics;
            }
            unsafe {
                cppgc_ProcessHeapStatistics.total_allocated_object_size_.fetch_sub(delta, Ordering::Relaxed);
            }
        }

        pub fn increase_total_allocated_space(delta: usize) {
            extern "C" {
                static cppgc_ProcessHeapStatistics: ProcessHeapStatistics;
            }
            unsafe {
                cppgc_ProcessHeapStatistics.total_allocated_space_.fetch_add(delta, Ordering::Relaxed);
            }
        }

        pub fn decrease_total_allocated_space(delta: usize) {
            extern "C" {
                static cppgc_ProcessHeapStatistics: ProcessHeapStatistics;
            }
            unsafe {
                cppgc_ProcessHeapStatistics.total_allocated_space_.fetch_sub(delta, Ordering::Relaxed);
            }
        }
    }
}