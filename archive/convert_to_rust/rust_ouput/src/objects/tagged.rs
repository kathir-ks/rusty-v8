// Converted from V8 C++ source files:
// Header: tagged.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tagged_mod {
    use std::marker::PhantomData;

    use crate::common::globals::kHeapObjectTag;
    use crate::objects::slots_inl::HeapObjectReferenceType;
    use crate::objects::tagged_impl::TaggedImpl;
    use crate::objects::union::Union;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::object::Object;
    use crate::objects::smi::Smi;
    use crate::objects::tagged_index::TaggedIndex;
    use crate::objects::field_type::FieldType;
    use crate::objects::heap_object_layout::HeapObjectLayout;
    use crate::objects::trusted_object::TrustedObject;
    use crate::objects::trusted_object_layout::TrustedObjectLayout;
    use crate::objects::fixed_array_base::FixedArrayBase;
    use crate::objects::fixed_array::FixedArray;
    use crate::objects::fixed_double_array::FixedDoubleArray;
    use crate::objects::byte_array::ByteArray;
    use crate::objects::name_dictionary::NameDictionary;
    use crate::objects::number_dictionary::NumberDictionary;
    use crate::objects::ordered_hash_map::OrderedHashMap;
    use crate::objects::ordered_hash_set::OrderedHashSet;
    use crate::objects::ordered_name_dictionary::OrderedNameDictionary;
    use crate::objects::script_context_table::ScriptContextTable;
    use crate::objects::array_list::ArrayList;
    use crate::objects::sloppy_arguments_elements::SloppyArgumentsElements;
    use crate::v8::internal::kNullAddress;
    use crate::v8::internal::Address;
    use crate::v8::internal::Internals;
    use crate::v8::internal::kWeakHeapObjectTag;
    use crate::v8::internal::kSmiTagSize;
    //use crate::objects::bigint::BigInt;
    //use crate::objects::heap_number::HeapNumber;
    //use crate::objects::visitors::HeapObject;

    #[derive(Debug, PartialEq, Eq)]
    pub struct MaybeWeak<T> {
        _phantom: PhantomData<T>,
    }

    pub struct ClearedWeakValue {}

    pub type StrongTaggedBase = TaggedImpl<HeapObjectReferenceType::STRONG, Address>;
    pub type WeakTaggedBase = TaggedImpl<HeapObjectReferenceType::WEAK, Address>;

    pub trait Taggable {}

    impl Taggable for HeapObject {}
    impl Taggable for Object {}
    impl Taggable for Smi {}
    impl Taggable for TaggedIndex {}
    impl Taggable for FieldType {}

    #[derive(Debug, Copy, Clone)]
    pub struct Tagged<T> {
        ptr: Address,
        _phantom: PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn ptr(&self) -> Address {
            self.ptr
        }
    }

    impl<T> Default for Tagged<T> {
        fn default() -> Self {
            Tagged {
                ptr: 0,
                _phantom: PhantomData,
            }
        }
    }

    impl<T> From<Address> for Tagged<T> {
        fn from(ptr: Address) -> Self {
            Tagged {
                ptr,
                _phantom: PhantomData,
            }
        }
    }

    impl Tagged<Object> {
        pub const fn new(o: Address) -> Self {
            Tagged { ptr: o, _phantom: PhantomData }
        }

        pub const fn default() -> Self {
            Tagged { ptr: kNullAddress, _phantom: PhantomData }
        }
        // Implicit conversion for subclasses -- all classes are subclasses of Object,
        // so allow all tagged pointers.
        // NOLINTNEXTLINE
        pub const fn from(other: StrongTaggedBase) -> Self {
            Tagged { ptr: other.ptr(), _phantom: PhantomData }
        }
        pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
            Tagged { ptr: (ptr as Address) + kHeapObjectTag, _phantom: PhantomData }
        }

        pub fn operator_equals(&self, other: &Tagged<Object>) -> bool {
            self.ptr == other.ptr
        }
    }

    impl Tagged<Smi> {
        pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }

        pub const fn from(ptr: Address) -> Self {
            Tagged { ptr, _phantom: PhantomData }
        }

        pub const fn is_heap_object(&self) -> bool {
            false
        }
        pub const fn is_smi(&self) -> bool {
            true
        }

        pub const fn value(&self) -> i32 {
            Internals::smi_value(self.ptr)
        }
    }
    impl Tagged<TaggedIndex> {
        pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }

        pub const fn from(ptr: Address) -> Self {
            Tagged { ptr, _phantom: PhantomData }
        }

        pub const fn is_heap_object(&self) -> bool {
            false
        }
        pub const fn is_smi(&self) -> bool {
            true
        }

          // Returns the integer value.
        pub const fn value(&self) -> i64 {
            // Truncate and shift down (requires >> to be sign extending).
            (self.ptr as i64) >> kSmiTagSize
        }
    }

    impl Tagged<HeapObject> {
        pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }
         pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
            Tagged { ptr: (ptr as Address) + kHeapObjectTag, _phantom: PhantomData }
        }

        pub fn operator_equals(&self, other: &Tagged<HeapObject>) -> bool {
            self.ptr == other.ptr
        }
        pub const fn is_null(&self) -> bool {
             self.ptr == kNullAddress
        }
        pub const fn is_smi(&self) -> bool {
            false
        }
        pub const fn is_heap_object(&self) -> bool {
            true
        }

    }

    impl Tagged<MaybeWeak<Object>> {
        pub const fn new(o: Address) -> Self {
            Tagged { ptr: o, _phantom: PhantomData }
        }

        pub const fn default() -> Self {
            Tagged { ptr: kNullAddress, _phantom: PhantomData }
        }
    }

    impl Tagged<MaybeWeak<HeapObject>> {
        pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }
        pub fn from_heap_object_layout(ptr: *const HeapObjectLayout) -> Self {
            Tagged { ptr: (ptr as Address) + kHeapObjectTag, _phantom: PhantomData }
        }
        pub const fn is_null(&self) -> bool {
             self.ptr == kNullAddress
        }
        pub const fn is_smi(&self) -> bool {
            false
        }
    }

    impl<T> Tagged<Union<T>> {
        pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }
    }

    impl<T: Taggable> Tagged<T> {
         pub const fn new() -> Self {
            Tagged { ptr: 0, _phantom: PhantomData }
        }
    }
    impl ClearedWeakValue {
        pub const fn new() -> Self {
            ClearedWeakValue {}
        }
    }

    impl Tagged<ClearedWeakValue> {
        pub fn new(ptr: Address) -> Self {
            Tagged { ptr, _phantom: PhantomData }
        }
    }
     impl Tagged<MaybeWeak<HeapObject>> {
        pub fn from_type<U>(other: Tagged<U>, type_: HeapObjectReferenceType) -> Self
            where U: Taggable
        {
            Tagged {
                ptr: if type_ == HeapObjectReferenceType::WEAK {
                    MakeWeak(other).ptr()
                } else {
                    MakeStrong(other).ptr()
                },
                _phantom: PhantomData,
            }
        }
    }

    pub fn make_weak<T>(value: Tagged<T>) -> Tagged<MaybeWeak<T>> {
        Tagged { ptr: value.ptr() | kWeakHeapObjectTag, _phantom: PhantomData }
    }

    pub fn make_weak_maybe_weak<T>(value: Tagged<MaybeWeak<T>>) -> Tagged<MaybeWeak<T>> {
        Tagged { ptr: value.ptr() | kWeakHeapObjectTag, _phantom: PhantomData }
    }

    pub fn make_strong<T>(value: Tagged<T>) -> Tagged<T> {
        Tagged { ptr: value.ptr() & (!kWeakHeapObjectTag | kHeapObjectTag), _phantom: PhantomData }
    }

    pub fn make_strong_maybe_weak<T>(value: Tagged<MaybeWeak<T>>) -> Tagged<T> {
        Tagged { ptr: value.ptr() & (!kWeakHeapObjectTag | kHeapObjectTag), _phantom: PhantomData }
    }

    pub trait BaseType {}
    impl BaseType for Object{}
    impl BaseType for HeapObject{}
    impl BaseType for HeapObjectLayout{}
    impl BaseType for TrustedObject{}
    impl BaseType for TrustedObjectLayout{}

    pub trait ListType {}
    impl ListType for FixedArrayBase{}
    impl ListType for FixedArray{}
    impl ListType for FixedDoubleArray{}
    impl ListType for ByteArray{}
    impl ListType for NameDictionary{}
    impl ListType for NumberDictionary{}
    impl ListType for OrderedHashMap{}
    impl ListType for OrderedHashSet{}
    impl ListType for OrderedNameDictionary{}
    impl ListType for ScriptContextTable{}
    impl ListType for ArrayList{}
    impl ListType for SloppyArgumentsElements{}
}

