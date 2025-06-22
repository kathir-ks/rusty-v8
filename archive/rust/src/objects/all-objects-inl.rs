// This file includes all inline headers from src/objects, which is handy for
// compilation units that need it like object printing or verification.
// New inline headers should be added here.

pub mod allocation_site_inl;
pub mod allocation_site_scopes_inl;
pub mod api_callbacks_inl;
pub mod arguments_inl;
pub mod call_site_info_inl;
pub mod cell_inl;
pub mod code_inl;
pub mod compilation_cache_table_inl;
pub mod compressed_slots_inl;
pub mod contexts_inl;
pub mod data_handler_inl;
pub mod debug_objects_inl;
pub mod descriptor_array_inl;
pub mod dictionary_inl;
pub mod elements_inl;
pub mod embedder_data_array_inl;
pub mod embedder_data_slot_inl;
pub mod feedback_cell_inl;
pub mod feedback_vector_inl;
pub mod field_index_inl;
pub mod fixed_array_inl;
pub mod foreign_inl;
pub mod free_space_inl;
pub mod hash_table_inl;
pub mod heap_number_inl;
pub mod heap_object_inl;
pub mod instance_type_inl;
pub mod js_array_buffer_inl;
pub mod js_array_inl;
pub mod js_atomics_synchronization_inl;
pub mod js_collection_inl;
pub mod js_disposable_stack_inl;
pub mod js_function_inl;
pub mod js_generator_inl;
pub mod js_iterator_helpers_inl;
pub mod js_objects_inl;
pub mod js_promise_inl;
pub mod js_proxy_inl;
pub mod js_raw_json_inl;
pub mod js_regexp_inl;
pub mod js_regexp_string_iterator_inl;
pub mod js_shadow_realm_inl;
pub mod js_shared_array_inl;
pub mod js_struct_inl;
pub mod js_temporal_objects_inl;
pub mod js_weak_refs_inl;
pub mod literal_objects_inl;
pub mod lookup_cache_inl;
pub mod lookup_inl;
pub mod map_inl;
pub mod maybe_object_inl;
pub mod megadom_handler_inl;
pub mod microtask_inl;
pub mod module_inl;
pub mod name_inl;
pub mod objects_inl;
pub mod oddball_inl;
pub mod ordered_hash_table_inl;
pub mod primitive_heap_object_inl;
pub mod promise_inl;
pub mod property_array_inl;
pub mod property_cell_inl;
pub mod property_descriptor_object_inl;
pub mod prototype_info_inl;
pub mod scope_info_inl;
pub mod script_inl;
pub mod shared_function_info_inl;
pub mod slots_atomic_inl;
pub mod slots_inl;
pub mod string_forwarding_table_inl;
pub mod string_inl;
pub mod string_set_inl;
pub mod string_table_inl;
pub mod struct_inl;
pub mod swiss_name_dictionary_inl;
pub mod synthetic_module_inl;
pub mod tagged_field_inl;
pub mod tagged_impl_inl;
pub mod tagged_value_inl;
pub mod template_objects_inl;
pub mod templates_inl;
pub mod torque_defined_classes_inl;
pub mod transitions_inl;
pub mod trusted_object_inl;
pub mod turbofan_types_inl;
pub mod turboshaft_types_inl;

// The following section needs to be conditionally compiled based on the `V8_INTL_SUPPORT` flag
// This flag seems to be related to Internationalization support.
// Since the actual implementation details for the `v8_intl_support` feature
// are not provided, this is a placeholder.
#[cfg(feature = "v8_intl_support")]
pub mod i18n {
    pub mod js_break_iterator_inl;
    pub mod js_collator_inl;
    pub mod js_date_time_format_inl;
    pub mod js_display_names_inl;
    pub mod js_duration_format_inl;
    pub mod js_list_format_inl;
    pub mod js_locale_inl;
    pub mod js_number_format_inl;
    pub mod js_plural_rules_inl;
    pub mod js_relative_time_format_inl;
    pub mod js_segment_iterator_inl;
    pub mod js_segmenter_inl;
    pub mod js_segments_inl;
}

#[cfg(not(feature = "v8_intl_support"))]
pub mod i18n {
    // Dummy modules to avoid compilation errors when the feature is disabled.
    pub mod js_break_iterator_inl;
    pub mod js_collator_inl;
    pub mod js_date_time_format_inl;
    pub mod js_display_names_inl;
    pub mod js_duration_format_inl;
    pub mod js_list_format_inl;
    pub mod js_locale_inl;
    pub mod js_number_format_inl;
    pub mod js_plural_rules_inl;
    pub mod js_relative_time_format_inl;
    pub mod js_segment_iterator_inl;
    pub mod js_segmenter_inl;
    pub mod js_segments_inl;
}
