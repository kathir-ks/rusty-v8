// Converted from V8 C++ source files:
// Header: ios-headers.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file includes the necessary headers that are not part of the
// iOS public SDK in order to support memory allocation on iOS.

use std::ffi::c_int;

// These type definitions are necessary because the mach crate does not expose them.
// They are taken from /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/mach_types.h

pub type vm_map_t = mach::port::mach_port_t;
pub type mach_vm_address_t = mach::vm_types::mach_vm_address_t;
pub type mach_vm_size_t = mach::vm_types::mach_vm_size_t;
pub type mach_vm_offset_t = mach::vm_types::mach_vm_offset_t;
pub type vm_prot_t = c_int;
pub type vm_inherit_t = c_int;
pub type mem_entry_name_port_t = mach::port::mach_port_t;
pub type memory_object_offset_t = u64;
pub type boolean_t = mach::boolean::boolean_t;

extern "C" {
    pub fn mach_vm_remap(
        target_task: vm_map_t,
        target_address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: c_int,
        src_task: vm_map_t,
        src_address: mach_vm_address_t,
        copy: boolean_t,
        cur_protection: *mut vm_prot_t,
        max_protection: *mut vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> mach::kern_return::kern_return_t;

    pub fn mach_vm_map(
        target_task: vm_map_t,
        address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: c_int,
        object: mem_entry_name_port_t,
        offset: memory_object_offset_t,
        copy: boolean_t,
        cur_protection: vm_prot_t,
        max_protection: vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> mach::kern_return::kern_return_t;
}
