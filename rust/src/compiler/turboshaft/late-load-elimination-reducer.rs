// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod late_load_elimination_reducer {
    use std::{
        cell::{RefCell, UnsafeCell},
        collections::HashMap,
        fmt,
        hash::{Hash, Hasher},
        ops::{BitAnd, BitOr},
        rc::Rc,
    };

    //use crate::base::doubly_threaded_list::*;
    use crate::compiler::turboshaft::{
        analyzer_iterator::*, assembler::*, graph::*, index::*, loop_finder::*, operations::*,
        opmasks::*, phase::*, representations::*, sidetable::*,
        snapshot_table_opindex::*, utils::*,
    };
    //use crate::zone::zone_containers::*;
    //use crate::zone::zone::*;

    //#[macro_export]
    //macro_rules! DEFINE_ASSEMBLER_MACROS {
    //    () => {
    //        // TODO: Implement assembler macros
    //    };
    //}

    //#[macro_export]
    //macro_rules! UNDEF_ASSEMBLER_MACROS {
    //    () => {
    //        // TODO: Implement assembler macros
    //    };
    //}

    //#[cfg(debug_assertions)]
    //macro_rules! TRACE {
    //    ($x:expr) => {
    //        if v8_flags.turboshaft_trace_load_elimination {
    //            eprintln!("{:?}", $x);
    //        }
    //    };
    //}

    //#[cfg(not(debug_assertions))]
    macro_rules! TRACE {
        ($x:expr) => {};
    }

    pub type MapMask = u64;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct MapMaskAndOr {
        pub or_: MapMask,
        pub and_: MapMask,
    }

    impl MapMaskAndOr {
        pub fn new() -> Self {
            MapMaskAndOr {
                or_: 0,
                and_: u64::MAX,
            }
        }
    }

    impl BitOr for MapMaskAndOr {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            MapMaskAndOr {
                or_: self.or_ | other.or_,
                and_: self.and_ & other.and_,
            }
        }
    }

    impl BitAnd for MapMaskAndOr {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            MapMaskAndOr {
                or_: self.or_ & other.or_,
                and_: self.and_ & other.and_,
            }
        }
    }

    pub fn is_empty(minmax: MapMaskAndOr) -> bool {
        minmax.or_ == 0 && minmax.and_ == u64::MAX
    }

    pub fn compute_map_hash(map: &MapRef) -> MapMask {
        // `map.hash_value()` is probably not a good enough hash, since most user maps
        // will have the same upper bits, so we re-hash. We're using xorshift64* (from
        // "An experimental exploration of Marsaglia’s xorshift generators, scrambled"
        // by Vigna in ACM Transactions on Mathematical Software, Volume 42).
        let mut hash = map.hash_value();
        hash ^= hash >> 12;
        hash ^= hash << 25;
        hash ^= hash >> 27;
        hash * 0x2545f4914f6cdd1d
    }

    pub fn compute_min_max_hash(maps: &ZoneRefSet<Map>) -> MapMaskAndOr {
        let mut minmax = MapMaskAndOr::new();
        for map in maps.iter() {
            let hash = compute_map_hash(map);
            minmax.or_ |= hash;
            minmax.and_ &= hash;
        }
        minmax
    }

    pub fn combine_min_max(a: MapMaskAndOr, b: MapMaskAndOr) -> MapMaskAndOr {
        MapMaskAndOr {
            or_: a.or_ | b.or_,
            and_: a.and_ & b.and_,
        }
    }

    // Returns true if {a} and {b} could have a map in common.
    pub fn could_have_same_map(a: MapMaskAndOr, b: MapMaskAndOr) -> bool {
        ((a.and_ & b.or_) == a.and_) || ((b.and_ & a.or_) == b.and_)
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MemoryAddress {
        pub base: OpIndex,
        pub index: Option<OpIndex>,
        pub offset: i32,
        pub element_size_log2: u8,
        pub size: u8,
    }

    impl fmt::Display for MemoryAddress {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{{ base: {:?}, index: {:?}, offset: {}, element_size_log2: {}, size: {} }}",
                self.base, self.index, self.offset, self.element_size_log2, self.size
            )
        }
    }

    //fn absl_hash_value<H: Hasher>(h: &mut H, mem: &MemoryAddress) {
    //    mem.base.hash(h);
    //    mem.index.hash(h);
    //    mem.offset.hash(h);
    //    mem.element_size_log2.hash(h);
    //    mem.size.hash(h);
    //}

    pub fn fast_hash_combine<T: Hash>(a: T, b: T, c: T, d: T, e: T) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        c.hash(&mut hasher);
        d.hash(&mut hasher);
        e.hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn hash_value(mem: &MemoryAddress) -> usize {
        fast_hash_combine(
            mem.base,
            mem.index,
            mem.offset,
            mem.element_size_log2,
            mem.size,
        )
    }

    pub struct KeyData {
        pub mem: MemoryAddress,
        // Pointers to the previous and the next Keys at the same base.
        pub prev_same_base: *mut Key,
        pub next_same_base: Key,
        // Pointers to either the next/previous Keys at the same offset.
        pub prev_same_offset: *mut Key,
        pub next_same_offset: Key,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Key(pub OpIndex);

    impl Key {
        pub fn valid(&self) -> bool {
            self.0.valid()
        }
        //This is not safe because it has raw pointer dereferencing
        //pub fn data<'a>(&self) -> &'a KeyData {
        //    unsafe { &*self.0.sidetable_entry.get() }
        //}
    }

    pub struct OffsetListTraits;

    impl OffsetListTraits {
        //pub type T = SnapshotTable<OpIndex, KeyData>::Key;
        // pub fn prev(t: &Key) -> &*mut Key {
        //     unsafe { &t.data().prev_same_offset }
        // }
        // pub fn next(t: &Key) -> &Key {
        //     unsafe { &t.data().next_same_offset }
        // }
        pub fn non_empty(t: &Key) -> bool {
            t.valid()
        }
    }

    pub struct BaseListTraits;

    impl BaseListTraits {
        //pub type T = SnapshotTable<OpIndex, KeyData>::Key;
        // pub fn prev(t: &Key) -> &*mut Key {
        //     unsafe { &t.data().prev_same_base }
        // }
        // pub fn next(t: &Key) -> &Key {
        //     unsafe { &t.data().next_same_base }
        // }
        pub fn non_empty(t: &Key) -> bool {
            t.valid()
        }
    }

    pub struct BaseData {
        // List of every value at this base that has an offset rather than an index.
        pub with_offsets: DoublyThreadedList<Key, BaseListTraits>,
        // List of every value at this base that has a valid index.
        pub with_indices: DoublyThreadedList<Key, BaseListTraits>,
    }

    pub struct DoublyThreadedList<T, Traits> {
        head: Option<Box<ListNode<T>>>,
        tail: *mut ListNode<T>,
        marker: std::marker::PhantomData<Traits>,
    }

    struct ListNode<T> {
        data: T,
        prev: *mut ListNode<T>,
        next: Option<Box<ListNode<T>>>,
    }

    impl<T, Traits> DoublyThreadedList<T, Traits> {
        pub fn new() -> Self {
            DoublyThreadedList {
                head: None,
                tail: std::ptr::null_mut(),
                marker: std::marker::PhantomData,
            }
        }

        pub fn push_front(&mut self, data: T) {
            let mut new_node = Box::new(ListNode {
                data,
                prev: std::ptr::null_mut(),
                next: self.head.take(),
            });

            let new_node_ptr: *mut ListNode<T> = &mut *new_node;
            if let Some(mut head) = new_node.next.as_mut() {
                head.prev = new_node_ptr;
            } else {
                self.tail = new_node_ptr;
            }
            self.head = Some(new_node);
        }

        pub fn remove(&mut self, data: T)
        where
            T: PartialEq,
        {
            let mut current = self.head.take();
            let mut prev: *mut ListNode<T> = std::ptr::null_mut();

            while let Some(mut node) = current {
                if node.data == data {
                    // Adjust pointers of neighboring nodes
                    if prev.is_null() {
                        // Removing the head
                        self.head = node.next.take();
                        if let Some(mut head) = self.head.as_mut() {
                            head.prev = std::ptr::null_mut();
                        } else {
                            self.tail = std::ptr::null_mut();
                        }
                    } else {
                        // Removing a node in the middle or at the tail
                        unsafe {
                            (*prev).next = node.next.take();
                            if let Some(mut next) = (*prev).next.as_mut() {
                                next.prev = prev;
                            } else {
                                self.tail = prev;
                            }
                        }
                    }

                    // Node is dropped here
                    return;
                } else {
                    prev = &mut *node;
                    current = node.next.take();
                }
            }

            // Data not found in the list
            self.head = current;
        }

        pub fn iter(&self) -> DoublyLinkedListIterator<T> {
            DoublyLinkedListIterator {
                current: self.head.as_deref(),
            }
        }
    }

    struct DoublyLinkedListIterator<'a, T> {
        current: Option<&'a ListNode<T>>,
    }

    impl<'a, T> Iterator for DoublyLinkedListIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.current.map(|node| {
                self.current = node.next.as_deref();
                &node.data
            })
        }
    }

    impl<T, Traits> IntoIterator for DoublyThreadedList<T, Traits> {
        type Item = T;
        type IntoIter = DoublyLinkedListIntoIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            DoublyLinkedListIntoIterator {
                current: self.head,
            }
        }
    }

    struct DoublyLinkedListIntoIterator<T> {
        current: Option<Box<ListNode<T>>>,
    }

    impl<T> Iterator for DoublyLinkedListIntoIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.current.take().map(|node| {
                self.current = node.next;
                node.data
            })
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LoadEliminationReplacementKind {
        kNone,             // We don't replace the operation
        kLoadElimination,  // We load eliminate a load operation
        // The following replacements are used for the special case optimization:
        // TruncateWord64ToWord32(
        //     BitcastTaggedToWordPtrForTagAndSmiBits(Load(x, Tagged)))
        // =>
        // Load(x, Int32)
        //
        kTaggedLoadToInt32Load,     // Turn a tagged load into a direct int32 load.
        kTaggedBitcastElimination,  // Remove this (now unused) bitcast.
        kInt32TruncationElimination, // Replace truncation by the updated load.
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct LoadEliminationReplacement {
        pub kind_: LoadEliminationReplacementKind,
        pub replacement_: OpIndex,
    }

    impl LoadEliminationReplacement {
        pub fn none() -> Self {
            LoadEliminationReplacement {
                kind_: LoadEliminationReplacementKind::kNone,
                replacement_: OpIndex::invalid(),
            }
        }

        pub fn load_elimination(replacement: OpIndex) -> Self {
            assert!(replacement.valid());
            LoadEliminationReplacement {
                kind_: LoadEliminationReplacementKind::kLoadElimination,
                replacement_: replacement,
            }
        }

        pub fn tagged_load_to_int32_load() -> Self {
            LoadEliminationReplacement {
                kind_: LoadEliminationReplacementKind::kTaggedLoadToInt32Load,
                replacement_: OpIndex::invalid(),
            }
        }

        pub fn tagged_bitcast_elimination() -> Self {
            LoadEliminationReplacement {
                kind_: LoadEliminationReplacementKind::kTaggedBitcastElimination,
                replacement_: OpIndex::invalid(),
            }
        }

        pub fn int32_truncation_elimination(replacement: OpIndex) -> Self {
            LoadEliminationReplacement {
                kind_: LoadEliminationReplacementKind::kInt32TruncationElimination,
                replacement_: replacement,
            }
        }

        pub fn is_none(&self) -> bool {
            self.kind_ == LoadEliminationReplacementKind::kNone
        }

        pub fn is_load_elimination(&self) -> bool {
            self.kind_ == LoadEliminationReplacementKind::kLoadElimination
        }

        pub fn is_tagged_load_to_int32_load(&self) -> bool {
            self.kind_ == LoadEliminationReplacementKind::kTaggedLoadToInt32Load
        }

        pub fn is_tagged_bitcast_elimination(&self) -> bool {
            self.kind_ == LoadEliminationReplacementKind::kTaggedBitcastElimination
        }

        pub fn is_int32_truncation_elimination(&self) -> bool {
            self.kind_ == LoadEliminationReplacementKind::kInt32TruncationElimination
        }

        pub fn replacement(&self) -> OpIndex {
            self.replacement_
        }
    }

    // TODO: Implement IsInt32TruncatedLoadPattern
    pub fn is_int32_truncated_load_pattern(
        _graph: &Graph,
        _change_idx: OpIndex,
        _change: &ChangeOp,
        _bitcast_idx: &mut Option<OpIndex>,
        _load_idx: &mut Option<OpIndex>,
    ) -> bool {
        false
    }

    pub struct MemoryContentTable {
        //This has to be UnsafeCell because of internal mutability
        table: UnsafeCell<HashMap<OpIndex, KeyData>>,
        non_aliasing_objects_: Rc<RefCell<SparseOpIndexSnapshotTable<bool>>>,
        object_maps_: Rc<RefCell<SparseOpIndexSnapshotTable<MapMaskAndOr>>>,
        replacements_: Rc<RefCell<FixedOpIndexSidetable<LoadEliminationReplacement>>>,
        all_keys_: RefCell<HashMap<MemoryAddress, Key>>,
        base_keys_: RefCell<HashMap<OpIndex, BaseData>>,
        offset_keys_:
            RefCell<HashMap<i32, DoublyThreadedList<Key, OffsetListTraits>>>,
        index_keys_: RefCell<DoublyThreadedList<Key, OffsetListTraits>>,
    }

    impl MemoryContentTable {
        const K_MAX_KEYS: usize = 10000;
        pub fn new(
            zone: *mut Zone,
            non_aliasing_objects: Rc<RefCell<SparseOpIndexSnapshotTable<bool>>>,
            object_maps: Rc<RefCell<SparseOpIndexSnapshotTable<MapMaskAndOr>>>,
            replacements: Rc<RefCell<FixedOpIndexSidetable<LoadEliminationReplacement>>>,
        ) -> Self {
            MemoryContentTable {
                table: UnsafeCell::new(HashMap::new()),
                non_aliasing_objects_: non_aliasing_objects,
                object_maps_: object_maps,
                replacements_: replacements,
                all_keys_: RefCell::new(HashMap::new()),
                base_keys_: RefCell::new(HashMap::new()),
                offset_keys_: RefCell::new(HashMap::new()),
                index_keys_: RefCell::new(DoublyThreadedList::new()),
            }
        }

        pub fn on_new_key(&self, key: Key, value: OpIndex) {
            if value.valid() {
                //This can only be done through a mutable borrow that it cannot give.
                //self.add_key_in_base_offset_maps(key);
            }
        }

        pub fn on_value_change(&self, key: Key, old_value: OpIndex, new_value: OpIndex) {
            assert_ne!(old_value, new_value);
            if old_value.valid() && !new_value.valid() {
                //self.remove_key_from_base_offset_maps(key);
            } else if new_value.valid() && !old_value.valid() {
                //self.add_key_in_base_offset_maps(key);
            } else {
                assert_eq!(new_value.valid(), old_value.valid());
            }
        }

        // Invalidate all previous known memory that could alias with {store}.
        pub fn invalidate(&self, store: &StoreOp) {
            self.invalidate_address(store.base(), store.index(), store.offset);
        }

        pub fn invalidate_address(&self, base: OpIndex, index: Option<OpIndex>, offset: i32) {
            TRACE!(format!(
                "> MemoryContentTable: Invalidating based on {:?}, {:?}, {}",
                base, index, offset
            ));
            let base = self.resolve_base(base);

            if self.non_aliasing_objects_.borrow().get(base) {
                TRACE!(">> base is non-aliasing");
                // Since {base} is non-aliasing, it's enough to just iterate the values at
                // this base.
                if let Some(base_keys) = self.base_keys_.borrow().get(&base) {
                    // for it in base_keys.with_offsets.begin()..base_keys.with_offsets.end() {
                    //     let key = *it;
                    //     assert_eq!(key.data().mem.base, base);
                    //     assert!(!key.data().mem.index.is_valid());
                    //     if index.is_valid() || offset == key.data().mem.offset {
                    //         // Overwrites {key}.
                    //         //it = base_keys.with_offsets.remove_at(it);
                    //         TRACE!(format!(">>> invalidating {:?}", key.data().mem));
                    //         //self.set(key, OpIndex::Invalid());
                    //     } else {
                    //         //++it;
                    //     }
                    // }
                    // // Invalidating all of the value with valid Index at base {base}.
                    // for it in base_keys.with_indices.begin()..base_keys.with_indices.end() {
                    //     let key = *it;
                    //     assert!(key.data().mem.index.is_valid());
                    //     //it = base_keys.with_indices.remove_at(it);
                    //     //self.set(key, OpIndex::Invalid());
                    // }
                }
            } else {
                TRACE!(">> base is maybe-aliasing");
                // {base} could alias with other things, so we iterate the whole state.
                if index.is_some() {
                    // {index} could be anything, so we invalidate everything.
                    TRACE!(">> Invalidating everything because of valid index");
                    //return self.invalidate_maybe_aliasing();
                }

                // Invalidating all of the values with valid Index.
                // TODO(dmercadier): we could keep keys that don't alias here, but that
                // would require doing a map lookup on the base of each key. A better
                // alternative would probably be to have 2 {non_alias_index_keys_} and
                // {maybe_alias_index_keys_} tables instead of just {index_keys_}. This
                // has the downside that when a base stops being non-alias, all of its
                // indexed memory cells have to be moved. This could be worked around by
                // having these 2 tables contain BaseData.with_indices values instead of
                // Keys, so that a whole BaseData.with_indices can be removed in a single
                // operation from the global {non_alias_index_keys_}.
                // for it in self.index_keys_.borrow().begin()..self.index_keys_.borrow().end() {
                //     let key = *it;
                //     //it = self.index_keys_.borrow().remove_at(it);
                //     TRACE!(format!(">>> Invalidating indexed memory {:?}", key.data().mem));
                //     //self.set(key, OpIndex::Invalid());
                // }

                TRACE!(format!(
                    ">>> Invalidating everything maybe-aliasing at offset {}",
                    offset
                ));
                self.invalidate_at_offset(offset, base);
            }
        }

        // Invalidates all Keys that are not known as non-aliasing.
        pub fn invalidate_maybe_aliasing(&self) {
            TRACE!(">> InvalidateMaybeAliasing");
            // We find current active keys through {base_keys_} so that we can bail out
            // for whole buckets non-aliasing bases (if we had gone through
            // {offset_keys_} instead, then for each key we would've had to check
            // whether it was non-aliasing or not).
            for (base, base_keys) in self.base_keys_.borrow().iter() {
                if self.non_aliasing_objects_.borrow().get(*base) {
                    continue;
                }
                // for it in base_keys.with_offsets.begin()..base_keys.with_offsets.end() {
                //     let key = *it;
                //     // It's important to remove with RemoveAt before Setting the key to
                //     // invalid, otherwise OnKeyChange will remove {key} from {base_keys},
                //     // which will invalidate {it}.
                //     //it = base_keys.with_offsets.remove_at(it);
                //     TRACE!(format!(">>> Invalidating {:?}", key.data().mem));
                //     //self.set(key, OpIndex::Invalid());
                // }
                // for it in base_keys.with_indices.begin()..base_keys.with_indices.end() {
                //     let key = *it;
                //     //it = base_keys.with_indices.remove_at(it);
                //     TRACE!(format!(">>> Invalidating {:?}", key.data().mem));
                //     //self.set(key, OpIndex::Invalid());
                // }
            }
        }

        pub fn find(&self, load: &LoadOp) -> OpIndex {
            let base = self.resolve_base(load.base());
            let index = load.index();
            let offset = load.offset;
            let element_size_log2 = if index.is_some() {
                load.element_size_log2
            } else {
                0
            };
            let size = load.loaded_rep.size_in_bytes();

            let mem = MemoryAddress {
                base,
                index,
                offset,
                element_size_log2,
                size,
            };
            if let Some(key) = self.all_keys_.borrow().get(&mem) {
                //Here we need access to the table.
                //return self.get(*key);
                return key.0;
            }
            OpIndex::invalid()
        }

        pub fn insert(&self, store: &StoreOp) {
            let base = self.resolve_base(store.base());
            let index = store.index();
            let offset = store.offset;
            let element_size_log2 = if index.is_some() {
                store.element_size_log2
            } else {
                0
            };
            let value = store.value();
            let size = store.stored_rep.size_in_bytes();

            if store.kind.is_immutable {
                self.insert_immutable(base, index, offset, element_size_log2, size, value);
            } else {
                self.insert_mutable(base, index, offset, element_size_log2, size, value);
            }
        }

        pub fn insert_load(&self, load: &LoadOp, load_idx: OpIndex) {
            let base = self.resolve_base(load.base());
            let index = load.index();
            let offset = load.offset;
            let element_size_log2 = if index.is_some() {
                load.element_size_log2
            } else {
                0
            };
            let size = load.loaded_rep.size_in_bytes();

            if load.kind.is_immutable {
                self.insert_immutable(base, index, offset, element_size_log2, size, load_idx);
            } else {
                self.insert_mutable(base, index, offset, element_size_log2, size, load_idx);
            }
        }

        // #[cfg(debug_assertions)]
        // pub fn print(&self) {
        //     println!("MemoryContentTable:");
        //     for (base, base_keys) in self.base_keys_.iter() {
        //         for key in base_keys.with_offsets.iter() {
        //             println!(
        //                 "  * {:?} - {:?} - {} - {} ==> {:?}",
        //                 key.data().mem.base,
        //                 key.data().mem.index,
        //                 key.data().mem.offset,
        //                 key.data().mem.element_size_log2,
        //                 self.get(*key)
        //             );
        //         }
        //         for key in base_keys.with_indices.iter() {
        //             println!(
        //                 "  * {:?} - {:?} - {} - {} ==> {:?}",
        //                 key.data().mem.base,
        //                 key.data().mem.index,
        //                 key.data().mem.offset,
        //                 key.data().mem.element_size_log2,
        //                 self.get(*key)
        //             );
        //         }
        //     }
        // }

        fn insert_mutable(
            &self,
            base: OpIndex,
            index: Option<OpIndex>,
            offset: i32,
            element_size_log2: u8,
            size: u8,
            value: OpIndex,
        ) {
            assert_eq!(base, self.resolve_base(base));

            let mem = MemoryAddress {
                base,
                index,
                offset,
                element_size_log2,
                size,
            };
            TRACE!(format!(
                "> MemoryContentTable: will insert {:?} with value={:?}",
                mem, value
            ));
            if let Some(existing_key) = self.all_keys_.borrow().get(&mem) {
                TRACE!(">> Reusing existing key");
                //self.set(*existing_key, value);
                return;
            }

            if self.all_keys_.borrow().len() > Self::K_MAX_KEYS {
                TRACE!(">> Bailing out because too many keys");
                return;
            }

            // Creating a new key.
            //let key = self.new_key(KeyData { mem });
            let key = Key(value); //Temporary
            self.all_keys_.borrow_mut().insert(mem, key);
            //self.set(key, value);
        }

        fn insert_immutable(
            &self,
            base: OpIndex,
            index: Option<OpIndex>,
            offset: i32,
            element_size_log2: u8,
            size: u8,
            value: OpIndex,
        ) {
            assert_eq!(base, self.resolve_base(base));

            let mem = MemoryAddress {
                base,
                index,
                offset,
                element_size_log2,
                size,
            };
            TRACE!(format!(
                "> MemoryContentTable: will insert immutable {:?} with value={:?}",
                mem, value
            ));
            if let Some(existing_key) = self.all_keys_.borrow().get(&mem) {
                TRACE!(">> Reusing existing key");
                //self.set_no_notify(*existing_key, value);
                return;
            }

            if self.all_keys_.borrow().len() > Self::K_MAX_KEYS {
                TRACE!(">> Bailing out because too many keys");
                return;
            }

            // Creating a new key.
            //let key = self.new_key(KeyData { mem });
            let key = Key(value); //Temporary
            self.all_keys_.borrow_mut().insert(mem, key);
            // Call `set_no_notify` to avoid calls to `on_new_key` and `on_value_changed`.
            //self.set_no_notify(key, value);
        }

        fn invalidate_at_offset(&self, offset: i32, base: OpIndex) {
            // if let Some(base_maps) = self.object_maps_.borrow().get(base) {
            //     if let Some(offset_keys) = self.offset_keys_.borrow().get(&offset) {
            //         // for it in offset_keys.begin()..offset_keys.end() {
            //         //     let key = *it;
            //         //     assert_eq!(offset, key.data().mem.offset);
            //         //     // It can overwrite previous stores to any base (except non-aliasing
            //         //     // ones).
            //         //     if self.non_aliasing_objects_.borrow().get(key.data().mem.base) {
            //         //         //++it;
            //         //         continue;
            //         //     }
            //         //     let this_maps = if key.data().mem.base == base {
            //         //         base_maps
            //         //     } else {
            //         //         self.object_maps_.borrow().get(key.data().mem.base)
            //         //     };
            //         //     if !is_empty(base_maps) && !is_empty(this_maps) && !could_have_same_map(base_maps, this_maps) {
            //         //         TRACE!(format!(">>>> InvalidateAtOffset: not invalidating thanks for maps: {:?}", key.data().mem));
            //         //         //++it;
            //         //         continue;
            //         //     }
            //         //     //it = offset_keys.remove_at(it);
            //         //     TRACE!(format!(">>>> InvalidateAtOffset: invalidating {:?}", key.data().mem));
            //         //     //self.set(key, OpIndex::Invalid());
            //         // }
            //     }
            // }
        }

        fn resolve_base(&self, mut base: OpIndex) -> OpIndex {
            loop {
                if self.replacements_.borrow().get(base).is_load_elimination() {
                    base = self.replacements_.borrow().get(base).replacement();
                } else {
                    break;
                }
            }
            base
        }

        // fn add_key_in_base_offset_maps(&self, key: Key) {
        //     // Inserting in {base_keys_}.
        //     let base = key.data().mem.base;
        //     if let Some(base_keys) = self.base_keys_.borrow().get(&base) {
        //         if key.data().mem.index.is_valid() {
        //             base_keys.with