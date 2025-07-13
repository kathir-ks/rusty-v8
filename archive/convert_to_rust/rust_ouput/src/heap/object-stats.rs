// Converted from V8 C++ source files:
// Header: object-stats.h
// Implementation: object-stats.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Mutex;
use crate::FixedArray;
use crate::Object;
use crate::code;

pub struct InstructionStreamSlot {}
pub struct HeapObject {}
pub struct Heap{}
pub struct Isolate{}
pub struct String {}
pub struct MaybeObject {}
pub struct Tagged<T> {}
pub struct TrustedByteArray {}
pub struct FeedbackVector {}
pub struct FixedArrayBase {}
pub struct PropertyArray {}
pub struct JSObject {}
pub struct NameDictionary {}
pub struct PropertyDetails {}
pub struct FieldIndex {}
pub struct ObjectSlot {}
pub struct MaybeObjectSlot {}
pub struct EnumCache {}
pub struct WeakArrayList {}
pub struct Context {}
pub struct ExternalString {}
pub struct SharedFunctionInfo {}
pub struct DeoptimizationData {}
pub struct ArrayBoilerplateDescription {}
pub struct PtrComprCageBase {}
pub struct GlobalDictionary {}
pub struct Script {}
pub struct JSGlobalObject {}
pub struct FunctionTemplateInfo {}
pub struct AllocationSite {}
pub struct JSArray {}
pub struct NumberDictionary {}
pub struct BytecodeArray {}
pub struct JSCollection {}
pub struct FieldType {}
pub struct InstructionStream {}
pub struct UnionOf<T, U> {}
pub struct InterceptorInfo {}
pub struct DescriptorArray {}

pub enum InstanceType {
    MAP_TYPE,
    JS_OBJECT_TYPE,
    JS_GLOBAL_OBJECT_TYPE,
    JS_FUNCTION_TYPE,
    BYTECODE_ARRAY_TYPE,
    FIXED_ARRAY_TYPE,
    DESCRIPTOR_ARRAY_TYPE,
    NATIVE_CONTEXT_TYPE,
    FUNCTION_CONTEXT_TYPE,
    EXTERNAL_STRING_TYPE,
    SHARED_FUNCTION_INFO_TYPE,
    WEAK_FIXED_ARRAY_TYPE,
	FEEDBACK_VECTOR_TYPE,
	CODE_TYPE,
	FIXED_COW_ARRAY_TYPE,
    OTHER_TYPE,
    LAST_TYPE
}
pub enum CodeKind {
    NONE,
    TURBOPROP,
    DEBUG_BREAK,
    JS_TO_WASM_FUNCTION,
    WASM_TO_JS_FUNCTION,
    INTERPRETED_FUNCTION,
    STUB,
    BUILTIN,
    HANDLER,
    WASM_FUNCTION,
    OTHER
}
pub enum RelocInfoMode {
    EMBEDDED_OBJECT,
	CODE_TARGET,
    OTHER
}
pub struct RelocInfo {}
impl RelocInfo {
	pub fn rmode(&self) -> RelocInfoMode {
		RelocInfoMode::OTHER
	}
	pub fn EmbeddedObjectModeMask() -> i32 {
		0
	}
	pub fn IsEmbeddedObjectMode(_mode: RelocInfoMode) -> bool {
		false
	}
	pub fn target_object(&self, _cage_base: PtrComprCageBase) -> Tagged<Object> {
		Tagged {}
	}
}
pub struct RelocIterator {}
impl RelocIterator {
	pub fn new(_code: Tagged<code>, _mode_mask: i32) -> Self {
		RelocIterator {}
	}
	pub fn done(&self) -> bool {
		true
	}
	pub fn next(&mut self) {}
	pub fn rinfo(&self) -> &RelocInfo {
		&RelocInfo {}
	}
}
pub enum PrototypeUsers {}
pub struct PrototypeInfo {}
impl PrototypeInfo {
	pub fn prototype_users(&self) -> Tagged<Object> {
		Tagged {}
	}
}
pub enum FeedbackSlotKind {
    kCall,
    kLoadProperty,
    kLoadGlobalInsideTypeof,
    kLoadGlobalNotInsideTypeof,
    kLoadKeyed,
    kHasKeyed,
    kSetNamedSloppy,
    kSetNamedStrict,
    kDefineNamedOwn,
    kStoreGlobalSloppy,
    kStoreGlobalStrict,
    kSetKeyedSloppy,
    kSetKeyedStrict,
    kBinaryOp,
    kCompareOp,
    kOther
}
pub struct ReadOnlyRoots{}
impl ReadOnlyRoots {
	pub fn empty_property_array(&self) -> Tagged<Object> {
		Tagged {}
	}
	pub fn empty_fixed_array(&self) -> Tagged<FixedArray> {
		Tagged {}
	}
	pub fn fixed_cow_array_map(&self) -> Tagged<Map> {
		Tagged {}
	}
	pub fn empty_descriptor_array(&self) -> Tagged<DescriptorArray> {
		Tagged {}
	}
	pub fn empty_slow_element_dictionary(&self) -> Tagged<FixedArray> {
		Tagged {}
	}
	pub fn empty_property_dictionary(&self) -> Tagged<FixedArray> {
		Tagged {}
	}
}
pub struct Factory {}
impl Factory {
	pub fn uninitialized_symbol(&self) -> Tagged<Object> {
		Tagged {}
	}
}
pub struct IsolateForSandbox {}
impl Isolate {
	pub fn time_millis_since_init(&self) -> f64 {
		0.0
	}
	pub fn factory(&self) -> &Factory {
		&Factory {}
	}
	pub fn heap(&self) -> &Heap {
		&Heap {}
	}
	pub fn roots(&self) -> &ReadOnlyRoots {
		&ReadOnlyRoots {}
	}
}

