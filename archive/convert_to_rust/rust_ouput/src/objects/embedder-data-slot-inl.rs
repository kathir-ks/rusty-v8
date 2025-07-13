// Converted from V8 C++ source files:
// Header: embedder-data-slot-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::atomic::{AtomicUsize, Ordering};

struct SlotBase {
    address: usize,
}

impl SlotBase {
    fn new(address: usize) -> Self {
        SlotBase { address }
    }

    fn address(&self) -> usize {
        self.address
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HeapObject {}

impl HeapObject {
    fn address(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JSObject {}

impl JSObject {
    fn GetEmbedderFieldOffset(&self, _embedder_field_index: i32) -> i32 {
        0
    }
    fn address(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmbedderDataArray {}

impl EmbedderDataArray {
    fn OffsetOfElementAt(_entry_index: i32) -> i32 {
        0
    }
    fn address(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Object {}

#[derive(Debug, Clone, Copy)]
pub struct Smi {}

impl Smi {
    fn zero() -> Self {
        Smi {}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl Tagged<EmbedderDataArray> {
    fn ptr(&self) -> *mut EmbedderDataArray {
        std::ptr::null_mut()
    }
}

impl Tagged<JSObject> {
    fn address(&self) -> usize {
        0
    }
    fn ptr(&self) -> *mut JSObject {
        std::ptr::null_mut()
    }
}

impl Tagged<Object> {
    fn ptr(&self) -> *mut Object {
        std::ptr::null_mut()
    }
}

impl Tagged<Smi> {
    fn ptr(&self) -> *mut Smi {
        std::ptr::null_mut()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReadOnlyHeap {}

impl ReadOnlyHeap {
    fn Contains(_object: HeapObject) -> bool {
        false
    }
}

fn IsSmi<T>(_value: T) -> bool {
    false
}

fn Cast<T>(_value: Object) -> T {
    todo!()
}

pub struct ObjectSlot {
    address: usize,
}

impl ObjectSlot {
    fn new(address: usize) -> Self {
        ObjectSlot { address }
    }

    fn Relaxed_Store<T>(&self, _value: T) {}
    fn Relaxed_Load<T>(&self) -> T {
        todo!()
    }
}

struct V8HeapCompressionScheme {}

impl V8HeapCompressionScheme {
    fn GetPtrComprCageBaseAddress(_ptr: *mut EmbedderDataArray) -> usize {
        0
    }
    fn GetPtrComprCageBaseAddress_object(_ptr: *mut JSObject) -> usize {
        0
    }
}

macro_rules! CHECK {
    ($x:expr) => {
        assert!($x)
    };
}

macro_rules! WRITE_BARRIER {
    ($object:expr, $offset:expr, $value:expr) => {};
}

#[derive(Debug, Clone, Copy)]
pub struct IsolateForSandbox {}

#[derive(Debug, Clone, Copy)]
pub struct DisallowGarbageCollection {}

struct ExternalPointerHandle {}

const kNullExternalPointerHandle: i32 = 0;

struct AsAtomicTagged {}

impl AsAtomicTagged {
    fn Relaxed_Store(_address: *mut AtomicUsize, _value: Tagged_t) {}
}

type Address = usize;
type Tagged_t = usize;

const COMPRESS_POINTERS_BOOL: bool = false;
const HAS_SMI_TAG_BOOL: bool = false;

macro_rules! HAS_SMI_TAG {
    ($value:expr) => {
        HAS_SMI_TAG_BOOL
    };
}

const kSmiShiftSize: i32 = 0;

fn SmiValuesAre31Bits() -> bool {
    true
}

const kTaggedSize: i32 = 4;
const kInt32Size: i32 = 4;

mod base {
    pub fn ReadUnalignedValue<T>(_address: usize) -> T {
        todo!()
    }
    pub struct AsAtomic32 {}
    impl AsAtomic32 {
        pub fn Relaxed_Load(_location: *mut ExternalPointerHandle) -> i32 {
            0
        }
    }
}

mod v8 {
    pub mod internal {
        use super::*;

        const FIELD_ADDR_OFFSET: usize = 0;

        macro_rules! FIELD_ADDR {
            ($object:expr, $offset:expr) => {
                ($object.address() as usize) + ($offset as usize) + FIELD_ADDR_OFFSET
            };
        }

        pub(crate) use FIELD_ADDR;

        const kTaggedPayloadOffset: i32 = 0;
        const kRawPayloadOffset: i32 = 4;
        const kExternalPointerOffset: i32 = 8;
        const kEmbedderDataSlotPayloadTag: i32 = 0;

        pub struct EmbedderDataSlot {
            slot_base: SlotBase,
        }

        impl EmbedderDataSlot {
            pub fn new(array: Tagged<EmbedderDataArray>, entry_index: i32) -> Self {
                let address = FIELD_ADDR!(
                    array,
                    EmbedderDataArray::OffsetOfElementAt(entry_index)
                );
                EmbedderDataSlot {
                    slot_base: SlotBase::new(address),
                }
            }

            pub fn new_object(object: Tagged<JSObject>, embedder_field_index: i32) -> Self {
                let address = FIELD_ADDR!(
                    object,
                    object.GetEmbedderFieldOffset(embedder_field_index)
                );
                EmbedderDataSlot {
                    slot_base: SlotBase::new(address),
                }
            }

            fn address(&self) -> usize {
                self.slot_base.address()
            }

            pub fn Initialize(&mut self, initial_value: Tagged<Object>) {
                CHECK!(IsSmi(initial_value) || ReadOnlyHeap::Contains(Cast::<HeapObject>(initial_value)));
                ObjectSlot::new(self.address() + kTaggedPayloadOffset as usize).Relaxed_Store(initial_value);
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    ObjectSlot::new(self.address() + kRawPayloadOffset as usize).Relaxed_Store(Smi::zero());
                }
            }

            pub fn load_tagged(&self) -> Tagged<Object> {
                ObjectSlot::new(self.address() + kTaggedPayloadOffset as usize).Relaxed_Load()
            }

            pub fn store_smi(&self, value: Tagged<Smi>) {
                ObjectSlot::new(self.address() + kTaggedPayloadOffset as usize).Relaxed_Store(value);
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    ObjectSlot::new(self.address() + kRawPayloadOffset as usize).Relaxed_Store(Smi::zero());
                }
            }

            pub fn store_tagged(array: Tagged<EmbedderDataArray>, entry_index: i32, value: Tagged<Object>) {
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    CHECK!(IsSmi(value) ||
                        V8HeapCompressionScheme::GetPtrComprCageBaseAddress(value.ptr()) ==
                            V8HeapCompressionScheme::GetPtrComprCageBaseAddress(array.ptr()));
                }
                let slot_offset = EmbedderDataArray::OffsetOfElementAt(entry_index);
                ObjectSlot::new(FIELD_ADDR!(array, slot_offset + kTaggedPayloadOffset))
                    .Relaxed_Store(value);
                WRITE_BARRIER!(array, slot_offset + kTaggedPayloadOffset, value);

                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    ObjectSlot::new(FIELD_ADDR!(array, slot_offset + kRawPayloadOffset))
                        .Relaxed_Store(Smi::zero());
                }
            }

            pub fn store_tagged_object(object: Tagged<JSObject>, embedder_field_index: i32, value: Tagged<Object>) {
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    CHECK!(IsSmi(value) ||
                        V8HeapCompressionScheme::GetPtrComprCageBaseAddress(value.ptr()) ==
                            V8HeapCompressionScheme::GetPtrComprCageBaseAddress_object(object.ptr()));
                }
                let slot_offset = object.GetEmbedderFieldOffset(embedder_field_index);
                ObjectSlot::new(FIELD_ADDR!(object, slot_offset + kTaggedPayloadOffset))
                    .Relaxed_Store(value);
                WRITE_BARRIER!(object, slot_offset + kTaggedPayloadOffset, value);
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    ObjectSlot::new(FIELD_ADDR!(object, slot_offset + kRawPayloadOffset))
                        .Relaxed_Store(Smi::zero());
                }
            }

            pub fn ToAlignedPointer(&self, _isolate: IsolateForSandbox, out_pointer: &mut *mut std::ffi::c_void) -> bool {
                unsafe {
                    *out_pointer = self.address() as *mut std::ffi::c_void;
                }
                true
            }

            pub fn store_aligned_pointer(&self, _isolate: IsolateForSandbox, _host: Tagged<HeapObject>, ptr: *mut std::ffi::c_void) -> bool {
                let value = ptr as usize;
                if !HAS_SMI_TAG!(value) {
                    return false;
                }
                self.gc_safe_store(_isolate, value);
                true
            }

            type RawData = usize;

            pub fn load_raw(&self, _isolate: IsolateForSandbox, _no_gc: &DisallowGarbageCollection) -> Self::RawData {
                let address = self.address();
                base::ReadUnalignedValue::<Self::RawData>(address)
            }

            pub fn store_raw(&self, _isolate: IsolateForSandbox, data: Self::RawData, _no_gc: &DisallowGarbageCollection) {
                self.gc_safe_store(_isolate, data);
            }

            fn gc_safe_store(&self, _isolate: IsolateForSandbox, value: Address) {
                if cfg!(feature = "V8_COMPRESS_POINTERS") {
                    assert_eq!(kSmiShiftSize, 0);
                    assert!(SmiValuesAre31Bits());
                    assert_eq!(kTaggedSize, kInt32Size);

                    let lo = value as i32;
                    ObjectSlot::new(self.address() + kTaggedPayloadOffset as usize).Relaxed_Store(Tagged::<Smi>{_phantom: std::marker::PhantomData});
                    let hi = (value >> 32) as Tagged_t;
                    unsafe {
                        AsAtomicTagged::Relaxed_Store(
                            (self.address() + kRawPayloadOffset as usize) as *mut AtomicUsize,
                            hi,
                        );
                    }
                } else {
                    ObjectSlot::new(self.address() + kTaggedPayloadOffset as usize)
                        .Relaxed_Store(Tagged::<Smi>{_phantom: std::marker::PhantomData});
                }
            }
            
            pub fn MustClearDuringSerialization(&self, _no_gc: &DisallowGarbageCollection) -> bool {
                false
            }
        }
    }
}
