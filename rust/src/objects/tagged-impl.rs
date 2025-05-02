// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: Several V8-specific types and functions are assumed to be defined elsewhere,
// such as `Address`, `Tagged_t`, `Smi`, `HeapObject`, `Object`, `Tagged`,
// `HAS_SMI_TAG`, `HAS_STRONG_HEAP_OBJECT_TAG`, `HAS_WEAK_HEAP_OBJECT_TAG`,
// `kClearedWeakHeapObjectLower32`, `kSystemPointerSize`, `Cast`,
// `HeapObjectReferenceType`, `V8HeapCompressionScheme`, `Isolate`,
// `CompressedObjectSlot`, `CompressedMaybeObjectSlot`, `FullObjectSlot`,
// `FullMaybeObjectSlot`, `FullHeapObjectSlot`.
// These need to be defined or imported for the code to compile.
// Also, error handling and optional features (V8_EXTERNAL_CODE_SPACE, etc.) are simplified.

pub mod tagged_impl {
    use std::{
        fmt::{self, Debug, Display, Formatter},
        marker::PhantomData,
        mem,
    };

    //use crate::base::export_template::EXPORT_TEMPLATE_DECLARE; // Assuming a similar macro exists
    //use crate::common::{checks::DCHECK, globals::V8_COMPRESS_POINTERS};  // Assuming similar macros/constants exist

    // Dummy definitions for V8-specific types and constants.  These need to be replaced
    // with actual implementations from the V8 codebase.

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Address(usize);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Tagged_t(usize);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Smi(usize);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct HeapObject(Address);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Object(Address);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Tagged<T>(Address, PhantomData<T>);

    impl<T> Tagged<T> {
        pub fn new(ptr: Address) -> Self {
            Tagged(ptr, PhantomData)
        }

        pub fn ptr(&self) -> Address {
            self.0
        }
    }

