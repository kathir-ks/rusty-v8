// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::{max, min};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, Once};
use std::{mem, ptr, slice};
use v8_flags as flags;
use v8_globals as globals;

//use crate::base::bits; // Assuming base::bits module exists in Rust
use crate::common::globals as common_globals;
//use crate::diagnostics::unwinding_info_win64; // Assuming diagnostics module exists in Rust
//use crate::utils::allocation; // Assuming utils::allocation module exists in Rust
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicPtr, Ordering};

mod v8_flags {
    pub static trace_code_range_allocation: bool = false;
    pub static jitless: bool = false;
    pub static better_code_range_allocation: bool = false;
    pub static abort_on_far_code_range: bool = false;
}

mod v8_globals {
    pub const V8_ENABLE_NEAR_CODE_RANGE_BOOL: bool = true;
    pub const V8_EXTERNAL_CODE_SPACE_BOOL: bool = false;
    pub const COMPRESS_POINTERS_IN_SHARED_CAGE_BOOL: bool = false;
    pub const kMaxPCRelativeCodeRangeInMB: usize = 2048;
    pub const kPlatformRequiresCodeRange: bool = false;
    pub const kMinimumCodeRangeSize: usize = 4096;
    pub const kMaximalCodeRangeSize: usize = 2147483648; // 2GB
    pub const kReservedCodeRangePages: usize = 2;
}

mod common {
    pub mod globals {
        pub type Address = usize;
        pub const kNullAddress: Address = 0;
    }
}

mod base {
    pub mod bits {
        pub fn round_up_to_power_of_two(x: usize) -> usize {
            let mut p = 1;
            while p < x {
                p <<= 1;
            }
            p
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct AddressRegion {
        begin: usize,
        size: usize,
    }

    impl AddressRegion {
        pub fn new(begin: usize, size: usize) -> Self {
            AddressRegion { begin, size }
        }

        pub fn begin(&self) -> usize {
            self.begin
        }

        pub fn end(&self) -> usize {
            self.begin + self.size
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        pub fn contains(&self, address: usize) -> bool {
            address >= self.begin && address < self.end()
        }

        pub fn contains_range(&self, start: usize, size: usize) -> bool {
            start >= self.begin && (start + size) <= self.end()
        }

        pub fn contains_region(&self, other: AddressRegion) -> bool {
            self.contains_range(other.begin, other.size)
        }
    }

    pub mod os {
        #[derive(Debug, Copy, Clone)]
        pub struct MemoryRange {
            pub start: usize,
            pub size: usize,
        }
        
        pub fn get_first_free_memory_range_within(
            start: usize,
            end: usize,
            requested_size: usize,
            alignment: usize,
        ) -> Option<MemoryRange> {
            // Placeholder implementation.  In a real implementation, this would
            // query the OS for free memory ranges. This mock version always fails to find a range.
            None
        }

        pub enum MemoryPermission {
            kReadWrite,
            kReadExecute,
            kNoAccess,
        }

        pub fn remap_pages(
            _src: *const u8,
            _size: usize,
            _dest: *mut u8,
            _permissions: MemoryPermission,
        ) -> bool {
            // Placeholder implementation - always fails
            false
        }

        pub fn is_remap_page_supported() -> bool {
            false
        }
    }
}

mod codegen {
    pub mod constants_arch {
        // Define any architecture-specific constants here
    }
}

mod src {
    pub mod heap {
        pub mod heap_inl {} // Assuming heap_inl does not need any direct translation
    }
}

fn is_aligned(address: usize, alignment: usize) -> bool {
    address % alignment == 0
}

fn round_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 {
        return value;
    }
    let remainder = value % alignment;
    if remainder == 0 {
        value
    } else {
        value + alignment - remainder
    }
}

fn round_down(value: usize, alignment: usize) -> usize {
    value - (value % alignment)
}

fn function_addr<T>(_: T) -> usize {
    unimplemented!() // Placeholder: Cannot directly get address of function in Rust
}

// Stubbed functions
struct Isolate {}
impl Isolate {
    fn get_short_builtins_call_region() -> base::AddressRegion {
        base::AddressRegion::new(0, 0) // Stubbed
    }

