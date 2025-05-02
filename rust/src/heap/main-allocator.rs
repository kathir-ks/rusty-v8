// src/heap/main_allocator.rs

use std::cmp;
use std::sync::{Mutex, MutexGuard};
//use std::option::Option;

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! check_not_null {
            ($arg:expr) => {
                if $arg.is_null() {
                    panic!("Argument cannot be null");
                }
            };
        }

        #[macro_export]
        macro_rules! check {
            ($condition:expr) => {
                if !$condition {
                    panic!("Check failed: {}", stringify!($condition));
                }
            };
        }

        #[macro_export]
        macro_rules! dcheck {
            ($condition:expr) => {
                if cfg!(debug_assertions) && !$condition {
                    panic!("Debug check failed: {}", stringify!($condition));
                }
            };
        }
        
        #[macro_export]
        macro_rules! check_eq {
            ($left:expr, $right:expr) => {
                if $left != $right {
                    panic!("Check failed: {} == {}", $left, $right);
                }
            };
        }
        
        #[macro_export]
        macro_rules! check_le {
            ($left:expr, $right:expr) => {
                if $left > $right {
                    panic!("Check failed: {} <= {}", $left, $right);
                }
            };
        }

        #[macro_export]
        macro_rules! check_ge {
            ($left:expr, $right:expr) => {
                if $left < $right {
                    panic!("Check failed: {} >= {}", $left, $right);
                }
            };
        }

        #[macro_export]
        macro_rules! check_ne {
            ($left:expr, $right:expr) => {
              if $left == $right {
                  panic!("Check failed: {} != {}", $left, $right);
              }
            }
        }

        #[macro_export]
        macro_rules! check_implies {
            ($condition:expr, $implication:expr) => {
                if $condition && !$implication {
                    panic!("Check failed: {} implies {}", stringify!($condition), stringify!($implication));
                }
            };
        }

        #[macro_export]
        macro_rules! dcheck_eq {
          ($left:expr, $right:expr) => {
              if cfg!(debug_assertions) && $left != $right {
                  panic!("Debug check failed: {} == {}", $left, $right);
              }
          }
        }

        #[macro_export]
        macro_rules! dcheck_le {
          ($left:expr, $right:expr) => {
              if cfg!(debug_assertions) && $left > $right {
                  panic!("Debug check failed: {} <= {}", $left, $right);
              }
          }
        }

        #[macro_export]
        macro_rules! dcheck_ge {
          ($left:expr, $right:expr) => {
              if cfg!(debug_assertions) && $left < $right {
                  panic!("Debug check failed: {} >= {}", $left, $right);
              }
          }
        }

        #[macro_export]
        macro_rules! dcheck_ne {
            ($left:expr, $right:expr) => {
              if cfg!(debug_assertions) && $left == $right {
                  panic!("Debug check failed: {} != {}", $left, $right);
              }
            }
        }

        #[macro_export]
        macro_rules! dcheck_implies {
            ($condition:expr, $implication:expr) => {
                if cfg!(debug_assertions) && $condition && !$implication {
                    panic!("Debug check failed: {} implies {}", stringify!($condition), stringify!($implication));
                }
            };
        }

        #[macro_export]
        macro_rules! trace_gc_epoch_with_flow {
            ($tracer:expr, $scope_id:expr, $sweeping_scope_kind:expr, $trace_id:expr, $flags:expr) => {
                // Placeholder for tracing macro. In a real implementation, this
                // would use a tracing library.
                if cfg!(debug_assertions) {
                    println!("TRACE_GC_EPOCH_WITH_FLOW: scope_id={}, kind={:?}, trace_id={}, flags={}",
                             $scope_id, $sweeping_scope_kind, $trace_id, $flags);
                }
            };
        }
    }
}

mod common {
    pub mod globals {
        pub const K_NULL_ADDRESS: Address = 0;
        pub const K_TAGGED_SIZE: usize = 8;

        pub type Address = usize;
        pub type SizeT = usize;

        pub fn is_aligned(value: SizeT, alignment: SizeT) -> bool {
          value % alignment == 0
        }

        pub fn align_to_allocation_alignment(size: usize) -> usize {
            // Replace with actual allocation alignment logic.
            (size + 7) & !7 // Assuming 8-byte alignment
        }
    }
}