    impl From<Address> for Tagged<Object> {
        fn from(address: Address) -> Self {
            Tagged::new(address)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HeapObjectReferenceType {
        STRONG,
        WEAK,
    }

    const V8_EXTERNAL_CODE_SPACE_BOOL: bool = false;
    const V8_ENABLE_SANDBOX_BOOL: bool = false;
    const V8_COMPRESS_POINTERS: bool = false;

    pub struct Isolate;

    //macro_rules! DCHECK {
    //    ($condition:expr) => {
    //        if !$condition {
    //            panic!("DCHECK failed: {}", stringify!($condition));
    //        }
    //    };
    //}

    const kClearedWeakHeapObjectLower32: u32 = 0xFFFF_FFFF; // Dummy value
    const kSystemPointerSize: usize = mem::size_of::<usize>(); // Assuming usize is the system pointer size.

    // Helper functions simulating the C++ macros
    const fn HAS_SMI_TAG(ptr: Tagged_t) -> bool {
        (ptr.0 & 1) == 1 //Example, needs correct implementation
    }

    const fn HAS_STRONG_HEAP_OBJECT_TAG(ptr: Tagged_t) -> bool {
        (ptr.0 & 1) == 0 //Example, needs correct implementation
    }

    const fn HAS_WEAK_HEAP_OBJECT_TAG(ptr: Tagged_t) -> bool {
        (ptr.0 & 2) == 2 //Example, needs correct implementation
    }

    // Dummy definitions for Cage Base Address, replace with actual logic.
    pub struct V8HeapCompressionScheme;
    impl V8HeapCompressionScheme{
        pub fn GetPtrComprCageBaseAddress(ptr: Tagged_t) -> Tagged_t {
            Tagged_t(ptr.0 & 0xFFFFFF00)
        }
        pub fn base() -> Tagged_t{
            Tagged_t(0)
        }
    }

    // Placeholder for external function
    fn check_object_comparison_allowed(a: Address, b: Address) -> bool {
        true // Dummy implementation
    }

    pub fn cast<T>(obj: Tagged<Object>) -> Tagged<T> {
        Tagged::new(obj.ptr())
    }

    /// A base class for Object (which is either a Smi or a strong reference to a HeapObject)
    /// and Tagged<MaybeObject> (which is either a Smi, a strong reference to a HeapObject, a weak
    /// reference to a HeapObject, or a cleared weak reference. This class provides storage and
    /// one canonical implementation of various predicates that check Smi and heap object tags'
    /// values and also take into account whether the tagged value is expected to be weak reference
    /// to a HeapObject or cleared weak reference.
    pub struct TaggedImpl<const K_REF_TYPE: u32, StorageType> {
        ptr_: StorageType,
        _phantom: PhantomData<()>, // This field ensures that TaggedImpl is not Send or Sync
    }

    impl<const K_REF_TYPE: u32, StorageType> TaggedImpl<K_REF_TYPE, StorageType> {
        const IS_FULL_ADDRESS: bool = mem::size_of::<StorageType>() == mem::size_of::<usize>();
        const IS_WEAK: bool = K_REF_TYPE == HeapObjectReferenceType::WEAK as u32;

        fn static_asserts() {
            assert!(
                std::mem::size_of::<StorageType>() == std::mem::size_of::<Address>()
                    || std::mem::size_of::<StorageType>() == std::mem::size_of::<Tagged_t>(),
                "StorageType must be either Address or Tagged_t"
            );
        }

        /// True for those TaggedImpl instantiations that represent uncompressed
        /// tagged values and false for TaggedImpl instantiations that represent
        /// compressed tagged values.
        pub const K_IS_FULL: bool = Self::IS_FULL_ADDRESS;

        pub const K_CAN_BE_WEAK: bool = Self::IS_WEAK;

        /// Creates a new `TaggedImpl` with a null pointer.
        pub const fn new() -> Self
        where
            StorageType: Copy + Default,
        {
            TaggedImpl {
                ptr_: Default::default(),
                _phantom: PhantomData,
            }
        }

        /// Creates a new `TaggedImpl` with the given pointer.
        pub const fn from_ptr(ptr: StorageType) -> Self {
            TaggedImpl {
                ptr_: ptr,
                _phantom: PhantomData,
            }
        }

        // Make clang on Linux catch what MSVC complains about on Windows:
        // explicit operator bool() const = delete;
        //  In Rust this can be achieved by not implementing the From<T> trait.

        // Don't use this operator for comparing with stale or invalid pointers
        // because CheckObjectComparisonAllowed() might crash when trying to access
        // the object's page header. Use SafeEquals() instead.
        pub const fn eq<const K_OTHER_REF_TYPE: u32, U>(&self, other: &TaggedImpl<K_OTHER_REF_TYPE, U>) -> bool
        where
            U: Copy,
            StorageType: Copy,
        {
            assert!(
                std::mem::size_of::<U>() == std::mem::size_of::<Address>()
                    || std::mem::size_of::<U>() == std::mem::size_of::<Tagged_t>(),
                "U must be either Address or Tagged_t"
            );

            // When comparing two full pointer values ensure that it's allowed.
            if Self::K_IS_FULL && TaggedImpl::<K_OTHER_REF_TYPE, U>::K_IS_FULL {
                if let (Some(a), Some(b)) = (self.ptr_ as usize).checked_into(), (other.ptr_ as usize).checked_into() {
                    //SLOW_DCHECK(CheckObjectComparisonAllowed(Address(a), Address(b)));
                    let _ = check_object_comparison_allowed(Address(a), Address(b)); // Assuming CheckObjectComparisonAllowed exists
                }
            }

            (self.ptr_ as usize) == (other.ptr_ as usize)
        }

        pub const fn ne<const K_OTHER_REF_TYPE: u32, U>(&self, other: &TaggedImpl<K_OTHER_REF_TYPE, U>) -> bool
        where
            U: Copy,
            StorageType: Copy,
        {
            assert!(
                std::mem::size_of::<U>() == std::mem::size_of::<Address>()
                    || std::mem::size_of::<U>() == std::mem::size_of::<Tagged_t>(),
                "U must be either Address or Tagged_t"
            );

            // When comparing two full pointer values ensure that it's allowed.
            if Self::K_IS_FULL && TaggedImpl::<K_OTHER_REF_TYPE, U>::K_IS_FULL {
                if let (Some(a), Some(b)) = (self.ptr_ as usize).checked_into(), (other.ptr_ as usize).checked_into() {
                    //SLOW_DCHECK(CheckObjectComparisonAllowed(Address(a), Address(b)));
                    let _ = check_object_comparison_allowed(Address(a), Address(b)); // Assuming CheckObjectComparisonAllowed exists
                }
            }

            (self.ptr_ as usize) != (other.ptr_ as usize)
        }

        // A variant of operator== which allows comparing objects in different
        // pointer compression cages. In particular, this should be used when
        // comparing objects in trusted- or code space with objects in the main
        // pointer compression cage.
        pub const fn safe_equals<const K_OTHER_REF_TYPE: u32>(
            &self,
            other: &TaggedImpl<K_OTHER_REF_TYPE, StorageType>,
        ) -> bool
        where
            StorageType: Copy,
        {
            assert!(Self::K_IS_FULL, "Safe comparison is allowed only for full tagged values");
            if V8_EXTERNAL_CODE_SPACE_BOOL || V8_ENABLE_SANDBOX_BOOL {
                self.ptr_ as usize == other.ptr_ as usize
            } else {
                self.eq(other)
            }
        }

        // For using in std::set and std::map.
        pub const fn lt(&self, other: &Self) -> bool
        where
            StorageType: Copy,
        {
            // When comparing two full pointer values ensure that it's allowed.
            if Self::K_IS_FULL {
                if let (Some(a), Some(b)) = (self.ptr_ as usize).checked_into(), (other.ptr_ as usize).checked_into() {
                    //SLOW_DCHECK(CheckObjectComparisonAllowed(Address(a), Address(b)));
                    let _ = check_object_comparison_allowed(Address(a), Address(b)); // Assuming CheckObjectComparisonAllowed exists
                }
            }

            (self.ptr_ as usize) < (other.ptr_ as usize)
        }

        pub const fn ptr(&self) -> StorageType
        where
            StorageType: Copy,
        {
            self.ptr_
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject or
        // Smi.
        pub const fn is_object(&self) -> bool
        where
            StorageType: Copy,
        {
            !self.is_weak_or_cleared()
        }

        // Returns true if this tagged value is a Smi.
        pub const fn is_smi(&self) -> bool
        where
            StorageType: Copy,
        {
            HAS_SMI_TAG(Tagged_t(self.ptr_ as usize))
        }

        pub fn to_smi(&self) -> Tagged<Smi>
        where StorageType: Copy {
            if self.is_smi() {
                Tagged::new(Address(self.ptr_ as usize))
            } else {
                panic!("Not a Smi");
            }
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject.
        pub const fn is_heap_object(&self) -> bool
        where
            StorageType: Copy,
        {
            self.is_strong()
        }

        // Returns true if this tagged value is a cleared weak reference.
        pub const fn is_cleared(&self) -> bool
        where
            StorageType: Copy,
        {
            Self::K_CAN_BE_WEAK && (self.ptr_ as u32 == kClearedWeakHeapObjectLower32)
        }

        // Returns true if this tagged value is a strong or weak pointer to a
        // HeapObject.
        pub const fn is_strong_or_weak(&self) -> bool
        where
            StorageType: Copy,
        {
            !self.is_smi() && !self.is_cleared()
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject.
        pub const fn is_strong(&self) -> bool
        where
            StorageType: Copy,
        {
            if Self::K_CAN_BE_WEAK {
                HAS_STRONG_HEAP_OBJECT_TAG(Tagged_t(self.ptr_ as usize))
            } else {
                !self.is_smi()
            }
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject, or a
        // Smi.
        pub const fn is_strong_or_smi(&self) -> bool
        where
            StorageType: Copy,
        {
            !Self::K_CAN_BE_WEAK || !HAS_WEAK_HEAP_OBJECT_TAG(Tagged_t(self.ptr_ as usize))
        }

        // Returns true if this tagged value is a weak pointer to a HeapObject.
        pub const fn is_weak(&self) -> bool
        where
            StorageType: Copy,
        {
            self.is_weak_or_cleared() && !self.is_cleared()
        }

        // Returns true if this tagged value is a weak pointer to a HeapObject or
        // cleared weak reference.
        pub const fn is_weak_or_cleared(&self) -> bool
        where
            StorageType: Copy,
        {
            Self::K_CAN_BE_WEAK && HAS_WEAK_HEAP_OBJECT_TAG(Tagged_t(self.ptr_ as usize))
        }

        #[cfg(V8_COMPRESS_POINTERS)]
        // Returns true if this tagged value is a pointer to an object in the given
        // cage base.
        pub const fn is_in_main_cage_base(&self) -> bool
        where
            StorageType: Copy,
        {
            if self.is_smi() {
                false // Added to match the intended functionality in the original code.
            } else {
                V8HeapCompressionScheme::GetPtrComprCageBaseAddress(Tagged_t(self.ptr_ as usize))
                    == V8HeapCompressionScheme::GetPtrComprCageBaseAddress(V8HeapCompressionScheme::base())
            }
        }

        // The following set of methods get HeapObject out of the tagged value
        // which may involve decompression in which case the isolate root is required.
        // If the pointer compression is not enabled then the variants with
        // isolate parameter will be exactly the same as the ones witout isolate
        // parameter.

        // If this tagged value is a strong pointer to a HeapObject, returns true and
        // sets *result. Otherwise returns false.
        pub fn get_heap_object_if_strong(&self, result: &mut Tagged<HeapObject>) -> bool
        where StorageType: Copy
        {
            if self.is_strong() {
                *result = Tagged::new(Address(self.ptr_ as usize));
                true
            } else {
                false
            }
        }

        pub fn get_heap_object_if_strong_with_isolate(
            &self,
            _isolate: &Isolate,
            result: &mut Tagged<HeapObject>,
        ) -> bool
        where StorageType: Copy {
            self.get_heap_object_if_strong(result)
        }

        // DCHECKs that this tagged value is a strong pointer to a HeapObject and
        // returns the HeapObject.
        pub fn get_heap_object_assume_strong(&self) -> Tagged<HeapObject>
        where StorageType: Copy {
            assert!(self.is_strong());
            Tagged::new(Address(self.ptr_ as usize))
        }

        pub fn get_heap_object_assume_strong_with_isolate(&self, _isolate: &Isolate) -> Tagged<HeapObject>
        where StorageType: Copy {
            self.get_heap_object_assume_strong()
        }

        // If this tagged value is a weak pointer to a HeapObject, returns true and
        // sets *result. Otherwise returns false.
        pub fn get_heap_object_if_weak(&self, result: &mut Tagged<HeapObject>) -> bool
        where StorageType: Copy {
            if self.is_weak() {
                *result = Tagged::new(Address(self.ptr_ as usize));
                true
            } else {
                false
            }
        }

        pub fn get_heap_object_if_weak_with_isolate(
            &self,
            _isolate: &Isolate,
            result: &mut Tagged<HeapObject>,
        ) -> bool
        where StorageType: Copy {
            self.get_heap_object_if_weak(result)
        }

        // DCHECKs that this tagged value is a weak pointer to a HeapObject and
        // returns the HeapObject.
        pub fn get_heap_object_assume_weak(&self) -> Tagged<HeapObject>
        where StorageType: Copy {
            assert!(self.is_weak());
            Tagged::new(Address(self.ptr_ as usize))
        }

        pub fn get_heap_object_assume_weak_with_isolate(&self, _isolate: &Isolate) -> Tagged<HeapObject>
        where StorageType: Copy {
            self.get_heap_object_assume_weak()
        }

        // If this tagged value is a strong or weak pointer to a HeapObject, returns
        // true and sets *result. Otherwise returns false.
        pub fn get_heap_object(&self, result: &mut Tagged<HeapObject>) -> bool
        where StorageType: Copy {
            if self.is_strong_or_weak() {
                *result = Tagged::new(Address(self.ptr_ as usize));
                true
            } else {
                false
            }
        }

        pub fn get_heap_object_with_isolate(
            &self,
            _isolate: &Isolate,
            result: &mut Tagged<HeapObject>,
        ) -> bool
        where StorageType: Copy {
            self.get_heap_object(result)
        }

        pub fn get_heap_object_with_reference_type(
            &self,
            result: &mut Tagged<HeapObject>,
            reference_type: &mut HeapObjectReferenceType,
        ) -> bool
        where StorageType: Copy {
            if self.is_strong() {
                *result = Tagged::new(Address(self.ptr_ as usize));
                *reference_type = HeapObjectReferenceType::STRONG;
                true
            } else if self.is_weak() {
                *result = Tagged::new(Address(self.ptr_ as usize));
                *reference_type = HeapObjectReferenceType::WEAK;
                true
            } else {
                false
            }
        }

        pub fn get_heap_object_with_isolate_and_reference_type(
            &self,
            _isolate: &Isolate,
            result: &mut Tagged<HeapObject>,
            reference_type: &mut HeapObjectReferenceType,
        ) -> bool
        where StorageType: Copy {
            self.get_heap_object_with_reference_type(result, reference_type)
        }

        // DCHECKs that this tagged value is a strong or a weak pointer to a
        // HeapObject and returns the HeapObject.
        pub fn get_heap_object(&self) -> Tagged<HeapObject>
        where StorageType: Copy {
            assert!(self.is_strong_or_weak());
            Tagged::new(Address(self.ptr_ as usize))
        }

        pub fn get_heap_object_with_isolate(&self, _isolate: &Isolate) -> Tagged<HeapObject>
        where StorageType: Copy {
            self.get_heap_object()
        }

        // DCHECKs that this tagged value is a strong or a weak pointer to a
        // HeapObject or a Smi and returns the HeapObject or Smi.
        pub fn get_heap_object_or_smi(&self) -> Tagged<Object>
        where StorageType: Copy {
            assert!(self.is_strong_or_smi());
            Tagged::new(Address(self.ptr_ as usize))
        }

        pub fn get_heap_object_or_smi_with_isolate(&self, _isolate: &Isolate) -> Tagged<Object>
        where StorageType: Copy {
            self.get_heap_object_or_smi()
        }

        // Cast operation is available only for full non-weak tagged values.
        pub fn cast_to<T>(&self) -> Tagged<T>
        where StorageType: Copy{
            assert!(Self::K_IS_FULL);
            assert!(!HAS_WEAK_HEAP_OBJECT_TAG(Tagged_t(self.ptr_ as usize)));
            cast::<T>(Tagged::new(Address(self.ptr_ as usize)))
        }

        pub fn ptr_location(&mut self) -> *mut StorageType {
            &mut self.ptr_
        }

        pub fn ptr_location_const(&self) -> *const StorageType {
            &self.ptr_
        }
    }

    impl<const K_REF_TYPE: u32, StorageType> PartialEq for TaggedImpl<K_REF_TYPE, StorageType>
    where
        StorageType: Copy,
    {
        fn eq(&self, other: &Self) -> bool {
            self.ptr_ as usize == other.ptr_ as usize
        }
    }

    impl<const K_REF_TYPE: u32, StorageType> Eq for TaggedImpl<K_REF_TYPE, StorageType> where
        StorageType: Copy {}

    impl<const K_REF_TYPE: u32, StorageType> PartialOrd for TaggedImpl<K_REF_TYPE, StorageType>
    where
        StorageType: Copy,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.ptr_ as usize).partial_cmp(&Some(other.ptr_ as usize))
        }
    }

    impl<const K_REF_TYPE: u32, StorageType> Ord for TaggedImpl<K_REF_TYPE, StorageType>
    where
        StorageType: Copy,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (self.ptr_ as usize).cmp(&(other.ptr_ as usize))
        }
    }

