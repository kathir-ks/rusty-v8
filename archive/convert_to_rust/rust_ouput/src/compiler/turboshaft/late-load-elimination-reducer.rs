// Converted from V8 C++ source files:
// Header: late-load-elimination-reducer.h
// Implementation: late-load-elimination-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod late_load_elimination_reducer {
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::ops::BitAnd;
use std::rc::Rc;

use crate::base::doubly_threaded_list::List as DoublyThreadedList;
use crate::compiler::turboshaft::analyzer_iterator::AnalyzerIterator;
use crate::compiler::turboshaft::assembler::Assembler;
use crate::compiler::turboshaft::graph::Graph;
use crate::compiler::turboshaft::index::OpIndex;
use crate::compiler::turboshaft::loop_finder::LoopFinder;
use crate::compiler::turboshaft::operations::{
    AllocateOp, AssumeMapOp, CallOp, ChangeOp, ConstantOp, GotoOp, LoadOp, Operation,
    StoreOp, FrameStateOp, TaggedBitcastOp, WordBinopOp,
};
use crate::compiler::turboshaft::opmasks::ExternalReference;
use crate::compiler::turboshaft::phase::PipelineData;
use crate::compiler::turboshaft::representations::{
    MemoryRepresentation, RegisterRepresentation,
};
use crate::compiler::turboshaft::sidetable::FixedOpIndexSidetable;
use crate::compiler::turboshaft::snapshot_table_opindex::{
    ChangeTrackingSnapshotTable, SparseOpIndexSnapshotTable, SnapshotTableKey,
};
use crate::execution::isolate::Isolate;
use crate::objects::code_inl::Builtin;
use crate::zone::zone_containers::ZoneRefSet;
use crate::zone::zone::Zone;

pub const TURBOSHAFT_TRACE_LOAD_ELIMINATION: bool = false;

#[macro_export]
macro_rules! TRACE {
    ($x:expr) => {
        if $crate::late_load_elimination_reducer::TURBOSHAFT_TRACE_LOAD_ELIMINATION {
            println!("{}", $x);
        }
    };
}

pub type MapMask = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapMaskAndOr {
    pub or_: MapMask,
    pub and_: MapMask,
}

impl MapMaskAndOr {
    pub fn new() -> Self {
        MapMaskAndOr { or_: 0, and_: u64::MAX }
    }
}

impl Default for MapMaskAndOr {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_empty(minmax: MapMaskAndOr) -> bool {
    minmax.or_ == 0 && minmax.and_ == u64::MAX
}

pub fn compute_map_hash(map: &MapRef) -> MapMask {
    let mut hasher = DefaultHasher::new();
    map.hash_value().hash(&mut hasher);
    let mut hash = hasher.finish();
    hash ^= hash >> 12;
    hash ^= hash << 25;
    hash ^= hash >> 27;
    (hash as u64) * 0x2545f4914f6cdd1d
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

pub fn could_have_same_map(a: MapMaskAndOr, b: MapMaskAndOr) -> bool {
    ((a.and_ & b.or_) == a.and_) || ((b.and_ & a.or_) == b.and_)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryAddress {
    pub base: OpIndex,
    pub index: OptionalOpIndex,
    pub offset: i32,
    pub element_size_log2: u8,
    pub size: u8,
}

impl MemoryAddress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.index.hash(state);
        self.offset.hash(state);
        self.element_size_log2.hash(state);
        self.size.hash(state);
    }
}

impl std::fmt::Display for MemoryAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoryAddress{{base={}, index={:?}, offset={}, elem_size_log2={}, size={}}}",
            self.base, self.index, self.offset, self.element_size_log2, self.size
        )
    }
}

#[derive(Debug)]
pub struct KeyData {
    pub mem: MemoryAddress,
    pub prev_same_base: *mut Key,
    pub next_same_base: SnapshotTableKey<OpIndex, KeyData>,
    pub prev_same_offset: *mut Key,
    pub next_same_offset: SnapshotTableKey<OpIndex, KeyData>,
}

#[derive(Debug)]
pub struct OffsetListTraits {}
impl OffsetListTraits {
    pub type T = SnapshotTable<OpIndex, KeyData>::Key;
    pub fn prev(t: &Self::T) -> *mut *mut Self::T {
        unsafe { &mut (*(t as *const _ as *const Key)).data.prev_same_offset }
    }
    pub fn next(t: &Self::T) -> *mut Self::T {
        unsafe { &mut (*(t as *const _ as *const Key)).data.next_same_offset }
    }
    pub fn non_empty(t: &Self::T) -> bool {
        t.valid()
    }
}

