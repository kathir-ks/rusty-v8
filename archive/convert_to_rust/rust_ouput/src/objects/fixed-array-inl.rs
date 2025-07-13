// Converted from V8 C++ source files:
// Header: fixed-array-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
pub struct V8 {}
pub struct code {}
struct TaggedField<T, const OFFSET: usize>;
pub enum void {}
pub struct ReleaseStoreTag {}
pub enum class ExceptionStatus {
    kNoError,
    kOutOfMemory,
    kInvalidArguments,
    kUnexpected,
}
pub struct This {
    dummy: i32,
}
pub enum WriteBarrierMode {
    kNoWriteBarrier,
    kMapWriteBarrier,
    kFullWriteBarrier,
}
pub struct RelaxedStoreTag {}
pub enum SeqCstAccessTag {}
pub struct DisallowGarbageCollection {}
pub enum SlotType {
    kEmpty,
    kTagged,
    kUnboxedDouble,
    kUnboxedInt32,
}
pub struct ArrayList {}
pub struct PtrComprCageBase {}
pub enum AllocationType {
    kYoung,
    kOld,
    kCode,
    kMap,
    kLast,
    kSharedOld,
    kSharedCode,
    kSharedMap,
    kTrusted,
    kSharedTrusted,
}
pub struct JavaScript {}
pub struct ReadOnlyRoots {}
struct TVARIABLE<'a, T> {
    dummy: i32,
}
#[derive(Debug)]
struct IndirectHandle<T> {
    value: T,
}
struct Local<T> {
    dummy: i32,
}
struct PersistentBase<T> {
    dummy: i32,
}
struct RegisterT {
    dummy: i32,
}
struct IsolateForSandbox {
    dummy: i32,
}
struct Tagged<T> {
    dummy: i32,
}
struct TaggedObject {
    dummy: i32,
}
type Object = i32;
type OpIndex = i32;
type InstructionOperand = i32;
#[derive(Debug)]
struct Map {
    dummy: i32,
}
#[derive(Debug)]
struct Isolate {
    dummy: i32,
}
struct HeapObject {
    dummy: i32,
}
#[derive(Debug)]
struct FixedArray {
    header: detail::ArrayHeaderBase<FixedArray, true>,
}
#[derive(Debug)]
struct TrustedFixedArray {
    header: detail::ArrayHeaderBase<TrustedFixedArray, true>,
}
#[derive(Debug)]
struct ProtectedFixedArray {
    header: detail::ArrayHeaderBase<ProtectedFixedArray, true>,
}
#[derive(Debug)]
struct WeakFixedArray {
    header: detail::ArrayHeaderBase<WeakFixedArray, true>,
}
#[derive(Debug)]
struct TrustedWeakFixedArray {
    header: detail::ArrayHeaderBase<TrustedWeakFixedArray, true>,
}
#[derive(Debug)]
struct ProtectedWeakFixedArray {
    header: detail::ArrayHeaderBase<ProtectedWeakFixedArray, true>,
}
#[derive(Debug)]
struct FixedDoubleArray {
    header: detail::ArrayHeaderBase<FixedDoubleArray, true>,
}
#[derive(Debug)]
struct ByteArray {
    header: detail::ArrayHeaderBase<ByteArray, true>,
}
#[derive(Debug)]
struct TrustedByteArray {
    header: detail::ArrayHeaderBase<TrustedByteArray, true>,
}
#[derive(Debug)]
struct FixedArrayBase {
    header: detail::ArrayHeaderBase<FixedArrayBase, true>,
}
#[derive(Debug)]
struct FixedAddressArrayBase<Base> {
    header: detail::ArrayHeaderBase<FixedAddressArrayBase<Base>, true>,
}
#[derive(Debug)]
struct FixedIntegerArrayBase<T, Base> {
    header: detail::ArrayHeaderBase<FixedIntegerArrayBase<T, Base>, true>,
}
#[derive(Debug)]
struct PodArray<T> {
    header: detail::ArrayHeaderBase<PodArray<T>, true>,
}
#[derive(Debug)]
struct TrustedPodArray<T> {
    header: detail::ArrayHeaderBase<TrustedPodArray<T>, true>,
}
#[derive(Debug)]
struct WeakArrayList {
    header: detail::ArrayHeaderBase<WeakArrayList, true>,
}
mod detail {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};
    #[derive(Debug)]
    pub struct ArrayHeaderBase<S, const HAS_LENGTH: bool> {
        capacity_: AtomicI32,
        length_: AtomicI32,
        _phantom: std::marker::PhantomData<S>,
    }
    impl<S> ArrayHeaderBase<S, false> {
        pub fn capacity(&self) -> i32 {
            self.capacity_.load(Ordering::Relaxed)
        }
        pub fn capacity_acquire(&self) -> i32 {
            self.capacity_.load(Ordering::Acquire)
        }
        pub fn set_capacity(&self, value: i32) {
            self.capacity_.store(value, Ordering::Relaxed);
        }
        pub fn set_capacity_release(&self, value: i32) {
            self.capacity_.store(value, Ordering::Release);
        }
    }
    impl<S> ArrayHeaderBase<S, true> {
        pub fn length(&self) -> i32 {
            self.length_.load(Ordering::Relaxed)
        }
        pub fn length_acquire(&self) -> i32 {
            self.length_.load(Ordering::Acquire)
        }
        pub fn set_length(&self, value: i32) {
            self.length_.store(value, Ordering::Relaxed);
        }
        pub fn set_length_release(&self, value: i32) {
            self.length_.store(value, Ordering::Release);
        }
        pub fn capacity(&self) -> i32 {
            self.length()
        }
        pub fn capacity_acquire(&self) -> i32 {
            self.length_acquire()
        }
        pub fn set_capacity(&self, value: i32) {
            self.set_length(value);
        }
        pub fn set_capacity_release(&self, value: i32) {
            self.set_length_release(value);
        }
    }
}
const kHeapObjectTag: i32 = 0;
struct Smi {}
impl Smi {
    fn zero() -> Self {
        Smi {}
    }
    fn FromInt(value: i32) -> Self {
        Smi {}
    }
}
trait ArrayBase {
    fn capacity(&self) -> i32;
    fn set_capacity(&mut self, value: i32);
    fn length(&self) -> i32;
}
trait PrimitiveArray {
    type ElementMemberT;
    fn values(&self) -> &[Self::ElementMemberT];
    fn values_mut(&mut self) -> &mut [Self::ElementMemberT];
    fn length(&self) -> i32;
}
const SKIP_WRITE_BARRIER: WriteBarrierMode = WriteBarrierMode::kNoWriteBarrier;
const kRelaxedLoad: i32 = 0;
const kRelaxedStore: i32 = 0;
impl FixedArray {
    fn RawFieldOfFirstElement(&self) -> i32 {
        0
    }
}
impl FixedDoubleArray {
    fn values(&self) -> &[Double] {
        &[Double { value: 0.0 }]
    }
}
impl ByteArray {
    fn values(&self) -> &[u8] {
        &[0u8]
    }
}
impl TrustedByteArray {
    fn values(&self) -> &[u8] {
        &[0u8]
    }
}
struct Double {
    value: f64,
}
impl Double {
    fn value(&self) -> f64 {
        self.value
    }
    fn value_as_bits(&self) -> u64 {
        self.value.to_bits()
    }
    fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    fn set_value_as_bits(&mut self, bits: u64) {
        self.value = f64::from_bits(bits);
    }
}
const kHoleNanInt64: u64 = 0;
const kUndefinedNanInt64: u64 = 0;
impl<D, S, P> TaggedArrayBase<D, S, P> {
    fn EarlyGetReadOnlyRoots(&self) -> ReadOnlyRoots {
        ReadOnlyRoots {}
    }
}
impl WeakArrayList {
    fn GetPtrComprCageBase(&self) -> PtrComprCageBase {
        PtrComprCageBase {}
    }
    fn data_start(&self) -> i32 {
        0
    }
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    fn map(&self) -> i32 {
        0
    }
    fn set_map_after_allocation(&self, isolate: &Isolate, map: Map, mode: WriteBarrierMode) {}
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    fn objects(&self) -> &[i32] {
        &[0]
    }
}
impl<Base> FixedAddressArrayBase<Base> {
    fn get_element_address(&self, index: i32) -> Address {
        Address {}
    }
}
struct Address {}
impl Isolate {
    fn heap(&self) -> Heap {
        Heap {}
    }
    fn factory(&self) -> Factory {
        Factory {}
    }
}
struct Heap {}
struct Factory {}
impl Factory {
    fn AllocateRawArray(&self, size: i32, allocation: AllocationType) -> i32 {
        0
    }
    fn empty_fixed_array(&self) -> Handle<FixedArray> {
        Handle { dummy: 0 }
    }
    fn the_hole_value(&self) -> TaggedObject {
        TaggedObject { dummy: 0 }
    }
    fn NewNumber(&self, value: f64) -> Handle<Object> {
        Handle { dummy: 0 }
    }
    fn undefined_value(&self) -> Handle<Object> {
        Handle { dummy: 0 }
    }
    fn empty_weak_fixed_array(&self) -> Handle<WeakFixedArray> {
        Handle { dummy: 0 }
    }
    fn NewByteArray(&self, byte_length: i32, allocation: AllocationType) -> Handle<ByteArray> {
        Handle { dummy: 0 }
    }
    fn NewTrustedByteArray(&self, byte_length: i32) -> Handle<TrustedByteArray> {
        Handle { dummy: 0 }
    }
    fn empty_array_list(&self) -> DirectHandle<ArrayList> {
        DirectHandle { dummy: 0 }
    }
}
struct Handle<T> {
    dummy: i32,
}
struct DirectHandle<T> {
    dummy: i32,
}
struct MaybeDirectHandle<T> {
    dummy: i32,
    _phantom: std::marker::PhantomData<T>,
}
impl<T> MaybeDirectHandle<T> {
    fn is_null(&self) -> bool {
        true
    }
    fn ToHandleChecked(&self) -> Handle<T> {
        Handle { dummy: 0 }
    }
}
struct ReadOnlyHeap {}
impl ReadOnlyHeap {
    fn Contains(map: Map) -> bool {
        true
    }
}
impl ReadOnlyRoots {
    fn object_at(&self, index: i32) -> TaggedObject {
        TaggedObject { dummy: 0 }
    }
    fn the_hole_value(&self) -> TaggedObject {
        TaggedObject { dummy: 0 }
    }
    fn undefined_value(&self) -> TaggedObject {
        TaggedObject { dummy: 0 }
    }
}
impl Heap {
    fn MoveRange(&self, dst: FixedArray, dst_slot: SlotType, src_slot: SlotType, len: i32, mode: WriteBarrierMode) {}
    fn CopyRange(&self, dst: FixedArray, dst_slot: SlotType, src_slot: SlotType, len: i32, mode: WriteBarrierMode) {}
    fn RightTrimArray(&self, array: FixedArray, new_capacity: i32, old_capacity: i32) {}
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    const kMaxCapacity: i32 = 1024 * 1024;
}
impl<T, Base> FixedIntegerArrayBase<T, Base> {
    fn values(&self) -> &[u8] {
        &[0u8]
    }
}
const DEBUG_BOOL: bool = false;
const OBJECT_POINTER_ALIGN_SIZE: usize = 8;
fn OBJECT_POINTER_ALIGN(size: usize) -> usize {
    (size + (OBJECT_POINTER_ALIGN_SIZE - 1)) & !(OBJECT_POINTER_ALIGN_SIZE - 1)
}
fn UNCHECKED_CAST<T>(value: i32) -> T {
    unsafe { std::mem::transmute_copy(&value) }
}
macro_rules! TQ_OBJECT_CONSTRUCTORS_IMPL {
    ($name:ident) => {
        impl $name {
            fn new() -> Self {
                $name {
                    header: detail::ArrayHeaderBase {
                        capacity_: std::sync::atomic::AtomicI32::new(0),
                        length_: std::sync::atomic::AtomicI32::new(0),
                        _phantom: std::marker::PhantomData,
                    },
                }
            }
        }
    };
}
macro_rules! NEVER_READ_ONLY_SPACE_IMPL {
    ($name:ident) => {
        impl $name {
            fn never_read_only_space() -> bool {
                false
            }
        }
    };
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    const kMaxLength: i32 = 1024 * 1024;
}
const fn SizeFor(capacity: i32) -> i32 {
    capacity
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    fn IsInBounds(&self, index: i32) -> bool {
        (index as u32) < (self.capacity() as u32)
    }
    fn IsCowArray(&self) -> bool {
        self.map() == self.EarlyGetReadOnlyRoots().unchecked_fixed_cow_array_map()
    }
    fn get(&self, index: i32) -> i32 {
        0
    }
    fn get_relaxed(&self, index: i32) -> i32 {
        0
    }
    fn get_acquire(&self, index: i32) -> i32 {
        0
    }
    fn get_seqcst(&self, index: i32) -> i32 {
        0
    }
    fn set(&self, index: i32, value: i32, mode: WriteBarrierMode) {}
    fn set_smi(&self, index: i32, value: Smi) {}
    fn set_relaxed(&self, index: i32, value: i32, tag: RelaxedStoreTag, mode: WriteBarrierMode) {}
    fn set_smi_relaxed(&self, index: i32, value: Smi, tag: RelaxedStoreTag) {}
    fn set_release(&self, index: i32, value: i32, tag: ReleaseStoreTag, mode: WriteBarrierMode) {}
    fn set_smi_release(&self, index: i32, value: Smi, tag: ReleaseStoreTag) {}
    fn set_seqcst(&self, index: i32, value: i32, tag: SeqCstAccessTag, mode: WriteBarrierMode) {}
    fn set_smi_seqcst(&self, index: i32, value: Smi, tag: SeqCstAccessTag) {}
    fn swap(&self, index: i32, value: i32, tag: SeqCstAccessTag, mode: WriteBarrierMode) -> i32 {
        0
    }
    fn compare_and_swap(&self, index: i32, expected: i32, value: i32, tag: SeqCstAccessTag, mode: WriteBarrierMode) -> i32 {
        0
    }
}
impl FixedArray {
    fn New(isolate: &Isolate, capacity: i32, allocation: AllocationType) -> Handle<FixedArray> {
        if capacity as u32 > FixedArrayBase::kMaxLength as u32 {
            panic!("Fatal JavaScript invalid size error {}", capacity);
        } else if capacity == 0 {
            return isolate.factory().empty_fixed_array();
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<FixedArray, FixedArray, FixedArray>::Allocate(
            isolate,
            capacity,
            &mut no_gc,
            allocation,
        );
        let roots = ReadOnlyRoots {  };
        unsafe {MemsetTagged((*result).RawFieldOfFirstElement(), roots.undefined_value(), capacity)};
        return result;
    }
    fn is_the_hole(isolate: &Isolate, index: i32) -> bool {
        true
    }
    fn set_the_hole(isolate: &Isolate, index: i32) {}
    fn set_the_hole_ro(roots: ReadOnlyRoots, index: i32) {}
    fn FillWithHoles(&self, from: i32, to: i32) {}
    fn MoveElements(&self, isolate: &Isolate, dst_index: i32, src_index: i32, len: i32, mode: WriteBarrierMode) {}
    fn CopyElements(&self, isolate: &Isolate, dst_index: i32, src: Tagged<FixedArray>, src_index: i32, len: i32, mode: WriteBarrierMode) {}
    fn Resize(isolate: &Isolate, xs: DirectHandle<FixedArray>, new_capacity: i32, allocation: AllocationType, mode: WriteBarrierMode) -> Handle<FixedArray> {
        let ys = FixedArray::New(isolate, new_capacity, allocation);
        let elements_to_copy = std::cmp::min(new_capacity, xs.dummy);
        FixedArray::CopyElements(isolate, *ys, 0, *UNCHECKED_CAST(xs), 0, elements_to_copy, mode);
        return ys;
    }
    fn GetReadOnlyRoots(&self) -> ReadOnlyRoots {
        ReadOnlyRoots {}
    }
}
unsafe fn MemsetTagged(dst: i32, value: TaggedObject, count: i32) {}
impl TrustedFixedArray {
    fn New(isolate: &Isolate, capacity: i32, allocation: AllocationType) -> Handle<TrustedFixedArray> {
        if capacity as u32 > TrustedFixedArray::kMaxLength as u32 {
            panic!("Fatal JavaScript invalid size error {}", capacity);
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<TrustedFixedArray, TrustedFixedArray, TrustedFixedArray>::Allocate(
            isolate,
            capacity,
            &mut no_gc,
            allocation,
        );
        unsafe {MemsetTagged((*result).RawFieldOfFirstElement(), Smi::zero(), capacity)};
        return result;
    }
}
impl ProtectedFixedArray {
    fn New(isolate: &Isolate, capacity: i32) -> Handle<ProtectedFixedArray> {
        if capacity as u32 > ProtectedFixedArray::kMaxLength as u32 {
            panic!("Fatal JavaScript invalid size error {}", capacity);
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<ProtectedFixedArray, ProtectedFixedArray, ProtectedFixedArray>::Allocate(
            isolate,
            capacity,
            &mut no_gc,
            AllocationType::kTrusted,
        );
        unsafe {MemsetTagged((*result).RawFieldOfFirstElement(), Smi::zero(), capacity)};
        return result;
    }
}
impl WeakArrayList {
    fn Get(&self, index: i32) -> i32 {
        0
    }
    fn get(&self, index: i32) -> i32 {
        0
    }
    fn GetCage(&self, cage_base: PtrComprCageBase, index: i32) -> i32 {
        0
    }
    fn Set(&self, index: i32, value: i32, mode: WriteBarrierMode) {}
    fn SetSmi(&self, index: i32, value: Smi) {}
    fn CopyElements(&self, isolate: &Isolate, dst_index: i32, src: Tagged<WeakArrayList>, src_index: i32, len: i32, mode: WriteBarrierMode) {}
    fn objects(&self, cage_base: PtrComprCageBase, index: i32, mode: i32) -> Tagged<MaybeObject> {
        Tagged{dummy:0}
    }
    fn set_objects(&self, index: i32, value: Tagged<MaybeObject>, mode: i32, mode2: WriteBarrierMode) {}
}
impl WeakArrayList {
    struct Iterator {
        array_: WeakArrayList,
        index_: i32,
    }
    fn Next(&mut self) -> Tagged<HeapObject> {
        Tagged{dummy: 0}
    }
}
impl ArrayList {
    fn New(isolate: &Isolate, capacity: i32, allocation: AllocationType) -> DirectHandle<ArrayList> {
        if capacity == 0 {
            return isolate.factory().empty_array_list();
        }
        if capacity > ArrayList::kMaxCapacity {
            panic!("Fatal JavaScript invalid size error {}", capacity);
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<ArrayList, ArrayList, ArrayList>::Allocate(
            isolate,
            capacity,
            &mut no_gc,
            allocation,
        );
        let obj = UNCHECKED_CAST::<ArrayList>(*result);
        obj.set_length(0);
        let roots = ReadOnlyRoots {  };
        unsafe {MemsetTagged(obj.RawFieldOfFirstElement(), roots.undefined_value(), capacity)};
        return UNCHECKED_CAST::<DirectHandle<ArrayList>>(*result);
    }
    fn RawFieldOfFirstElement(&self) -> i32 {
        0
    }
}
impl ByteArray {
    fn New(isolate: &Isolate, length: i32, allocation: AllocationType) -> Handle<ByteArray> {
        if length as u32 > ByteArray::kMaxLength as u32 {
            panic!("Fatal JavaScript invalid size error {}", length);
        } else if length == 0 {
            return isolate.factory().empty_byte_array();
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<ByteArray, ByteArray, ByteArray>::Allocate(
            isolate,
            length,
            &mut no_gc,
            allocation,
        );
        let padding_size = SizeFor(length) - ByteArray::OffsetOfElementAt(length);
        unsafe {
            let ptr = (*result).values().as_ptr();
            let padding = &mut (*(ptr.add(length) as *mut [u8; 0]));
            std::ptr::write_bytes(padding as *mut [u8; 0], 0, padding_size as usize);
        }
        return result;
    }
    fn get_int(&self, offset: i32) -> u32 {
        0
    }
    fn set_int(&self, offset: i32, value: u32) {}
    fn IsInBounds(&self, offset: i32) -> bool {
        true
    }
}
impl TrustedByteArray {
    fn New(isolate: &Isolate, length: i32, allocation_type: AllocationType) -> Handle<TrustedByteArray> {
        if allocation_type != AllocationType::kTrusted && allocation_type != AllocationType::kSharedTrusted {
            panic!("Allocation type must be kTrusted or kSharedTrusted");
        }
        if length as u32 > TrustedByteArray::kMaxLength as u32 {
            panic!("Fatal JavaScript invalid size error {}", length);
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<TrustedByteArray, TrustedByteArray, TrustedByteArray>::Allocate(
            isolate,
            length,
            &mut no_gc,
            allocation_type,
        );
        let padding_size = SizeFor(length) - TrustedByteArray::OffsetOfElementAt(length);
        unsafe {
            let ptr = (*result).values().as_ptr();
            let padding = &mut (*(ptr.add(length) as *mut [u8; 0]));
            std::ptr::write_bytes(padding as *mut [u8; 0], 0, padding_size as usize);
        }
        return result;
    }
    fn get_int(&self, offset: i32) -> u32 {
        0
    }
    fn set_int(&self, offset: i32, value: u32) {}
    fn IsInBounds(&self, offset: i32) -> bool {
        true
    }
}
impl<T, Base> FixedIntegerArrayBase<T, Base> {
    fn get_element_address(&self, index: i32) -> Address {
        Address {}
    }
    fn get(&self, index: i32) -> T {
        0 as T
    }
    fn set(&self, index: i32, value: T) {}
}
impl<Base> FixedAddressArrayBase<Base> {
    fn get_sandboxed_pointer(&self, index: i32) -> Address {
        Address {}
    }
    fn set_sandboxed_pointer(&self, index: i32, value: Address) {}
}
impl<T> PodArray<T> {
    fn New(isolate: &Isolate, length: i32, allocation: AllocationType) -> Handle<PodArray<T>> {
        let byte_length = length * std::mem::size_of::<T>() as i32;
        if length < 0 || byte_length < 0 || (length > 0 && byte_length / length != std::mem::size_of::<T>() as i32) {
          panic!("SignedMulOverflow32");
        }
        TaggedArrayBase::<PodArray<T>, PodArray<T>, PodArray<T>>::Allocate(isolate, length, &mut None, allocation);
        isolate.factory().NewByteArray(byte_length, allocation);
        Handle { dummy: 0 }
    }
}
impl<T> TrustedPodArray<T> {
    fn New(isolate: &Isolate, length: i32) -> DirectHandle<TrustedPodArray<T>> {
        let byte_length = length * std::mem::size_of::<T>() as i32;
        if length < 0 || byte_length < 0 || (length > 0 && byte_length / length != std::mem::size_of::<T>() as i32) {
          panic!("SignedMulOverflow32");
        }
        TaggedArrayBase::<TrustedPodArray<T>, TrustedPodArray<T>, TrustedPodArray<T>>::Allocate(isolate, length, &mut None, AllocationType::kTrusted);
        isolate.factory().NewTrustedByteArray(byte_length);
        DirectHandle { dummy: 0 }
    }
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    fn Allocate(isolate: &Isolate, capacity: i32, no_gc_out: &mut Option<DisallowGarbageCollection>, allocation: AllocationType) -> Handle<D> {
        let xs = isolate.factory().AllocateRawArray(SizeFor(capacity), allocation);
        if no_gc_out.is_none() && DEBUG_BOOL {
            no_gc_out.replace(DisallowGarbageCollection {});
        }
        let roots = ReadOnlyRoots {  };
        let map = Map { dummy: 0 };
        unsafe {
            let xs: Tagged<D> = Tagged { dummy: 0 };
            let d: *const D = &xs as *const D;
            println!("set_map_after_allocation is unsafe, this code is UNTESTED");
        }
        let obj : Handle<D> = Handle{ dummy: 0 };
        obj
    }
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
    const fn NewCapacityForIndex(index: i32, old_capacity: i32) -> i32 {
        let mut capacity = old_capacity;
        while capacity <= index {
            capacity = capacity + (capacity >> 1) + 16;
        }
        return capacity;
    }
}
impl FixedArray {
    fn is_the_hole(self, index: i32) -> bool {
        true
    }
    fn set_the_hole(self, index: i32) {}
    fn MoveElements(&self, isolate: &Isolate, dst_index: i32, src_index: i32, len: i32, mode: WriteBarrierMode) {}
    fn CopyElements(&self, isolate: &Isolate, dst_index: i32, src: FixedArray, src_index: i32, len: i32, mode: WriteBarrierMode) {}
}
impl FixedDoubleArray {
    fn get_scalar(&self, index: i32) -> f64 {
        0.0
    }
    fn get(&self, array: Tagged<FixedDoubleArray>, index: i32, isolate: &Isolate) -> Handle<Object> {
        if array.dummy == 0 && index == 0 {
          return isolate.factory().the_hole_value();
        }
        return isolate.factory().NewNumber(1.0)
    }
    fn set(&self, index: i32, value: f64) {}
    fn set_the_hole(&self, isolate: &Isolate, index: i32) {}
    fn set_the_hole(self, index: i32) {}
    fn is_the_hole(self, isolate: &Isolate, index: i32) -> bool {
        false
    }
    fn is_the_hole(self, index: i32) -> bool {
        false
    }
    fn MoveElements(&self, isolate: &Isolate, dst_index: i32, src_index: i32, len: i32, mode: WriteBarrierMode) {}
    fn FillWithHoles(&self, from: i32, to: i32) {}
}
impl FixedArrayBase {
    const kMaxLength: i32 = 1024 * 1024;
}
impl TrustedFixedArray {
    const kMaxLength: i32 = 1024 * 1024;
}
impl ProtectedFixedArray {
    const kMaxLength: i32 = 1024 * 1024;
}
impl FixedDoubleArray {
    const kMaxLength: i32 = 1024 * 1024;
    const kElementSize: usize = std::mem::size_of::<Double>();
}
impl ByteArray {
    const kMaxLength: i32 = 1024 * 1024;
    fn OffsetOfElementAt(index: i32) -> i32 {
        index
    }
}
impl TrustedByteArray {
    const kMaxLength: i32 = 1024 * 1024;
    fn OffsetOfElementAt(index: i32) -> i32 {
        index
    }
}
impl ArrayList {
    const kMaxCapacity: i32 = 1024 * 1024;
}
impl<D, S, P> TaggedArrayBase<D, S, P> {
  fn AllocatedSize(&self) -> i32 {
    SizeFor(self.capacity_acquire())
  }
    type SlotType = i32;
    type ElementT = i32;
    fn RawFieldOfFirstElement(&self) -> Self::SlotType {
        self.RawFieldOfElementAt(0)
    }
    fn RawFieldOfElementAt(&self, index: i32) -> Self::SlotType {
        0
    }
}
impl WeakFixedArray {
    fn New(isolate: &Isolate, capacity: i32, allocation: AllocationType, initial_value: MaybeDirectHandle<Object>) -> Handle<WeakFixedArray> {
        if capacity as u32 > WeakFixedArray::kMaxCapacity as u32 {
            panic!("Fatal JavaScript invalid size error {}", capacity);
        }
        if capacity == 0 {
            return isolate.factory().empty_weak_fixed_array();
        }
        let mut no_gc: Option<DisallowGarbageCollection> = None;
        let result = TaggedArrayBase::<WeakFixedArray, WeakFixedArray, WeakFixedArray>::Allocate(
            isolate,
            capacity,
            &mut no_gc,
            allocation,
        );
        let roots = ReadOnlyRoots {  };
        let initial = if initial_value.is_null() { roots.undefined_value() } else {*initial_value.ToHandleChecked()};
        unsafe {MemsetTagged((*result).RawFieldOfFirstElement(), initial, capacity)};
        return result;
    }
}
impl TrustedWeakFixedArray {
    fn New(isolate: &Isolate, capacity: i32) -> Handle<TrustedWeakFixedArray> {
        if capacity as u32 > TrustedFixedArray::kMaxLength as
