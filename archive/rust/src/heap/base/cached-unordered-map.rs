// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap_base {
    use hashbrown::HashMap;
    use std::hash::Hasher;
    use std::borrow::Borrow;

    // A cached map that speeds up `operator[]` if used in LRU fashion.
    pub struct CachedUnorderedMap<K, V, H = DefaultHasher>
    where
        K: Eq + std::hash::Hash,
        H: Fn(&K) -> u64,
    {
        last_key: Option<K>,
        last_mapped: *mut V,
        map_: HashMap<K, V, H>,
        hash_builder: H,
    }

    impl<K, V, H> CachedUnorderedMap<K, V, H>
    where
        K: Eq + std::hash::Hash + Clone,
        H: Fn(&K) -> u64 + Default,
    {
        pub fn new() -> Self
        where
            H: Default,
        {
            CachedUnorderedMap {
                last_key: None,
                last_mapped: std::ptr::null_mut(),
                map_: HashMap::default(),
                hash_builder: H::default(),
            }
        }
    }

    impl<K, V, H> CachedUnorderedMap<K, V, H>
    where
        K: Eq + std::hash::Hash + Clone,
        H: Fn(&K) -> u64,
    {
        pub fn with_hasher(hash_builder: H) -> Self {
            CachedUnorderedMap {
                last_key: None,
                last_mapped: std::ptr::null_mut(),
                map_: HashMap::with_hasher(hash_builder),
                hash_builder,
            }
        }

        pub fn get_mut(&mut self, key: &K) -> &mut V {
            if let Some(last_key) = &self.last_key {
                if *last_key == *key {
                    // SAFETY: `last_mapped` is guaranteed to be a valid pointer to a value in `map_`
                    // when `last_key` is `Some`.
                    return unsafe { &mut *self.last_mapped };
                }
            }

            let entry = self.map_.entry(key.clone()).or_insert_with(|| V::default());

            self.last_key = Some(key.clone());
            self.last_mapped = entry as *mut V;

            entry
        }

        pub fn erase<Q: ?Sized>(&mut self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: std::hash::Hash + Eq,
        {
            if let Some(last_key) = &self.last_key {
                if last_key.borrow() == key {
                    self.last_key = None;
                    self.last_mapped = std::ptr::null_mut();
                }
            }
            self.map_.remove(key).is_some()
        }

        pub fn find<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: std::hash::Hash + Eq,
        {
            self.map_.get(key)
        }

        pub fn find_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: std::hash::Hash + Eq,
        {
            self.map_.get_mut(key)
        }

        pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
            self.map_.iter()
        }

        pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, K, V> {
            self.map_.iter_mut()
        }

        pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: std::hash::Hash + Eq,
        {
            self.map_.contains_key(key)
        }

        pub fn clear(&mut self) {
            self.last_key = None;
            self.last_mapped = std::ptr::null_mut();
            self.map_.clear();
        }

        pub fn is_empty(&self) -> bool {
            self.map_.is_empty()
        }

        pub fn take(mut self) -> HashMap<K, V, H> {
            self.last_key = None;
            self.last_mapped = std::ptr::null_mut();
            let mut tmp = HashMap::with_hasher(self.hash_builder);
            std::mem::swap(&mut tmp, &mut self.map_);
            tmp
        }
    }

    impl<K, V, H> Default for CachedUnorderedMap<K, V, H>
    where
        K: Eq + std::hash::Hash + Clone,
        H: Fn(&K) -> u64 + Default,
        V: Default,
    {
        fn default() -> Self {
            CachedUnorderedMap::new()
        }
    }
}