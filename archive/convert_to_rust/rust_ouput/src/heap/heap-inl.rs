// Converted from V8 C++ source files:
// Header: heap-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

use crate::heap::heap_allocator::{AllocationAlignment, AllocationOrigin, AllocationResult, AllocationType};
use crate::heap::heap_layout_inl::HeapLayout;
use crate::heap::memory_chunk::MemoryChunk;
use crate::heap::memory_chunk_inl::PageMetadata;
use crate::heap::new_spaces_inl::PagedNewSpace;
use crate::heap::new_spaces_inl::SemiSpaceNewSpace;
use crate::heap::paged_spaces_inl::PagedSpace;
use crate::heap::read_only_heap::ReadOnlyRoots;
use crate::heap::safepoint::V8;
use crate::heap::spaces_inl::BaseSpace;
use crate::heap::spaces_inl::Space;
use crate::heap::spaces_inl::StickySpace;
use crate::objects::objects_inl::HeapObject;
use crate::objects::objects_inl::MaybeObject;
use crate::objects::objects_inl::Object;
use crate::objects::slots_inl::Visited;
use crate::roots::static_roots::RootIndex;
use crate::v8::internal::{
    Address, Cast, IndirectHandle, Isolate, IsolateData, MutexGuard as MutexGuardInternal, String, TemplateInfo,
};

const kTaggedSize: usize = 8;

const kMaxRegularHeapObjectSize: i32 = 1024;

const kInitialNumberStringCacheSize: usize = 16;

const kNullAddress: Address = Address {};

#[derive(Debug)]
pub struct Heap {
    external_memory_: ExternalMemory,
    roots_table_: Box<RootsTable>,
    deserialization_complete_: bool,
    heap_allocator_: Box<HeapAllocator>,
    space_: Vec<Option<Box<dyn GenericSpace>>>,
    new_space_: Option<Box<dyn NewSpace>>,
    old_space_: Option<Box<dyn OldSpace>>,
    code_range_: Option<Box<CodeRange>>,
    max_regular_code_object_size_: i32,
    external_string_table_: ExternalStringTable,
    next_template_serial_number_: Box<Smi>,
    max_semi_space_size_: usize,
    backing_store_bytes_: AtomicU64,
    always_allocate_scope_count_: i32,
    ignore_local_gc_requests_depth_: i32,
}

impl Heap {
    pub fn isolate(&self) -> &mut Isolate {
        Isolate::from_heap(self)
    }

    pub fn is_main_thread(&self) -> bool {
        self.isolate().thread_id() == ThreadId::Current()
    }

    pub fn external_memory(&self) -> u64 {
        self.external_memory_.total()
    }

    pub fn roots_table(&mut self) -> &mut RootsTable {
        &mut self.roots_table_
    }

