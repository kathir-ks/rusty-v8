// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashSet;

mod heap_base {
    pub struct Worklist<T, const N: usize> {
        items: Vec<T>,
    }

    impl<T, const N: usize> Worklist<T, const N> {
        pub fn new() -> Self {
            Worklist { items: Vec::new() }
        }

        pub fn push(&mut self, item: T) {
            self.items.push(item);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        pub fn clear(&mut self) {
            self.items.clear();
        }
    }
}

pub mod internal {
    use super::heap_base::Worklist;

    /// Manages worklists used during compaction.
    pub struct CompactionWorklists {
        movable_slots_worklist: MovableReferencesWorklist,
    }

    impl CompactionWorklists {
        /// Creates a new `CompactionWorklists`.
        pub fn new() -> Self {
            CompactionWorklists {
                movable_slots_worklist: MovableReferencesWorklist::new(),
            }
        }

        /// A reference to a movable object.  In C++ this is `const void*`.
        pub type MovableReference = *const std::ffi::c_void;

        /// A worklist for movable references.
        pub type MovableReferencesWorklist = Worklist<MovableReference, 256>;

        /// Returns a mutable reference to the worklist for movable slots.
        pub fn movable_slots_worklist(&mut self) -> &mut MovableReferencesWorklist {
            &mut self.movable_slots_worklist
        }

        /// Clears the worklists for testing purposes.
        pub fn clear_for_testing(&mut self) {
            self.movable_slots_worklist.clear();
        }
    }
}