    fn current_embedded_blob_code() -> *const u8 {
        std::ptr::null() // Stubbed
    }

    fn current_embedded_blob_code_size() -> usize {
        0 // Stubbed
    }
}

struct MutablePageMetadata {}
impl MutablePageMetadata {
    const kPageSize: usize = 4096;
}

struct PageAllocator {
    base: usize,
    size: usize,
}

impl PageAllocator {
    fn allocate_page_size(&self) -> usize {
        4096
    }

    fn commit_page_size(&self) -> usize {
        4096
    }

    fn begin(&self) -> usize {
        self.base
    }

    fn size(&self) -> usize {
        self.size
    }

    fn allocate_pages(
        &self,
        hint: *mut std::ffi::c_void,
        size: usize,
        alignment: usize,
        permissions: Permission,
    ) -> *mut std::ffi::c_void {
        // Stubbed allocation
        hint
    }

    fn allocate_pages_at(&self, address: usize, size: usize, permissions: Permission) -> bool {
        true // Stubbed allocation
    }

    fn set_permissions(&self, address: *mut std::ffi::c_void, size: usize, permissions: Permission) -> bool {
        true // Stubbed
    }

    fn recommit_pages(&self, address: *mut std::ffi::c_void, size: usize, permissions: Permission) -> bool {
        true // Stubbed
    }

    fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
        true // Stubbed
    }

    fn seal_pages(&self, address: *mut std::ffi::c_void, size: usize) {
        // Stubbed
    }
}

#[derive(Copy, Clone)]
enum Permission {
    kNoAccess,
    kReadWrite,
    kReadWriteExecute,
    kNoAccessWillJitLater
}

fn get_platform_page_allocator() -> &'static PageAllocator {
    unimplemented!() // Placeholder
}

struct VirtualMemoryCage {
    reservation: Option<Box<Reservation>>,
}

struct Reservation {
    region: base::AddressRegion,
    page_allocator: Box<PageAllocator>
}

impl VirtualMemoryCage {
    fn new() -> Self {
        VirtualMemoryCage { reservation: None }
    }

    fn init_reservation(&mut self, params: ReservationParams) -> bool {
        if self.is_reserved() {
            self.free();
        }

        //let reservation = Box::new(Reservation { region: base::AddressRegion::new(params.requested_start_hint, params.reservation_size) } );

        let mut allocator = Box::new(PageAllocator{ base: params.requested_start_hint, size: params.reservation_size });

        if params.page_allocator.allocate_pages_at(params.requested_start_hint, MutablePageMetadata::kPageSize, Permission::kNoAccess) {
            if allocator.set_permissions(params.requested_start_hint as *mut std::ffi::c_void, params.reservation_size, params.permissions) {
                self.reservation = Some(Box::new(Reservation { region: base::AddressRegion::new(params.requested_start_hint, params.reservation_size), page_allocator: allocator }));
                return true;
            }
        }

        false
    }

    fn is_reserved(&self) -> bool {
        self.reservation.is_some()
    }

    fn region(&self) -> base::AddressRegion {
        match &self.reservation {
            Some(reservation) => reservation.region,
            None => base::AddressRegion::new(0,0),
        }
    }

    fn base(&self) -> usize {
        match &self.reservation {
            Some(reservation) => reservation.region.begin,
            None => 0,
        }
    }

    fn size(&self) -> usize {
        match &self.reservation {
            Some(reservation) => reservation.region.size,
            None => 0,
        }
    }

    fn free(&mut self) {
        self.reservation = None;
    }

    fn reservation(&self) -> &Reservation {
        match &self.reservation {
            Some(reservation) => reservation.as_ref(),
            None => panic!("No reservation"),
        }
    }
}

