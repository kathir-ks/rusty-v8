// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod js_heap_broker {
    use std::sync::{Mutex, MutexGuard};

    // Placeholder for LocalIsolate and Isolate types.  Needs more context.
    pub struct LocalIsolate {}
    pub struct Isolate {
        map_updater_access: Mutex<()>,
        boilerplate_migration_access: Mutex<()>,
    }

    pub struct JSHeapBroker {
        local_isolate_or_isolate: Box<dyn Fn() -> LocalIsolate + Send + Sync>,
        isolate: Box<dyn Fn() -> Isolate + Send + Sync>,
        map_updater_mutex_depth: i32,
        boilerplate_migration_mutex_depth: i32,
    }

    impl JSHeapBroker {
        pub fn new(
            local_isolate_or_isolate: Box<dyn Fn() -> LocalIsolate + Send + Sync>,
            isolate: Box<dyn Fn() -> Isolate + Send + Sync>,
        ) -> Self {
            JSHeapBroker {
                local_isolate_or_isolate,
                isolate,
                map_updater_mutex_depth: 0,
                boilerplate_migration_mutex_depth: 0,
            }
        }

        pub struct RecursiveMutexGuardIfNeeded<'a> {
            mutex_depth_address_: *mut i32,
            initial_mutex_depth_: i32,
            mutex_guard_: Option<MutexGuard<'a, ()>>,
        }

        impl<'a> RecursiveMutexGuardIfNeeded<'a> {
            pub fn new(
                local_isolate: &LocalIsolate,
                mutex: &'a Mutex<()>,
                mutex_depth_address: *mut i32,
            ) -> Self {
                unsafe {
                    let initial_mutex_depth = *mutex_depth_address;
                    let mutex_guard_option = if initial_mutex_depth == 0 {
                        Some(mutex.lock().unwrap())
                    } else {
                        None
                    };

                    let mut guard = RecursiveMutexGuardIfNeeded {
                        mutex_depth_address_: mutex_depth_address,
                        initial_mutex_depth_: initial_mutex_depth,
                        mutex_guard_: mutex_guard_option,
                    };

                    (*mutex_depth_address) += 1;
                    guard
                }
            }
        }

        impl<'a> Drop for RecursiveMutexGuardIfNeeded<'a> {
            fn drop(&mut self) {
                unsafe {
                    (*self.mutex_depth_address_) -= 1;
                }
            }
        }

        pub struct MapUpdaterGuardIfNeeded<'a> {
            recursive_mutex_guard: RecursiveMutexGuardIfNeeded<'a>,
        }

        impl<'a> MapUpdaterGuardIfNeeded<'a> {
            pub fn new(broker: &'a JSHeapBroker) -> Self {
                let isolate_val = (broker.isolate)();
                let local_isolate_val = (broker.local_isolate_or_isolate)();
                MapUpdaterGuardIfNeeded {
                    recursive_mutex_guard: RecursiveMutexGuardIfNeeded::new(
                        &local_isolate_val,
                        &isolate_val.map_updater_access,
                        &mut broker.map_updater_mutex_depth as *mut i32,
                    ),
                }
            }
        }

        pub struct BoilerplateMigrationGuardIfNeeded<'a> {
            recursive_mutex_guard: RecursiveMutexGuardIfNeeded<'a>,
        }

        impl<'a> BoilerplateMigrationGuardIfNeeded<'a> {
            pub fn new(broker: &'a JSHeapBroker) -> Self {
                let isolate_val = (broker.isolate)();
                let local_isolate_val = (broker.local_isolate_or_isolate)();
                BoilerplateMigrationGuardIfNeeded {
                    recursive_mutex_guard: RecursiveMutexGuardIfNeeded::new(
                        &local_isolate_val,
                        &isolate_val.boilerplate_migration_access,
                        &mut broker.boilerplate_migration_mutex_depth as *mut i32,
                    ),
                }
            }
        }
    }
}