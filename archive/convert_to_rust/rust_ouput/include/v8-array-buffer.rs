// Converted from V8 C++ source files:
// Header: v8-array-buffer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    pub struct BackingStoreBase {}
    pub type Address = usize;
}
use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr::null_mut;
use std::sync::Arc;

pub struct V8_EXPORT {}
pub struct Isolate {}
pub struct Object {}
pub struct SharedArrayBuffer {}
pub struct TypedArray {}

#[derive(Debug, PartialEq, Eq)]
pub enum BackingStoreError {
    AllocationFailed,
    InvalidLength,
}

#[allow(non_snake_case)]
pub mod v8 {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::{ptr::NonNull, sync::Mutex};

    #[repr(C)]
    pub struct BackingStore {
        base: internal::BackingStoreBase,
        data: *mut std::ffi::c_void,
        byte_length: usize,
        max_byte_length: usize,
        is_shared: bool,
        is_resizable_by_user_javascript: bool,
        deleter: Option<DeleterCallback>,
        deleter_data: *mut std::ffi::c_void,
        is_detached: AtomicBool,
    }

    impl BackingStore {
        fn new(
            data: *mut std::ffi::c_void,
            byte_length: usize,
            max_byte_length: usize,
            is_shared: bool,
            is_resizable_by_user_javascript: bool,
            deleter: Option<DeleterCallback>,
            deleter_data: *mut std::ffi::c_void,
        ) -> Self {
            Self {
                base: internal::BackingStoreBase {},
                data,
                byte_length,
                max_byte_length,
                is_shared,
                is_resizable_by_user_javascript,
                deleter,
                deleter_data,
                is_detached: AtomicBool::new(false),
            }
        }

        pub fn detach(&self) {
            self.is_detached.store(true, Ordering::SeqCst);
        }

        pub fn is_detached(&self) -> bool {
            self.is_detached.load(Ordering::SeqCst)
        }
    }

    pub type Local<'a, T> = &'a T;

    impl BackingStore {
        pub fn Data(&self) -> *mut std::ffi::c_void {
            self.data
        }

        pub fn ByteLength(&self) -> usize {
            self.byte_length
        }

        pub fn MaxByteLength(&self) -> usize {
            self.max_byte_length
        }

        pub fn IsShared(&self) -> bool {
            self.is_shared
        }

        pub fn IsResizableByUserJavaScript(&self) -> bool {
            self.is_resizable_by_user_javascript
        }

        pub fn EmptyDeleter(data: *mut std::ffi::c_void, length: usize, deleter_data: *mut std::ffi::c_void) {}
    }

    impl Drop for BackingStore {
        fn drop(&mut self) {
            if let Some(deleter) = self.deleter {
                deleter(self.data, self.byte_length, self.deleter_data);
            }
        }
    }

    pub type BackingStoreDeleterCallback =
        unsafe extern "C" fn(data: *mut std::ffi::c_void, length: usize, deleter_data: *mut std::ffi::c_void);
    pub type DeleterCallback = BackingStoreDeleterCallback;