mod execution {
  pub mod vm_state {
        // Placeholder
        pub struct VMState<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> VMState<T> {
            pub fn new() -> Self {
                VMState {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    }
}

mod heap {
    use crate::common::globals::{Address, SizeT, K_TAGGED_SIZE, is_aligned, align_to_allocation_alignment, K_NULL_ADDRESS};
    use crate::base::logging::{check_not_null, check, dcheck, check_eq, check_le, check_ge, check_implies, trace_gc_epoch_with_flow};
    use crate::execution::vm_state::VMState;
    use std::option::Option;

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum AllocationSpace {
        NEW_SPACE,
        OLD_SPACE,
        CODE_SPACE,
        SHARED_SPACE,
        MAP_SPACE,
        LO_SPACE,
    }

    pub struct Heap {
        // Placeholder fields
        pub incremental_marking: IncrementalMarking,
        pub sweeping_in_progress_: bool,
        pub isolate_: Isolate,
        pub force_oom_: bool,
        pub gc_tracer: GCTracer,
        pub sweeper: Sweeper,
    }

    impl Heap {
        pub fn new() -> Self {
            Heap {
                incremental_marking: IncrementalMarking::new(),
                sweeping_in_progress_: false,
                isolate_: Isolate::new(),
                force_oom_: false,
                gc_tracer: GCTracer::new(),
                sweeper: Sweeper::new(),
            }
        }

        pub fn create_filler_object_at(&self, address: Address, size: i32) {
            // Dummy implementation
            println!("Creating filler object at {:?} with size {}", address, size);
        }

        pub fn get_fill_to_align(address: Address, alignment: AllocationAlignment) -> i32 {
          // Dummy implementation
          if alignment == AllocationAlignment::K_TAGGED_ALIGNED {
              (alignment as i32 - (address as i32 % alignment as i32)) % alignment as i32
          } else {
            0 // Placeholder
          }
        }

        pub fn get_maximum_fill_to_align(alignment: AllocationAlignment) -> i32 {
            // Dummy implementation
            if alignment == AllocationAlignment::K_TAGGED_ALIGNED {
                (alignment as i32 - 1)
            } else {
              0 // Placeholder
            }
        }

        pub fn start_incremental_marking_if_allocation_limit_is_reached(
            &mut self,
            local_heap: &LocalHeap,
            gc_flags: i32,
            gc_callback_schedule_idle_garbage_collection: i32,
        ) {
            // Dummy implementation
            println!("start_incremental_marking_if_allocation_limit_is_reached with gc_flags: {}, gc_callback_schedule_idle_garbage_collection: {}", gc_flags, gc_callback_schedule_idle_garbage_collection);
        }

        pub fn should_expand_old_generation_on_slow_allocation(
            &self,
            local_heap: &LocalHeap,
            origin: AllocationOrigin,
        ) -> bool {
            // Dummy implementation
            println!("should_expand_old_generation_on_slow_allocation for origin: {:?}", origin);
            true
        }

        pub fn can_expand_old_generation(&self, area_size: SizeT) -> bool {
            // Dummy implementation
            println!("can_expand_old_generation with area_size: {}", area_size);
            true
        }

        pub fn marking_state(&self) -> &MarkingState {
            // Dummy implementation
            println!("Returning marking state");
            &MarkingState{}
        }

        pub fn should_expand_young_generation_on_slow_allocation(
            &self,
            page_size: SizeT,
        ) -> bool {
            // Dummy implementation
            println!("should_expand_young_generation_on_slow_allocation with page_size: {}", page_size);
            true
        }

        pub fn concurrent_marking(&self) -> &ConcurrentMarking {
            &ConcurrentMarking {}
        }

        pub fn is_inline_allocation_enabled(&self) -> bool {
            // Dummy implementation
            true
        }

        pub fn is_allocation_observer_active(&self) -> bool {
            // Dummy implementation
            true
        }

        pub fn isolate(&self) -> &Isolate {
            &self.isolate_
        }

        pub fn force_oom(&self) -> bool {
            self.force_oom_
        }
    }

    pub struct LocalHeap {
        // Placeholder
        heap_: *mut Heap,
        is_main_thread_: bool
    }

