// Converted from V8 C++ source files:
// Header: read-only-serializer-deserializer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ro {

    // Common functionality for RO serialization and deserialization.

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Bytecode {
        // kAllocatePage parameters:
        //   Uint30 page_index
        //   Uint30 area_size_in_bytes
        kAllocatePage,
        // kAllocatePageAt parameters:
        //   Uint30 page_index
        //   Uint30 area_size_in_bytes
        //   Uint32 compressed_page_address
        kAllocatePageAt,
        //
        // kSegment parameters:
        //   Uint30 page_index
        //   Uint30 offset
        //   Uint30 size_in_bytes
        //   ... segment byte stream
        kSegment,
        //
        // kRelocateSegment parameters:
        //   ... relocation byte stream
        kRelocateSegment,
        //
        // kReadOnlyRootsTable parameters:
        //   IF_STATIC_ROOTS(... ro roots table slots)
        kReadOnlyRootsTable,
        //
        kFinalizeReadOnlySpace,
    }
    pub const KNUMBER_OF_BYTECODES: i32 = Bytecode::kFinalizeReadOnlySpace as i32 + 1;

    // Like std::vector<bool> but with a known underlying encoding.
    #[derive(Debug)]
    pub struct BitSet {
        size_in_bits_: usize,
        data_: Vec<u8>,
        owns_data_: bool,
    }

    impl BitSet {
        pub fn new(size_in_bits: usize) -> Self {
            let size_in_bytes = (size_in_bits + 7) / 8;
            let data_ = vec![0u8; size_in_bytes];
            BitSet {
                size_in_bits_: size_in_bits,
                data_: data_,
                owns_data_: true,
            }
        }

        pub fn from_data(data: *mut u8, size_in_bits: usize) -> Self {
            let data_ = unsafe { Vec::from_raw_parts(data, (size_in_bits + 7) / 8, (size_in_bits + 7) / 8) };
            BitSet {
                size_in_bits_: size_in_bits,
                data_: data_,
                owns_data_: false,
            }
        }

        pub fn contains(&self, i: i32) -> bool {
            if !(0 <= i && i < self.size_in_bits_ as i32) {
                return false;
            }
            (self.data_[self.chunk_index(i as usize)] & self.bit_mask(i as usize)) != 0
        }

        pub fn set(&mut self, i: i32) {
            if !(0 <= i && i < self.size_in_bits_ as i32) {
                return;
            }
            let chunk_index = self.chunk_index(i as usize);
            let bit_mask = self.bit_mask(i as usize);
            self.data_[chunk_index] |= bit_mask;
        }

        pub fn size_in_bits(&self) -> usize {
            self.size_in_bits_
        }

        pub fn size_in_bytes(&self) -> usize {
            (self.size_in_bits_ + 7) / 8
        }

        pub fn data(&self) -> &[u8] {
            &self.data_
        }

        const KBITS_PER_CHUNK: usize = 8;
        fn chunk_index(&self, i: usize) -> usize {
            i / Self::KBITS_PER_CHUNK
        }
        fn bit_index(&self, i: usize) -> usize {
            i % Self::KBITS_PER_CHUNK
        }
        fn bit_mask(&self, i: usize) -> u8 {
            1 << self.bit_index(i)
        }
    }

    impl Drop for BitSet {
        fn drop(&mut self) {
            if self.owns_data_ {
                // `data_` is already a Vec, so it will handle deallocation correctly.
            }
        }
    }

    // Tagged slots need relocation after deserialization when V8_STATIC_ROOTS is
    // disabled.
    //
    // Note this encoding works for all remaining build configs, in particular for
    // all supported kTaggedSize values.
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct EncodedTagged {
        page_index: u32,
        offset: u32,
    }

    impl EncodedTagged {
        pub const KOFFSET_BITS: usize = 12;
        pub const KSIZE: usize = 4;
        pub const KPAGE_INDEX_BITS: usize = Self::KSIZE * 8 - Self::KOFFSET_BITS; // Determines max number of RO pages.

        pub fn new(page_index: u32, offset: u32) -> Self {
            assert!(page_index < (1 << Self::KPAGE_INDEX_BITS));
            assert!(offset < (1 << Self::KOFFSET_BITS));
            EncodedTagged {
                page_index: page_index,
                offset: offset,
            }
        }

        pub fn to_u32(&self) -> u32 {
            unsafe { std::mem::transmute_copy(self) }
        }

        pub fn from_u32(v: u32) -> Self {
            unsafe { std::mem::transmute_copy(&v) }
        }

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

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct EncodedExternalReference {
        is_api_reference: u32,
        index: u32,
    }

    impl EncodedExternalReference {
        pub const KIS_API_REFERENCE_BITS: usize = 1;
        pub const KINDEX_BITS: usize = 31;
        pub const KSIZE: usize = 4;

        pub fn new(is_api_reference: bool, index: u32) -> Self {
            EncodedExternalReference {
                is_api_reference: if is_api_reference { 1 } else { 0 },
                index: index,
            }
        }

        pub fn to_u32(&self) -> u32 {
            unsafe { std::mem::transmute_copy(self) }
        }

        pub fn from_u32(v: u32) -> Self {
            unsafe { std::mem::transmute_copy(&v) }
        }

        pub fn is_api_reference(&self) -> bool {
            self.is_api_reference != 0
        }

        pub fn index(&self) -> u32 {
            self.index
        }
    }
}
