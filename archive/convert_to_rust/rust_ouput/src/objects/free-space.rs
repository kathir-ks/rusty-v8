// Converted from V8 C++ source files:
// Header: free-space.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod free_space {
    use crate::objects::heap_object::HeapObject;
    use crate::objects::object_macros::*;
    use crate::objects::map::FreeSpace;
    use std::marker::PhantomData;
    use crate::objects::fixed_array_inl::void;
    use crate::objects::fixed_array_inl::code;
    use crate::objects::string::v8;
    use crate::codegen::x64::assembler_x64::base;
    use crate::objects::feedback_vector::Smi;
    use crate::objects::union::UseScratchRegisterScope;
    use crate::objects::feedback_vector::FeedbackSlotKind;
    use crate::objects::js_objects::BodyDescriptor;
    use crate::runtime::runtime_wasm::OpIndex;
    use crate::runtime::runtime_wasm::InstructionOperand;
    use crate::objects::off_heap_hash_table::free;
    use crate::torque::cfg::Block;
    use crate::objects::js_array_buffer_inl::super;
    use crate::objects::property_details::Representation;
    use crate::objects::js_segments::JsSegments;
    use crate::zone::zone_chunk_list::ZoneChunkList;
    use crate::objects::fixed_array::PrimitiveArrayBase;
    use crate::objects::lookup::Lookup;
    use crate::objects::objects::bool;
    use crate::objects::js_display_names_inl::DisplayNamesInternal;
    use crate::objects::managed::CppType;
    use crate::objects::fixed_array_inl::MaybeObject;
    use crate::objects::object::Object;
    use crate::objects::js_function_inl::Code;
    use crate::objects::string::String;
    use crate::objects::js_weak_refs_inl::HeapObject;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use crate::objects::fixed_array::FixedDoubleArray;
    use crate::objects::fixed_array::FixedDoubleArrayShape;
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use crate::base::Address;

    #[repr(C)]
    pub struct FreeSpaceStruct {
        heap_object: HeapObject,
        size: i32,
        next: Address,
    }

    impl Deref for FreeSpace {
        type Target = FreeSpaceStruct;
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }

    impl DerefMut for FreeSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *(self as *mut Self as *mut Self::Target) }
        }
    }

    impl Borrow<HeapObject> for FreeSpace {
        fn borrow(&self) -> &HeapObject {
            &self.heap_object
        }
    }

    impl BorrowMut<HeapObject> for FreeSpace {
        fn borrow_mut(&mut self) -> &mut HeapObject {
            &mut self.heap_object
        }
    }

    #[repr(C)]
    pub struct WritableFreeSpace {
        free_space: FreeSpace,
    }

    impl Deref for WritableFreeSpace {
        type Target = FreeSpace;
        fn deref(&self) -> &Self::Target {
            &self.free_space
        }
    }

    impl DerefMut for WritableFreeSpace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *(self as *mut Self as *mut FreeSpace) }
        }
    }

    impl WritableFreeSpace {
        pub fn size(&self) -> i32 {
            self.free_space.size
        }

        pub fn set_size(&mut self, size: i32) {
            self.free_space.size = size;
        }

        pub fn next(&self) -> Address {
            self.free_space.next
        }

        pub fn set_next(&mut self, next: Address) {
            self.free_space.next = next;
        }
    }

    impl FreeSpace {
        pub fn size(&self) -> i32 {
            self.size
        }

        pub fn set_size(&mut self, size: i32) {
            self.size = size;
        }

        pub fn next(&self) -> Address {
            self.next
        }

        pub fn set_next(&mut self, next: Address) {
            self.next = next;
        }

        pub fn set_size_relaxed(writable_free_space: &mut WritableFreeSpace, size: i32) {
            writable_free_space.set_size(size);
        }

        pub fn get_size(&self) -> i32 {
            self.size
        }

        pub fn set_size(writable_free_space: &WritableFreeSpace, size: i32, _relaxed_store_tag: RelaxedStoreTag) {
            unsafe {
                let ptr = writable_free_space as *const WritableFreeSpace as *mut WritableFreeSpace;
                (*ptr).size = size;
            }
        }

        pub fn size(&self) -> i32 {
            self.size
        }

        pub fn next(&self) -> Tagged<FreeSpace> {
            unsafe { Tagged::<FreeSpace>::from(self.next) }
        }

        pub fn set_next(&self, writable_free_space: &WritableFreeSpace, next: Tagged<FreeSpace>) {
            unsafe {
                let ptr = writable_free_space as *const WritableFreeSpace as *mut WritableFreeSpace;
                (*ptr).next = next.ptr();
            }
        }

        fn is_valid(&self) -> bool {
            true
        }

        pub fn body_descriptor(&self) -> BodyDescriptor {
            BodyDescriptor{}
        }
    }

    pub struct RelaxedStoreTag {}

    #[repr(C)]
    pub struct Tagged<T> {
        ptr: Address,
        phantom: PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub unsafe fn from(ptr: Address) -> Self {
            Tagged { ptr, phantom: PhantomData }
        }
        pub fn ptr(&self) -> Address {
            self.ptr
        }
    }
}
