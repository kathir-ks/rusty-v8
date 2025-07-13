// Converted from V8 C++ source files:
// Header: lookup-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::convert::TryInto;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroUsize;
use std::ops::Range;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub enum Configuration {
    DEFAULT,
    SKIP_INTERCEPTOR,
    OWN_SKIP_INTERCEPTOR,
    PROTOTYPE_CHAIN,
    PROTOTYPE_CHAIN_SKIP_INTERCEPTOR,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct LookupIterator {
    configuration_: Configuration,
    isolate_: *mut Isolate,
    name_: DirectHandle<Name>,
    receiver_: DirectHandle<JSAny>,
    lookup_start_object_: DirectHandle<JSAny>,
    index_: usize,
    holder_: DirectHandle<JSReceiver>,
    state_: State,
    transition_: Tagged<Object>,
    number_: InternalIndex,
    has_property_: bool,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct PropertyKey {
    name_: DirectHandle<Name>,
    index_: usize,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalIndex {
    index: usize,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    START,
    ACCESS_CHECK,
    PROPERTY,
    INTERCEPTOR,
    TRANSITION,
    DONE,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/value-serializer.h
pub struct V8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/runtime/runtime-wasm.cc
pub struct code {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/source-text-module.h
pub struct v8 {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/ppc/macro-assembler-ppc.h
struct UseScratchRegisterScope {
    dummy: i32,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/embedder-data-slot-inl.h
struct AsAtomicTagged {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/code-stub-assembler.h
struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
    phantom: PhantomData<T>,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/runtime/runtime-wasm.cc
pub enum This {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/runtime/runtime-wasm.cc
pub struct If {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/code-stub-assembler.h
struct TVARIABLE<'a, T> {
    phantom: PhantomData<&'a T>,
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/property.h
pub enum DirectHandle<T> {
    _PhantomData(PhantomData<T>),
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/embedder-data-slot-inl.h
pub struct DisallowGarbageCollection {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/embedder-data-slot-inl.h
pub struct JSObject {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/x64/assembler-x64.h
pub struct Shared {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/arm/assembler-arm-inl.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/js-break-iterator.h
pub struct ReadOnlyRoots {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct Name {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct JSReceiver {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct JSAny {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct JSPrimitive {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct Symbol {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct HeapObject {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct Map {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct PropertyCell {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct InterceptorInfo {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct Isolate {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct Tagged<T> {
    phantom: PhantomData<T>,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct JSGlobalProxy {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup.h
pub struct JSGlobalObject {}

impl LookupIterator {
    const kInvalidIndex: usize = usize::MAX;

    pub fn new(
        isolate: *mut Isolate,
        receiver: DirectHandle<JSAny>,
        name: DirectHandle<Name>,
        configuration: Configuration,
    ) -> Self {
        LookupIterator::new_internal(
            isolate,
            receiver,
            name,
            LookupIterator::kInvalidIndex,
            receiver,
            configuration,
        )
    }

    pub fn new_with_lookup_start_object(
        isolate: *mut Isolate,
        receiver: DirectHandle<JSAny>,
        name: DirectHandle<Name>,
        lookup_start_object: DirectHandle<JSAny>,
        configuration: Configuration,
    ) -> Self {
        LookupIterator::new_internal(
            isolate,
            receiver,
            name,
            LookupIterator::kInvalidIndex,
            Self::cast_to_jsany(lookup_start_object),
            configuration,
        )
    }

    pub fn new_with_index(
        isolate: *mut Isolate,
        receiver: DirectHandle<JSAny>,
        index: usize,
        configuration: Configuration,
    ) -> Self {
        assert_ne!(index, LookupIterator::kInvalidIndex);
        LookupIterator::new_internal(
            isolate,
            receiver,
            DirectHandle::<Name>::_PhantomData(PhantomData),
            index,
            receiver,
            configuration,
        )
    }

    pub fn new_with_index_and_lookup_start_object(
        isolate: *mut Isolate,
        receiver: DirectHandle<JSAny>,
        index: usize,
        lookup_start_object: DirectHandle<JSAny>,
        configuration: Configuration,
    ) -> Self {
        assert_ne!(index, LookupIterator::kInvalidIndex);
        LookupIterator::new_internal(
            isolate,
            receiver,
            DirectHandle::<Name>::_PhantomData(PhantomData),
            index,
            Self::cast_to_jsany(lookup_start_object),
            configuration,
        )
    }

    pub fn new_with_property_key(
        isolate: *mut Isolate,
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

    pub fn new_with_property_key_and_lookup_start_object(
        isolate: *mut Isolate,
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
            Self::cast_to_jsany(lookup_start_object),
            configuration,
        )
    }

    // This private constructor is the central bottleneck that all the other
    // constructors use.
    fn new_internal(
        isolate: *mut Isolate,
        receiver: DirectHandle<JSAny>,
        name: DirectHandle<Name>,
        index: usize,
        lookup_start_object: DirectHandle<JSAny>,
        configuration: Configuration,
    ) -> Self {
        let configuration_ = Self::compute_configuration(isolate, configuration, name);
        let mut lookup_iterator = LookupIterator {
            configuration_: configuration_,
            isolate_: isolate,
            name_: name,
            receiver_: receiver,
            lookup_start_object_: lookup_start_object,
            index_: index,
            holder_: DirectHandle::<JSReceiver>::_PhantomData(PhantomData), // Initialize with a default value
            state_: State::START,                                           // Initialize with a default value
            transition_: Tagged::<Object> {
                phantom: PhantomData,
            }, // Initialize with a default value
            number_: InternalIndex { index: 0 },                            // Initialize with a default value
            has_property_: false,
        };

        if lookup_iterator.is_element() {
            // If we're not looking at a TypedArray, we will need the key represented
            // as an internalized string.
            if lookup_iterator.index_ > Self::kMaxElementIndex
                && !lookup_iterator.is_js_typed_array(lookup_iterator.lookup_start_object_, isolate)
            {
                if lookup_iterator.name_.is_null() {
                    lookup_iterator.name_ = lookup_iterator.factory().size_to_string(lookup_iterator.index_);
                }
                lookup_iterator.name_ = lookup_iterator.factory().internalize_name(lookup_iterator.name_);
            } else if !lookup_iterator.name_.is_null() && !lookup_iterator.is_internalized_string(lookup_iterator.name_) {
                // Maintain the invariant that if name_ is present, it is internalized.
                lookup_iterator.name_ = DirectHandle::<Name>::_PhantomData(PhantomData);
            }
            lookup_iterator.start::<true>();
        } else {
            assert!(!lookup_iterator.name_.is_null());
            lookup_iterator.name_ = lookup_iterator.factory().internalize_name(lookup_iterator.name_);

            lookup_iterator.start::<false>();
        }
        lookup_iterator
    }

    pub fn new_with_symbol(
        isolate: *mut Isolate,
        configuration: Configuration,
        receiver: DirectHandle<JSAny>,
        name: DirectHandle<Symbol>,
    ) -> Self {
        LookupIterator {
            configuration_: configuration,
            isolate_: isolate,
            name_: DirectHandle::<Name>::_PhantomData(PhantomData), //name.into(),
            receiver_: receiver,
            lookup_start_object_: receiver,
            index_: LookupIterator::kInvalidIndex,
            holder_: DirectHandle::<JSReceiver>::_PhantomData(PhantomData), // Initialize with a default value
            state_: State::START,                                           // Initialize with a default value
            transition_: Tagged::<Object> {
                phantom: PhantomData,
            }, // Initialize with a default value
            number_: InternalIndex { index: 0 },                            // Initialize with a default value
            has_property_: false,
        }
    }

    fn factory(&self) -> Factory {
        Factory {}
    }

    fn start<const is_element: bool>(&mut self) {}

    fn compute_configuration(
        isolate: *mut Isolate,
        configuration: Configuration,
        name: DirectHandle<Name>,
    ) -> Configuration {
        if !name.is_null() && name.is_private() {
            Configuration::OWN_SKIP_INTERCEPTOR
        } else {
            configuration
        }
    }

    fn cast_to_jsany(lookup_start_object: DirectHandle<JSAny>) -> DirectHandle<JSAny> {
        lookup_start_object
    }

    fn is_js_typed_array(&self, object: DirectHandle<JSAny>, isolate: *mut Isolate) -> bool {
        false
    }

    fn is_internalized_string(&self, name: DirectHandle<Name>) -> bool {
        false
    }

    fn is_element(&self) -> bool {
        self.index_ != Self::kInvalidIndex
    }

    fn check_prototype_chain(&self) -> bool {
        false
    }
}

impl PropertyKey {
    pub fn new(isolate: *mut Isolate, index: f64) -> Self {
        assert_eq!(index, index as u64 as f64);
        if cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64") {
            PropertyKey {
                index_: index as usize,
                name_: DirectHandle::<Name>::_PhantomData(PhantomData),
            }
        } else {
            if index <= LookupIterator::kMaxElementIndex as f64 {
                PropertyKey {
                    index_: index as usize,
                    name_: DirectHandle::<Name>::_PhantomData(PhantomData),
                }
            } else {
                let mut key = PropertyKey {
                    index_: LookupIterator::kInvalidIndex,
                    name_: DirectHandle::<Name>::_PhantomData(PhantomData),
                };
                key.name_ = Factory {}.internalize_string(
                    Factory {}.heap_number_to_string(Factory {}.new_heap_number(index), index),
                );
                key
            }
        }
    }

    pub fn new_with_name<HandleType>(isolate: *mut Isolate, name: HandleType, index: usize) -> Self
    where
        HandleType: Into<DirectHandle<Name>>,
    {
        let name_: DirectHandle<Name> = name.into();
        assert!(
            (index == LookupIterator::kInvalidIndex && name_.is_null())
                || (index != LookupIterator::kInvalidIndex)
        );
        PropertyKey {
            name_: name_,
            index_: index,
        }
    }

    pub fn new_with_name_only<HandleType>(isolate: *mut Isolate, name: HandleType) -> Self
    where
        HandleType: Into<DirectHandle<Name>>,
    {
        let name_: DirectHandle<Name> = name.into();
        let mut key = PropertyKey {
            index_: 0,
            name_: name_,
        };
        if key.name_.as_integer_index(&mut key.index_) {
            key.name_ = name_;
        } else {
            key.index_ = LookupIterator::kInvalidIndex;
            key.name_ = Factory {}.internalize_name(name_);
        }
        key
    }

    pub fn new_with_valid_key<T, HandleType>(isolate: *mut Isolate, valid_key: HandleType) -> Self
    where
        HandleType: Into<DirectHandle<T>>,
    {
        let valid_key_: DirectHandle<T> = valid_key.into();
        let valid_obj: DirectHandle<Object> = unsafe { std::mem::transmute(valid_key_) };

        let mut key = PropertyKey {
            index_: 0,
            name_: DirectHandle::<Name>::_PhantomData(PhantomData),
        };

        if Object::to_integer_index(*valid_obj, &mut key.index_) {
            return key;
        }

        if Object::is_number(*valid_obj) {
            let valid_obj: DirectHandle<Object> =
                Factory {}.number_to_string(valid_obj);
        }
        assert!(Object::is_name(*valid_obj));

        key.name_ = unsafe { std::mem::transmute(valid_obj) };
        if !key.name_.as_integer_index(&mut key.index_) {
            key.index_ = LookupIterator::kInvalidIndex;
            key.name_ = Factory {}.internalize_name(key.name_);
        }
        key
    }

    pub fn new_with_key<T, HandleType>(
        isolate: *mut Isolate,
        key: HandleType,
        success: &mut bool,
    ) -> Self
    where
        HandleType: Into<DirectHandle<T>>,
    {
        let key_: DirectHandle<T> = key.into();
        let mut property_key = PropertyKey {
            name_: DirectHandle::<Name>::_PhantomData(PhantomData),
            index_: 0,
        };
        if Object::to_integer_index(*unsafe { std::mem::transmute(key_) }, &mut property_key.index_) {
            *success = true;
            return property_key;
        }
        let to_name_result = Object::to_name(isolate, unsafe { std::mem::transmute(key_) });
        if !to_name_result.to_handle(&mut property_key.name_) {
            *success = false;
            property_key.index_ = LookupIterator::kInvalidIndex;
            return property_key;
        }
        *success = true;

        if !property_key.name_.as_integer_index(&mut property_key.index_) {
            property_key.name_ = Factory {}.internalize_name(property_key.name_);
            property_key.index_ = LookupIterator::kInvalidIndex;
        }
        property_key
    }

    pub fn is_element(&self) -> bool {
        self.index_ != LookupIterator::kInvalidIndex
    }

    pub fn name(&self) -> DirectHandle<Name> {
        if self.name_.is_null() {
            assert!(self.is_element());
            Factory {}.size_to_string(self.index_)
        } else {
            self.name_
        }
    }

    pub fn index(&self) -> usize {
        self.index_
    }
}

impl LookupIterator {
    pub fn name(&self) -> DirectHandle<Name> {
        self.name_
    }

    pub fn get_name(&mut self) -> DirectHandle<Name> {
        if self.name_.is_null() {
            assert!(self.is_element());
            self.name_ = self.factory().size_to_string(self.index_);
        }
        self.name_
    }

    pub fn get_key(&self) -> PropertyKey {
        PropertyKey::new_with_name(PhantomData, self.name_, self.index_)
    }

    pub fn is_element_for_object(&self, object: Tagged<JSReceiver>) -> bool {
        self.index_ <= Self::kMaxElementIndex
    }

    pub fn is_private_name(&self) -> bool {
        !self.is_element() && self.name().is_private()
    }

    pub fn is_dictionary_holder(&self) -> bool {
        !self.holder_.has_fast_properties(self.isolate_)
    }

    pub fn extending_non_extensible(&self, receiver: DirectHandle<JSReceiver>) -> bool {
        true
    }

    const kMaxElementIndex: usize = 0x3FFFFFFF;
}

pub trait AsIntegerIndex {
    fn as_integer_index(&self, index: &mut usize) -> bool;
}

impl AsIntegerIndex for DirectHandle<Name> {
    fn as_integer_index(&self, index: &mut usize) -> bool {
        *index = 0;
        false
    }
}

pub trait IsNull {
    fn is_null(&self) -> bool;
}

impl<T> IsNull for DirectHandle<T> {
    fn is_null(&self) -> bool {
        match self {
            DirectHandle::<T>::_PhantomData(_)=>true,
        }
    }
}

pub trait IsPrivate {
    fn is_private(&self) -> bool;
}

impl IsPrivate for DirectHandle<Name> {
    fn is_private(&self) -> bool {
        false
    }
}

pub struct Factory {}

impl Factory {
    fn internalize_name(&self, name: DirectHandle<Name>) -> DirectHandle<Name> {
        DirectHandle::<Name>::_PhantomData(PhantomData)
    }
    fn size_to_string(&self, size: usize) -> DirectHandle<Name> {
        DirectHandle::<Name>::_PhantomData(PhantomData)
    }
    fn heap_number_to_string(&self, number: DirectHandle<Object>, index: f64) -> DirectHandle<Name> {
        DirectHandle::<Name>::_PhantomData(PhantomData)
    }

    fn internalize_string(&self, name: DirectHandle<Name>) -> DirectHandle<Name> {
        DirectHandle::<Name>::_PhantomData(PhantomData)
    }

    fn new_heap_number(&self, value: f64) -> DirectHandle<Object> {
        DirectHandle::<Object>::_PhantomData(PhantomData)
    }

    fn number_to_string(&self, number: DirectHandle<Object>) -> DirectHandle<Object> {
        DirectHandle::<Object>::_PhantomData(PhantomData)
    }
}

pub trait HasFastProperties {
    fn has_fast_properties(&self, isolate: *mut Isolate) -> bool;
}

impl HasFastProperties for DirectHandle<JSReceiver> {
    fn has_fast_properties(&self, isolate: *mut Isolate) -> bool {
        false
    }
}

pub trait ToIntegerIndex {
    fn to_integer_index(object: DirectHandle<Object>, index: &mut usize) -> bool;
}

impl ToIntegerIndex for Object {
    fn to_integer_index(object: DirectHandle<Object>, index: &mut usize) -> bool {
        *index = 0;
        false
    }
}

pub trait IsName {
    fn is_name(object: DirectHandle<Object>) -> bool;
}

impl IsName for Object {
    fn is_name(object: DirectHandle<Object>) -> bool {
        false
    }
}

pub trait IsNumber {
    fn is_number(object: DirectHandle<Object>) -> bool;
}

impl IsNumber for Object {
    fn is_number(object: DirectHandle<Object>) -> bool {
        false
    }
}

