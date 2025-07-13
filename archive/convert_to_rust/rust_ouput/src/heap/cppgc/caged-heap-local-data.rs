// Converted from V8 C++ source files:
// Header: N/A
// Implementation: caged-heap-local-data.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::{
    cmp::{max, min},
    mem::size_of,
  };

  pub const kCardSizeInBytes: usize = 256;

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum Age {
    kOld,
    kYoung,
    kMixed,
  }

  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum AdjacentCardsPolicy {
    kIgnore,
    kUpdate,
  }

  pub struct CagedHeapBase {}
  impl CagedHeapBase {
    pub fn GetAgeTableSize() -> usize {
      4096
    }
  }

  pub struct AgeTable {
    table_: [Age; 4096],
  }

  impl AgeTable {
    pub fn new() -> Self {
      Self {
        table_: [Age::kOld; 4096],
      }
    }

    fn RoundUp(value: usize, alignment: usize) -> usize {
      (value + alignment - 1) & !(alignment - 1)
    }

    fn RoundDown(value: usize, alignment: usize) -> usize {
      value & !(alignment - 1)
    }

    fn IsAligned(value: usize, alignment: usize) -> bool {
      value & (alignment - 1) == 0
    }

    fn SetAge(&mut self, offset: usize, age: Age) {
      let index = offset / kCardSizeInBytes;
      if index < self.table_.len() {
        self.table_[index] = age;
      }
    }

    fn GetAge(&self, offset: usize) -> Age {
      let index = offset / kCardSizeInBytes;
      if index < self.table_.len() {
        self.table_[index]
      } else {
        Age::kOld
      }
    }

    pub fn SetAgeForRange(
      &mut self,
      offset_begin: usize,
      offset_end: usize,
      age: Age,
      adjacent_cards_policy: AdjacentCardsPolicy,
    ) {
      let inner_card_offset_begin = Self::RoundUp(offset_begin, kCardSizeInBytes);
      let outer_card_offset_end = Self::RoundDown(offset_end, kCardSizeInBytes);

      for inner_offset in (inner_card_offset_begin..outer_card_offset_end).step_by(kCardSizeInBytes)
      {
        self.SetAge(inner_offset, age);
      }

      let set_age_for_outer_card =
        |this: &mut AgeTable, offset: usize| {
          if Self::IsAligned(offset, kCardSizeInBytes) {
            return;
          }
          if adjacent_cards_policy == AdjacentCardsPolicy::kIgnore {
            this.SetAge(offset, age);
          } else if this.GetAge(offset) != age {
            this.SetAge(offset, Age::kMixed);
          }
        };

      set_age_for_outer_card(self, offset_begin);
      set_age_for_outer_card(self, offset_end);
    }

    pub fn GetAgeForRange(&self, offset_begin: usize, offset_end: usize) -> Age {
      let mut result = self.GetAge(offset_begin);
      for offset in
        (offset_begin + kCardSizeInBytes..offset_end).step_by(kCardSizeInBytes)
      {
        if result != self.GetAge(offset) {
          result = Age::kMixed;
        }
      }
      return result;
    }

    pub fn ResetForTesting(&mut self) {
      self.table_.fill(Age::kOld);
    }
  }
}
