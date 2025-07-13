// Converted from V8 C++ source files:
// Header: wasm-load-elimination-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod wasm_load_elimination_reducer {
    use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

    use crate::base::doubly_threaded_list::DoublyThreadedList;
    use crate::compiler::turboshaft::{
        analyzer_iterator::AnalyzerIterator, assembler::Assembler, graph::Graph,
        loop_finder::LoopFinder, phase::Phase, snapshot_table_opindex::SnapshotTable,
        utils::V8,
    };
    use crate::compiler::{
        self, compilation_dependencies::Smi, turboshaft::copying_phase::OpIndex,
    };
    use crate::zone::zone::Zone;

    pub struct V8_ENABLE_WEBASSEMBLY {}

    pub mod wle {
        use std::{
            cell::RefCell,
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
            rc::Rc,
        };

        use crate::base::doubly_threaded_list::DoublyThreadedList;
        use crate::compiler::turboshaft::{
            copying_phase::OpIndex, graph::Graph,
            snapshot_table_opindex::SnapshotTableKey,
        };
        use crate::zone::zone::Zone;

        // We model array length and string canonicalization as fields at negative
        // indices.
        pub const K_ARRAY_LENGTH_FIELD_INDEX: i32 = -1;
        pub const K_STRING_PREPARE_FOR_GET_CODEUNIT_INDEX: i32 = -2;
        pub const K_STRING_AS_WTF16_INDEX: i32 = -3;
        pub const K_ANY_CONVERT_EXTERN_INDEX: i32 = -4;
        pub const K_ASSERT_NOT_NULL_INDEX: i32 = -5;

        // All "load-like" special cases use the same fake size and type. The specific
        // values we use don't matter; for accurate alias analysis, the type should
        // be "unrelated" to any struct type.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct ModuleTypeIndex {
            index: usize,
        }

        pub struct WasmModuleTypeIndex {
            heap_type: HeapType,
        }

        impl ModuleTypeIndex {
            pub fn new(index: usize) -> Self {
                ModuleTypeIndex { index }
            }
        }

        pub const K_LOAD_LIKE_SIZE: i32 = 4; // Chosen by fair dice roll.

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct WasmMemoryAddress {
            pub base: OpIndex,
            pub offset: i32,
            pub type_index: ModuleTypeIndex,
            pub size: u8,
            pub mutability: bool,
        }

        pub fn fast_hash_combine<T: Hash>(a: T, b: T, c: T, d: T, e: T) -> usize {
            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            b.hash(&mut hasher);
            c.hash(&mut hasher);
            d.hash(&mut hasher);
            e.hash(&mut hasher);
            hasher.finish() as usize
        }

        #[derive(Clone, Copy)]
        pub struct KeyData {
            pub mem: WasmMemoryAddress,
            // Pointers to the previous and the next Keys at the same base.
            pub prev_same_base: *mut KeyData,
            pub next_same_base: SnapshotTableKey<OpIndex, KeyData>,
            // Pointers to either the next/previous Keys at the same offset.
            pub prev_same_offset: *mut KeyData,
            pub next_same_offset: SnapshotTableKey<OpIndex, KeyData>,
        }

        impl KeyData {
            pub fn new() -> Self {
                KeyData {
                    mem: WasmMemoryAddress {
                        base: OpIndex {},
                        offset: 0,
                        type_index: ModuleTypeIndex { index: 0 },
                        size: 0,
                        mutability: false,
                    },
                    prev_same_base: std::ptr::null_mut(),
                    next_same_base: SnapshotTableKey {
                        index: OpIndex {},
                        data: None,
                    },
                    prev_same_offset: std::ptr::null_mut(),
                    next_same_offset: SnapshotTableKey {
                        index: OpIndex {},
                        data: None,
                    },
                }
            }
        }

        pub struct OffsetListTraits {}

        impl OffsetListTraits {
            pub type T = SnapshotTableKey<OpIndex, KeyData>;
            pub fn prev(t: Self::T) -> *mut *mut Self::T {
                unsafe { &mut (*t.data.unwrap()).prev_same_offset as *mut *mut KeyData as _ }
            }
            pub fn next(t: Self::T) -> *mut Self::T {
                unsafe { &mut (*t.data.unwrap()).next_same_offset as *mut SnapshotTableKey<OpIndex, KeyData> as _ }
            }
            pub fn non_empty(t: Self::T) -> bool {
                t.index.index.is_some()
            }
        }

        pub struct BaseListTraits {}

        impl BaseListTraits {
            pub type T = SnapshotTableKey<OpIndex, KeyData>;
            pub fn prev(t: Self::T) -> *mut *mut Self::T {
                unsafe { &mut (*t.data.unwrap()).prev_same_base as *mut *mut KeyData as _ }
            }
            pub fn next(t: Self::T) -> *mut Self::T {
                unsafe { &mut (*t.data.unwrap()).next_same_base as *mut SnapshotTableKey<OpIndex, KeyData> as _ }
            }
            pub fn non_empty(t: Self::T) -> bool {
                t.index.index.is_some()
            }
        }

        pub struct BaseData {
            pub with_offsets: DoublyThreadedList<SnapshotTableKey<OpIndex, KeyData>, BaseListTraits>,
        }

        impl BaseData {
            pub fn new() -> Self {
                BaseData {
                    with_offsets: DoublyThreadedList::new(),
                }
            }
        }

        pub struct WasmMemoryContentTable {
            change_tracking_snapshot_table: ChangeTrackingSnapshotTable,
            non_aliasing_objects: SparseOpIndexSnapshotTable<bool>,
            replacements: FixedOpIndexSidetable<OpIndex>,
            data: *mut PipelineData,
            graph: *mut Graph,
            all_keys: ZoneUnorderedMap<WasmMemoryAddress, SnapshotTableKey<OpIndex, KeyData>>,
            base_keys: ZoneUnorderedMap<OpIndex, BaseData>,
            offset_keys: ZoneUnorderedMap<i32, DoublyThreadedList<SnapshotTableKey<OpIndex, KeyData>, OffsetListTraits>>,
            module: *const WasmModule,
            zone: *mut Zone,
        }

        impl WasmMemoryContentTable {
            pub fn new(
                data: *mut PipelineData,
                zone: *mut Zone,
                non_aliasing_objects: SparseOpIndexSnapshotTable<bool>,
                replacements: FixedOpIndexSidetable<OpIndex>,
                graph: *mut Graph,
            ) -> Self {
                let module = unsafe { (*data).wasm_module };
                WasmMemoryContentTable {
                    change_tracking_snapshot_table: ChangeTrackingSnapshotTable::new(zone),
                    non_aliasing_objects,
                    replacements,
                    data,
                    graph,
                    all_keys: ZoneUnorderedMap::new(),
                    base_keys: ZoneUnorderedMap::new(),
                    offset_keys: ZoneUnorderedMap::new(),
                    module,
                    zone,
                }
            }

            pub fn on_new_key(&mut self, key: SnapshotTableKey<OpIndex, KeyData>, value: OpIndex) {
                if value.index.is_some() {
                    self.add_key_in_base_offset_maps(key);
                }
            }

            pub fn on_value_change(&mut self, key: SnapshotTableKey<OpIndex, KeyData>, old_value: OpIndex, new_value: OpIndex) {
                if old_value != new_value {
                    if old_value.index.is_some() && new_value.index.is_none() {
                        self.remove_key_from_base_offset_maps(key);
                    } else if new_value.index.is_some() && old_value.index.is_none() {
                        self.add_key_in_base_offset_maps(key);
                    } else {
                        assert_eq!(new_value.index.is_some(), old_value.index.is_some());
                    }
                }
            }

            pub fn types_unrelated(&self, type1: ModuleTypeIndex, type2: ModuleTypeIndex) -> bool {
                HeapTypesUnrelated(
                    unsafe { (*self.module).heap_type(type1) },
                    unsafe { (*self.module).heap_type(type2) },
                    self.module,
                    self.module,
                )
            }

            pub fn invalidate(&mut self, set: &StructSetOp) {
                let offset = self.field_offset(set.type_, set.field_index);
                if let Some(offset_keys) = self.offset_keys.map.get(&offset) {
                    let mut it = offset_keys.begin();
                    while it != offset_keys.end() {
                        let key = *it;
                        assert_eq!(offset, unsafe { (*key.data.unwrap()).mem.offset });
                        let base = unsafe { (*key.data.unwrap()).mem.base };

                        if self.non_aliasing_objects.get(base) {
                            it = offset_keys.remove_at(it);
                            continue;
                        }

                        if self.types_unrelated(set.type_index, unsafe { (*key.data.unwrap()).mem.type_index }) {
                            it = offset_keys.remove_at(it);
                            continue;
                        }
                        self.change_tracking_snapshot_table.set(key, OpIndex { index: None });
                        it = offset_keys.remove_at(it);
                    }
                }
            }

            pub enum EntriesWithOffsets {
                K_INVALIDATE,
                K_KEEP,
            }

            pub fn invalidate_maybe_aliasing(&mut self) {
                for base_keys in self.base_keys.map.iter() {
                    let base = *base_keys.0;
                    if self.non_aliasing_objects.get(base) {
                        continue;
                    }
                    let mut it = base_keys.1.with_offsets.begin();
                    while it != base_keys.1.with_offsets.end() {
                        let key = *it;
                        if unsafe { (*key.data.unwrap()).mem.mutability } == false {
                            it = base_keys.1.with_offsets.remove_at(it);
                            continue;
                        }
                        it = base_keys.1.with_offsets.remove_at(it);
                        self.change_tracking_snapshot_table.set(key, OpIndex { index: None });
                    }
                }
            }

            pub fn field_offset(&self, type_: *const StructType, field_index: i32) -> i32 {
                WasmStruct::k_header_size + unsafe { (*type_).field_offset(field_index) }
            }

            pub fn find(&mut self, get: &StructGetOp) -> OpIndex {
                let offset = self.field_offset(get.type_, get.field_index);
                let size = unsafe { (*get.type_).field(get.field_index).value_kind_size() };
                let mutability = unsafe { (*get.type_).mutability(get.field_index) };
                self.find_impl(self.resolve_base(get.object), offset, get.type_index, size, mutability)
            }

            pub fn has_value_with_incorrect_mutability(&mut self, set: &StructSetOp) -> bool {
                let offset = self.field_offset(set.type_, set.field_index);
                let size = unsafe { (*set.type_).field(set.field_index).value_kind_size() };
                let mutability = unsafe { (*set.type_).mutability(set.field_index) };
                let mem = WasmMemoryAddress {
                    base: self.resolve_base(set.object),
                    offset,
                    type_index: set.type_index,
                    size,
                    mutability: !mutability,
                };
                self.all_keys.map.contains_key(&mem)
            }

            pub fn find_load_like(&mut self, op_idx: OpIndex, offset_sentinel: i32) -> OpIndex {
                const MUTABILITY: bool = false;
                self.find_impl(
                    self.resolve_base(op_idx),
                    offset_sentinel,
                    ModuleTypeIndex { index: 0 },
                    K_LOAD_LIKE_SIZE as u8,
                    MUTABILITY,
                )
            }

            pub fn find_impl(
                &mut self,
                object: OpIndex,
                offset: i32,
                type_index: ModuleTypeIndex,
                size: u8,
                mutability: bool,
            ) -> OpIndex {
                let mem = WasmMemoryAddress {
                    base: object,
                    offset,
                    type_index,
                    size,
                    mutability,
                };
                match self.all_keys.map.get(&mem) {
                    Some(key) => self.change_tracking_snapshot_table.get(*key),
                    None => OpIndex { index: None },
                }
            }

            pub fn insert(&mut self, set: &StructSetOp) {
                let base = self.resolve_base(set.object);
                let offset = self.field_offset(set.type_, set.field_index);
                let size = unsafe { (*set.type_).field(set.field_index).value_kind_size() };
                let mutability = unsafe { (*set.type_).mutability(set.field_index) };
                self.insert_impl(base, offset, set.type_index, size, mutability, set.value);
            }

            pub fn insert_get(&mut self, get: &StructGetOp, get_idx: OpIndex) {
                let base = self.resolve_base(get.object);
                let offset = self.field_offset(get.type_, get.field_index);
                let size = unsafe { (*get.type_).field(get.field_index).value_kind_size() };
                let mutability = unsafe { (*get.type_).mutability(get.field_index) };
                self.insert_impl(base, offset, get.type_index, size, mutability, get_idx);
            }

            pub fn insert_load_like(&mut self, base_idx: OpIndex, offset_sentinel: i32, value_idx: OpIndex) {
                let base = self.resolve_base(base_idx);
                const MUTABILITY: bool = false;
                self.insert_impl(
                    base,
                    offset_sentinel,
                    ModuleTypeIndex { index: 0 },
                    K_LOAD_LIKE_SIZE as u8,
                    MUTABILITY,
                    value_idx,
                );
            }

            fn insert_impl(
                &mut self,
                base: OpIndex,
                offset: i32,
                type_index: ModuleTypeIndex,
                size: u8,
                mutability: bool,
                value: OpIndex,
            ) {
                assert_eq!(base, self.resolve_base(base));

                let mem = WasmMemoryAddress {
                    base,
                    offset,
                    type_index,
                    size,
                    mutability,
                };
                if let Some(existing_key) = self.all_keys.map.get(&mem) {
                    if mutability {
                        self.change_tracking_snapshot_table.set(*existing_key, value);
                    } else {
                        self.change_tracking_snapshot_table.set_no_notify(*existing_key, value);
                    }
                    return;
                }

                let key = SnapshotTableKey {
                    index: OpIndex { index: Some(1) },
                    data: Some(Box::new(KeyData {
                        mem,
                        prev_same_base: std::ptr::null_mut(),
                        next_same_base: SnapshotTableKey {
                            index: OpIndex {},
                            data: None,
                        },
                        prev_same_offset: std::ptr::null_mut(),
                        next_same_offset: SnapshotTableKey {
                            index: OpIndex {},
                            data: None,
                        },
                    })),
                };
                self.all_keys.map.insert(mem, key);
                if mutability {
                    self.change_tracking_snapshot_table.set(key, value);
                } else {
                    self.change_tracking_snapshot_table.set_no_notify(key, value);
                }
            }

            pub fn resolve_base(&mut self, mut base: OpIndex) -> OpIndex {
                loop {
                    if self.replacements.get(base).index.is_some() {
                        base = self.replacements.get(base);
                        continue;
                    }
                    let op = unsafe { &mut (*self.graph).get(base) };
                    if let Some(check) = op.try_cast::<AssertNotNullOp>() {
                        base = check.object;
                        continue;
                    }
                    if let Some(cast) = op.try_cast::<WasmTypeCastOp>() {
                        base = cast.object;
                        continue;
                    }
                    break;
                }
                base
            }

            fn add_key_in_base_offset_maps(&mut self, key: SnapshotTableKey<OpIndex, KeyData>) {
                let base = unsafe { (*key.data.unwrap()).mem.base };
                match self.base_keys.map.get_mut(&base) {
                    Some(base_keys) => {
                        base_keys.with_offsets.push_front(key);
                    }
                    None => {
                        let mut data = BaseData::new();
                        data.with_offsets.push_front(key);
                        self.base_keys.map.insert(base, data);
                    }
                }

                let offset = unsafe { (*key.data.unwrap()).mem.offset };
                match self.offset_keys.map.get_mut(&offset) {
                    Some(offset_keys) => {
                        offset_keys.push_front(key);
                    }
                    None => {
                        let mut list: DoublyThreadedList<SnapshotTableKey<OpIndex, KeyData>, OffsetListTraits> = DoublyThreadedList::new();
                        list.push_front(key);
                        self.offset_keys.map.insert(offset, list);
                    }
                }
            }

            fn remove_key_from_base_offset_maps(&mut self, key: SnapshotTableKey<OpIndex, KeyData>) {
                DoublyThreadedList::<SnapshotTableKey<OpIndex, KeyData>, BaseListTraits>::remove(key);
                DoublyThreadedList::<SnapshotTableKey<OpIndex, KeyData>, OffsetListTraits>::remove(key);
            }
        }

        pub struct ChangeTrackingSnapshotTable {
            zone: *mut Zone,
        }

        impl ChangeTrackingSnapshotTable {
            pub fn new(zone: *mut Zone) -> Self {
                ChangeTrackingSnapshotTable { zone }
            }

            pub fn set(&mut self, key: SnapshotTableKey<OpIndex, KeyData>, value: OpIndex) {
                // Implementation for set
            }

            pub fn set_no_notify(&mut self, key: SnapshotTableKey<OpIndex, KeyData>, value: OpIndex) {
                // Implementation for set_no_notify
            }

            pub fn get(&mut self, key: SnapshotTableKey<OpIndex, KeyData>) -> OpIndex {
                //Implementation for get
                OpIndex { index: None }
            }
        }

        pub struct FixedOpIndexSidetable<T> {
            table: Vec<T>,
        }

        impl<T: Copy + Clone> FixedOpIndexSidetable<T> {
            pub fn new(size: usize, default_value: T) -> Self {
                FixedOpIndexSidetable { table: vec![default_value; size] }
            }

            pub fn get(&self, index: OpIndex) -> T {
                self.table[index.index.unwrap() as usize]
            }

            pub fn set(&mut self, index: OpIndex, value: T) {
                self.table[index.index.unwrap() as usize] = value;
            }
        }

        pub struct SparseOpIndexSnapshotTable<T> {
            map: HashMap<OpIndex, T>,
            zone: *mut Zone,
        }

        impl<T: Copy + Clone> SparseOpIndexSnapshotTable<T> {
            pub fn new(zone: *mut Zone) -> Self {
                SparseOpIndexSnapshotTable { map: HashMap::new(), zone }
            }

            pub fn get(&self, index: OpIndex) -> bool {
                self.map.get(&index).copied().unwrap_or(false)
            }

            pub fn set(&mut self, index: OpIndex, value: T) {
                self.map.insert(index, value);
            }

            pub fn has_key_for(&self, index: OpIndex) -> bool {
                self.map.contains_key(&index)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum HeapType {
            kExtern,
        }

        pub struct WasmModule {}

        impl WasmModule {
            pub fn heap_type(&self, type_index: ModuleTypeIndex) -> HeapType {
                HeapType::kExtern
            }
        }

        pub fn HeapTypesUnrelated(
            _type1: HeapType,
            _type2: HeapType,
            _module1: *const WasmModule,
            _module2: *const WasmModule,
        ) -> bool {
            true
        }

        pub struct StructType {}

        impl StructType {
            pub fn field(&self, _field_index: i32) -> Field {
                Field {}
            }

            pub fn mutability(&self, _field_index: i32) -> bool {
                false
            }

            pub fn field_offset(&self, _field_index: i32) -> i32 {
                0
            }
        }

        pub struct Field {}

        impl Field {
            pub fn value_kind_size(&self) -> u8 {
                0
            }
        }

        pub struct WasmStruct {}

        impl WasmStruct {
            pub const K_HEADER_SIZE: i32 = 0;
        }

        pub struct PipelineData {
            wasm_module: *const WasmModule,
        }

        impl PipelineData {
            pub fn new(wasm_module: *const WasmModule) -> Self {
                PipelineData { wasm_module }
            }

            pub fn wasm_module(&self) -> *const WasmModule {
                self.wasm_module
            }
        }

        #[derive(Clone, Copy)]
        pub struct StructGetOp {
            pub object: OpIndex,
            pub type_: *const StructType,
            pub field_index: i32,
            pub type_index: ModuleTypeIndex,
        }

        #[derive(Clone, Copy)]
        pub struct StructSetOp {
            pub object: OpIndex,
            pub type_: *const StructType,
            pub field_index: i32,
            pub type_index: ModuleTypeIndex,
            pub value: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct ArrayLengthOp {
            pub array: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct WasmAllocateArrayOp {
            pub length: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct StringAsWtf16Op {
            pub string: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct StringPrepareForGetCodeUnitOp {
            pub string: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct AnyConvertExternOp {
            pub object: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct AssertNotNullOp {
            pub object: OpIndex,
        }

        #[derive(Clone, Copy)]
        pub struct WasmTypeCastOp {
            pub object: OpIndex,
        }

        pub struct ZoneUnorderedMap<K, V> {
            pub map: HashMap<K, V>,
        }

        impl<K: Eq + PartialEq + Hash + Copy, V: Copy + Clone> ZoneUnorderedMap<K, V> {
            pub fn new() -> Self {
                ZoneUnorderedMap { map: HashMap::new() }
            }

            pub fn insert(&mut self, key: K, value: V) {
                self.map.insert(key, value);
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.map.get(key)
            }

            pub fn remove(&mut self, key: &K) {
                self.map.remove(key);
            }
        }
    }

    pub struct WasmLoadEliminationAnalyzer {
        graph: *mut Graph,
        phase_zone: *mut Zone,
        replacements: wle::FixedOpIndexSidetable<OpIndex>,
        non_aliasing_objects: wle::SparseOpIndexSnapshotTable<bool>,
        memory: wle::WasmMemoryContentTable,
        block_to_snapshot_mapping: FixedBlockSidetable<Option<Snapshot>>,
        predecessor_alias_snapshots: ZoneVector<AliasSnapshot>,
        predecessor_memory_snapshots: ZoneVector<MemorySnapshot>,
        data: *mut wle::PipelineData,
        wasmmodeule: *const wle::WasmModule,
    }

    impl WasmLoadEliminationAnalyzer {
        pub fn new(data: *mut wle::PipelineData, graph: *mut Graph, phase_zone: *mut Zone) -> Self {
            let replacements = unsafe {
                wle::FixedOpIndexSidetable::new(
                    (*graph).op_id_count(),
                    OpIndex { index: None },
                )
            };
            let non_aliasing_objects = unsafe { wle::SparseOpIndexSnapshotTable::new(phase_zone) };
            let memory = unsafe {
                wle::WasmMemoryContentTable::new(
                    data,
                    phase_zone,
                    non_aliasing_objects,
                    replacements,
                    graph,
                )
            };
            let block_to_snapshot_mapping = unsafe {
                FixedBlockSidetable::new((*graph).block_count(), None)
            };
            let predecessor_alias_snapshots = ZoneVector::new();
            let predecessor_memory_snapshots = ZoneVector::new();
            let wasmmodeule = unsafe { (*data).wasm_module() };

            WasmLoadEliminationAnalyzer {
                graph,
                phase_zone,
                replacements,
                non_aliasing_objects,
                memory,
                block_to_snapshot_mapping,
                predecessor_alias_snapshots,
                predecessor_memory_snapshots,
                data,
                wasmmodeule,
            }
        }

        pub fn run(&mut self) {
            let loop_finder = unsafe { LoopFinder::new(self.phase_zone, &mut *self.graph) };
            let iterator = unsafe {
                AnalyzerIterator::new(
                    self.phase_zone,
                    &mut *self.graph,
                    loop_finder,
                )
            };

            let mut compute_start_snapshot = true;
            while iterator.has_next() {
                let block = iterator.next();

                self.process_block(unsafe { &*block }, compute_start_snapshot);
                compute_start_snapshot = true;

                if let Some(last) = unsafe { (&*block).last_operation(&mut *self.graph).try_cast::<GotoOp>() } {
                    if last.destination.is_loop() &&
                        unsafe { (&*last.destination).last_predecessor() == &*block } {
                        let loop_header = last.destination;
                        if self.begin_block::<true>(unsafe { &*loop_header }) {
                            let loop_1st_pred = unsafe { (&*loop_header).last_predecessor().neighboring_predecessor() };
                            self.finish_block(unsafe { &*loop_1st_pred });
                            let pred_snapshots = unsafe {
                                self.block_to_snapshot_mapping.get((*loop_1st_pred).index()).clone()
                            };
                            unsafe {
                                self.non_aliasing_objects.start_new_snapshot(
                                    pred_snapshots.as_ref().unwrap().alias_snapshot
                                );
                                self.memory.change_tracking_snapshot_table.start_new_snapshot(
                                    pred_snapshots.as_ref().unwrap().memory_snapshot
                                );
                            }

                            iterator.mark_loop_for_revisit();
                            compute_start_snapshot = false;
                        } else {
                            self.seal_and_discard();
                        }
                    }
                }
            }
        }

        pub fn replacement(&self, index: OpIndex) -> OpIndex {
            self.replacements.get(index)
        }

        fn process_block(&mut self, block: &Block, compute_start_snapshot: bool) {
            if compute_start_snapshot {
                self.begin_block::<false>(block);
            }
            if block.is_loop() && self.backedge_has_snapshot(block) {
                self.store_loop_snapshot_in_forward_predecessor(block);
            }

            for op_idx in unsafe { (&mut *self.graph).operation_indices(*block) } {
                if false {
                    continue;
                }
                let mut op = unsafe { (&mut *self.graph).get(op_idx) };
                if false {
                    continue;
                }
                match op.opcode {
                    Opcode::kStructGet => {
                        self.process_struct_get(op_idx, *op.cast::<wle::StructGetOp>());
                    }
                    Opcode::kStructSet => {
                        self.process_struct_set(op_idx, *op.cast::<wle::StructSetOp>());
                    }
                    Opcode::kArrayLength => {
                        self.process_array_length(op_idx, *op.cast::<wle::ArrayLengthOp>());
                    }
                    Opcode::kWasmAllocateArray => {
                        self.process_wasm_allocate_array(op_idx, *op.cast::<wle::WasmAllocateArrayOp>());
                    }
                    Opcode::kStringAsWtf16 => {
                        self.process_string_as_wtf16(op_idx, *op.cast::<wle::StringAsWtf16Op>());
                    }
                    Opcode::kStringPrepareForGetCodeUnit => {
                        self.process_string_prepare_for_get_code_unit(
                            op_idx,
                            *op.cast::<wle::StringPrepareForGetCodeUnitOp>(),
                        );
                    }
                    Opcode::kAnyConvertExtern => {
                        self.process_any_convert_extern(op_idx, *op.cast::<wle::AnyConvertExternOp>());
                    }
                    Opcode::kAssertNotNull => {
                        self.process_assert_not_null(op_idx, *op.cast::<wle::AssertNotNullOp>());
                    }
                    Opcode::kArraySet => {}
                    Opcode::kAllocate => {
                        self.process_allocate(op_idx, *op.cast::<AllocateOp>());
                    }
                    Opcode::kCall => {
                        self.process_call(op_idx, *op.cast::<CallOp>());
                    }
                    Opcode::kPhi => {
                        self.process_phi(op_idx, *op.cast::<PhiOp>());
                    }
                    Opcode::kLoad => {}
                    Opcode::kStore => {}
                    Opcode::kAssumeMap => {}
                    Opcode::kCatchBlockBegin => {}
                    Opcode::kRetain => {}
                    Opcode::kDidntThrow => {}
                    Opcode::kCheckException => {}
                    Opcode::kAtomicRMW => {}
                    Opcode::kAtomicWord32Pair => {}
                    Opcode::kMemoryBarrier => {}
                    Opcode::kJSStackCheck => {}
                    Opcode::kWasmStackCheck => {}
                    Opcode::kSimd128LaneMemory => {}
                    Opcode::kGlobalSet => {}
                    Opcode::kParameter => {}
                    Opcode::kWordBinop => {
                        self.dcheck_word_binop(op_idx, *op.cast::<WordBinopOp>());
                    }
                    Opcode::kFrameState => {}
                    Opcode::kDeoptimizeIf => {}
                    Opcode::kComparison => {}
                    Opcode::kTrapIf => {}
                    Opcode::kDeoptimize => {}
                    Opcode::kReturn => {}
                    _ => {
                        if !op.effects.can_write {
                            self.invalidate_all_non_aliasing_inputs(&op);
                        }
                    }
                }
            }

            self.finish_block(block);
        }

        fn process_struct_get(&mut self, op_idx: OpIndex, get: wle::StructGetOp) {
            let existing = unsafe { (&mut self.memory).find(&get) };
            if existing.index.is_some() {
                let replacement = unsafe { (&mut *self.graph).get(existing) };
                if rep_is_compatible(

