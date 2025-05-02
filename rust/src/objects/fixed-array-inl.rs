// src/objects/fixed_array.rs

// Note: This is a simplified translation and may require adjustments to fully
// integrate with a larger Rust codebase.  Some V8-specific functionalities
// like `Isolate`, `Handle`, garbage collection, write barriers, roots, and
// allocation strategies are represented by placeholders.

// use std::mem;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{marker::PhantomData, mem};

macro_rules! DCHECK {
    ($cond:expr) => {
        if !$cond {
            panic!("DCHECK failed: {}", stringify!($cond));
        }
    };
}

macro_rules! CHECK_GT {
    ($a:expr, $b:expr) => {
        if !($a > $b) {
            panic!("CHECK_GT failed: {} > {}", $a, $b);
        }
    };
}

macro_rules! CHECK_GE {
    ($a:expr, $b:expr) => {
        if !($a >= $b) {
            panic!("CHECK_GE failed: {} >= {}", $a, $b);
        }
    };
}

macro_rules! CHECK_LE {
    ($a:expr, $b:expr) => {
        if !($a <= $b) {
            panic!("CHECK_LE failed: {} <= {}", $a, $b);
        }
    };
}

macro_rules! CHECK_EQ {
    ($a:expr, $b:expr) => {
        if !($a == $b) {
            panic!("CHECK_EQ failed: {} == {}", $a, $b);
        }
    };
}

const DEBUG_BOOL: bool = true;

// Placeholders for V8 types and functions
type Address = usize;
type Isolate /*<T>*/ = ();
type LocalIsolate /*<T>*/ = ();
type Handle<T> = Box<T>;
type DirectHandle<T> = Box<T>;
type MaybeDirectHandle<T> = Option<Box<T>>;
type Tagged<T> = T;
type Object = usize;
type HeapObject = usize;
type MaybeObject = usize;
type Map = usize;
type Smi = i32;
type SeqCstAccessTag = ();
type AcquireLoadTag = ();
type ReleaseStoreTag = ();
type RelaxedLoadTag = ();
type RelaxedStoreTag = ();
type PtrComprCageBase = usize;
type ByteArray = [u8];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WriteBarrierMode {
    // TODO: Add more write barrier modes based on v8's WriteBarrierMode
    SkipWriteBarrier,
}

#[derive(Clone, Copy, Debug)]
enum AllocationType {
    // TODO: Add more allocation types based on v8's AllocationType
    Normal,
    Trusted,
    SharedTrusted,
}

// Placeholder for SizeFor function.
fn size_for(capacity: usize) -> usize {
    // Dummy implementation. Should calculate the size based on capacity and element size.
    capacity * 8 // Assuming 8 bytes per element for now.
}

// Placeholder for object pointer alignment
fn object_pointer_align(size: usize) -> usize {
    // Dummy implementation.
    (size + 7) & !7
}

fn unchecked_cast<T>(x: usize) -> T {
    unsafe { mem::transmute_copy(&x) }
}

fn read_sandboxed_pointer_field(_address: Address, _sandbox_base: PtrComprCageBase) -> Address {
    // Placeholder.  Implement the actual sandboxed pointer read.
    0
}

fn write_sandboxed_pointer_field(_address: Address, _sandbox_base: PtrComprCageBase, _value: Address) {
    // Placeholder. Implement the actual sandboxed pointer write.
}

#[allow(dead_code)]
fn get_ptr_compr_cage_base<T>(_obj: &T) -> PtrComprCageBase {
    // Placeholder
    0
}

// Placeholder for Memory operations
#[allow(dead_code)]
mod memory {
    pub fn memset_tagged<T>(_dst: *mut T, _value: T, _count: usize) {
        // Placeholder
    }

    pub fn memmove<T>(_dst: *mut T, _src: *const T, _count: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(_src, _dst, _count);
        }
    }
}

#[allow(dead_code)]
fn memmove<T>(dst: *mut T, src: *const T, count: usize) {
    memory::memmove(dst, src, count);
}

// Placeholder for ReadOnlyRoots
#[allow(dead_code)]
struct ReadOnlyRoots {
    undefined_value: usize,
    the_hole_value: usize,
}

#[allow(dead_code)]
impl ReadOnlyRoots {
    fn new(_isolate: &Isolate) -> Self {
        ReadOnlyRoots {
            undefined_value: 0,
            the_hole_value: 1,
        }
    }
    fn object_at(&self, _index: usize) -> usize {
        // Placeholder implementation
        0
    }
}

