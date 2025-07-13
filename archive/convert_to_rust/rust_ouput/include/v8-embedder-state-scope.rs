// Converted from V8 C++ source files:
// Header: v8-embedder-state-scope.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_embedder_state_scope {
    pub enum EmbedderStateTag {
        EMPTY = 0,
        OTHER = 1,
    }

    extern crate v8_sys;
    use std::ptr;
    use std::sync::Mutex;
    use std::sync::MutexGuard;

    pub struct EmbedderStateScope<'a> {
        isolate: *mut v8_sys::Isolate,
        context: v8_sys::Local<'a, v8_sys::Context>,
        tag: EmbedderStateTag,
        embedder_state_: Box<internal::EmbedderState>,
    }

    impl<'a> EmbedderStateScope<'a> {
        pub fn new(
            isolate: *mut v8_sys::Isolate,
            context: v8_sys::Local<'a, v8_sys::Context>,
            tag: EmbedderStateTag,
        ) -> Self {
            let embedder_state_ = Box::new(internal::EmbedderState::new());
            //push_embedder_state(isolate, &*embedder_state_);

            EmbedderStateScope {
                isolate,
                context,
                tag,
                embedder_state_,
            }
        }
    }

    impl<'a> Drop for EmbedderStateScope<'a> {
        fn drop(&mut self) {
            //pop_embedder_state(self.isolate);
            //drop(self.embedder_state_);
            //println!("EmbedderStateScope dropped");
        }
    }

    pub mod internal {
        pub struct EmbedderState {
           //data: i32,
        }

        impl EmbedderState {
            pub fn new() -> Self {
                EmbedderState {
                  //data:0,
                }
            }
        }
    }
}