    #[derive(Debug, Clone, Copy)]
    pub enum ArrayBufferCreationMode {
        kInternalized,
        kExternalized,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum BackingStoreInitializationMode {
        kZeroInitialized,
        kUninitialized,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum BackingStoreOnFailureMode {
        kReturnNull,
        kOutOfMemory,
    }

    pub struct ArrayBuffer {
        backing_store: Option<std::shared_ptr::SharedPtr<BackingStore>>,
        byte_length: usize,
        max_byte_length: usize,
    }

    impl ArrayBuffer {
        pub fn ByteLength(&self) -> usize {
            self.byte_length
        }

        pub fn MaxByteLength(&self) -> usize {
            self.max_byte_length
        }
        pub fn New(isolate: *mut Isolate, byte_length: usize, initialization_mode: BackingStoreInitializationMode) -> Local<'static, ArrayBuffer> {
        let backing_store = ArrayBuffer::NewBackingStore(
            unsafe { &mut *isolate },
            byte_length,
            initialization_mode,
            BackingStoreOnFailureMode::kOutOfMemory,
        )
        .expect("Failed to allocate backing store");
        let shared_backing_store = std::shared_ptr::SharedPtr::new(backing_store);
        let array_buffer = ArrayBuffer {
            backing_store: Some(shared_backing_store),
            byte_length,
            max_byte_length: byte_length,
        };
        Box::leak(Box::new(array_buffer))
    }
        pub fn NewBackingStore(
            isolate: &mut Isolate,
            byte_length: usize,
            initialization_mode: BackingStoreInitializationMode,
            on_failure: BackingStoreOnFailureMode,
        ) -> Result<Box<BackingStore>, BackingStoreError> {
            if byte_length > ArrayBuffer::kMaxByteLength {
                return Err(BackingStoreError::InvalidLength);
            }

            let layout = Layout::from_size_align(byte_length, 1).map_err(|_| BackingStoreError::AllocationFailed)?;

            let data = unsafe {
                match initialization_mode {
                    BackingStoreInitializationMode::kZeroInitialized => {
                        let ptr = System.allocate_zeroed(layout);
                        if ptr.is_null() {
                            match on_failure {
                                BackingStoreOnFailureMode::kReturnNull => return Err(BackingStoreError::AllocationFailed),
                                BackingStoreOnFailureMode::kOutOfMemory => panic!("Out of memory"),
                            }
                        }
                        ptr
                    }
                    BackingStoreInitializationMode::kUninitialized => {
                        let ptr = System.allocate(layout);
                        if ptr.is_null() {
                            match on_failure {
                                BackingStoreOnFailureMode::kReturnNull => return Err(BackingStoreError::AllocationFailed),
                                BackingStoreOnFailureMode::kOutOfMemory => panic!("Out of memory"),
                            }
                        }
                        ptr
                    }
                }
            };

            let deleter: DeleterCallback = ArrayBuffer::free_backing_store;
            let deleter_data: *mut std::ffi::c_void = null_mut();

            Ok(Box::new(BackingStore::new(
                data,
                byte_length,
                byte_length,
                false,
                false,
                Some(deleter),
                deleter_data,
            )))
        }

        pub fn NewBackingStore_data(
            data: *mut std::ffi::c_void,
            byte_length: usize,
            deleter: v8::DeleterCallback,
            deleter_data: *mut std::ffi::c_void,
        ) -> Result<Box<BackingStore>, BackingStoreError> {
            if data.is_null() {
                return Err(BackingStoreError::AllocationFailed);
            }

            if byte_length > ArrayBuffer::kMaxByteLength {
                return Err(BackingStoreError::InvalidLength);
            }

            Ok(Box::new(BackingStore::new(
                data,
                byte_length,
                byte_length,
                false,
                false,
                Some(deleter),
                deleter_data,
            )))
        }
        pub fn New(isolate: *mut Isolate, backing_store: std::shared_ptr::SharedPtr<BackingStore>) -> Local<'static, ArrayBuffer> {
        let array_buffer = ArrayBuffer {
            backing_store: Some(backing_store.clone()),
            byte_length: backing_store.ByteLength(),
            max_byte_length: backing_store.MaxByteLength(),
        };
        Box::leak(Box::new(array_buffer))
    }

        pub fn NewResizableBackingStore(byte_length: usize, max_byte_length: usize) -> Result<Box<BackingStore>, BackingStoreError> {
            if byte_length > max_byte_length {
                return Err(BackingStoreError::InvalidLength);
            }

            if max_byte_length > ArrayBuffer::kMaxByteLength {
                return Err(BackingStoreError::InvalidLength);
            }

            let layout = Layout::from_size_align(max_byte_length, 1).map_err(|_| BackingStoreError::AllocationFailed)?;

            let data = unsafe {
                let ptr = System.allocate_zeroed(layout);
                if ptr.is_null() {
                    panic!("Out of memory");
                }
                ptr
            };

            let deleter: DeleterCallback = ArrayBuffer::free_backing_store;
            let deleter_data: *mut std::ffi::c_void = null_mut();

            Ok(Box::new(BackingStore::new(
                data,
                byte_length,
                max_byte_length,
                false,
                true,
                Some(deleter),
                deleter_data,
            )))
        }
        pub fn IsDetachable(&self) -> bool {
            self.backing_store.as_ref().map_or(false, |bs| !bs.IsShared() && !bs.IsDetached())
        }

