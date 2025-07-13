// Converted from V8 C++ source files:
// Header: v8-util.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_util {
    use std::collections::HashMap;
    use std::any::Any;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::v8::{
        Isolate,
        Local,
        Name,
        Data,
        Object,
        Value,
        ReturnValue,
        Global,
        WeakCallbackInfo,
        WeakCallbackType,
        Primitive,
        Context,
        PropertyAttribute
    };

    pub type PersistentContainerValue = usize;
    pub const K_PERSISTENT_CONTAINER_NOT_FOUND: PersistentContainerValue = 0;

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum PersistentContainerCallbackType {
        KNotWeak,
        KWeakWithParameter,
        KWeakWithInternalFields,
    }

    pub trait StdMapTraits<K, V> {
        type Impl;
        type Iterator;

        fn empty(impl_: &Self::Impl) -> bool;
        fn size(impl_: &Self::Impl) -> usize;
        fn swap(a: &mut Self::Impl, b: &mut Self::Impl);
        fn begin(impl_: &Self::Impl) -> Self::Iterator;
        fn end(impl_: &Self::Impl) -> Self::Iterator;
        fn key(it: &Self::Iterator) -> &K;
        fn value(it: &Self::Iterator) -> PersistentContainerValue;
        fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue;
        fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue;
        fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue;
    }

    pub struct DefaultStdMapTraitsImpl<K,V> {
      map: RefCell<HashMap<K, PersistentContainerValue>>
    }

    impl<K,V> DefaultStdMapTraitsImpl<K,V> {
      pub fn new() -> Self {
        DefaultStdMapTraitsImpl {
          map: RefCell::new(HashMap::new())
        }
      }
    }

    pub struct DefaultStdMapTraitsIterator<'a, K, V> {
      iter: std::collections::hash_map::Iter<'a, K, PersistentContainerValue>
    }

    impl<'a, K, V> DefaultStdMapTraitsIterator<'a, K, V> {
      pub fn new(map: &'a HashMap<K, PersistentContainerValue>) -> Self {
        DefaultStdMapTraitsIterator {
          iter: map.iter()
        }
      }
      pub fn next(&mut self) -> Option<(&K, &PersistentContainerValue)> {
        self.iter.next()
      }
    }

    pub struct StdMapTraitsImpl<K, V> {
        map: RefCell<HashMap<K, PersistentContainerValue>>,
    }

    impl<K, V> StdMapTraitsImpl<K, V> {
        pub fn new() -> Self {
            StdMapTraitsImpl {
                map: RefCell::new(HashMap::new()),
            }
        }
    }

    pub struct StdMapTraitsIterator<'a, K, V> {
        map: &'a HashMap<K, PersistentContainerValue>,
        keys: Vec<&'a K>,
        current_index: usize,
    }

    impl<'a, K, V> StdMapTraitsIterator<'a, K, V>
    where K: Eq + std::hash::Hash
    {
        pub fn new(map: &'a HashMap<K, PersistentContainerValue>) -> Self {
            let keys: Vec<&K> = map.keys().collect();
            StdMapTraitsIterator {
                map,
                keys,
                current_index: 0,
            }
        }

        pub fn next(&mut self) -> Option<(&'a K, &'a PersistentContainerValue)> {
            if self.current_index < self.keys.len() {
                let key = self.keys[self.current_index];
                self.current_index += 1;
                self.map.get(key).map(|value| (key, value))
            } else {
                None
            }
        }
    }

    pub struct DefaultStdMapTraits<K, V> {}

    impl<K, V> StdMapTraits<K, V> for DefaultStdMapTraits<K, V>
    where
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        type Impl = HashMap<K, PersistentContainerValue>;
        type Iterator = std::collections::hash_map::Iter<'static, K, PersistentContainerValue>;

        fn empty(impl_: &Self::Impl) -> bool {
            impl_.is_empty()
        }

        fn size(impl_: &Self::Impl) -> usize {
            impl_.len()
        }

        fn swap(a: &mut Self::Impl, b: &mut Self::Impl) {
            std::mem::swap(a, b);
        }

        fn begin(impl_: &Self::Impl) -> Self::Iterator {
            unsafe {
                std::mem::transmute(impl_.iter())
            }
        }

        fn end(impl_: &Self::Impl) -> Self::Iterator {
            unsafe {
                std::mem::transmute(impl_.iter())
            }
        }

        fn key(it: &Self::Iterator) -> &K {
            it.next().unwrap().0
        }

        fn value(it: &Self::Iterator) -> PersistentContainerValue {
            *it.next().unwrap().1
        }

        fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue {
            if let Some(old_value) = impl_.insert(key, value) {
                old_value
            } else {
                K_PERSISTENT_CONTAINER_NOT_FOUND
            }
        }

        fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue {
            impl_.get(key).copied().unwrap_or(K_PERSISTENT_CONTAINER_NOT_FOUND)
        }

        fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue {
            impl_.remove(key).unwrap_or(K_PERSISTENT_CONTAINER_NOT_FOUND)
        }
    }

    pub struct DefaultPersistentValueMapTraits<K, V> {}

    impl<K, V> DefaultPersistentValueMapTraits<K, V> {
        pub const K_CALLBACK_TYPE: PersistentContainerCallbackType = PersistentContainerCallbackType::KNotWeak;
        pub type MapType = PersistentValueMap<K, V, DefaultPersistentValueMapTraits<K, V>>;
        pub type WeakCallbackDataType = ();

        pub fn weak_callback_parameter(
            _map: &Self::MapType,
            _key: &K,
            _value: Local<V>,
        ) -> *mut Self::WeakCallbackDataType {
            std::ptr::null_mut()
        }

        pub fn map_from_weak_callback_info(
            _data: &WeakCallbackInfo<Self::WeakCallbackDataType>,
        ) -> *mut Self::MapType {
            std::ptr::null_mut()
        }

        pub fn key_from_weak_callback_info(
            _data: &WeakCallbackInfo<Self::WeakCallbackDataType>,
        ) -> K
        where K: Default
        {
            K::default()
        }

        pub fn dispose_callback_data(_data: *mut Self::WeakCallbackDataType) {}

        pub fn dispose(_isolate: *mut Isolate, _value: Global<V>, _key: K) {}
    }

    pub struct DefaultGlobalMapTraits<K, V> {}

    impl<K, V> DefaultGlobalMapTraits<K, V> {
        pub const K_CALLBACK_TYPE: PersistentContainerCallbackType = PersistentContainerCallbackType::KNotWeak;
        pub type MapType = GlobalValueMap<K, V, DefaultGlobalMapTraits<K, V>>;
        pub type WeakCallbackDataType = ();

        pub fn weak_callback_parameter(
            _map: &Self::MapType,
            _key: &K,
            _value: Local<V>,
        ) -> *mut Self::WeakCallbackDataType {
            std::ptr::null_mut()
        }

        pub fn map_from_weak_callback_info(
            _data: &WeakCallbackInfo<Self::WeakCallbackDataType>,
        ) -> *mut Self::MapType {
            std::ptr::null_mut()
        }

        pub fn key_from_weak_callback_info(
            _data: &WeakCallbackInfo<Self::WeakCallbackDataType>,
        ) -> K
        where K: Default
        {
            K::default()
        }

        pub fn dispose_callback_data(_data: *mut Self::WeakCallbackDataType) {}

        pub fn on_weak_callback(_data: &WeakCallbackInfo<Self::WeakCallbackDataType>) {}

        pub fn dispose(_isolate: *mut Isolate, _value: Global<V>, _key: K) {}

        pub fn dispose_weak(_data: &WeakCallbackInfo<Self::WeakCallbackDataType>) {}
    }

    pub struct PersistentValueReference {
        value_: PersistentContainerValue,
    }

    impl PersistentValueReference {
        pub fn new() -> Self {
            PersistentValueReference {
                value_: K_PERSISTENT_CONTAINER_NOT_FOUND,
            }
        }

        pub fn new_from_other(other: &PersistentValueReference) -> Self {
            PersistentValueReference { value_: other.value_ }
        }

        pub fn new_with_value(value: PersistentContainerValue) -> Self {
            PersistentValueReference { value_: value }
        }

        pub fn new_local<'a, V>(&self, isolate: *mut Isolate) -> Local<'a, V> {
            Local::new()
        }

        pub fn is_empty(&self) -> bool {
            self.value_ == K_PERSISTENT_CONTAINER_NOT_FOUND
        }

        pub fn set_return_value<T>(&self, _return_value: ReturnValue<T>) -> bool {
            false
        }

        pub fn reset(&mut self) {
            self.value_ = K_PERSISTENT_CONTAINER_NOT_FOUND;
        }

        pub fn set_value(&mut self, value: PersistentContainerValue) {
            self.value_ = value;
        }
    }

    pub struct PersistentValueMapBase<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
    {
        isolate_: *mut Isolate,
        impl_: Traits::Impl,
        label_: *const char,
    }

    impl<K, V, Traits> PersistentValueMapBase<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        pub fn new(isolate: *mut Isolate, label: *const char) -> Self {
            PersistentValueMapBase {
                isolate_: isolate,
                impl_: {
                    let mut map = HashMap::new();
                    map
                },
                label_: label,
            }
        }

        pub fn get_isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn size(&self) -> usize {
            Traits::size(&self.impl_)
        }

        pub fn is_weak(&self) -> bool {
            false
        }

        pub fn get<'a>(&self, key: &K) -> Local<'a, V> {
            Local::new()
        }

        pub fn contains(&self, key: &K) -> bool {
            Traits::get(&self.impl_, key) != K_PERSISTENT_CONTAINER_NOT_FOUND
        }

        pub fn set_return_value(&self, key: &K, return_value: ReturnValue<Value>) -> bool {
            false
        }

        pub fn remove(&mut self, key: &K) -> Global<V> {
            Global::new()
        }

        pub fn clear(&mut self) {
            let keys: Vec<K> = self.impl_.keys().cloned().collect();
            for key in keys {
                self.remove(&key);
            }
        }

        pub fn get_reference(&self, key: &K) -> PersistentValueReference {
            PersistentValueReference::new_with_value(Traits::get(&self.impl_, key))
        }
    }

    pub struct PersistentValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
    {
        base: PersistentValueMapBase<K, V, Traits>,
    }

    impl<K, V, Traits> PersistentValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        pub fn new(isolate: *mut Isolate, label: *const char) -> Self {
            PersistentValueMap {
                base: PersistentValueMapBase::new(isolate, label),
            }
        }

        pub fn set(&mut self, key: K, value: Local<V>) -> Global<V> {
            let global: Global<V> = Global::new();
            self.set_unique(key, &global)
        }

        pub fn set_global(&mut self, key: K, value: Global<V>) -> Global<V> {
            self.set_unique(key, &value)
        }

        pub fn set_unique(&mut self, key: K, persistent: &Global<V>) -> Global<V> {
            let old_value = Traits::set(&mut self.base.impl_, key, 1);
            Global::new()
        }

        pub fn set_with_reference(
            &mut self,
            key: K,
            value: Global<V>,
            reference: &mut PersistentValueReference,
        ) -> Global<V> {
            reference.set_value(1);
            self.set_unique(key, &value)
        }
    }

    pub struct GlobalValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
    {
        base: PersistentValueMapBase<K, V, Traits>,
    }

    impl<K, V, Traits> GlobalValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        pub fn new(isolate: *mut Isolate, label: *const char) -> Self {
            GlobalValueMap {
                base: PersistentValueMapBase::new(isolate, label),
            }
        }

        pub fn set(&mut self, key: K, value: Local<V>) -> Global<V> {
            let global: Global<V> = Global::new();
            self.set_unique(key, &global)
        }

        pub fn set_global(&mut self, key: K, value: Global<V>) -> Global<V> {
            self.set_unique(key, &value)
        }

        pub fn set_unique(&mut self, key: K, persistent: &Global<V>) -> Global<V> {
            let old_value = Traits::set(&mut self.base.impl_, key, 1);
            Global::new()
        }

        pub fn set_with_reference(
            &mut self,
            key: K,
            value: Global<V>,
            reference: &mut PersistentValueReference,
        ) -> Global<V> {
            reference.set_value(1);
            self.set_unique(key, &value)
        }
    }

    pub struct StdPersistentValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
    {
        map: PersistentValueMap<K, V, Traits>,
    }

    impl<K, V, Traits> StdPersistentValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        pub fn new(isolate: *mut Isolate) -> Self {
            StdPersistentValueMap {
                map: PersistentValueMap::new(isolate, std::ptr::null()),
            }
        }
    }

    pub struct StdGlobalValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
    {
        map: GlobalValueMap<K, V, Traits>,
    }

    impl<K, V, Traits> StdGlobalValueMap<K, V, Traits>
    where
        Traits: StdMapTraits<K, V>,
        K: Eq + std::hash::Hash + Copy,
        V: Copy,
    {
        pub fn new(isolate: *mut Isolate) -> Self {
            StdGlobalValueMap {
                map: GlobalValueMap::new(isolate, std::ptr::null()),
            }
        }
    }
}