// Placeholder for EarlyGetReadOnlyRoots
#[allow(dead_code)]
trait EarlyGetReadOnlyRoots {
    fn unchecked_fixed_cow_array_map(&self) -> usize;
}

// Placeholder for Heap
#[allow(dead_code)]
struct Heap {}

#[allow(dead_code)]
impl Heap {
    fn move_range<T>(
        &self,
        _owner: Tagged<T>,
        _dst_slot: usize,
        _src_slot: usize,
        _len: usize,
        _mode: WriteBarrierMode,
    ) {
        // Placeholder
    }
    fn copy_range<T>(
        &self,
        _owner: Tagged<T>,
        _dst_slot: usize,
        _src_slot: usize,
        _len: usize,
        _mode: WriteBarrierMode,
    ) {
        // Placeholder
    }
    fn right_trim_array<T>(&self, _array: Tagged<T>, _new_capacity: usize, _old_capacity: usize) {
        // Placeholder
    }
}

// Placeholder for factory
#[allow(dead_code)]
struct Factory {}

#[allow(dead_code)]
impl Factory {
    fn allocate_raw_array(&self, _size: usize, _allocation: AllocationType) -> usize {
        // Placeholder.  Return raw memory address.
        0
    }

    fn empty_fixed_array(&self) -> Handle<FixedArray> {
        // Placeholder
        Box::new(FixedArray {
            map: 0,
            capacity_: AtomicI32::new(0),
            objects: Vec::new(),
            _phantom: PhantomData,
        })
    }

    fn empty_array_list(&self) -> DirectHandle<ArrayList> {
        // Placeholder
        Box::new(ArrayList {
            map: 0,
            length_: AtomicI32::new(0),
            objects: Vec::new(),
            _phantom: PhantomData,
        })
    }

    fn empty_byte_array(&self) -> Handle<ByteArray> {
        // Placeholder
        Box::new([])
    }

    fn new_trusted_byte_array(&self, _size: usize) -> Handle<TrustedByteArray> {
        // Placeholder
        Box::new([])
    }

    fn the_hole_value(&self) -> Handle<Object> {
        // Placeholder
        Box::new(0)
    }

    fn undefined_value(&self) -> Handle<Object> {
        // Placeholder
        Box::new(1)
    }

    fn new_number(&self, _value: f64) -> Handle<Object> {
        // Placeholder
        Box::new(2)
    }
}

// Placeholder for isolate.heap()
#[allow(dead_code)]
trait IsolateHeap {
    fn heap(&self) -> Heap;
}

#[allow(dead_code)]
impl IsolateHeap for Isolate {
    fn heap(&self) -> Heap {
        Heap {}
    }
}

// Placeholder for isolate.factory()
#[allow(dead_code)]
trait IsolateFactory {
    fn factory(&self) -> Factory;
}

#[allow(dead_code)]
impl IsolateFactory for Isolate {
    fn factory(&self) -> Factory {
        Factory {}
    }
}

// Placeholder for MemsetTagged
#[allow(dead_code)]
fn memset_tagged<T>(dst: *mut T, value: Tagged<T>, count: usize) {
    memory::memset_tagged(dst, value, count);
}

// Dummy implementation for Sandbox
#[allow(dead_code)]
mod sandbox {
    pub type SandboxedPointer = usize; // Placeholder
}

//Dummy implementation for base::bits
#[allow(dead_code)]
mod base {
    pub mod bits {
        pub fn signed_mul_overflow32(_a: i32, _b: i32, _result: &mut i32) -> bool {
            *_result = _a.wrapping_mul(_b);
            false // Indicate no overflow for simplicity
        }
    }

    pub fn read_unaligned_value<T>(ptr: Address) -> T {
        unsafe { std::ptr::read_unaligned(ptr as *const T) }
    }

    pub fn write_unaligned_value<T>(ptr: Address, value: T) {
        unsafe { std::ptr::write_unaligned(ptr as *mut T, value) }
    }
}

// --- ArrayHeaderBase ---
mod detail {
    use super::*;

