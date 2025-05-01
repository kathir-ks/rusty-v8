// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/node-marker.h equivalent (module declaration)
mod node_marker;
pub use node_marker::NodeMarkerBase;

mod turbofan_graph;
use turbofan_graph::TFGraph;

mod compiler {
    use super::*;

    /// Base class for node markers.
    pub struct NodeMarkerBase<'a> {
        mark_min: u32,
        mark_max: u32,
        graph: &'a mut TFGraph,
    }

    impl<'a> NodeMarkerBase<'a> {
        /// Constructs a new `NodeMarkerBase` instance.
        ///
        /// # Arguments
        ///
        /// * `graph`: A mutable reference to the `TFGraph`.
        /// * `num_states`: The number of states.
        pub fn new(graph: &'a mut TFGraph, num_states: u32) -> Self {
            debug_assert_ne!(0, num_states, "user error!");

            let mark_min = graph.mark_max;
            graph.mark_max += num_states;
            let mark_max = graph.mark_max;

            debug_assert!(mark_min < mark_max, "check for wraparound.");

            NodeMarkerBase {
                mark_min,
                mark_max,
                graph,
            }
        }
    }
}