// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/heap-visitor.h

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// use std::option::Option;

// mod base; // Assuming base/logging.h functionalities are moved to base module
// mod execution; // Assuming execution/local-isolate.h functionalities are moved to execution module
// mod objects; // Assuming objects/*.h functionalities are moved to objects module

// use base::logging::*;
// use execution::local_isolate::LocalIsolate;
// use objects::bytecode_array::BytecodeArray;
// use objects::contexts::Contexts;
// use objects::fixed_array::FixedArray;
// use objects::js_weak_refs::JsWeakRefs;
// use objects::map::Map;
// use objects::objects::*; // Assuming objects/objects.h functionalities are moved to objects module
// use objects::shared_function_info::SharedFunctionInfo;
// use objects::string::String;
// use objects::visitors::ObjectVisitor;

// Placeholder modules
mod base {
    pub mod logging {
        macro_rules! DCHECK_GT {
            ($left:expr, $right:expr) => {
                if !($left > $right) {
                    panic!("DCHECK_GT failed: {} > {}", $left, $right);
                }
            };
        }

        macro_rules! DCHECK {
            ($condition:expr) => {
                if !($condition) {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
        }
    }
}

mod execution {
    pub mod local_isolate {
        pub struct LocalIsolate {}
    }
}

mod objects {
    pub mod bytecode_array {
        pub struct BytecodeArray {}
    }
    pub mod contexts {
        pub struct Contexts {}
    }
    pub mod fixed_array {
        pub struct FixedArray {}
    }
    pub mod js_weak_refs {
        pub struct JsWeakRefs {}
    }
    pub mod map {
        pub struct Map {}
    }
    pub mod objects {
        use crate::execution::local_isolate::LocalIsolate;
        use crate::objects::map::Map;
        pub struct HeapObject {}
        pub struct Object {}
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
        pub struct WasmArray {}
        pub struct WasmContinuationObject {}
        pub struct WasmFuncRef {}
        pub struct WasmMemoryMapDescriptor {}
        pub struct WasmNull {}
        pub struct WasmResumeData {}
        pub struct WasmStruct {}
        pub struct WasmSuspenderObject {}
        pub struct WasmTypeInfo {}

        pub struct Filler {}
        pub struct FreeSpace {}

        pub struct ByteArray {}
        pub struct DescriptorArray {}
        pub struct FeedbackVector {}
        pub struct FixedDoubleArray {}
        pub struct ScopeInfo {}
        pub struct ShortcutCandidate {}
        pub struct WeakArrayList {}
        pub struct WeakFixedArray {}

        pub trait BodyDescriptor {}

        impl HeapObject {
          pub fn map(&self) -> Tagged<Map> {
            //Placeholder implementation
            Tagged::<Map>{}
          }
        }

        // Placeholder Tagged<T>
        #[derive(Clone, Copy)]
        pub struct Tagged<T> {
        }
        impl<T> Tagged<T> {
          pub fn unchecked_cast<U>(self) -> Tagged<U> {
            // Placeholder implementation
            Tagged::<U>{}
          }
        }

        pub struct Heap {

        }

        impl Heap {
            pub fn new() -> Self {
                Heap{}
            }
        }
    }
    pub mod shared_function_info {
        pub struct SharedFunctionInfo {}
    }
    pub mod string {
        pub struct String {}
    }
    pub mod visitors {
        pub struct ObjectVisitorWithCageBases {}
        pub trait ObjectVisitor {
          fn visit_instruction_stream_pointer(&mut self, code: crate::objects::objects::Tagged<crate::objects::objects::Code>, slot: InstructionStreamSlot);
          fn visit_code_target(&mut self, host: crate::objects::objects::Tagged<InstructionStream>, reloc_info: *mut RelocInfo);
          fn visit_embedded_pointer(&mut self, host: crate::objects::objects::Tagged<InstructionStream>, reloc_info: *mut RelocInfo);
          fn visit_map_pointer(&mut self, host: crate::objects::objects::Tagged<crate::objects::objects::HeapObject>);
        }

