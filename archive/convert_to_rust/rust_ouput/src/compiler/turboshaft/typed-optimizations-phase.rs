// Converted from V8 C++ source files:
// Header: typed-optimizations-phase.h
// Implementation: typed-optimizations-phase.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod typed_optimizations_phase {
    use crate::compiler::turboshaft::phase::PipelineData;
    use crate::compiler::turboshaft::copying_phase::copying_phase::CopyingPhase;
    use crate::compiler::turboshaft::type_inference_reducer::type_inference_reducer::{TypeInferenceReducer, TypeInferenceReducerArgs};
    use crate::compiler::turboshaft::typed_optimizations_reducer::typed_optimizations_reducer::TypedOptimizationsReducer;
    //use crate::compiler::turboshaft::utils::UnparkedScopeIfNeeded;
    //use crate::v8::internal::flags::v8_flags;

    pub struct TypedOptimizationsPhase {}

    impl TypedOptimizationsPhase {
        pub const NAME: &'static str = "TypedOptimizations";

        pub fn run(data: &mut PipelineData, temp_zone: &mut Zone) {
            //let scope = UnparkedScopeIfNeeded::new(data.broker(), v8_flags::turboshaft_trace_typing);

            let typing_args = TypeInferenceReducerArgs::Scope {
                input_graph_typing: TypeInferenceReducerArgs::InputGraphTyping::Precise,
                output_graph_typing: TypeInferenceReducerArgs::OutputGraphTyping::None,
            };

            CopyingPhase::<TypedOptimizationsReducer, TypeInferenceReducer>::run(data, temp_zone);
        }
    }
    pub struct Zone{}
}
