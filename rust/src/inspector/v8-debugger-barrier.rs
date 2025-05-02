// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_debugger_barrier {
    // Represents the V8InspectorClient trait
    pub trait V8InspectorClient {
        fn run_if_waiting_for_debugger(&self, context_group_id: i32);
    }

    /// A barrier that ensures the debugger is not waiting when it is dropped.
    pub struct V8DebuggerBarrier<'a> {
        client: &'a dyn V8InspectorClient,
        context_group_id: i32,
    }

    impl<'a> V8DebuggerBarrier<'a> {
        /// Creates a new `V8DebuggerBarrier`.
        pub fn new(client: &'a dyn V8InspectorClient, context_group_id: i32) -> Self {
            V8DebuggerBarrier {
                client,
                context_group_id,
            }
        }
    }

    impl<'a> Drop for V8DebuggerBarrier<'a> {
        /// Runs the debugger if it is waiting when the barrier is dropped.
        fn drop(&mut self) {
            self.client.run_if_waiting_for_debugger(self.context_group_id);
        }
    }
}