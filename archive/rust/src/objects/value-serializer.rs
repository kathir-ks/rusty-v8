// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add appropriate crate imports for base, common, handles, utils, and zone
// For now, using std

use std::any::Any;
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::mem::size_of;
use std::rc::Rc;

mod base {
    pub type Vector<T> = Vec<T>;
}

mod common {
    pub enum MessageTemplate {} // Placeholder
}

mod handles {
    pub struct MaybeHandle<T>(Option<T>);
    impl<T> MaybeHandle<T> {
        pub fn empty() -> Self {
            MaybeHandle(None)
        }
        pub fn from_value(value: T) -> Self {
            MaybeHandle(Some(value))
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_none()
        }
        pub fn to_option(&self) -> Option<&T> {
            self.0.as_ref()
        }
    }

    pub struct DirectHandle<T>(pub T);

    pub struct MaybeDirectHandle<T>(Option<T>);

    impl<T> MaybeDirectHandle<T> {
        pub fn empty() -> Self {
            MaybeDirectHandle(None)
        }

        pub fn from_value(value: T) -> Self {
            MaybeDirectHandle(Some(value))
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_none()
        }

        pub fn to_option(&self) -> Option<&T> {
            self.0.as_ref()
        }
    }

    impl<T> From<DirectHandle<T>> for MaybeDirectHandle<T> {
        fn from(handle: DirectHandle<T>) -> Self {
            MaybeDirectHandle::from_value(handle.0)
        }
    }

    pub struct IndirectHandle<T>(pub T);
    pub struct MaybeIndirectHandle<T>(Option<T>);
}

mod utils {
    use std::collections::HashMap;
    // ZoneAllocationPolicy is just a marker trait, no need to actually implement.
    pub trait ZoneAllocationPolicy {}

    #[derive(Default)]
    pub struct IdentityMap<V, A: ZoneAllocationPolicy> {
        map: HashMap<usize, V>,
        _phantom: std::marker::PhantomData<A>,
    }

    impl<V, A: ZoneAllocationPolicy> IdentityMap<V, A> {
        pub fn new() -> Self {
            IdentityMap {
                map: HashMap::new(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn insert(&mut self, key: usize, value: V) -> Option<V> {
            self.map.insert(key, value)
        }

        pub fn get(&self, key: usize) -> Option<&V> {
            self.map.get(&key)
        }

        pub fn remove(&mut self, key: usize) -> Option<V> {
            self.map.remove(&key)
        }

        pub fn contains_key(&self, key: usize) -> bool {
            self.map.contains_key(&key)
        }
    }
}

mod zone {
    // A basic zone allocator.  In a real implementation, it would likely
    // be more sophisticated.
    #[derive(Default)]
    pub struct Zone {
        allocations: Vec<Box<dyn Any>>,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {
                allocations: Vec::new(),
            }
        }

        pub fn allocate<T>(&mut self, value: T) -> &mut T {
            let boxed = Box::new(value);
            let ptr = Box::leak(boxed);
            self.allocations.push(Box::from_raw(ptr as *mut T as *mut dyn Any));
            ptr
        }
    }
}

pub mod v8 {
    pub struct ValueSerializer {
        // Opaque type representing v8::ValueSerializer
    }

    impl ValueSerializer {
        pub fn new(isolate: &Isolate, delegate: &mut dyn ValueSerializerDelegate) -> Self {
            ValueSerializer {} // Placeholder
        }

        pub fn write_header(&self) {}

        pub fn write_object(&self, object: &Object) -> Result<bool, DataCloneError> {
            Ok(true) // Placeholder
        }

        pub fn release(&self) -> (Vec<u8>, usize) {
            (Vec::new(), 0) // Placeholder
        }

        pub fn transfer_array_buffer(&self, transfer_id: u32, array_buffer: &JSArrayBuffer) {}

