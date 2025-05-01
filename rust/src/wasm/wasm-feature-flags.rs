// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module defines WebAssembly feature flags.
//
// Each feature flag corresponds to a V8 command-line flag with the prefix
// "--experimental-wasm-".
//
// For example, to enable "my_feature", pass
// --experimental-wasm-my-feature to d8, or
// --js-flags=--experimental-wasm-my-feature to Chrome.
//
// To disable "my_feature", add the "--no-" prefix:
// --no-experimental-wasm-my-feature.
//
// See https://github.com/WebAssembly/proposals for an overview of current
// WebAssembly proposals.

macro_rules! define_wasm_feature_flags {
    ($macro:ident) => {
        $macro!(type_reflection, "wasm type reflection in JS", false);
        $macro!(compilation_hints, "compilation hints section", false);
        $macro!(instruction_tracing, "instruction tracing section", false);
        $macro!(stack_switching, "stack switching", false);
        $macro!(shared, "shared-everything threads", false);
        $macro!(fp16, "fp16", false);
        $macro!(growable_stacks, "growable stacks for jspi", false);
        $macro!(memory_control, "memory control", false);
        $macro!(wasmfx, "core stack switching", false);
        $macro!(branch_hinting, "branch hinting", false);
        $macro!(stringref, "reference-typed strings", false);
        $macro!(imported_strings_utf8, "imported strings (utf8 features)", false);
        $macro!(exnref, "exnref", false);
        $macro!(jspi, "javascript promise integration", false);
        $macro!(legacy_eh, "legacy exception handling opcodes", true);
        $macro!(imported_strings, "imported strings", true);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature_flags_defined() {
        macro_rules! check_feature {
            ($name:ident, $desc:expr, $enabled:expr) => {
                // This test asserts that the feature flag $name is defined in the
                // define_wasm_feature_flags macro. If this test fails, it means
                // that the feature flag is missing from the macro definition.
                let _ = stringify!($name);
                let _ = $desc;
                let _ = $enabled;
            };
        }

        define_wasm_feature_flags!(check_feature);
    }
}

// Example of how the macro could be used to create a struct.  This part is not part of
// the original C++ header file, but shows how to use the macro.

#[derive(Debug, Default, Clone, Copy)]
pub struct WasmFeatureFlags {
    pub type_reflection: bool,
    pub compilation_hints: bool,
    pub instruction_tracing: bool,
    pub stack_switching: bool,
    pub shared: bool,
    pub fp16: bool,
    pub growable_stacks: bool,
    pub memory_control: bool,
    pub wasmfx: bool,
    pub branch_hinting: bool,
    pub stringref: bool,
    pub imported_strings_utf8: bool,
    pub exnref: bool,
    pub jspi: bool,
    pub legacy_eh: bool,
    pub imported_strings: bool,
}

impl WasmFeatureFlags {
    pub fn new() -> Self {
        let mut flags = Self::default();
        macro_rules! set_feature {
            ($name:ident, $desc:expr, $enabled:expr) => {
                flags.$name = $enabled;
            };
        }
        define_wasm_feature_flags!(set_feature);
        flags
    }
}

#[cfg(test)]
mod feature_flag_consistency_checks {
    use super::*;

    #[test]
    fn experimental_staging_off_by_default() {
        let flags = WasmFeatureFlags::new();

        assert_eq!(flags.type_reflection, false);
        assert_eq!(flags.compilation_hints, false);
        assert_eq!(flags.instruction_tracing, false);
        assert_eq!(flags.stack_switching, false);
        assert_eq!(flags.shared, false);
        assert_eq!(flags.fp16, false);
        assert_eq!(flags.growable_stacks, false);
        assert_eq!(flags.memory_control, false);
        assert_eq!(flags.wasmfx, false);
        assert_eq!(flags.branch_hinting, false);
        assert_eq!(flags.stringref, false);
        assert_eq!(flags.imported_strings_utf8, false);
        assert_eq!(flags.exnref, false);
        assert_eq!(flags.jspi, false);
    }

    #[test]
    fn shipped_on_by_default() {
        let flags = WasmFeatureFlags::new();

        assert_eq!(flags.legacy_eh, true);
        assert_eq!(flags.imported_strings, true);
    }
}