        pub fn WasDetached(&self) -> bool {
            self.backing_store.as_ref().map_or(false, |bs| bs.IsDetached())
        }

        pub fn Detach(&mut self) {
        if let Some(backing_store) = &mut self.backing_store {
            if self.IsDetachable() {
                backing_store.detach();
                self.byte_length = 0;
            }
        }
    }

        pub fn Detach_value(&mut self, key: v8::Local<v8::Value>) -> Maybe<bool> {
        if self.IsDetachable() {
            self.Detach();
            Just(true)
        } else {
            Nothing {}
        }
    }
        pub fn GetBackingStore(&self) -> std::shared_ptr::SharedPtr<BackingStore> {
            self.backing_store.clone().unwrap()
        }

        pub fn IsResizableByUserJavaScript(&self) -> bool {
        self.backing_store.as_ref().map_or(false, |bs| bs.IsResizableByUserJavaScript())
    }

        pub fn Data(&self) -> *mut std::ffi::c_void {
        self.backing_store.as_ref().map_or(null_mut(), |bs| bs.Data())
    }

        unsafe extern "C" fn free_backing_store(data: *mut std::ffi::c_void, length: usize, _deleter_data: *mut std::ffi::c_void) {
            if !data.is_null() {
                let layout = Layout::from_size_align(length, 1).unwrap();
                System.deallocate(data as *mut u8, layout);
            }
        }

        pub const kInternalFieldCount: i32 = 2;
        pub const kEmbedderFieldCount: i32 = Self::kInternalFieldCount;
        #[cfg(target_pointer_width = "32")]
        pub const kMaxByteLength: usize = std::i32::MAX as usize;
        #[cfg(target_pointer_width = "64")]
        pub const kMaxByteLength: usize = (1 << 53) - 1;
    }

    pub struct ArrayBufferView {}