    impl LocalHeap {
        pub fn new(heap: *mut Heap, is_main_thread: bool) -> Self {
          LocalHeap {
            heap_: heap,
            is_main_thread_: is_main_thread
          }
        }

        pub fn heap(&self) -> &Heap {
            unsafe { &*self.heap_ }
        }

        pub fn is_main_thread(&self) -> bool {
            self.is_main_thread_
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum AllocationAlignment {
        K_TAGGED_ALIGNED = 8,
        K_CODE_ALIGNED = 16,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum AllocationOrigin {
        // Placeholder
        kRuntime,
        kGC,
        kAllocationSite,
        kContext,
        kDebugger,
        kExtension,
        kNative,
        kApi,
        kBuiltin,
        kOptimizedCompilation,
        kUnoptimizedCompilation,
        kFeedbackVector,
        kMetadata,
        kWeakCell,
        kString,
        kArray,
        kDataObject,
        kJSFunction,
        kJSObject,
        kJSTypedArray,
        kMap,
        kSet,
        kWeakMap,
        kWeakSet,
        kPromise,
        kOther,
    }

    #[derive(Debug)]
    pub enum AllocationResult {
        Success(Address),
        Failure,
    }

    impl AllocationResult {
        pub fn is_failure(&self) -> bool {
            match self {
                AllocationResult::Failure => true,
                _ => false,
            }
        }

        pub fn to_address(&self) -> Address {
            match self {
                AllocationResult::Success(address) => *address,
                AllocationResult::Failure => panic!("Cannot convert failure to address"),
            }
        }

        pub fn from_address(address: Address) -> Self {
          AllocationResult::Success(address)
        }
    }

    pub struct SpaceWithLinearArea {
        // Placeholder
        heap_: *mut Heap,
        identity_: AllocationSpace
    }

    impl SpaceWithLinearArea {
        pub fn new(heap: *mut Heap, identity: AllocationSpace) -> Self {
            SpaceWithLinearArea {
              heap_: heap,
              identity_: identity
            }
        }

        pub fn create_allocator_policy<'a>(&self, allocator: &'a MainAllocator) -> Box<dyn AllocatorPolicy + 'a> {
          match self.identity_ {
            AllocationSpace::NEW_SPACE => {
              let paged_new_space = PagedNewSpace::new(self);
              Box::new(PagedNewSpaceAllocatorPolicy::new(&paged_new_space, allocator))
            },
            _ => Box::new(PagedSpaceAllocatorPolicy::new(self.paged_space(), allocator))
          }
        }

        pub fn heap(&self) -> &Heap {
          unsafe { &*self.heap_ }
        }

        pub fn identity(&self) -> AllocationSpace {
          self.identity_
        }

        pub fn paged_space(&self) -> PagedSpace {
          PagedSpace {}
        }
    }

    pub struct PagedSpaceBase {}

    impl PagedSpaceBase {
      pub fn reset_free_list(&self) {
        // Dummy Implementation
        println!("PagedSpaceBase::reset_free_list()");
      }
    }

    pub struct PagedSpace {}

    impl PagedSpace {
      pub fn new() -> Self {
        PagedSpace {}
      }

      pub fn usable_capacity(&self) -> SizeT {
          0 // Placeholder
      }

      pub fn total_capacity(&self) -> SizeT {
          0 // Placeholder
      }

      pub fn allocate_page(&self) -> bool {
          // Dummy Implementation
          println!("PagedSpace::AllocatePage()");
          true
      }
    }

    pub struct NewSpaces {}

    pub struct PagedNewSpace {
        space_: *mut SpaceWithLinearArea,
        paged_space_: PagedSpace,
        last_lab_page_: Option<PageMetadataPtr>
    }

    impl PagedNewSpace {
        pub fn new(space: *mut SpaceWithLinearArea) -> Self {
            PagedNewSpace {
              space_: space,
              paged_space_: PagedSpace::new(),
              last_lab_page_: None
            }
        }

        pub fn paged_space(&self) -> &PagedSpace {
          &self.paged_space_
        }
    }

    pub struct PageMetadata {}

    impl PageMetadata {
        pub fn from_allocation_area_address(address: Address) -> PageMetadataPtr {
          // Dummy Implementation
          println!("PageMetadata::from_allocation_area_address({:?})", address);
          PageMetadataPtr {}
        }

