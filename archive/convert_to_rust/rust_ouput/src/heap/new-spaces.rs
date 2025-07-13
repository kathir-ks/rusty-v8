// Converted from V8 C++ source files:
// Header: new-spaces.h
// Implementation: new-spaces.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.h
use std::sync::Mutex;
use std::cell::RefCell;
use std::rc::Rc;

// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PageMetadata {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SemiSpaceNewSpace {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct List<T> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SpaceWithLinearArea {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SemiSpaceObjectIterator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct NewSpace {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct HeapAllocator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ObjectIterator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MutatablePageMetadata {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SemiSpaceAllocatorPolicy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PagedNewSpaceAllocatorPolicy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ExternalString {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct TypedSlots {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MemoryChunk {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Object {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Address {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Space {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct HeapObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Heap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Isolate {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MainAllocator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SpaceVerificationVisitor {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PagedSpaceForNewSpace {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Flags {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SemiSpace {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct CompactionSpaceKind {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AllocationOrigin {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct FreeList {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct YoungArrayBufferBytes {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct DirectHandle<T> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MinorMS {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MemoryAllocator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct IncrementalMarking {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MemoryChunkLayout {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PagedNewSpace {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct V8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MutablePageMetadata {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AllocationObserver {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MarkCompact {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Safepoint {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ArrayBufferSweeper {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct GcTracer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MarkingState {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ConcurrentMarking {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PtrComprCageBase {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Isolate {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Map {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct YoungGenerationPageFlags {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct RootIndex {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct LocalHeap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct StatsCounter {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PagedNewSpaceAllocatorPolicy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AllocatorPolicy {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct GCType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AbortReason {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Condition {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MachineType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Block {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Operation {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct VisitorId {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct InstructionOperand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct CodePointerHandle {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct BlockVector {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Register {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Label {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MachineType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct V8_EXPORT_PRIVATE {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Visitor {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AllocationAlignment {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PagedSpaceBase {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Counters {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MarkCompactCollector {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSDispatchHandle {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct LargePageMetadata {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Range {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct String {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct DisplayNamesInternal {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Managed<T> {
    value: i32,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSWeakRefs {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Operand {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct FreeSpaceMap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Label {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct IrregexpImplementation {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Tagged<T> {
    value: i32,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct HeapFlags {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct GCType {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MarkingWorklistsLocal {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct RegisterT {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct AccessorPair {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct RegExpData {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct StringViewSet {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct MachineCode {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct NativeContext {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ArrayBufferSweeper {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct Smi {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct OrderedHashMap {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct HashMapEntry {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSArrayBuffer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSArrayBufferView {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSSharedStruct {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSSharedArrayBuffer {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct JSDataView {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct FeedbackVector {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct SlotSet {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct List<MutablePageMetadata> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct List<PageMetadata> {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PageIterator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct ConstPageIterator {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
pub struct PageRange {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
enum SemiSpaceId { kFromSpace = 0, kToSpace = 1 }
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
impl SemiSpace {
    pub fn swap(from: &mut SemiSpace, to: &mut SemiSpace) {
        // Implementation of Swap function
    }
    pub fn new(heap: *mut Heap, semispace: SemiSpaceId, initial_capacity: usize, minimum_capacity: usize, maximum_capacity: usize) -> SemiSpace{
        SemiSpace{id_ : semispace}
    }
    pub fn contains(&self, o: Tagged<HeapObject>) -> bool {
        true
    }
    pub fn contains_obj(&self, o: Tagged<Object>) -> bool {
        true
    }
    pub fn contains_t<T>(&self, o: Tagged<T>) -> bool {
        true
    }
    pub fn contains_slow(&self, a: Address) -> bool {
        true
    }
    pub fn commit(&mut self) -> bool {
        true
    }
    pub fn uncommit(&mut self){

    }
    pub fn is_committed(&self) -> bool {
        true
    }
    pub fn grow_to(&mut self, new_capacity: usize) -> bool {
        true
    }
    pub fn shrink_to(&mut self, new_capacity: usize){

    }
    pub fn ensure_current_capacity(&mut self) -> bool{
        true
    }
    pub fn space_start(&self) -> Address {
        Address {}
    }
    pub fn current_page(&mut self) -> *mut PageMetadata{
        0 as *mut PageMetadata
    }
    pub fn page_low(&self) -> Address{
        Address {}
    }
    pub fn page_high(&self) -> Address{
        Address {}
    }
    pub fn advance_page(&mut self) -> bool{
        true
    }
    pub fn reset(&mut self){

    }
    pub fn remove_page(&mut self, page: *mut PageMetadata){

    }
    pub fn move_page_to_the_end(&mut self, page: *mut PageMetadata){

    }
    pub fn initialize_page(&mut self, chunk: *mut MutatablePageMetadata) -> *mut PageMetadata{
        0 as *mut PageMetadata
    }
    pub fn age_mark(&self) -> Address{
        Address {}
    }
    pub fn set_age_mark(&mut self, mark: Address){

    }
    pub fn current_capacity(&self) -> usize{
        0
    }
    pub fn current_capacity_safe(&self) -> usize{
        0
    }
    pub fn target_capacity(&self) -> usize{
        0
    }
    pub fn maximum_capacity(&self) -> usize{
        0
    }
    pub fn minimum_capacity(&self) -> usize{
        0
    }
    pub fn id(&self) -> SemiSpaceId{
        SemiSpaceId::kFromSpace
    }
    pub fn committed_physical_memory(&self) -> usize{
        0
    }
    pub fn size(&self) -> usize{
        0
    }
    pub fn size_of_objects(&self) -> usize{
        0
    }
    pub fn available(&self) -> usize{
        0
    }
    pub fn first_page(&mut self) -> *mut PageMetadata{
        0 as *mut PageMetadata
    }
    pub fn last_page(&mut self) -> *mut PageMetadata{
        0 as *mut PageMetadata
    }
    pub fn first_page_const(&self) -> *const PageMetadata{
        0 as *const PageMetadata
    }
    pub fn last_page_const(&self) -> *const PageMetadata{
        0 as *const PageMetadata
    }
    pub fn begin(&mut self) -> PageIterator{
        PageIterator {}
    }
    pub fn end(&mut self) -> PageIterator{
        PageIterator {}
    }
    pub fn begin_const(&self) -> ConstPageIterator{
        ConstPageIterator {}
    }
    pub fn end_const(&self) -> ConstPageIterator{
        ConstPageIterator {}
    }
    pub fn get_object_iterator(&mut self, heap: *mut Heap) -> *mut ObjectIterator{
        0 as *mut ObjectIterator
    }
    pub fn print(&mut self){

    }
    pub fn assert_valid_range(from: Address, to: Address){

    }
    pub fn add_range_to_active_system_pages(&mut self, start: Address, end: Address){

    }
    pub fn move_quarantined_page(&mut self, chunk: *mut MemoryChunk){

    }
    pub fn fix_pages_flags(&mut self){

    }
    pub fn is_address_below_age_mark_for_space(&self, space: &SemiSpace, address: Address) -> bool{
        true
    }
    pub fn set_quarantined_size(&mut self, quarantined_size: usize){

    }
    pub committed_physical_memory_ : usize,
    id_ : SemiSpaceId,
    committed_ : std::sync::atomic::AtomicUsize,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
impl NewSpace {
    pub fn new(heap: *mut Heap) -> NewSpace{
        NewSpace{}
    }
    pub fn contains(&self, o: Tagged<Object>) -> bool{
        true
    }
    pub fn contains_heap_object(&self, o: Tagged<HeapObject>) -> bool{
        true
    }
    pub fn promote_page_to_old_space(&mut self, page: *mut PageMetadata){

    }
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
impl SemiSpaceNewSpace {
    pub fn from(space: *mut NewSpace) -> *mut SemiSpaceNewSpace{
        0 as *mut SemiSpaceNewSpace
    }
    pub fn new(heap: *mut Heap, initial_semispace_capacity: usize, min_semispace_capacity_: usize, max_semispace_capacity: usize) -> SemiSpaceNewSpace{
        SemiSpaceNewSpace{}
    }
    pub fn contains_slow(&self, a: Address) -> bool{
        true
    }
    pub fn grow(&mut self, new_capacity: usize){

    }
    pub fn set_age_mark_to_top(&mut self){

    }
    pub fn shrink(&mut self, new_capacity: usize){

    }
    pub fn committed_physical_memory(&self) -> usize{
        0
    }
    pub fn ensure_current_capacity(&mut self) -> bool{
        true
    }
    pub fn reset_current_space(&mut self){

    }
    pub fn add_fresh_page(&mut self) -> bool{
        true
    }
    pub fn allocate_on_new_page_beyond_capacity(&mut self, size_in_bytes: i32, alignment: AllocationAlignment) -> std::option::Option<std::pair<Address, Address>>{
        std::option::Option::None
    }
    pub fn add_parked_allocation_buffer(&mut self, size_in_bytes: i32, alignment: AllocationAlignment) -> bool{
        true
    }
    pub fn reset_parked_allocation_buffers(&mut self){

    }
    pub fn get_space_remaining_on_current_page_for_testing(&mut self) -> i32{
        0
    }
    pub fn fill_current_page_for_testing(&mut self){

    }
    pub fn verify(&mut self, isolate: *mut Isolate, visitor: *mut SpaceVerificationVisitor){

    }
    pub fn verify_objects(&mut self, isolate: *mut Isolate, visitor: *mut SpaceVerificationVisitor){

    }
    pub fn make_iterable(&mut self){

    }
    pub fn make_all_pages_in_from_space_iterable(&mut self){

    }
    pub fn make_unused_pages_in_to_space_iterable(&mut self){

    }
    pub fn is_address_below_age_mark(&mut self, address: Address) -> bool{
        true
    }
    pub fn should_be_promoted(&mut self, address: Address) -> bool{
        true
    }
    pub fn should_page_be_promoted(&mut self, address: Address) -> bool{
        true
    }
    pub fn get_object_iterator(&mut self, heap: *mut Heap) -> *mut ObjectIterator{
        0 as *mut ObjectIterator
    }
    pub fn size(&self) -> usize{
        0
    }
    pub fn allocated_since_last_gc(&self) -> usize{
        0
    }
    pub fn garbage_collection_prologue(&mut self){

    }
    pub fn evacuate_prologue(&mut self){

    }
    pub fn garbage_collection_epilogue(&mut self){

    }
    pub fn zap_unused_memory(&mut self){

    }
    pub fn remove_page(&mut self, page: *mut PageMetadata){

    }
    pub fn is_promotion_candidate(&self, page: *const MutatablePageMetadata) -> bool{
        true
    }
    pub fn allocate(&mut self, size_in_bytes: i32, alignment: AllocationAlignment) -> std::option::Option<std::pair<Address, Address>>{
        std::option::Option::None
    }
    pub fn free(&mut self, start: Address, end: Address){

    }
    pub fn create_allocator_policy(&mut self, allocator: *mut MainAllocator) -> *mut AllocatorPolicy{
        0 as *mut AllocatorPolicy
    }
    pub fn move_quarantined_page(&mut self, chunk: *mut MemoryChunk){

    }
    pub fn is_from_space_committed(&self) -> bool{
        true
    }
    pub fn quarantined_size(&self) -> usize{
        0
    }
    pub fn from_space(&mut self) -> &mut SemiSpace{
        &mut SemiSpace::new(0 as *mut Heap, SemiSpaceId::kFromSpace,0,0,0)
    }
    pub fn to_space(&mut self) -> &mut SemiSpace{
        &mut SemiSpace::new(0 as *mut Heap, SemiSpaceId::kToSpace,0,0,0)
    }
    pub fn capacity(&self) -> usize{
        0
    }
    pub fn available(&self) -> usize{
        0
    }
    pub fn quarantined_page_count(&self) -> usize{
        0
    }
    pub fn total_capacity(&self) -> usize{
        0
    }
    pub fn maximum_committed_memory(&self) -> usize{
        0
    }
    pub fn committed_memory(&self) -> usize{
        0
    }
    pub fn maximum_capacity(&self) -> usize{
        0
    }
    pub fn minimum_capacity(&self) -> usize{
        0
    }
    pub fn initial_total_capacity(&self) -> usize{
        0
    }
    pub fn first_allocatable_address(&mut self) -> Address{
        Address {}
    }
    pub fn age_mark(&self) -> Address{
        Address {}
    }
    
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
impl PagedSpaceForNewSpace {
    pub fn new(heap: *mut Heap, initial_capacity: usize, min_capacity: usize, max_capacity: usize) -> PagedSpaceForNewSpace{
        PagedSpaceForNewSpace{}
    }
    pub fn tear_down(&mut self){

    }
    pub fn grow(&mut self, new_capacity: usize){

    }
    pub fn start_shrinking(&mut self, new_target_capacity: usize) -> bool{
        true
    }
    pub fn finish_shrinking(&mut self){

    }
    pub fn allocated_since_last_gc(&self) -> usize{
        0
    }
    pub fn minimum_capacity(&self) -> usize{
        0
    }
    pub fn maximum_capacity(&self) -> usize{
        0
    }
    pub fn total_capacity(&self) -> usize{
        0
    }
    pub fn first_allocatable_address(&self) -> Address{
        Address {}
    }
    pub fn garbage_collection_epilogue(&mut self){

    }
    pub fn ensure_current_capacity(&mut self) -> bool{
        true
    }
    pub fn initialize_page(&mut self, chunk: *mut MutatablePageMetadata) -> *mut PageMetadata{
        0 as *mut PageMetadata
    }
    pub fn add_page(&mut self, page: *mut PageMetadata) -> usize{
        0
    }
    pub fn remove_page(&mut self, page: *mut PageMetadata){

    }
    pub fn release_page(&mut self, page: *mut PageMetadata){

    }
    pub fn external_backing_store_bytes(&mut self, type_: i32) -> usize{
        0
    }
    pub fn verify(&mut self, isolate: *mut Isolate, visitor: *mut SpaceVerificationVisitor){

    }
    pub fn make_iterable(&mut self){

    }
    pub fn should_release_empty_page(&self) -> bool{
        true
    }
    pub fn allocate_page_up_to_capacity_for_testing(&mut self){

    }
    pub fn is_promotion_candidate(&self, page: *const MutablePageMetadata) -> bool{
        true
    }
    pub fn available(&self) -> usize{
        0
    }
    pub fn usable_capacity(&self) -> usize{
        0
    }
    pub fn create_allocator_policy(&mut self, allocator: *mut MainAllocator) -> *mut AllocatorPolicy{
        0 as *mut AllocatorPolicy
    }
    pub fn allocate_page(&mut self) -> bool{
        true
    }
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/new-spaces.cc
impl PagedNewSpace {
    pub fn from(space: *mut NewSpace) -> *mut PagedNewSpace{
        0 as *mut PagedNewSpace
    }
    pub fn new(heap: *mut Heap, initial_capacity: usize, min_capacity: usize, max_capacity: usize) -> PagedNewSpace{
        PagedNewSpace{}
    }
    pub fn contains_slow(&self, a: Address) -> bool{
        true
    }
    pub fn grow(&mut self, new_capacity: usize){

    }
    pub fn start_shrinking(&mut self, new_target_capacity: usize) -> bool{
        true
    }
    pub fn finish_shrinking(&mut self){

    }
    pub fn size(&self) -> usize{
        0
    }
    pub fn size_of_objects(&self) -> usize{
        0
    }
    pub fn capacity(&self) -> usize{
        0
    }
    pub fn total_capacity(&self) -> usize{
        0
    }
    pub fn committed_memory(&self) -> usize{
        0
    }
    pub fn maximum_committed_memory(&self) -> usize{
        0
    }
    pub fn committed_physical_memory(&self) -> usize{
        0
    }
    pub fn available(&self) -> usize{
        0
    }
    pub fn external_backing_store_bytes(&mut self, type_: i32) -> usize{
        0
    }
    pub fn allocated_since_last_gc(&self) -> usize{
        0
    }
    pub fn minimum_capacity(&self) -> usize{
        0
    }
    pub fn maximum_capacity(&self) -> usize{
        0
    }
    pub fn first_allocatable_address(&self) -> Address{
        Address {}
    }
    pub fn verify(&mut self, isolate: *mut Isolate, visitor: *mut SpaceVerificationVisitor){

    }
    pub fn print(&mut self){

    }
    pub fn first_page(&mut self) -> *mut PageMetadata{
        
