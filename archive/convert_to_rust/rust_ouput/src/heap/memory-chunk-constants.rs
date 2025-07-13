// Converted from V8 C++ source files:
// Header: memory-chunk-constants.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn BitWidth(x: usize) -> usize {
            usize::BITS as usize - x.leading_zeros() as usize
        }
    }
}

pub mod common {
    pub mod globals {
        pub const kPtrComprCageReservationSize: usize = 2usize.pow(32);
        pub const kMaximalCodeRangeSize: usize = 2usize.pow(30);
        pub const kMaximalTrustedRangeSize: usize = 2usize.pow(28);
        pub const kRegularPageSize: usize = 4096;
    }
}

pub mod v8 {
    pub mod internal {
        use crate::AllStatic;
        use crate::common::globals::*;

        pub struct MemoryChunkConstants {}

        impl MemoryChunkConstants {
            #[cfg(v8_enable_sandbox)]
            pub const kPagesInMainCage: usize =
                kPtrComprCageReservationSize / kRegularPageSize;
            #[cfg(v8_enable_sandbox)]
            pub const kPagesInCodeCage: usize =
                kMaximalCodeRangeSize / kRegularPageSize;
            #[cfg(v8_enable_sandbox)]
            pub const kPagesInTrustedCage: usize =
                kMaximalTrustedRangeSize / kRegularPageSize;

            #[cfg(v8_enable_sandbox)]
            pub const kMainCageMetadataOffset: usize = 0;
            #[cfg(v8_enable_sandbox)]
            pub const kTrustedSpaceMetadataOffset: usize =
                MemoryChunkConstants::kMainCageMetadataOffset + MemoryChunkConstants::kPagesInMainCage;
            #[cfg(v8_enable_sandbox)]
            pub const kCodeRangeMetadataOffset: usize =
                MemoryChunkConstants::kTrustedSpaceMetadataOffset + MemoryChunkConstants::kPagesInTrustedCage;

            #[cfg(v8_enable_sandbox)]
            pub const kMetadataPointerTableSizeLog2: usize =
                crate::base::bits::BitWidth(
                    MemoryChunkConstants::kPagesInMainCage + MemoryChunkConstants::kPagesInCodeCage
                        + MemoryChunkConstants::kPagesInTrustedCage,
                );
            #[cfg(v8_enable_sandbox)]
            pub const kMetadataPointerTableSize: usize =
                1 << MemoryChunkConstants::kMetadataPointerTableSizeLog2;
            #[cfg(v8_enable_sandbox)]
            pub const kMetadataPointerTableSizeMask: usize =
                MemoryChunkConstants::kMetadataPointerTableSize - 1;
        }
    }
}
