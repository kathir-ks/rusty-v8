// src/init/isolate-group.rs

use std::cell::RefCell;
use std::ptr::null_mut;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, Once,
};

// Placeholder for platform-specific memory allocation.
mod platform {
    pub struct PageAllocator;
    impl PageAllocator {
        pub fn allocate_page_size(&self) -> usize {
            4096 // Default page size
        }
        pub fn get_random_mmap_addr(&self) -> usize {
            0 // Dummy value
        }
    }
    pub fn get_platform_page_allocator() -> PageAllocator {
        PageAllocator
    }

    pub enum Permission {
        NoAccess,
        ReadWrite,
        // kNoAccessWillJitLater
    }

    pub struct AddressRegion {
        pub base: usize,
        pub size: usize,
    }

    impl AddressRegion {
        pub fn new(base: usize, size: usize) -> Self {
            AddressRegion { base, size }
        }
    }
}

mod base {
    pub struct BoundedPageAllocator;
    impl BoundedPageAllocator {
        pub fn new() -> Self {
            BoundedPageAllocator
        }
    }

    pub enum PageInitializationMode {
        KAllocatedPagesCanBeUninitialized,
    }

    pub enum PageFreeingMode {
        KMakeInaccessible,
    }

    pub struct RegionAllocator {
        begin_: usize,
        size_: usize,
        granularity_: usize,
        current_: RefCell<usize>,
        on_merge_callback: RefCell<Option<Box<dyn Fn(usize, usize)>>>,
    }

    impl RegionAllocator {
        pub fn new(base: usize, size: usize, granularity: usize) -> Self {
            RegionAllocator {
                begin_: base,
                size_: size,
                granularity_: granularity,
                current_: RefCell::new(base),
                on_merge_callback: RefCell::new(None),
            }
        }

        pub fn allocate_region(&self, length: usize) -> usize {
            let mut current = self.current_.borrow_mut();
            let region = *current;
            let aligned_length = crate::utils::round_up(length, self.granularity_);

            if region + aligned_length > self.begin_ + self.size_ {
                return Self::KALLOCATION_FAILURE;
            }

            *current += aligned_length;
            region
        }

        pub fn free_region(&self, region: usize) -> bool {
            // Dummy implementation, no actual freeing
            true
        }

        pub fn set_on_merge_callback<F>(&self, callback: F)
        where
            F: Fn(usize, usize) + 'static,
        {
            *self.on_merge_callback.borrow_mut() = Some(Box::new(callback));
        }

        pub fn begin(&self) -> usize {
            self.begin_
        }

        pub fn end(&self) -> usize {
            self.begin_ + self.size_
        }

        pub fn size(&self) -> usize {
            self.size_
        }
        const KALLOCATION_FAILURE: usize = 0;
    }
}

mod common {
    pub mod ptr_compr_inl {
        pub const K_PTR_COMPR_CAGE_RESERVATION_SIZE: usize = 2 * 1024 * 1024 * 1024; // 2GB
        pub const K_PTR_COMPR_CAGE_BASE_ALIGNMENT: usize = 2 * 1024 * 1024;       // 2MB
    }
}

mod compiler_dispatcher {
    pub struct OptimizingCompileDispatcher;
}

mod execution {
    pub struct IsolateGroup;
    pub struct Isolate {
        pub isolate_group_: *mut IsolateGroup,
        pub owns_shareable_data_: bool,
        pub is_shared_space_isolate_: bool,
    }

    impl Isolate {
        pub fn new(isolate_group: *mut IsolateGroup) -> Self {
            Isolate {
                isolate_group_: isolate_group,
                owns_shareable_data_: false,
                is_shared_space_isolate_: false,
            }
        }
        pub fn isolate_group(&self) -> &IsolateGroup {
            unsafe { &(*self.isolate_group_) }
        }
    }
}

mod heap {
    pub struct CodeRange;
    impl CodeRange {
        pub fn new() -> Self {
            CodeRange
        }
        pub fn init_reservation(
            &mut self,
            _page_allocator: &crate::platform::PageAllocator,
            _requested_size: usize,
            _immutable: bool,
        ) -> bool {
            // Dummy Implementation
            true
        }