        pub fn write_uint32(&self, value: u32) {}
        pub fn write_uint64(&self, value: u64) {}
        pub fn write_raw_bytes(&self, source: &[u8]) {}
        pub fn write_double(&self, value: f64) {}
        pub fn write_byte(&self, value: u8) {}

        pub fn set_treat_array_buffer_views_as_host_objects(&self, mode: bool) {}
    }

    pub trait ValueSerializerDelegate {
        fn write_host_object(&mut self, object: &JSObject);
    }

    pub struct ValueDeserializer {
        // Opaque type representing v8::ValueDeserializer
    }

    impl ValueDeserializer {
        pub fn new(isolate: &Isolate, data: &[u8], delegate: &mut dyn ValueDeserializerDelegate) -> Self {
            ValueDeserializer {} // Placeholder
        }
        pub fn read_header(&self) -> Result<bool, DataCloneError> {
            Ok(true)
        }
        pub fn get_wire_format_version(&self) -> u32 {
            0
        }

        pub fn read_object_wrapper(&self) -> Result<Object, DataCloneError> {
            Ok(Object {})
        }

        pub fn read_object_using_entire_buffer_for_legacy_format(&self) -> Result<Object, DataCloneError> {
            Ok(Object {})
        }

        pub fn transfer_array_buffer(&self, transfer_id: u32, array_buffer: &JSArrayBuffer) {}

        pub fn read_uint32(&self, value: &mut u32) -> Result<bool, DataCloneError> {
            Ok(true)
        }

        pub fn read_uint64(&self, value: &mut u64) -> Result<bool, DataCloneError> {
            Ok(true)
        }

        pub fn read_double(&self, value: &mut f64) -> Result<bool, DataCloneError> {
            Ok(true)
        }

        pub fn read_raw_bytes(&self, length: usize, data: &mut &[u8]) -> Result<bool, DataCloneError> {
            Ok(true)
        }

        pub fn read_byte(&self, value: &mut u8) -> Result<bool, DataCloneError> {
            Ok(true)
        }
    }

    pub trait ValueDeserializerDelegate {
        fn read_host_object(&mut self) -> Object;
    }

    pub struct Isolate {} // Placeholder

    #[derive(Debug)]
    pub enum DataCloneError {
        GenericError,
        OutOfMemory,
    }

    impl std::fmt::Display for DataCloneError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for DataCloneError {}

    pub struct Object {} // Placeholder
}

pub mod internal {
    use super::*;
    use super::base::Vector;
    use super::handles::*;
    use super::utils::*;
    use super::zone::*;
    use std::io::Cursor;
    use std::mem::size_of;
    use std::ops::Deref;

    pub struct BigInt {} // Placeholder
    pub struct HeapNumber {} // Placeholder
    pub struct Isolate {} // Placeholder
    pub struct JSArrayBuffer {} // Placeholder
    pub struct JSArrayBufferView {} // Placeholder
    pub struct JSDate {} // Placeholder
    pub struct JSMap {} // Placeholder
    pub struct JSPrimitiveWrapper {} // Placeholder
    pub struct JSRegExp {} // Placeholder
    pub struct JSSet {} // Placeholder
    pub struct JSSharedArray {} // Placeholder
    pub struct JSSharedStruct {} // Placeholder
    pub struct Object {} // Placeholder
    pub struct Oddball {} // Placeholder
    pub struct SharedObjectConveyorHandles {} // Placeholder
    pub struct Smi {} // Placeholder
    pub struct WasmMemoryObject {} // Placeholder
    pub struct WasmModuleObject {} // Placeholder
    pub struct JSObject {}
    pub struct JSArray {}
    pub struct String {}
    pub struct FixedArray {}
    pub struct SimpleNumberDictionary {}

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(u8)]
    pub enum SerializationTag {
        String = 0, // Example
        BigInt = 1,
        // Add other tags as needed
    }

