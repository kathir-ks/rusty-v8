// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a placeholder.  The actual implementations are architecture-
// specific and would reside in separate modules.  This code only
// defines the structure.
// Also, `MacroAssembler` is not fully translated, so calls to it are commented
// out.

// use std::result;
// use std::string::String;

// mod codegen;
// use codegen::macro_assembler::MacroAssembler;
// mod common;
// use common::globals;

#[derive(Debug, Clone, Copy)]
enum InterpreterEntryTrampolineMode {
    kDefault,
    kForProfiling,
}

#[derive(Debug, Clone, Copy)]
enum ConvertReceiverMode {
    kAny,
    kNullOrUndefined,
}

#[derive(Debug, Clone, Copy)]
enum InterpreterPushArgsMode {
    kOther,
    kWithFinalSpread,
    kArrayFunction,
}

struct Builtins {}

impl Builtins {
    /// Generates the interpreter entry trampoline.
    fn generate_interpreter_entry_trampoline(
        masm: &mut /*MacroAssembler*/ i32, // Placeholder type
        mode: InterpreterEntryTrampolineMode,
    ) {
        // TODO(someone): Implement the architecture specific logic here.
        // masm.Placeholder(mode);
    }

    /// Generates the interpreter entry trampoline for profiling.
    fn generate_interpreter_entry_trampoline_for_profiling(masm: &mut /*MacroAssembler*/ i32) {
        Builtins::generate_interpreter_entry_trampoline(
            masm,
            InterpreterEntryTrampolineMode::kForProfiling,
        );
    }

    fn generate_interpreter_push_args_then_call(masm: &mut /*MacroAssembler*/ i32) {
        Builtins::generate_interpreter_push_args_then_call_impl(
            masm,
            ConvertReceiverMode::kAny,
            InterpreterPushArgsMode::kOther,
        );
    }

    fn generate_interpreter_push_undefined_and_args_then_call(masm: &mut /*MacroAssembler*/ i32) {
        Builtins::generate_interpreter_push_args_then_call_impl(
            masm,
            ConvertReceiverMode::kNullOrUndefined,
            InterpreterPushArgsMode::kOther,
        );
    }

    fn generate_interpreter_push_args_then_call_with_final_spread(
        masm: &mut /*MacroAssembler*/ i32,
    ) {
        Builtins::generate_interpreter_push_args_then_call_impl(
            masm,
            ConvertReceiverMode::kAny,
            InterpreterPushArgsMode::kWithFinalSpread,
        );
    }

    fn generate_interpreter_push_args_then_construct(masm: &mut /*MacroAssembler*/ i32) {
        Builtins::generate_interpreter_push_args_then_construct_impl(
            masm,
            InterpreterPushArgsMode::kOther,
        );
    }

    fn generate_interpreter_push_args_then_construct_with_final_spread(
        masm: &mut /*MacroAssembler*/ i32,
    ) {
        Builtins::generate_interpreter_push_args_then_construct_impl(
            masm,
            InterpreterPushArgsMode::kWithFinalSpread,
        );
    }

    fn generate_interpreter_push_args_then_construct_array_function(
        masm: &mut /*MacroAssembler*/ i32,
    ) {
        Builtins::generate_interpreter_push_args_then_construct_impl(
            masm,
            InterpreterPushArgsMode::kArrayFunction,
        );
    }

    fn generate_interpreter_push_args_then_call_impl(
        masm: &mut /*MacroAssembler*/ i32, // Placeholder type
        receiver_mode: ConvertReceiverMode,
        push_mode: InterpreterPushArgsMode,
    ) {
        // TODO(someone): Implement the architecture specific logic here.
        // masm.Placeholder(receiver_mode, push_mode);
    }

    fn generate_interpreter_push_args_then_construct_impl(
        masm: &mut /*MacroAssembler*/ i32, // Placeholder type
        push_mode: InterpreterPushArgsMode,
    ) {
        // TODO(someone): Implement the architecture specific logic here.
        // masm.Placeholder(push_mode);
    }
}