struct ReservationParams {
    page_allocator: &'static PageAllocator,
    reservation_size: usize,
    page_size: usize,
    permissions: Permission,
    page_initialization_mode: PageInitializationMode,
    page_freeing_mode: PageFreeingMode,
    base_alignment: usize,
    requested_start_hint: usize,
}

impl Default for ReservationParams {
    fn default() -> Self {
        ReservationParams {
            page_allocator: get_platform_page_allocator(), // stub
            reservation_size: 0,
            page_size: 4096,
            permissions: Permission::kNoAccess,
            page_initialization_mode: PageInitializationMode::kAllocatedPagesCanBeUninitialized,
            page_freeing_mode: PageFreeingMode::kMakeInaccessible,
            base_alignment: 4096,
            requested_start_hint: 0,
        }
    }
}

#[derive(Copy, Clone)]
enum PageInitializationMode {
    kAllocatedPagesCanBeUninitialized,
    kRecommitOnly,
}

#[derive(Copy, Clone)]
enum PageFreeingMode {
    kMakeInaccessible,
    kDiscard,
}

lazy_static! {
    static ref CODE_RANGE_ADDRESS_HINT: CodeRangeAddressHint = CodeRangeAddressHint::new();
}

struct CodeRangeAddressHint {
    mutex: Mutex<()>,
    recently_freed_: HashMap<usize, Vec<usize>>, // size, vec<address>
}

impl CodeRangeAddressHint {
    fn new() -> Self {
        CodeRangeAddressHint {
            mutex: Mutex::new(()),
            recently_freed_: HashMap::new(),
        }
    }

    fn get_address_hint(code_range_size: usize, alignment: usize) -> usize {
        let guard = CODE_RANGE_ADDRESS_HINT.mutex.lock().unwrap();

        // Try to allocate code range in the preferred region where we can use
        // short instructions for calling/jumping to embedded builtins.
        let preferred_region = Isolate::get_short_builtins_call_region();

        let mut result = 0;
        let it = CODE_RANGE_ADDRESS_HINT.recently_freed_.get(&code_range_size);
        // No recently freed region has been found, try to provide a hint for placing
        // a code region.
        if it.is_none() || it.unwrap().is_empty() {
            if globals::V8_ENABLE_NEAR_CODE_RANGE_BOOL && !preferred_region.is_empty() {
                let memory_ranges = base::os::get_first_free_memory_range_within(
                    preferred_region.begin(),
                    preferred_region.end(),
                    code_range_size,
                    alignment,
                );
                if let Some(memory_ranges) = memory_ranges {
                    result = memory_ranges.start;
                    assert!(is_aligned(result, alignment));
                    return result;
                }
                // The empty memory_ranges means that GetFirstFreeMemoryRangeWithin() API
                // is not supported, so use the lowest address from the preferred region
                // as a hint because it'll be at least as good as the fallback hint but
                // with a higher chances to point to the free address space range.
                return round_up(preferred_region.begin(), alignment);
            }
            return round_up(function_addr(|| {}), alignment);
        }

        // Try to reuse near code range first.
        if globals::V8_ENABLE_NEAR_CODE_RANGE_BOOL && !preferred_region.is_empty() {
            let freed_regions_for_size = it.unwrap();
            for code_range_start in freed_regions_for_size.iter().rev() {
                if preferred_region.contains_range(*code_range_start, code_range_size) {
                    assert!(is_aligned(*code_range_start, alignment));
                    let mut mutable_freed = CODE_RANGE_ADDRESS_HINT.recently_freed_.get_mut(&code_range_size).unwrap();
                    let index = mutable_freed.iter().position(|&x| x == *code_range_start).unwrap();
                    mutable_freed.remove(index);
                    return *code_range_start;
                }
            }
        }

        let freed_regions_for_size = it.unwrap();

        result = *freed_regions_for_size.last().unwrap();

        let mut mutable_freed = CODE_RANGE_ADDRESS_HINT.recently_freed_.get_mut(&code_range_size).unwrap();
        mutable_freed.pop();

        assert!(is_aligned(result, alignment));
        return result;
    }

