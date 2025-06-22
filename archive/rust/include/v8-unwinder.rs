// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file `v8-unwinder.h`.

/// Holds the callee saved registers needed for the stack unwinder.
/// It is the empty struct if no registers are required.
/// Implemented in include/v8-unwinder-state.h.
pub struct CalleeSavedRegisters {}

/// A RegisterState represents the current state of registers used
/// by the sampling profiler API.
#[derive(Debug, Clone)]
pub struct RegisterState {
    /// Instruction pointer.
    pub pc: *mut std::ffi::c_void,
    /// Stack pointer.
    pub sp: *mut std::ffi::c_void,
    /// Frame pointer.
    pub fp: *mut std::ffi::c_void,
    /// Link register (or nullptr on platforms without a link register).
    pub lr: *mut std::ffi::c_void,
    /// Callee saved registers (or null if no callee saved registers were stored)
    pub callee_saved: Option<Box<CalleeSavedRegisters>>,
}

impl RegisterState {
    pub fn new() -> Self {
        RegisterState {
            pc: std::ptr::null_mut(),
            sp: std::ptr::null_mut(),
            fp: std::ptr::null_mut(),
            lr: std::ptr::null_mut(),
            callee_saved: None,
        }
    }
}

impl Default for RegisterState {
    fn default() -> Self {
        Self::new()
    }
}

/// A StateTag represents a possible state of the VM.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum StateTag {
    JS,
    GC,
    PARSER,
    BYTECODE_COMPILER,
    COMPILER,
    OTHER,
    EXTERNAL,
    ATOMICS_WAIT,
    IDLE,
    LOGGING,
}

/// The output structure filled up by GetStackSample API function.
#[derive(Debug, Clone)]
pub struct SampleInfo {
    /// Number of frames collected.
    pub frames_count: usize,
    /// External callback address if VM is executing an external callback.
    pub external_callback_entry: *mut std::ffi::c_void,
    /// Incumbent native context address.
    pub context: *mut std::ffi::c_void,
    /// Native context address for embedder state
    pub embedder_context: *mut std::ffi::c_void,
    /// Current VM state.
    pub vm_state: StateTag,
    /// Current Embedder state
    pub embedder_state: EmbedderStateTag,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum EmbedderStateTag {
    NONE,
    API,
    NATIVES,
    WEB_ASSEMBLY,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRange {
    pub start: *const std::ffi::c_void,
    pub length_in_bytes: usize,
}

impl MemoryRange {
    pub fn new() -> Self {
        MemoryRange {
            start: std::ptr::null(),
            length_in_bytes: 0,
        }
    }
}

impl Default for MemoryRange {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JSEntryStub {
    pub code: MemoryRange,
}

#[derive(Debug, Copy, Clone)]
pub struct JSEntryStubs {
    pub js_entry_stub: JSEntryStub,
    pub js_construct_entry_stub: JSEntryStub,
    pub js_run_microtasks_entry_stub: JSEntryStub,
}

/// Various helpers for skipping over V8 frames in a given stack.
///
/// The unwinder API is only supported on the x64, ARM64 and ARM32 architectures.
pub struct Unwinder {}

impl Unwinder {
    /// Attempt to unwind the stack to the most recent C++ frame. This function is
    /// signal-safe and does not access any V8 state and thus doesn't require an
    /// Isolate.
    ///
    /// The unwinder needs to know the location of the JS Entry Stub (a piece of
    /// code that is run when C++ code calls into generated JS code). This is used
    /// for edge cases where the current frame is being constructed or torn down
    /// when the stack sample occurs.
    ///
    /// The unwinder also needs the virtual memory range of all possible V8 code
    /// objects. There are two ranges required - the heap code range and the range
    /// for code embedded in the binary.
    ///
    /// Available on x64, ARM64 and ARM32.
    ///
    /// \param code_pages A list of all of the ranges in which V8 has allocated
    /// executable code. The caller should obtain this list by calling
    /// Isolate::CopyCodePages() during the same interrupt/thread suspension that
    /// captures the stack.
    /// \param register_state The current registers. This is an in-out param that
    /// will be overwritten with the register values after unwinding, on success.
    /// \param stack_base The resulting stack pointer and frame pointer values are
    /// bounds-checked against the stack_base and the original stack pointer value
    /// to ensure that they are valid locations in the given stack. If these values
    /// or any intermediate frame pointer values used during unwinding are ever out
    /// of these bounds, unwinding will fail.
    ///
    /// \return True on success.
    pub fn try_unwind_v8_frames(
        entry_stubs: &JSEntryStubs,
        code_pages_length: usize,
        code_pages: &[MemoryRange],
        register_state: &mut RegisterState,
        stack_base: *const std::ffi::c_void,
    ) -> bool {
        // This function needs architecture specific implementation
        // that would likely involve inline assembly or usage of
        // platform specific APIs for stack walking.  For now, it
        // always returns false.
        false
    }

    /// Whether the PC is within the V8 code range represented by code_pages.
    ///
    /// If this returns false, then calling UnwindV8Frames() with the same PC
    /// and unwind_state will always fail. If it returns true, then unwinding may
    /// (but not necessarily) be successful.
    ///
    /// Available on x64, ARM64 and ARM32
    pub fn pc_is_in_v8(
        code_pages_length: usize,
        code_pages: &[MemoryRange],
        pc: *mut std::ffi::c_void,
    ) -> bool {
        for i in 0..code_pages_length {
            let range = &code_pages[i];
            if pc >= range.start as *mut std::ffi::c_void
                && pc < unsafe { range.start.add(range.length_in_bytes) } as *mut std::ffi::c_void
            {
                return true;
            }
        }
        false
    }
}