        pub fn base(&self) -> usize {
            0 // Dummy base address
        }
    }
    pub struct ReadOnlyHeap;
    pub mod read_only_spaces {
        pub struct SnapshotData;
    }
    pub mod trusted_range {
        pub struct TrustedRange;

        impl TrustedRange {
            pub fn ensure_process_wide_trusted_range(_size: usize) -> TrustedRange {
                TrustedRange
            }
        }
        pub const K_MAXIMAL_TRUSTED_RANGE_SIZE: usize = 256 * 1024 * 1024;
    }
    pub mod read_only_heap {
        use crate::execution::Isolate;
        use crate::heap::read_only_spaces::SnapshotData;

        pub fn set_up(_isolate: *mut Isolate, _read_only_snapshot_data: *mut SnapshotData, _can_rehash: bool) {
            // dummy implementation
        }
    }
}

mod sandbox {
    pub struct CodePointerTable;
    impl CodePointerTable {
        pub fn new() -> Self {
            CodePointerTable
        }
        pub fn initialize(&mut self) {}
        pub fn tear_down(&mut self) {}
    }
    pub struct Sandbox {
        address_space: VirtualAddressSpace,
        base_: usize,
        page_allocator: platform::PageAllocator,
        is_initialized_: bool,
    }

    impl Sandbox {
        pub fn new(address_space: VirtualAddressSpace) -> Self {
            Sandbox {
                address_space,
                base_: 0, //Needs to be initialized later
                page_allocator: platform::get_platform_page_allocator(),
                is_initialized_: false,
            }
        }
        pub fn is_initialized(&self) -> bool {
            self.is_initialized_
        }

        pub fn base(&self) -> usize {
            self.base_
        }

        pub fn address_space(&self) -> &VirtualAddressSpace {
            &self.address_space
        }

        pub fn page_allocator(&self) -> &platform::PageAllocator {
            &self.page_allocator
        }

        pub fn initialize(&mut self) -> Result<(), String> {
            self.base_ = self.address_space().allocate_pages(
                0,
                crate::common::ptr_compr_inl::K_PTR_COMPR_CAGE_RESERVATION_SIZE,
                crate::common::ptr_compr_inl::K_PTR_COMPR_CAGE_BASE_ALIGNMENT,
                platform::Permission::NoAccess,
            );

            if self.base_ == 0 {
                return Err("Failed to initialize Sandbox".to_string());
            }

            self.is_initialized_ = true;
            Ok(())
        }

        pub fn tear_down(&mut self) {
            // Free allocated resources here.
            // Dummy implementation, add proper deallocation
            self.is_initialized_ = false;
        }

        pub fn get_default() -> &'static Mutex<Option<Sandbox>> {
            static DEFAULT_SANDBOX: Mutex<Option<Sandbox>> = Mutex::new(None);
            static INIT: Once = Once::new();

            INIT.call_once(|| {
                let mut sandbox = Sandbox::new(VirtualAddressSpace::new());
                if let Err(e) = sandbox.initialize() {
                    panic!("Failed to initialize default sandbox: {}", e);
                }
                let mut guard = DEFAULT_SANDBOX.lock().unwrap();
                *guard = Some(sandbox);
            });

            unsafe {
                let ptr = &DEFAULT_SANDBOX as *const Mutex<Option<Sandbox>>;
                &(*ptr)
            }
        }
    }

    pub struct VirtualAddressSpace;

    impl VirtualAddressSpace {
        pub fn new() -> Self {
            VirtualAddressSpace
        }
        pub fn allocate_pages(
            &self,
            _hint: usize,
            size: usize,
            alignment: usize,
            _permissions: platform::Permission,
        ) -> usize {
            //Dummy implementation
            0 // Dummy address for now.
        }

        pub fn decommit_pages(&self, _start: usize, _size: usize) -> bool {
            true //Dummy Implementation
        }

        pub fn discard_system_pages(&self, _start: usize, _size: usize) -> bool {
            true //Dummy Implementation
        }

        pub fn free_pages(&self, _start: usize, _size: usize) {}

        pub fn set_page_permissions(&self, _start: usize, _size: usize, _permissions: platform::Permission) -> bool {
            true
        }
    }
}

mod utils {
    pub fn memcopy(_src: *const u8, _dest: *mut u8, _len: usize) {}

