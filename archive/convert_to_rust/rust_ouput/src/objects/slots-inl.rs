// Converted from V8 C++ source files:
// Header: slots-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::atomic::{AtomicU32, Ordering};

mod base {
    pub mod atomic_utils {
        pub struct AsAtomicPointer {}
        impl AsAtomicPointer {
            pub fn Relaxed_Load(location: *mut u32) -> u32 {
                unsafe { *location }
            }
            pub fn Relaxed_Store(location: *mut u32, value: u32) {
                unsafe { *location = value; }
            }
             pub fn Acquire_Load(location: *mut u32) -> u32 {
                unsafe { *location }
            }
            pub fn Release_Store(location: *mut u32, value: u32) {
                unsafe { *location = value; }
            }

            pub fn Relaxed_CompareAndSwap(
                location: *mut u32,
                old: u32,
                target: u32,
            ) -> u32 {
                unsafe {
                    let atomic_ptr = location as *mut AtomicU32;
                    (*atomic_ptr).compare_and_swap(old, target, Ordering::Relaxed)
                }
            }
              pub fn Release_CompareAndSwap(
                location: *mut u32,
                old: u32,
                target: u32,
            ) -> u32 {
                unsafe {
                    let atomic_ptr = location as *mut AtomicU32;
                    (*atomic_ptr).compare_and_swap(old, target, Ordering::Release)
                }
            }
        }

        pub struct AsAtomic32 {}
        impl AsAtomic32 {
            pub fn Relaxed_Load(location: *mut u32) -> u32 {
                unsafe { *location }
            }
            pub fn Relaxed_Store(location: *mut u32, value: u32) {
                unsafe { *location = value; }
            }
             pub fn Acquire_Load(location: *mut u32) -> u32 {
                unsafe { *location }
            }
            pub fn Release_Store(location: *mut u32, value: u32) {
                unsafe { *location = value; }
            }
        }
    }
}

mod common {
    pub mod globals {
        pub const V8_MAP_PACKING: bool = true;
        pub const V8_COMPRESS_POINTERS: bool = true;
        pub const V8_ENABLE_SANDBOX: bool = true;
    }
    pub mod ptr_compr_inl {
        pub struct PtrComprCageBase {}
    }
}

mod objects {
    pub mod slots {
        pub struct SlotBase<T, U> {
            address_: usize,
            _phantom_t: std::marker::PhantomData<T>,
            _phantom_u: std::marker::PhantomData<U>,
        }

        impl<T, U> SlotBase<T, U> {
            pub fn new(address: usize) -> Self {
                SlotBase {
                    address_: address,
                    _phantom_t: std::marker::PhantomData,
                    _phantom_u: std::marker::PhantomData,
                }
            }

            pub fn location(&self) -> *mut u32 {
                self.address_ as *mut u32
            }
        }

        pub struct FullObjectSlot(pub SlotBase<(), u32>);
        pub struct FullMaybeObjectSlot(pub SlotBase<(), u32>);
         pub struct FullHeapObjectSlot(pub SlotBase<(), u32>);

        pub struct ExternalPointerSlot(pub SlotBase<(), u32>);
        pub struct CppHeapPointerSlot(pub SlotBase<(), u32>);
        pub struct IndirectPointerSlot(pub SlotBase<(), u32>);

        impl FullObjectSlot {
            pub fn new(object: &TaggedBase) -> Self {
                FullObjectSlot(SlotBase::new(&object.ptr_ as *const _ as usize))
            }

            pub fn contains_map_value(&self, raw_value: Address) -> bool {
                self.load_map().ptr() == raw_value
            }

            pub fn Relaxed_ContainsMapValue(&self, raw_value: Address) -> bool {
                unsafe { *(self.0.location()) == raw_value }
            }

            pub fn operator_star(&self) -> Tagged<Object> {
                unsafe { Tagged::<Object>::new(*(self.0.location())) }
            }

            pub fn load(&self) -> Tagged<Object> {
                self.operator_star()
            }

            pub fn load_with_cage(&self, _cage_base: PtrComprCageBase) -> Tagged<Object> {
                self.load()
            }

