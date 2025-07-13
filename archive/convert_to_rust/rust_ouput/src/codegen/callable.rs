// Converted from V8 C++ source files:
// Header: callable.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod callable {
    use crate::codegen::interface_descriptors::CallInterfaceDescriptor;
    use crate::v8::internal::Code;
    use std::rc::Rc;

    // Associates a body of code with an interface descriptor.
    #[derive(Clone)]
    pub struct Callable {
        code_: Rc<Code>,
        descriptor_: CallInterfaceDescriptor,
    }

    impl Callable {
        pub fn new(code: Rc<Code>, descriptor: CallInterfaceDescriptor) -> Self {
            Callable {
                code_: code,
                descriptor_: descriptor,
            }
        }

        pub fn code(&self) -> Rc<Code> {
            self.code_.clone()
        }

        pub fn descriptor(&self) -> CallInterfaceDescriptor {
            self.descriptor_.clone()
        }
    }
} // namespace internal
} // namespace v8