#[derive(Debug)]
pub struct BaseListTraits {}
impl BaseListTraits {
    pub type T = SnapshotTable<OpIndex, KeyData>::Key;
    pub fn prev(t: &Self::T) -> *mut *mut Self::T {
        unsafe { &mut (*(t as *const _ as *const Key)).data.prev_same_base }
    }
    pub fn next(t: &Self::T) -> *mut Self::T {
        unsafe { &mut (*(t as *const _ as *const Key)).data.next_same_base }
    }
    pub fn non_empty(t: &Self::T) -> bool {
        t.valid()
    }
}

#[derive(Debug)]
pub struct BaseData {
    pub with_offsets: DoublyThreadedList<Key, BaseListTraits>,
    pub with_indices: DoublyThreadedList<Key, BaseListTraits>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadEliminationReplacementKind {
    kNone,
    kLoadElimination,
    kTaggedLoadToInt32Load,
    kTaggedBitcastElimination,
    kInt32TruncationElimination,
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

pub fn is_int32_truncated_load_pattern(
    graph: &Graph,
    change_idx: OpIndex,
    change: &ChangeOp,
    bitcast_idx: Option<&mut OpIndex>,
    load_idx: Option<&mut OpIndex>,
) -> bool {
    if !change.is::<TruncateWord64ToWord32>() {
        return false;
    }
    let bitcast = graph
        .get(change.input())
        .try_cast::<BitcastTaggedToWordPtrForTagAndSmiBits>();
    if bitcast.is_none() {
        return false;
    }
    if !bitcast.unwrap().saturated_use_count.is_one() {
        return false;
    }
    let load = graph.get(bitcast.unwrap().input()).try_cast::<LoadOp>();
    if load.is_none() {
        return false;
    }
    if load.unwrap().loaded_rep.size_in_bytes_log2()
        != MemoryRepresentation::Int32().size_in_bytes_log2()
    {
        return false;
    }

    if let Some(idx) = bitcast_idx {
        *idx = change.input();
    }
    if let Some(idx) = load_idx {
        *idx = bitcast.unwrap().input();
    }
    true
}

pub type Key = SnapshotTableKey<OpIndex, KeyData>;

#[derive(Debug)]
pub struct MemoryContentTable<'a> {
    pub change_tracking_snapshot_table: ChangeTrackingSnapshotTable<OpIndex, KeyData>,
    pub non_aliasing_objects_: &'a SparseOpIndexSnapshotTable<bool>,
    pub object_maps_: &'a SparseOpIndexSnapshotTable<MapMaskAndOr>,
    pub replacements_: &'a FixedOpIndexSidetable<LoadEliminationReplacement>,
    pub all_keys_: ZoneAbslFlatHashMap<MemoryAddress, Key>,
    pub base_keys_: ZoneAbslFlatHashMap<OpIndex, BaseData>,
    pub offset_keys_: ZoneAbslFlatHashMap<i32, DoublyThreadedList<Key, OffsetListTraits>>,
    pub index_keys_: DoublyThreadedList<Key, OffsetListTraits>,
}

impl<'a> MemoryContentTable<'a> {
    pub const K_MAX_KEYS: usize = 10000;
    pub type Replacement = LoadEliminationReplacement;
    pub fn new(
        zone: &mut Zone,
        non_aliasing_objects: &'a SparseOpIndexSnapshotTable<bool>,
        object_maps: &'a SparseOpIndexSnapshotTable<MapMaskAndOr>,
        replacements: &'a FixedOpIndexSidetable<LoadEliminationReplacement>,
    ) -> Self {
        MemoryContentTable {
            change_tracking_snapshot_table: ChangeTrackingSnapshotTable::new(zone),
            non_aliasing_objects_: non_aliasing_objects,
            object_maps_: object_maps,
            replacements_: replacements,
            all_keys_: ZoneAbslFlatHashMap::new(zone),
            base_keys_: ZoneAbslFlatHashMap::new(zone),
            offset_keys_: ZoneAbslFlatHashMap::new(zone),
            index_keys_: DoublyThreadedList::new(),
        }
    }

