// Converted from V8 C++ source files:
// Header: pointer-with-payload.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
  use std::{marker::PhantomData, mem, ops::Deref};

  pub struct PointerWithPayloadTraits<T> {
    _phantom: PhantomData<T>,
  }

  impl<T> PointerWithPayloadTraits<T> {
    const fn k_available_bits() -> usize {
      if mem::align_of::<T>() >= 8 {
        3
      } else if mem::align_of::<T>() >= 4 {
        2
      } else {
        1
      }
    }
  }

  impl PointerWithPayloadTraits<std::ffi::c_void> {
    const fn k_available_bits() -> usize {
      PointerWithPayloadTraits::<*mut std::ffi::c_void>::k_available_bits()
    }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct PointerWithPayload<PointerType, PayloadType, const NumPayloadBits: usize> {
    pointer_with_payload_: usize,
    _pointer_type: PhantomData<PointerType>,
    _payload_type: PhantomData<PayloadType>,
  }

  impl<PointerType, PayloadType, const NumPayloadBits: usize>
    PointerWithPayload<PointerType, PayloadType, NumPayloadBits>
  {
    pub fn new() -> Self {
      Self {
        pointer_with_payload_: 0,
        _pointer_type: PhantomData,
        _payload_type: PhantomData,
      }
    }

    pub fn from_pointer(pointer: *mut PointerType) -> Self {
      let mut result = Self::new();
      result.pointer_with_payload_ = pointer as usize;
      assert_eq!(result.get_pointer(), pointer);
      assert_eq!(result.get_payload(), 0 as PayloadType);
      result
    }

    pub fn from_payload(payload: PayloadType) -> Self {
      let mut result = Self::new();
      result.pointer_with_payload_ = payload as usize;
      assert_eq!(result.get_pointer(), std::ptr::null_mut());
      assert_eq!(result.get_payload(), payload);
      result
    }

    pub fn new_with_pointer_and_payload(
      pointer: *mut PointerType,
      payload: PayloadType,
    ) -> Self {
      let mut result = Self::new();
      result.update(pointer, payload);
      result
    }

    #[inline]
    pub fn get_pointer(&self) -> *mut PointerType {
      (self.pointer_with_payload_ & Self::k_pointer_mask()) as *mut PointerType
    }

    #[inline]
    pub fn get_pointer_with_known_payload(&self, payload: PayloadType) -> *mut PointerType
    where
      PayloadType: Eq + std::fmt::Debug + Copy + Into<usize>,
    {
      assert_eq!(self.get_payload(), payload);
      (self.pointer_with_payload_ - (payload.into() as usize)) as *mut PointerType
    }

    #[inline]
    pub fn update(&mut self, new_pointer: *mut PointerType, new_payload: PayloadType)
    where
      PayloadType: Into<usize> + Copy + Eq + std::fmt::Debug,
    {
      self.pointer_with_payload_ =
        new_pointer as usize | (new_payload.into() as usize & Self::k_payload_mask());
      assert_eq!(self.get_payload(), new_payload);
      assert_eq!(self.get_pointer(), new_pointer);
    }

    #[inline]
    pub fn set_pointer(&mut self, new_ptr: *mut PointerType) {
      assert_eq!((new_ptr as usize) & Self::k_payload_mask(), 0);
      self.pointer_with_payload_ =
        new_ptr as usize | (self.pointer_with_payload_ & Self::k_payload_mask());
      assert_eq!(self.get_pointer(), new_ptr);
    }

    #[inline]
    pub fn get_payload(&self) -> PayloadType
    where
      PayloadType: From<usize>,
    {
      (self.pointer_with_payload_ & Self::k_payload_mask()).into()
    }

    #[inline]
    pub fn set_payload(&mut self, new_payload: PayloadType)
    where
      PayloadType: Into<usize> + Copy + Eq + std::fmt::Debug,
    {
      let new_payload_ptr = new_payload.into() as usize;
      assert_eq!(new_payload_ptr & Self::k_payload_mask(), new_payload_ptr);
      self.pointer_with_payload_ =
        (self.pointer_with_payload_ & Self::k_pointer_mask()) | new_payload_ptr;
      assert_eq!(self.get_payload(), new_payload);
    }

    pub fn raw(&self) -> usize {
      self.pointer_with_payload_
    }

    const fn k_available_bits() -> usize {
      PointerWithPayloadTraits::<
        <PointerType as RemoveConst>::Type,
      >::k_available_bits()
    }

    const fn k_payload_mask() -> usize {
      (1_usize << NumPayloadBits) - 1
    }

    const fn k_pointer_mask() -> usize {
      !Self::k_payload_mask()
    }
  }

  trait RemoveConst {
    type Type;
  }

  impl<T> RemoveConst for T {
    type Type = T;
  }

  impl<'a, T> RemoveConst for &'a T {
    type Type = T;
  }
}
