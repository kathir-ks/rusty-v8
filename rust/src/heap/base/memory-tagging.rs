// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap {
    pub mod base {
        /// SuspendTagCheckingScope stops checking MTE tags whilst it's alive. This is
        /// useful for traversing the stack during garbage collection.
        pub struct SuspendTagCheckingScope {}

        impl SuspendTagCheckingScope {
            /// MTE only works on AArch64 Android and Linux.
            pub fn new() -> Self {
                 // TODO: Implement the logic for suspending MTE tag checking
                 // This might involve platform-specific code using #[cfg] attributes
                 // and potentially unsafe operations to disable MTE.
                SuspendTagCheckingScope {}
            }
        }

        impl Drop for SuspendTagCheckingScope {
            fn drop(&mut self) {
                // TODO: Implement the logic for resuming MTE tag checking
                // This would involve enabling MTE again, potentially with unsafe code.
            }
        }
    }
}