        pub fn from_heap_object<T>(object: &T) -> PageMetadataPtr {
          // Dummy Implementation
          println!("PageMetadata::from_heap_object()");
          PageMetadataPtr {}
        }

        pub fn from_address(address: Address) -> PageMetadataPtr {
          // Dummy Implementation
          println!("PageMetadata::from_address()");
          PageMetadataPtr {}
        }

        pub fn create_black_area(&self, top: Address, limit: Address) {
            // Dummy implementation
            println!("Creating black area at {:?} with limit {:?}", top, limit);
        }

        pub fn destroy_black_area(&self, top: Address, limit: Address) {
            // Dummy implementation
            println!("Destroying black area at {:?} with limit {:?}", top, limit);
        }

        pub fn increase_allocated_lab_size(&mut self, size: SizeT) {
            // Dummy implementation
            println!("Increase allocated lab size by {:?}", size);
        }

        pub fn decrease_allocated_lab_size(&mut self, size: SizeT) {
            // Dummy implementation
            println!("Decrease allocated lab size by {:?}", size);
        }

        pub const K_PAGE_SIZE: SizeT = 4096;
    }

    pub struct PageMetadataPtr {}

    pub struct MemoryChunkMetadata {}

    impl MemoryChunkMetadata {
        pub fn update_high_water_mark(top: Address) {
            // Dummy implementation
            println!("Updating high water mark to {:?}", top);
        }
    }

    pub struct LinearAllocationArea {
        start: Address,
        top: Address,
        limit: Address,
    }

    impl LinearAllocationArea {
        pub fn new() -> Self {
            LinearAllocationArea {
                start: 0,
                top: 0,
                limit: 0,
            }
        }

        pub fn reset(&mut self, start: Address, end: Address) {
            self.start = start;
            self.top = start;
            self.limit = end;
        }

        pub fn start(&self) -> Address {
            self.start
        }

        pub fn top(&self) -> Address {
            self.top
        }

        pub fn limit(&self) -> Address {
            self.limit
        }

        pub fn increment_top(&mut self, increment: i32) {
          self.top += increment as usize;
        }

        pub fn reset_start(&mut self) {
          self.start = self.top;
        }

        pub fn set_limit(&mut self, limit: Address) {
          self.limit = limit;
        }
    }

    pub struct OwnedLinearAllocationArea {
        linear_allocation_area: LinearAllocationArea,
    }

    impl OwnedLinearAllocationArea {
      pub fn new() -> Self {
        OwnedLinearAllocationArea {
          linear_allocation_area: LinearAllocationArea::new()
        }
      }

      pub fn linear_allocation_area(&self) -> &LinearAllocationArea {
        &self.linear_allocation_area
      }

      pub fn linear_allocation_area_mut(&mut self) -> &mut LinearAllocationArea {
        &mut self.linear_allocation_area
      }
    }

    pub struct AllocationCounter {
        // Placeholder
    }

    impl AllocationCounter {
        pub fn new() -> Self {
            AllocationCounter {}
        }

        pub fn is_step_in_progress(&self) -> bool {
            // Dummy Implementation
            false
        }

        pub fn add_allocation_observer(&self, observer: *mut AllocationObserver) {
            // Dummy Implementation
            println!("AllocationCounter::add_allocation_observer()");
        }

        pub fn remove_allocation_observer(&self, observer: *mut AllocationObserver) {
            // Dummy Implementation
            println!("AllocationCounter::remove_allocation_observer()");
        }

        pub fn next_bytes(&self) -> SizeT {
            // Dummy Implementation
            4096
        }

        pub fn invoke_allocation_observers(
            &self,
            soon_object: Address,
            size_in_bytes: SizeT,
            allocation_size: SizeT,
        ) {
            // Dummy Implementation
            println!(
                "AllocationCounter::invoke_allocation_observers({}, {}, {})",
                soon_object, size_in_bytes, allocation_size
            );
        }
    }

    pub struct AllocationObserver {}

