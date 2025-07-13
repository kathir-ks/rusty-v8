// Converted from V8 C++ source files:
// Header: snapshot.h
// Implementation: snapshot.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub mod internal {
        use std::rc::Rc;

        pub struct Context;
        pub struct Isolate;
        pub struct JSGlobalProxy;
        pub struct SafepointScope;
        pub struct SnapshotData;
        pub struct AllStatic;
        pub struct StartupData {
            pub data: *mut char,
            pub raw_size: i32,
        }
        pub struct SerializeEmbedderFieldsCallback;
        pub struct DirectHandle<T>(*mut T);
        impl<T> DirectHandle<T> {
            pub fn FromSlot(location: *mut *mut T) -> DirectHandle<T> {
                DirectHandle(*unsafe { *location })
            }
        }
        pub struct DirectHandleExt {}
        pub struct NativeContext;
        pub struct HeapObject;
        pub struct ArrayList;
        pub struct Object;
        pub struct FixedArray;
        pub struct DescriptorLookupCache;
        pub struct Heap {
            pub serialized_objects: *mut Object,
        }
        impl Heap {
            pub fn SetSerializedObjects(&mut self, obj: Object) {}
        }
        pub struct ReadOnlySpace;
        impl ReadOnlySpace {
            pub fn writable(&self) -> bool {
                true
            }
        }
        pub struct ReadOnlyHeap;
        impl ReadOnlyHeap {
            pub fn OnCreateHeapObjectsComplete(&self, isolate: *mut Isolate) {}
        }
        pub struct EmbedderStackStateScope;
        pub struct GarbageCollectionReason;
        pub struct SharedFunctionInfo;
        pub struct Script;
        pub struct JSFunction;
        pub struct WeakFixedArray;
        pub struct SerializeEmbedderFieldsCallback;
        pub struct V8_EXPORT_PRIVATE;
        pub struct Flags<T>;
        pub struct DirectHandle<T>(*mut T);
        pub struct FixedArrayExt {}
        pub struct Tagged<T>(*mut T);
        pub struct ArrayBuffer {
            pub allocator_: *mut ArrayBufferAllocator,
        }
        pub struct ArrayBufferAllocator;
        impl ArrayBufferAllocator {
             pub fn NewDefaultAllocator() -> Box<ArrayBufferAllocator> {
                Box::new(ArrayBufferAllocator {})
            }
        }
        pub struct IsolateCreateParams {
            pub array_buffer_allocator: *mut ArrayBufferAllocator,
            pub snapshot_blob: *const StartupData,
            pub external_references: *const i64,
            pub array_buffer_allocator_shared: Option<Box<ArrayBufferAllocator>>
        }
        pub struct GlobalHandles;
        pub struct IsolateExt {}
        pub struct SafepointKind;
        pub struct StartupSerializer;
        pub struct ReadOnlySerializer;
        pub struct SharedHeapSerializer;
        pub struct ContextSerializer;
        pub struct DisallowGarbageCollection;
        pub struct SerializedHandleChecker;
        pub struct String;
        pub struct ScriptOrigin;
        pub mod utils {
            use super::{DirectHandle, Context};
            pub fn OpenDirectHandle(context: *mut Context) -> DirectHandle<Context> {
                DirectHandle(context)
            }
        }
        pub struct Factory;
        pub mod factory {
            use super::{FixedArray, Smi};
            pub fn NewFixedArray(_size: i32, _alloc: super::AllocationType) -> FixedArray {
                FixedArray {}
            }
        }
        pub mod flags {}
        pub mod builtins {}
        pub mod global_handles {
            use super::Isolate;
            pub fn Create<T>(_isolate: *mut Isolate, _val: T) -> *mut T {
                unsafe { std::mem::transmute(1_usize) }
            }

            pub fn Destroy(_location: *mut usize) {}
        }
        pub mod read_only_roots {
            use super::Object;
            pub struct ReadOnlyRoots {
                pub undefined_value_: Object,
                pub empty_fixed_array_: Object,
            }
            pub fn undefined_value() -> Object {
                Object {}
            }
            pub fn empty_fixed_array() -> Object {
                Object {}
            }
        }
        pub mod descriptor_lookup_cache {
            use super::Isolate;
            pub fn Clear(_isolate: *mut Isolate) {}
        }
        pub mod heap {
             use super::Object;
             pub fn CompactWeakArrayLists() {}
             pub fn CollectAllAvailableGarbage(_gcr:super::GarbageCollectionReason){}
             pub fn SetFunctionsMarkedForManualOptimization(_obj:Object){}
             pub fn ConfigureHeap(_constraints: u64, _cpp_heap:u64) {}

        }
        pub mod base {
            use super::StartupData;
            pub fn ReadLittleEndianValue<T>(_address: *mut i8) -> T {
                unsafe { std::mem::zeroed() }
            }
            pub fn WriteLittleEndianValue<T>(_address: *mut i8, _value: T) {}

             pub struct Vector<T> {
                data: *const T,
                length: usize,
            }

            impl<T> Vector<T> {
                pub fn new(data: *const T, length: usize) -> Self {
                    Vector { data, length }
                }

                pub fn begin(&self) -> *const T {
                    self.data
                }

                pub fn length(&self) -> usize {
                    self.length
                }
            }
            pub struct BaseExt {}
        }
        pub mod version {
            pub fn GetString(_vector: super::base::Vector<char>) {}
        }

        pub enum class AllocationType {}
        pub enum class GarbageCollectionReason {}
        pub struct StartupSerializerExt {}
        pub struct DisallowGarbageCollectionExt {}
        pub struct SafepointScopeExt {}
        pub struct TaggedExt {}
        pub struct HandleScope {}
        pub struct EmbedderStackStateScope {}
        pub mod utils_impl {
            pub struct UtilsImpl {}
        }
        pub struct IsolateState {}
        pub struct GlobalHandlesExt {}
        pub struct SafepointScopeState {}
        pub struct LocalIsolate {
           pub isolate_state_: *mut IsolateState,
           pub global_handles_:*mut GlobalHandles,
           pub local_heap_: *mut LocalHeap,
           pub is_execution_pausable_: bool,
        }
        impl LocalIsolate {
          pub fn ExecuteMainThreadWhileParked<F>(&self, f: F)
           where F: FnOnce() {

           }
        }
        pub struct HeapVerifier {}
        pub mod heap_verifier {
           use super::Heap;
           pub fn VerifyHeap(_heap: *mut Heap) -> bool{
              true
           }
        }
        pub struct Isolate {
           pub snapshot_blob_: *const StartupData,
           pub array_buffer_allocator_: *mut ArrayBufferAllocator,
           pub api_external_references_: *const i64,
           pub array_buffer_allocator_shared_: Option<Box<ArrayBufferAllocator>>,
           pub has_shared_space_: bool,
           pub local_isolate_: *mut LocalIsolate,
           pub enable_serializer_: bool,
        }

        impl Isolate {
           pub fn New() -> *mut Isolate {
              Box::into_raw(Box::new(Isolate{
                snapshot_blob_: std::ptr::null(),
                array_buffer_allocator_: std::ptr::null_mut(),
                api_external_references_: std::ptr::null(),
                array_buffer_allocator_shared_: None,
                has_shared_space_: false,
                local_isolate_: std::ptr::null_mut(),
                enable_serializer_:false,
              }))
           }

           pub fn Delete(isolate: *mut Isolate) {
               unsafe {
                   drop(Box::from_raw(isolate));
               }
           }

           pub fn snapshot_blob(&self) -> *const StartupData{
              self.snapshot_blob_
           }

           pub fn set_snapshot_blob(&mut self, blob: *const StartupData){
              self.snapshot_blob_ = blob
           }
           pub fn array_buffer_allocator(&self) -> *mut ArrayBufferAllocator {
              self.array_buffer_allocator_
           }

           pub fn set_array_buffer_allocator(&mut self, allocator: *mut ArrayBufferAllocator) {
              self.array_buffer_allocator_ = allocator
           }

           pub fn set_api_external_references(&mut self, external_references: *const i64) {
              self.api_external_references_ = external_references
           }

           pub fn set_array_buffer_allocator_shared(&mut self, allocator:Option<Box<ArrayBufferAllocator>>) {
             self.array_buffer_allocator_shared_ = allocator
           }

           pub fn has_shared_space(&self) -> bool {
             self.has_shared_space_
           }

           pub fn local_isolate(&self) -> *mut LocalIsolate {
              self.local_isolate_
           }
           pub fn Enter(&self) {}
           pub fn Exit(&self) {}
           pub fn InitWithoutSnapshot(&self) {}
           pub fn InitWithSnapshot(&self, _startup_snapshot_data: *const SnapshotData, _read_only_snapshot_data: *const SnapshotData, _shared_heap_snapshot_data: *const SnapshotData, _extract_rehashability: bool) -> bool {true}
           pub fn heap(&mut self) -> &mut Heap {
               unsafe { std::mem::transmute(1_usize) }
           }
           pub fn descriptor_lookup_cache(&mut self) -> &mut DescriptorLookupCache {
                unsafe { std::mem::transmute(1_usize) }
           }
           pub fn read_only_heap(&self) -> &ReadOnlyHeap {
              unsafe { std::mem::transmute(1_usize) }
           }
           pub fn set_serialized_global_proxy_sizes(&self, _global_proxy_sizes: FixedArray) {}
           pub fn main_thread_local_isolate(&self) -> *mut LocalIsolate{
             unsafe { std::mem::transmute(1_usize) }
           }
           pub fn set_enable_serializer(&mut self) {
              self.enable_serializer_ = true;
           }
           pub fn enable_serializer(&self) -> bool {
             self.enable_serializer_
           }
           pub fn counters(&self) -> &Counters{
                unsafe { std::mem::transmute(1_usize) }
           }
           pub fn bootstrapper(&self) -> &Bootstrapper{
             unsafe { std::mem::transmute(1_usize) }
           }
        }

        pub struct LocalHeap;
        impl LocalHeap {

        }

        pub struct SnapshotCreator {
           pub impl_: *mut SnapshotCreatorImpl,
        }

        impl SnapshotCreator {
           pub fn GetIsolate(&self) -> *mut Isolate {
              unsafe {
                (*self.impl_).isolate_
              }
           }
        }

        pub struct Counters{
          pub snapshot_decompress_:RuntimeCallStats
        }

        pub struct RuntimeCallStats{}

        impl RuntimeCallStats {
             pub fn new() -> Self {
              RuntimeCallStats{}
           }
        }

        pub struct Bootstrapper{}
        impl Bootstrapper {
            pub fn CreateEnvironmentForTesting(&self) -> DirectHandle<NativeContext> {
              DirectHandle(unsafe { std::mem::transmute(1_usize) })
            }
        }
        pub struct WeakArrayList;
        pub enum class SafepointKind {}
        pub struct StackState{}
        pub enum class EmbedderStackStateOrigin {}
        pub struct SerializeEmbedderFieldsCallback{

        }
        pub mod wasm{
        }
        pub enum class DisallowGarbageCollection {}
        pub struct Tagged<T>{
           obj : *mut T
        }
        pub enum class FunctionCodeHandling{}
        impl FunctionCodeHandling{
          pub const kClear: FunctionCodeHandling = FunctionCodeHandling{};
          pub const kKeep: FunctionCodeHandling = FunctionCodeHandling{};
        }
    }
}