    pub fn on_new_key(&mut self, key: Key, value: OpIndex) {
        if value.valid() {
            self.add_key_in_base_offset_maps(key);
        }
    }

    pub fn on_value_change(&mut self, key: Key, old_value: OpIndex, new_value: OpIndex) {
        assert_ne!(old_value, new_value);
        if old_value.valid() && !new_value.valid() {
            self.remove_key_from_base_offset_maps(key);
        } else if new_value.valid() && !old_value.valid() {
            self.add_key_in_base_offset_maps(key);
        } else {
            assert_eq!(new_value.valid(), old_value.valid());
        }
    }

    pub fn invalidate(&mut self, store: &StoreOp) {
        self.invalidate_opindex(store.base(), store.index(), store.offset);
    }

    pub fn invalidate_opindex(&mut self, base: OpIndex, index: OptionalOpIndex, offset: i32) {
        TRACE!(format!(
            "> MemoryContentTable: Invalidating based on {}, {:?}, {}",
            base, index, offset
        ));
        let mut base = self.resolve_base(base);

        if self.non_aliasing_objects_.get(base) {
            TRACE!(">> base is non-aliasing");

            if let Some(base_keys) = self.base_keys_.get_mut(&base) {
                let mut it = base_keys.with_offsets.begin();
                while it.valid() {
                    let key = *it;
                    let data = unsafe { &(*(key.ptr() as *const Key)).data };
                    assert_eq!(data.mem.base, base);
                    assert!(!data.mem.index.valid());

                    if index.valid() || offset == data.mem.offset {
                        it = base_keys.with_offsets.remove_at(it);
                        TRACE!(format!(">>> invalidating {}", data.mem));
                        self.change_tracking_snapshot_table.set(key, OpIndex::invalid());
                    } else {
                        it.next();
                    }
                }

                let mut it = base_keys.with_indices.begin();
                while it.valid() {
                    let key = *it;
                    let data = unsafe { &(*(key.ptr() as *const Key)).data };
                    assert!(data.mem.index.valid());

                    it = base_keys.with_indices.remove_at(it);
                    self.change_tracking_snapshot_table.set(key, OpIndex::invalid());
                }
            }
        } else {
            TRACE!(">> base is maybe-aliasing");

            if index.valid() {
                TRACE!(">> Invalidating everything because of valid index");
                return self.invalidate_maybe_aliasing();
            }

            let mut it = self.index_keys_.begin();
            while it.valid() {
                let key = *it;
                it = self.index_keys_.remove_at(it);
                let data = unsafe { &(*(key.ptr() as *const Key)).data };
                TRACE!(format!(">>> Invalidating indexed memory {}", data.mem));
                self.change_tracking_snapshot_table.set(key, OpIndex::invalid());
            }

            TRACE!(format!(
                ">>> Invalidating everything maybe-aliasing at offset {}",
                offset
            ));
            self.invalidate_at_offset(offset, base);
        }
    }

    pub fn invalidate_maybe_aliasing(&mut self) {
        TRACE!(">> InvalidateMaybeAliasing");

        let mut to_remove = Vec::new();

        for (base, base_data) in self.base_keys_.iter_mut() {
            if self.non_aliasing_objects_.get(*base) {
                continue;
            }

            let mut it_offsets = base_data.with_offsets.begin();
            while it_offsets.valid() {
                let key = *it_offsets;
                to_remove.push((key, *base, true));
                it_offsets.next();
            }

            let mut it_indices = base_data.with_indices.begin();
            while it_indices.valid() {
                let key = *it_indices;
                to_remove.push((key, *base, false));
                it_indices.next();
            }
        }

        for (key, base, is_offset) in to_remove {
            let base_data = self.base_keys_.get_mut(&base).unwrap();
            if is_offset {
                base_data.with_offsets.remove(key);
            } else {
                base_data.with_indices.remove(key);
            }
            TRACE!(format!(
                ">>> Invalidating {}",
                unsafe { (*(key.ptr() as *const Key)).data }.mem
            ));
            self.change_tracking_snapshot_table.set(key, OpIndex::invalid());
        }
    }

    pub fn find(&mut self, load: &LoadOp) -> OpIndex {
        let base = self.resolve_base(load.base());
        let index = load.index();
        let offset = load.offset;
        let element_size_log2 = if index.valid() { load.element_size_log2 } else { 0 };
        let size = load.loaded_rep.size_in_bytes();

        let mem = MemoryAddress {
            base,
            index,
            offset,
            element_size_log2,
            size,
        };

        if let Some(key) = self.all_keys_.get(&mem) {
            self.change_tracking_snapshot_table.get(*key)
        } else {
            OpIndex::invalid()
        }
    }

