// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module defines constants related to WebAssembly limits.

// These constants limit the amount of *declared* memory. At runtime, memory can
// only grow up to kV8MaxWasmMemory{32,64}Pages.
// The spec limits are defined in
// https://webassembly.github.io/spec/js-api/index.html#limits.
pub const K_SPEC_MAX_MEMORY32_PAGES: usize = 65_536; // 4GB
pub const K_SPEC_MAX_MEMORY64_PAGES: usize = 262_144; // 16GB

// The following limits are imposed by V8 on WebAssembly modules.
// The limits are agreed upon with other engines for consistency.
pub const K_V8_MAX_WASM_TYPES: usize = 1_000_000;
pub const K_V8_MAX_WASM_DEFINED_FUNCTIONS: usize = 1_000_000;
pub const K_V8_MAX_WASM_IMPORTS: usize = 1_000_000;
pub const K_V8_MAX_WASM_EXPORTS: usize = 1_000_000;
pub const K_V8_MAX_WASM_GLOBALS: usize = 1_000_000;
pub const K_V8_MAX_WASM_TAGS: usize = 1_000_000;
pub const K_V8_MAX_WASM_EXCEPTION_TYPES: usize = 1_000_000;
pub const K_V8_MAX_WASM_DATA_SEGMENTS: usize = 100_000;

// This indicates the maximum memory size our implementation supports.
// Do not use this limit directly; use {max_mem{32,64}_pages()} instead to take
// the spec'ed limit as well as command line flag into account.
// Also, do not use this limit to validate declared memory, use
// kSpecMaxMemory{32,64}Pages for that.
#[cfg(target_pointer_width = "32")]
pub const K_V8_MAX_WASM_MEMORY32_PAGES: usize = 32_767; // = 2 GiB - 64Kib
#[cfg(target_pointer_width = "64")]
pub const K_V8_MAX_WASM_MEMORY32_PAGES: usize = 65_536; // = 4 GiB

#[cfg(target_pointer_width = "32")]
pub const K_V8_MAX_WASM_MEMORY64_PAGES: usize = 32_767; // = 2 GiB - 64Kib
#[cfg(target_pointer_width = "64")]
pub const K_V8_MAX_WASM_MEMORY64_PAGES: usize = 262_144; // = 16 GiB

pub const K_V8_MAX_WASM_STRING_SIZE: usize = 100_000;
pub const K_V8_MAX_WASM_MODULE_SIZE: usize = 1024 * 1024 * 1024; // = 1 GiB
pub const K_V8_MAX_WASM_FUNCTION_SIZE: usize = 7_654_321;
pub const K_V8_MAX_WASM_FUNCTION_LOCALS: usize = 50_000;
pub const K_V8_MAX_WASM_FUNCTION_PARAMS: usize = 1_000;
pub const K_V8_MAX_WASM_FUNCTION_RETURNS: usize = 1_000;
pub const K_V8_MAX_WASM_FUNCTION_BR_TABLE_SIZE: usize = 65_520;

// Don't use this limit directly, but use the value of
// v8_flags.wasm_max_table_size.
pub const K_V8_MAX_WASM_TABLE_SIZE: usize = 10_000_000;
pub const K_V8_MAX_WASM_TABLE_INIT_ENTRIES: usize = 10_000_000;
pub const K_V8_MAX_WASM_TABLES: usize = 100_000;
pub const K_V8_MAX_WASM_MEMORIES: usize = 100_000;

// GC proposal.
pub const K_V8_MAX_WASM_STRUCT_FIELDS: usize = 10_000;
pub const K_V8_MAX_RTT_SUBTYPING_DEPTH: u32 = 63;
pub const K_V8_MAX_WASM_ARRAY_NEW_FIXED_LENGTH: usize = 10_000;

// Stringref proposal. This limit is not standardized yet.
pub const K_V8_MAX_WASM_STRING_LITERALS: usize = 1_000_000;

const _: () = assert!(
    K_V8_MAX_WASM_TABLE_SIZE <= 4294967295,
    "v8 should not exceed WebAssembly's non-web embedding limits"
);
const _: () = assert!(
    K_V8_MAX_WASM_TABLE_INIT_ENTRIES <= K_V8_MAX_WASM_TABLE_SIZE,
    "JS-API should not exceed v8's limit"
);

// 64-bit platforms support the full spec'ed memory limits.
#[cfg(target_pointer_width = "64")]
const _: () = assert!(
    K_V8_MAX_WASM_MEMORY32_PAGES == K_SPEC_MAX_MEMORY32_PAGES
        && K_V8_MAX_WASM_MEMORY64_PAGES == K_SPEC_MAX_MEMORY64_PAGES
);

pub const K_WASM_MAX_HEAP_OFFSET: u64 =
    u32::MAX as u64 + u32::MAX as u64;

// This limit is a result of the limits for defined functions and the maximum of
// imported functions.
pub const K_V8_MAX_WASM_TOTAL_FUNCTIONS: usize =
    K_V8_MAX_WASM_DEFINED_FUNCTIONS + K_V8_MAX_WASM_IMPORTS;

// The following functions are defined in wasm-engine.cc.
// They need to be declared as extern "C" and have #[no_mangle]
// for Rust to be able to use them.  Since we do not have the
// wasm-engine.cc implementation, we can only declare the functions
// without implementation, which will cause linking errors.

// Maximum number of pages we can allocate, for memory32 and memory64. This
// might be lower than the number of pages that can be declared (e.g. as
// maximum): kSpecMaxMemory{32,64}Pages.
// Even for 64-bit memory, the number of pages is still a 32-bit number for now,
// which allows for up to 128 TB memories (2**31 * 64k).
const _: () = assert!(K_V8_MAX_WASM_MEMORY64_PAGES <= u32::MAX as usize);

extern "C" {
    // #[no_mangle]  // Need to be added to the actual implementation.
    pub fn max_mem32_pages() -> u32;

    // #[no_mangle]  // Need to be added to the actual implementation.
    pub fn max_mem64_pages() -> u32;
    // #[no_mangle]  // Need to be added to the actual implementation.
    pub fn max_table_size() -> u32;
    // #[no_mangle]  // Need to be added to the actual implementation.
    pub fn max_table_init_entries() -> u32;
    // #[no_mangle]  // Need to be added to the actual implementation.
    pub fn max_module_size() -> usize;
}

pub const K_WASM_PAGE_SIZE: u64 = 65536; // 64 KiB

pub fn max_mem32_bytes() -> u64 {
    unsafe { u64::from(max_mem32_pages()) * K_WASM_PAGE_SIZE }
}

pub fn max_mem64_bytes() -> u64 {
    unsafe { u64::from(max_mem64_pages()) * K_WASM_PAGE_SIZE }
}

// The maximum memory64 size supported by our implementation, in bytes.
pub const K_MAX_MEMORY64_SIZE: usize =
    K_V8_MAX_WASM_MEMORY64_PAGES * K_WASM_PAGE_SIZE as usize;