// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_int;
use std::fmt;
use std::any::Any;

mod internals {
    pub const kSmiTagSize: usize = 1;
    pub const kSmiShiftSize: usize = 0;
    pub const kSmiTag: usize = 0;
    pub const kSmiMinValue: i32 = -1073741824;
    pub const kSmiMaxValue: i32 = 1073741823;

    #[inline]
    pub fn integral_to_smi(value: i32) -> usize {
        (value as usize) << (super::kSmiTagSize + super::kSmiShiftSize)
    }

    #[inline]
    pub fn is_valid_smi<T: num::Integer + num::Signed>(value: T) -> bool {
        value >= T::from(kSmiMinValue).unwrap() && value <= T::from(kSmiMaxValue).unwrap()
    }
}

const kSmiTagSize: usize = internals::kSmiTagSize;
const kSmiShiftSize: usize = internals::kSmiShiftSize;
const kSmiValueSize: usize = 31;
const kSmiMinValue: i32 = internals::kSmiMinValue;
const kSmiMaxValue: i32 = internals::kSmiMaxValue;
const kNullAddress: usize = 0;

/// A tagged pointer.  In V8, tagged pointers have the least significant bit
/// set to indicate whether it is a pointer to an object on the heap, or
/// whether it is a small integer (Smi).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tagged<T> {
    ptr: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    #[inline]
    pub const fn new(ptr: usize) -> Self {
        Tagged { ptr, _phantom: std::marker::PhantomData }
    }

    #[inline]
    pub const fn value(&self) -> usize {
        self.ptr
    }

    #[inline]
    pub fn from_raw(ptr: usize) -> Self {
        Tagged { ptr, _phantom: std::marker::PhantomData }
    }

    #[inline]
    pub fn to_raw(self) -> usize {
        self.ptr
    }
}

// Marker trait for objects that are allocated on the heap.
pub trait HeapObject {}

pub struct Object {}

impl HeapObject for Object {}

// Smi represents integer Numbers that can be stored in 31 bits.
// Smis are immediate which means they are NOT allocated in the heap.
// The ptr_ value has the following format: [31 bit signed int] 0
// For long smis it has the following format:
//     [32 bit signed int] [31 bits zero padding] 0
// Smi stands for small integer.
pub struct Smi {}

impl Smi {
    #[inline]
    pub const fn to_uint32_smi(smi: Tagged<Smi>) -> Tagged<Smi> {
        if smi.value() <= 0 {
            Smi::from_int(0)
        } else {
            Smi::from_int(smi.value() as u32 as i32)
        }
    }

    // Convert a Smi object to an int.
    #[inline]
    pub fn to_int(object: Tagged<Object>) -> i32 {
        Tagged::<Smi>::new(object.ptr).value() as i32
    }

    // Convert a value to a Smi object.
    #[inline]
    pub const fn from_int(value: i32) -> Tagged<Smi> {
        debug_assert!(Smi::is_valid(value));
        Tagged::<Smi>::new((value as usize) << (kSmiTagSize + kSmiShiftSize))
    }

    #[inline]
    pub const fn from_intptr(value: isize) -> Tagged<Smi> {
        debug_assert!(Smi::is_valid(value));
        let smi_shift_bits = kSmiTagSize + kSmiShiftSize;
        Tagged::<Smi>::new(((value as usize) << smi_shift_bits) | internals::kSmiTag)
    }

    // Given {value} in [0, 2^31-1], force it into Smi range by changing at most
    // the MSB (leaving the lower 31 bit unchanged).
    #[inline]
    pub const fn from_31bit_pattern(value: i32) -> Tagged<Smi> {
        Smi::from_int((value << (32 - kSmiValueSize)) >> (32 - kSmiValueSize))
    }

    #[inline]
    pub fn from_enum<E: num::ToPrimitive>(value: E) -> Tagged<Smi> {
        Smi::from_int(value.to_i32().unwrap())
    }

    // Returns whether value can be represented in a Smi.
    #[inline]
    pub fn is_valid<T: num::Integer + num::Signed>(value: T) -> bool {
        internals::is_valid_smi(value)
    }

    // Returns whether value can be represented in a Smi.
    #[inline]
    pub fn is_valid_unsigned<T: num::Integer + num::Unsigned>(value: T) -> bool {
        internals::is_valid_smi(value)
    }

    // Compare two Smis x, y as if they were converted to strings and then
    // compared lexicographically. Returns:
    // -1 if x < y.
    //  0 if x == y.
    //  1 if x > y.
    // Returns the result (a tagged Smi) as a raw Address for ExternalReference
    // usage.
    // TODO: Implement LexicographicCompare
    pub fn lexicographic_compare(_isolate: &Isolate, x: Tagged<Smi>, y: Tagged<Smi>) -> usize {
        if x.value() < y.value() {
            -1
        } else if x.value() > y.value() {
            1
        } else {
            0
        }
    }

    // Dispatched behavior.
    // TODO: Implement SmiPrint
    pub fn smi_print(smi: Tagged<Smi>, os: &mut dyn std::io::Write) {
        write!(os, "{}", smi.value()).unwrap();
    }

    // Since this is a constexpr, "calling" it is just as efficient
    // as reading a constant.
    #[inline]
    pub const fn zero() -> Tagged<Smi> {
        Smi::from_int(0)
    }

    // Smi value for filling in not-yet initialized tagged field values with a
    // valid tagged pointer. A field value equal to this doesn't necessarily
    // indicate that a field is uninitialized, but an uninitialized field should
    // definitely equal this value.
    //
    // This _has_ to be kNullAddress, so that an uninitialized field value read as
    // an embedded pointer field is interpreted as nullptr. This is so that
    // uninitialised embedded pointers are not forwarded to the embedder as part
    // of embedder tracing (and similar mechanisms), as nullptrs are skipped for
    // those cases and otherwise the embedder would try to dereference the
    // uninitialized pointer value.
    #[inline]
    pub const fn uninitialized_deserialization_value() -> Tagged<Smi> {
        Tagged::<Smi>::new(kNullAddress)
    }
}

pub struct Isolate {}