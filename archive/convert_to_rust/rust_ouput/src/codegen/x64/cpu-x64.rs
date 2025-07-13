// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
use std::os::raw::c_void;

pub struct CpuFeatures {}

impl CpuFeatures {
    pub fn flush_i_cache(start: *mut c_void, size: usize) {
        // No need to flush the instruction cache on Intel. On Intel instruction
        // cache flushing is only necessary when multiple cores running the same
        // code simultaneously. V8 (and JavaScript) is single threaded and when code
        // is patched on an intel CPU the core performing the patching will have its
        // own instruction cache updated automatically.

        // If flushing of the instruction cache becomes necessary Windows has the
        // API function FlushInstructionCache.

        // By default, valgrind only checks the stack for writes that might need to
        // invalidate already cached translated code.  This leads to random
        // instability when code patches or moves are sometimes unnoticed.  One
        // solution is to run valgrind with --smc-check=all, but this comes at a big
        // performance cost.  We can notify valgrind to invalidate its cache.
        // VALGRIND_DISCARD_TRANSLATIONS is not available in Rust, skipping the
        // equivalent functionality
    }
}
