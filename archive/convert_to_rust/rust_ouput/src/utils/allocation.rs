// Converted from V8 C++ source files:
// Header: allocation.h
// Implementation: allocation.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct AddressRegion {
        begin_: usize,
        size_: usize,
    }

    impl AddressRegion {
        pub fn new(begin: usize, size: usize) -> Self {
            AddressRegion { begin_: begin, size_: size }
        }

        pub fn begin(&self) -> usize {
            self.begin_
        }

        pub fn size(&self) -> usize {
            self.size_
        }

        pub fn end(&self) -> usize {
            self.begin_ + self.size_
        }

        pub fn is_empty(&self) -> bool {
            self.size_ == 0
        }

        pub fn contains(&self, address: usize, size: usize) -> bool {
            address >= self.begin_ && (address + size) <= (self.begin_ + self.size_)
        }

        pub fn set_size(&mut self, size: usize) {
            self.size_ = size;
        }
    }
}

pub mod v8 {
    pub enum Permission {
        kNoAccess,
        kRead,
        kReadWrite,
        kReadExecute,
        kReadWriteExecute,
    }

    pub trait PageAllocator {
        fn allocate_page_size(&self) -> usize;
        fn commit_page_size(&self) -> usize;
        fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void;
        fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: Permission,
        ) -> *mut std::ffi::c_void;
        fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn release_pages(&mut self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool;
        fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool;
        fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool;
        fn discard_system_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
        ) -> bool;
    }

    pub trait VirtualAddressSpace {}
}

pub mod internal {
    use crate::base::AddressRegion;
    use crate::v8::{PageAllocator, VirtualAddressSpace};
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;
    use std::sync::Mutex;

    pub struct V8 {}

    impl V8 {
        pub fn fatal_process_out_of_memory(_: *mut std::ffi::c_void, s: &str) -> ! {
            panic!("Fatal process out of memory: {}", s);
        }

