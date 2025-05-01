// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::hash::Hash;

pub(crate) mod compiler {
    pub struct Node {}

    /// A cache for nodes based on a key. Useful for implementing canonicalization of
    /// nodes such as constants, parameters, etc.
    pub struct NodeCache<K, H = DefaultHasher<K>, P = std::cmp::Eq>
    where
        K: Eq + Hash + Copy,
        H: Fn(&K) -> u64,
        P: Fn(&K, &K) -> bool,
    {
        map_: HashMap<K, *mut Node>, // Using raw pointer. Needs careful memory management elsewhere
        _hasher: H,
        _pred: P,
    }

    impl<K, H, P> NodeCache<K, H, P>
    where
        K: Eq + Hash + Copy,
        H: Fn(&K) -> u64,
        P: Fn(&K, &K) -> bool,
    {
        pub fn new() -> Self {
            NodeCache {
                map_: HashMap::new(),
                _hasher: |_| 0, // Dummy values, replaced by proper default hashing and comparison later if needed
                _pred: |_, _| true,
            }
        }

        /// Search for node associated with {key} and return a pointer to a memory
        /// location in this cache that stores an entry for the key. If the location
        /// returned by this method contains a non-nullptr node, the caller can use
        /// that node. Otherwise it is the responsibility of the caller to fill the
        /// entry with a new node.
        pub fn find(&mut self, key: K) -> &mut *mut Node {
            self.map_.entry(key).or_insert(std::ptr::null_mut())
        }

        /// Appends all nodes from this cache to {nodes}.
        pub fn get_cached_nodes(&self, nodes: &mut Vec<*mut Node>) {
            for (_key, &node) in &self.map_ {
                if !node.is_null() {
                    nodes.push(node);
                }
            }
        }
    }

    impl<K: Eq + Hash + Copy> Default for NodeCache<K> {
        fn default() -> Self {
            Self::new()
        }
    }

    pub type Int32NodeCache = NodeCache<i32>;
    pub type Int64NodeCache = NodeCache<i64>;

    // All we want is the numeric value of the RelocInfo::Mode enum. We typedef
    // below to avoid pulling in assembler.h
    pub type RelocInfoMode = i8;
    pub type RelocInt32Key = (i32, RelocInfoMode);
    pub type RelocInt64Key = (i64, RelocInfoMode);
    pub type RelocInt32NodeCache = NodeCache<RelocInt32Key>;
    pub type RelocInt64NodeCache = NodeCache<RelocInt64Key>;

    #[cfg(target_arch = "x86_32")]
    pub type IntPtrNodeCache = Int32NodeCache;
    #[cfg(not(target_arch = "x86_32"))]
    pub type IntPtrNodeCache = Int64NodeCache;

    struct DefaultHasher<K>(std::marker::PhantomData<K>);

    impl<K> DefaultHasher<K> {
        const fn new() -> Self {
            Self(std::marker::PhantomData)
        }
    }

    impl<K: Hash> FnOnce<(&K,)> for DefaultHasher<K> {
        type Output = u64;

        extern "rust-call" fn call_once(self, args: (&K,)) -> Self::Output {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut s = DefaultHasher::new();
            args.0.hash(&mut s);
            s.finish()
        }
    }

    impl<K: Hash> FnMut<(&K,)> for DefaultHasher<K> {
        extern "rust-call" fn call_mut(&mut self, args: (&K,)) -> Self::Output {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut s = DefaultHasher::new();
            args.0.hash(&mut s);
            s.finish()
        }
    }

    impl<K: Hash> Fn<(&K,)> for DefaultHasher<K> {
        extern "rust-call" fn call(&self, args: (&K,)) -> Self::Output {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut s = DefaultHasher::new();
            args.0.hash(&mut s);
            s.finish()
        }
    }
} // namespace compiler