        pub struct InstructionStreamSlot {}
        pub struct InstructionStream {}
        pub struct RelocInfo {}
    }
}

use base::logging::*;
use execution::local_isolate::LocalIsolate;
use objects::objects::*;
use objects::visitors::ObjectVisitorWithCageBases;

// Assuming WASM is always enabled
macro_rules! IF_WASM {
    ($($tokens:tt)*) => {
        $($tokens)*
    };
}

macro_rules! SIMPLE_HEAP_OBJECT_LIST1 {
  ($V:ident) => {};
}
macro_rules! CONCRETE_TRUSTED_OBJECT_TYPE_LIST1 {
  ($V:ident) => {};
}
macro_rules! TORQUE_VISITOR_ID_LIST {
  ($V:ident) => {};
}
macro_rules! TRUSTED_VISITOR_ID_LIST {
  ($V:ident) => {};
}

#[derive(Clone, Copy)]
struct MaybeObjectSize {
    raw_size_: usize,
}

impl MaybeObjectSize {
    fn new(size: usize) -> Self {
        DCHECK_GT!(size, 0);
        MaybeObjectSize { raw_size_: size }
    }

    fn none() -> Self {
        MaybeObjectSize { raw_size_: 0 }
    }

    fn assume_size(&self) -> usize {
        DCHECK_GT!(self.raw_size_, 0);
        self.raw_size_
    }

    fn is_none(&self) -> bool {
        self.raw_size_ == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VisitorId {
    AccessorInfo,
    AllocationSite,
    BigInt,
    BytecodeWrapper,
    CallSiteInfo,
    Cell,
    CodeWrapper,
    ConsString,
    ContextSidePropertyCell,
    CoverageInfo,
    DataHandler,
    DebugInfo,
    EmbedderDataArray,
    EphemeronHashTable,
    ExternalString,
    FeedbackCell,
    FeedbackMetadata,
    Foreign,
    FunctionTemplateInfo,
    HeapNumber,
    Hole,
    Map,
    NativeContext,
    Oddball,
    PreparseData,
    PropertyArray,
    PropertyCell,
    PrototypeInfo,
    RegExpBoilerplateDescription,
    RegExpDataWrapper,
    SeqOneByteString,
    SeqTwoByteString,
    SharedFunctionInfo,
    SlicedString,
    SloppyArgumentsElements,
    SmallOrderedHashMap,
    SmallOrderedHashSet,
    SmallOrderedNameDictionary,
    SourceTextModule,
    SwissNameDictionary,
    Symbol,
    SyntheticModule,
    ThinString,
    TransitionArray,
    WeakCell,
    WasmArray,
    WasmContinuationObject,
    WasmFuncRef,
    WasmMemoryMapDescriptor,
    WasmNull,
    WasmResumeData,
    WasmStruct,
    WasmSuspenderObject,
    WasmTypeInfo,
    JSArrayBuffer,
    JSDataViewOrRabGsabDataView,
    JSDate,
    JSExternalObject,
    JSFinalizationRegistry,
    JSFunction,
    JSObject,
    JSRegExp,
    JSSynchronizationPrimitive,
    JSTypedArray,
    JSWeakCollection,
    JSWeakRef,
    WasmGlobalObject,
    WasmInstanceObject,
    WasmMemoryObject,
    WasmSuspendingObject,
    WasmTableObject,
    WasmTagObject,
    Filler,
    FreeSpace,
    ByteArray,
    DescriptorArray,
    FeedbackVector,
    FixedArray,
    FixedDoubleArray,
    ScopeInfo,
    ShortcutCandidate,
    WeakArrayList,
    WeakFixedArray
}

struct HeapVisitor<ConcreteVisitor> {
    heap_: *const Heap,
    _phantom: std::marker::PhantomData<ConcreteVisitor>,
}

impl<ConcreteVisitor> HeapVisitor<ConcreteVisitor> {
    fn new_with_local_isolate(_isolate: *mut LocalIsolate) -> Self {
      let heap = Heap::new();
      HeapVisitor{heap_: Box::leak(Box::new(heap)), _phantom: std::marker::PhantomData}
    }

    fn new_with_isolate(_isolate: *mut Isolate) -> Self {
      let heap = Heap::new();
      HeapVisitor{heap_: Box::leak(Box::new(heap)), _phantom: std::marker::PhantomData}
    }

    fn new_with_heap(heap: *const Heap) -> Self {
      HeapVisitor{heap_: heap, _phantom: std::marker::PhantomData}
    }

    fn concrete_visitor(&mut self) -> &mut ConcreteVisitor
    where
        ConcreteVisitor: HeapVisitorTrait,
    {
        unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) }
    }

