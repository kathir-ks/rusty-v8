// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

mod common;
mod execution;
mod heap;
mod objects;
mod wasm;

use std::{
    marker::PhantomData,
    num::TryFromIntError,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use self::{
    common::globals::V8_ENABLE_WEBASSEMBLY,
    heap::factory::Factory,
    objects::{
        descriptor_array::PropertyDetails,
        js_objects::{JSArray, JSGlobalObject, JSObject, JSReceiver},
        map::Map,
        objects::{Object, PropertyCell, String, Symbol},
        property_details::{
            FieldIndex, PropertyAttributes, PropertyConstness, PropertyLocation, Representation,
        },
    },
};

/// Placeholder for Isolate, Heap, Factory, etc.
pub struct Isolate {
    heap: Heap,
    factory: Factory,
}

impl Isolate {
    pub fn heap(&self) -> &Heap {
        &self.heap
    }
    pub fn factory(&self) -> &Factory {
        &self.factory
    }
}

pub struct Heap {}

impl Heap {
    pub fn new() -> Self {
        Heap {}
    }
}

pub struct LocalIsolate {}

pub mod lookup {
    use super::*;
    use std::{
        fmt::{Debug, Display},
        limits::MAX,
    };

    /// Represents a valid handle type for Name or Number.
    pub trait ValidKey: Object {}

    impl ValidKey for Name {}
    impl ValidKey for f64 {}

    /// Represents a PropertyKey in V8.
    #[derive(Clone, Copy)]
    pub struct PropertyKey {
        name_: DirectHandle<Name>,
        index_: usize,
    }

    impl PropertyKey {
        pub fn new_from_index(isolate: &mut Isolate, index: f64) -> Self {
            PropertyKey {
                name_: DirectHandle::empty(),
                index_: index as usize, //FIXME
            }
        }

        pub fn new_from_name(isolate: &mut Isolate, name: DirectHandle<Name>) -> Self {
            PropertyKey {
                name_: name,
                index_: usize::MAX,
            }
        }

        pub fn new<T: ValidKey>(isolate: &mut Isolate, valid_key: DirectHandle<T>) -> Self {
            PropertyKey {
                name_: DirectHandle::empty(), //FIXME
                index_: usize::MAX,
            }
        }

        pub fn new_with_success<T>(
            isolate: &mut Isolate,
            key: DirectHandle<T>,
            success: &mut bool,
        ) -> Self {
            *success = true;
            PropertyKey {
                name_: DirectHandle::empty(), //FIXME
                index_: usize::MAX,
            }
        }

        pub fn is_element(&self) -> bool {
            self.index_ != usize::MAX
        }

        pub fn name(&self) -> DirectHandle<Name> {
            self.name_
        }

        pub fn index(&self) -> usize {
            self.index_
        }

        pub fn get_name(&self, isolate: &mut Isolate) -> DirectHandle<Name> {
            self.name_
        }

        fn new_internal(
            isolate: &mut Isolate,
            name: DirectHandle<Name>,
            index: usize,
        ) -> PropertyKey {
            PropertyKey { name_: name, index_: index }
        }
    }

    /// Represents a LookupIterator in V8.
    pub struct LookupIterator {
        configuration_: Configuration,
        state_: State,
        has_property_: bool,
        interceptor_state_: InterceptorState,
        property_details_: PropertyDetails,
        isolate_: *mut Isolate, // raw pointer
        name_: DirectHandle<Name>,
        transition_: DirectHandle<UnionOfMapPropertyCell>,
        receiver_: DirectHandle<JSAny>,
        holder_: DirectHandle<JSReceiver>,
        lookup_start_object_: DirectHandle<JSAny>,
        index_: usize,
        number_: InternalIndex,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Configuration {
        OWN_SKIP_INTERCEPTOR = 0,
        OWN = kInterceptor as isize,
        PROTOTYPE_CHAIN_SKIP_INTERCEPTOR = kPrototypeChain as isize,
        PROTOTYPE_CHAIN = (kPrototypeChain | kInterceptor) as isize,
        DEFAULT = PROTOTYPE_CHAIN as isize,
    }

    const kInterceptor: i32 = 1 << 0;
    const kPrototypeChain: i32 = 1 << 1;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        NOT_FOUND,
        TYPED_ARRAY_INDEX_NOT_FOUND,
        ACCESS_CHECK,
        INTERCEPTOR,
        JSPROXY,
        ACCESSOR,
        DATA,
        WASM_OBJECT,
        TRANSITION,
        BEFORE_PROPERTY = INTERCEPTOR as isize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum InterceptorState {
        kUninitialized,
        kSkipNonMasking,
        kProcessNonMasking,
    }

    impl LookupIterator {
        pub fn new(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Name>,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                name,
                usize::MAX,
                receiver,
                configuration,
            )
        }

        pub fn new_with_start_object(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Name>,
            lookup_start_object: DirectHandle<JSAny>,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                name,
                usize::MAX,
                lookup_start_object,
                configuration,
            )
        }

        pub fn new_from_index(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            index: usize,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                DirectHandle::empty(),
                index,
                receiver,
                configuration,
            )
        }

        pub fn new_from_index_with_start_object(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            index: usize,
            lookup_start_object: DirectHandle<JSAny>,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                DirectHandle::empty(),
                index,
                lookup_start_object,
                configuration,
            )
        }

        pub fn new_from_property_key(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            key: &PropertyKey,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                key.name(),
                key.index(),
                receiver,
                configuration,
            )
        }

        pub fn new_from_property_key_with_start_object(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            key: &PropertyKey,
            lookup_start_object: DirectHandle<JSAny>,
            configuration: Configuration,
        ) -> Self {
            LookupIterator::new_internal(
                isolate,
                receiver,
                key.name(),
                key.index(),
                lookup_start_object,
                configuration,
            )
        }

        pub fn new_for_private_symbol(
            isolate: &mut Isolate,
            configuration: Configuration,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Symbol>,
        ) -> Self {
            LookupIterator::new_internal_private_symbol(
                isolate,
                configuration,
                receiver,
                name,
                receiver,
            )
        }

        fn new_internal(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Name>,
            index: usize,
            lookup_start_object: DirectHandle<JSAny>,
            configuration: Configuration,
        ) -> Self {
            let mut this = LookupIterator {
                configuration_: configuration,
                state_: State::NOT_FOUND,
                has_property_: false,
                interceptor_state_: InterceptorState::kUninitialized,
                property_details_: PropertyDetails::empty(),
                isolate_: isolate, // Store raw pointer
                name_: name,
                transition_: DirectHandle::empty(),
                receiver_: receiver,
                holder_: DirectHandle::empty(),
                lookup_start_object_: lookup_start_object,
                index_: index,
                number_: InternalIndex::NotFound(),
            };
            if this.is_element() {
                this.start::<true>();
            } else {
                this.start::<false>();
            }
            this
        }

        fn new_internal_private_symbol(
            isolate: &mut Isolate,
            configuration: Configuration,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Symbol>,
            lookup_start_object: DirectHandle<JSAny>,
        ) -> Self {
            let mut this = LookupIterator {
                configuration_: configuration,
                state_: State::NOT_FOUND,
                has_property_: false,
                interceptor_state_: InterceptorState::kUninitialized,
                property_details_: PropertyDetails::empty(),
                isolate_: isolate, // Store raw pointer
                name_: DirectHandle::empty(), //FIXME
                transition_: DirectHandle::empty(),
                receiver_: receiver,
                holder_: DirectHandle::empty(),
                lookup_start_object_: lookup_start_object,
                index_: usize::MAX,
                number_: InternalIndex::NotFound(),
            };
            this
        }

        pub fn restart(&mut self) {
            let state = InterceptorState::kUninitialized;
            if self.is_element() {
                self.restart_internal::<true>(state);
            } else {
                self.restart_internal::<false>(state);
            }
        }

        pub fn recheck_typed_array_bounds(&mut self) {
            todo!()
        }

        pub fn isolate(&self) -> &Isolate {
            unsafe { &*self.isolate_ }
        }

        pub fn state(&self) -> State {
            self.state_
        }

        pub fn name(&self) -> DirectHandle<Name> {
            self.name_
        }

        pub fn get_name(&self) -> DirectHandle<Name> {
            self.name_
        }

        pub fn index(&self) -> usize {
            self.index_
        }

        pub fn array_index(&self) -> u32 {
            assert!(self.index_ <= JSArray::kMaxArrayIndex as usize);
            self.index_ as u32
        }

        pub fn get_key(&self) -> PropertyKey {
            PropertyKey::new_internal(unsafe { &mut *self.isolate_ }, self.name_, self.index_)
        }

        pub fn is_element(&self) -> bool {
            self.index_ != usize::MAX
        }

        pub fn is_element_for_object(&self, object: Tagged<JSReceiver>) -> bool {
            todo!()
        }

        pub fn is_private_name(&self) -> bool {
            todo!()
        }

        pub fn is_found(&self) -> bool {
            self.state_ != State::NOT_FOUND
        }

        pub fn next(&mut self) {
            todo!()
        }

        pub fn not_found(&mut self) {
            self.has_property_ = false;
            self.state_ = State::NOT_FOUND;
        }

        pub fn heap(&self) -> &Heap {
            self.isolate().heap()
        }

        pub fn factory(&self) -> &Factory {
            self.isolate().factory()
        }

        pub fn get_receiver(&self) -> DirectHandle<JSAny> {
            self.receiver_
        }

        pub fn get_store_target<T>(&self) -> DirectHandle<T> {
            todo!()
        }

        pub fn is_dictionary_holder(&self) -> bool {
            todo!()
        }

        pub fn transition_map(&self) -> DirectHandle<Map> {
            todo!()
        }

        pub fn transition_cell(&self) -> DirectHandle<PropertyCell> {
            todo!()
        }

        pub fn get_holder<T>(&self) -> DirectHandle<T> {
            todo!()
        }

        pub fn lookup_start_object(&self) -> DirectHandle<JSAny> {
            self.lookup_start_object_
        }

        pub fn holder_is_receiver(&self) -> bool {
            todo!()
        }

        pub fn holder_is_receiver_or_hidden_prototype(&self) -> bool {
            todo!()
        }

        pub fn check_prototype_chain(&self) -> bool {
            (self.configuration_ as i32 & kPrototypeChain) != 0
        }

        pub fn has_access(&self) -> bool {
            todo!()
        }

        pub fn extending_non_extensible(&self, receiver: DirectHandle<JSReceiver>) -> bool {
            todo!()
        }

        pub fn prepare_for_data_property(&mut self, value: DirectHandle<Object>) {
            todo!()
        }

        pub fn prepare_transition_to_data_property(
            &mut self,
            receiver: DirectHandle<JSReceiver>,
            value: DirectHandle<Object>,
            attributes: PropertyAttributes,
            store_origin: StoreOrigin,
        ) {
            todo!()
        }

        pub fn is_cacheable_transition(&self) -> bool {
            todo!()
        }

        pub fn apply_transition_to_data_property(&mut self, receiver: DirectHandle<JSReceiver>) {
            todo!()
        }

        pub fn reconfigure_data_property(
            &mut self,
            value: DirectHandle<Object>,
            attributes: PropertyAttributes,
        ) {
            todo!()
        }

        pub fn delete(&mut self) {
            todo!()
        }

        pub fn transition_to_accessor_property(
            &mut self,
            getter: DirectHandle<Object>,
            setter: DirectHandle<Object>,
            attributes: PropertyAttributes,
        ) {
            todo!()
        }

        pub fn transition_to_accessor_pair(
            &mut self,
            pair: DirectHandle<Object>,
            attributes: PropertyAttributes,
        ) {
            todo!()
        }

        pub fn property_details(&self) -> PropertyDetails {
            assert!(self.has_property_);
            self.property_details_
        }

        pub fn property_attributes(&self) -> PropertyAttributes {
            self.property_details().attributes()
        }

        pub fn is_configurable(&self) -> bool {
            self.property_details().is_configurable()
        }

        pub fn is_read_only(&self) -> bool {
            self.property_details().is_read_only()
        }

        pub fn is_enumerable(&self) -> bool {
            self.property_details().is_enumerable()
        }

        pub fn representation(&self) -> Representation {
            self.property_details().representation()
        }

        pub fn location(&self) -> PropertyLocation {
            self.property_details().location()
        }

        pub fn constness(&self) -> PropertyConstness {
            self.property_details().constness()
        }

        pub fn get_field_index(&self) -> FieldIndex {
            todo!()
        }

        pub fn get_field_descriptor_index(&self) -> i32 {
            todo!()
        }

        pub fn get_accessor_index(&self) -> i32 {
            todo!()
        }

        pub fn get_property_cell(&self) -> DirectHandle<PropertyCell> {
            todo!()
        }

        pub fn get_accessors(&self) -> DirectHandle<Object> {
            todo!()
        }

        pub fn get_interceptor(&self) -> DirectHandle<InterceptorInfo> {
            todo!()
        }

        pub fn get_interceptor_for_failed_access_check(&self) -> DirectHandle<InterceptorInfo> {
            todo!()
        }

        pub fn get_data_value(
            &self,
            allocation_policy: AllocationPolicy,
        ) -> Handle<Object> {
            todo!()
        }

        pub fn write_data_value(&mut self, value: DirectHandle<Object>, initializing_store: bool) {
            todo!()
        }

        pub fn get_data_value_with_tag(&self, tag: SeqCstAccessTag) -> DirectHandle<Object> {
            todo!()
        }

        pub fn write_data_value_with_tag(&mut self, value: DirectHandle<Object>, tag: SeqCstAccessTag) {
            todo!()
        }

        pub fn swap_data_value(
            &mut self,
            value: DirectHandle<Object>,
            tag: SeqCstAccessTag,
        ) -> DirectHandle<Object> {
            todo!()
        }

        pub fn compare_and_swap_data_value(
            &mut self,
            expected: DirectHandle<Object>,
            value: DirectHandle<Object>,
            tag: SeqCstAccessTag,
        ) -> DirectHandle<Object> {
            todo!()
        }

        pub fn update_protector(&mut self) {
            todo!()
        }

        pub fn update_protector_static(
            isolate: &mut Isolate,
            receiver: DirectHandle<JSAny>,
            name: DirectHandle<Name>,
        ) {
            todo!()
        }

        pub fn try_lookup_cached_property(&mut self, accessor: DirectHandle<AccessorPair>) -> bool {
            todo!()
        }

        pub fn try_lookup_cached_property_no_accessor(&mut self) -> bool {
            todo!()
        }

        pub fn has_internal_marker_property(
            isolate: &mut Isolate,
            object: Tagged<JSReceiver>,
            marker: DirectHandle<Symbol>,
        ) -> bool {
            todo!()
        }

        fn lookup_cached_property(&mut self, accessor: DirectHandle<AccessorPair>) -> bool {
            todo!()
        }

        fn get_receiver_map(&self) -> DirectHandle<Map> {
            todo!()
        }

        fn next_holder(&self, map: Tagged<Map>) -> Tagged<JSReceiver> {
            todo!()
        }

        fn is_js_array_element(&self, is_element: bool) -> bool {
            is_element && self.index_ <= JSArray::kMaxArrayIndex as usize
        }

        fn start<const is_element: bool>(&mut self) {
            todo!()
        }

        fn next_internal<const is_element: bool>(&mut self, map: Tagged<Map>, holder: Tagged<JSReceiver>) {
            todo!()
        }

        fn lookup_in_holder<const is_element: bool>(
            &mut self,
            map: Tagged<Map>,
            holder: Tagged<JSReceiver>,
        ) -> State {
            if is_special_receiver_map(map) {
                self.lookup_in_special_holder::<is_element>(map, holder)
            } else {
                self.lookup_in_regular_holder::<is_element>(map, holder)
            }
        }

        fn lookup_in_regular_holder<const is_element: bool>(
            &mut self,
            map: Tagged<Map>,
            holder: Tagged<JSReceiver>,
        ) -> State {
            todo!()
        }

        fn lookup_in_special_holder<const is_element: bool>(
            &mut self,
            map: Tagged<Map>,
            holder: Tagged<JSReceiver>,
        ) -> State {
            todo!()
        }

        fn restart_lookup_for_non_masking_interceptors<const is_element: bool>(&mut self) {
            self.restart_internal::<is_element>(InterceptorState::kProcessNonMasking);
        }

        fn restart_internal<const is_element: bool>(&mut self, interceptor_state: InterceptorState) {
            todo!()
        }

        fn fetch_value(&self, allocation_policy: AllocationPolicy) -> DirectHandle<Object> {
            todo!()
        }

        fn can_stay_const(&self, value: Tagged<Object>) -> bool {
            todo!()
        }

        fn dict_can_stay_const(&self, value: Tagged<Object>) -> bool {
            todo!()
        }

        fn reload_property_information<const is_element: bool>(&mut self) {
            todo!()
        }

        fn skip_interceptor<const is_element: bool>(&mut self, holder: Tagged<JSObject>) -> bool {
            todo!()
        }

        fn get_interceptor_for_holder<const is_element: bool>(
            &self,
            holder: Tagged<JSObject>,
        ) -> Tagged<InterceptorInfo> {
            todo!()
        }

        fn check_interceptor(&self) -> bool {
            (self.configuration_ as i32 & kInterceptor) != 0
        }

        fn descriptor_number(&self) -> InternalIndex {
            todo!()
        }

        fn dictionary_entry(&self) -> InternalIndex {
            todo!()
        }

        fn compute_configuration(
            isolate: &mut Isolate,
            configuration: Configuration,
            name: DirectHandle<Name>,
        ) -> Configuration {
            todo!()
        }

        fn get_root_for_non_js_receiver(
            isolate: &mut Isolate,
            lookup_start_object: DirectHandle<JSPrimitive>,
            index: usize,
            configuration: Configuration,
        ) -> Result<DirectHandle<JSReceiver>, ()> {
            todo!()
        }

        fn get_root(
            isolate: &mut Isolate,
            lookup_start_object: DirectHandle<JSAny>,
            index: usize,
            configuration: Configuration,
        ) -> Result<DirectHandle<JSReceiver>, ()> {
            todo!()
        }

        fn not_found_for_holder(&self, holder: Tagged<JSReceiver>) -> State {
            todo!()
        }
    }

    /// ConcurrentLookupIterator
    pub struct ConcurrentLookupIterator {}

    impl ConcurrentLookupIterator {
        pub fn try_get_own_cow_element(
            isolate: &mut Isolate,
            array_elements: Tagged<FixedArray>,
            elements_kind: ElementsKind,
            array_length: i32,
            index: usize,
        ) -> Option<Tagged<Object>> {
            todo!()
        }

        pub fn try_get_own_constant_element(
            result_out: &mut Tagged<Object>,
            isolate: &mut Isolate,
            local_isolate: &mut LocalIsolate,
            holder: Tagged<JSObject>,
            elements: Tagged<FixedArrayBase>,
            elements_kind: ElementsKind,
            index: usize,
        ) -> ResultType {
            todo!()
        }

        pub fn try_get_own_char(
            result_out: &mut Tagged<String>,
            isolate: &mut Isolate,
            local_isolate: &mut LocalIsolate,
            string: Tagged<String>,
            index: usize,
        ) -> ResultType {
            todo!()
        }

        pub fn try_get_property_cell(
            isolate: &mut Isolate,
            local_isolate: &mut LocalIsolate,
            holder: DirectHandle<JSGlobalObject>,
            name: DirectHandle<Name>,
        ) -> Option<DirectHandle<PropertyCell>> {
            todo!()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ResultType {
        kPresent,
        kNotPresent,
        kGaveUp,
    }
}

pub struct Name {}
pub struct JSPrimitive {}

pub struct AccessorPair {}
pub struct InterceptorInfo {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElementsKind {
    HOLEY_ELEMENTS,
    PACKED_ELEMENTS,
}

pub struct FixedArray {}
pub struct FixedArrayBase {}

pub enum AllocationPolicy {
    kAllocationAllowed,
    kAllocationForbidden,
}

pub struct SeqCstAccessTag {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOrigin {
    kMaybeWeak,
    kStrong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalIndex {
    NotFound(),
    Found(usize),
}

impl InternalIndex {
    const fn not_found() -> Self {
        InternalIndex::NotFound()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tagged<T> {
    _phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    fn empty() -> Self {
        Tagged {
            _phantom: PhantomData,
        }
    }
}

pub struct Handle<T> {
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    fn empty() -> Self {
        Handle {
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DirectHandle<T> {
    _phantom: PhantomData<T>,
}

impl<T> DirectHandle<T> {
    fn empty() -> Self {
        DirectHandle {
            _phantom: PhantomData,
        }
    }
}

pub union UnionOfMapPropertyCell {
    map: *mut Map,
    property_cell: *mut PropertyCell,
}