pub mod v8_go{
    use std::rc::Rc;

    pub struct V8 {}
    pub struct Isolate;

    pub struct SafepointScope {}
    pub struct DisallowGarbageCollection {}
    pub struct SerializeEmbedderFieldsCallback {}
    pub struct AllStatic {}

    pub enum Snapshot {
    }

    pub enum SerializerFlag {
        kAllowUnknownExternalReferencesForTesting = 1 << 0,
        kAllowActiveIsolateForTesting = 1 << 1,
        kReconstructReadOnlyAndSharedObjectCachesForTesting = 1 << 2,
    }

    pub type SerializerFlags = base::Flags<SerializerFlag>;
    pub const kDefaultSerializerFlags: SerializerFlags = base::Flags {};

    impl Snapshot {
        pub fn ClearReconstructableDataForSerialization(
            isolate: *mut internal::Isolate,
            clear_recompilable_data: bool,
        ) {
           internal::Snapshot::ClearReconstructableDataForSerialization(isolate,clear_recompilable_data)
        }

        pub fn Create(
            isolate: *mut internal::Isolate,
            contexts: &mut Vec<internal::Tagged<internal::Context>>,
            embedder_fields_serializers: &Vec<SerializeEmbedderFieldsCallback>,
            safepoint_scope: &SafepointScope,
            no_gc: &DisallowGarbageCollection,
            flags: SerializerFlags,
        ) -> internal::StartupData {
           internal::Snapshot::Create(isolate, contexts, embedder_fields_serializers, unsafe { std::mem::transmute(safepoint_scope) }, unsafe { std::mem::transmute(no_gc) }, flags)
        }

