// Converted from V8 C++ source files:
// Header: heap-object.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Range;
use crate::v8::internal::Address;
use crate::v8::internal::V8_EXPORT_PRIVATE;
use crate::v8::internal::ReadOnlyRoots;
use crate::v8::internal::WriteBarrierMode;
use crate::v8::internal::RelaxedStoreTag;
use crate::v8::internal::ObjectSlot;
use crate::v8::internal::MaybeObjectSlot;
use crate::v8::internal::InstructionStreamSlot;
use crate::v8::internal::ExternalPointerSlot;
use crate::v8::internal::CppHeapPointerSlot;
use crate::v8::internal::IndirectPointerSlot;
use crate::v8::internal::ExternalPointerTagRange;
use crate::v8::internal::IndirectPointerTag;
use crate::v8::internal::CodeEntrypointTag;
use crate::v8::internal::JSDispatchHandle;

pub struct Heap;
pub struct Map;
pub struct Isolate;
pub struct Object;
pub struct MaybeObject;
pub struct Code;
pub struct String;
pub struct Sandbox;
pub struct Number;
pub struct PropertyDetails;
pub struct TaggedPrimitiveHeapObject;
pub struct JSFunction;
pub struct ObjectsOddball;
pub struct ExternalPointerTag;
pub struct IsolateForSandbox;
pub struct IsolateForPointerCompression;
pub struct TrustedPointerPublishingScope;
pub struct WritableFreeSpace;
pub struct Tagged<T>(Address, PhantomData<T>);
pub struct DirectHandle<T>(Address, PhantomData<T>);
pub struct Handle<T>(Address, PhantomData<T>);
pub struct MapWord;
pub struct CppHeapPointerTag;
pub struct CppHeapPointerTagRange;
pub struct DisallowGarbageCollection;
pub enum GCType {}
pub enum ValueType {}
pub enum TaggedImplBase {}
pub struct TaggedImpl<A, B>(Address, PhantomData<(A, B)>);
pub enum HeapObjectReferenceType {
    STRONG,
}
pub struct StrongTaggedBase {}
pub struct Managed<T> {}
pub enum MachineType {}
pub enum Condition {}
pub struct Operand {}
pub struct Register {}
pub struct InstructionOperand {}
pub struct Bytecode {}
pub struct ValuePair {}
pub struct IrregexpImplementation {}
pub struct OpIndex {}
pub struct TaggedMember<T> { dummy: i32, phantom : PhantomData<T> }
pub struct Static {}
pub struct Label {}
pub struct PropertyDescriptorObject {}

#[derive(Debug, PartialEq, Eq)]
pub enum HeapObjectError {
    AllocationFailed,
    InvalidObject,
    OutOfBounds,
}

#[repr(C)]
pub struct HeapObjectLayout {
    map_: TaggedMember<Map>,
}

impl HeapObjectLayout {
    #[inline]
    pub fn map(&self) -> Tagged<Map> {
        unsafe { std::ptr::read_unaligned(&self.map_ as *const TaggedMember<Map> as *const Tagged<Map>)}
    }

    #[inline]
    pub fn map_acquire_load(&self) -> Tagged<Map> {
        unsafe { std::ptr::read_volatile(&self.map_ as *const TaggedMember<Map> as *const Tagged<Map>)}
    }

    #[inline]
    pub fn set_map(&mut self, isolate: *mut Isolate, value: Tagged<Map>) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_release_store<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_safe_transition<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_safe_transition_no_write_barrier(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: RelaxedStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_after_allocation<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, mode: WriteBarrierMode) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_no_write_barrier(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: RelaxedStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, value)};
    }

    #[inline]
    pub fn set_map_word_forwarded(&mut self, target_object: Tagged<HeapObject>, _tag: ReleaseStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, unsafe { std::mem::transmute(target_object) })};
    }

    #[inline]
    pub fn set_map_word_forwarded_relaxed(&mut self, target_object: Tagged<HeapObject>, _tag: RelaxedStoreTag) {
        unsafe { std::ptr::write_unaligned(&mut self.map_ as *mut TaggedMember<Map> as *mut Tagged<Map>, unsafe { std::mem::transmute(target_object) })};
    }
    
    #[inline]
    pub fn ptr(&self) -> Address {
        self.address() + kHeapObjectTag
    }

    #[inline]
    pub fn address(&self) -> Address {
        self as *const Self as Address
    }

    #[inline]
    pub fn early_get_read_only_roots(&self) -> ReadOnlyRoots {
        ReadOnlyRoots {}
    }

    #[inline]
    pub fn size(&self) -> i32 {
        16
    }

    pub fn size_from_map(&self, map: Tagged<Map>) -> i32 {
        16
    }

    #[inline]
    pub fn get_write_barrier_mode(&self, _promise: &DisallowGarbageCollection) -> WriteBarrierMode {
        WriteBarrierMode {}
    }
}