        pub fn get_current_platform() -> &'static dyn Platform {
            static PLATFORM: Leaky<DefaultPlatform> = Leaky::new(DefaultPlatform {});
            &PLATFORM
        }
    }

    pub trait Platform {
        fn get_page_allocator(&self) -> &dyn PageAllocator;
        fn on_critical_memory_pressure(&self);
    }

    struct DefaultPlatform {}

    impl Platform {
        pub fn on_critical_memory_pressure(&self) {}
    }

    impl Platform for DefaultPlatform {
        fn get_page_allocator(&self) -> &dyn PageAllocator {
            static PAGE_ALLOCATOR: Leaky<DefaultPageAllocator> = Leaky::new(DefaultPageAllocator {});
            &PAGE_ALLOCATOR
        }

        fn on_critical_memory_pressure(&self) {}
    }

    struct DefaultPageAllocator {}

    impl PageAllocator for DefaultPageAllocator {
        fn allocate_page_size(&self) -> usize {
            4096
        }

        fn commit_page_size(&self) -> usize {
            4096
        }

        fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void {
            std::ptr::null_mut()
        }

        fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            _access: crate::v8::Permission,
        ) -> *mut std::ffi::c_void {
            unsafe {
                let layout = Layout::from_size_align(size, alignment).unwrap();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return std::ptr::null_mut();
                }
                ptr as *mut std::ffi::c_void
            }
        }

        fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            unsafe {
                let layout = Layout::from_size_align(size, 4096).unwrap();
                dealloc(address as *mut u8, layout);
            }
            true
        }

        fn release_pages(&mut self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool {
            true
        }

        fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            _access: crate::v8::Permission,
        ) -> bool {
            true
        }

        fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: crate::v8::Permission,
        ) -> bool {
            true
        }

         fn discard_system_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
        ) -> bool {
            true
        }
    }

    struct Leaky<T> {
        value: T,
    }

    impl<T> Leaky<T> {
        const fn new(value: T) -> Self {
            Leaky { value }
        }

        fn get(&'static self) -> &'static T {
            &self.value
        }
    }

    impl<T> std::ops::Deref for Leaky<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    #[macro_export]
    macro_rules! V8_UNLIKELY {
        ($condition:expr) => {
            $condition
        };
    }

    #[macro_export]
    macro_rules! V8_LIKELY {
        ($condition:expr) => {
            $condition
        };
    }

    pub struct Malloced {}

    impl Malloced {
        pub fn operator_new(size: usize) -> *mut std::ffi::c_void {
            let result = alloc_with_retry(size, None);
            if unsafe { V8_UNLIKELY!(result.is_null()) } {
                V8::fatal_process_out_of_memory(std::ptr::null_mut(), "Malloced operator new");
            }
            result
        }

        pub fn operator_delete(p: *mut std::ffi::c_void) {
            unsafe { base::free(p) };
        }
    }

    pub fn on_critical_memory_pressure() {
        V8::get_current_platform().on_critical_memory_pressure();
    }

    pub fn new_array<T>(size: usize) -> *mut T {
        unsafe {
            let result =
                std::alloc::alloc(std::alloc::Layout::array::<T>(size).unwrap()) as *mut T;
            if V8_UNLIKELY!(result.is_null()) {
                on_critical_memory_pressure();
                let result =
                    std::alloc::alloc(std::alloc::Layout::array::<T>(size).unwrap()) as *mut T;
                if result.is_null() {
                    V8::fatal_process_out_of_memory(std::ptr::null_mut(), "NewArray");
                }
                return result;
            }
            result
        }
    }

    pub fn new_array_with_default<T>(size: usize, default_val: T) -> *mut T
    where
        T: Copy,
    {
        unsafe {
            let result = new_array::<u8>(std::mem::size_of::<T>() * size) as *mut T;
            for i in 0..size {
                (result.add(i)).write(default_val);
            }
            result
        }
    }

    pub fn delete_array<T>(array: *mut T) {
        unsafe {
            std::alloc::dealloc(
                array as *mut u8,
                std::alloc::Layout::array::<T>(0).unwrap(), // Provide a zero-sized layout for deallocation
            );
        }
    }

    struct ArrayDeleter<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> ArrayDeleter<T> {
        fn new() -> Self {
            ArrayDeleter {
                _phantom: std::marker::PhantomData,
            }
        }

        fn call(&self, array: *mut T) {
            delete_array(array);
        }
    }

    pub type ArrayUniquePtr<T> = std::unique::UniquePtr<T, ArrayDeleter<T>>;

    pub fn str_dup(str: &str) -> *mut i8 {
        let length = str.len();
        let result = new_array::<i8>(length + 1);
        unsafe {
            std::ptr::copy_nonoverlapping(str.as_ptr() as *const i8, result, length);
            (result.add(length)).write(0);
        }
        result
    }

    pub fn str_ndup(str: &str, n: usize) -> *mut i8 {
        let length = str.len();
        let length = if n < length { n } else { length };
        let result = new_array::<i8>(length + 1);
        unsafe {
            std::ptr::copy_nonoverlapping(str.as_ptr() as *const i8, result, length);
            (result.add(length)).write(0);
        }
        result
    }

    pub struct FreeStoreAllocationPolicy {}

    impl FreeStoreAllocationPolicy {
        pub fn allocate_array<T>(length: usize) -> *mut T {
            unsafe { Malloced::operator_new(length * std::mem::size_of::<T>()) as *mut T }
        }

        pub fn delete_array<T>(p: *mut T, length: usize) {
            unsafe { Malloced::operator_delete(p as *mut std::ffi::c_void) }
        }
    }

    type MallocFn = fn(usize) -> *mut std::ffi::c_void;

    fn alloc_with_retry(size: usize, malloc_fn: Option<MallocFn>) -> *mut std::ffi::c_void {
        let malloc_fn = malloc_fn.unwrap_or(base::malloc);
        for _ in 0..2 {
            let result = malloc_fn(size);
            if !result.is_null() {
                return result;
            }
            on_critical_memory_pressure();
        }
        std::ptr::null_mut()
    }

    pub struct AllocationResult<T> {
        pub ptr: *mut T,
        pub count: usize,
    }

    fn alloc_at_least_with_retry(size: usize) -> AllocationResult<std::ffi::c_void> {
        for _ in 0..2 {
            let result = base::allocate_at_least::<i8>(size);
            if !result.ptr.is_null() {
                return AllocationResult {
                    ptr: result.ptr as *mut std::ffi::c_void,
                    count: result.count,
                };
            }
            on_critical_memory_pressure();
        }
        AllocationResult {
            ptr: std::ptr::null_mut(),
            count: 0,
        }
    }

    pub fn aligned_alloc_with_retry(size: usize, alignment: usize) -> *mut std::ffi::c_void {
        for _ in 0..2 {
            let result = unsafe { base::aligned_alloc(alignment, size) };
            if !result.is_null() {
                return result;
            }
            on_critical_memory_pressure();
        }
        V8::fatal_process_out_of_memory(std::ptr::null_mut(), "AlignedAlloc");
    }

    pub fn aligned_free(ptr: *mut std::ffi::c_void) {
        unsafe { base::aligned_free(ptr) };
    }

    pub fn get_platform_page_allocator() -> &'static dyn PageAllocator {
        V8::get_current_platform().get_page_allocator()
    }

    pub fn get_platform_virtual_address_space() -> &'static dyn VirtualAddressSpace {
        static VAS: Leaky<DefaultVirtualAddressSpace> = Leaky::new(DefaultVirtualAddressSpace {});
        &VAS
    }

    struct DefaultVirtualAddressSpace {}

    impl VirtualAddressSpace for DefaultVirtualAddressSpace {}

    pub fn get_array_buffer_page_allocator() -> &'static dyn PageAllocator {
        get_platform_page_allocator()
    }

    pub fn set_platform_page_allocator_for_testing(
        page_allocator: &'static dyn PageAllocator,
    ) -> &'static dyn PageAllocator {
        todo!()
    }

    pub fn allocate_page_size() -> usize {
        get_platform_page_allocator().allocate_page_size()
    }

    pub fn commit_page_size() -> usize {
        get_platform_page_allocator().commit_page_size()
    }

    pub fn get_random_mmap_addr() -> *mut std::ffi::c_void {
        get_platform_page_allocator().get_random_mmap_addr()
    }

    pub fn allocate_pages(
        page_allocator: &mut dyn PageAllocator,
        hint: *mut std::ffi::c_void,
        size: usize,
        alignment: usize,
        access: crate::v8::Permission,
    ) -> *mut std::ffi::c_void {
        unsafe {
            assert!(is_aligned(hint as usize, alignment));
        }

        let mut hint_ = hint;

        for _ in 0..2 {
            let result = page_allocator.allocate_pages(hint_, size, alignment, access);
            if !result.is_null() {
                return result;
            }
            on_critical_memory_pressure();
        }
        std::ptr::null_mut()
    }

    pub fn free_pages(
        page_allocator: &mut dyn PageAllocator,
        address: *mut std::ffi::c_void,
        size: usize,
    ) {
        assert!(is_aligned(size, page_allocator.allocate_page_size()));
        if !page_allocator.free_pages(address, size) {
            V8::fatal_process_out_of_memory(std::ptr::null_mut(), "FreePages");
        }
    }

    pub fn release_pages(
        page_allocator: &mut dyn PageAllocator,
        address: *mut std::ffi::c_void,
        size: usize,
        new_size: usize,
    ) {
        assert!(new_size < size);
        assert!(is_aligned(new_size, page_allocator.commit_page_size()));
        assert!(page_allocator.release_pages(address, size, new_size));
    }

    pub fn set_permissions(
        page_allocator: &mut dyn PageAllocator,
        address: *mut std::ffi::c_void,
        size: usize,
        access: crate::v8::Permission,
    ) -> bool {
        page_allocator.set_permissions(address, size, access)
    }

    pub enum JitPermission {
        kNoJit,
        kMapAsJittable,
    }

    pub struct VirtualMemory {
        page_allocator_: Option<&'static mut dyn PageAllocator>,
        region_: AddressRegion,
    }

    const K_NULL_ADDRESS: usize = 0;

    impl VirtualMemory {
        pub fn new() -> Self {
            VirtualMemory {
                page_allocator_: None,
                region_: AddressRegion { begin_: 0, size_: 0 },
            }
        }

        pub fn with_page_allocator(
            page_allocator: &'static mut dyn PageAllocator,
            size: usize,
            hint: *mut std::ffi::c_void,
            alignment: usize,
            permissions: crate::v8::Permission,
        ) -> Self {
            assert!(is_aligned(size, page_allocator.commit_page_size()));
            let page_size = page_allocator.allocate_page_size();
            let alignment = round_up(alignment, page_size);

            let address = allocate_pages(
                page_allocator,
                hint,
                round_up(size, page_size),
                alignment,
                permissions,
            );

            if address as usize != K_NULL_ADDRESS {
                assert!(is_aligned(address as usize, alignment));
                VirtualMemory {
                    page_allocator_: Some(page_allocator),
                    region_: AddressRegion {
                        begin_: address as usize,
                        size_: size,
                    },
                }
            } else {
                VirtualMemory {
                    page_allocator_: None,
                    region_: AddressRegion { begin_: 0, size_: 0 },
                }
            }
        }

        pub fn from_address_and_size(
            page_allocator: &'static mut dyn PageAllocator,
            address: usize,
            size: usize,
        ) -> Self {
            assert!(!page_allocator.eq(&std::ptr::null()));
            assert!(is_aligned(address, page_allocator.allocate_page_size()));
            assert!(is_aligned(size, page_allocator.commit_page_size()));
            VirtualMemory {
                page_allocator_: Some(page_allocator),
                region_: AddressRegion {
                    begin_: address,
                    size_: size,
                },
            }
        }

        pub fn is_reserved(&self) -> bool {
            self.region_.begin() != K_NULL_ADDRESS
        }

        pub fn reset(&mut self) {
            self.page_allocator_ = None;
            self.region_ = AddressRegion { begin_: 0, size_: 0 };
        }

        pub fn page_allocator(&self) -> Option<&dyn PageAllocator> {
            self.page_allocator_.map(|p| unsafe { &*p })
        }

        pub fn region(&self) -> &AddressRegion {
            &self.region_
        }

        pub fn address(&self) -> usize {
            assert!(self.is_reserved());
            self.region_.begin()
        }

        pub fn end(&self) -> usize {
            assert!(self.is_reserved());
            self.region_.end()
        }

        pub fn size(&self) -> usize {
            self.region_.size()
        }

        pub fn set_permissions(
            &mut self,
            address: usize,
            size: usize,
            access: crate::v8::Permission,
        ) -> bool {
            assert!(self.in_vm(address, size));
            let page_allocator = self.page_allocator_.unwrap();
            page_allocator.set_permissions(address as *mut std::ffi::c_void, size, access)
        }

        pub fn recommit_pages(
            &mut self,
            address: usize,
            size: usize,
            access: crate::v8::Permission,
        ) -> bool {
            assert!(self.in_vm(address, size));
            let page_allocator = self.page_allocator_.unwrap();
            page_allocator.recommit_pages(address as *mut std::ffi::c_void, size, access)
        }

        pub fn discard_system_pages(&mut self, address: usize, size: usize) -> bool {
            assert!(self.in_vm(address, size));
            let page_allocator = self.page_allocator_.unwrap();
            page_allocator.discard_system_pages(address as *mut std::ffi::c_void, size)
        }

        pub fn release(&mut self, free_start: usize) -> usize {
            assert!(self.is_reserved());
            let page_allocator = self.page_allocator_.unwrap();
            assert!(is_aligned(free_start, page_allocator.commit_page_size()));

            let old_size = self.region_.size();
            let free_size = old_size - (free_start - self.region_.begin());
            assert!(self.in_vm(free_start, free_size));
            self.region_.set_size(old_size - free_size);

            release_pages(
                page_allocator,
                self.region_.begin() as *mut std::ffi::c_void,
                old_size,
                self.region_.size(),
            );
            free_size
        }

        pub fn free(&mut self) {
            assert!(self.is_reserved());

            let page_allocator = self.page_allocator_.unwrap();
            let region = self.region_;
            self.reset();
            free_pages(
                page_allocator,
                region.begin() as *mut std::ffi::c_void,
                round_up(region.size(), page_allocator.allocate_page_size()),
            );
        }

        pub fn in_vm(&self, address: usize, size: usize) -> bool {
            self.region_.contains(address, size)
        }
    }

    pub struct VirtualMemoryCage {
        base_: usize,
        size_: usize,
        page_allocator_: Option<Box<base::BoundedPageAllocator>>,
        reservation_: VirtualMemory,
    }

    impl VirtualMemoryCage {
        pub fn new() -> Self {
            VirtualMemoryCage {
                base_: K_NULL_ADDRESS,
                size_: 0,
                page_allocator_: None,
                reservation_: VirtualMemory::new(),
            }
        }

        pub fn base(&self) -> usize {
            self.base_
        }

        pub fn size(&self) -> usize {
            self.size_
        }

        pub fn region(&self) -> AddressRegion {
            AddressRegion {
                begin_: self.base_,
                size_: self.size_,
            }
        }

        pub fn page_allocator(&self) -> Option<&base::BoundedPageAllocator> {
            self.page_allocator_.as_deref()
        }

        pub fn reservation(&mut self) -> &mut VirtualMemory {
            &mut self.reservation_
        }

        pub fn is_reserved(&self) -> bool {
            assert_eq!(self.base_ != K_NULL_ADDRESS, self.reservation_.is_reserved());
            assert_eq!(self.base_ != K_NULL_ADDRESS, self.size_ != 0);
            self.reservation_.is_reserved()
        }

        pub struct ReservationParams {
            pub page_allocator: &'static mut dyn PageAllocator,
            pub reservation_size: usize,
            pub base_alignment: usize,
            pub page_size: usize,
            pub requested_start_hint: *mut std::ffi::c_void,
            pub permissions: crate::v8::Permission,
            pub page_initialization_mode: base::PageInitializationMode,
            pub page_freeing_mode: base::PageFreeingMode,
        }

        pub fn init_reservation(
            &mut self,
            params: &ReservationParams,
            existing_reservation: AddressRegion,
        ) -> bool {
            assert!(!self.reservation_.is_reserved());

            let allocate_page_size = params.page_allocator.allocate_page_size();
            assert!(is_aligned(params.reservation_size, allocate_page_size));
            assert!(
                params.base_alignment == 1
                    || is_aligned(params.base_alignment, allocate_page_size)
            );

            if !existing_reservation.is_empty() {
                assert_eq!(existing_reservation.size(), params.reservation_size);
                assert!(
                    params.base_alignment == 1
                        || is_aligned(existing_reservation.begin(), params.base_alignment)
                );
                self.reservation_ = VirtualMemory::from_address_and_size(
                    params.page_allocator,
                    existing_reservation.begin(),
                    existing_reservation.size(),
                );
                self.base_ = self.reservation_.address();
            } else {
                let hint = params.requested_start_hint;
                assert!(is_aligned(hint as usize, params.base_alignment));
                self.reservation_ = VirtualMemory::with_page_allocator(
                    params.page_allocator,
                    params.reservation_size,
                    hint,
                    params.base_alignment,
                    params.permissions,
                );
                if !self.reservation_.is_reserved() {
                    return false;
                }

                self.base_ = self.reservation_.address();
                assert_eq!(self.reservation_.size(), params.reservation_size);
            }
            assert_ne!(self.base_, K_NULL_ADDRESS);
            assert!(is_aligned(self.base_, params.base_alignment));

            let allocatable_base = round_up(self.base_, params.page_size);
            let allocatable_size = round_down(
                params.reservation_size - (allocatable_base - self.base_),
                params.page_size,
            );
            self.size_ = allocatable_base + allocatable_size - self.base_;

            self.page_allocator_ = Some(Box::new(base::BoundedPageAllocator::new(
                params.page_allocator,
                allocatable_base,
                allocatable_size,
                params.page_size,
                params.page_initialization_mode,
                params.page_freeing_mode,
            )));
            true
        }

        pub fn free(&mut self) {
            if self.is_reserved() {
                self.base_ = K_NULL_ADDRESS;
                self.size_ = 0;
                self.page_allocator_ = None;
                self.reservation_.free();
            }
        }
    }

    impl Drop for VirtualMemoryCage {
        fn drop(&mut self) {
            self.free();
        }
    }

    unsafe fn is_aligned(address: usize, alignment: usize) -> bool {
        (address & (alignment - 1)) == 0
    }

    fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }

    fn round_down(value: usize, alignment: usize) -> usize {
        value & !(alignment - 1)
    }

    pub mod base {
        use crate::v8::PageAllocator;
        use std::sync::Mutex;

        pub enum PageInitializationMode {
            kNoInitialization,
            kZeroInitialize,
        }

        pub enum PageFreeingMode {
            kNormal,
            kEager,
        }

        pub struct BoundedPageAllocator {
            page_allocator_: &'static mut dyn PageAllocator,
            base_: usize,
            size_: usize,
            page_size_: usize,
            page_initialization_mode_: PageInitializationMode,
            page_freeing_mode_: PageFreeingMode,
            mutex: Mutex<()>, // added to handle the FreePages Mutex
        }

        impl BoundedPageAllocator {
            pub fn new(
                page_allocator: &'static mut dyn PageAllocator,
                base: usize,
                size: usize,
                page_size: usize,
                page_initialization_mode: PageInitializationMode,
                page_freeing_mode: PageFreeingMode,
            ) -> Self {
                BoundedPageAllocator {
                    page_allocator_: page_allocator,
                    base_: base,
                    size_: size,
                    page_size_: page_size,
                    page_initialization_mode_: page_initialization_mode,
                    page_freeing_mode_: page_freeing_mode,
                    mutex: Mutex::new(()),
                }
            }

            pub fn allocate_pages(&mut self) -> *mut std::ffi::c_void {
                todo!()
            }
        }

        extern "C" {
            #[no_mangle]
            pub fn free(ptr: *mut std::ffi::c_void);
            #[no_mangle]
            pub fn malloc(size: usize) -> *mut std::ffi::c_void;
            #[no_mangle]
            pub fn aligned_alloc(alignment: usize, size: usize) -> *mut std::ffi::c_void;
            #[no_mangle]
            pub fn aligned_free(ptr: *mut std::ffi::c_void);
        }

        pub struct AllocationResult<T> {
            pub ptr: *mut T,
            pub count: usize,
        }

        pub fn allocate_at_least<T>(size: usize) -> AllocationResult<T> {
            unsafe {
                let ptr = malloc(size) as *mut T;
                if ptr.is_null() {
                    return AllocationResult { ptr: std::ptr::null_mut(), count: 0 };
                }
                AllocationResult { ptr, count: size }
            }
        }

        impl BoundedPageAllocator {
            pub fn FreePages(allocator: *mut BoundedPageAllocator, address: *mut Segment, size: usize) {
            }
        }
    }

    pub struct Segment {}
}