    impl ArrayBufferView {
        pub const kInternalFieldCount: i32 = 2;
        pub const kEmbedderFieldCount: i32 = Self::kInternalFieldCount;
    }
    pub struct DataView {}
    impl DataView {
        pub fn New(array_buffer: Local<ArrayBuffer>, byte_offset: usize, length: usize) -> Local<'static, DataView> {
        Box::leak(Box::new(DataView {}))
    }
        pub fn New_shared(shared_array_buffer: Local<SharedArrayBuffer>, byte_offset: usize, length: usize) -> Local<'static, DataView> {
        Box::leak(Box::new(DataView {}))
    }
    }

    pub struct SharedArrayBuffer_ {
        backing_store: Option<std::shared_ptr::SharedPtr<BackingStore>>,
        byte_length: usize,
        max_byte_length: usize,
    }
    impl SharedArrayBuffer_ {
        pub fn ByteLength(&self) -> usize {
            self.byte_length
        }

        pub fn MaxByteLength(&self) -> usize {
            self.max_byte_length
        }
        pub fn New(isolate: *mut Isolate, byte_length: usize, initialization_mode: BackingStoreInitializationMode) -> Local<'static, SharedArrayBuffer_> {
        let backing_store = SharedArrayBuffer_::NewBackingStore(
            unsafe { &mut *isolate },
            byte_length,
            initialization_mode,
            BackingStoreOnFailureMode::kOutOfMemory,
        )
        .expect("Failed to allocate backing store");
        let shared_backing_store = std::shared_ptr::SharedPtr::new(backing_store);
        let shared_array_buffer = SharedArrayBuffer_ {
            backing_store: Some(shared_backing_store),
            byte_length,
            max_byte_length: byte_length,
        };
        Box::leak(Box::new(shared_array_buffer))
    }

        pub fn NewBackingStore(
            isolate: &mut Isolate,
            byte_length: usize,
            initialization_mode: BackingStoreInitializationMode,
            on_failure: BackingStoreOnFailureMode,
        ) -> Result<Box<BackingStore>, BackingStoreError> {
            if byte_length > ArrayBuffer::kMaxByteLength {
                return Err(BackingStoreError::InvalidLength);
            }

            let layout = Layout::from_size_align(byte_length, 1).map_err(|_| BackingStoreError::AllocationFailed)?;

            let data = unsafe {
                match initialization_mode {
                    BackingStoreInitializationMode::kZeroInitialized => {
                        let ptr = System.allocate_zeroed(layout);
                        if ptr.is_null() {
                            match on_failure {
                                BackingStoreOnFailureMode::kReturnNull => return Err(BackingStoreError::AllocationFailed),
                                BackingStoreOnFailureMode::kOutOfMemory => panic!("Out of memory"),
                            }
                        }
                        ptr
                    }
                    BackingStoreInitializationMode::kUninitialized => {
                        let ptr = System.allocate(layout);
                        if ptr.is_null() {
                            match on_failure {
                                BackingStoreOnFailureMode::kReturnNull => return Err(BackingStoreError::AllocationFailed),
                                BackingStoreOnFailureMode::kOutOfMemory => panic!("Out of memory"),
                            }
                        }
                        ptr
                    }
                }
            };

            let deleter: DeleterCallback = ArrayBuffer::free_backing_store;
            let deleter_data: *mut std::ffi::c_void = null_mut();

            Ok(Box::new(BackingStore::new(
                data,
                byte_length,
                byte_length,
                true,
                false,
                Some(deleter),
                deleter_data,
            )))
        }

        pub fn NewBackingStore_data(
            data: *mut std::ffi::c_void,
            byte_length: usize,
            deleter: v8::DeleterCallback,
            deleter_data: *mut std::ffi::c_void,
        ) -> Result<Box<BackingStore>, BackingStoreError> {
            if data.is_null() {
                return Err(BackingStoreError::AllocationFailed);
            }

            if byte_length > ArrayBuffer::kMaxByteLength {
                return Err(BackingStoreError::InvalidLength);
            }

            Ok(Box::new(BackingStore::new(
                data,
                byte_length,
                byte_length,
                true,
                false,
                Some(deleter),
                deleter_data,
            )))
        }

        pub fn GetBackingStore(&self) -> std::shared_ptr::SharedPtr<BackingStore> {
            self.backing_store.clone().unwrap()
        }
        pub fn Data(&self) -> *mut std::ffi::c_void {
        self.backing_store.as_ref().map_or(null_mut(), |bs| bs.Data())
    }

        pub const kInternalFieldCount: i32 = 2;
    }

    pub struct SharedArrayBuffer {}
    pub struct MaybeLocal<'a, T> {
        value: Option<&'a T>,
    }
    impl<'a, T> MaybeLocal<'a, T> {
        pub fn from_local(local: Local<'a, T>) -> Self {
            MaybeLocal { value: Some(local) }
        }

        pub fn empty() -> Self {
            MaybeLocal { value: None }
        }

        pub fn is_empty(&self) -> bool {
            self.value.is_none()
        }

        pub fn to_local(&self) -> Option<Local<'a, T>> {
            self.value
        }
    }
    pub trait Value {}

    impl Value for ArrayBuffer {}
    impl Value for DataView {}
    impl Value for SharedArrayBuffer {}
    impl Value for Object {}

    pub struct Maybe<T> {
        value: Option<T>,
    }

    pub fn Just<T>(value: T) -> Maybe<T> {
        Maybe { value: Some(value) }
    }

    pub struct Nothing {}

    impl<T> Maybe<T> {
        pub fn is_nothing(&self) -> bool {
            self.value.is_none()
        }

        pub fn from_just(value: T) -> Self {
            Maybe { value: Some(value) }
        }

        pub fn nothing() -> Self {
            Maybe { value: None }
        }
    }
    unsafe impl GlobalAlloc for System {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout)
        }
    }
}
