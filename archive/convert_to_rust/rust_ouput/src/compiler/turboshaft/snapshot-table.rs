// Converted from V8 C++ source files:
// Header: snapshot-table.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::Rc;

pub struct V8 {}

pub struct Value {}

pub struct NoKeyData {}

struct Entry<Key, Value> {}

struct PointerWithPayload<T, const TAG: usize> {}

enum Any {}

pub struct KeyData {}

pub struct Key {}

mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn empty() -> Self {
            Vector { data: Vec::new() }
        }

        pub fn first(&self) -> &T {
            &self.data[0]
        }

        pub fn SubVectorFrom(&self, start: usize) -> Self {
            Vector {
                data: self.data[start..].to_vec(),
            }
        }
    }

    pub struct Reversed<T> {
        data: Vec<T>,
    }

    impl<T> Reversed<T> {
        pub fn new(data: Vec<T>) -> Self {
            let mut reversed_data = data.clone();
            reversed_data.reverse();
            Reversed { data: reversed_data }
        }
    }

    impl<T> IntoIterator for Reversed<T>
    where
        T: Clone,
    {
        type Item = T;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    pub fn VectorOf<T: Clone>(slice: &[T]) -> Vector<T> {
        Vector {
            data: slice.to_vec(),
        }
    }

    impl<T> IntoIterator for Vector<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }
}

mod compiler {
    pub const kNoWriteBarrier: i32 = 0;
}

mod zone {
    use std::{cell::RefCell, rc::Rc};

    pub struct Zone {
        // Placeholder for zone implementation
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }

    pub struct ZoneDeque<T> {
        zone: Zone,
        data: VecDeque<T>,
    }

    impl<T> ZoneDeque<T> {
        pub fn new(zone: &Zone) -> Self {
            ZoneDeque {
                zone: zone.clone(),
                data: VecDeque::new(),
            }
        }

        pub fn emplace_back(&mut self, value: T) -> &mut T {
            self.data.push_back(value);
            self.data.back_mut().unwrap()
        }

        pub fn pop_back(&mut self) {
            self.data.pop_back();
        }

        pub fn back(&self) -> Option<&T> {
            self.data.back()
        }
    }

    impl<T> Clone for ZoneDeque<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            ZoneDeque {
                zone: Zone::new(),
                data: self.data.clone(),
            }
        }
    }

    pub struct ZoneVector<T> {
        zone: Zone,
        data: Vec<T>,
    }

    impl<T> ZoneVector<T> {
        pub fn new(zone: &Zone) -> Self {
            ZoneVector {
                zone: zone.clone(),
                data: Vec::new(),
            }
        }

        pub fn push_back(&mut self, value: T) {
            self.data.push_back(value);
        }

        pub fn insert(&mut self, index: usize, count: usize, value: T)
        where
            T: Clone,
        {
            self.data.splice(index..index, std::iter::repeat(value).take(count));
        }

        pub fn clear(&mut self) {
            self.data.clear();
        }

        pub fn pop_back(&mut self) {
            self.data.pop();
        }

        pub fn resize(&mut self, new_len: usize, value: T)
        where
            T: Clone,
        {
            self.data.resize(new_len, value);
        }
    }

    impl<T> Clone for ZoneVector<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            ZoneVector {
                zone: Zone::new(),
                data: self.data.clone(),
            }
        }
    }
}

struct NoChangeCallback {}

impl NoChangeCallback {
    fn new() -> Self {
        NoChangeCallback {}
    }
}

impl NoChangeCallback {
    fn call<Key, Value>(&self, _key: Key, _old_value: &Value, _new_value: &Value) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SnapshotTableKey<Value, KeyData> {
    entry_: *mut SnapshotTableEntry<Value, KeyData>,
    _phantom: std::marker::PhantomData<(Value, KeyData)>,
}

impl<Value, KeyData> SnapshotTableKey<Value, KeyData> {
    fn new(entry: *mut SnapshotTableEntry<Value, KeyData>) -> Self {
        SnapshotTableKey {
            entry_: entry,
            _phantom: PhantomData,
        }
    }
    fn valid(&self) -> bool {
        self.entry_.is_null() == false
    }
}

struct SnapshotTableEntry<Value, KeyData> {
    value: Value,
    merge_offset: u32,
    last_merged_predecessor: u32,
    key_data: KeyData,
}

impl<Value, KeyData> SnapshotTableEntry<Value, KeyData> {
    const K_NO_MERGE_OFFSET: u32 = u32::MAX;
    const K_NO_MERGED_PREDECESSOR: u32 = u32::MAX;

