// Converted from V8 C++ source files:
// Header: read-only-serializer.h
// Implementation: read-only-serializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
pub mod roots_serializer {
    pub struct RootsSerializer {}
}
pub mod snapshot {
    pub struct SerializerFlags {}
    pub struct Snapshot {}
    impl Snapshot {
        pub enum SerializerFlags {
        }
    }
}
pub mod heap {
    pub struct Heap {}
    impl Heap {
        pub fn read_only_heap(&self) -> &ReadOnlyHeap {
            todo!()
        }
    }
    pub struct ReadOnlyHeap {}
}
pub mod isolate {
    pub struct Isolate {}
    impl Isolate{
        pub fn heap(&mut self) -> &mut Heap {
            todo!()
        }
    }
}
pub mod common {
    pub mod globals{
        pub const kTaggedSize:usize = 8;
    }
}
pub mod objects{
    pub struct HeapObject{}
    impl HeapObject {
        pub fn map<T>(&self, isolate: T) -> &Map {
            todo!()
        }
        pub fn size(&self) -> usize {
            todo!()
        }
    }
    pub struct Map{

    }
}
pub mod snapshot_data{
    pub struct DisallowGarbageCollection{}
}
pub mod base{
    pub fn IsInRange(start:usize, pos:usize, high_water_mark:usize) -> bool{
        todo!()
    }
}
pub mod external_reference_encoder{
    pub struct ExternalReferenceEncoder{

    }
    impl ExternalReferenceEncoder{
        pub fn Encode(&mut self, value:usize) -> Value {
            todo!()
        }
    }
    pub struct Value{
        index:usize,
        is_from_api:bool,
    }
    impl Value{
        pub fn index(&self) -> usize{
            self.index
        }
        pub fn is_from_api(&self) -> bool{
            self.is_from_api
        }
    }
}
pub mod code_serializer{
    pub struct Code{

    }
    pub struct ReadOnlyRoots{

    }
}
pub mod read_only_serializer_deserializer{
    pub mod ro{
        pub struct EncodedExternalReference{

        }
        impl EncodedExternalReference{
            pub const kIndexBits:usize = 32;
            pub fn index(&self) -> usize {
                todo!()
            }
            pub fn is_api_reference(&self) -> bool {
                todo!()
            }
            pub fn FromUint32(content:u32) -> EncodedExternalReference {
                todo!()
            }
        }
        pub struct EncodedTagged{

        }
        impl EncodedTagged{
            pub const kPageIndexBits:usize = 32;
            pub const kSize:usize = 8;
            pub fn ToUint32(&self) -> u32 {
                todo!()
            }
        }
        pub struct BitSet{

        }
        impl BitSet{
            pub fn set(&mut self, slot:usize){
                todo!()
            }
            pub fn data(&self) -> *const u8 {
                todo!()
            }
            pub fn size_in_bytes(&self) -> usize {
                todo!()
            }
        }
    }
}
pub mod objects_inl{

}
pub mod slots{
    pub struct ObjectSlot{}
    pub struct MaybeObjectSlot{}
    pub struct ExternalPointerSlot{}
    pub struct InstructionStreamSlot{}
}
pub mod heap_inl{

}
pub mod visit_object{
    pub fn VisitObject<T,V>(isolate:T,obj:T,v:V){
        todo!()
    }
}
pub mod wasm{
    pub struct WasmNull{

    }
    impl WasmNull{
        pub fn payload(&self) -> usize {
            todo!()
        }
        pub const kPayloadSize:usize = 32;
    }
}
pub mod zone{
    pub struct Zone{}
}
pub mod external_reference_table{
    pub const kSize:usize = 32;
}
pub mod v8heapcompression{
    pub struct V8HeapCompressionScheme{}
    impl V8HeapCompressionScheme{
        pub fn CompressAny(page_addr:usize) -> u32{
            todo!()
        }
    }
}
pub mod free_space{
    pub struct FreeSpace{}
    impl FreeSpace{
        pub const kHeaderSize:usize = 32;
    }
}
pub mod tagged{
    pub struct Tagged<T>{}
    impl Tagged<objects::HeapObject>{
        pub fn address(&self) -> usize {
            todo!()
        }
        pub fn ptr(&self) -> usize {
            todo!()
        }
    }
    impl Tagged<WasmNull>{
        pub fn payload(&self) -> usize {
            todo!()
        }
    }
    impl Tagged<FreeSpace>{
    }
}
pub mod memory_chunk{
    pub struct MemoryChunkMetadata{}
    impl MemoryChunkMetadata{
        pub fn FromAddress(o_address:usize) -> *mut MemoryChunkMetadata {
            todo!()
        }
        pub fn Offset(&mut self, o_address:usize) -> usize {
            todo!()
        }
    }
}
pub mod ptr_compr_cage_base{
    pub struct PtrComprCageBase{}
    impl PtrComprCageBase{
        pub fn new<T>(t:T) -> PtrComprCageBase {
            todo!()
        }
    }
}
pub mod reloc_info{
    pub struct RelocInfo{}
}
pub mod snapshot_byte_sink{
    pub struct SnapshotByteSink{}
    impl SnapshotByteSink{
        pub fn Position(&self) -> i32 {
            todo!()
        }
        pub fn Put(&mut self,b:ro::Bytecode,s:&str) {
            todo!()
        }
        pub fn PutUint30(&mut self,b:u32,s:&str){
            todo!()
        }
        pub fn PutUint32(&mut self,b:u32,s:&str){
            todo!()
        }
        pub fn PutRaw(&mut self,b:*const u8,size:i32,s:&str){
            todo!()
        }
    }
}
pub mod flags{
    pub struct Flags{}
    impl Flags{
        pub fn serialization_statistics(&self) -> bool{
            todo!()
        }
    }
}
pub mod snapshot_space{
    pub enum SnapshotSpace{
        kReadOnlyHeap,
    }
}
pub mod instance_type{
    pub enum InstanceType{

    }
}
pub mod internal {
    use crate::roots_serializer::RootsSerializer;
    use crate::snapshot::SerializerFlags;
    use crate::isolate::Isolate;
    use crate::heap::ReadOnlyHeap;
    use crate::common::globals::kTaggedSize;
    use crate::objects::{HeapObject,Map};
    use crate::snapshot_data::DisallowGarbageCollection;
    use std::ptr;
    use crate::base::IsInRange;
    use std::vec::Vec;
    use crate::external_reference_encoder::{ExternalReferenceEncoder,Value};
    use crate::code_serializer::{Code,ReadOnlyRoots};
    use crate::read_only_serializer_deserializer::{ro,ro::EncodedTagged};
    use crate::objects_inl;
    use crate::slots::{ObjectSlot,MaybeObjectSlot,ExternalPointerSlot,InstructionStreamSlot};
    use crate::heap_inl;
    use crate::visit_object::VisitObject;
    use crate::wasm::WasmNull;
    use crate::zone::Zone;
    use crate::external_reference_table::kSize;
    use crate::v8heapcompression::V8HeapCompressionScheme;
    use crate::free_space::FreeSpace;
    use crate::tagged::Tagged;
    use crate::memory_chunk::MemoryChunkMetadata;
    use crate::ptr_compr_cage_base::PtrComprCageBase;
    use crate::reloc_info::RelocInfo;
    use crate::snapshot_byte_sink::SnapshotByteSink;
    use crate::flags::Flags;
    use crate::snapshot_space::SnapshotSpace;
    use crate::instance_type::InstanceType;

