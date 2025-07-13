// Converted from V8 C++ source files:
// Header: tagged-index.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct Tagged<T>(T);

impl<T> Tagged<T> {
    pub fn ptr(&self) -> usize {
        0
    }
}

#[macro_export]
macro_rules! DECL_STATIC_VERIFIER {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn verify() -> bool {
                true
            }
        }
    };
}
const kSmiTagSize: usize = 1;
const kUintptrAllBitsSet: usize = usize::MAX;
const kSmiTag: usize = 0;

pub struct TaggedIndex {}

impl TaggedIndex {
    pub fn FromIntptr(value: isize) -> Tagged<TaggedIndex> {
        assert!(TaggedIndex::IsValid(value));
        let value_usize = value as usize;
        Tagged::<TaggedIndex>(TaggedIndex{})
    }

    pub const fn IsValid(value: isize) -> bool {
        const MIN_VALUE: isize = (kUintptrAllBitsSet as isize) >> (Self::kTaggedValueSize - 1);
        const MAX_VALUE: isize = -(MIN_VALUE + 1);
        MIN_VALUE <= value && value <= MAX_VALUE
    }

    pub const kTaggedValueSize: usize = 31;
    pub const kMinValue: isize = (kUintptrAllBitsSet as isize) << (Self::kTaggedValueSize - 1);
    pub const kMaxValue: isize = -(Self::kMinValue + 1);
}

DECL_STATIC_VERIFIER!(TaggedIndex);

pub struct CastTraits<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl CastTraits<TaggedIndex> {
    pub fn AllowFrom(value: Tagged<Object>) -> bool {
        let ptr = value.ptr();
        (ptr & 1) != 0
    }
    pub fn AllowFromHeapObject(value: Tagged<HeapObject>) -> bool {
        false
    }
}
pub struct Object {}
pub struct HeapObject {}
