// Converted from V8 C++ source files:
// Header: read-only-deserializer.h
// Implementation: read-only-deserializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod read_only_deserializer {
use crate::snapshot::deserializer::Address;
use crate::snapshot::snapshot::SnapshotData;
use crate::v8::internal::ReadOnlyHeapImageDeserializer;
use crate::v8::internal::SnapshotByteSource;
use crate::v8::internal::ExternalPointerSlot;
use crate::v8::internal::ro;
use crate::v8::internal::ReadOnlySpace;
use crate::v8::internal::ReadOnlyPageMetadata;
use crate::v8::internal::ReadOnlyHeap;
use crate::v8::internal::ReadOnlyRoots;
use crate::v8::internal::EmbeddedData;
use crate::v8::internal::Isolate;
use crate::v8::internal::HeapObject;
use crate::v8::internal::InstanceType;
use crate::v8::internal::String;
use crate::v8::internal::AccessorInfo;
use crate::v8::internal::FunctionTemplateInfo;
use crate::v8::internal::Code;
use crate::v8::internal::SharedFunctionInfo;
use crate::v8::internal::Heap;
use crate::v8::internal::FixedArray;
use std::time::Instant;
use std::ptr;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::v8::internal::PtrComprCageBase;
use crate::v8::internal::Name;
use crate::v8::internal::direct_handle;
use crate::v8::internal::V8HeapCompressionScheme;
use crate::v8::internal::ExternalPointerTable;

#[derive(Debug)]
pub enum ReadOnlyDeserializerError {
    GenericError(String),
    InvalidMagicNumber,
    OutOfMemory,
    IoError(std::io::Error),
}

impl From<std::io::Error> for ReadOnlyDeserializerError {
    fn from(err: std::io::Error) -> Self {
        ReadOnlyDeserializerError::IoError(err)
    }
}

pub struct ReadOnlyDeserializer<'a> {
    isolate: &'a mut Isolate,
    data: &'a SnapshotData,
    source: SnapshotByteSource,
    can_rehash: bool,
    deserialized_objects: Vec<Address>,
}

impl<'a> ReadOnlyDeserializer<'a> {
    pub fn new(isolate: &'a mut Isolate, data: &'a SnapshotData, can_rehash: bool) -> Self {
        let payload = data.payload();
        let magic_number = data.get_magic_number();
        ReadOnlyDeserializer {
            isolate,
            data,
            source: SnapshotByteSource::new(payload.to_vec()),
            can_rehash,
            deserialized_objects: Vec::new(),
        }
    }

    pub fn deserialize_into_isolate(&mut self) -> Result<(), ReadOnlyDeserializerError> {
        let start = Instant::now();

        ReadOnlyHeapImageDeserializer::deserialize(self.isolate, &mut self.source);
        let ro_heap = self.isolate.read_only_heap();
        ro_heap.read_only_space().repair_free_spaces_after_deserialization();
        self.post_process_new_objects()?;

        let roots = ReadOnlyRoots::new(self.isolate);
        roots.verify_name_for_protectors_pages();
        roots.verify_types();
        roots.verify_name_for_protectors();

        if self.should_rehash() {
            self.isolate.heap_mut().initialize_hash_seed();
            self.rehash();
        }
        let elapsed = start.elapsed();
        println!(
            "[Deserializing read-only space ({} bytes) took {:.3} ms]",
            self.source.length(),
            elapsed.as_secs_f64() * 1000.0
        );

        Ok(())
    }

    fn should_rehash(&self) -> bool {
        self.can_rehash
    }

    fn rehash(&mut self) {
        for &addr in &self.deserialized_objects {
            println!("rehashing address: {:?}", addr);
        }
    }

    fn post_process_new_objects(&mut self) -> Result<(), ReadOnlyDeserializerError> {
        let cage_base = PtrComprCageBase::new(self.isolate);

        let mut post_processor = ObjectPostProcessor::new(self.isolate);
        let mut it = ReadOnlyHeapObjectIterator::new(self.isolate.read_only_heap());
        while let Some(o) = it.next() {
            let instance_type = o.map(&cage_base).instance_type();

            if self.should_rehash() {
                if let InstanceType::String = instance_type {
                    let str_obj = unsafe { HeapObject::cast::<String>(o) };
                    str_obj.set_raw_hash_field(Name::kEmptyHashField);
                    self.push_object_to_rehash(direct_handle(str_obj, self.isolate));
                } else if o.needs_rehashing(instance_type) {
                    self.push_object_to_rehash(direct_handle(o, self.isolate));
                }
            }

            post_processor.post_process_if_needed(o, instance_type);
        }

        post_processor.finalize();
        Ok(())
    }

