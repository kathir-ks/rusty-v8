// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use std::ptr;

/// Represents the callee-saved registers for the ARM architecture.
#[derive(Debug)]
pub struct CalleeSavedRegisters {
    pub arm_r4: *mut std::ffi::c_void,
    pub arm_r5: *mut std::ffi::c_void,
    pub arm_r6: *mut std::ffi::c_void,
    pub arm_r7: *mut std::ffi::c_void,
    pub arm_r8: *mut std::ffi::c_void,
    pub arm_r9: *mut std::ffi::c_void,
    pub arm_r10: *mut std::ffi::c_void,
}

/// Represents the register state.
#[derive(Debug)]
pub struct RegisterState {
    pub callee_saved: Option<Box<CalleeSavedRegisters>>,
}

impl RegisterState {
    pub fn new() -> Self {
        RegisterState {
            callee_saved: None,
        }
    }
}

const K_SYSTEM_POINTER_SIZE: usize = mem::size_of::<usize>();

mod entry_frame_constants {
  pub const K_DIRECT_CALLER_GENERAL_REGISTERS_OFFSET: usize = 8; // Example value, replace with actual value
}

/// Loads a value from memory at the given address.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.  The caller
/// must ensure that the pointer is valid and that the memory it points to
/// is accessible.
unsafe fn load_usize(address: usize) -> *mut std::ffi::c_void {
    *(address as *const *mut std::ffi::c_void)
}

/// Retrieves the callee-saved registers from an entry frame.
///
/// # Safety
///
/// The `fp` pointer must point to a valid entry frame.
pub unsafe fn get_callee_saved_registers_from_entry_frame(
    fp: *mut std::ffi::c_void,
    register_state: &mut RegisterState,
) {
    let base_addr = (fp as usize) + entry_frame_constants::K_DIRECT_CALLER_GENERAL_REGISTERS_OFFSET;

    if register_state.callee_saved.is_none() {
        register_state.callee_saved = Some(Box::new(CalleeSavedRegisters {
            arm_r4: ptr::null_mut(),
            arm_r5: ptr::null_mut(),
            arm_r6: ptr::null_mut(),
            arm_r7: ptr::null_mut(),
            arm_r8: ptr::null_mut(),
            arm_r9: ptr::null_mut(),
            arm_r10: ptr::null_mut(),
        }));
    }

    if let Some(callee_saved) = &mut register_state.callee_saved {
        callee_saved.arm_r4 = load_usize(base_addr + 0 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r5 = load_usize(base_addr + 1 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r6 = load_usize(base_addr + 2 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r7 = load_usize(base_addr + 3 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r8 = load_usize(base_addr + 4 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r9 = load_usize(base_addr + 5 * K_SYSTEM_POINTER_SIZE);
        callee_saved.arm_r10 = load_usize(base_addr + 6 * K_SYSTEM_POINTER_SIZE);
    }
}