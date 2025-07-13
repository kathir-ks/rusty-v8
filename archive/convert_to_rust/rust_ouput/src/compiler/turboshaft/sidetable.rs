// Converted from V8 C++ source files:
// Header: sidetable.h
// Implementation: sidetable.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::ops::{Index, IndexMut};
use std::mem;

use absl::container::flat_hash_map as ZoneAbslFlatHashMap;

mod absl {
    pub mod container {
        pub mod flat_hash_map {
            use std::collections::HashMap;

            pub struct FlatHashMap<K, V> {
                map: HashMap<K, V>,
            }

            impl<K, V> FlatHashMap<K, V>
            where
                K: std::hash::Hash + Eq + Copy,
            {
                pub fn new() -> Self {
                    FlatHashMap { map: HashMap::new() }
                }

                pub fn insert(&mut self, key: K, value: V) -> Option<V> {
                    self.map.insert(key, value)
                }

                pub fn get(&self, key: &K) -> Option<&V> {
                    self.map.get(key)
                }

                pub fn contains_key(&self, key: &K) -> bool {
                    self.map.contains_key(key)
                }

                pub fn remove(&mut self, key: &K) -> Option<V> {
                    self.map.remove(key)
                }

                pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
                    self.map.iter()
                }
                
                pub fn find(&self, key: K) -> Option<(&K, &V)> {
                    self.map.get_key_value(&key)
                }

                pub fn erase(&mut self, it: Option<(&K, &V)>) {
                    if let Some((k, _)) = it {
                        self.map.remove(k);
                    }
                }

                pub fn begin(&self) -> std::collections::hash_map::Iter<K, V> {
                    self.map.iter()
                }

                pub fn end(&self) -> std::collections::hash_map::Iter<K, V> {
                    self.map.iter()
                }
                
                pub fn at(&self, index: K) -> &V where K: std::cmp::Eq + std::hash::Hash {
                    self.map.get(&index).unwrap()
                }
            }
        }
    }
}

pub struct V8 {}
pub struct OpIndex {}
pub struct BlockIndex {}
pub struct Zone {}
pub struct Graph {}

impl OpIndex {
    pub fn valid(&self) -> bool {
        true // Replace with actual logic if needed
    }

    pub fn id(&self) -> usize {
        0 // Replace with actual logic if needed
    }
}

impl BlockIndex {
    pub fn valid(&self) -> bool {
        true // Replace with actual logic if needed
    }

    pub fn id(&self) -> usize {
        0 // Replace with actual logic if needed
    }
}

impl Graph {
    pub fn BelongsToThisGraph(&self, _index: OpIndex) -> bool {
        true // Replace with actual logic if needed
    }
}

pub struct ZoneVector<T> {
    data: Vec<T>,
}