        pub fn Initialize(isolate: *mut internal::Isolate) -> bool {
            internal::Snapshot::Initialize(isolate)
        }

        pub fn NewContextFromSnapshot(
            isolate: *mut internal::Isolate,
            global_proxy: internal::DirectHandle<internal::JSGlobalProxy>,
            context_index: usize,
            embedder_fields_deserializer: SerializeEmbedderFieldsCallback,
        ) -> Option<internal::DirectHandle<internal::Context>> {
           let res = internal::Snapshot::NewContextFromSnapshot(isolate, global_proxy, context_index, unsafe { std::mem::transmute(embedder_fields_deserializer)});
           if res.0.is_null(){
              None
           }else{
              Some(res)
           }

        }

        pub fn SerializeDeserializeAndVerifyForTesting(
            isolate: *mut internal::Isolate,
            default_context: internal::DirectHandle<internal::Context>,
        ) {
           internal::Snapshot::SerializeDeserializeAndVerifyForTesting(isolate, default_context)
        }

        pub fn HasContextSnapshot(isolate: *mut internal::Isolate, index: usize) -> bool {
            internal::Snapshot::HasContextSnapshot(isolate, index)
        }
        pub fn EmbedsScript(isolate: *mut internal::Isolate) -> bool {
            false
        }
        pub fn GetExpectedChecksum(data: *const internal::StartupData) -> u32 {
            internal::Snapshot::GetExpectedChecksum(unsafe { &*data })
        }
        pub fn CalculateChecksum(data: *const internal::StartupData) -> u32 {
           internal::Snapshot::CalculateChecksum(unsafe { &*data })
        }
        pub fn VerifyChecksum(data: *const internal::StartupData) -> bool {
           internal::Snapshot::VerifyChecksum(unsafe { &*data })
        }
        pub fn ExtractRehashability(data: *const internal::StartupData) -> bool {
            internal::Snapshot::ExtractRehashability(unsafe { &*data })
        }
        pub fn ExtractReadOnlySnapshotChecksum(data: *const internal::StartupData) -> u32 {
           internal::Snapshot::ExtractReadOnlySnapshotChecksum(unsafe { &*data })
        }
        pub fn VersionIsValid(data: *const internal::StartupData) -> bool {
            internal::Snapshot::VersionIsValid(unsafe { &*data })
        }

