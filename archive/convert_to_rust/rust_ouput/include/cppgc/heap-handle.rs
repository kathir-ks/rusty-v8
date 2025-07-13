// Converted from V8 C++ source files:
// Header: heap-handle.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    pub struct HeapBase {}
    pub struct WriteBarrierTypeForCagedHeapPolicy {}
    pub struct WriteBarrierTypeForNonCagedHeapPolicy {}
}

/**
 * Opaque handle used for additional heap APIs.
 */
#[derive(Default)]
pub struct HeapHandle {
  is_incremental_marking_in_progress_: bool,
  is_young_generation_enabled_: bool,
}

impl HeapHandle {
  // Deleted copy ctor to avoid treating the type by value.
  // HeapHandle(const HeapHandle&) = delete;
  // HeapHandle& operator=(const HeapHandle&) = delete;

  // private:
  // HeapHandle() = default;

  #[inline]
  pub fn is_incremental_marking_in_progress(&self) -> bool {
    self.is_incremental_marking_in_progress_
  }

  #[inline]
  pub fn is_young_generation_enabled(&self) -> bool {
    self.is_young_generation_enabled_
  }

  // friend class internal::HeapBase;
  // friend class internal::WriteBarrierTypeForCagedHeapPolicy;
  // friend class internal::WriteBarrierTypeForNonCagedHeapPolicy;
}