    fn new(value: Value, key_data: KeyData) -> Self {
        SnapshotTableEntry {
            value,
            merge_offset: Self::K_NO_MERGE_OFFSET,
            last_merged_predecessor: Self::K_NO_MERGED_PREDECESSOR,
            key_data,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Snapshot {
    data_: *mut SnapshotData,
}

impl Snapshot {
    fn new(data_: *mut SnapshotData) -> Self {
        Snapshot { data_ }
    }
}

struct MaybeSnapshot {
    data_: *mut SnapshotData,
}

impl MaybeSnapshot {
    fn new() -> Self {
        MaybeSnapshot {
            data_: std::ptr::null_mut(),
        }
    }
    fn has_value(&self) -> bool {
        self.data_.is_null() == false
    }
    fn value(&self) -> Snapshot {
        assert!(self.has_value());
        Snapshot::new(self.data_)
    }
    fn set(&mut self, snapshot: Snapshot) {
        self.data_ = snapshot.data_;
    }
}

struct LogEntry<Value, KeyData> {
    table_entry: *mut SnapshotTableEntry<Value, KeyData>,
    old_value: Value,
    new_value: Value,
}

struct SnapshotData {
    parent: *mut SnapshotData,
    depth: u32,
    log_begin: usize,
    log_end: usize,
}

impl SnapshotData {
    const K_INVALID_OFFSET: usize = usize::MAX;

    fn new(parent: *mut SnapshotData, log_begin: usize) -> Self {
        let depth = if parent.is_null() {
            0
        } else {
            unsafe { (*parent).depth + 1 }
        };
        SnapshotData {
            parent,
            depth,
            log_begin,
            log_end: Self::K_INVALID_OFFSET,
        }
    }
}

impl SnapshotData {
    fn common_ancestor(&self, other: &SnapshotData) -> *mut SnapshotData {
        let mut self_ptr: *const SnapshotData = self;
        let mut other_ptr: *const SnapshotData = other;

        unsafe {
            while (*other_ptr).depth > (*self_ptr).depth {
                other_ptr = (*other_ptr).parent;
                if other_ptr.is_null() {
                    return self_ptr as *mut SnapshotData;
                }
            }

            while (*self_ptr).depth > (*other_ptr).depth {
                self_ptr = (*self_ptr).parent;
                if self_ptr.is_null() {
                    return other_ptr as *mut SnapshotData;
                }
            }

            while self_ptr != other_ptr {
                self_ptr = (*self_ptr).parent;
                other_ptr = (*other_ptr).parent;

                if self_ptr.is_null() || other_ptr.is_null() {
                    break;
                }
            }
        }

        self_ptr as *mut SnapshotData
    }

    fn seal(&mut self, end: usize) {
        assert!(self.log_end == Self::K_INVALID_OFFSET);
        self.log_end = end;
    }
    fn is_sealed(&self) -> bool {
        self.log_end != Self::K_INVALID_OFFSET
    }
}

struct SnapshotTable<Value, KeyData = NoKeyData> {
    zone_: zone::Zone,
    table_: zone::ZoneDeque<SnapshotTableEntry<Value, KeyData>>,
    snapshots_: zone::ZoneDeque<SnapshotData>,
    log_: zone::ZoneVector<LogEntry<Value, KeyData>>,
    root_snapshot_: *mut SnapshotData,
    current_snapshot_: *mut SnapshotData,
    merging_entries_: zone::ZoneVector<*mut SnapshotTableEntry<Value, KeyData>>,
    merge_values_: zone::ZoneVector<Value>,
    path_: zone::ZoneVector<*mut SnapshotData>,
    snapshot_was_created_with_merge: bool,
    _phantom: std::marker::PhantomData<(Value, KeyData)>,
}

impl<Value, KeyData> SnapshotTable<Value, KeyData>
where
    Value: Clone + PartialEq,
    KeyData: Clone,
{
    fn new(zone: &zone::Zone) -> Self {
        let mut table = SnapshotTable {
            zone_: zone.clone(),
            table_: zone::ZoneDeque::new(zone),
            snapshots_: zone::ZoneDeque::new(zone),
            log_: zone::ZoneVector::new(zone),
            root_snapshot_: std::ptr::null_mut(),
            current_snapshot_: std::ptr::null_mut(),
            merging_entries_: zone::ZoneVector::new(zone),
            merge_values_: zone::ZoneVector::new(zone),
            path_: zone::ZoneVector::new(zone),
            snapshot_was_created_with_merge: false,
            _phantom: PhantomData,
        };

        let root_snapshot = table.new_snapshot(std::ptr::null_mut());
        unsafe {
            (*root_snapshot).seal(0);
        }
        table.root_snapshot_ = root_snapshot;
        table.current_snapshot_ = root_snapshot;

        table
    }

    fn seal(&mut self) -> Snapshot {
        unsafe {
            (*self.current_snapshot_).seal(self.log_.data.len());
        }
        for entry in &mut self.merging_entries_.data {
            unsafe {
                (**entry).last_merged_predecessor =
                    SnapshotTableEntry::<Value, KeyData>::K_NO_MERGED_PREDECESSOR;
                (**entry).merge_offset = SnapshotTableEntry::<Value, KeyData>::K_NO_MERGE_OFFSET;
            }
        }
        self.merge_values_.clear();
        self.merging_entries_.clear();

        if unsafe { (*self.current_snapshot_).log_begin == (*self.current_snapshot_).log_end } {
            let parent = unsafe { (*self.current_snapshot_).parent };
            assert_eq!(
                self.current_snapshot_,
                self.snapshots_.data.back().unwrap() as *mut SnapshotData
            );
            self.snapshots_.pop_back();
            self.current_snapshot_ = parent;
            return Snapshot::new(parent);
        }

        Snapshot::new(self.current_snapshot_)
    }

    fn get(&self, key: SnapshotTableKey<Value, KeyData>) -> &Value {
        unsafe { &(*(key.entry_)).value }
    }

    fn get_predecessor_value(
        &self,
        key: SnapshotTableKey<Value, KeyData>,
        predecessor_index: i32,
    ) -> &Value {
        assert!(!unsafe { (*self.current_snapshot_).is_sealed() });
        assert!(self.snapshot_was_created_with_merge);
        unsafe {
            if (*(key.entry_)).merge_offset == SnapshotTableEntry::<Value, KeyData>::K_NO_MERGE_OFFSET
            {
                return self.get(key);
            }
            &self.merge_values_.data[(*(key.entry_)).merge_offset as usize + predecessor_index as usize]
        }
    }

    fn set(&mut self, key: SnapshotTableKey<Value, KeyData>, new_value: Value) -> bool {
        assert!(!unsafe { (*self.current_snapshot_).is_sealed() });
        unsafe {
            if (*(key.entry_)).value == new_value {
                return false;
            }
            let log_entry = LogEntry {
                table_entry: key.entry_,
                old_value: (*(key.entry_)).value.clone(),
                new_value: new_value.clone(),
            };
            self.log_.push_back(log_entry);
            (*(key.entry_)).value = new_value;
            true
        }
    }

    fn new_key(&mut self, data: KeyData, initial_value: Value) -> SnapshotTableKey<Value, KeyData> {
        let entry = self
            .table_
            .emplace_back(SnapshotTableEntry::new(initial_value.clone(), data));
        SnapshotTableKey::new(entry as *mut SnapshotTableEntry<Value, KeyData>)
    }
    fn new_key_without_data(&mut self, initial_value: Value) -> SnapshotTableKey<Value, KeyData>
    where
        KeyData: Default,
    {
        self.new_key(KeyData::default(), initial_value)
    }

    fn is_sealed(&self) -> bool {
        unsafe { (*self.current_snapshot_).is_sealed() }
    }

    fn new_snapshot(&mut self, parent: *mut SnapshotData) -> *mut SnapshotData {
        self.snapshots_.emplace_back(SnapshotData::new(parent, self.log_.data.len()))
            as *mut SnapshotData
    }

    fn log_entries(&self, s: &SnapshotData) -> base::Vector<LogEntry<Value, KeyData>> {
        base::Vector::new(
            self.log_.data[s.log_begin..s.log_end]
                .to_vec()
        )
    }

    fn revert_current_snapshot<ChangeCallback>(&mut self, change_callback: &mut ChangeCallback)
    where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        assert!(unsafe { (*self.current_snapshot_).is_sealed() });
        let log_entries = self.log_entries(unsafe { &(*self.current_snapshot_) });
        for entry in base::Reversed::new(log_entries.data) {
            unsafe {
                assert_eq!((*(entry.table_entry)).value, entry.new_value);
                assert_ne!(entry.new_value, entry.old_value);
                change_callback(
                    SnapshotTableKey::new(entry.table_entry),
                    &entry.new_value,
                    &entry.old_value,
                );
                (*(entry.table_entry)).value = entry.old_value;
            }
        }
        self.current_snapshot_ = unsafe { (*self.current_snapshot_).parent };
        assert!(self.current_snapshot_.is_null() == false);
    }

    fn replay_snapshot<ChangeCallback>(
        &mut self,
        snapshot: *mut SnapshotData,
        change_callback: &mut ChangeCallback,
    ) where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        unsafe {
            assert_eq!((*snapshot).parent, self.current_snapshot_);
        }
        let log_entries = self.log_entries(unsafe { &(*snapshot) });
        for entry in log_entries.data {
            unsafe {
                assert_eq!((*(entry.table_entry)).value, entry.old_value);
                assert_ne!(entry.new_value, entry.old_value);
                change_callback(
                    SnapshotTableKey::new(entry.table_entry),
                    &entry.old_value,
                    &entry.new_value,
                );
                (*(entry.table_entry)).value = entry.new_value;
            }
        }
        self.current_snapshot_ = snapshot;
    }

    fn record_merge_value(
        &mut self,
        entry: &mut SnapshotTableEntry<Value, KeyData>,
        value: &Value,
        predecessor_index: u32,
        predecessor_count: u32,
    ) {
        if predecessor_index == entry.last_merged_predecessor {
            assert_ne!(
                entry.merge_offset,
                SnapshotTableEntry::<Value, KeyData>::K_NO_MERGE_OFFSET
            );
            return;
        }

        if entry.merge_offset == SnapshotTableEntry::<Value, KeyData>::K_NO_MERGE_OFFSET {
            assert_eq!(
                entry.last_merged_predecessor,
                SnapshotTableEntry::<Value, KeyData>::K_NO_MERGED_PREDECESSOR
            );
            assert!(
                (self.merge_values_.data.len() as u32) + predecessor_count <= u32::MAX
            );
            entry.merge_offset = self.merge_values_.data.len() as u32;
            self.merging_entries_.push_back(entry);
            self.merge_values_.insert(
                self.merge_values_.data.len(),
                predecessor_count as usize,
                entry.value.clone(),
            );
        }
        self.merge_values_.data[entry.merge_offset as usize + predecessor_index as usize] =
            value.clone();
        entry.last_merged_predecessor = predecessor_index;
    }

    fn move_to_new_snapshot<ChangeCallback>(
        &mut self,
        predecessors: base::Vector<Snapshot>,
        mut change_callback: ChangeCallback,
    ) -> *mut SnapshotData
    where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        assert!(unsafe { (*self.current_snapshot_).is_sealed() });

        let common_ancestor = if predecessors.data.is_empty() {
            self.root_snapshot_
        } else {
            let mut common_ancestor = unsafe { (*predecessors.first().data_).clone() };
            for s in predecessors.SubVectorFrom(1).data {
                unsafe {
                    common_ancestor =
                        *(common_ancestor.common_ancestor(&(*s.data_)))
                };
            }
            common_ancestor.common_ancestor(unsafe { &(*self.current_snapshot_) })
        };

        let mut go_back_to = unsafe { (*self.current_snapshot_).common_ancestor(&(*common_ancestor)) };
        while self.current_snapshot_ != go_back_to {
            self.revert_current_snapshot(&mut change_callback);
        }

        self.path_.clear();
        let mut s = common_ancestor;
        unsafe {
            while s != go_back_to {
                self.path_.push_back(s);
                s = (*(s)).parent;
            }
        }

        for s in base::Reversed::new(self.path_.data.clone()) {
            self.replay_snapshot(s, &mut change_callback);
        }

        assert_eq!(self.current_snapshot_, common_ancestor);
        let new_snapshot = self.new_snapshot(common_ancestor);
        self.current_snapshot_ = new_snapshot;
        new_snapshot
    }