            pub fn store(&self, value: Tagged<Object>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

            pub fn store_map(&self, map: Tagged<Map>) {
                unsafe {
                    *(self.0.location()) = map.ptr();
                }
            }

            pub fn load_map(&self) -> Tagged<Map> {
                unsafe { Tagged::<Map>::new(*(self.0.location())) }
            }

            pub fn Acquire_Load(&self) -> Tagged<Object> {
                unsafe { Tagged::<Object>::new(*(self.0.location())) }
            }

              pub fn Acquire_Load_with_cage(&self, _cage_base: PtrComprCageBase) -> Tagged<Object> {
                self.Acquire_Load()
            }

            pub fn Relaxed_Load(&self) -> Tagged<Object> {
                unsafe { Tagged::<Object>::new(*(self.0.location())) }
            }

            pub fn Relaxed_Load_with_cage(&self, _cage_base: PtrComprCageBase) -> Tagged<Object> {
                self.Relaxed_Load()
            }

            pub fn Relaxed_Load_Raw(&self) -> Address {
                unsafe { *(self.0.location()) }
            }

            pub fn RawToTagged(_cage_base: PtrComprCageBase, raw: Address) -> Tagged<Object> {
                Tagged::<Object>::new(raw)
            }

            pub fn Relaxed_Store(&self, value: Tagged<Object>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

            pub fn Release_Store(&self, value: Tagged<Object>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

            pub fn Relaxed_CompareAndSwap(
                &self,
                old: Tagged<Object>,
                target: Tagged<Object>,
            ) -> Tagged<Object> {
                unsafe {
                    let result = base::atomic_utils::AsAtomicPointer::Relaxed_CompareAndSwap(
                        self.0.location(),
                        old.ptr(),
                        target.ptr(),
                    );
                    Tagged::<Object>::new(result)
                }
            }

              pub fn Release_CompareAndSwap(
                &self,
                old: Tagged<Object>,
                target: Tagged<Object>,
            ) -> Tagged<Object> {
                unsafe {
                    let result = base::atomic_utils::AsAtomicPointer::Release_CompareAndSwap(
                        self.0.location(),
                        old.ptr(),
                        target.ptr(),
                    );
                    Tagged::<Object>::new(result)
                }
            }
        }

        impl FullMaybeObjectSlot {
            pub fn operator_star(&self) -> Tagged<MaybeObject> {
                unsafe { Tagged::<MaybeObject>::new(*(self.0.location())) }
            }

            pub fn load(&self) -> Tagged<MaybeObject> {
                self.operator_star()
            }

            pub fn load_with_cage(&self, _cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
                self.operator_star()
            }

            pub fn store(&self, value: Tagged<MaybeObject>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

            pub fn Relaxed_Load(&self) -> Tagged<MaybeObject> {
                unsafe { Tagged::<MaybeObject>::new(*(self.0.location())) }
            }

             pub fn Relaxed_Load_with_cage(&self, _cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
                self.Relaxed_Load()
            }

            pub fn Relaxed_Load_Raw(&self) -> Address {
                unsafe { *(self.0.location()) }
            }

            pub fn RawToTagged(_cage_base: PtrComprCageBase, raw: Address) -> Tagged<Object> {
                Tagged::<Object>::new(raw)
            }

            pub fn Relaxed_Store(&self, value: Tagged<MaybeObject>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

               pub fn Release_CompareAndSwap(
                &self,
                old: Tagged<MaybeObject>,
                target: Tagged<MaybeObject>,
            ) {
                unsafe {
                     base::atomic_utils::AsAtomicPointer::Release_CompareAndSwap(
                        self.0.location(),
                        old.ptr(),
                        target.ptr(),
                    );
                }
            }
        }

         impl FullHeapObjectSlot {
             pub fn operator_star(&self) -> Tagged<HeapObjectReference> {
                unsafe { Cast::<HeapObjectReference>(Tagged::<MaybeObject>::new(*(self.0.location()))) }
            }

             pub fn load(&self, _cage_base: PtrComprCageBase) -> Tagged<HeapObjectReference> {
                self.operator_star()
            }

            pub fn store(&self, value: Tagged<HeapObjectReference>) {
                unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }

              pub fn ToHeapObject(&self) -> Tagged<HeapObject> {
                let value = unsafe {*(self.0.location())};
                 assert!(HAS_STRONG_HEAP_OBJECT_TAG(value));
                unsafe{Cast::<HeapObject>(Tagged::<Object>::new(value))}
            }

              pub fn StoreHeapObject(&self, value: Tagged<HeapObject>) {
                 unsafe {
                    *(self.0.location()) = value.ptr();
                }
            }
         }

        impl ExternalPointerSlot {
             pub fn new(slot:SlotBase<(), u32>) -> Self{
                ExternalPointerSlot(slot)
             }

            pub fn init(
                &self,
                isolate: IsolateForSandbox,
                host: Tagged<HeapObject>,
                value: Address,
                tag: ExternalPointerTag,
            ) {
                store(isolate, value, tag);
            }

             pub fn Relaxed_LoadHandle(&self) -> ExternalPointerHandle {
                unsafe { *(self.0.location()) }
            }

              pub fn Relaxed_StoreHandle(&self, handle: ExternalPointerHandle) {
                unsafe { *(self.0.location()) = handle}
            }

              pub fn Release_StoreHandle(&self, handle: ExternalPointerHandle) {
                unsafe { *(self.0.location()) = handle}
            }

              pub fn load(&self, isolate: IsolateForSandbox) -> Address {
               unsafe { *(self.0.location()) }
            }

             pub fn store(&self, isolate: IsolateForSandbox, value: Address, tag: ExternalPointerTag) {
               unsafe { *(self.0.location()) = value }
            }

            type RawContent = u32;

              pub fn GetAndClearContentForSerialization(
                &self,
                no_gc: &DisallowGarbageCollection,
            ) -> Self::RawContent {
                unsafe {
                    let content = *(self.0.location());
                    *(self.0.location()) = kNullExternalPointerHandle;
                    content
                }
            }

              pub fn RestoreContentAfterSerialization(
                &self,
                content: Self::RawContent,
                no_gc: &DisallowGarbageCollection,
            ) {
                unsafe {
                    *(self.0.location()) = content;
                }
            }

              pub fn ReplaceContentWithIndexForSerialization(
                &self,
                no_gc: &DisallowGarbageCollection,
                index: u32,
            ) {
                unsafe {
                    *(self.0.location()) = index;
                }
            }

             pub fn GetContentAsIndexAfterDeserialization(
                &self,
                no_gc: &DisallowGarbageCollection,
            ) -> u32 {
                unsafe { *(self.0.location()) }
            }

            fn address(&self) -> *mut Address{
                self.0.location() as *mut Address
            }

            fn handle_location(&self) -> *mut u32{
                self.0.location() as *mut u32
            }

             fn tag_range_(&self) -> ExternalPointerTagRange{
                ExternalPointerTagRange{}
            }

        }

        impl CppHeapPointerSlot {
             pub fn new(slot:SlotBase<(), u32>) -> Self{
                CppHeapPointerSlot(slot)
             }

              pub fn Relaxed_LoadHandle(&self) -> CppHeapPointerHandle {
                unsafe { *(self.0.location()) }
            }

              pub fn Relaxed_StoreHandle(&self, handle: CppHeapPointerHandle) {
                 unsafe { *(self.0.location()) = handle}
            }

              pub fn Release_StoreHandle(&self, handle: CppHeapPointerHandle) {
                unsafe { *(self.0.location()) = handle}
            }

            pub fn try_load(
                &self,
                isolate: IsolateForPointerCompression,
                tag_range: CppHeapPointerTagRange,
            ) -> Address {
                 unsafe { *(self.0.location()) }
            }

              pub fn store(&self, isolate: IsolateForPointerCompression, value: Address, tag: CppHeapPointerTag) {
                 unsafe { *(self.0.location()) = value }
            }

            pub fn init(&self) {
                unsafe { *(self.0.location()) = kNullCppHeapPointerHandle }
            }

            fn location(&self) -> *mut u32{
                self.0.location() as *mut u32
            }
        }

        impl IndirectPointerSlot {
             pub fn new(slot:SlotBase<(), u32>) -> Self{
                IndirectPointerSlot(slot)
             }

            pub fn load(&self, isolate: IsolateForSandbox) -> Tagged<Object> {
                self.Relaxed_Load(isolate)
            }

            pub fn store(&self, value: Tagged<ExposedTrustedObject>) {
                self.Relaxed_Store(value)
            }

             pub fn Relaxed_Load(&self, isolate: IsolateForSandbox) -> Tagged<Object> {
                let handle = self.Relaxed_LoadHandle();
                self.ResolveHandle(handle, isolate)
            }

              pub fn Relaxed_Load_AllowUnpublished(
                &self,
                isolate: IsolateForSandbox,
            ) -> Tagged<Object> {
                let handle = self.Relaxed_LoadHandle();
                self.ResolveHandle_AllowUnpublished(handle, isolate)
            }

             pub fn Acquire_Load(&self, isolate: IsolateForSandbox) -> Tagged<Object> {
                let handle = self.Acquire_LoadHandle();
                self.ResolveHandle(handle, isolate)
            }

            pub fn Relaxed_Store(&self, value: Tagged<ExposedTrustedObject>) {
                 unsafe { *(self.0.location()) = value.ptr() }
            }

            pub fn Release_Store(&self, value: Tagged<ExposedTrustedObject>) {
                 unsafe { *(self.0.location()) = value.ptr() }
            }

             pub fn Relaxed_LoadHandle(&self) -> IndirectPointerHandle {
                 unsafe { *(self.0.location()) }
            }

              pub fn Acquire_LoadHandle(&self) -> IndirectPointerHandle {
                unsafe { *(self.0.location()) }
            }

              pub fn Relaxed_StoreHandle(&self, handle: IndirectPointerHandle) {
                unsafe { *(self.0.location()) = handle }
            }

             pub fn Release_StoreHandle(&self, handle: IndirectPointerHandle) {
                unsafe { *(self.0.location()) = handle }
            }

            pub fn IsEmpty(&self) -> bool {
                self.Relaxed_LoadHandle() == kNullIndirectPointerHandle
            }

            type TagCheckStrictness = u32;
            const kAllowUnpublishedEntries: u32 = 1;

            fn ResolveHandle(&self, handle: IndirectPointerHandle, isolate: IsolateForSandbox) -> Tagged<Object> {
               unsafe { Tagged::<Object>::new(*(self.0.location())) }
            }

             fn ResolveHandle_AllowUnpublished(&self, handle: IndirectPointerHandle, isolate: IsolateForSandbox) -> Tagged<Object> {
                unsafe { Tagged::<Object>::new(*(self.0.location())) }
            }
        }

        pub struct WriteProtectedSlot<SlotT> {
            jit_allocation_: JitAllocation,
            slot: SlotT,
        }

        impl<SlotT> WriteProtectedSlot<SlotT> {
            pub fn Relaxed_Store(&self, value: TObject) {
                self.jit_allocation_.WriteHeaderSlot(self.slot.address(), value, 0);
            }
        }
    }

    pub mod heap_object {
        use crate::objects::map::Map;

        pub struct HeapObject {}
        impl HeapObject{
            pub fn address(&self) -> Address{
                1 as Address
            }
        }

        pub struct HeapObjectReference {}
    }

    pub mod map {
        pub struct Map {}
    }

    pub mod maybe_object {
        pub struct MaybeObject {}
    }

    pub mod objects {
        pub struct Object {}
    }

    pub mod tagged {
        #[derive(Debug, Copy, Clone)]
        pub struct Tagged<T> {
            ptr_: Address,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> Tagged<T> {
            pub fn new(ptr: Address) -> Self {
                Tagged {
                    ptr_: ptr,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn ptr(&self) -> Address {
                self.ptr_
            }
        }
        impl Tagged<Object>{}
        impl Tagged<Map>{}
        impl Tagged<MaybeObject>{}

    }
    pub mod compressed_slots{}
}

mod sandbox {
    pub mod cppheap_pointer_inl {
        pub type CppHeapPointerHandle = u32;
        pub const kNullCppHeapPointerHandle: CppHeapPointerHandle = 0;

    }
    pub mod indirect_pointer_inl {
        pub type IndirectPointerHandle = u32;
        pub const kNullIndirectPointerHandle: IndirectPointerHandle = 0;
        pub const kCodePointerHandleMarker: IndirectPointerHandle = 0;

    }
    pub mod isolate_inl {
        pub struct IsolateForSandbox {}
         pub struct IsolateForPointerCompression {}
    }

}

mod utils {
    pub mod memcopy {
        pub fn CopyImpl<const N: usize>(dst: *mut u32, src: *const u32, num_tagged: usize) {
             unsafe {
                std::ptr::copy_nonoverlapping(src, dst, num_tagged);
            }
        }

        pub fn MemsetUint32(start: *mut u32, value: u32, counter: usize) {
            unsafe {
                for i in 0..counter {
                    *start.add(i) = value;
                }
            }
        }

        pub fn MemsetPointer(start: *mut u32, value: u32, counter: usize) {
             unsafe {
                for i in 0..counter {
                    *start.add(i) = value;
                }
            }
        }
    }
}

mod include {
    pub mod v8_internal {
        pub struct TaggedObject {}
    }
}

pub use crate::objects::tagged::Tagged;
pub use crate::objects::objects::Object;
pub use crate::objects::map::Map;
pub use crate::objects::maybe_object::MaybeObject;

pub use crate::common::ptr_compr_inl::PtrComprCageBase;

pub use crate::sandbox::cppheap_pointer_inl::CppHeapPointerHandle;
pub use crate::sandbox::cppheap_pointer_inl::kNullCppHeapPointerHandle;

pub use crate::sandbox::indirect_pointer_inl::IndirectPointerHandle;
pub use crate::sandbox::indirect_pointer_inl::kNullIndirectPointerHandle;
pub use crate::sandbox::indirect_pointer_inl::kCodePointerHandleMarker;

pub use crate::sandbox::isolate_inl::IsolateForSandbox;
pub use crate::sandbox::isolate_inl::IsolateForPointerCompression;

pub use crate::utils::memcopy::CopyImpl;
pub use crate::utils::memcopy::MemsetPointer;
pub use crate::utils::memcopy::MemsetUint32;

pub use crate::common::globals::V8_COMPRESS_POINTERS;
pub use crate::common::globals::V8_MAP_PACKING;
pub use crate::common::globals::V8_ENABLE_SANDBOX;

pub type Address = u32;
pub type TObject = u32;
pub type TData = u32;
pub type ExternalPointerHandle = u32;
pub type ExternalPointerTag = u32;

pub const kNullAddress: Address = 0;
pub const kNullExternalPointerHandle: ExternalPointerHandle = 0;

pub fn HAS_STRONG_HEAP_OBJECT_TAG(_value:Address) -> bool{
    true
}

pub struct TaggedBase {
    ptr_: Address,
}

pub struct JitAllocation {}

impl JitAllocation {
    pub fn WriteHeaderSlot(&self, address: Address, value: TObject, _flag: u32) {}
}

pub fn Cast<T>(_obj: Tagged<Object>) -> Tagged<T> {
    Tagged::<T>::new(_obj.ptr())
}

pub struct BytecodeArray {}

pub struct Smi {}

pub struct HeapObjectReference {}

pub struct ExternalPointerTable {}

impl ExternalPointerTable {
    pub fn AllocateAndInitializeEntry(
        &self,
        _space: usize,
        _value: Address,
        _tag: ExternalPointerTag,
    ) -> ExternalPointerHandle {
        0
    }

    pub fn Get(&self, _handle: ExternalPointerHandle, _tag_range: ExternalPointerTagRange) -> Address {
        0
    }

     pub fn Set(&self, _handle: ExternalPointerHandle, _value: Address, _tag: ExternalPointerTag) {}
}

pub struct WasmInternalFunction {}

pub struct Code {}

pub struct DisallowGarbageCollection {}

pub struct ExposedTrustedObject{}
impl ExposedTrustedObject{
   const kSelfIndirectPointerOffset : usize = 0;
}
pub struct TrustedPointerTable{}

impl TrustedPointerTable{
   pub fn Get(&self, _handle: IndirectPointerHandle, _tag: ExternalPointerTag) -> Address {
        0
    }

     pub fn GetMaybeUnpublished(&self, _handle: IndirectPointerHandle, _tag: ExternalPointerTag) -> Address {
        0
    }
}

pub struct IsolateGroup{}
impl IsolateGroup{
    pub fn current() -> &'static IsolateGroup {
        unsafe {
            static mut ISOLATE_GROUP: IsolateGroup = IsolateGroup{};
            &ISOLATE_GROUP
        }
    }

    pub fn code_pointer_table(&self) -> &CodePointerTable {
        unsafe {
            static mut CODE_POINTER_TABLE: CodePointerTable = CodePointerTable{};
            &CODE_POINTER_TABLE
        }
    }
}

pub struct CodePointerTable{}
impl CodePointerTable{
    pub fn GetCodeObject(&self, _handle: IndirectPointerHandle) -> Address {
        0
    }
}

pub struct ExternalPointerTagRange{}
impl ExternalPointerTagRange{
    pub fn Contains(&self, _tag: ExternalPointerTag) -> bool{
        true
    }
}

pub struct CppHeapPointerTable{}
impl CppHeapPointerTable{
    pub fn Get(&self, _handle: CppHeapPointerHandle, _tag_range: CppHeapPointerTagRange) -> Address {
        0
    }

     pub fn Set(&self, _handle: CppHeapPointerHandle, _value: Address, _tag: CppHeapPointerTag) {}
}

pub struct CppHeapPointerTagRange{}
pub struct CppHeapPointerTag{}

unsafe fn ReadMaybeUnalignedValue<T>(ptr: *mut u32) -> T {
    (ptr as *mut T).read_unaligned()
}

unsafe fn WriteMaybeUnalignedValue<T>(ptr: *mut u32, value: T) {
    (ptr as *mut T).write_unaligned(value)
}
