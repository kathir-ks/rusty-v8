pub mod heap_snapshot_generator {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    pub struct HeapEntry {
        index_: usize,
        children_count_: usize, // Part of a union with children_end_index_
        children_end_index_: usize,
        snapshot_: Rc<HeapSnapshot>,
    }

    impl HeapEntry {
        pub fn set_children_index(&mut self, index: usize) -> usize {
            let next_index = index + self.children_count_;
            self.children_end_index_ = index;
            next_index
        }

        pub fn add_child(&mut self, edge: Rc<HeapGraphEdge>) {
            self.snapshot_.children.lock().unwrap()[self.children_end_index_] = edge;
            self.children_end_index_ += 1;
        }

        pub fn child(&self, i: usize) -> Rc<HeapGraphEdge> {
            self.children_begin()[i].clone()
        }

        pub fn children_begin(&self) -> std::slice::Iter<Rc<HeapGraphEdge>> {
            if self.index_ == 0 {
                self.snapshot_.children.lock().unwrap().iter()
            } else {
                let prev_entry = &self.snapshot_.entries[self.index_ - 1];
                prev_entry.children_end()
            }
        }

        pub fn children_end(&self) -> std::slice::Iter<Rc<HeapGraphEdge>> {
            assert!(self.children_end_index_ >= 0);
            self.snapshot_.children.lock().unwrap().iter().take(self.children_end_index_)
        }

        pub fn children_count(&self) -> usize {
            self.children_end_index_ - self.children_begin().len()
        }

        pub fn isolate(&self) -> &Isolate {
            self.snapshot_.profiler.isolate()
        }

        pub fn index(&self) -> usize {
            self.index_
        }
    }

    pub struct HeapGraphEdge {
        from_index_: usize,
        to_entry_: Rc<HeapEntry>,
    }

    impl HeapGraphEdge {
        pub fn from(&self) -> &HeapEntry {
            &self.snapshot().entries[self.from_index()]
        }

        pub fn isolate(&self) -> &Isolate {
            self.to_entry_.isolate()
        }

        pub fn snapshot(&self) -> &HeapSnapshot {
            &self.to_entry_.snapshot()
        }
    }

    pub struct HeapSnapshot {
        entries: Vec<HeapEntry>,
        children: Mutex<Vec<Rc<HeapGraphEdge>>>,
        profiler: Rc<HeapProfiler>,
    }

    impl HeapSnapshot {
        //Need implementations from C++
    }

    pub struct HeapProfiler {
        isolate: Isolate,
    }

    impl HeapProfiler {
        pub fn isolate(&self) -> &Isolate {
            &self.isolate
        }
    }

    pub struct Isolate {}

    pub struct HeapSnapshotJSONSerializer {
        trace_function_count_: usize,
    }

    impl HeapSnapshotJSONSerializer {
        pub fn string_hash(string: &str) -> u32 {
            string_hasher::hash_sequential_string(string, string_hasher::K_ZERO_HASH_SEED)
        }

        pub fn to_node_index_entry(&self, e: &HeapEntry) -> usize {
            self.to_node_index_int(e.index())
        }

        pub fn to_node_index_int(&self, entry_index: usize) -> usize {
            entry_index
                * (if self.trace_function_count_ > 0 {
                    K_NODE_FIELDS_COUNT_WITH_TRACE_NODE_ID
                } else {
                    K_NODE_FIELDS_COUNT_WITHOUT_TRACE_NODE_ID
                })
        }
    }

    const K_NODE_FIELDS_COUNT_WITH_TRACE_NODE_ID: usize = 10;
    const K_NODE_FIELDS_COUNT_WITHOUT_TRACE_NODE_ID: usize = 8;

    pub mod string_hasher {
        pub const K_ZERO_HASH_SEED: u32 = 0;

        pub fn hash_sequential_string(s: &str, seed: u32) -> u32 {
            let mut hash = seed;
            for byte in s.bytes() {
                hash = hash * 31 + byte as u32;
            }
            hash
        }
    }
}