    pub fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }

    pub fn round_down(value: usize, alignment: usize) -> usize {
        value & !(alignment - 1)
    }

    pub fn is_aligned(value: usize, alignment: usize) -> bool {
        value & (alignment - 1) == 0
    }
}

// Mock V8 interface
mod v8 {
    pub fn fatal_process_out_of_memory(_location: *const u8, message: &str) -> ! {
        panic!("Out of memory: {}", message);
    }
}

// Flags placeholder
mod v8_flags {
    pub static SHARED_HEAP: bool = false;
}

mod external_reference {
    pub struct ExternalReferenceTable;
    impl ExternalReferenceTable {
        pub fn initialize_once_per_isolate_group(&self) {}
    }
}

// external code compression scheme placeholder
mod external_code_compression_scheme {
    pub fn init_base(_base: usize) {}

    pub fn prepare_cage_base_address(_base: usize) -> usize {
        _base
    }
}

// v8 heap compression scheme placeholder
mod v8_heap_compression_scheme {
    pub fn init_base(_base: usize) {}

    pub fn base() -> usize {
        0
    }
}

pub struct IsolateGroup {
    reference_count_: AtomicUsize,
    isolate_count_: usize,
    process_wide_: bool,
    page_allocator_: *mut platform::PageAllocator, // Option<Box<platform::PageAllocator>>,
    optimizing_compile_task_executor_: Option<Box<compiler_dispatcher::OptimizingCompileDispatcher>>,
    code_range_: Option<Box<heap::CodeRange>>,
    read_only_artifacts_: Option<Box<ReadOnlyArtifacts>>,
    mutex_: Mutex<()>,
    init_code_range_: Once,
    shared_space_isolate_: *mut execution::Isolate,
    pointer_compression_cage_: *const VirtualMemoryCage, //Option<Box<VirtualMemoryCage>>,
    trusted_pointer_compression_cage_: heap::trusted_range::TrustedRange,
    sandbox_: *mut sandbox::Sandbox,
    js_dispatch_table_: JsDispatchTable,
    code_pointer_table_: sandbox::CodePointerTable,
    external_ref_table_: external_reference::ExternalReferenceTable,
}

unsafe impl Send for IsolateGroup {}
unsafe impl Sync for IsolateGroup {}

impl IsolateGroup {
    thread_local! {
        static CURRENT: *mut IsolateGroup = null_mut();
    }

    pub fn current_non_inlined() -> *mut IsolateGroup {
        IsolateGroup::CURRENT.with(|c| *c)
    }

    pub fn set_current_non_inlined(group: *mut IsolateGroup) {
        IsolateGroup::CURRENT.with(|c| *c = group);
    }

    pub fn current() -> *mut IsolateGroup {
        IsolateGroup::CURRENT.with(|c| *c)
    }

    pub fn set_current(group: *mut IsolateGroup) {
        IsolateGroup::CURRENT.with(|c| *c = group);
    }

    pub fn get_ptr_compr_cage_base(&self) -> usize {
        0 //Dummy implementation
    }

    pub fn code_pointer_table(&mut self) -> &mut sandbox::CodePointerTable {
        &mut self.code_pointer_table_
    }

    pub fn js_dispatch_table(&mut self) -> &mut JsDispatchTable {
        &mut self.js_dispatch_table_
    }

    const DEFAULT_ISOLATE_GROUP: Mutex<*mut IsolateGroup> = Mutex::new(null_mut());

    pub fn default_isolate_group() -> *mut IsolateGroup {
        *IsolateGroup::DEFAULT_ISOLATE_GROUP.lock().unwrap()
    }

    pub fn set_default_isolate_group(group: *mut IsolateGroup) {
        *IsolateGroup::DEFAULT_ISOLATE_GROUP.lock().unwrap() = group;
    }

    fn new() -> Self {
        Self {
            reference_count_: AtomicUsize::new(1),
            isolate_count_: 0,
            process_wide_: false,
            page_allocator_: null_mut(), //None,
            optimizing_compile_task_executor_: None,
            code_range_: None,
            read_only_artifacts_: None,
            mutex_: Mutex::new(()),
            init_code_range_: Once::new(),
            shared_space_isolate_: null_mut(),
            pointer_compression_cage_: null_mut(), //None,
            trusted_pointer_compression_cage_: heap::trusted_range::TrustedRange::ensure_process_wide_trusted_range(
                heap::trusted_range::K_MAXIMAL_TRUSTED_RANGE_SIZE,
            ),
            sandbox_: null_mut(),
            js_dispatch_table_: JsDispatchTable::new(),
            code_pointer_table_: sandbox::CodePointerTable::new(),
            external_ref_table_: external_reference::ExternalReferenceTable,
        }
    }

