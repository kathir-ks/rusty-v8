// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    /// Represents the age of a card in the age table.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Age {
        Old,
        Young,
        Mixed,
    }

    /// Policy for handling adjacent cards when setting age.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AdjacentCardsPolicy {
        Ignore,
        Consider,
    }

    // TODO(https://github.com/rust-lang/rust/issues/101282): Use const generics
    // once they're stable.  For now, we'll use a const.
    const AGE_TABLE_SIZE: usize = 4096; // Example value, should be derived based on CagedHeapBase::GetAgeTableSize()
                                        // but that depends on other parts of the codebase not provided.
                                        // Update this value accordingly when integrating with other modules.

    /// Represents the age table for the caged heap.
    #[derive(Debug)]
    pub struct AgeTable {
        table: [Age; AGE_TABLE_SIZE],
    }

    impl AgeTable {
        /// Creates a new `AgeTable` with all ages initialized to `Age::Old`.
        pub fn new() -> Self {
            AgeTable {
                table: [Age::Old; AGE_TABLE_SIZE],
            }
        }

        /// Sets the age for a range of offsets in the age table.
        ///
        /// # Arguments
        ///
        /// * `offset_begin` - The starting offset of the range.
        /// * `offset_end` - The ending offset of the range.
        /// * `age` - The age to set for the range.
        /// * `adjacent_cards_policy` - The policy for handling adjacent cards.
        pub fn set_age_for_range(
            &mut self,
            offset_begin: usize,
            offset_end: usize,
            age: Age,
            adjacent_cards_policy: AdjacentCardsPolicy,
        ) {
            const CARD_SIZE_IN_BYTES: usize = 512; // Example value, update if different
            let inner_card_offset_begin = Self::round_up(offset_begin, CARD_SIZE_IN_BYTES);
            let outer_card_offset_end = Self::round_down(offset_end, CARD_SIZE_IN_BYTES);

            for inner_offset in (inner_card_offset_begin..outer_card_offset_end).step_by(CARD_SIZE_IN_BYTES) {
                self.set_age(inner_offset, age);
            }

            let set_age_for_outer_card = |offset: usize| {
                if Self::is_aligned(offset, CARD_SIZE_IN_BYTES) {
                    return;
                }
                if adjacent_cards_policy == AdjacentCardsPolicy::Ignore {
                    self.set_age(offset, age);
                } else if self.get_age(offset) != age {
                    self.set_age(offset, Age::Mixed);
                }
            };

            set_age_for_outer_card(offset_begin);
            set_age_for_outer_card(offset_end);
        }

        /// Gets the age for a range of offsets in the age table.
        ///
        /// # Arguments
        ///
        /// * `offset_begin` - The starting offset of the range.
        /// * `offset_end` - The ending offset of the range.
        ///
        /// # Returns
        ///
        /// The age for the range.  If the age is not the same for all offsets in the range,
        /// returns `Age::Mixed`.
        pub fn get_age_for_range(&self, offset_begin: usize, offset_end: usize) -> Age {
            const CARD_SIZE_IN_BYTES: usize = 512; // Example value, update if different
            let mut result = self.get_age(offset_begin);
            for offset in ((offset_begin + CARD_SIZE_IN_BYTES)..offset_end).step_by(CARD_SIZE_IN_BYTES) {
                if result != self.get_age(offset) {
                    result = Age::Mixed;
                    break;
                }
            }
            result
        }

        /// Resets the age table for testing purposes. All ages are set to `Age::Old`.
        pub fn reset_for_testing(&mut self) {
            self.table.fill(Age::Old);
        }

        fn set_age(&mut self, offset: usize, age: Age) {
            let index = offset / 1; // Assuming 1 byte indexing, update if different
            if index < AGE_TABLE_SIZE {
                self.table[index] = age;
            }
        }

        fn get_age(&self, offset: usize) -> Age {
            let index = offset / 1; // Assuming 1 byte indexing, update if different
            if index < AGE_TABLE_SIZE {
                self.table[index]
            } else {
                Age::Old // Default value if out of bounds, consider other options based on intended behavior
            }
        }

        const fn round_up(num_to_round: usize, multiple: usize) -> usize {
            if multiple == 0 {
                return num_to_round;
            }
        
            let remainder = num_to_round % multiple;
            if remainder == 0 {
                return num_to_round;
            }
        
            num_to_round + multiple - remainder
        }
        
        const fn round_down(num_to_round: usize, multiple: usize) -> usize {
            if multiple == 0 {
                return num_to_round;
            }
        
            num_to_round - (num_to_round % multiple)
        }

        const fn is_aligned(x: usize, alignment: usize) -> bool {
            (x & (alignment - 1)) == 0
        }
    }

    impl Default for AgeTable {
        fn default() -> Self {
            Self::new()
        }
    }
}