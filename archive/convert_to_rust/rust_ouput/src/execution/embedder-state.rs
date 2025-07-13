// Converted from V8 C++ source files:
// Header: embedder-state.h
// Implementation: embedder-state.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub struct Isolate {}
    pub struct Context {}
    pub type Local<'a, T> = &'a T;
}

pub mod internal {
    use super::v8;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Address {
        address: usize,
    }

    impl Address {
        pub fn is_null(&self) -> bool {
            self.address == 0
        }
        pub fn address(&self) -> usize {
            self.address
        }
    }

    pub const kNullAddress: Address = Address { address: 0 };

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum EmbedderStateTag {
        Tag1,
        Tag2,
    }

    pub struct Isolate {
        current_embedder_state: *mut EmbedderState,
    }

    impl Isolate {
        pub fn current_embedder_state(&self) -> *mut EmbedderState {
            self.current_embedder_state
        }

        pub fn set_current_embedder_state(&mut self, state: *mut EmbedderState) {
            self.current_embedder_state = state;
        }
    }

    pub struct EmbedderState {
        isolate_: *mut Isolate,
        tag_: EmbedderStateTag,
        native_context_address_: Address,
        previous_embedder_state_: *mut EmbedderState,
    }

    impl EmbedderState {
        pub fn new(
            isolate: *mut v8::Isolate,
            context: v8::Local<v8::Context>,
            tag: EmbedderStateTag,
        ) -> EmbedderState {
            let isolate_internal = unsafe { &mut *(isolate as *mut Isolate) };
            let native_context_address = if !context.is_null() {
                let context_ptr = context as *const v8::Context;
                Address {
                    address: context_ptr as usize
                }
            } else {
                kNullAddress
            };
            let previous_embedder_state_ = isolate_internal.current_embedder_state();

            let mut new_state = EmbedderState {
                isolate_: isolate,
                tag_: tag,
                native_context_address_: native_context_address,
                previous_embedder_state_: previous_embedder_state_,
            };

            assert_ne!(&new_state as *const EmbedderState, isolate_internal.current_embedder_state());
            isolate_internal.set_current_embedder_state(&mut new_state as *mut EmbedderState);
            new_state
        }

        pub fn get_state(&self) -> EmbedderStateTag {
            self.tag_
        }

        pub fn native_context_address(&self) -> Address {
            self.native_context_address_
        }

        pub fn on_move_event(&mut self, from: Address, to: Address) {
            let mut state: *mut EmbedderState = self;
            loop {
                let current_state = unsafe { &mut *state };
                if current_state.native_context_address_ == from {
                    current_state.native_context_address_ = to;
                }
                state = current_state.previous_embedder_state_;
                if state.is_null() {
                    break;
                }
            }
        }
    }

    impl Drop for EmbedderState {
        fn drop(&mut self) {
            let isolate_internal = unsafe { &mut *(self.isolate_ as *mut Isolate) };
            assert_eq!(self as *const EmbedderState, isolate_internal.current_embedder_state() as *const EmbedderState);
            isolate_internal.set_current_embedder_state(self.previous_embedder_state_);
        }
    }
}
