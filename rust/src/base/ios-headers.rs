// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file includes the necessary headers that are not part of the
// iOS public SDK in order to support memory allocation on iOS.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub mod ios_headers {
    use libc::{c_int, c_ulong, c_uint, c_void};

    pub type kern_return_t = c_int;
    pub type vm_map_t = c_uint; // task_t
    pub type mach_vm_address_t = c_ulong;
    pub type mach_vm_size_t = c_ulong;
    pub type mach_vm_offset_t = c_ulong;
    pub type boolean_t = c_uint;
    pub type vm_prot_t = c_int;
    pub type vm_inherit_t = c_int;
    pub type mem_entry_name_port_t = c_uint; //mach_port_t
    pub type memory_object_offset_t = c_ulong;

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
        ) -> kern_return_t;

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
        ) -> kern_return_t;
    }
}