impl<T> ZoneVector<T> {
    pub fn new(zone: *mut Zone) -> Self {
        // 'zone' is not used in Rust, memory is managed by Vec
        ZoneVector { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize, zone: *mut Zone) -> Self {
        // 'zone' is not used in Rust, memory is managed by Vec
        ZoneVector {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn resize(&mut self, new_size: usize)
    where
        T: Default + Clone,
    {
        self.data.resize(new_size, T::default());
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn begin(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn end(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    
    pub fn new_with_val(size: usize, initial_value: T, zone: *mut Zone) -> Self where T: Clone {
        ZoneVector { data: vec![initial_value; size] }
    }
}

impl<T> Index<usize> for ZoneVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for ZoneVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

mod detail {
    use super::*;

    pub struct GrowingSidetable<T, Key> {
        table_: ZoneVector<T>,
        _phantom: std::marker::PhantomData<Key>,
    }

    impl<T, Key> GrowingSidetable<T, Key>
    where
        T: Default + Clone,
        Key: Copy,
        Self: GrowingSidetableImpl<T, Key>,
    {
        pub fn new(zone: *mut Zone) -> Self {
            GrowingSidetableImpl::new(zone)
        }

        pub fn new_with_size(size: usize, initial_value: T, zone: *mut Zone) -> Self {
            GrowingSidetableImpl::new_with_size(size, initial_value, zone)
        }

        pub fn get(&self, index: Key) -> &T {
            GrowingSidetableImpl::get_impl(self, index)
        }

        pub fn get_mut(&mut self, index: Key) -> &mut T {
            GrowingSidetableImpl::get_mut_impl(self, index)
        }

        pub fn reset(&mut self) {
            std::fill(self.table_.begin(), self.table_.end(), T::default());
        }

        pub fn empty(&self) -> bool {
            self.table_.data.is_empty()
        }

        fn next_size(&self, out_of_bounds_index: usize) -> usize {
            assert!(out_of_bounds_index >= self.table_.len());
            out_of_bounds_index + out_of_bounds_index / 2 + 32
        }
    }

    trait GrowingSidetableImpl<T, Key> {
        fn new(zone: *mut Zone) -> Self;
        fn new_with_size(size: usize, initial_value: T, zone: *mut Zone) -> Self;
        fn get_impl(&self, index: Key) -> &T;
        fn get_mut_impl(&mut self, index: Key) -> &mut T;
    }

    impl<T> GrowingSidetableImpl<T, OpIndex> for GrowingSidetable<T, OpIndex>
    where
        T: Default + Clone,
    {
        fn new(zone: *mut Zone) -> Self {
            GrowingSidetable {
                table_: ZoneVector::new(zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn new_with_size(size: usize, initial_value: T, zone: *mut Zone) -> Self {
            GrowingSidetable {
                table_: ZoneVector::new_with_val(size, initial_value, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn get_impl(&self, index: OpIndex) -> &T {
            let i = (index.id()) as usize;
            if i >= self.table_.data.len() {
                panic!("Index out of bounds"); // Or handle resizing if needed
            }
            &self.table_.data[i]
        }

        fn get_mut_impl(&mut self, index: OpIndex) -> &mut T {
            let i = (index.id()) as usize;
            if i >= self.table_.data.len() {
                let next_size = self.next_size(i);
                self.table_.resize(next_size);
                self.table_.resize(self.table_.capacity());
            }
            &mut self.table_.data[i]
        }
    }

    impl<T> GrowingSidetableImpl<T, BlockIndex> for GrowingSidetable<T, BlockIndex>
    where
        T: Default + Clone,
    {
        fn new(zone: *mut Zone) -> Self {
            GrowingSidetable {
                table_: ZoneVector::new(zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn new_with_size(size: usize, initial_value: T, zone: *mut Zone) -> Self {
            GrowingSidetable {
                table_: ZoneVector::new_with_val(size, initial_value, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn get_impl(&self, index: BlockIndex) -> &T {
            let i = (index.id()) as usize;
            if i >= self.table_.data.len() {
                panic!("Index out of bounds"); // Or handle resizing if needed
            }
            &self.table_.data[i]
        }

        fn get_mut_impl(&mut self, index: BlockIndex) -> &mut T {
            let i = (index.id()) as usize;
            if i >= self.table_.data.len() {
                let next_size = self.next_size(i);
                self.table_.resize(next_size);
                self.table_.resize(self.table_.capacity());
            }
            &mut self.table_.data[i]
        }
    }

    pub struct FixedSidetable<T, Key> {
        table_: ZoneVector<T>,
        _phantom: std::marker::PhantomData<Key>,
    }

    impl<T, Key> FixedSidetable<T, Key>
    where
        T: Default + Clone,
        Key: Copy,
        Self: FixedSidetableImpl<T, Key>,
    {
        pub fn new(size: usize, zone: *mut Zone) -> Self {
            FixedSidetableImpl::new(size, zone)
        }

        pub fn new_with_val(size: usize, default_value: T, zone: *mut Zone) -> Self {
            FixedSidetableImpl::new_with_val(size, default_value, zone)
        }

        pub fn get(&self, op: Key) -> &T {
            FixedSidetableImpl::get_impl(self, op)
        }

        pub fn get_mut(&mut self, op: Key) -> &mut T {
            FixedSidetableImpl::get_mut_impl(self, op)
        }
    }

    trait FixedSidetableImpl<T, Key> {
        fn new(size: usize, zone: *mut Zone) -> Self;
        fn new_with_val(size: usize, default_value: T, zone: *mut Zone) -> Self;
        fn get_impl(&self, op: Key) -> &T;
        fn get_mut_impl(&mut self, op: Key) -> &mut T;
    }

    impl<T> FixedSidetableImpl<T, OpIndex> for FixedSidetable<T, OpIndex>
    where
        T: Default + Clone,
    {
        fn new(size: usize, zone: *mut Zone) -> Self {
            FixedSidetable {
                table_: ZoneVector::new_with_capacity(size, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn new_with_val(size: usize, default_value: T, zone: *mut Zone) -> Self {
            FixedSidetable {
                table_: ZoneVector::new_with_val(size, default_value, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn get_impl(&self, op: OpIndex) -> &T {
            assert!((op.id() as usize) < self.table_.data.len());
            &self.table_.data[op.id() as usize]
        }

        fn get_mut_impl(&mut self, op: OpIndex) -> &mut T {
            assert!((op.id() as usize) < self.table_.data.len());
            &mut self.table_.data[op.id() as usize]
        }
    }

    impl<T> FixedSidetableImpl<T, BlockIndex> for FixedSidetable<T, BlockIndex>
    where
        T: Default + Clone,
    {
        fn new(size: usize, zone: *mut Zone) -> Self {
            FixedSidetable {
                table_: ZoneVector::new_with_capacity(size, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn new_with_val(size: usize, default_value: T, zone: *mut Zone) -> Self {
            FixedSidetable {
                table_: ZoneVector::new_with_val(size, default_value, zone),
                _phantom: std::marker::PhantomData,
            }
        }

        fn get_impl(&self, op: BlockIndex) -> &T {
            assert!((op.id() as usize) < self.table_.data.len());
            &self.table_.data[op.id() as usize]
        }

        fn get_mut_impl(&mut self, op: BlockIndex) -> &mut T {
            assert!((op.id() as usize) < self.table_.data.len());
            &mut self.table_.data[op.id() as usize]
        }
    }
}

pub struct GrowingBlockSidetable<T> {
    base: detail::GrowingSidetable<T, BlockIndex>,
}

impl<T> GrowingBlockSidetable<T>
where
    T: Default + Clone,
{
    pub fn new(zone: *mut Zone) -> Self {
        GrowingBlockSidetable {
            base: detail::GrowingSidetable::new(zone),
        }
    }

    pub fn new_with_size(size: usize, initial_value: T, zone: *mut Zone) -> Self {
        GrowingBlockSidetable {
            base: detail::GrowingSidetable::new_with_size(size, initial_value, zone),
        }
    }

    pub fn get(&self, index: BlockIndex) -> &T {
        self.base.get(index)
    }

    pub fn get_mut(&mut self, index: BlockIndex) -> &mut T {
        self.base.get_mut(index)
    }

    pub fn reset(&mut self) {
        self.base.reset();
    }

    pub fn empty(&self) -> bool {
        self.base.empty()
    }
}

pub struct FixedBlockSidetable<T> {
    base: detail::FixedSidetable<T, BlockIndex>,
}

impl<T> FixedBlockSidetable<T>
where
    T: Default + Clone,
{
    pub fn new(size: usize, zone: *mut Zone) -> Self {
        FixedBlockSidetable {
            base: detail::FixedSidetable::new(size, zone),
        }
    }

    pub fn new_with_val(size: usize, initial_value: T, zone: *mut Zone) -> Self {
        FixedBlockSidetable {
            base: detail::FixedSidetable::new_with_val(size, initial_value, zone),
        }
    }

    pub fn get(&self, op: BlockIndex) -> &T {
        self.base.get(op)
    }

    pub fn get_mut(&mut self, op: BlockIndex) -> &mut T {
        self.base.get_mut(op)
    }
}

pub struct GrowingOpIndexSidetable<T> {
    base: detail::GrowingSidetable<T, OpIndex>,
    #[cfg(debug_assertions)]
    graph_: *const Graph,
}

impl<T> GrowingOpIndexSidetable<T>
where
    T: Default + Clone,
{
    pub fn new(zone: *mut Zone, graph: *const Graph) -> Self {
        GrowingOpIndexSidetable {
            base: detail::GrowingSidetable::new(zone),
            #[cfg(debug_assertions)]
            graph_: graph,
        }
    }

    pub fn new_with_size(size: usize, initial_value: T, zone: *mut Zone, graph: *const Graph) -> Self {
        GrowingOpIndexSidetable {
            base: detail::GrowingSidetable::new_with_size(size, initial_value, zone),
            #[cfg(debug_assertions)]
            graph_: graph,
        }
    }

    pub fn get(&self, index: OpIndex) -> &T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.base.get(index)
    }

    pub fn get_mut(&mut self, index: OpIndex) -> &mut T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.base.get_mut(index)
    }

    pub fn swap_data(&mut self, other: &mut GrowingOpIndexSidetable<T>) {
        mem::swap(&mut self.base.table_, &mut other.base.table_);
    }
}

pub struct FixedOpIndexSidetable<T> {
    base: detail::FixedSidetable<T, OpIndex>,
    #[cfg(debug_assertions)]
    graph_: *const Graph,
}

impl<T> FixedOpIndexSidetable<T>
where
    T: Default + Clone,
{
    pub fn new(size: usize, zone: *mut Zone, graph: *const Graph) -> Self {
        FixedOpIndexSidetable {
            base: detail::FixedSidetable::new(size, zone),
            #[cfg(debug_assertions)]
            graph_: graph,
        }
    }

    pub fn new_with_val(size: usize, default_value: T, zone: *mut Zone, graph: *const Graph) -> Self {
        FixedOpIndexSidetable {
            base: detail::FixedSidetable::new_with_val(size, default_value, zone),
            #[cfg(debug_assertions)]
            graph_: graph,
        }
    }

    pub fn get(&self, index: OpIndex) -> &T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.base.get(index)
    }

    pub fn get_mut(&mut self, index: OpIndex) -> &mut T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.base.get_mut(index)
    }

    pub fn swap_data(&mut self, other: &mut FixedOpIndexSidetable<T>) {
        mem::swap(&mut self.base.table_, &mut other.base.table_);
    }
}

pub struct SparseOpIndexSideTable<T> {
    data_: ZoneAbslFlatHashMap::FlatHashMap<OpIndex, T>,
    #[cfg(debug_assertions)]
    graph_: *const Graph,
}

impl<T> SparseOpIndexSideTable<T>
where
    T: Default + Clone + Copy,
    OpIndex: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn new(zone: *mut Zone, graph: *const Graph) -> Self {
        SparseOpIndexSideTable {
            data_: ZoneAbslFlatHashMap::FlatHashMap::new(),
            #[cfg(debug_assertions)]
            graph_: graph,
        }
    }

    pub fn get(&self, index: OpIndex) -> &T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.data_.get(&index).unwrap()
    }

    pub fn get_mut(&mut self, index: OpIndex) -> &mut T {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        if !self.data_.contains_key(&index) {
            self.data_.insert(index, T::default());
        }
        self.data_.map.get_mut(&index).unwrap()
    }

    pub fn contains(&self, index: OpIndex) -> bool {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.data_.contains_key(&index)
    }

    pub fn contains_with_value(&self, index: OpIndex) -> Option<&T> {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.data_.get(&index)
    }

    pub fn remove(&mut self, index: OpIndex) {
        #[cfg(debug_assertions)]
        assert!(OpIndexBelongsToTableGraph(self.graph_ as *const Graph, index));
        self.data_.remove(&index);
    }

    pub fn begin(&self) -> std::collections::hash_map::Iter<'_, OpIndex, T> {
        self.data_.begin()
    }

    pub fn end(&self) -> std::collections::hash_map::Iter<'_, OpIndex, T> {
        self.data_.end()
    }
}

#[cfg(debug_assertions)]
fn OpIndexBelongsToTableGraph(graph: *const Graph, index: OpIndex) -> bool {
    unsafe {
        (*graph).BelongsToThisGraph(index)
    }
}
