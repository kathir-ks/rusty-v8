// Converted from V8 C++ source files:
// Header: compaction-worklists.h
// Implementation: compaction-worklists.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compaction_worklists {
    use std::collections::HashSet;

    pub struct CompactionWorklists {
        movable_slots_worklist: MovableReferencesWorklist,
    }

    impl CompactionWorklists {
        pub fn new() -> Self {
            CompactionWorklists {
                movable_slots_worklist: MovableReferencesWorklist::new(),
            }
        }

        pub fn movable_slots_worklist(&mut self) -> &mut MovableReferencesWorklist {
            &mut self.movable_slots_worklist
        }

        pub fn clear_for_testing(&mut self) {
            self.movable_slots_worklist.clear();
        }
    }

    pub type MovableReference = *const std::ffi::c_void;

    pub struct MovableReferencesWorklist {
        items: Vec<MovableReference>,
        local_entries: usize,
    }

    impl MovableReferencesWorklist {
        const LOCAL_ENTRIES: usize = 256;

        pub fn new() -> Self {
            MovableReferencesWorklist {
                items: Vec::new(),
                local_entries: Self::LOCAL_ENTRIES,
            }
        }

        pub fn push(&mut self, item: MovableReference) {
            self.items.push(item);
        }

        pub fn pop(&mut self) -> Option<MovableReference> {
            self.items.pop()
        }

        pub fn clear(&mut self) {
            self.items.clear();
        }

        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }
}
