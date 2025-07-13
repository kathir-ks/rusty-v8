// Converted from V8 C++ source files:
// Header: sandbox.h
// Implementation: sandbox.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct CPU {}
    impl CPU {
        pub fn exposes_num_virtual_address_bits(&self) -> bool {
            true
        }
        pub fn num_virtual_address_bits(&self) -> u32 {
            48
        }
    }

    pub mod bits {
        pub fn CountLeadingZeros(x: usize) -> u32 {
            x.leading_zeros() as u32
        }
        pub fn IsPowerOfTwo(x: usize) -> bool {
            x.is_power_of_two()
        }
    }

    pub mod utils {
        pub struct RandomNumberGenerator {
            seed: u64,
        }
        impl RandomNumberGenerator {
            pub fn new() -> Self {
                Self { seed: 0 }
            }
            pub fn SetSeed(&mut self, seed: u32) {
                self.seed = seed as u64;
            }
            pub fn NextInt64(&mut self) -> u64 {
                self.seed = self.seed.wrapping_mul(25214903917).wrapping_add(11);
                self.seed
            }
        }
    }

    pub mod sys_info {
        pub fn AddressSpaceEnd() -> usize {
            //  8TB limit
            8 * 1024 * 1024 * 1024 * 1024
        }
    }

    pub fn IsInHalfOpenRange(addr: usize, start: usize, end: usize) -> bool {
        addr >= start && addr < end
    }
    pub mod bounded_page_allocator {
        pub struct BoundedPageAllocator {}
    }

    pub mod emulated_virtual_address_subspace {
        use std::ptr::null_mut;

        pub struct EmulatedVirtualAddressSubspace {
            vas: *mut super::super::v8::VirtualAddressSpace,
            reservation_base: usize,
            reservation_size: usize,
            size: usize,
        }

        impl EmulatedVirtualAddressSubspace {
            pub fn new(
                vas: *mut super::super::v8::VirtualAddressSpace,
                reservation_base: usize,
                reservation_size: usize,
                size: usize,
            ) -> Self {
                EmulatedVirtualAddressSubspace {
                    vas,
                    reservation_base,
                    reservation_size,
                    size,
                }
            }
            pub fn AllocateGuardRegion(&mut self, _address: usize, _size: usize) -> bool {
                true
            }
        }
    }
    pub mod virtual_address_space_page_allocator {
        pub struct VirtualAddressSpacePageAllocator {}
    }
}

pub mod flags {
    pub static mut random_seed: u32 = 0;
    pub static mut sandbox_testing: bool = false;
    pub static mut sandbox_fuzzing: bool = false;
}
pub mod sandbox {
    pub mod hardware_support {
        pub fn TryEnable(_base: usize, _size: usize) {}
    }
}
pub mod utils {
    pub mod allocation {}
}
pub mod trap_handler {
    pub fn RegisterV8Sandbox(_base: usize, _size: usize) -> bool {
        true
    }
    pub fn UnregisterV8Sandbox(_base: usize, _size: usize) {}
}

use std::cmp::min;
use std::mem::size_of;
use std::ptr::null_mut;
use std::sync::Mutex;
use std::{
    ptr,
    sync::{Arc, Once},
};

pub mod v8 {
    pub struct Isolate {}
    pub struct VirtualAddressSpace {}

    #[derive(Debug, PartialEq, Eq)]
    pub enum PagePermissions {
        kNoAccess,
        kReadWrite,
        // Add other permission types as needed
    }

    impl VirtualAddressSpace {
        pub fn CanAllocateSubspaces(&self) -> bool {
            true
        }

        pub fn AllocateSubspace(
            &self,
            hint: usize,
            size: usize,
            alignment: usize,
            permissions: PagePermissions,
        ) -> Option<Box<VirtualAddressSpace>> {
            if size % alignment != 0 {
                return None;
            }
            Some(Box::new(VirtualAddressSpace {}))
        }

        pub fn AllocatePages(
            &self,
            hint: usize,
            size: usize,
            alignment: usize,
            permissions: PagePermissions,
        ) -> usize {
            if size % alignment != 0 {
                return 0;
            }
            hint // return hint as base address
        }