    fn notify_freed_code_range(code_range_start: usize, code_range_size: usize) {
        let mut guard = CODE_RANGE_ADDRESS_HINT.mutex.lock().unwrap();
        CODE_RANGE_ADDRESS_HINT
            .recently_freed_
            .entry(code_range_size)
            .or_insert_with(Vec::new)
            .push(code_range_start);
    }
}

struct CodeRange {
    cage: VirtualMemoryCage,
    immutable_: bool,
    embedded_blob_code_copy_: AtomicPtr<u8>,
    remap_embedded_builtins_mutex_: Mutex<()>,
}

impl CodeRange {
    fn new() -> Self {
        CodeRange {
            cage: VirtualMemoryCage::new(),
            immutable_: false,
            embedded_blob_code_copy_: AtomicPtr::new(ptr::null_mut()),
            remap_embedded_builtins_mutex_: Mutex::new(()),
        }
    }

    fn get_writable_reserved_area_size() -> usize {
        globals::kReservedCodeRangePages * 4096 //MemoryAllocator::GetCommitPageSize()
    }

    fn init_reservation(&mut self, page_allocator: &PageAllocator, requested: usize, immutable: bool) -> bool {
        if requested == 0 {
            panic!("requested cannot be zero");
        }
        let mut page_allocator = page_allocator;
        if globals::V8_EXTERNAL_CODE_SPACE_BOOL {
            page_allocator = get_platform_page_allocator();
        }

        let mut requested = requested;
        if requested <= globals::kMinimumCodeRangeSize {
            requested = globals::kMinimumCodeRangeSize;
        }

        let k_page_size = MutablePageMetadata::kPageSize;
        assert!(is_aligned(k_page_size, page_allocator.allocate_page_size()));

        // When V8_EXTERNAL_CODE_SPACE_BOOL is enabled the allocatable region must
        // not cross the 4Gb boundary and thus the default compression scheme of
        // truncating the InstructionStream pointers to 32-bits still works. It's
        // achieved by specifying base_alignment parameter.
        let base_alignment = if globals::V8_EXTERNAL_CODE_SPACE_BOOL {
            base::bits::round_up_to_power_of_two(requested)
        } else {
            k_page_size
        };

        assert!(!globals::kPlatformRequiresCodeRange || requested <= globals::kMaximalCodeRangeSize);

        let mut params = ReservationParams::default();
        params.page_allocator = page_allocator;
        params.reservation_size = requested;
        params.page_size = k_page_size;

        if flags::jitless {
            params.permissions = Permission::kNoAccess;
            params.page_initialization_mode =
                PageInitializationMode::kAllocatedPagesCanBeUninitialized;
            params.page_freeing_mode = PageFreeingMode::kMakeInaccessible;
        } else {
            params.permissions = Permission::kNoAccessWillJitLater;
            params.page_initialization_mode = PageInitializationMode::kRecommitOnly;
            params.page_freeing_mode = PageFreeingMode::kDiscard;
        }

        let allocate_page_size = page_allocator.allocate_page_size();
        let k_radius_in_mb = if globals::kMaxPCRelativeCodeRangeInMB > 1024 {
            globals::kMaxPCRelativeCodeRangeInMB
        } else {
            4096
        };
        let preferred_region = Self::get_preferred_region(k_radius_in_mb, k_page_size);

        if flags::trace_code_range_allocation {
            println!(
                "=== Preferred region: [{:p}, {:p})",
                preferred_region.begin() as *const u8,
                preferred_region.end() as *const u8
            );
        }

        // For configurations with enabled pointer compression and shared external
        // code range we can afford trying harder to allocate code range near .text
        // section.
        let k_should_try_harder = globals::V8_EXTERNAL_CODE_SPACE_BOOL
            && globals::COMPRESS_POINTERS_IN_SHARED_CAGE_BOOL
            && flags::better_code_range_allocation;

        if k_should_try_harder {
            // Relax alignment requirement while trying to allocate code range inside
            // preferred region.
            params.base_alignment = k_page_size;

            // TODO(v8:11880): consider using base::OS::GetFirstFreeMemoryRangeWithin()
            // to avoid attempts that's going to fail anyway.

            let mut candidate_cage = VirtualMemoryCage::new();

            // Try to allocate code range at the end of preferred region, by going
            // towards the start in steps.
            let k_allocation_tries = 16;
            params.requested_start_hint =
                round_down(preferred_region.end() - requested, k_page_size);
            let step = round_down(preferred_region.size() / k_allocation_tries, k_page_size);
            for i in 0..k_allocation_tries {
                if flags::trace_code_range_allocation {
                    println!(
                        "=== Attempt #{}, hint={:p}",
                        i,
                        params.requested_start_hint as *const u8
                    );
                }
                if candidate_cage.init_reservation(params) {
                    if flags::trace_code_range_allocation {
                        println!(
                            "=== Attempt #{} ({:p}): [{:p}, {:p})",
                            i,
                            params.requested_start_hint as *const u8,
                            candidate_cage.region().begin() as *const u8,
                            candidate_cage.region().end() as *const u8
                        );
                    }
                    // Allocation succeeded, check if it's in the preferred range.
                    if preferred_region.contains_region(candidate_cage.region()) {
                        break;
                    }
                    // This allocation is not the one we are searhing for.
                    candidate_cage.free();
                }
                if step == 0 {
                    break;
                }
                params.requested_start_hint -= step;
            }
            if candidate_cage.is_reserved() {
                mem::swap(&mut self.cage, &mut candidate_cage);
            }
        }

        if !self.cage.is_reserved() {
            // TODO(v8:11880): Use base_alignment here once ChromeOS issue is fixed.
            let the_hint = CodeRangeAddressHint::get_address_hint(
                requested,
                allocate_page_size,
            );
            let the_hint = round_down(the_hint, base_alignment);
            // Last resort, use whatever region we get.
            params.base_alignment = base_alignment;
            params.requested_start_hint = the_hint;
            if !self.cage.init_reservation(params) {
                params.requested_start_hint = common_globals::kNullAddress;
                if !self.cage.init_reservation(params) {
                    return false;
                }
            }
            if flags::trace_code_range_allocation {
                println!(
                    "=== Fallback attempt, hint={:p}: [{:p}, {:p})",
                    params.requested_start_hint as *const u8,
                    self.cage.region().begin() as *const u8,
                    self.cage.region().end() as *const u8
                );
            }
        }

        if flags::abort_on_far_code_range && !preferred_region.contains_region(self.cage.region()) {
            // We didn't manage to allocate the code range close enough.
            panic!("Failed to allocate code range close to the .text section");
        }

        // On some platforms, specifically Win64, we need to reserve some pages at
        // the beginning of an executable space. See
        //   https://cs.chromium.org/chromium/src/components/crash/content/
        //     app/crashpad_win.cc?rcl=fd680447881449fba2edcf0589320e7253719212&l=204
        // for details.
        let reserved_area = Self::get_writable_reserved_area_size();
        if reserved_area > 0 {
            assert!(reserved_area <= k_page_size);
            // Exclude the reserved area from further allocations.
            assert!(params.page_allocator.allocate_pages_at(
                self.cage.base(),
                k_page_size,
                Permission::kNoAccess
            ));
            // Commit required amount of writable memory.
            if !self.cage.reservation().page_allocator.set_permissions(
                self.cage.base() as *mut std::ffi::c_void,
                reserved_area,
                Permission::kReadWrite,
            ) {
                return false;
            }
            // #if defined(V8_OS_WIN64)
            // if (win64_unwindinfo::CanRegisterUnwindInfoForNonABICompliantCodeRange()) {
            // win64_unwindinfo::RegisterNonABICompliantCodeRange(
            // reinterpret_cast<void*>(base()), size());
            // }
            // #endif  // V8_OS_WIN64
        }

        // Don't pre-commit the code cage on Windows since it uses memory and it's not
        // required for recommit.
        // iOS cannot adjust page permissions for MAP_JIT'd pages, they are set as RWX
        // at the start.
        // #if !defined(V8_OS_WIN) && !defined(V8_OS_IOS)
        if params.page_initialization_mode == PageInitializationMode::kRecommitOnly {
            let base = (page_allocator.begin() + reserved_area) as *mut std::ffi::c_void;
            let size = page_allocator.size() - reserved_area;
            // if ThreadIsolation::Enabled() {
            // if (!ThreadIsolation::MakeExecutable(reinterpret_cast<Address>(base),
            // size)) {
            // return false;
            // }
            // } else
            if !params.page_allocator.set_permissions(base, size, Permission::kReadWriteExecute) {
                return false;
            }
            if immutable {
                // #ifdef DEBUG
                self.immutable_ = true;
                // #endif
                // #ifdef V8_ENABLE_MEMORY_SEALING
                params.page_allocator.seal_pages(base, size);
                // #endif
            }
            // DiscardSealedMemoryScope discard_scope("Discard global code range.");
            if !params.page_allocator.discard_system_pages(base, size) {
                return false;
            }
        }
        // #endif  // !defined(V8_OS_WIN)

        return true;
    }