    pub struct ArrayHeaderBase<S, LEN> {
        capacity_: AtomicI32,
        length_: AtomicI32,
        _phantom: PhantomData<(S, LEN)>,
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

// --- TaggedArrayBase ---
struct TaggedArrayBase<D, S, P> {
    map: Map,
    header: detail::ArrayHeaderBase<S, false>,
    objects: Vec<Object>, // Simplified.  Use a proper memory management strategy in production.
    _phantom: PhantomData<(D, S, P)>,
}

impl<D, S, P> TaggedArrayBase<D, S, P> {
    const K_MAX_CAPACITY: usize = 1024; // Dummy value
    type ElementT = Object;
    type SlotType = *mut Object;

    fn is_in_bounds(&self, index: usize) -> bool {
        (index as i32) < self.header.capacity()
    }

    fn is_cow_array(&self) -> bool {
        self.map == 1 // Dummy value
                      //self.map() == self.EarlyGetReadOnlyRoots().unchecked_fixed_cow_array_map()
    }

    fn get(&self, index: usize) -> Tagged<Object> {
        DCHECK!(self.is_in_bounds(index));
        self.objects[index] // Simplified
    }

    fn get_relaxed(&self, index: usize) -> Tagged<Object> {
        DCHECK!(self.is_in_bounds(index));
        self.objects[index]
    }

    fn get_acquire(&self, index: usize) -> Tagged<Object> {
        DCHECK!(self.is_in_bounds(index));
        self.objects[index]
    }

    fn get_seq_cst(&self, index: usize) -> Tagged<Object> {
        DCHECK!(self.is_in_bounds(index));
        self.objects[index]
    }

    fn set(&mut self, index: usize, value: Tagged<Object>, mode: WriteBarrierMode) {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        self.objects[index] = value;
    }

    fn set_relaxed(&mut self, index: usize, value: Tagged<Object>, mode: WriteBarrierMode) {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        self.objects[index] = value;
    }

    fn set_release(&mut self, index: usize, value: Tagged<Object>, mode: WriteBarrierMode) {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        self.objects[index] = value;
    }

    fn set_seq_cst(&mut self, index: usize, value: Tagged<Object>, mode: WriteBarrierMode) {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        self.objects[index] = value;
    }

    fn swap(&mut self, index: usize, value: Tagged<Object>, _tag: SeqCstAccessTag, mode: WriteBarrierMode) -> Tagged<Object> {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        let old_value = self.objects[index];
        self.objects[index] = value;
        old_value
    }

    fn compare_and_swap(&mut self, index: usize, expected: Tagged<Object>, value: Tagged<Object>, _tag: SeqCstAccessTag, mode: WriteBarrierMode) -> Tagged<Object> {
        DCHECK!(!self.is_cow_array());
        DCHECK!(self.is_in_bounds(index));
        if self.objects[index] == expected {
            let old_value = self.objects[index];
            self.objects[index] = value;
            old_value
        } else {
            self.objects[index]
        }
    }

    fn move_elements(
        _isolate: &Isolate,
        dst: &mut TaggedArrayBase<D, S, P>,
        dst_index: usize,
        src: &mut TaggedArrayBase<D, S, P>,
        src_index: usize,
        len: usize,
        mode: WriteBarrierMode,
    ) {
        if len == 0 {
            return;
        }

        DCHECK_GE!(len as i32, 0);
        DCHECK!(dst.is_in_bounds(dst_index));
        DCHECK_LE!((dst_index + len) as i32, dst.header.capacity());
        DCHECK!(src.is_in_bounds(src_index));
        DCHECK_LE!((src_index + len) as i32, src.header.capacity());
        //DisallowGarbageCollection no_gc;
        //SlotType dst_slot(&dst->objects()[dst_index]);
        //SlotType src_slot(&src->objects()[src_index]);
        //isolate->heap()->MoveRange(dst, dst_slot, src_slot, len, mode);
        // This is a placeholder. A real implementation will use a heap.
        unsafe {
            std::ptr::copy(src.objects.as_ptr().add(src_index),
                dst.objects.as_mut_ptr().add(dst_index),
                len);
        }
    }

    fn copy_elements(
        _isolate: &Isolate,
        dst: &mut TaggedArrayBase<D, S, P>,
        dst_index: usize,
        src: &mut TaggedArrayBase<D, S, P>,
        src_index: usize,
        len: usize,
        mode: WriteBarrierMode,
    ) {
        if len == 0 {
            return;
        }

        DCHECK_GE!(len as i32, 0);
        DCHECK!(dst.is_in_bounds(dst_index));
        DCHECK_LE!((dst_index + len) as i32, dst.header.capacity());
        DCHECK!(src.is_in_bounds(src_index));
        DCHECK_LE!((src_index + len) as i32, src.header.capacity());

        //DisallowGarbageCollection no_gc;
        //SlotType dst_slot(&dst->objects()[dst_index]);
        //SlotType src_slot(&src->objects()[src_index]);
        //isolate->heap()->CopyRange(dst, dst_slot, src_slot, len, mode);
        unsafe {
            std::ptr::copy_nonoverlapping(src.objects.as_ptr().add(src_index),
                                          dst.objects.as_mut_ptr().add(dst_index),
                                          len);
        }
    }

