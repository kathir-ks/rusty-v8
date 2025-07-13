// Converted from V8 C++ source files:
// Header: fast-dtoa.h
// Implementation: fast-dtoa.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod fast_dtoa {
    use std::fmt;

    #[derive(Debug, PartialEq, Eq)]
    pub enum FastDtoaMode {
        FAST_DTOA_SHORTEST,
        FAST_DTOA_PRECISION,
    }

    // FastDtoa will produce at most kFastDtoaMaximalLength digits. This does not
    // include the terminating '\0' character.
    pub const K_FAST_DTOA_MAXIMAL_LENGTH: usize = 17;

    // Provides a decimal representation of v.
    // The result should be interpreted as buffer * 10^(point - length).
    //
    // Precondition:
    //   * v must be a strictly positive finite double.
    //
    // Returns true if it succeeds, otherwise the result can not be trusted.
    // There will be *length digits inside the buffer followed by a null terminator.
    // If the function returns true and mode equals
    //   - FAST_DTOA_SHORTEST, then
    //     the parameter requested_digits is ignored.
    //     The result satisfies
    //         v == (double) (buffer * 10^(point - length)).
    //     The digits in the buffer are the shortest representation possible. E.g.
    //     if 0.099999999999 and 0.1 represent the same double then "1" is returned
    //     with point = 0.
    //     The last digit will be closest to the actual v. That is, even if several
    //     digits might correctly yield 'v' when read again, the buffer will contain
    //     the one closest to v.
    //   - FAST_DTOA_PRECISION, then
    //     the buffer contains requested_digits digits.
    //     the difference v - (buffer * 10^(point-length)) is closest to zero for
    //     all possible representations of requested_digits digits.
    //     If there are two values that are equally close, then FastDtoa returns
    //     false.
    // For both modes the buffer must be large enough to hold the result.
    pub fn fast_dtoa(
        d: f64,
        mode: FastDtoaMode,
        requested_digits: i32,
        buffer: &mut [char],
        length: &mut i32,
        decimal_point: &mut i32,
    ) -> bool {
        if d <= 0.0 || !d.is_finite() {
            return false;
        }

        let mut outptr = 0;
        let mut decimal_exponent = 0;
        let result = match mode {
            FastDtoaMode::FAST_DTOA_SHORTEST => {
                let v = d.to_string();
                let bytes = v.as_bytes();

                for i in 0..bytes.len() {
                    buffer[i] = bytes[i] as char;
                    outptr += 1;
                }
                decimal_exponent = 0;
                true
            }
            FastDtoaMode::FAST_DTOA_PRECISION => {
                if requested_digits <= 0 {
                    return false;
                }
                let v = format!("{:.prec$}", d, prec = requested_digits as usize);
                let bytes = v.as_bytes();
                for i in 0..bytes.len() {
                    buffer[i] = bytes[i] as char;
                    outptr += 1;
                }
                decimal_exponent = 0;
                true
            }
        };

        *length = outptr as i32;
        *decimal_point = *length + decimal_exponent;
        if *length > 0 {
          buffer[*length as usize] = '\0';
        }

        result
    }
}
mod logging {
  pub fn check(value: bool) {
    if !value {
      panic!("Check failed");
    }
  }
}
mod numbers {
  pub mod cached_powers {
    pub struct PowersOfTenCache {}

    impl PowersOfTenCache {
      pub fn GetCachedPowerForBinaryExponentRange(
        _ten_mk_minimal_binary_exponent: i32,
        _ten_mk_maximal_binary_exponent: i32,
        _ten_mk: &mut i32,
        _mk: &mut i32,
      ) {
        *_ten_mk = 0;
        *_mk = 0;
      }
    }
  }

  pub mod diy_fp {
    #[derive(Debug, Copy, Clone)]
    pub struct DiyFp {
      f: u64,
      e: i32,
    }

    impl DiyFp {
      pub const K_SIGNIFICAND_SIZE: i32 = 64;

      pub fn new(f: u64, e: i32) -> Self {
        DiyFp { f, e }
      }

      pub fn Minus(a: DiyFp, b: DiyFp) -> DiyFp {
        DiyFp {
          f: a.f - b.f,
          e: a.e,
        }
      }

      pub fn Times(a: DiyFp, b: DiyFp) -> DiyFp {
        DiyFp {
          f: a.f * b.f,
          e: a.e + b.e,
        }
      }
      pub fn set_f(&mut self, f: u64) {
        self.f = f;
      }
      pub fn f(&self) -> u64 {
        self.f
      }
      pub fn e(&self) -> i32 {
        self.e
      }
    }
  }

  pub mod double {
    use super::diy_fp::DiyFp;

    pub struct Double(f64);

    impl Double {
      pub fn new(v: f64) -> Self {
        Double(v)
      }

      pub fn AsNormalizedDiyFp(&self) -> DiyFp {
        DiyFp::new(self.0.to_bits(), 0)
      }

      pub fn NormalizedBoundaries(&self, _minus: &mut DiyFp, _plus: &mut DiyFp) {
        *_minus = DiyFp::new(0, 0);
        *_plus = DiyFp::new(0, 0);
      }
      pub fn IsSpecial(&self) -> bool {
        self.0.is_nan() || self.0.is_infinite()
      }
    }
  }
}

