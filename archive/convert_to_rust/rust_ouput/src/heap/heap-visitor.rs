// Converted from V8 C++ source files:
// Header: heap-visitor.h
// Implementation: heap-visitor.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct MaybeObjectSize {
    raw_size_: usize,
}

impl MaybeObjectSize {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        MaybeObjectSize { raw_size_: size }
    }

    pub fn none() -> Self {
        MaybeObjectSize { raw_size_: 0 }
    }

    pub fn assume_size(&self) -> usize {
        assert!(self.raw_size_ > 0);
        self.raw_size_
    }

    pub fn is_none(&self) -> bool {
        self.raw_size_ == 0
    }
}

pub trait ObjectVisitorWithCageBases {
    // Define the methods that the ObjectVisitorWithCageBases should have.
    // For example:
    // fn visit_pointer(&mut self, object: &mut HeapObject);
}
pub struct Heap {}
pub struct Isolate {}
pub struct LocalIsolate {}
pub struct HeapObject {}
pub struct Map {}
pub struct AccessorInfo {}
pub struct AllocationSite {}
pub struct BigInt {}
pub struct BytecodeWrapper {}
pub struct CallSiteInfo {}
pub struct Cell {}
pub struct CodeWrapper {}
pub struct ConsString {}
pub struct ContextSidePropertyCell {}
pub struct CoverageInfo {}
pub struct DataHandler {}
pub struct DebugInfo {}
pub struct EmbedderDataArray {}
pub struct EphemeronHashTable {}
pub struct ExternalString {}
pub struct FeedbackCell {}
pub struct FeedbackMetadata {}
pub struct Foreign {}
pub struct FunctionTemplateInfo {}
pub struct HeapNumber {}
pub struct Hole {}
pub struct NativeContext {}
pub struct Oddball {}
pub struct PreparseData {}
pub struct PropertyArray {}
pub struct PropertyCell {}
pub struct PrototypeInfo {}
pub struct RegExpBoilerplateDescription {}
pub struct RegExpDataWrapper {}
pub struct SeqOneByteString {}
pub struct SeqTwoByteString {}
pub struct SharedFunctionInfo {}
pub struct SlicedString {}
pub struct SloppyArgumentsElements {}
pub struct SmallOrderedHashMap {}
pub struct SmallOrderedHashSet {}
pub struct SmallOrderedNameDictionary {}
pub struct SourceTextModule {}
pub struct SwissNameDictionary {}
pub struct Symbol {}
pub struct SyntheticModule {}
pub struct ThinString {}
pub struct TransitionArray {}
pub struct WeakCell {}
pub struct WasmArray {}
pub struct WasmContinuationObject {}
pub struct WasmFuncRef {}
pub struct WasmMemoryMapDescriptor {}
pub struct WasmNull {}
pub struct WasmResumeData {}
pub struct WasmStruct {}
pub struct WasmSuspenderObject {}
pub struct WasmTypeInfo {}
pub struct ByteArray {}
pub struct DescriptorArray {}
pub struct FeedbackVector {}
pub struct Filler {}
pub struct FixedArray {}
pub struct FixedDoubleArray {}
pub struct FreeSpace {}
pub struct ScopeInfo {}
pub struct ShortcutCandidate {}
pub struct WeakArrayList {}
pub struct WeakFixedArray {}
pub struct JSArrayBuffer {}
pub struct JSDataViewOrRabGsabDataView {}
pub struct JSDate {}
pub struct JSExternalObject {}
pub struct JSFinalizationRegistry {}
pub struct JSFunction {}
pub struct JSObject {}
pub struct JSRegExp {}
pub struct JSSynchronizationPrimitive {}
pub struct JSTypedArray {}
pub struct JSWeakCollection {}
pub struct JSWeakRef {}
pub struct WasmGlobalObject {}
pub struct WasmInstanceObject {}
pub struct WasmMemoryObject {}
pub struct WasmSuspendingObject {}
pub struct WasmTableObject {}
pub struct WasmTagObject {}
pub struct Object {}
pub struct InstructionStream {}
pub struct RelocInfo {}
pub struct HeapVisitor<ConcreteVisitor> {
    heap_: *const Heap,
}

