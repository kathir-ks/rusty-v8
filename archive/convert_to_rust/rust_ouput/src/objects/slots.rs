// Converted from V8 C++ source files:
// Header: slots.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod slots {
    use std::marker::PhantomData;
    use crate::base::memory::IsAligned;
    use crate::objects::fixed_array_inl::{code};
    use crate::objects::heap_object::Object;
    use crate::sandbox::external_pointer_table::ExternalPointerTable;
    use crate::sandbox::external_pointer::ExternalPointer_t;
    use crate::sandbox::indirect_pointer_tag::IndirectPointerTag;
    use crate::sandbox::isolate::IsolateForSandbox;
    use crate::objects::tagged_field::TaggedMemberBase;
    use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
    use std::fmt;
    use std::mem;
    use std::ptr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use crate::objects::name::HashFieldType;

    pub struct V8 {}
    pub struct Address {}
    pub struct HeapObjectReference {}
    pub struct HeapObject {}
    pub struct JSArrayBuffer {}
    pub struct RawContent {}
    pub struct WritableJitAllocation {}
    pub struct DisallowGarbageCollection {}
    pub struct CppHeapPointer_t {}
    pub struct CppHeapPointerTag {}
    pub struct String {}
    pub struct RegExpData {}
    pub struct ExternalPointerHandle {}
    pub struct JSMap {}
    pub struct DirectHandle<T> {
        _phantom: PhantomData<T>
    }
    pub enum GCType {}
    pub struct Register {}
    pub struct Operand {}
    pub enum Condition {}
    pub struct Script {}
    pub struct CpuFeatures {}
    pub struct JSPluralRules {dummy : i32}
    pub struct ValueType {}
    pub struct InstructionOperand {}
    pub struct Isolate {}
    pub struct FeedbackSlot {}
    pub struct PtrComprCageBase {}
    pub struct StoreRepresentation {}
    pub struct MaybeObject {}
    pub struct ZoneVector<T> {}
    pub struct SourceRange {}
    pub struct ZoneSnapshot {}
    pub struct Zone {}
    pub struct CaseClause {}
    pub struct Range {}
    pub struct CodeEntrypointTag {}
    pub struct Macro {}
    pub struct Representation {}
    pub struct Sandbox {}
    pub struct OpIndex {}
    pub struct Label {}
    pub struct AtomicMemoryOrder {}
    pub struct InstructionOperand {}
    pub struct Block {}
    pub struct Operation {}
    pub struct RegExpDataWrapper {}
    pub struct Iterator {}
    pub struct FixedArray {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum HeapObjectReferenceType {
        WEAK,
        STRONG,
    }

    pub struct TaggedImpl<Type, T> {
        _phantom_type: PhantomData<Type>,
        _phantom_data: PhantomData<T>,
    }

    impl<Type, T> TaggedImpl<Type, T> {
        pub fn new() -> Self {
            TaggedImpl {
                _phantom_type: PhantomData,
                _phantom_data: PhantomData,
            }
        }
    }

    pub type TaggedBase = TaggedImpl<HeapObjectReferenceType::STRONG, Address>;

    pub trait SlotData {
        const SIZE: usize;
        const ALIGNMENT: usize;
    }

    impl SlotData for Address {
        const SIZE: usize = std::mem::size_of::<Address>();
        const ALIGNMENT: usize = std::mem::align_of::<Address>();
    }

    impl SlotData for i32 {
        const SIZE: usize = std::mem::size_of::<i32>();
        const ALIGNMENT: usize = std::mem::align_of::<i32>();
    }

    const kNullAddress: Address = Address {};
    const kTaggedSize: usize = 8;
    const kSystemPointerSize: usize = 8;
    const kExternalPointerNullTag: ExternalPointerTag = ExternalPointerTag::kExternalPointerNullTag;
    const kWaiterQueueNodeTag: ExternalPointerTag = ExternalPointerTag::kWaiterQueueNodeTag;
    const kArrayBufferExtensionTag: ExternalPointerTag = ExternalPointerTag::kArrayBufferExtensionTag;
    const kIndirectPointerNullTag: IndirectPointerTag = IndirectPointerTag::kIndirectPointerNullTag;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ExternalPointerTag {
        kExternalPointerNullTag,
        kArrayBufferExtensionTag,
        kWaiterQueueNodeTag,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ExternalPointerTagRange {
        first: ExternalPointerTag,
        last: ExternalPointerTag,
    }

    impl ExternalPointerTagRange {
        fn Size(&self) -> i32 {
            1
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CppHeapPointerTagRange {
        range,
    }

    impl CppHeapPointerTagRange {
        fn Size(&self) -> i32 {
            1
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IndirectPointerTag {
        kIndirectPointerNullTag,
        range,
    }

    impl IndirectPointerTag {
        fn Size(&self) -> i32 {
            1
        }
    }

    pub trait ExternalPointerMemberTrait {
        fn storage_address(&self) -> Address;
    }

    pub struct ExternalPointerMember<const TAG: ExternalPointerTag> {}

    impl<const TAG: ExternalPointerTag> ExternalPointerMemberTrait for ExternalPointerMember<TAG> {
        fn storage_address(&self) -> Address {
            Address {}
        }
    }

    impl ExternalPointerTagRange {
        pub fn new() -> Self {
            ExternalPointerTagRange {
                first: ExternalPointerTag::kExternalPointerNullTag,
                last: ExternalPointerTag::kExternalPointerNullTag,
            }
        }
    }

    pub struct SlotBase<Subclass, Data, const SlotDataAlignment: usize = { mem::size_of::<Data>() }> {
        ptr_: Address,
        _phantom_subclass: PhantomData<Subclass>,
        _phantom_data: PhantomData<Data>,
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> SlotBase<Subclass, Data, SlotDataAlignment>
        where
            Subclass: Copy,
    {
        pub fn new(ptr: Address) -> Self {
            Self {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn address(&self) -> Address {
            self.ptr_
        }

        pub fn location(&self) -> *mut Data {
            self.ptr_ as *mut Data
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> fmt::Debug for SlotBase<Subclass, Data, SlotDataAlignment> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SlotBase")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> Copy for SlotBase<Subclass, Data, SlotDataAlignment> where Subclass: Copy {}
    impl<Subclass, Data, const SlotDataAlignment: usize> Clone for SlotBase<Subclass, Data, SlotDataAlignment> where Subclass: Copy {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> PartialEq for SlotBase<Subclass, Data, SlotDataAlignment> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr_ == other.ptr_
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> Eq for SlotBase<Subclass, Data, SlotDataAlignment> {}

    impl<Subclass, Data, const SlotDataAlignment: usize> PartialOrd for SlotBase<Subclass, Data, SlotDataAlignment> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.ptr_.partial_cmp(&other.ptr_)
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> Ord for SlotBase<Subclass, Data, SlotDataAlignment> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.ptr_.cmp(&other.ptr_)
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> SlotBase<Subclass, Data, SlotDataAlignment> {
        pub fn to_void_ptr(&self) -> *mut std::ffi::c_void {
            self.address() as *mut std::ffi::c_void
        }
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> SlotBase<Subclass, Data, SlotDataAlignment>
        where
            Subclass: Copy + From<SlotBase<Subclass, Data, SlotDataAlignment>>,
            Data: Copy,
    {
        const kSlotDataSize: usize = mem::size_of::<Data>();
        const kSlotDataAlignment_const: usize = SlotDataAlignment;
    }

    impl<Subclass, Data, const SlotDataAlignment: usize> SlotBase<Subclass, Data, SlotDataAlignment> {
        pub fn new_aligned(ptr: Address) -> Self {
            assert!(IsAligned(ptr, SlotDataAlignment));
            SlotBase {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }
    }

    macro_rules! implement_slot_base_operators {
    ($Subclass:ident, $Data:ty, $SlotDataAlignment:expr) => {
        impl $Subclass {
            fn kSlotDataSize() -> usize {
                mem::size_of::<$Data>()
            }
        }
        impl Add<usize> for $Subclass {
            type Output = $Subclass;

            fn add(self, i: usize) -> Self {
                let ptr = (self.ptr() as usize) + i * Self::kSlotDataSize();
                $Subclass::new(Address {  })
            }
        }

        impl Sub<usize> for $Subclass {
            type Output = $Subclass;

            fn sub(self, i: usize) -> Self {
                let ptr = (self.ptr() as usize) - i * Self::kSlotDataSize();
                $Subclass::new(Address {  })
            }
        }

        impl AddAssign<usize> for $Subclass {
            fn add_assign(&mut self, i: usize) {
                self.set_ptr((self.ptr() as usize + i * Self::kSlotDataSize()) as Address);
            }
        }

        impl SubAssign<usize> for $Subclass {
            fn sub_assign(&mut self, i: usize) {
                self.set_ptr((self.ptr() as usize - i * Self::kSlotDataSize()) as Address);
            }
        }

        impl Sub<$Subclass> for $Subclass {
            type Output = usize;

            fn sub(self, other: $Subclass) -> Self::Output {
                assert!(self.ptr() >= other.ptr());
                ((self.ptr() as usize - other.ptr() as usize) / Self::kSlotDataSize()) as usize
            }
        }

        impl PartialOrd for $Subclass {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.ptr().partial_cmp(&other.ptr())
            }
        }

        impl Ord for $Subclass {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.ptr().cmp(&other.ptr())
            }
        }

        impl<T> From<SlotBase<T, $Data, $SlotDataAlignment>> for $Subclass where T: Copy {
            fn from(slot: SlotBase<T, $Data, $SlotDataAlignment>) -> Self {
                $Subclass {
                    ptr_: slot.ptr_,
                    _phantom_subclass: PhantomData,
                    _phantom_data: PhantomData,
                }
            }
        }

        impl From<$Subclass> for SlotBase<$Subclass, $Data, $SlotDataAlignment> {
            fn from(slot: $Subclass) -> Self {
                SlotBase {
                    ptr_: slot.ptr_,
                    _phantom_subclass: PhantomData,
                    _phantom_data: PhantomData,
                }
            }
        }
    };
}

    #[derive(Copy, Clone)]
    pub struct FullObjectSlot {
        ptr_: Address,
        _phantom_subclass: PhantomData<FullObjectSlot>,
        _phantom_data: PhantomData<Address>,
    }

    impl fmt::Debug for FullObjectSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FullObjectSlot")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl FullObjectSlot {
        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
            self.ptr_ = ptr_;
        }
    }

    impl FullObjectSlot {
        pub type TObject = Tagged<Object>;
        pub type THeapObjectSlot = FullHeapObjectSlot;
        pub const kCanBeWeak: bool = false;

        pub fn new() -> Self {
            FullObjectSlot {
                ptr_: kNullAddress,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address(ptr: Address) -> Self {
            FullObjectSlot {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address_ptr(ptr: *const Address) -> Self {
            FullObjectSlot {
                ptr_: unsafe { *ptr },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_tagged_base(object: *mut TaggedBase) -> Self {
            FullObjectSlot {
                ptr_: unsafe { *object as *mut Address as Address },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_from_slot<T, Tagged_t>(slot: SlotBase<T, Tagged_t>) -> Self where T: Copy{
            FullObjectSlot {
                ptr_: slot.address(),
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn contains_map_value(&self, raw_value: Address) -> bool {
            unsafe { *(self.ptr_ as *const Address) == raw_value }
        }

        pub fn relaxed_contains_map_value(&self, raw_value: Address) -> bool {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                ptr.read().load(Ordering::Relaxed()) as Address == raw_value
            }
        }

        pub fn operator_star(&self) -> Tagged<Object> {
            self.load()
        }

        pub fn load(&self) -> Tagged<Object> {
            unsafe { *(self.ptr_ as *const Address) }.into()
        }

        pub fn load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
            self.load()
        }

        pub fn store(&self, value: Tagged<Object>) {
            unsafe { *(self.ptr_ as *mut Address) = value.into() };
        }

        pub fn store_map(&self, map: Tagged<Map>) {
            unsafe { *(self.ptr_ as *mut Address) = map.into() };
        }

        pub fn load_map(&self) -> Tagged<Map> {
            unsafe { *(self.ptr_ as *const Address) }.into()
        }

        pub fn acquire_load(&self) -> Tagged<Object> {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                (ptr.read().load(Ordering::Acquire()) as Address).into()
            }
        }

        pub fn acquire_load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
            self.acquire_load()
        }

        pub fn relaxed_load(&self) -> Tagged<Object> {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                (ptr.read().load(Ordering::Relaxed()) as Address).into()
            }
        }

        pub fn relaxed_load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
            self.relaxed_load()
        }

        pub fn relaxed_load_raw(&self) -> Address {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                ptr.read().load(Ordering::Relaxed()) as Address
            }
        }

        pub fn raw_to_tagged(cage_base: PtrComprCageBase, raw: Address) -> Tagged<Object> {
            raw.into()
        }

        pub fn relaxed_store(&self, value: Tagged<Object>) {
            unsafe {
                let ptr = self.ptr_ as *mut AtomicUsize;
                ptr.write(AtomicUsize::new(value.into() as usize));
                ptr.load(Ordering::Relaxed());
            }
        }

        pub fn release_store(&self, value: Tagged<Object>) {
            unsafe {
                let ptr = self.ptr_ as *mut AtomicUsize;
                ptr.write(AtomicUsize::new(value.into() as usize));
                ptr.load(Ordering::Release);
            }
        }

        pub fn relaxed_compare_and_swap(&self, old: Tagged<Object>, target: Tagged<Object>) -> Tagged<Object> {
            unsafe {
                let ptr = self.ptr_ as *mut AtomicUsize;
                let old_raw: usize = old.into() as usize;
                let target_raw: usize = target.into() as usize;
                (ptr.fetch_compare_and_swap(old_raw, target_raw, Ordering::Relaxed()) as Address).into()
            }
        }

        pub fn release_compare_and_swap(&self, old: Tagged<Object>, target: Tagged<Object>) -> Tagged<Object> {
            unsafe {
                let ptr = self.ptr_ as *mut AtomicUsize;
                let old_raw: usize = old.into() as usize;
                let target_raw: usize = target.into() as usize;
                (ptr.fetch_compare_and_swap(old_raw, target_raw, Ordering::Release) as Address).into()
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct FullMaybeObjectSlot {
        ptr_: Address,
        _phantom_subclass: PhantomData<FullMaybeObjectSlot>,
        _phantom_data: PhantomData<Address>,
    }

    impl fmt::Debug for FullMaybeObjectSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FullMaybeObjectSlot")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl FullMaybeObjectSlot {
        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
            self.ptr_ = ptr_;
        }
    }

    impl FullMaybeObjectSlot {
        pub type TObject = Tagged<MaybeObject>;
        pub type THeapObjectSlot = FullHeapObjectSlot;
        pub const kCanBeWeak: bool = true;

        pub fn new() -> Self {
            FullMaybeObjectSlot {
                ptr_: kNullAddress,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address(ptr: Address) -> Self {
            FullMaybeObjectSlot {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_ptr(ptr: *mut TaggedBase) -> Self {
            FullMaybeObjectSlot {
                ptr_: unsafe { ptr as *mut Address as Address },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_tagged_member(member: *const TaggedMemberBase) -> Self {
            FullMaybeObjectSlot {
                ptr_: unsafe { (*member).ptr_location() as *mut Address as Address },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_from_slot<T, Tagged_t>(slot: SlotBase<T, Tagged_t>) -> Self where T: Copy{
            FullMaybeObjectSlot {
                ptr_: slot.address(),
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn operator_star(&self) -> Tagged<MaybeObject> {
            self.load()
        }

        pub fn load(&self) -> Tagged<MaybeObject> {
            unsafe { *(self.ptr_ as *const Address) }.into()
        }

        pub fn load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
            self.load()
        }

        pub fn store(&self, value: Tagged<MaybeObject>) {
            unsafe { *(self.ptr_ as *mut Address) = value.into() };
        }

        pub fn relaxed_load(&self) -> Tagged<MaybeObject> {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                (ptr.read().load(Ordering::Relaxed()) as Address).into()
            }
        }

        pub fn relaxed_load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
            self.relaxed_load()
        }

        pub fn relaxed_load_raw(&self) -> Address {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                ptr.read().load(Ordering::Relaxed()) as Address
            }
        }

        pub fn raw_to_tagged(cage_base: PtrComprCageBase, raw: Address) -> Tagged<Object> {
            raw.into()
        }

        pub fn relaxed_store(&self, value: Tagged<MaybeObject>) {
            unsafe { *(self.ptr_ as *mut Address) = value.into() };
        }

        pub fn release_compare_and_swap(&self, old: Tagged<MaybeObject>, target: Tagged<MaybeObject>) {
            unsafe {
                let ptr = self.ptr_ as *mut AtomicUsize;
                let old_raw: usize = old.into() as usize;
                let target_raw: usize = target.into() as usize;
                ptr.fetch_compare_and_swap(old_raw, target_raw, Ordering::Release);
            };
        }
    }

    #[derive(Copy, Clone)]
    pub struct FullHeapObjectSlot {
        ptr_: Address,
        _phantom_subclass: PhantomData<FullHeapObjectSlot>,
        _phantom_data: PhantomData<Address>,
    }

    impl fmt::Debug for FullHeapObjectSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FullHeapObjectSlot")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl FullHeapObjectSlot {
        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
            self.ptr_ = ptr_;
        }
    }

    impl FullHeapObjectSlot {
        pub fn new() -> Self {
            FullHeapObjectSlot {
                ptr_: kNullAddress,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address(ptr: Address) -> Self {
            FullHeapObjectSlot {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_ptr(ptr: *mut TaggedBase) -> Self {
            FullHeapObjectSlot {
                ptr_: unsafe { ptr as *mut Address as Address },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_tagged_member(member: *const TaggedMemberBase) -> Self {
            FullHeapObjectSlot {
                ptr_: unsafe { (*member).ptr_location() as *mut Address as Address },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_from_slot<T, Tagged_t>(slot: SlotBase<T, Tagged_t>) -> Self where T: Copy{
            FullHeapObjectSlot {
                ptr_: slot.address(),
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn operator_star(&self) -> Tagged<HeapObjectReference> {
            self.load_with_cage_base(PtrComprCageBase{} )
        }

        pub fn load_with_cage_base(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObjectReference> {
            unsafe { *(self.ptr_ as *const Address) }.into()
        }

        pub fn store(&self, value: Tagged<HeapObjectReference>) {
            unsafe { *(self.ptr_ as *mut Address) = value.into() };
        }

        pub fn to_heap_object(&self) -> Tagged<HeapObject> {
            unsafe { *(self.ptr_ as *const Address) }.into()
        }

        pub fn store_heap_object(&self, value: Tagged<HeapObject>) {
            unsafe { *(self.ptr_ as *mut Address) = value.into() };
        }
    }

    #[derive(Copy, Clone)]
    pub struct UnalignedSlot<T> {
        ptr_: Address,
        _phantom_subclass: PhantomData<UnalignedSlot<T>>,
        _phantom_data: PhantomData<T>,
    }

    impl<T> UnalignedSlot<T> {
        pub type Reference = UnalignedSlotReference<T>;
        pub type DifferenceType = i32;
        pub type ValueType = T;
        pub type Pointer = *mut T;
        pub type IteratorCategory = std::iter::RandomAccessIterator;

        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
            self.ptr_ = ptr_;
        }
    }

    impl<T> UnalignedSlot<T> {
        pub fn new() -> Self {
            UnalignedSlot {
                ptr_: kNullAddress,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address(address: Address) -> Self {
            UnalignedSlot {
                ptr_: address,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_ptr(address: *mut T) -> Self {
            UnalignedSlot {
                ptr_: address as Address,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn operator_star(&self) -> UnalignedSlotReference<T> {
            UnalignedSlotReference::new(self.ptr_)
        }

        pub fn operator_index(&self, i: DifferenceType) -> UnalignedSlotReference<T> {
            UnalignedSlotReference::new((self.ptr_ as usize + i as usize * std::mem::size_of::<T>()) as Address)
        }

        pub fn address(&self) -> Address {
            self.ptr_
        }
    }

    impl<T> From<UnalignedSlot<T>> for Address {
        fn from(slot: UnalignedSlot<T>) -> Self {
            slot.ptr_
        }
    }

    impl<T> Sub for UnalignedSlot<T> {
        type Output = i32;

        fn sub(self, other: Self) -> Self::Output {
            ((self.address() as usize - other.address() as usize) / std::mem::size_of::<T>()) as i32
        }
    }

    impl<T> Copy for UnalignedSlot<T> {}
    impl<T> Clone for UnalignedSlot<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    pub struct UnalignedSlotReference<T> {
        address_: Address,
        _phantom: PhantomData<T>,
    }

    impl<T> UnalignedSlotReference<T> {
        pub fn new(address: Address) -> Self {
            UnalignedSlotReference {
                address_: address,
                _phantom: PhantomData,
            }
        }

        pub fn value(&self) -> T where T: Copy {
            unsafe { std::ptr::read_unaligned(self.address_ as *const T) }
        }

        pub fn set_value(&mut self, value: T) {
            unsafe { std::ptr::write_unaligned(self.address_ as *mut T, value) };
        }

        pub fn swap(&mut self, other: &mut Self) {
            unsafe {
                let tmp = std::ptr::read_unaligned(self.address_ as *const T);
                std::ptr::write_unaligned(self.address_ as *mut T, std::ptr::read_unaligned(other.address_ as *const T));
                std::ptr::write_unaligned(other.address_ as *mut T, tmp);
            }
        }

        pub fn address(&self) -> Address {
            self.address_
        }
    }

    impl<T: Copy> Copy for UnalignedSlotReference<T> {}
    impl<T: Copy> Clone for UnalignedSlotReference<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: PartialOrd> PartialOrd for UnalignedSlotReference<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value().partial_cmp(&other.value())
        }
    }

    impl<T: PartialEq> PartialEq for UnalignedSlotReference<T> {
        fn eq(&self, other: &Self) -> bool {
            self.value() == other.value()
        }
    }

    pub struct OffHeapFullObjectSlot {
        ptr_: Address,
        _phantom_subclass: PhantomData<OffHeapFullObjectSlot>,
        _phantom_data: PhantomData<Address>,
    }

    impl fmt::Debug for OffHeapFullObjectSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OffHeapFullObjectSlot")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl OffHeapFullObjectSlot {
        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
            self.ptr_ = ptr_;
        }
    }

    impl OffHeapFullObjectSlot {
        pub fn new() -> Self {
            OffHeapFullObjectSlot {
                ptr_: kNullAddress,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address(ptr: Address) -> Self {
            OffHeapFullObjectSlot {
                ptr_: ptr,
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn new_with_address_ptr(ptr: *const Address) -> Self {
            OffHeapFullObjectSlot {
                ptr_: unsafe { *ptr },
                _phantom_subclass: PhantomData,
                _phantom_data: PhantomData,
            }
        }

        pub fn relaxed_load(&self) -> Tagged<Object> {
            unsafe {
                let ptr = self.ptr_ as *const AtomicUsize;
                (ptr.read().load(Ordering::Relaxed()) as Address).into()
            }
        }
    }

    impl Copy for OffHeapFullObjectSlot {}
    impl Clone for OffHeapFullObjectSlot {
        fn clone(&self) -> Self {
            *self
        }
    }

    #[derive(Copy, Clone)]
    pub struct ExternalPointerSlot {
        ptr_: Address,
    #[cfg(feature = "v8_compress_pointers")]
        tag_range_: ExternalPointerTagRange,
    }

    impl fmt::Debug for ExternalPointerSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalPointerSlot")
                .field("ptr_", &(&self.ptr_ as *const Address))
                .finish()
        }
    }

    impl ExternalPointerSlot {
        pub fn ptr(&self) -> Address {
            self.ptr_
        }

        fn set_ptr(&mut self, ptr_: Address) {