pub struct V8_NOINLINE {}
pub struct MutexGuard {}
pub struct base {}
impl base {
    pub struct LazyMutex {
        pub ptr: *mut Mutex<()>,
    }
}
impl base::LazyMutex {
    pub const fn new() -> Self {
        base::LazyMutex {
            ptr: unsafe { mem::transmute(0usize) }
        }
    }
    pub fn Pointer(&self) -> &Mutex<()> {
        unsafe { &*(self.ptr) }
    }
}
pub const LAZY_MUTEX_INITIALIZER: base::LazyMutex = base::LazyMutex::new();
pub struct ObjectVisitorWithCageBases {
    cage_base_: PtrComprCageBase,
    heap_: *mut Heap
}
impl ObjectVisitorWithCageBases {
    pub fn new(heap: *mut Heap) -> Self {
        ObjectVisitorWithCageBases {
            cage_base_: PtrComprCageBase {},
            heap_: heap
        }
    }

    pub fn cage_base(&self) -> PtrComprCageBase {
        self.cage_base_
    }
	pub fn VisitPointers(&mut self, _host: Tagged<HeapObject>, _start: ObjectSlot, _end: ObjectSlot) {}
	pub fn VisitPointers_MaybeObjectSlot(&mut self, _host: Tagged<HeapObject>, _start: MaybeObjectSlot, _end: MaybeObjectSlot) {}
}
pub mod detail {
    pub struct ArrayHeaderBase<Super, const kReadOnly: bool> {
        _phantom: std::marker::PhantomData<Super>,
    }
}
pub struct Counters{}