        pub fn FreePages(&self, address: usize, size: usize) {}

        pub fn AllocateGuardRegion(&self, address: usize, size: usize) -> bool {
            true
        }

        pub fn RandomPageAddress(&self) -> usize {
            4096
        }
        pub fn allocation_granularity(&self) -> usize {
            4096
        }
        pub fn base(&self) -> usize {
            0
        }
    }
    pub struct PageAllocator {}
}

pub mod internal {
    use super::base;
    use super::v8;
    use std::cmp::min;
    use std::mem::size_of;
    use std::ptr::null_mut;
    use std::sync::{Arc, Mutex, Once};
    const GB: usize = 1024 * 1024 * 1024;

    pub type Address = usize;
    const kNullAddress: Address = 0;

    const kSandboxGuardRegionSize: usize = 32 * 1024 * 1024; // 32MB
    const kSandboxSize: usize = 1024 * GB; // 1TB
    const kSandboxMinimumReservationSize: usize = 64 * 1024 * 1024; // 64MB
    const kSandboxAlignment: usize = 2 * 1024 * 1024; // 2MB

    pub static COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL: bool = false;

    pub struct Sandbox {
        base_: Address,
        end_: Address,
        size_: usize,
        reservation_base_: Address,
        reservation_size_: usize,
        initialized_: bool,
        trap_handler_initialized_: bool,
        address_space_: Option<Box<v8::VirtualAddressSpace>>,
        sandbox_page_allocator_: Option<Box<v8::PageAllocator>>,
        constants_: SandboxedPointerConstants,
        first_four_gb_of_address_space_are_reserved_: bool,
    }

    impl Sandbox {
        pub fn new() -> Self {
            Sandbox {
                base_: kNullAddress,
                end_: kNullAddress,
                size_: 0,
                reservation_base_: kNullAddress,
                reservation_size_: 0,
                initialized_: false,
                trap_handler_initialized_: false,
                address_space_: None,
                sandbox_page_allocator_: None,
                constants_: SandboxedPointerConstants::new(),
                first_four_gb_of_address_space_are_reserved_: false,
            }
        }
        pub const kFallbackToPartiallyReservedSandboxAllowed: bool = true;

        pub fn Initialize(&mut self, vas: &mut v8::VirtualAddressSpace) {
            let address_space_limit = Self::DetermineAddressSpaceLimit();
            let mut max_reservation_size = address_space_limit / 4;
            if !vas.CanAllocateSubspaces() {
                max_reservation_size = kSandboxMinimumReservationSize;
            }

            let mut success = false;
            let mut reservation_size = min(kSandboxSize, max_reservation_size);
            if reservation_size < kSandboxSize {
                assert!(max_reservation_size >= kSandboxMinimumReservationSize);
                success = self.InitializeAsPartiallyReservedSandbox(
                    vas,
                    kSandboxSize,
                    reservation_size,
                );
            } else {
                assert_eq!(kSandboxSize, reservation_size);
                let use_guard_regions = true;
                success = self.InitializeWithGuardRegions(vas, kSandboxSize, use_guard_regions);
            }
            while !success && reservation_size > kSandboxMinimumReservationSize {
                assert!(Self::kFallbackToPartiallyReservedSandboxAllowed);
                reservation_size /= 2;
                assert!(reservation_size >= kSandboxMinimumReservationSize);
                success = self.InitializeAsPartiallyReservedSandbox(
                    vas,
                    kSandboxSize,
                    reservation_size,
                );
            }

            if !success {
                Self::FatalProcessOutOfMemory(
                    ptr::null_mut(),
                    "Failed to reserve the virtual address space for the V8 sandbox",
                );
            }

            if super::trap_handler::RegisterV8Sandbox(self.base(), self.size()) {
                self.trap_handler_initialized_ = true;
            } else {
                Self::FatalProcessOutOfMemory(
                    ptr::null_mut(),
                    "Failed to allocate sandbox record for trap handling.",
                );
            }

            super::sandbox::hardware_support::TryEnable(self.base(), self.size());

            assert!(self.initialized_);
        }

