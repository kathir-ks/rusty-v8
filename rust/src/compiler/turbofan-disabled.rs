// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This file stubs out the Turbofan API when TF is disabled.
// See also v8_enable_turbofan in BUILD.gn.

// TODO: Define Rust equivalents for C++ headers
// For example:
// mod codegen;
// mod compiler;

// use codegen::Compiler;
// use compiler::turbofan::TurbofanCompilationJob;
// use v8::base::types::*; // Assuming v8::internal::Isolate, JSFunction, etc. are defined similarly

// Assuming these are defined elsewhere in the v8 crate or similar
// pub type Isolate = u32; // Placeholder
// pub type Handle<T> = u32; // Placeholder
// pub type JSFunction = u32; // Placeholder
// pub type BytecodeOffset = u32; // Placeholder

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum IsScriptAvailable {
    Yes,
    No,
}

pub mod compiler {
    // use super::*;
    // use std::result::Result;

    pub struct TurbofanCompilationJob {}

    pub fn new_compilation_job(
        _isolate: u32,
        _function: u32,
        _has_script: super::IsScriptAvailable,
        _osr_offset: u32,
    ) -> Result<std::unique_ptr<TurbofanCompilationJob>, String> {
        // Since turbofan is disabled, we should panic or return an error if this function is called.
        // In C++, this is done with FATAL which usually aborts the program.
        Err("compiler::NewCompilationJob must not be called when Turbofan is disabled (`v8_enable_turbofan = false`)".to_string())
    }

}