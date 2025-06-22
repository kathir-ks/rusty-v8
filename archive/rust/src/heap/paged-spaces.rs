pub mod paged_spaces {
    use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};
    use std::{mem, ptr};
    use std::ops::{Deref, DerefMut};
    use std::fmt;

    //use crate::base::bounds; // Assuming a crate `base` with `bounds` module
    //use crate::base::macros; // Assuming a crate `base` with `macros` module
    use crate::flags::flags; // Assuming a crate `flags` with `flags` module
    use crate::heap::allocation_observer; // Assuming a crate `heap` with `allocation_observer` module
    use crate::heap::allocation_stats; // Assuming a crate `heap` with `allocation_stats` module
    use crate::heap::heap_verifier; // Assuming a crate `heap` with `heap_verifier` module
    use crate::heap::heap; // Assuming a crate `heap` with `heap` module
    use crate::heap::memory_chunk_layout; // Assuming a crate `heap` with `memory_chunk_layout` module
    use crate::heap::mutable_page_metadata; // Assuming a crate `heap` with `mutable_page_metadata` module
    use crate::heap::spaces; // Assuming a crate `heap` with `spaces` module
    use crate::common::globals::*;
    

    const KB: usize = 1024;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CompactionSpaceKind {
        kNone,
        // TODO: Add other variants as needed based on the C++ code.
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Executability {
        EXECUTABLE,
        NOT_EXECUTABLE,
    }

    // Placeholder for Tagged<HeapObject> - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct Tagged<T>(*mut T);

    impl<T> Tagged<T> {
        pub fn new(ptr: *mut T) -> Self {
            Tagged(ptr)
        }
    }

    // Placeholder for PtrComprCageBase - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct PtrComprCageBase;

    impl PtrComprCageBase {
        pub fn new() -> Self {
            PtrComprCageBase {}
        }
    }

    // Placeholder for PageMetadata - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct PageMetadata;

    impl PageMetadata {
        pub fn new() -> Self {
            PageMetadata {}
        }
    }

    // Placeholder for MutablePageMetadata - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct MutablePageMetadata;

    impl MutablePageMetadata {
        pub fn new() -> Self {
            MutablePageMetadata {}
        }
    }

    // Placeholder for FreeList - needs more context to translate correctly
    #[derive(Debug)]
    pub struct FreeList;

    impl FreeList {
        pub fn CreateFreeList() -> Box<Self> {
            Box::new(FreeList {})
        }

        pub fn ResetForNonBlackAllocatedPages(&mut self) {}
        pub fn Reset(&mut self) {}
    }

    // Placeholder for Object - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct Object;

    // Placeholder for HeapObject - needs more context to translate correctly
    #[derive(Debug, Clone, Copy)]
    pub struct HeapObject;

    impl HeapObject {
        pub fn FromAddress(address: Address) -> Self {
            HeapObject {}
        }
    }
    
    // Placeholder for ConstPageRange - needs more context to translate correctly
    #[derive(Debug, Clone)]
    pub struct ConstPageRange;

    impl ConstPageRange {
        pub fn new() -> Self {
            ConstPageRange {}
        }

        pub struct iterator;

        impl ConstPageRange {
            pub fn iter(&self) -> ConstPageRange::iterator {
                ConstPageRange::iterator {}
            }
        }

        impl Iterator for ConstPageRange::iterator {
            type Item = *const PageMetadata;

            fn next(&mut self) -> Option<Self::Item> {
                None // Replace with actual iteration logic
            }
        }

        pub type iterator = ConstPageRangeIterator;
    }

    // Placeholder for ConstPageRangeIterator - needs more context to translate correctly
    #[derive(Debug, Clone)]
    pub struct ConstPageRangeIterator;

    impl ConstPageRangeIterator {
        pub fn new() -> Self {
            ConstPageRangeIterator {}
        }
    }
    
    #[derive(Debug)]
    pub struct HeapObjectRange {
        page_: *const PageMetadata,
    }

    impl HeapObjectRange {
        pub fn new(page: *const PageMetadata) -> Self {
            HeapObjectRange { page_ }
        }

        pub fn begin(&self) -> HeapObjectRangeIterator {
            HeapObjectRangeIterator::new(self.page_)
        }

        pub fn end(&self) -> HeapObjectRangeIterator {
             HeapObjectRangeIterator::new(ptr::null())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HeapObjectRangeIterator {
        cage_base_: PtrComprCageBase,
        cur_addr_: Address,
        cur_size_: i32,
        cur_end_: Address,
    }

    impl HeapObjectRangeIterator {
        pub fn new(page: *const PageMetadata) -> Self {
            let mut iterator = HeapObjectRangeIterator {
                cage_base_: PtrComprCageBase::new(),
                cur_addr_: kNullAddress,
                cur_size_: 0,
                cur_end_: kNullAddress,
            };
            
            if !page.is_null() {
                //TODO: Implement the correct logic from C++ iterator(const PageMetadata* page)
                //      to initialize cur_addr_, cur_size_, and cur_end_ from the page
            }
            
            iterator
        }

        fn cage_base(&self) -> PtrComprCageBase {
            self.cage_base_
        }

        fn advance_to_next_object(&mut self) {
            // TODO: Implement the logic for advancing the iterator
            // based on the memory layout and object sizes within the page.
            // This would likely involve reading object headers to determine sizes.
            
            // Placeholder implementation:
            self.cur_addr_ = kNullAddress; // Mark as end for now
        }
    }

    impl Iterator for HeapObjectRangeIterator {
        type Item = Tagged<HeapObject>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur_addr_ == kNullAddress {
                return None;
            }

            let current_object = HeapObject::FromAddress(self.cur_addr_);
            self.advance_to_next_object();
            Some(Tagged::new(current_object as *mut HeapObject))
        }
    }

    // Placeholder for ObjectIterator - needs more context to translate correctly
    pub trait ObjectIterator {
        fn Next(&mut self) -> Tagged<HeapObject>;
    }

    #[derive(Debug)]
    pub struct PagedSpaceObjectIterator {
        cur_: HeapObjectRangeIterator,
        end_: HeapObjectRangeIterator,
        space_: *const PagedSpaceBase,
        page_range_: ConstPageRange,
        current_page_: ConstPageRangeIterator,
        heap_: *mut heap::Heap,
    }

    impl PagedSpaceObjectIterator {
        pub fn new(heap: *mut heap::Heap, space: *const PagedSpaceBase) -> Self {
            unsafe {
                let page = (*space).first_page() as *const PageMetadata;
                PagedSpaceObjectIterator {
                    cur_: HeapObjectRangeIterator::new(page),
                    end_: HeapObjectRangeIterator::new(ptr::null()),
                    space_: space,
                    page_range_: ConstPageRange::new(),
                    current_page_: ConstPageRangeIterator::new(),
                    heap_: heap,
                }
            }
        }

        fn advance_to_next_page(&mut self) -> bool {
            self.current_page_.next();
            
            unsafe {
                if let Some(page) = self.current_page_.next() {
                    self.cur_ = HeapObjectRangeIterator::new(page);
                    self.end_ = HeapObjectRangeIterator::new(ptr::null());
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    impl ObjectIterator for PagedSpaceObjectIterator {
        fn Next(&mut self) -> Tagged<HeapObject> {
            unsafe {
                if let Some(object) = self.cur_.next() {
                    object
                } else {
                    if self.advance_to_next_page() {
                        if let Some(object) = self.cur_.next() {
                            object
                        } else {
                            Tagged::new(ptr::null_mut())
                        }
                    } else {
                        Tagged::new(ptr::null_mut())
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct PagedSpaceBase {
        linear_area: spaces::SpaceWithLinearArea,
        executable_: Executability,
        compaction_space_kind_: CompactionSpaceKind,
        area_size_: usize,
        accounting_stats_: allocation_stats::AllocationStats,
        space_mutex_: Mutex<()>,
        committed_physical_memory_: AtomicUsize,
        size_at_last_gc_: usize,
        free_list_: Box<FreeList>,
        memory_chunk_list_: Vec<*mut PageMetadata>,
        heap_: *mut heap::Heap,
        id_: spaces::AllocationSpace,
    }

    impl PagedSpaceBase {
        pub const kCompactionMemoryWanted: usize = 500 * KB;

        pub fn new(heap: *mut heap::Heap, id: spaces::AllocationSpace, executable: Executability, free_list: Box<FreeList>, compaction_space_kind: CompactionSpaceKind) -> Self {
            let area_size = unsafe{ (*heap).memory_chunk_layout().area_size() };
            PagedSpaceBase {
                linear_area: spaces::SpaceWithLinearArea::new(),
                executable_: executable,
                compaction_space_kind_: compaction_space_kind,
                area_size_: area_size,
                accounting_stats_: allocation_stats::AllocationStats::new(),
                space_mutex_: Mutex::new(()),
                committed_physical_memory_: AtomicUsize::new(0),
                size_at_last_gc_: 0,
                free_list_: free_list,
                memory_chunk_list_: Vec::new(),
                heap_: heap,
                id_: id,
            }
        }

        pub fn id(&self) -> spaces::AllocationSpace {
            self.id_
        }
        
        // Checks whether an object/address is in this space.
        pub fn contains(&self, a: Address) -> bool {
            //TODO: Implement this properly
            self.ContainsSlow(a)
        }
    
        pub fn contains_obj(&self, o: Tagged<Object>) -> bool {
            //TODO: Implement this properly
            unsafe {
                self.ContainsSlow(o.0 as Address)
            }
        }
    
        pub fn ContainsSlow(&self, addr: Address) -> bool {
            //TODO: Implement this properly
            false
        }

        // Does the space need executable memory?
        pub fn executable(&self) -> Executability {
            self.executable_
        }

        // Current capacity without growing (Size() + Available()).
        pub fn Capacity(&self) -> usize {
            self.accounting_stats_.Capacity()
        }

        // Approximate amount of physical memory committed for this space.
        pub fn CommittedPhysicalMemory(&self) -> usize {
            self.committed_physical_memory_.load(Ordering::Relaxed)
        }

        #[cfg(debug_assertions)]
        pub fn VerifyCommittedPhysicalMemory(&self) {
            // TODO: Implement verification logic
        }

        pub fn IncrementCommittedPhysicalMemory(&self, increment_value: usize) {
            self.committed_physical_memory_.fetch_add(increment_value, Ordering::Relaxed);
        }

        pub fn DecrementCommittedPhysicalMemory(&self, decrement_value: usize) {
            self.committed_physical_memory_.fetch_sub(decrement_value, Ordering::Relaxed);
        }

        // Sets the capacity, the available space and the wasted space to zero.
        pub fn ClearAllocatorState(&mut self) {
            self.accounting_stats_.ClearSize();
            if flags::FLAGS.black_allocated_pages {
                self.free_list_.ResetForNonBlackAllocatedPages();
            } else {
                self.free_list_.Reset();
            }
        }

        // Available bytes without growing.
        pub fn Available(&self) -> usize {
            // TODO: Implement this properly
            0
        }

        // Allocated bytes in this space.
        pub fn Size(&self) -> usize {
            self.accounting_stats_.Size()
        }

        // Wasted bytes in this space.
        pub fn Waste(&self) -> usize {
            // TODO: Implement this properly
            0
        }

        // Allocate the requested number of bytes in the space from a background
        // thread.
        pub fn RawAllocateBackground(&self, local_heap: *mut heap::LocalHeap, min_size_in_bytes: usize, max_size_in_bytes: usize, origin: allocation_observer::AllocationOrigin) -> Option<(Address, usize)> {
            // TODO: Implement allocation logic
            None
        }

        // Free a block of memory.
        pub fn Free(&self, start: Address, size_in_bytes: usize) -> usize {
            self.FreeInternal::<false>(start, size_in_bytes)
        }

        pub fn FreeDuringSweep(&self, start: Address, size_in_bytes: usize) -> usize {
            self.FreeInternal::<true>(start, size_in_bytes)
        }

        fn FreeInternal<const DURING_SWEEP: bool>(&self, start: Address, size_in_bytes: usize) -> usize {
            // TODO: Implement freeing logic, update accounting stats, etc.
            0
        }

        pub fn ResetFreeList(&mut self) {
            // TODO: Implement reset logic
        }

        pub fn DecreaseAllocatedBytes(&mut self, bytes: usize, page: *mut PageMetadata) {
            self.accounting_stats_.DecreaseAllocatedBytes(bytes, page);
        }

        pub fn IncreaseAllocatedBytes(&mut self, bytes: usize, page: *mut PageMetadata) {
            self.accounting_stats_.IncreaseAllocatedBytes(bytes, page);
        }

        pub fn DecreaseCapacity(&mut self, bytes: usize) {
            self.accounting_stats_.DecreaseCapacity(bytes);
        }

        pub fn IncreaseCapacity(&mut self, bytes: usize) {
            self.accounting_stats_.IncreaseCapacity(bytes);
        }

        pub fn InitializePage(&mut self, chunk: *mut MutablePageMetadata) -> *mut PageMetadata {
            // TODO: Implement page initialization logic
            unsafe { mem::transmute(chunk) }
        }

        pub fn ReleasePage(&mut self, page: *mut PageMetadata) {
            // TODO: Implement page release logic
        }

        // Adds the page to this space and returns the number of bytes added to the
        // free list of the space.
        pub fn AddPage(&mut self, page: *mut PageMetadata) -> usize {
            self.AddPageImpl(page);
            0
        }

        pub fn RemovePage(&mut self, page: *mut PageMetadata) {
             // TODO: Implement remove page logic
        }

        // Remove a page if it has at least |size_in_bytes| bytes available that can
        // be used for allocation.
        pub fn RemovePageSafe(&mut self, size_in_bytes: i32) -> *mut PageMetadata {
            // TODO: Implement remove page safe logic
            ptr::null_mut()
        }

        #[cfg(feature = "verify_heap")]
        pub fn Verify(&self, isolate: *mut Isolate, visitor: *mut SpaceVerificationVisitor) {
            // TODO: Implement verification logic
        }

        #[cfg(feature = "verify_heap")]
        pub fn VerifyLiveBytes(&self) {
            // TODO: Implement live bytes verification logic
        }

        #[cfg(debug_assertions)]
        pub fn VerifyCountersAfterSweeping(&self, heap: *mut heap::Heap) {
            // TODO: Implement verification logic
        }

        #[cfg(debug_assertions)]
        pub fn VerifyCountersBeforeConcurrentSweeping(&self) {
            // TODO: Implement verification logic
        }

        #[cfg(debug_assertions)]
        pub fn Print(&self) {
            // TODO: Implement printing logic
        }

        #[cfg(debug_assertions)]
        pub fn ReportCodeStatistics(isolate: *mut Isolate) {
            // TODO: Implement reporting logic
        }

        #[cfg(debug_assertions)]
        pub fn ResetCodeStatistics(isolate: *mut Isolate) {
            // TODO: Implement resetting logic
        }

        pub fn CanExpand(&self, size: usize) -> bool {
            // TODO: Implement expansion check logic
            true
        }

        // Returns the number of total pages in this space.
        pub fn CountTotalPages(&self) -> usize {
             self.memory_chunk_list_.len()
        }

        // Return size of allocatable area on a page in this space.
        pub fn AreaSize(&self) -> i32 {
            self.area_size_ as i32
        }

        pub fn is_compaction_space(&self) -> bool {
            self.compaction_space_kind_ != CompactionSpaceKind::kNone
        }

        pub fn compaction_space_kind(&self) -> CompactionSpaceKind {
            self.compaction_space_kind_
        }

        // Merges {other} into the current space.
        pub fn MergeCompactionSpace(&mut self, other: *mut CompactionSpace) {
            // TODO: Implement merge logic
        }

        // Refills the free list from the corresponding free list filled by the
        // sweeper.
        pub fn RefillFreeList(&mut self) {
            // TODO: Implement refill logic
        }

        pub fn mutex(&self) -> &Mutex<()> {
            &self.space_mutex_
        }

        pub fn UnlinkFreeListCategories(&mut self, page: *mut PageMetadata) {
            // TODO: Implement unlink logic
        }

        pub fn RelinkFreeListCategories(&mut self, page: *mut PageMetadata) -> usize {
            // TODO: Implement relink logic
            0
        }

        pub fn first_page(&self) -> *mut PageMetadata {
            if self.memory_chunk_list_.is_empty() {
                ptr::null_mut()
            } else {
                *self.memory_chunk_list_.first().unwrap()
            }
        }

        pub fn last_page(&self) -> *mut PageMetadata {
            if self.memory_chunk_list_.is_empty() {
                ptr::null_mut()
            } else {
                *self.memory_chunk_list_.last().unwrap()
            }
        }
        
        pub fn begin(&self) -> PageIterator {
            PageIterator::new(self.first_page())
        }
    
        pub fn end(&self) -> PageIterator {
            PageIterator::new(ptr::null_mut())
        }

        pub fn ShrinkImmortalImmovablePages(&mut self) {
            // TODO: Implement shrink logic
        }

        pub fn ShrinkPageToHighWaterMark(&mut self, page: *mut PageMetadata) -> usize {
            // TODO: Implement shrink logic
            0
        }

        pub fn GetObjectIterator(&self, heap: *mut heap::Heap) -> Box<dyn ObjectIterator> {
            Box::new(PagedSpaceObjectIterator::new(heap, self))
        }

        pub fn AddRangeToActiveSystemPages(&mut self, page: *mut PageMetadata, start: Address, end: Address) {
            // TODO: Implement add range logic
        }

        pub fn ReduceActiveSystemPages(&mut self, page: *mut PageMetadata, active_system_pages: ActiveSystemPages) {
            // TODO: Implement reduce logic
        }

        // Expands the space by a single page and returns true on success.
        pub fn TryExpand(&mut self, local_heap: *mut heap::LocalHeap, origin: allocation_observer::AllocationOrigin) -> bool {
            // TODO: Implement try expand logic
            false
        }

        pub fn RefineAllocatedBytesAfterSweeping(&mut self, page: *mut PageMetadata) {
            // TODO: Implement refine logic
        }
        
        pub fn AdjustDifferenceInAllocatedBytes(&mut self, diff: usize) {
            // TODO: Implement refine logic
        }

        fn snapshotable(&self) -> bool {
            true
        }

        fn HasPages(&self) -> bool {
            !self.memory_chunk_list_.is_empty()
        }

        fn TearDown(&mut self) {
            // TODO: Implement tear down logic
        }

        fn NotifyNewPage(&mut self, page: *mut PageMetadata) {
            // TODO: Implement notify new page logic
        }
        
        fn AddPageImpl(&mut self, page: *mut PageMetadata) {
            self.memory_chunk_list_.push(page);
        }

        fn ReleasePageImpl(&mut self, page: *mut PageMetadata, free_mode: MemoryAllocator::FreeMode) {
            // TODO: Implement release page logic
        }

        fn SupportsConcurrentAllocation(&self) -> bool {
            !self.is_compaction_space() && (self.id() != spaces::AllocationSpace::NEW_SPACE)
        }
    }

    impl Drop for PagedSpaceBase {
        fn drop(&mut self) {
            self.TearDown();
        }
    }

    pub struct PagedSpace {
        base: PagedSpaceBase,
    }

    impl PagedSpace {
        pub fn new(heap: *mut heap::Heap, id: spaces::AllocationSpace, executable: Executability, free_list: Box<FreeList>, compaction_space_kind: CompactionSpaceKind) -> Self {
            PagedSpace {
                base: PagedSpaceBase::new(heap, id, executable, free_list, compaction_space_kind),
            }
        }

        pub fn CreateAllocatorPolicy(&self, allocator: *mut MainAllocator) -> *mut AllocatorPolicy {
            // TODO: Implement create allocator policy logic
            ptr::null_mut()
        }
    }

    impl Deref for PagedSpace {
        type Target = PagedSpaceBase;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for PagedSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum DestinationHeap {
        kSameHeap,
        kSharedSpaceHeap
    }

    pub struct CompactionSpace {
        base: PagedSpace,
        new_pages_: Vec<*mut PageMetadata>,
        destination_heap_: DestinationHeap,
    }

    impl CompactionSpace {
        pub fn new(heap: *mut heap::Heap, id: spaces::AllocationSpace, executable: Executability, compaction_space_kind: CompactionSpaceKind, destination_heap: DestinationHeap) -> Self {
            CompactionSpace {
                base: PagedSpace::new(heap, id, executable, FreeList::CreateFreeList(), compaction_space_kind),
                new_pages_: Vec::new(),
                destination_heap_: destination_heap,
            }
        }

        pub fn GetNewPages(&self) -> &Vec<*mut PageMetadata> {
            &self.new_pages_
        }

        pub fn RefillFreeList(&mut self) {
            self.base.RefillFreeList();
        }

        pub fn destination_heap(&self) -> DestinationHeap {
            self.destination_heap_
        }
    }

    impl Deref for CompactionSpace {
        type Target = PagedSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for CompactionSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    impl Drop for CompactionSpace {
        fn drop(&mut self) {
            //TODO: Properly implement drop
        }
    }

    #[derive(Debug)]
    pub struct CompactionSpaceCollection {
        old_space_: CompactionSpace,
        code_space_: CompactionSpace,
        shared_space_: Option<CompactionSpace>,
        trusted_space_: CompactionSpace,
    }

    impl CompactionSpaceCollection {
        pub fn new(heap: *mut heap::Heap, compaction_space_kind: CompactionSpaceKind) -> Self {
            let shared_space = if unsafe{ (*heap).isolate().is_shared() } {
                Some(CompactionSpace::new(heap, spaces::AllocationSpace::SHARED_SPACE, Executability::NOT_EXECUTABLE, compaction_space_kind, DestinationHeap::kSharedSpaceHeap))
            } else {
                None
            };
            CompactionSpaceCollection {
                old_space_: CompactionSpace::new(heap, spaces::AllocationSpace::OLD_SPACE, Executability::NOT_EXECUTABLE, compaction_space_kind, DestinationHeap::kSameHeap),
                code_space_: CompactionSpace::new(heap, spaces::AllocationSpace::CODE_SPACE, Executability::EXECUTABLE, compaction_space_kind, DestinationHeap::kSameHeap),
                shared_space_: shared_space,
                trusted_space_: CompactionSpace::new(heap, spaces::AllocationSpace::TRUSTED_SPACE, Executability::NOT_EXECUTABLE, compaction_space_kind, DestinationHeap::kSameHeap),
            }
        }

        pub fn Get(&mut self, space: spaces::AllocationSpace) -> &mut CompactionSpace {
            match space {
                spaces::AllocationSpace::OLD_SPACE => &mut self.old_space_,
                spaces::AllocationSpace::CODE_SPACE => &mut self.code_space_,
                spaces::AllocationSpace::SHARED_SPACE => self.shared_space_.as_mut().expect("Shared space should exist"),
                spaces::AllocationSpace::TRUSTED_SPACE => &mut self.trusted_space_,
                _ => panic!("Unexpected allocation space"),
            }
        }
    }
    
    #[derive(Debug)]
    pub struct OldSpace {
        base: PagedSpace,
        external_backing_store_bytes_: [usize; 2]
    }

    impl OldSpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            OldSpace {
                base: PagedSpace::new(heap, spaces::AllocationSpace::OLD_SPACE, Executability::NOT_EXECUTABLE, FreeList::CreateFreeList(), CompactionSpaceKind::kNone),
                external_backing_store_bytes_: [0; 2],
            }
        }
        
        pub fn AddPromotedPage(&mut self, page: *mut PageMetadata) {
             // TODO: Implement AddPromotedPage logic
        }

        pub fn ReleasePage(&mut self, page: *mut PageMetadata) {
            self.base.ReleasePage(page);
        }

        pub fn ExternalBackingStoreBytes(&self, type_: ExternalBackingStoreType) -> usize {
            if type_ == ExternalBackingStoreType::kArrayBuffer {
                unsafe { (*self.base.heap_).OldArrayBufferBytes() }
            } else {
                self.external_backing_store_bytes_[type_ as usize]
            }
        }
    }

    impl Deref for OldSpace {
        type Target = PagedSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for OldSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }
    
    pub struct StickySpace {
        base: OldSpace,
        allocated_old_size_: usize,
    }

    impl StickySpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            StickySpace {
                base: OldSpace::new(heap),
                allocated_old_size_: 0,
            }
        }

        pub fn From(space: *mut OldSpace) -> *mut Self {
            //assert!(flags::FLAGS.sticky_mark_bits); //TODO: Add assertion in flags
            unsafe {
                space as *mut StickySpace
            }
        }

        pub fn young_objects_size(&self) -> usize {
            //assert!(self.Size() >= self.allocated_old_size_);
            self.Size() - self.allocated_old_size_
        }

        pub fn old_objects_size(&self) -> usize {
            //assert!(self.Size() >= self.allocated_old_size_);
            self.allocated_old_size_
        }

        pub fn set_old_objects_size(&mut self, allocated_old_size: usize) {
            self.allocated_old_size_ = allocated_old_size;
        }

        pub fn NotifyBlackAreaCreated(&mut self, size: usize) {
            //assert!(size <= self.Capacity());
            self.allocated_old_size_ += size;
        }

        pub fn NotifyBlackAreaDestroyed(&mut self, size: usize) {
            //assert!(size <= self.Capacity());
            self.allocated_old_size_ -= size;
        }
    }

    impl Deref for StickySpace {
        type Target = OldSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for StickySpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    #[derive(Debug)]
    pub struct CodeSpace {
        base: PagedSpace,
    }

    impl CodeSpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            CodeSpace {
                base: PagedSpace::new(heap, spaces::AllocationSpace::CODE_SPACE, Executability::EXECUTABLE, FreeList::CreateFreeList(), CompactionSpaceKind::kNone),
            }
        }
    }

    impl Deref for CodeSpace {
        type Target = PagedSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for CodeSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    #[derive(Debug)]
    pub struct SharedSpace {
        base: PagedSpace,
        external_backing_store_bytes_: [usize; 2]
    }

    impl SharedSpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            SharedSpace {
                base: PagedSpace::new(heap, spaces::AllocationSpace::SHARED_SPACE, Executability::NOT_EXECUTABLE, FreeList::CreateFreeList(), CompactionSpaceKind::kNone),
                external_backing_store_bytes_: [0; 2],
            }
        }
        
        pub fn ReleasePage(&mut self, page: *mut PageMetadata) {
            self.base.ReleasePage(page);
        }

        pub fn ExternalBackingStoreBytes(&self, type_: ExternalBackingStoreType) -> usize {
            if type_ == ExternalBackingStoreType::kArrayBuffer {
                return 0;
            }
            //assert_eq!(type_, ExternalBackingStoreType::kExternalString);
            self.external_backing_store_bytes_[type_ as usize]
        }
    }

    impl Deref for SharedSpace {
        type Target = PagedSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for SharedSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    #[derive(Debug)]
    pub struct TrustedSpace {
        base: PagedSpace,
        external_backing_store_bytes_: [usize; 2]
    }

    impl TrustedSpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            TrustedSpace {
                base: PagedSpace::new(heap, spaces::AllocationSpace::TRUSTED_SPACE, Executability::NOT_EXECUTABLE, FreeList::CreateFreeList(), CompactionSpaceKind::kNone),
                external_backing_store_bytes_: [0; 2],
            }
        }
        
        pub fn ExternalBackingStoreBytes(&self, type_: ExternalBackingStoreType) -> usize {
            if type_ == ExternalBackingStoreType::kArrayBuffer {
                return 0;
            }
            //assert_eq!(type_, ExternalBackingStoreType::kExternalString);
            self.external_backing_store_bytes_[type_ as usize]
        }
    }

    impl Deref for TrustedSpace {
        type Target = PagedSpace;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for TrustedSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    #[derive(Debug)]
    pub struct SharedTrustedSpace {
        base: PagedSpace,
        external_backing_store_bytes_: [usize; 2]
    }

    impl SharedTrustedSpace {
        pub fn new(heap: *mut heap::Heap) -> Self {
            SharedTrustedSpace {
                base: PagedSpace::new(heap, spaces::AllocationSpace::SHARED_TRUSTED_SPACE, Executability::NOT_EXECUTABLE, FreeList::CreateFreeList(), CompactionSpaceKind::kNone),
                external_backing_store_bytes_: [0; 2],
            }
        }
        
        pub fn ExternalBackingStoreBytes(&self,