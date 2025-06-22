// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ieee754 {

    /// Returns `x` to the power of `y`.
    ///
    /// The result of base ** exponent when base is 1 or -1 and exponent is
    /// +Infinity or -Infinity differs from IEEE 754-2008. The first edition
    /// of ECMAScript specified a result of NaN for this operation, whereas
    /// later versions of IEEE 754-2008 specified 1. The historical ECMAScript
    /// behaviour is preserved for compatibility reasons.
    pub fn pow(x: f64, y: f64) -> f64 {
        // This is a placeholder.  The actual implementation would need to
        // preserve the historical ECMAScript behavior.
        x.powf(y)
    }
}