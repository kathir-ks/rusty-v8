// Converted from V8 C++ source files:
// Header: v8-microtask.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {

pub struct Isolate;

pub type MicrotasksCompletedCallbackWithData =
    Option<unsafe extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void)>;
pub type MicrotaskCallback = Option<unsafe extern "C" fn(data: *mut std::ffi::c_void)>;

/**
 * Policy for running microtasks:
 *   - explicit: microtasks are invoked with the
 *               Isolate::PerformMicrotaskCheckpoint() method;
 *   - scoped: microtasks invocation is controlled by MicrotasksScope objects;
 *   - auto: microtasks are invoked when the script call depth decrements
 *           to zero.
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MicrotasksPolicy {
    kExplicit,
    kScoped,
    kAuto,
}

} // namespace v8