    fn concrete_visitor_const(&self) -> &ConcreteVisitor
    where
        ConcreteVisitor: HeapVisitorTrait,
    {
        unsafe { &*(self as *const Self as *const ConcreteVisitor) }
    }
}

trait HeapVisitorTrait: ObjectVisitor + Sized {
    fn use_precomputed_object_size() -> bool {
        false
    }

    fn should_visit_map_pointer() -> bool {
        true
    }

    fn should_visit_read_only_map_pointer() -> bool {
        true
    }

    fn can_encounter_filler_or_free_space() -> bool {
        true
    }

    fn should_use_unchecked_cast() -> bool {
        false
    }

    fn enable_concurrent_visitation() -> bool {
        false
    }

    fn should_visit_full_js_object() -> bool {
        false
    }
}

impl<ConcreteVisitor: HeapVisitorTrait> HeapVisitor<ConcreteVisitor> {
  fn visit_heap_object(&mut self, object: Tagged<HeapObject>) -> usize
    where ConcreteVisitor: HeapVisitorTrait
  {
      if ConcreteVisitor::use_precomputed_object_size() {
        panic!("UsePrecomputedObjectSize is true, but the wrong visit function was called");
      }
      let map = object.map();
      self.visit_with_map(map, object, MaybeObjectSize::none())
  }

  fn visit_map_heap_object(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>) -> usize
    where ConcreteVisitor: HeapVisitorTrait
  {
      if ConcreteVisitor::use_precomputed_object_size() {
        panic!("UsePrecomputedObjectSize is true, but the wrong visit function was called");
      }
      self.visit_with_map(map, object, MaybeObjectSize::none())
  }

  fn visit_map_heap_object_size(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, object_size: i32) -> usize
    where ConcreteVisitor: HeapVisitorTrait
  {
    if !ConcreteVisitor::use_precomputed_object_size() {
      panic!("UsePrecomputedObjectSize is false, but the wrong visit function was called");
    }
    self.visit_with_map(map, object, MaybeObjectSize::new(object_size as usize))
  }