    fn right_trim(_isolate: &Isolate, new_capacity: usize) {
        // Placeholder.  Needs isolate and heap interaction.
    }

    fn allocated_size(&self) -> usize {
        size_for(self.header.capacity_acquire() as usize)
    }

    fn raw_field_of_first_element(&self) -> Self::SlotType {
        self.raw_field_of_element_at(0)
    }

    fn raw_field_of_element_at(&self, index: usize) -> Self::SlotType {
        &mut self.objects[index] as *mut Object
    }

    fn allocate<IsolateT>(
        isolate: &IsolateT,
        capacity: usize,
    ) -> Handle<FixedArray>
    where IsolateT: IsolateFactory {
        TaggedArrayBase::<FixedArray, Object, Object>::allocate_with_type(isolate, capacity, AllocationType::Normal)
    }

    fn allocate_with_type<IsolateT>(_isolate: &IsolateT, capacity: usize, _allocation: AllocationType) -> Handle<D>
    where IsolateT: IsolateFactory
    {
        if capacity > Self::K_MAX_CAPACITY {
            panic!(
                "Fatal JavaScript invalid size error {} (see crbug.com/1201626)",
                capacity
            );
        }
        DCHECK_GE!(capacity as i32, 0);
        DCHECK_LE!(capacity, Self::K_MAX_CAPACITY);
        //DCHECK(!no_gc_out.has_value());

        let xs = 0;//UncheckedCast::<D>(isolate.factory().AllocateRawArray(SizeFor(capacity), allocation));

        //ReadOnlyRoots roots{isolate};
        //if (DEBUG_BOOL) no_gc_out.emplace();
        //Tagged<Map> map = Cast<Map>(roots.object_at(S::kMapRootIndex));
        //DCHECK(ReadOnlyHeap::Contains(map));
        //xs.set_map_after_allocation(isolate, map, SKIP_WRITE_BARRIER);
        //xs.set_capacity(capacity);

        let mut array = TaggedArrayBase::<FixedArray, Object, Object> {
            map: 0,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(capacity as i32),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: vec![0; capacity],
            _phantom: PhantomData,
        };
        Box::new(FixedArray {
            map: 0,
            capacity_: AtomicI32::new(capacity as i32),
            objects: vec![0; capacity],
            _phantom: PhantomData,
        })
    }

    const fn new_capacity_for_index(index: usize, old_capacity: usize) -> usize {
        DCHECK_GE!(index as i32, old_capacity as i32);

        let mut capacity = old_capacity;
        loop {
            capacity = capacity + (capacity >> 1) + 16;
            if capacity > index {
                break;
            }
        }
        capacity
    }
}

// --- FixedArray ---
struct FixedArray {
    map: Map,
    capacity_: AtomicI32,
    objects: Vec<Object>,
    _phantom: PhantomData<Object>,
}

impl FixedArray {
    const K_MAX_LENGTH: usize = 1024; // Dummy value

    fn new<IsolateT>(isolate: &IsolateT, capacity: usize) -> Handle<Self>
    where IsolateT: IsolateFactory
    {
       Self::new_with_allocation(isolate, capacity, AllocationType::Normal)
    }

    fn new_with_allocation<IsolateT>(isolate: &IsolateT, capacity: usize, allocation: AllocationType) -> Handle<Self>
    where IsolateT: IsolateFactory
    {
        if capacity > Self::K_MAX_LENGTH {
            panic!(
                "Fatal JavaScript invalid size error {} (see crbug.com/1201626)",
                capacity
            );
        } else if capacity == 0 {
            return isolate.factory().empty_fixed_array();
        }

        //std::optional::DisallowGarbageCollection no_gc;
        let mut result = TaggedArrayBase::<FixedArray, Object, Object>::allocate_with_type(isolate, capacity, allocation);
        let roots = ReadOnlyRoots::new(&());// isolate
        memset_tagged(
            (&mut result.objects[0]) as *mut Object,
            roots.undefined_value,
            capacity,
        );
        result
    }