        pub fn DefaultSnapshotBlob() -> *const internal::StartupData {
            internal::Snapshot::DefaultSnapshotBlob()
        }
        pub fn ShouldVerifyChecksum(data: *const internal::StartupData) -> bool {
           internal::Snapshot::ShouldVerifyChecksum(unsafe { &*data })
        }
    }

    pub fn CreateSnapshotDataBlobInternal(
        function_code_handling: v8::SnapshotCreator::FunctionCodeHandling,
        embedded_source: *const i8,
        serializer_flags: SerializerFlags,
    ) -> internal::StartupData {
       let embedded_source_str = if embedded_source.is_null() {
          None
       } else {
           unsafe {
               let c_str = std::ffi::CStr::from_ptr(embedded_source);
               c_str.to_str().ok()
           }
       };
       if let Some(embedded_source_str) = embedded_source_str {
          _create_snapshot_data_blob_internal_cstr(function_code_handling, embedded_source_str, serializer_flags)
       } else {
          internal::StartupData { data: std::ptr::null_mut(), raw_size: 0 }
       }
    }

    pub fn CreateSnapshotDataBlobInternalForInspectorTest(
      function_code_handling: v8::SnapshotCreator::FunctionCodeHandling,
      embedded_source: *const i8,
    ) -> internal::StartupData {
       let embedded_source_str = if embedded_source.is_null() {
          None
       } else {
           unsafe {
               let c_str = std::ffi::CStr::from_ptr(embedded_source);
               c_str.to_str().ok()
           }
       };
       if let Some(embedded_source_str) = embedded_source_str {
           _create_snapshot_data_blob_internal_for_inspector_test(function_code_handling, embedded_source_str)
       } else {
           internal::StartupData { data: std::ptr::null_mut(), raw_size: 0 }
       }
    }