#[macro_export]
macro_rules! INSTANCE_TYPE_LIST {
    ($callback:ident) => {
        $callback!(MAP_TYPE);
        $callback!(JS_OBJECT_TYPE);
        $callback!(JS_GLOBAL_OBJECT_TYPE);
        $callback!(JS_FUNCTION_TYPE);
        $callback!(BYTECODE_ARRAY_TYPE);
        $callback!(FIXED_ARRAY_TYPE);
        $callback!(DESCRIPTOR_ARRAY_TYPE);
        $callback!(NATIVE_CONTEXT_TYPE);
        $callback!(FUNCTION_CONTEXT_TYPE);
        $callback!(EXTERNAL_STRING_TYPE);
        $callback!(SHARED_FUNCTION_INFO_TYPE);
		$callback!(CODE_TYPE);
        $callback!(OTHER_TYPE);
    };
}
#[macro_export]
macro_rules! CODE_KIND_LIST {
    ($callback:ident) => {
        $callback!(NONE);
        $callback!(TURBOPROP);
        $callback!(DEBUG_BREAK);
        $callback!(JS_TO_WASM_FUNCTION);
        $callback!(WASM_TO_JS_FUNCTION);
        $callback!(INTERPRETED_FUNCTION);
        $callback!(STUB);
        $callback!(BUILTIN);
        $callback!(HANDLER);
        $callback!(WASM_FUNCTION);
    };
}
#[macro_export]
macro_rules! VIRTUAL_INSTANCE_TYPE_LIST {
    ($callback:ident) => {
        CODE_KIND_LIST!($callback);
        $callback!(ARRAY_BOILERPLATE_DESCRIPTION_ELEMENTS_TYPE);
        $callback!(ARRAY_DICTIONARY_ELEMENTS_TYPE);
        $callback!(ARRAY_ELEMENTS_TYPE);
        $callback!(BOILERPLATE_ELEMENTS_TYPE);
        $callback!(BOILERPLATE_PROPERTY_ARRAY_TYPE);
        $callback!(BOILERPLATE_PROPERTY_DICTIONARY_TYPE);
        $callback!(BYTECODE_ARRAY_CONSTANT_POOL_TYPE);
        $callback!(BYTECODE_ARRAY_HANDLER_TABLE_TYPE);
        $callback!(COW_ARRAY_TYPE);
        $callback!(DEOPTIMIZATION_DATA_TYPE);
        $callback!(DEPENDENT_CODE_TYPE);
        $callback!(DEPRECATED_DESCRIPTOR_ARRAY_TYPE);
        $callback!(EMBEDDED_OBJECT_TYPE);
        $callback!(ENUM_KEYS_CACHE_TYPE);
        $callback!(ENUM_INDICES_CACHE_TYPE);
        $callback!(FEEDBACK_VECTOR_ENTRY_TYPE);
        $callback!(FEEDBACK_VECTOR_HEADER_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_CALL_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_CALL_UNUSED_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_ENUM_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_LOAD_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_LOAD_UNUSED_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_OTHER_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_STORE_TYPE);
        $callback!(FEEDBACK_VECTOR_SLOT_STORE_UNUSED_TYPE);
        $callback!(FUNCTION_TEMPLATE_INFO_ENTRIES_TYPE);
        $callback!(GLOBAL_ELEMENTS_TYPE);
        $callback!(GLOBAL_PROPERTIES_TYPE);
        $callback!(JS_ARRAY_BOILERPLATE_TYPE);
        $callback!(JS_COLLECTION_TABLE_TYPE);
        $callback!(JS_OBJECT_BOILERPLATE_TYPE);
        $callback!(JS_UNCOMPILED_FUNCTION_TYPE);
        $callback!(MAP_ABANDONED_PROTOTYPE_TYPE);
        $callback!(MAP_DEPRECATED_TYPE);
        $callback!(MAP_DICTIONARY_TYPE);
        $callback!(MAP_PROTOTYPE_DICTIONARY_TYPE);
        $callback!(MAP_PROTOTYPE_TYPE);
        $callback!(MAP_STABLE_TYPE);
        $callback!(NUMBER_STRING_CACHE_TYPE);
        $callback!(OBJECT_DICTIONARY_ELEMENTS_TYPE);
        $callback!(OBJECT_ELEMENTS_TYPE);
        $callback!(OBJECT_PROPERTY_ARRAY_TYPE);
        $callback!(OBJECT_PROPERTY_DICTIONARY_TYPE);
        $callback!(OBJECT_TO_CODE_TYPE);
        $callback!(OPTIMIZED_CODE_LITERALS_TYPE);
        $callback!(OTHER_CONTEXT_TYPE);
        $callback!(PROTOTYPE_DESCRIPTOR_ARRAY_TYPE);
        $callback!(PROTOTYPE_PROPERTY_ARRAY_TYPE);
        $callback!(PROTOTYPE_PROPERTY_DICTIONARY_TYPE);
        $callback!(PROTOTYPE_USERS_TYPE);
        $callback!(REGEXP_MULTIPLE_CACHE_TYPE);
        $callback!(RELOC_INFO_TYPE);
        $callback!(RETAINED_MAPS_TYPE);
        $callback!(SCRIPT_LIST_TYPE);
        $callback!(SCRIPT_INFOS_TYPE);
        $callback!(SCRIPT_SOURCE_EXTERNAL_ONE_BYTE_TYPE);
        $callback!(SCRIPT_SOURCE_EXTERNAL_TWO_BYTE_TYPE);
        $callback!(SCRIPT_SOURCE_NON_EXTERNAL_ONE_BYTE_TYPE);
        $callback!(SCRIPT_SOURCE_NON_EXTERNAL_TWO_BYTE_TYPE);
        $callback!(SERIALIZED_OBJECTS_TYPE);
        $callback!(SINGLE_CHARACTER_STRING_TABLE_TYPE);
        $callback!(STRING_SPLIT_CACHE_TYPE);
        $callback!(STRING_EXTERNAL_RESOURCE_ONE_BYTE_TYPE);
        $callback!(STRING_EXTERNAL_RESOURCE_TWO_BYTE_TYPE);
		$callback!(SOURCE_POSITION_TABLE_TYPE);
        $callback!(UNCOMPILED_SHARED_FUNCTION_INFO_TYPE);
        $callback!(WASTED_DESCRIPTOR_ARRAY_DETAILS_TYPE);
        $callback!(WASTED_DESCRIPTOR_ARRAY_VALUES_TYPE);
        $callback!(WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE);
    };
}