    pub struct MainAllocator {
        local_heap_: *mut LocalHeap,
        isolate_heap_: *mut Heap,
        space_: *mut SpaceWithLinearArea,
        allocation_info_: *mut LinearAllocationArea,
        owned_allocation_info_: LinearAllocationArea,
        allocator_policy_: Box<dyn AllocatorPolicy>,
        supports_extending_lab_: bool,
        black_allocation_: BlackAllocation,
        allocation_counter_: Option<AllocationCounter>,
        linear_area_original_data_: Option<LinearAreaOriginalData>,
    }

    impl MainAllocator {
        const K_LAB_SIZE_IN_GC: usize = 1024;

        pub fn new(
            local_heap: *mut LocalHeap,
            space: *mut SpaceWithLinearArea,
            is_new_generation: IsNewGeneration,
            allocation_info: Option<*mut LinearAllocationArea>,
        ) -> Self {
            unsafe {
                check_not_null!(local_heap);
                let space_ref = &*space;
                let heap = (*local_heap).heap();

                let mut owned_allocation_info_ = LinearAllocationArea::new();
                let allocation_info_ptr = match allocation_info {
                    Some(ptr) => ptr,
                    None => &mut owned_allocation_info_ as *mut LinearAllocationArea,
                };

                let mut allocator = MainAllocator {
                    local_heap_: local_heap,
                    isolate_heap_: heap as *mut Heap,
                    space_: space,
                    allocation_info_: allocation_info_ptr,
                    owned_allocation_info_: owned_allocation_info_,
                    allocator_policy_: space_ref.create_allocator_policy(&MainAllocator {
                        local_heap_: local_heap,
                        isolate_heap_: heap as *mut Heap,
                        space_: space,
                        allocation_info_: allocation_info_ptr,
                        owned_allocation_info_: LinearAllocationArea::new(),
                        allocator_policy_: Box::new(DummyAllocatorPolicy {}),
                        supports_extending_lab_: false,
                        black_allocation_: BlackAllocation::kAlwaysDisabled,
                        allocation_counter_: None,
                        linear_area_original_data_: None,
                    }),
                    supports_extending_lab_: false, //allocator_policy.supports_extending_lab(),
                    black_allocation_: MainAllocator::compute_black_allocation(is_new_generation),
                    allocation_counter_: if (*local_heap).is_main_thread() {
                        Some(AllocationCounter::new())
                    } else {
                        None
                    },
                    linear_area_original_data_: if (*local_heap).is_main_thread() {
                        Some(LinearAreaOriginalData::new())
                    } else {
                        None
                    },
                };
                allocator.supports_extending_lab_ = allocator.allocator_policy_.supports_extending_lab();
                allocator
            }
        }

        pub fn new_in_gc(heap: *mut Heap, space: *mut SpaceWithLinearArea) -> Self {
            MainAllocator {
                local_heap_: std::ptr::null_mut(),
                isolate_heap_: heap,
                space_: space,
                allocation_info_: &mut LinearAllocationArea::new() as *mut LinearAllocationArea,
                owned_allocation_info_: LinearAllocationArea::new(),
                allocator_policy_: Box::new(DummyAllocatorPolicy {}), // Placeholder
                supports_extending_lab_: false,
                black_allocation_: BlackAllocation::kAlwaysDisabled,
                allocation_counter_: None,
                linear_area_original_data_: None,
            }
        }

        const fn compute_black_allocation(is_new_generation: IsNewGeneration) -> BlackAllocation {
            if is_new_generation == IsNewGeneration::kYes {
                return BlackAllocation::kAlwaysDisabled;
            }
            if V8_FLAGS.sticky_mark_bits {
                return BlackAllocation::kAlwaysEnabled;
            }
            BlackAllocation::kEnabledOnMarking
        }

        pub fn align_top_for_testing(&mut self, alignment: AllocationAlignment, offset: i32) -> Address {
            dcheck!(self.top() != 0);

            let filler_size = Heap::get_fill_to_align(self.top(), alignment);

            if filler_size + offset > 0 {
                unsafe {
                  (*self.isolate_heap_).create_filler_object_at(self.top(), filler_size + offset);
                  self.allocation_info_mut().increment_top(filler_size + offset);
                }
            }

            self.top()
        }