    pub fn WarmUpSnapshotDataBlobInternal(
        cold_snapshot_blob: internal::StartupData,
        warmup_source: *const i8,
    ) -> internal::StartupData {
       let warmup_source_str = if warmup_source.is_null() {
          None
       } else {
           unsafe {
               let c_str = std::ffi::CStr::from_ptr(warmup_source);
               c_str.to_str().ok()
           }
       };
       if let Some(warmup_source_str) = warmup_source_str {
          _warm_up_snapshot_data_blob_internal(cold_snapshot_blob, warmup_source_str)
       } else {
          internal::StartupData { data: std::ptr::null_mut(), raw_size: 0 }
       }
    }
}
pub mod base {
    use super::v8::internal::Flags;
    pub struct Vector<T> {
        data: *const T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new(data: *const T, length: usize) -> Self {
            Vector { data, length }
        }
    }
    impl<T> Vector<T> {
        pub fn begin(&self) -> *const T {
            self.data
        }

        pub fn length(&self) -> usize {
            self.length
        }
    }

    pub struct BaseExt {}
    impl BaseExt{}
    impl From<i32> for Vector<u8> {
      fn from(num: i32) -> Self {
         Vector{data: std::ptr::null(), length:num as usize}
      }
    }
    impl From<i32> for Vector<char> {
      fn from(num: i32) -> Self {
         Vector{data: std::ptr::null(), length:num as usize}
      }
    }
}

pub mod snapshot_impl{
    use super::v8::internal::*;
    use std::mem;

    pub fn CreateSnapshotBlob(
        startup_snapshot_in: *const SnapshotData,
        read_only_snapshot_in: *const SnapshotData,
        shared_heap_snapshot_in: *const SnapshotData,
        context_snapshots_in: &Vec<*mut SnapshotData>,
        can_be_rehashed: bool,
    ) -> StartupData {

       StartupData{data: std::ptr::null_mut(), raw_size: 0}
    }

    pub fn ExtractNumContexts(data: *const StartupData) -> u32 {
        GetHeaderValue(data, kNumberOfContextsOffset)
    }

    pub fn ExtractContextOffset(data: *const StartupData, index: u32) -> u32 {
        let context_offset = GetHeaderValue(data, ContextSnapshotOffsetOffset(index));
        assert!(context_offset < unsafe { (*data).raw_size } as u32);
        context_offset
    }

