// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This header should only be included if WebAssembly is enabled.");

use std::cell::RefCell;
use std::rc::Rc;

// Placeholder for AccountingAllocator.  Needs a real implementation.
pub struct AccountingAllocator {}

impl AccountingAllocator {
    pub fn new() -> Self {
        AccountingAllocator {}
    }
}

// Placeholder for Zone.  Needs a real implementation.
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

pub mod compiler {
    // Placeholder for CallDescriptor. Needs a real implementation.
    #[derive(Debug)]
    pub struct CallDescriptor {}

    impl CallDescriptor {
        pub fn new() -> Self {
            CallDescriptor {}
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    use super::{AccountingAllocator, Zone};

    pub struct WasmCallDescriptors {
        zone_: Rc<RefCell<Zone>>,
        bigint_to_i64_descriptor_: Rc<CallDescriptor>,
        bigint_to_i64_descriptor_with_framestate_: Rc<CallDescriptor>,

        #[cfg(target_arch = "x86_32")]
        bigint_to_i32pair_descriptor_: Rc<CallDescriptor>,
        #[cfg(target_arch = "x86_32")]
        bigint_to_i32pair_descriptor_with_framestate_: Rc<CallDescriptor>,
    }

    impl WasmCallDescriptors {
        pub fn new(allocator: &mut AccountingAllocator) -> Self {
            let zone_ = Rc::new(RefCell::new(Zone::new()));
            WasmCallDescriptors {
                zone_: zone_,
                bigint_to_i64_descriptor_: Rc::new(CallDescriptor::new()),
                bigint_to_i64_descriptor_with_framestate_: Rc::new(CallDescriptor::new()),
                #[cfg(target_arch = "x86_32")]
                bigint_to_i32pair_descriptor_: Rc::new(CallDescriptor::new()),
                #[cfg(target_arch = "x86_32")]
                bigint_to_i32pair_descriptor_with_framestate_: Rc::new(CallDescriptor::new()),
            }
        }

        pub fn get_bigint_to_i64_descriptor(&self, needs_frame_state: bool) -> Rc<CallDescriptor> {
            if needs_frame_state {
                Rc::clone(&self.bigint_to_i64_descriptor_with_framestate_)
            } else {
                Rc::clone(&self.bigint_to_i64_descriptor_)
            }
        }

        #[cfg(target_arch = "x86_32")]
        pub fn get_lowered_call_descriptor(&self, original: &CallDescriptor) -> Rc<CallDescriptor> {
            // The original C++ version has some architecture-specific code for 32-bit architectures
            // that may require a more complete implementation based on the actual use case.
            // This is a placeholder.
            Rc::new(CallDescriptor::new())
        }

        #[cfg(not(target_arch = "x86_32"))]
        pub fn get_lowered_call_descriptor(&self, original: &CallDescriptor) -> Rc<CallDescriptor> {
            // The original C++ version has UNREACHABLE() here, which means the function should never be called
            // on non-32-bit architectures.  We panic here to match that behavior.
            panic!("UNREACHABLE: get_lowered_call_descriptor should not be called on non-32-bit architectures");
        }
    }
}