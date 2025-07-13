// Converted from V8 C++ source files:
// Header: serializer.h
// Implementation: serializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod serializer {
    use std::any::Any;
    use std::cell::RefCell;
    use std::cmp::max;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::ptr;
    use std::rc::Rc;
    use base::HashMap as BaseHashMap;
    use bitflags::bitflags;
    use failure::Error;
    use crate::execution::isolate::Isolate;
    use crate::handles::global_handles::GlobalHandle;
    use crate::logging::log::Log;
    use crate::objects::abstract_code::AbstractCode;
    use crate::objects::bytecode_array::BytecodeArray;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::instruction_stream::InstructionStream;
    use crate::objects::objects::Object;
    use crate::snapshot::serializer_deserializer::SerializerDeserializer;
    use crate::snapshot::snapshot_source_sink::SnapshotByteSink;
    use crate::snapshot::snapshot::Snapshot;
    use crate::utils::identity_map::IdentityMap;
    use crate::codegen::external_reference_encoder::ExternalReferenceEncoder;
    use crate::execution::frames::PtrComprCageBase;
    use crate::objects::map::Map;
    use crate::heap::heap::Heap;
    use crate::handles::handles::Handle;
    use crate::objects::smi::Smi;
    use crate::heap::read_only_heap::ReadOnlyHeap;
    use crate::heap::spaces::AllocationSpace;
    use crate::heap::mutable_page_metadata::MutablePageMetadata;
    use crate::objects::instance_type::InstanceType;
    use crate::objects::slots::SlotType;
    use crate::objects::fixed_array::FixedArray;
    use std::marker::PhantomData;
    use crate::objects::thin_string::ThinString;
    use crate::objects::code::Code;
    use crate::objects::script::Script;
    use crate::objects::js_array_buffer::JSArrayBuffer;
    use crate::objects::external_string::ExternalString;
    use crate::objects::js_typed_array::JSTypedArray;
    use crate::objects::seq_string::SeqString;
    use crate::objects::descriptor_array::DescriptorArray;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::sandbox::js_dispatch_table::JSDispatchHandle;
    use crate::objects::trusted_object::TrustedObject;
    use crate::objects::js_object::JSObject;
    use crate::compiler::call_descriptor::CallDescriptor;
    use crate::objects::instance_type_checker::InstanceTypeChecker;
    use crate::heap::spaces::ReadOnlySpace;
    use crate::objects::maybe_object::MaybeObject;
    use crate::objects::embedder_data_array::EmbedderDataArray;
    use crate::objects::js_synchronization_primitive::JSSynchronizationPrimitive;
    use crate::wasm::wasm_code::WasmCode;
    use std::sync::Mutex;
    use std::sync::RwLock;
    use crate::objects::free_space::FreeSpaceOrFiller;
    use crate::compiler::js_create_lowering::GCType;
    use crate::objects::descriptor_array::DescriptorArrayMarkingState;
    use crate::objects::slots::HeapObjectReferenceType;
    use crate::execution::local_isolate::ReadOnlyRoots;
    use crate::handles::handles::DirectHandle;
    use crate::heap::gc_state::HeapObjectReference;
    use crate::compiler::turboshaft::operations::MemoryRepresentation;
    use crate::compiler::turboshaft::operations::StoreOp;
    use crate::compiler::turboshaft::operations::WriteBarrierKind;
    use crate::objects::native_context::NativeContext;
    use crate::handles::direct_handles::MaybeDirectHandle;
    use crate::wasm::value_type::ValueType;
    use crate::wasm::value_type::ValueTypeKind;
    use crate::wasm::value_type::HeapTypeRepresentation;
    use crate::compiler::backend::instruction_selector_adapter::OpIndex;
    use std::ffi::c_void;
    use crate::deoptimizer::frame_description::FrameDescription;
    use crate::compiler::pipeline_data::Common;
    use crate::compiler::ts_call_descriptor::TSCallDescriptor;
    use crate::execution::isolate_group::IsolateGroup;
    use crate::asmjs::asm_js::StandardMember;
    use crate::zone::zone::Zone;

    #[allow(dead_code)]
    pub struct CodeAddressMap {
        code_event_logger: CodeEventLogger,
    }

    impl CodeAddressMap {
        pub fn new(isolate: *mut Isolate) -> Self {
            unsafe {
                let isolate_ref = isolate.as_mut().unwrap();
                let logger = isolate_ref.logger();
                logger.add_listener(&Self { code_event_logger: CodeEventLogger::new(isolate) });
            }

            Self {
                code_event_logger: CodeEventLogger::new(isolate),
            }
        }

        pub fn lookup(&self, address: usize) -> Option<&str> {
            self.code_event_logger.address_to_name_map.lookup(address)
        }
    }

    impl Drop for CodeAddressMap {
        fn drop(&mut self) {
            unsafe {
                self.code_event_logger.isolate.as_mut().unwrap().logger().remove_listener(self);
            }
        }
    }

    #[allow(dead_code)]
    struct CodeEventLogger {
        isolate: *mut Isolate,
        address_to_name_map: NameMap,
    }

    impl CodeEventLogger {
        pub fn new(isolate: *mut Isolate) -> Self {
            Self {
                isolate,
                address_to_name_map: NameMap::new(),
            }
        }

        pub fn code_move_event(&mut self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>) {
            self.address_to_name_map.move_entry(from.ptr(), to.ptr());
        }

        pub fn bytecode_move_event(&mut self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>) {
            self.address_to_name_map.move_entry(from.ptr(), to.ptr());
        }

        pub fn code_disable_opt_event(&mut self, code: DirectHandle<AbstractCode>, shared: DirectHandle<SharedFunctionInfo>) {}

        pub fn log_recorded_buffer(&mut self, code_address: usize, name: &str) {
            self.address_to_name_map.insert(code_address, name);
        }
    }

    trait Listener {
        fn code_move_event(&mut self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>);
        fn bytecode_move_event(&mut self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>);
        fn code_disable_opt_event(&mut self, code: DirectHandle<AbstractCode>, shared: DirectHandle<SharedFunctionInfo>);
        fn log_recorded_buffer(&mut self, code_address: usize, name: &str);
    }

    #[allow(dead_code)]
    struct NameMap {
        impl_: BaseHashMap<usize, String>,
    }

    impl NameMap {
        pub fn new() -> Self {
            Self {
                impl_: BaseHashMap::new(),
            }
        }

        pub fn insert(&mut self, code_address: usize, name: &str) {
            self.impl_.insert(code_address, name.to_string());
        }

        pub fn lookup(&self, code_address: usize) -> Option<&str> {
            self.impl_.get(&code_address).map(|s| s.as_str())
        }

        pub fn remove(&mut self, code_address: usize) {
            self.impl_.remove(&code_address);
        }

        pub fn move_entry(&mut self, from: usize, to: usize) {
            if from == to {
                return;
            }
            if let Some(value) = self.impl_.remove(&from) {
                self.impl_.insert(to, value);
            }
        }
    }

    #[derive(Debug)]
    pub enum ObjectCacheIndexMapError {
        NotFound,
    }

    pub struct ObjectCacheIndexMap {
        map_: IdentityMap<i32, base::DefaultAllocationPolicy>,
        next_index_: i32,
        phantom: PhantomData<base::DefaultAllocationPolicy>
    }

    impl ObjectCacheIndexMap {
        pub fn new(heap: *mut Heap) -> Self {
            Self {
                map_: IdentityMap::new(),
                next_index_: 0,
                phantom: PhantomData
            }
        }

        pub fn lookup_or_insert(&mut self, obj: Tagged<HeapObject>, index_out: &mut i32) -> bool {
            if let Some(index) = self.map_.find(obj) {
                *index_out = *index;
                return true;
            } else {
                self.map_.insert(obj, self.next_index_);
                *index_out = self.next_index_;
                self.next_index_ += 1;
                return false;
            }
        }

        pub fn lookup_or_insert_handle(&mut self, obj: DirectHandle<HeapObject>, index_out: &mut i32) -> bool {
            self.lookup_or_insert(*obj, index_out)
        }

        pub fn lookup(&self, obj: Tagged<HeapObject>, index_out: &mut i32) -> bool {
            if let Some(index) = self.map_.find(obj) {
                *index_out = *index;
                true
            } else {
                false
            }
        }

        pub fn values(&self, isolate: *mut Isolate) -> DirectHandle<FixedArray> {
            unsafe {
                if self.next_index_ == 0 {
                    return isolate.as_mut().unwrap().factory().empty_fixed_array();
                }
                let externals = isolate.as_mut().unwrap().factory().new_fixed_array(self.next_index_ as usize);
                let raw = *externals;

                for (key, value) in self.map_.iter() {
                    raw.set(key as usize, Smi::from(value));
                }
                externals
            }
        }

        pub fn size(&self) -> i32 {
            self.next_index_
        }
    }

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SerializerFlags: u32 {
            const kAllowUnknownExternalReferencesForTesting = 1 << 0;
            const kAllowActiveIsolateForTesting = 1 << 1;
            const kReconstructReadOnlyAndSharedObjectCachesForTesting = 1 << 2;
        }
    }

    pub struct Serializer<'a> {
        isolate_: *mut Isolate,
        hot_objects_: HotObjectsList<'a>,
        reference_map_: SerializerReferenceMap,
        external_reference_encoder_: ExternalReferenceEncoder,
        root_index_map_: RootIndexMap<'a>,
        code_address_map_: Option<Box<CodeAddressMap>>,
        code_buffer_: Vec<u8>,
        deferred_objects_: GlobalHandleVector<HeapObject>,
        num_back_refs_: i32,
        dispatch_handle_map_: HashMap<JSDispatchHandle, u32>,
        next_forward_ref_id_: i32,
        unresolved_forward_refs_: i32,
        forward_refs_per_pending_object_: IdentityMap<PendingObjectReferences, base::DefaultAllocationPolicy>,
        seen_backing_stores_index_: u32,
        recursion_depth_: i32,
        flags_: SerializerFlags,
        serializer_tracks_serialization_statistics_: bool,
        allocation_size_: [usize; kNumberOfSnapshotSpaces],
        pub sink_: SnapshotByteSink,
        no_gc_: DisallowGarbageCollection,
        cage_base_: PtrComprCageBase,
        back_refs_: GlobalHandleVector<HeapObject>,
        stack_: GlobalHandleVector<HeapObject>,
        instance_type_count_: [Option<Box<[i32; LAST_TYPE_PLUS_ONE]>>; kNumberOfSnapshotSpaces],
        instance_type_size_: [Option<Box<[usize; LAST_TYPE_PLUS_ONE]>>; kNumberOfSnapshotSpaces],
    }

    const LAST_TYPE_PLUS_ONE: usize = LAST_TYPE + 1;
    const KNUMBER_OF_SNAPSHOT_SPACES: usize = kNumberOfSnapshotSpaces;

    impl<'a> Serializer<'a> {
        pub fn new(isolate: *mut Isolate, flags: SerializerFlags) -> Self {
            unsafe {
                let mut instance_type_count_array: [Option<Box<[i32; LAST_TYPE_PLUS_ONE]>>; kNumberOfSnapshotSpaces] = [None, None, None];
                let mut instance_type_size_array: [Option<Box<[usize; LAST_TYPE_PLUS_ONE]>>; kNumberOfSnapshotSpaces] = [None, None, None];

                if (*isolate).flags().serialization_statistics {
                    for space in 0..kNumberOfSnapshotSpaces {
                        instance_type_count_array[space] = Some(Box::new([0i32; LAST_TYPE_PLUS_ONE]));
                        instance_type_size_array[space] = Some(Box::new([0usize; LAST_TYPE_PLUS_ONE]));
                    }
                }

                Self {
                    isolate_: isolate,
                    hot_objects_: HotObjectsList::new((*isolate).heap()),
                    reference_map_: SerializerReferenceMap::new(isolate),
                    external_reference_encoder_: ExternalReferenceEncoder::new(isolate),
                    root_index_map_: RootIndexMap::new(isolate),
                    code_address_map_: None,
                    code_buffer_: Vec::new(),
                    deferred_objects_: GlobalHandleVector::new((*isolate).heap()),
                    num_back_refs_: 0,
                    dispatch_handle_map_: HashMap::new(),
                    next_forward_ref_id_: 0,
                    unresolved_forward_refs_: 0,
                    forward_refs_per_pending_object_: IdentityMap::new(),
                    seen_backing_stores_index_: 1,
                    recursion_depth_: 0,
                    flags_: flags,
                    serializer_tracks_serialization_statistics_: true,
                    allocation_size_: [0; kNumberOfSnapshotSpaces],
                    sink_: SnapshotByteSink::new(),
                    no_gc_: DisallowGarbageCollection::new(),
                    cage_base_: PtrComprCageBase::new(isolate),
                    back_refs_: GlobalHandleVector::new((*isolate).heap()),
                    stack_: GlobalHandleVector::new((*isolate).heap()),
                    instance_type_count_: instance_type_count_array,
                    instance_type_size_: instance_type_size_array,
                }
            }
        }

        pub fn payload(&self) -> &Vec<u8> {
            self.sink_.data()
        }

        pub fn reference_map_contains(&self, o: DirectHandle<HeapObject>) -> bool {
            self.reference_map_.lookup_reference(o).is_some()
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn cage_base(&self) -> PtrComprCageBase {
            self.cage_base_
        }

        pub fn total_allocation_size(&self) -> usize {
            self.allocation_size_.iter().sum()
        }

        pub fn is_not_mapped_symbol(&self, obj: Tagged<HeapObject>) -> bool {
            unsafe {
                if self.flags_.contains(SerializerFlags::kAllowActiveIsolateForTesting) {
                    obj == (*self.isolate_).read_only_roots().not_mapped_symbol()
                } else {
                    obj == (*self.isolate_).heap().not_mapped_symbol()
                }
            }
        }

        pub fn serialize_deferred_objects(&mut self) {
            unsafe {
                if (*self.isolate_).flags().trace_serializer {
                    println!("Serializing deferred objects");
                }
                while !self.deferred_objects_.is_empty() {
                    let obj = Handle::from_raw(self.deferred_objects_.pop().unwrap(), self.isolate_);
                    let mut obj_serializer = ObjectSerializer::new(self, obj, &mut self.sink_);
                    obj_serializer.serialize_deferred();
                }
                self.sink_.put(kSynchronize, "Finished with deferred objects");
            }
        }

        pub fn serialize_object(&mut self, obj: Handle<HeapObject>, slot_type: SlotType) {
            if ThinString::is_thin_string(*obj, unsafe { &(*self.isolate_).cage_base() }) {
                let thin_string = unsafe { ThinString::unchecked_cast(*obj) };
                self.serialize_object(Handle::from_raw(thin_string.actual(), self.isolate_), slot_type);
            } else if Code::is_code(*obj, unsafe { &(*self.isolate_).cage_base() }) {
                let code = unsafe { Code::unchecked_cast(*obj) };
                if code.kind() == crate::objects::code::CodeKind::BASELINE {
                    self.serialize_object(Handle::from_raw(code.bytecode_or_interpreter_data(), self.isolate_), slot_type);
                } else {
                    assert!(code.is_builtin());
                }
            } else {
                self.serialize_object_impl(obj, slot_type);
            }
        }

        pub fn must_be_deferred(&self, object: Tagged<HeapObject>) -> bool {
            false
        }

        pub fn visit_root_pointers(&mut self, root: RootIndex, description: &str, start: usize, end: usize) {
            for i in start..end {
                self.serialize_root_object(i);
            }
        }

        pub fn serialize_root_object(&mut self, slot: usize) {
            unsafe {
                let o = (*self.isolate_).root_if_const(slot);
                if Smi::is_smi(o) {
                    self.put_smi_root(slot);
                } else {
                    self.serialize_object(Handle::from_raw(o, self.isolate_), SlotType::kAnySlot);
                }
            }
        }

        pub fn put_root(&mut self, root_index: RootIndex) {
            unsafe {
                let root_index_value = root_index as i32;
                let object = Tagged::<HeapObject>::from_ptr((*self.isolate_).root(root_index) as usize);
                if (*self.isolate_).flags().trace_serializer {
                    println!(" Encoding root {}: {:?}", root_index_value, object);
                }

                if root_index_value < kRootArrayConstantsCount as i32 && !HeapLayout::in_young_generation(object) {
                    self.sink_.put(RootArrayConstant::encode(root_index), "RootConstant");
                } else {
                    self.sink_.put(kRootArray, "RootSerialization");
                    self.sink_.put_u30(root_index as u32, "root_index");
                    self.hot_objects_.add(object);
                }
            }
        }

        pub fn put_smi_root(&mut self, slot: usize) {
            unsafe {
                let raw_value = (*self.isolate_).root_if_const(slot);
                let smi = Smi::from_ptr(raw_value);
                self.sink_.put(FixedRawDataWithSize::encode(1), "Smi");
                self.sink_.put_raw(&(smi.value_ as i32).to_le_bytes(), 4, "Bytes");
            }
        }

        pub fn put_back_reference(&mut self, object: Tagged<HeapObject>, reference: SerializerReference) {
            unsafe {
                println!("put_back_reference {:?}", object);
                self.sink_.put_u30(reference.back_ref_index() as u32, "BackRefIndex");
                self.hot_objects_.add(object);
            }
        }

        pub fn put_attached_reference(&mut self, reference: SerializerReference) {
            self.sink_.put(kAttachedReference, "AttachedRef");
            self.sink_.put_u30(reference.attached_reference_index() as u32, "AttachedRefIndex");
        }

        pub fn put_repeat_root(&mut self, repeat_count: i32, root_index: RootIndex) {
            if repeat_count <= kLastEncodableFixedRepeatRootCount as i32 {
                self.sink_.put(FixedRepeatRootWithCount::encode(repeat_count as usize), "FixedRepeatRoot");
            } else {
                self.sink_.put(kVariableRepeatRoot, "VariableRepeatRoot");
                self.sink_.put_u30(VariableRepeatRootCount::encode(repeat_count as usize) as u32, "repeat count");
            }
            self.sink_.put(root_index as u8, "root index");
        }

        pub fn put_pending_forward_reference(&mut self, refs: &mut PendingObjectReferences) {
            self.sink_.put(kRegisterPendingForwardRef, "RegisterPendingForwardRef");
            self.unresolved_forward_refs_ += 1;
            let forward_ref_id = self.next_forward_ref_id_;
            self.next_forward_ref_id_ += 1;
            refs.push(forward_ref_id);
        }

        pub fn resolve_pending_forward_reference(&mut self, forward_reference_id: i32) {
            self.sink_.put(kResolvePendingForwardRef, "ResolvePendingForwardRef");
            self.sink_.put_u30(forward_reference_id as u32, "with this index");
            self.unresolved_forward_refs_ -= 1;

            if self.unresolved_forward_refs_ == 0 {
                self.next_forward_ref_id_ = 0;
            }
        }

        pub fn encode_external_reference(&mut self, addr: usize) -> ExternalReferenceEncoder::Value {
            self.external_reference_encoder_.try_encode(addr).unwrap()
        }

        pub fn register_object_is_pending(&mut self, obj: Tagged<HeapObject>) {
            if self.is_not_mapped_symbol(obj) {
                return;
            }
        }

        pub fn resolve_pending_object(&mut self, obj: Tagged<HeapObject>) {
            if self.is_not_mapped_symbol(obj) {
                return;
            }
        }

        pub fn pad(&mut self, padding_offset: usize) {
            for _ in 0..3 {
                self.sink_.put(kNop, "Padding");
            }

            while (self.sink_.position() + padding_offset) % 8 != 0 {
                self.sink_.put(kNop, "Padding");
            }
        }

        pub fn initialize_code_address_map(&mut self) {
            unsafe {
                (*self.isolate_).initialize_logging_and_counters();
                self.code_address_map_ = Some(Box::new(CodeAddressMap::new(self.isolate_)));
            }
        }

        pub fn copy_code(&mut self, istream: Tagged<InstructionStream>) -> Tagged<InstructionStream> {
            self.code_buffer_.clear();
            self.code_buffer_.resize(InstructionStream::kCodeAlignmentMinusCodeHeader);
            let size = istream.size();
            let istream_ptr = istream.ptr();
            let istream_address = istream_ptr as *const u8;
            let istream_slice = unsafe { std::slice::from_raw_parts(istream_address, size) };

            self.code_buffer_.extend_from_slice(istream_slice);
            unsafe {
                Tagged::<InstructionStream>::from_ptr(&self.code_buffer_[InstructionStream::kCodeAlignmentMinusCodeHeader])
            }
        }

        pub fn queue_deferred_object(&mut self, obj: Tagged<HeapObject>) {
            unsafe {
                self.deferred_objects_.push(obj);
            }
        }

        pub fn serializer_tracks_serialization_statistics(&self) -> bool {
            self.serializer_tracks_serialization_statistics_
        }

        pub fn set_serializer_tracks_serialization_statistics(&mut self, v: bool) {
            self.serializer_tracks_serialization_statistics_ = v;
        }

        pub fn allow_unknown_external_references_for_testing(&self) -> bool {
            self.flags_.contains(SerializerFlags::kAllowUnknownExternalReferencesForTesting)
        }

        pub fn allow_active_isolate_for_testing(&self) -> bool {
            self.flags_.contains(SerializerFlags::kAllowActiveIsolateForTesting)
        }

        pub fn reconstruct_read_only_and_shared_object_caches_for_testing(&self) -> bool {
            self.flags_.contains(SerializerFlags::kReconstructReadOnlyAndSharedObjectCachesForTesting)
        }

        pub fn deferred_objects_empty(&self) -> bool {
            self.deferred_objects_.is_empty()
        }

        pub fn output_statistics(&self, name: &str) {
            if unsafe { !(*self.isolate_).flags().serialization_statistics } {
                return;
            }

            println!("{}:", name);
            if !self.serializer_tracks_serialization_statistics() {
                println!("  <serialization statistics are not tracked>");
                return;
            }

            println!("  Spaces (bytes):");

            static K_ALL_SNAPSHOT_SPACES: [SnapshotSpace; 3] = [
                SnapshotSpace::kReadOnlyHeap,
                SnapshotSpace::kOld,
                SnapshotSpace::kCode,
            ];

            for space in &K_ALL_SNAPSHOT_SPACES {
                print!("{:16}", to_string(*space));
            }
            println!();

            for space in &K_ALL_SNAPSHOT_SPACES {
                print!("{:16}", self.allocation_size_[*space as usize]);
            }
            println!();

            // Implementation for verbose output, omitted due to complexity
        }
        fn serialize_object_impl(&mut self, obj: Handle<HeapObject>, slot_type: SlotType) {
            let mut obj_serializer = ObjectSerializer::new(self, obj, &mut self.sink_);
            obj_serializer.serialize(slot_type);
        }

        fn serialize_root(&mut self, obj: Tagged<HeapObject>) -> bool {
            let mut root_index: i32 = 0;

            unsafe {
                if self.root_index_map_.lookup(obj, &mut root_index) {
                    self.put_root(RootIndex::from(root_index as u8));
                    return true;
                }
            }

            false
        }

        fn serialize_hot_object(&mut self, obj: Tagged<HeapObject>) -> bool {
            let index = self.hot_objects_.find(obj);

            if index != HotObjectsList::K_NOT_FOUND {
                if unsafe { (*self.isolate_).flags().trace_serializer } {
                    println!(" Encoding hot object {}: {:?}", index, obj);
                }

                self.sink_.put(HotObject::encode(index), "HotObject");
                return true;
            }
            false
        }

        fn serialize_back_reference(&mut self, obj: Tagged<HeapObject>) -> bool {
            let reference = self.reference_map_.lookup_reference(obj);

            if reference.is_none() {
                return false;
            }

            let reference = reference.unwrap();

            if reference.is_attached_reference() {
                if unsafe { (*self.isolate_).flags().trace_serializer } {
                    println!(" Encoding attached reference {}", reference.attached_reference_index());
                }
                self.put_attached_reference(reference);
            } else {
                if unsafe { (*self.isolate_).flags().trace_serializer } {
                    println!(" Encoding back reference to: {:?}", obj);
                }
                self.sink_.put(kBackref, "Backref");
                self.put_back_reference(obj, reference);
            }

            true
        }

        fn serialize_pending_object(&mut self, obj: Tagged<HeapObject>) -> bool {
            let refs_to_object = self.forward_refs_per_pending_object_.find(obj);

            if refs_to_object.is_none() {
                return false;
            }

            self.put_pending_forward_reference(refs_to_object.unwrap());
            true
        }

        fn object_is_bytecode_handler(&self, obj: Tagged<HeapObject>) -> bool {
            if !Code::is_code(obj, unsafe { &(*self.isolate_).cage_base() }) {
                return false;
            }

            let code = unsafe { Code::unchecked_cast(obj) };

            code.kind() == crate::objects::code::CodeKind::BYTECODE_HANDLER
        }
    }

    impl<'a> Drop for Serializer<'a> {
        fn drop(&mut self) {
            assert_eq!(self.unresolved_forward_refs_, 0);
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum RootIndex {
        kInvalidRoot = 0,
        kArgumentsMarker,
        kArrayBoilerplateDescription,
        kArrayConstructor,
        kArrayIteratorMap,
        kArrayMap,
        kArrayValuesMap,
        kAsyncFunctionConstructor,
        kAsyncFunctionPrototype,
        kAsyncGeneratorFunctionConstructor,
        kAsyncGeneratorFunctionPrototype,
        kAsyncGeneratorObjectNext,
        kAsyncGeneratorObjectReturn,
        kAsyncGeneratorObjectThrow,
        kBooleanConstructor,
        kBoundFunctionMap,
        kBuiltins,
        kCall,
        kCatch,
        kClassBoilerplateDescription,
        kClassConstructor,
        kContinue,
        kDebugEvaluate,
        kDebugPrepareStepInIfStepping,
        kErrorToString,
        kEval,
        kFinalizeClassDefinition,
        kForAwaitValue,
        kFunctionApply,
        kFunctionBind,
        kFunctionConstructor,
        kGeneratorFunctionConstructor,
        kGeneratorFunctionPrototype,
        kGeneratorObjectNext,
        kGeneratorObjectReturn,
        kGeneratorObjectThrow,
        kHandleDebuggerStatement,
        kHandleError,
        kHandleDebuggerAbort,
        kHandleDebuggerBreak,
        kInfinityValue,
        kIsConcatSpreadableSymbol,
        kIteratorResultMap,
        kJSProxyConstructor,
        kJSProxyRevocableResultMap,
        kJsonStringify,
        kLazyCompile,
        kLazyCompileWithVector,
        kLookupIteratorMap,
        kMapConstructor,
        kMapIteratorMap,
        kMapMap,
        kMapValuesMap,
        kMathAcos,
        kMathAcosh,
        kMathAsin,
        kMathAsinh,
        kMathAtan,
        kMathAtan2,
        kMathAtanh,
        kMathCbrt,
        kMathCos,
        kMathCosh,
        kMathExp,
        kMathExpm1,
        kMathLog,
        kMathLog1p,
        kMathLog2,
        kMathLog10,
        kMathSin,
        kMathSinh,
        kMathSqrt,
        kMathTan,
        kMathTanh,
        kNumberConstructor,
        kObjectConstructor,
        kObjectHasOwnProperty,
        kObjectToString,
        kObjectToPrimitive,
        kPromiseAllResolveElement,
        kPromiseConstructor,
        kPromisePrototypeThen,
        kPromiseResolve,
        kProxyGetProperty,
        kProxySetProperty,
        kReflectApply,
        kRegExpConstructor,
        kStringConstructor,
        kStringCharCodeAt,
        kStringCodePointAt,
        kStringFromCharCode,
        kStringFromCodePoint,
        kStringIteratorMap,
        kStringReplace,
        kStringReplaceAll,
        kStringReplaceRegExp,
        kStringSplit,
        kStringSubstring,
        kSymbolConstructor,
        kThrow,
        kThrowApplyNonCallable,
        kThrowConstructNonCallable,
        kThrowIllegalInvocation,
        kToString,
        kTryCatchMap,
        kTryFinallyMap,
        kUint8ArrayToString,
        kWeakMapConstructor,
        kWeakMapMap,
        kWeakSetConstructor,
        kWeakSetMap,
        kWasmCompileLazy,
        kWasmGetOrCreateArrayBuffer,
        kSymbolToString,
        kBigIntConstructor,
        kBigIntAsUintN,
        kBigIntAsIntN,
        kBigIntAdd,
        kBigIntAnd,
        kBigIntCompare,
        kBigIntDivide,
        kBigIntEqualTo,
        kBigIntLeftShift,
        kBigIntLessThan,
        kBigIntLessThanOrEqual,
        kBigIntMultiply,
        kBigIntNot,
        kBigIntNotEqualTo,
        kBigIntOr,
        kBigIntRemainder,
        kBigIntRightShift,
        kBigIntSignedRightShift,
        kBigIntSubtract,
        kBigIntToString,
        kBigIntUnsignedRightShift,
        kBigIntXor,
        kPromiseAll,
        kPromiseAllSettled,
        kPromiseAny,
        kPromiseRace,
        kModuleCompileLazy,
        kGetImportMetaObject,
        kNotifyContextCreated,
        kNotifyContextDestroyed,
        kNotifyRuntimeAvailable,
        kAsyncFromSyncIteratorNext,
        kModuleEvaluate,
        kImportCall,
        kHostPromiseRejectionTracker,
        kSetSerializedObject,
        k