    pub fn get_default() -> *mut IsolateGroup {
        let mut default_group = IsolateGroup::default_isolate_group();

        if default_group.is_null() {
            let new_group = Box::into_raw(Box::new(IsolateGroup::new()));
            IsolateGroup::set_default_isolate_group(new_group);
            default_group = new_group;
        }

        default_group
    }

    #[allow(unused_variables)]
    fn initialize(&mut self, process_wide: bool, sandbox: *mut sandbox::Sandbox) {
        self.process_wide_ = process_wide;

        let sandbox_ref: &mut sandbox::Sandbox = unsafe { &mut (*sandbox) };

        if let Err(e) = sandbox_ref.initialize() {
            v8::fatal_process_out_of_memory(null_mut(), &format!("Sandbox initialization failed: {}", e));
        }

        self.sandbox_ = sandbox;
        self.page_allocator_ = sandbox_ref.page_allocator() as *const platform::PageAllocator as *mut platform::PageAllocator;

        self.optimizing_compile_task_executor_ = Some(Box::new(compiler_dispatcher::OptimizingCompileDispatcher));
        self.code_pointer_table().initialize();
        self.js_dispatch_table().initialize();
    }

    pub fn initialize_once_per_process() {
        let mut default_group = IsolateGroup::default_isolate_group();
        if !default_group.is_null() {
            return;
        }

        let group = Box::into_raw(Box::new(IsolateGroup::new()));

        IsolateGroup::set_default_isolate_group(group);

        let group_ref: &mut IsolateGroup = unsafe { &mut (*group) };
        let sandbox_guard = sandbox::Sandbox::get_default();
        let sandbox_ptr: *mut sandbox::Sandbox;

        {
            let sandbox_option = sandbox_guard.lock().unwrap();

            if let Some(ref sandbox) = *sandbox_option {
                sandbox_ptr = sandbox as *const sandbox::Sandbox as *mut sandbox::Sandbox;
            } else {
                panic!("Default sandbox not initialized");
            }
        }

        group_ref.initialize(true, sandbox_ptr);

        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        {
            v8_heap_compression_scheme::init_base(group_ref.get_ptr_compr_cage_base());
        }

        #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
        {
            external_code_compression_scheme::init_base(v8_heap_compression_scheme::base());
        }

        IsolateGroup::set_current(group);
    }

    pub fn tear_down_once_per_process() {
        IsolateGroup::release_default();
    }

    pub fn release(&self) {
        let prev_count = self.reference_count_.load(Ordering::Relaxed);
        if prev_count == 0 {
            return;
        }

        let new_count = self.reference_count_.fetch_sub(1, Ordering::Relaxed) - 1;

        if new_count == 0 {
            // This is the last reference, so drop the IsolateGroup.
            drop(self);
        }
    }

    pub fn ensure_code_range(&mut self, requested_size: usize) -> *mut heap::CodeRange {
        let page_allocator = unsafe { &mut (*self.page_allocator_) };

        self.init_code_range_.call_once(|| {
            let mut code_range = Box::new(heap::CodeRange::new());
            if !code_range.init_reservation(page_allocator, requested_size, self.process_wide_) {
                v8::fatal_process_out_of_memory(null_mut(), "Failed to reserve virtual memory for CodeRange");
            }
            self.code_range_ = Some(code_range);
        });

        match &mut self.code_range_ {
            Some(code_range) => Box::into_raw(code_range),
            None => std::ptr::null_mut(),
        }
    }

    pub fn initialize_read_only_artifacts(&mut self) -> *mut ReadOnlyArtifacts {
        let _guard = self.mutex_.lock().unwrap();
        if self.read_only_artifacts_.is_none() {
            self.read_only_artifacts_ = Some(Box::new(ReadOnlyArtifacts::new()));
        }
        match &mut self.read_only_artifacts_ {
            Some(artifacts) => artifacts as *mut ReadOnlyArtifacts,
            None => std::ptr::null_mut(),
        }
    }