    fn push_object_to_rehash(&mut self, handle: Address) {
        self.deserialized_objects.push(handle);
    }
}

struct ReadOnlyHeapObjectIterator<'a> {
    ro_heap: &'a ReadOnlyHeap,
    current_page_index: usize,
    current_object_offset: usize,
}

impl<'a> ReadOnlyHeapObjectIterator<'a> {
    fn new(ro_heap: &'a ReadOnlyHeap) -> Self {
        ReadOnlyHeapObjectIterator {
            ro_heap,
            current_page_index: 0,
            current_object_offset: 0,
        }
    }

    fn next(&mut self) -> Option<HeapObject> {
        loop {
            if self.current_page_index >= self.ro_heap.read_only_space().pages().len() {
                return None;
            }

            let page = &self.ro_heap.read_only_space().pages()[self.current_page_index];
            let page_start = page.area_start();
            let page_end = page.area_end();

            if self.current_object_offset >= (page_end as usize - page_start as usize) {
                self.current_page_index += 1;
                self.current_object_offset = 0;
                continue;
            }

            let object_address = page_start as usize + self.current_object_offset;
            let object = unsafe { HeapObject::unchecked_from_address(object_address as Address) };
            
            self.current_object_offset += 16; 

            return Some(object);
        }
    }
}

fn no_external_references_callback() {
    panic!("No external references provided via API");
}

struct ObjectPostProcessor<'a> {
    isolate: &'a mut Isolate,
    embedded_data: EmbeddedData,
    external_pointer_slots_: Vec<ExternalPointerSlot>,
}

impl<'a> ObjectPostProcessor<'a> {
    fn new(isolate: &'a mut Isolate) -> Self {
        ObjectPostProcessor {
            isolate,
            embedded_data: EmbeddedData::from_blob(isolate),
            external_pointer_slots_: Vec::new(),
        }
    }

    fn finalize(&mut self) {
        let mut registry = Vec::new();
        registry.reserve(self.external_pointer_slots_.len());
        for slot in &self.external_pointer_slots_ {
          // registry.emplace_back(slot.relaxed_load_handle(), slot.load(self.isolate),
          //                          slot.exact_tag());
        }

        //self.isolate.read_only_artifacts().set_external_pointer_registry(registry);
    }

    fn post_process_if_needed(&mut self, o: HeapObject, instance_type: InstanceType) {
        match instance_type {
            InstanceType::AccessorInfo => {
                self.post_process_accessor_info(unsafe {HeapObject::cast::<AccessorInfo>(o)});
            }
            InstanceType::FunctionTemplateInfo => {
                self.post_process_function_template_info(unsafe {HeapObject::cast::<FunctionTemplateInfo>(o)});
            }
            InstanceType::Code => {
                self.post_process_code(unsafe {HeapObject::cast::<Code>(o)});
            }
            InstanceType::SharedFunctionInfo => {
                self.post_process_shared_function_info(unsafe {HeapObject::cast::<SharedFunctionInfo>(o)});
            }
            _ => {}
        }
    }

    fn get_any_external_reference_at(&self, index: i32, is_api_reference: bool) -> Address {
        if is_api_reference {
            let refs = self.isolate.api_external_references();
            let address = if refs.is_null() {
                no_external_references_callback as Address
            } else {
                unsafe { *refs.offset(index as isize) }
            };
            assert_ne!(address, 0);
            address
        } else {
            self.isolate.external_reference_table_unsafe().address(index)
        }
    }

    fn decode_external_pointer_slot(&mut self, host: HeapObject, slot: ExternalPointerSlot) {
        let encoded = ro::EncodedExternalReference::from_uint32(
            slot.get_content_as_index_after_deserialization() as u32
        );

        let slot_value = self.get_any_external_reference_at(encoded.index as i32, encoded.is_api_reference);
        //assert!(slot.exact_tag_is_known());

        slot.init(self.isolate, host, slot_value, 0);
        //self.external_pointer_slots_.push(slot);
    }

    fn post_process_accessor_info(&mut self, o: AccessorInfo) {
        // self.decode_external_pointer_slot(
        //     o,
        //     o.raw_external_pointer_field(AccessorInfo::kSetterOffset, 0) //kAccessorInfoSetterTag
        // );
        // self.decode_external_pointer_slot(
        //     o,
        //     o.raw_external_pointer_field(AccessorInfo::kMaybeRedirectedGetterOffset, 0) //kAccessorInfoGetterTag
        // );
        if true {
            //o.init_getter_redirection(self.isolate);
        }
    }

    fn post_process_function_template_info(&mut self, o: FunctionTemplateInfo) {
        // self.decode_external_pointer_slot(
        //     o,
        //     o.raw_external_pointer_field(FunctionTemplateInfo::kMaybeRedirectedCallbackOffset, 0) //kFunctionTemplateInfoCallbackTag
        // );
        if true {
           // o.init_callback_redirection(self.isolate);
        }
    }

