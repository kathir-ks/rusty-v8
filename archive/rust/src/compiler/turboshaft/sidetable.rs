// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod sidetable {
    use crate::compiler::turboshaft::graph::Graph;
    use crate::compiler::turboshaft::index::OpIndex;

    /// Checks if an OpIndex belongs to a specific Graph.
    #[cfg(debug_assertions)]
    pub fn op_index_belongs_to_table_graph(graph: &Graph, index: OpIndex) -> bool {
        graph.belongs_to_this_graph(index)
    }

    #[cfg(not(debug_assertions))]
    pub fn op_index_belongs_to_table_graph(_graph: &Graph, _index: OpIndex) -> bool {
        //In release builds, we don't need to check if the index belongs to the graph.
        true
    }
}

pub mod graph {
    use crate::compiler::turboshaft::index::OpIndex;

    pub struct Graph {}

    impl Graph {
        pub fn belongs_to_this_graph(&self, _index: OpIndex) -> bool {
            // Dummy implementation, replace with actual logic
            true
        }
    }
}

pub mod index {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct OpIndex(pub usize); // Example implementation for OpIndex
}