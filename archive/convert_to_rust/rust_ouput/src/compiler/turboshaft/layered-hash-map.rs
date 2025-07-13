// Converted from V8 C++ source files:
// Header: layered-hash-map.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn round_up_to_power_of_two32(v: u32) -> u32 {
            let mut v = v;
            v -= 1;
            v |= v >> 1;
            v |= v >> 2;
            v |= v >> 4;
            v |= v >> 8;
            v |= v >> 16;
            v += 1;
            v
        }

        pub fn count_population(mask: usize) -> u32 {
            mask.count_ones()
        }

        pub fn count_leading_zeros(mask: usize) -> u32 {
            mask.leading_zeros()
        }
    }
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(size: usize) -> Vector<T>
        where
            T: Default + Clone,
        {
            Vector {
                data: vec![T::default(); size],
            }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            self.data.get_mut(index)
        }
    }

    impl<T> std::ops::Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T> std::ops::IndexMut<usize> for Vector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }
}

pub mod compiler {
    pub mod turboshaft {
        use crate::base;
        use crate::zone::Zone;
        use std::cmp;
        use std::marker::PhantomData;
        use std::mem::size_of;
        use std::optional::Option;

        pub struct FastHash<K> {
            _phantom: PhantomData<K>,
        }

        impl<K> FastHash<K> {
            pub fn new() -> Self {
                FastHash {
                    _phantom: PhantomData,
                }
            }
            pub fn call(&self, key: K) -> usize
            where
                K: std::hash::Hash,
            {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};

                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                hasher.finish() as usize
            }
        }

        // LayeredHashMap is a hash map whose elements are groupped into layers, such
        // that it's efficient to remove all of the items from the last inserted layer.
        // In addition to the regular Insert/Get/Contains functions of hash maps, it
        // thus provides two additional functions: StartLayer to indicate that future
        // insertions are part of a new layer, and DropLastLayer to remove all of the
        // items of the last layer.
        //
        // LayeredHashMap does not support inserting multiple values with the same key,
        // and does not support updating already-inserted items in the map. If you need
        // to update an existing key, you'll need to remove it (by calling DropLastLayer
        // as many times as needed), and then re-insert it.
        //
        // The implementation uses a regular ZoneVector for the main hash table, while
        // keeping a linked list of items per layer. When inserting an item in the
        // LayeredHashMap, we insert it into the ZoneVector and link it to the linked
        // list of the current (=latest) layer. In order to remove all of the items from
        // the last layer, we iterate its linked list, and remove the items one by one
        // from the ZoneVector, after which we drop the linked list alltogether.

        pub struct LayeredHashMap<Key, Value>
        where
            Key: Default + Clone + PartialEq,
            Value: Default + Clone,
        {
            mask_: usize,
            entry_count_: usize,
            table_: base::Vector<Entry<Key, Value>>,
            depths_heads_: ZoneVector<*mut Entry<Key, Value>>,
            zone_: *mut Zone,
            _key_type: PhantomData<Key>,
            _value_type: PhantomData<Value>,
        }

        #[derive(Clone)]
        struct Entry<Key, Value>
        where
            Key: Default + Clone + PartialEq,
            Value: Default + Clone,
        {
            hash: usize,
            key: Key,
            value: Value,
            depth_neighboring_entry: *mut Entry<Key, Value>,
        }

        impl<Key, Value> Default for Entry<Key, Value>
        where
            Key: Default + Clone + PartialEq,
            Value: Default + Clone,
        {
            fn default() -> Self {
                Entry {
                    hash: 0,
                    key: Key::default(),
                    value: Value::default(),
                    depth_neighboring_entry: std::ptr::null_mut(),
                }
            }
        }

