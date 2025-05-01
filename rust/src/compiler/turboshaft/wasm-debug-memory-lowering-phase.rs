#![cfg(debug_assertions)]

// src/compiler/turboshaft/wasm-debug-memory-lowering-phase.rs

mod memory_optimization_reducer;
mod phase;
mod unparked_scope;

use memory_optimization_reducer::MemoryOptimizationReducer;
use phase::CopyingPhase;
use unparked_scope::UnparkedScopeIfNeeded;

// Assuming PipelineData and Zone are defined elsewhere and accessible.
// Replace with actual definitions or placeholders.
pub struct PipelineData {
    //pub broker: Broker,  // Placeholder, define appropriately
}
pub struct Zone {}

// Assuming v8_flags are defined elsewhere and accessible.
// Replace with actual definitions or placeholders.
mod v8_flags {
    pub const turboshaft_trace_reduction: bool = true;
}

// Assuming Broker is defined elsewhere and accessible
// Replace with actual definition or placeholder
pub struct Broker {}

pub struct WasmDebugMemoryLoweringPhase {}

impl WasmDebugMemoryLoweringPhase {
    pub fn run(data: &mut PipelineData, temp_zone: &mut Zone) {
        let broker = Broker{}; //Placeholder
        let scope = UnparkedScopeIfNeeded::new(&broker, v8_flags::turboshaft_trace_reduction);
        CopyingPhase::<MemoryOptimizationReducer>::run(data, temp_zone);
        drop(scope); // Ensure scope is dropped at end of function.
    }
}