    impl<const K_REF_TYPE: u32, StorageType> Clone for TaggedImpl<K_REF_TYPE, StorageType>
    where
        StorageType: Copy,
    {
        fn clone(&self) -> Self {
            TaggedImpl {
                ptr_: self.ptr_,
                _phantom: PhantomData,
            }
        }
    }

    impl<const K_REF_TYPE: u32, StorageType> Copy for TaggedImpl<K_REF_TYPE, StorageType> where
        StorageType: Copy {}

    // Prints this object without details.
    pub fn short_print<const K_REF_TYPE: u32, StorageType>(ptr: TaggedImpl<K_REF_TYPE, StorageType>, out: &mut dyn std::io::Write)
    where StorageType: Copy,
          fmt::Debug: for<'a> fmt::Debug,
    {
        write!(out, "{:?}", ptr.ptr()).unwrap();
    }

    // Prints this object without details to a message accumulator.
    pub fn short_print_string_stream<const K_REF_TYPE: u32, StorageType>(
        ptr: TaggedImpl<K_REF_TYPE, StorageType>,
        accumulator: &mut String,
    ) where StorageType: Copy {
        accumulator.push_str(&format!("{:?}", ptr.ptr()));
    }

    // Prints this object without details to a message accumulator.
    pub fn short_print_ostream<const K_REF_TYPE: u32, StorageType>(
        ptr: TaggedImpl<K_REF_TYPE, StorageType>,
        os: &mut std::io::Write,
    ) where StorageType: Copy,
            fmt::Debug: for<'a> fmt::Debug,
    {
        write!(os, "{:?}", ptr.ptr()).unwrap();
    }