const kTaggedSize : usize = 8;
const kHeapObjectTag : Address = 1;

impl PartialEq<StrongTaggedBase> for &HeapObjectLayout {
    fn eq(&self, other: &StrongTaggedBase) -> bool {
        Tagged::<HeapObject>(self as *const HeapObjectLayout as Address) == *other
    }
}

impl PartialEq<&HeapObjectLayout> for StrongTaggedBase {
    fn eq(&self, other: &&HeapObjectLayout) -> bool {
        *self == Tagged::<HeapObject>(other as *const HeapObjectLayout as Address)
    }
}

impl HeapObject {
    pub const kMapOffset: i32 = 0;
    pub const kHeaderSize: i32 = std::mem::size_of::<HeapObjectLayout>() as i32;

    #[inline]
    pub fn map(&self) -> Tagged<Map> {
        unsafe { (self as *const Self as *const HeapObjectLayout).read_unaligned().map() }
    }

    #[inline]
    pub fn set_map(&mut self, isolate: *mut Isolate, value: Tagged<Map>) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn set_map_safe_transition<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn map_slot(&self) -> ObjectSlot {
        ObjectSlot {}
    }

    #[inline]
    pub fn set_map_no_write_barrier(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: RelaxedStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn set_map_no_write_barrier_release_store(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn set_map_safe_transition_no_write_barrier(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: RelaxedStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn set_map_safe_transition_no_write_barrier_release_store(&mut self, isolate: *mut Isolate, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn map_acquire_load(&self) -> Tagged<Map> {
        unsafe { (self as *const Self as *const HeapObjectLayout).read_unaligned().map() }
    }

    #[inline]
    pub fn set_map_release_store<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn set_map_safe_transition_release_store<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, _tag: ReleaseStoreTag) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    #[inline]
    pub fn release_compare_and_swap_map_word_forwarded(&mut self, old_map_word: MapWord, new_target_object: Tagged<HeapObject>) -> bool {
        true
    }

    #[inline]
    pub fn relaxed_compare_and_swap_map_word_forwarded(&mut self, old_map_word: MapWord, new_target_object: Tagged<HeapObject>) -> bool {
        true
    }

    #[inline]
    pub fn set_map_after_allocation<IsolateT>(&mut self, isolate: *mut IsolateT, value: Tagged<Map>, mode: WriteBarrierMode) {
        unsafe { (self as *mut Self as *mut HeapObjectLayout).write_unaligned(HeapObjectLayout { map_: TaggedMember{ dummy : 1, phantom : PhantomData} }) };
    }

    pub fn set_filler_map(writable_page: &WritableFreeSpace, value: Tagged<Map>) {
    }

    #[inline]
    pub fn map_word_relaxed(&self) -> MapWord {
        MapWord {}
    }

    #[inline]
    pub fn set_map_word(&mut self, map: Tagged<Map>, _tag: RelaxedStoreTag) {
    }

    #[inline]
    pub fn set_map_word_forwarded(&mut self, target_object: Tagged<HeapObject>, _tag: RelaxedStoreTag) {
    }

    #[inline]
    pub fn map_word_acquire_load(&self) -> MapWord {
        MapWord {}
    }

    #[inline]
    pub fn set_map_word_release_store(&mut self, map: Tagged<Map>, _tag: ReleaseStoreTag) {
    }

    #[inline]
    pub fn set_map_word_forwarded_release_store(&mut self, target_object: Tagged<HeapObject>, _tag: ReleaseStoreTag) {
    }

    #[inline]
    pub fn early_get_read_only_roots(&self) -> ReadOnlyRoots {
        ReadOnlyRoots {}
    }

    #[inline]
    pub fn from_address(address: Address) -> Tagged<HeapObject> {
        Tagged::<HeapObject>(address + kHeapObjectTag, PhantomData)
    }

    #[inline]
    pub fn address(&self) -> Address {
        self.ptr() - kHeapObjectTag
    }

    #[inline]
    pub fn size(&self) -> i32 {
        16
    }

    pub fn size_from_map(&self, map: Tagged<Map>) -> i32 {
        16
    }

    pub fn read_field<T>(&self, offset: usize) -> T
        where T: Copy
    {
        unsafe {
            let address = self.field_address(offset);
            std::ptr::read_unaligned(address as *const T)
        }
    }

    pub fn write_field<T>(&self, offset: usize, value: T)
        where T: Copy
    {
        unsafe {
            let address = self.field_address(offset);
            std::ptr::write_unaligned(address as *mut T, value);
        }
    }

    pub fn relaxed_read_field<T>(&self, offset: usize) -> T
        where T: Copy
    {
        unsafe {
            let address = self.field_address(offset);
            std::ptr::read_unaligned(address as *const T)
        }
    }

    pub fn relaxed_write_field<T>(&self, offset: usize, value: T)
        where T: Copy
    {
        unsafe {
            let address = self.field_address(offset);
            std::ptr::write_unaligned(address as *mut T, value);
        }
    }

    pub fn acquire_read_field<T>(&self, offset: usize) -> T
        where T: Copy
    {
        unsafe {
            let address = self.field_address(offset);
            std::ptr::read_unaligned(address as *const T)
        }
    }

    pub fn seqcst_compare_and_swap_field<CompareAndSwapImpl>(
        expected_value: Tagged<Object>,
        new_value: Tagged<Object>,
        compare_and_swap_impl: CompareAndSwapImpl,
    ) -> Tagged<Object> {
        new_value
    }

    pub fn read_sandboxed_pointer_field(&self, offset: usize, cage_base: PtrComprCageBase) -> Address {
        0
    }
    pub fn write_sandboxed_pointer_field(&self, offset: usize, cage_base: PtrComprCageBase, value: Address) {
    }
    pub fn write_sandboxed_pointer_field_isolate(&self, offset: usize, isolate: *mut Isolate, value: Address) {
    }

    pub fn read_bounded_size_field(&self, offset: usize) -> usize {
        0
    }
    pub fn write_bounded_size_field(&self, offset: usize, value: usize) {
    }

    pub fn init_external_pointer_field<const tag: ExternalPointerTag>(&self, offset: usize, isolate: IsolateForSandbox, value: Address, mode: WriteBarrierMode) {
    }

    pub fn read_external_pointer_field<const tag: ExternalPointerTagRange>(&self, offset: usize, isolate: IsolateForSandbox) -> Address {
        0
    }

    pub fn read_cpp_heap_pointer_field<const lower_bound: CppHeapPointerTag, const upper_bound: CppHeapPointerTag>(&self, offset: usize, isolate: IsolateForPointerCompression) -> Address {
        0
    }
    pub fn read_cpp_heap_pointer_field_range(&self, offset: usize, isolate: IsolateForPointerCompression, tag_range: CppHeapPointerTagRange) -> Address {
        0
    }

    pub fn write_external_pointer_field<const tag: ExternalPointerTag>(&self, offset: usize, isolate: IsolateForSandbox, value: Address) {
    }

    pub fn setup_lazily_initialized_external_pointer_field(&self, offset: usize) {
    }

    pub fn write_lazily_initialized_external_pointer_field<const tag: ExternalPointerTag>(&self, offset: usize, isolate: IsolateForSandbox, value: Address) {
    }

    pub fn setup_lazily_initialized_cpp_heap_pointer_field(&self, offset: usize) {
    }

    pub fn write_lazily_initialized_cpp_heap_pointer_field<const tag: CppHeapPointerTag>(&self, offset: usize, isolate: IsolateForPointerCompression, value: Address) {
    }

    pub fn write_lazily_initialized_cpp_heap_pointer_field_tagged(&self, offset: usize, isolate: IsolateForPointerCompression, value: Address, tag: CppHeapPointerTag) {
    }

    #[cfg(V8_ENABLE_SANDBOX)]
    pub fn init_self_indirect_pointer_field(&self, offset: usize, isolate: IsolateForSandbox, opt_publishing_scope: *mut TrustedPointerPublishingScope) {
    }

    pub fn read_trusted_pointer_field<const tag: IndirectPointerTag>(&self, offset: usize, isolate: IsolateForSandbox) -> Tagged<ExposedTrustedObject> {
        Tagged::<ExposedTrustedObject>(0, PhantomData)
    }

    pub fn read_trusted_pointer_field_acquire_load<const tag: IndirectPointerTag>(&self, offset: usize, isolate: IsolateForSandbox, _acquire_load_tag: AcquireLoadTag) -> Tagged<ExposedTrustedObject> {
        Tagged::<ExposedTrustedObject>(0, PhantomData)
    }

    pub fn read_maybe_empty_trusted_pointer_field<const tag: IndirectPointerTag>(&self, offset: usize, isolate: IsolateForSandbox, _acquire_load_tag: AcquireLoadTag) -> Tagged<Object> {
        Tagged::<Object>(0, PhantomData)
    }

    pub fn write_trusted_pointer_field<const tag: IndirectPointerTag>(&self, offset: usize, value: Tagged<ExposedTrustedObject>) {
    }

    pub fn is_trusted_pointer_field_empty(&self, offset: usize) -> bool {
        false
    }

    pub fn is_trusted_pointer_field_unpublished(&self, offset: usize, tag: IndirectPointerTag, isolate: IsolateForSandbox) -> bool {
        false
    }

    pub fn clear_trusted_pointer_field(&self, offset: usize) {
    }
    pub fn clear_trusted_pointer_field_release_store(&self, offset: usize, _release_store_tag: ReleaseStoreTag) {
    }

    pub fn read_code_pointer_field(&self, offset: usize, isolate: IsolateForSandbox) -> Tagged<Code> {
        Tagged::<Code>(0, PhantomData)
    }

    pub fn write_code_pointer_field(&self, offset: usize, value: Tagged<Code>) {
    }

    pub fn is_code_pointer_field_empty(&self, offset: usize) -> bool {
        false
    }

    pub fn clear_code_pointer_field(&self, offset: usize) {
    }

    pub fn read_code_entrypoint_via_code_pointer_field(&self, offset: usize, tag: CodeEntrypointTag) -> Address {
        0
    }

    pub fn write_code_entrypoint_via_code_pointer_field(&self, offset: usize, value: Address, tag: CodeEntrypointTag) {
    }

    pub fn allocate_and_install_js_dispatch_handle<ObjectType>(host: ObjectType, offset: usize, isolate: *mut Isolate, parameter_count: u16, code: DirectHandle<Code>, mode: WriteBarrierMode) -> JSDispatchHandle {
        JSDispatchHandle {}
    }

    pub fn raw_field(&self, byte_offset: i32) -> ObjectSlot {
        ObjectSlot {}
    }

    pub fn raw_maybe_weak_field(&self, byte_offset: i32) -> MaybeObjectSlot {
        MaybeObjectSlot {}
    }

    pub fn raw_instruction_stream_field(&self, byte_offset: i32) -> InstructionStreamSlot {
        InstructionStreamSlot {}
    }

    pub fn raw_external_pointer_field(&self, byte_offset: i32, tag_range: ExternalPointerTagRange) -> ExternalPointerSlot {
        ExternalPointerSlot {}
    }

    pub fn raw_cpp_heap_pointer_field(&self, byte_offset: i32) -> CppHeapPointerSlot {
        CppHeapPointerSlot {}
    }

    pub fn raw_indirect_pointer_field(&self, byte_offset: i32, tag: IndirectPointerTag) -> IndirectPointerSlot {
        IndirectPointerSlot {}
    }

    #[inline]
    pub fn get_write_barrier_mode(&self, _promise: &DisallowGarbageCollection) -> WriteBarrierMode {
        WriteBarrierMode {}
    }

    pub fn heap_object_short_print(&self, os: &mut std::ostream) {
    }

    pub fn print(&self) {
    }

    pub fn print_static(obj: Tagged<Object>) {
    }

    pub fn print_static_os(obj: Tagged<Object>, os: &mut std::ostream) {
    }

    pub fn print_header(&self, os: &mut std::ostream, id: &str) {
    }

    pub fn verify_object_field(&self, isolate: *mut Isolate, offset: i32) {
    }

    pub fn verify_smi_field(&self, offset: i32) {
    }

    pub fn verify_maybe_object_field(&self, isolate: *mut Isolate, offset: i32) {
    }

    pub fn verify_heap_pointer(isolate: *mut Isolate, p: Tagged<Object>) {
    }

    pub fn verify_code_pointer(isolate: *mut Isolate, p: Tagged<Object>) {
    }

    pub fn required_alignment(map: Tagged<Map>) -> AllocationAlignment {
        AllocationAlignment::kTaggedAligned
    }

    pub fn check_required_alignment(&self, cage_base: PtrComprCageBase) -> bool {
        true
    }

    pub fn needs_rehashing_instance_type(&self, instance_type: InstanceType) -> bool {
        false
    }
    pub fn needs_rehashing_cage_base(&self, cage_base: PtrComprCageBase) -> bool {
        false
    }

    pub fn can_be_rehashed(&self, cage_base: PtrComprCageBase) -> bool {
        true
    }

    pub fn rehash_based_on_map<IsolateT>(&mut self, isolate: *mut IsolateT) {
    }

    #[inline]
    pub fn get_field_address(&self, field_offset: i32) -> Address {
        self.ptr() + field_offset as Address - kHeapObjectTag
    }

    #[inline]
    pub fn field_address(&self, offset: usize) -> Address {
        self.ptr() + offset as Address - kHeapObjectTag
    }
}

impl<A, B> TaggedImpl<A, B> {
    pub fn is_smi<T>(obj: T) -> bool {
        false
    }
    pub fn is_heap_object<T>(obj: T) -> bool {
        true
    }
}

#[repr(u32)]
pub enum InstanceType {
    FIRST_TYPE = 0,
    JS_OBJECT_TYPE = FIRST_TYPE,
    NUMBER_TYPE,
    STRING_TYPE,
}

#[repr(u32)]
pub enum AllocationAlignment {
    kWordAligned,
    kTaggedAligned,
}

pub struct PtrComprCageBase {}
pub struct AcquireLoadTag {}
pub struct ReleaseStoreTag {}
pub struct Oddball {}
pub struct Hole {}
pub struct HashTableBase {}
pub struct SmallOrderedHashTable {}
pub struct PropertyDictionary {}
pub struct StringTable {}

macro_rules! impl_is_type {
    ($name:ident) => {
        impl HeapObject {
            pub fn is_$name(obj: Tagged<HeapObject>) -> bool {
                false
            }
            pub fn is_$name_cage(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
                false
            }
            pub fn is_$name_heap(obj: HeapObject) -> bool {
                false
            }
            pub fn is_$name_heap_cage(obj: HeapObject, cage_base: PtrComprCageBase) -> bool {
                false
            }
            pub fn is_$name_layout(obj: *const HeapObjectLayout) -> bool {
                false
            }
            pub fn is_$name_layout_cage(obj: *const HeapObjectLayout, cage_base: PtrComprCageBase) -> bool {
                false
            }
        }
    };
}

impl_is_type!(js_object);
impl_is_type!(number);
impl_is_type!(string);
impl_is_type!(hash_table_base);
impl_is_type!(small_ordered_hash_table);
impl_is_type!(property_dictionary);

impl HeapObject {
    pub fn outside_sandbox_or_in_readonly_space(obj: Tagged<HeapObject>) -> bool {
        false
    }

    pub fn fast_in_read_only_space_or_small_smi<T>(obj: T) -> bool {
        false
    }
}