    /// Writes V8 objects in a binary format that allows the objects to be cloned
    /// according to the HTML structured clone algorithm.
    ///
    /// Format is based on Blink's previous serialization logic.
    pub struct ValueSerializer<'a> {
        isolate_: *mut Isolate, // Raw pointer, needs careful handling
        delegate_: *mut dyn v8::ValueSerializerDelegate, // Raw pointer, needs careful handling
        buffer_: Vec<u8>,
        buffer_size_: usize,
        buffer_capacity_: usize,
        has_custom_host_objects_: bool,
        treat_array_buffer_views_as_host_objects_: bool,
        out_of_memory_: bool,
        zone_: Zone,
        id_map_: IdentityMap<u32, ZoneAllocationPolicyImpl>,
        next_id_: u32,
        array_buffer_transfer_map_: IdentityMap<u32, ZoneAllocationPolicyImpl>,
        shared_object_conveyor_: *mut SharedObjectConveyorHandles, // Raw pointer, needs careful handling
        _phantom: std::marker::PhantomData<&'a ()>,
    }

    // Dummy implementation for ZoneAllocationPolicy
    #[derive(Default)]
    pub struct ZoneAllocationPolicyImpl {}
    impl ZoneAllocationPolicy for ZoneAllocationPolicyImpl {}

    impl<'a> ValueSerializer<'a> {
        pub fn new(
            isolate: *mut Isolate,
            delegate: *mut dyn v8::ValueSerializerDelegate,
        ) -> Self {
            ValueSerializer {
                isolate_: isolate,
                delegate_: delegate,
                buffer_: Vec::new(),
                buffer_size_: 0,
                buffer_capacity_: 0,
                has_custom_host_objects_: false,
                treat_array_buffer_views_as_host_objects_: false,
                out_of_memory_: false,
                zone_: Zone::new(),
                id_map_: IdentityMap::new(),
                next_id_: 0,
                array_buffer_transfer_map_: IdentityMap::new(),
                shared_object_conveyor_: std::ptr::null_mut(),
                _phantom: std::marker::PhantomData,
            }
        }

        /// Writes out a header, which includes the format version.
        pub fn write_header(&mut self) {
            // Placeholder implementation
            // In a real implementation, write the version and other header data to the buffer.
            self.write_uint32(1); // Example version number
        }

        /// Serializes a V8 object into the buffer.
        pub fn write_object(&mut self, object: DirectHandle<Object>) -> Result<bool, v8::DataCloneError> {
            // Placeholder implementation
            // In a real implementation, this would recursively serialize the object
            // based on its type and contents.
            println!("Writing object");
            Ok(true)
        }

        /// Returns the buffer, allocated via the delegate, and its size.
        /// Caller assumes ownership of the buffer.
        pub fn release(self) -> (Vec<u8>, usize) {
            // In a real implementation, you might need to use the delegate to allocate the buffer.
            (self.buffer_, self.buffer_size_)
        }

        /// Marks an ArrayBuffer as havings its contents transferred out of band.
        /// Pass the corresponding JSArrayBuffer in the deserializing context to
        /// ValueDeserializer::TransferArrayBuffer.
        pub fn transfer_array_buffer(
            &mut self,
            transfer_id: u32,
            array_buffer: DirectHandle<JSArrayBuffer>,
        ) {
            // Placeholder implementation
        }

        /// Publicly exposed wire format writing methods.
        /// These are intended for use within the delegate's WriteHostObject method.
        pub fn write_uint32(&mut self, value: u32) {
            self.buffer_.extend_from_slice(&value.to_le_bytes());
            self.buffer_size_ += size_of::<u32>();
        }
        pub fn write_uint64(&mut self, value: u64) {
            self.buffer_.extend_from_slice(&value.to_le_bytes());
            self.buffer_size_ += size_of::<u64>();
        }
        pub fn write_raw_bytes(&mut self, source: &[u8]) {
            self.buffer_.extend_from_slice(source);
            self.buffer_size_ += source.len();
        }
        pub fn write_double(&mut self, value: f64) {
            self.buffer_.extend_from_slice(&value.to_le_bytes());
            self.buffer_size_ += size_of::<f64>();
        }
        pub fn write_byte(&mut self, value: u8) {
            self.buffer_.push(value);
            self.buffer_size_ += 1;
        }

