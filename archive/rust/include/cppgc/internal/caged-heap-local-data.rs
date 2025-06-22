// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(feature = "cppgc_caged_heap")]
pub mod caged_heap_local_data {
    use crate::api_constants;
    use crate::caged_heap::CagedHeapBase;
    use crate::logging::CPPGC_DCHECK;
    //use crate::platform::Platform; // TODO: Implement Platform
    //use crate::v8config; // TODO: Find a suitable replacement for v8config

    #[cfg(feature = "cppgc_young_generation")]
    pub mod age_table {
        use crate::api_constants;
        use crate::caged_heap::CagedHeapBase;
        use crate::logging::CPPGC_DCHECK;
        use std::mem;

        pub struct AgeTable {
            table: Vec<Age>,
        }

        impl AgeTable {
            const kRequiredSize: usize = 1 * api_constants::kMB;
            const kAllocationGranularity: usize = api_constants::kAllocationGranularity;

            pub const kCardSizeInBytes: usize =
                api_constants::kCagedHeapDefaultReservationSize / Self::kRequiredSize;

            pub fn calculate_age_table_size_for_heap_size(heap_size: usize) -> usize {
                heap_size / Self::kCardSizeInBytes
            }

            pub fn new(size: usize) -> Self {
                AgeTable {
                    table: vec![Age::KOld; size],
                }
            }

            pub fn set_age(&mut self, cage_offset: usize, age: Age) {
                self.table[self.card(cage_offset)] = age;
            }

            pub fn get_age(&self, cage_offset: usize) -> Age {
                self.table[self.card(cage_offset)]
            }

            // TODO: Implement SetAgeForRange and GetAgeForRange
            pub fn set_age_for_range(
                &mut self,
                cage_offset_begin: usize,
                cage_offset_end: usize,
                age: Age,
                adjacent_cards_policy: AdjacentCardsPolicy,
            ) {
                 for offset in cage_offset_begin..cage_offset_end {
                     self.set_age(offset, age);
                 }
            }

            pub fn get_age_for_range(
                &self,
                cage_offset_begin: usize,
                cage_offset_end: usize,
            ) -> Age {
                let mut mixed = false;
                let first_age = self.get_age(cage_offset_begin);
                 for offset in cage_offset_begin..cage_offset_end {
                     if self.get_age(offset) != first_age {
                         mixed = true;
                         break;
                     }
                 }
                 if mixed {
                     Age::KMixed
                 } else {
                     first_age
                 }

            }

            pub fn reset_for_testing(&mut self) {
                for age in &mut self.table {
                    *age = Age::KOld;
                }
            }

            #[inline]
            fn card(&self, offset: usize) -> usize {
                const kGranularityBits: usize = {
                    let card_size = Self::kCardSizeInBytes as u32;
                    card_size.trailing_zeros() as usize
                };
                assert_eq!(1 << kGranularityBits, Self::kCardSizeInBytes);
                let entry = offset >> kGranularityBits;
                CPPGC_DCHECK!(CagedHeapBase::get_age_table_size() > entry);
                entry
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Age {
            KOld,
            KYoung,
            KMixed,
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum AdjacentCardsPolicy {
            KConsider,
            KIgnore,
        }
    }

    #[cfg(feature = "cppgc_young_generation")]
    use age_table::AgeTable;

    pub struct CagedHeapLocalData {
        #[cfg(feature = "cppgc_young_generation")]
        pub age_table: AgeTable,
    }

    impl CagedHeapLocalData {
        #[inline]
        pub fn get() -> &'static mut CagedHeapLocalData {
            unsafe { &mut *(CagedHeapBase::get_base() as *mut CagedHeapLocalData) }
        }

        pub fn calculate_local_data_size_for_heap_size(heap_size: usize) -> usize {
            #[cfg(feature = "cppgc_young_generation")]
            {
                AgeTable::calculate_age_table_size_for_heap_size(heap_size)
            }
            #[cfg(not(feature = "cppgc_young_generation"))]
            {
                0
            }
        }
        #[cfg(feature = "cppgc_young_generation")]
        pub fn new(age_table_size: usize) -> Self {
            CagedHeapLocalData {
                age_table: AgeTable::new(age_table_size),
            }
        }
    }
}