impl<ConcreteVisitor> HeapVisitor<ConcreteVisitor> {
    #[inline]
    pub fn new(isolate: *mut LocalIsolate) -> Self {
       unsafe {HeapVisitor { heap_: (*isolate).isolate as *const Heap}}
    }

    #[inline]
    pub fn from_isolate(isolate: *mut Isolate) -> Self {
        HeapVisitor { heap_: unsafe {(*isolate).heap_ as *const Heap} }
    }

    #[inline]
    pub fn from_heap(heap: *mut Heap) -> Self {
        HeapVisitor { heap_: heap as *const Heap }
    }

    #[inline]
    pub fn visit(&mut self, object: *mut HeapObject) -> usize
    where ConcreteVisitor: UsePrecomputedObjectSize {
        if ConcreteVisitor::UsePrecomputedObjectSize() {
            panic!("Visit(Tagged<HeapObject> object) requires (!ConcreteVisitor::UsePrecomputedObjectSize())");
        }
        let map = unsafe {(*object).map};
        self.visit_with_map(map, object, MaybeObjectSize::none())
    }

    #[inline]
    pub fn visit_with_map(&mut self, map: *mut Map, object: *mut HeapObject) -> usize
    where ConcreteVisitor: UsePrecomputedObjectSize {
        if ConcreteVisitor::UsePrecomputedObjectSize() {
            panic!("Visit(Tagged<Map> map, Tagged<HeapObject> object) requires (!ConcreteVisitor::UsePrecomputedObjectSize())");
        }
        self.visit_with_map(map, object, MaybeObjectSize::none())
    }

