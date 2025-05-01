// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This Rust code is a direct translation and may not be the most idiomatic
// or efficient way to achieve the same functionality in Rust. It aims to mirror
// the C++ code structure.

// include/v8-internal.h is assumed to define Address. Here we use usize.
// src/common/ptr-compr-inl.h is partially implemented with PtrComprCageBase.
// src/sandbox/sandbox.h is partially implemented with Sandbox.
// src/sandbox/sandboxed-pointer.h is partially implemented with SandboxedPointer_t.

// The original C++ code uses preprocessor macros and inline functions.
// We use const values and inline functions in Rust.

pub type Address = usize; // Or u64, depending on architecture.

// This could be defined in src/sandbox/sandboxed-pointer.h
pub type SandboxedPointer_t = usize; // Or u64, depending on architecture.

// Assume kSandboxedPointerShift is defined elsewhere
const K_SANDBOXED_POINTER_SHIFT: usize = 3; // Example value, adapt as needed

// A minimal representation of PtrComprCageBase from src/common/ptr-compr-inl.h
#[derive(Clone, Copy)]
pub struct PtrComprCageBase {
    address_: Address,
}

impl PtrComprCageBase {
    pub fn new(address: Address) -> Self {
        PtrComprCageBase { address_: address }
    }
    pub fn address(&self) -> Address {
        self.address_
    }
}

// A minimal representation of Sandbox from src/sandbox/sandbox.h
pub struct Sandbox {
    // Add fields here that define the sandbox boundaries, if needed.
    start: Address,
    end: Address,
}

impl Sandbox {
    pub fn new(start: Address, end: Address) -> Self {
        Sandbox { start, end }
    }
    pub fn contains(&self, address: Address) -> bool {
        address >= self.start && address < self.end
    }

    // Returns a global singleton representing the "current" sandbox
    pub fn current() -> &'static Sandbox {
        // This is a placeholder. A more robust mechanism for managing the sandbox
        // instance might be needed (e.g., thread-local storage, etc.).
        static SANDBOX: Sandbox = Sandbox::new(0x100000000000, 0x200000000000); // Example values
        &SANDBOX
    }
}

#[inline]
pub fn read_sandboxed_pointer_field(field_address: Address, cage_base: PtrComprCageBase) -> Address {
    #[cfg(feature = "sandbox")]
    {
        let sandboxed_pointer = unsafe { (field_address as *const SandboxedPointer_t).read_unaligned() };

        let offset = sandboxed_pointer >> K_SANDBOXED_POINTER_SHIFT;
        let pointer = cage_base.address() + offset;
        pointer
    }

    #[cfg(not(feature = "sandbox"))]
    {
        unsafe { (field_address as *const Address).read_unaligned() }
    }
}

#[inline]
pub fn write_sandboxed_pointer_field(field_address: Address, cage_base: PtrComprCageBase, pointer: Address) {
    #[cfg(feature = "sandbox")]
    {
        // The pointer must point into the sandbox.
        debug_assert!(Sandbox::current().contains(pointer));

        let offset = pointer - cage_base.address();
        let sandboxed_pointer = offset << K_SANDBOXED_POINTER_SHIFT;
        unsafe { (field_address as *mut SandboxedPointer_t).write_unaligned(sandboxed_pointer) };
    }

    #[cfg(not(feature = "sandbox"))]
    {
        unsafe { (field_address as *mut Address).write_unaligned(pointer) };
    }
}