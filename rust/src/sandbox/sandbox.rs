// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: The following features are enabled based on conditional compilation flags from the original C++ code.
//       Make sure to enable them in your Rust project if they are required.
// #![feature(allocator_api)]
// #![feature(const_fn_trait_bound)]
// #![feature(core_intrinsics)]
// #![feature(nonnull_slice_from_raw_parts)]
// #![feature(strict_provenance)]

use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Once, RwLock};

// Mock imports for v8-internal.h, v8-platform.h, v8config.h, globals.h, bounds.h
// Replace with actual crate imports when available
mod v8 {
    pub type Address = usize;
    pub trait VirtualAddressSpace {
        fn allocate_subspace(&mut self, size: usize) -> Option<usize>;
        fn deallocate_subspace(&mut self, address: usize, size: usize);
    }
    pub trait PageAllocator {}
}

mod base {
    pub fn is_in_half_open_range(addr: usize, start: usize, end: usize) -> bool {
        addr >= start && addr < end
    }
}

const V8_ENABLE_WEBASSEMBLY: bool = true;
const V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES: bool = true;
const USING_V8_SHARED_PRIVATE: bool = false;

const kNullAddress: usize = 0;

#[cfg(test)]
mod testing {
    pub mod gtest {
        #[macro_export]
        macro_rules! assert_eq {
            ($left:expr, $right:expr) => {
                if $left != $right {
                    panic!("assertion failed: `(left == right)`\
                      left: `{:?}`,\
                     right: `{:?}`", $left, $right)
                }
            };
        }

        #[macro_export]
        macro_rules! assert {
            ($condition:expr) => {
                if !$condition {
                    panic!("assertion failed: `{}`", stringify!($condition))
                }
            };
        }
    }
}

#[cfg(test)]
mod sandbox_test {
    use super::*;
    #[test]
    fn initialization_with_size() {
        let mut vas = MockVirtualAddressSpace::new();
        let mut sandbox = Sandbox::new();
        sandbox.initialize(&mut vas, 4096, false);
        assert_eq!(sandbox.is_initialized(), true);
        sandbox.tear_down();
    }

    #[test]
    fn partially_reserved_sandbox() {
        let mut vas = MockVirtualAddressSpace::new();
        let mut sandbox = Sandbox::new();
        sandbox.initialize_as_partially_reserved_sandbox(&mut vas, 8192, 4096);
        assert_eq!(sandbox.is_initialized(), true);
        assert_eq!(sandbox.is_partially_reserved(), true);
        sandbox.tear_down();
    }

    struct MockVirtualAddressSpace {}
    impl MockVirtualAddressSpace {
        pub fn new() -> Self {
            MockVirtualAddressSpace {}
        }
    }
    impl v8::VirtualAddressSpace for MockVirtualAddressSpace {
        fn allocate_subspace(&mut self, size: usize) -> Option<usize> {
            Some(0x1000)
        }
        fn deallocate_subspace(&mut self, address: usize, size: usize) {
            // do nothing
        }
    }
}

#[derive(Default)]
pub struct Sandbox {
    base_: v8::Address,
    end_: v8::Address,
    size_: usize,
    reservation_base_: v8::Address,
    reservation_size_: usize,
    initialized_: bool,
    #[cfg(all(V8_ENABLE_WEBASSEMBLY, feature = "trap_handler_supported"))]
    trap_handler_initialized_: bool,
    address_space_: Option<Box<dyn v8::VirtualAddressSpace + Send + Sync>>,
    sandbox_page_allocator_: Option<Box<dyn v8::PageAllocator + Send + Sync>>,
    constants_: SandboxedPointerConstants,
}

impl Sandbox {
    pub const K_FALLBACK_TO_PARTIALLY_RESERVED_SANDBOX_ALLOWED: bool = true;

    pub fn new() -> Self {
        Sandbox::default()
    }

    pub fn initialize(&mut self, vas: &mut dyn v8::VirtualAddressSpace) {
        self.initialize_with_guard_regions(vas, true);
    }

    fn initialize_with_guard_regions(&mut self, vas: &mut dyn v8::VirtualAddressSpace, use_guard_regions: bool) -> bool {
        let size = 1024 * 1024; // Example size, needs to come from v8config
        self.initialize_with_size(vas, size, use_guard_regions)
    }

