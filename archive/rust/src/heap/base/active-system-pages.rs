// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap {
    pub mod base {

        /// Class implements a bitset of system pages on a heap page.
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct ActiveSystemPages {
            value_: bitset_t,
        }

        impl ActiveSystemPages {
            /// Defines the maximum number of system pages that can be tracked in one
            /// instance.
            pub const K_MAX_PAGES: usize = 64;

            type bitset_t = std::bitset::BitSet<u64, {Self::K_MAX_PAGES}>;

            /// Initializes the set of active pages to the system pages for the header.
            pub fn init(&mut self, header_size: usize, page_size_bits: usize, user_page_size: usize) -> usize {
                 // V8_EXPORT_PRIVATE functionality.  In this case, assume is no-op

                //TODO: Add proper implementation based on header size, page_size_bits, user_page_size
                //       and how the original C++ does it.  This is just a placeholder.

                self.value_.clear();
                let num_pages = (header_size + user_page_size -1 ) / user_page_size;
                for i in 0..std::cmp::min(num_pages, Self::K_MAX_PAGES) {
                    self.value_.set(i, true);
                }
                self.value_.count_ones() as usize
            }

            /// Adds the pages for this memory range. Returns the number of freshly added pages.
            pub fn add(&mut self, start: usize, end: usize, page_size_bits: usize) -> usize {
                // V8_EXPORT_PRIVATE functionality.  In this case, assume is no-op

                //TODO: Add proper implementation based on start, end, and page_size_bits
                //       and how the original C++ does it.  This is just a placeholder.

                let start_page = start >> page_size_bits;
                let end_page = end >> page_size_bits;

                let mut added_count = 0;
                for i in start_page..std::cmp::min(end_page, Self::K_MAX_PAGES) {
                    if !self.value_.get(i) {
                        self.value_.set(i, true);
                        added_count += 1;
                    }
                }
                added_count as usize
            }

            /// Replaces the current bitset with the given argument. The new bitset needs
            /// to be a proper subset of the current pages, which means this operation
            /// can't add pages. Returns the number of removed pages.
            pub fn reduce(&mut self, updated_value: ActiveSystemPages) -> usize {
                // V8_EXPORT_PRIVATE functionality.  In this case, assume is no-op

                //TODO: Add proper implementation that enforces that the new bitset must be a
                //       subset of the current value.

                let mut removed_count = 0;
                for i in 0..Self::K_MAX_PAGES {
                    if self.value_.get(i) && !updated_value.value_.get(i) {
                        self.value_.set(i, false);
                        removed_count += 1;
                    }
                }
                removed_count as usize
            }

            /// Removes all pages. Returns the number of removed pages.
            pub fn clear(&mut self) -> usize {
                // V8_EXPORT_PRIVATE functionality.  In this case, assume is no-op
                let count = self.value_.count_ones();
                self.value_.clear();
                count as usize
            }

            /// Returns the memory used with the given page size.
            pub fn size(&self, page_size_bits: usize) -> usize {
                // V8_EXPORT_PRIVATE functionality.  In this case, assume is no-op
                self.value_.count_ones() as usize * (1 << page_size_bits)
            }
        }
    }  // namespace base
}  // namespace heap