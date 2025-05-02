// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod liftoff_compiler {
    use std::ptr::NonNull;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::marker::PhantomData;

    pub use crate::wasm::function_compiler::*; // Assuming function_compiler.h is converted and available here.

    pub mod wasm {
        use std::ptr::NonNull;
        use std::rc::Rc;
        use std::cell::RefCell;
        use std::marker::PhantomData;
        use crate::wasm::function_compiler::*;

        #[repr(i8)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LiftoffBailoutReason {
            // Nothing actually failed.
            Success = 0,
            // Compilation failed, but not because of Liftoff.
            DecodeError = 1,
            // Liftoff is not implemented on that architecture.
            UnsupportedArchitecture = 2,
            // More complex code would be needed because a CPU feature is not present.
            MissingCPUFeature = 3,
            // Liftoff does not implement a complex (and rare) instruction.
            ComplexOperation = 4,
            // Unimplemented proposals:
            Simd = 5,
            RefTypes = 6,
            ExceptionHandling = 7,
            MultiValue = 8,
            TailCall = 9,
            Atomics = 10,
            BulkMemory = 11,
            NonTrappingFloatToInt = 12,
            GC = 13,
            RelaxedSimd = 14,
            // A little gap, for forward compatibility.
            // Any other reason (use rarely; introduce new reasons if this spikes).
            OtherReason = 20,
            // Marker:
            NumBailoutReasons
        }

        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LocationKindForDeopt {
            None,
            EagerDeopt,   // The location is the point of an eager deopt.
            InlinedCall,  // The location is an inlined call, not a deopt.
        }

        pub struct LiftoffOptions<'a> {
            pub func_index: i32,
            pub for_debugging: ForDebugging,
            pub counters: Option<NonNull<Counters>>,
            pub detected_features: Option<NonNull<WasmDetectedFeatures>>,
            pub breakpoints: Vec<i32>,
            pub debug_sidetable: Option<Box<DebugSideTable>>,
            pub dead_breakpoint: i32,
            pub max_steps: Option<NonNull<i32>>,
            pub detect_nondeterminism: bool,
            pub deopt_info_bytecode_offset: u32,
            pub deopt_location_kind: LocationKindForDeopt,
            _phantom: PhantomData<&'a ()>,
        }

        impl<'a> Default for LiftoffOptions<'a> {
            fn default() -> Self {
                LiftoffOptions {
                    func_index: -1,
                    for_debugging: ForDebugging::NotForDebugging,
                    counters: None,
                    detected_features: None,
                    breakpoints: Vec::new(),
                    debug_sidetable: None,
                    dead_breakpoint: 0,
                    max_steps: None,
                    detect_nondeterminism: false,
                    deopt_info_bytecode_offset: std::u32::MAX,
                    deopt_location_kind: LocationKindForDeopt::None,
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a> LiftoffOptions<'a> {
            pub fn new() -> Self {
                Default::default()
            }

            pub fn set_func_index(mut self, new_value: i32) -> Self {
                assert_eq!(self.func_index, LiftoffOptions::default().func_index);
                self.func_index = new_value;
                self
            }

            pub fn set_for_debugging(mut self, new_value: ForDebugging) -> Self {
                assert_eq!(self.for_debugging, LiftoffOptions::default().for_debugging);
                self.for_debugging = new_value;
                self
            }

            pub fn set_counters(mut self, new_value: Option<NonNull<Counters>>) -> Self {
                 self.counters = new_value;
                self
            }

            pub fn set_detected_features(mut self, new_value: Option<NonNull<WasmDetectedFeatures>>) -> Self {
                 self.detected_features = new_value;
                self
            }

            pub fn set_breakpoints(mut self, new_value: Vec<i32>) -> Self {
                self.breakpoints = new_value;
                self
            }

            pub fn set_debug_sidetable(mut self, new_value: Option<Box<DebugSideTable>>) -> Self {
                self.debug_sidetable = new_value;
                self
            }

            pub fn set_dead_breakpoint(mut self, new_value: i32) -> Self {
                 self.dead_breakpoint = new_value;
                self
            }

             pub fn set_max_steps(mut self, new_value: Option<NonNull<i32>>) -> Self {
                self.max_steps = new_value;
                self
            }

            pub fn set_detect_nondeterminism(mut self, new_value: bool) -> Self {
                self.detect_nondeterminism = new_value;
                self
            }

            pub fn set_deopt_info_bytecode_offset(mut self, new_value: u32) -> Self {
                self.deopt_info_bytecode_offset = new_value;
                self
            }

            pub fn set_deopt_location_kind(mut self, new_value: LocationKindForDeopt) -> Self {
                self.deopt_location_kind = new_value;
                self
            }

            pub fn is_initialized(&self) -> bool {
                self.func_index >= 0
            }
        }

        // Opaque types that need to be defined elsewhere.
        pub struct CompilationEnv {}
        pub struct FunctionBody {}
        pub struct WasmDetectedFeatures {}
        pub struct Counters {}
        pub struct DebugSideTable {}
        pub struct WasmCode {}

        // Assume WasmCompilationResult is defined in function_compiler.rs
        // pub struct WasmCompilationResult {}

        extern "C" {
            // Replace with actual FFI if needed. This is a placeholder.
            pub fn ExecuteLiftoffCompilation(
                env: *mut CompilationEnv,
                body: *const FunctionBody,
                options: *const LiftoffOptions,
            ) -> WasmCompilationResult;

            pub fn GenerateLiftoffDebugSideTable(code: *const WasmCode) -> *mut DebugSideTable;
        }
    }
}