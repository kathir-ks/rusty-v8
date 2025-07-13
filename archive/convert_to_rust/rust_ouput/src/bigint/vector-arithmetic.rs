// Converted from V8 C++ source files:
// Header: vector-arithmetic.h
// Implementation: vector-arithmetic.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bigint {
    use crate::bigint::digit_arithmetic::*;

    pub type digit_t = u32; // Assuming digit_t is u32

    #[derive(Copy, Clone, Debug)]
    pub struct Digits<'a> {
        data: &'a [digit_t],
    }

    impl<'a> Digits<'a> {
        pub fn new(data: &'a [digit_t]) -> Self {
            Digits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn msd(&self) -> digit_t {
            if self.is_empty() {
                0
            } else {
                self.data[self.len() - 1]
            }
        }
        
        pub fn get(&self, index: usize) -> Option<digit_t> {
            self.data.get(index).copied()
        }
    }

    #[derive(Debug)]
    pub struct RWDigits<'a> {
        data: &'a mut [digit_t],
    }

    impl<'a> RWDigits<'a> {
        pub fn new(data: &'a mut [digit_t]) -> Self {
            RWDigits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<'a> Digits<'a> {
        pub fn normalize(&self) -> Digits<'a> {
            let mut len = self.len();
            while len > 0 && self.data[len - 1] == 0 {
                len -= 1;
            }
            Digits::new(&self.data[..len])
        }
    }

    impl<'a> RWDigits<'a> {
        pub fn normalize(&mut self) {
            let mut len = self.len();
            while len > 0 && self.data[len - 1] == 0 {
                len -= 1;
            }
            self.data = &mut self.data[..len];
        }
    }

    // Z += X. Returns carry on overflow.
    pub fn add_and_return_overflow(z: RWDigits, x: Digits) -> digit_t {
        let x_normalized = x.normalize();
        if x_normalized.len() == 0 {
            return 0;
        }

        let mut carry: digit_t = 0;
        let mut i: usize = 0;

        for _ in 0..x_normalized.len() {
            if let (Some(z_i), Some(x_i)) = (z.data.get(i).cloned(), x_normalized.data.get(i).cloned()) {
                let result = digit_add3(z_i, x_i, carry, &mut carry);
                z.data[i] = result;
                i += 1;
            } else {
                break;
            }
        }
        

        for _ in i..z.len() {
            if carry != 0 {
                if let Some(z_i) = z.data.get(i).cloned() {
                  let result = digit_add2(z_i, carry, &mut carry);
                  z.data[i] = result;
                  i += 1;
                } else {
                  break;
                }
            } else {
                break;
            }
        }

        carry
    }

    // Z -= X. Returns borrow on overflow.
    pub fn sub_and_return_borrow(mut z: RWDigits, x: Digits) -> digit_t {
        let x_normalized = x.normalize();
        if x_normalized.len() == 0 {
            return 0;
        }

        let mut borrow: digit_t = 0;
        let mut i: usize = 0;

        for _ in 0..x_normalized.len() {
            if let (Some(z_i), Some(x_i)) = (z.data.get(i).cloned(), x_normalized.data.get(i).cloned()) {
                let result = digit_sub2(z_i, x_i, borrow, &mut borrow);
                z.data[i] = result;
                i += 1;
            } else {
                break;
            }
        }
        
        for _ in i..z.len() {
            if borrow != 0 {
              if let Some(z_i) = z.data.get(i).cloned() {
                let result = digit_sub(z_i, borrow, &mut borrow);
                z.data[i] = result;
                i += 1;
              } else {
                break;
              }
            } else {
                break;
            }
        }

        borrow
    }

    // X += y.
    pub fn add(x: &mut [digit_t], y: digit_t) {
        let mut carry: digit_t = y;
        let mut i: usize = 0;
        while carry != 0 {
            if i < x.len() {
                let mut temp_carry = 0;
                x[i] = digit_add2(x[i], carry, &mut temp_carry);
                carry = temp_carry;
                i += 1;
            } else {
                break;
            }
        }
    }

    // X -= y.
    pub fn subtract(x: &mut [digit_t], y: digit_t) {
        let mut borrow: digit_t = y;
        let mut i: usize = 0;
        while borrow != 0 {
            if i < x.len() {
                x[i] = digit_sub(x[i], borrow, &mut borrow);
                i += 1;
            } else {
                break;
            }
        }
    }

    // These add exactly Y's digits to the matching digits in X, storing the
    // result in (part of) Z, and return the carry/borrow.
    pub fn add_and_return_carry(z: &mut [digit_t], x: Digits, y: Digits) -> digit_t {
        assert!(z.len() >= y.len() && x.len() >= y.len());

        let mut carry: digit_t = 0;
        for i in 0..y.len() {
            if let (Some(x_i), Some(y_i)) = (x.data.get(i).cloned(), y.data.get(i).cloned()) {
                z[i] = digit_add3(x_i, y_i, carry, &mut carry);
            }
        }
        carry
    }

    pub fn subtract_and_return_borrow(z: &mut [digit_t], x: Digits, y: Digits) -> digit_t {
        assert!(z.len() >= y.len() && x.len() >= y.len());
        let mut borrow: digit_t = 0;
        for i in 0..y.len() {
            if let (Some(x_i), Some(y_i)) = (x.data.get(i).cloned(), y.data.get(i).cloned()) {
                z[i] = digit_sub2(x_i, y_i, borrow, &mut borrow);
            }
        }
        borrow
    }

    pub fn is_digit_normalized(x: Digits) -> bool {
        x.len() == 0 || x.msd() != 0
    }

    pub fn is_bit_normalized(x: Digits) -> bool {
        (x.msd() >> (k_digit_bits - 1)) == 1
    }

    pub fn greater_than_or_equal(a: Digits, b: Digits) -> bool {
        compare(a, b) >= 0
    }

    pub fn bit_length(x: Digits) -> i32 {
        x.len() as i32 * k_digit_bits as i32 - count_leading_zeros(x.msd()) as i32
    }

    pub fn add_digits(mut z: RWDigits, x: Digits, y: Digits) {
        if x.len() < y.len() {
            return add_digits(z, Digits::new(y.data), Digits::new(x.data));
        }
        let mut i: usize = 0;
        let mut carry: digit_t = 0;
        for _ in 0..y.len() {
            if let (Some(x_i), Some(y_i)) = (x.data.get(i).cloned(), y.data.get(i).cloned()) {
              z.data[i] = digit_add3(x_i, y_i, carry, &mut carry);
              i += 1;
            } else {
                break;
            }
        }
        
        for _ in i..x.len() {
          if let Some(x_i) = x.data.get(i).cloned() {
            z.data[i] = digit_add2(x_i, carry, &mut carry);
            i += 1;
          } else {
            break;
          }
        }
        
        for _ in i..z.len() {
          z.data[i] = carry;
          carry = 0;
          i += 1;
        }
    }

    pub fn subtract_digits(mut z: RWDigits, x: Digits, y: Digits) {
        let x_normalized = x.normalize();
        let y_normalized = y.normalize();
        assert!(x_normalized.len() >= y_normalized.len());
        let mut i: usize = 0;
        let mut borrow: digit_t = 0;
        for _ in 0..y_normalized.len() {
            if let (Some(x_i), Some(y_i)) = (x_normalized.data.get(i).cloned(), y_normalized.data.get(i).cloned()) {
              z.data[i] = digit_sub2(x_i, y_i, borrow, &mut borrow);
              i += 1;
            } else {
                break;
            }
        }

        for _ in i..x_normalized.len() {
            if let Some(x_i) = x_normalized.data.get(i).cloned() {
              z.data[i] = digit_sub(x_i, borrow, &mut borrow);
              i += 1;
            } else {
              break;
            }
        }

        assert!(borrow == 0);
        for _ in i..z.len() {
          z.data[i] = 0;
          i += 1;
        }
    }

    pub fn add_signed(
        mut z: RWDigits,
        x: Digits,
        x_negative: bool,
        y: Digits,
        y_negative: bool,
    ) -> bool {
        if x_negative == y_negative {
            add_digits(z, x, y);
            return x_negative;
        }
        if greater_than_or_equal(x, y) {
            subtract_digits(z, x, y);
            return x_negative;
        }
        subtract_digits(z, y, x);
        return !x_negative;
    }

    pub fn subtract_signed(
        mut z: RWDigits,
        x: Digits,
        x_negative: bool,
        y: Digits,
        y_negative: bool,
    ) -> bool {
        if x_negative != y_negative {
            add_digits(z, x, y);
            return x_negative;
        }
        if greater_than_or_equal(x, y) {
            subtract_digits(z, x, y);
            return x_negative;
        }
        subtract_digits(z, y, x);
        return !x_negative;
    }

    pub fn add_one(mut z: RWDigits, x: Digits) {
        let mut carry: digit_t = 1;
        let mut i: usize = 0;
        for _ in 0..x.len() {
          if carry > 0 {
            if let Some(x_i) = x.data.get(i).cloned() {
              z.data[i] = digit_add2(x_i, carry, &mut carry);
              i += 1;
            } else {
              break;
            }
          } else {
            break;
          }
        }
        if carry > 0 {
          if i < z.len() {
            z.data[i] = carry;
            i += 1;
          }
        }
        for _ in i..x.len() {
          if let Some(x_i) = x.data.get(i).cloned() {
            z.data[i] = x_i;
            i += 1;
          } else {
            break;
          }
        }
        for _ in i..z.len() {
          z.data[i] = 0;
          i += 1;
        }
    }

    pub fn subtract_one(mut z: RWDigits, x: Digits) {
        let mut borrow: digit_t = 1;
        let mut i: usize = 0;
        while borrow > 0 {
            z.data[i] = digit_sub(x.data[i], borrow, &mut borrow);
            i += 1;
        }
        for _ in i..x.len() {
          z.data[i] = x.data[i];
          i += 1;
        }
        for _ in i..z.len() {
          z.data[i] = 0;
          i += 1;
        }
    }
    
    fn compare(a: Digits, b: Digits) -> i32 {
        if a.len() > b.len() {
            return 1;
        } else if a.len() < b.len() {
            return -1;
        } else {
            for i in (0..a.len()).rev() {
                if a.data[i] > b.data[i] {
                    return 1;
                } else if a.data[i] < b.data[i] {
                    return -1;
                }
            }
            return 0;
        }
    }

} // namespace bigint