    pub fn ExtractStartupData(data: *const StartupData) -> base::Vector<u8> {
        assert!(SnapshotIsValid(data));

        let num_contexts = ExtractNumContexts(data);
        ExtractData(data, StartupSnapshotOffset(num_contexts), GetHeaderValue(data, kReadOnlyOffsetOffset))
    }

    pub fn ExtractReadOnlyData(data: *const StartupData) -> base::Vector<u8> {
        assert!(SnapshotIsValid(data));

        ExtractData(data, GetHeaderValue(data, kReadOnlyOffsetOffset), GetHeaderValue(data, kSharedHeapOffsetOffset))
    }

    pub fn ExtractSharedHeapData(data: *const StartupData) -> base::Vector<u8> {
        assert!(SnapshotIsValid(data));

        ExtractData(data, GetHeaderValue(data, kSharedHeapOffsetOffset), GetHeaderValue(data, ContextSnapshotOffsetOffset(0)))
    }

    pub fn ExtractContextData(data: *const StartupData, index: u32) -> base::Vector<u8> {
        let num_contexts = ExtractNumContexts(data);
        assert!(index < num_contexts);

        let context_offset = ExtractContextOffset(data, index);
        let mut next_context_offset;
        if index == num_contexts - 1 {
            next_context_offset = unsafe { (*data).raw_size } as u32;
        } else {
            next_context_offset = ExtractContextOffset(data, index + 1);
            assert!(next_context_offset < unsafe { (*data).raw_size } as u32);
        }

        let context_data = unsafe { (*data).data.add(context_offset as usize) as *const u8 };
        let context_length = next_context_offset - context_offset;
        base::Vector::new(context_data, context_length as usize)
    }

    pub fn GetHeaderValue(data: *const StartupData, offset: u32) -> u32 {
        assert!(!data.is_null());
        assert!(offset < unsafe { (*data).raw_size } as u32);
        unsafe {
            let address = (*data).data.add(offset as usize) as *mut i8;
            base::ReadLittleEndianValue(address)
        }
    }

    pub fn SetHeaderValue(data: *mut i8, offset: u32, value: u32) {
        unsafe {
            let address = data.add(offset as usize);
            base::WriteLittleEndianValue(address, value);
        }
    }

    pub fn CheckVersion(data: *const StartupData) {
        if !VersionIsValid(data) {
            let mut version:base::Vector<char> = base::Vector::from(kVersionStringLength as i32);
            unsafe {
                memset(version.data as *mut i8, 0, kVersionStringLength as usize);
                version::GetString(version);
                println!("Version mismatch between V8 binary and snapshot.\n");
                println!("#   V8 binary version: {:#?}", std::str::from_utf8(std::slice::from_raw_parts(version.data as *const u8, kVersionStringLength as usize)).unwrap());
                println!("#    Snapshot version: {:#?}", std::str::from_utf8(std::slice::from_raw_parts((*data).data.add(kVersionStringOffset as usize) as *const u8, kVersionStringLength as usize)).unwrap());
                println!("# The snapshot consists of {} bytes and contains {} context(s).", (*data).raw_size, ExtractNumContexts(data));
            }
            panic!("Version mismatch between V8 binary and snapshot.");
        }
    }
    pub fn ChecksummedContent(data: *const StartupData) -> base::Vector<u8> {
        static_assert(kReadOnlySnapshotChecksumOffset == kChecksumOffset + mem::size_of::<u32>() as u32);
        let kChecksumStart = kReadOnlySnapshotChecksumOffset;
        let data_ptr = unsafe { (*data).data.add(kChecksumStart as usize) as *const u8};
        let data_len = unsafe { (*data).raw_size } as u32 - kChecksumStart;
        base::Vector::new(data_ptr, data_len as usize)
    }
    pub fn StartupSnapshotOffset(num_contexts: u32) -> u32 {
        let offset = kFirstContextOffsetOffset + num_contexts * mem::size_of::<i32>() as u32;
        (offset + mem::size_of::<usize>() as u32 - 1) & !(mem::size_of::<usize>() as u32 - 1)
    }

