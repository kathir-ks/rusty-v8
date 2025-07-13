// Converted from V8 C++ source files:
// Header: heap-object-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_object_list {

    macro_rules! create_builtin_with_sfi_object_list_adapter {
        ($v:ident, $camel_name:ident, $underscore_name:ident, $($rest:tt)*) => {
            $v!($camel_name##SharedFun, $underscore_name##_shared_fun, $camel_name##SharedFun);
        };
    }

    macro_rules! create_builtins_with_sfi_object_list {
        ($v:ident) => {
            create_builtins_with_sfi_list_generator!(create_builtin_with_sfi_object_list_adapter, $v);
        };
    }

    macro_rules! create_heap_mutable_immovable_object_list {
        ($v:ident) => {
            $v!(ArrayIteratorProtector, array_iterator_protector, ArrayIteratorProtector);
            $v!(ArraySpeciesProtector, array_species_protector, ArraySpeciesProtector);
            $v!(IsConcatSpreadableProtector, is_concat_spreadable_protector, IsConcatSpreadableProtector);
            $v!(MapIteratorProtector, map_iterator_protector, MapIteratorProtector);
            $v!(NoElementsProtector, no_elements_protector, NoElementsProtector);
            $v!(MegaDOMProtector, mega_dom_protector, MegaDOMProtector);
            $v!(NumberStringCache, number_string_cache, NumberStringCache);
            $v!(NumberStringNotRegexpLikeProtector, number_string_not_regexp_like_protector, NumberStringNotRegexpLikeProtector);
            $v!(PromiseResolveProtector, promise_resolve_protector, PromiseResolveProtector);
            $v!(PromiseSpeciesProtector, promise_species_protector, PromiseSpeciesProtector);
            $v!(PromiseThenProtector, promise_then_protector, PromiseThenProtector);
            $v!(RegExpSpeciesProtector, regexp_species_protector, RegExpSpeciesProtector);
            $v!(SetIteratorProtector, set_iterator_protector, SetIteratorProtector);
            $v!(StringIteratorProtector, string_iterator_protector, StringIteratorProtector);
            $v!(StringWrapperToPrimitiveProtector, string_wrapper_to_primitive_protector, StringWrapperToPrimitiveProtector);
            $v!(TypedArrayLengthProtector, typed_array_length_protector, TypedArrayLengthProtector);
            $v!(TypedArraySpeciesProtector, typed_array_species_protector, TypedArraySpeciesProtector);
            create_builtins_with_sfi_object_list!($v);
        };
    }

    macro_rules! create_unique_instance_type_immutable_immovable_map_adapter {
        ($v:ident, $root_index_name:ident, $root_accessor_name:ident, $class_name:ident) => {
            $v!($root_index_name, $root_accessor_name, $class_name##Map);
        };
    }

    macro_rules! create_heap_immutable_immovable_object_list {
        ($v:ident) => {
            $v!(AllocationSiteWithoutWeakNextMap, allocation_site_without_weaknext_map, AllocationSiteWithoutWeakNextMap);
            $v!(AllocationSiteWithWeakNextMap, allocation_site_map, AllocationSiteMap);
            $v!(arguments_to_string, arguments_to_string, ArgumentsToString);
            $v!(ArrayListMap, array_list_map, ArrayListMap);
            $v!(Array_string, Array_string, ArrayString);
            $v!(array_to_string, array_to_string, ArrayToString);
            $v!(BooleanMap, boolean_map, BooleanMap);
            $v!(boolean_to_string, boolean_to_string, BooleanToString);
            $v!(class_fields_symbol, class_fields_symbol, ClassFieldsSymbol);
            $v!(ConsOneByteStringMap, cons_one_byte_string_map, ConsOneByteStringMap);
            $v!(ConsTwoByteStringMap, cons_two_byte_string_map, ConsTwoByteStringMap);
            $v!(constructor_string, constructor_string, ConstructorString);
            $v!(date_to_string, date_to_string, DateToString);
            $v!(default_string, default_string, DefaultString);
            $v!(EmptyArrayList, empty_array_list, EmptyArrayList);
            $v!(EmptyByteArray, empty_byte_array, EmptyByteArray);
            $v!(EmptyFixedArray, empty_fixed_array, EmptyFixedArray);
            $v!(EmptyOrderedHashSet, empty_ordered_hash_set, EmptyOrderedHashSet);
            $v!(EmptyScopeInfo, empty_scope_info, EmptyScopeInfo);
            $v!(EmptyPropertyDictionary, empty_property_dictionary, EmptyPropertyDictionary);
            $v!(EmptyOrderedPropertyDictionary, empty_ordered_property_dictionary, EmptyOrderedPropertyDictionary);
            $v!(EmptySwissPropertyDictionary, empty_swiss_property_dictionary, EmptySwissPropertyDictionary);
            $v!(EmptySlowElementDictionary, empty_slow_element_dictionary, EmptySlowElementDictionary);
            $v!(empty_string, empty_string, EmptyString);
            $v!(error_to_string, error_to_string, ErrorToString);
            $v!(error_string, error_string, ErrorString);
            $v!(errors_string, errors_string, ErrorsString);
            $v!(FalseValue, false_value, False);
            $v!(FixedArrayMap, fixed_array_map, FixedArrayMap);
            $v!(FixedCOWArrayMap, fixed_cow_array_map, FixedCOWArrayMap);
            $v!(Function_string, Function_string, FunctionString);
            $v!(function_to_string, function_to_string, FunctionToString);
            $v!(get_string, get_string, GetString);
            $v!(has_instance_symbol, has_instance_symbol, HasInstanceSymbol);
            $v!(has_string, has_string, HasString);
            $v!(Infinity_string, Infinity_string, InfinityString);
            $v!(is_concat_spreadable_symbol, is_concat_spreadable_symbol, IsConcatSpreadableSymbol);
            $v!(Iterator_string, Iterator_string, IteratorString);
            $v!(iterator_symbol, iterator_symbol, IteratorSymbol);
            $v!(keys_string, keys_string, KeysString);
            $v!(async_iterator_symbol, async_iterator_symbol, AsyncIteratorSymbol);
            $v!(length_string, length_string, LengthString);
            $v!(ManyClosuresCellMap, many_closures_cell_map, ManyClosuresCellMap);
            $v!(match_symbol, match_symbol, MatchSymbol);
            $v!(megamorphic_symbol, megamorphic_symbol, MegamorphicSymbol);
            $v!(mega_dom_symbol, mega_dom_symbol, MegaDOMSymbol);
            $v!(message_string, message_string, MessageString);
            $v!(minus_Infinity_string, minus_Infinity_string, MinusInfinityString);
            $v!(MinusZeroValue, minus_zero_value, MinusZero);
            $v!(name_string, name_string, NameString);
            $v!(NanValue, nan_value, Nan);
            $v!(NaN_string, NaN_string, NaNString);
            $v!(next_string, next_string, NextString);
            $v!(NoClosuresCellMap, no_closures_cell_map, NoClosuresCellMap);
            $v!(null_to_string, null_to_string, NullToString);
            $v!(NullValue, null_value, Null);
            $v!(number_string, number_string, NumberString);
            $v!(number_to_string, number_to_string, NumberToString);
            $v!(Object_string, Object_string, ObjectString);
            $v!(object_string, object_string, objectString);
            $v!(object_to_string, object_to_string, ObjectToString);
            $v!(SeqOneByteStringMap, seq_one_byte_string_map, SeqOneByteStringMap);
            $v!(OneClosureCellMap, one_closure_cell_map, OneClosureCellMap);
            $v!(OnePointerFillerMap, one_pointer_filler_map, OnePointerFillerMap);
            $v!(PromiseCapabilityMap, promise_capability_map, PromiseCapabilityMap);
            $v!(promise_forwarding_handler_symbol, promise_forwarding_handler_symbol, PromiseForwardingHandlerSymbol);
            $v!(PromiseFulfillReactionJobTaskMap, promise_fulfill_reaction_job_task_map, PromiseFulfillReactionJobTaskMap);
            $v!(promise_handled_by_symbol, promise_handled_by_symbol, PromiseHandledBySymbol);
            $v!(PromiseReactionMap, promise_reaction_map, PromiseReactionMap);
            $v!(PromiseRejectReactionJobTaskMap, promise_reject_reaction_job_task_map, PromiseRejectReactionJobTaskMap);
            $v!(PromiseResolveThenableJobTaskMap, promise_resolve_thenable_job_task_map, PromiseResolveThenableJobTaskMap);
            $v!(prototype_string, prototype_string, PrototypeString);
            $v!(replace_symbol, replace_symbol, ReplaceSymbol);
            $v!(regexp_to_string, regexp_to_string, RegexpToString);
            $v!(resolve_string, resolve_string, ResolveString);
            $v!(return_string, return_string, ReturnString);
            $v!(search_symbol, search_symbol, SearchSymbol);
            $v!(SingleCharacterStringTable, single_character_string_table, SingleCharacterStringTable);
            $v!(size_string, size_string, SizeString);
            $v!(species_symbol, species_symbol, SpeciesSymbol);
            $v!(StaleRegister, stale_register, StaleRegister);
            $v!(StoreHandler0Map, store_handler0_map, StoreHandler0Map);
            $v!(string_string, string_string, StringString);
            $v!(string_to_string, string_to_string, StringToString);
            $v!(suppressed_string, suppressed_string, SuppressedString);
            $v!(SeqTwoByteStringMap, seq_two_byte_string_map, SeqTwoByteStringMap);
            $v!(TheHoleValue, the_hole_value, TheHole);
            $v!(PropertyCellHoleValue, property_cell_hole_value, PropertyCellHole);
            $v!(HashTableHoleValue, hash_table_hole_value, HashTableHole);
            $v!(PromiseHoleValue, promise_hole_value, PromiseHole);
            $v!(then_string, then_string, ThenString);
            $v!(toJSON_string, toJSON_string, ToJSONString);
            $v!(toString_string, toString_string, ToStringString);
            $v!(to_primitive_symbol, to_primitive_symbol, ToPrimitiveSymbol);
            $v!(to_string_tag_symbol, to_string_tag_symbol, ToStringTagSymbol);
            $v!(TrueValue, true_value, True);
            $v!(undefined_to_string, undefined_to_string, UndefinedToString);
            $v!(UndefinedValue, undefined_value, Undefined);
            $v!(uninitialized_symbol, uninitialized_symbol, UninitializedSymbol);
            $v!(valueOf_string, valueOf_string, ValueOfString);
            $v!(wasm_cross_instance_call_symbol, wasm_cross_instance_call_symbol, WasmCrossInstanceCallSymbol);
            $v!(zero_string, zero_string, ZeroString);
            create_unique_instance_type_map_list_generator!(create_unique_instance_type_immutable_immovable_map_adapter, $v);
        };
    }

    macro_rules! create_heap_immovable_object_list {
        ($v:ident) => {
            create_heap_mutable_immovable_object_list!($v);
            create_heap_immutable_immovable_object_list!($v);
        };
    }

    macro_rules! create_builtins_with_sfi_list_generator {
        ($adapter:ident, $v:ident) => {};
    }

    macro_rules! create_unique_instance_type_map_list_generator {
        ($adapter:ident, $v:ident) => {};
    }
}
