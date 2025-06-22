// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This header should only be included if WebAssembly is enabled.");

pub mod compilation_environment {
    use crate::wasm::wasm_code_manager::DynamicTiering;
    use crate::wasm::compilation_environment::WasmEnabledFeatures;
    use std::ptr::NonNull;

    // Assuming NativeModule, Module, WasmEnabledFeatures, DynamicTiering, FastApiTargets, and FastApiSignatures are defined elsewhere.
    // Replace with actual definitions.
    pub struct NativeModule {
        module: *const Module,
        enabled_features: WasmEnabledFeatures,
        compilation_state: Box<CompilationState>, // Assuming CompilationState requires heap allocation.
        fast_api_targets: *const FastApiTargets,
        fast_api_signatures: *const FastApiSignatures,
    }

    impl NativeModule {
        pub fn module(&self) -> *const Module {
            self.module
        }

        pub fn enabled_features(&self) -> WasmEnabledFeatures {
            self.enabled_features
        }

        pub fn compilation_state(&self) -> &CompilationState {
            &self.compilation_state
        }

        pub fn fast_api_targets(&self) -> *const FastApiTargets {
            self.fast_api_targets
        }

        pub fn fast_api_signatures(&self) -> *const FastApiSignatures {
            self.fast_api_signatures
        }
    }

    pub struct Module {}
    pub struct FastApiTargets {}
    pub struct FastApiSignatures {}

    pub struct CompilationEnv {
        module: *const Module,
        enabled_features: WasmEnabledFeatures,
        dynamic_tiering: DynamicTiering,
        fast_api_targets: *const FastApiTargets,
        fast_api_signatures: *const FastApiSignatures,
    }

    pub struct CompilationState {}

    impl CompilationEnv {
        pub fn for_module(native_module: &NativeModule) -> Self {
            CompilationEnv {
                module: native_module.module(),
                enabled_features: native_module.enabled_features(),
                dynamic_tiering: native_module.compilation_state().dynamic_tiering(),
                fast_api_targets: native_module.fast_api_targets(),
                fast_api_signatures: native_module.fast_api_signatures(),
            }
        }

        pub const fn no_module_all_features_for_testing() -> Self {
            CompilationEnv {
                module: std::ptr::null(),
                enabled_features: WasmEnabledFeatures::All,
                dynamic_tiering: DynamicTiering::kNoDynamicTiering,
                fast_api_targets: std::ptr::null(),
                fast_api_signatures: std::ptr::null(),
            }
        }
    }
}