    #[inline]
    pub fn visit_with_size(&mut self, map: *mut Map, object: *mut HeapObject, object_size: i32) -> usize
    where ConcreteVisitor: UsePrecomputedObjectSize {
        if !ConcreteVisitor::UsePrecomputedObjectSize() {
            panic!("Visit(Tagged<Map> map, Tagged<HeapObject> object, int object_size) requires (ConcreteVisitor::UsePrecomputedObjectSize())");
        }
        self.visit_with_map(map, object, MaybeObjectSize::new(object_size as usize))
    }

    
    fn visit_with_map(&mut self, map: *mut Map, object: *mut HeapObject, maybe_object_size: MaybeObjectSize) -> usize {
        unsafe {
        let instance_type = (*map).instance_type;

        match instance_type {
            // Add cases for all the InstanceTypes here.
            1 => self.visit_accessor_info(map, object as *mut AccessorInfo, maybe_object_size),
            2 => self.visit_allocation_site(map, object as *mut AllocationSite, maybe_object_size),
            3 => self.visit_big_int(map, object as *mut BigInt, maybe_object_size),
            4 => self.visit_bytecode_wrapper(map, object as *mut BytecodeWrapper, maybe_object_size),
            5 => self.visit_call_site_info(map, object as *mut CallSiteInfo, maybe_object_size),
            6 => self.visit_cell(map, object as *mut Cell, maybe_object_size),
            7 => self.visit_code_wrapper(map, object as *mut CodeWrapper, maybe_object_size),
            8 => self.visit_cons_string(map, object as *mut ConsString, maybe_object_size),
            9 => self.visit_context_side_property_cell(map, object as *mut ContextSidePropertyCell, maybe_object_size),
            10 => self.visit_coverage_info(map, object as *mut CoverageInfo, maybe_object_size),
            11 => self.visit_data_handler(map, object as *mut DataHandler, maybe_object_size),
            12 => self.visit_debug_info(map, object as *mut DebugInfo, maybe_object_size),
            13 => self.visit_embedder_data_array(map, object as *mut EmbedderDataArray, maybe_object_size),
            14 => self.visit_ephemeron_hash_table(map, object as *mut EphemeronHashTable, maybe_object_size),
            15 => self.visit_external_string(map, object as *mut ExternalString, maybe_object_size),
            16 => self.visit_feedback_cell(map, object as *mut FeedbackCell, maybe_object_size),
            17 => self.visit_feedback_metadata(map, object as *mut FeedbackMetadata, maybe_object_size),
            18 => self.visit_foreign(map, object as *mut Foreign, maybe_object_size),
            19 => self.visit_function_template_info(map, object as *mut FunctionTemplateInfo, maybe_object_size),
            20 => self.visit_heap_number(map, object as *mut HeapNumber, maybe_object_size),
            21 => self.visit_hole(map, object as *mut Hole, maybe_object_size),
            22 => self.visit_map(map, object as *mut Map, maybe_object_size),
            23 => self.visit_native_context(map, object as *mut NativeContext, maybe_object_size),
            24 => self.visit_oddball(map, object as *mut Oddball, maybe_object_size),
            25 => self.visit_preparse_data(map, object as *mut PreparseData, maybe_object_size),
            26 => self.visit_property_array(map, object as *mut PropertyArray, maybe_object_size),
            27 => self.visit_property_cell(map, object as *mut PropertyCell, maybe_object_size),
            28 => self.visit_prototype_info(map, object as *mut PrototypeInfo, maybe_object_size),
            29 => self.visit_reg_exp_boilerplate_description(map, object as *mut RegExpBoilerplateDescription, maybe_object_size),
            30 => self.visit_reg_exp_data_wrapper(map, object as *mut RegExpDataWrapper, maybe_object_size),
            31 => self.visit_seq_one_byte_string(map, object as *mut SeqOneByteString, maybe_object_size),
            32 => self.visit_seq_two_byte_string(map, object as *mut SeqTwoByteString, maybe_object_size),
            33 => self.visit_shared_function_info(map, object as *mut SharedFunctionInfo, maybe_object_size),
            34 => self.visit_sliced_string(map, object as *mut SlicedString, maybe_object_size),
            35 => self.visit_sloppy_arguments_elements(map, object as *mut SloppyArgumentsElements, maybe_object_size),
            36 => self.visit_small_ordered_hash_map(map, object as *mut SmallOrderedHashMap, maybe_object_size),
            37 => self.visit_small_ordered_hash_set(map, object as *mut SmallOrderedHashSet, maybe_object_size),
            38 => self.visit_small_ordered_name_dictionary(map, object as *mut SmallOrderedNameDictionary, maybe_object_size),
            39 => self.visit_source_text_module(map, object as *mut SourceTextModule, maybe_object_size),
            40 => self.visit_swiss_name_dictionary(map, object as *mut SwissNameDictionary, maybe_object_size),
            41 => self.visit_symbol(map, object as *mut Symbol, maybe_object_size),
            42 => self.visit_synthetic_module(map, object as *mut SyntheticModule, maybe_object_size),
            43 => self.visit_thin_string(map, object as *mut ThinString, maybe_object_size),
            44 => self.visit_transition_array(map, object as *mut TransitionArray, maybe_object_size),
            45 => self.visit_weak_cell(map, object as *mut WeakCell, maybe_object_size),
            46 => self.visit_wasm_array(map, object as *mut WasmArray, maybe_object_size),
            47 => self.visit_wasm_continuation_object(map, object as *mut WasmContinuationObject, maybe_object_size),
            48 => self.visit_wasm_func_ref(map, object as *mut WasmFuncRef, maybe_object_size),
            49 => self.visit_wasm_memory_map_descriptor(map, object as *mut WasmMemoryMapDescriptor, maybe_object_size),
            50 => self.visit_wasm_null(map, object as *mut WasmNull, maybe_object_size),
            51 => self.visit_wasm_resume_data(map, object as *mut WasmResumeData, maybe_object_size),
            52 => self.visit_wasm_struct(map, object as *mut WasmStruct, maybe_object_size),
            53 => self.visit_wasm_suspender_object(map, object as *mut WasmSuspenderObject, maybe_object_size),
            54 => self.visit_wasm_type_info(map, object as *mut WasmTypeInfo, maybe_object_size),
            55 => self.visit_js_array_buffer(map, object as *mut JSArrayBuffer, maybe_object_size),
            56 => self.visit_js_data_view_or_rab_gsab_data_view(map, object as *mut JSDataViewOrRabGsabDataView, maybe_object_size),
            57 => self.visit_js_date(map, object as *mut JSDate, maybe_object_size),
            58 => self.visit_js_external_object(map, object as *mut JSExternalObject, maybe_object_size),
            59 => self.visit_js_finalization_registry(map, object as *mut JSFinalizationRegistry, maybe_object_size),
            60 => self.visit_js_function(map, object as *mut JSFunction, maybe_object_size),
            61 => self.visit_js_object(map, object as *mut JSObject, maybe_object_size),
            62 => self.visit_js_reg_exp(map, object as *mut JSRegExp, maybe_object_size),
            63 => self.visit_js_synchronization_primitive(map, object as *mut JSSynchronizationPrimitive, maybe_object_size),
            64 => self.visit_js_typed_array(map, object as *mut JSTypedArray, maybe_object_size),
            65 => self.visit_js_weak_collection(map, object as *mut JSWeakCollection, maybe_object_size),
            66 => self.visit_js_weak_ref(map, object as *mut JSWeakRef, maybe_object_size),
            67 => self.visit_wasm_global_object(map, object as *mut WasmGlobalObject, maybe_object_size),
            68 => self.visit_wasm_instance_object(map, object as *mut WasmInstanceObject, maybe_object_size),
            69 => self.visit_wasm_memory_object(map, object as *mut WasmMemoryObject, maybe_object_size),
            70 => self.visit_wasm_suspending_object(map, object as *mut WasmSuspendingObject, maybe_object_size),
            71 => self.visit_wasm_table_object(map, object as *mut WasmTableObject, maybe_object_size),
            72 => self.visit_wasm_tag_object(map, object as *mut WasmTagObject, maybe_object_size),
            73 => self.visit_byte_array(map, object as *mut ByteArray, maybe_object_size),
            74 => self.visit_descriptor_array(map, object as *mut DescriptorArray, maybe_object_size),
            75 => self.visit_feedback_vector(map, object as *mut FeedbackVector, maybe_object_size),
            76 => self.visit_filler(map, object as *mut Filler, maybe_object_size),
            77 => self.visit_fixed_array(map, object as *mut FixedArray, maybe_object_size),
            78 => self.visit_fixed_double_array(map, object as *mut FixedDoubleArray, maybe_object_size),
            79 => self.visit_free_space(map, object as *mut FreeSpace, maybe_object_size),
            80 => self.visit_scope_info(map, object as *mut ScopeInfo, maybe_object_size),

            _ => {
                println!("Unknown instance type: {}", instance_type);
                0
            }
        }
    }
    }

