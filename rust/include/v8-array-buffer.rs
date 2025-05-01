pub mod array_buffer {
    use std::sync::Arc;
    use std::{ptr::null_mut, mem::MaybeUninit, alloc::{Layout, alloc_zeroed, dealloc}};
    use std::ops::{Deref, DerefMut};

    pub use v8_rs::value::Value;
    pub use v8_rs::object::Object;

    mod v8_rs {
        pub mod value {
            pub struct Value {
                // Opaque field, needs more information to properly represent.
                _private: [u8; 0],
            }
        }

        pub mod object {
            use super::value::Value;
            pub struct Object {
                // Opaque field, needs more information to properly represent.
                _private: [u8; 0],
            }

            impl Object {
                pub fn cast<'a>(value: &'a Value) -> &'a Object {
                  unsafe {
                    &*(value as *const Value as *const Object)
                  }
                }
            }
        }
    }

    pub mod platform {
        // Placeholder for platform related types/functions
        pub struct PageAllocator {}
    }

    pub mod internal {
        pub const kMaxSafeBufferSizeForSandbox: usize = 1 << 20; // Example Value. Needs actual value from V8.
        //pub struct BackingStoreBase {}
        // Need more information on what BackingStoreBase is to properly implement.
    }

    // TODO: Find a better name than array_buffer here.
    pub const V8_ARRAY_BUFFER_INTERNAL_FIELD_COUNT: usize = 2;

    #[derive(Clone, Copy, Debug)]
    pub enum ArrayBufferCreationMode {
        kInternalized,
        kExternalized,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum BackingStoreInitializationMode {
        kZeroInitialized,
        kUninitialized,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum BackingStoreOnFailureMode {
        kReturnNull,
        kOutOfMemory,
    }

    // Opaque type representing BackingStoreBase
    pub struct BackingStoreBase {
        _private: [u8; 0],
    }

    /// A wrapper around the backing store (i.e. the raw memory) of an array buffer.
    /// See a document linked in http://crbug.com/v8/9908 for more information.
    ///
    /// The allocation and destruction of backing stores is generally managed by
    /// V8. Clients should always use standard C++ memory ownership types (i.e.
    /// std::unique_ptr and std::shared_ptr) to manage lifetimes of backing stores
    /// properly, since V8 internal objects may alias backing stores.
    ///
    /// This object does not keep the underlying |ArrayBuffer::Allocator| alive by
    /// default. Use Isolate::CreateParams::array_buffer_allocator_shared when
    /// creating the Isolate to make it hold a reference to the allocator itself.
    pub struct BackingStore {
        data: *mut u8,
        byte_length: usize,
        max_byte_length: usize,
        is_shared: bool,
        is_resizable_by_user_javascript: bool,
        deleter: Option<DeleterCallback>,
        deleter_data: *mut std::ffi::c_void,
    }

    //type BackingStorePtr = *mut BackingStore;

    pub type DeleterCallback = unsafe extern "C" fn(data: *mut std::ffi::c_void, length: usize, deleter_data: *mut std::ffi::c_void);

    impl BackingStore {
        // Private constructor, accessible only within the module.
        fn new(data: *mut u8, byte_length: usize, max_byte_length: usize, is_shared: bool, is_resizable_by_user_javascript: bool, deleter: Option<DeleterCallback>, deleter_data: *mut std::ffi::c_void) -> Self {
            BackingStore {
                data,
                byte_length,
                max_byte_length,
                is_shared,
                is_resizable_by_user_javascript,
                deleter,
                deleter_data,
            }
        }

        /// Return a pointer to the beginning of the memory block for this backing
        /// store. The pointer is only valid as long as this backing store object
        /// lives.
        pub fn data(&self) -> *mut u8 {
            self.data
        }

        /// The length (in bytes) of this backing store.
        pub fn byte_length(&self) -> usize {
            self.byte_length
        }

        /// The maximum length (in bytes) that this backing store may grow to.
        ///
        /// If this backing store was created for a resizable ArrayBuffer or a growable
        /// SharedArrayBuffer, it is >= ByteLength(). Otherwise it is ==
        /// ByteLength().
        pub fn max_byte_length(&self) -> usize {
            self.max_byte_length
        }

        /// Indicates whether the backing store was created for an ArrayBuffer or
        /// a SharedArrayBuffer.
        pub fn is_shared(&self) -> bool {
            self.is_shared
        }

        /// Indicates whether the backing store was created for a resizable ArrayBuffer
        /// or a growable SharedArrayBuffer, and thus may be resized by user JavaScript
        /// code.
        pub fn is_resizable_by_user_javascript(&self) -> bool {
            self.is_resizable_by_user_javascript
        }

        /// If the memory block of a BackingStore is static or is managed manually,
        /// then this empty deleter along with nullptr deleter_data can be passed to
        /// ArrayBuffer::NewBackingStore to indicate that.
        ///
        /// The manually managed case should be used with caution and only when it
        /// is guaranteed that the memory block freeing happens after detaching its
        /// ArrayBuffer.
        pub unsafe extern "C" fn empty_deleter(data: *mut std::ffi::c_void, length: usize, deleter_data: *mut std::ffi::c_void) {
            // Intentionally empty.
            std::ptr::drop_in_place(data);
        }
    }

    impl Drop for BackingStore {
        fn drop(&mut self) {
            if let Some(deleter) = self.deleter {
                unsafe {
                    deleter(self.data as *mut std::ffi::c_void, self.byte_length, self.deleter_data);
                }
            } else if !self.data.is_null() {
                unsafe {
                    dealloc(self.data, Layout::from_size_align(self.byte_length, std::mem::align_of::<u8>()).unwrap());
                }
            }
        }
    }

    /// An instance of the built-in ArrayBuffer constructor (ES6 draft 15.13.5).
    pub struct ArrayBuffer {
        // Opaque internal data, needs more information to properly represent.
        _private: [u8; 0],
    }

    /// A thread-safe allocator that V8 uses to allocate |ArrayBuffer|'s memory.
    /// The allocator is a global V8 setting. It has to be set via
    /// Isolate::CreateParams.
    ///
    /// Memory allocated through this allocator by V8 is accounted for as external
    /// memory by V8. Note that V8 keeps track of the memory for all internalized
    /// |ArrayBuffer|s. Responsibility for tracking external memory (using
    /// Isolate::AdjustAmountOfExternalAllocatedMemory) is handed over to the
    /// embedder upon externalization and taken over upon internalization (creating
    /// an internalized buffer from an existing buffer).
    ///
    /// Note that it is unsafe to call back into V8 from any of the allocator
    /// functions.
    pub trait Allocator {
        /// Allocate |length| bytes. Return nullptr if allocation is not successful.
        /// Memory should be initialized to zeroes.
        fn allocate(&self, length: usize) -> *mut std::ffi::c_void;

        /// Allocate |length| bytes. Return nullptr if allocation is not successful.
        /// Memory does not have to be initialized.
        fn allocate_uninitialized(&self, length: usize) -> *mut std::ffi::c_void;

        /// Free the memory block of size |length|, pointed to by |data|.
        /// That memory is guaranteed to be previously allocated by |Allocate|.
        fn free(&self, data: *mut std::ffi::c_void, length: usize);

        /// Returns a size_t that determines the largest ArrayBuffer that can be
        /// allocated.  Override if your Allocator is more restrictive than the
        /// default.  Will only be called once, and the value returned will be
        /// cached.
        /// Should not return a value that is larger than kMaxByteLength.
        fn max_allocation_size(&self) -> usize {
            ArrayBuffer::kMaxByteLength
        }

        /// ArrayBuffer allocation mode. kNormal is a malloc/free style allocation,
        /// while kReservation is for larger allocations with the ability to set
        /// access permissions.
        type AllocationMode;

        /// Returns page allocator used by this Allocator instance.
        ///
        /// When the sandbox used by Allocator it is expected that this returns
        /// sandbox's page allocator.
        /// Otherwise, it should return system page allocator.
        fn get_page_allocator(&self) -> *mut platform::PageAllocator {
            std::ptr::null_mut()
        }

        // Need IsolateGroup struct
        // /// Convenience allocator.
        // ///
        // /// When the sandbox is enabled, this allocator will allocate its backing
        // /// memory inside the sandbox that belongs to passed isolate group.
        // /// Otherwise, it will rely on malloc/free.
        // ///
        // /// Caller takes ownership, i.e. the returned object needs to be freed using
        // /// |delete allocator| once it is no longer in use.
        // #[cfg(all(feature = "V8_COMPRESS_POINTERS", not(feature = "V8_COMPRESS_POINTERS_IN_SHARED_CAGE")))]
        // fn new_default_allocator(group: &IsolateGroup) -> Box<dyn Allocator>;

        /// Convenience allocator.
        ///
        /// When the sandbox is enabled, this allocator will allocate its backing
        /// memory inside the default global sandbox. Otherwise, it will rely on
        /// malloc/free.
        ///
        /// Caller takes ownership, i.e. the returned object needs to be freed using
        /// |delete allocator| once it is no longer in use.
        fn new_default_allocator() -> Box<dyn Allocator> {
            Box::new(DefaultAllocator {})
        }
    }

    // A default allocator
    struct DefaultAllocator {}

    impl Allocator for DefaultAllocator {
        type AllocationMode = ();

        fn allocate(&self, length: usize) -> *mut std::ffi::c_void {
            unsafe {
                let layout = Layout::from_size_align(length, std::mem::align_of::<u8>()).unwrap();
                alloc_zeroed(layout) as *mut std::ffi::c_void
            }
        }

        fn allocate_uninitialized(&self, length: usize) -> *mut std::ffi::c_void {
            unsafe {
                let layout = Layout::from_size_align(length, std::mem::align_of::<u8>()).unwrap();
                std::alloc::alloc(layout) as *mut std::ffi::c_void
            }
        }

        fn free(&self, data: *mut std::ffi::c_void, length: usize) {
            unsafe {
                let layout = Layout::from_size_align(length, std::mem::align_of::<u8>()).unwrap();
                dealloc(data as *mut u8, layout);
            }
        }
    }

    impl ArrayBuffer {
        pub const kInternalFieldCount: usize = V8_ARRAY_BUFFER_INTERNAL_FIELD_COUNT;
        pub const kEmbedderFieldCount: usize = Self::kInternalFieldCount;

        #[cfg(all(feature = "V8_ENABLE_SANDBOX"))]
        pub const kMaxByteLength: usize = internal::kMaxSafeBufferSizeForSandbox;
        #[cfg(all(not(feature = "V8_ENABLE_SANDBOX"), feature = "V8_HOST_ARCH_32_BIT"))]
        pub const kMaxByteLength: usize = std::i32::MAX as usize;
        #[cfg(all(not(feature = "V8_ENABLE_SANDBOX"), not(feature = "V8_HOST_ARCH_32_BIT")))]
        pub const kMaxByteLength: usize = (1_u64 << 53) as usize - 1;

        /// Data length in bytes.
        pub fn byte_length(&self) -> usize {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.byte_length()
            }
        }

        /// Maximum length in bytes.
        pub fn max_byte_length(&self) -> usize {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.max_byte_length()
            }
        }

        // Isolate struct is required
        // /// Attempt to create a new ArrayBuffer. Allocate |byte_length| bytes.
        // /// Allocated memory will be owned by a created ArrayBuffer and
        // /// will be deallocated when it is garbage-collected,
        // /// unless the object is externalized. If allocation fails, the Maybe
        // /// returned will be empty.
        // pub fn maybe_new(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        // ) -> Option<Local<ArrayBuffer>>;

        // Isolate struct is required
        // /// Create a new ArrayBuffer. Allocate |byte_length| bytes, which are either
        // /// zero-initialized or uninitialized. Allocated memory will be owned by a
        // /// created ArrayBuffer and will be deallocated when it is garbage-collected,
        // /// unless the object is externalized.
        // pub fn new(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        // ) -> Local<ArrayBuffer>;

        // Isolate struct is required
        // /// Create a new ArrayBuffer with an existing backing store.
        // /// The created array keeps a reference to the backing store until the array
        // /// is garbage collected. Note that the IsExternal bit does not affect this
        // /// reference from the array to the backing store.
        // ///
        // /// In future IsExternal bit will be removed. Until then the bit is set as
        // /// follows. If the backing store does not own the underlying buffer, then
        // /// the array is created in externalized state. Otherwise, the array is created
        // /// in internalized state. In the latter case the array can be transitioned
        // /// to the externalized state using Externalize(backing_store).
        // pub fn new(isolate: &mut Isolate, backing_store: std::shared_ptr<BackingStore>) -> Local<ArrayBuffer>;

        // Isolate struct is required
        // /// Returns a new standalone BackingStore that is allocated using the array
        // /// buffer allocator of the isolate. The allocation can either be zero
        // /// initialized, or uninitialized. The result can be later passed to
        // /// ArrayBuffer::New.
        // ///
        // /// If the allocator returns nullptr, then the function may cause GCs in the
        // /// given isolate and re-try the allocation.
        // ///
        // /// If GCs do not help and on_failure is kOutOfMemory, then the
        // /// function will crash with an out-of-memory error.
        // ///
        // /// Otherwise if GCs do not help (or the allocation is too large for GCs to
        // /// help) and on_failure is kReturnNull, then a null result is returned.
        // pub fn new_backing_store(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        //     on_failure: BackingStoreOnFailureMode,
        // ) -> std::unique_ptr<BackingStore>;

        /// Returns a new standalone BackingStore that takes over the ownership of
        /// the given buffer. The destructor of the BackingStore invokes the given
        /// deleter callback.
        ///
        /// The result can be later passed to ArrayBuffer::New. The raw pointer
        /// to the buffer must not be passed again to any V8 API function.
        pub fn new_backing_store(
            data: *mut std::ffi::c_void,
            byte_length: usize,
            deleter: Option<DeleterCallback>,
            deleter_data: *mut std::ffi::c_void,
        ) -> Box<BackingStore> {
            Box::new(BackingStore::new(data as *mut u8, byte_length, byte_length, false, false, deleter, deleter_data))
        }

        /// Returns a new resizable standalone BackingStore that is allocated using the
        /// array buffer allocator of the isolate. The result can be later passed to
        /// ArrayBuffer::New.
        ///
        /// |byte_length| must be <= |max_byte_length|.
        ///
        /// This function is usable without an isolate. Unlike |NewBackingStore| calls
        /// with an isolate, GCs cannot be triggered, and there are no
        /// retries. Allocation failure will cause the function to crash with an
        /// out-of-memory error.
        pub fn new_resizable_backing_store(byte_length: usize, max_byte_length: usize) -> Box<BackingStore> {
            unsafe {
                let layout = Layout::from_size_align(max_byte_length, std::mem::align_of::<u8>()).unwrap();
                let data = alloc_zeroed(layout) as *mut u8;
                if data.is_null() {
                    panic!("Allocation failed for resizable backing store");
                }
                Box::new(BackingStore::new(data, byte_length, max_byte_length, false, true, None, null_mut()))
            }
        }

        /// Returns true if this ArrayBuffer may be detached.
        pub fn is_detachable(&self) -> bool {
            // Needs ArrayBuffer internal state info
            false
        }

        /// Returns true if this ArrayBuffer has been detached.
        pub fn was_detached(&self) -> bool {
            // Needs ArrayBuffer internal state info
            false
        }

        // V8_DEPRECATED
        // /// Detaches this ArrayBuffer and all its views (typed arrays).
        // /// Detaching sets the byte length of the buffer and all typed arrays to zero,
        // /// preventing JavaScript from ever accessing underlying backing store.
        // /// ArrayBuffer should have been externalized and must be detachable.
        // pub fn detach(&mut self);

        // /// Detaches this ArrayBuffer and all its views (typed arrays).
        // /// Detaching sets the byte length of the buffer and all typed arrays to zero,
        // /// preventing JavaScript from ever accessing underlying backing store.
        // /// ArrayBuffer should have been externalized and must be detachable. Returns
        // /// Nothing if the key didn't pass the [[ArrayBufferDetachKey]] check,
        // /// Just(true) otherwise.
        // pub fn detach(&mut self, key: Local<Value>) -> Option<bool>;

        // /// Sets the ArrayBufferDetachKey.
        // pub fn set_detach_key(&mut self, key: Local<Value>);

        /// Get a shared pointer to the backing store of this array buffer. This
        /// pointer coordinates the lifetime management of the internal storage
        /// with any live ArrayBuffers on the heap, even across isolates. The embedder
        /// should not attempt to manage lifetime of the storage through other means.
        ///
        /// The returned shared pointer will not be empty, even if the ArrayBuffer has
        /// been detached. Use |WasDetached| to tell if it has been detached instead.
        pub fn get_backing_store(&self) -> Arc<BackingStore> {
            // This is a placeholder.  Needs ArrayBuffer internal state information
            // to return the correct Arc<BackingStore>
            unsafe {
              let layout = Layout::from_size_align(0, std::mem::align_of::<u8>()).unwrap();
              let data = alloc_zeroed(layout) as *mut u8;
              let backing_store = BackingStore::new(data, 0, 0, false, false, None, null_mut());
              Arc::new(backing_store)
            }
        }

        /// More efficient shortcut for
        /// GetBackingStore()->IsResizableByUserJavaScript().
        pub fn is_resizable_by_user_javascript(&self) -> bool {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.is_resizable_by_user_javascript()
            }
        }

        /// More efficient shortcut for GetBackingStore()->Data(). The returned pointer
        /// is valid as long as the ArrayBuffer is alive.
        pub fn data(&self) -> *mut u8 {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.data()
            }
        }

        pub fn cast<'a>(value: &'a Value) -> &'a ArrayBuffer {
          unsafe {
            &*(value as *const Value as *const ArrayBuffer)
          }
        }
    }

    /// A base class for an instance of one of "views" over ArrayBuffer,
    /// including TypedArrays and DataView (ES6 draft 15.13).
    pub struct ArrayBufferView {
        // Opaque internal data, needs more information to properly represent.
        _private: [u8; 0],
    }

    impl ArrayBufferView {
        /// Returns underlying ArrayBuffer.
        pub fn buffer(&self) -> ArrayBuffer {
            // Needs ArrayBufferView internal state info to return the correct ArrayBuffer
            ArrayBuffer{ _private: [] }
        }

        /// Byte offset in |Buffer|.
        pub fn byte_offset(&self) -> usize {
            // Needs ArrayBufferView internal state info
            0
        }

        /// Size of a view in bytes.
        pub fn byte_length(&self) -> usize {
            // Needs ArrayBufferView internal state info
            0
        }

        /// Copy the contents of the ArrayBufferView's buffer to an embedder defined
        /// memory without additional overhead that calling ArrayBufferView::Buffer
        /// might incur.
        ///
        /// Will write at most min(|byte_length|, ByteLength) bytes starting at
        /// ByteOffset of the underlying buffer to the memory starting at |dest|.
        /// Returns the number of bytes actually written.
        pub fn copy_contents(&self, dest: *mut std::ffi::c_void, byte_length: usize) -> usize {
            // Needs ArrayBufferView internal state info
            0
        }

        /// Returns the contents of the ArrayBufferView's buffer as a MemorySpan. If
        /// the contents are on the V8 heap, they get copied into `storage`. Otherwise
        /// a view into the off-heap backing store is returned. The provided storage
        /// should be at least as large as the maximum on-heap size of a TypedArray,
        /// was defined in gn with `typed_array_max_size_in_heap`. The default value is
        /// 64 bytes.
        pub fn get_contents(&self, storage: MemorySpan<u8>) -> MemorySpan<u8> {
            // Needs ArrayBufferView internal state info
            storage
        }

        /// Returns true if ArrayBufferView's backing ArrayBuffer has already been
        /// allocated.
        pub fn has_buffer(&self) -> bool {
            // Needs ArrayBufferView internal state info
            false
        }

        pub fn cast<'a>(value: &'a Value) -> &'a ArrayBufferView {
          unsafe {
            &*(value as *const Value as *const ArrayBufferView)
          }
        }

        pub const kInternalFieldCount: usize = 2;
        pub const kEmbedderFieldCount: usize = Self::kInternalFieldCount;
    }

    /// An instance of DataView constructor (ES6 draft 15.13.7).
    pub struct DataView {
        // Opaque internal data, needs more information to properly represent.
        _private: [u8; 0],
    }

    impl DataView {
        // SharedArrayBuffer struct is required
        // pub fn new(array_buffer: Local<ArrayBuffer>, byte_offset: usize, length: usize) -> Local<DataView>;
        // pub fn new(shared_array_buffer: Local<SharedArrayBuffer>, byte_offset: usize, length: usize) -> Local<DataView>;

        pub fn cast<'a>(value: &'a Value) -> &'a DataView {
          unsafe {
            &*(value as *const Value as *const DataView)
          }
        }
    }

    /// An instance of the built-in SharedArrayBuffer constructor.
    pub struct SharedArrayBuffer {
        // Opaque internal data, needs more information to properly represent.
        _private: [u8; 0],
    }

    impl SharedArrayBuffer {
        /// Data length in bytes.
        pub fn byte_length(&self) -> usize {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.byte_length()
            }
        }

        /// Maximum length in bytes.
        pub fn max_byte_length(&self) -> usize {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.max_byte_length()
            }
        }

        // Isolate struct is required
        // /// Create a new SharedArrayBuffer. Allocate |byte_length| bytes, which are
        // /// either zero-initialized or uninitialized. Allocated memory will be owned by
        // /// a created SharedArrayBuffer and will be deallocated when it is
        // /// garbage-collected, unless the object is externalized.
        // pub fn new(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        // ) -> Local<SharedArrayBuffer>;

        // Isolate struct is required
        // /// Create a new SharedArrayBuffer. Allocate |byte_length| bytes, which are
        // /// either zero-initialized or uninitialized. Allocated memory will be owned by
        // /// a created SharedArrayBuffer and will be deallocated when it is
        // /// garbage-collected, unless the object is externalized.  If allocation
        // /// fails, the Maybe returned will be empty.
        // pub fn maybe_new(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        // ) -> Option<Local<SharedArrayBuffer>>;

        // Isolate struct is required
        // /// Create a new SharedArrayBuffer with an existing backing store.
        // /// The created array keeps a reference to the backing store until the array
        // /// is garbage collected. Note that the IsExternal bit does not affect this
        // /// reference from the array to the backing store.
        // ///
        // /// In future IsExternal bit will be removed. Until then the bit is set as
        // /// follows. If the backing store does not own the underlying buffer, then
        // /// the array is created in externalized state. Otherwise, the array is created
        // /// in internalized state. In the latter case the array can be transitioned
        // /// to the externalized state using Externalize(backing_store).
        // pub fn new(
        //     isolate: &mut Isolate,
        //     backing_store: std::shared_ptr<BackingStore>,
        // ) -> Local<SharedArrayBuffer>;

        // Isolate struct is required
        // /// Returns a new standalone BackingStore that is allocated using the array
        // /// buffer allocator of the isolate. The allocation can either be zero
        // /// initialized, or uninitialized. The result can be later passed to
        // /// SharedArrayBuffer::New.
        // ///
        // /// If the allocator returns nullptr, then the function may cause GCs in the
        // /// given isolate and re-try the allocation.
        // ///
        // /// If on_failure is kOutOfMemory and GCs do not help, then the function will
        // /// crash with an out-of-memory error.
        // ///
        // /// Otherwise, if on_failure is kReturnNull and GCs do not help (or the
        // /// byte_length is so large that the allocation cannot succeed), then a null
        // /// result is returned.
        // pub fn new_backing_store(
        //     isolate: &mut Isolate,
        //     byte_length: usize,
        //     initialization_mode: BackingStoreInitializationMode,
        //     on_failure: BackingStoreOnFailureMode,
        // ) -> std::unique_ptr<BackingStore>;

        /// Returns a new standalone BackingStore that takes over the ownership of
        /// the given buffer. The destructor of the BackingStore invokes the given
        /// deleter callback.
        ///
        /// The result can be later passed to SharedArrayBuffer::New. The raw pointer
        /// to the buffer must not be passed again to any V8 functions.
        pub fn new_backing_store(
            data: *mut std::ffi::c_void,
            byte_length: usize,
            deleter: Option<DeleterCallback>,
            deleter_data: *mut std::ffi::c_void,
        ) -> Box<BackingStore> {
            Box::new(BackingStore::new(data as *mut u8, byte_length, byte_length, true, false, deleter, deleter_data))
        }

        /// Get a shared pointer to the backing store of this array buffer. This
        /// pointer coordinates the lifetime management of the internal storage
        /// with any live ArrayBuffers on the heap, even across isolates. The embedder
        /// should not attempt to manage lifetime of the storage through other means.
        pub fn get_backing_store(&self) -> Arc<BackingStore> {
            // This is a placeholder.  Needs SharedArrayBuffer internal state information
            // to return the correct Arc<BackingStore>
            unsafe {
              let layout = Layout::from_size_align(0, std::mem::align_of::<u8>()).unwrap();
              let data = alloc_zeroed(layout) as *mut u8;
              let backing_store = BackingStore::new(data, 0, 0, true, false, None, null_mut());
              Arc::new(backing_store)
            }
        }

        /// More efficient shortcut for GetBackingStore()->Data(). The returned pointer
        /// is valid as long as the ArrayBuffer is alive.
        pub fn data(&self) -> *mut u8 {
            unsafe {
                let backing_store = self.get_backing_store();
                backing_store.data()
            }
        }

        pub fn cast<'a>(value: &'a Value) -> &'a SharedArrayBuffer {
          unsafe {
            &*(value as *const Value as *const SharedArrayBuffer)
          }
        }

        pub const kInternalFieldCount: usize = V8_ARRAY_BUFFER_INTERNAL_FIELD_COUNT;
    }

    /// A memory span.
    #[derive(Clone, Copy, Debug)]
    pub struct MemorySpan<'a, T> {
        data: *mut T,
        len: usize,
        _phantom: std::marker::PhantomData<&'a mut [T]>,
    }

    impl<'a, T> MemorySpan<'a, T> {
        /// Creates a new `MemorySpan` from a pointer and a length.
        ///
        /// # Safety
        ///
        /// The pointer must be valid for reads and writes of `len` elements of type `T`.
        pub unsafe fn from_raw_parts(data: *mut T, len: usize) -> Self {
            MemorySpan {
                data,
                len,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Returns a pointer to the start of the memory span.
        pub fn as_mut_ptr(&self) -> *mut T {
            self.data
        }

        /// Returns the length of the memory span.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns true if the memory span is empty.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    impl<'a, T> Deref for MemorySpan<'a, T> {
        type Target = [T];

        fn deref(&self) -> &Self::Target {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
        }
    }

    impl<'a, T> DerefMut for MemorySpan<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}