        fn InitializeWithGuardRegions(
            &mut self,
            vas: &mut v8::VirtualAddressSpace,
            size: usize,
            use_guard_regions: bool,
        ) -> bool {
            assert!(!self.initialized_);
            assert!(size.is_power_of_two());
            assert!(vas.CanAllocateSubspaces());

            let mut reservation_size = size;
            let true_reservation_size = size;
            let kAdditionalTrailingGuardRegionSize: usize;

            #[cfg(target_os = "android")]
            {
                kAdditionalTrailingGuardRegionSize = 0;
            }
            #[cfg(not(target_os = "android"))]
            {
                const K_TOTAL_TRAILING_GUARD_REGION_SIZE: usize = 260 * GB;
                kAdditionalTrailingGuardRegionSize =
                    K_TOTAL_TRAILING_GUARD_REGION_SIZE - kSandboxGuardRegionSize;
            }
            if use_guard_regions {
                reservation_size += 2 * kSandboxGuardRegionSize;
            }
            let hint = Self::RoundDown(vas.RandomPageAddress(), kSandboxAlignment);

            self.address_space_ = vas.AllocateSubspace(
                hint,
                reservation_size,
                kSandboxAlignment,
                v8::PagePermissions::kReadWrite,
            );

            if self.address_space_.is_none() {
                return false;
            }
            let address_space = self.address_space_.as_mut().unwrap();
            self.reservation_base_ = address_space.base();
            self.base_ =
                self.reservation_base_ + (if use_guard_regions { kSandboxGuardRegionSize } else { 0 });
            self.size_ = size;
            self.end_ = self.base_ + self.size_;
            self.reservation_size_ = reservation_size;
            self.sandbox_page_allocator_ = Some(Box::new(v8::PageAllocator {}));
            if use_guard_regions {
                let front = self.reservation_base_;
                let back = self.end_;

                assert!(address_space.AllocateGuardRegion(front, kSandboxGuardRegionSize));
                assert!(address_space.AllocateGuardRegion(
                    back,
                    kSandboxGuardRegionSize + kAdditionalTrailingGuardRegionSize
                ));
            }
            if !Self::first_four_gb_of_address_space_are_reserved_ {
                let end = 4 * GB;
                let step = address_space.allocation_granularity();
                let mut start = 0;
                while start <= 1 * 1024 * 1024 {
                    if vas.AllocateGuardRegion(start, end - start) {
                        Self::first_four_gb_of_address_space_are_reserved_ = true;
                        break;
                    }
                    start += step;
                }
            }

            self.initialized_ = true;

            self.FinishInitialization();

            assert!(!self.is_partially_reserved());
            return true;
        }
        fn InitializeAsPartiallyReservedSandbox(
            &mut self,
            vas: &mut v8::VirtualAddressSpace,
            size: usize,
            size_to_reserve: usize,
        ) -> bool {
            assert!(!self.initialized_);
            assert!(size.is_power_of_two());
            assert!(size_to_reserve.is_power_of_two());
            assert!(size_to_reserve < size);

            let mut rng = base::utils::RandomNumberGenerator::new();
            if unsafe {super::flags::random_seed != 0} {
                rng.SetSeed(unsafe {super::flags::random_seed});
            }
            let address_space_end = Self::DetermineAddressSpaceLimit();
            let highest_allowed_address = address_space_end / 2;
            assert!(highest_allowed_address.is_power_of_two());
            const K_MAX_ATTEMPTS: i32 = 10;
            for i in 1..=K_MAX_ATTEMPTS {
                let mut hint = rng.NextInt64() % highest_allowed_address as u64;
                hint = Self::RoundDown(hint as usize, kSandboxAlignment) as u64;
                self.reservation_base_ = vas.AllocatePages(
                    hint as usize,
                    size_to_reserve,
                    kSandboxAlignment,
                    v8::PagePermissions::kNoAccess,
                );
                if self.reservation_base_ == 0 {
                    return false;
                }

                if self.reservation_base_ <= highest_allowed_address || i == K_MAX_ATTEMPTS {
                    break;
                }
                vas.FreePages(self.reservation_base_, size_to_reserve);
                self.reservation_base_ = kNullAddress;
            }
            assert!(self.reservation_base_ != 0);
            self.base_ = self.reservation_base_;
            self.size_ = size;
            self.end_ = self.base_ + self.size_;
            self.reservation_size_ = size_to_reserve;
            self.initialized_ = true;
            self.address_space_ = Some(Box::new(super::base::emulated_virtual_address_subspace::EmulatedVirtualAddressSubspace::new(vas, self.reservation_base_, self.reservation_size_, self.size_)));
            self.sandbox_page_allocator_ = Some(Box::new(v8::PageAllocator {}));
            self.FinishInitialization();

            assert!(self.is_partially_reserved());
            return true;
        }

