// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header defines the ProbeMemory function to be used by simulators to
// trigger a signal at a defined location, before doing an actual memory access.

#[cfg(feature = "trap_handler_via_simulator")]
pub mod trap_handler_simulator {
    use std::ffi::c_uint;

    /// Probe a memory address by doing a 1-byte read from the given address. If the
    /// address is not readable, this will cause a trap as usual, but the trap
    /// handler will recognise the address of the instruction doing the access and
    /// treat it specially. It will use the given {pc} to look up the respective
    /// landing pad and return to this function to return that landing pad. If {pc}
    /// is not registered as a protected instruction, the signal will be propagated
    /// as usual.
    /// If the read at {address} succeeds, this function returns {0} instead.
    #[cfg(target_os = "macos")]
    #[link(name = "v8_internal_simulator_ProbeMemory", kind = "static")]
    extern "C" {
        pub fn v8_internal_simulator_ProbeMemory(address: usize, pc: usize) -> usize;
    }

    #[cfg(not(target_os = "macos"))]
    extern "C" {
        #[link_name = "v8_internal_simulator_ProbeMemory"]
        pub fn v8_internal_simulator_ProbeMemory(address: usize, pc: usize) -> usize;
    }
}