  fn visit_with_map(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, maybe_object_size: MaybeObjectSize) -> usize
  where ConcreteVisitor: HeapVisitorTrait
  {
    let visitor_id = VisitorId::JSObject; // Placeholder, need to get from map
    match visitor_id {
      VisitorId::AccessorInfo => self.visit_accessor_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::AllocationSite => self.visit_allocation_site(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::BigInt => self.visit_big_int(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::BytecodeWrapper => self.visit_bytecode_wrapper(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::CallSiteInfo => self.visit_call_site_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Cell => self.visit_cell(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::CodeWrapper => self.visit_code_wrapper(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ConsString => self.visit_cons_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ContextSidePropertyCell => self.visit_context_side_property_cell(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::CoverageInfo => self.visit_coverage_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::DataHandler => self.visit_data_handler(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::DebugInfo => self.visit_debug_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::EmbedderDataArray => self.visit_embedder_data_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::EphemeronHashTable => self.visit_ephemeron_hash_table(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ExternalString => self.visit_external_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FeedbackCell => self.visit_feedback_cell(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FeedbackMetadata => self.visit_feedback_metadata(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Foreign => self.visit_foreign(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FunctionTemplateInfo => self.visit_function_template_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::HeapNumber => self.visit_heap_number(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Hole => self.visit_hole(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Map => self.visit_map(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::NativeContext => self.visit_native_context(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Oddball => self.visit_oddball(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::PreparseData => self.visit_preparse_data(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::PropertyArray => self.visit_property_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::PropertyCell => self.visit_property_cell(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::PrototypeInfo => self.visit_prototype_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::RegExpBoilerplateDescription => self.visit_reg_exp_boilerplate_description(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::RegExpDataWrapper => self.visit_reg_exp_data_wrapper(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SeqOneByteString => self.visit_seq_one_byte_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SeqTwoByteString => self.visit_seq_two_byte_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SharedFunctionInfo => self.visit_shared_function_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SlicedString => self.visit_sliced_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SloppyArgumentsElements => self.visit_sloppy_arguments_elements(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SmallOrderedHashMap => self.visit_small_ordered_hash_map(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SmallOrderedHashSet => self.visit_small_ordered_hash_set(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SmallOrderedNameDictionary => self.visit_small_ordered_name_dictionary(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SourceTextModule => self.visit_source_text_module(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SwissNameDictionary => self.visit_swiss_name_dictionary(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Symbol => self.visit_symbol(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::SyntheticModule => self.visit_synthetic_module(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ThinString => self.visit_thin_string(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::TransitionArray => self.visit_transition_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WeakCell => self.visit_weak_cell(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmArray => self.visit_wasm_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmContinuationObject => self.visit_wasm_continuation_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmFuncRef => self.visit_wasm_func_ref(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmMemoryMapDescriptor => self.visit_wasm_memory_map_descriptor(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmNull => self.visit_wasm_null(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmResumeData => self.visit_wasm_resume_data(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmStruct => self.visit_wasm_struct(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmSuspenderObject => self.visit_wasm_suspender_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmTypeInfo => self.visit_wasm_type_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSArrayBuffer => self.visit_js_array_buffer(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSDataViewOrRabGsabDataView => self.visit_js_data_view_or_rab_gsab_data_view(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSDate => self.visit_js_date(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSExternalObject => self.visit_js_external_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSFinalizationRegistry => self.visit_js_finalization_registry(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSFunction => self.visit_js_function(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSObject => {
        self.visit_js_object(map, object.unchecked_cast(), maybe_object_size)
      }
      VisitorId::JSRegExp => self.visit_js_reg_exp(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSSynchronizationPrimitive => self.visit_js_synchronization_primitive(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSTypedArray => self.visit_js_typed_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSWeakCollection => self.visit_js_weak_collection(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::JSWeakRef => self.visit_js_weak_ref(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmGlobalObject => self.visit_wasm_global_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmInstanceObject => self.visit_wasm_instance_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmMemoryObject => self.visit_wasm_memory_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmSuspendingObject => self.visit_wasm_suspending_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmTableObject => self.visit_wasm_table_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WasmTagObject => self.visit_wasm_tag_object(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::Filler => self.visit_filler(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FreeSpace => self.visit_free_space(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ByteArray => self.visit_byte_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::DescriptorArray => self.visit_descriptor_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FeedbackVector => self.visit_feedback_vector(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FixedArray => self.visit_fixed_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::FixedDoubleArray => self.visit_fixed_double_array(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ScopeInfo => self.visit_scope_info(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::ShortcutCandidate => self.visit_shortcut_candidate(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WeakArrayList => self.visit_weak_array_list(map, object.unchecked_cast(), maybe_object_size),
      VisitorId::WeakFixedArray => self.visit_weak_fixed_array(map, object.unchecked_cast(), maybe_object_size),
    }
  }

  fn visit_map_pointer_if_needed(&mut self, host: Tagged<HeapObject>)
    where ConcreteVisitor: HeapVisitorTrait
  {
      if ConcreteVisitor::should_visit_map_pointer() {
          // self.VisitMapPointer(host);  //TODO: Implement this line
          todo!()
      }
  }

  fn visit_accessor_info(&mut self, map: Tagged<Map>, object: Tagged<AccessorInfo>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::AccessorInfo}, AccessorInfo>(map, object, maybe_object_size)
  }
  fn visit_allocation_site(&mut self, map: Tagged<Map>, object: Tagged<AllocationSite>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::AllocationSite}, AllocationSite>(map, object, maybe_object_size)
  }
  fn visit_big_int(&mut self, map: Tagged<Map>, object: Tagged<BigInt>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::BigInt}, BigInt>(map, object, maybe_object_size)
  }
  fn visit_bytecode_wrapper(&mut self, map: Tagged<Map>, object: Tagged<BytecodeWrapper>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::BytecodeWrapper}, BytecodeWrapper>(map, object, maybe_object_size)
  }
  fn visit_call_site_info(&mut self, map: Tagged<Map>, object: Tagged<CallSiteInfo>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::CallSiteInfo}, CallSiteInfo>(map, object, maybe_object_size)
  }
  fn visit_cell(&mut self, map: Tagged<Map>, object: Tagged<Cell>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::Cell}, Cell>(map, object, maybe_object_size)
  }
  fn visit_code_wrapper(&mut self, map: Tagged<Map>, object: Tagged<CodeWrapper>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::CodeWrapper}, CodeWrapper>(map, object, maybe_object_size)
  }
  fn visit_cons_string(&mut self, map: Tagged<Map>, object: Tagged<ConsString>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::ConsString}, ConsString>(map, object, maybe_object_size)
  }
  fn visit_context_side_property_cell(&mut self, map: Tagged<Map>, object: Tagged<ContextSidePropertyCell>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::ContextSidePropertyCell}, ContextSidePropertyCell>(map, object, maybe_object_size)
  }
  fn visit_coverage_info(&mut self, map: Tagged<Map>, object: Tagged<CoverageInfo>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::CoverageInfo}, CoverageInfo>(map, object, maybe_object_size)
  }
  fn visit_data_handler(&mut self, map: Tagged<Map>, object: Tagged<DataHandler>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::DataHandler}, DataHandler>(map, object, maybe_object_size)
  }
  fn visit_debug_info(&mut self, map: Tagged<Map>, object: Tagged<DebugInfo>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::DebugInfo}, DebugInfo>(map, object, maybe_object_size)
  }
  fn visit_embedder_data_array(&mut self, map: Tagged<Map>, object: Tagged<EmbedderDataArray>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::EmbedderDataArray}, EmbedderDataArray>(map, object, maybe_object_size)
  }
  fn visit_ephemeron_hash_table(&mut self, map: Tagged<Map>, object: Tagged<EphemeronHashTable>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::EphemeronHashTable}, EphemeronHashTable>(map, object, maybe_object_size)
  }
  fn visit_external_string(&mut self, map: Tagged<Map>, object: Tagged<ExternalString>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::ExternalString}, ExternalString>(map, object, maybe_object_size)
  }
  fn visit_feedback_cell(&mut self, map: Tagged<Map>, object: Tagged<FeedbackCell>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::FeedbackCell}, FeedbackCell>(map, object, maybe_object_size)
  }
  fn visit_feedback_metadata(&mut self, map: Tagged<Map>, object: Tagged<FeedbackMetadata>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::FeedbackMetadata}, FeedbackMetadata>(map, object, maybe_object_size)
  }
  fn visit_foreign(&mut self, map: Tagged<Map>, object: Tagged<Foreign>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::Foreign}, Foreign>(map, object, maybe_object_size)
  }
  fn visit_function_template_info(&mut self, map: Tagged<Map>, object: Tagged<FunctionTemplateInfo>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::FunctionTemplateInfo}, FunctionTemplateInfo>(map, object, maybe_object_size)
  }
  fn visit_heap_number(&mut self, map: Tagged<Map>, object: Tagged<HeapNumber>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::HeapNumber}, HeapNumber>(map, object, maybe_object_size)
  }
  fn visit_hole(&mut self, map: Tagged<Map>, object: Tagged<Hole>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::Hole}, Hole>(map, object, maybe_object_size)
  }
  fn visit_map(&mut self, map: Tagged<Map>, object: Tagged<Map>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::Map}, Map>(map, object, maybe_object_size)
  }
  fn visit_native_context(&mut self, map: Tagged<Map>, object: Tagged<NativeContext>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::NativeContext}, NativeContext>(map, object, maybe_object_size)
  }
  fn visit_oddball(&mut self, map: Tagged<Map>, object: Tagged<Oddball>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::Oddball}, Oddball>(map, object, maybe_object_size)
  }
  fn visit_preparse_data(&mut self, map: Tagged<Map>, object: Tagged<PreparseData>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::PreparseData}, PreparseData>(map, object, maybe_object_size)
  }
  fn visit_property_array(&mut self, map: Tagged<Map>, object: Tagged<PropertyArray>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::PropertyArray}, PropertyArray>(map, object, maybe_object_size)
  }
  fn visit_property_cell(&mut self, map: Tagged<Map>, object: Tagged<PropertyCell>, maybe_object_size: MaybeObjectSize) -> usize {
    self.visit_with_body_descriptor::<{VisitorId::PropertyCell}, PropertyCell>(map, object, maybe_object_size)
  }
  fn visit_prototype_info(&mut self, map: Tagged<Map>, object: Tagged<PrototypeInfo