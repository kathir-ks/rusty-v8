// Converted from V8 C++ source files:
// Header: factory-base-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/factory-base.h
pub struct FactoryBase<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> FactoryBase<T> {
    pub fn new() -> Self {
        FactoryBase {
            _phantom: std::marker::PhantomData,
        }
    }
}

// dummy definitions
pub struct Isolate {
}
impl Isolate{
    fn roots_table(&self)->RootsTable{
        RootsTable{}
    }
    fn heap(&self)->Heap{
        Heap{}
    }
}
pub struct RootsTable{}
impl RootsTable{
    fn boolean_string(&self)->Handle<String>{
        Handle::new()
    }
    fn true_value(&self)->Handle<Boolean>{
        Handle::new()
    }
    fn false_value(&self)->Handle<Boolean>{
        Handle::new()
    }
    fn empty_string(&self)->Handle<String>{
        Handle::new()
    }
    fn arguments_string(&self)->Handle<String>{
        Handle::new()
    }
    fn array_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_string(&self)->Handle<String>{
        Handle::new()
    }
    fn symbol_string(&self)->Handle<String>{
        Handle::new()
    }
    fn number_string(&self)->Handle<String>{
        Handle::new()
    }
    fn function_string(&self)->Handle<String>{
        Handle::new()
    }
    fn error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_string(&self)->Handle<String>{
        Handle::new()
    }
    fn null_string(&self)->Handle<String>{
        Handle::new()
    }
    fn undefined_string(&self)->Handle<String>{
        Handle::new()
    }
    fn infinity_string(&self)->Handle<String>{
        Handle::new()
    }
    fn nan_string(&self)->Handle<String>{
        Handle::new()
    }
    fn eval_string(&self)->Handle<String>{
        Handle::new()
    }
    fn wasm_string(&self)->Handle<String>{
        Handle::new()
    }
    fn asm_string(&self)->Handle<String>{
        Handle::new()
    }
    fn eval_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn range_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn reference_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn syntax_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn type_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn uri_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn native_error_string(&self)->Handle<String>{
        Handle::new()
    }
    fn global_this_string(&self)->Handle<String>{
        Handle::new()
    }
    fn prototype_string(&self)->Handle<String>{
        Handle::new()
    }
    fn constructor_string(&self)->Handle<String>{
        Handle::new()
    }
    fn length_string(&self)->Handle<String>{
        Handle::new()
    }
    fn name_string(&self)->Handle<String>{
        Handle::new()
    }
    fn message_string(&self)->Handle<String>{
        Handle::new()
    }
    fn stack_string(&self)->Handle<String>{
        Handle::new()
    }
    fn arguments_caller_string(&self)->Handle<String>{
        Handle::new()
    }
    fn arguments_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn arguments_callee_string(&self)->Handle<String>{
        Handle::new()
    }
    fn iterator_string(&self)->Handle<String>{
        Handle::new()
    }
    fn async_iterator_string(&self)->Handle<String>{
        Handle::new()
    }
    fn value_string(&self)->Handle<String>{
        Handle::new()
    }
    fn done_string(&self)->Handle<String>{
        Handle::new()
    }
    fn then_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_resolve_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_reject_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_any_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_all_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_race_string(&self)->Handle<String>{
        Handle::new()
    }
    fn finally_string(&self)->Handle<String>{
        Handle::new()
    }
    fn catch_string(&self)->Handle<String>{
        Handle::new()
    }
    fn finally_string(&self)->Handle<String>{
        Handle::new()
    }
    fn await_string(&self)->Handle<String>{
        Handle::new()
    }
    fn async_string(&self)->Handle<String>{
        Handle::new()
    }
    fn get_string(&self)->Handle<String>{
        Handle::new()
    }
    fn set_string(&self)->Handle<String>{
        Handle::new()
    }
    fn delete_string(&self)->Handle<String>{
        Handle::new()
    }
    fn has_string(&self)->Handle<String>{
        Handle::new()
    }
    fn construct_string(&self)->Handle<String>{
        Handle::new()
    }
    fn apply_string(&self)->Handle<String>{
        Handle::new()
    }
    fn call_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn value_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_locale_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_source_string(&self)->Handle<String>{
        Handle::new()
    }
    fn unshift_string(&self)->Handle<String>{
        Handle::new()
    }
    fn shift_string(&self)->Handle<String>{
        Handle::new()
    }
    fn push_string(&self)->Handle<String>{
        Handle::new()
    }
    fn pop_string(&self)->Handle<String>{
        Handle::new()
    }
    fn concat_string(&self)->Handle<String>{
        Handle::new()
    }
    fn join_string(&self)->Handle<String>{
        Handle::new()
    }
    fn reverse_string(&self)->Handle<String>{
        Handle::new()
    }
    fn slice_string(&self)->Handle<String>{
        Handle::new()
    }
    fn splice_string(&self)->Handle<String>{
        Handle::new()
    }
    fn sort_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_reversed_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_sorted_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_spliced_string(&self)->Handle<String>{
        Handle::new()
    }
    fn with_string(&self)->Handle<String>{
        Handle::new()
    }
    fn copy_within_string(&self)->Handle<String>{
        Handle::new()
    }
    fn fill_string(&self)->Handle<String>{
        Handle::new()
    }
    fn find_string(&self)->Handle<String>{
        Handle::new()
    }
    fn find_index_string(&self)->Handle<String>{
        Handle::new()
    }
    fn last_index_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn index_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn for_each_string(&self)->Handle<String>{
        Handle::new()
    }
    fn map_string(&self)->Handle<String>{
        Handle::new()
    }
    fn filter_string(&self)->Handle<String>{
        Handle::new()
    }
    fn reduce_string(&self)->Handle<String>{
        Handle::new()
    }
    fn reduce_right_string(&self)->Handle<String>{
        Handle::new()
    }
    fn every_string(&self)->Handle<String>{
        Handle::new()
    }
    fn some_string(&self)->Handle<String>{
        Handle::new()
    }
    fn entries_string(&self)->Handle<String>{
        Handle::new()
    }
    fn keys_string(&self)->Handle<String>{
        Handle::new()
    }
    fn values_string(&self)->Handle<String>{
        Handle::new()
    }
    fn from_string(&self)->Handle<String>{
        Handle::new()
    }
    fn of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn is_array_string(&self)->Handle<String>{
        Handle::new()
    }
    fn is_finite_string(&self)->Handle<String>{
        Handle::new()
    }
    fn is_nan_string(&self)->Handle<String>{
        Handle::new()
    }
    fn is_safe_integer_string(&self)->Handle<String>{
        Handle::new()
    }
    fn parse_int_string(&self)->Handle<String>{
        Handle::new()
    }
    fn parse_float_string(&self)->Handle<String>{
        Handle::new()
    }
    fn to_string_tag_string(&self)->Handle<String>{
        Handle::new()
    }
    fn has_instance_string(&self)->Handle<String>{
        Handle::new()
    }
    fn unhandled_rejection_string(&self)->Handle<String>{
        Handle::new()
    }
    fn rejection_handled_string(&self)->Handle<String>{
        Handle::new()
    }
    fn suppress_uncaught_exceptions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn iterator_result_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_status_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_value_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_fulfill_reactions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_reject_reactions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_has_more_reactions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn thenable_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_proto_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_resolve_functions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn promise_reject_functions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn iterator_next_string(&self)->Handle<String>{
        Handle::new()
    }
    fn iterator_return_string(&self)->Handle<String>{
        Handle::new()
    }
    fn iterator_throw_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_create_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_define_property_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_descriptor_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_names_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_symbols_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_prevent_extensions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_extensible_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_seal_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_sealed_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_freeze_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_frozen_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_set_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_keys_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_entries_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_values_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_assign_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_define_properties_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_descriptors_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_has_own_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_set_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_delete_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_has_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_construct_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_apply_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_call_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_value_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_locale_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_source_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_property_is_enumerable_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_symbols_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_prevent_extensions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_extensible_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_seal_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_sealed_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_freeze_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_frozen_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_set_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_keys_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_entries_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_values_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_assign_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_define_properties_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_descriptors_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_has_own_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_own_property_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_set_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_delete_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_has_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_construct_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_apply_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_call_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_value_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_locale_string_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_to_source_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_property_is_enumerable_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_prevent_extensions_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_extensible_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_seal_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_sealed_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_freeze_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_is_frozen_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_get_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_set_prototype_of_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_keys_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_entries_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_values_string(&self)->Handle<String>{
        Handle::new()
    }
    fn object_assign_string(&self)->Handle<String>{
        Handle::new()
    }
}

