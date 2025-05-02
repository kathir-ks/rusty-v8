// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a Rust translation of v8/src/builtins/builtins-utils-inl.h

pub mod builtins_utils {
    // Re-exporting from builtins-utils.rs to keep module structure similar to C++
    pub use crate::builtins::builtins_utils::*;
}

pub mod execution {
    // Re-exporting from arguments.rs to keep module structure similar to C++
    pub use crate::execution::arguments::*;
}

pub mod internal {
    use crate::builtins::builtins_utils::BuiltinArguments;
    use crate::objects::js_objects::JSAny;
    use crate::objects::js_objects::JSFunction;
    use crate::objects::heap_object::HeapObject;
    use crate::isolate::Isolate;
    use crate::objects::object::Object;
    use crate::base::handle::Handle;

    impl BuiltinArguments {
        /// Returns the argument at the given index, or undefined if the index is out of bounds.
        pub fn at_or_undefined(&self, isolate: &mut Isolate, index: usize) -> Handle<Object> {
            if index >= self.length() {
                isolate.factory().undefined_value()
            } else {
                self.at::<Object>(index)
            }
        }

        /// Returns the receiver of the builtin call.
        pub fn receiver(&self) -> Handle<JSAny> {
            Handle::new(self.address_of_arg_at(execution::arguments::kReceiverIndex))
        }

        /// Returns the target of the builtin call.
        pub fn target(&self) -> Handle<JSFunction> {
            Handle::new(self.address_of_arg_at(execution::arguments::kTargetIndex))
        }

        /// Returns the new target of the builtin call.
        pub fn new_target(&self) -> Handle<HeapObject> {
            Handle::new(self.address_of_arg_at(execution::arguments::kNewTargetIndex))
        }
    }
}