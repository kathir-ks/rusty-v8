// Converted from V8 C++ source files:
// Header: caged-heap-local-data.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod api_constants {
    pub const K_MB: usize = 1024 * 1024;
    pub const K_ALLOCATION_GRANULARITY: usize = 64;
    pub const K_CAGED_HEAP_DEFAULT_RESERVATION_SIZE: usize = 256 * K_MB;
}

pub mod logging {
    #[macro_export]
    macro_rules! cppgc_dcheck {
        ($condition:expr) => {
            if !$condition {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        };
    }
}

pub struct HeapBase {}
pub struct HeapBaseHandle {}

pub struct CagedHeapBase {}
impl CagedHeapBase {
    pub fn get_base() -> *mut u8 {
        // Return a dummy pointer for now, replace with actual logic later.
        static mut BASE: u8 = 0;
        unsafe { &mut BASE } as *mut u8
    }
    pub fn get_age_table_size() -> usize {
        1024 // A reasonable default for tests.
    }
}

#[cfg(feature = "cppgc_caged_heap")]
pub mod caged_heap_local_data {
    use super::*;
    use crate::api_constants;
    use crate::logging::cppgc_dcheck;

    #[cfg(feature = "cppgc_young_generation")]
    pub struct AgeTable {
        table: Vec<Age>,
    }

    #[cfg(feature = "cppgc_young_generation")]
    impl AgeTable {
        const K_REQUIRED_SIZE: usize = 1 * api_constants::K_MB;
        const K_ALLOCATION_GRANULARITY: usize = api_constants::K_ALLOCATION_GRANULARITY;

        pub const K_CARD_SIZE_IN_BYTES: usize =
            api_constants::K_CAGED_HEAP_DEFAULT_RESERVATION_SIZE / Self::K_REQUIRED_SIZE;

        pub const fn calculate_age_table_size_for_heap_size(heap_size: usize) -> usize {
            heap_size / Self::K_CARD_SIZE_IN_BYTES
        }

        pub fn new(heap_size: usize) -> Self {
            let size = Self::calculate_age_table_size_for_heap_size(heap_size);
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

        pub fn set_age_for_range(
            &mut self,
            cage_offset_begin: usize,
            cage_offset_end: usize,
            age: Age,
            adjacent_cards_policy: AdjacentCardsPolicy,
        ) {
            let begin_card = self.card(cage_offset_begin);
            let end_card = self.card(cage_offset_end);

            match adjacent_cards_policy {
                AdjacentCardsPolicy::KConsider => {
                    for i in begin_card..=end_card {
                        self.table[i] = age;
                    }
                }
                AdjacentCardsPolicy::KIgnore => {
                    for i in begin_card..=end_card {
                        self.table[i] = age;
                    }
                }
            }
        }

        pub fn get_age_for_range(&self, cage_offset_begin: usize, cage_offset_end: usize) -> Age {
            let begin_card = self.card(cage_offset_begin);
            let end_card = self.card(cage_offset_end);

            // For simplicity, return the age of the first card in the range.
            // A more sophisticated implementation might check if all cards in the range
            // have the same age and return that age, or return Mixed if the ages differ.
            self.table[begin_card]
        }

        pub fn reset_for_testing(&mut self) {
            for i in 0..self.table.len() {
                self.table[i] = Age::KOld;
            }
        }

        #[inline]
        fn card(&self, offset: usize) -> usize {
            const K_GRANULARITY_BITS: usize = 12;
            assert_eq!(1 << K_GRANULARITY_BITS, Self::K_CARD_SIZE_IN_BYTES);

            let entry = offset >> K_GRANULARITY_BITS;
            cppgc_dcheck!(super::CagedHeapBase::get_age_table_size() > entry);
            entry
        }
    }

    #[cfg(feature = "cppgc_young_generation")]
    impl Default for AgeTable {
        fn default() -> Self {
            // Provide a reasonable default heap size for tests.
            let heap_size = 64 * api_constants::K_MB;
            let size = Self::calculate_age_table_size_for_heap_size(heap_size);
            AgeTable {
                table: vec![Age::KOld; size],
            }
        }
    }

    #[cfg(feature = "cppgc_young_generation")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Age {
        KOld,
        KYoung,
        KMixed,
    }

    #[cfg(feature = "cppgc_young_generation")]
    pub enum AdjacentCardsPolicy {
        KConsider,
        KIgnore,
    }

    #[derive(Default)]
    pub struct CagedHeapLocalData {
        #[cfg(feature = "cppgc_young_generation")]
        pub age_table: AgeTable,
    }

    impl CagedHeapLocalData {
        #[inline]
        pub fn get() -> &'static mut CagedHeapLocalData {
            unsafe { &mut *(super::CagedHeapBase::get_base() as *mut CagedHeapLocalData) }
        }

        pub const fn calculate_local_data_size_for_heap_size(heap_size: usize) -> usize {
            AgeTable::calculate_age_table_size_for_heap_size(heap_size)
        }
    }
}