        impl<Key, Value> LayeredHashMap<Key, Value>
        where
            Key: Default + Clone + PartialEq + std::hash::Hash,
            Value: Default + Clone,
        {
            pub fn new(zone: *mut Zone, initial_capacity: u32) -> Self {
                // Setting the minimal capacity to 16
                let mut initial_capacity = cmp::max::<u32>(initial_capacity, 16);
                // {initial_capacity} should be a power of 2, so that we can compute offset
                // in {table_} with a mask rather than a modulo.
                initial_capacity = base::bits::round_up_to_power_of_two32(initial_capacity);
                let mask_ = (initial_capacity - 1) as usize;
                // Allocating the table_

                let mut result = LayeredHashMap {
                    mask_: mask_,
                    entry_count_: 0,
                    table_: unsafe { (*zone).NewVector::<Entry<Key, Value>>(initial_capacity as usize) },
                    depths_heads_: ZoneVector::new(zone),
                    zone_: zone,
                    _key_type: PhantomData,
                    _value_type: PhantomData,
                };
                result
            }

            pub fn start_layer(&mut self) {
                self.depths_heads_.push_back(std::ptr::null_mut());
            }

            pub fn drop_last_layer(&mut self) {
                if self.depths_heads_.size() == 0 {
                    return;
                }

                let mut entry = self.depths_heads_.back();

                while entry != std::ptr::null_mut() {
                    self.entry_count_ -= 1;
                    let next = unsafe { (*entry).depth_neighboring_entry };
                    unsafe {
                        *entry = Entry::default();
                    }
                    entry = next;
                }

                self.depths_heads_.pop_back();
            }

            pub fn insert_new_key(&mut self, key: Key, value: Value) {
                self.resize_if_needed();
                let hash = self.compute_hash(key.clone());
                let destination = self.find_entry_for_key(key.clone(), hash);
                if unsafe { (*destination).hash != 0 } {
                    panic!("DCHECK_EQ(destination->hash, 0) failed");
                }
                unsafe {
                    *destination = Entry {
                        hash: hash,
                        key: key.clone(),
                        value: value.clone(),
                        depth_neighboring_entry: self.depths_heads_.back(),
                    };

                    if self.depths_heads_.size() > 0 {
                      self.depths_heads_.set_back(destination);
                    }
                }
                self.entry_count_ += 1;
            }

            pub fn get(&mut self, key: Key) -> Option<Value> {
                let destination = self.find_entry_for_key(key.clone(), self.compute_hash(key));
                if unsafe { (*destination).hash == 0 } {
                    return Option::None;
                }
                unsafe { Option::Some((*destination).value.clone()) }
            }

            pub fn contains(&mut self, key: Key) -> bool {
                self.get(key).is_some()
            }

            fn resize_if_needed(&mut self) {
                let k_need_resize_percentage: f64 = 0.75;
                let k_growth_factor: usize = 2;

                if (self.table_.len() as f64 * k_need_resize_percentage) > self.entry_count_ as f64 {
                    return;
                }

                if self.table_.len() > usize::MAX / k_growth_factor {
                    panic!("CHECK_LE(table_.size(), std::numeric_limits<size_t>::max() / kGrowthFactor) failed")
                }
                let new_size = self.table_.len() * k_growth_factor;
                self.table_ = unsafe { (*self.zone_).NewVector::<Entry<Key, Value>>(new_size) };
                self.mask_ = self.table_.len() - 1;
                if base::bits::count_population(self.mask_)
                    != (size_of::<usize>() * 8 - base::bits::count_leading_zeros(self.mask_)) as u32
                {
                    println!(
                        "mask = {}, population = {}, zeros = {}",
                        self.mask_,
                        base::bits::count_population(self.mask_),
                        base::bits::count_leading_zeros(self.mask_)
                    );
                    panic!("DCHECK_EQ(base::bits::CountPopulation(mask_), sizeof(mask_) * 8 - base::bits::CountLeadingZeros(mask_)) failed");
                }

                for depth_idx in 0..self.depths_heads_.size() {
                    // It's important to fill the new hash by inserting data in increasing
                    // depth order, in order to avoid holes when later calling DropLastLayer.
                    // Consider for instance:
                    //
                    //  ---+------+------+------+----
                    //     |  a1  |  a2  |  a3  |
                    //  ---+------+------+------+----
                    //
                    // Where a1, a2 and a3 have the same hash. By construction, we know that
                    // depth(a1) <= depth(a2) <= depth(a3). If, when re-hashing, we were to
                    // insert them in another order, say:
                    //
                    //  ---+------+------+------+----
                    //     |  a3  |  a1  |  a2  |
                    //  ---+------+------+------+----
                    //
                    // Then, when we'll call DropLastLayer to remove entries from a3's depth,
                    // we'll get this:
                    //
                    //  ---+------+------+------+----
                    //     | null |  a1  |  a2  |
                    //  ---+------+------+------+----
                    //
                    // And, when looking if a1 is in the hash, we'd find a "null" where we
                    // expect it, and assume that it's not present. If, instead, we always
                    // conserve the increasing depth order, then when removing a3, we'd get:
                    //
                    //  ---+------+------+------+----
                    //     |  a1  |  a2  | null |
                    //  ---+------+------+------+----
                    //
                    // Where we can still find a1 and a2.
                    let mut entry = self.depths_heads_[depth_idx];
                    self.depths_heads_.set(depth_idx, std::ptr::null_mut());

                    while entry != std::ptr::null_mut() {
                        let new_entry_loc =
                            self.find_entry_for_key(unsafe { (*entry).key.clone() }, unsafe { (*entry).hash });
                        unsafe {
                            *new_entry_loc = (*entry).clone();
                        }
                        let next_entry = unsafe { (*entry).depth_neighboring_entry };
                        unsafe {
                            (*new_entry_loc).depth_neighboring_entry = self.depths_heads_[depth_idx];
                            if self.depths_heads_.size() > 0 {
                               self.depths_heads_.set(depth_idx, new_entry_loc);
                            }
                        }
                        entry = next_entry;
                    }
                }
            }

            fn next_entry_index(&self, index: usize) -> usize {
                (index + 1) & self.mask_
            }

            fn find_entry_for_key(&mut self, key: Key, hash: usize) -> *mut Entry<Key, Value> {
                let mut i = hash & self.mask_;
                loop {
                    let table_entry = self.table_.get_mut(i).unwrap();
                    if table_entry.hash == 0 {
                        return table_entry;
                    }
                    if table_entry.hash == hash && table_entry.key == key {
                        return table_entry;
                    }
                    i = self.next_entry_index(i);
                }
            }

            fn compute_hash(&self, key: Key) -> usize {
                let hash = FastHash::new().call(key);
                if hash == 0 {
                    1
                } else {
                    hash
                }
            }
        }

