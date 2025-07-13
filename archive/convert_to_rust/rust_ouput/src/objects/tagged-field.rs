// Converted from V8 C++ source files:
// Header: tagged-field.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod atomicops {
        pub type Atomic32 = i32;
    }
    pub mod macros {
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE")
            };
        }
    }
    pub mod template_meta_programming {
        pub mod functional {
            pub struct lazy_true<T> {
                pub value: bool,
                pub _phantom: std::marker::PhantomData<T>,
            }
            impl<T> lazy_true<T> {
                pub const value: bool = true;
            }
        }
    }
}
pub mod common {
    pub mod globals {
        pub type Address = usize;
        pub type size_t = usize;
        pub type intptr_t = isize;
        pub type uintptr_t = usize;
        pub type bool = i32;
        pub type MaybeObject = usize;
    }
    pub mod ptr_compr {
        pub struct PtrComprCageBase {}
    }
}
pub mod objects {
    pub mod tagged_value {
        use crate::common::globals::Address;
        pub type Tagged_t = Address;
    }
}
pub mod internal {
    use crate::objects::tagged_value::Tagged_t;
    pub struct HeapObject {}
    pub struct Object {}
    pub struct Smi {}
    pub struct MapWord {}
    pub struct Isolate {}
    pub struct HeapObjectLayout {}
    pub struct FixedArray {}
    pub struct JSFunction {}
    pub struct Script {}
    pub struct DirectHandle<T> {}
    pub struct IndirectHandle<T> {}
    pub struct Root {}
    pub struct AcquireLoadTag {}
    pub struct Code {}

    pub enum HeapObjectReferenceType {
        WEAK,
        STRONG,
    }

    pub enum WriteBarrierMode {
        UPDATE_WRITE_BARRIER,
    }

    pub trait TaggedTrait {}
    impl TaggedTrait for HeapObject {}
    impl TaggedTrait for Object {}
    impl TaggedTrait for Smi {}
    pub struct Tagged<T: TaggedTrait> {
        pub _phantom: std::marker::PhantomData<T>,
        pub value: usize,
    }

    impl<T: TaggedTrait> Copy for Tagged<T> {}
    impl<T: TaggedTrait> Clone for Tagged<T> {
        fn clone(&self) -> Self {
            Self {
                _phantom: self._phantom.clone(),
                value: self.value.clone(),
            }
        }
    }

    impl<T: TaggedTrait> Tagged<T> {
        pub fn new(value: usize) -> Self {
            Tagged {
                _phantom: std::marker::PhantomData,
                value,
            }
        }
    }

    pub struct AllStatic {}

    pub struct V8HeapCompressionScheme {}
    pub struct TaggedImpl<const REF_TYPE: HeapObjectReferenceType, T> {}
    pub fn is_taggable_v<T>() -> bool {
        true
    }
    pub fn is_subtype<T, U>() -> bool {
        true
    }
}

pub mod objects {
    pub mod tagged_field {
        use crate::base;
        use crate::common::globals::Address;
        use crate::common::ptr_compr::PtrComprCageBase;
        use crate::internal;
        use crate::objects::tagged_value::Tagged_t;
        use std::{cell::RefCell, rc::Rc};

        pub struct TaggedMember<T, CompressionScheme> {
            _phantom: std::marker::PhantomData<(T, CompressionScheme)>,
            pub value: Tagged_t,
        }
        impl<T, CompressionScheme> Copy for TaggedMember<T, CompressionScheme> {}
        impl<T, CompressionScheme> Clone for TaggedMember<T, CompressionScheme> {
            fn clone(&self) -> Self {
                Self {
                    _phantom: self._phantom.clone(),
                    value: self.value.clone(),
                }
            }
        }

        impl<T, CompressionScheme> TaggedMember<T, CompressionScheme> {
            pub const fn new(value: Tagged_t) -> Self {
                TaggedMember {
                    _phantom: std::marker::PhantomData,
                    value,
                }
            }
        }

        impl<T, CompressionScheme> TaggedMember<T, CompressionScheme> {
            pub fn load(&self) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                internal::Tagged::<T>::new(self.value as usize)
            }

            pub fn store(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
                // Simulate write barrier if needed
            }

            pub fn store_no_write_barrier(&mut self, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn Relaxed_Load(&self) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                internal::Tagged::<T>::new(self.value as usize)
            }