    fn initialize_with_size(&mut self, vas: &mut dyn v8::VirtualAddressSpace, size: usize, use_guard_regions: bool) -> bool {
        if use_guard_regions {
            // TODO: Implement guard regions, requires knowing guard region size
            let reservation_size = size; //+ 2 * guard_region_size;
            if let Some(reservation_base) = vas.allocate_subspace(reservation_size) {
                self.reservation_base_ = reservation_base;
                self.reservation_size_ = reservation_size;
                self.base_ = reservation_base; //+ guard_region_size;
                self.size_ = size;
                self.end_ = self.base_ + self.size_;
                self.address_space_ = Some(Box::new(EmulatedVirtualAddressSubspace::new(reservation_base, size)));
                self.sandbox_page_allocator_ = Some(Box::new(MockPageAllocator {}));
                self.finish_initialization();
                true
            } else {
                if Self::K_FALLBACK_TO_PARTIALLY_RESERVED_SANDBOX_ALLOWED {
                    self.initialize_as_partially_reserved_sandbox(vas, size, size)
                } else {
                    panic!("OOM crash: Could not allocate virtual address space for sandbox."); // Proper OOM handling required
                }
            }
        } else {
            if let Some(base) = vas.allocate_subspace(size) {
                self.reservation_base_ = base;
                self.reservation_size_ = size;
                self.base_ = base;
                self.size_ = size;
                self.end_ = self.base_ + self.size_;
                self.address_space_ = Some(Box::new(EmulatedVirtualAddressSubspace::new(base, size)));
                self.sandbox_page_allocator_ = Some(Box::new(MockPageAllocator {}));
                self.finish_initialization();
                true
            } else {
                panic!("OOM crash: Could not allocate virtual address space for sandbox.");
            }
        }
    }

    fn initialize_as_partially_reserved_sandbox(&mut self, vas: &mut dyn v8::VirtualAddressSpace, size: usize, size_to_reserve: usize) -> bool {
        if let Some(base) = vas.allocate_subspace(size_to_reserve) {
            self.reservation_base_ = base;
            self.reservation_size_ = size_to_reserve;
            self.base_ = base;
            self.size_ = size;
            self.end_ = self.base_ + self.size_;
            self.address_space_ = Some(Box::new(EmulatedVirtualAddressSubspace::new(base, size)));
            self.sandbox_page_allocator_ = Some(Box::new(MockPageAllocator {}));
            self.finish_initialization();
            true
        } else {
            panic!("OOM crash: Could not allocate virtual address space for sandbox.");
        }
    }

    fn finish_initialization(&mut self) {
        self.initialized_ = true;
        self.initialize_constants();

        // Attempt to make the first four GB inaccessible. This can fail.
        first_four_gb_of_address_space_are_reserved().store(true, Ordering::Relaxed);
    }

    fn initialize_constants(&mut self) {
        //TODO: Proper implementation of constant initialization inside sandbox
        self.constants_ = SandboxedPointerConstants::default();
    }

    pub fn tear_down(&mut self) {
        if self.initialized_ {
            if let Some(mut address_space) = self.address_space_.take() {
                address_space.deallocate_subspace(self.reservation_base_, self.reservation_size_);
            }
            self.initialized_ = false;
            self.base_ = kNullAddress;
            self.end_ = kNullAddress;
            self.size_ = 0;
            self.reservation_base_ = kNullAddress;
            self.reservation_size_ = 0;

            #[cfg(all(V8_ENABLE_WEBASSEMBLY, feature = "trap_handler_supported"))]
            {
                self.trap_handler_initialized_ = false;
            }

            first_four_gb_of_address_space_are_reserved().store(false, Ordering::Relaxed);
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized_
    }

    pub fn is_partially_reserved(&self) -> bool {
        self.reservation_size_ < self.size_
    }

    pub fn smi_address_range_is_inaccessible(&self) -> bool {
        first_four_gb_of_address_space_are_reserved().load(Ordering::Relaxed)
    }

    pub fn base(&self) -> v8::Address {
        self.base_
    }

    pub fn end(&self) -> v8::Address {
        self.end_
    }

    pub fn size(&self) -> usize {
        self.size_
    }

    pub fn reservation_size(&self) -> usize {
        self.reservation_size_
    }

    pub fn address_space(&self) -> Option<&dyn v8::VirtualAddressSpace> {
        self.address_space_.as_ref().map(|x| x.as_ref())
    }

    pub fn page_allocator(&self) -> Option<&dyn v8::PageAllocator> {
        self.sandbox_page_allocator_.as_ref().map(|x| x.as_ref())
    }

    pub fn contains(&self, addr: v8::Address) -> bool {
        base::is_in_half_open_range(addr, self.base_, self.base_ + self.size_)
    }

    pub fn contains_ptr(&self, ptr: *mut std::ffi::c_void) -> bool {
        self.contains(ptr as v8::Address)
    }

    pub fn reservation_contains(&self, addr: v8::Address) -> bool {
        base::is_in_half_open_range(addr, self.reservation_base_, self.reservation_base_ + self.reservation_size_)
    }

    pub fn constants(&self) -> &SandboxedPointerConstants {
        &self.constants_
    }

    pub fn base_address(&self) -> v8::Address {
        (&self.base_) as *const _ as v8::Address
    }
    pub fn end_address(&self) -> v8::Address {
        (&self.end_) as *const _ as v8::Address
    }
    pub fn size_address(&self) -> v8::Address {
        (&self.size_) as *const _ as v8::Address
    }

    pub fn initialize_default_once_per_process(vas: &mut dyn v8::VirtualAddressSpace) {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let mut default_sandbox = Sandbox::new();
            default_sandbox.initialize(vas);
            *DEFAULT_SANDBOX.write().unwrap() = Some(Box::new(default_sandbox));
        });
    }

    pub fn tear_down_default() {
        if let Some(mut sandbox) = DEFAULT_SANDBOX.write().unwrap().take() {
            sandbox.tear_down();
        }
    }

    pub fn new_sandbox(vas: &mut dyn v8::VirtualAddressSpace) -> Box<Sandbox> {
        let mut sandbox = Sandbox::new();
        sandbox.initialize(vas);
        Box::new(sandbox)
    }

    #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
    pub fn current() -> Option<&'static Sandbox> {
        CURRENT.with(|current| current.borrow().map(|s| unsafe { &*(&**s as *const Sandbox) }))
    }

    #[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
    pub fn set_current(sandbox: Option<Box<Sandbox>>) {
        CURRENT.with(|current| {
            *current.borrow_mut() = sandbox;
        });
    }

    #[cfg(not(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES))]
    pub fn current() -> Option<&'static Sandbox> {
        Self::get_default()
    }
    #[cfg(not(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES))]
    pub fn set_current(_sandbox: Option<Box<Sandbox>>) {
        // do nothing
    }

    pub fn get_default() -> Option<&'static Sandbox> {
        DEFAULT_SANDBOX.read().unwrap().as_ref().map(|s| unsafe { &*(&**s as *const Sandbox) })
    }
}