    pub fn ContextSnapshotOffsetOffset(index: u32) -> u32 {
        kFirstContextOffsetOffset + index * mem::size_of::<i32>() as u32
    }

    fn ExtractData(snapshot: *const StartupData, start_offset: u32, end_offset: u32) -> base::Vector<u8> {
        assert!(start_offset < end_offset);
        assert!(end_offset < unsafe { (*snapshot).raw_size } as u32);
        let length = end_offset - start_offset;
        let data = unsafe { (*snapshot).data.add(start_offset as usize) as *const u8 };
        base::Vector::new(data, length as usize)
    }
     pub fn VersionIsValid(data: *const StartupData) -> bool {
        let mut version = base::Vector::<char>::from(kVersionStringLength as i32);
        unsafe {
            memset(version.data as *mut i8, 0, kVersionStringLength as usize);
            CheckLt(kVersionStringOffset + kVersionStringLength, (*data).raw_size as u32);
            version::GetString(version);
            strncmp(version.data as *const i8, (*data).data.add(kVersionStringOffset as usize) as *const i8, kVersionStringLength as usize) == 0
        }
    }
    pub fn Checksum(data: base::Vector<u8>) -> u32 {
        0
    }

     unsafe fn CheckLt(a: u32, b: u32) {
        if a >= b {
            panic!("CHECK_LT failed: {} >= {}", a, b);
        }
    }

     extern "C" {
        fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
        fn memset(s: *mut i8, c: i32, n: usize);
    }

     pub fn SnapshotIsValid(snapshot_blob: *const StartupData) -> bool {
        ExtractNumContexts(snapshot_blob) > 0
    }

    pub const kNumberOfContextsOffset: u32 = 0;
    pub const kRehashabilityOffset: u32 = kNumberOfContextsOffset + 4;
    pub const kChecksumOffset: u32 = kRehashabilityOffset + 4;
    pub const kReadOnlySnapshotChecksumOffset: u32 = kChecksumOffset + 4;
    pub const kVersionStringOffset: u32 = kReadOnlySnapshotChecksumOffset + 4;
    pub const kVersionStringLength: u32 = 64;
    pub const kReadOnlyOffsetOffset: u32 = kVersionStringOffset + kVersionStringLength;
    pub const kSharedHeapOffsetOffset: u32 = kReadOnlyOffsetOffset + 4;
    pub const kFirstContextOffsetOffset: u32 = kSharedHeapOffsetOffset + 4;
}
pub mod snapshot_creator_impl {
   use super::v8::internal::*;
   pub struct SerializableContext{

   }
}
pub mod global{
    use super::v8::internal::*;
}
extern "C" {
   pub fn _create_snapshot_data_blob_internal_cstr(
        function_code_handling: v8::SnapshotCreator::FunctionCodeHandling,
        embedded_source: &str,
        serializer_flags: v8_go::SerializerFlags,
   ) -> internal::StartupData;
   pub fn _warm_up_snapshot_data_blob_internal(
        cold_snapshot_blob: internal::StartupData,
        warmup_source: &str,
    ) -> internal::StartupData;
   pub fn _create_snapshot_data_blob_internal_for_inspector_test(
        function_code_handling: v8::SnapshotCreator::FunctionCodeHandling,
        embedded_source: &str,
   ) -> internal::StartupData;
}

