// Converted from V8 C++ source files:
// Header: globals.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

pub type Address = *mut u8;
pub type ConstAddress = *const u8;

pub const K_KB: usize = 1024;
pub const K_MB: usize = K_KB * 1024;
pub const K_GB: usize = K_MB * 1024;

// AccessMode used for choosing between atomic and non-atomic accesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    KNonAtomic,
    KAtomic,
}

// See 6.7.6 (http://eel.is/c++draft/basic.align) for alignment restrictions. We
// do not fully support all alignment restrictions (following
// alignof(std​::​max_­align_­t)) but limit to alignof(double).
//
// This means that any scalar type with stricter alignment requirements (in
// practice: long double) cannot be used unrestricted in garbage-collected
// objects.
#[cfg(target_arch = "x86_64")]
pub const K_ALLOCATION_GRANULARITY: usize = 8;
#[cfg(not(target_arch = "x86_64"))]
pub const K_ALLOCATION_GRANULARITY: usize = 4;
pub const K_ALLOCATION_MASK: usize = K_ALLOCATION_GRANULARITY - 1;

pub const K_PAGE_SIZE_LOG2: usize = 17;
pub const K_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_LOG2;
pub const K_PAGE_OFFSET_MASK: usize = K_PAGE_SIZE - 1;
pub const K_PAGE_BASE_MASK: usize = !K_PAGE_OFFSET_MASK;

pub const K_LARGE_OBJECT_SIZE_THRESHOLD: usize = K_PAGE_SIZE / 2;

pub const K_FREE_LIST_GC_INFO_INDEX: usize = 0;
pub const K_FREE_LIST_ENTRY_SIZE: usize = 2 * std::mem::size_of::<usize>();

#[cfg(feature = "cppgc_pointer_compression")]
pub const K_SLOT_SIZE: usize = std::mem::size_of::<u32>();
#[cfg(not(feature = "cppgc_pointer_compression"))]
pub const K_SLOT_SIZE: usize = std::mem::size_of::<usize>();
}  // namespace internal
}  // namespace cppgc
