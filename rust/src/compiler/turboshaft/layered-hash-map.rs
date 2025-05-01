// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{BitAnd, Add};
use std::cmp::{max, Ordering};
use std::num::Wrapping;

//use crate::base::bits;  // Assuming 'base' crate is defined elsewhere
//use crate::compiler::turboshaft::fast_hash; // Assuming 'fast_hash' module is defined elsewhere

/// LayeredHashMap is a hash map whose elements are grouped into layers, such
/// that it's efficient to remove all of the items from the last inserted layer.
/// In addition to the regular Insert/Get/Contains functions of hash maps, it
/// thus provides two additional functions: StartLayer to indicate that future
/// insertions are part of a new layer, and DropLastLayer to remove all of the
/// items of the last layer.
///
/// LayeredHashMap does not support inserting multiple values with the same key,
/// and does not support updating already-inserted items in the map. If you need
/// to update an existing key, you'll need to remove it (by calling DropLastLayer
/// as many times as needed), and then re-insert it.
///
/// The implementation uses a regular Vec for the main hash table, while
/// keeping a linked list of items per layer. When inserting an item in the
/// LayeredHashMap, we insert it into the Vec and link it to the linked
/// list of the current (=latest) layer. In order to remove all of the items from
/// the last layer, we iterate its linked list, and remove the items one by one
/// from the Vec, after which we drop the linked list altogether.
#[derive(Debug)]
pub struct LayeredHashMap<K, V> {
    mask_: usize,
    entry_count_: usize,
    table_: Vec<Entry<K, V>>,
    depths_heads_: Vec<Option<Box<Entry<K, V>>>>,
}

#[derive(Debug)]
struct Entry<K, V> {
    hash: usize,
    key: K,
    value: V,
    depth_neighboring_entry: Option<Box<Entry<K, V>>>,
}

const K_NEED_RESIZE_PERCENTAGE: f64 = 0.75;
const K_GROWTH_FACTOR: usize = 2;

impl<K: Eq + Hash + Clone, V: Clone> LayeredHashMap<K, V> {
    /// Creates a new LayeredHashMap with the specified initial capacity.
    pub fn new(initial_capacity: u32) -> Self {
        // Setting the minimal capacity to 16
        let mut initial_capacity = max(initial_capacity, 16);
        // {initial_capacity} should be a power of 2, so that we can compute offset
        // in {table_} with a mask rather than a modulo.
        initial_capacity = round_up_to_power_of_two(initial_capacity);
        let mask_ = initial_capacity as usize - 1;
        // Allocating the table_
        let table_: Vec<Entry<K, V>> = (0..initial_capacity)
            .map(|_| Entry {
                hash: 0,
                key: unsafe { MaybeUninit::zeroed().assume_init() }, // Placeholder, overwritten on insertion
                value: unsafe { MaybeUninit::zeroed().assume_init() }, // Placeholder, overwritten on insertion
                depth_neighboring_entry: None,
            })
            .collect();

        LayeredHashMap {
            mask_,
            entry_count_: 0,
            table_,
            depths_heads_: Vec::new(),
        }
    }

    /// Starts a new layer.
    pub fn start_layer(&mut self) {
        self.depths_heads_.push(None);
    }

    /// Drops the last layer, removing all its items from the map.
    pub fn drop_last_layer(&mut self) {
        assert!(self.depths_heads_.len() > 0);

        if let Some(mut entry) = self.depths_heads_.pop().unwrap() {
            while let Some(mut next) = entry.depth_neighboring_entry.take() {
                self.entry_count_ -= 1;
                let index = self.find_entry_for_key(&entry.key, entry.hash) as *const Entry<K, V> as usize - self.table_.as_ptr() as usize;
                //let index = self.find_entry_for_key(&entry.key, entry.hash) as usize - &self.table_[0] as *const Entry<K, V> as usize;
                //if let Some(e) = self.table_.get_mut(index) { *e = Entry { hash: 0, key: unsafe { MaybeUninit::zeroed().assume_init() }, value: unsafe { MaybeUninit::zeroed().assume_init() }, depth_neighboring_entry: None }; }
                let mut e = self.table_.get_mut(index / std::mem::size_of::<Entry<K,V>>()).unwrap();
                *e = Entry {
                    hash: 0,
                    key: unsafe { MaybeUninit::zeroed().assume_init() },
                    value: unsafe { MaybeUninit::zeroed().assume_init() },
                    depth_neighboring_entry: None,
                };

                entry = next;
            }
            self.entry_count_ -= 1;
            let index = self.find_entry_for_key(&entry.key, entry.hash) as *const Entry<K, V> as usize - self.table_.as_ptr() as usize;
            //let index = self.find_entry_for_key(&entry.key, entry.hash) as usize - &self.table_[0] as *const Entry<K, V> as usize;
            //if let Some(e) = self.table_.get_mut(index) { *e = Entry { hash: 0, key: unsafe { MaybeUninit::zeroed().assume_init() }, value: unsafe { MaybeUninit::zeroed().assume_init() }, depth_neighboring_entry: None }; }
            let mut e = self.table_.get_mut(index / std::mem::size_of::<Entry<K,V>>()).unwrap();
            *e = Entry {
                hash: 0,
                key: unsafe { MaybeUninit::zeroed().assume_init() },
                value: unsafe { MaybeUninit::zeroed().assume_init() },
                depth_neighboring_entry: None,
            };
        }
    }