    pub fn insert(&mut self, store: &StoreOp) {
        let base = self.resolve_base(store.base());
        let index = store.index();
        let offset = store.offset;
        let element_size_log2 = if index.valid() { store.element_size_log2 } else { 0 };
        let value = store.value();
        let size = store.stored_rep.size_in_bytes();
        if store.kind.is_immutable {
            self.insert_immutable(base, index, offset, element_size_log2, size, value);
        } else {
            self.insert_mutable(base, index, offset, element_size_log2, size, value);
        }
    }

    pub fn insert_load(&mut self, load: &LoadOp, load_idx: OpIndex) {
        let base = self.resolve_base(load.base());
        let index = load.index();
        let offset = load.offset;
        let element_size_log2 = if index.valid() { load.element_size_log2 } else { 0 };
        let size = load.loaded_rep.size_in_bytes();

        if load.kind.is_immutable {
            self.insert_immutable(base, index, offset, element_size_log2, size, load_idx);
        } else {
            self.insert_mutable(base, index, offset, element_size_log2, size, load_idx);
        }
    }

    fn insert_mutable(
        &mut self,
        base: OpIndex,
        index: OptionalOpIndex,
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
            "> MemoryContentTable: will insert {} with value={}",
            mem, value
        ));
        if let Some(existing_key) = self.all_keys_.get(&mem) {
            TRACE!(">> Reusing existing key");
            self.change_tracking_snapshot_table.set(*existing_key, value);
            return;
        }

        if self.all_keys_.len() > Self::K_MAX_KEYS {
            TRACE!(">> Bailing out because too many keys");
            return;
        }

        let mut key = self.change_tracking_snapshot_table.new_key(KeyData {
            mem,
            prev_same_base: std::ptr::null_mut(),
            next_same_base: SnapshotTableKey::invalid(),
            prev_same_offset: std::ptr::null_mut(),
            next_same_offset: SnapshotTableKey::invalid(),
        });

        self.all_keys_.insert(mem, key);
        self.change_tracking_snapshot_table.set(key, value);
    }

    fn insert_immutable(
        &mut self,
        base: OpIndex,
        index: OptionalOpIndex,
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
            "> MemoryContentTable: will insert immutable {} with value={}",
            mem, value
        ));
        if let Some(existing_key) = self.all_keys_.get(&mem) {
            TRACE!(">> Reusing existing key");
            self.change_tracking_snapshot_table.set_no_notify(*existing_key, value);
            return;
        }

        if self.all_keys_.len() > Self::K_MAX_KEYS {
            TRACE!(">> Bailing out because too many keys");
            return;
        }

        let mut key = self.change_tracking_snapshot_table.new_key(KeyData {
            mem,
            prev_same_base: std::ptr::null_mut(),
            next_same_base: SnapshotTableKey::invalid(),
            prev_same_offset: std::ptr::null_mut(),
            next_same_offset: SnapshotTableKey::invalid(),
        });

        self.all_keys_.insert(mem, key);
        self.change_tracking_snapshot_table.set_no_notify(key, value);
    }

    fn invalidate_at_offset(&mut self, offset: i32, base: OpIndex) {
        let base_maps = self.object_maps_.get(base);
        if let Some(offset_keys) = self.offset_keys_.get_mut(&offset) {
            let mut it = offset_keys.begin();
            while it.valid() {
                let key = *it;
                let key_ptr = key.ptr() as *const Key;
                let data = unsafe { &(*key_ptr).data };
                assert_eq!(offset, data.mem.offset);

                if self.non_aliasing_objects_.get(data.mem.base) {
                    it.next();
                    continue;
                }

                let this_maps = if data.mem.base == base {
                    base_maps
                } else {
                    self.object_maps_.get(data.mem.base)
                };

                if !is_empty(base_maps) && !is_empty(this_maps) && !could_have_same_map(base_maps, this_maps) {
                    TRACE!(format!(
                        ">>>> InvalidateAtOffset: not invalidating thanks for maps: {}",
                        data.mem
                    ));
                    it.next();
                    continue;
                }
                it = offset_keys.remove_at(it);
                TRACE!(format!(">>>> InvalidateAtOffset: invalidating {}", data.mem));
                self.change_tracking_snapshot_table.set(key, OpIndex::invalid());
            }
        }
    }

    fn resolve_base(&self, mut base: OpIndex) -> OpIndex {
        while self.replacements_.get(base).is_load_elimination() {
            base = self.replacements_.get(base).replacement();
        }
        base
    }

    fn add_key_in_base_offset_maps(&mut self, key: Key) {
        let data = unsafe { &(*(key.ptr() as *const Key)).data };

        let base = data.mem.base;
        match self.base_keys_.get_mut(&base) {
            Some(base_keys) => {
                if data.mem.index.valid() {
                    base_keys.with_indices.push_front(key);
                } else {
                    base_keys.with_offsets.push_front(key);
                }
            }
            None => {
                let mut data_new = BaseData {
                    with_offsets: DoublyThreadedList::new(),
                    with_indices: DoublyThreadedList::new(),
                };
                if data.mem.index.valid() {
                    data_new.with_indices.push_front(key);
                } else {
                    data_new.with_offsets.push_front(key);
                }
                self.base_keys_.insert(base, data_new);
            }
        }

        if data.mem.index.valid() {
            self.index_keys_.push_front(key);
        } else {
            let offset = data.mem.offset;
            match self.offset_keys_.get_mut(&offset) {
                Some(offset_keys) => {
                    offset_keys.push_front(key);
                }
                None => {
                    let mut list = DoublyThreadedList::new();
                    list.push_front(key);
                    self.offset_keys_.insert(offset, list);
                }
            }
        }
    }

    fn remove_key_from_base_offset_maps(&mut self, key: Key) {
        DoublyThreadedList::<Key, BaseListTraits>::remove(key);
        DoublyThreadedList::<Key, OffsetListTraits>::remove(key);
    }
}