    fn merge_predecessors<MergeFun, ChangeCallback>(
        &mut self,
        predecessors: base::Vector<Snapshot>,
        merge_fun: MergeFun,
        mut change_callback: ChangeCallback,
    ) where
        MergeFun: FnMut(
            SnapshotTableKey<Value, KeyData>,
            base::Vector<&Value>,
        ) -> Value,
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        assert!((predecessors.data.len() as u32) <= u32::MAX);
        let predecessor_count = predecessors.data.len() as u32;
        if predecessor_count < 1 {
            return;
        }

        assert!(self.merge_values_.data.is_empty());
        assert!(self.merging_entries_.data.is_empty());
        let common_ancestor = unsafe { (*self.current_snapshot_).parent };

        for i in 0..predecessor_count {
            let mut predecessor = unsafe { (*predecessors.data[i as usize].data_).clone() };

            unsafe {
                while (*predecessor.parent) as *mut SnapshotData != common_ancestor {
                    let log_entries = self.log_entries(&predecessor);
                    for entry in base::Reversed::new(log_entries.data) {
                        self.record_merge_value(
                            &mut (*(entry.table_entry)),
                            &entry.new_value,
                            i,
                            predecessor_count,
                        );
                    }
                    predecessor = *predecessor.parent;
                }
            }
        }