    pub fn undefined_value(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kUndefined as usize].into()
    }
    pub fn set_undefined_value(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kUndefined as usize] = value.into();
    }

    pub fn the_hole_value(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kTheHoleValue as usize].into()
    }
    pub fn set_the_hole_value(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kTheHoleValue as usize] = value.into();
    }

    pub fn null_value(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kNullValue as usize].into()
    }
    pub fn set_null_value(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kNullValue as usize] = value.into();
    }

    pub fn true_value(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kTrueValue as usize].into()
    }
    pub fn set_true_value(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kTrueValue as usize] = value.into();
    }

    pub fn false_value(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kFalseValue as usize].into()
    }
    pub fn set_false_value(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kFalseValue as usize] = value.into();
    }

    pub fn empty_string(&self) -> Tagged<String> {
        self.roots_table_.roots[RootIndex::kEmptyString as usize].into()
    }
    pub fn set_empty_string(&mut self, value: Tagged<String>) {
        self.roots_table_.roots[RootIndex::kEmptyString as usize] = value.into();
    }

    pub fn array_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayPrototype as usize].into()
    }
    pub fn set_array_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayPrototype as usize] = value.into();
    }

    pub fn array_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayConstructor as usize].into()
    }
    pub fn set_array_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayConstructor as usize] = value.into();
    }

    pub fn object_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kObjectPrototype as usize].into()
    }
    pub fn set_object_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kObjectPrototype as usize] = value.into();
    }

    pub fn object_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kObjectConstructor as usize].into()
    }
    pub fn set_object_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kObjectConstructor as usize] = value.into();
    }

    pub fn function_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kFunctionPrototype as usize].into()
    }
    pub fn set_function_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kFunctionPrototype as usize] = value.into();
    }

    pub fn function_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kFunctionConstructor as usize].into()
    }
    pub fn set_function_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kFunctionConstructor as usize] = value.into();
    }

    pub fn string_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kStringPrototype as usize].into()
    }
    pub fn set_string_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kStringPrototype as usize] = value.into();
    }

    pub fn string_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kStringConstructor as usize].into()
    }
    pub fn set_string_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kStringConstructor as usize] = value.into();
    }

    pub fn number_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kNumberPrototype as usize].into()
    }
    pub fn set_number_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kNumberPrototype as usize] = value.into();
    }

    pub fn number_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kNumberConstructor as usize].into()
    }
    pub fn set_number_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kNumberConstructor as usize] = value.into();
    }

    pub fn boolean_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kBooleanPrototype as usize].into()
    }
    pub fn set_boolean_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kBooleanPrototype as usize] = value.into();
    }

    pub fn boolean_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kBooleanConstructor as usize].into()
    }
    pub fn set_boolean_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kBooleanConstructor as usize] = value.into();
    }

    pub fn symbol_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSymbolPrototype as usize].into()
    }
    pub fn set_symbol_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSymbolPrototype as usize] = value.into();
    }

    pub fn symbol_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSymbolConstructor as usize].into()
    }
    pub fn set_symbol_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSymbolConstructor as usize] = value.into();
    }

    pub fn map_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kMapPrototype as usize].into()
    }
    pub fn set_map_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kMapPrototype as usize] = value.into();
    }

    pub fn map_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kMapConstructor as usize].into()
    }
    pub fn set_map_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kMapConstructor as usize] = value.into();
    }

    pub fn set_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSetPrototype as usize].into()
    }
    pub fn set_set_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSetPrototype as usize] = value.into();
    }

    pub fn set_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSetConstructor as usize].into()
    }
    pub fn set_set_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSetConstructor as usize] = value.into();
    }

    pub fn weak_map_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kWeakMapPrototype as usize].into()
    }
    pub fn set_weak_map_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kWeakMapPrototype as usize] = value.into();
    }

    pub fn weak_map_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kWeakMapConstructor as usize].into()
    }
    pub fn set_weak_map_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kWeakMapConstructor as usize] = value.into();
    }

    pub fn weak_set_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kWeakSetPrototype as usize].into()
    }
    pub fn set_weak_set_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kWeakSetPrototype as usize] = value.into();
    }

    pub fn weak_set_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kWeakSetConstructor as usize].into()
    }
    pub fn set_weak_set_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kWeakSetConstructor as usize] = value.into();
    }

    pub fn array_buffer_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayBufferPrototype as usize].into()
    }
    pub fn set_array_buffer_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayBufferPrototype as usize] = value.into();
    }

    pub fn array_buffer_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayBufferConstructor as usize].into()
    }
    pub fn set_array_buffer_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayBufferConstructor as usize] = value.into();
    }

    pub fn data_view_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kDataViewPrototype as usize].into()
    }
    pub fn set_data_view_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kDataViewPrototype as usize] = value.into();
    }

    pub fn data_view_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kDataViewConstructor as usize].into()
    }
    pub fn set_data_view_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kDataViewConstructor as usize] = value.into();
    }

    pub fn promise_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromisePrototype as usize].into()
    }
    pub fn set_promise_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromisePrototype as usize] = value.into();
    }

    pub fn promise_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromiseConstructor as usize].into()
    }
    pub fn set_promise_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromiseConstructor as usize] = value.into();
    }

    pub fn promise_resolve(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromiseResolve as usize].into()
    }
    pub fn set_promise_resolve(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromiseResolve as usize] = value.into();
    }

    pub fn promise_reject(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromiseReject as usize].into()
    }
    pub fn set_promise_reject(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromiseReject as usize] = value.into();
    }

    pub fn promise_finally(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromiseFinally as usize].into()
    }
    pub fn set_promise_finally(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromiseFinally as usize] = value.into();
    }

    pub fn async_function_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAsyncFunctionPrototype as usize].into()
    }
    pub fn set_async_function_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAsyncFunctionPrototype as usize] = value.into();
    }

    pub fn async_function_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAsyncFunctionConstructor as usize].into()
    }
    pub fn set_async_function_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAsyncFunctionConstructor as usize] = value.into();
    }

    pub fn async_generator_function_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAsyncGeneratorFunctionPrototype as usize].into()
    }
    pub fn set_async_generator_function_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAsyncGeneratorFunctionPrototype as usize] = value.into();
    }

    pub fn async_generator_function_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAsyncGeneratorFunctionConstructor as usize].into()
    }
    pub fn set_async_generator_function_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAsyncGeneratorFunctionConstructor as usize] = value.into();
    }

    pub fn generator_function_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kGeneratorFunctionPrototype as usize].into()
    }
    pub fn set_generator_function_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kGeneratorFunctionPrototype as usize] = value.into();
    }

    pub fn generator_function_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kGeneratorFunctionConstructor as usize].into()
    }
    pub fn set_generator_function_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kGeneratorFunctionConstructor as usize] = value.into();
    }

    pub fn regexp_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kRegexpPrototype as usize].into()
    }
    pub fn set_regexp_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kRegexpPrototype as usize] = value.into();
    }

    pub fn regexp_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kRegexpConstructor as usize].into()
    }
    pub fn set_regexp_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kRegexpConstructor as usize] = value.into();
    }

    pub fn error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kErrorPrototype as usize].into()
    }
    pub fn set_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kErrorPrototype as usize] = value.into();
    }

    pub fn error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kErrorConstructor as usize].into()
    }
    pub fn set_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kErrorConstructor as usize] = value.into();
    }

    pub fn eval_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kEvalErrorPrototype as usize].into()
    }
    pub fn set_eval_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kEvalErrorPrototype as usize] = value.into();
    }

    pub fn eval_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kEvalErrorConstructor as usize].into()
    }
    pub fn set_eval_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kEvalErrorConstructor as usize] = value.into();
    }

    pub fn range_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kRangeErrorPrototype as usize].into()
    }
    pub fn set_range_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kRangeErrorPrototype as usize] = value.into();
    }

    pub fn range_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kRangeErrorConstructor as usize].into()
    }
    pub fn set_range_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kRangeErrorConstructor as usize] = value.into();
    }

    pub fn reference_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReferenceErrorPrototype as usize].into()
    }
    pub fn set_reference_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReferenceErrorPrototype as usize] = value.into();
    }

    pub fn reference_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReferenceErrorConstructor as usize].into()
    }
    pub fn set_reference_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReferenceErrorConstructor as usize] = value.into();
    }

    pub fn syntax_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSyntaxErrorPrototype as usize].into()
    }
    pub fn set_syntax_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSyntaxErrorPrototype as usize] = value.into();
    }

    pub fn syntax_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSyntaxErrorConstructor as usize].into()
    }
    pub fn set_syntax_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSyntaxErrorConstructor as usize] = value.into();
    }

    pub fn type_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kTypeErrorPrototype as usize].into()
    }
    pub fn set_type_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kTypeErrorPrototype as usize] = value.into();
    }

    pub fn type_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kTypeErrorConstructor as usize].into()
    }
    pub fn set_type_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kTypeErrorConstructor as usize] = value.into();
    }

    pub fn uri_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kURIErrorPrototype as usize].into()
    }
    pub fn set_uri_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kURIErrorPrototype as usize] = value.into();
    }

    pub fn uri_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kURIErrorConstructor as usize].into()
    }
    pub fn set_uri_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kURIErrorConstructor as usize] = value.into();
    }

    pub fn oob_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kOOBErrorPrototype as usize].into()
    }
    pub fn set_oob_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kOOBErrorPrototype as usize] = value.into();
    }

    pub fn oob_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kOOBErrorConstructor as usize].into()
    }
    pub fn set_oob_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kOOBErrorConstructor as usize] = value.into();
    }

    pub fn aggregate_error_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAggregateErrorPrototype as usize].into()
    }
    pub fn set_aggregate_error_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAggregateErrorPrototype as usize] = value.into();
    }

    pub fn aggregate_error_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kAggregateErrorConstructor as usize].into()
    }
    pub fn set_aggregate_error_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kAggregateErrorConstructor as usize] = value.into();
    }

    pub fn promise_is_user_visible_symbol(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kPromiseIsUserVisibleSymbol as usize].into()
    }
    pub fn set_promise_is_user_visible_symbol(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kPromiseIsUserVisibleSymbol as usize] = value.into();
    }

    pub fn array_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayIteratorPrototype as usize].into()
    }
    pub fn set_array_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayIteratorPrototype as usize] = value.into();
    }

    pub fn regexp_string_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kRegexpStringIteratorPrototype as usize].into()
    }
    pub fn set_regexp_string_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kRegexpStringIteratorPrototype as usize] = value.into();
    }

    pub fn string_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kStringIteratorPrototype as usize].into()
    }
    pub fn set_string_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kStringIteratorPrototype as usize] = value.into();
    }

    pub fn map_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kMapIteratorPrototype as usize].into()
    }
    pub fn set_map_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kMapIteratorPrototype as usize] = value.into();
    }

    pub fn set_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kSetIteratorPrototype as usize].into()
    }
    pub fn set_set_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kSetIteratorPrototype as usize] = value.into();
    }

    pub fn array_grouping_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayGroupingPrototype as usize].into()
    }
    pub fn set_array_grouping_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayGroupingPrototype as usize] = value.into();
    }

    pub fn array_grouping_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayGroupingConstructor as usize].into()
    }
    pub fn set_array_grouping_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayGroupingConstructor as usize] = value.into();
    }

    pub fn array_grouping_iterator_prototype(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayGroupingIteratorPrototype as usize].into()
    }
    pub fn set_array_grouping_iterator_prototype(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayGroupingIteratorPrototype as usize] = value.into();
    }

    pub fn array_grouping_iterator_constructor(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kArrayGroupingIteratorConstructor as usize].into()
    }
    pub fn set_array_grouping_iterator_constructor(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kArrayGroupingIteratorConstructor as usize] = value.into();
    }

    pub fn json_stringifier(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kJsonStringifier as usize].into()
    }
    pub fn set_json_stringifier(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kJsonStringifier as usize] = value.into();
    }

    pub fn reflect_apply(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectApply as usize].into()
    }
    pub fn set_reflect_apply(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReflectApply as usize] = value.into();
    }

    pub fn reflect_construct(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectConstruct as usize].into()
    }
    pub fn set_reflect_construct(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReflectConstruct as usize] = value.into();
    }

    pub fn reflect_get(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectGet as usize].into()
    }
    pub fn set_reflect_get(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReflectGet as usize] = value.into();
    }

    pub fn reflect_set(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectSet as usize].into()
    }
    pub fn set_reflect_set(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReflectSet as usize] = value.into();
    }

    pub fn reflect_delete_property(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectDeleteProperty as usize].into()
    }
    pub fn set_reflect_delete_property(&mut self, value: Tagged<Object>) {
        self.roots_table_.roots[RootIndex::kReflectDeleteProperty as usize] = value.into();
    }

    pub fn reflect_has(&self) -> Tagged<Object> {
        self.roots_table_.roots[RootIndex::kReflectHas as usize].into()
    }
    pub fn set_reflect_has(&mut self, value: Tagged<Object>) {
        self.roots