    fn post_process_code(&mut self, o: Code) {
        //o.init_self_indirect_pointer(self.isolate);
        //o.wrapper().set_code(o);

        assert!(o.is_builtin());
        assert!(!o.has_instruction_stream());
        // o.set_instruction_start_for_off_heap_builtin(
        //     self.isolate,
        //     EmbeddedData::from_blob(self.isolate).instruction_start_of(o.builtin_id()),
        // );
    }

    fn post_process_shared_function_info(&mut self, o: SharedFunctionInfo) {
        o.set_unique_id(self.isolate.get_and_inc_next_unique_sfi_id());
    }
}

pub mod ro {
    use crate::snapshot::deserializer::Address;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Bytecode {
        kAllocatePage,
        kAllocatePageAt,
        kSegment,
        kRelocateSegment,
        kReadOnlyRootsTable,
        kFinalizeReadOnlySpace,
    }

    impl From<u8> for Bytecode {
        fn from(value: u8) -> Self {
            match value {
                0 => Bytecode::kAllocatePage,
                1 => Bytecode::kAllocatePageAt,
                2 => Bytecode::kSegment,
                3 => Bytecode::kRelocateSegment,
                4 => Bytecode::kReadOnlyRootsTable,
                5 => Bytecode::kFinalizeReadOnlySpace,
                _ => panic!("Invalid bytecode value: {}", value),
            }
        }
    }

    pub const kNumberOfBytecodes: usize = 6;
    pub struct EncodedTagged {
        pub page_index: usize,
        pub offset: usize,
    }

    impl EncodedTagged {
        pub fn from_uint32(encoded: u32) -> Self {
            EncodedTagged {
                page_index: (encoded >> 20) as usize,
                offset: (encoded & 0xFFFFF) as usize,
            }
        }

        pub fn from_address(addr: Address) -> Self {
            EncodedTagged {
                page_index: 0,
                offset: 0
            }
        }
    }
    pub struct EncodedExternalReference {
        pub index: usize,
        pub is_api_reference: bool,
    }

    impl EncodedExternalReference {
        pub fn from_uint32(encoded: u32) -> Self {
            EncodedExternalReference {
                index: (encoded >> 1) as usize,
                is_api_reference: (encoded & 1) != 0,
            }
        }
    }

    pub struct BitSet {
        data: *mut u8,
        size_in_bits: usize,
    }

    impl BitSet {
        pub fn new(data: *mut u8, size_in_bits: usize) -> Self {
            BitSet { data, size_in_bits }
        }

        pub fn contains(&self, index: i32) -> bool {
            true
        }

        pub fn size_in_bytes(&self) -> usize {
            1
        }
    }
}

pub mod internal {
    use crate::snapshot::deserializer::Address;
    use crate::v8::internal::ro;
    use crate::v8::internal::Isolate;
    use crate::v8::internal::ReadOnlySpace;
    use crate::v8::internal::ReadOnlyPageMetadata;
    use crate::v8::internal::ReadOnlyRoots;
    use crate::v8::internal::SnapshotByteSource;
    use crate::v8::internal::PtrComprCageBase;
    use crate::v8::internal::ExternalPointerSlot;
    use std::mem;
    use std::ptr;