        for entry in self.merging_entries_.data.clone() {
            let key = SnapshotTableKey::new(entry);
            let mut values = Vec::new();
            unsafe {
                for i in 0..predecessor_count {
                    values.push(&self.merge_values_.data[(*(entry)).merge_offset as usize + i as usize]);
                }
            }

            let value = merge_fun(
                key,
                base::Vector::new(values)
            );
            let old_value = unsafe { (*(entry)).value.clone() };
            if self.set(key, value.clone()) {
                unsafe {
                    change_callback(key, &old_value, &(*(entry)).value);
                }
            }
        }
    }
}

impl<Value: Clone + PartialEq, KeyData: Clone + Default> SnapshotTable<Value, KeyData> {
    fn start_new_snapshot<ChangeCallback>(
        &mut self,
        predecessors: base::Vector<Snapshot>,
        mut change_callback: ChangeCallback,
    ) where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        assert!(unsafe { (*self.current_snapshot_).is_sealed() });
        self.move_to_new_snapshot(predecessors, change_callback);
        self.snapshot_was_created_with_merge = false;
    }
    fn start_new_snapshot_from_list<ChangeCallback>(
        &mut self,
        predecessors: &[Snapshot],
        change_callback: ChangeCallback,
    ) where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        self.start_new_snapshot(base::Vector::VectorOf(predecessors), change_callback);
    }
    fn start_new_snapshot_from_parent<ChangeCallback>(
        &mut self,
        parent: Snapshot,
        change_callback: ChangeCallback,
    ) where
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        self.start_new_snapshot(base::Vector::VectorOf(&[parent]), change_callback);
    }
    fn start_new_snapshot_with_merge<MergeFun, ChangeCallback>(
        &mut self,
        predecessors: base::Vector<Snapshot>,
        merge_fun: MergeFun,
        change_callback: ChangeCallback,
    ) where
        MergeFun: FnMut(
            SnapshotTableKey<Value, KeyData>,
            base::Vector<&Value>,
        ) -> Value,
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        self.start_new_snapshot(predecessors.clone(), change_callback);
        self.merge_predecessors(predecessors, merge_fun, change_callback);
        self.snapshot_was_created_with_merge = true;
    }

    fn start_new_snapshot_from_list_with_merge<MergeFun, ChangeCallback>(
        &mut self,
        predecessors: &[Snapshot],
        merge_fun: MergeFun,
        change_callback: ChangeCallback,
    ) where
        MergeFun: FnMut(
            SnapshotTableKey<Value, KeyData>,
            base::Vector<&Value>,
        ) -> Value,
        ChangeCallback: FnMut(
            SnapshotTableKey<Value, KeyData>,
            &Value,
            &Value,
        ),
    {
        self.start_new_snapshot_with_merge(
            base::Vector::VectorOf(predecessors),
            merge_fun,
            change_callback,
        );
    }
}

