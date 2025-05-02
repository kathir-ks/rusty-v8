// This code is a partial translation of the C++ file
// `src/heap/factory.cc` from the V8 JavaScript engine.
// It's not a complete or runnable program and may require
// additional context and dependencies to function correctly.

// Note: Due to the complexity and size of the original C++ code,
// this Rust translation is incomplete and focuses on demonstrating
// the general structure and key elements of the conversion process.
// Many functions are represented as stubs, and detailed implementations
// are omitted.

//use std::alloc::{alloc, dealloc, Layout};
//use std::any::Any;
//use std::cell::{Cell, RefCell};
//use std::collections::HashMap;
//use std::error::Error;
//use std::ffi::CString;
//use std::fmt;
//use std::mem;
//use std::ops::{Deref, DerefMut};
//use std::ptr;
//use std::rc::Rc;
//use std::sync::{Arc, Mutex};

// Placeholder for V8 flags.  Replace with actual flag management if needed.
mod v8_flags {
    pub const verify_heap: bool = false;
    pub const turbo_profiling_verbose: bool = false;
    pub const allocation_site_pretenuring: bool = false;
    pub const shared_string_table: bool = false;
    pub const const_tracking_let: bool = false;
    pub const script_context_mutable_heap_number: bool = false;
    pub const string_slices: bool = true;
    pub const log_maps: bool = false;
    pub const log_function_events: bool = false;
}

// Placeholder for globals.
mod globals {
    pub const kTaggedSize: usize = 8; // Assuming 64-bit architecture.
    pub const kCodeAlignment: usize = 16; // Example alignment.
    pub const kMaxRegularHeapObjectSize: usize = 1024 * 1024; // Example size.
    pub const kMaxInt: usize = usize::MAX >> 1;
}

// Placeholder for constants.
mod constants {
    pub const kInvalidEnumCacheSentinel: i32 = -1;
}

// Placeholder for roots.
mod roots {
    use crate::objects::{HeapObject, Object, Smi};

    pub struct ReadOnlyRoots {}

    impl ReadOnlyRoots {
        pub fn undefined_value(&self) -> Object {
            // Placeholder.  Replace with actual undefined value.
            Object::from(Smi::zero())
        }
        pub fn instruction_stream_map(&self) -> HeapObject {
            // Placeholder.  Replace with actual map.
            HeapObject::default()
        }
        pub fn self_reference_marker(&self) -> Object {
            // Placeholder.  Replace with actual marker.
            Object::from(Smi::zero())
        }

        pub fn hole_value(&self) -> Object {
            Object::from(Smi::zero()) // Placeholder. Replace with the actual Hole object
        }

        pub fn fixed_array_map(&self) -> HeapObject {
            HeapObject::default() // Placeholder
        }

        pub fn property_array_map(&self) -> HeapObject {
            HeapObject::default() // Placeholder
        }

        pub fn transition_array_map(&self) -> HeapObject {
            HeapObject::default() // Placeholder
        }

        pub fn empty_descriptor_array(&self) -> crate::objects::DescriptorArray {
            crate::objects::DescriptorArray::default() // Placeholder
        }

        pub fn invalid_prototype_validity_cell(&self) -> super::Cell {
            super::Cell::default() // Placeholder
        }

        pub fn swiss_name_dictionary_map(&self) -> HeapObject {
            HeapObject::default() // Placeholder
        }

        pub fn symbol_map(&self) -> HeapObject {
            HeapObject::default()
        }

        pub fn null_value(&self) -> Object {
            Object::from(Smi::zero()) // Placeholder
        }

        pub fn empty_scope_info(&self) -> crate::objects::ScopeInfo {
            crate::objects::ScopeInfo::default() // Placeholder
        }

        pub fn empty_fixed_array(&self) -> crate::objects::FixedArray {
            crate::objects::FixedArray::default() // Placeholder
        }

        pub fn is_initialized(&self, _root_index: RootIndex) -> bool {
            true
        }

        pub fn empty_weak_fixed_array(&self) -> crate::objects::WeakFixedArray {
            crate::objects::WeakFixedArray::default()
        }

        pub fn no_closures_cell_map(&self) -> HeapObject {
            HeapObject::default()
        }