    fn get_preferred_region(
        radius_in_megabytes: usize,
        allocate_page_size: usize,
    ) -> base::AddressRegion {
        // #ifdef V8_TARGET_ARCH_64_BIT
        // Compute builtins location.
        let embedded_blob_code_start = Isolate::current_embedded_blob_code() as usize;
        let embedded_blob_code_end;
        if embedded_blob_code_start == common_globals::kNullAddress {
            // When there's no embedded blob use address of a function from the binary
            // as an approximation.
            let fun_addr = || {};
            let function_address = function_addr(fun_addr);
            let embedded_blob_code_start = function_address;

            embedded_blob_code_end = embedded_blob_code_start + 1;
        } else {
            embedded_blob_code_end =
                embedded_blob_code_start + Isolate::current_embedded_blob_code_size();
        }

        // Fulfil requirement (a).
        let radius = radius_in_megabytes * 1024 * 1024;

        let mut region_start = round_up(embedded_blob_code_end - radius, allocate_page_size);
        if region_start > embedded_blob_code_end {
            // |region_start| underflowed.
            region_start = 0;
        }
        let mut region_end =
            round_down(embedded_blob_code_start + radius, allocate_page_size);
        if region_end < embedded_blob_code_start {
            // |region_end| overflowed.
            region_end = usize::MAX - (usize::MAX % allocate_page_size);
        }

        // Fulfil requirement (b).
        let k4gb = 4 * 1024 * 1024 * 1024;
        let four_gb_cage_start = round_down(embedded_blob_code_start, k4gb);
        let four_gb_cage_end = four_gb_cage_start + k4gb;

        region_start = max(region_start, four_gb_cage_start);
        region_end = min(region_end, four_gb_cage_end);

        // #ifdef V8_EXTERNAL_CODE_SPACE
        // If ExternalCodeCompressionScheme ever changes then the requirements might
        // need to be updated.
        // static_assert(k4GB <= kPtrComprCageReservationSize);
        // DCHECK_EQ(four_gb_cage_start,
        // ExternalCodeCompressionScheme::PrepareCageBaseAddress(
        // embedded_blob_code_start));
        // #endif  // V8_EXTERNAL_CODE_SPACE

        base::AddressRegion::new(region_start, region_end - region_start)
        // #else
        // return {};
        // #endif  // V8_TARGET_ARCH_64_BIT
    }

