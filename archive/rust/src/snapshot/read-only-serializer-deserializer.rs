// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ro {
    /// Common functionality for RO serialization and deserialization.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Bytecode {
        /// kAllocatePage parameters:
        ///   Uint30 page_index
        ///   Uint30 area_size_in_bytes
        AllocatePage,
        /// kAllocatePageAt parameters:
        ///   Uint30 page_index
        ///   Uint30 area_size_in_bytes
        ///   Uint32 compressed_page_address
        AllocatePageAt,
        /// kSegment parameters:
        ///   Uint30 page_index
        ///   Uint30 offset
        ///   Uint30 size_in_bytes
        ///   ... segment byte stream
        Segment,
        /// kRelocateSegment parameters:
        ///   ... relocation byte stream
        RelocateSegment,
        /// kReadOnlyRootsTable parameters:
        ///   IF_STATIC_ROOTS(... ro roots table slots)
        ReadOnlyRootsTable,
        /// No parameters
        FinalizeReadOnlySpace,
    }

    pub const K_NUMBER_OF_BYTECODES: usize = Bytecode::FinalizeReadOnlySpace as usize + 1;

    /// Like std::vector<bool> but with a known underlying encoding.
    pub struct BitSet {
        size_in_bits: usize,
        data: Box<[u8]>,
        owns_data: bool,
    }

    impl BitSet {
        /// Constructs a new `BitSet` with the given size in bits.
        pub fn new(size_in_bits: usize) -> Self {
            let size_in_bytes = Self::size_in_bytes_from_bits(size_in_bits);
            let data = vec![0u8; size_in_bytes].into_boxed_slice();
            Self {
                size_in_bits,
                data,
                owns_data: true,
            }
        }

        /// Constructs a `BitSet` from existing data.
        pub fn from_data(data: &mut [u8], size_in_bits: usize) -> Self {
            Self {
                size_in_bits,
                data: data.into(),
                owns_data: false,
            }
        }

        /// Checks if the bit at the given index is set.
        pub fn contains(&self, i: usize) -> bool {
            debug_assert!(i < self.size_in_bits);
            (self.data[Self::chunk_index(i)] & Self::bit_mask(i)) != 0
        }

        /// Sets the bit at the given index.
        pub fn set(&mut self, i: usize) {
            debug_assert!(i < self.size_in_bits);
            self.data[Self::chunk_index(i)] |= Self::bit_mask(i);
        }

        /// Returns the size of the `BitSet` in bits.
        pub fn size_in_bits(&self) -> usize {
            self.size_in_bits
        }

        /// Returns the size of the `BitSet` in bytes.
        pub fn size_in_bytes(&self) -> usize {
            Self::size_in_bytes_from_bits(self.size_in_bits)
        }

        const fn size_in_bytes_from_bits(size_in_bits: usize) -> usize {
            (size_in_bits + (crate::internal::K_BITS_PER_BYTE - 1)) / crate::internal::K_BITS_PER_BYTE
        }

        /// Returns a pointer to the underlying data.
        pub fn data(&self) -> &[u8] {
            &self.data
        }

        const K_BITS_PER_CHUNK: usize = crate::internal::K_UINT8_SIZE * crate::internal::K_BITS_PER_BYTE;

        const fn chunk_index(i: usize) -> usize {
            i / Self::K_BITS_PER_CHUNK
        }

        const fn bit_index(i: usize) -> usize {
            i % Self::K_BITS_PER_CHUNK
        }

        const fn bit_mask(i: usize) -> u8 {
            1 << Self::bit_index(i)
        }
    }

    impl Drop for BitSet {
        fn drop(&mut self) {
            // Box<[u8]> handles deallocation when owns_data is true.
        }
    }

    /// Tagged slots need relocation after deserialization when V8_STATIC_ROOTS is
    /// disabled.
    ///
    /// Note this encoding works for all remaining build configs, in particular for
    /// all supported kTaggedSize values.
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct EncodedTagged {
        page_index: u32,
        offset: u32,
    }

    impl EncodedTagged {
        pub const K_OFFSET_BITS: usize = crate::internal::K_PAGE_SIZE_BITS;
        pub const K_SIZE: usize = crate::internal::K_UINT32_SIZE;
        pub const K_PAGE_INDEX_BITS: usize = Self::K_SIZE * 8 - Self::K_OFFSET_BITS; // Determines max number of RO pages.

        /// Constructs a new `EncodedTagged`.
        pub fn new(page_index: u32, offset: u32) -> Self {
            debug_assert!(page_index < (1 << Self::K_PAGE_INDEX_BITS));
            debug_assert!(offset < (1 << Self::K_OFFSET_BITS));
            Self { page_index, offset }
        }

        /// Converts the `EncodedTagged` to a `u32`.
        pub fn to_u32(self) -> u32 {
            debug_assert_eq!(Self::K_SIZE, crate::internal::K_UINT32_SIZE);
            unsafe { std::mem::transmute_copy(&self) }
        }

        /// Constructs an `EncodedTagged` from a `u32`.
        pub fn from_u32(v: u32) -> Self {
            unsafe { std::mem::transmute_copy(&v) }
        }

        /// Constructs an `EncodedTagged` from an `Address`.
        pub fn from_address(address: usize) -> Self {
            unsafe { *(address as *const EncodedTagged) }
        }

        pub fn page_index(&self) -> u32 {
            self.page_index
        }

        pub fn offset(&self) -> u32 {
            self.offset
        }
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct EncodedExternalReference {
        is_api_reference: u32,
        index: u32,
    }

    impl EncodedExternalReference {
        pub const K_IS_API_REFERENCE_BITS: usize = 1;
        pub const K_INDEX_BITS: usize = 31;
        pub const K_SIZE: usize = crate::internal::K_UINT32_SIZE;

        /// Converts the `EncodedExternalReference` to a `u32`.
        pub fn to_u32(self) -> u32 {
            debug_assert_eq!(Self::K_SIZE, crate::internal::K_UINT32_SIZE);
            unsafe { std::mem::transmute_copy(&self) }
        }

        /// Constructs an `EncodedExternalReference` from a `u32`.
        pub fn from_u32(v: u32) -> Self {
            unsafe { std::mem::transmute_copy(&v) }
        }

        // This ctor is needed to convert parameter types. We can't use bool/uint32_t
        // as underlying member types since that messes with field packing on
        // windows.
        /// Constructs a new `EncodedExternalReference`.
        pub fn new(is_api_reference: bool, index: u32) -> Self {
            let is_api_reference_int = if is_api_reference { 1 } else { 0 };
            Self {
                is_api_reference: is_api_reference_int,
                index,
            }
        }

        pub fn is_api_reference(&self) -> bool {
            self.is_api_reference != 0
        }

        pub fn index(&self) -> u32 {
            self.index
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bitset() {
            let mut bitset = BitSet::new(100);
            assert_eq!(bitset.size_in_bits(), 100);
            assert_eq!(bitset.size_in_bytes(), 13);
            assert!(!bitset.contains(5));
            bitset.set(5);
            assert!(bitset.contains(5));
            assert!(!bitset.contains(6));
        }

        #[test]
        fn test_encoded_tagged() {
            let page_index = 10;
            let offset = 20;
            let encoded = EncodedTagged::new(page_index, offset);
            assert_eq!(encoded.page_index(), page_index);
            assert_eq!(encoded.offset(), offset);
            let encoded_u32 = encoded.to_u32();
            let decoded = EncodedTagged::from_u32(encoded_u32);
            assert_eq!(decoded.page_index(), page_index);
            assert_eq!(decoded.offset(), offset);
        }

        #[test]
        fn test_encoded_external_reference() {
            let is_api_reference = true;
            let index = 30;
            let encoded = EncodedExternalReference::new(is_api_reference, index);
            assert_eq!(encoded.is_api_reference(), is_api_reference);
            assert_eq!(encoded.index(), index);

            let encoded_u32 = encoded.to_u32();
            let decoded = EncodedExternalReference::from_u32(encoded_u32);
            assert_eq!(decoded.is_api_reference(), is_api_reference);
            assert_eq!(decoded.index(), index);
        }
    }
}

mod internal {
    pub const K_BITS_PER_BYTE: usize = 8;
    pub const K_UINT8_SIZE: usize = 1;
    pub const K_UINT32_SIZE: usize = 4;
    pub const K_PAGE_SIZE_BITS: usize = 12;
}