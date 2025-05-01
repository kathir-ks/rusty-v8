// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(feature = "webassembly")]

pub mod wasm {
    /// Estimates the memory consumption of a `Vec<T>`.
    ///
    /// This function uses `capacity()` rather than `len()` to compute the
    /// actual memory consumption.  The size of the `Vec` struct itself is
    /// *not* included.
    pub fn content_size_vec<T>(vector: &Vec<T>) -> usize {
        vector.capacity() * std::mem::size_of::<T>()
    }

    /// Estimates the memory consumption of a `Vec<T>`.
    ///
    /// This function uses `size()` rather than `len()` to compute the
    /// actual memory consumption.  The size of the `Vec` struct itself is
    /// *not* included.

    pub fn content_size_owned_vector<T>(vector: &Vec<T>) -> usize {
        vector.len() * std::mem::size_of::<T>()
    }

    /// Estimates the memory consumption of a `HashMap<K, V>`.
    ///
    /// Very rough lower bound approximation: two internal pointers per entry.
    /// The size of the `HashMap` struct itself is *not* included.
    pub fn content_size_hashmap<K, V, H>(map: &std::collections::HashMap<K, V, H>) -> usize {
        map.len() * (std::mem::size_of::<K>() + std::mem::size_of::<V>() + 2 * std::mem::size_of::<usize>()) * 4 / 3 // 75% fill ratio
    }

     /// Estimates the memory consumption of a `BTreeMap<K, V>`.
    ///
    /// Very rough lower bound approximation: two internal pointers per entry.
    /// The size of the `BTreeMap` struct itself is *not* included.
    pub fn content_size_btreemap<K, V>(map: &std::collections::BTreeMap<K, V>) -> usize {
        map.len() * (std::mem::size_of::<K>() + std::mem::size_of::<V>() + 2 * std::mem::size_of::<usize>())
    }

    /// Estimates the memory consumption of a `HashSet<T>`.
    ///
    /// Very rough lower bound approximation: two internal pointers per entry.
    /// The size of the `HashSet` struct itself is *not* included.
    pub fn content_size_hashset<T, H>(set: &std::collections::HashSet<T, H>) -> usize {
        set.len() * (std::mem::size_of::<T>() + 2 * std::mem::size_of::<usize>()) * 4 / 3 // 75% fill ratio
    }
}