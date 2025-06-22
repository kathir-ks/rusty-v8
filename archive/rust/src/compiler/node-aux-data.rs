// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Represents a Node in the compilation graph.  The actual
/// definition should be defined elsewhere.  This is a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        NodeId(id)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Represents a Node in the compilation graph.  The actual
/// definition should be defined elsewhere.  This is a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    id: NodeId,
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Node { id }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }
}

/// A simple Zone allocator.  In a real implementation, this would
/// be more sophisticated.
#[derive(Debug)]
pub struct Zone {
    // For simplicity, just use a Vec<Box<dyn Any>>.  In reality,
    // this would likely use pre-allocated chunks.
    data: Vec<Box<dyn std::any::Any>>,
}

impl Zone {
    pub fn new() -> Self {
        Zone { data: Vec::new() }
    }

    pub fn alloc<T: 'static>(&mut self, value: T) -> &mut T {
        let boxed = Box::new(value);
        let ptr = Box::leak(boxed);
        self.data.push(Box::from_raw(ptr as *mut T as *mut dyn std::any::Any));
        ptr
    }
}

/// Wrapper around a `Vec<T>` that can be grown.
#[derive(Debug, Clone)]
pub struct ZoneVector<T> {
    data: Vec<T>,
    default_value: T,
    //zone: *mut Zone, // Not used in Rust version
}

impl<T: Clone> ZoneVector<T> {
    pub fn new() -> Self {
        ZoneVector {
            data: Vec::new(),
            default_value: T::clone(&T::default()),
           // zone: std::ptr::null_mut(),
        }
    }

    pub fn with_default(default_value: T) -> Self {
        ZoneVector {
            data: Vec::new(),
            default_value,
           // zone: std::ptr::null_mut(),
        }
    }

    pub fn with_capacity(capacity: usize, default_value: T) -> Self {
        ZoneVector {
            data: vec![default_value.clone(); capacity],
            default_value,
           // zone: std::ptr::null_mut(),
        }
    }

    pub fn resize(&mut self, new_len: usize, default_value: T) {
        self.data.resize(new_len, default_value);
    }
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

impl<T: PartialEq> PartialEq for ZoneVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

/// `DefaultConstruct` is not needed in Rust.  The `Default` trait is used instead.

/// `ZoneConstruct` is not needed in Rust.  The `Default` trait is used instead.

/// A data structure for associating auxiliary data with nodes.
#[derive(Debug)]
pub struct NodeAuxData<T, F = fn() -> T>
where
    T: Clone + PartialEq,
    F: Fn() -> T,
{
    zone_: Zone, // Zone is kept to follow C++ implementation, though not needed in rust
    aux_data_: ZoneVector<T>,
    default_fn: F,
}

impl<T, F> NodeAuxData<T, F>
where
    T: Clone + PartialEq + Default,
    F: Fn() -> T + Default,
{
    /// Creates a new `NodeAuxData`.
    pub fn new(zone: Zone) -> Self {
        NodeAuxData {
            zone_: zone,
            aux_data_: ZoneVector::with_default(T::default()),
            default_fn: F::default(),
        }
    }

    /// Creates a new `NodeAuxData` with a specified initial size.
    pub fn with_initial_size(initial_size: usize, zone: Zone) -> Self {
        NodeAuxData {
            zone_: zone,
            aux_data_: ZoneVector::with_capacity(initial_size, T::default()),
            default_fn: F::default(),
        }
    }

    /// Updates an entry.  Returns `true` if the entry was changed.
    pub fn set(&mut self, node: &Node, data: T) -> bool {
        self.set_by_id(node.id(), data)
    }

    /// Updates an entry.  Returns `true` if the entry was changed.
    pub fn set_by_id(&mut self, id: NodeId, data: T) -> bool {
        let id_value = id.value();
        if id_value >= self.aux_data_.len() {
            let default_value = (self.default_fn)();
            self.aux_data_.resize(id_value + 1, default_value);
        }
        if self.aux_data_[id_value] != data {
            self.aux_data_[id_value] = data;
            true
        } else {
            false
        }
    }

    /// Gets the data associated with a node.
    pub fn get(&self, node: &Node) -> T {
        self.get_by_id(node.id())
    }

    /// Gets the data associated with a node ID.
    pub fn get_by_id(&self, id: NodeId) -> T {
        let id_value = id.value();
        if id_value < self.aux_data_.len() {
            self.aux_data_[id_value].clone()
        } else {
            (self.default_fn)().clone()
        }
    }

    /// Returns an iterator over the data.
    pub fn iter(&self) -> ConstIterator<'_, T> {
        ConstIterator {
            data_: &self.aux_data_,
            current_: 0,
        }
    }
}

impl<T> NodeAuxData<T, fn() -> T>
where
    T: Clone + PartialEq + Default,
{
    pub fn with_default_fn<F>(self, default_fn: F) -> NodeAuxData<T, F>
    where
        F: Fn() -> T,
    {
        NodeAuxData {
            zone_: self.zone_,
            aux_data_: self.aux_data_,
            default_fn,
        }
    }
}

/// A const iterator for `NodeAuxData`.
#[derive(Debug)]
pub struct ConstIterator<'a, T> {
    data_: &'a ZoneVector<T>,
    current_: usize,
}

impl<'a, T: Clone> ConstIterator<'a, T> {
    /// Returns the current element.
    pub fn current(&self) -> Option<(usize, T)> {
        if self.current_ < self.data_.len() {
            Some((self.current_, self.data_[self.current_].clone()))
        } else {
            None
        }
    }
}

impl<'a, T: Clone> Iterator for ConstIterator<'a, T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ < self.data_.len() {
            let result = Some((self.current_, self.data_[self.current_].clone()));
            self.current_ += 1;
            result
        } else {
            None
        }
    }
}

/// A data structure for associating auxiliary data with nodes, using a map.
#[derive(Debug)]
pub struct NodeAuxDataMap<T> {
    map_: HashMap<NodeId, T>,
}

impl<T> NodeAuxDataMap<T>
where
    T: Clone + PartialEq,
{
    /// Creates a new `NodeAuxDataMap`.
    pub fn new(_zone: Zone) -> Self {
        NodeAuxDataMap {
            map_: HashMap::new(),
        }
    }

    /// Puts a value into the map.
    pub fn put(&mut self, key: NodeId, value: T) {
        self.map_.insert(key, value);
    }

    /// Gets a value from the map.
    pub fn get(&self, key: NodeId, k_non_existent: &T) -> T {
        self.map_.get(&key).cloned().unwrap_or(k_non_existent.clone())
    }

    /// Reserves space in the map.
    pub fn reserve(&mut self, count: usize) {
        let new_capacity = self.map_.len() + count;
        self.map_.reserve(new_capacity);
    }
}