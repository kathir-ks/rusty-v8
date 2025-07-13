// Converted from V8 C++ source files:
// Header: remembered-set.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Worklist<T, const SIZE: usize> {
        _dummy: i32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const SIZE: usize> Worklist<T, SIZE> {
        pub struct Local {
            _dummy: i32,
        }
        pub fn new() -> Self {
            Self {
                _dummy: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn Push(&mut self, _item: T) {}
    }
}

pub mod heap {
    pub struct Heap {}
    pub mod base {
        pub struct Worklist<T, const SIZE: usize> {
            _dummy: i32,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const SIZE: usize> Worklist<T, SIZE> {
            pub struct Local {
                _dummy: i32,
            }
            pub fn new() -> Self {
                Self {
                    _dummy: 0,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn Push(&mut self, _item: T) {}
        }
    }
}

pub mod internal {
    use std::sync::Mutex;
    use std::sync::Arc;
    pub use super::SlotCallbackResult;
    pub struct HeapObject {}
    pub struct Address {
        address: usize,
    }

    impl Address {
        pub fn new(address: usize) -> Self {
            Address { address }
        }
    }

    pub struct MutablePageMetadata {
        chunk: MemoryChunk,
        slot_sets: Mutex<[Option<Box<SlotSet>>; 3]>, // Assuming 3 different remembered set types
        typed_slot_sets: Mutex<[Option<Box<TypedSlotSet>>; 3]>, // Assuming 3 different remembered set types
        possibly_empty_buckets: PossiblyEmptyBuckets,
    }

    impl MutablePageMetadata {
        pub fn new(chunk: MemoryChunk) -> Self {
            MutablePageMetadata {
                chunk: chunk,
                slot_sets: Mutex::new([None, None, None]),
                typed_slot_sets: Mutex::new([None, None, None]),
                possibly_empty_buckets: PossiblyEmptyBuckets::new(),
            }
        }

        pub fn ChunkAddress(&self) -> Address {
            self.chunk.address
        }

        pub fn BucketsInSlotSet(&self) -> usize {
            1024 // A reasonable default
        }

        pub fn Chunk(&self) -> &MemoryChunk {
            &self.chunk
        }

        pub fn Offset(&self, slot_addr: Address) -> usize {
            slot_addr.address - self.chunk.address.address
        }

        pub fn OffsetMaybeOutOfRange(&self, slot_addr: Address) -> usize {
            slot_addr.address - self.chunk.address.address
        }

        pub fn Contains(&self, slot_addr: Address) -> bool {
            slot_addr.address >= self.chunk.address.address && slot_addr.address < self.chunk.address.address + 4096
        }

        pub fn AllocateSlotSet(&mut self, _type: RememberedSetType) -> *mut SlotSet {
            let mut slot_sets = self.slot_sets.lock().unwrap();
            let index = match _type {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2, // Assuming same index as OLD_TO_OLD
            };
            slot_sets[index] = Some(Box::new(SlotSet::new()));
            slot_sets[index].as_mut().map(|x| &mut **x).unwrap() as *mut SlotSet
        }

        pub fn ReleaseSlotSet(&mut self, _type: RememberedSetType) {
            let mut slot_sets = self.slot_sets.lock().unwrap();
            let index = match _type {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2, // Assuming same index as OLD_TO_OLD
            };
            slot_sets[index] = None;
        }

        pub fn slot_set<const TYPE: RememberedSetType>(&self) -> *mut SlotSet {
            self.slot_set_generic::<TYPE, AccessMode::NON_ATOMIC>()
        }

        pub fn slot_set_generic<const TYPE: RememberedSetType, const MODE: AccessMode>(&self) -> *mut SlotSet {
            let slot_sets = self.slot_sets.lock().unwrap();
            let index = match TYPE {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2,
            };

            slot_sets[index].as_ref().map(|x| &mut **x as *mut SlotSet).unwrap_or(std::ptr::null_mut())
        }

        pub fn set_slot_set<const TYPE: RememberedSetType, const MODE: AccessMode>(&mut self, other_slot_set: &SlotSet) {
            let mut slot_sets = self.slot_sets.lock().unwrap();
            let index = match TYPE {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2,
            };
            slot_sets[index] = Some(Box::new(SlotSet::clone(other_slot_set)));
        }

        pub fn AllocateTypedSlotSet(&mut self, _type: RememberedSetType) -> *mut TypedSlotSet {
            let mut typed_slot_sets = self.typed_slot_sets.lock().unwrap();
            let index = match _type {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2, // Assuming same index as OLD_TO_OLD
            };
            typed_slot_sets[index] = Some(Box::new(TypedSlotSet::new()));
            typed_slot_sets[index].as_mut().map(|x| &mut **x).unwrap() as *mut TypedSlotSet
        }

        pub fn typed_slot_set<const TYPE: RememberedSetType>(&self) -> *mut TypedSlotSet {
            self.typed_slot_set_generic::<TYPE, AccessMode::NON_ATOMIC>()
        }

        pub fn typed_slot_set_generic<const TYPE: RememberedSetType, const MODE: AccessMode>(&self) -> *mut TypedSlotSet {
            let typed_slot_sets = self.typed_slot_sets.lock().unwrap();
             let index = match TYPE {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2,
            };
            typed_slot_sets[index].as_ref().map(|x| &mut **x as *mut TypedSlotSet).unwrap_or(std::ptr::null_mut())
        }
        pub fn set_typed_slot_set<const TYPE: RememberedSetType, const MODE: AccessMode>(&mut self, other_typed_slot_set: &TypedSlotSet) {
            let mut typed_slot_sets = self.typed_slot_sets.lock().unwrap();
            let index = match TYPE {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2,
            };
            typed_slot_sets[index] = Some(Box::new(TypedSlotSet::clone(other_typed_slot_set)));
        }

        pub fn ReleaseTypedSlotSet(&mut self, _type: RememberedSetType) {
            let mut typed_slot_sets = self.typed_slot_sets.lock().unwrap();
             let index = match _type {
                RememberedSetType::OLD_TO_NEW => 0,
                RememberedSetType::OLD_TO_NEW_BACKGROUND => 1,
                RememberedSetType::OLD_TO_OLD => 2,
                RememberedSetType::TRUSTED_TO_CODE => 2, // Assuming same index as OLD_TO_OLD
            };
            typed_slot_sets[index] = None;
        }

         pub fn possibly_empty_buckets(&mut self) -> &mut PossiblyEmptyBuckets {
            &mut self.possibly_empty_buckets
        }
    }

    #[derive(Clone)]
    pub struct MemoryChunk {
        address: Address,
    }
    impl MemoryChunk{
        pub fn Offset(&self, addr: Address) -> usize{
            addr.address - self.address.address
        }
        pub fn address(&self) -> Address {
            self.address
        }
    }

    pub struct OldGenerationMemoryChunkIterator<'a> {
        heap: &'a Heap,
        current_chunk: *mut MutablePageMetadata, // Replace with appropriate iterator logic
    }

    impl<'a> OldGenerationMemoryChunkIterator<'a> {
        pub fn new(heap: &'a Heap) -> Self {
            OldGenerationMemoryChunkIterator {
                heap,
                current_chunk: std::ptr::null_mut(), // Replace with actual initialization logic
            }
        }

        pub fn next(&mut self) -> *mut MutablePageMetadata {
            // Replace with actual iteration logic based on heap structure
            if self.current_chunk.is_null() {
               std::ptr::null_mut()
            } else {
                let next_chunk = std::ptr::null_mut(); // replace with actual next chunk
                let current = self.current_chunk;
                self.current_chunk = next_chunk;
                current
            }
        }
    }

    pub struct SlotSet {
        slots: Vec<usize>,
    }
    impl SlotSet {
        pub fn new() -> Self {
            SlotSet { slots: Vec::new() }
        }
        pub fn clone(other: &SlotSet) -> Self {
            SlotSet {
                slots: other.slots.clone(),
            }
        }
        pub fn Insert<const MODE: SlotSet::AccessMode>(&mut self, slot_offset: usize) {
            self.slots.push(slot_offset);
        }

        pub fn Iterate<const MODE: AccessMode>(
            &mut self,
            chunk_address: Address,
            start_bucket: usize,
            end_bucket: usize,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            mode: SlotSet::EmptyBucketMode,
        ) -> i32 {
            let mut slots = 0;
            for &slot_offset in &self.slots {
                if slot_offset >= start_bucket as usize && slot_offset <= end_bucket as usize{
                    let slot_address = Address::new(chunk_address.address + slot_offset);
                    let maybe_object_slot = MaybeObjectSlot { address: slot_address };
                    let result = callback(maybe_object_slot);
                    if result == SlotCallbackResult::KEEP_SLOT {
                        slots += 1;
                    }
                }
            }
            slots
        }

        pub fn Remove(&mut self, offset: usize) {
            self.slots.retain(|&x| x != offset);
        }

        pub fn RemoveRange(&mut self, start_offset: i32, end_offset: i32, buckets_in_slot_set: usize, mode: SlotSet::EmptyBucketMode) {
            self.slots.retain(|&x| (x as i32) < start_offset || (x as i32) > end_offset);
        }

        pub fn BucketForSlot(offset: usize) -> usize {
            offset / 8 // Example bucket size
        }
        pub fn address(&self) -> Address{
           Address{address: 0}
        }
        pub fn Contains(&self, offset: usize) -> bool{
            self.slots.contains(&offset)
        }
        pub fn Merge(&mut self, other: &SlotSet, buckets_in_slot_set: usize) {
            self.slots.extend(other.slots.iter());
        }

         pub fn IterateAndTrackEmptyBuckets(
            &mut self,
            chunk_address: Address,
            start_bucket: usize,
            end_bucket: usize,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            possibly_empty_buckets: &mut PossiblyEmptyBuckets,
        ) -> i32 {
            let mut slots = 0;
            for &slot_offset in &self.slots {
                 if slot_offset >= start_bucket as usize && slot_offset <= end_bucket as usize{
                    let slot_address = Address::new(chunk_address.address + slot_offset);
                    let maybe_object_slot = MaybeObjectSlot { address: slot_address };
                    let result = callback(maybe_object_slot);
                    if result == SlotCallbackResult::KEEP_SLOT {
                        slots += 1;
                    }
                 }
            }
            slots
        }

         pub fn CheckPossiblyEmptyBuckets(&mut self, buckets_in_slot_set: usize, possibly_empty_buckets: &mut PossiblyEmptyBuckets) -> bool{
            true
         }
    }

    pub struct TypedSlotSet {
        typed_slots: Vec<(SlotType, u32)>,
    }
    impl TypedSlotSet {
        pub fn new() -> Self {
            TypedSlotSet {
                typed_slots: Vec::new(),
            }
        }
        pub fn clone(other: &TypedSlotSet) -> Self {
             TypedSlotSet {
                typed_slots: other.typed_slots.clone(),
            }
        }
        pub fn Insert(&mut self, slot_type: SlotType, offset: u32) {
            self.typed_slots.push((slot_type, offset));
        }
        pub fn Iterate<Callback>(&mut self, callback: Callback, mode: TypedSlotSet::EmptyBucketMode) -> i32
        where
            Callback: Fn(SlotType, Address) -> SlotCallbackResult,
        {
            let mut count = 0;
            self.typed_slots.retain(|&(slot_type, offset)| {
                let address = Address::new(offset as usize);
                let result = callback(slot_type, address);
                if result == SlotCallbackResult::REMOVE_SLOT {
                    false
                } else {
                    count += 1;
                    true
                }
            });
            count
        }
         pub fn Merge(&mut self, other: &TypedSlots) {
            self.typed_slots.extend(other.slots.iter().cloned());
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum SlotType {
        CodeTarget,
        EmbeddedObject,
        Other,
    }

    #[derive(PartialEq, Eq)]
    pub enum SlotCallbackResult {
        KEEP_SLOT,
        REMOVE_SLOT,
    }

    pub struct MaybeObjectSlot {
        address: Address,
    }

    impl MaybeObjectSlot {
        pub fn address(&self) -> Address {
            self.address
        }
    }

    pub struct FullMaybeObjectSlot(*mut HeapObject);

    #[derive(Clone)]
    pub struct TypedSlots {
        slots: Vec<(SlotType, u32)>,
    }
    pub struct PossiblyEmptyBuckets{
        is_empty: bool,
    }

    impl PossiblyEmptyBuckets{
        pub fn new() -> Self {
            Self{
                is_empty: false,
            }
        }
        pub fn IsEmpty(&self) -> bool{
            self.is_empty
        }
    }
    pub enum AccessMode {
        ATOMIC,
        NON_ATOMIC,
    }

    pub enum class AccessMode {
        ATOMIC,
        NON_ATOMIC,
    }

    pub mod base {
        pub struct Worklist<T, const SIZE: usize> {
            _dummy: i32,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const SIZE: usize> Worklist<T, SIZE> {
            pub struct Local {
                _dummy: i32,
            }
            pub fn new() -> Self {
                Self {
                    _dummy: 0,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn Push(&mut self, _item: T) {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RememberedSetType {
    OLD_TO_NEW,
    OLD_TO_NEW_BACKGROUND,
    OLD_TO_OLD,
    TRUSTED_TO_CODE
}

pub mod memory_chunk_layout {
}

pub mod paged_spaces {
}

pub mod spaces {
}

pub mod codegen {
    pub struct RelocInfo {}
    impl RelocInfo {
        pub fn IsCodeTargetMode(_rmode: i32) -> bool {
            true
        }
        pub fn IsEmbeddedObjectMode(_rmode: i32) -> bool {
            true
        }
        pub fn target_address(&self) -> internal::Address {
            internal::Address{address: 0}
        }
        pub fn set_target_address(&mut self, _address: internal::Address){}
        pub fn target_object(&self, _isolate: i32) -> internal::HeapObject{
            internal::HeapObject{}
        }
        pub fn set_target_object(&mut self, _object: internal::HeapObject){}
        pub fn rmode(&self) -> i32 {0}
    }

}

pub mod heap {
    use std::sync::Mutex;
    use std::sync::Arc;

    use super::internal::{
        Address, HeapObject, MemoryChunk, MutablePageMetadata, OldGenerationMemoryChunkIterator,
        SlotCallbackResult, SlotSet, SlotType, TypedSlotSet, MaybeObjectSlot
    };
    use super::{RememberedSetType};

    pub struct RememberedSetOperations {}

    impl RememberedSetOperations {
        pub fn Insert<const ACCESS_MODE: i32>(slot_set: *mut SlotSet, slot_offset: usize) {
            if !slot_set.is_null(){
                unsafe{
                   (*slot_set).Insert::<{if ACCESS_MODE == 0 {SlotSet::AccessMode::ATOMIC} else {SlotSet::AccessMode::NON_ATOMIC}}>(slot_offset);
                }
            }
        }

        pub fn Iterate<const ACCESS_MODE: i32>(
            slot_set: *mut SlotSet,
            chunk: *const MutablePageMetadata,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            mode: SlotSet::EmptyBucketMode,
        ) -> i32 {
            if slot_set.is_null(){
                return 0;
            }
            let chunk_ref: &MutablePageMetadata = unsafe { &*chunk };
            unsafe {
                (*slot_set).Iterate::<{if ACCESS_MODE == 0 {internal::AccessMode::ATOMIC as i32} else {internal::AccessMode::NON_ATOMIC as i32}}>(
                    chunk_ref.ChunkAddress(),
                    0,
                    chunk_ref.BucketsInSlotSet(),
                    callback,
                    mode,
                )
            }
        }

        pub fn Remove(slot_set: *mut SlotSet, chunk: *mut MutablePageMetadata, slot_addr: Address) {
             if slot_set.is_null(){
                return;
            }
            let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            unsafe {
                let offset = chunk_ref.Offset(slot_addr);
                (*slot_set).Remove(offset);
            }
        }

        pub fn RemoveRange(
            slot_set: *mut SlotSet,
            page: *mut MutablePageMetadata,
            start: Address,
            end: Address,
            mode: SlotSet::EmptyBucketMode,
        ) {
             if slot_set.is_null(){
                return;
            }
            let page_ref: &mut MutablePageMetadata = unsafe { &mut *page };
            let chunk = page_ref.Chunk();
            let start_offset = chunk.Offset(start);
            let end_offset = chunk.OffsetMaybeOutOfRange(end);
            assert!(start_offset <= end_offset);

            unsafe {
                (*slot_set).RemoveRange(
                    start_offset as i32,
                    end_offset as i32,
                    page_ref.BucketsInSlotSet(),
                    mode,
                );
            }
        }

        pub fn CheckNoneInRange(slot_set: *mut SlotSet, chunk: *mut MemoryChunk, start: Address, end: Address) {
             if slot_set.is_null(){
                return;
            }
            let chunk_ref: &mut MemoryChunk = unsafe { &mut *chunk };
            let start_bucket = SlotSet::BucketForSlot(chunk_ref.Offset(start));
            let end_bucket = SlotSet::BucketForSlot(chunk_ref.OffsetMaybeOutOfRange(end) - 8) + 1;
            unsafe {
                (*slot_set).Iterate(
                    chunk_ref.address(),
                    start_bucket,
                    end_bucket,
                    |slot: MaybeObjectSlot| -> SlotCallbackResult {
                        if slot.address().address() < start.address() || slot.address().address() >= end.address() {
                            SlotCallbackResult::KEEP_SLOT
                        } else {
                            panic!("Check failed: slot in range")
                        }
                    },
                    SlotSet::EmptyBucketMode::KEEP_EMPTY_BUCKETS,
                );
            }
        }
    }

    pub struct RememberedSet {}

    impl RememberedSet {
        pub fn Insert<const TYPE: RememberedSetType, const ACCESS_MODE: i32>(page: *mut MutablePageMetadata, slot_offset: usize) {
            let page_ref: &mut MutablePageMetadata = unsafe { &mut *page };
            let slot_set_ptr = page_ref.slot_set_generic::<TYPE, {if ACCESS_MODE == 0 {super::internal::AccessMode::ATOMIC} else {super::internal::AccessMode::NON_ATOMIC}}>();
             let slot_set = if slot_set_ptr.is_null() {
                page_ref.AllocateSlotSet(TYPE)
            } else {
                slot_set_ptr
            };
            RememberedSetOperations::Insert::<ACCESS_MODE>(slot_set, slot_offset);
        }

        pub fn MergeAndDelete<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata, other_slot_set: SlotSet) {
           let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
             if slot_set_ptr.is_null() {
                chunk_ref.set_slot_set::<TYPE, {super::internal::AccessMode::NON_ATOMIC as i32}>(&other_slot_set);
                return;
            }
           unsafe{
                (*slot_set_ptr).Merge(&other_slot_set, chunk_ref.BucketsInSlotSet());
           }
        }
        pub fn MergeAndDeleteTyped<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata, other_typed_slot_set: TypedSlotSet) {
            let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let typed_slot_set_ptr = chunk_ref.typed_slot_set::<TYPE>();
            if typed_slot_set_ptr.is_null() {
                chunk_ref.set_typed_slot_set::<TYPE, {super::internal::AccessMode::NON_ATOMIC as i32}>(&other_typed_slot_set);
                return;
            }

            unsafe{
                (*typed_slot_set_ptr).Merge(&other_typed_slot_set);
            }

        }
        pub fn DeleteTyped(other_typed_slot_set: TypedSlotSet) {
        }

        pub fn Contains<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata, slot_addr: Address) -> bool {
             let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            assert!(chunk_ref.Contains(slot_addr));
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
             if slot_set_ptr.is_null() {
                return false;
            }
            let offset = chunk_ref.Offset(slot_addr);
            unsafe{
                (*slot_set_ptr).Contains(offset)
            }
        }
        pub fn CheckNoneInRange<const TYPE: RememberedSetType>(page: *mut MutablePageMetadata, start: Address, end: Address) {
           let page_ref: &mut MutablePageMetadata = unsafe { &mut *page };
            let slot_set_ptr = page_ref.slot_set::<TYPE>();
             RememberedSetOperations::CheckNoneInRange(slot_set_ptr, page_ref.Chunk() as *mut MemoryChunk, start, end);
        }

        pub fn Remove<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata, slot_addr: Address) {
           let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            assert!(chunk_ref.Contains(slot_addr));
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
             RememberedSetOperations::Remove(slot_set_ptr, chunk as *mut MutablePageMetadata, slot_addr);
        }

        pub fn RemoveRange<const TYPE: RememberedSetType>(
            chunk: *mut MutablePageMetadata,
            start: Address,
            end: Address,
            mode: SlotSet::EmptyBucketMode,
        ) {
            let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
            RememberedSetOperations::RemoveRange(slot_set_ptr, chunk as *mut MutablePageMetadata, start, end, mode);
        }
        pub fn IterateMemoryChunks<const TYPE: RememberedSetType>(heap: *mut super::heap::Heap, callback: impl Fn(*mut MutablePageMetadata)) {
            let heap_ref: &mut super::heap::Heap = unsafe { &mut *heap };
            let mut it = OldGenerationMemoryChunkIterator::new(heap_ref);
            while let Some(chunk) = unsafe { it.next().as_mut() } {
                let slot_set_ptr = chunk.slot_set::<TYPE>();
                let typed_slot_set_ptr = chunk.typed_slot_set::<TYPE>();
                if !slot_set_ptr.is_null() || !typed_slot_set_ptr.is_null() {
                    callback(chunk);
                }
            }
        }

        pub fn Iterate<const TYPE: RememberedSetType, const ACCESS_MODE: i32>(
            chunk: *mut MutablePageMetadata,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            mode: SlotSet::EmptyBucketMode,
        ) -> i32 {
            let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
             Self::IterateGeneric::<TYPE, ACCESS_MODE>(slot_set_ptr, chunk, callback, mode)
        }

        pub fn IterateGeneric<const TYPE: RememberedSetType, const ACCESS_MODE: i32>(
            slot_set: *mut SlotSet,
            chunk: *mut MutablePageMetadata,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            mode: SlotSet::EmptyBucketMode,
        ) -> i32 {
            RememberedSetOperations::Iterate::<ACCESS_MODE>(slot_set, chunk, callback, mode)
        }

        pub fn IterateAndTrackEmptyBuckets<const TYPE: RememberedSetType>(
            chunk: *mut MutablePageMetadata,
            callback: impl Fn(MaybeObjectSlot) -> SlotCallbackResult,
            empty_chunks: &mut super::base::Worklist<*mut MutablePageMetadata, 64>::Local,
        ) -> i32 {
            let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.slot_set::<TYPE>();
            let mut slots = 0;
             if !slot_set_ptr.is_null() {
               let possibly_empty_buckets = chunk_ref.possibly_empty_buckets();
                unsafe {
                    slots += (*slot_set_ptr).IterateAndTrackEmptyBuckets(
                        chunk_ref.ChunkAddress(),
                        0,
                        chunk_ref.BucketsInSlotSet(),
                        callback,
                        possibly_empty_buckets,
                    );
                }

                if !possibly_empty_buckets.IsEmpty() {
                    empty_chunks.Push(chunk);
                }
            }
            slots
        }

        pub fn CheckPossiblyEmptyBuckets<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata) -> bool {
           let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.slot_set_generic::<TYPE, {super::internal::AccessMode::NON_ATOMIC as i32}>();
             if !slot_set_ptr.is_null() {
                unsafe {
                    if (*slot_set_ptr).CheckPossiblyEmptyBuckets(chunk_ref.BucketsInSlotSet(), chunk_ref.possibly_empty_buckets()) {
                        chunk_ref.ReleaseSlotSet(TYPE);
                        return true;
                    }
                }
            }
            false
        }

        pub fn InsertTyped<const TYPE: RememberedSetType>(memory_chunk: *mut MutablePageMetadata, slot_type: SlotType, offset: u32) {
             let memory_chunk_ref: &mut MutablePageMetadata = unsafe { &mut *memory_chunk };
            let slot_set_ptr = memory_chunk_ref.typed_slot_set::<TYPE>();
             let slot_set = if slot_set_ptr.is_null() {
                memory_chunk_ref.AllocateTypedSlotSet(TYPE)
            } else {
                slot_set_ptr
            };
            unsafe{
                (*slot_set).Insert(slot_type, offset);
            }
        }

        pub fn MergeTyped<const TYPE: RememberedSetType>(page: *mut MutablePageMetadata, other: *mut super::internal::TypedSlots) {
            let page_ref: &mut MutablePageMetadata = unsafe { &mut *page };
            let slot_set_ptr = page_ref.typed_slot_set::<TYPE>();
             let slot_set = if slot_set_ptr.is_null() {
                page_ref.AllocateTypedSlotSet(TYPE)
            } else {
                slot_set_ptr
            };
            let other_ref: &mut super::internal::TypedSlots = unsafe { &mut *other };
            unsafe{
                 (*slot_set).Merge(other_ref);
            }
        }

        pub fn RemoveRangeTyped<const TYPE: RememberedSetType>(page: *mut MutablePageMetadata, start: Address, end: Address) {
            let page_ref: &mut MutablePageMetadata = unsafe { &mut *page };
            let slot_set_ptr = page_ref.typed_slot_set::<TYPE>();
             if !slot_set_ptr.is_null() {
                unsafe{
                     (*slot_set_ptr).Iterate(
                        |slot_type: SlotType, slot_addr: Address| -> SlotCallbackResult {
                            if start.address() <= slot_addr.address() && slot_addr.address() < end.address() {
                                SlotCallbackResult::REMOVE_SLOT
                            } else {
                                SlotCallbackResult::KEEP_SLOT
                            }
                        },
                        TypedSlotSet::EmptyBucketMode::FREE_EMPTY_CHUNKS,
                    );
                }
            }
        }

        pub fn IterateTyped<const TYPE: RememberedSetType>(chunk: *mut MutablePageMetadata, callback: impl Fn(SlotType, Address) -> SlotCallbackResult) -> i32 {
           let chunk_ref: &mut MutablePageMetadata = unsafe { &mut *chunk };
            let slot_set_ptr = chunk_ref.typed_slot_set::<TYPE>();
             if slot_set_ptr.is_null() {
                return 0;
            }
            Self::IterateTypedGeneric(slot_set_ptr, callback)
        }

        pub fn IterateTypedGeneric(slot_set: *mut TypedSlotSet, callback: impl Fn(SlotType, Address) -> SlotCallbackResult) -> i32 {
             assert!(!slot_set.is_null());
            unsafe{
                (*slot_set).Iterate(callback, TypedSlotSet::EmptyBucketMode::KEEP_EMPTY_CHUNKS)
            }
        }

        pub fn ClearAll(heap: *mut super::heap::Heap) {
            let heap_ref: &mut super::heap::Heap = unsafe { &mut *heap };
            let mut it = OldGenerationMemoryChunkIterator::new(heap_ref);
            while let Some(chunk) = unsafe { it.next().as_mut() } {
                chunk.ReleaseSlotSet(RememberedSetType::OLD_TO_OLD);
                chunk
