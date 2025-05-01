pub mod snapshot_table {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::fmt;
    use std::rc::Rc;
    use std::vec::Vec;

    pub struct NoKeyData {}

    pub struct NoChangeCallback {}

    impl NoChangeCallback {
        pub fn invoke<K, V>(&self, _key: K, _old_value: &V, _new_value: &V) {}
    }

    #[derive(Debug)]
    struct SnapshotTableEntry<V, KD> {
        data: KD,
        value: V,
        merge_offset: u32,
        last_merged_predecessor: u32,
    }

    impl<V, KD> SnapshotTableEntry<V, KD> {
        const NO_MERGE_OFFSET: u32 = u32::MAX;
        const NO_MERGED_PREDECESSOR: u32 = u32::MAX;

        fn new(value: V, data: KD) -> Self {
            SnapshotTableEntry {
                data,
                value,
                merge_offset: Self::NO_MERGE_OFFSET,
                last_merged_predecessor: Self::NO_MERGED_PREDECESSOR,
            }
        }
    }

    #[derive(Clone)]
    pub struct SnapshotTableKey<V, KD> {
        entry: Option<Rc<RefCell<SnapshotTableEntry<V, KD>>>>,
    }

    impl<V, KD> SnapshotTableKey<V, KD> {
        fn new(entry: Rc<RefCell<SnapshotTableEntry<V, KD>>>) -> Self {
            SnapshotTableKey { entry: Some(entry) }
        }

        pub fn valid(&self) -> bool {
            self.entry.is_some()
        }

        pub fn data(&self) -> impl std::ops::Deref<Target = KD> {
            struct DataDeref<'a, KD>(&'a KD);
            impl<'a, KD> std::ops::Deref for DataDeref<'a, KD> {
                type Target = KD;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            DataDeref(&self.entry.as_ref().unwrap().borrow().data)
        }

        pub fn data_mut(&self) -> impl std::ops::DerefMut<Target = KD> {
            struct DataDerefMut<'a, KD>(&'a RefCell<KD>);
            impl<'a, KD> std::ops::Deref for DataDerefMut<'a, KD> {
                type Target = KD;
                fn deref(&self) -> &Self::Target {
                    &self.0.borrow()
                }
            }
            impl<'a, KD> std::ops::DerefMut for DataDerefMut<'a, KD> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0.borrow_mut()
                }
            }

            DataDerefMut(unsafe {
                std::mem::transmute::<_, &RefCell<KD>>(
                    &self.entry.as_ref().unwrap().borrow().data,
                )
            })
        }
    }

    impl<V, KD> PartialEq for SnapshotTableKey<V, KD> {
        fn eq(&self, other: &Self) -> bool {
            match (&self.entry, &other.entry) {
                (Some(entry1), Some(entry2)) => Rc::ptr_eq(entry1, entry2),
                (None, None) => true,
                _ => false,
            }
        }
    }

    impl<V, KD> Eq for SnapshotTableKey<V, KD> {}

    struct LogEntry<V, KD> {
        table_entry: Rc<RefCell<SnapshotTableEntry<V, KD>>>,
        old_value: V,
        new_value: V,
    }

    struct SnapshotData {
        parent: Option<Rc<RefCell<SnapshotData>>>,
        depth: u32,
        log_begin: usize,
        log_end: usize,
    }

    impl SnapshotData {
        const INVALID_OFFSET: usize = usize::MAX;

        fn new(parent: Option<Rc<RefCell<SnapshotData>>>, log_begin: usize) -> Self {
            SnapshotData {
                parent,
                depth: parent.as_ref().map_or(0, |p| p.borrow().depth + 1),
                log_begin,
                log_end: Self::INVALID_OFFSET,
            }
        }

        fn common_ancestor(
            self_rc: &Rc<RefCell<SnapshotData>>,
            other_rc: &Rc<RefCell<SnapshotData>>,
        ) -> Rc<RefCell<SnapshotData>> {
            let mut self_data = self_rc.borrow();
            let mut other_data = other_rc.borrow();

            let mut other = other_rc.clone();
            while other_data.depth > self_data.depth {
                other = other_data.parent.as_ref().unwrap().clone();
                other_data = other.borrow();
            }

            let mut this = self_rc.clone();
            while self_data.depth > other_data.depth {
                this = self_data.parent.as_ref().unwrap().clone();
                self_data = this.borrow();
            }

            while !Rc::ptr_eq(&this, &other) {
                this = self_data.parent.as_ref().unwrap().clone();
                other = other_data.parent.as_ref().unwrap().clone();
                self_data = this.borrow();
                other_data = other.borrow();
            }

            this
        }

        fn seal(&mut self, end: usize) {
            assert!(
                !self.is_sealed(),
                "A Snapshot can only be sealed once"
            );
            self.log_end = end;
        }

        fn is_sealed(&self) -> bool {
            self.log_end != Self::INVALID_OFFSET
        }
    }

    #[derive(Clone)]
    pub struct Snapshot {
        data: Option<Rc<RefCell<SnapshotData>>>,
    }

    impl Snapshot {
        fn new(data: Rc<RefCell<SnapshotData>>) -> Self {
            Snapshot { data: Some(data) }
        }

        fn none() -> Self {
            Snapshot { data: None }
        }
    }

    impl PartialEq for Snapshot {
        fn eq(&self, other: &Self) -> bool {
            match (&self.data, &other.data) {
                (Some(data1), Some(data2)) => Rc::ptr_eq(data1, data2),
                (None, None) => true,
                _ => false,
            }
        }
    }

    impl Eq for Snapshot {}

    pub struct MaybeSnapshot {
        data: Option<Rc<RefCell<SnapshotData>>>,
    }

    impl MaybeSnapshot {
        pub fn has_value(&self) -> bool {
            self.data.is_some()
        }

        pub fn value(&self) -> Snapshot {
            assert!(self.has_value());
            Snapshot {
                data: self.data.clone(),
            }
        }

        pub fn set(&mut self, snapshot: Snapshot) {
            self.data = snapshot.data;
        }

        pub fn new(snapshot: Snapshot) -> Self {
            MaybeSnapshot { data: snapshot.data }
        }

        pub fn none() -> Self {
            MaybeSnapshot { data: None }
        }
    }

    pub struct SnapshotTable<V, KD = NoKeyData> {
        table: VecDeque<Rc<RefCell<SnapshotTableEntry<V, KD>>>>,
        snapshots: VecDeque<Rc<RefCell<SnapshotData>>>,
        log: RefCell<Vec<LogEntry<V, KD>>>,
        root_snapshot: Rc<RefCell<SnapshotData>>,
        current_snapshot: Rc<RefCell<SnapshotData>>,
        merging_entries: RefCell<Vec<Rc<RefCell<SnapshotTableEntry<V, KD>>>>>,
        merge_values: RefCell<Vec<V>>,
        path: RefCell<Vec<Rc<RefCell<SnapshotData>>>>,

        #[cfg(debug_assertions)]
        snapshot_was_created_with_merge: RefCell<bool>,
    }

    impl<V, KD> SnapshotTable<V, KD>
    where
        V: Clone + PartialEq + fmt::Debug,
        KD: fmt::Debug,
    {
        pub fn new() -> Self {
            let mut snapshots = VecDeque::new();
            let root_snapshot = Rc::new(RefCell::new(SnapshotData::new(None, 0)));
            snapshots.push_back(root_snapshot.clone());
            root_snapshot.borrow_mut().seal(0);

            SnapshotTable {
                table: VecDeque::new(),
                snapshots,
                log: RefCell::new(Vec::new()),
                root_snapshot: root_snapshot.clone(),
                current_snapshot: root_snapshot,
                merging_entries: RefCell::new(Vec::new()),
                merge_values: RefCell::new(Vec::new()),
                path: RefCell::new(Vec::new()),

                #[cfg(debug_assertions)]
                snapshot_was_created_with_merge: RefCell::new(false),
            }
        }

        fn new_snapshot(&self, parent: Option<Rc<RefCell<SnapshotData>>>) -> Rc<RefCell<SnapshotData>> {
            let log_len = self.log.borrow().len();
            let snapshot = Rc::new(RefCell::new(SnapshotData::new(parent, log_len)));
            self.snapshots.push_back(snapshot.clone());
            snapshot
        }

        fn log_entries(&self, s: &Rc<RefCell<SnapshotData>>) -> Vec<LogEntry<V, KD>> {
            let borrowed_log = self.log.borrow();
            let snapshot = s.borrow();
            borrowed_log[snapshot.log_begin..snapshot.log_end].to_vec()
        }

        fn revert_current_snapshot<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, change_callback: &CB) {
            let log_entries = self.log_entries(&self.current_snapshot);
            for entry in log_entries.iter().rev() {
                let mut table_entry = entry.table_entry.borrow_mut();
                assert_eq!(table_entry.value, entry.new_value);
                assert_ne!(entry.new_value, entry.old_value);

                change_callback(
                    SnapshotTableKey::new(entry.table_entry.clone()),
                    &entry.new_value,
                    &entry.old_value,
                );
                table_entry.value = entry.old_value.clone();
            }
            let current_snapshot_borrowed = self.current_snapshot.borrow();
            self.current_snapshot = current_snapshot_borrowed.parent.as_ref().unwrap().clone();
        }

        fn replay_snapshot<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, snapshot: &Rc<RefCell<SnapshotData>>, change_callback: &CB) {
            let log_entries = self.log_entries(&snapshot);
            let snapshot_borrowed = snapshot.borrow();
            assert!(Rc::ptr_eq(
                self.current_snapshot.borrow().parent.as_ref().unwrap(),
                &snapshot_borrowed.parent.as_ref().unwrap()
            ));

            for entry in log_entries.iter() {
                let mut table_entry = entry.table_entry.borrow_mut();
                assert_eq!(table_entry.value, entry.old_value);
                assert_ne!(entry.new_value, entry.old_value);

                change_callback(
                    SnapshotTableKey::new(entry.table_entry.clone()),
                    &entry.old_value,
                    &entry.new_value,
                );
                table_entry.value = entry.new_value.clone();
            }
            self.current_snapshot = snapshot.clone();
        }

        fn record_merge_value(
            &self,
            entry: &Rc<RefCell<SnapshotTableEntry<V, KD>>>,
            value: &V,
            predecessor_index: u32,
            predecessor_count: u32,
        ) {
            let mut entry_borrowed = entry.borrow_mut();
            if predecessor_index == entry_borrowed.last_merged_predecessor {
                assert_ne!(entry_borrowed.merge_offset, SnapshotTableEntry::<V, KD>::NO_MERGE_OFFSET);
                return;
            }

            if entry_borrowed.merge_offset == SnapshotTableEntry::<V, KD>::NO_MERGE_OFFSET {
                assert_eq!(
                    entry_borrowed.last_merged_predecessor,
                    SnapshotTableEntry::<V, KD>::NO_MERGED_PREDECESSOR
                );

                let mut merge_values = self.merge_values.borrow_mut();
                assert!(
                    (merge_values.len() as u32)
                        .checked_add(predecessor_count)
                        .map_or(false, |sum| sum <= u32::MAX)
                );

                entry_borrowed.merge_offset = merge_values.len() as u32;
                self.merging_entries.borrow_mut().push(entry.clone());

                let parent_value = entry_borrowed.value.clone();
                merge_values.extend(std::iter::repeat(parent_value).take(predecessor_count as usize));
            }

            let mut merge_values = self.merge_values.borrow_mut();
            merge_values[entry_borrowed.merge_offset as usize + predecessor_index as usize] = value.clone();
            entry_borrowed.last_merged_predecessor = predecessor_index;
        }

        fn move_to_new_snapshot<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, predecessors: &[Snapshot], change_callback: &CB) -> Rc<RefCell<SnapshotData>> {
            assert!(
                self.is_sealed(),
                "A new Snapshot was opened before the previous Snapshot was sealed"
            );

            let common_ancestor = if predecessors.is_empty() {
                self.root_snapshot.clone()
            } else {
                let mut common_ancestor = predecessors.first().unwrap().data.clone().unwrap();
                for s in &predecessors[1..] {
                    common_ancestor = SnapshotData::common_ancestor(&common_ancestor, &s.data.clone().unwrap());
                }
                common_ancestor
            };

            let go_back_to = SnapshotData::common_ancestor(&common_ancestor, &self.current_snapshot);

            while !Rc::ptr_eq(&self.current_snapshot, &go_back_to) {
                self.revert_current_snapshot(change_callback);
            }

            {
                let mut path = self.path.borrow_mut();
                path.clear();

                let mut s = common_ancestor.clone();
                while !Rc::ptr_eq(&s, &go_back_to) {
                    let s_borrowed = s.borrow();
                    path.push(s.clone());
                    s = s_borrowed.parent.as_ref().unwrap().clone();
                }

                for s in path.iter().rev() {
                    self.replay_snapshot(s, change_callback);
                }
            }

            assert!(Rc::ptr_eq(&self.current_snapshot, &common_ancestor));
            let new_snapshot = self.new_snapshot(Some(common_ancestor));
            self.current_snapshot = new_snapshot.clone();
            new_snapshot
        }

        fn merge_predecessors<
            MF: Fn(SnapshotTableKey<V, KD>, &[V]) -> V,
            CB: Fn(SnapshotTableKey<V, KD>, &V, &V),
        >(
            &self,
            predecessors: &[Snapshot],
            merge_fun: &MF,
            change_callback: &CB,
        ) {
            assert!((predecessors.len() as u32) <= u32::MAX);
            let predecessor_count = predecessors.len() as u32;
            if predecessor_count < 1 {
                return;
            }

            assert!(self.merge_values.borrow().is_empty());
            assert!(self.merging_entries.borrow().is_empty());
            let common_ancestor = self.current_snapshot.borrow().parent.clone().unwrap();

            for i in 0..predecessor_count {
                let mut predecessor_data = predecessors[i as usize].data.clone().unwrap();
                while !Rc::ptr_eq(&predecessor_data, &common_ancestor) {
                    let log_entries = self.log_entries(&predecessor_data);
                    for entry in log_entries.iter().rev() {
                        self.record_merge_value(
                            &entry.table_entry,
                            &entry.new_value,
                            i,
                            predecessor_count,
                        );
                    }
                    predecessor_data = predecessor_data.borrow().parent.as_ref().unwrap().clone();
                }
            }

            for entry in self.merging_entries.borrow().iter() {
                let key = SnapshotTableKey::new(entry.clone());
                let entry_borrowed = entry.borrow();
                let merge_values = self.merge_values.borrow();
                let values: &[V] = &merge_values[entry_borrowed.merge_offset as usize
                    ..(entry_borrowed.merge_offset + predecessor_count) as usize];
                let value = merge_fun(key.clone(), values);

                let old_value = entry_borrowed.value.clone();
                if self.set(key.clone(), value) {
                    change_callback(key, &old_value, &entry.borrow().value);
                }
            }
        }

        pub fn start_new_snapshot<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, predecessors: &[Snapshot], change_callback: &CB) {
            assert!(std::any::TypeId::of::<(CB)>() != std::any::TypeId::of::<NoChangeCallback>());
            assert!(self.current_snapshot.borrow().is_sealed());
            self.move_to_new_snapshot(predecessors, change_callback);

            #[cfg(debug_assertions)]
            {
                *self.snapshot_was_created_with_merge.borrow_mut() = false;
            }
        }

        pub fn start_new_snapshot_il<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, predecessors: &[Snapshot], change_callback: &CB) {
            self.start_new_snapshot(predecessors, change_callback);
        }

        pub fn start_new_snapshot_single<CB: Fn(SnapshotTableKey<V, KD>, &V, &V)>(&self, parent: Snapshot, change_callback: &CB) {
            self.start_new_snapshot(&[parent], change_callback);
        }

        pub fn start_new_snapshot_merged<
            MF: Fn(SnapshotTableKey<V, KD>, &[V]) -> V,
            CB: Fn(SnapshotTableKey<V, KD>, &V, &V),
        >(
            &self,
            predecessors: &[Snapshot],
            merge_fun: &MF,
            change_callback: &CB,
        ) {
            assert!(std::any::TypeId::of::<(MF)>() != std::any::TypeId::of::<NoChangeCallback>());
            self.start_new_snapshot(predecessors, change_callback);
            self.merge_predecessors(predecessors, merge_fun, change_callback);

            #[cfg(debug_assertions)]
            {
                *self.snapshot_was_created_with_merge.borrow_mut() = true;
            }
        }

        pub fn start_new_snapshot_merged_il<
            MF: Fn(SnapshotTableKey<V, KD>, &[V]) -> V,
            CB: Fn(SnapshotTableKey<V, KD>, &V, &V),
        >(
            &self,
            predecessors: &[Snapshot],
            merge_fun: &MF,
            change_callback: &CB,
        ) {
            self.start_new_snapshot_merged(predecessors, merge_fun, change_callback);
        }

        pub fn seal(&self) -> Snapshot {
            let log_size = self.log.borrow().len();
            self.current_snapshot.borrow_mut().seal(log_size);

            for entry in self.merging_entries.borrow().iter() {
                let mut entry_borrowed = entry.borrow_mut();
                entry_borrowed.last_merged_predecessor = SnapshotTableEntry::<V, KD>::NO_MERGED_PREDECESSOR;
                entry_borrowed.merge_offset = SnapshotTableEntry::<V, KD>::NO_MERGE_OFFSET;
            }
            self.merge_values.borrow_mut().clear();
            self.merging_entries.borrow_mut().clear();

            if self.current_snapshot.borrow().log_begin == self.current_snapshot.borrow().log_end {
                let parent = self.current_snapshot.borrow().parent.clone().unwrap();
                assert!(Rc::ptr_eq(
                    self.snapshots.back().unwrap(),
                    &self.current_snapshot
                ));
                self.snapshots.pop_back();
                self.current_snapshot = parent.clone();
                return Snapshot::new(parent);
            }
            Snapshot::new(self.current_snapshot.clone())
        }

        pub fn get(&self, key: &SnapshotTableKey<V, KD>) -> V {
            key.entry.as_ref().unwrap().borrow().value.clone()
        }

        pub fn get_predecessor_value(&self, key: &SnapshotTableKey<V, KD>, predecessor_index: usize) -> V {
            assert!(!self.current_snapshot.borrow().is_sealed());

            #[cfg(debug_assertions)]
            assert!(*self.snapshot_was_created_with_merge.borrow());

            let entry = key.entry.as_ref().unwrap();
            if entry.borrow().merge_offset == SnapshotTableEntry::<V, KD>::NO_MERGE_OFFSET {
                return self.get(key);
            }

            let merge_values = self.merge_values.borrow();
            merge_values[entry.borrow().merge_offset as usize + predecessor_index].clone()
        }

        pub fn set(&self, key: SnapshotTableKey<V, KD>, new_value: V) -> bool {
            assert!(!self.current_snapshot.borrow().is_sealed());

            let entry = key.entry.as_ref().unwrap().clone();
            if entry.borrow().value == new_value {
                return false;
            }

            let old_value = entry.borrow().value.clone();
            self.log.borrow_mut().push(LogEntry {
                table_entry: entry.clone(),
                old_value: old_value.clone(),
                new_value: new_value.clone(),
            });
            entry.borrow_mut().value = new_value;
            true
        }

        pub fn new_key(&self, data: KD, initial_value: V) -> SnapshotTableKey<V, KD> {
            let entry = Rc::new(RefCell::new(SnapshotTableEntry::new(initial_value, data)));
            self.table.push_back(entry.clone());
            SnapshotTableKey::new(entry)
        }

        pub fn new_key_simple(&self, initial_value: V) -> SnapshotTableKey<V, KD>
        where
            KD: Default,
        {
            self.new_key(KD::default(), initial_value)
        }

        pub fn is_sealed(&self) -> bool {
            self.current_snapshot.borrow().is_sealed()
        }
    }

    pub trait ChangeTracking {
        type Value;
        type KeyData;
        fn on_new_key(&mut self, key: SnapshotTableKey<Self::Value, Self::KeyData>, initial_value: &Self::Value);
        fn on_value_change(&mut self, key: SnapshotTableKey<Self::Value, Self::KeyData>, old_value: &Self::Value, new_value: &Self::Value);
    }

    pub struct ChangeTrackingSnapshotTable<D, V, KD = NoKeyData>
    where
        D: ChangeTracking<Value = V, KeyData = KD>,
    {
        super_table: SnapshotTable<V, KD>,
        derived: D,
    }

    impl<D, V, KD> ChangeTrackingSnapshotTable<D, V, KD>
    where
        D: ChangeTracking<Value = V, KeyData = KD>,
        V: Clone + PartialEq + fmt::Debug,
        KD: fmt::Debug + Default,
    {
        pub fn new(derived: D) -> Self {
            ChangeTrackingSnapshotTable {
                super_table: SnapshotTable::new(),
                derived,
            }
        }

        pub fn start_new_snapshot(&mut self, predecessors: &[Snapshot]) {
            let derived = unsafe { &mut *(&mut self.derived as *mut D) };
            self.super_table.start_new_snapshot(predecessors, &|key, old_value, new_value| {
                derived.on_value_change(key, old_value, new_value);
            });
        }

        pub fn start_new_snapshot_il(&mut self, predecessors: &[Snapshot]) {
            self.start_new_snapshot(predecessors);
        }

        pub fn start_new_snapshot_single(&mut self, parent: Snapshot) {
            self.start_new_snapshot(&[parent]);
        }

        pub fn start_new_snapshot_merged<MF: Fn(SnapshotTableKey<V, KD>, &[V]) -> V>(&mut self, predecessors: &[Snapshot], merge_fun: &MF) {
            let derived = unsafe { &mut *(&mut self.derived as *mut D) };
            self.super_table.start_new_snapshot_merged(predecessors, merge_fun, &|key, old_value, new_value| {
                derived.on_value_change(key, old_value, new_value);
            });
        }

        pub fn start_new_snapshot_merged_il<MF: Fn(SnapshotTableKey<V, KD>, &[V]) -> V>(&mut self, predecessors: &[Snapshot], merge_fun: &MF) {
            self.start_new_snapshot_merged(predecessors, merge_fun);
        }

        pub fn set(&mut self, key: SnapshotTableKey<V, KD>, new_value: V) {
            let old_value = self.super_table.get(&key);
            if self.super_table.set(key.clone(), new_value) {
                let derived = unsafe { &mut *(&mut self.derived as *mut D) };
                derived.on_value_change(key, &old_value, &self.super_table.get(&key));
            }
        }

        pub fn set_no_notify(&self, key: SnapshotTableKey<V, KD>, new_value: V) {
            self.super_table.set(key, new_value);
        }

        pub fn new_key(&mut self, data: KD, initial_value: V) -> SnapshotTableKey<V, KD> {
            let key = self.super_table.new_key(data, initial_value);
            let derived = unsafe { &mut *(&mut self.derived as *mut D) };
            derived.on_new_key(key.clone(), &self.super_table.get(&key));
            key
        }

        pub fn new_key_simple(&mut self, initial_value: V) -> SnapshotTableKey<V, KD>
        where
            KD: Default,
        {
            self.new_key(KD::default(), initial_value)
        }
    }
}