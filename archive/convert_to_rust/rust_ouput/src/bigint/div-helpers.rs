// Converted from V8 C++ source files:
// Header: div-helpers.h
// Implementation: div-helpers.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bigint {
    use std::cmp::min;
    use std::mem::MaybeUninit;
    use std::ptr;

    const kDigitBits: i32 = 32;
    pub type digit_t = u32;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        MemoryAllocationError,
        Other(String),
    }

    pub struct Digits<'a> {
        digits_: *mut digit_t,
        len_: usize,
        _phantom: std::marker::PhantomData<&'a digit_t>,
    }

    impl<'a> Digits<'a> {
        pub fn new(digits: *mut digit_t, len: usize) -> Self {
            Digits {
                digits_: digits,
                len_: len,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn len(&self) -> usize {
            self.len_
        }

        pub fn msd(&self) -> digit_t {
            if self.len_ > 0 {
                unsafe { *self.digits_.add(self.len_ - 1) }
            } else {
                0
            }
        }

        pub fn normalize(&mut self) {
            while self.len_ > 0 && unsafe { *self.digits_.add(self.len_ - 1) } == 0 {
                self.len_ -= 1;
            }
        }
    }

    impl<'a> std::ops::Index<usize> for Digits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.len_ {
                panic!("Index out of bounds");
            }
            unsafe { &*self.digits_.add(index) }
        }
    }

    pub struct RWDigits<'a> {
        digits_: *mut digit_t,
        len_: usize,
        _phantom: std::marker::PhantomData<&'a digit_t>,
    }

    impl<'a> RWDigits<'a> {
        pub fn new(digits: *mut digit_t, len: usize) -> Self {
            RWDigits {
                digits_: digits,
                len_: len,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn len(&self) -> usize {
            self.len_
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.len_ {
                panic!("Index out of bounds");
            }
            unsafe { &*self.digits_.add(index) }
        }
    }

    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index >= self.len_ {
                panic!("Index out of bounds");
            }
            unsafe { &mut *self.digits_.add(index) }
        }
    }

    fn count_leading_zeros(x: digit_t) -> i32 {
        x.leading_zeros() as i32
    }

    pub fn put_at(z: RWDigits, a: Digits, count: usize) {
        let len = min(a.len(), count);
        let mut i = 0;
        for _ in 0..len {
            unsafe {
                *z.digits_.add(i) = *a.digits_.add(i);
            }
            i += 1;
        }
        for _ in i..count {
            unsafe {
                *z.digits_.add(i) = 0;
            }
            i += 1;
        }
    }

    pub struct ShiftedDigits<'a> {
        digits_: *mut digit_t,
        len_: usize,
        shift_: i32,
        inplace_: bool,
        storage_: Option<Box<[digit_t]>>,
        _phantom: std::marker::PhantomData<&'a digit_t>,
    }

    impl<'a> ShiftedDigits<'a> {
        pub fn new(original: &mut Digits<'a>, shift: i32, allow_inplace: bool) -> Self {
            let leading_zeros = count_leading_zeros(original.msd());
            let mut shift_val = shift;
            let mut allow_inplace_val = allow_inplace;
            let mut len_val = original.len_;

            if shift_val < 0 {
                shift_val = leading_zeros;
            } else if shift_val > leading_zeros {
                allow_inplace_val = false;
                len_val += 1;
            }

            if shift_val == 0 {
                return ShiftedDigits {
                    digits_: original.digits_,
                    len_: len_val,
                    shift_: shift_val,
                    inplace_: true,
                    storage_: None,
                    _phantom: std::marker::PhantomData,
                };
            }

            let mut digits_: *mut digit_t;
            let mut storage_: Option<Box<[digit_t]>> = None;
            let inplace_: bool;

            if !allow_inplace_val {
                let mut digits = Vec::with_capacity(len_val);
                unsafe {
                  digits.set_len(len_val);
                }
                storage_ = Some(digits.into_boxed_slice());
                digits_ = storage_.as_mut().unwrap().as_mut_ptr() as *mut digit_t;
                inplace_ = false;
            } else {
                digits_ = original.digits_;
                inplace_ = true;
            }

            let rw_view = RWDigits {
                digits_: digits_,
                len_: len_val,
                _phantom: std::marker::PhantomData,
            };

            left_shift(rw_view, Digits { digits_:original.digits_, len_: original.len_, _phantom: std::marker::PhantomData }, shift_val);

            ShiftedDigits {
                digits_: digits_,
                len_: len_val,
                shift_: shift_val,
                inplace_: inplace_,
                storage_: storage_,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn reset(&mut self) {
            if self.inplace_ {
                let rw_view = RWDigits {
                    digits_: self.digits_,
                    len_: self.len_,
                    _phantom: std::marker::PhantomData,
                };
                right_shift(rw_view, RWDigits { digits_: self.digits_, len_: self.len_, _phantom: std::marker::PhantomData }, self.shift_);
            }
        }

        pub fn shift(&self) -> i32 {
            self.shift_
        }
    }

    // Z := X << shift
    // Z and X may alias for an in-place shift.
    pub fn left_shift(z: RWDigits, x: Digits, shift: i32) {
        assert!(shift >= 0);
        assert!(shift < kDigitBits);
        assert!(z.len() >= x.len());

        if shift == 0 {
            copy(z, x);
            return;
        }

        let mut carry: digit_t = 0;
        let mut i = 0;

        for _ in 0..x.len() {
            unsafe {
                let d: digit_t = *x.digits_.add(i);
                *z.digits_.add(i) = (d << shift) | carry;
                carry = d >> (kDigitBits - shift);
            }
            i += 1;
        }

        if i < z.len() {
            unsafe {
                *z.digits_.add(i) = carry;
            }
            i += 1;
        } else {
            assert!(carry == 0);
        }

        for _ in i..z.len() {
            unsafe {
                *z.digits_.add(i) = 0;
            }
            i += 1;
        }
    }

    // Z := X >> shift
    // Z and X may alias for an in-place shift.
    pub fn right_shift(mut z: RWDigits, mut x: RWDigits, shift: i32) {
        assert!(shift >= 0);
        assert!(shift < kDigitBits);

        let mut digits_struct = Digits {
            digits_: x.digits_,
            len_: x.len_,
            _phantom: std::marker::PhantomData
        };

        digits_struct.normalize();
        assert!(z.len() >= digits_struct.len());

        if shift == 0 {
            copy(z, digits_struct);
            return;
        }

        let mut i = 0;

        if digits_struct.len() > 0 {
            unsafe {
                let mut carry: digit_t = *digits_struct.digits_.add(0) >> shift;
                let last = digits_struct.len() - 1;

                for _ in 0..last {
                    let d: digit_t = *digits_struct.digits_.add(i + 1);
                    *z.digits_.add(i) = (d << (kDigitBits - shift)) | carry;
                    carry = d >> shift;
                    i += 1;
                }

                *z.digits_.add(i) = carry;
                i += 1;
            }
        }

        for _ in i..z.len() {
            unsafe {
                *z.digits_.add(i) = 0;
            }
            i += 1;
        }
    }

    fn copy(z: RWDigits, x: Digits) {
        if z.digits_ == x.digits_ {
            return;
        }

        let mut i = 0;
        for _ in 0..x.len() {
            unsafe {
                *z.digits_.add(i) = *x.digits_.add(i);
            }
            i += 1;
        }

        for _ in i..z.len() {
            unsafe {
                *z.digits_.add(i) = 0;
            }
            i += 1;
        }
    }
}