        pub fn allocate_raw_force_alignment_for_testing(
            &mut self,
            size_in_bytes: i32,
            alignment: AllocationAlignment,
            origin: AllocationOrigin,
        ) -> AllocationResult {
            let size_in_bytes = align_to_allocation_alignment(size_in_bytes as usize) as i32;

            let result = self.allocate_fast_aligned(size_in_bytes as usize, None, alignment, origin);

            if result.is_failure() {
                self.allocate_raw_slow_aligned(size_in_bytes, alignment, origin)
            } else {
                result
            }
        }

        pub fn is_black_allocation_enabled(&self) -> bool {
            match self.black_allocation_ {
                BlackAllocation::kAlwaysDisabled => false,
                BlackAllocation::kAlwaysEnabled => true,
                BlackAllocation::kEnabledOnMarking => unsafe {
                    (*self.isolate_heap_).incremental_marking.black_allocation()
                },
            }
        }

        pub fn add_allocation_observer(&mut self, observer: *mut AllocationObserver) {
            check!(!self.allocation_counter().as_ref().unwrap().is_step_in_progress());
            dcheck!(!self.is_lab_valid());
            self.allocation_counter().as_ref().unwrap().add_allocation_observer(observer);
        }

        pub fn remove_allocation_observer(&mut self, observer: *mut AllocationObserver) {
            self.allocation_counter().as_ref().unwrap().remove_allocation_observer(observer);
        }

        pub fn pause_allocation_observers(&self) {
            dcheck!(!self.is_lab_valid());
        }

        pub fn resume_allocation_observers(&self) {
            dcheck!(!self.is_lab_valid());
        }

        pub fn advance_allocation_observers(&mut self) {
            if self.supports_allocation_observer() && self.allocation_info().top() != 0 &&
               self.allocation_info().start() != self.allocation_info().top() {
                unsafe {
                    if (*self.isolate_heap_).is_allocation_observer_active() {
                        self.allocation_counter().as_ref().unwrap().invoke_allocation_observers(
                            self.allocation_info().top() - self.allocation_info().start(),
                            self.allocation_info().top() - self.allocation_info().start(),
                            self.allocation_info().top() - self.allocation_info().start()
                        );
                    }
                }
                self.mark_lab_start_initialized();
            }
        }

        pub fn mark_lab_start_initialized(&mut self) {
          self.allocation_info_mut().reset_start();
          // TODO: Verify
        }

        pub fn invoke_allocation_observers(
            &mut self,
            soon_object: Address,
            size_in_bytes: SizeT,
            aligned_size_in_bytes: SizeT,
            allocation_size: SizeT,
        ) {
            dcheck_le!(size_in_bytes, aligned_size_in_bytes);
            dcheck_le!(aligned_size_in_bytes, allocation_size);
            dcheck!(size_in_bytes == aligned_size_in_bytes || aligned_size_in_bytes == allocation_size);

            if !self.supports_allocation_observer() || unsafe { !(*self.isolate_heap_).is_allocation_observer_active() } {
                return;
            }

            if allocation_size >= self.allocation_counter().as_ref().unwrap().next_bytes() {
                dcheck_eq!(soon_object, self.allocation_info().start() + aligned_size_in_bytes - size_in_bytes);
                dcheck_eq!(self.allocation_info().top() + allocation_size - aligned_size_in_bytes, self.allocation_info().limit());

                unsafe {
                    (*self.isolate_heap_).create_filler_object_at(soon_object, size_in_bytes as i32);

                    let saved_allocation_info = self.allocation_info();

                    self.allocation_counter().as_ref().unwrap().invoke_allocation_observers(soon_object, size_in_bytes, allocation_size);

                    dcheck_eq!(saved_allocation_info.start(), self.allocation_info().start());
                    dcheck_eq!(saved_allocation_info.top(), self.allocation_info().top());
                    dcheck_eq!(saved_allocation_info.limit(), self.allocation_info().limit());
                }
            }

            dcheck_le!(self.allocation_info().limit() - self.allocation_info().start(), self.allocation_counter().as_ref().unwrap().next_bytes());
        }