    #[cfg(OBJECT_PRINT)]
    pub fn print<const K_REF_TYPE: u32, StorageType>(ptr: TaggedImpl<K_REF_TYPE, StorageType>) {
        //OBJECT_PRINT macro missing.
        short_print(ptr, &mut std::io::stdout());
    }

    #[cfg(not(OBJECT_PRINT))]
    pub fn print<const K_REF_TYPE: u32, StorageType>(ptr: TaggedImpl<K_REF_TYPE, StorageType>, out: &mut dyn std::io::Write)
    where StorageType: Copy,
          fmt::Debug: for<'a> fmt::Debug,
    {
        short_print(ptr, out);
    }

    #[cfg(OBJECT_PRINT)]
    pub fn print_ostream<const K_REF_TYPE: u32, StorageType>(ptr: TaggedImpl<K_REF_TYPE, StorageType>, os: &mut std::io::Write)
    where StorageType: Copy,
          fmt::Debug: for<'a> fmt::Debug,
    {
        //OBJECT_PRINT macro missing.
        short_print_ostream(ptr, os);
    }

    #[cfg(not(OBJECT_PRINT))]
    pub fn print_ostream<const K_REF_TYPE: u32, StorageType>(ptr: TaggedImpl<K_REF_TYPE, StorageType>, os: &mut dyn std::io::Write)
    where StorageType: Copy,
          fmt::Debug: for<'a> fmt::Debug,
    {
        short_print_ostream(ptr, os);
    }

    struct StringStream;
    impl StringStream {
        fn new() -> Self {StringStream{}}
        fn push_str(&mut self, _s: &str){}
    }

    impl Debug for StringStream {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "StringStream")
        }
    }

    trait CheckedInto<T> {
        fn checked_into(self) -> Option<T>;
    }

    impl CheckedInto<u32> for usize {
        fn checked_into(self) -> Option<u32> {
            if self <= u32::MAX as usize {
                Some(self as u32)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tagged_impl() {
            // Example usage
            let tagged_address: TaggedImpl<0, Address> = TaggedImpl::from_ptr(Address(0x12345678));
            let mut heap_object = Tagged::<HeapObject>::new(Address(0));
            let _ = tagged_address.get_heap_object_if_strong(&mut heap_object);
            let _ = tagged_address.get_heap_object_if_weak(&mut heap_object);

            assert_eq!(
                tagged_address,
                TaggedImpl::from_ptr(Address(0x12345678))
            );
            assert_ne!(tagged_address, TaggedImpl::from_ptr(Address(0x87654321)));
        }
    }
}