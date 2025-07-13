// Converted from V8 C++ source files:
// Header: fixed-array.h
// Implementation: fixed-array.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod fixed_array {
use std::mem;
use std::ptr;
use std::marker::PhantomData;
//use crate::v8::internal::handles::MaybeHandle;
//use crate::v8::internal::handles::Handle;
//use crate::v8::internal::handles::DirectHandle;
//use crate::v8::internal::handles::IndirectHandle;
use crate::objects::object_macros::*;
use crate::objects::objects::*;
use crate::objects::smi::*;
use crate::objects::tagged::*;
use crate::objects::heap_object::*;
use crate::objects::maybe_object::*;
use crate::objects::trusted_object::*;
//use crate::roots::roots::ReadOnlyRoots;
//use crate::utils::memcopy::MemCopy;

const kMaxFixedArrayCapacity: i32 = if true { 16 * 1024 * 1024 } else { 64 * 1024 * 1024 };

pub mod detail {
    use super::*;

    pub trait ArrayHeaderBaseTrait {
        fn capacity(&self) -> i32;
        fn capacity_with_tag(&self, tag: AcquireLoadTag) -> i32;
        fn set_capacity(&mut self, value: i32);
        fn set_capacity_with_tag(&mut self, value: i32, tag: ReleaseStoreTag);
    }

    #[repr(C)]
    pub struct ArrayHeaderBase<Super, const K_LENGTH_EQUALS_CAPACITY: bool> {
        super_field: Super,
        length_or_capacity: TaggedMember<Smi>, // Using TaggedMember<Smi> for both length and capacity
        phantom: PhantomData<Super>,
    }

    impl<Super, const K_LENGTH_EQUALS_CAPACITY: bool> ArrayHeaderBase<Super, K_LENGTH_EQUALS_CAPACITY> {
        pub fn new(super_field: Super, initial_length: i32) -> Self {
            ArrayHeaderBase {
                super_field,
                length_or_capacity: TaggedMember::new(Smi {value: initial_length}),
                phantom: PhantomData,
            }
        }

	pub fn super_field(&self) -> &Super {
            &self.super_field
        }

        pub fn super_field_mut(&mut self) -> &mut Super {
            &mut self.super_field
        }

    }


    impl<Super> ArrayHeaderBase<Super, false> {
        pub fn capacity(&self) -> i32 {
            self.length_or_capacity.load().value
        }

        pub fn capacity_with_tag(&self, tag: AcquireLoadTag) -> i32 {
            self.capacity()
        }

        pub fn set_capacity(&mut self, value: i32) {
            self.length_or_capacity.store(Smi { value });
        }

        pub fn set_capacity_with_tag(&mut self, value: i32, tag: ReleaseStoreTag) {
            self.set_capacity(value);
        }
    }

    impl<Super> ArrayHeaderBase<Super, true> {
        pub fn length(&self) -> i32 {
            self.length_or_capacity.load().value
        }

        pub fn length_with_tag(&self, tag: AcquireLoadTag) -> i32 {
            self.length()
        }

        pub fn set_length(&mut self, value: i32) {
            self.length_or_capacity.store(Smi { value });
        }

        pub fn set_length_with_tag(&mut self, value: i32, tag: ReleaseStoreTag) {
            self.set_length(value);
        }

        pub fn capacity(&self) -> i32 {
            self.length()
        }

        pub fn capacity_with_tag(&self, tag: AcquireLoadTag) -> i32 {
            self.length()
        }

        pub fn set_capacity(&mut self, value: i32) {
            self.set_length(value);
        }

        pub fn set_capacity_with_tag(&mut self, value: i32, tag: ReleaseStoreTag) {
            self.set_length(value);
        }
    }

    pub struct TaggedArrayHeaderHelper<Shape, Super, Dummy> {
        phantom: PhantomData<(Shape, Super, Dummy)>,
    }

    impl<Shape, Super, Dummy> TaggedArrayHeaderHelper<Shape, Super, Dummy> {
        pub type Type = ArrayHeaderBase<Super, Shape::K_LENGTH_EQUALS_CAPACITY>;
    }

