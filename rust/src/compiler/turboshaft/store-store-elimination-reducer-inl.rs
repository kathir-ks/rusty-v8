// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod store_store_elimination_reducer {
    use std::{
        cmp::max,
        collections::{HashMap, HashSet},
        fmt,
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    // Assuming these are defined elsewhere, but needed for compilation.
    pub type OpIndex = usize; // Placeholder
    pub type BlockIndex = usize;
    pub type Zone = (); // Placeholder
    pub type Graph = (); // Placeholder
    pub type Block = (); // Placeholder
    pub struct Operation {
        pub opcode: Opcode,
    }
    impl Operation {
        pub fn Effects(&self) -> OpEffects {
            OpEffects {}
        }
        pub fn Cast<T>(&self) -> &T {
            unimplemented!()
        }
    }
    pub struct StoreOp {
        pub kind: StoreOpKind,
        pub stored_rep: MemoryRepresentation,
        pub write_barrier: WriteBarrierKind,
        pub base: fn() -> OpIndex,
        pub offset: i32,
        pub index: fn() -> OpIndex,
        pub value: fn() -> OpIndex,
        pub maybe_initializing_or_transitioning: bool,
    }
    impl StoreOp {
        pub fn Kind() -> StoreOpKind {
            StoreOpKind::TaggedBase
        }
    }

    pub struct LoadOp {
        pub base: fn() -> OpIndex,
        pub offset: i32,
        pub index: fn() -> OpIndex,
        pub kind: LoadOpKind,
    }
    impl LoadOp {}

    pub struct ConstantOp {
        pub kind: ConstantOpKind,
    }
    impl ConstantOp {
        pub fn handle(&self) -> &HeapObject {
            unimplemented!()
        }
    }
    pub struct HeapObject {}
    impl HeapObject {
        pub fn ptr(&self) -> usize {
            unimplemented!()
        }
    }

    pub struct OpEffects {}
    impl OpEffects {
        pub fn can_read_mutable_memory(&self) -> bool {
            false
        }
        pub fn requires_consistent_heap(&self) -> bool {
            false
        }
    }

    #[derive(PartialEq, Eq)]
    pub struct IntrusiveSetIndex {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum StoreObservability {
        Unobservable,
        GCObservable,
        Observable,
    }

    impl Display for StoreObservability {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StoreObservability::Unobservable => write!(f, "Unobservable"),
                StoreObservability::GCObservable => write!(f, "GCObservable"),
                StoreObservability::Observable => write!(f, "Observable"),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MaybeRedundantStoresKeyData {
        pub base: OpIndex,
        pub offset: i32,
        pub size: u8,
        pub active_keys_index: IntrusiveSetIndex,
    }

    pub trait SnapshotTableTrait {
        type Key;
        type Value;
        fn get(&self, key: &Self::Key) -> Self::Value;
        fn set(&mut self, key: Self::Key, value: Self::Value);
    }

    pub struct ChangeTrackingSnapshotTable {
        table: HashMap<MaybeRedundantStoresKeyData, StoreObservability>,
        zone: Zone, // Placeholder
    }

    impl ChangeTrackingSnapshotTable {
        pub fn new(zone: Zone) -> Self {
            ChangeTrackingSnapshotTable {
                table: HashMap::new(),
                zone,
            }
        }
        fn Get(&self, key: MaybeRedundantStoresKeyData) -> StoreObservability {
            *self.table.get(&key).unwrap_or(&StoreObservability::Observable)
        }

        fn Set(&mut self, key: MaybeRedundantStoresKeyData, value: StoreObservability) {
            self.table.insert(key, value);
        }
    }

    // Placeholder types.  Replace with actual implementations.
    pub struct GrowingBlockSidetable<T> {
        data: HashMap<usize, T>,
        zone: Zone,
    }

    impl<T> GrowingBlockSidetable<T> {
        pub fn new(zone: Zone) -> Self {
            GrowingBlockSidetable {
                data: HashMap::new(),
                zone,
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(&index)
        }

        pub fn insert(&mut self, index: usize, value: T) {
            self.data.insert(index, value);
        }
    }

    pub struct ZoneAbslFlatHashMap<K, V> {
        data: HashMap<K, V>,
        zone: Zone,
    }

    impl<K: Eq + std::hash::Hash, V> ZoneAbslFlatHashMap<K, V> {
        pub fn new(zone: Zone) -> Self {
            ZoneAbslFlatHashMap {
                data: HashMap::new(),
                zone,
            }
        }

        pub fn find(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }

        pub fn emplace(&mut self, key: K, value: V) {
            self.data.insert(key, value);
        }
    }

    pub struct ZoneIntrusiveSet<K, F> {
        data: HashSet<K>,
        _phantom: std::marker::PhantomData<F>,
        zone: Zone,
    }

    impl<K: Eq + std::hash::Hash, F> ZoneIntrusiveSet<K, F> {
        pub fn new(zone: Zone) -> Self {
            ZoneIntrusiveSet {
                data: HashSet::new(),
                _phantom: std::marker::PhantomData,
                zone,
            }
        }
        pub fn Add(&mut self, key: K) {
            self.data.insert(key);
        }
        pub fn Remove(&mut self, key: K) {
            self.data.remove(&key);
        }
        pub fn Contains(&self, key: K) -> bool {
            self.data.contains(&key)
        }
    }

    pub struct ZoneVector<T> {
        data: Vec<T>,
        zone: Zone,
    }
    impl<T> Deref for ZoneVector<T> {
        type Target = Vec<T>;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    impl<T> DerefMut for ZoneVector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    impl<T> ZoneVector<T> {
        pub fn new(zone: Zone) -> Self {
            ZoneVector {
                data: Vec::new(),
                zone,
            }
        }
        pub fn push_back(&mut self, value: T) {
            self.data.push(value);
        }
        pub fn clear(&mut self) {
            self.data.clear();
        }
    }

    pub struct ZoneSet<T> {
        data: HashSet<T>,
        zone: Zone,
    }

    impl<T: Eq + std::hash::Hash> ZoneSet<T> {
        pub fn new(zone: Zone) -> Self {
            ZoneSet {
                data: HashSet::new(),
                zone,
            }
        }
        pub fn insert(&mut self, value: T) {
            self.data.insert(value);
        }
        pub fn count(&self, value: T) -> usize {
            if self.data.contains(&value) {
                1
            } else {
                0
            }
        }
    }

    pub struct ZoneMap<K, V> {
        data: HashMap<K, V>,
        zone: Zone,
    }

    impl<K: Eq + std::hash::Hash, V> ZoneMap<K, V> {
        pub fn new(zone: Zone) -> Self {
            ZoneMap {
                data: HashMap::new(),
                zone,
            }
        pub fn insert(&mut self, key: K, value: V) {
            self.data.insert(key, value);
        }
        pub fn count(&self, key: K) -> usize {
            if self.data.contains_key(&key) {
                1
            } else {
                0
            }
        }
        pub fn get(&self, key: K) -> Option<&V> {
            self.data.get(&key)
        }
    }

    pub struct RedundantStoreAnalysis {
        graph_: Graph,
        table_: MaybeRedundantStoresTable,
    }

    impl RedundantStoreAnalysis {
        pub fn new(graph: Graph, phase_zone: Zone) -> Self {
            RedundantStoreAnalysis {
                graph_: graph,
                table_: MaybeRedundantStoresTable::new(&graph, phase_zone),
            }
        }

        pub fn run(
            &mut self,
            eliminable_stores: &mut ZoneSet<OpIndex>,
            mergeable_store_pairs: &mut ZoneMap<OpIndex, u64>,
        ) {
            let mut eliminable_stores_ptr: *mut ZoneSet<OpIndex> = eliminable_stores;
            let mut mergeable_store_pairs_ptr: *mut ZoneMap<OpIndex, u64> =
                mergeable_store_pairs;

            for processed in (0..1).rev() {
                // Assuming graph_.block_count() returns 1 for now.
                let block_index = processed as BlockIndex;

                // Assuming this retrieves the only block for now.
                let block = Block {}; //graph_.Get(block_index);
                                       // self.process_block(&block, unsafe { &mut *eliminable_stores_ptr }, unsafe { &mut *mergeable_store_pairs_ptr });
                self.process_block(&block, eliminable_stores, mergeable_store_pairs);
                // If this block is a loop header, check if this loop needs to be
                // revisited.
                // if block.IsLoop() {
                //     DCHECK(!table_.IsSealed());
                //     bool needs_revisit = false;
                //     table_.Seal(&needs_revisit);
                //     if (needs_revisit) {
                //         Block* back_edge = block.LastPredecessor();
                //         DCHECK_GE(back_edge->index(), block_index);
                //         processed = back_edge->index().id() + 1;
                //     }
                // }
            }
            eliminable_stores_ptr = std::ptr::null_mut();
            mergeable_store_pairs_ptr = std::ptr::null_mut();
        }

        fn process_block(
            &mut self,
            block: &Block,
            eliminable_stores: &mut ZoneSet<OpIndex>,
            mergeable_store_pairs: &mut ZoneMap<OpIndex, u64>,
        ) {
            // self.table_.BeginBlock(block);
            // Assuming graph_.OperationIndices() returns a single element range.
            // for it in graph_.OperationIndices(block) {
            //     OpIndex index = *it;
            //     const Operation& op = graph_.Get(index);
            // match op.opcode {
            //     Opcode::kStore => {
            //         const StoreOp& store = op.Cast<StoreOp>();
            //         // TODO(nicohartmann@): Use the new effect flags to distinguish heap
            //         // access once available.
            //         const bool is_on_heap_store = store.kind.tagged_base;
            //         const bool is_field_store = !store.index().valid();
            //         const uint8_t size = store.stored_rep.SizeInBytes();
            //         // For now we consider only stores of fields of objects on the heap.
            //         if (is_on_heap_store && is_field_store) {
            //             bool is_eliminable_store = false;
            //             match self.table_.GetObservability(store.base(), store.offset, size) {
            //                 StoreObservability::kUnobservable => {
            //                     eliminable_stores_->insert(index);
            //                     last_field_initialization_store_ = OpIndex::Invalid();
            //                     is_eliminable_store = true;
            //                 }
            //                 StoreObservability::kGCObservable => {
            //                     if (store.maybe_initializing_or_transitioning) {
            //                         // We cannot eliminate this store, but we mark all following
            //                         // stores to the same `base+offset` as unobservable.
            //                         self.table_.MarkStoreAsUnobservable(store.base(), store.offset, size);
            //                     } else {
            //                         eliminable_stores_->insert(index);
            //                         last_field_initialization_store_ = OpIndex::Invalid();
            //                         is_eliminable_store = true;
            //                     }
            //                 }
            //                 StoreObservability::kObservable => {
            //                     // We cannot eliminate this store, but we mark all following
            //                     // stores to the same `base+offset` as unobservable.
            //                     self.table_.MarkStoreAsUnobservable(store.base(), store.offset, size);
            //                 }
            //             }
            //             // Try to merge 2 consecutive 32-bit stores into a single 64-bit
            //             // one.
            //             if COMPRESS_POINTERS_BOOL && !is_eliminable_store &&
            //                 store.maybe_initializing_or_transitioning &&
            //                 store.kind == StoreOp::Kind::TaggedBase() &&
            //                 store.write_barrier == WriteBarrierKind::kNoWriteBarrier &&
            //                 store.stored_rep.IsCompressibleTagged() {
            //                 if last_field_initialization_store_.valid() &&
            //                     graph_.NextIndex(index) == last_field_initialization_store_ {
            //                     const StoreOp& store0 = store;
            //                     const StoreOp& store1 =
            //                         graph_.Get(last_field_initialization_store_)
            //                             .Cast<StoreOp>();
            //                     DCHECK(!store0.index().valid());
            //                     DCHECK(!store1.index().valid());
            //                     const ConstantOp* c0 =
            //                         graph_.Get(store0.value()).TryCast<ConstantOp>();
            //                     const ConstantOp* c1 =
            //                         graph_.Get(store1.value()).TryCast<ConstantOp>();
            //                     // TODO(dmercadier): for now, we only apply this optimization
            //                     // when storing read-only values, because otherwise the GC will
            //                     // lose track of Handles when we convert them to a raw Word64.
            //                     // However, if we were to keep the reloc info up-to-date, then
            //                     // this might work for any object. To do this, we might need to
            //                     // delay this optimization to later (instruction selector for
            //                     // instance).
            //                     if c0 && c1 && c0->kind == ConstantOp::Kind::kHeapObject &&
            //                         c1->kind == ConstantOp::Kind::kHeapObject &&
            //                         store1.offset - store0.offset == 4 &&
            //                         HeapLayout::InReadOnlySpace(*c0->handle()) &&
            //                         HeapLayout::InReadOnlySpace(*c1->handle()) {
            //                         uint32_t high = static_cast<uint32_t>(c1->handle()->ptr());
            //                         uint32_t low = static_cast<uint32_t>(c0->handle()->ptr());
            //                         #if V8_TARGET_BIG_ENDIAN
            //                         uint64_t merged = make_uint64(low, high);
            //                         #else
            //                         uint64_t merged = make_uint64(high, low);
            //                         #endif
            //                         mergeable_store_pairs_->insert({index, merged});
            //                         eliminable_stores_->insert(last_field_initialization_store_);
            //                         last_field_initialization_store_ = OpIndex::Invalid();
            //                     }
            //                 } else {
            //                     last_field_initialization_store_ = index;
            //                 }
            //             }
            //         }
            //         break;
            //     }
            //     Opcode::kLoad => {
            //         const LoadOp& load = op.Cast<LoadOp>();
            //         // TODO(nicohartmann@): Use the new effect flags to distinguish heap
            //         // access once available.
            //         const bool is_on_heap_load = load.kind.tagged_base;
            //         const bool is_field_load = !load.index().valid();
            //         // For now we consider only loads of fields of objects on the heap.
            //         if (is_on_heap_load && is_field_load) {
            //             table_.MarkPotentiallyAliasingStoresAsObservable(load.base(), load.offset);
            //         }
            //         break;
            //     }
            //     _ => {
            //         OpEffects effects = op.Effects();
            //         if (effects.can_read_mutable_memory()) {
            //             table_.MarkAllStoresAsObservable();
            //         } else if (effects.requires_consistent_heap()) {
            //             table_.MarkAllStoresAsGCObservable();
            //         }
            //     }
            // }
            // }
        }
    }

    pub struct MaybeRedundantStoresTable {
        graph_: Graph,
        block_to_snapshot_mapping_: GrowingBlockSidetable<Option<Snapshot>>,
        key_mapping_: ZoneAbslFlatHashMap<(OpIndex, i32), Key>,
        active_keys_: ZoneIntrusiveSet<Key, GetActiveKeysIndex>,
        current_block_: *const Block,
        successor_snapshots_: ZoneVector<Snapshot>,
        table: ChangeTrackingSnapshotTable,
    }

    impl MaybeRedundantStoresTable {
        pub fn new(graph: &Graph, zone: Zone) -> Self {
            MaybeRedundantStoresTable {
                graph_: *graph,
                block_to_snapshot_mapping_: GrowingBlockSidetable::new(zone),
                key_mapping_: ZoneAbslFlatHashMap::new(zone),
                active_keys_: ZoneIntrusiveSet::new(zone),
                current_block_: std::ptr::null(),
                successor_snapshots_: ZoneVector::new(zone),
                table: ChangeTrackingSnapshotTable::new(zone),
            }
        }
        fn GetObservability(&self, base: fn() -> OpIndex, offset: i32, size: u8) -> StoreObservability {
            let key = self.map_to_key(base(), offset, size);
           // if key.data().size < size {
           //     return StoreObservability::kObservable;
           // }
           // self.table.Get(key.data())
           StoreObservability::Observable
        }
    }

    struct Key {
        data: MaybeRedundantStoresKeyData
    }

    impl Key {
        fn data(&self) -> MaybeRedundantStoresKeyData {
            self.data
        }
    }

    struct Snapshot {}

    // Needs proper implementation of Assembler, Graph, and Operation types.
    impl MaybeRedundantStoresTable {
        fn map_to_key(&self, base: OpIndex, offset: i32, size: u8) -> Key {
            let p = (base, offset);
            // if let Some(it) = self.key_mapping_.find(&p) {
            //     return *it;
            // }
            // let new_key = self.NewKey(MaybeRedundantStoresKeyData{base, offset, size},
            //                         StoreObservability::kObservable);
            // self.key_mapping_.emplace(p, new_key);
            let new_key = Key {data: MaybeRedundantStoresKeyData{base, offset, size, active_keys_index: IntrusiveSetIndex{}}};
            new_key
        }
    }

    struct GetActiveKeysIndex;
    impl GetActiveKeysIndex {
      // IntrusiveSetIndex& operator()(Key key) const {
      //   return key.data().active_keys_index;
      // }
    }

    // Enums placeholders
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Opcode {
        kStore,
        kLoad,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StoreOpKind {
        TaggedBase,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum LoadOpKind {
        TaggedBase,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MemoryRepresentation {
        Uint64,
    }
    impl MemoryRepresentation {
        fn SizeInBytes(&self) -> u8 {
            8
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum WriteBarrierKind {
        kNoWriteBarrier,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ConstantOpKind {
        kHeapObject,
    }

    fn make_uint64(high: u32, low: u32) -> u64 {
        ((high as u64) << 32) | (low as u64)
    }

    // Placeholder functions
    fn SuccessorBlocks(_op: &Operation) -> Vec<*const Block> {
        Vec::new()
    }

    // Placeholder constants

    const COMPRESS_POINTERS_BOOL: bool = false;
}