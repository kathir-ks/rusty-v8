// Converted from V8 C++ source files:
// Header: late-escape-analysis-reducer.h
// Implementation: late-escape-analysis-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod late_escape_analysis_reducer {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    pub struct Graph {
        operations: Vec<Operation>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                operations: Vec::new(),
            }
        }

        pub fn add_operation(&mut self, op: Operation) -> OpIndex {
            let index = OpIndex(self.operations.len());
            self.operations.push(op);
            index
        }

        pub fn get(&self, index: OpIndex) -> &Operation {
            &self.operations[index.0]
        }

        pub fn get_mut(&mut self, index: OpIndex) -> &mut Operation {
            &mut self.operations[index.0]
        }

        pub fn index(&self, op: &Operation) -> OpIndex {
            for (i, o) in self.operations.iter().enumerate() {
                if std::ptr::eq(op, o) {
                    return OpIndex(i);
                }
            }
            panic!("Operation not found in graph.");
        }

        pub fn all_operations(&mut self) -> &mut Vec<Operation> {
            &mut self.operations
        }

        pub fn kill_operation(&mut self, index: OpIndex) {
            self.operations[index.0].kind = OperationKind::Dead;
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OpIndex(pub usize);

    #[derive(Debug, Clone, Copy)]
    pub enum OperationKind {
        Allocate,
        Store,
        Other,
        Dead,
    }

    #[derive(Clone, Copy)]
    pub struct SaturatedUseCount {
        count: usize,
    }

    impl SaturatedUseCount {
        pub fn get(&self) -> usize {
            self.count
        }
    }

    #[derive(Clone)]
    pub struct Operation {
        pub kind: OperationKind,
        pub inputs: Vec<OpIndex>,
        pub saturated_use_count: SaturatedUseCount,
    }

    impl Operation {
        pub fn new(kind: OperationKind, inputs: Vec<OpIndex>) -> Self {
            Operation {
                kind,
                inputs,
                saturated_use_count: SaturatedUseCount { count: 0 },
            }
        }

        pub fn is<T>(&self) -> bool
        where
            T: 'static,
        {
            match self.kind {
                OperationKind::Allocate => std::any::TypeId::of::<T>() == std::any::TypeId::of::<AllocateOp>(),
                OperationKind::Store => std::any::TypeId::of::<T>() == std::any::TypeId::of::<StoreOp>(),
                _ => false,
            }
        }

        pub fn try_cast<T>(&self) -> Option<&T>
        where
            T: 'static,
        {
            if self.is::<T>() {
                Some(unsafe { &*(self as *const Operation as *const T) })
            } else {
                None
            }
        }

        pub fn cast<T>(&self) -> &T
        where
            T: 'static,
        {
            self.try_cast::<T>().unwrap()
        }

        pub fn inputs(&self) -> &[OpIndex] {
            &self.inputs
        }
    }

    pub struct AllocateOp {}
    impl AllocateOp {
        pub fn is<T>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<AllocateOp>()
        }
    }

    pub struct StoreOp {
        value: OpIndex,
    }

    impl StoreOp {
        pub fn is<T>(&self) -> bool {
            std::any::TypeId::of::<T>() == std::any::TypeId::of::<StoreOp>()
        }
        pub fn value(&self) -> OpIndex {
            self.value
        }
    }

    pub type ZoneVector<T> = Vec<T>;
    pub type ZoneAbslFlatHashMap<K, V> = HashMap<K, V>;

    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }

    thread_local! {
        static SHOULD_SKIP_OPTIMIZATION_STEP: RefCell<bool> = RefCell::new(false);
    }

    pub fn set_should_skip_optimization_step(value: bool) {
        SHOULD_SKIP_OPTIMIZATION_STEP.with(|flag| {
            *flag.borrow_mut() = value;
        });
    }

    pub fn should_skip_optimization_step() -> bool {
        SHOULD_SKIP_OPTIMIZATION_STEP.with(|flag| *flag.borrow())
    }

    pub struct LateEscapeAnalysisAnalyzer<'a> {
        graph_: &'a mut Graph,
        phase_zone_: &'a Zone,
        alloc_uses_: ZoneAbslFlatHashMap<OpIndex, ZoneVector<OpIndex>>,
        allocs_: ZoneVector<OpIndex>,
    }

    impl<'a> LateEscapeAnalysisAnalyzer<'a> {
        pub fn new(graph: &'a mut Graph, zone: &'a Zone) -> Self {
            LateEscapeAnalysisAnalyzer {
                graph_: graph,
                phase_zone_: zone,
                alloc_uses_: HashMap::new(),
                allocs_: Vec::new(),
            }
        }

        pub fn run(&mut self) {
            self.collect_uses_and_allocations();
            self.find_removable_allocations();
        }

        fn record_allocate_use(&mut self, alloc: OpIndex, use_: OpIndex) {
            let uses = self.alloc_uses_.entry(alloc).or_insert_with(|| Vec::new());
            uses.push(use_);
        }

        // Collects the Allocate Operations and their uses.
        fn collect_uses_and_allocations(&mut self) {
            for op in self.graph_.all_operations().iter() {
                if should_skip_operation(op) {
                    continue;
                }
                let op_index = self.graph_.index(op);
                for &input in op.inputs() {
                    if self.graph_.get(input).is::<AllocateOp>() {
                        self.record_allocate_use(input, op_index);
                    }
                }
                if op.is::<AllocateOp>() {
                    self.allocs_.push(op_index);
                }
            }
        }

        fn find_removable_allocations(&mut self) {
            while !self.allocs_.is_empty() {
                let current_alloc = self.allocs_.pop().unwrap();

                if should_skip_operation(self.graph_.get(current_alloc)) {
                    // We are re-visiting an allocation that we've actually already removed.
                    continue;
                }

                if !self.allocation_is_escaping(current_alloc) {
                    self.mark_to_remove(current_alloc);
                }
            }
        }

        fn allocation_is_escaping(&self, alloc: OpIndex) -> bool {
            if !self.alloc_uses_.contains_key(&alloc) {
                return false;
            }
            for &use_ in self.alloc_uses_.get(&alloc).unwrap() {
                if self.escapes_through_use(alloc, use_) {
                    return true;
                }
            }
            // We haven't found any non-store use
            return false;
        }

        // Returns true if {using_op_idx} is an operation that forces {alloc} to be
        // emitted.
        fn escapes_through_use(&self, alloc: OpIndex, using_op_idx: OpIndex) -> bool {
            if should_skip_operation(self.graph_.get(alloc)) {
                // {using_op_idx} is an Allocate itself, which has been removed.
                return false;
            }
            let op = self.graph_.get(using_op_idx);
            if let Some(store_op) = op.try_cast::<StoreOp>() {
                // A StoreOp only makes {alloc} escape if it uses {alloc} as the {value} or
                // the {index}. Put otherwise, StoreOp makes {alloc} escape if it writes
                // {alloc}, but not if it writes **to** {alloc}.
                return store_op.value() == alloc;
            }
            return true;
        }

        fn mark_to_remove(&mut self, alloc: OpIndex) {
            if should_skip_optimization_step() {
                return;
            }
            self.graph_.kill_operation(alloc);
            if !self.alloc_uses_.contains_key(&alloc) {
                return;
            }

            // The uses of {alloc} should also be skipped.
            for &use_ in self.alloc_uses_.get(&alloc).unwrap() {
                let store = self.graph_.get(use_).cast::<StoreOp>();
                if self.graph_.get(store.value()).is::<AllocateOp>() {
                    // This store was storing the result of an allocation. Because we now
                    // removed this store, we might be able to remove the other allocation
                    // as well.
                    self.allocs_.push(store.value());
                }
                self.graph_.kill_operation(use_);
            }
        }
    }

    fn should_skip_operation(_op: &Operation) -> bool {
        false
    }

    pub struct LateEscapeAnalysisReducer<Next> {
        analyzer_: LateEscapeAnalysisAnalyzer<'static>,
        next: Next,
    }

    impl<Next> LateEscapeAnalysisReducer<Next> {
        pub fn new(graph: &'static mut Graph, zone: &'static Zone, next: Next) -> Self {
            LateEscapeAnalysisReducer {
                analyzer_: LateEscapeAnalysisAnalyzer::new(graph, zone),
                next,
            }
        }

        pub fn analyze(&mut self) {
            self.analyzer_.run();
        }
    }
}