pub enum VirtualInstanceType {
    NONE,
    TURBOPROP,
    DEBUG_BREAK,
    JS_TO_WASM_FUNCTION,
    WASM_TO_JS_FUNCTION,
    INTERPRETED_FUNCTION,
    STUB,
    BUILTIN,
    HANDLER,
    WASM_FUNCTION,
	ARRAY_BOILERPLATE_DESCRIPTION_ELEMENTS_TYPE,
	ARRAY_DICTIONARY_ELEMENTS_TYPE,
	ARRAY_ELEMENTS_TYPE,
	BOILERPLATE_ELEMENTS_TYPE,
	BOILERPLATE_PROPERTY_ARRAY_TYPE,
	BOILERPLATE_PROPERTY_DICTIONARY_TYPE,
	BYTECODE_ARRAY_CONSTANT_POOL_TYPE,
	BYTECODE_ARRAY_HANDLER_TABLE_TYPE,
	COW_ARRAY_TYPE,
	DEOPTIMIZATION_DATA_TYPE,
	DEPENDENT_CODE_TYPE,
	DEPRECATED_DESCRIPTOR_ARRAY_TYPE,
	EMBEDDED_OBJECT_TYPE,
	ENUM_KEYS_CACHE_TYPE,
	ENUM_INDICES_CACHE_TYPE,
	FEEDBACK_VECTOR_ENTRY_TYPE,
	FEEDBACK_VECTOR_HEADER_TYPE,
	FEEDBACK_VECTOR_SLOT_CALL_TYPE,
	FEEDBACK_VECTOR_SLOT_CALL_UNUSED_TYPE,
	FEEDBACK_VECTOR_SLOT_ENUM_TYPE,
	FEEDBACK_VECTOR_SLOT_LOAD_TYPE,
	FEEDBACK_VECTOR_SLOT_LOAD_UNUSED_TYPE,
	FEEDBACK_VECTOR_SLOT_OTHER_TYPE,
	FEEDBACK_VECTOR_SLOT_STORE_TYPE,
	FEEDBACK_VECTOR_SLOT_STORE_UNUSED_TYPE,
	FUNCTION_TEMPLATE_INFO_ENTRIES_TYPE,
	GLOBAL_ELEMENTS_TYPE,
	GLOBAL_PROPERTIES_TYPE,
	JS_ARRAY_BOILERPLATE_TYPE,
	JS_COLLECTION_TABLE_TYPE,
	JS_OBJECT_BOILERPLATE_TYPE,
	JS_UNCOMPILED_FUNCTION_TYPE,
	MAP_ABANDONED_PROTOTYPE_TYPE,
	MAP_DEPRECATED_TYPE,
	MAP_DICTIONARY_TYPE,
	MAP_PROTOTYPE_DICTIONARY_TYPE,
	MAP_PROTOTYPE_TYPE,
	MAP_STABLE_TYPE,
	NUMBER_STRING_CACHE_TYPE,
	OBJECT_DICTIONARY_ELEMENTS_TYPE,
	OBJECT_ELEMENTS_TYPE,
	OBJECT_PROPERTY_ARRAY_TYPE,
	OBJECT_PROPERTY_DICTIONARY_TYPE,
	OBJECT_TO_CODE_TYPE,
	OPTIMIZED_CODE_LITERALS_TYPE,
	OTHER_CONTEXT_TYPE,
	PROTOTYPE_DESCRIPTOR_ARRAY_TYPE,
	PROTOTYPE_PROPERTY_ARRAY_TYPE,
	PROTOTYPE_PROPERTY_DICTIONARY_TYPE,
	PROTOTYPE_USERS_TYPE,
	REGEXP_MULTIPLE_CACHE_TYPE,
	RELOC_INFO_TYPE,
	RETAINED_MAPS_TYPE,
	SCRIPT_LIST_TYPE,
	SCRIPT_INFOS_TYPE,
	SCRIPT_SOURCE_EXTERNAL_ONE_BYTE_TYPE,
	SCRIPT_SOURCE_EXTERNAL_TWO_BYTE_TYPE,
	SCRIPT_SOURCE_NON_EXTERNAL_ONE_BYTE_TYPE,
	SCRIPT_SOURCE_NON_EXTERNAL_TWO_BYTE_TYPE,
	SERIALIZED_OBJECTS_TYPE,
	SINGLE_CHARACTER_STRING_TABLE_TYPE,
	STRING_SPLIT_CACHE_TYPE,
	STRING_EXTERNAL_RESOURCE_ONE_BYTE_TYPE,
	STRING_EXTERNAL_RESOURCE_TWO_BYTE_TYPE,
	SOURCE_POSITION_TABLE_TYPE,
	UNCOMPILED_SHARED_FUNCTION_INFO_TYPE,
    WASTED_DESCRIPTOR_ARRAY_DETAILS_TYPE,
    WASTED_DESCRIPTOR_ARRAY_VALUES_TYPE,
    WEAK_NEW_SPACE_OBJECT_TO_CODE_TYPE,
    LAST_VIRTUAL_TYPE
}
impl From<CodeKind> for VirtualInstanceType {
    fn from(kind: CodeKind) -> Self {
        match kind {
            CodeKind::NONE => VirtualInstanceType::NONE,
            CodeKind::TURBOPROP => VirtualInstanceType::TURBOPROP,
            CodeKind::DEBUG_BREAK => VirtualInstanceType::DEBUG_BREAK,
            CodeKind::JS_TO_WASM_FUNCTION => VirtualInstanceType::JS_TO_WASM_FUNCTION,
            CodeKind::WASM_TO_JS_FUNCTION => VirtualInstanceType::WASM_TO_JS_FUNCTION,
            CodeKind::INTERPRETED_FUNCTION => VirtualInstanceType::INTERPRETED_FUNCTION,
            CodeKind::STUB => VirtualInstanceType::STUB,
            CodeKind::BUILTIN => VirtualInstanceType::BUILTIN,
            CodeKind::HANDLER => VirtualInstanceType::HANDLER,
            CodeKind::WASM_FUNCTION => VirtualInstanceType::WASM_FUNCTION,
            CodeKind::OTHER => VirtualInstanceType::LAST_VIRTUAL_TYPE,
        }
    }
}
fn CodeKindIsOptimizedJSFunction(_kind: CodeKind) -> bool {
    false
}

