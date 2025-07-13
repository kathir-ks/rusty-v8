// Converted from V8 C++ source files:
// Header: isolate-group.h
// Implementation: isolate-group.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod isolate_group {
    // Copyright 2024 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    use crate::base::once::OnceType;
    use crate::flags::flags::COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL;
    use crate::heap::memory_chunk_constants::MemoryChunkConstants;
    use crate::sandbox::code_pointer_table::CodePointerTable;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Mutex as StdMutex;

    #[cfg(V8_ENABLE_LEAPTIERING)]
    use crate::sandbox::js_dispatch_table::JSDispatchTable;

    #[cfg(V8_ENABLE_SANDBOX)]
    use crate::base::region_allocator::RegionAllocator;

    use crate::init::v8::V8;
    use crate::init::v8::Isolate;
    use crate::heap::code_range::CodeRange;
    use crate::init::isolate_group::VirtualMemoryCage;
    use crate::init::isolate_group::Address;
    use crate::codegen::external_reference_table::ExternalReferenceTable;
    use std::mem::MaybeUninit;
    use crate::heap::read_only_heap::ReadOnlyHeap;
    use crate::snapshot::snapshot::SnapshotData;
    use crate::init::isolate_group::PageAllocator;
    use crate::init::isolate_group::OptimizingCompileTaskExecutor;
    use crate::heap::read_only_artifacts::ReadOnlyArtifacts;

    #[cfg(V8_ENABLE_SANDBOX)]
    use crate::sandbox::sandbox::Sandbox;

    #[cfg(V8_ENABLE_SANDBOX)]
    use crate::init::isolate_group::SandboxedArrayBufferAllocator;
    use crate::base::platform::memory::GetPlatformPageAllocator;
    use crate::base::platform::memory::RoundUp;
    use crate::base::platform::memory::RoundDown;
    use crate::heap::trusted_range::TrustedRange;
    use crate::flags::flags::kMaximalTrustedRangeSize;
    use crate::flags::flags::kPtrComprCageBaseAlignment;
    use crate::flags::flags::kPtrComprCageReservationSize;
    use crate::flags::flags::kPageSizeBits;
    use crate::init::isolate_group::VirtualAddressSpace;
    use crate::init::isolate_group::PagePermissions;
    use crate::utils::allocation::IsAligned;
    use crate::flags::flags::v8_flags;
    use std::sync::Mutex;
    use crate::common::globals::MB;

    pub struct IsolateGroup {
        reference_count_: AtomicI32,
        isolate_count_: i32,
        page_allocator_: *mut PageAllocator,

        #[cfg(V8_COMPRESS_POINTERS)]
        trusted_pointer_compression_cage_: *mut VirtualMemoryCage,
        #[cfg(V8_COMPRESS_POINTERS)]
        pointer_compression_cage_: *mut VirtualMemoryCage,
        #[cfg(V8_COMPRESS_POINTERS)]
        reservation_: VirtualMemoryCage,

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        current_: Option<*mut IsolateGroup>,

        init_code_range_: OnceType,
        code_range_: Option<Box<CodeRange>>,
        external_ref_table_: [Address; ExternalReferenceTable::kSizeIsolateIndependent],

        process_wide_: bool,

        mutex_: StdMutex<()>,
        read_only_artifacts_: Option<Box<ReadOnlyArtifacts>>,
        shared_read_only_heap_: *mut ReadOnlyHeap,
        shared_space_isolate_: *mut Isolate,
        optimizing_compile_task_executor_: Option<Box<OptimizingCompileTaskExecutor>>,

        #[cfg(V8_ENABLE_SANDBOX)]
        sandbox_: *mut Sandbox,
        #[cfg(V8_ENABLE_SANDBOX)]
        code_pointer_table_: CodePointerTable,
        #[cfg(V8_ENABLE_SANDBOX)]
        metadata_pointer_table_: [*mut crate::sandbox::sandbox::MemoryChunkMetadata; MemoryChunkConstants::kMetadataPointerTableSize],
        #[cfg(V8_ENABLE_SANDBOX)]
        backend_allocator_: SandboxedArrayBufferAllocator,

        #[cfg(V8_ENABLE_LEAPTIERING)]
        js_dispatch_table_: JSDispatchTable,
    }

    impl IsolateGroup {
        pub fn AcquireDefault() -> *mut IsolateGroup {
            Self::GetDefault().Acquire()
        }

        pub const fn CanCreateNewGroups() -> bool {
            COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL
        }

        pub fn New() -> *mut IsolateGroup {
            if !Self::CanCreateNewGroups() {
                panic!(
                    "Creation of new isolate groups requires enabling \
                     multiple pointer compression cages at build-time"
                );
            }

            let group = Box::new(IsolateGroup::default());
            let group_ptr = Box::into_raw(group);
            #[cfg(V8_ENABLE_SANDBOX)]
            let sandbox = Sandbox::New(GetPlatformVirtualAddressSpace());
            #[cfg(V8_ENABLE_SANDBOX)]
            unsafe {(*group_ptr).Initialize(false, sandbox)};
            #[cfg(not(V8_ENABLE_SANDBOX))]
            unsafe {(*group_ptr).Initialize(false)};
            unsafe {
                assert!(((*group_ptr).page_allocator_ as *const _).is_null() == false);
            }

            let group_access_scope = IsolateGroupAccessScope::new(group_ptr);
            unsafe {
                ExternalReferenceTable::InitializeOncePerIsolateGroup(
                    (*group_ptr).external_ref_table_.as_mut_ptr(),
                );
            }
            group_access_scope.exit();

            group_ptr
        }

        pub fn InitializeOncePerProcess() {
            unsafe {
                assert!((Self::default_isolate_group_ as *const _).is_null());
            }
            Self::default_isolate_group_ = Box::into_raw(Box::new(IsolateGroup::default()));
            let group = Self::GetDefault();

            unsafe {
                assert!(((*group).page_allocator_ as *const _).is_null());
            }
            #[cfg(V8_ENABLE_SANDBOX)]
            unsafe {(*group).Initialize(true, Sandbox::GetDefault())};
            #[cfg(not(V8_ENABLE_SANDBOX))]
            unsafe {(*group).Initialize(true)};
            unsafe {
                assert!(((*group).page_allocator_ as *const _).is_null() == false);
            }

            #[cfg(V8_COMPRESS_POINTERS)]
            unsafe {
                V8HeapCompressionScheme::InitBase((*group).GetPtrComprCageBase());
            }
            #[cfg(V8_EXTERNAL_CODE_SPACE)]
            unsafe {
                ExternalCodeCompressionScheme::InitBase(V8HeapCompressionScheme::base());
            }
            #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
            {
                Self::set_current(group);
            }
        }

        pub fn TearDownOncePerProcess() {
            Self::ReleaseDefault();
        }

        pub fn Acquire(self_: *mut Self) -> *mut IsolateGroup {
            unsafe {
                assert!((*self_).reference_count_.load(Ordering::Relaxed) > 0);
                (*self_).reference_count_.fetch_add(1, Ordering::Relaxed);
            }
            self_
        }

        pub fn Release(self_: *mut Self) {
            unsafe {
                assert!((*self_).reference_count_.load(Ordering::Relaxed) > 0);

                if (*self_).reference_count_.fetch_sub(1, Ordering::Relaxed) == 1 {
                    drop(Box::from_raw(self_));
                }
            }
        }

        pub fn page_allocator(self_: *const Self) -> *mut PageAllocator {
            unsafe { (*self_).page_allocator_ }
        }

        #[cfg(V8_COMPRESS_POINTERS)]
        pub fn GetPtrComprCage(self_: *const Self) -> *mut VirtualMemoryCage {
            unsafe { (*self_).pointer_compression_cage_ }
        }

        #[cfg(V8_COMPRESS_POINTERS)]
        pub fn GetTrustedPtrComprCage(self_: *const Self) -> *mut VirtualMemoryCage {
            unsafe { (*self_).trusted_pointer_compression_cage_ }
        }

        #[cfg(V8_COMPRESS_POINTERS)]
        pub fn GetPtrComprCageBase(self_: *const Self) -> Address {
            unsafe { (*self_).GetPtrComprCageBase() }
        }

        #[cfg(V8_COMPRESS_POINTERS)]
        pub fn GetTrustedPtrComprCageBase(self_: *const Self) -> Address {
            unsafe { (*self_).GetTrustedPtrComprCageBase() }
        }

        pub fn EnsureCodeRange(self_: *mut Self, requested_size: usize) -> *mut CodeRange {
            unsafe {
                let data = InitCodeRangeData {
                    code_range_member: &mut (*self_).code_range_,
                    page_allocator: (*self_).page_allocator_,
                    requested_size,
                    process_wide: (*self_).process_wide_,
                };
                (*self_).init_code_range_.call_once(init_code_range_once, data);
                match &(*self_).code_range_ {
                    Some(code_range) => Box::into_raw(Box::new(*code_range.clone())),
                    None => std::ptr::null_mut(), // Or handle the None case appropriately
                }
            }
        }

        pub fn GetCodeRange(self_: *const Self) -> *mut CodeRange {
            unsafe {
                match &(*self_).code_range_ {
                    Some(code_range) => Box::into_raw(Box::new(*code_range.clone())),
                    None => std::ptr::null_mut(), // Or handle the None case appropriately
                }
            }
        }

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        pub fn current() -> *mut IsolateGroup {
            Self::current_non_inlined()
        }

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        pub fn set_current(group: *mut IsolateGroup) {
            Self::set_current_non_inlined(group);
        }

        #[cfg(not(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES))]
        pub fn current() -> *mut IsolateGroup {
            Self::GetDefault()
        }

        pub fn external_ref_table(self_: *mut Self) -> MemorySpan<Address> {
            unsafe { MemorySpan::new((*self_).external_ref_table_.as_mut_ptr(), ExternalReferenceTable::kSizeIsolateIndependent) }
        }

        pub fn has_shared_space_isolate(self_: *const Self) -> bool {
            unsafe { (*self_).shared_space_isolate_.is_null() == false }
        }

        pub fn shared_space_isolate(self_: *const Self) -> *mut Isolate {
            unsafe { (*self_).shared_space_isolate_ }
        }

        pub fn init_shared_space_isolate(self_: *mut Self, isolate: *mut Isolate) {
            unsafe {
                assert!((*self_).has_shared_space_isolate() == false);
                (*self_).shared_space_isolate_ = isolate;
            }
        }

        pub fn optimizing_compile_task_executor(self_: *mut Self) -> *mut OptimizingCompileTaskExecutor {
            unsafe {
                match &mut (*self_).optimizing_compile_task_executor_ {
                    Some(executor) => executor.as_mut(),
                    None => std::ptr::null_mut(), // Handle the None case appropriately
                }
            }
        }

        pub fn shared_read_only_heap(self_: *const Self) -> *mut ReadOnlyHeap {
            unsafe { (*self_).shared_read_only_heap_ }
        }

        pub fn set_shared_read_only_heap(self_: *mut Self, heap: *mut ReadOnlyHeap) {
            unsafe { (*self_).shared_read_only_heap_ = heap; }
        }

        pub fn mutex(self_: *mut Self) -> *mut StdMutex<()> {
            unsafe { &mut (*self_).mutex_ as *mut _ }
        }

        pub fn read_only_artifacts(self_: *mut Self) -> *mut ReadOnlyArtifacts {
            unsafe {
                match &mut (*self_).read_only_artifacts_ {
                    Some(artifacts) => artifacts.as_mut(),
                    None => std::ptr::null_mut(),
                }
            }
        }

        pub fn InitializeReadOnlyArtifacts(self_: *mut Self) -> *mut ReadOnlyArtifacts {
            unsafe {
                let _guard = (*self_).mutex_.lock().unwrap();
                assert!((*self_).read_only_artifacts_.is_none());
                (*self_).read_only_artifacts_ = Some(Box::new(ReadOnlyArtifacts::default()));

                match &mut (*self_).read_only_artifacts_ {
                    Some(artifacts) => artifacts.as_mut(),
                    None => std::ptr::null_mut(),
                }
            }
        }

        pub fn GetBackingStorePageAllocator(self_: *mut Self) -> *mut PageAllocator {
            #[cfg(V8_ENABLE_SANDBOX)]
            unsafe {
                return (*self_).sandbox().page_allocator();
            }
            #[cfg(not(V8_ENABLE_SANDBOX))]
            {
                return GetPlatformPageAllocator();
            }
        }

        #[cfg(V8_ENABLE_SANDBOX)]
        pub fn sandbox(self_: *mut Self) -> *mut Sandbox {
            unsafe { (*self_).sandbox_ }
        }

        #[cfg(V8_ENABLE_SANDBOX)]
        pub fn code_pointer_table(self_: *mut Self) -> *mut CodePointerTable {
            unsafe { &mut (*self_).code_pointer_table_ as *mut CodePointerTable }
        }

        #[cfg(V8_ENABLE_SANDBOX)]
        pub fn metadata_pointer_table(self_: *mut Self) -> *mut [*mut crate::sandbox::sandbox::MemoryChunkMetadata; MemoryChunkConstants::kMetadataPointerTableSize] {
            unsafe { &mut (*self_).metadata_pointer_table_ as *mut _ }
        }

        #[cfg(V8_ENABLE_SANDBOX)]
        pub fn GetSandboxedArrayBufferAllocator(self_: *mut Self) -> *mut SandboxedArrayBufferAllocator {
            unsafe {
                (*self_).backend_allocator_.LazyInitialize((*self_).sandbox());
                &mut (*self_).backend_allocator_ as *mut SandboxedArrayBufferAllocator
            }
        }

        #[cfg(V8_ENABLE_LEAPTIERING)]
        pub fn js_dispatch_table(self_: *mut Self) -> *mut JSDispatchTable {
            unsafe { &mut (*self_).js_dispatch_table_ as *mut JSDispatchTable }
        }

        pub fn SetupReadOnlyHeap(self_: *mut Self, isolate: *mut Isolate, read_only_snapshot_data: *mut SnapshotData, can_rehash: bool) {
            unsafe {
                assert_eq!((*isolate).isolate_group(), self_);
                let _guard = (*self_).mutex_.lock().unwrap();
                ReadOnlyHeap::SetUp(isolate, read_only_snapshot_data, can_rehash);
            }
        }

        pub fn AddIsolate(self_: *mut Self, isolate: *mut Isolate) {
            unsafe {
                assert_eq!((*isolate).isolate_group(), self_);
                let _guard = (*self_).mutex_.lock().unwrap();
                (*self_).isolate_count_ += 1;

                if let Some(executor) = &mut (*self_).optimizing_compile_task_executor_ {
                    executor.EnsureInitialized();
                }

                if v8_flags.shared_heap {
                    if (*self_).has_shared_space_isolate() {
                        (*isolate).owns_shareable_data_ = false;
                    } else {
                        (*self_).init_shared_space_isolate(isolate);
                        (*isolate).is_shared_space_isolate_ = true;
                        assert!((*isolate).owns_shareable_data_);
                    }
                }
            }
        }

        pub fn RemoveIsolate(self_: *mut Self, isolate: *mut Isolate) {
            unsafe {
                let _guard = (*self_).mutex_.lock().unwrap();

                (*self_).isolate_count_ -= 1;

                if (*self_).isolate_count_ == 0 {
                    (*self_).read_only_artifacts_ = None;

                    assert_eq!((*self_).has_shared_space_isolate(), (*isolate).is_shared_space_isolate());

                    if (*isolate).is_shared_space_isolate() {
                        assert_eq!(isolate, (*self_).shared_space_isolate_);
                        (*self_).shared_space_isolate_ = std::ptr::null_mut();
                    }
                } else {
                    assert!(!(*isolate).is_shared_space_isolate());
                }
            }
        }

        pub fn GetDefault() -> *mut IsolateGroup {
            unsafe { Self::default_isolate_group_ }
        }
    }

    impl IsolateGroup {
        fn default() -> Self {
            IsolateGroup {
                reference_count_: AtomicI32::new(1),
                isolate_count_: 0,
                page_allocator_: std::ptr::null_mut(),
                #[cfg(V8_COMPRESS_POINTERS)]
                trusted_pointer_compression_cage_: std::ptr::null_mut(),
                #[cfg(V8_COMPRESS_POINTERS)]
                pointer_compression_cage_: std::ptr::null_mut(),
                #[cfg(V8_COMPRESS_POINTERS)]
                reservation_: VirtualMemoryCage::default(),
                #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
                current_: None,
                init_code_range_: OnceType::new(),
                code_range_: None,
                external_ref_table_: [0; ExternalReferenceTable::kSizeIsolateIndependent],
                process_wide_: false,
                mutex_: StdMutex::new(()),
                read_only_artifacts_: None,
                shared_read_only_heap_: std::ptr::null_mut(),
                shared_space_isolate_: std::ptr::null_mut(),
                optimizing_compile_task_executor_: None,
                #[cfg(V8_ENABLE_SANDBOX)]
                sandbox_: std::ptr::null_mut(),
                #[cfg(V8_ENABLE_SANDBOX)]
                code_pointer_table_: CodePointerTable::default(),
                #[cfg(V8_ENABLE_SANDBOX)]
                metadata_pointer_table_: [std::ptr::null_mut(); MemoryChunkConstants::kMetadataPointerTableSize],
                #[cfg(V8_ENABLE_SANDBOX)]
                backend_allocator_: SandboxedArrayBufferAllocator::default(),
                #[cfg(V8_ENABLE_LEAPTIERING)]
                js_dispatch_table_: JSDispatchTable::default(),
            }
        }

        fn ReleaseDefault() {
            let group = Self::GetDefault();
            unsafe {
                assert_eq!((*group).reference_count_.load(Ordering::Relaxed), 1);
                assert!((*group).has_shared_space_isolate() == false);
                Self::Release(group);
                Self::default_isolate_group_ = std::ptr::null_mut();
            }
        }

        #[cfg(V8_ENABLE_SANDBOX)]
        unsafe fn Initialize(&mut self, process_wide: bool, sandbox: *mut Sandbox) {
            assert!((self.reservation_.IsReserved()) == false);
            assert!((*sandbox).is_initialized());
            self.process_wide_ = process_wide;
            let params = PtrComprCageReservationParams::default();
            let base = (*sandbox).address_space().AllocatePages(
                (*sandbox).base(),
                params.reservation_size,
                params.base_alignment,
                PagePermissions::kNoAccess,
            );
            assert_eq!((*sandbox).base(), base);
            let existing_reservation = base::AddressRegion::new(base, params.reservation_size);
            //params.page_allocator = sandbox.page_allocator();
            //if !self.reservation_.InitReservation(params, existing_reservation) {
            //    V8::FatalProcessOutOfMemory(
            //        nullptr,
            //        "Failed to reserve virtual memory for process-wide V8 "
            //        "pointer compression cage");
            //}
            self.page_allocator_ = (*sandbox).page_allocator();
            self.pointer_compression_cage_ = &mut self.reservation_ as *mut VirtualMemoryCage;
            self.trusted_pointer_compression_cage_ = TrustedRange::EnsureProcessWideTrustedRange(kMaximalTrustedRangeSize);
            self.sandbox_ = sandbox;

            (*self.code_pointer_table()).Initialize();
            self.optimizing_compile_task_executor_ = Some(Box::new(OptimizingCompileTaskExecutor::default()));

            #[cfg(V8_ENABLE_LEAPTIERING)]
            (*self.js_dispatch_table()).Initialize();
        }

        #[cfg(all(not(V8_ENABLE_SANDBOX), V8_COMPRESS_POINTERS))]
        unsafe fn Initialize(&mut self, process_wide: bool) {
            assert!((self.reservation_.IsReserved()) == false);
            self.process_wide_ = process_wide;
            let params = PtrComprCageReservationParams::default();
            //if !self.reservation_.InitReservation(params) {
            //    V8::FatalProcessOutOfMemory(
            //        nullptr,
            //        "Failed to reserve virtual memory for process-wide V8 "
            //        "pointer compression cage");
            //}
            self.page_allocator_ = self.reservation_.page_allocator();
            self.pointer_compression_cage_ = &mut self.reservation_ as *mut VirtualMemoryCage;
            self.trusted_pointer_compression_cage_ = &mut self.reservation_ as *mut VirtualMemoryCage;
            self.optimizing_compile_task_executor_ = Some(Box::new(OptimizingCompileTaskExecutor::default()));
            #[cfg(V8_ENABLE_LEAPTIERING)]
            (*self.js_dispatch_table()).Initialize();
        }

        #[cfg(all(not(V8_ENABLE_SANDBOX), not(V8_COMPRESS_POINTERS)))]
        unsafe fn Initialize(&mut self, process_wide: bool) {
            self.process_wide_ = process_wide;
            self.page_allocator_ = GetPlatformPageAllocator();
            self.optimizing_compile_task_executor_ = Some(Box::new(OptimizingCompileTaskExecutor::default()));
            #[cfg(V8_ENABLE_LEAPTIERING)]
            (*self.js_dispatch_table()).Initialize();
        }

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        fn current_non_inlined() -> *mut IsolateGroup {
            thread_local! {
                static CURRENT: Cell<Option<*mut IsolateGroup>> = Cell::new(None);
            }
            CURRENT.with(|c| c.get().unwrap_or(std::ptr::null_mut()))
        }

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        fn set_current_non_inlined(group: *mut IsolateGroup) {
            thread_local! {
                static CURRENT: Cell<Option<*mut IsolateGroup>> = Cell::new(None);
            }
            CURRENT.with(|c| c.set(Some(group)));
        }
    }

    impl Drop for IsolateGroup {
        fn drop(&mut self) {
            unsafe {
                assert_eq!(self.reference_count_.load(Ordering::Relaxed), 0);
                assert_eq!(self.isolate_count_, 0);

                #[cfg(V8_ENABLE_LEAPTIERING)]
                (*self.js_dispatch_table()).TearDown();

                #[cfg(V8_ENABLE_SANDBOX)]
                (*self.code_pointer_table()).TearDown();

                self.code_range_.take();

                #[cfg(V8_COMPRESS_POINTERS)]
                {
                    assert!((self.reservation_.IsReserved()));
                    self.reservation_.Free();
                }

                #[cfg(V8_ENABLE_SANDBOX)]
                (*self.sandbox_).TearDown();
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct MemorySpan<T> {
        data_: *mut T,
        length_: usize,
    }

    impl<T> MemorySpan<T> {
        pub fn new(data_: *mut T, length_: usize) -> Self {
            MemorySpan { data_, length_ }
        }

        pub fn data(&self) -> *mut T {
            self.data_
        }

        pub fn length(&self) -> usize {
            self.length_
        }
    }

    #[cfg(V8_COMPRESS_POINTERS)]
    #[derive(Default)]
    struct PtrComprCageReservationParams {
        page_allocator: *mut PageAllocator,
        reservation_size: usize,
        base_alignment: usize,
        page_size: usize,
        requested_start_hint: Address,
        permissions: PagePermissions,
        page_initialization_mode: base::PageInitializationMode,
        page_freeing_mode: base::PageFreeingMode,
    }

    #[cfg(V8_COMPRESS_POINTERS)]
    impl PtrComprCageReservationParams {
        fn default() -> Self {
            let page_allocator = GetPlatformPageAllocator();

            let reservation_size = kPtrComprCageReservationSize;
            let base_alignment = kPtrComprCageBaseAlignment;

            let page_size =
                RoundUp(1 << kPageSizeBits, page_allocator.AllocatePageSize());
            let requested_start_hint = RoundDown(
                page_allocator.GetRandomMmapAddr() as Address,
                base_alignment,
            );

            #[cfg(all(V8_OS_FUCHSIA, not(V8_EXTERNAL_CODE_SPACE)))]
            let permissions = PagePermissions::kNoAccessWillJitLater;
            #[cfg(not(all(V8_OS_FUCHSIA, not(V8_EXTERNAL_CODE_SPACE))))]
            let permissions = PagePermissions::kNoAccess;

            let page_initialization_mode =
                base::PageInitializationMode::kAllocatedPagesCanBeUninitialized;
            let page_freeing_mode = base::PageFreeingMode::kMakeInaccessible;

            PtrComprCageReservationParams {
                page_allocator,
                reservation_size,
                base_alignment,
                page_size,
                requested_start_hint,
                permissions,
                page_initialization_mode,
                page_freeing_mode,
            }
        }
    }

    struct InitCodeRangeData {
        code_range_member: &mut Option<Box<CodeRange>>,
        page_allocator: *mut PageAllocator,
        requested_size: usize,
        process_wide: bool,
    }

    unsafe extern "C" fn init_code_range_once(data: *mut std::ffi::c_void) {
        let data = data as *mut InitCodeRangeData;
        let code_range = CodeRange::new();
        let page_allocator = (*data).page_allocator;
        let requested_size = (*data).requested_size;
        let process_wide = (*data).process_wide;
        let immutable = process_wide;

        if code_range.InitReservation(page_allocator, requested_size, immutable) == false {
            V8::FatalProcessOutOfMemory(
                std::ptr::null_mut(), "Failed to reserve virtual memory for CodeRange");
        }

        (*data).code_range_member.replace(Box::new(code_range.clone()));
        #[cfg(V8_EXTERNAL_CODE_SPACE)]
        {
            #[cfg(V8_COMPRESS_POINTERS_IN_SHARED_CAGE)]
            ExternalCodeCompressionScheme::InitBase(
                ExternalCodeCompressionScheme::PrepareCageBaseAddress(code_range.base()));
        }
    }

    struct IsolateGroupAccessScope {
        previous_: *mut IsolateGroup,
    }

    impl IsolateGroupAccessScope {
        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        pub fn new(group: *mut IsolateGroup) -> Self {
            let previous_ = IsolateGroup::current();
            IsolateGroup::set_current(group);
            IsolateGroupAccessScope { previous_ }
        }

        #[cfg(not(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES))]
        pub fn new(group: *mut IsolateGroup) -> Self {
            IsolateGroupAccessScope { previous_: std::ptr::null_mut() }
        }

        #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
        pub fn exit(self) {
            IsolateGroup::set_current(self.previous_);
        }

        #[cfg(not(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES))]
        pub fn exit(self) {}
    }

    pub mod base {
        pub enum PageInitializationMode {
            kAllocatedPagesCanBeUninitialized,
        }

        pub enum PageFreeingMode {
            kMakeInaccessible,
        }

        #[derive(Clone, Copy)]
        pub struct AddressRegion {
            base_: usize,
            size_: usize,
        }

        impl AddressRegion {
            pub fn new(base_: usize, size_: usize) -> Self {
                AddressRegion { base_, size_ }
            }
        }
    }

    pub mod platform {
        pub mod memory {
            use crate::init::isolate_group::PageAllocator;

            pub fn GetPlatformPageAllocator() -> *mut PageAllocator {
                todo!()
            }

            pub fn RoundUp(size: usize, alignment: usize) -> usize {
                (size + alignment - 1) & !(alignment - 1)
            }

            pub fn RoundDown(size: usize, alignment: usize) -> usize {
                size & !(alignment - 1)
            }
        }
    }

    pub mod heap {
        pub mod trusted_range {
            use crate::init::isolate_group::VirtualMemoryCage;
            use crate::init::isolate_group::PageAllocator;

            pub fn EnsureProcessWideTrustedRange(size: usize) -> *mut VirtualMemoryCage {
                todo!()
            }
        }
    }

    pub mod flags {
        pub mod flags {
            pub const kMaximalTrustedRangeSize: usize = 0;
            pub const kPtrComprCageBaseAlignment: usize = 0;
            pub const kPtrComprCageReservationSize: usize = 0;
            pub const kPageSizeBits: usize = 0;

            pub struct Flags {
                pub shared_heap: bool,
            }

            impl Flags {
                pub fn new() -> Self {
                    Flags {
                        shared_heap: false,
                    }
                }
            }

            pub static v8_flags: Flags = Flags { shared_heap: false };
        }
    }

    pub mod sandbox {
        pub mod sandbox {
            pub struct Sandbox {
                is_initialized_: bool,
            }

            impl Sandbox {
                pub fn New(address_space: *mut VirtualAddressSpace) -> *mut Sandbox {
                    todo!()
                }

                pub fn is_initialized(&self) -> bool {
                    self.is_initialized_
                }

                pub fn base(&self) -> Address {
                    todo!()
                }

                pub fn GetDefault() -> *mut Sandbox {
                    todo!()
                }

                pub fn page_allocator(&self) -> *mut PageAllocator {
                    todo!()
                }

                pub fn address_space(&self) -> *mut VirtualAddressSpace {
                    todo!()
                }

                pub fn TearDown(&mut self) {
                    todo!()
                }
            }

            #[derive(Default)]
            pub struct MemoryChunkMetadata {}
        }

    }

    pub mod codegen {
        pub mod external_reference_table {
            pub struct ExternalReferenceTable {}

            impl ExternalReferenceTable {
                pub const kSizeIsolateIndependent: usize = 0;

                pub unsafe fn InitializeOncePerIsolateGroup(external_ref_table: *mut Address) {
                    todo!()
                }
            }
        }
    }
    pub mod utils{
        pub mod allocation {
            pub fn IsAligned(address: Address, chunk_size: usize) -> bool {
                todo!()
            }
        }
    }

    pub struct VirtualMemoryCage {
        reserved: bool,
    }

    impl VirtualMemoryCage {
        pub fn IsReserved(&self) -> bool {
            self.reserved
        }

        pub fn Free(&mut self) {
            self.reserved = false;
        }

        pub fn InitReservation(&mut self, params: PtrComprCageReservationParams) -> bool {
            self.reserved = true;
            true
        }

        pub fn page_allocator(&self) -> *mut PageAllocator {
            todo!()
        }

        pub fn base(&self) -> Address {
            todo!()
        }

        pub fn InitReservation(&mut self, params: PtrComprCageReservationParams, existing_reservation: base::AddressRegion) -> bool {
            self.reserved