        pub struct ZoneVector<T> {
            data: Vec<T>,
            zone_: *mut Zone,
        }

        impl<T> ZoneVector<T> {
            pub fn new(zone: *mut Zone) -> Self {
                ZoneVector {
                    data: Vec::new(),
                    zone_: zone,
                }
            }

            pub fn push_back(&mut self, value: T)
            where
                T: Clone,
            {
                self.data.push(value);
            }

            pub fn pop_back(&mut self) {
                self.data.pop();
            }

            pub fn back(&mut self) -> *mut T {
                if let Some(last) = self.data.last_mut() {
                    last as *mut T
                } else {
                    std::ptr::null_mut()
                }
            }
            pub fn set_back(&mut self, value: *mut T){
                if let Some(last) = self.data.last_mut() {
                    *last = unsafe { *value };
                }
            }
            pub fn set(&mut self, index: usize, value: *mut T){
                self.data[index] = unsafe { *value };
            }
            pub fn size(&self) -> usize {
                self.data.len()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }
        }

        impl<T> std::ops::Index<usize> for ZoneVector<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.data[index]
            }
        }
        impl<T> std::ops::IndexMut<usize> for ZoneVector<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index]
            }
        }
    }
}

pub mod zone {
    use crate::base;

    pub struct Zone {
        // For now, just use a placeholder.  Need to properly implement memory
        // allocation within a zone.
        name_: String,
    }
    impl Zone {
        pub fn NewVector<T>(&mut self, initial_capacity: usize) -> base::Vector<T>
        where
            T: Default + Clone,
        {
            base::Vector::new(initial_capacity)
        }
        pub fn new(name: String) -> Zone {
            Zone { name_: name }
        }
    }
}
