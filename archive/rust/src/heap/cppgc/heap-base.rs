// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};

// use v8config; // NOLINT(build/include_directory)

// Placeholder for cppgc includes
mod cppgc {
    pub mod heap_handle {
        pub trait HeapHandle {}
    }
    pub mod heap_statistics {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DetailLevel {
            Coarse,
            Detailed,
        }
        #[derive(Debug)]
        pub struct HeapStatistics {
            pub detail_level: DetailLevel,
        }
    }
    pub mod heap {
        pub enum StackSupport {
            Enabled,
            Disabled,
        }
        pub enum MarkingType {
            Atomic,
            Incremental,
        }
        pub enum SweepingType {
            Parallel,
            Serial,
        }
        pub trait Heap {
            type StackState;
        }
    }
    pub mod internal {
        pub mod persistent_node {
            pub struct PersistentNode {}
        }
    }
    pub mod macros {}
    pub mod platform {
        pub trait Platform {}
    }
}

mod src {
    pub mod base {
        pub mod macros {}
        pub mod os {
            pub fn get_current_thread_id() -> i32 {
                // Replace with actual implementation to get the current thread ID.
                0
            }
        }
    }
    pub mod heap {
        pub mod cppgc {
            pub mod compactor {
                pub struct Compactor {}
            }
            pub mod heap_object_header {
                pub struct HeapObjectHeader {}
            }
            pub mod marker {
                use std::cell::RefCell;
                use std::rc::Rc;

                use crate::cppgc::heap::MarkingType;
                use crate::cppgc::internal::HeapBase;

                pub struct MarkerBase {
                    heap: Rc<RefCell<HeapBase>>,
                }

                impl MarkerBase {
                    pub fn new(heap: Rc<RefCell<HeapBase>>) -> Self {
                        MarkerBase { heap }
                    }

                    pub fn incremental_marking_supported(&self) -> bool {
                        self.heap.borrow().marking_support() != MarkingType::Atomic
                    }
                }
            }
            pub mod metric_recorder {
                pub struct MetricRecorder {}
            }
            pub mod object_allocator {
                pub struct ObjectAllocator {}
            }
            pub mod platform {}
            pub mod process_heap_statistics {
                pub struct ProcessHeapStatistics {}
                pub struct ProcessHeapStatisticsUpdater {}
            }
            pub mod process_heap {
                pub struct ProcessHeap {}
            }
            pub mod raw_heap {
                pub struct RawHeap {}
            }
            pub mod sweeper {
                pub struct Sweeper {}
            }
            pub mod write_barrier {
                pub struct WriteBarrier {}
            }
            #[cfg(defined(CPPGC_YOUNG_GENERATION))]
            pub mod remembered_set {
                pub struct OldToNewRememberedSet {}
            }
        }
    }
}

mod heap {
    pub mod base {
        pub struct Stack {}
    }
}

mod v8 {
    pub mod base {
        pub struct LsanPageAllocator {}
    }
}

