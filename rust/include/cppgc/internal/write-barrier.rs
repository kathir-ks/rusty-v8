// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::Cell;
use std::marker::PhantomData;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

// TODO: Add a Rust equivalent for v8config.h.  For now, disabling it.
// mod v8config;

mod heap_handle;
mod heap_state;
mod api_constants;
mod atomic_entry_flag;
mod base_page_handle;
mod member_storage;
mod platform;
mod sentinel_pointer;
mod trace_trait;

pub use heap_handle::HeapHandle;
pub use heap_state::HeapState;
pub use api_constants::ApiConstants;
pub use atomic_entry_flag::AtomicEntryFlag;
pub use base_page_handle::BasePageHandle;
pub use member_storage::MemberStorage;
pub use platform::Platform;
pub use sentinel_pointer::kSentinelPointer;
pub use trace_trait::TraceTrait;

#[cfg(feature = "caged_heap")]
mod caged_heap_local_data;
#[cfg(feature = "caged_heap")]
mod caged_heap;

#[cfg(feature = "caged_heap")]
pub use caged_heap_local_data::CagedHeapLocalData;
#[cfg(feature = "caged_heap")]
pub use caged_heap::CagedHeap;

pub mod internal {
    use super::*;
    use std::sync::atomic::{AtomicU8, Ordering};

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u8)]
    pub enum Type {
        kNone,
        kMarking,
        kGenerational,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(u8)]
    pub enum GenerationalBarrierType {
        kPreciseSlot,
        kPreciseUncompressedSlot,
        kImpreciseSlot,
    }

    #[derive(Debug)]
    pub struct Params<'a> {
        pub heap: Option<&'a HeapHandle>,
        #[cfg(debug_assertions)]
        pub r#type: Type,
        #[cfg(feature = "caged_heap")]
        pub slot_offset: usize,
        #[cfg(feature = "caged_heap")]
        pub value_offset: usize,
    }

    impl<'a> Default for Params<'a> {
        fn default() -> Self {
            Params {
                heap: None,
                #[cfg(debug_assertions)]
                r#type: Type::kNone,
                #[cfg(feature = "caged_heap")]
                slot_offset: 0,
                #[cfg(feature = "caged_heap")]
                value_offset: 0,
            }
        }
    }

    pub enum ValueMode {
        kValuePresent,
        kNoValuePresent,
    }

    pub struct WriteBarrier {}

    impl WriteBarrier {
        fn new() {}

        pub fn get_write_barrier_type<'a>(
            slot: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
        ) -> Type {
            WriteBarrierTypePolicy::get::<ValueMode::kValuePresent>(
                slot,
                value,
                params,
                || {},
            )
        }

        pub fn get_write_barrier_type_memberstorage<'a, M: MemberStorage>(
            slot: *const std::ffi::c_void,
            value: M,
            params: &mut Params<'a>,
        ) -> Type {
            WriteBarrierTypePolicy::get::<ValueMode::kValuePresent, _>(
                slot,
                value,
                params,
                || {},
            )
        }

        pub fn get_write_barrier_type_heaphandlecallback<'a, F: FnOnce() -> ()>(
            slot: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: F,
        ) -> Type {
            WriteBarrierTypePolicy::get::<ValueMode::kNoValuePresent>(
                slot,
                std::ptr::null(),
                params,
                callback,
            )
        }

        pub fn get_write_barrier_type_value<'a>(
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
        ) -> Type {
            WriteBarrierTypePolicy::get::<ValueMode::kValuePresent>(
                value,
                params,
                || {},
            )
        }

        #[cfg(feature = "slim_write_barrier")]
        pub unsafe fn combined_write_barrier_slow<T: WriteBarrierSlotType>(slot: *const std::ffi::c_void) {
            todo!()
        }

        pub fn dijkstra_marking_barrier<'a>(params: &Params<'a>, object: *const std::ffi::c_void) {
            Self::check_params(Type::kMarking, params);
            #[cfg(feature = "caged_heap")]
            Self::dijkstra_marking_barrier_slow(object);
            #[cfg(not(feature = "caged_heap"))]
            Self::dijkstra_marking_barrier_slow_with_sentinel_check(object);
        }

        pub fn dijkstra_marking_barrier_range<'a>(
            params: &Params<'a>,
            first_element: *const std::ffi::c_void,
            element_size: usize,
            number_of_elements: usize,
            trace_callback: fn(*const std::ffi::c_void),
        ) {
            Self::check_params(Type::kMarking, params);
            // TODO: fix the unwrap here
            Self::dijkstra_marking_barrier_range_slow(
                params.heap.unwrap(),
                first_element,
                element_size,
                number_of_elements,
                trace_callback,
            );
        }

        pub fn steele_marking_barrier<'a>(params: &Params<'a>, object: *const std::ffi::c_void) {
            Self::check_params(Type::kMarking, params);
            #[cfg(feature = "caged_heap")]
            Self::steele_marking_barrier_slow(object);
            #[cfg(not(feature = "caged_heap"))]
            Self::steele_marking_barrier_slow_with_sentinel_check(object);
        }

        #[cfg(feature = "young_generation")]
        pub fn generational_barrier<'a, const T: GenerationalBarrierType>(
            params: &Params<'a>,
            slot: *const std::ffi::c_void,
        ) {
            Self::check_params(Type::kGenerational, params);

            let local_data = CagedHeapLocalData::get();
            let age_table = &local_data.age_table;

            // Bail out if the slot (precise or imprecise) is in young generation.
            if age_table.get_age(params.slot_offset) == caged_heap::AgeTable::Age::kYoung {
                return;
            }

            // Dispatch between different types of barriers.
            // TODO(chromium:1029379): Consider reload local_data in the slow path to
            // reduce register pressure.
            match T {
                GenerationalBarrierType::kPreciseSlot => {
                    Self::generational_barrier_slow(
                        &local_data,
                        age_table,
                        slot,
                        params.value_offset,
                        params.heap.unwrap(), // TODO: Fix unwrap
                    );
                }
                GenerationalBarrierType::kPreciseUncompressedSlot => {
                    Self::generational_barrier_for_uncompressed_slot_slow(
                        &local_data,
                        age_table,
                        slot,
                        params.value_offset,
                        params.heap.unwrap(), // TODO: Fix unwrap
                    );
                }
                GenerationalBarrierType::kImpreciseSlot => {
                    Self::generational_barrier_for_source_object_slow(
                        &local_data,
                        slot,
                        params.heap.unwrap(), // TODO: Fix unwrap
                    );
                }
            }
        }

        #[cfg(not(feature = "young_generation"))]
        pub fn generational_barrier<'a, const T: GenerationalBarrierType>(
            params: &Params<'a>,
            slot: *const std::ffi::c_void,
        ) {
            let _ = (params, slot); // Suppress unused variable warnings
        }

        #[cfg(debug_assertions)]
        fn check_params<'a>(expected_type: Type, params: &Params<'a>) {
            assert_eq!(expected_type, params.r#type);
        }

        #[cfg(not(debug_assertions))]
        fn check_params<'a>(_expected_type: Type, _params: &Params<'a>) {}

        pub fn is_enabled() -> bool {
            Self::write_barrier_enabled_.might_be_entered()
        }

        fn dijkstra_marking_barrier_slow(value: *const std::ffi::c_void) {
            todo!()
        }

        fn dijkstra_marking_barrier_slow_with_sentinel_check(
            value: *const std::ffi::c_void,
        ) {
            todo!()
        }

        fn dijkstra_marking_barrier_range_slow(
            heap_handle: &HeapHandle,
            first_element: *const std::ffi::c_void,
            element_size: usize,
            number_of_elements: usize,
            trace_callback: fn(*const std::ffi::c_void),
        ) {
            todo!()
        }

        fn steele_marking_barrier_slow(value: *const std::ffi::c_void) {
            todo!()
        }

        fn steele_marking_barrier_slow_with_sentinel_check(
            value: *const std::ffi::c_void,
        ) {
            todo!()
        }

        #[cfg(feature = "young_generation")]
        fn get_local_data(heap_handle: &HeapHandle) -> &CagedHeapLocalData {
            todo!()
        }

        #[cfg(feature = "young_generation")]
        fn generational_barrier_slow<'a>(
            local_data: &CagedHeapLocalData,
            age_table: &caged_heap::AgeTable,
            slot: *const std::ffi::c_void,
            value_offset: usize,
            heap_handle: &HeapHandle,
        ) {
            todo!()
        }

        #[cfg(feature = "young_generation")]
        fn generational_barrier_for_uncompressed_slot_slow<'a>(
            local_data: &CagedHeapLocalData,
            age_table: &caged_heap::AgeTable,
            slot: *const std::ffi::c_void,
            value_offset: usize,
            heap_handle: &HeapHandle,
        ) {
            todo!()
        }

        #[cfg(feature = "young_generation")]
        fn generational_barrier_for_source_object_slow<'a>(
            local_data: &CagedHeapLocalData,
            object: *const std::ffi::c_void,
            heap_handle: &HeapHandle,
        ) {
            todo!()
        }

        static WRITE_BARRIER_ENABLED_: AtomicEntryFlag = AtomicEntryFlag::new();
        pub fn flagupdater() -> FlagUpdater {
            FlagUpdater {}
        }
        static write_barrier_enabled_: AtomicEntryFlag = AtomicEntryFlag::new();
    }

    pub struct FlagUpdater;
    impl FlagUpdater {
        pub fn new() -> Self {
            FlagUpdater {}
        }
    }
    pub trait WriteBarrierSlotType {}

    const unsafe fn set_and_return_type<'a, const TYPE: Type>(params: &mut Params<'a>) -> Type {
        if TYPE == Type::kNone {
            return Type::kNone;
        }
        #[cfg(debug_assertions)]
        {
            params.r#type = TYPE;
        }
        TYPE
    }

    #[cfg(feature = "caged_heap")]
    pub struct WriteBarrierTypeForCagedHeapPolicy {}

    #[cfg(feature = "caged_heap")]
    impl WriteBarrierTypeForCagedHeapPolicy {
        fn new() {}
        pub fn get<'a, const VALUE_MODE: ValueMode>(
            slot: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            ValueModeDispatch::<VALUE_MODE>::get(slot, value, params, callback)
        }

        pub fn get_memberstorage<'a, const VALUE_MODE: ValueMode, M: MemberStorage>(
            slot: *const std::ffi::c_void,
            value: M,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            ValueModeDispatch::<VALUE_MODE>::get_memberstorage(slot, value, params, callback)
        }

        pub fn get_value<'a, const VALUE_MODE: ValueMode>(
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            Self::get_no_slot(value, params, callback)
        }

        fn get_no_slot<'a>(
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            let within_cage = CagedHeap::is_within_cage(value);
            if !within_cage {
                return Type::kNone;
            }

            // We know that |value| points either within the normal page or to the
            // beginning of large-page, so extract the page header by bitmasking.
            let page = BasePageHandle::from_payload(value as *mut std::ffi::c_void);

            let heap_handle = page.heap_handle();
            if heap_handle.is_incremental_marking_in_progress() {
                return unsafe {set_and_return_type::<{ Type::kMarking }>(params)};
            }

            unsafe {set_and_return_type::<{ Type::kNone }>(params)}
        }
    }

    #[cfg(feature = "caged_heap")]
    mod value_mode_dispatch {
        use super::*;
        pub struct ValueModeDispatch<const VALUE_MODE: ValueMode>;

        impl ValueModeDispatch<{ ValueMode::kValuePresent }> {
            pub fn get<'a>(
                slot: *const std::ffi::c_void,
                value: *const std::ffi::c_void,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
                if !WriteBarrier::is_enabled() {
                    return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                }

                Self::barrier_enabled_get(slot, value, params)
            }
            pub fn get_memberstorage<'a, M: MemberStorage>(
                slot: *const std::ffi::c_void,
                storage: M,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
                if !WriteBarrier::is_enabled() {
                    return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                }

                Self::barrier_enabled_get(slot, storage.load(), params)
            }

            fn barrier_enabled_get<'a>(
                slot: *const std::ffi::c_void,
                value: *const std::ffi::c_void,
                params: &mut Params<'a>,
            ) -> Type {
                let within_cage = CagedHeap::are_within_cage(slot, value);
                if !within_cage {
                    return Type::kNone;
                }

                // We know that |value| points either within the normal page or to the
                // beginning of large-page, so extract the page header by bitmasking.
                let page = BasePageHandle::from_payload(value as *mut std::ffi::c_void);

                let heap_handle = page.heap_handle();
                if !heap_handle.is_incremental_marking_in_progress() {
                    #[cfg(feature = "young_generation")]
                    {
                        if !heap_handle.is_young_generation_enabled() {
                            return Type::kNone;
                        }
                        params.heap = Some(&heap_handle);
                        params.slot_offset = CagedHeap::offset_from_address(slot);
                        params.value_offset = CagedHeap::offset_from_address(value);
                        return unsafe {set_and_return_type::<{ Type::kGenerational }>(params)};
                    }
                    #[cfg(not(feature = "young_generation"))]
                    {
                        return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                    }
                }

                // Use marking barrier.
                params.heap = Some(&heap_handle);
                unsafe {set_and_return_type::<{ Type::kMarking }>(params)}
            }
        }

        impl ValueModeDispatch<{ ValueMode::kNoValuePresent }> {
            pub fn get<'a>(
                slot: *const std::ffi::c_void,
                value: *const std::ffi::c_void,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
                if !WriteBarrier::is_enabled() {
                    return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                }

                let handle = callback();
                #[cfg(feature = "young_generation")]
                {
                    if !handle.is_incremental_marking_in_progress() {
                        if !handle.is_young_generation_enabled() {
                            return Type::kNone;
                        }
                        params.heap = Some(&handle);
                        // Check if slot is on stack.
                        if !CagedHeap::is_within_cage(slot) {
                            return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                        }
                        params.slot_offset = CagedHeap::offset_from_address(slot);
                        return unsafe {set_and_return_type::<{ Type::kGenerational }>(params)};
                    }
                }
                #[cfg(not(feature = "young_generation"))]
                {
                    if !handle.is_incremental_marking_in_progress() {
                        return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                    }
                }
                params.heap = Some(&handle);
                unsafe {set_and_return_type::<{ Type::kMarking }>(params)}
            }
             pub fn get_memberstorage<'a, M: MemberStorage>(
                slot: *const std::ffi::c_void,
                storage: M,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
               todo!()
            }
        }

    }

    #[cfg(not(feature = "caged_heap"))]
    pub struct WriteBarrierTypeForNonCagedHeapPolicy {}

    #[cfg(not(feature = "caged_heap"))]
    impl WriteBarrierTypeForNonCagedHeapPolicy {
        fn new() {}
        pub fn get<'a, const VALUE_MODE: ValueMode>(
            slot: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            ValueModeDispatch::<VALUE_MODE>::get(slot, value, params, callback)
        }

        pub fn get_rawpointer<'a, const VALUE_MODE: ValueMode>(
            slot: *const std::ffi::c_void,
            value: RawPointer,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            ValueModeDispatch::<VALUE_MODE>::get(slot, value.load(), params, callback)
        }

        pub fn get_value<'a>(
            value: *const std::ffi::c_void,
            params: &mut Params<'a>,
            callback: impl FnOnce(),
        ) -> Type {
            // The slot will never be used in `Get()` below.
            Self::get::<{ ValueMode::kValuePresent }>(std::ptr::null(), value, params, callback)
        }
    }

    #[cfg(not(feature = "caged_heap"))]
    mod value_mode_dispatch {
        use super::*;
        pub struct ValueModeDispatch<const VALUE_MODE: ValueMode>;

        impl ValueModeDispatch<{ ValueMode::kValuePresent }> {
            pub fn get<'a>(
                _slot: *const std::ffi::c_void,
                object: *const std::ffi::c_void,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
                // The following check covers nullptr as well as sentinel pointer.
                if object <= kSentinelPointer as *const std::ffi::c_void {
                    return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                }
                if !WriteBarrier::is_enabled() {
                    return unsafe {set_and_return_type::<{ Type::kNone }>(params)};
                }
                // We know that |object| is within the normal page or in the beginning of a
                // large page, so extract the page header by bitmasking.
                let page = BasePageHandle::from_payload(object as *mut std::ffi::c_void);

                let heap_handle = page.heap_handle();
                if heap_handle.is_incremental_marking_in_progress() {
                    return unsafe {set_and_return_type::<{ Type::kMarking }>(params)};
                }
                unsafe {set_and_return_type::<{ Type::kNone }>(params)}
            }
        }

        impl ValueModeDispatch<{ ValueMode::kNoValuePresent }> {
            pub fn get<'a>(
                _slot: *const std::ffi::c_void,
                _value: *const std::ffi::c_void,
                params: &mut Params<'a>,
                callback: impl FnOnce(),
            ) -> Type {
                if WriteBarrier::is_enabled() {
                    let handle = callback();
                    if handle.is_incremental_marking_in_progress() {
                        params.heap = Some(&handle);
                        return unsafe {set_and_return_type::<{ Type::kMarking }>(params)};
                    }
                }
                Type::kNone
            }
        }
    }

    #[derive(Debug)]
    pub struct RawPointer {
        ptr: *mut std::ffi::c_void,
    }

    impl RawPointer {
        pub fn load(&self) -> *const std::ffi::c_void {
            self.ptr
        }
    }
    #[cfg(feature = "caged_heap")]
    type WriteBarrierTypePolicy = WriteBarrierTypeForCagedHeapPolicy;
    #[cfg(not(feature = "caged_heap"))]
    type WriteBarrierTypePolicy = WriteBarrierTypeForNonCagedHeapPolicy;
}