// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod copying_phase;
mod dataview_lowering_reducer;
mod fast_api_call_lowering_reducer;
mod js_generic_lowering_reducer;
mod machine_lowering_reducer;
mod machine_optimization_reducer;
mod required_optimization_reducer;
mod select_lowering_reducer;
mod string_escape_analysis_reducer;
mod variable_reducer;

use copying_phase::CopyingPhase;
use dataview_lowering_reducer::DataViewLoweringReducer;
use fast_api_call_lowering_reducer::FastApiCallLoweringReducer;
use js_generic_lowering_reducer::JSGenericLoweringReducer;
use machine_lowering_reducer::MachineLoweringReducer;
use machine_optimization_reducer::MachineOptimizationReducer;
use select_lowering_reducer::SelectLoweringReducer;
use string_escape_analysis_reducer::StringEscapeAnalysisReducer;
use variable_reducer::VariableReducer;

pub struct PipelineData {} // Placeholder, replace with actual data structure
pub struct Zone {} // Placeholder, replace with actual data structure

pub struct MachineLoweringPhase {}

impl MachineLoweringPhase {
    /// Runs the machine lowering phase.
    pub fn run(data: &mut PipelineData, temp_zone: &mut Zone) {
        CopyingPhase::<
            StringEscapeAnalysisReducer,
            JSGenericLoweringReducer,
            DataViewLoweringReducer,
            MachineLoweringReducer,
            FastApiCallLoweringReducer,
            VariableReducer,
            SelectLoweringReducer,
            MachineOptimizationReducer,
        >::run(data, temp_zone);
    }
}