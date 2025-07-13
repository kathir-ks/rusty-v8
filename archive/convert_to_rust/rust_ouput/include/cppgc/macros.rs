// Converted from V8 C++ source files:
// Header: macros.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {

  #[cfg(feature = "clang")]
  macro_rules! CPPGC_STACK_ALLOCATED {
      () => {
          pub type IsStackAllocatedTypeMarker = i32;
          #[allow(dead_code)]
          fn new(_size: usize) -> ! {
              panic!("Allocation on heap is forbidden for this type");
          }
          #[allow(dead_code)]
          fn new_with_place(_size: usize, _place: *mut std::ffi::c_void) -> ! {
              panic!("Placement new is forbidden for this type");
          }
      };
  }

  #[cfg(not(feature = "clang"))]
  macro_rules! CPPGC_STACK_ALLOCATED {
      () => {
          static_assert!(true);
      };
  }

  #[cfg(feature = "clang")]
  macro_rules! CPPGC_STACK_ALLOCATED_IGNORE {
      ($bug_or_reason:expr) => {
          #[allow(unused_attributes)]
          #[rustfmt::skip]
          #[cfg_attr(feature = "clang", link_section = ".llvm.metadata")]
          #[cfg_attr(feature = "clang", used)]
          static STACK_ALLOCATED_IGNORE_MARKER: &'static [u8] = b"stack_allocated_ignore\0";
      };
  }

  #[cfg(not(feature = "clang"))]
  macro_rules! CPPGC_STACK_ALLOCATED_IGNORE {
      ($bug_or_reason:expr) => {};
  }
  pub(crate) use CPPGC_STACK_ALLOCATED;
  pub(crate) use CPPGC_STACK_ALLOCATED_IGNORE;
  pub trait IsStackAllocatedType {
      type IsStackAllocatedTypeMarker;
  }
} // namespace cppgc