#[derive(Default)]
pub struct SandboxedPointerConstants {
    empty_backing_store_buffer_: v8::Address,
}

impl SandboxedPointerConstants {
    pub fn empty_backing_store_buffer(&self) -> v8::Address {
        self.empty_backing_store_buffer_
    }
    pub fn empty_backing_store_buffer_address(&self) -> v8::Address {
        (&self.empty_backing_store_buffer_) as *const _ as v8::Address
    }
    pub fn set_empty_backing_store_buffer(&mut self, value: v8::Address) {
        self.empty_backing_store_buffer_ = value;
    }

    pub fn reset(&mut self) {
        self.empty_backing_store_buffer_ = 0;
    }
}

// Helper function that can be used to ensure that certain objects are not
// located inside the sandbox. Typically used for trusted objects.
// Will always return false when the sandbox is disabled or partially reserved.
#[inline]
pub fn inside_sandbox(address: usize) -> bool {
    #[cfg(V8_ENABLE_SANDBOX)]
    {
        if let Some(sandbox) = Sandbox::current() {
            // Use ReservationContains (instead of just Contains) to correctly handle the
            // case of partially-reserved sandboxes.
            return sandbox.reservation_contains(address);
        }
    }
    false
}

#[inline]
pub fn empty_backing_store_buffer() -> *mut std::ffi::c_void {
    #[cfg(V8_ENABLE_SANDBOX)]
    {
        if let Some(sandbox) = Sandbox::current() {
            return sandbox.constants().empty_backing_store_buffer() as *mut std::ffi::c_void;
        }
    }
    std::ptr::null_mut()
}

// Mock implementations for VirtualAddressSpace and PageAllocator
struct EmulatedVirtualAddressSubspace {
    base: usize,
    size: usize,
}

impl EmulatedVirtualAddressSubspace {
    fn new(base: usize, size: usize) -> Self {
        EmulatedVirtualAddressSubspace { base, size }
    }
}

impl v8::VirtualAddressSpace for EmulatedVirtualAddressSubspace {
    fn allocate_subspace(&mut self, size: usize) -> Option<usize> {
        // Simple emulation, always returns the base address if enough space is available.
        if size <= self.size {
            Some(self.base)
        } else {
            None
        }
    }

    fn deallocate_subspace(&mut self, address: usize, size: usize) {
        // In a real implementation, this would free the memory.
        // Here, we just check that the address is correct.
        assert_eq!(address, self.base);
        assert!(size <= self.size);
    }
}

struct MockPageAllocator {}

impl v8::PageAllocator for MockPageAllocator {}

static DEFAULT_SANDBOX: RwLock<Option<Box<Sandbox>>> = RwLock::new(None);
static FIRST_FOUR_GB_OF_ADDRESS_SPACE_ARE_RESERVED: AtomicBool = AtomicBool::new(false);

fn first_four_gb_of_address_space_are_reserved() -> &'static AtomicBool {
    &FIRST_FOUR_GB_OF_ADDRESS_SPACE_ARE_RESERVED
}

#[cfg(V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)]
thread_local! {
    static CURRENT: std::cell::RefCell<Option<Box<Sandbox>>> = std::cell::RefCell::new(None);
}