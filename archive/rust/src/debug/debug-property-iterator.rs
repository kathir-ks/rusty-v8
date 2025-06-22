// src/debug/debug_property_iterator.rs

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use crate::api;
use crate::base::flags::Flags;
use crate::objects::js_array_buffer::JsArrayBuffer;
use crate::objects::keys::{KeyAccumulator, KeyCollectionMode, GetKeysConversion};
use crate::objects::property_descriptor::PropertyDescriptor;
use crate::objects::property_details::PropertyDetails;
use crate::objects::*; // Import object types
use crate::isolate::Isolate;
use crate::utils::Utils;
use crate::v8::{self, Local};
use crate::property::PropertyKey;
use crate::lookup::LookupIterator;
use crate::accessor::AccessorInfo;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum NativeAccessorType {
    None = 0,
    HasGetter = 1 << 0,
    HasSetter = 1 << 1,
    IsValueUnavailable = 1 << 2,
}

impl NativeAccessorType {
    pub fn from_bits(bits: i32) -> Self {
        match bits {
            0 => NativeAccessorType::None,
            1 => NativeAccessorType::HasGetter,
            2 => NativeAccessorType::HasSetter,
            4 => NativeAccessorType::IsValueUnavailable,
            _ => panic!("Invalid NativeAccessorType bits"),
        }
    }