    fn free(&mut self) {
        if self.cage.is_reserved() {
            // #if defined(V8_OS_WIN64)
            // if (win64_unwindinfo::CanRegisterUnwindInfoForNonABICompliantCodeRange()) {
            // win64_unwindinfo::UnregisterNonABICompliantCodeRange(
            // reinterpret_cast<void*>(base()));
            // }
            // #endif  // V8_OS_WIN64
            CodeRangeAddressHint::notify_freed_code_range(
                self.cage.region().begin(),
                self.cage.region().size(),
            );
            self.cage.free();
        }
    }

    fn remap_embedded_builtins(
        &self,
        isolate: &Isolate,
        embedded_blob_code: *const u8,
        embedded_blob_code_size: usize,
    ) -> *mut u8 {
        let guard = self.remap_embedded_builtins_mutex_.lock().unwrap();

        // Remap embedded builtins into the end of the address range controlled by
        // the BoundedPageAllocator.
        let page_allocator = self.cage.reservation().page_allocator;
        let code_region = base::AddressRegion::new(page_allocator.begin(), page_allocator.size());

        assert_ne!(code_region.begin(), common_globals::kNullAddress);
        assert!(!code_region.is_empty());

        let embedded_blob_code_copy =
            self.embedded_blob_code_copy_.load(Ordering::Acquire);
        if !embedded_blob_code_copy.is_null() {
            assert!(code_region.contains_range(
                embedded_blob_code_copy as usize,
                embedded_blob_code_size,
            ));

            unsafe {
                let slice1 = slice::from_raw_parts(embedded_blob_code, embedded_blob_code_size);
                let slice2 = slice::from_raw_parts(embedded_blob_code_copy, embedded_blob_code_size);

                assert_eq!(slice1, slice2);
            }

            return embedded_blob_code_copy;
        }

        let k_allocate_page_size = page_allocator.allocate_page_size();
        let k_commit_page_size = page_allocator.commit_page_size();
        let allocate_code_size = round_up(embedded_blob_code_size, k_allocate_page_size);

        // Allocate the re-embedded code blob in such a way that it will be reachable
        // by PC-relative addressing from biggest possible region.
        let max_pc_relative_code_range =
            globals::kMaxPCRelativeCodeRangeInMB as usize * 1024 * 1024;
        let hint_offset = min(max_pc_relative_code_range, code_region.size()) - allocate_code_size;
        let hint = (code_region.begin() + hint_offset) as *mut std::ffi::c_void;

        let embedded_blob_code_copy = page_allocator.allocate_pages(
            hint,
            allocate_code_size,
            k_allocate_page_size,
            Permission::kNoAccessWillJitLater,
        ) as *mut u8;

        if embedded_blob_code_copy.is_null() {
            V8::fatal_process_out_of_memory(
                isolate,
                "Can't allocate space for re-embedded builtins",
            );
        }

        assert_eq!(embedded_blob_code_copy, hint as *mut u8);

        if code_region.size() > max_pc_relative_code_range {
            // The re-embedded code blob might not be reachable from the end part of
            // the code range, so ensure that code pages will never be allocated in
            // the "unreachable" area.
            let unreachable_start = embedded_blob_code_copy as usize + max_pc_relative_code_range;

            if code_region.contains(unreachable_start) {
                let unreachable_size = code_region.end() - unreachable_start;

                let result = page_allocator.allocate_pages(
                    unreachable_start as *mut std::ffi::c_void,
                    unreachable_size,
                    k_allocate_page_size,
                    Permission::kNoAccess,
                ) as *mut u8;
                assert_eq!(result as usize, unreachable_start);
            }
        }

        let code_size = round_up(embedded_blob_code_size, k_commit_page_size);
        
        //if constexpr (base::OS::IsRemapPageSupported()) {
        // By default, the embedded builtins are not remapped, but copied. This
        // costs memory, since builtins become private dirty anonymous memory,
        // rather than shared, clean, file-backed memory for the embedded version.
        // If the OS supports it, we can remap the builtins *on top* of the space
        // allocated in the code range, making the "copy" shared, clean, file-backed
        // memory, and thus saving sizeof(builtins).
        //
        // Builtins should start at a page boundary, see
        // platform-embedded-file-writer-mac.cc. If it'