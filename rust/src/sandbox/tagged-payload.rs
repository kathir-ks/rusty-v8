// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Implement proper error handling and remove unwraps.
// TODO: Consider using `NonNull` for `Address` where applicable to ensure validity.
// TODO: Explore using `bitflags` crate for managing flags more ergonomically.

/// Represents a memory address.  For now, it's a u64.
pub type Address = u64;

/// Struct providing common utilities for pointer tagging.
pub struct TaggedPayload<PayloadTaggingScheme> {
    encoded_word_: Address,
    _phantom: std::marker::PhantomData<PayloadTaggingScheme>, // Zero-sized type to hold PayloadTaggingScheme
}

impl<PayloadTaggingScheme: PayloadTagging> TaggedPayload<PayloadTaggingScheme> {
    /// Creates a new `TaggedPayload` from a pointer and a tag.
    pub fn new(pointer: Address, tag: PayloadTaggingScheme::TagType) -> Self {
        assert!(PayloadTaggingScheme::kMarkBit != 0, "Invalid kMarkBit specified in tagging scheme.");
        assert!((PayloadTaggingScheme::kMarkBit & PayloadTaggingScheme::kTagMask) == 0, "The mark bit does not overlap with the TagMask.");
        Self {
            encoded_word_: Self::tag(pointer, tag),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Untags the payload with the given tag, removing the tag and the mark bit.
    pub fn untag(&self, tag: PayloadTaggingScheme::TagType) -> Address {
        self.encoded_word_ & !(tag as Address | PayloadTaggingScheme::kMarkBit as Address)
    }

    /// Tags the pointer with the given tag.
    pub fn tag(pointer: Address, tag: PayloadTaggingScheme::TagType) -> Address {
        pointer | tag as Address
    }

    /// Checks if the payload is tagged with the given tag.
    pub fn is_tagged_with(&self, tag: PayloadTaggingScheme::TagType) -> bool {
        (self.encoded_word_ & PayloadTaggingScheme::kTagMask as Address) == tag as Address
    }

    /// Sets the tag of the payload to the new tag.
    pub fn set_tag(&mut self, new_tag: PayloadTaggingScheme::TagType) {
        self.encoded_word_ = (self.encoded_word_ & !PayloadTaggingScheme::kTagMask as Address) | new_tag as Address;
    }

    /// Sets the mark bit of the payload.
    pub fn set_mark_bit(&mut self) {
        self.encoded_word_ |= PayloadTaggingScheme::kMarkBit as Address;
    }

    /// Clears the mark bit of the payload.
    pub fn clear_mark_bit(&mut self) {
        self.encoded_word_ &= !(PayloadTaggingScheme::kMarkBit as Address);
    }

    /// Checks if the mark bit is set.
    pub fn has_mark_bit_set(&self) -> bool {
        (self.encoded_word_ & PayloadTaggingScheme::kMarkBit as Address) != 0
    }

    /// Extracts the freelist link from the payload.
    pub fn extract_freelist_link(&self) -> u32 {
        self.encoded_word_ as u32
    }

    /// Extracts the tag from the payload.
    pub fn extract_tag(&self) -> PayloadTaggingScheme::TagType {
        ((self.encoded_word_ & PayloadTaggingScheme::kTagMask as Address) | PayloadTaggingScheme::kMarkBit as Address) as PayloadTaggingScheme::TagType
    }

    /// Checks if the payload contains a freelist link.
    pub fn contains_freelist_link(&self) -> bool {
        self.is_tagged_with(PayloadTaggingScheme::kFreeEntryTag)
    }

    /// Checks if the payload contains an evacuation entry.
    pub fn contains_evacuation_entry(&self) -> bool {
        if PayloadTaggingScheme::kSupportsEvacuation {
            self.is_tagged_with(PayloadTaggingScheme::kEvacuationEntryTag)
        } else {
            false
        }
    }

    /// Checks if the payload is zapped.
    pub fn is_zapped(&self) -> bool {
        if PayloadTaggingScheme::kSupportsZapping {
            self.is_tagged_with(PayloadTaggingScheme::kZappedEntryTag)
        } else {
            false
        }
    }

    /// Extracts the evacuation entry handle location from the payload.
    pub fn extract_evacuation_entry_handle_location(&self) -> Address {
        if PayloadTaggingScheme::kSupportsEvacuation {
            self.untag(PayloadTaggingScheme::kEvacuationEntryTag)
        } else {
            panic!("UNREACHABLE"); // In C++, this was UNREACHABLE()
        }
    }

    /// Checks if the payload contains a pointer.
    pub fn contains_pointer(&self) -> bool {
        !self.contains_freelist_link() && !self.contains_evacuation_entry() && !self.is_zapped()
    }
}

impl<PayloadTaggingScheme> PartialEq for TaggedPayload<PayloadTaggingScheme> {
    fn eq(&self, other: &Self) -> bool {
        self.encoded_word_ == other.encoded_word_
    }
}

impl<PayloadTaggingScheme> Eq for TaggedPayload<PayloadTaggingScheme> {}

impl<PayloadTaggingScheme> TaggedPayload<PayloadTaggingScheme> {
    fn encoded_word(&self) -> Address {
        self.encoded_word_
    }
}

pub trait PayloadTagging {
    type TagType: Copy + Eq + std::convert::From<u32> + std::convert::Into<u32>;
    const kMarkBit: u32;
    const kTagMask: u32;
    const kFreeEntryTag: Self::TagType;
    const kSupportsEvacuation: bool;
    const kEvacuationEntryTag: Self::TagType;
    const kSupportsZapping: bool;
    const kZappedEntryTag: Self::TagType;
}