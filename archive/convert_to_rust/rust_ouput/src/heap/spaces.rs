// Converted from V8 C++ source files:
// Header: spaces.h
// Implementation: spaces.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap {
    use crate::V8_EXPORT_PRIVATE;
    use crate::Address;
    use crate::AllocationSpace;
    use crate::BaseSpace;
    use crate::Heap;
    use crate::Isolate;
    use crate::LargePageMetadata;
    use crate::MutablePageMetadata;
    use crate::PageMetadata;
    use crate::Tagged;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    pub struct FreeList {}
    impl FreeList {
        pub fn new() -> Self {
            FreeList {}
        }
    }
    pub struct MemoryChunkLayout {}
    impl MemoryChunkLayout {
        pub fn MaxRegularCodeObjectSize() -> usize {
            4096
        }
    }

    pub enum ExternalBackingStoreType {
        kArrayBuffer,
        kNative,
        kNumValues,
    }

    pub trait SpaceTrait {
        fn size_of_objects(&self) -> usize;
        fn available(&self) -> usize;
        fn get_object_iterator(&self, heap: *mut Heap) -> Result<Box<dyn ObjectIteratorTrait>, String>;
        fn increment_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize);
        fn decrement_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize);
        fn external_backing_store_bytes(&self, type_: ExternalBackingStoreType) -> usize;
        fn first_page(&self) -> *mut MutablePageMetadata;
        fn last_page(&self) -> *mut MutablePageMetadata;
        fn memory_chunk_list(&mut self) -> &mut List<MutablePageMetadata>;
        fn initialize_page(&mut self, chunk: *mut MutablePageMetadata) -> *mut PageMetadata;
        fn notify_black_area_created(&mut self, size: usize);
        fn notify_black_area_destroyed(&mut self, size: usize);
        fn free_list(&mut self) -> &mut FreeList;
        fn first_page_address(&self) -> Address;
        fn print(&self);
        fn id(&self) -> AllocationSpace;
    }

    pub struct Space {
        heap: *mut Heap,
        id: AllocationSpace,
        memory_chunk_list: List<MutablePageMetadata>,
        external_backing_store_bytes: [AtomicUsize; ExternalBackingStoreType::kNumValues as usize],
        free_list: Box<FreeList>,
    }

    impl Space {
        pub fn new(heap: *mut Heap, id: AllocationSpace, free_list: Box<FreeList>) -> Self {
            Space {
                heap,
                id,
                memory_chunk_list: List::new(),
                external_backing_store_bytes: [
                    AtomicUsize::new(0),
                    AtomicUsize::new(0),
                ],
                free_list,
            }
        }

        pub fn move_external_backing_store_bytes(
            _type: ExternalBackingStoreType,
            from: &mut Space,
            to: &mut Space,
            amount: usize,
        ) {
            from.decrement_external_backing_store_bytes(_type, amount);
            to.increment_external_backing_store_bytes(_type, amount);
        }
    }
    impl SpaceTrait for Space {
        fn size_of_objects(&self) -> usize {
            self.memory_chunk_list.size()
        }
        fn available(&self) -> usize {
            self.memory_chunk_list.size()
        }
        fn get_object_iterator(&self, _heap: *mut Heap) -> Result<Box<dyn ObjectIteratorTrait>, String> {
             Err("Not implemented".to_string())
        }
        fn increment_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize) {
            self.external_backing_store_bytes[type_ as usize].fetch_add(amount, Ordering::Relaxed);
        }

        fn decrement_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize) {
            self.external_backing_store_bytes[type_ as usize].fetch_sub(amount, Ordering::Relaxed);
        }

        fn external_backing_store_bytes(&self, type_: ExternalBackingStoreType) -> usize {
            self.external_backing_store_bytes[type_ as usize].load(Ordering::Relaxed)
        }

        fn first_page(&self) -> *mut MutablePageMetadata {
            self.memory_chunk_list.front() as *mut MutablePageMetadata
        }

        fn last_page(&self) -> *mut MutablePageMetadata {
            self.memory_chunk_list.back() as *mut MutablePageMetadata
        }

        fn memory_chunk_list(&mut self) -> &mut List<MutablePageMetadata> {
            &mut self.memory_chunk_list
        }

        fn initialize_page(&mut self, _chunk: *mut MutablePageMetadata) -> *mut PageMetadata {
            panic!("UNREACHABLE");
        }

        fn notify_black_area_created(&mut self, _size: usize) {}
        fn notify_black_area_destroyed(&mut self, _size: usize) {}

        fn free_list(&mut self) -> &mut FreeList {
            &mut self.free_list
        }

        fn first_page_address(&self) -> Address {
            let first_page = self.first_page();
            assert!(!first_page.is_null());
            Address { address: 0 }
        }
        fn print(&self) {
            println!("Space");
        }
        fn id(&self) -> AllocationSpace {
            self.id
        }
    }

    pub trait ObjectIteratorTrait {
        fn next(&mut self) -> Tagged<HeapObject>;
    }

    pub struct ObjectIterator {
    }

    impl ObjectIterator {
        pub fn new() -> Self {
            ObjectIterator {}
        }
    }
    impl ObjectIteratorTrait for ObjectIterator {
        fn next(&mut self) -> Tagged<HeapObject> {
            Tagged{_address : 0}
        }
    }
    pub struct PageIteratorImpl<PageType> {
        p_: *mut PageType,
    }

    impl<PageType> PageIteratorImpl<PageType> {
        pub fn new(p: *mut PageType) -> Self {
            PageIteratorImpl { p_: p }
        }

        pub fn increment(&mut self) -> &mut Self {
            unsafe {
                self.p_ = self.p_.offset(1);
            }
            self
        }

        pub fn deref(&self) -> *mut PageType {
            self.p_
        }

        pub fn equals(&self, other: &Self) -> bool {
            self.p_ == other.p_
        }

        pub fn not_equals(&self, other: &Self) -> bool {
            self.p_ != other.p_
        }
    }

    pub type PageIterator = PageIteratorImpl<PageMetadata>;
    pub type ConstPageIterator = PageIteratorImpl<PageMetadata>;
    pub type LargePageIterator = PageIteratorImpl<LargePageMetadata>;
    pub type ConstLargePageIterator = PageIteratorImpl<LargePageMetadata>;

    pub struct PageRange {
        begin_: *mut PageMetadata,
        end_: *mut PageMetadata,
    }

    impl PageRange {
        pub fn new(begin: *mut PageMetadata, end: *mut PageMetadata) -> Self {
            PageRange { begin_: begin, end_: end }
        }

        pub fn from_page(page: *mut PageMetadata) -> Self {
            PageRange { begin_: page, end_: page }
        }

        pub fn begin(&self) -> PageIterator {
            PageIterator::new(self.begin_)
        }

        pub fn end(&self) -> PageIterator {
            PageIterator::new(self.end_)
        }
    }

    pub struct ConstPageRange {
        begin_: *const PageMetadata,
        end_: *const PageMetadata,
    }

    impl ConstPageRange {
        pub fn new(begin: *const PageMetadata, end: *const PageMetadata) -> Self {
            ConstPageRange { begin_: begin, end_: end }
        }

        pub fn from_page(page: *const PageMetadata) -> Self {
            ConstPageRange { begin_: page, end_: page }
        }

        pub fn begin(&self) -> ConstPageIterator {
            ConstPageIterator::new(self.begin_ as *mut PageMetadata)
        }

        pub fn end(&self) -> ConstPageIterator {
            ConstPageIterator::new(self.end_ as *mut PageMetadata)
        }
    }
    pub trait AllocatorPolicy {}
    pub struct SpaceWithLinearArea {
        space: Space,
    }
    impl SpaceWithLinearArea {
        pub fn new(heap: *mut Heap, id: AllocationSpace, free_list: Box<FreeList>) -> Self {
            SpaceWithLinearArea {
                space: Space::new(heap, id, free_list),
            }
        }
        pub fn create_allocator_policy(&self, _allocator: *mut MainAllocator) -> Box<dyn AllocatorPolicy> {
             Box::new(DummyAllocatorPolicy{})
        }

    }
    impl SpaceTrait for SpaceWithLinearArea{
        fn size_of_objects(&self) -> usize {
           self.space.size_of_objects()
        }
        fn available(&self) -> usize {
            self.space.available()
        }
        fn get_object_iterator(&self, heap: *mut Heap) -> Result<Box<dyn ObjectIteratorTrait>, String> {
            self.space.get_object_iterator(heap)
        }
        fn increment_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize) {
           self.space.increment_external_backing_store_bytes(type_, amount)
        }

        fn decrement_external_backing_store_bytes(&mut self, type_: ExternalBackingStoreType, amount: usize) {
            self.space.decrement_external_backing_store_bytes(type_, amount)
        }

        fn external_backing_store_bytes(&self, type_: ExternalBackingStoreType) -> usize {
            self.space.external_backing_store_bytes(type_)
        }

        fn first_page(&self) -> *mut MutablePageMetadata {
            self.space.first_page()
        }

        fn last_page(&self) -> *mut MutablePageMetadata {
            self.space.last_page()
        }

        fn memory_chunk_list(&mut self) -> &mut List<MutablePageMetadata> {
            self.space.memory_chunk_list()
        }

        fn initialize_page(&mut self, chunk: *mut MutablePageMetadata) -> *mut PageMetadata {
            self.space.initialize_page(chunk)
        }

        fn notify_black_area_created(&mut self, size: usize) {
            self.space.notify_black_area_created(size)
        }
        fn notify_black_area_destroyed(&mut self, size: usize) {
            self.space.notify_black_area_destroyed(size)
        }

        fn free_list(&mut self) -> &mut FreeList {
            self.space.free_list()
        }

        fn first_page_address(&self) -> Address {
            self.space.first_page_address()
        }
        fn print(&self) {
            println!("Space");
        }
        fn id(&self) -> AllocationSpace {
            self.space.id()
        }

    }
    struct DummyAllocatorPolicy {}
    impl AllocatorPolicy for DummyAllocatorPolicy {}
    pub struct SpaceIterator {
        heap: *mut Heap,
        current_space: usize,
    }

    impl SpaceIterator {
        pub fn new(heap: *mut Heap) -> Self {
            SpaceIterator {
                heap,
                current_space: AllocationSpace::kFirstMutableSpace as usize,
            }
        }

        pub fn has_next(&mut self) -> bool {
            while self.current_space <= AllocationSpace::kLastMutableSpace as usize {
                let space = unsafe {
                    let heap = &*self.heap;
                    heap.space(self.current_space as AllocationSpace)
                };
                if !space.is_null() {
                    return true;
                }
                self.current_space += 1;
            }
            false
        }

        pub fn next(&mut self) -> *mut dyn SpaceTrait {
            assert!(self.current_space <= AllocationSpace::kLastMutableSpace as usize);
            let space = unsafe {
                let heap = &*self.heap;
                heap.space(self.current_space as AllocationSpace)
            };
            self.current_space += 1;
            assert!(!space.is_null());
            space
        }
    }

    pub struct MemoryChunkIterator {
        space_iterator: SpaceIterator,
        current_chunk: *mut MutablePageMetadata,
    }

    impl MemoryChunkIterator {
        pub fn new(heap: *mut Heap) -> Self {
            MemoryChunkIterator {
                space_iterator: SpaceIterator::new(heap),
                current_chunk: std::ptr::null_mut(),
            }
        }

        pub fn has_next(&mut self) -> bool {
            if !self.current_chunk.is_null() {
                return true;
            }
            while self.space_iterator.has_next() {
                let space = self.space_iterator.next();
                if !space.is_null() {
                    //TODO: Implement the page iteration correctly
                    return false;
                }
            }
            false
        }

        pub fn next(&mut self) -> *mut MutablePageMetadata {
            if self.has_next() {
                let chunk = self.current_chunk;
                self.current_chunk = std::ptr::null_mut();
                return chunk;
            }
            std::ptr::null_mut()
        }
    }

    pub struct List<T> {
        elements: Vec<*mut T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { elements: Vec::new() }
        }

        pub fn push_back(&mut self, element: *mut T) {
            self.elements.push(element);
        }

        pub fn front(&self) -> *mut T {
            if self.elements.is_empty() {
                std::ptr::null_mut()
            } else {
                *self.elements.first().unwrap()
            }
        }

        pub fn back(&self) -> *mut T {
            if self.elements.is_empty() {
                std::ptr::null_mut()
            } else {
                *self.elements.last().unwrap()
            }
        }

        pub fn size(&self) -> usize {
            self.elements.len()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }
    }

    pub struct MainAllocator {}
    impl MainAllocator{
        pub fn new() -> Self{
            MainAllocator{}
        }
    }

    pub trait HeapTrait {
        fn space(&self, id: AllocationSpace) -> *mut dyn SpaceTrait;
    }
} // namespace heap
} // namespace v8
