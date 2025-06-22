// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_graph_verifier {
    use crate::maglev::maglev_compilation_info::MaglevCompilationInfo;
    use crate::maglev::maglev_graph_labeller::MaglevGraphLabeller;
    use crate::maglev::maglev_graph_processor::{
        BlockProcessResult, ProcessResult, ProcessingState,
    };
    use crate::maglev::maglev_ir::{BasicBlock, Graph, Input, Node, Opcode};
    use crate::maglev::maglev_ir::{kFirstOpcode, kLastOpcode};
    use crate::maglev::maglev_ir::Dead;

    /// TODO(victorgomes): Add more verification.
    pub struct MaglevGraphVerifier<'a> {
        graph_labeller: Option<&'a MaglevGraphLabeller>,
    }

    impl<'a> MaglevGraphVerifier<'a> {
        pub fn new(compilation_info: &'a MaglevCompilationInfo) -> Self {
            let graph_labeller = if compilation_info.has_graph_labeller() {
                Some(compilation_info.graph_labeller())
            } else {
                None
            };
            MaglevGraphVerifier { graph_labeller }
        }

        pub fn pre_process_graph(&self, _graph: &mut Graph) {}
        pub fn post_process_graph(&self, _graph: &mut Graph) {}
        pub fn post_process_basic_block(&self, _block: &mut BasicBlock) {}

        pub fn pre_process_basic_block(&self, _block: &mut BasicBlock) -> BlockProcessResult {
            BlockProcessResult::Continue
        }

        pub fn post_phi_processing(&self) {}

        pub fn process(&self, node: &Dead, _state: &ProcessingState) -> ProcessResult {
            node.verify_inputs(self.graph_labeller);
            ProcessResult::Continue
        }

        pub fn process_node<NodeT: Node>(&self, node: &NodeT, _state: &ProcessingState) -> ProcessResult {
            for input in node.inputs() {
                let op = input.node().opcode();
                assert!(op >= kFirstOpcode);
                assert!(op <= kLastOpcode);
            }
            node.verify_inputs(self.graph_labeller);
            ProcessResult::Continue
        }
    }
}