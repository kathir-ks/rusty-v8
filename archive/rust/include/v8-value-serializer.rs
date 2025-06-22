// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8 {
    pub struct SharedValueConveyor {
        private_: Box<internal::SharedObjectConveyorHandles>,
    }

    impl SharedValueConveyor {
        // Constructor is private, only internal::ValueSerializer and internal::ValueDeserializer can create it.
        fn new(isolate: &Isolate) -> Self {
            SharedValueConveyor {
                private_: Box::new(internal::SharedObjectConveyorHandles::new(isolate)),
            }
        }

        pub fn take(&mut self) -> SharedValueConveyor {
            SharedValueConveyor {
                private_: std::mem::replace(&mut self.private_, Box::new(internal::SharedObjectConveyorHandles::new(&Isolate{}))),
            }
        }
    }

    impl Drop for SharedValueConveyor {
        fn drop(&mut self) {
            // Drop implementation. The underlying data structure in the private_ field is deallocated here.
        }
    }

    pub struct ValueSerializer {
        private_: Box<PrivateData>,
    }

    impl ValueSerializer {
        pub fn new(isolate: &Isolate) -> Self {
            ValueSerializer {
                private_: Box::new(PrivateData::new(isolate, None)),
            }
        }
        pub fn with_delegate(isolate: &Isolate, delegate: Box<dyn Delegate>) -> Self {
             ValueSerializer {
                private_: Box::new(PrivateData::new(isolate, Some(delegate))),
            }
        }

        pub fn write_header(&mut self) {
            self.private_.write_header();
        }

        pub fn write_value(&mut self, context: &Context, value: &Value) -> Result<bool, ()> {
            self.private_.write_value(context, value)
        }

        pub fn release(&mut self) -> (*mut u8, usize) {
            self.private_.release()
        }

        pub fn transfer_array_buffer(&mut self, transfer_id: u32, array_buffer: &ArrayBuffer) {
            self.private_.transfer_array_buffer(transfer_id, array_buffer);
        }

        pub fn set_treat_array_buffer_views_as_host_objects(&mut self, mode: bool) {
            self.private_.set_treat_array_buffer_views_as_host_objects(mode);
        }

        pub fn write_uint32(&mut self, value: u32) {
            self.private_.write_uint32(value);
        }

        pub fn write_uint64(&mut self, value: u64) {
            self.private_.write_uint64(value);
        }

        pub fn write_double(&mut self, value: f64) {
            self.private_.write_double(value);
        }

        pub fn write_raw_bytes(&mut self, source: *const std::ffi::c_void, length: usize) {
            self.private_.write_raw_bytes(source, length);
        }
    }

    impl Drop for ValueSerializer {
        fn drop(&mut self) {
             // Drop implementation. The underlying data structure in the private_ field is deallocated here.
        }
    }

    pub trait Delegate {
        fn throw_data_clone_error(&mut self, message: &String);
        fn has_custom_host_object(&mut self, isolate: &Isolate) -> bool;
        fn is_host_object(&mut self, isolate: &Isolate, object: &Object) -> Result<bool, ()>;
        fn write_host_object(&mut self, isolate: &Isolate, object: &Object) -> Result<bool, ()>;
        fn get_shared_array_buffer_id(&mut self, isolate: &Isolate, shared_array_buffer: &SharedArrayBuffer) -> Result<u32, ()>;
        fn get_wasm_module_transfer_id(&mut self, isolate: &Isolate, module: &WasmModuleObject) -> Result<u32, ()>;
        fn adopt_shared_value_conveyor(&mut self, isolate: &Isolate, conveyor: SharedValueConveyor) -> bool;
        fn reallocate_buffer_memory(&mut self, old_buffer: *mut std::ffi::c_void, size: usize, actual_size: &mut usize) -> *mut std::ffi::c_void;
        fn free_buffer_memory(&mut self, buffer: *mut std::ffi::c_void);
    }

    pub struct ValueDeserializer {
        private_: Box<PrivateDataDeserializer>,
    }

    impl ValueDeserializer {
        pub fn new(isolate: &Isolate, data: *const u8, size: usize) -> Self {
            ValueDeserializer {
                private_: Box::new(PrivateDataDeserializer::new(isolate, data, size, None)),
            }
        }

        pub fn with_delegate(isolate: &Isolate, data: *const u8, size: usize, delegate: Box<dyn DelegateDeserializer>) -> Self {
            ValueDeserializer {
                private_: Box::new(PrivateDataDeserializer::new(isolate, data, size, Some(delegate))),
            }
        }

        pub fn read_header(&mut self, context: &Context) -> Result<bool, ()> {
            self.private_.read_header(context)
        }

        pub fn read_value(&mut self, context: &Context) -> Result<Value, ()> {
            self.private_.read_value(context)
        }

        pub fn transfer_array_buffer(&mut self, transfer_id: u32, array_buffer: &ArrayBuffer) {
            self.private_.transfer_array_buffer(transfer_id, array_buffer);
        }

        pub fn transfer_shared_array_buffer(&mut self, id: u32, shared_array_buffer: &SharedArrayBuffer) {
            self.private_.transfer_shared_array_buffer(id, shared_array_buffer);
        }

        pub fn set_supports_legacy_wire_format(&mut self, supports_legacy_wire_format: bool) {
            self.private_.set_supports_legacy_wire_format(supports_legacy_wire_format);
        }

        pub fn get_wire_format_version(&self) -> u32 {
            self.private_.get_wire_format_version()
        }

        pub fn read_uint32(&mut self, value: &mut u32) -> Result<bool, ()> {
            self.private_.read_uint32(value)
        }

        pub fn read_uint64(&mut self, value: &mut u64) -> Result<bool, ()> {
            self.private_.read_uint64(value)
        }

        pub fn read_double(&mut self, value: &mut f64) -> Result<bool, ()> {
            self.private_.read_double(value)
        }

        pub fn read_raw_bytes(&mut self, length: usize, data: &mut *const std::ffi::c_void) -> Result<bool, ()> {
            self.private_.read_raw_bytes(length, data)
        }
    }

    impl Drop for ValueDeserializer {
        fn drop(&mut self) {
            // Drop implementation. The underlying data structure in the private_ field is deallocated here.
        }
    }

    pub trait DelegateDeserializer {
        fn read_host_object(&mut self, isolate: &Isolate) -> Result<Object, ()>;
        fn get_wasm_module_from_id(&mut self, isolate: &Isolate, transfer_id: u32) -> Result<WasmModuleObject, ()>;
        fn get_shared_array_buffer_from_id(&mut self, isolate: &Isolate, clone_id: u32) -> Result<SharedArrayBuffer, ()>;
        fn get_shared_value_conveyor(&mut self, isolate: &Isolate) -> Option<&SharedValueConveyor>;
    }

    pub struct Isolate {}
    pub struct Context {}
    pub struct Value {}
    pub struct ArrayBuffer {}
    pub struct SharedArrayBuffer {}
    pub struct Object {}
    pub struct String {}
    pub struct WasmModuleObject {}

    struct PrivateData {
        isolate: *mut Isolate,
        delegate: Option<Box<dyn Delegate>>,
    }

    impl PrivateData {
        fn new(isolate: &Isolate, delegate: Option<Box<dyn Delegate>>) -> Self {
            PrivateData {
                isolate: isolate as *const Isolate as *mut Isolate,
                delegate,
            }
        }
        fn write_header(&mut self) {}
        fn write_value(&mut self, _context: &Context, _value: &Value) -> Result<bool, ()> {
            Ok(true)
        }
        fn release(&mut self) -> (*mut u8, usize) {
            (std::ptr::null_mut(), 0)
        }
        fn transfer_array_buffer(&mut self, _transfer_id: u32, _array_buffer: &ArrayBuffer) {}
        fn set_treat_array_buffer_views_as_host_objects(&mut self, _mode: bool) {}
        fn write_uint32(&mut self, _value: u32) {}
        fn write_uint64(&mut self, _value: u64) {}
        fn write_double(&mut self, _value: f64) {}
        fn write_raw_bytes(&mut self, _source: *const std::ffi::c_void, _length: usize) {}
    }

    struct PrivateDataDeserializer {
        isolate: *mut Isolate,
        data: *const u8,
        size: usize,
        delegate: Option<Box<dyn DelegateDeserializer>>,
    }

    impl PrivateDataDeserializer {
        fn new(isolate: &Isolate, data: *const u8, size: usize, delegate: Option<Box<dyn DelegateDeserializer>>) -> Self {
            PrivateDataDeserializer {
                isolate: isolate as *const Isolate as *mut Isolate,
                data,
                size,
                delegate,
            }
        }

        fn read_header(&mut self, _context: &Context) -> Result<bool, ()> {
            Ok(true)
        }

        fn read_value(&mut self, _context: &Context) -> Result<Value, ()> {
            Err(())
        }

        fn transfer_array_buffer(&mut self, _transfer_id: u32, _array_buffer: &ArrayBuffer) {}

        fn transfer_shared_array_buffer(&mut self, _id: u32, _shared_array_buffer: &SharedArrayBuffer) {}

        fn set_supports_legacy_wire_format(&mut self, _supports_legacy_wire_format: bool) {}

        fn get_wire_format_version(&self) -> u32 {
            0
        }

        fn read_uint32(&mut self, _value: &mut u32) -> Result<bool, ()> {
            Ok(true)
        }

        fn read_uint64(&mut self, _value: &mut u64) -> Result<bool, ()> {
            Ok(true)
        }

        fn read_double(&mut self, _value: &mut f64) -> Result<bool, ()> {
            Ok(true)
        }

        fn read_raw_bytes(&mut self, _length: usize, _data: &mut *const std::ffi::c_void) -> Result<bool, ()> {
            Ok(true)
        }
    }

    mod internal {
        use super::{Isolate, SharedValueConveyor};
        pub struct ScriptStreamingData {}
        pub struct SharedObjectConveyorHandles {
            isolate: *mut Isolate,
        }

        impl SharedObjectConveyorHandles {
            pub fn new(isolate: &Isolate) -> Self {
                SharedObjectConveyorHandles {
                    isolate: isolate as *const Isolate as *mut Isolate,
                }
            }
        }

        pub struct ValueSerializer {}
        pub struct ValueDeserializer {}
    }
}