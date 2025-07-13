// Converted from V8 C++ source files:
// Header: write-barrier.h
// Implementation: write-barrier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod write_barrier {
    use std::sync::{Mutex, Arc};
    use crate::heap::cppgc::globals::kSentinelPointer;
    use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
    use crate::heap::cppgc::heap_page::BasePage;
    use crate::heap::cppgc::heap::HeapBase;
    use crate::heap::cppgc::marker::MarkerBase;
    use crate::heap::cppgc::marking_visitor::TraceCallback;
    use crate::base::atomic_entry_flag::AtomicEntryFlag;

    pub struct FlagUpdater {}

    impl FlagUpdater {
        pub fn Enter() {
            WriteBarrier::write_barrier_enabled_.Enter();
        }
        pub fn Exit() {
            WriteBarrier::write_barrier_enabled_.Exit();
        }
    }

    #[cfg(defined(CPPGC_YOUNG_GENERATION))]
    pub struct YoungGenerationEnabler {
        is_enabled_: usize,
        mutex_: Mutex<()>,
    }

    #[cfg(defined(CPPGC_YOUNG_GENERATION))]
    impl YoungGenerationEnabler {
        fn Instance() -> &'static YoungGenerationEnabler {
            static INSTANCE: std::sync::OnceLock<YoungGenerationEnabler> = std::sync::OnceLock::new();
            INSTANCE.get_or_init(|| YoungGenerationEnabler {
                is_enabled_: 0,
                mutex_: Mutex::new(()),
            })
        }

        pub fn Enable() {
            let instance = Self::Instance();
            let _guard = instance.mutex_.lock().unwrap();
            instance.is_enabled_ += 1;
            if instance.is_enabled_ == 1 {
                FlagUpdater::Enter();
            }
        }

        pub fn Disable() {
            let instance = Self::Instance();
            let _guard = instance.mutex_.lock().unwrap();
            assert!(instance.is_enabled_ > 0);
            instance.is_enabled_ -= 1;
            if instance.is_enabled_ == 0 {
                FlagUpdater::Exit();
            }
        }

        pub fn IsEnabled() -> bool {
            let instance = Self::Instance();
            let _guard = instance.mutex_.lock().unwrap();
            instance.is_enabled_ > 0
        }
    }

    pub struct WriteBarrier {}

    impl WriteBarrier {
        pub(crate) static write_barrier_enabled_: AtomicEntryFlag = AtomicEntryFlag::new();

        pub fn DijkstraMarkingBarrierSlowWithSentinelCheck(value: *const std::ffi::c_void) {
            if value.is_null() || value == kSentinelPointer as *const std::ffi::c_void {
                return;
            }
            WriteBarrier::DijkstraMarkingBarrierSlow(value);
        }

        pub fn DijkstraMarkingBarrierSlow(value: *const std::ffi::c_void) {
            unsafe {
                let page = BasePage::FromPayload(value);
                let heap = page.heap();

                if heap.marker().is_none() {
                    eprintln!("Heap marker is none!");
                    return;
                }
                if heap.in_atomic_pause() {
                    eprintln!("Heap is in atomic pause!");
                    return;
                }
                if !heap.is_incremental_marking_in_progress() {
                    eprintln!("Heap is not in incremental marking!");
                    return;
                }

                let header = page.ObjectHeaderFromInnerAddress::<()>(value);
                if let Some(marker) = heap.marker() {
                    marker.WriteBarrierForObject::<MarkerBase::WriteBarrierType::kDijkstra>(
                        header.as_ref().unwrap());
                }
            }
        }

        pub fn DijkstraMarkingBarrierRangeSlow(
            heap_handle: &mut HeapBase,
            first_element: *const std::ffi::c_void,
            element_size: usize,
            number_of_elements: usize,
            trace_callback: TraceCallback,
        ) {
            if heap_handle.marker().is_none() {
                eprintln!("Heap marker is none!");
                return;
            }

            if heap_handle.in_atomic_pause() {
                eprintln!("Heap is in atomic pause!");
                return;
            }

            let disallow_gc_scope = crate::heap::cppgc::subtle::DisallowGarbageCollectionScope::new(heap_handle);

            let mut array = first_element as *const u8;
            for _ in 0..number_of_elements {
                trace_callback(
                    &mut heap_handle.marker().unwrap().Visitor(),
                    array as *const std::ffi::c_void,
                );
                array = unsafe { array.add(element_size) };
            }
        }

        pub fn SteeleMarkingBarrierSlowWithSentinelCheck(value: *const std::ffi::c_void) {
            if value.is_null() || value == kSentinelPointer as *const std::ffi::c_void {
                return;
            }

            WriteBarrier::SteeleMarkingBarrierSlow(value);
        }

        pub fn SteeleMarkingBarrierSlow(value: *const std::ffi::c_void) {
            unsafe {
                let page = BasePage::FromPayload(value);
                let heap = page.heap();

                if heap.marker().is_none() {
                    eprintln!("Heap marker is none!");
                    return;
                }
                if heap.in_atomic_pause() {
                    eprintln!("Heap is in atomic pause!");
                    return;
                }
                if !heap.is_incremental_marking_in_progress() {
                    eprintln!("Heap is not in incremental marking!");
                    return;
                }

                let header = page.ObjectHeaderFromInnerAddress::<()>(value);
                if let Some(marker) = heap.marker() {
                    marker.WriteBarrierForObject::<MarkerBase::WriteBarrierType::kSteele>(
                        header.as_ref().unwrap());
                }
            }
        }

        #[cfg(defined(CPPGC_YOUNG_GENERATION))]
        pub fn GenerationalBarrierSlow(local_data: &CagedHeapLocalData, age_table: &AgeTable, slot: *const std::ffi::c_void, value_offset: usize, heap_handle: &mut HeapBase) {
            if slot.is_null() || heap_handle as *mut HeapBase == std::ptr::null_mut() || value_offset >= api_constants::kCagedHeapMaxReservationSize as usize {
                panic!("GenerationalBarrierSlow preconditions failed.");
            }

            if heap_handle.in_atomic_pause() {
                return;
            }

            if value_offset > 0 && age_table.GetAge(value_offset) == AgeTable::Age::kOld {
                return;
            }

            heap_handle.remembered_set().AddSlot(slot as *mut std::ffi::c_void);
        }

        #[cfg(defined(CPPGC_YOUNG_GENERATION))]
        pub fn GenerationalBarrierForUncompressedSlotSlow(
            local_data: &CagedHeapLocalData,
            age_table: &AgeTable,
            slot: *const std::ffi::c_void,
            value_offset: usize,
            heap_handle: &mut HeapBase,
        ) {
            if slot.is_null()
                || heap_handle as *mut HeapBase == std::ptr::null_mut()
                || value_offset >= api_constants::kCagedHeapMaxReservationSize as usize
            {
                panic!("GenerationalBarrierForUncompressedSlotSlow preconditions failed.");
            }

            if heap_handle.in_atomic_pause() {
                return;
            }

            if value_offset > 0 && age_table.GetAge(value_offset) == AgeTable::Age::kOld {
                return;
            }

            heap_handle
                .remembered_set()
                .AddUncompressedSlot(slot as *mut std::ffi::c_void);
        }

        #[cfg(defined(CPPGC_YOUNG_GENERATION))]
        pub fn GenerationalBarrierForSourceObjectSlow(
            local_data: &CagedHeapLocalData,
            inner_pointer: *const std::ffi::c_void,
            heap_handle: &mut HeapBase,
        ) {
            if inner_pointer.is_null() || heap_handle as *mut HeapBase == std::ptr::null_mut() {
                panic!("GenerationalBarrierForSourceObjectSlow preconditions failed.");
            }

            let object_header = unsafe {
                BasePage::FromInnerAddress(heap_handle, inner_pointer)
                    .ObjectHeaderFromInnerAddress::<()>(inner_pointer)
            };
            heap_handle.remembered_set().AddSourceObject(
                unsafe { &mut *(object_header.unwrap() as *const _ as *mut HeapObjectHeader) },
            );
        }
    }

    #[cfg(V8_ENABLE_CHECKS)]
    impl WriteBarrier {
        pub fn CheckParams(expected_type: Type, params: &Params) {
            assert_eq!(expected_type as u32, params.type_ as u32);
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Type {
        kNone,
        kGenerational,
        kMarking,
    }

    pub struct Params {
        type_: Type,
    }

    #[allow(dead_code)]
    mod api_constants {
        pub const kCagedHeapMaxReservationSize: usize = 256 * 1024 * 1024;
    }

    #[allow(dead_code)]
    mod age_table {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum Age {
            kYoung,
            kOld,
        }

        pub struct AgeTable {}

        impl AgeTable {
            pub fn GetAge(&self, _offset: usize) -> Age {
                Age::kYoung
            }
        }
    }
    use age_table::*;

    #[allow(dead_code)]
    mod caged_heap_local_data {
        pub struct CagedHeapLocalData {}
    }
    use caged_heap_local_data::*;

    #[allow(dead_code)]
    mod remembered_set {
        use crate::heap::cppgc::heap_object_header::HeapObjectHeader;

        pub struct RememberedSet {}

        impl RememberedSet {
            pub fn AddSlot(&mut self, _slot: *mut std::ffi::c_void) {}
            pub fn AddUncompressedSlot(&mut self, _slot: *mut std::ffi::c_void) {}
            pub fn AddSourceObject(&mut self, _header: &mut HeapObjectHeader) {}
        }
    }
    use remembered_set::*;

    impl HeapBase {
        fn remembered_set(&mut self) -> &mut RememberedSet {
            todo!()
        }

        pub fn in_atomic_pause(&self) -> bool {
            false
        }
    }

    impl BasePage {
        unsafe fn ObjectHeaderFromInnerAddress<'a, T>(&self, _address: *const std::ffi::c_void) -> Result<&'a HeapObjectHeader, ()> {
            todo!()
        }
    }
}
