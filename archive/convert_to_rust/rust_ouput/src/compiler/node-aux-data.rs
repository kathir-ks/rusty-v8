// Converted from V8 C++ source files:
// Header: node-aux-data.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod node_aux_data {
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::compiler::machine_operator::V8;
use crate::compiler::machine_operator::v8;
use crate::compiler::node::Node;
use crate::compiler::machine_graph::NodeId;
use crate::compiler::js_call_reducer::ZoneVector;

pub struct Zone {

}

impl Zone {
    pub fn new() -> Zone {
        Zone {}
    }
}

pub fn default_construct<T: Default>(_zone: &Zone) -> T {
    T::default()
}

pub fn zone_construct<T>(_zone: &Zone) -> T
where
    T: ZoneConstructable,
{
    T::zone_construct(_zone)
}

pub trait ZoneConstructable {
    fn zone_construct(_zone: &Zone) -> Self;
}

pub struct NodeAuxData<T, F = fn(&Zone) -> T>
where
    T: Clone + PartialEq,
{
    zone_: Zone,
    aux_data_: ZoneVector<T>,
    default_value_fn: PhantomData<F>,
}

impl<T: Clone + PartialEq + Default> NodeAuxData<T, fn(&Zone) -> T> {
    pub fn new(zone: &Zone) -> Self {
        NodeAuxData {
            zone_: Zone::new(),
            aux_data_: ZoneVector::new(),
            default_value_fn: PhantomData,
        }
    }

    pub fn new_with_size(initial_size: usize, zone: &Zone) -> Self {
        let mut aux_data_ = ZoneVector::new();
        for _ in 0..initial_size {
            aux_data_.push(T::default());
        }

        NodeAuxData {
            zone_: Zone::new(),
            aux_data_: aux_data_,
            default_value_fn: PhantomData,
        }
    }
}

impl<T: Clone + PartialEq, F> NodeAuxData<T, F>
where
    F: Fn(&Zone) -> T,
{
    pub fn set(&mut self, node: &Node, data: &T) -> bool {
        let id = node.id();
        self.set_by_id(id, data)
    }

    pub fn set_by_id(&mut self, id: NodeId, data: &T) -> bool {
        let id_usize = usize::try_from(id.0).unwrap();
        if id_usize >= self.aux_data_.len() {
            let default_value = (self.default_value_fn)(&self.zone_);
            self.aux_data_.resize(id_usize + 1, default_value);
        }
        if self.aux_data_[id_usize] != *data {
            self.aux_data_[id_usize] = data.clone();
            return true;
        }
        false
    }

    pub fn get(&self, node: &Node) -> T {
        self.get_by_id(node.id())
    }

    pub fn get_by_id(&self, id: NodeId) -> T {
        let id_usize = usize::try_from(id.0).unwrap();
        if id_usize < self.aux_data_.len() {
            self.aux_data_[id_usize].clone()
        } else {
            (self.default_value_fn)(&self.zone_)
        }
    }

    pub fn begin(&self) -> ConstIterator<'_, T, F> {
        ConstIterator {
            data_: &self.aux_data_,
            current_: 0,
            phantom: PhantomData,
        }
    }

    pub fn end(&self) -> ConstIterator<'_, T, F> {
        ConstIterator {
            data_: &self.aux_data_,
            current_: self.aux_data_.len(),
            phantom: PhantomData,
        }
    }
}

pub struct ConstIterator<'a, T, F>
where
    T: Clone + PartialEq,
{
    data_: &'a ZoneVector<T>,
    current_: usize,
    phantom: PhantomData<F>,
}

impl<'a, T, F> ConstIterator<'a, T, F>
where
    T: Clone + PartialEq,
{
    pub fn new(data: &'a ZoneVector<T>, current: usize) -> Self {
        ConstIterator {
            data_: data,
            current_: current,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> Iterator for ConstIterator<'a, T, F>
where
    T: Clone + PartialEq,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ < self.data_.len() {
            let current = self.current_;
            self.current_ += 1;
            Some((current, self.data_[current].clone()))
        } else {
            None
        }
    }
}

impl<'a, T, F> PartialEq for ConstIterator<'a, T, F>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.current_ == other.current_ && self.data_ == other.data_
    }
}

pub struct NodeAuxDataMap<T, const K_NON_EXISTENT: i32>
where
    T: Clone + PartialEq,
{
    map_: HashMap<NodeId, T>,
}

impl<T: Clone + PartialEq, const K_NON_EXISTENT: i32> NodeAuxDataMap<T, K_NON_EXISTENT> {
    pub fn new(_zone: &Zone) -> Self {
        NodeAuxDataMap {
            map_: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: NodeId, value: T) {
        self.map_.insert(key, value);
    }

    pub fn get(&self, key: NodeId) -> T
    where T: Default{
        self.map_.get(&key).map(|t| t.clone()).unwrap_or_default()
    }

    pub fn reserve(&mut self, count: usize) {
        let new_capacity = self.map_.len() + count;
        self.map_.reserve(new_capacity);
    }
}
}