        pub fn one_closure_cell_map(&self) -> HeapObject {
            HeapObject::default()
        }

        pub fn many_closures_cell_map(&self) -> HeapObject {
            HeapObject::default()
        }

        pub fn the_hole_value(&self) -> super::objects::Hole {
            super::objects::Hole::default() // Placeholder
        }

        pub fn holes_float_value(&self) -> f64 {
            0.0
        }

        pub fn to_name_string(&self) -> crate::objects::String {
            crate::objects::String::default() // Placeholder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum RootIndex {
        kEmptySwissPropertyDictionary,
    }
}

// Placeholder for objects.
mod objects {
    use crate::{heap::AllocationType, roots::ReadOnlyRoots};

    #[derive(Debug, Clone, Copy)]
    pub struct Smi(i64);

    impl Smi {
        pub fn zero() -> Self {
            Smi(0)
        }

        pub fn from_int(value: i32) -> Self {
            Smi(value as i64)
        }

        pub fn from_enum(_value: crate::wasm::MessageTemplate) -> Self {
            Smi(0) // Placeholder
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct HeapObject {
        // Placeholder fields
    }

    impl HeapObject {
        pub fn set_map_after_allocation(&mut self, _isolate: &Isolate, _map: Map, _write_barrier_mode: WriteBarrierMode) {}
        pub fn map(&self) -> Map {
            Map::default() // Placeholder
        }

        pub fn verify_code_pointer(_isolate: &Isolate, _raw_istream: InstructionStream) {
            // Placeholder function
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Object {
        value: usize, // Placeholder
    }

    impl Object {
        pub fn from(smi: Smi) -> Self {
            Object { value: smi.0 as usize }
        }
    }

    impl Default for Object {
        fn default() -> Self {
            Object { value: 0 }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Map {
        // Placeholder fields
    }

    impl Map {
        pub fn instance_size(&self) -> i32 {
            0 // Placeholder
        }

        pub fn set_native_context(&mut self, _context: NativeContext) {}

        pub fn set_bit_field(&mut self, _value: i32) {}
        pub fn set_bit_field2(&mut self, _value: i32) {}
        pub fn set_bit_field3(&mut self, _value: i32) {}

        pub fn set_instance_type(&mut self, _instance_type: InstanceType) {}

        pub fn init_prototype_and_constructor_or_back_pointer(&mut self, _roots: ReadOnlyRoots) {}

        pub fn set_inobject_properties_start_or_constructor_function_index(&mut self, _value: i32) {}
        pub fn set_prototype_validity_cell(&mut self, _value: Smi, _krelaxedstore: crate::heap::kRelaxedStore, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_dependent_code(&mut self, _dependent_code: DependentCode, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_raw_transitions(&mut self, _zero: Smi, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}

        pub fn owns_descriptors(&self) -> bool {
            false // Placeholder
        }

        pub fn has_no_prototype(&self) -> bool {
            false // Placeholder
        }

        pub fn is_immutable_proto(&self) -> bool {
            false // Placeholder
        }

        pub fn is_deprecated(&self) -> bool {
            false
        }

        pub fn elements_kind(&self) -> ElementsKind {
            ElementsKind::HOLEY_ELEMENTS // Placeholder
        }

        pub fn set_elements_kind(&mut self, _elements_kind: ElementsKind) {}

        pub fn GetInObjectProperties(&self) -> i32 {
            0
        }

        pub fn set_visitor_id(&mut self, _visitor_id: u32) {}

        pub fn clear_padding(&mut self) {}

        pub fn visitor_id(&self) -> u32 {
            0
        }

        pub fn is_undetectable(&self) -> bool {
            false
        }
        pub fn is_access_check_needed(&self) -> bool {
            false
        }

        pub fn set_is_extensible(&mut self, _value: bool) {}

        pub fn set_enum_length(&mut self, _length: i32) {}

        pub fn is_dictionary_map(&self) -> bool {
            false
        }
        pub fn prototype(&self) -> Object {
            Object::default() // Placeholder
        }

        pub fn set_prototype(&mut self, _object: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}

        pub fn has_named_interceptor(&self) -> bool {
            false
        }
        pub fn has_indexed_interceptor(&self) -> bool {
            false
        }

        pub fn inobject_properties_start_in_words(&self) -> i32 {
            0
        }

        pub fn set_has_hidden_prototype(&mut self) {}
        pub fn set_is_observed(&mut self) {}
        pub fn set_is_callable(&mut self) {}
        pub fn set_is_constructor(&mut self) {}

        pub fn set_may_have_interesting_symbols(&mut self) {}

        pub fn set_is_access_check_needed(&mut self) {}

        pub fn set_prototype_info(&mut self, _prototype_info: PrototypeInfo) {}
        pub fn set_back_pointer(&mut self, _map: Map) {}

        pub fn is_special_api_object(&self) -> bool {
            false
        }

        pub fn is_access_check_needed_on_prototype(&self) -> bool {
            false
        }

        pub fn set_has_instance_call_handler(&mut self) {}

        pub fn construction_counter(&self) -> i32 {
            0
        }

        pub fn set_construction_counter(&mut self, _value: i32) {}

        pub fn add_hidden_prototype_transition(&mut self, _object: Object) {}

        pub fn is_callable(&self) -> bool {
            false
        }
        pub fn is_constructor(&self) -> bool {
            false
        }

        pub fn is_undetectable(&self) -> bool {
            false
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct FixedArray {
        // Placeholder fields
    }

    impl FixedArray {
        pub fn size_for(length: i32) -> i32 {
            0 // Placeholder
        }

        pub fn length(&self) -> i32 {
            0
        }

        pub fn set_length(&mut self, _length: i32) {}

        pub fn raw_field_of_first_element(&self) -> &Object {
            // Placeholder
            static OBJ: Object = Object { value: 0 };
            &OBJ
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PropertyArray {
        // Placeholder fields
    }

    impl PropertyArray {
        pub const kNoHashSentinel: i32 = -1;
        pub fn data_start(&self) -> &Object {
            static OBJ: Object = Object { value: 0 };
            &OBJ // Placeholder
        }
        pub fn initialize_length(&mut self, _length: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct NameDictionary {
        // Placeholder fields
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SwissNameDictionary {
        // Placeholder fields
    }

    impl SwissNameDictionary {
        pub const kMetaTableEnumerationDataStartIndex: i32 = 0;
        pub fn size_for(capacity: i32) -> i32 {
            0
        }

        pub fn initialize(&mut self, _isolate: &Isolate, _empty_meta_table: ByteArray, _i: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PrototypeInfo {
        // Placeholder fields
    }

    impl PrototypeInfo {
        pub fn set_prototype_users(&mut self, _smi: Smi) {}
        pub fn set_registry_slot(&mut self, _unregister: i32) {}
        pub fn set_bit_field(&mut self, _i: i32) {}
        pub fn set_module_namespace(&mut self, _undefined_value: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Tuple2 {
        // Placeholder fields
    }

    impl Tuple2 {
        pub fn set_value1(&mut self, _object: Object) {}
        pub fn set_value2(&mut self, _object: Object) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Hole {}
    impl Hole {
        pub fn initialize(_isolate: &Isolate, _hole: super::DirectHandle<Hole>, _hole_nan_value: f64) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct FeedbackVector {}

    impl FeedbackVector {
        pub fn size_for(length: i32) -> i32 {
            0 // Placeholder
        }
        pub fn set_shared_function_info(&mut self, _shared: SharedFunctionInfo) {}
        pub fn set_length(&mut self, _length: i32) {}
        pub fn set_invocation_count(&mut self, _i: i32) {}
        pub fn set_invocation_count_before_stable(&mut self, _i: i32) {}
        pub fn reset_osr_state(&mut self) {}
        pub fn reset_flags(&mut self) {}
        pub fn set_maybe_optimized_code(&mut self, _cleared_value: ClearedValue) {}
        pub fn set_log_next_execution(&mut self, _v8flags_log_function_events: bool) {}
        pub fn set_closure_feedback_cell_array(&mut self, _closure_feedback_cell_array: ClosureFeedbackCellArray) {}
        pub fn set_parent_feedback_cell(&mut self, _parent_feedback_cell: FeedbackCell) {}

        pub fn slots_start(&self) -> &Object {
            static OBJ: Object = Object { value: 0 };
            &OBJ // Placeholder
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct EmbedderDataArray {}
    impl EmbedderDataArray {
        pub fn size_for(length: i32) -> i32 {
            0 // Placeholder
        }
        pub fn set_length(&mut self, _length: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SmallOrderedHashSet {}
    impl SmallOrderedHashSet {
        pub const kLoadFactor: i32 = 2;
        pub const kMinCapacity: i32 = 0;
        pub const kMaxCapacity: i32 = 0;

        pub fn size_for(capacity: i32) -> i32 {
            0
        }
        pub fn initialize(&mut self, _isolate: &Isolate, _capacity: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SmallOrderedHashMap {}

    impl SmallOrderedHashMap {
        pub const kLoadFactor: i32 = 2;
        pub const kMinCapacity: i32 = 0;
        pub const kMaxCapacity: i32 = 0;

        pub fn size_for(capacity: i32) -> i32 {
            0
        }

        pub fn initialize(&mut self, _isolate: &Isolate, _capacity: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SmallOrderedNameDictionary {}
    impl SmallOrderedNameDictionary {
        pub const kLoadFactor: i32 = 2;
        pub const kMinCapacity: i32 = 0;
        pub const kMaxCapacity: i32 = 0;

        pub fn size_for(capacity: i32) -> i32 {
            0
        }
        pub fn initialize(&mut self, _isolate: &Isolate, _capacity: i32) {}
        pub fn set_hash(&mut self, _no_hash_sentinel: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct OrderedHashSet {}
    impl OrderedHashSet {
        pub const kInitialCapacity: i32 = 0;
        pub fn allocate(_isolate: &Isolate, _initial_capacity: i32, _kyoung: AllocationType) -> Result<super::Handle<OrderedHashSet>, String> {
            Err("Placeholder".to_string())
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct OrderedHashMap {}
    impl OrderedHashMap {
        pub const kInitialCapacity: i32 = 0;
        pub fn allocate(_isolate: &Isolate, _initial_capacity: i32, _kyoung: AllocationType) -> Result<super::Handle<OrderedHashMap>, String> {
            Err("Placeholder".to_string())
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PropertyDescriptorObject {}
    impl PropertyDescriptorObject {
        pub fn set_flags(&mut self, _i: i32) {}
        pub fn set_value(&mut self, _the_hole: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_get(&mut self, _the_hole: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_set(&mut self, _the_hole: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct String {}

    impl String {
        pub const kMaxLength: i32 = 0;
        pub const kEmptyHashField: i32 = 0;

        pub fn length(&self) -> i32 {
            0
        }
        pub fn is_one_byte_representation(&self) -> bool {
            false
        }

        pub fn write_to_flat(_string: String, _dest: *mut u8, _begin: u32, _length: u32) {}
        pub fn flatten(_isolate: &Isolate, string: &super::Handle<String>) -> super::DirectHandle<String> {
            super::DirectHandle::from(string)
        }

        pub fn is_flat(&self) -> bool {
            false
        }

        pub fn get(&self, _begin: u32) -> u16 {
            0
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SeqOneByteString {}

    impl SeqOneByteString {
        pub fn GetChars(&self, _no_gc: crate::heap::DisallowGarbageCollection) -> *mut u8 {
            // Placeholder
            std::ptr::null_mut()
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SeqTwoByteString {}
    impl SeqTwoByteString {
        pub fn GetChars(&self, _no_gc: crate::heap::DisallowGarbageCollection) -> *mut u16 {
            // Placeholder
            std::ptr::null_mut()
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ExternalOneByteString {}

    impl ExternalOneByteString {
        pub fn InitExternalPointerFields(&mut self, _isolate: &Isolate) {}
        pub fn set_length(&mut self, _length: i32) {}
        pub fn set_raw_hash_field(&mut self, _kemptyhashfield: i32) {}
        pub fn set_resource(&mut self, _isolate: &Isolate, _resource: *const ()) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ExternalTwoByteString {}

    impl ExternalTwoByteString {
        pub fn InitExternalPointerFields(&mut self, _isolate: &Isolate) {}
        pub fn set_length(&mut self, _length: i32) {}
        pub fn set_raw_hash_field(&mut self, _kemptyhashfield: i32) {}
        pub fn set_resource(&mut self, _isolate: &Isolate, _resource: *const ()) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SlicedString {}

    impl SlicedString {
        pub const kMinLength: i32 = 0;

        pub fn parent(&self) -> String {
            String::default()
        }

        pub fn offset(&self) -> i32 {
            0
        }

        pub fn set_raw_hash_field(&mut self, _value: i32) {}

        pub fn set_length(&mut self, _length: i32) {}
        pub fn set_parent(&mut self, _str: String) {}
        pub fn set_offset(&mut self, _offset: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ThinString {}
    impl ThinString {
        pub fn actual(&self) -> String {
            String::default() // Placeholder
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct JSStringIterator {}
    impl JSStringIterator {
        pub fn set_string(&mut self, _string: String) {}
        pub fn set_index(&mut self, _i: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Symbol {}

    impl Symbol {
        pub fn set_raw_hash_field(&mut self, _value: i32) {}
        pub fn set_description(&mut self, _value: Object) {}
        pub fn set_flags(&mut self, _value: i32) {}
        pub fn is_private(&self) -> bool {
            false
        }
        pub fn set_is_private(&mut self, _value: bool) {}
        pub fn set_is_private_name(&mut self) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Context {}

    impl Context {
        pub const kTodoHeaderSize: i32 = 0;
        pub const MIN_CONTEXT_SLOTS: i32 = 0;
        pub const CONTEXT_SIDE_TABLE_PROPERTY_INDEX: usize = 0;
        pub const MIN_CONTEXT_EXTENDED_SLOTS: i32 = 0;
        pub const THROWN_OBJECT_INDEX: usize = 0;
        pub const WRAPPED_CONTEXT_INDEX: usize = 0;

        pub fn set_length(&mut self, _variadic_part_length: i32) {}
        pub fn size_from_map(&self, _map: Map) -> i32 {
            0
        }
        pub fn set_scope_info(&mut self, _scope_info: ScopeInfo) {}
        pub fn set_previous(&mut self, _outer: Context) {}
        pub fn set(&mut self, _context_side_table_property_index: usize, _side_data: FixedArray) {}
        pub fn is_script_context(&self) -> bool {
            false
        }
        pub fn set_extension(&mut self, _module: SourceTextModule) {}
        pub fn is_module_context(&self) -> bool {
            false
        }
        pub fn set_scope_info_skip_write_barrier(&mut self, _scope_info: ScopeInfo) {}
        pub fn set_previous_skip_write_barrier(&mut self, _previous: Context) {}

        pub fn init_microtask_queue(&mut self, _isolate: &Isolate, _nullable: *const ()) {}

        pub fn set_retained_maps(&mut self, _empty_weak_array_list: crate::objects::WeakArrayList) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ScriptContextTable {}
    impl ScriptContextTable {
        pub fn new(_isolate: &Isolate, _kinitialcapacity: i32) -> super::Handle<ScriptContextTable> {
            super::Handle::default() // Placeholder
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ScopeInfo {}

    impl ScopeInfo {
        pub fn context_length(&self) -> i32 {
            0
        }
        pub fn is_script_scope(&self) -> bool {
            false
        }
        pub fn scope_type(&self) -> ScopeType {
            ScopeType::EVAL_SCOPE
        }

        pub fn context_local_count(&self) -> i32 {
            0
        }
        pub fn is_debug_evaluate_scope(&self) -> bool {
            false
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SourceTextModule {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct AliasedArgumentsEntry {}
    impl AliasedArgumentsEntry {
        pub fn set_aliased_context_slot(&mut self, _aliased_context_slot: i32) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct AccessorInfo {}
    impl AccessorInfo {
        pub fn set_name(&mut self, _empty_string: String, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_data(&mut self, _undefined_value: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_flags(&mut self, _i: i32) {}
        pub fn set_is_sloppy(&mut self, _value: bool) {}
        pub fn set_initial_property_attributes(&mut self, _none: i32) {}
        pub fn init_getter(&mut self, _isolate: &Isolate, _knulladdress: i64) {}
        pub fn init_setter(&mut self, _isolate: &Isolate, _knulladdress: i64) {}
        pub fn clear_padding(&mut self) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ErrorStackData {}
    impl ErrorStackData {
        pub fn set_call_site_infos_or_formatted_stack(&mut self, _call_site_infos_or_formatted_stack: UnionOf<JSAny, FixedArray>, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_stack_trace(&mut self, _stack_trace: StackTraceInfo, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct StackTraceInfo {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Script {}
    impl Script {
        pub const kTemporaryScriptId: i32 = -1;

        pub fn id(&self) -> i32 {
            0
        }
        pub fn source(&self) -> Object {
            Object::default() // Placeholder
        }
        pub fn set_name(&mut self, _value: Object) {}

        pub fn set_id(&mut self, _value: i32) {}
        pub fn set_line_offset(&mut self, _value: i32) {}
        pub fn set_column_offset(&mut self, _value: i32) {}
        pub fn set_context_data(&mut self, _value: Object) {}
        pub fn set_type(&mut self, _value: ScriptType) {}
        pub fn set_line_ends(&mut self, _value: Smi) {}
        pub fn set_eval_from_shared_or_wrapped_arguments(&mut self, _value: Object) {}
        pub fn set_infos(&mut self, _empty_weak_fixed_array: WeakFixedArray, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_eval_from_position(&mut self, _value: i32) {}
        pub fn set_flags(&mut self, _value: i32) {}
        pub fn set_host_defined_options(&mut self, _value: Object) {}
        pub fn set_source_hash(&mut self, _undefined_value: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_compiled_lazy_function_positions(&mut self, _undefined_value: Object, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn eval_from_shared_or_wrapped_arguments(&self) -> Object {
            Object::default()
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct CallableTask {}
    impl CallableTask {
        pub fn set_callable(&mut self, _callable: JSReceiver, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_context(&mut self, _context: Context, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct CallbackTask {}
    impl CallbackTask {
        pub fn set_callback(&mut self, _callback: Foreign, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_data(&mut self, _data: Foreign, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PromiseResolveThenableJobTask {}
    impl PromiseResolveThenableJobTask {
        pub fn set_promise_to_resolve(&mut self, _promise_to_resolve: JSPromise, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_thenable(&mut self, _thenable: JSReceiver, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_then(&mut self, _then: JSReceiver, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_context(&mut self, _context: Context, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct JSReceiver {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct JSPromise {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Foreign {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Cell {}
    impl Cell {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct FeedbackCell {}
    impl FeedbackCell {
        pub const kAlignedSize: i32 = 0;

        pub fn set_value(&mut self, _value: Object) {}
        pub fn clear_interrupt_budget(&mut self) {}
        pub fn clear_dispatch_handle(&mut self) {}
        pub fn clear_padding(&mut self) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PropertyCell {}
    impl PropertyCell {
        pub const kSize: i32 = 0;
        pub fn set_dependent_code(&mut self, _value: DependentCode, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
        pub fn set_name(&mut self, _name: Name, _mode: crate::heap::WriteBarrierMode) {}
        pub fn set_value(&mut self, _value: Object, _mode: crate::heap::WriteBarrierMode) {}
        pub fn set_property_details_raw(&mut self, _property_details_raw: Smi, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct ContextSidePropertyCell {}
    impl ContextSidePropertyCell {
        pub const kSize: i32 = 0;
        pub fn set_context_side_property_raw(&mut self, _from_int: Smi, _kreleasestore: crate::heap::kReleaseStore) {}
        pub fn set_dependent_code(&mut self, _value: DependentCode, _skip_write_barrier: crate::heap::SkipWriteBarrier) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct AllocationSite {}
    impl AllocationSite {
        pub fn initialize(&self) {}
        pub fn set_weak_next(&self, _object: Object) {}

        pub fn increment_memento_create_count(&self) {}
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct AllocationMemento {}
    impl AllocationMemento {
        pub const kSize: i32 = 0;
        pub fn set_map_after_allocation(&mut self, _isolate: &