pub use snapshot_impl::*;
pub mod create_snapshot_data_blob_internal{
   use super::v8::internal::*;
}
pub struct SnapshotCreatorImpl {
    owns_isolate_: bool,
    isolate_: *mut internal::Isolate,
    array_buffer_allocator_: Box<internal::ArrayBufferAllocator>,
    contexts_: Vec<SerializableContext>,
}
pub struct SerializableContext{
   handle_location: *mut usize,
   callback: internal::SerializeEmbedderFieldsCallback
}
impl SnapshotCreatorImpl {
  pub const kDefaultContextIndex: usize = 0;
  pub const kFirstAddtlContextIndex: usize = Self::kDefaultContextIndex + 1;
  fn InitInternal(&mut self, blob: *const internal::StartupData) {
        unsafe{
          (*self.isolate_).set_enable_serializer();
          (*self.isolate_).Enter();

          if !blob.is_null() && (*blob).raw_size > 0 {
              (*self.isolate_).set_snapshot_blob(blob);
              internal::Snapshot::Initialize(self.isolate_);
          } else {
              (*self.isolate_).InitWithoutSnapshot();
          }
        }


        self.contexts_.push(SerializableContext{handle_location:std::ptr::null_mut(), callback: internal::SerializeEmbedderFieldsCallback{}});
        assert_eq!(self.contexts_.len(), Self::kDefaultContextIndex + 1);
  }
  pub fn FromSnapshotCreator(snapshot_creator: *mut v8::SnapshotCreator) -> *mut SnapshotCreatorImpl {
     unsafe { (*snapshot_creator).impl_ }
  }
  pub fn new(
      isolate: *mut internal::Isolate,
      api_external_references: *const i64,
      existing_blob: *const internal::StartupData,
      owns_isolate: bool,
  ) -> Self {
       let isolate_ = if isolate.is_null() {
            internal::Isolate::New()
        } else {
            isolate
        };
        let mut array_buffer_allocator_ = internal::ArrayBuffer::Allocator::NewDefaultAllocator();
        unsafe {
            (*isolate_).set_array_buffer_allocator(array_buffer_allocator_.as_mut().map(|alloc| &mut **alloc).unwrap() as *mut internal::ArrayBufferAllocator);
            (*isolate_).set_api_external_references(api_external_references);
        }
        let mut this = Self {
            owns_isolate_: owns_isolate,
            isolate_: isolate_,
            array_buffer_allocator_: array_buffer_allocator_,
            contexts_: Vec::new(),
        };
        this.InitInternal(if !existing_blob.is_null() {
            existing_blob
        } else {
            internal::Snapshot::DefaultSnapshotBlob()
        });
        this
    }

    pub fn new2(params: &v8::internal::IsolateCreateParams) -> Self{
        let owns_isolate_ = true;
        let isolate_ = internal::Isolate::New();

         let mut array_buffer_allocator_ = if let Some(allocator) = &params.array_buffer_allocator_shared {
          unsafe {
             assert!(params.array_buffer_allocator.is_null() || params.array_buffer_allocator == allocator.as_ref().map(|alloc| &**alloc).unwrap() as *const internal::ArrayBufferAllocator as *mut internal::ArrayBufferAllocator );
             (*isolate_).set_array_buffer_allocator(allocator.as_ref().map(|alloc| &**alloc).unwrap() as *mut internal::ArrayBufferAllocator);
             (*isolate_).set_array_buffer_allocator_shared(Some(allocator.clone()));

          }
          None

        } else {
          unsafe {
             assert!(!params.array_buffer_allocator.is_null());
             (*isolate_).set_array_buffer_allocator(params.array_buffer_allocator);
          }
           Some(internal::ArrayBuffer::Allocator::NewDefaultAllocator())

        };


        unsafe {
            (*isolate_).set_api_external_references(params.external_references);
            (*(*isolate_).heap()).ConfigureHeap(params.constraints, params.cpp_heap);
        }

       let mut this = Self {
            owns_isolate_: owns_isolate_,
            isolate_: isolate_,
            array_buffer_allocator_:  if let Some(allocator) = array_buffer_allocator_ {
               allocator
            }else{
               internal::ArrayBuffer::Allocator::NewDefaultAllocator()
            },
            contexts_: Vec::new(),
        };
        this.InitInternal(if !params.snapshot_blob.is_null() {
            params.snapshot_blob
        } else {
            internal::Snapshot::DefaultSnapshotBlob()
        });
        this
    }

    pub fn new3(isolate: *mut internal::Isolate, params: &v8::internal::IsolateCreateParams) -> Self {
        let owns_isolate_ = false