    pub struct ReadOnlySerializer {
        roots_serializer: RootsSerializer,
    }

    impl ReadOnlySerializer {
        pub fn new(isolate: *mut Isolate, flags: SerializerFlags) -> Self {
            ReadOnlySerializer {
                roots_serializer: RootsSerializer {},
            }
        }

        pub fn serialize(&mut self) {
            let mut no_gc = DisallowGarbageCollection {};
            let mut sink_ = SnapshotByteSink{};
            let mut i_isolate = unsafe {&mut *isolate};
            ReadOnlyHeapImageSerializer::Serialize(i_isolate, &mut sink_,
                                                   GetUnmappedRegions(i_isolate));

            // ReadOnlyHeapObjectIterator it(isolate()->read_only_heap());
            // for (Tagged<HeapObject> o = it.Next(); !o.is_null(); o = it.Next()) {
            //     CheckRehashability(o);
            //     if (v8_flags.serialization_statistics) {
            //         CountAllocation(o->map(), o->Size(), SnapshotSpace::kReadOnlyHeap);
            //     }
            // }
        }
    }

    struct InstanceTypeChecker {}

    impl InstanceTypeChecker {
        fn IsAccessorInfo(itype: InstanceType) -> bool {
            todo!()
        }
        fn IsFunctionTemplateInfo(itype: InstanceType) -> bool {
            todo!()
        }
        fn IsCode(itype: InstanceType) -> bool {
            todo!()
        }
    }

    struct ObjectPreProcessor {
        isolate_: *mut Isolate,
        extref_encoder_: ExternalReferenceEncoder,
    }

    impl ObjectPreProcessor {
        fn new(isolate: *mut Isolate) -> Self {
            ObjectPreProcessor {
                isolate_: isolate,
                extref_encoder_: ExternalReferenceEncoder {},
            }
        }

        fn PreProcessIfNeeded(&mut self, o: Tagged<HeapObject>) {
            let i_isolate = unsafe {&mut *self.isolate_};
            let itype = o.map(i_isolate)->instance_type();

            if InstanceTypeChecker::IsAccessorInfo(itype) {
                // return self.PreProcessAccessorInfo(Cast<AccessorInfo>(o));
                todo!()
            }
            if InstanceTypeChecker::IsFunctionTemplateInfo(itype) {
                // return self.PreProcessFunctionTemplateInfo(Cast<FunctionTemplateInfo>(o));
                todo!()
            }
            if InstanceTypeChecker::IsCode(itype) {
                // return self.PreProcessCode(Cast<Code>(o));
                todo!()
            }
        }
    }

