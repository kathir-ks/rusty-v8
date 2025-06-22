// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/sandbox/code-pointer-inl.h

// TODO: Implement the CodePointer, CodePointerTable, IsolateGroup, and related types/functions.
// These are currently stubbed out.

mod code_pointer {
    // Stub for CodePointerHandle and CodeEntrypointTag.  Replace with actual definitions.
    pub type CodePointerHandle = u32;
    pub type CodeEntrypointTag = u32;
}

mod code_pointer_table {
    use super::code_pointer::*;

    // Stub for CodePointerTable. Replace with actual implementation.
    pub struct CodePointerTable {}

    impl CodePointerTable {
        pub fn get_entrypoint(&self, _handle: CodePointerHandle, _tag: CodeEntrypointTag) -> usize {
            // Placeholder implementation. Replace with actual logic.
            0
        }

        pub fn set_entrypoint(&self, _handle: CodePointerHandle, _value: usize, _tag: CodeEntrypointTag) {
            // Placeholder implementation. Replace with actual logic.
        }
    }
}

mod isolate {
    use super::code_pointer_table::*;
    use std::cell::RefCell;

    // Stub for IsolateGroup. Replace with actual implementation.
    pub struct IsolateGroup {
        code_pointer_table: CodePointerTable,
    }

    impl IsolateGroup {
        pub fn new() -> Self {
            IsolateGroup {
                code_pointer_table: CodePointerTable {},
            }
        }

        pub fn code_pointer_table(&self) -> &CodePointerTable {
            &self.code_pointer_table
        }

        // Mutable access not needed in this conversion, but could be required in future.
        // pub fn code_pointer_table_mut(&mut self) -> &mut CodePointerTable {
        //     &mut self.code_pointer_table
        // }
    }

    thread_local! {
        static CURRENT_ISOLATE_GROUP: RefCell<Option<IsolateGroup>> = RefCell::new(None);
    }

    impl IsolateGroup {
        pub fn current() -> &'static IsolateGroup {
            CURRENT_ISOLATE_GROUP.with(|isolate_group| {
                let mut ig = isolate_group.borrow_mut();
                if ig.is_none() {
                    *ig = Some(IsolateGroup::new());
                }
                ig.as_ref().unwrap()
            })
        }
    }
}

pub mod code_pointer_inl {
    use super::code_pointer::*;
    use super::isolate::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    //V8_INLINE Address ReadCodeEntrypointViaCodePointerField(Address field_address, CodeEntrypointTag tag)
    #[cfg(feature = "v8_enable_sandbox")]
    pub fn read_code_entrypoint_via_code_pointer_field(
        field_address: usize,
        tag: CodeEntrypointTag,
    ) -> usize {
        // Handles may be written to objects from other threads so the handle needs
        // to be loaded atomically. We assume that the load from the table cannot
        // be reordered before the load of the handle due to the data dependency
        // between the two loads and therefore use relaxed memory ordering, but
        // technically we should use memory_order_consume here.
        let location = field_address as *mut AtomicU32;
        let handle = unsafe { (*location).load(Ordering::Relaxed) as CodePointerHandle };
        IsolateGroup::current().code_pointer_table().get_entrypoint(handle, tag)
    }

    #[cfg(not(feature = "v8_enable_sandbox"))]
    pub fn read_code_entrypoint_via_code_pointer_field(
        _field_address: usize,
        _tag: CodeEntrypointTag,
    ) -> usize {
        panic!("UNREACHABLE");
    }

    // V8_INLINE void WriteCodeEntrypointViaCodePointerField(Address field_address, Address value, CodeEntrypointTag tag)
    #[cfg(feature = "v8_enable_sandbox")]
    pub fn write_code_entrypoint_via_code_pointer_field(
        field_address: usize,
        value: usize,
        tag: CodeEntrypointTag,
    ) {
        // See comment above for why this is a Relaxed_Load.
        let location = field_address as *mut AtomicU32;
        let handle = unsafe { (*location).load(Ordering::Relaxed) as CodePointerHandle };
        IsolateGroup::current()
            .code_pointer_table()
            .set_entrypoint(handle, value, tag);
    }

    #[cfg(not(feature = "v8_enable_sandbox"))]
    pub fn write_code_entrypoint_via_code_pointer_field(
        _field_address: usize,
        _value: usize,
        _tag: CodeEntrypointTag,
    ) {
        panic!("UNREACHABLE");
    }
}