        /// Indicate whether to treat ArrayBufferView objects as host objects,
        /// i.e. pass them to Delegate::WriteHostObject. This should not be
        /// called when no Delegate was passed.
        ///
        /// The default is not to treat ArrayBufferViews as host objects.
        pub fn set_treat_array_buffer_views_as_host_objects(&mut self, mode: bool) {
            self.treat_array_buffer_views_as_host_objects_ = mode;
        }

        // Managing allocations of the internal buffer.
        fn expand_buffer(&mut self, required_capacity: usize) -> Result<bool, v8::DataCloneError> {
            if required_capacity > self.buffer_capacity_ {
                let new_capacity = required_capacity.max(self.buffer_capacity_ * 2 + 1);
                self.buffer_.reserve(new_capacity - self.buffer_capacity_);
                self.buffer_capacity_ = new_capacity;
            }
            Ok(true)
        }

        // Writing the wire format.
        fn write_tag(&mut self, tag: SerializationTag) {
            self.write_byte(tag as u8);
        }

        fn write_varint<T: Into<u64>>(&mut self, value: T) {
            let mut value = value.into();
            loop {
                let mut byte = (value & 0x7f) as u8;
                value >>= 7;
                if value != 0 {
                    byte |= 0x80;
                }
                self.write_byte(byte);
                if value == 0 {
                    break;
                }
            }
        }

        fn write_zig_zag<T: Into<i64>>(&mut self, value: T) {
            let value = value.into();
            self.write_varint(((value >> 63) ^ (value << 1)) as u64);
        }

        fn write_one_byte_string(&mut self, chars: Vector<u8>) {
            self.write_varint(chars.len() as u64);
            self.write_raw_bytes(&chars);
        }

        fn write_two_byte_string(&mut self, chars: Vector<u16>) {
            self.write_varint(chars.len() as u64);
            for char in chars {
                self.write_uint16(char);
            }
        }

        fn write_big_int_contents(&mut self, bigint: Tagged<BigInt>) {
            // Placeholder
        }

        fn reserve_raw_bytes(&mut self, bytes: usize) -> Result<&mut [u8], v8::DataCloneError> {
            self.expand_buffer(self.buffer_size_ + bytes)?;
            let start = self.buffer_size_;
            self.buffer_size_ += bytes;
            Ok(&mut self.buffer_[start..self.buffer_size_])
        }

        // Helper function to write u16 values
        fn write_uint16(&mut self, value: u16) {
            self.buffer_.extend_from_slice(&value.to_le_bytes());
            self.buffer_size_ += size_of::<u16>();
        }

        // Writing V8 objects of various kinds.
        fn write_oddball(&mut self, oddball: Tagged<Oddball>) {
            // Placeholder
        }

        fn write_smi(&mut self, smi: Tagged<Smi>) {
            // Placeholder
        }

        fn write_heap_number(&mut self, number: Tagged<HeapNumber>) {
            // Placeholder
        }

        fn write_big_int(&mut self, bigint: Tagged<BigInt>) {
            // Placeholder
        }

        fn write_string(&mut self, string: DirectHandle<String>) {
            // Placeholder
        }

