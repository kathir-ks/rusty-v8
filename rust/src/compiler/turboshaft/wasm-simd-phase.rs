// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod wasm_shuffle_reducer;
mod copying_phase;

use crate::compiler::turboshaft::copying_phase::CopyingPhase;
use crate::compiler::turboshaft::wasm_shuffle_reducer::WasmShuffleAnalyzer;

pub struct PipelineData<'a> {
    graph: &'a Graph,
    wasm_shuffle_analyzer: Option<&'a WasmShuffleAnalyzer<'a>>,
}

impl<'a> PipelineData<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        PipelineData {
            graph,
            wasm_shuffle_analyzer: None,
        }
    }

    pub fn graph(&self) -> &Graph {
        self.graph
    }

    pub fn set_wasm_shuffle_analyzer(&mut self, analyzer: &'a WasmShuffleAnalyzer<'a>) {
        self.wasm_shuffle_analyzer = Some(analyzer);
    }

    pub fn clear_wasm_shuffle_analyzer(&mut self) {
        self.wasm_shuffle_analyzer = None;
    }

    pub fn wasm_shuffle_analyzer(&self) -> Option<&'a WasmShuffleAnalyzer<'a>> {
        self.wasm_shuffle_analyzer
    }
}

pub struct Graph {}

impl Graph {
    pub fn new() -> Self {
        Graph {}
    }
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

pub struct WasmSimdPhase {}

impl WasmSimdPhase {
    pub fn run(data: &mut PipelineData, temp_zone: &Zone) {
        let analyzer = WasmShuffleAnalyzer::new(temp_zone, data.graph());

        if analyzer.should_reduce() {
            data.set_wasm_shuffle_analyzer(&analyzer);
            CopyingPhase::<wasm_shuffle_reducer::WasmShuffleReducer>::run(data, temp_zone);
            data.clear_wasm_shuffle_analyzer();
        }
    }
}

mod compiler {
    pub mod turboshaft {
        // Modules are defined in other files.
    }
}