    /// Inserts a new key-value pair into the map.  Panics if the key already exists.
    pub fn insert_new_key(&mut self, key: K, value: V) {
        self.resize_if_needed();
        let hash = self.compute_hash(&key);
        let destination = self.find_entry_for_key(key.clone(), hash);
        assert_eq!(destination.hash, 0);
        let destination_index = destination as *const Entry<K, V> as usize - self.table_.as_ptr() as usize;
        //let destination_index = destination as usize - &self.table_[0] as *const Entry<K, V> as usize;
        let new_entry = Entry {
            hash,
            key: key.clone(),
            value: value.clone(),
            depth_neighboring_entry: self.depths_heads_.last().and_then(|x| x.clone()),
        };
        let mut entry = self.table_.get_mut(destination_index / std::mem::size_of::<Entry<K,V>>()).unwrap();
        *entry = new_entry;

        if let Some(head) = self.depths_heads_.last_mut() {
            *head = Some(Box::new(Entry {
                hash,
                key,
                value,
                depth_neighboring_entry: None,
            }));
        }

        self.entry_count_ += 1;
    }

    /// Checks if the map contains the given key.
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Gets the value associated with the given key, if it exists.
    pub fn get(&self, key: &K) -> Option<V> {
        let destination = self.find_entry_for_key(key.clone(), self.compute_hash(key));
        if destination.hash == 0 {
            return None;
        }
        Some(destination.value.clone())
    }

    fn next_entry_index(&self, index: usize) -> usize {
        (index + 1) & self.mask_
    }

    // Returns a mutable reference to the Entry in the table for the given key
    fn find_entry_for_key(&mut self, key: K, hash: usize) -> &mut Entry<K, V> {
        let mut i = hash & self.mask_;
        loop {
            if self.table_[i].hash == 0 {
                return &mut self.table_[i];
            }
            if self.table_[i].hash == hash && self.table_[i].key == key {
                return &mut self.table_[i];
            }
            i = self.next_entry_index(i);
        }
    }

    fn compute_hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let mut hash = hasher.finish() as usize;
        if hash == 0 {
            hash = 1;
        }
        hash
    }

    fn resize_if_needed(&mut self) {
        if self.table_.len() as f64 * K_NEED_RESIZE_PERCENTAGE > self.entry_count_ as f64 {
            return;
        }

        assert!(self.table_.len() <= usize::MAX / K_GROWTH_FACTOR);

        let old_table = std::mem::replace(&mut self.table_, vec![
            Entry {
                hash: 0,
                key: unsafe { MaybeUninit::zeroed().assume_init() }, // Placeholder, overwritten on insertion
                value: unsafe { MaybeUninit::zeroed().assume_init() }, // Placeholder, overwritten on insertion
                depth_neighboring_entry: None,
            };
            self.table_.len() * K_GROWTH_FACTOR
        ]);

        self.mask_ = self.table_.len() - 1;
        assert_eq!(self.mask_.count_ones(), usize::BITS);

        let mut new_depths_heads: Vec<Option<Box<Entry<K, V>>>> = vec![None; self.depths_heads_.len()];

        for depth_idx in 0..self.depths_heads_.len() {
            let mut entry = self.depths_heads_[depth_idx].take();
            new_depths_heads[depth_idx] = None;

            while let Some(mut boxed_entry) = entry {
                let hash = boxed_entry.hash;
                let key = boxed_entry.key.clone(); // Clone the key before moving the entry
                let next_entry = boxed_entry.depth_neighboring_entry.take();
                let destination = self.find_entry_for_key(key.clone(), hash); // Use the cloned key
                let destination_index = destination as *const Entry<K, V> as usize - self.table_.as_ptr() as usize;
                //let destination_index = destination as usize - &self.table_[0] as *const Entry<K, V> as usize;
                let mut e = self.table_.get_mut(destination_index / std::mem::size_of::<Entry<K,V>>()).unwrap();
                *e = Entry {
                    hash: boxed_entry.hash,
                    key: boxed_entry.key,
                    value: boxed_entry.value,
                    depth_neighboring_entry: new_depths_heads[depth_idx].take(),
                };
                new_depths_heads[depth_idx] = Some(Box::new(Entry {
                    hash: e.hash,
                    key: e.key.clone(),
                    value: e.value.clone(),
                    depth_neighboring_entry: None,
                }));

                entry = next_entry;
            }
        }
        self.depths_heads_ = new_depths_heads;
    }
}

// Placeholder for base::bits::RoundUpToPowerOfTwo32
fn round_up_to_power_of_two(x: u32) -> u32 {
    let mut x = x;
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x + 1
}