pub struct HeapObjectIterator {}

lazy_static::lazy_static! {
    static ref OBJECT_STATS_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct ObjectStats {
    heap_: *mut Heap,
    object_counts_: [usize; 172],
    object_counts_last_time_: [usize; 172],
    object_sizes_: [usize; 172],
    object_sizes_last_time_: [usize; 172],
    over_allocated_: [usize; 172],
    size_histogram_: [[usize; 16]; 172],
    over_allocated_histogram_: [[usize; 16]; 172],
    tagged_fields_count_: usize,
    embedder_fields_count_: usize,
    inobject_smi_fields_count_: usize,
    boxed_double_fields_count_: usize,
    string_data_count_: usize,
    raw_fields_count_: usize,
}

impl ObjectStats {
    pub const kNoOverAllocation: usize = 0;
    const kFirstBucketShift: i32 = 5;
    const kLastBucketShift: i32 = 20;
    const kFirstBucket: i32 = 1 << Self::kFirstBucketShift;
    const kLastBucket: i32 = 1 << Self::kLastBucketShift;
    const kNumberOfBuckets: i32 = Self::kLastBucketShift - Self::kFirstBucketShift + 1;
    const kLastValueBucketIndex: i32 = Self::kLastBucketShift - Self::kFirstBucketShift;
    pub const FIRST_VIRTUAL_TYPE: i32 = 164;
    pub const OBJECT_STATS_COUNT: i32 =
        Self::FIRST_VIRTUAL_TYPE + VirtualInstanceType::LAST_VIRTUAL_TYPE as i32 + 1;

    pub fn new(heap: *mut Heap) -> Self {
        let mut stats = ObjectStats {
            heap_: heap,
            object_counts_: [0; 172],
            object_counts_last_time_: [0; 172],
            object_sizes_: [0; 172],
            object_sizes_last_time_: [0; 172],
            over_allocated_: [0; 172],
            size_histogram_: [[0; 16]; 172],
            over_allocated_histogram_: [[0; 16]; 172],
            tagged_fields_count_: 0,
            embedder_fields_count_: 0,
            inobject_smi_fields_count_: 0,
            boxed_double_fields_count_: 0,
            string_data_count_: 0,
            raw_fields_count_: 0,
        };
        stats.ClearObjectStats(true);
        stats
    }

    pub fn ClearObjectStats(&mut self, clear_last_time_stats: bool) {
        self.object_counts_ = [0; 172];
        self.object_sizes_ = [0; 172];
        self.over_allocated_ = [0; 172];
        self.size_histogram_ = [[0; 16]; 172];
        self.over_allocated_histogram_ = [[0; 16]; 172];
        if clear_last_time_stats {
            self.object_counts_last_time_ = [0; 172];
            self.object_sizes_last_time_ = [0; 172];
        }
        self.tagged_fields_count_ = 0;
        self.embedder_fields_count_ = 0;
        self.inobject_smi_fields_count_ = 0;
        self.boxed_double_fields_count_ = 0;
        self.string_data_count_ = 0;
        self.raw_fields_count_ = 0;
    }

    pub fn PrintJSON(&self, _key: &str) {
        todo!()
    }

    pub fn Dump(&mut self, _stream: &mut std::stringstream) {
        todo!()
    }

    pub fn CheckpointObjectStats(&mut self) {
        let _lock = OBJECT_STATS_MUTEX.lock().unwrap();
        self.object_counts_last_time_.copy_from_slice(&self.object_counts_);
        self.object_sizes_last_time_.copy_from_slice(&self.object_sizes_);
        self.ClearObjectStats(false);
    }

    pub fn RecordObjectStats(&mut self, type_: InstanceType, size: usize, over_allocated: usize) {
        let index = type_ as usize;
        assert!(index < 172);
        self.object_counts_[index] += 1;
        self.object_sizes_[index] += size;
        let hist_index = self.HistogramIndexFromSize(size);
        self.size_histogram_[index][hist_index] += 1;
        self.over_allocated_[index] += over_allocated;
        self.over_allocated_histogram_[index][hist_index] += 1;
    }

    pub fn RecordVirtualObjectStats(&mut self, type_: VirtualInstanceType, size: usize, over_allocated: usize) {
        let index = type_ as i32 + Self::FIRST_VIRTUAL_TYPE;
        assert!(index < Self::OBJECT_STATS_COUNT);
        let index = index as usize;
        self.object_counts_[index] += 1;
        self.object_sizes_[index] += size;
        let hist_index = self.HistogramIndexFromSize(size);
        self.size_histogram_[index][hist_index] += 1;
        self.over_allocated_[index] += over_allocated;
        self.over_allocated_histogram_[index][hist_index] += 1;
    }

    pub fn object_count_last_gc(&self, index: usize) -> usize {
        self.object_counts_last_time_[index]
    }

    pub fn object_size_last_gc(&self, index: usize) -> usize {
        self.object_sizes_last_time_[index]
    }

    pub fn isolate(&self) -> *mut Isolate {
        unsafe { (*self.heap_).isolate() }
    }

    pub fn heap(&self) -> *mut Heap {
        self.heap_
    }

    fn HistogramIndexFromSize(&self, size: usize) -> usize {
        if size == 0 {
            return 0;
        }
        let log2 = (usize::BITS - size.leading_zeros() - 1) as i32;
        cmp::min(cmp::max(log2 + 1 - Self::kFirstBucketShift, 0), Self::kLastValueBucketIndex) as usize
    }
    pub fn PrintKeyAndId(&self, _key: &str, _gc_count: i32) {}
    pub fn PrintInstanceTypeJSON(&self, _key: &str, _gc_count: i32, _name: &str, _index: i32) {}
    pub fn DumpInstanceTypeData(&mut self, _stream: &mut std::stringstream, _name: &str, _index: i32) {}
}
pub struct SafepointScope {}
pub struct AllowGarbageCollection {}
pub enum PropertyLocation {
    kField
}
struct JSObjectFieldStats {
    embedded_fields_count_: u32,
    smi_fields_count_: u32,
}
impl JSObjectFieldStats {
	fn new() -> Self {
		JSObjectFieldStats {
			embedded_fields_count_: 0,
			smi_fields_count_: 0
		}
	}
}
struct FieldStatsCollector {
    heap_: *mut Heap,
    cage_base_: PtrComprCageBase,
    tagged_fields_count_: *mut usize,
    embedder_fields_count_: *mut usize,
    inobject_smi_fields_count_: *mut usize,
    boxed_double_fields_count_: *mut usize,
    string_data_count_: *mut usize,
    raw_fields_count_: *mut usize,
    field_stats_cache_: HashMap<u64, JSObjectFieldStats>
}
impl FieldStatsCollector {
    fn new(
        heap: *mut Heap,
        tagged_fields_count: *mut usize,
        embedder_fields_count: *mut usize,
        inobject_smi_fields_count: *mut usize,
        boxed_double_fields_count: *mut usize,
        string_data_count: *mut usize,
        raw_fields_count: *mut usize,
    ) -> Self {
        FieldStatsCollector {
            heap_: heap,
            cage_base_: PtrComprCageBase {},
            tagged_fields_count_: tagged_fields_count,
            embedder_fields_count_: embedder_fields_count,
            inobject_smi_fields_count_: inobject_smi_fields_count,
            boxed_double_fields_count_: boxed_double_fields_count,
            string_data_count_: string_data_count,
            raw_fields_count_: raw_fields_count,
			field_stats_cache_: HashMap::new()
        }
    }

    fn RecordStats(&mut self, host: Tagged<HeapObject>) {
        let old_pointer_fields_count = unsafe { *self.tagged_fields_count_ };
        let mut visitor = ObjectVisitorWithCageBases::new(self.heap_);
        self.VisitObject(unsafe { (*self.heap_).isolate() }, host, &mut visitor);

        let tagged_fields_count_in_object = unsafe { *self.tagged_fields_count_ } - old_pointer_fields_count;

        let object_size_in_words = 0;// host.Size(self.cage_base()) / 8; //kTaggedSize;
        assert!(tagged_fields_count_in_object <= object_size_in_words);
        let raw_fields_count_in_object = object_size_in_words - tagged_fields_count_in_object;

        if self.IsJSObject(host, self.cage_base_) {
            let field_stats = self.GetInobjectFieldStats(unsafe { (*self.heap_).isolate() }, Tagged{});//host.map());
            // Embedder fields are already included into pointer words.
            assert!(field_stats.embedded_fields_count_ <= tagged_fields_count_in_object as u32);
            //tagged_fields_count_in_object -= field_stats.embedded_fields_count_ as usize;
           // *self.tagged_fields_count_ -= field_stats.embedded_fields_count_ as usize;
           // *self.embedder_fields_count_ += field_stats.embedded_fields_count_ as usize;

            // Smi fields are also included into pointer words.
            //tagged_fields_count_in_object -= field_stats.smi_fields_count_ as usize;
           // *self.tagged_fields_count_ -= field_stats.smi_fields_count_ as usize;
           // *self.inobject_smi_fields_count_ += field_stats.smi_fields_count_ as usize;
        } else if self.IsHeapNumber(host, self.cage_base_) {
            assert!(8 / 8 <= raw_fields_count_in_object);
            //raw_fields_count_in_object -= 8 / 8;
           // *self.boxed_double_fields_count_ += 1;
        } else if self.IsSeqString(host, self.cage_base_) {
           // let string_data =
            //    Cast::<SeqString>(host).length(kAcquireLoad) *
            //    (Cast::<String>(host).IsOneByteRepresentation() ? 1 : 2) / 8;
           // assert!(string_data <= raw_fields_count_in_object);
           // raw_fields_count_in_object -= string_data;
           // *self.string_data_count_ += string_data;
        }
       // *self.raw_fields_count_ += raw_fields_count_in_object;
    }
    fn VisitObject(&mut self, _isolate: *mut Isolate, _host: Tagged<HeapObject>, _visitor: &mut ObjectVisitorWithCageBases) {}

    fn GetInobjectFieldStats(&mut self, _isolate: *mut Isolate, map: Tagged<Map>) -> JSObjectFieldStats {
        let key = self.calculate_hash(&map);
		if let Some(stats) = self.field_stats_cache_.get(&key) {
			return JSObjectFieldStats {embedded_fields_count_: stats.embedded_fields_count_, smi_fields_count_: stats.smi_fields_count_};
		}
		let mut stats = JSObjectFieldStats::new();
        stats.embedded_fields_count_ = 0;// JSObject::GetEmbedderFieldCount(map);
        //if !map.is_dictionary_map() {
        //    let descriptors = map.instance_descriptors();
        //    for descriptor in map.IterateOwnDescriptors() {
        //        let details = descriptors.GetDetails(descriptor);
        //        if details.location() == PropertyLocation::kField {
        //            let index = FieldIndex::ForDetails(map, details);
        //            if !index.is_inobject() {
        //                break;
        //            }
        //            if details.representation().IsSmi() {
        //                stats.smi_fields_count_ += 1;
        //            }
        //        }
        //    }
        //}
		self.field_stats_cache_.insert(key, stats);
		stats
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    fn IsJSObject(&self, _host: Tagged<HeapObject>, _cage_base: PtrComprCageBase) -> bool {
        false
    }
    fn IsHeapNumber(&self, _host: Tagged<HeapObject>, _cage_base: PtrComprCageBase) -> bool {
        false
    }
    fn IsSeqString(&self, _host: Tagged<HeapObject>, _cage_base: PtrComprCageBase) -> bool {
        false
    }
    fn cage_base(&self) -> PtrComprCageBase {
        self.cage_base_
    }
}

pub struct ObjectStatsCollector {
    heap_: *mut Heap,
    live_: *mut ObjectStats,
    dead_: *mut ObjectStats,
}

impl ObjectStatsCollector {
    pub fn new(heap: *mut Heap, live: *mut ObjectStats, dead: *mut ObjectStats) -> Self {
        ObjectStatsCollector {
            heap_: heap,
            live_: live,
            dead_: dead,
        }
    }

    pub fn Collect(&mut self) {
        let mut live_collector = ObjectStatsCollectorImpl::new(self.heap_, self.live_);
        let mut dead_collector = ObjectStatsCollectorImpl::new(self.heap_, self.dead_);
        live_collector.CollectGlobalStatistics();

        for i in 0..ObjectStatsCollectorImpl::kNumberOfPhases {
            let phase = match i {
                0 => ObjectStatsCollectorImpl::Phase::kPhase1,
                1 => ObjectStatsCollectorImpl::Phase::kPhase2,
                _ => ObjectStatsCollectorImpl::Phase::kPhase1,
            };
            let mut visitor = ObjectStatsVisitor::new(
                self.heap_,
                &mut live_collector,
                &mut dead_collector,
                phase,
            );
            let _safepoint_scope = SafepointScope {};
            IterateHeap(self.heap_, &mut visitor);
        }
    }
}
struct ObjectStatsCollectorImpl {
    heap_: *mut Heap,
    stats_: *mut ObjectStats,
    marking_state_: *mut i32,
    virtual_objects_: HashMap<u64, bool>,
    external_resources_: HashMap<u64, bool>,
    field_