    struct ReadOnlySegmentForSerialization {
        page: usize, //* const ReadOnlyPageMetadata,
        segment_start: usize,
        segment_size: usize,
        segment_offset: usize,
        contents: Vec<u8>, //std::unique_ptr<uint8_t[]>,
        tagged_slots: ro::BitSet,
    }

    impl ReadOnlySegmentForSerialization {
        fn new(isolate: *mut Isolate,
               page: usize, //const ReadOnlyPageMetadata* page,
               segment_start: usize,
               segment_size: usize,
               pre_processor: &mut ObjectPreProcessor)
               -> ReadOnlySegmentForSerialization {
            //let isolate_ = unsafe {&mut *isolate};
            //let ro_space = isolate_.read_only_heap().read_only_space();
            // assert!(segment_size % kTaggedSize == 0);
            let segment_offset = segment_start - page;
            let mut contents = vec![0u8; segment_size];
            // MemCopy(contents.get(), reinterpret_cast<void*>(segment_start),
            //         segment_size);
            let mut tagged_slots = ro::BitSet {}; //(segment_size / kTaggedSize);

            // .. because tagged_slots records a bit for each slot:
            //  DCHECK(IsAligned(segment_size, kTaggedSize));
            // Ensure incoming pointers to this page are representable.
            // CHECK_LT(isolate->read_only_heap()->read_only_space()->IndexOf(page),
            //          1UL << ro::EncodedTagged::kPageIndexBits);

            // MemCopy(contents.get(), reinterpret_cast<void*>(segment_start),
            //         segment_size);
            // PreProcessSegment(pre_processor);
            // if (!V8_STATIC_ROOTS_BOOL) EncodeTaggedSlots(isolate);

            ReadOnlySegmentForSerialization {
                page,
                segment_start,
                segment_size,
                segment_offset,
                contents,
                tagged_slots,
            }
        }
    }

    struct ReadOnlyHeapImageSerializer {
        isolate_: *mut Isolate,
        sink_: *mut SnapshotByteSink,
        pre_processor_: ObjectPreProcessor,
    }

    impl ReadOnlyHeapImageSerializer {
        fn new(isolate: *mut Isolate, sink: *mut SnapshotByteSink) -> Self {
            ReadOnlyHeapImageSerializer {
                isolate_: isolate,
                sink_: sink,
                pre_processor_: ObjectPreProcessor::new(isolate),
            }
        }

        fn Serialize(isolate: *mut Isolate, sink: *mut SnapshotByteSink,
                     unmapped_regions: Vec<MemoryRegion>) {
            let mut serializer =
                ReadOnlyHeapImageSerializer::new(isolate, sink);
            serializer.SerializeImpl(unmapped_regions);
        }

        fn SerializeImpl(&mut self, unmapped_regions: Vec<MemoryRegion>) {
            // let sink_ = unsafe {&mut *self.sink_};
            // DCHECK_EQ(sink_->Position(), 0);

            let isolate_ = unsafe {&mut *self.isolate_};
            // ReadOnlySpace* ro_space = isolate_->read_only_heap()->read_only_space();

            // Allocate all pages first s.t. the deserializer can easily handle forward
            // references (e.g.: an object on page i points at an object on page i+1).
            // for (const ReadOnlyPageMetadata* page : ro_space->pages()) {
            //     EmitAllocatePage(page, unmapped_regions);
            // }

            // Now write the page contents.
            // for (const ReadOnlyPageMetadata* page : ro_space->pages()) {
            //     SerializePage(page, unmapped_regions);
            // }

            // EmitReadOnlyRootsTable();
            let sink_ = unsafe {&mut *self.sink_};
            sink_.Put(ro::Bytecode::kFinalizeReadOnlySpace, "space end");
        }
    }

    #[derive(Debug)]
    struct MemoryRegion {
        start: usize,
        size: usize,
    }

    fn GetUnmappedRegions(isolate: *mut Isolate) -> Vec<MemoryRegion> {
        //     ReadOnlyRoots ro_roots(isolate);
        //     Tagged<WasmNull> wasm_null = ro_roots.wasm_null();
        //     Tagged<HeapObject> wasm_null_padding = ro_roots.wasm_null_padding();
        //     CHECK(IsFreeSpace(wasm_null_padding));
        //     Address wasm_null_padding_start =
        //         wasm_null_padding.address() + FreeSpace::kHeaderSize;
        //     std::vector<ReadOnlyHeapImageSerializer::MemoryRegion> unmapped;
        //     if (wasm_null.address() > wasm_null_padding_start) {
        //         unmapped.push_back({wasm_null_padding_start,
        //                             wasm_null.address() - wasm_null_padding_start});
        //     }
        //     unmapped.push_back({wasm_null->payload(), WasmNull::kPayloadSize});
        //     return unmapped;
        // #else
        return Vec::new();
        // #endif  // V8_STATIC_ROOTS
    }
} // namespace internal
} // namespace v8

mod v8 {
    mod internal {
        pub mod ro {
            pub enum Bytecode {
                kAllocatePageAt,
                kAllocatePage,
                kSegment,
                kRelocateSegment,
                kReadOnlyRootsTable,
                kFinalizeReadOnlySpace,
            }
        }
    }
}