    fn is_the_hole(&self, _isolate: &Isolate, index: usize) -> bool {
        self.objects[index] == 2 // Placeholder for the_hole_value.
    }

    fn set_the_hole(&mut self, isolate: &Isolate, index: usize) {
        let roots = ReadOnlyRoots::new(isolate);
        self.objects[index] = roots.the_hole_value;
    }

    fn fill_with_holes(&mut self, from: usize, to: usize) {
        let roots = ReadOnlyRoots::new(&()); // Placeholder Isolate
        for i in from..to {
            self.objects[i] = roots.the_hole_value;
        }
    }

    fn move_elements(&mut self, isolate: &Isolate, dst_index: usize, src_index: usize, len: usize, mode: WriteBarrierMode) {
        let mut temp: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: 0,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(self.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: self.objects.clone(),
            _phantom: PhantomData,
        };

        let mut this: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: self.map,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(self.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: self.objects.clone(),
            _phantom: PhantomData,
        };
        TaggedArrayBase::move_elements(isolate, &mut this, dst_index, &mut temp, src_index, len, mode)
    }

    fn copy_elements(
        &mut self,
        isolate: &Isolate,
        dst_index: usize,
        src: &FixedArray,
        src_index: usize,
        len: usize,
        mode: WriteBarrierMode,
    ) {
       let mut temp: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: 0,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(src.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: src.objects.clone(),
            _phantom: PhantomData,
        };

        let mut this: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: self.map,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(self.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: self.objects.clone(),
            _phantom: PhantomData,
        };

        TaggedArrayBase::copy_elements(isolate, &mut this, dst_index, &mut temp, src_index, len, mode);
    }

    fn resize(
        isolate: &Isolate,
        xs: DirectHandle<FixedArray>,
        new_capacity: usize,
        allocation: AllocationType,
        mode: WriteBarrierMode,
    ) -> Handle<FixedArray> {
        let mut ys = FixedArray::new_with_allocation(isolate, new_capacity, allocation);
        let elements_to_copy = std::cmp::min(new_capacity, xs.objects.len());

        let mut temp: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: 0,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(xs.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: xs.objects.clone(),
            _phantom: PhantomData,
        };

        let mut this: TaggedArrayBase<FixedArray, Object, Object> = TaggedArrayBase {
            map: ys.map,
            header: detail::ArrayHeaderBase {
                capacity_: AtomicI32::new(ys.capacity_.load(Ordering::Relaxed)),
                length_: AtomicI32::new(0),
                _phantom: PhantomData,
            },
            objects: ys.objects.clone(),
            _phantom: PhantomData,
        };

        TaggedArrayBase::copy_elements(isolate, &mut this, 0, &mut temp, 0, elements_to_copy, mode);

        ys
    }
}

// --- TrustedFixedArray ---
struct TrustedFixedArray {
    map: Map,
    capacity_: AtomicI32,
    objects: Vec<Smi>,
    _phantom: PhantomData<Smi>,
}

impl TrustedFixedArray {
    const K_MAX_LENGTH: usize = 1024; // Dummy value

    fn new<IsolateT>(isolate: &IsolateT, capacity: usize) -> Handle<Self>
    where IsolateT: IsolateFactory {
        Self::new_with_allocation(isolate, capacity, AllocationType::Normal)
    }

    fn new_with_allocation<IsolateT>(isolate: &IsolateT, capacity: usize, allocation: AllocationType) -> Handle<Self>
    where IsolateT: IsolateFactory
    {
        DCHECK!(allocation == AllocationType::Trusted || allocation == AllocationType::SharedTrusted);

        if capacity > Self::K_MAX_LENGTH {
            panic!(
                "Fatal JavaScript invalid size error {} (see crbug.com/1201626)",
                capacity
            );
        }

        //std::optional::DisallowGarbageCollection no_gc;
        //Handle<TrustedFixedArray> result = Cast<TrustedFixedArray>(Allocate(isolate, capacity, &no_gc, allocation));
        //MemsetTagged((*result)->RawFieldOfFirstElement(), Smi::zero(), capacity);

        let mut result = Box::new(TrustedFixedArray {
            map: 0,
            capacity_: AtomicI32::new(capacity as i32),
            objects: vec![0; capacity],
            _phantom: PhantomData,
        });

        memset_tagged(
            (&mut result.objects[0]) as *mut Smi,
            0,
            capacity,
        );
        result
    }
}

// --- ProtectedFixedArray ---
struct ProtectedFixedArray {
    map: Map,
    capacity_: AtomicI32,
    objects: Vec<Smi>,
    _phantom: PhantomData<Smi>,
}

impl ProtectedFixedArray {
    const K_MAX_LENGTH: usize = 1024; // Dummy value