    pub fn get_backing_store_page_allocator(&self) -> *mut platform::PageAllocator {
        let sandbox_ref: &sandbox::Sandbox = unsafe { &mut (*self.sandbox_) };
        sandbox_ref.page_allocator() as *const platform::PageAllocator as *mut platform::PageAllocator
    }

    pub fn setup_read_only_heap(
        &mut self,
        isolate: *mut execution::Isolate,
        read_only_snapshot_data: *mut heap::read_only_spaces::SnapshotData,
        can_rehash: bool,
    ) {
        let _guard = self.mutex_.lock().unwrap();
        let isolate_ref: &mut execution::Isolate = unsafe { &mut (*isolate) };
        heap::read_only_heap::set_up(isolate, read_only_snapshot_data, can_rehash);
    }

    pub fn add_isolate(&mut self, isolate: *mut execution::Isolate) {
        let isolate_ref: &mut execution::Isolate = unsafe { &mut (*isolate) };
        let _guard = self.mutex_.lock().unwrap();
        self.isolate_count_ += 1;

        if let Some(executor) = &mut self.optimizing_compile_task_executor_ {
            // Ensure initialized.
        }

        if v8_flags::SHARED_HEAP {
            if self.has_shared_space_isolate() {
                isolate_ref.owns_shareable_data_ = false;
            } else {
                self.init_shared_space_isolate(isolate);
                isolate_ref.is_shared_space_isolate_ = true;
                assert!(isolate_ref.owns_shareable_data_);
            }
        }
    }

    pub fn remove_isolate(&mut self, isolate: *mut execution::Isolate) {
        let isolate_ref: &mut execution::Isolate = unsafe { &mut (*isolate) };
        let _guard = self.mutex_.lock().unwrap();

        self.isolate_count_ -= 1;

        if self.isolate_count_ == 0 {
            self.read_only_artifacts_ = None;

            assert_eq!(self.has_shared_space_isolate(), isolate_ref.is_shared_space_isolate_);

            if isolate_ref.is_shared_space_isolate_ {
                assert_eq!(isolate as *mut execution::Isolate, self.shared_space_isolate_);
                self.shared_space_isolate_ = null_mut();
            }
        } else {
            assert!(!isolate_ref.is_shared_space_isolate_);
        }
    }

    pub fn new_group() -> *mut IsolateGroup {
        let group = Box::into_raw(Box::new(IsolateGroup::new()));

        let sandbox = Box::into_raw(Box::new(sandbox::Sandbox::new(sandbox::VirtualAddressSpace::new())));
        let group_ref: &mut IsolateGroup = unsafe { &mut (*group) };

        if let Err(e) = unsafe { &mut (*sandbox) }.initialize() {
            v8::fatal_process_out_of_memory(null_mut(), &format!("Sandbox initialization failed: {}", e));
        }

        group_ref.initialize(false, sandbox);

        let group_access_scope = IsolateGroupAccessScope::new(group);

        group_ref.external_ref_table().initialize_once_per_isolate_group();

        group
    }

    pub fn release_default() {
        let group = IsolateGroup::get_default();
        let group_ref: &IsolateGroup = unsafe { &(*group) };

        assert_eq!(group_ref.reference_count_.load(Ordering::Relaxed), 1);
        assert!(!group_ref.has_shared_space_isolate());

        unsafe {
            Box::from_raw(group as *mut IsolateGroup);
        }
        IsolateGroup::set_default_isolate_group(null_mut());
    }

    pub fn optimizing_compile_task_executor(&mut self) -> &mut compiler_dispatcher::OptimizingCompileDispatcher {
        self.optimizing_compile_task_executor_.as_mut().unwrap()
    }

    pub fn sandbox(&mut self) -> &mut sandbox::Sandbox {
        unsafe { &mut (*self.sandbox_) }
    }

    pub fn external_ref_table(&mut self) -> &mut external_reference::ExternalReferenceTable {
        &mut self.external_ref_table_
    }

    fn has_shared_space_isolate(&self) -> bool {
        !self.shared_space_isolate_.is_null()
    }

    fn init_shared_space_isolate(&mut self, isolate: *mut execution::Isolate) {
        self.shared_space_isolate_ = isolate;
    }
}

