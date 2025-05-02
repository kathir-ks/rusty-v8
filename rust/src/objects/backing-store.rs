// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod backing_store {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex, Weak};
    use v8::ArrayBufferAllocator;
    use v8::Isolate;
    use v8::BackingStoreDeleterCallback;
    use v8::WasmMemoryObject;
    use v8::DirectHandle;
    use crate::base::EnumSet;
    use crate::sandbox::sandbox::EmptyBackingStoreBuffer;

    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum WasmMemoryFlag {
        NotWasm,
        WasmMemory32,
        WasmMemory64,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SharedFlag {
        NotShared,
        Shared,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum ResizableFlag {
        NotResizable,
        Resizable,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum InitializedFlag {
        Uninitialized,
        ZeroInitialized,
    }

    // Internal information for shared wasm memories. E.g. contains
    // a list of all memory objects (across all isolates) that share this
    // backing store.
    pub struct SharedWasmMemoryData {} // TODO: Add fields

    pub struct BackingStore {
        buffer_start_: *mut std::ffi::c_void,
        byte_length_: AtomicUsize,
        max_byte_length_: usize,
        byte_capacity_: usize,
        id_: u32,
        page_allocator_: *mut v8::PageAllocator, // Raw pointer because PageAllocator is opaque
        type_specific_data_: TypeSpecificData,
        flags_: AtomicUsize,
    }

    union TypeSpecificData {
        v8_api_array_buffer_allocator: *mut ArrayBufferAllocator,
        v8_api_array_buffer_allocator_shared: std::mem::ManuallyDrop<Option<Arc<ArrayBufferAllocator>>>,
        shared_wasm_memory_data: *mut SharedWasmMemoryData,
        deleter: std::mem::ManuallyDrop<DeleterInfo>,
    }

    impl TypeSpecificData {
        const fn new() -> Self {
            TypeSpecificData {
                v8_api_array_buffer_allocator: std::ptr::null_mut(),
            }
        }
    }

    impl Drop for TypeSpecificData {
        fn drop(&mut self) {
            unsafe {
                if !self.v8_api_array_buffer_allocator_shared.is_none() {
                    std::mem::ManuallyDrop::drop(&mut self.v8_api_array_buffer_allocator_shared)
                }
                if !self.deleter.callback.is_none() {
                    std::mem::ManuallyDrop::drop(&mut self.deleter);
                }
            }
        }
    }

    #[repr(C)]
    pub struct DeleterInfo {
        callback: BackingStoreDeleterCallback,
        data: *mut std::ffi::c_void,
    }

    impl BackingStore {
        pub fn new(
            page_allocator: *mut v8::PageAllocator,
            buffer_start: *mut std::ffi::c_void,
            byte_length: usize,
            max_byte_length: usize,
            byte_capacity: usize,
            shared: SharedFlag,
            resizable: ResizableFlag,
            is_wasm_memory: bool,
            is_wasm_memory64: bool,
            has_guard_regions: bool,
            custom_deleter: bool,
            empty_deleter: bool,
        ) -> Self {
            let mut flags = EnumSet::<Flag, u16>::new();
            if shared == SharedFlag::Shared {
                flags.insert(Flag::kIsShared);
            }
            if resizable == ResizableFlag::Resizable {
                flags.insert(Flag::kIsResizableByJs);
            }
            if is_wasm_memory {
                flags.insert(Flag::kIsWasmMemory);
            }
            if is_wasm_memory64 {
                flags.insert(Flag::kIsWasmMemory64);
            }
            if has_guard_regions {
                flags.insert(Flag::kHasGuardRegions);
            }
            if custom_deleter {
                flags.insert(Flag::kCustomDeleter);
            }
            if empty_deleter {
                flags.insert(Flag::kEmptyDeleter);
            }

            BackingStore {
                buffer_start_: buffer_start,
                byte_length_: AtomicUsize::new(byte_length),
                max_byte_length_: max_byte_length,
                byte_capacity_: byte_capacity,
                id_: 0, //TODO: Generate ID
                page_allocator_: page_allocator,
                type_specific_data_: TypeSpecificData::new(),
                flags_: AtomicUsize::new(flags.bits() as usize),
            }
        }

        pub fn buffer_start(&self) -> *mut std::ffi::c_void {
            if !self.buffer_start_.is_null() {
                self.buffer_start_
            } else {
                unsafe {EmptyBackingStoreBuffer()}
            }
        }

        pub fn byte_length(&self, memory_order: Ordering) -> usize {
            self.byte_length_.load(memory_order)
        }

        pub fn max_byte_length(&self) -> usize {
            self.max_byte_length_
        }

        pub fn byte_capacity(&self) -> usize {
            self.byte_capacity_
        }

        pub fn is_shared(&self) -> bool {
            self.has_flag(Flag::kIsShared)
        }

        pub fn is_resizable_by_js(&self) -> bool {
            self.has_flag(Flag::kIsResizableByJs)
        }

        pub fn is_wasm_memory(&self) -> bool {
            self.has_flag(Flag::kIsWasmMemory)
        }

        pub fn is_wasm_memory64(&self) -> bool {
            self.has_flag(Flag::kIsWasmMemory64)
        }

        pub fn has_guard_regions(&self) -> bool {
            self.has_flag(Flag::kHasGuardRegions)
        }

        pub fn is_empty(&self) -> bool {
            self.byte_capacity_ >= self.byte_length.load(Ordering::Relaxed) && self.byte_capacity_ == 0
        }

        pub enum ResizeOrGrowResult {
            kSuccess,
            kFailure,
            kRace,
        }

        pub fn resize_in_place(
            &mut self,
            _isolate: *mut Isolate,
            _new_byte_length: usize,
        ) -> ResizeOrGrowResult {
            todo!()
        }
        pub fn grow_in_place(
            &mut self,
            _isolate: *mut Isolate,
            _new_byte_length: usize,
        ) -> ResizeOrGrowResult {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn grow_wasm_memory_in_place(
            &mut self,
            _isolate: *mut Isolate,
            _delta_pages: usize,
            _max_pages: usize,
        ) -> Option<usize> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn copy_wasm_memory(
            &mut self,
            _isolate: *mut Isolate,
            _new_pages: usize,
            _max_pages: usize,
            _wasm_memory: WasmMemoryFlag,
        ) -> Result<Box<BackingStore>, ()> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn attach_shared_wasm_memory_object(
            &mut self,
            _isolate: *mut Isolate,
            _memory_object: DirectHandle<WasmMemoryObject>,
        ) {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn broadcast_shared_wasm_memory_grow(&self, _isolate: *mut Isolate) {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn remove_shared_wasm_memory_objects(_isolate: *mut Isolate) {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn update_shared_wasm_memory_objects(_isolate: *mut Isolate) {
            todo!()
        }

        pub fn per_isolate_accounting_length(&self) -> usize {
            if self.has_flag(Flag::kIsShared) {
                return 0;
            }
            if self.has_flag(Flag::kEmptyDeleter) {
                return 0;
            }
            self.byte_length(Ordering::Relaxed)
        }

        pub fn id(&self) -> u32 {
            self.id_
        }

        fn get_v8_api_array_buffer_allocator(&mut self) -> *mut ArrayBufferAllocator {
            unsafe { self.type_specific_data_.v8_api_array_buffer_allocator }
        }
        fn get_shared_wasm_memory_data(&self) -> *mut SharedWasmMemoryData {
            unsafe { self.type_specific_data_.shared_wasm_memory_data }
        }

        fn has_flag(&self, flag: Flag) -> bool {
            let flags = EnumSet::<Flag, u16>::from_bits_truncate(self.flags_.load(Ordering::Relaxed) as u16);
            flags.contains(flag)
        }

        fn set_flag(&self, flag: Flag) {
            let mut old_flags = EnumSet::<Flag, u16>::from_bits_truncate(self.flags_.load(Ordering::Relaxed) as u16);
            loop {
                let mut new_flags = old_flags;
                new_flags.insert(flag);
                let old_bits = old_flags.bits() as usize;
                let new_bits = new_flags.bits() as usize;
                match self.flags_.compare_exchange_weak(old_bits, new_bits, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => old_flags = EnumSet::<Flag, u16>::from_bits_truncate(x as u16),
                }
            }
        }

        fn clear_flag(&self, flag: Flag) {
            let mut old_flags = EnumSet::<Flag, u16>::from_bits_truncate(self.flags_.load(Ordering::Relaxed) as u16);
            loop {
                let mut new_flags = old_flags;
                new_flags.remove(flag);
                let old_bits = old_flags.bits() as usize;
                let new_bits = new_flags.bits() as usize;
                match self.flags_.compare_exchange_weak(old_bits, new_bits, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => old_flags = EnumSet::<Flag, u16>::from_bits_truncate(x as u16),
                }
            }
        }

        fn holds_shared_ptr_to_allocator(&self) -> bool {
            self.has_flag(Flag::kHoldsSharedPtrToAllocater)
        }
        fn custom_deleter(&self) -> bool {
            self.has_flag(Flag::kCustomDeleter)
        }
        fn globally_registered(&self) -> bool {
            self.has_flag(Flag::kGloballyRegistered)
        }
    }

    impl Drop for BackingStore {
        fn drop(&mut self) {
            //TODO: Deallocate memory if owned.
            // Call custom deleter if present

            if self.custom_deleter() {
                unsafe {
                    let deleter = &self.type_specific_data_.deleter;
                    let callback = deleter.callback;
                    let data = deleter.data;
                    if !callback.is_none() {
                        callback(self.buffer_start_, data, self.byte_capacity_);
                    }
                }
            }
            // Unregister from global registry if wasm memory and globally registered.
            if self.is_wasm_memory() && self.globally_registered() {
                GlobalBackingStoreRegistry::unregister(self);
            }
        }
    }

    impl BackingStore {
        pub fn allocate(
            _isolate: *mut Isolate,
            _byte_length: usize,
            _shared: SharedFlag,
            _initialized: InitializedFlag,
        ) -> Result<Box<Self>, ()> {
            todo!()
        }

        #[cfg(feature = "v8_enable_webassembly")]
        pub fn allocate_wasm_memory(
            _isolate: *mut Isolate,
            _initial_pages: usize,
            _maximum_pages: usize,
            _wasm_memory: WasmMemoryFlag,
            _shared: SharedFlag,
        ) -> Result<Box<Self>, ()> {
            todo!()
        }

        pub fn try_allocate_and_partially_commit_memory(
            _isolate: *mut Isolate,
            _byte_length: usize,
            _max_byte_length: usize,
            _page_size: usize,
            _initial_pages: usize,
            _maximum_pages: usize,
            _wasm_memory: WasmMemoryFlag,
            _shared: SharedFlag,
            _has_guard_regions: bool,
        ) -> Result<Box<Self>, ()> {
            todo!()
        }

        pub fn wrap_allocation(
            allocation_base: *mut std::ffi::c_void,
            allocation_length: usize,
            deleter: BackingStoreDeleterCallback,
            deleter_data: *mut std::ffi::c_void,
            shared: SharedFlag,
        ) -> Result<Box<Self>, ()> {
            let mut backing_store = Box::new(BackingStore::new(
                std::ptr::null_mut(), // page allocator
                allocation_base,
                allocation_length,
                allocation_length, // max_byte_length
                allocation_length, // byte_capacity
                shared,
                ResizableFlag::NotResizable,
                false,
                false,
                false,
                true,
                false,
            ));

            backing_store.set_flag(Flag::kCustomDeleter);
            unsafe {
                backing_store.type_specific_data_.deleter = std::mem::ManuallyDrop::new(DeleterInfo {
                    callback: deleter,
                    data: deleter_data,
                })
            }

            Ok(backing_store)
        }

        pub fn empty_backing_store(shared: SharedFlag) -> Result<Box<Self>, ()> {
            let backing_store = Box::new(BackingStore::new(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                shared,
                ResizableFlag::NotResizable,
                false,
                false,
                false,
                false,
                true,
            ));
            Ok(backing_store)
        }

        fn set_allocator_from_isolate(&mut self, _isolate: *mut Isolate) {
            todo!()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum Flag {
        kIsShared = 1 << 0,
        kIsResizableByJs = 1 << 1,
        kIsWasmMemory = 1 << 2,
        kIsWasmMemory64 = 1 << 3,
        kHoldsSharedPtrToAllocater = 1 << 4,
        kHasGuardRegions = 1 << 5,
        kGloballyRegistered = 1 << 6,
        kCustomDeleter = 1 << 7,
        kEmptyDeleter = 1 << 8
    }

    // A global, per-process mapping from buffer addresses to backing stores
    // of wasm memory objects.
    pub struct GlobalBackingStoreRegistry {}

    impl GlobalBackingStoreRegistry {
        // Register a backing store in the global registry. A mapping from the
        // {buffer_start} to the backing store object will be added. The backing
        // store will automatically unregister itself upon destruction.
        // Only wasm memory backing stores are supported.
        pub fn register(_backing_store: Arc<BackingStore>) {
            todo!()
        }

        // Unregister a backing store in the global registry.
        fn unregister(_backing_store: *mut BackingStore) {
            todo!()
        }

        // Adds the given memory object to the backing store's weak list
        // of memory objects (under the registry lock).
        fn add_shared_wasm_memory_object(
            _isolate: *mut Isolate,
            _backing_store: *mut BackingStore,
            _memory_object: DirectHandle<WasmMemoryObject>,
        ) {
            todo!()
        }

        // Purge any shared wasm memory lists that refer to this isolate.
        pub fn purge(_isolate: *mut Isolate) {
            todo!()
        }

        // Broadcast updates to all attached memory objects.
        pub fn broadcast_shared_wasm_memory_grow(_isolate: *mut Isolate, _backing_store: *const BackingStore) {
            todo!()
        }

        // Update all shared memory objects in the given isolate.
        pub fn update_shared_wasm_memory_objects(_isolate: *mut Isolate) {
            todo!()
        }
    }
}

mod base {
    use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct EnumSet<T, U> {
        bits: U,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, U: Default> EnumSet<T, U> {
        pub fn new() -> Self {
            EnumSet {
                bits: U::default(),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U: Copy> EnumSet<T, U> {
        pub fn from_bits_truncate(bits: U) -> Self {
            EnumSet {
                bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T: Copy, U: Copy + PartialEq + BitAnd<Output = U>> EnumSet<T, U> {
        pub fn contains(&self, flag: T) -> bool
            where U: From<T>
        {
            (self.bits & U::from(flag)) == U::from(flag)
        }
    }

    impl<T: Copy, U: Copy + BitOr<Output = U> + BitOrAssign> EnumSet<T, U> {
        pub fn insert(&mut self, flag: T)
            where U: From<T>
        {
            self.bits |= U::from(flag);
        }
    }

    impl<T: Copy, U: Copy + BitAnd<Output = U> + BitAndAssign + BitXor<Output = U> + BitXorAssign + PartialEq> EnumSet<T, U> {
        pub fn remove(&mut self, flag: T)
            where U: From<T>
        {
            if self.contains(flag) {
                self.bits ^= U::from(flag);
            }
        }
    }

    impl<T, U> EnumSet<T, U> {
        pub fn bits(&self) -> U
        {
            self.bits
        }
    }

    impl<T, U: Not<Output = U>> Not for EnumSet<T, U> {
        type Output = Self;

        fn not(self) -> Self::Output {
            EnumSet {
                bits: !self.bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U: BitAnd<Output = U>> BitAnd for EnumSet<T, U> {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits: self.bits & rhs.bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U: BitOr<Output = U>> BitOr for EnumSet<T, U> {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits: self.bits | rhs.bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U: BitXor<Output = U>> BitXor for EnumSet<T, U> {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits: self.bits ^ rhs.bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }
}

mod sandbox {
    pub mod sandbox {
        //TODO: Implement Sandbox class
        pub unsafe fn EmptyBackingStoreBuffer() -> *mut std::ffi::c_void {
            std::ptr::null_mut()
        }
    }
}