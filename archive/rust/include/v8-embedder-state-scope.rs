// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod embedder_state_scope {
    /// Represents a possible state of the embedder.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EmbedderStateTag {
        /// Reserved
        EMPTY = 0,
        /// Other
        OTHER = 1,
        // Embedder can define any state after
    }

    // Placeholder for Isolate.  In a real integration,
    // this would be a Rust struct representing the V8 isolate.
    pub struct Isolate {}

    // Placeholder for Context. In a real integration,
    // this would be a Rust struct representing the V8 context.
    pub struct Context {}

    impl Context {
        pub fn new() -> Self {
            Context {}
        }
    }

    pub mod internal {
        /// Placeholder for EmbedderState. In a real integration,
        /// this would represent the internal embedder state.
        pub struct EmbedderState {}

        impl EmbedderState {
            pub fn new() -> Self {
                EmbedderState {}
            }
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    /// A stack-allocated class that manages an embedder state on the isolate.
    /// After an EmbedderState scope has been created, a new embedder state will be
    /// pushed on the isolate stack.
    pub struct EmbedderStateScope {
        embedder_state_: Rc<RefCell<internal::EmbedderState>>,
    }

    impl EmbedderStateScope {
        /// Creates a new EmbedderStateScope.
        pub fn new(isolate: &mut Isolate, context: &Context, tag: EmbedderStateTag) -> Self {
            // Use isolate and context (and tag) to initialize the embedder state
            // appropriately.  This is a placeholder.

            // The below logic is incomplete. In C++, the embedder state is pushed
            // onto an isolate stack. In Rust, we would need to maintain this stack
            // within the `Isolate` struct, and access it from within this `new` function.
            // Since we do not have the real implementation of `Isolate` struct,
            // this implementation is just a placeholder to demonstrate the intention.
            let embedder_state = Rc::new(RefCell::new(internal::EmbedderState::new()));
            EmbedderStateScope {
                embedder_state_: embedder_state,
            }
        }
    }

    impl Drop for EmbedderStateScope {
        /// Destroys the EmbedderStateScope.  This pops the embedder state from the
        /// isolate stack.
        fn drop(&mut self) {
            // The below logic is incomplete. In C++, the embedder state is popped
            // from an isolate stack. In Rust, we would need to maintain this stack
            // within the `Isolate` struct, and access it from within this `drop` function.
            // Since we do not have the real implementation of `Isolate` struct,
            // this implementation is just a placeholder to demonstrate the intention.

            //Dropping embedder_state_ here, which should remove the state from the isolate.
            //Rc::strong_count(&self.embedder_state_) // should return 1 here.
            //println!("Dropping an EmbedderStateScope");
        }
    }
}