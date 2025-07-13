// Converted from V8 C++ source files:
// Header: analyzer-iterator.h
// Implementation: analyzer-iterator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    use crate::base::logging::DCHECK_IMPLIES;
    use crate::compiler::turboshaft::index::BlockIndex;
    use crate::compiler::turboshaft::loop_finder::LoopFinder;
    use crate::compiler::turboshaft::operations::GotoOp;

    pub struct AnalyzerIterator<'a> {
        graph_: &'a Graph,
        loop_finder_: &'a LoopFinder,
        visited_: FixedBlockSidetable<u64>,
        stack_: Vec<StackNode<'a>>,
        current_generation_: u64,
        curr_: StackNode<'a>,
    }

    impl<'a> AnalyzerIterator<'a> {
        pub fn new(
            phase_zone: &mut Zone,
            graph: &'a Graph,
            loop_finder: &'a LoopFinder,
        ) -> AnalyzerIterator<'a> {
            let mut iterator = AnalyzerIterator {
                graph_: graph,
                loop_finder_: loop_finder,
                visited_: FixedBlockSidetable::new(graph.block_count(), 0, phase_zone),
                stack_: Vec::new(),
                current_generation_: 1,
                curr_: StackNode {
                    block: std::ptr::null(),
                    generation: 0,
                },
            };
            iterator.stack_.push(StackNode {
                block: &graph.StartBlock(),
                generation: 1,
            });
            iterator
        }

        pub fn has_next(&self) -> bool {
            DCHECK_IMPLIES(!self.stack_.is_empty(), !self.is_outdated(self.stack_.last().unwrap()));
            !self.stack_.is_empty()
        }

        pub fn next(&mut self) -> *const Block {
            assert!(self.has_next());
            assert!(!self.is_outdated(self.stack_.last().unwrap()));

            self.curr_ = self.stack_.pop().unwrap();

            let curr_header = if unsafe { (*self.curr_.block).IsLoop() } {
                self.curr_.block
            } else {
                self.loop_finder_.GetLoopHeader(self.curr_.block)
            };

            let curr_block = unsafe {&*self.curr_.block};

            // Pushing on the stack the children that are not in the same loop as Next
            // (remember that since we're doing a DFS with a Last-In-First-Out stack,
            // pushing them first on the stack means that they will be visited last).
            let mut child = unsafe { (*self.curr_.block).LastChild() };
            while child != std::ptr::null() {
                if self.loop_finder_.GetLoopHeader(unsafe {&*child}) != curr_header {
                    self.stack_.push(StackNode {
                        block: child,
                        generation: self.current_generation_,
                    });
                }
                child = unsafe { (*child).NeighboringChild() };
            }

            // Pushing on the stack the children that are in the same loop as Next (they
            // are pushed last, so that they will be visited first).
             child = unsafe { (*self.curr_.block).LastChild() };
            while child != std::ptr::null() {
                if self.loop_finder_.GetLoopHeader(unsafe {&*child}) == curr_header {
                    self.stack_.push(StackNode {
                        block: child,
                        generation: self.current_generation_,
                    });
                }
                child = unsafe { (*child).NeighboringChild() };
            }

            self.visited_.set(unsafe { (*self.curr_.block).index() }, self.current_generation_);

            // Note that PopOutdated must be called after updating {visited_}, because
            // this way, if the stack contained initially [{Bx, 1}, {Bx, 2}] (where `Bx`
            // is the same block both time and it hasn't been visited before), then we
            // popped the second entry at the begining of this function, but if we call
            // PopOutdate before updating {visited_}, then it won't pop the first entry.
            self.pop_outdated();

            self.curr_.block
        }

        pub fn mark_loop_for_revisit(&mut self) {
            assert!(!self.curr_.block.is_null());
            assert_ne!(self.curr_.generation, 0);
            assert!(unsafe { (*self.curr_.block).HasBackedge(self.graph_) });

            let goto_op = unsafe { (*self.curr_.block).LastOperation(self.graph_).Cast::<GotoOp>() };
            let header = unsafe { (*goto_op).destination };
            self.stack_.push(StackNode {
                block: header,
                generation: self.current_generation_ + 1,
            });
            self.current_generation_ += 1;
        }

        pub fn mark_loop_for_revisit_skip_header(&mut self) {
            assert!(!self.curr_.block.is_null());
            assert_ne!(self.curr_.generation, 0);
            assert!(unsafe { (*self.curr_.block).HasBackedge(self.graph_) });

            let goto_op = unsafe { (*self.curr_.block).LastOperation(self.graph_).Cast::<GotoOp>() };
            let header = unsafe { (*goto_op).destination };

            let mut child = unsafe { (*header).LastChild() };
            while child != std::ptr::null() {
                self.stack_.push(StackNode {
                    block: child,
                    generation: self.current_generation_ + 1,
                });
                child = unsafe { (*child).NeighboringChild() };
            }
            self.current_generation_ += 1;
        }

        fn pop_outdated(&mut self) {
            while !self.stack_.is_empty() {
                if self.is_outdated(self.stack_.last().unwrap()) {
                    self.stack_.pop();
                } else {
                    return;
                }
            }
        }

        fn is_outdated(&self, node: &StackNode) -> bool {
            self.visited_.get(unsafe { (*node.block).index() }) >= node.generation
        }
    }

    struct StackNode<'a> {
        block: *const Block,
        generation: u64,
    }

    pub struct Graph {
    }

    impl Graph {
        pub fn StartBlock(&self) -> Block {
            Block{}
        }
        pub fn block_count(&self) -> usize {
            10
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Block {
    }

    impl Block {
        pub fn IsLoop(&self) -> bool {
            false
        }
        pub fn index(&self) -> usize {
            0
        }
        pub fn LastChild(&self) -> *const Block {
            std::ptr::null()
        }
         pub fn NeighboringChild(&self) -> *const Block {
            std::ptr::null()
        }
        pub fn LastOperation(&self, graph: &Graph) ->  turboshaft::operations::Operation {
             turboshaft::operations::Operation {}
        }
        pub fn HasBackedge(&self, graph: &Graph) -> bool {
            false
        }

    }

    pub struct Zone {

    }
    impl Zone {
          pub fn new() -> Self {
            Zone {}
        }
    }
    pub struct FixedBlockSidetable<T> {
        data: Vec<T>,
    }

    impl<T: Copy + Default> FixedBlockSidetable<T> {
        pub fn new(size: usize, default_value: T, _zone: &mut Zone) -> Self {
            FixedBlockSidetable {
                data: vec![default_value; size],
            }
        }

        pub fn get(&self, index: usize) -> T {
            self.data[index]
        }

        pub fn set(&mut self, index: usize, value: T) {
            self.data[index] = value;
        }
    }
    pub mod operations {
        pub struct Operation {}
        impl Operation {
             pub fn Cast<T>(&self) -> &T {
                unsafe { std::mem::transmute(self) }
            }
        }
        pub struct GotoOp {
            pub destination: *const super::Block,
        }
    }
    mod index {
        pub type BlockIndex = usize;
    }
    mod loop_finder {
        use crate::compiler::turboshaft::Block;
        pub struct LoopFinder {}
        impl LoopFinder {
            pub fn GetLoopHeader(&self, block: *const Block) -> *const Block {
                block
            }
        }
    }
    mod base {
        pub mod logging {
            pub fn DCHECK_IMPLIES(a: bool, b: bool) {}
        }
    }
}