             pub fn Relaxed_Store(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn Relaxed_Store_no_write_barrier(&mut self, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn Acquire_Load(&self) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                internal::Tagged::<T>::new(self.value as usize)
            }

            pub fn Release_Store(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn Release_Store_no_write_barrier(&mut self, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn SeqCst_Load(&self) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                internal::Tagged::<T>::new(self.value as usize)
            }

            pub fn SeqCst_Store(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn SeqCst_Store_no_write_barrier(&mut self, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                self.value = value.value as Tagged_t;
            }

            pub fn SeqCst_Swap(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let old_value = internal::Tagged::<T>::new(self.value as usize);
                 self.value = value.value as Tagged_t;
                 old_value
            }

            pub fn SeqCst_CompareAndSwap(&mut self, host: *mut internal::HeapObjectLayout, expected_value: internal::Tagged<T>, value: internal::Tagged<T>, mode: internal::WriteBarrierMode) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let old_value = internal::Tagged::<T>::new(self.value as usize);
                if self.value == expected_value.value as Tagged_t {
                    self.value = value.value as Tagged_t;
                }
                old_value
            }

            fn WriteBarrier(&mut self, host: *mut internal::HeapObjectLayout, value: internal::Tagged<T>, mode: internal::WriteBarrierMode)
                where T: internal::TaggedTrait {
                // Implement write barrier logic here based on the mode.
            }

            fn tagged_to_full(tagged_value: Tagged_t) -> Address {
                tagged_value as Address
            }

            fn full_to_tagged(value: Address) -> Tagged_t {
                value as Tagged_t
            }
        }
        static_assert!(
            std::mem::align_of::<TaggedMember<internal::Object, internal::V8HeapCompressionScheme>>()
                == std::mem::align_of::<Tagged_t>()
        );
        static_assert!(
            std::mem::size_of::<TaggedMember<internal::Object, internal::V8HeapCompressionScheme>>()
                == std::mem::size_of::<Tagged_t>()
        );

        pub struct UnalignedValueMember<T> {
            storage_: std::mem::MaybeUninit<[u8; std::mem::size_of::<T>()]>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> UnalignedValueMember<T> {
            pub fn new() -> Self {
                UnalignedValueMember {
                    storage_: std::mem::MaybeUninit::uninit(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn value(&self) -> T
                where T: Copy {
                unsafe {
                   let ptr = self.storage_.as_ptr() as *const T;
                    ptr.read_unaligned()
                }
            }

            pub fn set_value(&mut self, value: T)
                where T: Copy {
                unsafe {
                    let ptr = self.storage_.as_mut_ptr() as *mut T;
                    ptr.write_unaligned(value);
                }
            }
        }

        pub struct UnalignedDoubleMember {
            base: UnalignedValueMember<f64>,
        }

        impl UnalignedDoubleMember {
            pub fn new() -> Self {
                UnalignedDoubleMember {
                    base: UnalignedValueMember::new(),
                }
            }
            pub fn value_as_bits(&self) -> u64 {
                unsafe {
                   let ptr = self.base.storage_.as_ptr() as *const u64;
                   ptr.read_unaligned()
                }
            }
            pub fn set_value_as_bits(&mut self, value: u64) {
                unsafe {
                    let ptr = self.base.storage_.as_mut_ptr() as *mut u64;
                    ptr.write_unaligned(value);
                }
            }
        }

        static_assert!(
            std::mem::align_of::<UnalignedDoubleMember>() == std::mem::align_of::<Tagged_t>()
        );
        static_assert!(std::mem::size_of::<UnalignedDoubleMember>() == std::mem::size_of::<f64>());

        #[macro_export]
        macro_rules! FLEXIBLE_ARRAY_MEMBER {
            ($Type:ty, $name:ident) => {
                #[allow(dead_code)]
                struct FlexibleArrayMemberData<$Type> {
                    data: [$Type; 0],
                }
                impl<T> FlexibleArrayMemberData<T> {
                    #[inline]
                    fn new() -> FlexibleArrayMemberData<T> {
                        FlexibleArrayMemberData { data: [] }
                    }

                    #[inline]
                    fn as_slice(&self) -> &[T] {
                        &self.data
                    }

                    #[inline]
                    fn as_mut_slice(&mut self) -> &mut [T] {
                        &mut self.data
                    }

                    #[inline]
                    const fn offset_of_data_start<Class>() -> usize {
                        std::mem::offset_of!(Class, flexible_array_member_data_)
                    }
                }
                #[allow(dead_code)]
                flexible_array_member_data_: FlexibleArrayMemberData<$Type>,
                impl Self {
                    #[inline]
                    fn $name(&self) -> &[$Type] {
                        self.flexible_array_member_data_.as_slice()
                    }
                    #[inline]
                    fn $name(&mut self) -> &mut [$Type] {
                        self.flexible_array_member_data_.as_mut_slice()
                    }

                }
                type FlexibleDataType = $Type;
            };
        }

        #[macro_export]
        macro_rules! OFFSET_OF_DATA_START {
            ($Type:ty) => {
                 {
                    let temp :  $Type = unsafe {std::mem::zeroed()};
                    let offset = FlexibleArrayMemberData::< FlexibleDataType >::offset_of_data_start::<$Type>();
                    offset
                }

            };
        }

        pub struct TaggedField<T, const K_FIELD_OFFSET: usize = 0, CompressionScheme = internal::V8HeapCompressionScheme> {
            _phantom: std::marker::PhantomData<(T, CompressionScheme)>,
        }

        impl<T, const K_FIELD_OFFSET: usize, CompressionScheme> TaggedField<T, K_FIELD_OFFSET, CompressionScheme> {
            pub fn address(host: internal::Tagged<internal::HeapObject>, offset: i32) -> Address {
                host.value + K_FIELD_OFFSET + offset as usize
            }

            pub fn load(host: internal::Tagged<internal::HeapObject>, offset: i32) ->  internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn load_with_cage(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn store(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

             pub fn store_with_offset(host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn Relaxed_Load(host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

             pub fn Relaxed_Load_with_cage(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn Relaxed_Store(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

             pub fn Relaxed_Store_with_offset(host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn Acquire_Load(host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn Acquire_Load_No_Unpack(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn Acquire_Load_with_cage(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn SeqCst_Load(host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn SeqCst_Load_with_cage(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                 let address = Self::address(host, offset);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

            pub fn Release_Store(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn Release_Store_with_offset(host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn SeqCst_Store(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn SeqCst_Store_with_offset(host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn SeqCst_Swap(host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    let old_value = (address as *mut usize).replace(value.value);
                    internal::Tagged::<T>::new(old_value)
                }
            }

            pub fn SeqCst_Swap_with_cage(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>, offset: i32, value: internal::Tagged<T>) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    let old_value = (address as *mut usize).replace(value.value);
                    internal::Tagged::<T>::new(old_value)
                }
            }

             pub fn Release_CompareAndSwap(host: internal::Tagged<internal::HeapObject>, old: internal::Tagged<T>, value: internal::Tagged<T>) -> Tagged_t
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    let address_ptr = address as *mut usize;
                    let old_value = old.value;
                    let new_value = value.value;
                    let result = base::atomicops::Atomic32::compare_exchange_weak(address_ptr as *mut i32, old_value as i32, new_value as i32, std::sync::atomic::Ordering::Release, std::sync::atomic::Ordering::Relaxed);
                    result.unwrap() as Tagged_t
                }
            }

            pub fn Relaxed_CompareAndSwap(host: internal::Tagged<internal::HeapObject>, old: internal::Tagged<T>, value: internal::Tagged<T>) -> Tagged_t
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    let address_ptr = address as *mut usize;
                    let old_value = old.value;
                    let new_value = value.value;
                    let result = base::atomicops::Atomic32::compare_exchange_weak(address_ptr as *mut i32, old_value as i32, new_value as i32, std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed);
                    result.unwrap() as Tagged_t
                }
            }
            pub fn SeqCst_CompareAndSwap(host: internal::Tagged<internal::HeapObject>, offset: i32, old: internal::Tagged<T>, value: internal::Tagged<T>) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, offset);
                unsafe {
                    let address_ptr = address as *mut usize;
                    let old_value = old.value;
                    let new_value = value.value;
                   let result = base::atomicops::Atomic32::compare_exchange_weak(address_ptr as *mut i32, old_value as i32, new_value as i32, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::Relaxed);
                   match result{
                        Ok(old_value) => {internal::Tagged::<T>::new(old_value as usize)}
                        Err(_) => {internal::Tagged::<T>::new(old.value as usize)}
                   }
                }
            }

             pub fn Relaxed_Load_Map_Word(cage_base: PtrComprCageBase, host: internal::Tagged<internal::HeapObject>) -> internal::Tagged<T>
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                  let value = (address as *const usize).read_unaligned();
                  internal::Tagged::<T>::new(value)
                }
            }

             pub fn Relaxed_Store_Map_Word(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            pub fn Release_Store_Map_Word(host: internal::Tagged<internal::HeapObject>, value: internal::Tagged<T>)
                where T: internal::TaggedTrait {
                let address = Self::address(host, 0);
                unsafe {
                    (address as *mut usize).write_unaligned(value.value);
                }
            }

            fn location(host: internal::Tagged<internal::HeapObject>, offset: i32) -> *mut Tagged_t {
                let address = Self::address(host, offset);
                address as *mut Tagged_t
            }

            fn tagged_to_full<TOnHeapAddress>(on_heap_addr: TOnHeapAddress, tagged_value: Tagged_t) -> Address {
                tagged_value as Address
            }

            fn full_to_tagged(value: Address) -> Tagged_t {
                value as Tagged_t
            }
        }

        impl<T> TaggedField<internal::Tagged<T>> { }

        impl<T, const K_FIELD_OFFSET: usize> TaggedField<internal::Tagged<T>, K_FIELD_OFFSET> { }

        impl<T, const K_FIELD_OFFSET: usize, CompressionScheme> TaggedField<internal::Tagged<T>, K_FIELD_OFFSET, CompressionScheme> { }
    }
}
