// Converted from V8 C++ source files:
// Header: store-store-elimination-reducer-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod store_store_elimination_reducer_inl {
use std::cmp::max_element;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::compiler::turboshaft::assembler::*;
use crate::compiler::turboshaft::graph::*;
use crate::compiler::turboshaft::operations::*;
use crate::compiler::turboshaft::sidetable::*;
use crate::compiler::turboshaft::snapshot_table::*;
use crate::compiler::turboshaft::uniform_reducer_adapter::*;
use crate::heap::heap_layout_inl::*;
use crate::objects::heap_object_inl::*;
use crate::v8::internal::compiler::turboshaft::define_assembler_macros_inc::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum StoreObservability {
    kUnobservable = 0,
    kGCObservable = 1,
    kObservable = 2,
}

impl std::fmt::Display for StoreObservability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreObservability::kUnobservable => write!(f, "Unobservable"),
            StoreObservability::kGCObservable => write!(f, "GCObservable"),
            StoreObservability::kObservable => write!(f, "Observable"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaybeRedundantStoresKeyData {
    pub base: OpIndex,
    pub offset: i32,
    pub size: u8,
    pub active_keys_index: IntrusiveSetIndex,
}

impl MaybeRedundantStoresKeyData {
    pub fn new(base: OpIndex, offset: i32, size: u8) -> Self {
        MaybeRedundantStoresKeyData {
            base,
            offset,
            size,
            active_keys_index: IntrusiveSetIndex::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    data: MaybeRedundantStoresKeyData,
    id: usize,
}

impl Key {
    pub fn new(data: MaybeRedundantStoresKeyData, id: usize) -> Self {
        Key { data, id }
    }
    pub fn data(&self) -> MaybeRedundantStoresKeyData {
        self.data
    }
}

#[derive(Default)]
pub struct IntrusiveSetIndex {}

pub struct MaybeRedundantStoresTable {
    graph_: Graph,
    block_to_snapshot_mapping_: GrowingBlockSidetable<Option<Snapshot>>,
    key_mapping_: HashMap<(OpIndex, i32), Key>,
    active_keys_: ZoneIntrusiveSet<Key, GetActiveKeysIndex>,
    current_block_: Option<&'static Block>,
    successor_snapshots_: Vec<Snapshot>,
    zone: *mut Zone,
    table: SnapshotTable<StoreObservability, MaybeRedundantStoresKeyData>,
    next_key_id: usize,
}

impl MaybeRedundantStoresTable {
    pub fn new(graph: &Graph, zone: *mut Zone) -> Self {
        MaybeRedundantStoresTable {
            graph_: graph.clone(),
            block_to_snapshot_mapping_: GrowingBlockSidetable::new(zone),
            key_mapping_: HashMap::new(),
            active_keys_: ZoneIntrusiveSet::new(zone),
            current_block_: None,
            successor_snapshots_: Vec::new(),
            zone,
            table: SnapshotTable::new(zone),
            next_key_id: 0,
        }
    }

    fn new_key(&mut self, data: MaybeRedundantStoresKeyData, initial_value: StoreObservability) -> Key {
        let key = Key::new(data, self.next_key_id);
        self.next_key_id += 1;
        self.table.insert(key, initial_value);
        key
    }
    pub fn on_new_key(&self, key: Key, value: StoreObservability) {
        assert_eq!(value, StoreObservability::kObservable);
        assert!(!self.active_keys_.contains(&key));
    }

    pub fn on_value_change(&mut self, key: Key, old_value: StoreObservability, new_value: StoreObservability) {
        assert_ne!(old_value, new_value);
        if new_value == StoreObservability::kObservable {
            self.active_keys_.remove(&key);
        } else if old_value == StoreObservability::kObservable {
            self.active_keys_.insert(key);
        }
    }

    pub fn begin_block(&mut self, block: &'static Block) {
        if self.is_sealed() {
            assert!(self.current_block_.is_none());
        } else {
            self.seal();
        }

        let successors = Self::successor_blocks(block.last_operation(&self.graph_));
        self.successor_snapshots_.clear();
        for s in successors {
            let s_index = s.index();
            let s_snapshot = self.block_to_snapshot_mapping_.get(s_index);
            if s_snapshot.is_none() {
                if s.is_loop() {
                } else {
                }
                continue;
            }
            self.successor_snapshots_.push(s_snapshot.unwrap().clone());
        }

        let successor_values: Vec<StoreObservability> = self
            .successor_snapshots_
            .iter()
            .map(|snapshot| self.table.get_value(*snapshot).unwrap())
            .collect();

        let merged_value = *successor_values.iter().max().unwrap_or(&StoreObservability::kObservable);

        self.table.start_new_snapshot();
        self.current_block_ = Some(block);
    }

    fn successor_blocks(last_operation: OpIndex) -> Vec<&'static Block> {
        Vec::new()
    }

    pub fn get_observability(&mut self, base: OpIndex, offset: i32, size: u8) -> StoreObservability {
        let key = self.map_to_key(base, offset, size);
        if key.data().size < size {
            return StoreObservability::kObservable;
        }
        self.table.get(key).unwrap_or(StoreObservability::kObservable)
    }

    pub fn mark_store_as_unobservable(&mut self, base: OpIndex, offset: i32, size: u8) {
        let key = self.map_to_key(base, offset, size);
        if size < key.data().size {
            return;
        }
        self.table.set(key, StoreObservability::kUnobservable);
    }

    pub fn mark_potentially_aliasing_stores_as_observable(&mut self, base: OpIndex, offset: i32) {
        for key in self.active_keys_.iter() {
            if key.data().offset == offset {
                self.table.set(*key, StoreObservability::kObservable);
            }
        }
    }

    pub fn mark_all_stores_as_observable(&mut self) {
        for key in self.active_keys_.iter() {
            self.table.set(*key, StoreObservability::kObservable);
        }
    }

    pub fn mark_all_stores_as_gc_observable(&mut self) {
        for key in self.active_keys_.iter() {
            let current = self.table.get(*key).unwrap_or(StoreObservability::kObservable);
            assert_ne!(current, StoreObservability::kObservable);
            if current == StoreObservability::kUnobservable {
                self.table.set(*key, StoreObservability::kGCObservable);
            }
        }
    }

    pub fn seal(&mut self, snapshot_has_changed: Option<&mut bool>) {
        assert!(!self.is_sealed());
        assert!(self.current_block_.is_some());
        let current_block = self.current_block_.unwrap();
        assert!(current_block.index().valid());
        let snapshot = self.block_to_snapshot_mapping_.get(current_block.index());

        match snapshot_has_changed {
            None => {
                let new_snapshot = self.table.seal();
                self.block_to_snapshot_mapping_.set(current_block.index(), Some(new_snapshot));
            }
            Some(changed) => {
                if snapshot.is_none() {
                    *changed = true;
                    let new_snapshot = self.table.seal();
                    self.block_to_snapshot_mapping_.set(current_block.index(), Some(new_snapshot));
                } else {
                    *changed = false;
                    let new_snapshot = self.table.seal();
                    let old_snapshot = snapshot.unwrap();

                    let old_value = self.table.get_value(old_snapshot).unwrap();
                    let new_value = self.table.get_value(new_snapshot).unwrap();
                    if old_value != new_value {
                        *changed = true;
                    }

                    let merged_value = if old_value > new_value { old_value } else { new_value };
                    self.table.start_new_snapshot();
                    let merged_snapshot = self.table.seal();
                    self.block_to_snapshot_mapping_.set(current_block.index(), Some(merged_snapshot));
                }
            }
        }
        self.current_block_ = None;
    }

    fn map_to_key(&mut self, base: OpIndex, offset: i32, size: u8) -> Key {
        let p = (base, offset);
        if let Some(key) = self.key_mapping_.get(&p) {
            return *key;
        }
        let new_key_data = MaybeRedundantStoresKeyData::new(base, offset, size);
        let new_key = self.new_key(new_key_data, StoreObservability::kObservable);
        self.key_mapping_.insert(p, new_key);
        new_key
    }

    pub fn print(&self, os: &mut std::fmt::Write, sep: &str) -> std::fmt::Result {
        let mut first = true;
        for key in self.active_keys_.iter() {
            write!(
                os,
                "{}{}@{} store_observability {}",
                if first { "" } else { sep },
                key.data().base.id(),
                key.data().offset,
                self.table.get(*key).unwrap()
            )?;
            first = false;
        }
        Ok(())
    }

    fn is_sealed(&self) -> bool {
        self.table.is_sealed()
    }
}

struct GetActiveKeysIndex {}

impl GetActiveKeysIndex {
    fn get_index(key: &Key) -> IntrusiveSetIndex {
        key.data().active_keys_index
    }
}

pub struct RedundantStoreAnalysis {
    graph_: Graph,
    table_: MaybeRedundantStoresTable,
    eliminable_stores_: Option<HashSet<OpIndex>>,
    mergeable_store_pairs_: Option<HashMap<OpIndex, u64>>,
    last_field_initialization_store_: OpIndex,
    phase_zone: *mut Zone,
}

impl RedundantStoreAnalysis {
    pub fn new(graph: &Graph, phase_zone: *mut Zone) -> Self {
        RedundantStoreAnalysis {
            graph_: graph.clone(),
            table_: MaybeRedundantStoresTable::new(graph, phase_zone),
            eliminable_stores_: None,
            mergeable_store_pairs_: None,
            last_field_initialization_store_: OpIndex::Invalid(),
            phase_zone,
        }
    }

    pub fn run(&mut self, eliminable_stores: &mut HashSet<OpIndex>, mergeable_store_pairs: &mut HashMap<OpIndex, u64>) {
        self.eliminable_stores_ = Some(eliminable_stores.clone());
        self.mergeable_store_pairs_ = Some(mergeable_store_pairs.clone());

        for processed in (0..self.graph_.block_count()).rev() {
            let block_index = BlockIndex::new(processed as i32);
            let block = self.graph_.get(block_index);

            self.process_block(block);

            if block.is_loop() {
                let mut needs_revisit = false;
                self.table_.seal(Some(&mut needs_revisit));
                if needs_revisit {
                    let back_edge = block.last_predecessor();
                    assert!(back_edge.index() >= block_index);
                }
            }
        }

        self.eliminable_stores_ = None;
        self.mergeable_store_pairs_ = None;
    }

    fn process_block(&mut self, block: &'static Block) {
        self.table_.begin_block(block);

        let op_range = self.graph_.operation_indices(block);
        let mut it = op_range.end();
        while it != op_range.begin() {
            it -= 1;
            let index = *it;
            let op = self.graph_.get(index);

            match op.opcode {
                Opcode::kStore => {
                    let store = op.cast::<StoreOp>();
                    let is_on_heap_store = store.kind.tagged_base;
                    let is_field_store = !store.index().valid();
                    let size = store.stored_rep.size_in_bytes();

                    if is_on_heap_store && is_field_store {
                        let mut is_eliminable_store = false;
                        match self.table_.get_observability(store.base(), store.offset, size) {
                            StoreObservability::kUnobservable => {
                                self.eliminable_stores_.as_mut().unwrap().insert(index);
                                self.last_field_initialization_store_ = OpIndex::Invalid();
                                is_eliminable_store = true;
                            }
                            StoreObservability::kGCObservable => {
                                if store.maybe_initializing_or_transitioning {
                                    self.table_.mark_store_as_unobservable(store.base(), store.offset, size);
                                } else {
                                    self.eliminable_stores_.as_mut().unwrap().insert(index);
                                    self.last_field_initialization_store_ = OpIndex::Invalid();
                                    is_eliminable_store = true;
                                }
                            }
                            StoreObservability::kObservable => {
                                self.table_.mark_store_as_unobservable(store.base(), store.offset, size);
                            }
                        }

                        if COMPRESS_POINTERS_BOOL && !is_eliminable_store && store.maybe_initializing_or_transitioning && store.kind == StoreOp::Kind::TaggedBase() && store.write_barrier == WriteBarrierKind::kNoWriteBarrier && store.stored_rep.is_compressible_tagged() {
                            if self.last_field_initialization_store_.valid() && self.graph_.next_index(index) == self.last_field_initialization_store_ {
                                let store0 = store;
                                let store1 = self.graph_.get(self.last_field_initialization_store_).cast::<StoreOp>();

                                assert!(!store0.index().valid());
                                assert!(!store1.index().valid());

                                let c0 = self.graph_.get(store0.value()).try_cast::<ConstantOp>();
                                let c1 = self.graph_.get(store1.value()).try_cast::<ConstantOp>();

                                if let (Some(c0), Some(c1)) = (c0, c1) {
                                    if c0.kind == ConstantOp::Kind::kHeapObject && c1.kind == ConstantOp::Kind::kHeapObject && store1.offset - store0.offset == 4 {
                                        let handle0 = c0.handle();
                                        let handle1 = c1.handle();
                                        if HeapLayout::in_read_only_space(*handle0) && HeapLayout::in_read_only_space(*handle1) {
                                            let high = handle1.ptr() as u32;
                                            let low = handle0.ptr() as u32;
                                            let merged: u64;

                                            #[cfg(target_endian = "big")]
                                            {
                                                merged = ((low as u64) << 32) | (high as u64);
                                            }

                                            #[cfg(target_endian = "little")]
                                            {
                                                merged = ((high as u64) << 32) | (low as u64);
                                            }

                                            self.mergeable_store_pairs_.as_mut().unwrap().insert(index, merged);
                                            self.eliminable_stores_.as_mut().unwrap().insert(self.last_field_initialization_store_);
                                            self.last_field_initialization_store_ = OpIndex::Invalid();
                                        }
                                    }
                                }
                            } else {
                                self.last_field_initialization_store_ = index;
                            }
                        }
                    }
                }
                Opcode::kLoad => {
                    let load = op.cast::<LoadOp>();
                    let is_on_heap_load = load.kind.tagged_base;
                    let is_field_load = !load.index().valid();

                    if is_on_heap_load && is_field_load {
                        self.table_.mark_potentially_aliasing_stores_as_observable(load.base(), load.offset);
                    }
                }
                _ => {
                    let effects = op.effects();
                    if effects.can_read_mutable_memory() {
                        self.table_.mark_all_stores_as_observable();
                    } else if effects.requires_consistent_heap() {
                        self.table_.mark_all_stores_as_gc_observable();
                    }
                }
            }
        }
    }
}

pub struct StoreStoreEliminationReducer<Next> {
    next: Next,
    analysis_: RedundantStoreAnalysis,
    eliminable_stores_: HashSet<OpIndex>,
    mergeable_store_pairs_: HashMap<OpIndex, u64>,
}

impl<Next> StoreStoreEliminationReducer<Next> {
    pub fn new(next: Next, asm: &Assembler) -> Self {
        StoreStoreEliminationReducer {
            next,
            analysis_: RedundantStoreAnalysis::new(asm.input_graph(), asm.phase_zone()),
            eliminable_stores_: HashSet::new(),
            mergeable_store_pairs_: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, asm: &Assembler) {
        self.analysis_.run(&mut self.eliminable_stores_, &mut self.mergeable_store_pairs_);
        self.next.analyze(asm);
    }

    pub fn reduce_input_graph_store(&mut self, ig_index: OpIndex, store: &StoreOp, asm: &mut Assembler) -> OpIndex {
        if self.eliminable_stores_.contains(&ig_index) {
            OpIndex::Invalid()
        } else if self.mergeable_store_pairs_.contains_key(&ig_index) {
            assert!(COMPRESS_POINTERS_BOOL);
            let merged_value = self.mergeable_store_pairs_[&ig_index];
            let value = asm.word64_constant(merged_value);
            asm.store(
                asm.map_to_new_graph(store.base()),
                value,
                StoreOp::Kind::TaggedBase(),
                MemoryRepresentation::Uint64(),
                WriteBarrierKind::kNoWriteBarrier,
                store.offset,
            );
            OpIndex::Invalid()
        } else {
            self.next.reduce_input_graph_store(ig_index, store, asm)
        }
    }
}

pub mod define_assembler_macros_inc {
    // Since define-assembler-macros.inc is empty, we don't need to define anything here.
}

pub struct ZoneIntrusiveSet<K, GetIndex> {
    zone: *mut Zone,
    set: HashSet<K>,
    get_index: std::marker::PhantomData<GetIndex>,
}

impl<K: Eq + Hash + Copy, GetIndex> ZoneIntrusiveSet<K, GetIndex> {
    pub fn new(zone: *mut Zone) -> Self {
        ZoneIntrusiveSet {
            zone,
            set: HashSet::new(),
            get_index: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, key: K) {
        self.set.insert(key);
    }

    pub fn remove(&mut self, key: &K) {
        self.set.remove(key);
    }

    pub fn contains(&self, key: &K) -> bool {
        self.set.contains(key)
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<K> {
        self.set.iter()
    }
}
}
