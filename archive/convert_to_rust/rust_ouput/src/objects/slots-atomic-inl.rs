// Converted from V8 C++ source files:
// Header: slots-atomic-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod atomic_utils {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct AtomicTagged {
        address: AtomicUsize,
    }

    impl AtomicTagged {
        pub fn new(initial_value: usize) -> Self {
            AtomicTagged {
                address: AtomicUsize::new(initial_value),
            }
        }

        pub fn Relaxed_Load(&self) -> usize {
            self.address.load(Ordering::Relaxed)
        }

        pub fn Relaxed_Store(&self, value: usize) {
            self.address.store(value, Ordering::Relaxed)
        }
    }
}
}
pub mod objects {
pub mod compressed_slots {
    pub struct SlotBase<T, Tagged_t> {
        address: usize,
        _phantom_t: std::marker::PhantomData<T>,
        _phantom_tagged_t: std::marker::PhantomData<Tagged_t>,
    }

    impl<T, Tagged_t> SlotBase<T, Tagged_t> {
        pub fn new(address: usize) -> Self {
            SlotBase {
                address,
                _phantom_t: std::marker::PhantomData,
                _phantom_tagged_t: std::marker::PhantomData,
            }
        }

        pub fn address(&self) -> usize {
            self.address
        }
    }
}
pub mod slots {
    use super::compressed_slots::SlotBase;
    pub struct ObjectSlot {
        address: usize,
    }

    impl ObjectSlot {
        pub fn address(&self) -> usize {
            self.address
        }
    }

    pub struct MaybeObjectSlot {
        address: usize,
    }

    impl MaybeObjectSlot {
        pub fn address(&self) -> usize {
            self.address
        }
    }
}
}
const kNullAddress: usize = 0;
const kTaggedSize: usize = 8;

use std::marker::PhantomData;
use std::iter::Iterator;
use std::cmp::{Ordering, PartialEq, PartialOrd};

use crate::base::atomic_utils::AtomicTagged;
use crate::objects::slots::{ObjectSlot, MaybeObjectSlot};

#[derive(Clone, Copy)]
pub struct AtomicSlot {
    address: usize,
}

impl AtomicSlot {
    pub fn new() -> Self {
        AtomicSlot { address: kNullAddress }
    }

    pub fn from_address(address: usize) -> Self {
        AtomicSlot { address }
    }

    pub fn from_object_slot(slot: ObjectSlot) -> Self {
        AtomicSlot { address: slot.address() }
    }

    pub fn from_maybe_object_slot(slot: MaybeObjectSlot) -> Self {
        AtomicSlot { address: slot.address() }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn operator_star(&self) -> Reference {
        Reference::new(self.address as *mut Tagged_t)
    }

    pub fn operator_index(&self, i: isize) -> Reference {
        Reference::new((self.address as isize + i * kTaggedSize as isize) as *mut Tagged_t)
    }
}

impl PartialEq for AtomicSlot {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl PartialOrd for AtomicSlot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl std::ops::Sub for AtomicSlot {
    type Output = isize;

    fn sub(self, other: Self) -> Self::Output {
        (self.address as isize - other.address as isize) / kTaggedSize as isize
    }
}

pub struct Reference {
    address_: *mut Tagged_t,
}

impl Reference {
    pub fn new(address: *mut Tagged_t) -> Self {
        Reference { address_: address }
    }

    pub fn operator_equals(&mut self, other: &Reference) -> &mut Self {
        unsafe {
            AtomicTagged { address: self.address_ as usize }.Relaxed_Store(AtomicTagged { address: other.address_ as usize }.Relaxed_Load() as usize);
        }
        self
    }

    pub fn operator_equals_value(&mut self, value: Tagged_t) -> &mut Self {
        unsafe {
            AtomicTagged { address: self.address_ as usize }.Relaxed_Store(value as usize);
        }
        self
    }

    pub fn value(&self) -> Tagged_t {
        unsafe { AtomicTagged { address: self.address_ as usize }.Relaxed_Load() as Tagged_t }
    }

    pub fn swap(&mut self, other: &mut Reference) {
        let tmp = self.value();
        unsafe {
            AtomicTagged { address: self.address_ as usize }.Relaxed_Store(other.value() as usize);
            AtomicTagged { address: other.address_ as usize }.Relaxed_Store(tmp as usize);
        }
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Reference {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

pub type Tagged_t = usize;