type AliasTable = SparseOpIndexSnapshotTable<bool>;
type AliasKey = <AliasTable as SnapshotTableTrait>::Key;
type AliasSnapshot = <AliasTable as SnapshotTableTrait>::Snapshot;

type MapTable = SparseOpIndexSnapshotTable<MapMaskAndOr>;
type MapKey = <MapTable as SnapshotTableTrait>::Key;
type MapSnapshot = <MapTable as SnapshotTableTrait>::Snapshot;

type MemoryKey = <MemoryContentTable<'_> as SnapshotTableTrait>::Key;
type MemorySnapshot = <MemoryContentTable<'_> as SnapshotTableTrait>::Snapshot;

trait SnapshotTableTrait {
    type Key;
    type Snapshot;
}

impl<T, KD> SnapshotTableTrait for SnapshotTable<T, KD> {
    type Key = SnapshotTableKey<T, KD>;
    type Snapshot = <SnapshotTable<T, KD> as ChangeTracking>::Snapshot;
}

trait ChangeTracking {
    type Snapshot;
    fn seal(&mut self) -> Self::Snapshot;
}

impl<T, KD> ChangeTracking for SnapshotTable<T, KD> {
    type Snapshot = <SnapshotTable<T, KD> as ChangeTracking>::Snapshot;
    fn seal(&mut self) -> Self::Snapshot {
        self.end_tracking()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawBaseAssumption {
    kNoInnerPointer,
    kMaybeInnerPointer,
}

#[derive(Debug)]
pub struct LateLoadEliminationAnalyzer<'a> {
    pub data_: &'a PipelineData,
    pub graph_: &'a Graph,
    pub phase_zone_: &'a mut Zone,
    pub broker_: &'a mut JSHeapBroker,
    pub raw_base_assumption_: RawBaseAssumption,
    pub replacements_: FixedOpIndexSidetable<LoadEliminationReplacement>,
    pub non_aliasing_objects_: SparseOpIndexSnapshotTable<bool>,
    pub object_maps_: SparseOpIndexSnapshotTable<MapMaskAndOr>,
    pub memory_: MemoryContentTable<'a>,
    pub block_to_snapshot_mapping_: FixedBlockSidetable<Option<Snapshot>>,
    pub predecessor_alias_snapshots_: ZoneVector<AliasSnapshot>,
    pub predecessor_maps_snapshots_: ZoneVector<MapSnapshot>,
    pub predecessor_memory_snapshots_: ZoneVector<MemorySnapshot>,
    pub int32_truncated_loads_: std::collections::HashMap<
        OpIndex,
        base::SmallMap<std::collections::HashMap<OpIndex, OpIndex>, 4>,
    >,
}

impl<'a> LateLoadEliminationAnalyzer<'a> {
    pub fn new(
        data: &'a PipelineData,
        graph: &'a Graph,
        phase_zone: &'a mut Zone,
        broker: &'a mut JSHeapBroker,
        raw_base_assumption: RawBaseAssumption,
    ) -> Self {
        let mut replacements = FixedOpIndexSidetable::new(graph.op_id_count(), phase_zone, graph);
        let mut non_aliasing_objects = SparseOpIndexSnapshotTable::new(phase_zone);
        let mut object_maps = SparseOpIndexSnapshotTable::new(phase_zone);
        let memory = MemoryContentTable::new(
            phase_zone,
            &non_aliasing_objects,
            &object_maps,
            &replacements,
        );
        let block_to_snapshot_mapping = FixedBlockSidetable::new(graph.block_count(), phase_zone);
        let predecessor_alias_snapshots = ZoneVector::new(phase_zone);
        let predecessor_maps_snapshots = ZoneVector::new(phase_zone);
        let predecessor_memory_snapshots = ZoneVector::new(phase_zone);
        let int32_truncated_loads = std::collections::HashMap::new();

        LateLoadEliminationAnalyzer {
            data_: data,
            graph_: graph,
            phase_zone_: phase_zone,
            broker_: broker,
            raw_base_assumption_: raw_base_assumption,
            replacements_: replacements,
            non_aliasing_objects_: non_aliasing_objects,
            object_maps_: object_maps,
            memory_: memory,
            block_to_snapshot_mapping_: block_to_snapshot_mapping,
            predecessor_alias_snapshots_: predecessor_alias_snapshots,
            predecessor_maps_snapshots_: predecessor_maps_snapshots,
            predecessor_memory_snapshots_: predecessor_memory_snapshots,
            int32_truncated_loads_: int32_truncated_loads,
        }
    }