        fn FatalProcessOutOfMemory(arg1: *mut v8::Isolate, arg2: &str) {
            panic!("{}", arg2);
        }

        fn DetermineAddressSpaceLimit() -> usize {
            #[cfg(not(target_arch = "x86_64"))]
            {
                panic!("Unsupported target architecture.");
            }

            const K_DEFAULT_VIRTUAL_ADDRESS_BITS: u32 = 48;
            const K_MIN_VIRTUAL_ADDRESS_BITS: u32 = 36;
            const K_MAX_VIRTUAL_ADDRESS_BITS: u32 = 64;

            let mut hardware_virtual_address_bits = K_DEFAULT_VIRTUAL_ADDRESS_BITS;
            let cpu = base::CPU {};
            if cpu.exposes_num_virtual_address_bits() {
                hardware_virtual_address_bits = cpu.num_virtual_address_bits();
            }
            #[cfg(target_arch = "aarch64")]
            #[cfg(target_os = "android")]
            {
                hardware_virtual_address_bits = 40;
            }
            hardware_virtual_address_bits -= 1;

            let software_limit = base::sys_info::AddressSpaceEnd();
            let software_virtual_address_bits =
                64 - base::bits::CountLeadingZeros(software_limit - 1);

            let mut virtual_address_bits =
                min(hardware_virtual_address_bits, software_virtual_address_bits as u32);

            if virtual_address_bits < K_MIN_VIRTUAL_ADDRESS_BITS
                || virtual_address_bits > K_MAX_VIRTUAL_ADDRESS_BITS
            {
                virtual_address_bits = K_DEFAULT_VIRTUAL_ADDRESS_BITS;
            }

            1 << virtual_address_bits
        }

        fn RoundDown(value: usize, alignment: usize) -> usize {
            value & !(alignment - 1)
        }

        fn FinishInitialization(&mut self) {
            let allocation_granularity = self
                .address_space_
                .as_ref()
                .unwrap()
                .allocation_granularity();
            let success = self.address_space_.as_mut().unwrap().AllocateGuardRegion(
                self.end_ - allocation_granularity,
                allocation_granularity,
            );
            assert!(success || self.is_partially_reserved());
            self.InitializeConstants();
        }

        fn InitializeConstants(&mut self) {
            self.constants_
                .set_empty_backing_store_buffer(self.end_ - 1);
        }

        pub fn TearDown(&mut self) {
            if self.initialized_ {
                if self.trap_handler_initialized_ {
                    super::trap_handler::UnregisterV8Sandbox(self.base(), self.size());
                    self.trap_handler_initialized_ = false;
                }
                self.address_space_.take();
                self.sandbox_page_allocator_.take();
                self.base_ = kNullAddress;
                self.end_ = kNullAddress;
                self.size_ = 0;
                self.reservation_base_ = kNullAddress;
                self.reservation_size_ = 0;
                self.initialized_ = false;
                self.constants_.Reset();
            }
        }

        pub fn is_initialized(&self) -> bool {
            self.initialized_
        }

        pub fn is_partially_reserved(&self) -> bool {
            self.reservation_size_ < self.size_
        }

        pub fn smi_address_range_is_inaccessible(&self) -> bool {
            self.first_four_gb_of_address_space_are_reserved_
        }

        pub fn base(&self) -> Address {
            self.base_
        }

        pub fn end(&self) -> Address {
            self.end_
        }

        pub fn size(&self) -> usize {
            self.size_
        }

        pub fn reservation_size(&self) -> usize {
            self.reservation_size_
        }