        fn write_js_receiver(&mut self, receiver: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_object(&mut self, object: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_object_slow(&mut self, object: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_array(&mut self, array: DirectHandle<JSArray>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_date(&mut self, date: Tagged<JSDate>) {
            // Placeholder
        }

        fn write_js_primitive_wrapper(&mut self, value: DirectHandle<JSPrimitiveWrapper>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_regexp(&mut self, regexp: DirectHandle<JSRegExp>) {
            // Placeholder
        }

        fn write_js_map(&mut self, map: DirectHandle<JSMap>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_set(&mut self, map: DirectHandle<JSSet>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_array_buffer(&mut self, array_buffer: DirectHandle<JSArrayBuffer>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_array_buffer_view(&mut self, array_buffer: Tagged<JSArrayBufferView>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_error(&mut self, error: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_shared_array(&mut self, shared_array: DirectHandle<JSSharedArray>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_js_shared_struct(&mut self, shared_struct: DirectHandle<JSSharedStruct>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        fn write_wasm_module(&mut self, object: DirectHandle<WasmModuleObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        fn write_wasm_memory(&mut self, object: DirectHandle<WasmMemoryObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_shared_object(&mut self, object: DirectHandle<HeapObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        fn write_host_object(&mut self, object: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(true)
        }

        /// Reads the specified keys from the object and writes key-value pairs to the
        /// buffer. Returns the number of keys actually written, which may be smaller
        /// if some keys are not own properties when accessed.
        fn write_js_object_properties_slow(
            &mut self,
            object: DirectHandle<JSObject>,
            keys: DirectHandle<FixedArray>,
        ) -> Result<u32, v8::DataCloneError> {
            // Placeholder
            Ok(0)
        }

        fn is_host_object(&mut self, object: DirectHandle<JSObject>) -> Result<bool, v8::DataCloneError> {
            // Placeholder
            Ok(false)
        }

        /// Asks the delegate to handle an error that occurred during data cloning, by
        /// throwing an exception appropriate for the host.
        fn throw_data_clone_error(
            &mut self,
            template_index: common::MessageTemplate,
        ) -> Result<bool, v8::DataCloneError> {
            Err(v8::DataCloneError::GenericError) // Placeholder
        }

        fn throw_data_clone_error_with_arg(
            &mut self,
            template_index: common::MessageTemplate,
            arg0: DirectHandle<Object>,
        ) -> Result<bool, v8::DataCloneError> {
            Err(v8::DataCloneError::GenericError) // Placeholder
        }

        fn throw_if_out_of_memory(&mut self) -> Result<bool, v8::DataCloneError> {
            if self.out_of_memory_ {
                Err(v8::DataCloneError::OutOfMemory)
            } else {
                Ok(true)
            }
        }
    }

    impl<'a> Drop for ValueSerializer<'a> {
        fn drop(&mut self) {
            // Drop raw pointers if necessary. This is a placeholder,
            // and may require more sophisticated handling in a real
            // implementation.  For example, the delegate_ might
            // need to be informed of the end of serialization, or
            // some resources might need to be freed.
            //
            // For now, we'll just ensure that the raw pointers aren't
            // dereferenced in the drop implementation.
        }
    }

    #[derive(Debug)]
    pub struct Tagged<T>(T);
    pub struct HeapObject {}

    /// Deserializes values from data written with ValueSerializer, or a compatible
    /// implementation.
    pub struct ValueDeserializer<'a> {
        isolate_: *mut Isolate, // Raw pointer, needs careful handling
        delegate_: *mut dyn v8::ValueDeserializerDelegate, // Raw pointer, needs careful handling
        position_: Cursor<&'a [u8]>,
        end_: *const u8, // Raw pointer to the end of the data
        version_: u32,
        next_id_: u32,
        version_13_broken_data_mode_: bool,
        suppress_deserialization_errors_: bool,
        id_map_: handles::IndirectHandle<FixedArray>, // Always global handles.
        array_buffer_transfer_map_: handles::MaybeIndirectHandle<SimpleNumberDictionary>,
        shared_object_conveyor_: *const SharedObjectConveyorHandles, // Raw pointer, needs careful handling
    }

    impl<'a> ValueDeserializer<'a> {
        pub fn new(
            isolate: *mut Isolate,
            data: &'a [u8],
            delegate: *mut dyn v8::ValueDeserializerDelegate,
        ) -> Self {
            let end_ptr = data.as_ptr().wrapping_add(data.len());
            ValueDeserializer {
                isolate_: isolate,
                delegate_: delegate,
                position_: Cursor::new(data),
                end_: end_ptr,
                version_: 0,
                next_id_: 0,
                version_13_broken_data_mode_: false,
                suppress_deserialization_errors_: false,
                id_map_: handles::IndirectHandle(FixedArray {}), // Dummy
                array_buffer_transfer_map_: handles::MaybeIndirectHandle::empty(), // Dummy
                shared_object_conveyor_: std::ptr::null(), // Raw pointer, needs careful handling
            }
        }

        pub fn new_with_size(isolate: *mut Isolate, data: &'a [u8]) -> Self {
            let end_ptr = data.as_ptr().wrapping_add(data.len());
            ValueDeserializer {
                isolate_: isolate,
                delegate_: std::ptr::null_mut(),
                position_: Cursor::new(data),
                end_: end_ptr,
                version_: 0,
                next_id_: 0,
                version_13_broken_data_mode_: false,
                suppress_deserialization_errors_: false,
                id_map_: handles::IndirectHandle(FixedArray {}), // Dummy
                array_buffer_transfer_map_: handles::MaybeIndirectHandle::empty(), // Dummy
                shared_object_conveyor_: std::ptr::null(), // Raw pointer, needs careful handling
            }
        }

        /// Runs version detection logic, which may fail if the format is invalid.
        pub fn read_header(&mut self) -> Result<bool, v8::DataCloneError> {
            // Placeholder implementation.
            // In a real implementation, this would read the version and other
            // header data from the buffer.
            let mut version_buffer = [0u8; 4];
            self.read_raw_bytes_into_buffer(&mut version_buffer)?;
            self.version_ = u32::from_le_bytes(version_buffer);
            Ok(true)
        }

        /// Reads the underlying wire format version. Likely mostly to be useful to
        /// legacy code reading old wire format versions. Must be called after
        /// ReadHeader.
        pub fn get_wire_format_version(&self) -> u32 {
            self.version_
        }

        /// Deserializes a V8 object from the buffer.
        pub fn read_object_wrapper(&mut self) -> Result<handles::DirectHandle<Object>, v8::DataCloneError> {
            // Placeholder implementation
            println!("Reading object wrapper");
            Ok(handles::DirectHandle(Object {}))
        }

        /// Reads an object, consuming the entire buffer.
        ///
        /// This is required for the legacy "version 0" format, which did not allow
        /// reference deduplication, and instead relied on a "stack" model for
        /// deserializing, with the contents of objects and arrays provided first.
        pub fn read_object_using_entire_buffer_for_legacy_format(&mut self) -> Result<handles::DirectHandle<Object>, v8::DataCloneError> {
            // Placeholder implementation
            Ok(handles::DirectHandle(Object {}))
        }

        /// Accepts the array buffer corresponding to the one passed previously to
        /// ValueSerializer::TransferArrayBuffer.
        pub fn transfer_array_buffer(
            &mut self,
            transfer_id: u32,
            array_buffer: handles::DirectHandle<JSArrayBuffer>,
        ) {
            // Placeholder implementation
        }

        /// Publicly exposed wire format writing methods.
        /// These are intended for use within the delegate's WriteHostObject method.
        pub fn read_uint32(&mut self, value: &mut u32) -> Result<bool, v8::DataCloneError> {
            let mut buffer = [0u8; 4];
            self.read_raw_bytes_into_buffer(&mut buffer)?;
            *value = u32::from_le_bytes(buffer);
            Ok(true)
        }

        pub fn read_uint64(&mut self, value: &mut u64) -> Result<bool, v8::DataCloneError> {
            let mut buffer = [0u8; 8];
            self.read_raw_bytes_into_buffer(&mut buffer)?;
            *value = u64::from_le_bytes(buffer);
            Ok(true)
        }

        pub fn read_double(&mut self, value: &mut f64) -> Result<bool, v8::DataCloneError> {
            let mut buffer = [0u8; 8];
            self.read_raw_bytes_into_buffer(&mut buffer)?;
            *value = f64::from_le_bytes(buffer);
            Ok(true)
        }

        pub fn read_raw_bytes(&mut self, length: usize, data: &mut &[u8]) -> Result<bool, v8::DataCloneError> {
            let current_position = self.position_.position() as usize;
            let available_bytes = (self.position_.get_ref().len()).saturating_sub(current_position);

            if length > available_bytes {
                return Err(v8::DataCloneError::GenericError);
            }

            let start = current_position;
            let end = current_position + length;
            *data = &self.position_.get_ref()[start..end];
            self.position_.set_position(end as u64);

            Ok(true)
        }

        pub fn read_byte(&mut self, value: &mut u8) -> Result<bool, v8::DataCloneError> {
            let mut buffer = [0u8; 1];
            self.read_raw_bytes_into_buffer(&mut buffer)?;
            *value = buffer[0];
            Ok(true)
        }

        fn read_raw_bytes_into_buffer(&mut self, buffer: &mut [u8]) -> Result<(), v8::DataCloneError> {
            if self.position_.read_exact(buffer).is_err() {
                return Err(v8::DataCloneError::GenericError);
            }
            Ok(())
        }

        // Reading the wire format.
        fn peek_tag(&self) -> Result<SerializationTag, v8::DataCloneError> {
            let current_position = self.position_.position() as usize;
            let data = self.position_.get_ref();

            if current_position >= data.len() {
                return Err(v8::DataCloneError::GenericError); // Or a more specific error
            }
            let byte = data[current_position];

            match byte {
                0 => Ok(SerializationTag::String), // Example
                1 => Ok(SerializationTag::BigInt),
                _ => Err(v8::DataCloneError::GenericError),
            }
        }

        fn consume_tag(&mut self, peeked_tag: SerializationTag) {
            self.position_.set_position(self.position_.position() + 1);
        }

        fn read_tag(&mut self) -> Result<SerializationTag, v8::DataCloneError> {
            let tag = self.peek_tag()?;
            self.consume_tag(tag);
            Ok(tag)
        }

        fn read_varint<T: From<u64>>(&mut self) -> Result<T, v8::DataCloneError> {
            self.read_varint_loop()
        }

        fn read_varint_loop<T: From<u64>>(&mut self) -> Result<T, v8::DataCloneError> {
            let mut result: u64 = 0;
            let mut shift: u32 = 0;

            loop {
                let mut byte = 0u8;
                if !self.read_byte(&mut byte)? {
                    return Err(v8::DataCloneError::GenericError);
                }

                result |= ((byte & 0x7f) as u64) << shift;
                shift += 7;

                if byte & 0x80 == 0 {
                    break;
                }

                if shift > 63 {
                    return Err(v8::DataCloneError::GenericError); // Overflow
                }
            }

            Ok(result.into())
        }

        fn read_zig_zag<T: From<i64>>(&mut self) -> Result<T, v8::DataCloneError> {
            let value = self.read_varint::<u64>()?;
            let value = value as i64;
            let result = (value >> 1) ^ -(value & 1);
            Ok(result.into())
        }

        fn read_double(&mut self) -> Result<f64, v8::DataCloneError> {
            let mut buffer = [0u8; 8];
            self.read_raw_bytes_into_buffer(&mut buffer)?;
            Ok(f64::from_le_bytes(buffer))
        }

        fn read_raw_bytes(&mut self, size: usize) -> Result<base::Vector<u8>, v8::DataCloneError> {
            let mut buffer = vec![0u8; size];
            if self.position_.read_exact(&mut buffer).is_err() {
                return Err(v8::DataCloneError::GenericError);
            }
            Ok(buffer)
        }

        fn read_raw_two_bytes(&mut