    fn new<IsolateT>(isolate: &IsolateT, capacity: usize) -> Handle<Self>
    where IsolateT: IsolateFactory {
       if capacity > Self::K_MAX_LENGTH {
            panic!(
                "Fatal JavaScript invalid size error {} (see crbug.com/1201626)",
                capacity
            );
        }

        //std::optional::DisallowGarbageCollection no_gc;
        //Handle<TrustedFixedArray> result = Cast<TrustedFixedArray>(Allocate(isolate, capacity, &no_gc, allocation));
        //MemsetTagged((*result)->RawFieldOfFirstElement(), Smi::zero(), capacity);

        let mut result = Box::new(ProtectedFixedArray {
            map: 0,
            capacity_: AtomicI32::new(capacity as i32),
            objects: vec![0; capacity],
            _phantom: PhantomData,
        });

        memset_tagged(
            (&mut result.objects[0]) as *mut Smi,
            0,
            capacity,
        );
        result
    }
}

// --- WeakArrayList ---
struct WeakArrayList {
    map: Map,
    length_: AtomicI32,
    objects: Vec<MaybeObject>,
    _phantom: PhantomData<MaybeObject>,
}

impl WeakArrayList {
    fn get(&self, index: usize) -> Tagged<MaybeObject> {
        let _cage_base = get_ptr_compr_cage_base(self);
        self.objects[index]
    }

    fn set(&mut self, index: usize, value: Tagged<MaybeObject>, mode: WriteBarrierMode) {
        self.objects[index] = value;
    }

    fn set_smi(&mut self, index: usize, value: Tagged<Smi>) {
        self.set(index, value as Tagged<MaybeObject>, WriteBarrierMode::SkipWriteBarrier);
    }

    fn data_start(&mut self) -> *mut MaybeObject {
        self.objects.as_mut_ptr()
    }

    fn copy_elements(
        &mut self,
        isolate: &Isolate,
        dst_index: usize,
        src: &WeakArrayList,
        src_index: usize,
        len: usize,
        mode: WriteBarrierMode,
    ) {
        if len == 0 {
            return;
        }
        let capacity = self.length_.load(Ordering::Relaxed) as usize;
        DCHECK_LE!(dst_index + len, capacity);

        let capacity = src.length_.load(Ordering::Relaxed) as usize;
        DCHECK_LE!(src_index + len, capacity);

        //DisallowGarbageCollection no_gc;

        //MaybeObjectSlot dst_slot(data_start() + dst_index);
        //MaybeObjectSlot src_slot(src->data_start() + src_index);
        //isolate->heap()->CopyRange(*this, dst_slot, src_slot, len, mode);
        unsafe {
            std::ptr::copy_nonoverlapping(src.objects.as_ptr().add(src_index),
                                          self.objects.as_mut_ptr().add(dst_index),
                                          len);
        }
    }

    fn allocated_size(&self) -> usize {
        let capacity = self.length_.load(Ordering::Acquire) as usize;
        size_for(capacity)
    }

    fn new<IsolateT>(isolate: &IsolateT, capacity: usize, allocation: AllocationType, _initial_value: MaybeDirectHandle<Object>) -> Handle<WeakArrayList>
    where IsolateT: IsolateFactory
    {
       TaggedArrayBase::<WeakArrayList, Object, Object>::allocate_with_type(isolate, capacity, allocation)
    }

    fn is_null(&self) -> bool {
        false
    }

    fn length(&self) -> i32 {
        self.length_.load(Ordering::Relaxed)
    }

    fn iterator(&self) -> WeakArrayListIterator {
        WeakArrayListIterator::new(self)
    }
}

struct WeakArrayListIterator<'a> {
    array_: &'a WeakArrayList,
    index_: usize,
}

impl<'a> WeakArrayListIterator<'a> {
    fn new(array_: &'a WeakArrayList) -> Self {
        Self {
            array_: array_,
            index_: 0,
        }
    }

    fn next(&mut self) -> HeapObject {
        if self.array_.is_null() {
            return 0;
        }

        