        pub fn address_space(&mut self) -> Option<&mut v8::VirtualAddressSpace> {
            self.address_space_.as_deref_mut()
        }
        pub fn page_allocator(&mut self) -> Option<&mut v8::PageAllocator> {
            self.sandbox_page_allocator_.as_deref_mut()
        }

        pub fn Contains(&self, addr: Address) -> bool {
            super::base::IsInHalfOpenRange(addr, self.base_, self.base_ + self.size_)
        }
        pub fn ContainsPtr(&self, ptr: *mut std::ffi::c_void) -> bool {
            self.Contains(ptr as Address)
        }
        pub fn ReservationContains(&self, addr: Address) -> bool {
            super::base::IsInHalfOpenRange(
                addr,
                self.reservation_base_,
                self.reservation_base_ + self.reservation_size_,
            )
        }
        pub fn constants(&self) -> &SandboxedPointerConstants {
            &self.constants_
        }
        pub fn base_address(&self) -> Address {
            (&self.base_) as *const Address as Address
        }
        pub fn end_address(&self) -> Address {
            (&self.end_) as *const Address as Address
        }
        pub fn size_address(&self) -> Address {
            (&self.size_) as *const usize as Address
        }
        pub fn InitializeDefaultOncePerProcess(vas: &mut v8::VirtualAddressSpace) {
            static mut DEFAULT_SANDBOX: *mut Sandbox = ptr::null_mut();
            static ONCE: Once = Once::new();

            unsafe {
                ONCE.call_once(|| {
                    let mut default_sandbox = Box::new(Sandbox::new());
                    default_sandbox.Initialize(vas);
                    DEFAULT_SANDBOX = Box::into_raw(default_sandbox);
                });
                super::flags::sandbox_testing = true;
            }
        }
        pub fn TearDownDefault() {}
        pub fn New(vas: &mut v8::VirtualAddressSpace) -> Box<Sandbox> {
            if !COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL {
                panic!(
                    "Creation of new sandboxes requires enabling multiple pointer compression \
                     cages at build-time"
                );
            }
            let mut sandbox = Box::new(Sandbox::new());
            sandbox.Initialize(vas);
            assert!(!unsafe {super::flags::sandbox_testing} && !unsafe {super::flags::sandbox_fuzzing});
            sandbox
        }

        static mut default_sandbox_: *mut Sandbox = std::ptr::null_mut();
        pub fn GetDefault() -> &'static mut Sandbox {
            unsafe {
                static mut DEFAULT_SANDBOX: Option<Sandbox> = None;
                static ONCE: Once = Once::new();

                ONCE.call_once(|| {
                    DEFAULT_SANDBOX = Some(Sandbox::new());
                });

                &mut DEFAULT_SANDBOX.as_mut().unwrap()
            }
        }
    }
    impl Default for Sandbox {
        fn default() -> Self {
            Sandbox::new()
        }
    }
    pub struct SandboxedPointerConstants {
        empty_backing_store_buffer_: Address,
    }

    impl SandboxedPointerConstants {
        pub fn new() -> Self {
            SandboxedPointerConstants {
                empty_backing_store_buffer_: 0,
            }
        }
        pub fn empty_backing_store_buffer(&self) -> Address {
            self.empty_backing_store_buffer_
        }
        pub fn empty_backing_store_buffer_address(&self) -> Address {
            (&self.empty_backing_store_buffer_) as *const Address as Address
        }
        pub fn set_empty_backing_store_buffer(&mut self, value: Address) {
            self.empty_backing_store_buffer_ = value;
        }
        pub fn Reset(&mut self) {
            self.empty_backing_store_buffer_ = 0;
        }
    }
    static mut DEFAULT_SANDBOX: *mut Sandbox = std::ptr::null_mut();

    pub fn InsideSandbox(address: usize) -> bool {
        let sandbox = Sandbox::GetDefault();
        sandbox.ReservationContains(address)
    }

    pub fn EmptyBackingStoreBuffer() -> *mut std::ffi::c_void {
        let sandbox = Sandbox::GetDefault();
        sandbox.constants().empty_backing_store_buffer() as *mut std::ffi::c_void
    }
}
