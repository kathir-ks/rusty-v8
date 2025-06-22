// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod external_entity_table {
    use std::cell::UnsafeCell;
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
    use std::sync::{Arc, Mutex, MutexGuard, RwLock};

    /// A thread-safe table with a fixed maximum size for storing references to
    /// objects located outside of the sandbox.
    ///
    /// An external entity table provides the basic mechanisms to ensure
    /// safe access to objects located outside the sandbox, but referenced
    /// from within it. When an external entity table is used, objects located
    /// inside the sandbox reference outside objects through indices into the table.
    ///
    /// The ExternalEntityTable class should be seen an an incomplete class that
    /// needs to be extended by a concrete implementation class, such as the
    /// ExternalPointerTable class, as it is lacking some functionality. In
    /// particular, while the ExternalEntityTable implements basic table memory
    /// management as well as entry allocation routines, it does not implement any
    /// logic for reclaiming entries such as garbage collection. This must be done
    /// by the child classes.
    ///
    /// For the purpose of memory management, the table is partitioned into Segments
    /// (for example 64kb memory chunks) that are grouped together in "Spaces". All
    /// segments in a space share a freelist, and so entry allocation and garbage
    /// collection happen on the level of spaces.
    pub struct ExternalEntityTable<Entry, const SIZE: usize> {
        base: SegmentedTable<Entry, SIZE>,
        _phantom: PhantomData<Entry>,
    }

    impl<Entry, const SIZE: usize> ExternalEntityTable<Entry, SIZE> {
        pub const SUPPORTS_COMPACTION: bool = false;

        pub fn new() -> Self {
            ExternalEntityTable {
                base: SegmentedTable::new(),
                _phantom: PhantomData,
            }
        }

        /// Initializes the table by reserving the backing memory, allocating an
        /// initial segment, and populating the freelist.
        pub fn initialize(&mut self) {
            self.base.initialize();
        }

        /// Deallocates all memory associated with this table.
        pub fn tear_down(&mut self) {
            self.base.tear_down();
        }

        /// Initializes the given space for use with this table.
        pub fn initialize_space(&self, space: &mut Space<Entry, SIZE>) {
            space.owning_table.store(self as *const _ as *mut _, Ordering::Relaxed);
        }

        /// Deallocates all segments owned by the given space.
        pub fn tear_down_space(&self, space: &mut Space<Entry, SIZE>) {
            let mut guard = space.mutex.lock().unwrap();
            for segment in &space.segments {
                self.base.deallocate_segment(*segment);
            }
            space.segments.clear();
        }

        /// Attaches/detaches the given space to the internal read-only segment. Note
        /// the lifetime of the underlying segment itself is managed by the table.
        pub fn attach_space_to_read_only_segment(&mut self, space: &mut Space<Entry, SIZE>) {
            space.is_internal_read_only_space_ = true;
        }
        pub fn detach_space_from_read_only_segment(&mut self, space: &mut Space<Entry, SIZE>) {
            space.is_internal_read_only_space_ = false;
        }

        /// Use this scope to temporarily unseal the read-only segment (i.e. change
        /// permissions to RW).
        pub struct UnsealReadOnlySegmentScope<'a, Entry, const SIZE: usize> {
            table: &'a mut ExternalEntityTable<Entry, SIZE>,
        }

        impl<'a, Entry, const SIZE: usize> UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
            pub fn new(table: &'a mut ExternalEntityTable<Entry, SIZE>) -> Self {
                table.unseal_read_only_segment();
                UnsealReadOnlySegmentScope { table }
            }
        }

        impl<'a, Entry, const SIZE: usize> Drop for UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
            fn drop(&mut self) {
                self.table.seal_read_only_segment();
            }
        }
        const INTERNAL_READ_ONLY_SEGMENT_OFFSET: u32 = 0;
        const INTERNAL_NULL_ENTRY_INDEX: u32 = 0;
        const END_OF_INTERNAL_READ_ONLY_SEGMENT: usize =
            SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT;

        fn unseal_read_only_segment(&mut self) {
            self.base.unseal_read_only_segment();
        }
        fn seal_read_only_segment(&mut self) {
            self.base.seal_read_only_segment();
        }

        fn extend(&mut self, space: &mut Space<Entry, SIZE>, segment: Segment, freelist: FreelistHead) {
            let mut guard = space.mutex.lock().unwrap();
            space.segments.insert(segment);
        }

        fn allocate_entry(&self, space: &mut Space<Entry, SIZE>) -> u32 {
            if let Some(index) = self.try_allocate_entry(space) {
                index
            } else {
                let freelist = self
                    .try_extend(space)
                    .expect("Failed to extend space during allocation");
                self.try_allocate_entry_from_freelist(space, freelist)
                    .expect("Failed to allocate from new segment");
                self.try_allocate_entry(space).unwrap()
            }
        }

        fn try_allocate_entry(&self, space: &mut Space<Entry, SIZE>) -> Option<u32> {
            let freelist_head = space.freelist_head.load(Ordering::Acquire);
            if freelist_head == Self::ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER {
                return None;
            }
            if self.try_allocate_entry_from_freelist(space, freelist_head) {
                let index = freelist_head.index;
                return Some(index);
            }

            None
        }

        fn allocate_entry_below(&self, space: &mut Space<Entry, SIZE>, threshold_index: u32) -> u32 {
            let mut current_head = space.freelist_head.load(Ordering::Acquire);
            loop {
                if current_head == Self::ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER {
                    return 0; // Allocation is forbidden
                }
                if current_head.index >= threshold_index {
                    return 0; // No free entries below the threshold
                }

                if self.try_allocate_entry_from_freelist(space, current_head) {
                    return current_head.index;
                } else {
                    current_head = space.freelist_head.load(Ordering::Acquire);
                }
            }
        }

        fn try_allocate_entry_from_freelist(&self, space: &mut Space<Entry, SIZE>, freelist: FreelistHead) -> bool {
            let next_free = freelist.next;

            let new_head = FreelistHead {
                index: next_free,
                next: if next_free == u32::MAX {
                    u32::MAX
                } else {
                    self.base.get(next_free as usize).freelist_index.load(Ordering::Acquire)
                },
            };

            space.freelist_head.compare_exchange(
                freelist,
                new_head,
                Ordering::AcqRel,
                Ordering::Acquire,
            ).is_ok()
        }

        fn try_extend(&self, space: &mut Space<Entry, SIZE>) -> Option<FreelistHead> {
            let segment = self.base.allocate_segment()?;
            let mut guard = space.mutex.lock().unwrap();
            space.segments.insert(segment);
            let new_freelist_head = self.initialize_freelist_for_segment(segment);
            guard.unlock();

            Some(new_freelist_head)
        }

        fn generic_sweep(&self, space: &mut Space<Entry, SIZE>) -> u32
        where
            Entry: IsMarked + Unmark,
        {
            let mut live_entries = 0;
            self.iterate_entries_in(space, |index| {
                let entry = self.base.get_mut(index as usize);
                if entry.is_marked() {
                    entry.unmark();
                    live_entries += 1;
                } else {
                    // Free the entry by adding it to the freelist
                    self.add_entry_to_freelist(space, index);
                }
            });
            live_entries
        }

        fn generic_sweep_with_callback<Callback>(&self, space: &mut Space<Entry, SIZE>, mut marked: Callback) -> u32
        where
            Entry: IsMarked + Unmark,
            Callback: FnMut(u32),
        {
            let mut live_entries = 0;
            self.iterate_entries_in(space, |index| {
                let entry = self.base.get_mut(index as usize);
                if entry.is_marked() {
                    entry.unmark();
                    live_entries += 1;
                    marked(index); // Invoke the callback for marked entries
                } else {
                    // Free the entry by adding it to the freelist
                    self.add_entry_to_freelist(space, index);
                }
            });
            live_entries
        }

        fn iterate_entries_in<Callback>(&self, space: &Space<Entry, SIZE>, mut callback: Callback)
        where
            Callback: FnMut(u32),
        {
            let guard = space.mutex.lock().unwrap();
            for segment in &space.segments {
                for i in 0..SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT {
                    let index = segment.index * SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT + i;
                    callback(index as u32);
                }
            }
        }
        const ENTRY_ALLOCATION_IS_FORBIDDEN_MARKER: FreelistHead = FreelistHead { index: u32::MAX, next: u32::MAX };

        fn add_entry_to_freelist(&self, space: &mut Space<Entry, SIZE>, index: u32) {
            let mut current_head = space.freelist_head.load(Ordering::Acquire);

            loop {
                // Update the freelist index of the entry to be freed to point to the current head.
                self.base.get_mut(index as usize).freelist_index.store(current_head.index, Ordering::Release);

                let new_head = FreelistHead {
                    index,
                    next: current_head.index,
                };

                match space.freelist_head.compare_exchange(
                    current_head,
                    new_head,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => break,
                    Err(new_current_head) => current_head = new_current_head,
                }
            }
        }

        fn initialize_freelist_for_segment(&self, segment: Segment) -> FreelistHead {
            let start_index = segment.index * SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT;
            let end_index = start_index + SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT;

            for i in start_index..end_index {
                let mut next_index = (i + 1) as u32;
                if i == end_index - 1 {
                    next_index = u32::MAX;
                }
                self.base.get_mut(i as usize).freelist_index.store(next_index, Ordering::Release);
            }
            FreelistHead {
                index: start_index as u32,
                next: start_index as u32 + 1,
            }
        }
    }

    impl<Entry, const SIZE: usize> Default for ExternalEntityTable<Entry, SIZE> {
        fn default() -> Self {
            Self::new()
        }
    }

    trait IsMarked {
        fn is_marked(&self) -> bool;
    }

    trait Unmark {
        fn unmark(&mut self);
    }

    impl IsMarked for u32 {
        fn is_marked(&self) -> bool {
            false
        }
    }

    impl Unmark for u32 {
        fn unmark(&mut self) {}
    }

    /// A collection of segments in an external entity table.
    ///
    /// For the purpose of memory management, a table is partitioned into segments
    /// of a fixed size (e.g. 64kb). A Space is a collection of segments that all
    /// share the same freelist. As such, entry allocation and freeing (e.g.
    /// through garbage collection) all happen on the level of spaces.
    ///
    /// Spaces allow implementing features such as:
    /// * Young generation GC support (a separate space is used for all entries
    ///   belonging to the young generation)
    /// * Having double-width entries in a table (a dedicated space is used that
    ///   contains only double-width entries)
    /// * Sharing one table between multiple isolates that perform GC independently
    ///   (each Isolate owns one space)
    pub struct Space<Entry, const SIZE: usize> {
        owning_table: AtomicU64,
        freelist_head: AtomicFreelistHead,
        segments: HashSet<Segment>,
        is_internal_read_only_space_: AtomicBool,
        mutex: Mutex<()>,
        _phantom: PhantomData<Entry>,
    }

    impl<Entry, const SIZE: usize> Space<Entry, SIZE> {
        pub fn new() -> Self {
            Space {
                owning_table: AtomicU64::new(0),
                freelist_head: AtomicFreelistHead::new(FreelistHead { index: u32::MAX, next: u32::MAX }),
                segments: HashSet::new(),
                is_internal_read_only_space_: AtomicBool::new(false),
                mutex: Mutex::new(()),
                _phantom: PhantomData,
            }
        }

        /// Determines the number of entries currently on the freelist.
        /// As entries can be allocated from other threads, the freelist size may
        /// have changed by the time this method returns. As such, the returned
        /// value should only be treated as an approximation.
        pub fn freelist_length(&self) -> u32 {
            //TODO implement
            0
        }

        /// Returns the current number of segments currently associated with this
        /// space.
        /// The caller must lock the mutex.
        pub fn num_segments(&self) -> u32 {
            self.segments.len() as u32
        }

        /// Returns whether this space is currently empty.
        /// The caller must lock the mutex.
        pub fn is_empty(&self) -> bool {
            self.num_segments() == 0
        }

        /// Returns the current capacity of this space.
        /// The capacity of a space is the total number of entries it can contain.
        /// The caller must lock the mutex.
        pub fn capacity(&self) -> u32 {
            self.num_segments() * SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT as u32
        }

        /// Returns true if this space contains the entry with the given index.
        pub fn contains(&self, index: u32) -> bool {
            for segment in &self.segments {
                let start_index = segment.index * SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT;
                let end_index = start_index + SegmentedTable::<Entry, SIZE>::ENTRIES_PER_SEGMENT;
                if (start_index as u32..end_index as u32).contains(&index) {
                    return true;
                }
            }
            false
        }

        /// Whether this space is attached to a table's internal read-only segment.
        pub fn is_internal_read_only_space(&self) -> bool {
            self.is_internal_read_only_space_.load(Ordering::Relaxed)
        }

        #[cfg(debug_assertions)]
        /// Check whether this space belongs to the given external entity table.
        pub fn belongs_to(&self, table: *const ExternalEntityTable<Entry, SIZE>) -> bool {
            self.owning_table.load(Ordering::Relaxed) == table as u64
        }

        #[cfg(not(debug_assertions))]
        pub fn belongs_to(&self, _table: *const ExternalEntityTable<Entry, SIZE>) -> bool {
            true
        }

        pub fn num_segments_for_testing(&self) -> u32 {
            let _guard = self.mutex.lock().unwrap();
            self.num_segments()
        }
    }

    impl<Entry, const SIZE: usize> Drop for Space<Entry, SIZE> {
        fn drop(&mut self) {
            // Ensure proper cleanup when the space is dropped.  Deallocate segments here if necessary.
        }
    }

    impl<Entry, const SIZE: usize> Default for Space<Entry, SIZE> {
        fn default() -> Self {
            Self::new()
        }
    }

    /// A Space that supports black allocations.
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

    impl<Entry, const SIZE: usize> std::ops::Deref for SpaceWithBlackAllocationSupport<Entry, SIZE> {
        type Target = Space<Entry, SIZE>;

        fn deref(&self) -> &Self::Target {
            &self.space
        }
    }

    impl<Entry, const SIZE: usize> std::ops::DerefMut for SpaceWithBlackAllocationSupport<Entry, SIZE> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.space
        }
    }

    impl<Entry, const SIZE: usize> Default for SpaceWithBlackAllocationSupport<Entry, SIZE> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    struct Segment {
        index: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FreelistHead {
        index: u32,
        next: u32,
    }

    impl FreelistHead {
        const fn new(index: u32, next: u32) -> Self {
            FreelistHead { index, next }
        }
    }

    struct AtomicFreelistHead {
        inner: AtomicU64,
    }

    impl AtomicFreelistHead {
        const fn new(head: FreelistHead) -> Self {
            let combined = ((head.index as u64) << 32) | (head.next as u64);
            AtomicFreelistHead {
                inner: AtomicU64::new(combined),
            }
        }

        fn load(&self, order: Ordering) -> FreelistHead {
            let combined = self.inner.load(order);
            FreelistHead {
                index: (combined >> 32) as u32,
                next: (combined & 0xFFFFFFFF) as u32,
            }
        }

        fn store(&self, head: FreelistHead, order: Ordering) {
            let combined = ((head.index as u64) << 32) | (head.next as u64);
            self.inner.store(combined, order);
        }

        fn compare_exchange(
            &self,
            current: FreelistHead,
            new: FreelistHead,
            success: Ordering,
            failure: Ordering,
        ) -> Result<FreelistHead, FreelistHead> {
            let current_combined = ((current.index as u64) << 32) | (current.next as u64);
            let new_combined = ((new.index as u64) << 32) | (new.next as u64);

            match self.inner.compare_exchange(current_combined, new_combined, success, failure) {
                Ok(_) => Ok(new),
                Err(actual_combined) => {
                    Err(FreelistHead {
                        index: (actual_combined >> 32) as u32,
                        next: (actual_combined & 0xFFFFFFFF) as u32,
                    })
                }
            }
        }
    }

    ///A generic segmented table
    struct SegmentedTable<Entry, const SIZE: usize> {
        segments: Arc<RwLock<Vec<Segment>>>,
        data: Vec<UnsafeCell<Entry>>,
    }

    unsafe impl<Entry, const SIZE: usize> Sync for SegmentedTable<Entry, SIZE> {}

    impl<Entry, const SIZE: usize> SegmentedTable<Entry, SIZE> {
        const SEGMENT_SIZE: usize = 65536; // 64KB
        const ENTRY_SIZE: usize = mem::size_of::<Entry>();
        const ENTRIES_PER_SEGMENT: usize = Self::SEGMENT_SIZE / Self::ENTRY_SIZE;

        fn new() -> Self {
            SegmentedTable {
                segments: Arc::new(RwLock::new(Vec::new())),
                data: Vec::new(),
            }
        }

        fn initialize(&mut self) {
            let initial_segment = self.allocate_segment().expect("Failed to allocate initial segment");
            self.segments.write().unwrap().push(initial_segment);
        }

        fn tear_down(&mut self) {
            self.segments.write().unwrap().clear();
            self.data.clear();
        }

        fn allocate_segment(&self) -> Option<Segment> {
            let mut segments = self.segments.write().unwrap();
            let new_index = segments.len();

            //Calculate number of needed new entries
            let num_new_entries = Self::ENTRIES_PER_SEGMENT;

            //Extend the vector with default values
            let mut new_data: Vec<UnsafeCell<Entry>> = Vec::with_capacity(self.data.len() + num_new_entries);
            for _ in 0..self.data.len() {
                //SAFETY: Here we assume that every value inside data is valid
                unsafe {
                    new_data.push(UnsafeCell::new(std::ptr::read(self.data.last().unwrap().get())));
                }
            }
            //Extend the vector with default values
            for _ in 0..num_new_entries {
                new_data.push(UnsafeCell::new(unsafe { std::mem::zeroed() }));
            }

            //Update data vector
            self.data = new_data;

            segments.push(Segment { index: new_index });
            Some(Segment { index: new_index })
        }

        fn deallocate_segment(&self, segment: Segment) {
            let mut segments = self.segments.write().unwrap();
            segments.retain(|s| s != &segment);
        }

        fn get(&self, index: usize) -> &Entry {
            assert!(index < self.data.len(), "Index out of bounds");
            // SAFETY: We ensure that the index is within the bounds of the vector.
            unsafe { &*self.data[index].get() }
        }

        fn get_mut(&self, index: usize) -> &mut Entry {
            assert!(index < self.data.len(), "Index out of bounds");
            // SAFETY: We ensure that the index is within the bounds of the vector,
            // and we only hand out one mutable reference at a time.
            unsafe { &mut *self.data[index].get() }
        }

        fn unseal_read_only_segment(&mut self) {
            // Placeholder for unsealing logic (e.g., changing memory permissions).
            // This might involve platform-specific code to modify memory protection.
            // This operation is inherently unsafe and requires careful handling.
        }

        fn seal_read_only_segment(&mut self) {
            // Placeholder for sealing logic (e.g., restoring memory permissions).
            // Similar to unseal_read_only_segment, this might involve platform-specific code.
            // This operation is inherently unsafe and requires careful handling.
        }
    }
}