// Converted from V8 C++ source files:
// Header: emulated-virtual-address-subspace.h
// Implementation: emulated-virtual-address-subspace.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use crate::base::RegionAllocator;
    use std::sync::Mutex;

    pub enum PagePermissions {
        Read,
        Write,
        ReadWrite,
        Execute,
        ReadExecute,
        ReadWriteExecute,
        None,
    }

    pub type Address = usize;

    pub trait VirtualAddressSpace {
        fn page_size(&self) -> usize;
        fn allocation_granularity(&self) -> usize;
        fn base(&self) -> Address;
        fn size(&self) -> usize;
        fn max_page_permissions(&self) -> PagePermissions;
        fn set_random_seed(&mut self, seed: i64);
        fn random_page_address(&mut self) -> Address;
        fn allocate_pages(
            &mut self,
            hint: Address,
            size: usize,
            alignment: usize,
            permissions: PagePermissions,
        ) -> Result<Address, String>;
        fn free_pages(&mut self, address: Address, size: usize) -> Result<(), String>;
        fn allocate_shared_pages(
            &mut self,
            hint: Address,
            size: usize,
            permissions: PagePermissions,
            handle: PlatformSharedMemoryHandle,
            offset: u64,
        ) -> Result<Address, String>;
        fn free_shared_pages(&mut self, address: Address, size: usize) -> Result<(), String>;
        fn set_page_permissions(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> Result<bool, String>;
        fn allocate_guard_region(&mut self, address: Address, size: usize) -> Result<bool, String>;
        fn free_guard_region(&mut self, address: Address, size: usize) -> Result<(), String>;
        fn can_allocate_subspaces(&self) -> bool;
        fn allocate_subspace(
            &mut self,
            hint: Address,
            size: usize,
            alignment: usize,
            max_page_permissions: PagePermissions,
        ) -> Result<Box<dyn VirtualAddressSpace>, String>;
        fn recommit_pages(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> Result<bool, String>;
        fn discard_system_pages(&mut self, address: Address, size: usize) -> Result<bool, String>;
        fn decommit_pages(&mut self, address: Address, size: usize) -> Result<bool, String>;
    }

    #[derive(Debug)]
    pub struct PlatformSharedMemoryHandle {
        id: u64,
    }

    pub struct EmulatedVirtualAddressSubspace {
        page_size_: usize,
        allocation_granularity_: usize,
        base_: Address,
        size_: usize,
        max_page_permissions_: PagePermissions,
        mapped_size_: usize,
        parent_space_: Box<dyn VirtualAddressSpace>,
        mutex_: Mutex<()>,
        region_allocator_: RegionAllocator,
        rng_: RandomNumberGenerator,
    }

    impl EmulatedVirtualAddressSubspace {
        pub fn new(
            parent_space: Box<dyn VirtualAddressSpace>,
            base: Address,
            mapped_size: usize,
            total_size: usize,
        ) -> Self {
            assert!(is_power_of_two(mapped_size));
            assert!(is_power_of_two(total_size));
            let page_size = parent_space.page_size();
            EmulatedVirtualAddressSubspace {
                page_size_: parent_space.page_size(),
                allocation_granularity_: parent_space.allocation_granularity(),
                base_: base,
                size_: total_size,
                max_page_permissions_: parent_space.max_page_permissions(),
                mapped_size_: mapped_size,
                parent_space_: parent_space,
                mutex_: Mutex::new(()),
                region_allocator_: RegionAllocator::new(base, mapped_size, page_size),
                rng_: RandomNumberGenerator::new(),
            }
        }

        fn mapped_size(&self) -> usize {
            self.mapped_size_
        }
        fn unmapped_size(&self) -> usize {
            self.size() - self.mapped_size_
        }

        fn mapped_base(&self) -> Address {
            self.base()
        }
        fn unmapped_base(&self) -> Address {
            self.base() + self.mapped_size_
        }

        fn contains(&self, outer_start: Address, outer_size: usize, inner_start: Address,
                    inner_size: usize) -> bool {
            (inner_start >= outer_start) &&
                ((inner_start + inner_size) <= (outer_start + outer_size))
        }

        fn contains_addr(&self, addr: Address, length: usize) -> bool {
            self.contains(self.base(), self.size(), addr, length)
        }

        fn mapped_region_contains(&self, addr: Address, length: usize) -> bool {
            self.contains(self.mapped_base(), self.mapped_size(), addr, length)
        }

        fn unmapped_region_contains(&self, addr: Address, length: usize) -> bool {
            self.contains(self.unmapped_base(), self.unmapped_size(), addr, length)
        }

        // Helper function to define a limit for the size of allocations in the
        // unmapped region. This limit makes it possible to estimate the expected
        // runtime of some loops in the Allocate methods.
        fn is_usable_size_for_unmapped_region(&self, size: usize) -> bool {
            size <= (self.unmapped_size() / 2)
        }
    }

    impl VirtualAddressSpace for EmulatedVirtualAddressSubspace {
        fn page_size(&self) -> usize {
            self.page_size_
        }

        fn allocation_granularity(&self) -> usize {
            self.allocation_granularity_
        }

        fn base(&self) -> Address {
            self.base_
        }

        fn size(&self) -> usize {
            self.size_
        }

        fn max_page_permissions(&self) -> PagePermissions {
            self.max_page_permissions_
        }

        fn set_random_seed(&mut self, seed: i64) {
            let _guard = self.mutex_.lock().unwrap();
            self.rng_.set_seed(seed);
        }

        fn random_page_address(&mut self) -> Address {
            let _guard = self.mutex_.lock().unwrap();
            let addr = self.base() + (self.rng_.next_int64() % self.size() as i64) as usize;
            round_down(addr, self.allocation_granularity())
        }

        fn allocate_pages(
            &mut self,
            hint: Address,
            size: usize,
            alignment: usize,
            permissions: PagePermissions,
        ) -> Result<Address, String> {
            const K_NO_HINT: Address = 0;

            if hint == K_NO_HINT || self.mapped_region_contains(hint, size) {
                let _guard = self.mutex_.lock().unwrap();

                // Attempt to find a region in the mapped region.
                let address_result = self.region_allocator_.allocate_region(hint, size, alignment);

                match address_result {
                    Ok(address) => {
                        // Success. Only need to adjust the page permissions.
                        match self.parent_space_.set_page_permissions(address, size, permissions) {
                            Ok(true) => {
                                return Ok(address);
                            }
                            _ => {
                                // Probably ran out of memory, but still try to allocate in the unmapped
                                // space.
                                let freed_size = self.region_allocator_.free_region(address).unwrap();
                                assert_eq!(size, freed_size);
                            }
                        }
                    }
                    Err(_) => {
                        // Allocation failure in the mapped region.
                    }
                }
            }

            // No luck or hint is outside of the mapped region. Try to allocate pages in
            // the unmapped space using page allocation hints instead.
            if !self.is_usable_size_for_unmapped_region(size) {
                return Err("Size is not usable for unmapped region".to_string());
            }

            const K_MAX_ATTEMPTS: i32 = 10;
            for _i in 0..K_MAX_ATTEMPTS {
                // If an unmapped region exists, it must cover at least 50% of the whole
                // space (unmapped + mapped region). Since we limit the size of allocation
                // to 50% of the unmapped region (see IsUsableSizeForUnmappedRegion), a
                // random page address has at least a 25% chance of being a usable base. As
                // such, this loop should usually terminate quickly.
                assert!(self.unmapped_size() >= self.mapped_size());
                let mut hint_local = hint;
                while !self.unmapped_region_contains(hint_local, size) {
                    hint_local = self.random_page_address();
                }
                hint_local = round_down(hint_local, alignment);

                match self.parent_space_.allocate_pages(hint_local, size, alignment, permissions) {
                    Ok(result) => {
                        if self.unmapped_region_contains(result, size) {
                            return Ok(result);
                        } else {
                            match self.parent_space_.free_pages(result, size) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error freeing pages: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error allocating pages: {}", e);
                    }
                }

                // Retry at a different address.
                hint_local = self.random_page_address();
            }

            Err("Failed to allocate pages after multiple attempts".to_string())
        }

        fn free_pages(&mut self, address: Address, size: usize) -> Result<(), String> {
            if self.mapped_region_contains(address, size) {
                let _guard = self.mutex_.lock().unwrap();
                let freed_size = self.region_allocator_.free_region(address).unwrap();
                assert_eq!(size, freed_size);
                self.parent_space_.decommit_pages(address, size)?;
                Ok(())
            } else {
                assert!(self.unmapped_region_contains(address, size));
                self.parent_space_.free_pages(address, size)
            }
        }

        fn allocate_shared_pages(
            &mut self,
            hint: Address,
            size: usize,
            permissions: PagePermissions,
            handle: PlatformSharedMemoryHandle,
            offset: u64,
        ) -> Result<Address, String> {
            // Can only allocate shared pages in the unmapped region.
            if !self.is_usable_size_for_unmapped_region(size) {
                return Err("Size is not usable for unmapped region".to_string());
            }

            const K_MAX_ATTEMPTS: i32 = 10;
            for _i in 0..K_MAX_ATTEMPTS {
                // See AllocatePages() for why this loop usually terminates quickly.
                assert!(self.unmapped_size() >= self.mapped_size());
                let mut hint_local = hint;
                while !self.unmapped_region_contains(hint_local, size) {
                    hint_local = self.random_page_address();
                }

                match self.parent_space_.allocate_shared_pages(hint_local, size, permissions, handle, offset) {
                    Ok(region) => {
                        if self.unmapped_region_contains(region, size) {
                            return Ok(region);
                        } else {
                            match self.parent_space_.free_shared_pages(region, size) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error freeing shared pages: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error allocating shared pages: {}", e);
                    }
                }

                hint_local = self.random_page_address();
            }

            Err("Failed to allocate shared pages after multiple attempts".to_string())
        }

        fn free_shared_pages(&mut self, address: Address, size: usize) -> Result<(), String> {
            assert!(self.unmapped_region_contains(address, size));
            self.parent_space_.free_shared_pages(address, size)
        }

        fn set_page_permissions(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> Result<bool, String> {
            assert!(self.contains_addr(address, size));
            self.parent_space_.set_page_permissions(address, size, permissions)
        }

        fn allocate_guard_region(&mut self, address: Address, size: usize) -> Result<bool, String> {
            if self.mapped_region_contains(address, size) {
                let _guard = self.mutex_.lock().unwrap();
                self.region_allocator_.allocate_region_at(address, size)
            } else {
                if !self.unmapped_region_contains(address, size) {
                    return Ok(false);
                }
                self.parent_space_.allocate_guard_region(address, size)
            }
        }

        fn free_guard_region(&mut self, address: Address, size: usize) -> Result<(), String> {
            if self.mapped_region_contains(address, size) {
                let _guard = self.mutex_.lock().unwrap();
                let freed_size = self.region_allocator_.free_region(address).unwrap();
                assert_eq!(size, freed_size);
                Ok(())
            } else {
                assert!(self.unmapped_region_contains(address, size));
                self.parent_space_.free_guard_region(address, size)
            }
        }

        fn can_allocate_subspaces(&self) -> bool {
            // This is not supported, mostly because it's not (yet) needed in practice.
            false
        }

        fn allocate_subspace(
            &mut self,
            _hint: Address,
            _size: usize,
            _alignment: usize,
            _max_page_permissions: PagePermissions,
        ) -> Result<Box<dyn VirtualAddressSpace>, String> {
            Err("Allocation of subspaces is not supported".to_string())
        }

        fn recommit_pages(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> Result<bool, String> {
            assert!(self.contains_addr(address, size));
            self.parent_space_.recommit_pages(address, size, permissions)
        }

        fn discard_system_pages(&mut self, address: Address, size: usize) -> Result<bool, String> {
            assert!(self.contains_addr(address, size));
            self.parent_space_.discard_system_pages(address, size)
        }

        fn decommit_pages(&mut self, address: Address, size: usize) -> Result<bool, String> {
            assert!(self.contains_addr(address, size));
            self.parent_space_.decommit_pages(address, size)
        }
    }

    impl Drop for EmulatedVirtualAddressSubspace {
        fn drop(&mut self) {
            self.parent_space_.free_pages(self.base(), self.mapped_size_).unwrap();
        }
    }

    fn is_power_of_two(n: usize) -> bool {
        n != 0 && (n & (n - 1)) == 0
    }

    fn round_down(addr: Address, alignment: usize) -> Address {
        addr & !(alignment - 1)
    }

    const K_MAX_INT: i64 = i32::MAX as i64;

    pub struct RandomNumberGenerator {
        seed: u64,
    }

    impl RandomNumberGenerator {
        pub fn new() -> Self {
            RandomNumberGenerator { seed: 0 }
        }

        pub fn set_seed(&mut self, seed: i64) {
            self.seed = seed as u64;
        }

        pub fn next_int64(&mut self) -> i64 {
            self.seed = self.seed.wrapping_mul(25214903917).wrapping_add(11);
            (self.seed >> 16) as i64
        }

        pub fn next_double(&mut self) -> f64 {
            let i64_val = self.next_int64();
            (i64_val as f64) / (K_MAX_INT as f64)
        }
    }
}