impl Drop for IsolateGroup {
    fn drop(&mut self) {
        assert_eq!(self.reference_count_.load(Ordering::Relaxed), 0);
        assert_eq!(self.isolate_count_, 0);
        self.js_dispatch_table_.tear_down();
        self.code_pointer_table_.tear_down();

        if let Some(code_range) = self.code_range_.take() {
            drop(code_range);
        }
    }
}

#[derive(Default)]
struct IsolateGroupAccessScope {
    previous_: *mut IsolateGroup,
}

impl IsolateGroupAccessScope {
    fn new(group: *mut IsolateGroup) -> Self {
        let previous_ = IsolateGroup::current();
        IsolateGroup::set_current(group);
        IsolateGroupAccessScope { previous_ }
    }
}

impl Drop for IsolateGroupAccessScope {
    fn drop(&mut self) {
        IsolateGroup::set_current(self.previous_);
    }
}

struct PtrComprCageReservationParams {
    page_allocator: platform::PageAllocator,
    reservation_size: usize,
    base_alignment: usize,
    page_size: usize,
    requested_start_hint: usize,
    permissions: platform::Permission,
    page_initialization_mode: base::PageInitializationMode,
    page_freeing_mode: base::PageFreeingMode,
}

impl PtrComprCageReservationParams {
    fn new() -> Self {
        let page_allocator = platform::get_platform_page_allocator();
        let page_size = utils::round_up(
            1 << 12, //kPageSizeBits
            page_allocator.allocate_page_size(),
        );
        let requested_start_hint = utils::round_down(
            page_allocator.get_random_mmap_addr(),
            common::ptr_compr_inl::K_PTR_COMPR_CAGE_BASE_ALIGNMENT,
        );

        PtrComprCageReservationParams {
            page_allocator,
            reservation_size: common::ptr_compr_inl::K_PTR_COMPR_CAGE_RESERVATION_SIZE,
            base_alignment: common::ptr_compr_inl::K_PTR_COMPR_CAGE_BASE_ALIGNMENT,
            page_size,
            requested_start_hint,
            permissions: platform::Permission::NoAccess,
            page_initialization_mode: base::PageInitializationMode::KAllocatedPagesCanBeUninitialized,
            page_freeing_mode: base::PageFreeingMode::KMakeInaccessible,
        }
    }
}

struct VirtualMemoryCage {
    reservation_params: PtrComprCageReservationParams,
}

impl VirtualMemoryCage {
    pub fn new() -> Self {
        VirtualMemoryCage {
            reservation_params: PtrComprCageReservationParams::new(),
        }
    }

    pub fn is_reserved(&self) -> bool {
        false //Dummy Implementation
    }

    pub fn free(&mut self) {}

    pub fn init_reservation(&mut self, _params: PtrComprCageReservationParams, _existing_reservation: platform::AddressRegion) -> bool {
        true
    }

    pub fn page_allocator(&self) -> &platform::PageAllocator {
        &self.reservation_params.page_allocator
    }
}

struct ReadOnlyArtifacts;

impl ReadOnlyArtifacts {
    fn new() -> Self {
        ReadOnlyArtifacts
    }
}

struct JsDispatchTable;

impl JsDispatchTable {
    fn new() -> Self {
        JsDispatchTable
    }

    fn initialize(&mut self) {}

    fn tear_down(&mut self) {}
}

#[derive(Default)]
pub struct SandboxedArrayBufferAllocator {
    mutex_: Mutex<()>,
    region_alloc_: Option<Box<base::RegionAllocator>>,
    end_of_accessible_region_: usize,
    sandbox_: *mut sandbox::Sandbox,
    is_initialized_: bool,
}

impl SandboxedArrayBufferAllocator {
    const KCHUNK_SIZE: usize = 2 * 1024 * 1024;
    const KALLOCATION_GRANULARITY: usize = 16;