    pub fn to_bits(&self) -> i32 {
        match self {
            NativeAccessorType::None => 0,
            NativeAccessorType::HasGetter => 1,
            NativeAccessorType::HasSetter => 2,
            NativeAccessorType::IsValueUnavailable => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Stage {
    ExoticIndices,
    EnumerableStrings,
    AllProperties,
}

#[derive(Debug)]
pub struct DebugPropertyIterator {
    isolate: *mut Isolate, // Raw pointer to Isolate
    prototype_iterator: PrototypeIterator,
    skip_indices: bool,
    current_key_index: usize,
    current_keys: Handle<FixedArray>,
    current_keys_length: usize,
    stage_: Stage,
    is_done_: bool,
    is_own_: bool,
    calculated_native_accessor_flags_: bool,
    native_accessor_flags_: i32, // Using i32 for Flags<NativeAccessorType, int>
}

impl DebugPropertyIterator {
    /// Creates a new DebugPropertyIterator.
    pub fn create(
        isolate: *mut Isolate, // Raw pointer to Isolate
        receiver: Handle<JSReceiver>,
        skip_indices: bool,
    ) -> Option<Box<DebugPropertyIterator>> {
        let mut iterator = Box::new(DebugPropertyIterator {
            isolate,
            prototype_iterator: PrototypeIterator::new(isolate, receiver, PrototypeIteratorFlags::kStartAtReceiver, PrototypeIteratorFlags::END_AT_NULL),
            skip_indices,
            current_key_index: 0,
            current_keys: unsafe { (*isolate).factory().empty_fixed_array() },
            current_keys_length: 0,
            stage_: Stage::ExoticIndices,
            is_done_: false,
            is_own_: true,
            calculated_native_accessor_flags_: false,
            native_accessor_flags_: 0,
        });

        if receiver.is_js_proxy() {
            iterator.advance_to_prototype();
        }

        if !iterator.fill_keys_for_current_prototype_and_stage() {
            return None;
        }
        if iterator.should_move_to_next_stage() && !iterator.advance_internal() {
            return None;
        }

        Some(iterator)
    }

    /// Returns whether the iterator is done.
    pub fn done(&self) -> bool {
        self.is_done_
    }

    fn advance_to_prototype(&mut self) {
        self.stage_ = Stage::ExoticIndices;
        self.is_own_ = false;
        if !self.prototype_iterator.has_access() {
            self.is_done_ = true;
        }
        self.prototype_iterator.advance_ignoring_proxies();
        if self.prototype_iterator.is_at_end() {
            self.is_done_ = true;
        }
    }

    fn advance_internal(&mut self) -> bool {
        self.current_key_index += 1;
        self.calculated_native_accessor_flags_ = false;
        while self.should_move_to_next_stage() {
            match self.stage_ {
                Stage::ExoticIndices => {
                    self.stage_ = Stage::EnumerableStrings;
                }
                Stage::EnumerableStrings => {
                    self.stage_ = Stage::AllProperties;
                }
                Stage::AllProperties => {
                    self.advance_to_prototype();
                }
            }
            if !self.fill_keys_for_current_prototype_and_stage() {
                return false;
            }
        }
        true
    }

    fn is_native_accessor(&mut self) -> bool {
        self.calculate_native_accessor_flags();
        self.native_accessor_flags_ != 0
    }

    fn has_native_getter(&mut self) -> bool {
        self.calculate_native_accessor_flags();
        (self.native_accessor_flags_ & NativeAccessorType::HasGetter.to_bits()) != 0
    }

    fn has_native_setter(&mut self) -> bool {
        self.calculate_native_accessor_flags();
        (self.native_accessor_flags_ & NativeAccessorType::HasSetter.to_bits()) != 0
    }

    fn raw_name(&self) -> Handle<Name> {
        assert!(!self.done());
        if self.stage_ == Stage::ExoticIndices {
            unsafe {
                (*self.isolate).factory().size_to_string(self.current_key_index)
            }
        } else {
            unsafe {
                let obj = self.current_keys.get(self.current_key_index) as *mut Object;
                Handle::cast(obj)
            }
        }
    }

    pub fn name(&self) -> Local<v8::Name> {
        Utils::to_local(self.raw_name())
    }

    pub fn attributes(&self) -> Result<v8::PropertyAttribute, ()> {
        let receiver = self.prototype_iterator.current::<JSReceiver>();
        let result = unsafe {
            JSReceiver::get_property_attributes(self.isolate, receiver, self.raw_name())
        };

        match result {
            Ok(attr) => {
                // #if DEBUG
                //  base::ScopedVector<char> property_message(128);
                //  base::ScopedVector<char> name_buffer(100);
                //  raw_name()->NameShortPrint(name_buffer);
                //  v8::base::SNPrintF(property_message, "Invalid result for property \"%s\"\n",
                //                     name_buffer.begin());
                //  DCHECK_WITH_MSG(result.FromJust() != ABSENT, property_message.begin());
                // #endif
                Ok(attr)
            },
            Err(_) => Err(())
        }
    }

    pub fn descriptor(&self) -> Result<v8::debug::PropertyDescriptor, ()> {
        let receiver = self.prototype_iterator.current::<JSReceiver>();

        let mut descriptor = PropertyDescriptor::default();

        let did_get_descriptor = unsafe {
            JSReceiver::get_own_property_descriptor(self.isolate, receiver, self.raw_name(), &mut descriptor)
        };

        match did_get_descriptor {
            Ok(true) => {
                Ok(v8::debug::PropertyDescriptor {
                    enumerable: descriptor.enumerable(),
                    has_enumerable: descriptor.has_enumerable(),
                    configurable: descriptor.configurable(),
                    has_configurable: descriptor.has_configurable(),
                    writable: descriptor.writable(),
                    has_writable: descriptor.has_writable(),
                    value: if descriptor.has_value() { Utils::to_local(descriptor.value()) } else { v8::Local::empty() },
                    get: if descriptor.has_get() { Utils::to_local(descriptor.get()) } else { v8::Local::empty() },
                    set: if descriptor.has_set() { Utils::to_local(descriptor.set()) } else { v8::Local::empty() },
                })
            }
            Ok(false) => {
                Ok(v8::debug::PropertyDescriptor {
                    enumerable: false,
                    has_enumerable: false,
                    configurable: false,
                    has_configurable: false,
                    writable: false,
                    has_writable: false,
                    value: v8::Local::empty(),
                    get: v8::Local::empty(),
                    set: v8::Local::empty(),
                })
            }
            Err(_) => Err(()),
        }
    }

    pub fn is_own(&self) -> bool {
        self.is_own_
    }

    pub fn is_array_index(&self) -> bool {
        if self.stage_ == Stage::ExoticIndices {
            return true;
        }
        let key = PropertyKey::new(self.isolate, self.raw_name());
        key.is_element()
    }

    fn fill_keys_for_current_prototype_and_stage(&mut self) -> bool {
        self.current_key_index = 0;
        self.current_keys = unsafe { (*self.isolate).factory().empty_fixed_array() };
        self.current_keys_length = 0;
        if self.is_done_ {
            return true;
        }

        let receiver = self.prototype_iterator.current::<JSReceiver>();

        if self.stage_ == Stage::ExoticIndices {
            if self.skip_indices || !receiver.is_js_typed_array() {
                return true;
            }
            let typed_array = unsafe { Handle::<JSTypedArray>::cast(receiver.into()) };
            self.current_keys_length = if unsafe { typed_array.is_detached() } {
                0
            } else {
                unsafe { typed_array.length() }
            };
            return true;
        }

        let filter = if self.stage_ == Stage::EnumerableStrings {
            PropertyFilter::ENUMERABLE_STRINGS
        } else {
            PropertyFilter::ALL_PROPERTIES
        };

        let conversion = GetKeysConversion::kConvertToString;

        let skip = self.skip_indices || receiver.is_js_typed_array();

        match unsafe { KeyAccumulator::get_keys(self.isolate, receiver, KeyCollectionMode::kOwnOnly, filter, conversion, false, skip) } {
            Ok(keys) => {
                self.current_keys = keys;
                self.current_keys_length = unsafe { self.current_keys.length() };
                true
            }
            Err(_) => false,
        }
    }

    fn should_move_to_next_stage(&self) -> bool {
        !self.is_done_ && self.current_key_index >= self.current_keys_length
    }

    fn calculate_native_accessor_flags(&mut self) {
        if self.calculated_native_accessor_flags_ {
            return;
        }
        if self.stage_ == Stage::ExoticIndices {
            self.native_accessor_flags_ = 0;
        } else {
            let receiver = self.prototype_iterator.current::<JSReceiver>();
            self.native_accessor_flags_ = get_native_accessor_descriptor_internal(receiver, self.raw_name()).to_bits();
        }
        self.calculated_native_accessor_flags_ = true;
    }
}

fn get_native_accessor_descriptor_internal(
    object: Handle<JSReceiver>,
    name: Handle<Name>,
) -> NativeAccessorType {
    let isolate = unsafe { (*object.get_isolate()).deref() };
    let key = PropertyKey::new(object.get_isolate(), name);
    if key.is_element() {
        return NativeAccessorType::None;
    }
    let mut it = LookupIterator::new(object.get_isolate(), object, key, LookupIteratorFlags::OWN);
    if !it.is_found() {
        return NativeAccessorType::None;
    }
    if it.state() != LookupIteratorState::ACCESSOR {
        return NativeAccessorType::None;
    }

    let structure = it.get_accessors();

    if !structure.is_accessor_info() {
        return NativeAccessorType::None;
    }
    if structure == unsafe { isolate.factory().value_unavailable_accessor() } {
        return NativeAccessorType::IsValueUnavailable;
    }

    // Macro expansion
    // #define IS_BUILTIN_ACCESSOR(_, name, ...)                   \
    // if (*structure == *isolate->factory()->name##_accessor()) \
    //   return debug::NativeAccessorType::None;
    // ACCESSOR_INFO_LIST_GENERATOR(IS_BUILTIN_ACCESSOR, /* not used */)
    // #undef IS_BUILTIN_ACCESSOR

    let accessor_info = unsafe { Handle::<AccessorInfo>::cast(structure.into()) };
    let mut result = NativeAccessorType::None;

    if unsafe { accessor_info.has_getter(isolate.get_raw()) } {
        result = unsafe { std::mem::transmute(result as i32 | NativeAccessorType::HasGetter as i32) };
    }
    if unsafe { accessor_info.has_setter(isolate.get_raw()) } {
        result = unsafe { std::mem::transmute(result as i32 | NativeAccessorType::HasSetter as i32) };
    }

    result
}
