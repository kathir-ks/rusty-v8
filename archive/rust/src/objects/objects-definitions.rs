pub mod objects_definitions {

    /// All Maps have a field instance_type containing an InstanceType.
    /// It describes the type of the instances.
    ///
    /// As an example, a JavaScript object is a heap object and its map
    /// instance_type is JS_OBJECT_TYPE.
    ///
    /// The names of the string instance types are intended to systematically mirror
    /// their encoding in the instance_type field of the map.  The other
    /// representations (e.g. CONS, EXTERNAL) are explicitly mentioned.  Finally,
    /// the string is either a STRING_TYPE (if it is a normal string) or an
    /// INTERNALIZED_STRING_TYPE (if it is an internalized string).
    ///
    /// NOTE: The following things are some that depend on the string types having
    /// instance_types that are less than those of all other types:
    /// HeapObject::Size, HeapObject::IterateBody, the typeof operator, and
    /// Object::IsString.

    macro_rules! instance_type_list_base {
        ($V:ident) => {
            $V!(INTERNALIZED_TWO_BYTE_STRING_TYPE);
            $V!(EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE);
            $V!(INTERNALIZED_ONE_BYTE_STRING_TYPE);
            $V!(EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE);
            $V!(UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE);
            $V!(UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE);
            $V!(SEQ_TWO_BYTE_STRING_TYPE);
            $V!(CONS_TWO_BYTE_STRING_TYPE);
            $V!(EXTERNAL_TWO_BYTE_STRING_TYPE);
            $V!(SLICED_TWO_BYTE_STRING_TYPE);
            $V!(THIN_TWO_BYTE_STRING_TYPE);
            $V!(SEQ_ONE_BYTE_STRING_TYPE);
            $V!(CONS_ONE_BYTE_STRING_TYPE);
            $V!(EXTERNAL_ONE_BYTE_STRING_TYPE);
            $V!(SLICED_ONE_BYTE_STRING_TYPE);
            $V!(THIN_ONE_BYTE_STRING_TYPE);
            $V!(UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE);
            $V!(UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE);
            $V!(SHARED_SEQ_TWO_BYTE_STRING_TYPE);
            $V!(SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE);
            $V!(SHARED_SEQ_ONE_BYTE_STRING_TYPE);
            $V!(SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE);
            $V!(SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE);
            $V!(SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE);
        };
    }

    macro_rules! instance_type_list {
        ($V:ident) => {
            instance_type_list_base!($V);
            // TORQUE_ASSIGNED_INSTANCE_TYPE_LIST($V) - Needs Torque integration.  Skipping for now.
        };
    }

    /// Since string types are not consecutive, this macro is used to iterate over
    /// them. The order matters for read only heap layout. The maps are placed such
    /// that string types map to address ranges of maps.
    macro_rules! string_type_list {
        ($V:ident) => {
            /* Start sequential strings*/
            $V!(SEQ_TWO_BYTE_STRING_TYPE, kVariableSizeSentinel, seq_two_byte_string, SeqTwoByteString);
            $V!(SEQ_ONE_BYTE_STRING_TYPE, kVariableSizeSentinel, seq_one_byte_string, SeqOneByteString);
            $V!(SHARED_SEQ_TWO_BYTE_STRING_TYPE, kVariableSizeSentinel, shared_seq_two_byte_string, SharedSeqTwoByteString);
            $V!(SHARED_SEQ_ONE_BYTE_STRING_TYPE, kVariableSizeSentinel, shared_seq_one_byte_string, SharedSeqOneByteString);
            /* Start internalized strings*/
            $V!(INTERNALIZED_TWO_BYTE_STRING_TYPE, kVariableSizeSentinel, internalized_two_byte_string, InternalizedTwoByteString);
            $V!(INTERNALIZED_ONE_BYTE_STRING_TYPE, kVariableSizeSentinel, internalized_one_byte_string, InternalizedOneByteString);
            /* End sequential strings*/
            /* Start external strings*/
            $V!(EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE, size_of::<ExternalTwoByteString>(), external_internalized_two_byte_string, ExternalInternalizedTwoByteString);
            $V!(EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE, size_of::<ExternalOneByteString>(), external_internalized_one_byte_string, ExternalInternalizedOneByteString);
            /* Start uncached external strings*/
            $V!(UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), uncached_external_internalized_two_byte_string, UncachedExternalInternalizedTwoByteString);
            $V!(UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), uncached_external_internalized_one_byte_string, UncachedExternalInternalizedOneByteString);
            /* End internalized strings*/
            $V!(UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), uncached_external_two_byte_string, UncachedExternalTwoByteString);
            $V!(UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), uncached_external_one_byte_string, UncachedExternalOneByteString);
            $V!(SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), shared_uncached_external_two_byte_string, SharedUncachedExternalTwoByteString);
            $V!(SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE, size_of::<UncachedExternalString>(), shared_uncached_external_one_byte_string, SharedUncachedExternalOneByteString);
            /* End uncached external strings*/
            $V!(EXTERNAL_TWO_BYTE_STRING_TYPE, size_of::<ExternalTwoByteString>(), external_two_byte_string, ExternalTwoByteString);
            $V!(EXTERNAL_ONE_BYTE_STRING_TYPE, size_of::<ExternalOneByteString>(), external_one_byte_string, ExternalOneByteString);
            $V!(SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE, size_of::<ExternalTwoByteString>(), shared_external_two_byte_string, SharedExternalTwoByteString);
            $V!(SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE, size_of::<ExternalOneByteString>(), shared_external_one_byte_string, SharedExternalOneByteString);
            /* End external strings*/

            $V!(CONS_TWO_BYTE_STRING_TYPE, size_of::<ConsString>(), cons_two_byte_string, ConsTwoByteString);
            $V!(CONS_ONE_BYTE_STRING_TYPE, size_of::<ConsString>(), cons_one_byte_string, ConsOneByteString);
            $V!(SLICED_TWO_BYTE_STRING_TYPE, size_of::<SlicedString>(), sliced_two_byte_string, SlicedTwoByteString);
            $V!(SLICED_ONE_BYTE_STRING_TYPE, size_of::<SlicedString>(), sliced_one_byte_string, SlicedOneByteString);
            $V!(THIN_TWO_BYTE_STRING_TYPE, size_of::<ThinString>(), thin_two_byte_string, ThinTwoByteString);
            $V!(THIN_ONE_BYTE_STRING_TYPE, size_of::<ThinString>(), thin_one_byte_string, ThinOneByteString);
        };
    }

    // Placeholder values
    const kVariableSizeSentinel: usize = 0;

    // Placeholder structs.  Need actual implementations.
    struct ExternalTwoByteString {}
    struct ExternalOneByteString {}
    struct UncachedExternalString {}
    struct ConsString {}
    struct SlicedString {}
    struct ThinString {}

    use std::mem::size_of;

    /// A struct is a simple object a set of object-valued fields.  Including an
    /// object type in this causes the compiler to generate most of the boilerplate
    /// code for the class including allocation and garbage collection routines,
    /// casts and predicates.  All you need to define is the class, methods and
    /// object verification routines.  Easy, no?
    macro_rules! struct_list_generator {
        ($V:ident, $IGNORE:tt) => {
            $V!($IGNORE, PROMISE_FULFILL_REACTION_JOB_TASK_TYPE, PromiseFulfillReactionJobTask, promise_fulfill_reaction_job_task);
            $V!($IGNORE, PROMISE_REJECT_REACTION_JOB_TASK_TYPE, PromiseRejectReactionJobTask, promise_reject_reaction_job_task);
            $V!($IGNORE, CALLABLE_TASK_TYPE, CallableTask, callable_task);
            $V!($IGNORE, CALLBACK_TASK_TYPE, CallbackTask, callback_task);
            $V!($IGNORE, PROMISE_RESOLVE_THENABLE_JOB_TASK_TYPE, PromiseResolveThenableJobTask, promise_resolve_thenable_job_task);
            $V!($IGNORE, ACCESS_CHECK_INFO_TYPE, AccessCheckInfo, access_check_info);
            $V!($IGNORE, ACCESSOR_PAIR_TYPE, AccessorPair, accessor_pair);
            $V!($IGNORE, ALIASED_ARGUMENTS_ENTRY_TYPE, AliasedArgumentsEntry, aliased_arguments_entry);
            $V!($IGNORE, ALLOCATION_MEMENTO_TYPE, AllocationMemento, allocation_memento);
            $V!($IGNORE, ARRAY_BOILERPLATE_DESCRIPTION_TYPE, ArrayBoilerplateDescription, array_boilerplate_description);
            // IF_WASM($V, _, ASM_WASM_DATA_TYPE, AsmWasmData, asm_wasm_data)  - WASM conditional.  Skipping for now.
            $V!($IGNORE, ASYNC_GENERATOR_REQUEST_TYPE, AsyncGeneratorRequest, async_generator_request);
            $V!($IGNORE, BREAK_POINT_TYPE, BreakPoint, break_point);
            $V!($IGNORE, BREAK_POINT_INFO_TYPE, BreakPointInfo, break_point_info);
            $V!($IGNORE, BYTECODE_WRAPPER_TYPE, BytecodeWrapper, bytecode_wrapper);
            $V!($IGNORE, CALL_SITE_INFO_TYPE, CallSiteInfo, call_site_info);
            $V!($IGNORE, CLASS_BOILERPLATE_TYPE, ClassBoilerplate, class_boilerplate);
            $V!($IGNORE, CLASS_POSITIONS_TYPE, ClassPositions, class_positions);
            $V!($IGNORE, CODE_WRAPPER_TYPE, CodeWrapper, code_wrapper);
            $V!($IGNORE, DEBUG_INFO_TYPE, DebugInfo, debug_info);
            $V!($IGNORE, ENUM_CACHE_TYPE, EnumCache, enum_cache);
            $V!($IGNORE, ERROR_STACK_DATA_TYPE, ErrorStackData, error_stack_data);
            $V!($IGNORE, FUNCTION_TEMPLATE_RARE_DATA_TYPE, FunctionTemplateRareData, function_template_rare_data);
            $V!($IGNORE, INTERCEPTOR_INFO_TYPE, InterceptorInfo, interceptor_info);
            $V!($IGNORE, MODULE_REQUEST_TYPE, ModuleRequest, module_request);
            $V!($IGNORE, PROMISE_CAPABILITY_TYPE, PromiseCapability, promise_capability);
            $V!($IGNORE, PROMISE_REACTION_TYPE, PromiseReaction, promise_reaction);
            $V!($IGNORE, PROPERTY_DESCRIPTOR_OBJECT_TYPE, PropertyDescriptorObject, property_descriptor_object);
            $V!($IGNORE, PROTOTYPE_INFO_TYPE, PrototypeInfo, prototype_info);
            $V!($IGNORE, REG_EXP_BOILERPLATE_DESCRIPTION_TYPE, RegExpBoilerplateDescription, regexp_boilerplate_description);
            $V!($IGNORE, REG_EXP_DATA_WRAPPER_TYPE, RegExpDataWrapper, regexp_data_wrapper);
            $V!($IGNORE, SCRIPT_TYPE, Script, script);
            $V!($IGNORE, SCRIPT_OR_MODULE_TYPE, ScriptOrModule, script_or_module);
            $V!($IGNORE, SOURCE_TEXT_MODULE_INFO_ENTRY_TYPE, SourceTextModuleInfoEntry, module_info_entry);
            $V!($IGNORE, STACK_FRAME_INFO_TYPE, StackFrameInfo, stack_frame_info);
            $V!($IGNORE, STACK_TRACE_INFO_TYPE, StackTraceInfo, stack_trace_info);
            $V!($IGNORE, TEMPLATE_OBJECT_DESCRIPTION_TYPE, TemplateObjectDescription, template_object_description);
            $V!($IGNORE, TUPLE2_TYPE, Tuple2, tuple2);
            // IF_WASM($V, _, WASM_EXCEPTION_TAG_TYPE, WasmExceptionTag, wasm_exception_tag) - WASM Conditional.  Skipping for now.
        };
    }

    // Placeholder types.  Need actual implementations.
    type PromiseFulfillReactionJobTask = u32;
    type PromiseRejectReactionJobTask = u32;
    type CallableTask = u32;
    type CallbackTask = u32;
    type PromiseResolveThenableJobTask = u32;
    type AccessCheckInfo = u32;
    type AccessorPair = u32;
    type AliasedArgumentsEntry = u32;
    type AllocationMemento = u32;
    type ArrayBoilerplateDescription = u32;
    type AsyncGeneratorRequest = u32;
    type BreakPoint = u32;
    type BreakPointInfo = u32;
    type BytecodeWrapper = u32;
    type CallSiteInfo = u32;
    type ClassBoilerplate = u32;
    type ClassPositions = u32;
    type CodeWrapper = u32;
    type DebugInfo = u32;
    type EnumCache = u32;
    type ErrorStackData = u32;
    type FunctionTemplateRareData = u32;
    type InterceptorInfo = u32;
    type ModuleRequest = u32;
    type PromiseCapability = u32;
    type PromiseReaction = u32;
    type PropertyDescriptorObject = u32;
    type PrototypeInfo = u32;
    type RegExpBoilerplateDescription = u32;
    type RegExpDataWrapper = u32;
    type Script = u32;
    type ScriptOrModule = u32;
    type SourceTextModuleInfoEntry = u32;
    type StackFrameInfo = u32;
    type StackTraceInfo = u32;
    type TemplateObjectDescription = u32;
    type Tuple2 = u32;
    type Map = u32;

    // Adapts one STRUCT_LIST_GENERATOR entry to the STRUCT_LIST entry
    macro_rules! struct_list_adapter {
        ($V:ident, $NAME:ident, $Name:ident, $name:ident) => {
            $V!($NAME, $Name, $name);
        };
    }

    // Produces (NAME, Name, name) entries.
    macro_rules! struct_list {
        ($V:ident) => {
            struct_list_generator!(struct_list_adapter, $V);
        };
    }

    // Adapts one STRUCT_LIST_GENERATOR entry to the STRUCT_MAPS_LIST entry
    macro_rules! struct_maps_list_adapter {
        ($V:ident, $NAME:ident, $Name:ident, $name:ident) => {
            $V!(Map, concat_idents::concat_idents!($name, _map), concat_idents::concat_idents!($Name, Map));
        };
    }

    // Produces (Map, struct_name_map, StructNameMap) entries
    macro_rules! struct_maps_list {
        ($V:ident) => {
            struct_list_generator!(struct_maps_list_adapter, $V);
        };
    }

    //
    // The following macros define list of allocation size objects and list of
    // their maps.
    //
    macro_rules! allocation_site_list {
        ($V:ident, $IGNORE:tt) => {
            $V!($IGNORE, ALLOCATION_SITE_TYPE, AllocationSite, WithWeakNext, allocation_site);
            $V!($IGNORE, ALLOCATION_SITE_TYPE, AllocationSite, WithoutWeakNext, allocation_site_without_weaknext);
        };
    }

    // Placeholder for AllocationSite
    type AllocationSite = u32;

    // Adapts one ALLOCATION_SITE_LIST entry to the ALLOCATION_SITE_MAPS_LIST entry
    macro_rules! allocation_site_maps_list_adapter {
        ($V:ident, $TYPE:ident, $Name:ident, $Size:ident, $name_size:ident) => {
            $V!(Map, concat_idents::concat_idents!($name_size, _map), concat_idents::concat_idents!($Name, $Size, Map));
        };
    }

    // Produces (Map, allocation_site_name_map, AllocationSiteNameMap) entries
    macro_rules! allocation_site_maps_list {
        ($V:ident) => {
            allocation_site_list!(allocation_site_maps_list_adapter, $V);
        };
    }

    //
    // The following macros define list of data handler objects and list of their
    // maps.
    //
    macro_rules! data_handler_list {
        ($V:ident, $IGNORE:tt) => {
            $V!($IGNORE, LOAD_HANDLER_TYPE, LoadHandler, 1, load_handler1);
            $V!($IGNORE, LOAD_HANDLER_TYPE, LoadHandler, 2, load_handler2);
            $V!($IGNORE, LOAD_HANDLER_TYPE, LoadHandler, 3, load_handler3);
            $V!($IGNORE, STORE_HANDLER_TYPE, StoreHandler, 0, store_handler0);
            $V!($IGNORE, STORE_HANDLER_TYPE, StoreHandler, 1, store_handler1);
            $V!($IGNORE, STORE_HANDLER_TYPE, StoreHandler, 2, store_handler2);
            $V!($IGNORE, STORE_HANDLER_TYPE, StoreHandler, 3, store_handler3);
        };
    }

    // Placeholder for LoadHandler and StoreHandler
    type LoadHandler = u32;
    type StoreHandler = u32;

    // Adapts one DATA_HANDLER_LIST entry to the DATA_HANDLER_MAPS_LIST entry.
    macro_rules! data_handler_maps_list_adapter {
        ($V:ident, $TYPE:ident, $Name:ident, $Size:tt, $name_size:ident) => {
            $V!(Map, concat_idents::concat_idents!($name_size, _map), concat_idents::concat_idents!($Name, $Size, Map));
        };
    }

    // Produces (Map, handler_name_map, HandlerNameMap) entries
    macro_rules! data_handler_maps_list {
        ($V:ident) => {
            data_handler_list!(data_handler_maps_list_adapter, $V);
        };
    }

    // Example usage (need to expand each usage)
    // instance_type_list!(println);
    // string_type_list!(println);
    // struct_list!(println);
    // struct_maps_list!(println);
    // allocation_site_list!(println, _);
    // allocation_site_maps_list!(println);
    // data_handler_list!(println, _);
    // data_handler_maps_list!(println);
}