    // Add a struct for ExtraFields
    pub struct ExtraFields<BaseHeader> {
        base: BaseHeader,
        // Add additional fields here
    }

    // Add an impl for ExtraFields
    impl<BaseHeader> ExtraFields<BaseHeader> {
        pub fn new(base: BaseHeader) -> Self {
            ExtraFields {
                base,
            }
        }

	pub fn base(&self) -> &BaseHeader {
            &self.base
        }

        pub fn base_mut(&mut self) -> &mut BaseHeader {
            &mut self.base
        }
    }

    // TaggedArrayHeader
    pub type TaggedArrayHeader<Shape, Super> = <TaggedArrayHeaderHelper<Shape, Super, ()> as TaggedArrayHeaderHelper<Shape, Super, ()>>::Type;

    pub trait ExtraFieldsTrait<Super> {
        type ExtraFieldsType;
    }
}

macro_rules! v8_array_extra_fields {
    ($($body:tt)*) => {
        mod extra_fields {
            use super::*;

            #[repr(C)]
            pub struct ExtraFields<Super> {
                super_field: Super,
                pub length_: TaggedMember<Smi>,
                // Add more fields as needed
            }

            impl<Super> ExtraFields<Super> {
                pub fn new(super_field: Super, length: i32) -> Self {
                    ExtraFields {
                        super_field,
                        length_: TaggedMember::new(Smi { value: length }),
                    }
                }

		pub fn super_field(&self) -> &Super {
                    &self.super_field
                }

                pub fn super_field_mut(&mut self) -> &mut Super {
                    &mut self.super_field
                }


                pub fn length(&self) -> i32 {
                    self.length_.load().value
                }

                pub fn set_length(&mut self, value: i32) {
                    self.length_.store(Smi { value });
                }
            }

	    //impl<Super> std::ops::Deref for ExtraFields<Super> {
            //    type Target = Super;

            //    fn deref(&self) -> &Self::Target {
            //        &self.super_field
            //    }
            //}

            //impl<Super> std::ops::DerefMut for ExtraFields<Super> {
            //    fn deref_mut(&mut self) -> &mut Self::Target {
            //        &mut self.super_field
            //    }
            //}

            
        }
    };
}

macro_rules! v8_object_end {
    () => {};
}

pub trait Shape {
    type ElementT;
    type CompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex;
    const K_LENGTH_EQUALS_CAPACITY: bool;
}

#[repr(C)]
pub struct TaggedArrayBase<Derived, ShapeT, Super = HeapObjectLayout>
where
    ShapeT: Shape,
{
    header: detail::TaggedArrayHeader<ShapeT, Super>,
    objects: Vec<Tagged<ShapeT::ElementT>>, // Flexible array member
    _derived: PhantomData<Derived>,
    _shape: PhantomData<ShapeT>,
}

