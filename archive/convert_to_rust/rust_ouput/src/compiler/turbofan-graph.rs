// Converted from V8 C++ source files:
// Header: turbofan-graph.h
// Implementation: turbofan-graph.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turbofan_graph {
    use std::array;
    use std::convert::From;
    use std::marker::PhantomData;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    use crate::compiler::graph_trimmer::StdoutStream;
    use crate::compiler::scheduler::ZoneObject;
    use crate::compiler::string_builder_optimizer::Node;
    use crate::compiler::string_builder_optimizer::Operator;
    use crate::compiler::machine_graph::NodeId;
    use crate::compiler::zone::Zone;
    use crate::compiler::zone_stats::V8_EXPORT_PRIVATE;
    use crate::compiler::zone_stats::V8_NODISCARD;
    use crate::compiler::verifier::VerifyNode;

    pub type Mark = u32;

    pub struct TFGraph {
        zone_: *mut Zone,
        start_: *mut Node,
        end_: *mut Node,
        mark_max_: Mark,
        next_node_id_: AtomicU32,
        decorators_: Mutex<Vec<*mut GraphDecorator>>,
        has_simd_: bool,
        simd_stores_: Mutex<Vec<*mut Node>>,
    }

    impl TFGraph {
        pub fn new(zone: *mut Zone) -> Self {
            // Nodes use compressed pointers, so zone must support pointer compression.
            // If the check fails, ensure the zone is created with kCompressGraphZone
            // flag.
            TFGraph {
                zone_: zone,
                start_: null_mut(),
                end_: null_mut(),
                mark_max_: 0,
                next_node_id_: AtomicU32::new(0),
                decorators_: Mutex::new(Vec::new()),
                has_simd_: false,
                simd_stores_: Mutex::new(Vec::new()),
            }
        }

        // Scope used when creating a subgraph for inlining. Automatically preserves
        // the original start and end nodes of the graph, and resets them when you
        // leave the scope.
        pub struct SubgraphScope<'a> {
            graph_: &'a mut TFGraph,
            start_: *mut Node,
            end_: *mut Node,
        }

        impl<'a> SubgraphScope<'a> {
            pub fn new(graph: &'a mut TFGraph) -> Self {
                let start_ = graph.start();
                let end_ = graph.end();
                SubgraphScope {
                    graph_: graph,
                    start_: start_,
                    end_: end_,
                }
            }
        }

        impl<'a> Drop for SubgraphScope<'a> {
            fn drop(&mut self) {
                self.graph_.SetStart(self.start_);
                self.graph_.SetEnd(self.end_);
            }
        }

        // Base implementation used by all factory methods.
        pub fn NewNodeUnchecked(
            &mut self,
            op: *const Operator,
            input_count: i32,
            inputs: *const *mut Node,
            incomplete: bool,
        ) -> *mut Node {
            let node_id = self.NextNodeId();
            let node = unsafe { Node::New(self.zone(), node_id, op, input_count, inputs, incomplete) };

            self.Decorate(node);
            node
        }

        // Factory that checks the input count.
        pub fn NewNode(
            &mut self,
            op: *const Operator,
            input_count: i32,
            inputs: *const *mut Node,
            incomplete: bool,
        ) -> *mut Node {
            let node = self.NewNodeUnchecked(op, input_count, inputs, incomplete);
            unsafe { VerifyNode(node) };
            node
        }

        // Factory template for nodes with static input counts.
        // Note: Template magic below is used to ensure this method is only considered
        // for argument types convertible to Node* during overload resolution.
        pub fn NewNode_arr<const N: usize>(&mut self, op: *const Operator, nodes: [*mut Node; N]) -> *mut Node {
            self.NewNode(op, N as i32, nodes.as_ptr() as *const *mut Node, false)
        }

        // Clone the {node}, and assign a new node id to the copy.
        pub fn CloneNode(&mut self, node: *const Node) -> *mut Node {
            assert!(!node.is_null());
            let clone = unsafe { Node::Clone(self.zone(), self.NextNodeId(), node) };
            self.Decorate(clone);
            clone
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }
        pub fn start(&self) -> *mut Node {
            self.start_
        }
        pub fn end(&self) -> *mut Node {
            self.end_
        }

        pub fn SetStart(&mut self, start: *mut Node) {
            self.start_ = start;
        }
        pub fn SetEnd(&mut self, end: *mut Node) {
            self.end_ = end;
        }

        pub fn NodeCount(&self) -> usize {
            self.next_node_id_.load(Ordering::Relaxed) as usize
        }

        pub fn Decorate(&mut self, node: *mut Node) {
            let decorators = self.decorators_.lock().unwrap();
            for decorator in decorators.iter() {
                unsafe {
                    (*decorator).Decorate(node);
                }
            }
        }

        pub fn AddDecorator(&mut self, decorator: *mut GraphDecorator) {
            let mut decorators = self.decorators_.lock().unwrap();
            decorators.push(decorator);
        }

        pub fn RemoveDecorator(&mut self, decorator: *mut GraphDecorator) {
            let mut decorators = self.decorators_.lock().unwrap();
            if let Some(index) = decorators.iter().position(|&x| x == decorator) {
                decorators.remove(index);
            }
        }

        // Very simple print API usable in a debugger.
        pub fn Print(&self) const {
            unsafe {
              StdoutStream{}.write_str(&AsRPO(*self)).unwrap();
              StdoutStream{}.flush().unwrap();
            }
        }

        pub fn HasSimd(&self) -> bool {
            self.has_simd_
        }
        pub fn SetSimd(&mut self, has_simd: bool) {
            self.has_simd_ = has_simd;
        }

        pub fn RecordSimdStore(&mut self, store: *mut Node) {
            let mut simd_stores = self.simd_stores_.lock().unwrap();
            simd_stores.push(store);
        }

        pub fn GetSimdStoreNodes(&self) -> Vec<*mut Node> {
            let simd_stores = self.simd_stores_.lock().unwrap();
            simd_stores.clone()
        }

        fn NextNodeId(&self) -> NodeId {
            // A node's id is internally stored in a bit field using fewer bits than
            // NodeId (see Node::IdField). Hence the addition below won't ever overflow.
            let next_id = self.next_node_id_.fetch_add(1, Ordering::Relaxed);
            next_id
        }
    }

    // A graph decorator can be used to add behavior to the creation of nodes
    // in a graph.
    pub trait GraphDecoratorTrait {
        fn Decorate(&mut self, node: *mut Node);
    }

    pub struct GraphDecorator {
        vtable: *const GraphDecoratorVTable,
        data: *mut std::ffi::c_void,
    }

    impl GraphDecorator {
        pub fn new<T: GraphDecoratorTrait + 'static>(decorator: T) -> Self {
            let data = Box::into_raw(Box::new(decorator)) as *mut std::ffi::c_void;
            let vtable = Box::into_raw(Box::new(GraphDecoratorVTable {
                decorate: Self::decorate_helper::<T>,
                drop: Self::drop_helper::<T>,
            }));
            Self { vtable, data }
        }

        unsafe extern "C" fn decorate_helper<T: GraphDecoratorTrait>(
            data: *mut std::ffi::c_void,
            node: *mut Node,
        ) {
            let decorator = (data as *mut T).as_mut().unwrap();
            decorator.Decorate(node);
        }

        unsafe extern "C" fn drop_helper<T: GraphDecoratorTrait>(data: *mut std::ffi::c_void) {
            drop(Box::from_raw(data as *mut T));
        }

        pub unsafe fn Decorate(&self, node: *mut Node) {
            let vtable = &*self.vtable;
            (vtable.decorate)(self.data, node);
        }
    }

    impl Drop for GraphDecorator {
        fn drop(&mut self) {
            unsafe {
                let vtable = Box::from_raw(self.vtable as *mut GraphDecoratorVTable);
                (vtable.drop)(self.data);
            }
        }
    }

    #[repr(C)]
    struct GraphDecoratorVTable {
        decorate: unsafe extern "C" fn(data: *mut std::ffi::c_void, node: *mut Node),
        drop: unsafe extern "C" fn(data: *mut std::ffi::c_void),
    }

    pub struct AsRPO<'a> {
      graph: &'a TFGraph,
    }

    impl<'a> AsRPO<'a> {
      pub fn new(graph: &'a TFGraph) -> Self {
        AsRPO { graph }
      }
    }

    use std::fmt;
    impl fmt::Display for AsRPO<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Graph as RPO")
        }
    }

    impl<'a> From<&'a TFGraph> for AsRPO<'a> {
        fn from(graph: &'a TFGraph) -> Self {
            AsRPO::new(graph)
        }
    }
}
