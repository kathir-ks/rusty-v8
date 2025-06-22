// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides utility structures and traits for managing persistent
// values within a map-like data structure.  It's designed to assist in
// scenarios where direct usage of standard containers with V8's Global
// values isn't feasible (e.g., pre-C++11 environments).

use std::collections::HashMap;
use std::any::Any;

pub type PersistentContainerValue = usize;
pub const PERSISTENT_CONTAINER_NOT_FOUND: PersistentContainerValue = 0;

#[derive(PartialEq, Eq)]
pub enum PersistentContainerCallbackType {
  NotWeak,
  WeakWithParameter,
  WeakWithInternalFields,
}

pub trait PersistentValueMapTraits<K, V> {
  type Impl;
  type Iterator<'a>;
  type WeakCallbackDataType;
  type MapType;

  fn empty(impl_: &Self::Impl) -> bool;
  fn size(impl_: &Self::Impl) -> usize;
  fn swap(a: &mut Self::Impl, b: &mut Self::Impl);
  fn begin(impl_: &Self::Impl) -> Self::Iterator<'_>;
  fn end(impl_: &Self::Impl) -> Self::Iterator<'_>;
  fn key(it: &Self::Iterator<'_>) -> &K;
  fn value(it: &Self::Iterator<'_>) -> PersistentContainerValue;
  fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue;
  fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue;
  fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue;
  const CALLBACK_TYPE: PersistentContainerCallbackType;
  fn weak_callback_parameter(map: &Self::MapType, key: &K, value: &V) -> Option<Box<dyn Any>>;
  fn map_from_weak_callback_info(data: &dyn Any) -> &Self::MapType;
  fn key_from_weak_callback_info(data: &dyn Any) -> K;
  fn dispose_callback_data(data: &mut dyn Any);
  fn dispose(isolate: &mut Isolate, value: PersistentContainerValue, key: K);
}

pub struct StdMapTraits<K, V> { }

impl<K: Eq + std::hash::Hash + Clone, V> PersistentValueMapTraits<K, V> for StdMapTraits<K, V> {
  type Impl = HashMap<K, PersistentContainerValue>;
  type Iterator<'a> = std::collections::hash_map::Iter<'a, K, PersistentContainerValue>;
  type WeakCallbackDataType = ();
  type MapType = PersistentValueMapBase<K, V, Self>;

  fn empty(impl_: &Self::Impl) -> bool {
    impl_.is_empty()
  }

  fn size(impl_: &Self::Impl) -> usize {
    impl_.len()
  }

  fn swap(a: &mut Self::Impl, b: &mut Self::Impl) {
    std::mem::swap(a, b);
  }

  fn begin(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn end(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn key(it: &Self::Iterator<'_>) -> &K {
    it.key()
  }

  fn value(it: &Self::Iterator<'_>) -> PersistentContainerValue {
    *it.value()
  }

  fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue {
    match impl_.insert(key.clone(), value) {
      Some(old_value) => old_value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.get(key) {
      Some(&value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.remove(key) {
      Some(value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  const CALLBACK_TYPE: PersistentContainerCallbackType = PersistentContainerCallbackType::NotWeak;

  fn weak_callback_parameter(_map: &Self::MapType, _key: &K, _value: &V) -> Option<Box<dyn Any>> {
    None
  }

  fn map_from_weak_callback_info(_data: &dyn Any) -> &Self::MapType {
        panic!("Not supported!");
  }

  fn key_from_weak_callback_info(_data: &dyn Any) -> K {
        panic!("Not supported!");
  }

  fn dispose_callback_data(_data: &mut dyn Any) {
        panic!("Not supported!");
  }

  fn dispose(_isolate: &mut Isolate, _value: PersistentContainerValue, _key: K) {}
}

pub struct DefaultPersistentValueMapTraits<K, V> {}

impl<K: Eq + std::hash::Hash + Clone, V> PersistentValueMapTraits<K, V> for DefaultPersistentValueMapTraits<K, V> {
  type Impl = HashMap<K, PersistentContainerValue>;
  type Iterator<'a> = std::collections::hash_map::Iter<'a, K, PersistentContainerValue>;
  type WeakCallbackDataType = ();
  type MapType = PersistentValueMapBase<K, V, Self>;

  fn empty(impl_: &Self::Impl) -> bool {
    impl_.is_empty()
  }

  fn size(impl_: &Self::Impl) -> usize {
    impl_.len()
  }

  fn swap(a: &mut Self::Impl, b: &mut Self::Impl) {
    std::mem::swap(a, b);
  }

  fn begin(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn end(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn key(it: &Self::Iterator<'_>) -> &K {
    it.key()
  }

  fn value(it: &Self::Iterator<'_>) -> PersistentContainerValue {
    *it.value()
  }

  fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue {
    match impl_.insert(key.clone(), value) {
      Some(old_value) => old_value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.get(key) {
      Some(&value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.remove(key) {
      Some(value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  const CALLBACK_TYPE: PersistentContainerCallbackType = PersistentContainerCallbackType::NotWeak;

  fn weak_callback_parameter(_map: &Self::MapType, _key: &K, _value: &V) -> Option<Box<dyn Any>> {
    None
  }

  fn map_from_weak_callback_info(_data: &dyn Any) -> &Self::MapType {
      panic!("Not supported!");
  }

  fn key_from_weak_callback_info(_data: &dyn Any) -> K {
      panic!("Not supported!");
  }

  fn dispose_callback_data(_data: &mut dyn Any) {
      panic!("Not supported!");
  }

  fn dispose(_isolate: &mut Isolate, _value: PersistentContainerValue, _key: K) {}
}

pub struct DefaultGlobalMapTraits<K, V> {}

impl<K: Eq + std::hash::Hash + Clone, V> PersistentValueMapTraits<K, V> for DefaultGlobalMapTraits<K, V> {
  type Impl = HashMap<K, PersistentContainerValue>;
  type Iterator<'a> = std::collections::hash_map::Iter<'a, K, PersistentContainerValue>;
  type WeakCallbackDataType = ();
  type MapType = PersistentValueMapBase<K, V, Self>;

  fn empty(impl_: &Self::Impl) -> bool {
    impl_.is_empty()
  }

  fn size(impl_: &Self::Impl) -> usize {
    impl_.len()
  }

  fn swap(a: &mut Self::Impl, b: &mut Self::Impl) {
    std::mem::swap(a, b);
  }

  fn begin(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn end(impl_: &Self::Impl) -> Self::Iterator<'_> {
    impl_.iter()
  }

  fn key(it: &Self::Iterator<'_>) -> &K {
    it.key()
  }

  fn value(it: &Self::Iterator<'_>) -> PersistentContainerValue {
    *it.value()
  }

  fn set(impl_: &mut Self::Impl, key: K, value: PersistentContainerValue) -> PersistentContainerValue {
    match impl_.insert(key.clone(), value) {
      Some(old_value) => old_value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn get(impl_: &Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.get(key) {
      Some(&value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  fn remove(impl_: &mut Self::Impl, key: &K) -> PersistentContainerValue {
    match impl_.remove(key) {
      Some(value) => value,
      None => PERSISTENT_CONTAINER_NOT_FOUND,
    }
  }

  const CALLBACK_TYPE: PersistentContainerCallbackType = PersistentContainerCallbackType::NotWeak;

  fn weak_callback_parameter(_map: &Self::MapType, _key: &K, _value: &V) -> Option<Box<dyn Any>> {
      None
  }

  fn map_from_weak_callback_info(_data: &dyn Any) -> &Self::MapType {
        panic!("Not supported!");
  }

  fn key_from_weak_callback_info(_data: &dyn Any) -> K {
        panic!("Not supported!");
  }

  fn dispose_callback_data(_data: &mut dyn Any) {
        panic!("Not supported!");
  }

  fn dispose(_isolate: &mut Isolate, _value: PersistentContainerValue, _key: K) {}
}

// Dummy structs/enums, since v8-function-callback.h and v8-persistent-handle.h are not convertible to Rust.
pub struct Isolate {}
pub struct Value {}
pub struct Local<T> {
    _phantom: std::marker::PhantomData<T>,
}
impl<T> Local<T> {
    pub fn new(_isolate: &Isolate, _value: &T) -> Self {
        Local {
            _phantom: std::marker::PhantomData
        }
    }
}
pub struct Global<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Global<T> {
    pub fn new(_isolate: &Isolate, _value: Local<T>) -> Self {
        Global{
            _phantom: std::marker::PhantomData
        }
    }
    pub fn pass(self) -> Self {
        self
    }
}

pub struct ReturnValue<T> {
    _phantom: std::marker::PhantomData<T>,
}
impl<T> ReturnValue<T> {
    pub fn set_internal(&mut self, _address: usize) {}
}

pub enum WeakCallbackType {
    kParameter,
    kInternalFields,
}

pub struct WeakCallbackInfo<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> WeakCallbackInfo<T> {
    pub fn get_isolate(&self) -> &Isolate {
        &Isolate {}
    }
    pub fn get_parameter(&self) -> &T {
        panic!("Not supported!")
    }
    pub fn set_second_pass_callback(&self, _callback: fn(_: &WeakCallbackInfo<T>)) {}
}

mod internal {
    pub struct ValueHelper {}
    impl ValueHelper {
        pub fn slot_as_value<T>(_address: usize) -> *mut T {
            std::ptr::null_mut()
        }
    }
}

/// A map wrapper that allows using `Global` as a mapped value.
///
/// C++11 embedders don't need this class, as they can use `Global`
/// directly in std containers.
///
/// The map relies on a backing map, whose type and accessors are described
/// by the `Traits` class. The backing map will handle values of type
/// `PersistentContainerValue`, with all conversion into and out of V8
/// handles being transparently handled by this class.
pub struct PersistentValueMapBase<K, V, Traits: PersistentValueMapTraits<K, V>> {
  isolate_: *mut Isolate,
  impl_: Traits::Impl,
  label_: *const i8, //const char*
  _phantom: std::marker::PhantomData<(K, V, Traits)>,
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> PersistentValueMapBase<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
  fn get_isolate(&self) -> *mut Isolate {
    self.isolate_
  }

  /// Return the size of the map.
  fn size(&self) -> usize {
    Traits::size(&self.impl_)
  }

  /// Return whether the map holds weak persistents.
  fn is_weak(&self) -> bool {
    Traits::CALLBACK_TYPE != PersistentContainerCallbackType::NotWeak
  }

  /// Get the value stored in the map.
  fn get(&self, key: &K) -> Option<Local<V>> {
    let p = Self::from_val(Traits::get(&self.impl_, key));
    if p.is_null() {
        return None;
    }
    //TODO: Check V8_ENABLE_DIRECT_HANDLE
    Some(Local::<V>::new(unsafe { &mut *self.isolate_ }, unsafe { &*p }))
  }

  /// Check whether a value is contained in the map.
  fn contains(&self, key: &K) -> bool {
    Traits::get(&self.impl_, key) != PERSISTENT_CONTAINER_NOT_FOUND
  }

  /// Get value stored in the map and set it in returnValue.
  /// Return true if a value was found.
  fn set_return_value(&self, key: &K, return_value: &mut ReturnValue<Value>) -> bool {
    self.set_return_value_from_val(return_value, Traits::get(&self.impl_, key))
  }

  /// Return value for key and remove it from the map.
  fn remove(&mut self, key: &K) -> Option<Global<V>> {
    let value = Traits::remove(&mut self.impl_, key);
    if value == PERSISTENT_CONTAINER_NOT_FOUND {
        return None;
    }
    Some(self.release(value))
  }

  /// Traverses the map repeatedly, in case side effects of disposal cause insertions.
  fn clear(&mut self) {
    while !Traits::empty(&self.impl_) {
      let mut impl_ = Traits::Impl::default(); //TODO: Correct way to initialize this?
      Traits::swap(&mut self.impl_, &mut impl_);

      let mut it = Traits::begin(&impl_);
      let end = Traits::end(&impl_);

      while Traits::key(&it) != Traits::key(&end) {
          let key = Traits::key(&it).clone();
          let value = Traits::value(&it);
          Traits::dispose(unsafe { &mut *self.isolate_ }, value, key);
          it = Traits::begin(&impl_); //Reassign iterator because of possible modification
      }
    }
  }

  /// Helper class for GetReference/SetWithReference. Do not use outside
  /// that context.
  pub struct PersistentValueReference {
    value_: PersistentContainerValue,
  }

  impl PersistentValueReference {
    pub fn new() -> Self {
      PersistentValueReference {
        value_: PERSISTENT_CONTAINER_NOT_FOUND,
      }
    }

    pub fn new_from_ref(other: &PersistentValueReference) -> Self {
      PersistentValueReference { value_: other.value_ }
    }

    pub fn new_local(&self, isolate: &Isolate) -> Option<Local<V>> {
        //TODO: Check V8_ENABLE_DIRECT_HANDLE
      let address = self.value_ as usize;
      let p = internal::ValueHelper::slot_as_value::<V>(address);
      if p.is_null() {
          return None;
      }
      Some(Local::<V>::new(isolate, unsafe { &*p }))
    }

    pub fn is_empty(&self) -> bool {
      self.value_ == PERSISTENT_CONTAINER_NOT_FOUND
    }

    pub fn set_return_value<T>(&self, return_value: &mut ReturnValue<T>) -> bool {
      self.set_return_value_from_val(return_value, self.value_)
    }

    pub fn reset(&mut self) {
      self.value_ = PERSISTENT_CONTAINER_NOT_FOUND;
    }

    fn new_with_value(value: PersistentContainerValue) -> Self {
      PersistentValueReference { value_: value }
    }

    fn set_value(&mut self, value: PersistentContainerValue) {
      self.value_ = value;
    }
  }

  /// Get a reference to a map value. This enables fast, repeated access
  /// to a value stored in the map while the map remains unchanged.
  ///
  /// Careful: This is potentially unsafe, so please use with care.
  /// The value will become invalid if the value for this key changes
  /// in the underlying map, as a result of Set or Remove for the same
  /// key; as a result of the weak callback for the same key; or as a
  /// result of calling Clear() or destruction of the map.
  fn get_reference(&self, key: &K) -> PersistentValueReference {
    PersistentValueReference::new_with_value(Traits::get(&self.impl_, key))
  }

  fn new(isolate: *mut Isolate, label: *const i8) -> Self {
    PersistentValueMapBase {
      isolate_: isolate,
      impl_: Traits::Impl::default(),
      label_: label,
      _phantom: std::marker::PhantomData,
    }
  }

  unsafe fn from_val(v: PersistentContainerValue) -> *mut V {
    let address = v as usize;
    internal::ValueHelper::slot_as_value::<V>(address)
  }

  fn clear_and_leak(persistent: &mut Global<V>) -> PersistentContainerValue {
    //TODO: Get the slot()
    let address: usize = 0; //persistent.slot()
    //persistent.Clear();
    address as PersistentContainerValue
  }

  fn leak(persistent: &Global<V>) -> PersistentContainerValue {
    //TODO: Get the slot()
    let address: usize = 0; //persistent.slot()
    address as PersistentContainerValue
  }

  /// Return a container value as Global and make sure the weak
  /// callback is properly disposed of. All remove functionality should go
  /// through this.
  fn release(&self, v: PersistentContainerValue) -> Global<V> {
    let mut p = Global::<V>{
        _phantom: std::marker::PhantomData,
    };
    //p.slot() = v as usize;
    if Traits::CALLBACK_TYPE != PersistentContainerCallbackType::NotWeak {
        //TODO:  && p.IsWeak()
        //Traits::DisposeCallbackData(p.template ClearWeak<typename Traits::WeakCallbackDataType>());
    }
    p
  }

  fn remove_weak(&mut self, key: &K) {
    let value = Traits::remove(&mut self.impl_, key);
    let mut p = Global::<V>{
        _phantom: std::marker::PhantomData,
    };
    //p.slot() = value as usize;
    //p.Reset();
  }

  fn annotate_strong_retainer(&self, persistent: &mut Global<V>) {
    //persistent.AnnotateStrongRetainer(self.label_);
    //TODO: Implement AnnotateStrongRetainer
  }

  fn set_return_value_from_val<T>(&self, return_value: &mut ReturnValue<T>, value: PersistentContainerValue) -> bool {
    let has_value = value != PERSISTENT_CONTAINER_NOT_FOUND;
    if has_value {
      return_value.set_internal(value as usize);
    }
    has_value
  }
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> Drop for PersistentValueMapBase<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
  fn drop(&mut self) {
    self.clear();
  }
}

pub struct PersistentValueMap<K, V, Traits: PersistentValueMapTraits<K, V>> {
    base: PersistentValueMapBase<K, V, Traits>
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> PersistentValueMap<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
    pub fn new(isolate: *mut Isolate, label: *const i8) -> Self {
        PersistentValueMap {
            base: PersistentValueMapBase::<K,V,Traits>::new(isolate, label)
        }
    }

  /// Put value into map. Depending on Traits::kIsWeak, the value will be held
  /// by the map strongly or weakly.
  /// Returns old value as Global.
  pub fn set(&mut self, key: K, value: Local<V>) -> Option<Global<V>> {
    let persistent = Global::<V>::new(unsafe { &mut *self.base.isolate_ }, value);
    self.set_unique(key, persistent)
  }

  /// Put value into map, like Set(const K&, Local<V>).
  pub fn set_global(&mut self, key: K, value: Global<V>) -> Option<Global<V>> {
    self.set_unique(key, value)
  }

  /// Put the value into the map, and set the 'weak' callback when demanded
  /// by the Traits class.
  fn set_unique(&mut self, key: K, mut persistent: Global<V>) -> Option<Global<V>> {
    if Traits::CALLBACK_TYPE == PersistentContainerCallbackType::NotWeak {
      self.base.annotate_strong_retainer(&mut persistent);
    } else {
        //TODO: Implement
      /*
      WeakCallbackType callback_type =
          Traits::kCallbackType == kWeakWithInternalFields
              ? WeakCallbackType::kInternalFields
              : WeakCallbackType::kParameter;
      auto value = Local<V>::New(this->isolate(), *persistent);
      persistent->template SetWeak<typename Traits::WeakCallbackDataType>(
          Traits::WeakCallbackParameter(this, key, value), WeakCallback,
          callback_type);
       */
    }

    let old_value = Traits::set(&mut self.base.impl_, key, PersistentValueMapBase::<K, V, Traits>::clear_and_leak(&mut persistent));
    if old_value == PERSISTENT_CONTAINER_NOT_FOUND {
        return None;
    }
    Some(self.base.release(old_value))
  }

  /// Put a value into the map and update the reference.
  /// Restrictions of GetReference apply here as well.
  pub fn set_with_reference(&mut self, key: K, mut value: Global<V>, reference: &mut PersistentValueMapBase<K, V, Traits>::PersistentValueReference) -> Option<Global<V>> {
    reference.set_value(PersistentValueMapBase::<K, V, Traits>::leak(&value));
    self.set_unique(key, value)
  }
}

pub struct GlobalValueMap<K, V, Traits: PersistentValueMapTraits<K, V>> {
    base: PersistentValueMapBase<K, V, Traits>
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> GlobalValueMap<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
    pub fn new(isolate: *mut Isolate, label: *const i8) -> Self {
        GlobalValueMap {
            base: PersistentValueMapBase::<K,V,Traits>::new(isolate, label)
        }
    }

  /// Put value into map. Depending on Traits::kIsWeak, the value will be held
  /// by the map strongly or weakly.
  /// Returns old value as Global.
  pub fn set(&mut self, key: K, value: Local<V>) -> Option<Global<V>> {
    let persistent = Global::<V>::new(unsafe { &mut *self.base.isolate_ }, value);
    self.set_unique(key, persistent)
  }

  /// Put value into map, like Set(const K&, Local<V>).
  pub fn set_global(&mut self, key: K, value: Global<V>) -> Option<Global<V>> {
    self.set_unique(key, value)
  }

  /// Put the value into the map, and set the 'weak' callback when demanded
  /// by the Traits class.
  fn set_unique(&mut self, key: K, mut persistent: Global<V>) -> Option<Global<V>> {
    if Traits::CALLBACK_TYPE == PersistentContainerCallbackType::NotWeak {
      self.base.annotate_strong_retainer(&mut persistent);
    } else {
        //TODO: Implement
      /*
      WeakCallbackType callback_type =
          Traits::kCallbackType == kWeakWithInternalFields
              ? WeakCallbackType::kInternalFields
              : WeakCallbackType::kParameter;
      auto value = Local<V>::New(this->isolate(), *persistent);
      persistent->template SetWeak<typename Traits::WeakCallbackDataType>(
          Traits::WeakCallbackParameter(this, key, value), OnWeakCallback,
          callback_type);
       */
    }

    let old_value = Traits::set(&mut self.base.impl_, key, PersistentValueMapBase::<K, V, Traits>::clear_and_leak(&mut persistent));
    if old_value == PERSISTENT_CONTAINER_NOT_FOUND {
        return None;
    }
    Some(self.base.release(old_value))
  }

  /// Put a value into the map and update the reference.
  /// Restrictions of GetReference apply here as well.
  pub fn set_with_reference(&mut self, key: K, mut value: Global<V>, reference: &mut PersistentValueMapBase<K, V, Traits>::PersistentValueReference) -> Option<Global<V>> {
    reference.set_value(PersistentValueMapBase::<K, V, Traits>::leak(&value));
    self.set_unique(key, value)
  }

  fn remove_weak(&mut self, key: &K) {
    self.base.remove_weak(key);
  }
}

//Not supported because Rust doesn't support static methods inside traits
/*
  fn on_weak_callback(data: &WeakCallbackInfo<typename Traits::WeakCallbackDataType>) {
    if Traits::kCallbackType != PersistentContainerCallbackType::NotWeak {
        //TODO: Implement
      /*
      auto map = Traits::MapFromWeakCallbackInfo(data);
      K key = Traits::KeyFromWeakCallbackInfo(data);
      map->RemoveWeak(key);
      Traits::OnWeakCallback(data);
      data.SetSecondPassCallback(SecondWeakCallback);
       */
    }
  }

  fn second_weak_callback(data: &WeakCallbackInfo<typename Traits::WeakCallbackDataType>) {
    //Traits::DisposeWeak(data);
      //TODO: Implement
  }
*/

pub struct StdPersistentValueMap<K, V, Traits = DefaultPersistentValueMapTraits<K, V>> {
    map: PersistentValueMap<K, V, Traits>
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> StdPersistentValueMap<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
    pub fn new(isolate: *mut Isolate) -> Self {
        StdPersistentValueMap {
            map: PersistentValueMap::<K,V,Traits>::new(isolate, std::ptr::null())
        }
    }
}

pub struct StdGlobalValueMap<K, V, Traits = DefaultGlobalMapTraits<K, V>> {
    map: GlobalValueMap<K, V, Traits>
}

impl<K, V, Traits: PersistentValueMapTraits<K, V>> StdGlobalValueMap<K, V, Traits>
where K: Eq + std::hash::Hash + Clone {
    pub fn new(isolate: *mut Isolate) -> Self {
        StdGlobalValueMap {
            map: GlobalValueMap::<K,V,Traits>::new(isolate, std::ptr::null())
        }
    }
}