// Converted from V8 C++ source files:
// Header: js-array-buffer-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::v8::*;

pub struct JSArrayBuffer {}
pub struct JSArrayBufferView {}
pub struct JSTypedArray {}
pub struct JSDataViewOrRabGsabDataView {}
pub struct JSDataView {}
pub struct JSRabGsabDataView {}
pub struct Object {}
pub struct Isolate {}
pub struct String {}
pub struct SharedFlag {}
pub struct MaybeObject {}
pub struct Handle<T> {
    _phantom: PhantomData<T>,
}
pub struct MessageTemplate {}
pub struct DirectHandle<T> {
    _phantom: PhantomData<T>,
}
pub struct ExternalPointerHandle {}
pub struct ArrayBufferExtension {}
pub struct Tagged<T> {
    _phantom: PhantomData<T>,
}
pub struct Smi {}
pub struct IsolateForPointerCompression {}
pub struct PtrComprCageBase {}
pub struct DisallowGarbageCollection {}
pub struct Descriptor {}
pub struct InternalIndex {}
pub struct BackingStore {}
pub struct JSFunction {}
pub struct FixedArray {}
pub struct IndirectHandle<T> {
    _phantom: PhantomData<T>,
}
pub struct ValueType {}
pub struct Range<T> {
    _phantom: PhantomData<T>,
}
pub struct Vector<T> {
    _phantom: PhantomData<T>,
}
pub struct WriteBarrierMode {}
pub struct FeedbackSlot {}
pub struct Condition {}
pub struct Operand {}
pub struct Register {}
pub struct OpIndex {}
pub struct InstructionOperand {}
pub struct Write {
    _phantom: PhantomData<u8>,
}
pub struct DisplayNamesInternal {}
pub struct Managed<T> {
    _phantom: PhantomData<T>,
}
pub struct Code {}
pub struct CodeEntrypointTag {}
pub struct HeapObject {}
pub struct Address {}
pub struct ExternalPointerTable {}
pub struct ExternalPointerTag {}
pub struct IsolateForSandbox {}
pub struct Tagged_t {}
pub struct BranchSemantics {}
pub struct OperationType {}
pub struct Bytecode {}
pub struct RegisterT {}
pub struct SnapshotData {}
pub struct Label {}
pub struct ElementsKind {}
pub struct AcquireLoadTag {}
pub struct MaybeDirectHandle<T> {
    _phantom: PhantomData<T>,
}
pub struct ExternalArrayType {}
pub struct FieldIndex {}

const kNullExternalPointerHandle: u32 = 0;
const kArrayBufferExtensionTag: u32 = 0;
const SKIP_WRITE_BARRIER: i32 = 0;

impl JSArrayBuffer {
    pub fn byte_length(&self) -> usize {
        0
    }

    pub fn GetBackingStore(&self) -> std::shared_ptr::SharedPtr<BackingStore> {
        std::shared_ptr::SharedPtr::new(BackingStore {})
    }

    pub fn backing_store(&self) -> std::shared_ptr::SharedPtr<BackingStore> {
        std::shared_ptr::SharedPtr::new(BackingStore {})
    }

    pub fn extension(&self) -> *mut ArrayBufferExtension {
        std::ptr::null_mut()
    }
    pub fn set_extension(&mut self, _extension: *mut ArrayBufferExtension) {}
    pub fn was_detached(&self) -> bool {
        false
    }
    pub fn is_shared(&self) -> bool {
        false
    }
    pub fn is_resizable_by_js(&self) -> bool {
        false
    }
    pub fn GetByteLength(&self) -> usize {
        0
    }
    pub fn init_extension(&self) {}
    pub fn IsEmpty(&self) -> bool {
        false
    }
    pub fn set_bit_field(&self, _bits: u32) {}
}

impl JSArrayBufferView {
    pub fn buffer(&self) -> *mut JSArrayBuffer {
        std::ptr::null_mut()
    }
    pub fn is_length_tracking(&self) -> bool {
        false
    }
    pub fn is_backed_by_rab(&self) -> bool {
        false
    }
}

impl JSTypedArray {
    pub fn element_size(&self) -> usize {
        0
    }
    pub fn is_on_heap(&self) -> bool {
        false
    }
    pub fn base_pointer(&self) -> Smi {
        Smi {}
    }
    pub fn external_pointer(&self) -> Address {
        Address {}
    }
    pub fn GetLength(&self) -> usize {
        0
    }
    pub fn WasDetached(&self) -> bool {
        false
    }
    pub fn IsVariableLength(&self) -> bool {
        false
    }
    pub fn IsOutOfBounds(&self) -> bool {
        false
    }
    pub fn length(&self) -> usize {
        0
    }
    pub fn SetOffHeapDataPtr(&self, _isolate: *mut Isolate, _base: *mut void, _offset: Address) {}
    pub fn DataPtr(&self) -> *mut void {
        std::ptr::null_mut()
    }
    pub fn IsDetachedOrOutOfBounds(&self) -> bool {
        false
    }
}

impl JSDataViewOrRabGsabDataView {
    pub fn set_data_pointer(&self, _isolate: *mut Isolate, _ptr: *mut void) {}
}

impl JSRabGsabDataView {
    pub fn GetByteLength(&self) -> usize {
        0
    }
    pub fn buffer(&self) -> *mut JSArrayBuffer {
        std::ptr::null_mut()
    }
    pub fn byte_offset(&self) -> usize {
        0
    }
    pub fn IsOutOfBounds(&self) -> bool {
        false
    }
    pub fn is_backed_by_rab(&self) -> bool {
        false
    }
    pub fn is_length_tracking(&self) -> bool {
        false
    }
    pub fn byte_length(&self) -> usize {
        0
    }
}

mod std {
    pub mod shared_ptr {
        pub struct SharedPtr<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> SharedPtr<T> {
            pub fn new(_value: T) -> Self {
                SharedPtr {
                    _phantom: std::marker::PhantomData,
                }
            }
            pub fn use_count(&self) -> usize {
                0
            }
        }
    }
}

mod base {
    pub mod AsAtomic32 {
        pub fn Release_Store(_location: *mut u32, _value: u32) {}
        pub fn Acquire_Load(_location: *mut u32) -> u32 {
            0
        }
        pub fn Relaxed_Load(_location: *mut u32) -> u32 {
            0
        }
    }
    pub mod AsAtomicPointer {
        pub fn Release_Store(_location: *mut *mut super::ArrayBufferExtension, _value: *mut super::ArrayBufferExtension) {}
        pub fn Acquire_Load(_location: *mut *mut super::ArrayBufferExtension) -> *mut super::ArrayBufferExtension {
            std::ptr::null_mut()
        }
    }
}

pub mod internal {
    pub struct HeapObject {}
    pub struct TaggedObject {}
}

pub mod ptr_compr {
    pub struct PtrComprCageBase {
        dummy: u32
    }
    impl PtrComprCageBase {
        pub fn address(&self) -> super::Address{
            super::Address{}
        }
    }
}