    pub fn lazy_initialize(&mut self, sandbox: *mut sandbox::Sandbox) {
        let _guard = self.mutex_.lock().unwrap();

        if self.is_initialized() {
            return;
        }

        self.sandbox_ = sandbox;
        let sandbox_ref: &sandbox::Sandbox = unsafe { &mut (*sandbox) };
        assert!(sandbox_ref.is_initialized());

        const MAX_BACKING_MEMORY_SIZE: usize = 8 * 1024 * 1024 * 1024;
        const MIN_BACKING_MEMORY_SIZE: usize = 1 * 1024 * 1024 * 1024;

        let mut backing_memory_size = MAX_BACKING_MEMORY_SIZE;
        let mut backing_memory_base = 0;

        while backing_memory_base == 0 && backing_memory_size >= MIN_BACKING_MEMORY_SIZE {
            backing_memory_base = sandbox_ref.address_space().allocate_pages(
                0,
                backing_memory_size,
                Self::KCHUNK_SIZE,
                platform::Permission::NoAccess,
            );

            if backing_memory_base == 0 {
                backing_memory_size /= 2;
            }
        }

        if backing_memory_base == 0 {
            v8::fatal_process_out_of_memory(
                null_mut(),
                "Could not reserve backing memory for ArrayBufferAllocators",
            );
        }
        assert!(utils::is_aligned(backing_memory_base, Self::KCHUNK_SIZE));

        self.region_alloc_ = Some(Box::new(base::RegionAllocator::new(
            backing_memory_base,
            backing_memory_size,
            Self::KALLOCATION_GRANULARITY,
        )));
        self.end_of_accessible_region_ = self.region_alloc_.as_ref().unwrap().begin();

        let region_alloc = self.region_alloc_.as_ref().unwrap();
        let on_merge_callback = |start: usize, size: usize| {
            let _guard = self.mutex_.lock().unwrap();
            let end = start + size;
            if end == region_alloc.end()
                && start <= self.end_of_accessible_region_ - Self::KCHUNK_SIZE
            {
                let new_end_of_accessible_region = utils::round_up(start, Self::KCHUNK_SIZE);
                let size_to_decommit =
                    self.end_of_accessible_region_ - new_end_of_accessible_region;

                if !sandbox_ref.address_space().decommit_pages(
                    new_end_of_accessible_region,
                    size_to_decommit,
                ) {
                    v8::fatal_process_out_of_memory(
                        null_mut(),
                        "SandboxedArrayBufferAllocator()",
                    );
                }
                self.end_of_accessible_region_ = new_end_of_accessible_region;
            } else if size >= 2 * Self::KCHUNK_SIZE {
                let chunk_start = utils::round_up(start, Self::KCHUNK_SIZE);
                let chunk_end = utils::round_down(start + size, Self::KCHUNK_SIZE);

                if !sandbox_ref.address_space().discard_system_pages(
                    chunk_start,
                    chunk_end - chunk_start,
                ) {
                    v8::fatal_process_out_of_memory(
                        null_mut(),
                        "SandboxedArrayBufferAllocator()",
                    );
                }
            }
        };
        self.region_alloc_
            .as_ref()
            .unwrap()
            .set_on_merge_callback(on_merge_callback);
        self.is_initialized_ = true;
    }

    pub fn allocate(&mut self, length: usize) -> *mut u8 {
        let _guard = self.mutex_.lock().unwrap();

        let length = utils::round_up(length, Self::KALLOCATION_GRANULARITY);
        let region_alloc = self.region_alloc_.as_ref().unwrap();
        let region = region_alloc.allocate_region(length);

        if region == base::RegionAllocator::KALLOCATION_FAILURE {
            return null_mut();
        }

        let end = region + length;
        let mut length_to_memset = length;

        if end > self.end_of_accessible_region_ {
            let new_end_of_accessible_region = utils::round_up(end, Self::KCHUNK_SIZE);
            let size = new_end_of_accessible_region - self.end_of_accessible_region_;
            let sandbox_ref: &sandbox::Sandbox = unsafe { &mut (*self.sandbox_) };
            if !sandbox_ref.address_space().set_page_permissions(
                self.end_of_accessible_region_,
                size,
                platform::Permission::ReadWrite,
            ) {
                if !region_alloc.free_region(region) {
                    v8::fatal_process_out_of_memory(
                        null_mut(),
                        "SandboxedArrayBufferAllocator::Allocate()",
                    );
                }
                return null_mut();
            }

            length_to_memset = self.end_of_accessible_region_ - region;
            self.end_of_accessible_region_ = new_end_of_accessible_region;
        }

        let mem = region as *mut u8;
        unsafe {
            std::ptr::write_bytes(mem, 0, length_to_memset);
        }
        mem
    }

    pub fn free(&mut self, data