// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Module representing an address region of certain size.
pub mod address_region {
    use std::cmp::{max, min};
    use std::fmt;
    use std::mem;
    use std::ops::Sub;
    use std::any::type_name;

    /// Represents an address.  Uses `usize` because it is guaranteed to be large enough to hold a pointer
    pub type Address = usize;

    /// Helper struct representing an address region.
    #[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
    pub struct AddressRegion {
        address_: Address,
        size_: usize,
    }

    impl AddressRegion {
        /// Creates a new `AddressRegion` with the given address and size.
        pub const fn new(address: Address, size: usize) -> Self {
            Self {
                address_: address,
                size_: size,
            }
        }

        /// Returns the starting address of the region.
        pub const fn begin(&self) -> Address {
            self.address_
        }

        /// Returns the ending address of the region.
        pub const fn end(&self) -> Address {
            self.address_ + self.size_
        }

        /// Returns the size of the region.
        pub const fn size(&self) -> usize {
            self.size_
        }

        /// Sets the size of the region.
        pub fn set_size(&mut self, size: usize) {
            self.size_ = size;
        }

        /// Returns true if the region is empty (size is 0).
        pub const fn is_empty(&self) -> bool {
            self.size_ == 0
        }

        /// Returns true if the region contains the given address.
        pub fn contains(&self, address: Address) -> bool {
            address.checked_sub(self.begin()).map_or(false, |offset| offset < self.size())
        }

        /// Returns true if the region contains the address and size.
        pub fn contains_with_size(&self, address: Address, size: usize) -> bool {
            address.checked_sub(self.begin()).map_or(false, |offset| {
                offset < self.size_ && offset.checked_add(size).map_or(false, |sum| sum <= self.size_)
            })
        }

        /// Returns true if the region contains the given region.
        pub fn contains_region(&self, region: AddressRegion) -> bool {
            self.contains_with_size(region.address_, region.size_)
        }

        /// Returns the overlapping region between this region and the given region.
        pub fn get_overlap(&self, region: AddressRegion) -> AddressRegion {
            let overlap_start = max(self.begin(), region.begin());
            let overlap_end = max(overlap_start, min(self.end(), region.end()));
            AddressRegion::new(overlap_start, overlap_end - overlap_start)
        }
    }

    impl fmt::Display for AddressRegion {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{:p}+{}]", self.begin() as *const u8, self.size())
        }
    }

    /// Construct an AddressRegion from a start pointer and a size.
    pub fn address_region_of<T>(ptr: *const T, size: usize) -> AddressRegion {
        AddressRegion::new(ptr as Address, mem::size_of::<T>() * size)
    }

    /// Construct an AddressRegion from anything providing a {data()} and {size()} accessor.
    pub fn address_region_of_container<Container>(c: &Container) -> AddressRegion
    where
        Container: HasDataAndSize,
    {
        address_region_of(c.data(), c.size())
    }

    pub trait HasDataAndSize {
        type ElemType;
        fn data(&self) -> *const Self::ElemType;
        fn size(&self) -> usize;
    }

    pub struct StartAddressLess;

    impl StartAddressLess {
        pub fn compare(a: AddressRegion, b: AddressRegion) -> bool {
            a.begin() < b.begin()
        }
    }
}