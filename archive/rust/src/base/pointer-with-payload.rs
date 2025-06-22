// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        marker::PhantomData,
        mem::{align_of, transmute},
        ops::Deref,
    };

    trait PointerWithPayloadTraitsHelper<T> {
        const AVAILABLE_BITS: usize;
    }

    macro_rules! impl_pointer_with_payload_traits {
        ($type:ty, $bits:expr) => {
            impl<T> PointerWithPayloadTraitsHelper<$type> for T {
                const AVAILABLE_BITS: usize = $bits;
            }
        };
    }

    impl_pointer_with_payload_traits!(u64, 3);
    impl_pointer_with_payload_traits!(u32, 2);
    impl_pointer_with_payload_traits!(u16, 1);
    impl_pointer_with_payload_traits!(u8, 1);

    trait PointerWithPayloadTraits<PointerType> {
        const AVAILABLE_BITS: usize;
    }

    impl<PointerType> PointerWithPayloadTraits<PointerType> for () {
        default const AVAILABLE_BITS: usize = {
            if align_of::<PointerType>() >= 8 {
                3
            } else if align_of::<PointerType>() >= 4 {
                2
            } else {
                1
            }
        };
    }

    impl PointerWithPayloadTraits<*const std::ffi::c_void> for () {
        const AVAILABLE_BITS: usize = <() as PointerWithPayloadTraits<*const std::ffi::c_void>>.AVAILABLE_BITS;
    }

    /// `PointerWithPayload` combines a `PointerType` and a small `PayloadType` into
    /// one. The bits of the storage type get packed into the lower bits of the
    /// pointer that are free due to alignment. The user needs to specify how many
    /// bits are needed to store the `PayloadType`, allowing Types that by default are
    /// larger to be stored.
    ///
    /// Example:
    /// ```
    /// use v8::base::PointerWithPayload;
    ///
    /// let mut data: i32 = 10;
    /// let data_ptr: *mut i32 = &mut data;
    ///
    /// let data_and_flag: PointerWithPayload<i32, bool, 1> =
    ///     PointerWithPayload::new(data_ptr, true);
    ///
    /// assert_eq!(unsafe { *data_and_flag.get_pointer() }, 10);
    /// assert_eq!(data_and_flag.get_payload(), true);
    /// ```
    #[derive(Copy, Clone)]
    pub struct PointerWithPayload<PointerType, PayloadType, const NumPayloadBits: usize> {
        pointer_with_payload_: usize,
        _pointer_type: PhantomData<PointerType>,
        _payload_type: PhantomData<PayloadType>,
    }

    impl<PointerType, PayloadType, const NumPayloadBits: usize>
        PointerWithPayload<PointerType, PayloadType, NumPayloadBits>
    {
        const AVAILABLE_BITS: usize = <() as PointerWithPayloadTraits<PointerType>>::AVAILABLE_BITS;
        const PAYLOAD_MASK: usize = (1 << NumPayloadBits) - 1;
        const POINTER_MASK: usize = !Self::PAYLOAD_MASK;

        /// Creates a `PointerWithPayload` with default values.
        pub fn new_empty() -> Self {
            Self {
                pointer_with_payload_: 0,
                _pointer_type: PhantomData,
                _payload_type: PhantomData,
            }
        }

        /// Creates a `PointerWithPayload` from a pointer.
        pub fn new_from_pointer(pointer: *mut PointerType) -> Self
        where
            PayloadType: From<u8>,
        {
            let pointer_with_payload_ = pointer as usize;
            let result = Self {
                pointer_with_payload_,
                _pointer_type: PhantomData,
                _payload_type: PhantomData,
            };
            debug_assert_eq!(result.get_pointer(), pointer);
            debug_assert_eq!(result.get_payload(), PayloadType::from(0));
            result
        }

        /// Creates a `PointerWithPayload` from a payload.
        pub fn new_from_payload(payload: PayloadType) -> Self
        where
            PointerType: From<usize>,
            PayloadType: Into<usize> + Copy
        {
            let pointer_with_payload_ = payload.into();
            let result = Self {
                pointer_with_payload_,
                _pointer_type: PhantomData,
                _payload_type: PhantomData,
            };
            debug_assert_eq!(result.get_pointer(), PointerType::from(0 as usize));
            debug_assert_eq!(result.get_payload(), payload);
            result
        }

        /// Creates a `PointerWithPayload` from a pointer and a payload.
        pub fn new(pointer: *mut PointerType, payload: PayloadType) -> Self
        where PayloadType: Into<usize> + Copy
        {
            let mut result = Self::new_empty();
            result.update(pointer, payload);
            result
        }

        /// Returns the pointer.
        pub fn get_pointer(&self) -> *mut PointerType {
            (self.pointer_with_payload_ & Self::POINTER_MASK) as *mut PointerType
        }

        /// Returns an optimized version of `get_pointer` for when we know the payload value.
        pub fn get_pointer_with_known_payload(&self, payload: PayloadType) -> *mut PointerType
        where PayloadType: Into<usize> + Copy
        {
            debug_assert_eq!(self.get_payload(), payload);
            (self.pointer_with_payload_ - payload.into()) as *mut PointerType
        }

        /// Updates the pointer and payload.
        pub fn update(&mut self, new_pointer: *mut PointerType, new_payload: PayloadType)
        where PayloadType: Into<usize> + Copy
        {
            self.pointer_with_payload_ =
                new_pointer as usize | new_payload.into();
            debug_assert_eq!(self.get_payload(), new_payload);
            debug_assert_eq!(self.get_pointer(), new_pointer);
        }

        /// Sets the pointer.
        pub fn set_pointer(&mut self, new_ptr: *mut PointerType) {
            debug_assert_eq!(new_ptr as usize & Self::PAYLOAD_MASK, 0);
            self.pointer_with_payload_ =
                new_ptr as usize | (self.pointer_with_payload_ & Self::PAYLOAD_MASK);
            debug_assert_eq!(self.get_pointer(), new_ptr);
        }

        /// Returns the payload.
        pub fn get_payload(&self) -> PayloadType
        where PayloadType: From<usize>
        {
            (self.pointer_with_payload_ & Self::PAYLOAD_MASK).into()
        }

        /// Sets the payload.
        pub fn set_payload(&mut self, new_payload: PayloadType)
        where PayloadType: Into<usize> + Copy
        {
            let new_payload_ptr = new_payload.into();
            debug_assert_eq!(new_payload_ptr & Self::PAYLOAD_MASK, new_payload_ptr);
            self.pointer_with_payload_ =
                (self.pointer_with_payload_ & Self::POINTER_MASK) | new_payload_ptr;
            debug_assert_eq!(self.get_payload(), new_payload.into());
        }

        /// Returns the raw `usize` value.
        pub fn raw(&self) -> usize {
            self.pointer_with_payload_
        }
    }

    impl<PointerType, PayloadType, const NumPayloadBits: usize> PartialEq
        for PointerWithPayload<PointerType, PayloadType, NumPayloadBits>
    {
        fn eq(&self, other: &Self) -> bool {
            self.raw() == other.raw()
        }
    }

    impl<PointerType, PayloadType, const NumPayloadBits: usize>
        PointerWithPayload<PointerType, PayloadType, NumPayloadBits>
    {
        const _ASSERT: () = assert!(
            Self::AVAILABLE_BITS >= NumPayloadBits,
            "Ptr does not have sufficient alignment for the selected amount of storage bits. Override PointerWithPayloadTraits to guarantee available bits manually."
        );
    }
}