    pub fn run(&mut self) {
        TRACE!("LateLoadElimination: Starting analysis");
        let mut loop_finder = LoopFinder::new(self.phase_zone_, self.graph_);
        let mut iterator = AnalyzerIterator::new(self.phase_zone_, self.graph_, &mut loop_finder);

        let mut compute_start_snapshot = true;
        while iterator.has_next() {
            let block = iterator.next();

            self.process_block(block, compute_start_snapshot);
            compute_start_snapshot = true;

            if let Some(last) = block.last_operation(self.graph_).try_cast::<GotoOp>() {
                if last.destination.is_loop() && last.destination.last_predecessor() == block {
                    TRACE!(format!("> Considering reprocessing loop header {:?}", last.destination));
                    let loop_header = last.destination;

                    if self.begin_block::<true>(loop_header) {
                        TRACE!(">> Will need to revisit loop");

                        let loop_1st_pred = loop_header
                            .last_predecessor()
                            .neighboring_predecessor();

                        self.finish_block(loop_1st_pred);

                        let pred_snapshots = self.block_to_snapshot_mapping_[loop_1st_pred.index()];
                        self.non_aliasing_objects_.start_new_snapshot(pred_snapshots.as_ref().unwrap().alias_snapshot);
                        self.object_maps_.start_new_snapshot(pred_snapshots.as_ref().unwrap().maps_snapshot);
                        self.memory_.start_new_snapshot(pred_snapshots.as_ref().unwrap().memory_snapshot);

                        iterator.mark_loop_for_revisit();
                        compute_start_snapshot = false;
                    } else {
                        TRACE!(">> No need to revisit loop");
                        self.seal_and_discard();
                    }
                }
            }
        }

        let mut total_use_counts = FixedOpIndexSidetable::new(
            self.graph_.op_id_count(),
            self.phase_zone_,
            self.graph_,
        );

        for (load_idx, truncations) in self.int32_truncated_loads_.iter() {
            total_use_counts[*load_idx] = SaturatredUint8(self.graph_.get(*load_idx).saturated_use_count);
        }

        let to_erase: Vec<_> = self.int32_truncated_loads_.iter().filter_map(|(load_idx, _)|{
                let rep = self.replacements_.get(*load_idx);
                let saturated = total_use_counts[*load_idx];
                let size = truncations.len();

                let keep = match rep.kind_ {
                  LoadEliminationReplacementKind::k
