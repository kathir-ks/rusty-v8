// Copyright 2012 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Copyright 2023 the V8 project authors. All rights reserved.
// This file is a clone of "base/containers/small_map.h" in chromium.
// Keep in sync, especially when fixing bugs.

pub mod small_map {
    use std::cmp::Eq;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash};
    use std::mem::{self, MaybeUninit};
    use std::ops::{Deref, DerefMut};
    use std::ptr;

    /// SmallMap is a container with a std::map-like interface. It starts out backed
    /// by an unsorted array but switches to some other container type if it grows
    /// beyond this fixed size.
    ///
    /// PROS
    ///
    ///  - Good memory locality and low overhead for smaller maps.
    ///  - Handles large maps without the degenerate performance of an array.
    ///
    /// CONS
    ///
    ///  - Larger code size than the alternatives.
    ///
    /// IMPORTANT NOTES
    ///
    ///  - Iterators are invalidated across mutations.
    ///
    /// DETAILS
    ///
    /// SmallMap will pick up the comparator from the underlying map type. In
    /// std::map only a "less" operator is defined, which requires us to do two
    /// comparisons per element when doing the brute-force search in the simple
    /// array. std::unordered_map has a key_equal function which will be used.
    ///
    /// We define default overrides for the common map types to avoid this
    /// double-compare, but you should be aware of this if you use your own operator<
    /// for your map and supply your own version of == to the SmallMap. You can use
    /// regular operator== by just doing:
    ///
    ///   SmallMap<std::map<MyKey, MyValue>, 4, std::equal_to<MyKey>>
    ///
    ///
    /// USAGE
    /// -----
    ///
    /// NormalMap:  The map type to fall back to. This also defines the key and value
    ///             types for the SmallMap.
    /// kArraySize:  The size of the initial array of results. This will be allocated
    ///              with the SmallMap object rather than separately on the heap.
    ///              Once the map grows beyond this size, the map type will be used
    ///              instead.
    /// EqualKey:  A functor which tests two keys for equality. If the wrapped map
    ///            type has a "key_equal" member (unordered_map does), then that will
    ///            be used by default. If the wrapped map type has a strict weak
    ///            ordering "key_compare" (std::map does), that will be used to
    ///            implement equality by default.
    /// MapInit: A functor that takes a NormalMap* and uses it to initialize the map.
    ///          This functor will be called at most once per SmallMap, when the map
    ///          exceeds the threshold of kArraySize and we are about to copy values
    ///          from the array to the map. The functor *must* initialize the
    ///          NormalMap* argument with placement new, since after it runs we
    ///          assume that the NormalMap has been initialized.
    ///
    /// Example:
    ///   SmallMap<std::map<string, int>> days;
    ///   days["sunday"   ] = 0;
    ///   days["monday"   ] = 1;
    ///   days["tuesday"  ] = 2;
    ///   days["wednesday"] = 3;
    ///   days["thursday" ] = 4;
    ///   days["friday"   ] = 5;
    ///   days["saturday" ] = 6;

    mod internal {
        use std::collections::hash_map::RandomState;
        use std::hash::BuildHasher;

        pub struct SmallMapDefaultInit {}

        impl SmallMapDefaultInit {
            pub fn new() -> Self {
                SmallMapDefaultInit {}
            }
        }

        // This cannot be directly translated to Rust
        // Rust doesn't allow trait implementations based on the existence of associated types
        // pub trait HasKeyEqual<M> {
        //     const VALUE: bool;
        // }

        // Implementations would go here, but they're not possible without specialization or
        // a similar feature.
        // impl<M> HasKeyEqual<M> for M {
        //     default const VALUE: bool = false;
        // }

        // impl<M: MyTraitWithKeyEqual> HasKeyEqual<M> for M {
        //     const VALUE: bool = true;
        // }

        pub struct SelectEqualKey<M> {
            _phantom: std::marker::PhantomData<M>,
        }

        impl<M> SelectEqualKey<M> {
            pub fn new() -> Self {
                SelectEqualKey {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<K, V, S> SelectEqualKey<std::collections::HashMap<K, V, S>>
        where
            K: Eq + Hash,
            S: BuildHasher,
        {
            // This associated type cannot be implemented due to current rust limitations
            // Need to use trait specialization/defaults
            // type equal_key = std::collections::hash_map::DefaultHasher;
        }

        // Base template used for map types that do NOT have an M::key_equal member,
        // e.g., std::map<>. These maps have a strict weak ordering comparator rather
        // than an equality functor, so equality will be implemented in terms of that
        // comparator.
        //
        // There's a partial specialization of this template below for map types that do
        // have an M::key_equal member.

        pub struct EqualKey<M>(std::marker::PhantomData<M>);

        impl<M> EqualKey<M> {
            pub fn new() -> Self {
                EqualKey(std::marker::PhantomData)
            }

            pub fn compare<T: PartialOrd>(left: &T, right: &T) -> bool {
                left.lt(right)
            }
        }
    }

    pub struct SmallMap<NormalMap, const K_ARRAY_SIZE: usize, EqualKey, MapInit> {
        size_: usize,
        functor_: MapInit,
        storage: SmallMapStorage<NormalMap, K_ARRAY_SIZE>,
        _equal_key: std::marker::PhantomData<EqualKey>,
    }

    union SmallMapStorage<NormalMap, const K_ARRAY_SIZE: usize> {
        array: [MaybeUninit<(
            <NormalMap as MapLike>::Key,
            <NormalMap as MapLike>::Value,
        )>; K_ARRAY_SIZE],
        map_: MaybeUninit<NormalMap>,
    }

    trait MapLike {
        type Key;
        type Value;
        type Iter<'a>: Iterator<Item = (&'a Self::Key, &'a Self::Value)>
        where
            Self: 'a;
        type IterMut<'a>: Iterator<Item = (&'a Self::Key, &'a mut Self::Value)>
        where
            Self: 'a;

        fn new() -> Self;
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
        fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
        fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;
        fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;
        fn len(&self) -> usize;
        fn clear(&mut self);
        fn iter(&self) -> Self::Iter<'_>;
        fn iter_mut(&mut self) -> Self::IterMut<'_>;
    }

    impl<K: Eq + Hash, V, S: BuildHasher + Default> MapLike for std::collections::HashMap<K, V, S> {
        type Key = K;
        type Value = V;
        type Iter<'a> = std::collections::hash_map::Iter<'a, K, V>;
        type IterMut<'a> = std::collections::hash_map::IterMut<'a, K, V>;

        fn new() -> Self {
            std::collections::HashMap::with_hasher(S::default())
        }

        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            std::collections::HashMap::insert(self, key, value)
        }

        fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
            std::collections::HashMap::get(self, key)
        }

        fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
            std::collections::HashMap::get_mut(self, key)
        }

        fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
            std::collections::HashMap::remove(self, key)
        }

        fn len(&self) -> usize {
            std::collections::HashMap::len(self)
        }

        fn clear(&mut self) {
            std::collections::HashMap::clear(self)
        }

        fn iter(&self) -> Self::Iter<'_> {
            std::collections::HashMap::iter(self)
        }

        fn iter_mut(&mut self) -> Self::IterMut<'_> {
            std::collections::HashMap::iter_mut(self)
        }
    }

    impl<
        NormalMap: MapLike,
        const K_ARRAY_SIZE: usize,
        EqualKey,
        MapInit: Default, // Use default trait bound
    > SmallMap<NormalMap, K_ARRAY_SIZE, EqualKey, MapInit>
    {
        const K_USING_FULL_MAP_SENTINEL: usize = usize::MAX;

        // static_assert(kArraySize > 0, "Initial size must be greater than 0");
        // static_assert(kArraySize != kUsingFullMapSentinel,
        //               "Initial size out of range");

        pub fn new() -> Self {
            Self {
                size_: 0,
                functor_: MapInit::default(),
                storage: SmallMapStorage {
                    array: unsafe { MaybeUninit::uninit().assume_init() },
                },
                _equal_key: std::marker::PhantomData,
            }
        }

        pub fn with_functor(functor: MapInit) -> Self {
            Self {
                size_: 0,
                functor_: functor,
                storage: SmallMapStorage {
                    array: unsafe { MaybeUninit::uninit().assume_init() },
                },
                _equal_key: std::marker::PhantomData,
            }
        }

        pub fn len(&self) -> usize {
            if self.using_full_map() {
                self.map().len()
            } else {
                self.size_
            }
        }

        pub fn is_empty(&self) -> bool {
            if self.using_full_map() {
                self.map().is_empty()
            } else {
                self.size_ == 0
            }
        }

        fn array(&self) -> &[MaybeUninit<(NormalMap::Key, NormalMap::Value)>; K_ARRAY_SIZE] {
            unsafe { &self.storage.array }
        }

        fn array_mut(&mut self) -> &mut [MaybeUninit<(NormalMap::Key, NormalMap::Value)>; K_ARRAY_SIZE] {
            unsafe { &mut self.storage.array }
        }

        fn map(&self) -> &NormalMap {
            assert!(self.using_full_map());
            unsafe { self.storage.map_.assume_init_ref() }
        }

        fn map_mut(&mut self) -> &mut NormalMap {
            assert!(self.using_full_map());
            unsafe { self.storage.map_.assume_init_mut() }
        }

        pub fn insert(&mut self, key: NormalMap::Key, value: NormalMap::Value) -> Option<NormalMap::Value>
        where
            EqualKey: Fn(&NormalMap::Key, &NormalMap::Key) -> bool,
        {
            let compare =
            |a: &NormalMap::Key, b: &NormalMap::Key| -> bool {
                let equal_key: EqualKey = unsafe { std::mem::transmute_copy(&self._equal_key) };
                equal_key(a,b)
            };

            if self.using_full_map() {
                return self.map_mut().insert(key, value);
            }

            for i in 0..self.size_ {
                let existing_key = unsafe {&self.array()[i].assume_init_ref().0};
                if compare(existing_key, &key) {
                    let mut existing_pair = unsafe {self.array_mut()[i].assume_init_mut()};
                    return Some(std::mem::replace(&mut existing_pair.1, value));
                }
            }

            if self.size_ == K_ARRAY_SIZE {
                self.convert_to_real_map();
                return self.map_mut().insert(key, value);
            }

            let pair = (key, value);
            unsafe {
                self.array_mut()[self.size_].as_mut_ptr().write(pair);
            }

            self.size_ += 1;

            None
        }

        pub fn get(&self, key: &NormalMap::Key) -> Option<&NormalMap::Value>
        where
            EqualKey: Fn(&NormalMap::Key, &NormalMap::Key) -> bool,
        {
            let compare =
            |a: &NormalMap::Key, b: &NormalMap::Key| -> bool {
                let equal_key: EqualKey = unsafe { std::mem::transmute_copy(&self._equal_key) };
                equal_key(a,b)
            };

            if self.using_full_map() {
                return self.map().get(key);
            }

            for i in 0..self.size_ {
                let existing_key = unsafe {&self.array()[i].assume_init_ref().0};

                if compare(existing_key, key) {
                    let existing_value = unsafe {&self.array()[i].assume_init_ref().1};
                    return Some(existing_value);
                }
            }

            None
        }

        pub fn get_mut(&mut self, key: &NormalMap::Key) -> Option<&mut NormalMap::Value>
        where
            EqualKey: Fn(&NormalMap::Key, &NormalMap::Key) -> bool,
        {
            let compare =
            |a: &NormalMap::Key, b: &NormalMap::Key| -> bool {
                let equal_key: EqualKey = unsafe { std::mem::transmute_copy(&self._equal_key) };
                equal_key(a,b)
            };

            if self.using_full_map() {
                return self.map_mut().get_mut(key);
            }

            for i in 0..self.size_ {
                let existing_key = unsafe {&self.array()[i].assume_init_ref().0};

                if compare(existing_key, key) {
                    let existing_value = unsafe {&mut self.array_mut()[i].assume_init_mut().1};
                    return Some(existing_value);
                }
            }

            None
        }

        pub fn remove(&mut self, key: &NormalMap::Key) -> Option<NormalMap::Value>
        where
            EqualKey: Fn(&NormalMap::Key, &NormalMap::Key) -> bool,
        {
            let compare =
            |a: &NormalMap::Key, b: &NormalMap::Key| -> bool {
                let equal_key: EqualKey = unsafe { std::mem::transmute_copy(&self._equal_key) };
                equal_key(a,b)
            };

            if self.using_full_map() {
                return self.map_mut().remove(key);
            }

            for i in 0..self.size_ {
                let existing_key = unsafe {&self.array()[i].assume_init_ref().0};

                if compare(existing_key, key) {
                    let removed_pair = unsafe { ptr::read(self.array_mut()[i].as_mut_ptr()) };
                    self.size_ -= 1;

                    if i != self.size_ {
                        let last_pair = unsafe { ptr::read(self.array_mut()[self.size_].as_mut_ptr()) };
                         unsafe { ptr::write(self.array_mut()[i].as_mut_ptr(), last_pair); }
                         //Manually drop the last pair
                         unsafe { ptr::drop_in_place(self.array_mut()[self.size_].as_mut_ptr()); }
                    }
                    else {
                        unsafe { ptr::drop_in_place(self.array_mut()[self.size_].as_mut_ptr()); }
                    }

                    return Some(removed_pair.1);
                }
            }

            None
        }

        fn convert_to_real_map(&mut self) {
            // Storage for the elements in the temporary array.
            let mut temp: [MaybeUninit<(NormalMap::Key, NormalMap::Value)>; K_ARRAY_SIZE] =
                unsafe { MaybeUninit::uninit().assume_init() };

            // Move the current elements into a temporary array.
            for i in 0..K_ARRAY_SIZE {
                let pair = unsafe { ptr::read(self.array_mut()[i].as_mut_ptr()) };
                unsafe {
                    temp[i].as_mut_ptr().write(pair);
                }
            }

            // Initialize the map.
            self.size_ = Self::K_USING_FULL_MAP_SENTINEL;
            let map = NormalMap::new();
            unsafe {
                self.storage.map_.write(map);
            }

            // Insert elements into it.
            for i in 0..K_ARRAY_SIZE {
                let mut pair = unsafe { ptr::read(temp[i].as_mut_ptr()) };
                self.map_mut().insert(pair.0,pair.1);
                //Drop the temporary array elements that were moved.
                unsafe { ptr::drop_in_place(temp[i].as_mut_ptr()); }
            }
        }

        pub fn clear(&mut self) {
            if self.using_full_map() {
                self.map_mut().clear();
            } else {
                for i in 0..self.size_ {
                    unsafe {
                        ptr::drop_in_place(self.array_mut()[i].as_mut_ptr());
                    }
                }
            }
            self.size_ = 0;
        }

        pub fn using_full_map(&self) -> bool {
            self.size_ == Self::K_USING_FULL_MAP_SENTINEL
        }
    }

    impl<
        NormalMap: MapLike,
        const K_ARRAY_SIZE: usize,
        EqualKey,
        MapInit: Default,
    > Drop for SmallMap<NormalMap, K_ARRAY_SIZE, EqualKey, MapInit>
    {
        fn drop(&mut self) {
            if self.using_full_map() {
                unsafe {
                    ptr::drop_in_place(self.storage.map_.as_mut_ptr());
                }
            } else {
                for i in 0..self.size_ {
                    unsafe {
                        ptr::drop_in_place(self.array_mut()[i].as_mut_ptr());
                    }
                }
            }
        }
    }
}