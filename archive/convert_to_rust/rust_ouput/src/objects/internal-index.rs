// Converted from V8 C++ source files:
// Header: internal-index.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::cmp::{max, min};
    use std::fmt;
    use std::fmt::Formatter;
    use std::marker::PhantomData;
    use std::num::TryFromIntError;
    use std::ops::{Add, Deref, DerefMut, Mul, Sub};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::{i32, i64, u32, usize};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InternalIndex {
        entry_: usize,
    }

    impl InternalIndex {
        pub const fn new(raw: usize) -> Self {
            InternalIndex { entry_: raw }
        }

        pub fn not_found() -> Self {
            InternalIndex {
                entry_: usize::MAX,
            }
        }

        pub fn adjust_down(&self, subtract: usize) -> Result<Self, String> {
            if self.entry_ < subtract {
                Err("subtract is greater than entry_".to_string())
            } else {
                Ok(InternalIndex {
                    entry_: self.entry_ - subtract,
                })
            }
        }

        pub fn adjust_up(&self, add: usize) -> Result<Self, String> {
            if self.entry_ > usize::MAX - add {
                Err("add would cause overflow".to_string())
            } else {
                Ok(InternalIndex {
                    entry_: self.entry_ + add,
                })
            }
        }

        pub fn is_found(&self) -> bool {
            self.entry_ != usize::MAX
        }

        pub fn is_not_found(&self) -> bool {
            self.entry_ == usize::MAX
        }

        pub fn raw_value(&self) -> usize {
            self.entry_
        }

        pub fn as_uint32(&self) -> Result<u32, String> {
            if self.entry_ > u32::MAX as usize {
                Err("entry_ is greater than u32::MAX".to_string())
            } else {
                Ok(self.entry_ as u32)
            }
        }

        pub fn as_int(&self) -> Result<i32, String> {
            if self.entry_ > i32::MAX as usize {
                Err("entry_ is greater than i32::MAX".to_string())
            } else {
                Ok(self.entry_ as i32)
            }
        }
    }

    impl Iterator for InternalIndex {
        type Item = InternalIndex;

        fn next(&mut self) -> Option<Self::Item> {
            if self.entry_ != usize::MAX {
                let current = *self;
                self.entry_ += 1;
                Some(current)
            } else {
                None
            }
        }
    }

    impl fmt::Debug for InternalIndex {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "InternalIndex({})", self.entry_)
        }
    }

    impl InternalIndex {
        pub struct Range {
            min_: usize,
            max_: usize,
        }

        impl Range {
            pub fn new(max: usize) -> Self {
                Range { min_: 0, max_: max }
            }

            pub fn new_with_min(min: usize, max: usize) -> Self {
                Range { min_: min, max_: max }
            }

            pub fn begin(&self) -> InternalIndex {
                InternalIndex { entry_: self.min_ }
            }

            pub fn end(&self) -> InternalIndex {
                InternalIndex { entry_: self.max_ }
            }
        }
    }
}
