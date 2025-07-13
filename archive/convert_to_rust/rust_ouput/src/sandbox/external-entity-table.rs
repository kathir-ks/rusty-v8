// Converted from V8 C++ source files:
// Header: external-entity-table.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod atomicops {
        pub struct Atomic<T> {
            value: std::sync::atomic::AtomicU64,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> Atomic<T> {
            pub fn new(_value: T) -> Self {
                Atomic {
                    value: std::sync::atomic::AtomicU64::new(0),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn load(&self) -> T {
                unsafe { std::mem::transmute(self.value.load(std::sync::atomic::Ordering::Relaxed)) }
            }

            pub fn store(&self, value: T) {
                let value_as_u64: u64 = unsafe { std::mem::transmute(value) };
                self.value.store(value_as_u64, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
    pub mod memory {
        pub fn aligned_malloc(size: usize, alignment: usize) -> *mut std::ffi::c_void {
            unsafe {
                let mut result: *mut std::ffi::c_void = std::mem::MaybeUninit::uninit().assume_init();
                if alignment == 0 {
                    result = libc::malloc(size) as *mut std::ffi::c_void;
                } else {
                    libc::posix_memalign(&mut result, alignment, size);
                }
                result
            }
        }

        pub fn aligned_free(ptr: *mut std::ffi::c_void) {
            unsafe {
                libc::free(ptr);
            }
        }
    }
    pub mod platform {
        use std::sync::{Mutex, MutexGuard, PoisonError};

        pub struct MutexWrapper(Mutex<()>);

        impl MutexWrapper {
            pub const fn new() -> Self {
                MutexWrapper(Mutex::new(()))
            }

            pub fn lock(&self) -> Result<MutexGuard<()>, PoisonError<MutexGuard<()>>> {
                self.0.lock()
            }
        }
    }
}

pub mod common {
pub mod code_memory_access {}
pub mod globals {
        pub enum GarbageCollectionType {
            kMinor,
            kMajor,
        }
    }
    pub mod segmented_table {
        use std::collections::HashSet;
        use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
        use std::sync::{Mutex, MutexGuard, PoisonError};

        pub struct SegmentedTable<Entry, const SIZE: usize> {
            spaces: Vec<Space<Entry, SIZE>>,
            internal_read_only_space: Space<Entry, SIZE>,
            first_segment: Option<Box<Segment<Entry, SIZE>>>,
        }

        impl<Entry, const SIZE: usize> SegmentedTable<Entry, SIZE> {
            pub fn new() -> Self {
                SegmentedTable {
                    spaces: Vec::new(),
                    internal_read_only_space: Space::new(),
                    first_segment: None,
                }
            }
        }
        impl<Entry, const SIZE: usize> Default for SegmentedTable<Entry, SIZE> {
            fn default() -> Self {
                Self::new()
            }
        }
        #[derive(Debug)]
        pub struct FreelistHead {
            index: AtomicU32,
            length: AtomicU32,
        }

        impl FreelistHead {
            pub const fn new() -> Self {
                FreelistHead {
                    index: AtomicU32::new(0),
                    length: AtomicU32::new(0),
                }
            }

            pub const fn with_values(index: u32, length: u32) -> Self {
                FreelistHead {
                    index: AtomicU32::new(index),
                    length: AtomicU32::new(length),
                }
            }
            pub fn index(&self) -> u32 {
                self.index.load(Ordering::Relaxed)
            }
            pub fn set_index(&self, index: u32) {
                self.index.store(index, Ordering::Relaxed);
            }
            pub fn length(&self) -> u32 {
                self.length.load(Ordering::Relaxed)
            }
            pub fn set_length(&self, length: u32) {
                self.length.store(length, Ordering::Relaxed);
            }
        }
        #[derive(Debug)]
        pub struct Segment<Entry, const SIZE: usize> {
            entries: Vec<Entry>,
            start_index: u32,
            is_read_only: bool,
        }

        impl<Entry, const SIZE: usize> Segment<Entry, SIZE> {
            pub fn new(start_index: u32) -> Self
            where
                Entry: Default + Clone,
            {
                Segment {
                    entries: vec![Entry::default(); Self::kEntriesPerSegment],
                    start_index,
                    is_read_only: false,
                }
            }
            pub fn start_index(&self) -> u32 {
                self.start_index
            }
            pub fn is_read_only(&self) -> bool {
                self.is_read_only
            }
            pub fn set_is_read_only(&mut self, is_read_only: bool) {
                self.is_read_only = is_read_only;
            }

            pub fn get_entry(&self, index: u32) -> &Entry {
                &self.entries[(index - self.start_index) as usize]
            }

            pub fn get_entry_mut(&mut self, index: u32) -> &mut Entry {
                &mut self.entries[(index - self.start_index) as usize]
            }
        }

        pub struct Space<Entry, const SIZE: usize> {
            freelist_head: AtomicU64, // Combined index and length
            segments: Mutex<HashSet<u32>>, // Store start indices of segments
            is_internal_read_only_space: bool,
            owning_table: AtomicU64,
        }

        impl<Entry, const SIZE: usize> Space<Entry, SIZE> {
            pub fn new() -> Self {
                Space {
                    freelist_head: AtomicU64::new(0),
                    segments: Mutex::new(HashSet::new()),
                    is_internal_read_only_space: false,
                    owning_table: AtomicU64::new(0),
                }
            }
        }
        impl<Entry, const SIZE: usize> Default for Space<Entry, SIZE> {
            fn default() -> Self {
                Self::new()
            }
        }
        pub struct WriteIterator {}

        impl<Entry, const SIZE: usize> SegmentedTable<Entry, SIZE> {
            pub const kSegmentSize: usize = 64 * 1024;
            pub const kEntrySize: usize = std::mem::size_of::<Entry>();
            pub const kEntriesPerSegment: usize = Self::kSegmentSize / Self::kEntrySize;
        }
        impl<Entry, const SIZE: usize> Segment<Entry, SIZE> {
            pub const kSegmentSize: usize = 64 * 1024;
            pub const kEntrySize: usize = std::mem::size_of::<Entry>();
            pub const kEntriesPerSegment: usize = Self::kSegmentSize / Self::kEntrySize;
        }
        impl<Entry, const SIZE: usize> Space<Entry, SIZE> {
            pub const kSegmentSize: usize = 64 * 1024;
            pub const kEntrySize: usize = std::mem::size_of::<Entry>();
            pub const kEntriesPerSegment: usize = Self::kSegmentSize / Self::kEntrySize;
        }
    }
}
pub mod sandbox {
    use crate::base::atomicops::Atomic;
    use crate::base::platform::MutexWrapper;
    use crate::common::segmented_table::{FreelistHead, Segment, SegmentedTable, Space};
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use std::mem::size_of;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
    use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub struct Isolate {}

    pub struct ExternalEntityTable<Entry, const SIZE: usize> {
        base: SegmentedTable<Entry, SIZE>,
        spaces: Vec<Space<Entry, SIZE>>,
        internal_read_only_space: Space<Entry, SIZE>,
        first_segment: Option<Box<Segment<Entry, SIZE>>>,
        read_only_segment_unsealed: AtomicBool,
    }

    impl<Entry, const SIZE: usize> ExternalEntityTable<Entry, SIZE>
    where
        Entry: Default + Clone + IsMarked + Unmark,
    {
        const kInternalReadOnlySegmentOffset: u32 = 0;
        const kInternalNullEntryIndex: u32 = 0;
        const kEndOfInternalReadOnlySegment: u32 = Segment::<Entry, SIZE>::kEntriesPerSegment as u32;
        pub const kSupportsCompaction: bool = false;
        const kEntryAllocationIsForbiddenMarker: FreelistHead = FreelistHead::with_values(u32::MAX, u32::MAX);
        pub fn new() -> Self {
            ExternalEntityTable {
                base: SegmentedTable::new(),
                spaces: Vec::new(),
                internal_read_only_space: Space::new(),
                first_segment: None,
                read_only_segment_unsealed: AtomicBool::new(false),
            }
        }

        pub fn initialize(&mut self) {
            self.InitializeSpace(&mut self.internal_read_only_space);
            let mut segment = Box::new(Segment::new(Self::kInternalReadOnlySegmentOffset));
            segment.set_is_read_only(true);
            self.first_segment = Some(segment);
            self.AttachSpaceToReadOnlySegment(&mut self.internal_read_only_space);
        }

        pub fn tear_down(&mut self) {
            for space in &mut self.spaces {
                self.TearDownSpace(space);
            }
            self.DetachSpaceFromReadOnlySegment(&mut self.internal_read_only_space);
        }

        pub fn initialize_space(&mut self, space: &mut Space<Entry, SIZE>) {
            let owning_table_ptr = self as *mut Self as u64;
            space.owning_table.store(owning_table_ptr, Ordering::Relaxed);
            let initial_index = 1;
            let initial_length = Segment::<Entry, SIZE>::kEntriesPerSegment as u32 - 1;
            space.freelist_head.store(
                ((initial_index as u64) << 32) | (initial_length as u64),
                Ordering::Relaxed,
            );
            self.spaces.push(Space {
                freelist_head: AtomicU64::new(((initial_index as u64) << 32) | (initial_length as u64)),
                segments: Mutex::new(HashSet::new()),
                is_internal_read_only_space: false,
                owning_table: AtomicU64::new(owning_table_ptr),
            });
        }

        pub fn tear_down_space(&mut self, space: &mut Space<Entry, SIZE>) {
            let mut segments = space.segments.lock().unwrap();
            for segment_start_index in segments.drain() {
                // Find the segment in the table and deallocate it.
                if let Some(segment) = self
                    .first_segment
                    .as_mut()
                    .filter(|s| s.start_index() == segment_start_index)
                {
                    // Deallocate the segment's memory.
                    // Assuming the segment was allocated using Box::new
                    self.first_segment = None;
                }
            }
        }

        pub fn attach_space_to_read_only_segment(&mut self, space: &mut Space<Entry, SIZE>) {
            space.is_internal_read_only_space = true;
            let mut segments = space.segments.lock().unwrap();
            if let Some(segment) = &self.first_segment {
                segments.insert(segment.start_index());
            }
        }

        pub fn detach_space_from_read_only_segment(&mut self, space: &mut Space<Entry, SIZE>) {
            space.is_internal_read_only_space = false;
            let mut segments = space.segments.lock().unwrap();
            if let Some(segment) = &self.first_segment {
                segments.remove(&segment.start_index());
            }
        }
        fn allocate_entry(&mut self, space: &mut Space<Entry, SIZE>) -> u32 {
            if let Some(index) = self.TryAllocateEntry(space) {
                return index;
            }
            let mut freelist_head = self.TryExtend(space);
            if freelist_head.is_none() {
                return 0;
            }
            self.TryAllocateEntry(space).unwrap()
        }
        fn TryAllocateEntry(&mut self, space: &mut Space<Entry, SIZE>) -> Option<u32> {
            let current_head = space.freelist_head.load(Ordering::Relaxed);
            let index = (current_head >> 32) as u32;
            let length = (current_head & 0xFFFFFFFF) as u32;
            if length == 0 {
                return None;
            }

            let new_index = index + 1;
            let new_length = length - 1;
            let new_head = ((new_index as u64) << 32) | (new_length as u64);

            if space
                .freelist_head
                .compare_exchange(current_head, new_head, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                Some(index)
            } else {
                None
            }
        }
        fn TryExtend(&mut self, space: &mut Space<Entry, SIZE>) -> Option<FreelistHead> {
            let mut next_index = 0;
            {
                let segment_size = Segment::<Entry, SIZE>::kSegmentSize;
                let entry_size = Segment::<Entry, SIZE>::kEntrySize;
                let entries_per_segment = segment_size / entry_size;
                let new_segment = Segment::<Entry, SIZE>::new(
                    (space.segments.lock().unwrap().len() * entries_per_segment) as u32,
                );

                next_index = new_segment.start_index;

                if space.segments.lock().unwrap().len() >= SIZE {
                    return None;
                }
                space.segments.lock().unwrap().insert(next_index);
                self.first_segment = Some(Box::new(new_segment));
            }

            Some(FreelistHead::with_values(
                next_index,
                Segment::<Entry, SIZE>::kEntriesPerSegment as u32,
            ))
        }

        fn allocate_entry_below(&mut self, space: &mut Space<Entry, SIZE>, threshold_index: u32) -> u32 {
            let current_head = space.freelist_head.load(Ordering::Relaxed);
            let index = (current_head >> 32) as u32;
            let length = (current_head & 0xFFFFFFFF) as u32;

            if index >= threshold_index {
                return 0;
            }

            if length == 0 {
                return 0;
            }

            let new_index = index + 1;
            let new_length = length - 1;
            let new_head = ((new_index as u64) << 32) | (new_length as u64);

            if space
                .freelist_head
                .compare_exchange(current_head, new_head, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return index;
            } else {
                0
            }
        }

        fn try_allocate_entry_from_freelist(&mut self, space: &mut Space<Entry, SIZE>, freelist: FreelistHead) -> bool {
            let current_head = space.freelist_head.load(Ordering::Relaxed);
            if current_head != ((freelist.index() as u64) << 32) | (freelist.length() as u64) {
                return false;
            }

            let new_index = freelist.index() + 1;
            let new_length = freelist.length() - 1;
            let new_head = ((new_index as u64) << 32) | (new_length as u64);

            space
                .freelist_head
                .compare_exchange(current_head, new_head, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        }

        fn generic_sweep(&mut self, space: &mut Space<Entry, SIZE>) -> u32 {
            let mut live_entries = 0;
            let mut current_index = 0;
            if let Some(segment) = &mut self.first_segment {
                while current_index < Segment::<Entry, SIZE>::kEntriesPerSegment as u32 {
                    let entry = segment.get_entry_mut(current_index);
                    if entry.is_marked() {
                        entry.unmark();
                        live_entries += 1;
                    } else {
                        // Free the entry by adding it to the freelist.
                        // This is a simplified implementation; a real implementation
                        // would need to handle concurrency and ensure the freelist
                        // remains consistent.
                        //space.freelist.push(current_index);
                    }
                    current_index += 1;
                }
            }
            live_entries
        }

        fn generic_sweep_with_callback<Callback>(&mut self, space: &mut Space<Entry, SIZE>, mut marked: Callback) -> u32
        where
            Callback: FnMut(u32),
        {
            let mut live_entries = 0;
            let mut current_index = 0;
            if let Some(segment) = &mut self.first_segment {
                while current_index < Segment::<Entry, SIZE>::kEntriesPerSegment as u32 {
                    let entry = segment.get_entry_mut(current_index);
                    if entry.is_marked() {
                        entry.unmark();
                        live_entries += 1;
                        marked(current_index);
                    } else {
                        // Free the entry by adding it to the freelist.
                        // This is a simplified implementation; a real implementation
                        // would need to handle concurrency and ensure the freelist
                        // remains consistent.
                        //space.freelist.push(current_index);
                    }
                    current_index += 1;
                }
            }

            live_entries
        }

        fn iterate_entries_in<Callback>(&self, space: &Space<Entry, SIZE>, mut callback: Callback)
        where
            Callback: FnMut(u32),
        {
            if let Some(segment) = &self.first_segment {
                for i in 0..Segment::<Entry, SIZE>::kEntriesPerSegment as u32 {
                    callback(segment.start_index() + i);
                }
            }
        }
        fn unseal_read_only_segment(&mut self) {
            if let Some(segment) = &mut self.first_segment {
                segment.set_is_read_only(false);
            }
        }
        fn seal_read_only_segment(&mut self) {
            if let Some(segment) = &mut self.first_segment {
                segment.set_is_read_only(true);
            }
        }
        fn extend(&mut self, space: &mut Space<Entry, SIZE>, segment: Segment<Entry, SIZE>, freelist: FreelistHead) {
            space.segments.lock().unwrap().insert(segment.start_index());
            self.first_segment = Some(Box::new(segment));
        }
        pub fn UnsealReadOnlySegment(&mut self) {
            self.unseal_read_only_segment();
        }
        pub fn SealReadOnlySegment(&mut self) {
            self.seal_read_only_segment();
        }
        pub fn AttachSpaceToReadOnlySegment(&mut self, space: &mut Space<Entry, SIZE>) {
            self.attach_space_to_read_only_segment(space);
        }
        pub fn DetachSpaceFromReadOnlySegment(&mut self, space: &mut Space<Entry, SIZE>) {
            self.detach_space_from_read_only_segment(space);
        }
    }

    pub trait IsMarked {
        fn is_marked(&self) -> bool;
    }

    pub trait Unmark {
        fn unmark(&mut self);
    }

    impl<Entry, const SIZE: usize> Space<Entry, SIZE> {
        pub fn freelist_length(&self) -> u32 {
            (self.freelist_head.load(Ordering::Relaxed) & 0xFFFFFFFF) as u32
        }

        pub fn num_segments(&self) -> u32 {
            self.segments.lock().unwrap().len() as u32
        }

        pub fn is_empty(&self) -> bool {
            self.num_segments() == 0
        }

        pub fn capacity(&self) -> u32 {
            self.num_segments() * Segment::<Entry, SIZE>::kEntriesPerSegment as u32
        }

        pub fn contains(&self, index: u32) -> bool {
            self.segments
                .lock()
                .unwrap()
                .iter()
                .any(|&segment_start_index| {
                    index >= segment_start_index
                        && index < segment_start_index + Segment::<Entry, SIZE>::kEntriesPerSegment as u32
                })
        }

        pub fn is_internal_read_only_space(&self) -> bool {
            self.is_internal_read_only_space
        }

        #[cfg(debug_assertions)]
        pub fn belongs_to(&self, table: *const std::ffi::c_void) -> bool {
            self.owning_table.load(Ordering::Relaxed) == table as u64
        }

        pub fn num_segments_for_testing(&self) -> u32 {
            self.segments.lock().unwrap().len() as u32
        }
    }

    impl<Entry, const SIZE: usize> Drop for Space<Entry, SIZE> {
        fn drop(&mut self) {
        }
    }

    pub struct SpaceWithBlackAllocationSupport<Entry, const SIZE: usize> {
        space: Space<Entry, SIZE>,
        allocate_black: AtomicBool,
    }

    impl<Entry, const SIZE: usize> SpaceWithBlackAllocationSupport<Entry, SIZE> {
        pub fn new() -> Self {
            SpaceWithBlackAllocationSupport {
                space: Space::new(),
                allocate_black: AtomicBool::new(false),
            }
        }

        pub fn allocate_black(&self) -> bool {
            self.allocate_black.load(Ordering::Relaxed)
        }

        pub fn set_allocate_black(&self, allocate_black: bool) {
            self.allocate_black.store(allocate_black, Ordering::Relaxed);
        }
    }
    impl<Entry, const SIZE: usize> Default for SpaceWithBlackAllocationSupport<Entry, SIZE> {
        fn default() -> Self {
            Self::new()
        }
    }
}