mod std {
    pub mod unique {
        use std::marker::PhantomData;
        use std::ops::{Deref, DerefMut};

        pub struct UniquePtr<T, D> {
            ptr: *mut T,
            deleter: D,
            _phantom: PhantomData<T>,
        }

        impl<T, D> UniquePtr<T, D> {
            pub fn new(ptr: *mut T, deleter: D) -> Self {
                UniquePtr {
                    ptr,
                    deleter,
                    _phantom: PhantomData,
                }
            }
        }

        impl<T, D: Deleter<T>> Drop for UniquePtr<T, D> {
            fn drop(&mut self) {
                self.deleter.call(self.ptr);
            }
        }

        impl<T, D> Deref for UniquePtr<T, D> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.ptr }
            }
        }

        impl<T, D> DerefMut for UniquePtr<T, D> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *self.ptr }
            }
        }

        pub trait Deleter<T> {
            fn call(&self, ptr: *mut T);
        }
    }

    pub mod alloc {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr::NonNull;

        pub unsafe fn allocate<T>(size: usize) -> *mut T {
            let layout = Layout::array::<T>(size).unwrap();
            alloc(layout) as *mut T
        }

        pub unsafe fn deallocate<T>(ptr: *mut T, size: usize) {
            let layout = Layout::array::<T>(size).unwrap();
            dealloc(ptr as *mut u8, layout);
        }
    }

    pub mod mem {
        pub fn copy<T>(src: *const T, dst: *mut T, count: usize) {
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst, count);
            }
        }
    }

    pub mod ptr {
        pub unsafe fn write<T>(dst: *mut T, value: T) {
            *dst = value;
        }
    }

    pub mod marker {
        pub struct PhantomData<T>(std::marker::PhantomData<T>);

        impl<T> PhantomData<T> {
            pub fn new() -> Self {
                PhantomData(std::marker::PhantomData)
            }
        }
    }
}

trait Deleter<T> {
    fn call(&self, ptr: *mut T);
}

fn is_aligned(address: usize, alignment: usize) -> bool {
    (address & (alignment - 1)) == 0
}

fn round_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

fn round_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}
