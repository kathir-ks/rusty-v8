// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod codegen {
    pub mod callable {
        use crate::codegen::interface_descriptors::CallInterfaceDescriptor;
        use crate::utils::allocation::Ownable;
        use std::rc::Rc;

        // Placeholder for InstructionStream, as its definition is not available.
        pub struct InstructionStream {}

        /// Associates a body of code with an interface descriptor.
        #[derive(Clone)]
        pub struct Callable {
            code: Rc<Code>,
            descriptor: CallInterfaceDescriptor,
        }

        impl Callable {
            pub fn new(code: Rc<Code>, descriptor: CallInterfaceDescriptor) -> Self {
                Callable {
                    code,
                    descriptor,
                }
            }

            pub fn code(&self) -> Rc<Code> {
                self.code.clone()
            }

            pub fn descriptor(&self) -> CallInterfaceDescriptor {
                self.descriptor.clone()
            }
        }

        // Placeholder for Code, as its definition is not available.
        #[derive(Clone)]
        pub struct Code {}

        impl Code {
            pub fn new() -> Self {
                Code {}
            }
        }
    }
    pub mod interface_descriptors {
        #[derive(Clone)]
        pub struct CallInterfaceDescriptor {}
    }
}

pub mod utils {
    pub mod allocation {
        pub trait Ownable {}
    }
}