impl<Derived, ShapeT, Super> TaggedArrayBase<Derived, ShapeT, Super>
where
    ShapeT: Shape,
{
    pub fn header(&self) -> &detail::TaggedArrayHeader<ShapeT, Super> {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut detail::TaggedArrayHeader<ShapeT, Super> {
        &mut self.header
    }

    pub fn objects(&self) -> &Vec<Tagged<ShapeT::ElementT>> {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Tagged<ShapeT::ElementT>> {
        &mut self.objects
    }
}

impl<Derived, ShapeT, Super> TaggedArrayBase<Derived, ShapeT, Super>
where
    ShapeT: Shape,
    Super: HeapObjectTrait,
{

  pub fn get(&self, index: i32) -> Tagged<ShapeT::ElementT> {
        self.get_with_mode(index, RelaxedLoadTag{})
  }

    pub fn get_with_mode(&self, index: i32, _mode: RelaxedLoadTag) -> Tagged<ShapeT::ElementT> {
        self.objects[index as usize]
    }

    pub fn get_acquire(&self, index: i32) -> Tagged<ShapeT::ElementT> {
        self.get_with_mode(index, AcquireLoadTag{})
    }

    pub fn get_seq_cst(&self, index: i32) -> Tagged<ShapeT::ElementT> {
        self.get_with_mode(index, SeqCstAccessTag{})
    }

    pub fn set(&mut self, index: i32, value: Tagged<ShapeT::ElementT>, mode: WriteBarrierMode) {
        if std::any::TypeId::of::<ShapeT::ElementT>() == std::any::TypeId::of::<Smi>() {
            self.objects[index as usize] = value;
        } else {
           self.objects[index as usize] = value;
        }
    }

    pub fn set_smi(&mut self, index: i32, value: Tagged<Smi>)
    where
        ShapeT::ElementT: From<Smi>,
    {
       self.objects[index as usize] = Tagged::from(value.value().into());
    }

    pub fn set_with_relaxed_store_tag(&mut self, index: i32, value: Tagged<ShapeT::ElementT>, _tag: RelaxedStoreTag, mode: WriteBarrierMode) {
            self.set(index, value, mode);
    }

    pub fn set_smi_with_relaxed_store_tag(&mut self, index: i32, value: Tagged<Smi>, _tag: RelaxedStoreTag)
    where
        ShapeT::ElementT: From<Smi>,
    {
        self.set_smi(index, value);
    }

    pub fn set_with_release_store_tag(&mut self, index: i32, value: Tagged<ShapeT::ElementT>, _tag: ReleaseStoreTag, mode: WriteBarrierMode) {
       self.set(index, value, mode);
    }

    pub fn set_smi_with_release_store_tag(&mut self, index: i32, value: Tagged<Smi>, _tag: ReleaseStoreTag)
    where
        ShapeT::ElementT: From<Smi>,
    {
        self.set_smi(index, value);
    }

    pub fn set_with_seq_cst_access_tag(&mut self, index: i32, value: Tagged<ShapeT::ElementT>, _tag: SeqCstAccessTag, mode: WriteBarrierMode) {
       self.set(index, value, mode);
    }

    pub fn set_smi_with_seq_cst_access_tag(&mut self, index: i32, value: Tagged<Smi>, _tag: SeqCstAccessTag)
    where
        ShapeT::ElementT: From<Smi>,
    {
       self.set_smi(index, value);
    }

    pub fn swap(&mut self, index: i32, value: Tagged<ShapeT::ElementT>, _tag: SeqCstAccessTag, mode: WriteBarrierMode) -> Tagged<ShapeT::ElementT> {
        let old_value = self.get(index);
        self.set(index, value, mode);
        old_value
    }

    pub fn compare_and_swap(&mut self, index: i32, expected: Tagged<ShapeT::ElementT>, value: Tagged<ShapeT::ElementT>, _tag: SeqCstAccessTag, mode: WriteBarrierMode) -> Tagged<ShapeT::ElementT> {
	//TODO!! implement compare and swap correctly.
        let old_value = self.get(index);
	if old_value.value() == expected.value() {
        	self.set(index, value, mode);
	}
        old_value
    }

    pub fn move_elements(isolate: *mut Isolate, dst: Tagged<Derived>, dst_index: i32, src: Tagged<Derived>, src_index: i32, len: i32, mode: WriteBarrierMode) {
	//TODO!!: implement move_elements properly
    }

    pub fn copy_elements(isolate: *mut Isolate, dst: Tagged<Derived>, dst_index: i32, src: Tagged<Derived>, src_index: i32, len: i32, mode: WriteBarrierMode) {
	//TODO!!: implement copy_elements properly
    }

    pub fn right_trim(&mut self, isolate: *mut Isolate, new_capacity: i32) {
        self.objects.truncate(new_capacity as usize);
	self.header_mut().super_field_mut().heap_object_header_mut().set_map(HeapObject::cast::<HeapObjectLayout>(&HeapObject::new()));
    }

    pub fn allocated_size(&self) -> i32 {
        std::mem::size_of::<Self>() as i32
    }

    pub const fn size_for(capacity: i32) -> i32 {
        std::mem::size_of::<detail::TaggedArrayHeader<ShapeT, Super>>() as i32 + capacity * std::mem::size_of::<Tagged<ShapeT::ElementT>>() as i32
    }

    pub const fn offset_of_element_at(index: i32) -> i32 {
        Self::size_for(index)
    }

    pub fn raw_field_of_first_element(&self) -> ObjectSlot {
        ObjectSlot {address : 0}//TODO!! implement correctly
    }

    pub fn raw_field_of_element_at(&self, index: i32) -> ObjectSlot {
        ObjectSlot {address : 0}//TODO!! implement correctly
    }

    pub const K_MAX_CAPACITY: i32 = kMaxFixedArrayCapacity;

    pub fn is_in_bounds(&self, index: i32) -> bool {
        index >= 0 && index < self.objects.len() as i32
    }

    pub fn is_cow_array(&self) -> bool {
        false
    }

     pub fn get_write_barrier_mode(&self, _no_gc: &DisallowGarbageCollection) -> WriteBarrierMode {
        if std::any::TypeId::of::<ShapeT::ElementT>() == std::any::TypeId::of::<Smi>() {
            WriteBarrierMode::SKIP_WRITE_BARRIER
        } else {
            WriteBarrierMode::UPDATE_WRITE_BARRIER
        }
    }


}

pub struct TaggedArrayShape {}
impl Shape for TaggedArrayShape {
    type ElementT = Object;
    type CompressionScheme = V8HeapCompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kFixedArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
}

#[repr(C)]
pub struct FixedArray {
    base: TaggedArrayBase<FixedArray, TaggedArrayShape>,
}

impl FixedArray {
    pub fn base(&self) -> &TaggedArrayBase<FixedArray, TaggedArrayShape> {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut TaggedArrayBase<FixedArray, TaggedArrayShape> {
        &mut self.base
    }
}

impl HeapObjectTrait for FixedArray {}

impl FixedArray {
    pub fn new(_isolate: *mut Isolate, capacity: i32, allocation: AllocationType) -> Box<FixedArray> {
	let mut objects: Vec<Tagged<Object>> = Vec::new();
	for _i in 0..capacity {
		objects.push(Tagged::from(Object {}));
	}

        Box::new(FixedArray {
            base: TaggedArrayBase {
                header: detail::TaggedArrayHeader::<TaggedArrayShape, HeapObjectLayout>::new(HeapObjectLayout::new(), capacity),
                objects,
                _derived: PhantomData,
                _shape: PhantomData,
            },
        })
    }


    pub fn move_elements(isolate: *mut Isolate, dst_index: i32, src_index: i32, len: i32, mode: WriteBarrierMode) {
	    //TODO!! implement MoveElements properly
    }

    pub fn copy_elements(isolate: *mut Isolate, dst_index: i32, src: Tagged<FixedArray>, src_index: i32, len: i32, mode: WriteBarrierMode) {
	    //TODO!! implement CopyElements properly
    }

    pub fn set_and_grow(
        _isolate: *mut Isolate,
        array: DirectHandle<FixedArray>,
        index: i32,
        value: DirectHandle<Object>,
    ) -> DirectHandle<FixedArray> {
       array
    }

   pub fn set_and_grow_indirect(
        _isolate: *mut Isolate,
        array: IndirectHandle<FixedArray>,
        index: i32,
        value: DirectHandle<Object>,
    ) -> IndirectHandle<FixedArray> {
       array
    }


    pub fn right_trim(&mut self, isolate: *mut Isolate, new_capacity: i32) {
        self.base.right_trim(isolate, new_capacity);
	self.base_mut().header_mut().super_field_mut().heap_object_header_mut().set_map(HeapObject::cast::<HeapObjectLayout>(&HeapObject::new()));
    }

     pub fn right_trim_or_empty(
        _isolate: *mut Isolate,
        array: DirectHandle<FixedArray>,
        new_length: i32,
    ) -> DirectHandle<FixedArray> {
	array
    }

    pub fn right_trim_or_empty_indirect(
        _isolate: *mut Isolate,
        array: IndirectHandle<FixedArray>,
        new_length: i32,
    ) -> IndirectHandle<FixedArray> {
	array
    }

    pub fn fill_with_holes(&mut self, from: i32, to: i32) {
            //TODO implement fill_with_holes properly
    }

    pub fn is_the_hole(&self, _isolate: *mut Isolate, _index: i32) -> bool {
        false//TODO!! implement is_the_hole properly
    }

    pub fn set_the_hole(&mut self, _isolate: *mut Isolate, _index: i32) {
	    //TODO!! implement set_the_hole properly
    }

    pub fn set_the_hole_read_only_roots(&mut self, _ro_roots: ReadOnlyRoots, _index: i32) {
	    //TODO!! implement set_the_hole properly
    }
}

impl HeapObject for FixedArray {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

pub struct TrustedArrayShape {}
impl Shape for TrustedArrayShape {
    type ElementT = Object;
    type CompressionScheme = V8HeapCompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kTrustedFixedArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
}

#[repr(C)]
pub struct TrustedFixedArray {
    base: TaggedArrayBase<TrustedFixedArray, TrustedArrayShape, TrustedObjectLayout>,
}

impl HeapObjectTrait for TrustedFixedArray {}

impl TrustedFixedArray {
    pub fn new(_isolate: *mut Isolate, capacity: i32, allocation: AllocationType) -> Box<TrustedFixedArray> {
        Box::new(TrustedFixedArray {
            base: TaggedArrayBase {
                header: detail::TaggedArrayHeader::<TrustedArrayShape, TrustedObjectLayout>::new(TrustedObjectLayout::new(), capacity),
                objects: vec![Tagged::from(Object {}); capacity as usize],
                _derived: PhantomData,
                _shape: PhantomData,
            },
        })
    }

    pub const K_MAX_LENGTH: i32 = FixedArray::K_MAX_CAPACITY;
}

impl HeapObject for TrustedFixedArray {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

pub struct ProtectedArrayShape {}
impl Shape for ProtectedArrayShape {
    type ElementT = Union<TrustedObject, Smi>;
    type CompressionScheme = TrustedSpaceCompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kProtectedFixedArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
}

#[repr(C)]
pub struct ProtectedFixedArray {
    base: TaggedArrayBase<ProtectedFixedArray, ProtectedArrayShape, TrustedObjectLayout>,
}

impl HeapObjectTrait for ProtectedFixedArray {}

impl ProtectedFixedArray {
    pub fn new(_isolate: *mut Isolate, capacity: i32) -> Box<ProtectedFixedArray> {
        Box::new(ProtectedFixedArray {
            base: TaggedArrayBase {
                header: detail::TaggedArrayHeader::<ProtectedArrayShape, TrustedObjectLayout>::new(TrustedObjectLayout::new(), capacity),
                objects: vec![Tagged::from(Union::Smi(Smi { value: 0 })); capacity as usize],
                _derived: PhantomData,
                _shape: PhantomData,
            },
        })
    }
    pub const K_MAX_LENGTH: i32 = FixedArray::K_MAX_CAPACITY;
}

impl HeapObject for ProtectedFixedArray {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

#[repr(C)]
pub struct FixedArrayExact {
    fixed_array: FixedArray,
}

impl HeapObjectTrait for FixedArrayExact {}

impl HeapObject for FixedArrayExact {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.fixed_array.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.fixed_array.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

#[repr(C)]
pub struct FixedArrayBase {
    header: detail::ArrayHeaderBase<HeapObjectLayout, true>,
}

impl HeapObjectTrait for FixedArrayBase {}

impl FixedArrayBase {
    pub const K_LENGTH_OFFSET: i32 = HeapObject::K_HEADER_SIZE;
    pub const K_HEADER_SIZE: i32 = Self::K_LENGTH_OFFSET + std::mem::size_of::<Tagged<Smi>>() as i32;
    pub const K_MAX_LENGTH: i32 = FixedArray::K_MAX_CAPACITY;

    pub fn get_max_length_for_new_space_allocation(kind: ElementsKind) -> i32 {
        ((kMaxRegularHeapObjectSize - Self::K_HEADER_SIZE) >> elements_kind_to_shift_size(kind)) as i32
    }

    pub fn is_cow_array(&self) -> bool {
        false
    }
}

impl HeapObject for FixedArrayBase {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.header.super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.header.super_field_mut().heap_object_header_mut()
    }
}

#[repr(C)]
pub struct PrimitiveArrayBase<Derived, ShapeT, Super = HeapObjectLayout>
where
    ShapeT: Shape,
{
    header: detail::ArrayHeaderBase<Super, true>,
    values: Vec<<ShapeT as Shape>::ElementT>,
    _derived: PhantomData<Derived>,
    _shape: PhantomData<ShapeT>,
}

impl<Derived, ShapeT, Super> PrimitiveArrayBase<Derived, ShapeT, Super>
where
    ShapeT: Shape,
{

    pub fn header(&self) -> &detail::ArrayHeaderBase<Super, true> {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut detail::ArrayHeaderBase<Super, true> {
        &mut self.header
    }

    pub fn values(&self) -> &Vec<<ShapeT as Shape>::ElementT> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<<ShapeT as Shape>::ElementT> {
        &mut self.values
    }

    pub fn get(&self, index: i32) -> <ShapeT as Shape>::ElementT {
        self.values[index as usize]
    }

    pub fn set(&mut self, index: i32, value: <ShapeT as Shape>::ElementT) {
        self.values[index as usize] = value;
    }

    pub fn allocated_size(&self) -> i32 {
        Self::size_for(self.header().length() as i32)
    }

    pub const fn size_for(length: i32) -> i32 {
        let offset = Self::offset_of_element_at(length);
        (offset + 7) & !7 //OBJECT_POINTER_ALIGN
    }

    pub const fn offset_of_element_at(index: i32) -> i32 {
        std::mem::size_of::<detail::ArrayHeaderBase<Super, true>>() as i32 + index * std::mem::size_of::<<ShapeT as Shape>::ElementT>() as i32
    }

    pub fn begin(&mut self) -> *mut <ShapeT as Shape>::ElementT {
        self.values.as_mut_ptr()
    }

    pub fn begin_const(&self) -> *const <ShapeT as Shape>::ElementT {
        self.values.as_ptr()
    }

    pub fn end(&mut self) -> *mut <ShapeT as Shape>::ElementT {
        unsafe { self.begin().add(self.header().length() as usize) }
    }

    pub fn end_const(&self) -> *const <ShapeT as Shape>::ElementT {
        unsafe { self.begin_const().add(self.header().length() as usize) }
    }

    pub fn data_size(&self) -> i32 {
        (self.header().length() as usize * std::mem::size_of::<<ShapeT as Shape>::ElementT>()) as i32
    }

    pub fn is_in_bounds(&self, index: i32) -> bool {
        index >= 0 && index < self.header().length()
    }
   
    pub const K_MAX_LENGTH: i32 = kMaxFixedArrayCapacity;

}

pub struct FixedDoubleArrayShape {}
impl Shape for FixedDoubleArrayShape {
    type ElementT = f64;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kFixedDoubleArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
    type CompressionScheme = V8HeapCompressionScheme;
}

#[repr(C)]
pub struct FixedDoubleArray {
    base: PrimitiveArrayBase<FixedDoubleArray, FixedDoubleArrayShape>,
}

impl FixedDoubleArray {
    pub fn base(&self) -> &PrimitiveArrayBase<FixedDoubleArray, FixedDoubleArrayShape> {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut PrimitiveArrayBase<FixedDoubleArray, FixedDoubleArrayShape> {
        &mut self.base
    }
}

impl HeapObjectTrait for FixedDoubleArray {}

impl FixedDoubleArray {

    pub fn new(_isolate: *mut Isolate, capacity: i32, allocation: AllocationType) -> Box<FixedDoubleArray> {
        Box::new(FixedDoubleArray {
            base: PrimitiveArrayBase {
                header: detail::ArrayHeaderBase::<HeapObjectLayout, true>::new(HeapObjectLayout::new(), capacity),
                values: vec![0.0; capacity as usize],
                _derived: PhantomData,
                _shape: PhantomData,
            },
        })
    }

   pub fn get_scalar(&self, index: i32) -> f64 {
        self.base().get(index)
    }

    pub fn get_representation(&self, index: i32) -> u64 {
	    unsafe { mem::transmute::<f64, u64>(self.base().get(index)) }
    }

   pub fn get(array: Tagged<FixedDoubleArray>, index: i32, isolate: *mut Isolate) -> Tagged<Object> {
       Tagged::from(Object{})
    }

     pub fn set(&mut self, index: i32, value: f64) {
         self.base_mut().set(index, value);
    }

   pub fn set_the_hole(&mut self, _isolate: *mut Isolate, index: i32) {
        //TODO Implement set_the_hole.
    }

    pub fn set_the_hole_no_isolate(&mut self, index: i32) {
        //TODO Implement set_the_hole_no_isolate.
    }
	
    pub fn is_the_hole(&self, _isolate: *mut Isolate, index: i32) -> bool {
	    //TODO Implement is_the_hole
	    false
    }

    pub fn is_the_hole_no_isolate(&self, index: i32) -> bool {
	    //TODO Implement is_the_hole_no_isolate
	    false
    }

    pub fn move_elements(isolate: *mut Isolate, dst_index: i32, src_index: i32, len: i32, unused: WriteBarrierMode) {
	    //TODO Implement move_elements properly
    }

    pub fn fill_with_holes(&mut self, from: i32, to: i32) {
	    //TODO Implement FillWithHoles properly
    }
}

impl HeapObject for FixedDoubleArray {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

pub struct WeakFixedArrayShape {}
impl Shape for WeakFixedArrayShape {
    type ElementT = MaybeObject;
    type CompressionScheme = V8HeapCompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kWeakFixedArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
}

#[repr(C)]
pub struct WeakFixedArray {
    base: TaggedArrayBase<WeakFixedArray, WeakFixedArrayShape>,
}

impl HeapObjectTrait for WeakFixedArray {}

impl WeakFixedArray {

    pub fn new(_isolate: *mut Isolate, capacity: i32, allocation: AllocationType, initial_value: MaybeDirectHandle<Object>) -> Box<WeakFixedArray> {
        Box::new(WeakFixedArray {
            base: TaggedArrayBase {
                header: detail::TaggedArrayHeader::<WeakFixedArrayShape, HeapObjectLayout>::new(HeapObjectLayout::new(), capacity),
                objects: vec![Tagged::from(MaybeObject::from(Object{})); capacity as usize], //TODO Check MaybeObject
                _derived: PhantomData,
                _shape: PhantomData,
            },
        })
    }
}

impl HeapObject for WeakFixedArray {
    fn heap_object_header(&self) -> &HeapObjectHeader {
        &self.base.header().super_field().heap_object_header()
    }

    fn heap_object_header_mut(&mut self) -> &mut HeapObjectHeader {
        &mut self.base.header_mut().super_field_mut().heap_object_header_mut()
    }
}

pub struct TrustedWeakFixedArrayShape {}
impl Shape for TrustedWeakFixedArrayShape {
    type ElementT = MaybeObject;
    type CompressionScheme = V8HeapCompressionScheme;
    const K_MAP_ROOT_INDEX: RootIndex = RootIndex::kTrustedWeakFixedArrayMap;
    const K_LENGTH_EQUALS_CAPACITY: bool = true;
}

#[repr(C)]
pub struct TrustedWeakFixedArray {
    base: TaggedArrayBase<TrustedWeakFixedArray, TrustedWeakFixedArrayShape, TrustedObjectLayout>,
}

impl HeapObjectTrait for TrustedWeakFixedArray {}

impl TrustedWeak