    pub struct ReadOnlyHeapImageDeserializer<'a> {
        source_: &'a mut SnapshotByteSource,
        isolate_: &'a mut Isolate,
    }

    impl<'a> ReadOnlyHeapImageDeserializer<'a> {
        pub fn deserialize(isolate: &'a mut Isolate, source: &'a mut SnapshotByteSource) {
            let mut deserializer = ReadOnlyHeapImageDeserializer {
                source_: source,
                isolate_: isolate,
            };
            deserializer.deserialize_impl();
        }

        fn deserialize_impl(&mut self) {
            loop {
                let bytecode_as_int = self.source_.get();
                assert!(bytecode_as_int < ro::kNumberOfBytecodes as u8);

                match ro::Bytecode::from(bytecode_as_int) {
                    ro::Bytecode::kAllocatePage => {
                        self.allocate_page(false);
                    }
                    ro::Bytecode::kAllocatePageAt => {
                        self.allocate_page(true);
                    }
                    ro::Bytecode::kSegment => {
                        self.deserialize_segment();
                    }
                    ro::Bytecode::kRelocateSegment => {
                        unreachable!();
                    }
                    ro::Bytecode::kReadOnlyRootsTable => {
                        self.deserialize_read_only_roots_table();
                    }
                    ro::Bytecode::kFinalizeReadOnlySpace => {
                        self.ro_space().finalize_space_for_deserialization();
                        return;
                    }
                }
            }
        }

        fn allocate_page(&mut self, fixed_offset: bool) {
            let expected_page_index = self.source_.get_uint30() as usize;
            let actual_page_index: usize;
            let area_size_in_bytes = self.source_.get_uint30() as usize;

            if fixed_offset {
                let compressed_page_addr = self.source_.get_uint32();
                let pos = self.isolate_.cage_base() + compressed_page_addr as Address;
                actual_page_index = self.ro_space().allocate_next_page_at(pos);
            } else {
                actual_page_index = self.ro_space().allocate_next_page();
            }

            assert_eq!(actual_page_index, expected_page_index);
            self.ro_space().initialize_page_for_deserialization(
                self.page_at(actual_page_index),
                area_size_in_bytes,
            );
        }

        fn deserialize_segment(&mut self) {
            let page_index = self.source_.get_uint30() as u32;
            let page = self.page_at(page_index as usize);
            let start = page.area_start() + self.source_.get_uint30();
            let size_in_bytes = self.source_.get_uint30() as usize;

            assert!(start + size_in_bytes as Address <= page.area_end());
            self.source_.copy_raw(start as *mut std::ffi::c_void, size_in_bytes);

            let relocate_marker_bytecode = self.source_.get();
            assert_eq!(relocate_marker_bytecode, ro::Bytecode::kRelocateSegment as u8);

            let tagged_slots_size_in_bits = size_in_bytes / 8;

            let data = self.source_.data().as_ptr();
            let data = unsafe { data.offset(self.source_.position() as isize) as *mut u8};
            let tagged_slots = ro::BitSet::new(data, tagged_slots_size_in_bits);
            self.decode_tagged_slots(start, &tagged_slots);
            self.source_.advance(tagged_slots.size_in_bytes() as i32);
        }

        fn decode(&self, encoded: ro::EncodedTagged) -> Address {
            let page = self.page_at(encoded.page_index);
            page.offset_to_address(encoded.offset * 8)
        }

        fn decode_tagged_slots(&mut self, segment_start: Address, tagged_slots: &ro::BitSet) {
            for i in 0..tagged_slots.size_in_bits() {
                if !tagged_slots.contains(i as i32) {
                    continue;
                }

                let slot_addr = segment_start + (i * 8) as Address;
                let obj_addr = self.decode(ro::EncodedTagged::from_address(slot_addr));
                let obj_ptr = obj_addr + 1;

                let dst = slot_addr as *mut Address;

                unsafe {
                  *dst = obj_ptr;
                }
            }
        }

        fn page_at(&self, index: usize) -> &mut ReadOnlyPageMetadata {
            assert!(index < self.ro_space().pages().len());
            unsafe {
                &mut *(self.ro_space().pages()[index] as *mut ReadOnlyPageMetadata)
            }
        }

        fn deserialize_read_only_roots_table(&mut self) {
            let mut roots = ReadOnlyRoots::new(self.isolate_);

            for i in 0..ReadOnlyRoots::k_entries_count() {
                let encoded_as_int = self.source_.get_uint32();
                let rudolf = self.decode(ro::EncodedTagged::from_uint32(encoded_as_int));
                roots.read_only_roots_mut()[i] = rudolf + 1;
            }
        }

        fn ro_space(&self) -> &mut ReadOnlySpace {
            self.isolate_.read_only_heap().read_only_space_mut()
        }
    }
}

pub struct SnapshotByteSource {
    data: Vec<u8>,
    position: usize,
}

impl SnapshotByteSource {
    pub fn new(data: Vec<u8>) -> Self {
        SnapshotByteSource { data, position: 0 }
    }

    pub fn get(&mut self) -> u8 {
        let value = self.data[self.position];
        self.position += 1;
        value
    }

    pub fn get_uint30(&mut self) -> u32 {
        let mut value: u32 = 0;
        value |= (self.get() as u32) << 0;
        value |= (self.get() as u32) << 8;
        value |= (self.get() as u32) << 16;
        value
    }

    pub fn get_uint32(&mut self) -> u32 {
        let mut value: u32 = 0;
        value |= (self.get() as u32) << 0;
        value |= (self.get() as u32) << 8;
        value |= (self.get() as u32) << 16;
        value |= (self.get() as u32) << 24;
        value
    }

    pub fn copy_raw(&mut self, dest: *mut std::ffi::c_void, size_in_bytes: usize) {
        unsafe {
            ptr::copy_nonoverlapping(
                self.data.as_ptr().add(self.position) as *const std::ffi::c_void,
                dest,
                size_in_bytes,
            );
        }
        self.position += size_in_bytes;
    }

    pub fn advance(&mut self, count: i32) {
        self.position += count as usize;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}
}