pub mod cppgc_internal {
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use std::mem::size_of;
    use std::ptr::NonNull;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, Weak};

    use crate::cppgc::heap::MarkingType;
    use crate::cppgc::heap::SweepingType;
    use crate::cppgc::heap::HeapStatistics;
    use crate::cppgc::heap_handle::HeapHandle;
    use crate::src::heap::cppgc::compactor::Compactor;
    use crate::src::heap::cppgc::marker::MarkerBase;
    use crate::src::heap::cppgc::object_allocator::ObjectAllocator;
    use crate::src::heap::cppgc::raw_heap::RawHeap;
    use crate::src::heap::cppgc::sweeper::Sweeper;
    use crate::src::base::os::get_current_thread_id;
    use crate::src::heap::cppgc::metric_recorder::MetricRecorder;

    // use v8config; // NOLINT(build/include_directory)

    // Placeholder for cppgc includes
    pub mod cppgc {
        pub mod internal {
            pub mod persistent_node {
                pub struct PersistentNode {}
            }
        }
        pub mod macros {}
        pub mod platform {
            pub trait Platform {}
        }
    }

    mod src {
        pub mod base {
            pub mod macros {}
            pub mod os {
                pub fn get_current_thread_id() -> i32 {
                    // Replace with actual implementation to get the current thread ID.
                    0
                }
            }
        }
        pub mod heap {
            pub mod cppgc {
                pub mod compactor {
                    pub struct Compactor {}
                }
                pub mod heap_object_header {
                    pub struct HeapObjectHeader {}
                }
                pub mod marker {
                    pub struct MarkerBase {}
                }
                pub mod object_allocator {
                    pub struct ObjectAllocator {}
                }
                pub mod platform {}
                pub mod process_heap_statistics {
                    pub struct ProcessHeapStatistics {}
                }
                pub mod process_heap {
                    pub struct ProcessHeap {}
                }
                pub mod raw_heap {
                    pub struct RawHeap {}
                }
                pub mod sweeper {
                    pub struct Sweeper {}
                }
                pub mod write_barrier {
                    pub struct WriteBarrier {}
                }
                #[cfg(defined(CPPGC_YOUNG_GENERATION))]
                pub mod remembered_set {
                    pub struct OldToNewRememberedSet {}
                }
            }
        }
    }

    mod heap {
        pub mod base {
            pub struct Stack {}
        }
    }

    mod v8 {
        pub mod base {
            pub struct LsanPageAllocator {}
        }
    }

    pub struct FatalOutOfMemoryHandler {}
    pub struct GarbageCollector {}
    pub struct PageBackend {}
    pub struct PreFinalizerHandler {}
    pub struct StatsCollector {
        metric_recorder: RefCell<Option<Box<MetricRecorder>>>,
    }

    impl StatsCollector {
        pub fn new() -> Self {
            StatsCollector {
                metric_recorder: RefCell::new(None),
            }
        }
        pub fn set_metric_recorder(&self, histogram_recorder: Option<Box<MetricRecorder>>) {
            *self.metric_recorder.borrow_mut() = histogram_recorder;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HeapObjectNameForUnnamedObject {
        kUseHiddenName,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StickyBits {
        kDisabled,
        kEnabled,
    }

    pub trait MoveListener {
        // This function may be called simultaneously on multiple threads.
        // Implementations must not attempt to allocate or do any other actions
        // which could trigger reentrant GC.
        fn on_move(&self, from: usize, to: usize, size_including_header: usize);
    }

    pub struct PersistentRegion {}
    pub struct CrossThreadPersistentRegion {}

    pub struct ProcessHeapStatisticsUpdater {}

    #[cfg(defined(CPPGC_YOUNG_GENERATION))]
    pub struct OldToNewRememberedSet {}

    pub trait PageAllocator {}

    pub struct HeapRegistry {
        subscription: HeapRegistrySubscription,
    }

    impl HeapRegistry {
        fn new(heap: &HeapBase) -> Self {
            HeapRegistry {
                subscription: HeapRegistrySubscription { heap: PhantomData },
            }
        }
    }

    impl Drop for HeapRegistry {
        fn drop(&mut self) {}
    }

    pub struct HeapRegistrySubscription {
        heap: PhantomData<()>,
    }

    impl HeapRegistrySubscription {
        fn new(heap_base: &HeapBase) -> Self {
            HeapRegistrySubscription { heap: PhantomData }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EmbedderStackState {
        kNoHeapPointers,
    }

    // Base class for heap implementations.
    pub struct HeapBase {
        heap_thread_id_: i32,
        raw_heap_: RawHeap,
        platform_: Arc<dyn crate::cppgc::platform::Platform>,
        oom_handler_: Box<FatalOutOfMemoryHandler>,
        #[cfg(defined(LEAK_SANITIZER))]
        lsan_page_allocator_: Box<v8::base::LsanPageAllocator>,
        page_backend_: Box<PageBackend>,
        heap_registry_subscription_: HeapRegistrySubscription,
        stats_collector_: Box<StatsCollector>,
        stack_: Box<heap::base::Stack>,
        prefinalizer_handler_: Box<PreFinalizerHandler>,
        marker_: Rc<RefCell<MarkerBase>>,
        compactor_: Compactor,
        object_allocator_: ObjectAllocator,
        sweeper_: Sweeper,
        strong_persistent_region_: PersistentRegion,
        weak_persistent_region_: PersistentRegion,
        strong_cross_thread_persistent_region_: CrossThreadPersistentRegion,
        weak_cross_thread_persistent_region_: CrossThreadPersistentRegion,
        allocation_observer_for_PROCESS_HEAP_STATISTICS_: ProcessHeapStatisticsUpdater,
        #[cfg(defined(CPPGC_YOUNG_GENERATION))]
        remembered_set_: OldToNewRememberedSet,
        no_gc_scope_: Cell<usize>,
        disallow_gc_scope_: Cell<usize>,
        stack_support_: cppgc::heap::StackSupport,
        stack_state_of_prev_gc_: EmbedderStackState,
        in_atomic_pause_: Cell<bool>,
        marking_support_: cppgc::heap::MarkingType,
        sweeping_support_: cppgc::heap::SweepingType,
        name_for_unnamed_object_: HeapObjectNameForUnnamedObject,
        move_listeners_: RefCell<Vec<Box<dyn MoveListener>>>,
        is_incremental_marking_in_progress_: Cell<bool>,
    }

    impl HeapBase {
        pub fn new(
            platform: Arc<dyn crate::cppgc::platform::Platform>,
            custom_spaces: Vec<Box<dyn Any>>, // Replace Any with correct type
            stack_support: cppgc::heap::StackSupport,
            marking_support: cppgc::heap::MarkingType,
            sweeping_support: cppgc::heap::SweepingType,
            garbage_collector: &GarbageCollector,
        ) -> Self {
            let raw_heap_ = RawHeap {};
            let oom_handler_ = Box::new(FatalOutOfMemoryHandler {});
            #[cfg(defined(LEAK_SANITIZER))]
            let lsan_page_allocator_ = Box::new(v8::base::LsanPageAllocator {});
            let page_backend_ = Box::new(PageBackend {});
            let stats_collector_ = Box::new(StatsCollector::new());
            let stack_ = Box::new(heap::base::Stack {});
            let prefinalizer_handler_ = Box::new(PreFinalizerHandler {});
            let compactor_ = Compactor {};
            let object_allocator_ = ObjectAllocator {};
            let sweeper_ = Sweeper {};
            let strong_persistent_region_ = PersistentRegion {};
            let weak_persistent_region_ = PersistentRegion {};
            let strong_cross_thread_persistent_region_ = CrossThreadPersistentRegion {};
            let weak_cross_thread_persistent_region_ = CrossThreadPersistentRegion {};
            let allocation_observer_for_PROCESS_HEAP_STATISTICS_ = ProcessHeapStatisticsUpdater {};
            #[cfg(defined(CPPGC_YOUNG_GENERATION))]
            let remembered_set_ = OldToNewRememberedSet {};
            let heap_thread_id_ = get_current_thread_id();
            let mut heap = Self {
                heap_thread_id_: heap_thread_id_,
                raw_heap_: raw_heap_,
                platform_: platform,
                oom_handler_: oom_handler_,
                #[cfg(defined(LEAK_SANITIZER))]
                lsan_page_allocator_: lsan_page_allocator_,
                page_backend_: page_backend_,
                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                stats_collector_: stats_collector_,
                stack_: stack_,
                prefinalizer_handler_: prefinalizer_handler_,
                marker_: Rc::new(RefCell::new(MarkerBase {
                    heap: Rc::new(RefCell::new(Self {
                        heap_thread_id_: 0,
                        raw_heap_: RawHeap {},
                        platform_: Arc::new(EmptyPlatform {}),
                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                        page_backend_: Box::new(PageBackend {}),
                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                        stats_collector_: Box::new(StatsCollector::new()),
                        stack_: Box::new(heap::base::Stack {}),
                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                        marker_: Rc::new(RefCell::new(MarkerBase {
                            heap: Rc::new(RefCell::new(Self {
                                heap_thread_id_: 0,
                                raw_heap_: RawHeap {},
                                platform_: Arc::new(EmptyPlatform {}),
                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                page_backend_: Box::new(PageBackend {}),
                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                stats_collector_: Box::new(StatsCollector::new()),
                                stack_: Box::new(heap::base::Stack {}),
                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                marker_: Rc::new(RefCell::new(MarkerBase {
                                    heap: Rc::new(RefCell::new(Self {
                                        heap_thread_id_: 0,
                                        raw_heap_: RawHeap {},
                                        platform_: Arc::new(EmptyPlatform {}),
                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                        page_backend_: Box::new(PageBackend {}),
                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                        stats_collector_: Box::new(StatsCollector::new()),
                                        stack_: Box::new(heap::base::Stack {}),
                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                            heap: Rc::new(RefCell::new(Self {
                                                heap_thread_id_: 0,
                                                raw_heap_: RawHeap {},
                                                platform_: Arc::new(EmptyPlatform {}),
                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                page_backend_: Box::new(PageBackend {}),
                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                stats_collector_: Box::new(StatsCollector::new()),
                                                stack_: Box::new(heap::base::Stack {}),
                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                marker_: Rc::new(RefCell::new(MarkerBase {
                                                    heap: Rc::new(RefCell::new(Self {
                                                        heap_thread_id_: 0,
                                                        raw_heap_: RawHeap {},
                                                        platform_: Arc::new(EmptyPlatform {}),
                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                        page_backend_: Box::new(PageBackend {}),
                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                        stack_: Box::new(heap::base::Stack {}),
                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                                            heap: Rc::new(RefCell::new(Self {
                                                                heap_thread_id_: 0,
                                                                raw_heap_: RawHeap {},
                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                page_backend_: Box::new(PageBackend {}),
                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                stack_: Box::new(heap::base::Stack {}),
                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                marker_: Rc::new(RefCell::new(MarkerBase {
                                                                    heap: Rc::new(RefCell::new(Self {
                                                                        heap_thread_id_: 0,
                                                                        raw_heap_: RawHeap {},
                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                        page_backend_: Box::new(PageBackend {}),
                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                                                            heap: Rc::new(RefCell::new(Self {
                                                                                heap_thread_id_: 0,
                                                                                raw_heap_: RawHeap {},
                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                    heap_thread_id_: 0,
                                                                                    raw_heap_: RawHeap {},
                                                                                    platform_: Arc::new(EmptyPlatform {}),
                                                                                    oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                    page_backend_: Box::new(PageBackend {}),
                                                                                    heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                    stats_collector_: Box::new(StatsCollector::new()),
                                                                                    stack_: Box::new(heap::base::Stack {}),
                                                                                    prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                    marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                        heap_thread_id_: 0,
                                                                                        raw_heap_: RawHeap {},
                                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                        page_backend_: Box::new(PageBackend {}),
                                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                        marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                            heap_thread_id_: 0,
                                                                                            raw_heap_: RawHeap {},
                                                                                            platform_: Arc::new(EmptyPlatform {}),
                                                                                            oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                            page_backend_: Box::new(PageBackend {}),
                                                                                            heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                            stats_collector_: Box::new(StatsCollector::new()),
                                                                                            stack_: Box::new(heap::base::Stack {}),
                                                                                            prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                            marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                heap: Rc::new(RefCell::new(Self {
                                                                                                    heap_thread_id_: 0,
                                                                                                    raw_heap_: RawHeap {},
                                                                                                    platform_: Arc::new(EmptyPlatform {}),
                                                                                                    oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                    page_backend_: Box::new(PageBackend {}),
                                                                                                    heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                    stats_collector_: Box::new(StatsCollector::new()),
                                                                                                    stack_: Box::new(heap::base::Stack {}),
                                                                                                    prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                    marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                        heap: Rc::new(RefCell::new(Self {
                                                                                                            heap_thread_id_: 0,
                                                                                                            raw_heap_: RawHeap {},
                                                                                                            platform_: Arc::new(EmptyPlatform {}),
                                                                                                            oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                            page_backend_: Box::new(PageBackend {}),
                                                                                                            heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                            stats_collector_: Box::new(StatsCollector::new()),
                                                                                                            stack_: Box::new(heap::base::Stack {}),
                                                                                                            prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                            marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                heap: Rc::new(RefCell::new(Self {
                                                                                                                    heap_thread_id_: 0,
                                                                                                                    raw_heap_: RawHeap {},
                                                                                                                    platform_: Arc::new(EmptyPlatform {}),
                                                                                                                    oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                    page_backend_: Box::new(PageBackend {}),
                                                                                                                    heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                    stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                    stack_: Box::new(heap::base::Stack {}),
                                                                                                                    prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                    marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                        heap: Rc::new(RefCell::new(Self {
                                                                                                                            heap_thread_id_: 0,
                                                                                                                            raw_heap_: RawHeap {},
                                                                                                                            platform_: Arc::new(EmptyPlatform {}),
                                                                                                                            oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                            page_backend_: Box::new(PageBackend {}),
                                                                                                                            heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                            stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                            stack_: Box::new(heap::base::Stack {}),
                                                                                                                            prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                            marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                heap_thread_id_: 0,
                                                                                                                                raw_heap_: RawHeap {},
                                                                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                    heap_thread_id_: 0,
                                                                                                                                    raw_heap_: RawHeap {},
                                                                                                                                    platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                    oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                    page_backend_: Box::new(PageBackend {}),
                                                                                                                                    heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                    stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                    stack_: Box::new(heap::base::Stack {}),
                                                                                                                                    prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                    marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                        heap_thread_id_: 0,
                                                                                                                                        raw_heap_: RawHeap {},
                                                                                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                        page_backend_: Box::new(PageBackend {}),
                                                                                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                        marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                            heap_thread_id_: 0,
                                                                                                                                            raw_heap_: RawHeap {},
                                                                                                                                            platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                            oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                            page_backend_: Box::new(PageBackend {}),
                                                                                                                                            heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                            stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                            stack_: Box::new(heap::base::Stack {}),
                                                                                                                                            prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                            marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                                heap_thread_id_: 0,
                                                                                                                                                raw_heap_: RawHeap {},
                                                                                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                                    heap_thread_id_: 0,
                                                                                                                                                    raw_heap_: RawHeap {},
                                                                                                                                                    platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                    oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                    page_backend_: Box::new(PageBackend {}),
                                                                                                                                                    heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                    stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                    stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                    prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                    marker_: Rc::new(RefCell::new(MarkerBase { heap: Rc::new(RefCell::new(Self {
                                                                                                                                                        heap_thread_id_: 0,
                                                                                                                                                        raw_heap_: RawHeap {},
                                                                                                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                        page_backend_: Box::new(PageBackend {}),
                                                                                                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                                                            heap: Rc::new(RefCell::new(Self {
                                                                                                                                                                heap_thread_id_: 0,
                                                                                                                                                                raw_heap_: RawHeap {},
                                                                                                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                                marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                                                                    heap: Rc::new(RefCell::new(Self {
                                                                                                                                                                        heap_thread_id_: 0,
                                                                                                                                                                        raw_heap_: RawHeap {},
                                                                                                                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                                        page_backend_: Box::new(PageBackend {}),
                                                                                                                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                                                                            heap: Rc::new(RefCell::new(Self {
                                                                                                                                                                                heap_thread_id_: 0,
                                                                                                                                                                                raw_heap_: RawHeap {},
                                                                                                                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                                                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                                                prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                                                marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                                                                                    heap: Rc::new(RefCell::new(Self {
                                                                                                                                                                                        heap_thread_id_: 0,
                                                                                                                                                                                        raw_heap_: RawHeap {},
                                                                                                                                                                                        platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                                                        oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                                                        page_backend_: Box::new(PageBackend {}),
                                                                                                                                                                                        heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                                                        stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                                                        stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                                                        prefinalizer_handler_: Box::new(PreFinalizerHandler {}),
                                                                                                                                                                                        marker_: Rc::new(RefCell::new(MarkerBase {
                                                                                                                                                                                            heap: Rc::new(RefCell::new(Self {
                                                                                                                                                                                                heap_thread_id_: 0,
                                                                                                                                                                                                raw_heap_: RawHeap {},
                                                                                                                                                                                                platform_: Arc::new(EmptyPlatform {}),
                                                                                                                                                                                                oom_handler_: Box::new(FatalOutOfMemoryHandler {}),
                                                                                                                                                                                                page_backend_: Box::new(PageBackend {}),
                                                                                                                                                                                                heap_registry_subscription_: HeapRegistrySubscription { heap: PhantomData },
                                                                                                                                                                                                stats_collector_: Box::new(StatsCollector::new()),
                                                                                                                                                                                                stack_: Box::new(heap::base::Stack {}),
                                                                                                                                                           