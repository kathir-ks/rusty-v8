// Converted from V8 C++ source files:
// Header: trap-handler-simulator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::ffi::c_void;

#[cfg(feature = "trap_handler_via_simulator")]
mod trap_handler_simulator {
    extern "C" {
        // Probe a memory address by doing a 1-byte read from the given address.
        // If the address is not readable, this will cause a trap as usual, but the trap
        // handler will recognise the address of the instruction doing the access and
        // treat it specially. It will use the given {pc} to look up the respective
        // landing pad and return to this function to return that landing pad. If {pc}
        // is not registered as a protected instruction, the signal will be propagated
        // as usual.
        // If the read at {address} succeeds, this function returns {0} instead.
        #[cfg(target_os = "macos")]
        #[link_name = "_v8_internal_simulator_ProbeMemory"]
        pub fn v8_internal_simulator_ProbeMemory(address: usize, pc: usize) -> usize;

        #[cfg(not(target_os = "macos"))]
        #[link_name = "v8_internal_simulator_ProbeMemory"]
        pub fn v8_internal_simulator_ProbeMemory(address: usize, pc: usize) -> usize;
    }

    pub fn ProbeMemory(address: usize, pc: usize) -> usize {
        unsafe { v8_internal_simulator_ProbeMemory(address, pc) }
    }
}

#[cfg(feature = "trap_handler_via_simulator")]
pub use trap_handler_simulator::ProbeMemory;