trait ChangeTracking {
    type Value;
    type KeyData;
    fn on_new_key(&mut self, key: SnapshotTableKey<Self::Value, Self::KeyData>, initial_value: &Self::Value);
    fn on_value_change(&mut self, key: SnapshotTableKey<Self::Value, Self::KeyData>, old_value: &Self::Value, new_value: &Self::Value);
}

struct ChangeTrackingSnapshotTable<Derived, Value, KeyData = NoKeyData> {
    super_: SnapshotTable<Value, KeyData>,
    derived_: PhantomData<Derived>,
}

impl<Derived, Value: Clone + PartialEq, KeyData: Clone + Default> ChangeTrackingSnapshotTable<Derived, Value, KeyData>
where
    Derived: ChangeTracking<Value = Value, KeyData = KeyData> + SnapshotTableTrait<Value = Value, KeyData = KeyData>,
{
    fn new(zone: &zone::Zone) -> Self {
        ChangeTrackingSnapshotTable {
            super_: SnapshotTable::new(zone),
            derived_: PhantomData,
        }
    }

    fn start_new_snapshot(&mut self, predecessors: base::Vector<Snapshot>)
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        self.super_.start_new_snapshot(
            predecessors,
            |key, old_value, new_value| {
                <Derived as ChangeTracking>::on_value_change(
                    unsafe { &mut *(self as *mut Self as *mut Derived) },
                    key,
                    old_value,
                    new_value,
                );
            },
        );
    }

    fn start_new_snapshot_from_list(&mut self, predecessors: &[Snapshot])
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        self.start_new_snapshot(base::Vector::VectorOf(predecessors));
    }

    fn start_new_snapshot_from_parent(&mut self, parent: Snapshot)
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        self.start_new_snapshot(base::Vector::VectorOf(&[parent]));
    }

    fn start_new_snapshot_with_merge<MergeFun>(&mut self, predecessors: base::Vector<Snapshot>, merge_fun: MergeFun)
    where
        MergeFun: FnMut(SnapshotTableKey<Value, KeyData>, base::Vector<&Value>) -> Value,
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        self.super_.start_new_snapshot_with_merge(
            predecessors,
            merge_fun,
            |key, old_value, new_value| {
                <Derived as ChangeTracking>::on_value_change(
                    unsafe { &mut *(self as *mut Self as *mut Derived) },
                    key,
                    old_value,
                    new_value,
                );
            },
        );
    }

    fn start_new_snapshot_from_list_with_merge<MergeFun>(&mut self, predecessors: &[Snapshot], merge_fun: MergeFun)
    where
        MergeFun: FnMut(SnapshotTableKey<Value, KeyData>, base::Vector<&Value>) -> Value,
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        self.start_new_snapshot_with_merge(base::Vector::VectorOf(predecessors), merge_fun);
    }

    fn set(&mut self, key: SnapshotTableKey<Value, KeyData>, new_value: Value)
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        let old_value = self.super_.get(key).clone();
        if self.super_.set(key, new_value.clone()) {
            <Derived as ChangeTracking>::on_value_change(
                unsafe { &mut *(self as *mut Self as *mut Derived) },
                key,
                &old_value,
                self.super_.get(key),
            );
        }
    }

    fn set_no_notify(&mut self, key: SnapshotTableKey<Value, KeyData>, new_value: Value) {
        self.super_.set(key, new_value);
    }

    fn new_key(&mut self, data: KeyData, initial_value: Value) -> SnapshotTableKey<Value, KeyData>
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
    {
        let key = self.super_.new_key(data, initial_value.clone());
        <Derived as ChangeTracking>::on_new_key(
            unsafe { &mut *(self as *mut Self as *mut Derived) },
            key,
            self.super_.get(key),
        );
        key
    }

    fn new_key_without_data(&mut self, initial_value: Value) -> SnapshotTableKey<Value, KeyData>
    where
        Derived: ChangeTracking<Value = Value, KeyData = KeyData>,
        KeyData: Default,
    {
        self.new_key(KeyData::default(), initial_value)
    }
}

trait SnapshotTableTrait {
    type Value;
    type KeyData;

    fn get(&self, key: SnapshotTableKey<Self::Value, Self::KeyData>) -> &Self::Value;
}

impl<Value: Clone + PartialEq, KeyData: Clone + Default> SnapshotTableTrait for SnapshotTable<Value, KeyData> {
    type Value = Value;
    type KeyData = KeyData;

    fn get(&self, key: SnapshotTableKey<Self::Value, Self::KeyData>) -> &Self::Value {
        self.get(key)
    }
}