pub struct Heap{}
impl Heap{
    fn empty_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn arguments_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn array_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn symbol_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn number_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn function_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn null_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn undefined_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn infinity_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn nan_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn eval_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn wasm_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn asm_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn eval_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn range_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn reference_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn syntax_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn type_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn uri_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn native_error_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn global_this_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn prototype_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn constructor_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn length_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn name_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn message_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn stack_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn arguments_caller_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn arguments_string_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn arguments_callee_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn iterator_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn async_iterator_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn value_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn done_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn then_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_resolve_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_reject_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_any_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_all_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_race_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn finally_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn catch_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn finally_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn await_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn async_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn get_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn set_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn delete_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn has_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn construct_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn apply_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn call_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_string_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn value_of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_locale_string_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_source_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn unshift_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn shift_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn push_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn pop_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn concat_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn join_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn reverse_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn slice_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn splice_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn sort_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_reversed_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_sorted_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_spliced_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn with_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn copy_within_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn fill_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn find_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn find_index_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn last_index_of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn index_of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn for_each_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn map_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn filter_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn reduce_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn reduce_right_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn every_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn some_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn entries_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn keys_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn values_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn from_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn is_array_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn is_finite_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn is_nan_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn is_safe_integer_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn parse_int_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn parse_float_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn to_string_tag_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn has_instance_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn unhandled_rejection_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn rejection_handled_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn suppress_uncaught_exceptions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn iterator_result_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_status_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_value_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_fulfill_reactions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_reject_reactions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_has_more_reactions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn thenable_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_string_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_proto_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_resolve_functions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn promise_reject_functions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn iterator_next_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn iterator_return_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn iterator_throw_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_create_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_define_property_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_get_own_property_descriptor_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_get_own_property_names_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_get_own_property_symbols_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_prevent_extensions_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_is_extensible_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_seal_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_is_sealed_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_freeze_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_is_frozen_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_get_prototype_of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_set_prototype_of_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_keys_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_entries_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_values_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_assign_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_define_properties_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_get_own_property_descriptors_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_is_string(&self)->Tagged<String>{
        Tagged::new()
    }
    fn object_has_own_string(&self)->Tagged<String>{
        Tagged::new