        pub fn allocate_raw_slow(
            &mut self,
            size_in_bytes: i32,
            alignment: AllocationAlignment,
            origin: AllocationOrigin,
        ) -> AllocationResult {
          // We are not supposed to allocate in fast c calls.
          unsafe {
            check_implies!(self.is_main_thread(),
                          V8_FLAGS.allow_allocation_in_fast_api_call || !(*self.isolate_heap_).isolate().in_fast_c_call());
          }

          let result = if USE_ALLOCATION_ALIGNMENT_BOOL && alignment != AllocationAlignment::K_TAGGED_ALIGNED {
              self.allocate_raw_slow_aligned(size_in_bytes, alignment, origin)
          } else {
              self.allocate_raw_slow_unaligned(size_in_bytes, origin)
          };
          return result;
        }

        pub fn allocate_raw_slow_unaligned(
            &mut self,
            size_in_bytes: i32,
            origin: AllocationOrigin,
        ) -> AllocationResult {
            if !self.ensure_allocation(size_in_bytes as usize, AllocationAlignment::K_TAGGED_ALIGNED, origin) {
                return AllocationResult::Failure;
            }

            let result = self.allocate_fast_unaligned(size_in_bytes as usize, origin);
            dcheck!(!result.is_failure());

            self.invoke_allocation_observers(result.to_address(), size_in_bytes as SizeT, size_in_bytes as SizeT, size_in_bytes as SizeT);

            return result;
        }

        pub fn allocate_raw_slow_aligned(
            &mut self,
            size_in_bytes: i32,
            alignment: AllocationAlignment,
            origin: AllocationOrigin,
        ) -> AllocationResult {
            if !self.ensure_allocation(size_in_bytes as usize, alignment, origin) {
                return AllocationResult::Failure;
            }

            let max_aligned_size = size_in_bytes + Heap::get_maximum_fill_to_align(alignment);
            let mut aligned_size_in_bytes: usize = 0;

            let result = self.allocate_fast_aligned(size_in_bytes as usize, Some(&mut aligned_size_in_bytes), alignment, origin);
            dcheck_ge!(max_aligned_size, aligned_size_in_bytes as i32);
            dcheck!(!result.is_failure());

            self.invoke_allocation_observers(result.to_address(), size_in_bytes as SizeT, aligned_size_in_bytes as SizeT, max_aligned_size as SizeT);

            return result;
        }

        pub fn make_linear_allocation_area_iterable(&mut self) {
          if !self.is_lab_valid() {
            return;
          }

          // TODO: Verify()

          let current_top = self.top();
          let current_limit = self.limit();
          if current_top != current_limit {
            unsafe {
              (*self.isolate_heap_).create_filler_object_at(
                current_top,
                (current_limit - current_top) as i32,
              );
            }
          }
        }

        pub fn mark_linear_allocation_area_black(&mut self) {
            dcheck!(self.is_black_allocation_enabled());
            let current_top = self.top();
            let current_limit = self.limit();
            if current_top != K_NULL_ADDRESS && current_top != current_limit {
                PageMetadata::from_allocation_area_address(current_top)
                    .create_black_area(current_top, current_limit);
            }
        }

        pub fn unmark_linear_allocation_area(&mut self) {
            let current_top = self.top();
            let current_limit = self.limit();
            if current_top != K_NULL_ADDRESS && current_top != current_limit {
                PageMetadata::from_allocation_area_address(current_top)
                    .destroy_black_area(current_top, current_limit);
            }
        }

        pub fn free_linear_allocation_area_and_reset_free_list(&mut self) {
          self.free_linear_allocation_area();
          unsafe {
            let main_space = (*self.space_).paged_space();
            main_space.reset_free_list();
          }
        }

        pub fn move_original_top_forward(&mut self) {
          dcheck!(self.supports_pending_allocation());
          let guard = self.linear_area_original_data().as_ref().unwrap().linear_area_lock().lock().unwrap();
          dcheck_ge!(self.top(), self.original_top_acquire());
          dcheck_le!(self.top(), self.original_limit_relaxed());
          self.linear_area_original_data().as_mut().unwrap().set_original_top_release(self.top());
        }

        pub fn reset_lab(&mut self, start: Address, end: Address, extended_end: Address) {
            dcheck_le!(start, end);
            dcheck_le!(end, extended_end);

            if self.is_lab_valid() {
                MemoryChunkMetadata::update_high_water_mark(self.top());
            }

            self.allocation_info_mut().reset(start, end);

            if self.supports_pending_allocation() {
              let guard = self.linear_area_original_data().as_ref().unwrap().linear