// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "enable_webassembly"))]
// compile_error!("This header should only be included if WebAssembly is enabled.");

use std::cmp;
use std::mem;
use std::vec::Vec;

//use crate::base::memory; // Assuming this exists in a translated form
//use crate::codegen::signature::CanonicalSig; // Assuming this exists in a translated form
//use crate::common::globals::kSystemPointerSize; // Assuming this is a constant
//use crate::wasm::value_type::CanonicalValueType; // Assuming this exists in a translated form

/// Helper class for {Push}ing Wasm value arguments onto the stack in the format
/// that the CWasmEntryStub expects, as well as for {Pop}ping return values.
/// {Reset} must be called if a packer instance used for pushing is then
/// reused for popping: it resets the internal pointer to the beginning of
/// the stack region.
pub struct CWasmArgumentsPacker {
    on_stack_buffer: [u8; CWasmArgumentsPacker::K_MAX_ON_STACK_BUFFER],
    heap_buffer: Vec<u8>,
    buffer: *mut u8,
    offset: usize,
}

impl CWasmArgumentsPacker {
    const K_MAX_ON_STACK_BUFFER: usize = 10 * 8; // Assuming kSystemPointerSize is 8 (typical 64-bit)

    pub fn new(buffer_size: usize) -> Self {
        let mut on_stack_buffer = [0u8; Self::K_MAX_ON_STACK_BUFFER];
        let mut heap_buffer = Vec::new();
        let buffer: *mut u8;

        if buffer_size <= Self::K_MAX_ON_STACK_BUFFER {
            buffer = on_stack_buffer.as_mut_ptr();
        } else {
            heap_buffer.resize(buffer_size, 0);
            buffer = heap_buffer.as_mut_ptr();
        }

        CWasmArgumentsPacker {
            on_stack_buffer,
            heap_buffer,
            buffer,
            offset: 0,
        }
    }

    pub fn argv(&self) -> *mut u8 {
        self.buffer
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn push<T>(&mut self, val: T) {
        let size = mem::size_of::<T>();
        unsafe {
            let address = self.buffer.add(self.offset) as *mut T;
            *address = val;
        }
        self.offset += size;
    }

    pub fn pop<T>(&mut self) -> T {
        let size = mem::size_of::<T>();
        let result: T;
        unsafe {
            let address = self.buffer.add(self.offset) as *const T;
            result = *address;
        }
        self.offset += size;
        result
    }

    // The CanonicalSig type and CanonicalValueType types are not available so TotalSize function is not fully translated.
    /*
    pub fn total_size(sig: &CanonicalSig) -> usize {
        let mut return_size = 0;
        for t in sig.returns() {
            return_size += t.value_kind_full_size();
        }
        let mut param_size = 0;
        for t in sig.parameters() {
            param_size += t.value_kind_full_size();
        }
        cmp::max(return_size, param_size)
    }
    */
}