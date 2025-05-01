// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod zone_type_traits {
    // use crate::common::globals::*; // Assuming globals.h has equivalent definitions

    // Assuming V8_COMPRESS_ZONES is a compile-time feature flag
    #[cfg(feature = "compress_zones")]
    use crate::zone::compressed_zone_ptr::CompressedZonePtr;

    // Placeholder for ZoneList. Needs a Rust implementation.
    // In C++, this likely manages a list of Ts within a Zone.
    // Implementing this accurately depends heavily on the Zone's memory
    // management strategy.  For now, we represent it with a Vec.
    pub struct ZoneList<T>(Vec<T>);

    impl<T> ZoneList<T> {
        pub fn new() -> Self {
            ZoneList(Vec::new())
        }

        pub fn push(&mut self, value: T) {
            self.0.push(value);
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            self.0.iter()
        }
    }

    /// ZonePtrList is a ZoneList of pointers to ZoneObjects allocated in the same
    /// zone as the list object.
    pub type ZonePtrList<T> = ZoneList<Box<T>>;

    pub type FullZonePtr<T> = Box<T>;

    // Placeholder for CompressedZonePtr.  Needs a Rust implementation.
    // This likely represents a pointer compressed relative to a Zone's base address.
    // Implementing this accurately depends heavily on the Zone's memory
    // management strategy.
    #[cfg(feature = "compress_zones")]
    pub use CompressedZonePtr;

    #[cfg(not(feature = "compress_zones"))]
    pub struct CompressedZonePtr<T>(*mut T);

    //
    // ZoneTypeTraits provides type aliases for compressed or full pointer
    // dependent types based on a static flag. It helps organizing fine-grained
    // control over which parts of the code base should use compressed zone
    // pointers.
    // For example:
    //   using ZoneNodePtr = typename ZoneTypeTraits<kCompressGraphZone>::Ptr<Node>;
    //
    // or
    //   template <typename T>
    //   using AstZonePtr = typename ZoneTypeTraits<kCompressAstZone>::Ptr<T>;
    //
    pub trait ZonePointer<T> {
        type Ptr;
    }

    #[cfg(not(feature = "compress_zones"))]
    pub struct ZoneTypeTraits;

    #[cfg(not(feature = "compress_zones"))]
    impl<T> ZonePointer<T> for ZoneTypeTraits {
        type Ptr = FullZonePtr<T>;
    }

    #[cfg(feature = "compress_zones")]
    pub struct ZoneTypeTraits;

    #[cfg(feature = "compress_zones")]
    impl<T> ZonePointer<T> for ZoneTypeTraits {
        type Ptr = CompressedZonePtr<T>;
    }

    //
    // is_compressed_pointer<T> predicate can be used for checking if T is a
    // compressed pointer.
    //
    pub trait IsCompressedPointer {
        const VALUE: bool;
    }

    impl<T> IsCompressedPointer for T {
        default const VALUE: bool = false;
    }

    #[cfg(feature = "compress_zones")]
    impl<T> IsCompressedPointer for CompressedZonePtr<T> {
        const VALUE: bool = true;
    }

    #[cfg(feature = "compress_zones")]
    impl<T> IsCompressedPointer for &CompressedZonePtr<T> {
        const VALUE: bool = true;
    }
}

pub use zone_type_traits::*;