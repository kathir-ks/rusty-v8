// Converted from V8 C++ source files:
// Header: double.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod numbers {
        use std::mem::transmute;

        use crate::base::macros::Use;

        use super::fast_dtoa::DiyFp;

        // We assume that doubles and uint64_t have the same endianness.
        #[inline]
        pub const fn double_to_uint64(d: f64) -> u64 {
            unsafe { transmute(d) }
        }
        #[inline]
        pub const fn uint64_to_double(d64: u64) -> f64 {
            unsafe { transmute(d64) }
        }

        // Helper functions for doubles.
        pub struct Double {
            d64_: u64,
        }

        impl Double {
            pub const kSignMask: u64 = 0x8000_0000_0000_0000;
            pub const kExponentMask: u64 = 0x7FF0_0000_0000_0000;
            pub const kSignificandMask: u64 = 0x000F_FFFF_FFFF_FFFF;
            pub const kHiddenBit: u64 = 0x0010_0000_0000_0000;
            pub const kPhysicalSignificandSize: i32 =
                52; // Excludes the hidden bit.
            pub const kSignificandSize: i32 = 53;

            pub const fn new() -> Self {
                Double { d64_: 0 }
            }
            pub const fn from_double(d: f64) -> Self {
                Double {
                    d64_: double_to_uint64(d),
                }
            }
            pub const fn from_uint64(d64: u64) -> Self {
                Double { d64_: d64 }
            }
            pub const fn from_diy_fp(diy_fp: DiyFp) -> Self {
                Double {
                    d64_: Self::diy_fp_to_uint64(diy_fp),
                }
            }

            // The value encoded by this Double must be greater or equal to +0.0.
            // It must not be special (infinity, or NaN).
            pub fn as_diy_fp(&self) -> DiyFp {
                if self.sign() <= 0 {
                    panic!("Sign must be greater than 0");
                }
                if self.is_special() {
                    panic!("Must not be special");
                }
                DiyFp::new(self.significand(), self.exponent())
            }

            // The value encoded by this Double must be strictly greater than 0.
            pub fn as_normalized_diy_fp(&self) -> DiyFp {
                if self.value() <= 0.0 {
                    panic!("Value must be greater than 0.0");
                }
                let mut f = self.significand();
                let mut e = self.exponent();

                // The current double could be a denormal.
                while (f & Self::kHiddenBit) == 0 {
                    f <<= 1;
                    e -= 1;
                }
                // Do the final shifts in one go.
                f <<= DiyFp::kSignificandSize - Self::kSignificandSize;
                e -= DiyFp::kSignificandSize - Self::kSignificandSize;
                DiyFp::new(f, e)
            }

            // Returns the double's bit as uint64.
            pub const fn as_uint64(&self) -> u64 {
                self.d64_
            }

            // Returns the next greater double. Returns +infinity on input +infinity.
            pub const fn next_double(&self) -> f64 {
                if self.d64_ == Self::kInfinity {
                    return Double::from_uint64(Self::kInfinity).value();
                }
                if self.sign() < 0 && self.significand() == 0 {
                    // -0.0
                    return 0.0;
                }
                if self.sign() < 0 {
                    return Double::from_uint64(self.d64_ - 1).value();
                } else {
                    return Double::from_uint64(self.d64_ + 1).value();
                }
            }

            pub const fn exponent(&self) -> i32 {
                if self.is_denormal() {
                    return Self::kDenormalExponent;
                }

                let d64 = self.as_uint64();
                let biased_e =
                    ((d64 & Self::kExponentMask) >> Self::kPhysicalSignificandSize)
                        as i32;
                biased_e - Self::kExponentBias
            }

            pub const fn significand(&self) -> u64 {
                let d64 = self.as_uint64();
                let significand = d64 & Self::kSignificandMask;
                if !self.is_denormal() {
                    significand + Self::kHiddenBit
                } else {
                    significand
                }
            }

            // Returns true if the double is a denormal.
            pub const fn is_denormal(&self) -> bool {
                let d64 = self.as_uint64();
                (d64 & Self::kExponentMask) == 0
            }

            // We consider denormals not to be special.
            // Hence only Infinity and NaN are special.
            pub const fn is_special(&self) -> bool {
                let d64 = self.as_uint64();
                (d64 & Self::kExponentMask) == Self::kExponentMask
            }

            pub const fn is_infinite(&self) -> bool {
                let d64 = self.as_uint64();
                ((d64 & Self::kExponentMask) == Self::kExponentMask)
                    && ((d64 & Self::kSignificandMask) == 0)
            }

            pub const fn sign(&self) -> i32 {
                let d64 = self.as_uint64();
                if (d64 & Self::kSignMask) == 0 {
                    1
                } else {
                    -1
                }
            }

            // Precondition: the value encoded by this Double must be greater or equal
            // than +0.0.
            pub fn upper_boundary(&self) -> DiyFp {
                if self.sign() <= 0 {
                    panic!("Sign must be greater than 0");
                }
                DiyFp::new(self.significand() * 2 + 1, self.exponent() - 1)
            }

            // Returns the two boundaries of this.
            // The bigger boundary (m_plus) is normalized. The lower boundary has the same
            // exponent as m_plus.
            // Precondition: the value encoded by this Double must be greater than 0.
            pub fn normalized_boundaries(
                &self,
                out_m_minus: &mut DiyFp,
                out_m_plus: &mut DiyFp,
            ) {
                if self.value() <= 0.0 {
                    panic!("Value must be greater than 0.0");
                }
                let v = self.as_diy_fp();
                let mut m_plus = DiyFp::normalize(DiyFp::new((v.f() << 1) + 1, v.e() - 1));
                let mut m_minus;
                if (self.as_uint64() & Self::kSignificandMask) == 0
                    && v.e() != Self::kDenormalExponent
                {
                    // The boundary is closer. Think of v = 1000e10 and v- = 9999e9.
                    // Then the boundary (== (v - v-)/2) is not just at a distance of 1e9 but
                    // at a distance of 1e8.
                    // The only exception is for the smallest normal: the largest denormal is
                    // at the same distance as its successor.
                    // Note: denormals have the same exponent as the smallest normals.
                    m_minus = DiyFp::new((v.f() << 2) - 1, v.e() - 2);
                } else {
                    m_minus = DiyFp::new((v.f() << 1) - 1, v.e() - 1);
                }
                m_minus.set_f(m_minus.f() << (m_minus.e() - m_plus.e()));
                m_minus.set_e(m_plus.e());
                *out_m_plus = m_plus;
                *out_m_minus = m_minus;
            }

            pub const fn value(&self) -> f64 {
                uint64_to_double(self.d64_)
            }

            // Returns the significand size for a given order of magnitude.
            // If v = f*2^e with 2^p-1 <= f <= 2^p then p+e is v's order of magnitude.
            // This function returns the number of significant binary digits v will have
            // once its encoded into a double. In almost all cases this is equal to
            // kSignificandSize. The only exception are denormals. They start with leading
            // zeroes and their effective significand-size is hence smaller.
            pub fn significand_size_for_order_of_magnitude(order: i32) -> i32 {
                if order >= (Self::kDenormalExponent + Self::kSignificandSize) {
                    Self::kSignificandSize
                } else if order <= Self::kDenormalExponent {
                    0
                } else {
                    order - Self::kDenormalExponent
                }
            }

            const fn diy_fp_to_uint64(diy_fp: DiyFp) -> u64 {
                let mut significand = diy_fp.f();
                let mut exponent = diy_fp.e();
                while significand > Self::kHiddenBit + Self::kSignificandMask {
                    significand >>= 1;
                    exponent += 1;
                }
                if exponent >= Self::kMaxExponent {
                    return Self::kInfinity;
                }
                if exponent < Self::kDenormalExponent {
                    return 0;
                }
                while exponent > Self::kDenormalExponent && (significand & Self::kHiddenBit) == 0 {
                    significand <<= 1;
                    exponent -= 1;
                }
                let biased_exponent: u64;
                if exponent == Self::kDenormalExponent && (significand & Self::kHiddenBit) == 0 {
                    biased_exponent = 0;
                } else {
                    biased_exponent = (exponent + Self::kExponentBias) as u64;
                }
                (significand & Self::kSignificandMask)
                    | (biased_exponent << Self::kPhysicalSignificandSize)
            }

            const kExponentBias: i32 = 0x3FF + Self::kPhysicalSignificandSize;
            const kDenormalExponent: i32 = -Self::kExponentBias + 1;
            const kMaxExponent: i32 = 0x7FF - Self::kExponentBias;
            const kInfinity: u64 = 0x7FF0_0000_0000_0000;
        }
    }
}
