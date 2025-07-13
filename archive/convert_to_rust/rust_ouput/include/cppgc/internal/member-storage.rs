// Converted from V8 C++ source files:
// Header: member-storage.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::sync::atomic::{AtomicU32, AtomicPtr, Ordering};
  use std::{
    mem::{align_of, size_of},
    ptr::null_mut,
  };

  pub enum WriteBarrierSlotType {
    kCompressed,
    kUncompressed,
  }

  #[cfg(feature = "CPPGC_POINTER_COMPRESSION")]
  mod pointer_compression {
    use super::*;
    use crate::SentinelPointer;

    const CPPGC_CONST: &str = "";
    const CPPGC_REQUIRE_CONSTANT_INIT: &str = "";

    pub struct CageBaseGlobal {}

    impl CageBaseGlobal {
      const K_LOWER_HALF_WORD_MASK: usize =
        crate::api_constants::K_CAGED_HEAP_RESERVATION_ALIGNMENT - 1;

      pub fn get() -> usize {
        assert!(Self::is_base_consistent());
        unsafe { g_base.base.load(Ordering::Relaxed) as usize }
      }

      pub fn is_set() -> bool {
        assert!(Self::is_base_consistent());
        (unsafe { g_base.base.load(Ordering::Relaxed) as usize }
          & !Self::K_LOWER_HALF_WORD_MASK)
          != 0
      }

      fn is_base_consistent() -> bool {
        Self::K_LOWER_HALF_WORD_MASK
          == (unsafe { g_base.base.load(Ordering::Relaxed) as usize }
            & Self::K_LOWER_HALF_WORD_MASK)
      }
    }

    #[repr(align(128))]
    union Base {
      base: AtomicUsize,
      cache_line: [u8; crate::api_constants::K_CACHELINE_SIZE],
    }

    static mut g_base: Base = Base {
      base: AtomicUsize::new(crate::api_constants::K_CAGED_HEAP_RESERVATION_ALIGNMENT - 1),
    };

    pub struct CompressedPointer {
      value_: u32,
    }

    impl CompressedPointer {
      pub type IntegralType = u32;
      pub const K_WRITE_BARRIER_SLOT_TYPE: WriteBarrierSlotType =
        WriteBarrierSlotType::kCompressed;

      pub fn new() -> Self {
        CompressedPointer { value_: 0 }
      }

      pub fn from_ptr_atomic(value: *const std::ffi::c_void) -> Self {
        let mut result = Self::new();
        result.store_atomic(value);
        result
      }

      pub fn from_ptr(ptr: *const std::ffi::c_void) -> Self {
        CompressedPointer {
          value_: Self::compress(ptr),
        }
      }

      pub fn from_nullptr() -> Self {
        CompressedPointer { value_: 0 }
      }

      pub fn from_sentinel() -> Self {
        CompressedPointer {
          value_: Self::K_COMPRESSED_SENTINEL,
        }
      }

      pub fn load(&self) -> *const std::ffi::c_void {
        Self::decompress(self.value_) as *const std::ffi::c_void
      }

      pub fn load_atomic(&self) -> *const std::ffi::c_void {
        let value = unsafe {
          (&self.value_ as *const u32 as *const AtomicU32)
            .as_ref()
            .unwrap()
            .load(Ordering::Relaxed)
        };
        Self::decompress(value) as *const std::ffi::c_void
      }

      pub fn store(&mut self, ptr: *const std::ffi::c_void) {
        self.value_ = Self::compress(ptr);
      }

      pub fn store_atomic(&mut self, value: *const std::ffi::c_void) {
        let compressed_value = Self::compress(value);
        unsafe {
          (&mut self.value_ as *mut u32 as *mut AtomicU32)
            .as_mut()
            .unwrap()
            .store(compressed_value, Ordering::Relaxed);
        }
      }

      pub fn clear(&mut self) {
        self.value_ = 0;
      }

      pub fn is_cleared(&self) -> bool {
        self.value_ == 0
      }

      pub fn is_sentinel(&self) -> bool {
        self.value_ == Self::K_COMPRESSED_SENTINEL
      }

      pub fn get_as_integer(&self) -> u32 {
        self.value_
      }

      const K_COMPRESSED_SENTINEL: u32 =
        (SentinelPointer::K_SENTINEL_VALUE >> crate::api_constants::K_POINTER_COMPRESSION_SHIFT)
          as u32;

      fn compress(ptr: *const std::ffi::c_void) -> u32 {
        assert_eq!(
          SentinelPointer::K_SENTINEL_VALUE,
          1 << crate::api_constants::K_POINTER_COMPRESSION_SHIFT,
          "The compression scheme relies on the sentinel encoded as 1 << kPointerCompressionShift"
        );

        const K_GIGA_CAGE_MASK: usize =
          !(crate::api_constants::K_CAGED_HEAP_RESERVATION_ALIGNMENT - 1);
        const K_POINTER_COMPRESSION_SHIFT_MASK: usize =
          (1 << crate::api_constants::K_POINTER_COMPRESSION_SHIFT) - 1;

        assert!(CageBaseGlobal::is_set());
        let base = CageBaseGlobal::get();
        assert!(
          ptr.is_null()
            || ptr as usize == SentinelPointer::kSentinelValue() as usize
            || (base & K_GIGA_CAGE_MASK)
              == (ptr as usize & K_GIGA_CAGE_MASK),
          "ptr={:p}, base={:#x}, K_GIGA_CAGE_MASK={:#x}",
          ptr,
          base,
          K_GIGA_CAGE_MASK
        );
        assert!(
          (ptr as usize & K_POINTER_COMPRESSION_SHIFT_MASK) == 0,
          "ptr={:p}, K_POINTER_COMPRESSION_SHIFT_MASK={:#x}",
          ptr,
          K_POINTER_COMPRESSION_SHIFT_MASK
        );

        let uptr = ptr as usize;
        let compressed = (uptr >> crate::api_constants::K_POINTER_COMPRESSION_SHIFT) as u32;
        assert!(
          compressed == 0
            || compressed == Self::K_COMPRESSED_SENTINEL
            || (compressed & (1 << 31)) != 0
        );
        compressed
      }

      fn decompress(ptr: u32) -> *mut std::ffi::c_void {
        assert!(CageBaseGlobal::is_set());
        let base = CageBaseGlobal::get();
        Self::decompress_with_base(ptr, base)
      }

      fn decompress_with_base(ptr: u32, base: usize) -> *mut std::ffi::c_void {
        assert!(CageBaseGlobal::is_set());
        assert!(base == CageBaseGlobal::get());

        let mask = (ptr as i32 as i64) << crate::api_constants::K_POINTER_COMPRESSION_SHIFT;
        (mask & base as i64) as *mut std::ffi::c_void
      }

      pub fn visit_possible_pointers<Callback>(address: *const std::ffi::c_void, callback: Callback)
      where
        Callback: Fn(*mut std::ffi::c_void),
      {
        let base = CageBaseGlobal::get();
        assert!(base != 0);

        let compressed_low = address as usize as u32;
        callback(Self::decompress_with_base(compressed_low, base));

        let compressed_high = (address as usize >> (size_of::<u32>() * 8)) as u32;
        callback(Self::decompress_with_base(compressed_high, base));

        const K_BIT_FOR_INTERMEDIATE_VALUE: usize =
          (size_of::<u32>() * 8) + crate::api_constants::K_POINTER_COMPRESSION_SHIFT;
        const K_SIGN_EXTENSION_MASK: usize = !((1usize << K_BIT_FOR_INTERMEDIATE_VALUE) - 1);
        let intermediate_sign_extended = (address as usize) | K_SIGN_EXTENSION_MASK;
        callback((intermediate_sign_extended & base) as *mut std::ffi::c_void);
      }
    }

    impl PartialEq for CompressedPointer {
      fn eq(&self, other: &Self) -> bool {
        self.value_ == other.value_
      }
    }

    impl Eq for CompressedPointer {}

    impl PartialOrd for CompressedPointer {
      fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value_.partial_cmp(&other.value_)
      }
    }

    impl Ord for CompressedPointer {
      fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value_.cmp(&other.value_)
      }
    }
  }

  #[cfg(not(feature = "CPPGC_POINTER_COMPRESSION"))]
  mod pointer_compression {}

  pub struct RawPointer {
    ptr_: *const std::ffi::c_void,
  }

  impl RawPointer {
    pub type IntegralType = usize;
    pub const K_WRITE_BARRIER_SLOT_TYPE: WriteBarrierSlotType =
      WriteBarrierSlotType::kUncompressed;

    pub fn new() -> Self {
      RawPointer { ptr_: null_mut() }
    }

    pub fn from_ptr_atomic(ptr: *const std::ffi::c_void) -> Self {
      let mut result = Self::new();
      result.store_atomic(ptr);
      result
    }

    pub fn from_ptr(ptr: *const std::ffi::c_void) -> Self {
      RawPointer { ptr_: ptr }
    }

    pub fn load(&self) -> *const std::ffi::c_void {
      self.ptr_
    }

    pub fn load_atomic(&self) -> *const std::ffi::c_void {
      unsafe {
        (&self.ptr_ as *const *const std::ffi::c_void as *const AtomicPtr<std::ffi::c_void>)
          .as_ref()
          .unwrap()
          .load(Ordering::Relaxed)
      }
    }

    pub fn store(&mut self, ptr: *const std::ffi::c_void) {
      self.ptr_ = ptr;
    }

    pub fn store_atomic(&mut self, ptr: *const std::ffi::c_void) {
      unsafe {
        (&mut self.ptr_ as *mut *const std::ffi::c_void as *mut AtomicPtr<std::ffi::c_void>)
          .as_mut()
          .unwrap()
          .store(ptr as *mut std::ffi::c_void, Ordering::Relaxed);
      }
    }

    pub fn clear(&mut self) {
      self.ptr_ = null_mut();
    }

    pub fn is_cleared(&self) -> bool {
      self.ptr_.is_null()
    }

    pub fn is_sentinel(&self) -> bool {
      self.ptr_ as usize == crate::SentinelPointer::kSentinelValue() as usize
    }

    pub fn get_as_integer(&self) -> usize {
      self.ptr_ as usize
    }

    pub fn visit_possible_pointers<Callback>(address: *const std::ffi::c_void, callback: Callback)
    where
      Callback: Fn(*mut std::ffi::c_void),
    {
      callback(address as *mut std::ffi::c_void);
    }
  }

  impl PartialEq for RawPointer {
    fn eq(&self, other: &Self) -> bool {
      self.ptr_ == other.ptr_
    }
  }

  impl Eq for RawPointer {}

  impl PartialOrd for RawPointer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      (self.ptr_ as usize).partial_cmp(&(other.ptr_ as usize))
    }
  }

  impl Ord for RawPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      (self.ptr_ as usize).cmp(&(other.ptr_ as usize))
    }
  }

  #[cfg(feature = "CPPGC_POINTER_COMPRESSION")]
  pub type DefaultMemberStorage = pointer_compression::CompressedPointer;
  #[cfg(not(feature = "CPPGC_POINTER_COMPRESSION"))]
  pub type DefaultMemberStorage = RawPointer;
}