    #[inline]
    const fn should_visit_map_pointer() -> bool {
        true
    }
    #[inline]
    const fn should_visit_read_only_map_pointer() -> bool {
        true
    }
    #[inline]
    const fn can_encounter_filler_or_free_space() -> bool {
        true
    }
    #[inline]
    const fn should_use_unchecked_cast() -> bool {
        false
    }
    #[inline]
    const fn enable_concurrent_visitation() -> bool {
        false
    }
    #[inline]
    const fn use_precomputed_object_size() -> bool {
        false
    }

    fn visit_map_pointer_if_needed<const VISITOR_ID: usize>(&mut self, host: *mut HeapObject) {
        if Self::should_visit_map_pointer() {
            // Implement the logic to visit the map pointer here.
            // This is a placeholder, replace with actual implementation.
        }
    }

    #[inline]
    const fn should_visit_full_js_object() -> bool {
        false
    }

    fn concrete_visitor(&mut self) -> &mut ConcreteVisitor {
        unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) }
    }

    fn concrete_visitor_const(&self) -> &ConcreteVisitor {
        unsafe { &*(self as *const Self as *const ConcreteVisitor) }
    }

    fn visit_accessor_info(&mut self, map: *mut Map, object: *mut AccessorInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{1}, AccessorInfo>(map, object, maybe_object_size)
    }
    fn visit_allocation_site(&mut self, map: *mut Map, object: *mut AllocationSite, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{2}, AllocationSite>(map, object, maybe_object_size)
    }
    fn visit_big_int(&mut self, map: *mut Map, object: *mut BigInt, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{3}, BigInt>(map, object, maybe_object_size)
    }
    fn visit_bytecode_wrapper(&mut self, map: *mut Map, object: *mut BytecodeWrapper, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{4}, BytecodeWrapper>(map, object, maybe_object_size)
    }
    fn visit_call_site_info(&mut self, map: *mut Map, object: *mut CallSiteInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{5}, CallSiteInfo>(map, object, maybe_object_size)
    }
    fn visit_cell(&mut self, map: *mut Map, object: *mut Cell, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{6}, Cell>(map, object, maybe_object_size)
    }
    fn visit_code_wrapper(&mut self, map: *mut Map, object: *mut CodeWrapper, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{7}, CodeWrapper>(map, object, maybe_object_size)
    }
    fn visit_cons_string(&mut self, map: *mut Map, object: *mut ConsString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{8}, ConsString>(map, object, maybe_object_size)
    }
    fn visit_context_side_property_cell(&mut self, map: *mut Map, object: *mut ContextSidePropertyCell, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{9}, ContextSidePropertyCell>(map, object, maybe_object_size)
    }
    fn visit_coverage_info(&mut self, map: *mut Map, object: *mut CoverageInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{10}, CoverageInfo>(map, object, maybe_object_size)
    }
    fn visit_data_handler(&mut self, map: *mut Map, object: *mut DataHandler, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{11}, DataHandler>(map, object, maybe_object_size)
    }
    fn visit_debug_info(&mut self, map: *mut Map, object: *mut DebugInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{12}, DebugInfo>(map, object, maybe_object_size)
    }
    fn visit_embedder_data_array(&mut self, map: *mut Map, object: *mut EmbedderDataArray, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{13}, EmbedderDataArray>(map, object, maybe_object_size)
    }
    fn visit_ephemeron_hash_table(&mut self, map: *mut Map, object: *mut EphemeronHashTable, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{14}, EphemeronHashTable>(map, object, maybe_object_size)
    }
    fn visit_external_string(&mut self, map: *mut Map, object: *mut ExternalString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{15}, ExternalString>(map, object, maybe_object_size)
    }
    fn visit_feedback_cell(&mut self, map: *mut Map, object: *mut FeedbackCell, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{16}, FeedbackCell>(map, object, maybe_object_size)
    }
    fn visit_feedback_metadata(&mut self, map: *mut Map, object: *mut FeedbackMetadata, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{17}, FeedbackMetadata>(map, object, maybe_object_size)
    }
    fn visit_foreign(&mut self, map: *mut Map, object: *mut Foreign, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{18}, Foreign>(map, object, maybe_object_size)
    }
    fn visit_function_template_info(&mut self, map: *mut Map, object: *mut FunctionTemplateInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{19}, FunctionTemplateInfo>(map, object, maybe_object_size)
    }
    fn visit_heap_number(&mut self, map: *mut Map, object: *mut HeapNumber, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{20}, HeapNumber>(map, object, maybe_object_size)
    }
    fn visit_hole(&mut self, map: *mut Map, object: *mut Hole, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{21}, Hole>(map, object, maybe_object_size)
    }
    fn visit_map(&mut self, map: *mut Map, object: *mut Map, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{22}, Map>(map, object, maybe_object_size)
    }
    fn visit_native_context(&mut self, map: *mut Map, object: *mut NativeContext, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{23}, NativeContext>(map, object, maybe_object_size)
    }
    fn visit_oddball(&mut self, map: *mut Map, object: *mut Oddball, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{24}, Oddball>(map, object, maybe_object_size)
    }
    fn visit_preparse_data(&mut self, map: *mut Map, object: *mut PreparseData, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{25}, PreparseData>(map, object, maybe_object_size)
    }
    fn visit_property_array(&mut self, map: *mut Map, object: *mut PropertyArray, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{26}, PropertyArray>(map, object, maybe_object_size)
    }
    fn visit_property_cell(&mut self, map: *mut Map, object: *mut PropertyCell, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{27}, PropertyCell>(map, object, maybe_object_size)
    }
    fn visit_prototype_info(&mut self, map: *mut Map, object: *mut PrototypeInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{28}, PrototypeInfo>(map, object, maybe_object_size)
    }
    fn visit_reg_exp_boilerplate_description(&mut self, map: *mut Map, object: *mut RegExpBoilerplateDescription, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{29}, RegExpBoilerplateDescription>(map, object, maybe_object_size)
    }
    fn visit_reg_exp_data_wrapper(&mut self, map: *mut Map, object: *mut RegExpDataWrapper, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{30}, RegExpDataWrapper>(map, object, maybe_object_size)
    }
    fn visit_seq_one_byte_string(&mut self, map: *mut Map, object: *mut SeqOneByteString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{31}, SeqOneByteString>(map, object, maybe_object_size)
    }
    fn visit_seq_two_byte_string(&mut self, map: *mut Map, object: *mut SeqTwoByteString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{32}, SeqTwoByteString>(map, object, maybe_object_size)
    }
    fn visit_shared_function_info(&mut self, map: *mut Map, object: *mut SharedFunctionInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{33}, SharedFunctionInfo>(map, object, maybe_object_size)
    }
    fn visit_sliced_string(&mut self, map: *mut Map, object: *mut SlicedString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{34}, SlicedString>(map, object, maybe_object_size)
    }
    fn visit_sloppy_arguments_elements(&mut self, map: *mut Map, object: *mut SloppyArgumentsElements, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{35}, SloppyArgumentsElements>(map, object, maybe_object_size)
    }
    fn visit_small_ordered_hash_map(&mut self, map: *mut Map, object: *mut SmallOrderedHashMap, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{36}, SmallOrderedHashMap>(map, object, maybe_object_size)
    }
    fn visit_small_ordered_hash_set(&mut self, map: *mut Map, object: *mut SmallOrderedHashSet, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{37}, SmallOrderedHashSet>(map, object, maybe_object_size)
    }
    fn visit_small_ordered_name_dictionary(&mut self, map: *mut Map, object: *mut SmallOrderedNameDictionary, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{38}, SmallOrderedNameDictionary>(map, object, maybe_object_size)
    }
    fn visit_source_text_module(&mut self, map: *mut Map, object: *mut SourceTextModule, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{39}, SourceTextModule>(map, object, maybe_object_size)
    }
    fn visit_swiss_name_dictionary(&mut self, map: *mut Map, object: *mut SwissNameDictionary, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{40}, SwissNameDictionary>(map, object, maybe_object_size)
    }
    fn visit_symbol(&mut self, map: *mut Map, object: *mut Symbol, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{41}, Symbol>(map, object, maybe_object_size)
    }
    fn visit_synthetic_module(&mut self, map: *mut Map, object: *mut SyntheticModule, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{42}, SyntheticModule>(map, object, maybe_object_size)
    }
    fn visit_thin_string(&mut self, map: *mut Map, object: *mut ThinString, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{43}, ThinString>(map, object, maybe_object_size)
    }
    fn visit_transition_array(&mut self, map: *mut Map, object: *mut TransitionArray, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{44}, TransitionArray>(map, object, maybe_object_size)
    }
    fn visit_weak_cell(&mut self, map: *mut Map, object: *mut WeakCell, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{45}, WeakCell>(map, object, maybe_object_size)
    }
    fn visit_wasm_array(&mut self, map: *mut Map, object: *mut WasmArray, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{46}, WasmArray>(map, object, maybe_object_size)
    }
    fn visit_wasm_continuation_object(&mut self, map: *mut Map, object: *mut WasmContinuationObject, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{47}, WasmContinuationObject>(map, object, maybe_object_size)
    }
    fn visit_wasm_func_ref(&mut self, map: *mut Map, object: *mut WasmFuncRef, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{48}, WasmFuncRef>(map, object, maybe_object_size)
    }
    fn visit_wasm_memory_map_descriptor(&mut self, map: *mut Map, object: *mut WasmMemoryMapDescriptor, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{49}, WasmMemoryMapDescriptor>(map, object, maybe_object_size)
    }
    fn visit_wasm_null(&mut self, map: *mut Map, object: *mut WasmNull, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{50}, WasmNull>(map, object, maybe_object_size)
    }
    fn visit_wasm_resume_data(&mut self, map: *mut Map, object: *mut WasmResumeData, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{51}, WasmResumeData>(map, object, maybe_object_size)
    }
    fn visit_wasm_struct(&mut self, map: *mut Map, object: *mut WasmStruct, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{52}, WasmStruct>(map, object, maybe_object_size)
    }
    fn visit_wasm_suspender_object(&mut self, map: *mut Map, object: *mut WasmSuspenderObject, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{53}, WasmSuspenderObject>(map, object, maybe_object_size)
    }
    fn visit_wasm_type_info(&mut self, map: *mut Map, object: *mut WasmTypeInfo, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{54}, WasmTypeInfo>(map, object, maybe_object_size)
    }
    fn visit_js_array_buffer(&mut self, map: *mut Map, object: *mut JSArrayBuffer, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<{55}, JSArrayBuffer>(map, object, maybe_object_size
