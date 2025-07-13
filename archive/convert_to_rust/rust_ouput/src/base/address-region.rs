// Converted from V8 C++ source files:
// Header: address-region.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    cmp::{max, min},
    fmt,
    marker::Copy,
    mem::size_of,
};

// Helper class representing an address region of certain size.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddressRegion {
    address_: usize,
    size_: usize,
}

impl AddressRegion {
    // Function object that compares the start address of two regions. Usable as
    // compare function on std data structures and algorithms.
    pub struct StartAddressLess {}

    pub type Address = usize;

    pub const fn new() -> Self {
        AddressRegion {
            address_: 0,
            size_: 0,
        }
    }

    pub const fn from_address_and_size(address: Address, size: usize) -> Self {
        AddressRegion {
            address_: address,
            size_: size,
        }
    }

    pub fn begin(&self) -> Address {
        self.address_
    }
    pub fn end(&self) -> Address {
        self.address_ + self.size_
    }

    pub fn size(&self) -> usize {
        self.size_
    }
    pub fn set_size(&mut self, size: usize) {
        self.size_ = size;
    }

    pub fn is_empty(&self) -> bool {
        self.size_ == 0
    }

    pub fn contains(&self, address: Address) -> bool {
        (address - self.begin()) < self.size()
    }

    pub fn contains_with_size(&self, address: Address, size: usize) -> bool {
        let offset = address - self.begin();
        (offset < self.size_) && (offset + size <= self.size_)
    }

    pub fn contains_region(&self, region: AddressRegion) -> bool {
        self.contains_with_size(region.address_, region.size_)
    }

    pub fn get_overlap(&self, region: AddressRegion) -> AddressRegion {
        let overlap_start = max(self.begin(), region.begin());
        let overlap_end = max(overlap_start, min(self.end(), region.end()));
        AddressRegion {
            address_: overlap_start,
            size_: overlap_end - overlap_start,
        }
    }
}

// Construct an AddressRegion from a start pointer and a size.
pub fn address_region_of<T>(ptr: *mut T, size: usize) -> AddressRegion {
    AddressRegion::from_address_and_size(ptr as usize, size_of::<T>() * size)
}

// Construct an AddressRegion from anything providing a {data()} and {size()}
// accessor.
pub fn address_region_of_container<Container>(c: &Container) -> AddressRegion
where
    Container: ContainerTrait,
{
    address_region_of(c.data(), c.size())
}

pub trait ContainerTrait {
    type Data;
    fn data(&self) -> *mut Self::Data;
    fn size(&self) -> usize;
}

impl fmt::Display for AddressRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:p}+{}]", self.address_ as *mut u8, self.size())
    }
}

#[test]
fn test_address_region() {
    let region1 = AddressRegion::from_address_and_size(0x1000, 0x1000);
    let region2 = AddressRegion::from_address_and_size(0x2000, 0x2000);

    assert_eq!(region1.begin(), 0x1000);
    assert_eq!(region1.end(), 0x2000);
    assert_eq!(region1.size(), 0x1000);

    assert!(region1.contains(0x1500));
    assert!(!region1.contains(0x2500));

    assert!(region1.contains_with_size(0x1100, 0x0100));
    assert!(!region1.contains_with_size(0x1900, 0x0800));

    assert_eq!(
        region1.get_overlap(AddressRegion::from_address_and_size(0x0000, 0x0800)),
        AddressRegion::from_address_and_size(0x1000, 0x0000)
    );
    assert_eq!(
        region1.get_overlap(AddressRegion::from_address_and_size(0x1800, 0x1000)),
        AddressRegion::from_address_and_size(0x1800, 0x0800)
    );
    assert_eq!(
        region1.get_overlap(AddressRegion::from_address_and_size(0x2000, 0x1000)),
        AddressRegion::from_address_and_size(0x2000, 0x0000)
    );
}
