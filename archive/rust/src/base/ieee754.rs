// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ieee754 {

    /// Returns the arc cosine of `x`; that is the value whose cosine is `x`.
    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    /// Returns the inverse hyperbolic cosine of `x`; that is the value whose
    /// hyperbolic cosine is `x`.
    pub fn acosh(x: f64) -> f64 {
        x.acosh()
    }

    /// Returns the arc sine of `x`; that is the value whose sine is `x`.
    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    /// Returns the inverse hyperbolic sine of `x`; that is the value whose
    /// hyperbolic sine is `x`.
    pub fn asinh(x: f64) -> f64 {
        x.asinh()
    }

    /// Returns the principal value of the arc tangent of `x`; that is the value
    /// whose tangent is `x`.
    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    /// Returns the principal value of the arc tangent of `y/x`, using the signs of
    /// the two arguments to determine the quadrant of the result.
    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "v8_use_libm_trig_functions")] {
            // To ensure there aren't problems with libm's sin/cos, both implementations
            // are shipped. The plan is to transition to libm once we ensure there are no
            // compatibility or performance issues.
            /// fdlibm version of sin
            pub fn fdlibm_sin(x: f64) -> f64 {
               // Placeholder. Needs a proper fdlibm implementation.
                x.sin()
            }

            /// fdlibm version of cos
            pub fn fdlibm_cos(x: f64) -> f64 {
                // Placeholder. Needs a proper fdlibm implementation.
                x.cos()
            }

            /// libm version of sin
            pub fn libm_sin(x: f64) -> f64 {
                x.sin()
            }

            /// libm version of cos
            pub fn libm_cos(x: f64) -> f64 {
                x.cos()
            }
        } else {
            /// Standard cos
            pub fn cos(x: f64) -> f64 {
                x.cos()
            }

            /// Standard sin
            pub fn sin(x: f64) -> f64 {
                x.sin()
            }
        }
    }

    /// Returns the base-e exponential of `x`.
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    /// Returns the inverse hyperbolic tangent of `x`.
    pub fn atanh(x: f64) -> f64 {
        x.atanh()
    }

    /// Returns the natural logarithm of `x`.
    pub fn log(x: f64) -> f64 {
        x.ln()
    }

    /// Returns a value equivalent to `log(1+x)`, but computed in a way that is
    /// accurate even if the value of `x` is near zero.
    pub fn log1p(x: f64) -> f64 {
        x.ln_1p()
    }

    /// Returns the base 2 logarithm of `x`.
    pub fn log2(x: f64) -> f64 {
        x.log2()
    }

    /// Returns the base 10 logarithm of `x`.
    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    /// Returns the cube root of `x`.
    pub fn cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    /// Returns exp(x)-1, the exponential of `x` minus 1.
    pub fn expm1(x: f64) -> f64 {
        x.exp_m1()
    }

    pub mod legacy {
        /// This function should not be used directly. Instead, use
        /// v8::internal::math::pow.
        ///
        /// Returns `x` to the power of `y`.
        /// The result of base ** exponent when base is 1 or -1 and exponent is
        /// +Infinity or -Infinity differs from IEEE 754-2008. The first edition
        /// of ECMAScript specified a result of NaN for this operation, whereas
        /// later versions of IEEE 754-2008 specified 1. The historical ECMAScript
        /// behaviour is preserved for compatibility reasons.
        pub fn pow(x: f64, y: f64) -> f64 {
            if (x == 1.0 || x == -1.0) && (y.is_infinite()) {
                f64::NAN
            } else {
                x.powf(y)
            }
        }
    } // namespace legacy

    /// Returns the tangent of `x`, where `x` is given in radians.
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    /// Returns the hyperbolic cosine of `x`, where `x` is given radians.
    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    /// Returns the hyperbolic sine of `x`, where `x` is given radians.
    pub fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    /// Returns the hyperbolic tangent of `x`, where `x` is given radians.
    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }
} // namespace ieee754