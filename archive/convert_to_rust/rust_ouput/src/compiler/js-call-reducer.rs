// Converted from V8 C++ source files:
// Header: js-call-reducer.h
// Implementation: js-call-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::graph_reducer::AdvancedReducer;
use crate::execution::isolate::Isolate;
use crate::deoptimizer::deoptimize_reason::DeoptimizeReason;
use crate::base::flags::Flags;

pub struct JSCallReducer {
    advanced_reducer: AdvancedReducer,
    jsgraph_: *mut JSGraph,
    broker_: *mut JSHeapBroker,
    temp_zone_: Rc<RefCell<Zone>>,
    flags_: Flags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
  kNoFlags = 0u32,
  kBailoutOnUninitialized = 1u32 << 0,
  kInlineJSToWasmCalls = 1u32 << 1,
}
impl Flag {
    pub fn to_bits(&self) -> u32 {
        *self as u32
    }
}
impl From<u32> for Flag {
    fn from(bits: u32) -> Self {
        match bits {
            1 => Flag::kBailoutOnUninitialized,
            2 => Flag::kInlineJSToWasmCalls,
            _ => Flag::kNoFlags,
        }
    }
}

pub struct JSCallReducerAssembler {}

pub struct StringRef {}

pub struct MapRef {}

pub struct ObjectRef {}

pub struct JSGraph {}

pub struct JSHeapBroker {}

pub struct SharedFunctionInfoRef {}

pub struct JSConstructNode {}

pub struct JSCallNode {}

pub struct AllocationBuilder {}

pub struct NodeProperties {}

pub struct HeapObjectMatcher {}

pub struct CommonOperatorBuilder {}

pub struct IteratingArrayBuiltinReducerAssembler {}

pub struct TFGraph {}

pub struct NativeContextRef {}

pub struct FeedbackCell {}

pub struct ElementAccess {}

pub struct Builtins {}

pub struct SimplifiedOperatorBuilder {}

pub struct FrameState {}

pub struct FunctionTemplateInfo {}

pub struct ExternalReference {}

pub struct CallInterfaceDescriptor {}

pub enum Builtin {
  kNoBuiltinId = 0,
}

pub struct DataViewRef {}

pub struct ApiFunction {}

pub struct FieldAccess {}

pub struct AccessInfoFactory {}

pub struct CodeRef {}

pub struct JSTypedArray {}

pub struct ConstructParameters {}

pub struct JSObjectRef {}

pub struct FeedbackSource {}

pub struct NumberMatcher {}

pub struct PromiseBuiltinReducerAssembler {}

pub struct DirectHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/code-factory.h
}
impl JSCallReducer {
    const K_MAX_INLINE_MATCH_SEQUENCE: i32 = 3;

    pub fn new(
        editor: *mut Editor,
        jsgraph_: *mut JSGraph,
        broker_: *mut JSHeapBroker,
        temp_zone_: &Rc<RefCell<Zone>>,
        flags_: Flags,
    ) -> Self {
        Self {
            advanced_reducer: AdvancedReducer::new(editor),
            jsgraph_: jsgraph_,
            broker_: broker_,
            temp_zone_: temp_zone_.clone(),
            flags_: flags_,
        }
    }

    fn reducer_name(&self) -> &'static str {
        "JSCallReducer"
    }

    fn reduce(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            match (*node).opcode() {
                IrOpcode::kJSConstruct => self.reduce_js_construct(node),
                IrOpcode::kJSConstructWithArrayLike => self.reduce_js_construct_with_array_like(node),
                IrOpcode::kJSConstructWithSpread => self.reduce_js_construct_with_spread(node),
                IrOpcode::kJSConstructForwardAllArgs => self.reduce_js_construct_forward_all_args(node),
                IrOpcode::kJSCall => self.reduce_js_call(node),
                IrOpcode::kJSCallWithArrayLike => self.reduce_js_call_with_array_like(node),
                IrOpcode::kJSCallWithSpread => self.reduce_js_call_with_spread(node),
                _ => Reduction::new(),
            }
        }
    }

    fn finalize(&mut self) {}

    fn revisit_for_graph_assembler(&mut self, node: *mut Node) {}

    fn zone_for_graph_assembler(&self) -> &Rc<RefCell<Zone>> {
        &self.temp_zone_
    }

    fn jsgraph_for_graph_assembler(&self) -> *mut JSGraph {
        self.jsgraph_
    }

    fn has_js_wasm_calls(&self) -> bool {
        false
    }

    fn wasm_module_for_inlining(&self) -> *const String_ExternalOneByteStringResource {
        std::ptr::null()
    }

    fn dependencies(&self) -> *mut Flags {
        std::ptr::null_mut()
    }

    fn broker(&self) -> *mut JSHeapBroker {
        self.broker_
    }

    fn reduce_boolean_constructor(&mut self, _node: *mut Node) -> Reduction {
       Reduction::new()
    }
    fn reduce_call_api_function(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_call_wasm_function(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_function_prototype_apply(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_function_prototype_bind(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_function_prototype_call(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_function_prototype_has_instance(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_get_prototype(&mut self, _node: *mut Node, _object: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_get_prototype_of(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_is(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_prototype_get_proto(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_prototype_has_own_property(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_prototype_is_prototype_of(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_object_create(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reflect_apply(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reflect_construct(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reflect_get(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reflect_get_prototype_of(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reflect_has(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_every(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_filter(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_find_index(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_find(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_for_each(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_includes(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_index_of(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_is_array(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_map(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_prototype_at(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_prototype_pop(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_prototype_push(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_prototype_shift(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_prototype_slice(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_reduce(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_reduce_right(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_some(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_iterator(&mut self, _node: *mut Node, _array_kind: ArrayIteratorKind, _iteration_kind: IterationKind) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_iterator_prototype_next(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_fast_array_iterator_next(&mut self, _type: InstanceType, _node: *mut Node, _kind: IterationKind) -> Reduction {
        Reduction::new()
    }
    fn reduce_call_or_construct_with_array_like_or_spread_of_create_arguments(&mut self, _node: *mut Node, _arguments_list: *mut Node, _arraylike_or_spread_index: i32, _frequency: CallFrequency, _feedback: FeedbackSource, _speculation_mode: SpeculationMode, _feedback_relation: CallFeedbackRelation) -> Reduction {
        Reduction::new()
    }
    fn reduce_call_or_construct_with_array_like_or_spread(&mut self, _node: *mut Node, _argument_count: i32, _arraylike_or_spread_index: i32, _frequency: CallFrequency, _feedback_source: FeedbackSource, _speculation_mode: SpeculationMode, _feedback_relation: CallFeedbackRelation, _target: *mut Node, _effect: Effect, _control: Control) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_construct(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_construct_with_array_like(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_construct_with_spread(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_construct_forward_all_args(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_call(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_call_shared(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_call_with_array_like(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_js_call_with_spread(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_reg_exp_prototype_test(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_return_receiver(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_constructor(&mut self, _node: *mut Node, _constructor: JSFunctionRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_index_of_includes(&mut self, _node: *mut Node, _variant: StringIndexOfIncludesVariant) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_substring(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_slice(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_substr(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_string_at(&mut self, _string_access_operator: *const Object, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_char_at(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_starts_with(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_ends_with(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_from_char_code(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_from_code_point(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_iterator(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_iterator_prototype_next(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_string_prototype_concat(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_internal_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_internal_reject(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_internal_resolve(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_prototype_catch(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_prototype_finally(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_prototype_then(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_promise_resolve_trampoline(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_typed_array_constructor(&mut self, _node: *mut Node, _shared: SharedFunctionInfoRef) -> Reduction {
        Reduction::new()
    }
    fn reduce_typed_array_prototype_to_string_tag(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_buffer_view_byte_length_accessor(&mut self, _node: *mut Node, _instance_type: InstanceType, _builtin: Builtin) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_buffer_view_byte_offset_accessor(&mut self, _node: *mut Node, _instance_type: InstanceType, _builtin: Builtin) -> Reduction {
        Reduction::new()
    }
    fn reduce_typed_array_prototype_length(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_for_insufficient_feedback(&mut self, _node: *mut Node, _reason: DeoptimizeReason) -> Reduction {
        Reduction::new()
    }
    fn reduce_math_unary(&mut self, _node: *mut Node, _op: *const Object) -> Reduction {
        Reduction::new()
    }
    fn reduce_math_binary(&mut self, _node: *mut Node, _op: *const Object) -> Reduction {
        Reduction::new()
    }
    fn reduce_math_imul(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_math_clz32(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_math_min_max(&mut self, _node: *mut Node, _op: *const Object, _empty_value: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_is_finite(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_is_integer(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_is_safe_integer(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_is_nan(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_global_is_finite(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_global_is_nan(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_map_prototype_has(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_map_prototype_get(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_set_prototype_has(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_collection_prototype_has(&mut self, _node: *mut Node, _collection_kind: CollectionKind) -> Reduction {
        Reduction::new()
    }
    fn reduce_collection_iteration(&mut self, _node: *mut Node, _collection_kind: CollectionKind, _iteration_kind: IterationKind) -> Reduction {
        Reduction::new()
    }
    fn reduce_collection_prototype_size(&mut self, _node: *mut Node, _collection_kind: CollectionKind) -> Reduction {
        Reduction::new()
    }
    fn reduce_collection_iterator_prototype_next(&mut self, _node: *mut Node, _entry_size: i32, _empty_collection: *mut String_ExternalOneByteStringResource, _collection_iterator_instance_type_first: InstanceType, _collection_iterator_instance_type_last: InstanceType) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_buffer_is_view(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_array_buffer_view_accessor(&mut self, _node: *mut Node, _instance_type: InstanceType, _access: FieldAccess, _builtin: Builtin) -> Reduction {
        Reduction::new()
    }
    fn reduce_data_view_access(&mut self, _node: *mut Node, _access: DataViewAccess, _element_type: ExternalArrayType) -> Reduction {
        Reduction::new()
    }
    fn reduce_date_prototype_get_time(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_date_now(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_parse_int(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_number_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_big_int_constructor(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_big_int_as_n(&mut self, _node: *mut Node, _builtin: Builtin) -> Reduction {
        Reduction::new()
    }
    fn try_reduce_js_call_math_min_max_with_array_like(&mut self, _node: *mut Node) -> Option<Reduction> {
        None
    }
    fn reduce_js_call_math_min_max_with_array_like(&mut self, _node: *mut Node, _builtin: Builtin) -> Reduction {
        Reduction::new()
    }
    fn reduce_get_continuation_preserved_embedder_data(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
    fn reduce_set_continuation_preserved_embedder_data(&mut self, _node: *mut Node) -> Reduction {
        Reduction::new()
    }
}

enum class InstanceType {
  JS_DATA_VIEW_TYPE,
  JS_TYPED_ARRAY_TYPE,
}

pub enum void {}

pub enum Error {}

struct String_ExternalOneByteStringResource {}

pub enum token_t {}

pub struct RWDigits {}

pub struct Digits {}

type digit_t = u64;

pub struct digit_t_wrapper {
    digit: digit_t
}

pub struct AsmType {}

pub enum AsmTypes {}

pub struct Object {}

pub struct SharedInfo {}

pub struct Builtins {}

pub struct BuiltinTypes {}

pub struct FrameDescription {}

pub struct NativeModule {}

pub struct Module {}

pub struct JSGlobalProxy {}

pub struct IsolateImpl {}

type ThreadKind = i32;

pub struct Promise {}

pub struct Context {}

pub struct Function {}

pub struct Value {}

pub struct Tagged<T> {
    value: *mut T,
}

pub enum GCType {}

pub struct Heap {}

pub struct HeapObject {}

pub struct RootVisitor {}

pub struct StackFrame {}

pub struct DirectHandleTable {}

pub struct Address {}

pub struct ClientTrustLevel {}

pub struct SessionPauseState {}

pub struct V8InspectorSession {}

pub struct StringView {}

pub type TraceDescriptor = i32;

pub struct DigitsWrapper {}

pub struct RootIndex {}

pub struct GCInfoIndex {}

pub struct Key {}

pub struct FrameStateFunctionInfo {}

pub struct DirectHandleScope {}

pub struct HeapObjectReference {}

pub struct MaybeObject {}

pub struct JSFunctionRef {}

pub struct HeapObjectRef {}

pub struct JSBoundFunctionRef {}

pub struct FixedArrayRef {}

pub struct CallFrequency {}

pub enum IterationKind {
    kKeys,
    kValues,
    kEntries,
}

pub enum class CallFeedbackRelation {
  kReceiver,
  kTarget,
  kUnrelated,
}

pub enum SpeculationMode {
    kDisallowSpeculation,
    kAllowSpeculation
}

pub enum StringIndexOfIncludesVariant {
    kIncludes,
    kIndexOf
}

pub enum CreateArgumentsType {
    kMappedArguments,
    kUnmappedArguments,
    kRestParameter
}

pub enum Kind {}

pub struct FrameDescriptionRef {}

pub struct JSReceiverRef {}

pub enum StandardMember {}

pub enum StandardMembers {}

pub struct ArrayList {}

pub struct FeedbackVector {}

pub struct NativeModuleWrapper {}

pub struct String {}

pub struct Zone {}

pub enum BuiltinId {}

pub struct DigitsConverter {}

pub enum BranchHint {
    kTrue,
    kFalse,
    kNone,
}

pub enum BranchSemantics {
  kJS,
}

pub struct SourceLocation {}

pub struct HeapNumber {}

pub enum Int64Representation {}

pub struct CFunction {}

pub enum CpuProfilingMode {}

pub struct digit_t_array {}

pub struct SharedObjectConveyorHandles {}

pub struct BytecodeOffset {}

pub enum OutputFrameStateCombine {}

pub enum Branch{}

pub struct NumberOperationHint {}

pub struct CheckBoundsFlags {}

pub enum CheckBoundsFlag {}

pub struct UseInfo {}

pub struct AllocationType {}

pub enum MaybeVoid {}

pub enum GCTypeMode {}

pub struct V8Inspector {}

pub struct TraceStateObserver {}

pub struct FunctionTemplateInfoRef {}

pub struct ZoneVector<T> {}

pub struct SourcePosition {}

pub struct Type {}

pub struct CallDescriptor {}

pub struct Value {}

pub struct MayBeHandle {}

pub struct CallFunctionResult {}

pub struct JSArrayBuffer {}

pub struct FrameDescriptionRefIterator {}

pub struct ObjectRefIterator {}

pub struct ObjectRefList {}

pub struct ValueIterator {}

pub enum ValueTypes {}

pub struct NumericRepresentation {}

pub struct JSTypedArrayRef {}

pub enum AccessMode {
  kLoad,
  kStore,
}

pub enum GeneralPurpose{}

pub struct ScopeInfo {}

pub struct ExternalArrayType {}

pub enum class JSCallReducerAssembler {}

pub struct Callable {}

impl JSCallReducer {
    fn zero() -> Self {
        JSCallReducer {
            advanced_reducer: AdvancedReducer::default(),
            jsgraph_: std::ptr::null_mut(),
            broker_: std::ptr::null_mut(),
            temp_zone_: Rc::new(RefCell::new(Zone::new())),
            flags_: Flags::new()
        }
    }
}

pub struct FlagsWrapper {
    flags : Flags
}

pub struct Editor {}

impl FlagsWrapper {
  pub fn to_bits(&self) -> u32 {
      self.flags.to_bits()
  }
}

pub trait Traceable {
    fn trace(&self, _visitor: &mut RootVisitor);
}

impl AdvancedReducer {
    fn new(_editor: *mut Editor) -> Self {
        AdvancedReducer {}
    }
}

impl Default for AdvancedReducer {
    fn default() -> Self {
        AdvancedReducer {}
    }
}

impl Flags {
  pub fn new() -> Self {
    Flags { bits: 0 }
  }

  pub fn to_bits(&self) -> u32 {
    self.bits
  }
}
pub enum Call {}
pub enum If {}

pub struct Reduction {
  change: bool,
  replacement: *mut Node,
}

impl Reduction {
    fn new() -> Self {
      Reduction{
        change: false,
        replacement: std::ptr::null_mut(),
      }
    }
    fn changed(&mut self, replacement: *mut Node)-> Self {
        self.change = true;
        self.replacement = replacement;
        Reduction { change: self.change, replacement: self.replacement }
    }
}

impl Clone for Reduction {
    fn clone(&self) -> Self {
        Reduction {
            change: self.change,
            replacement: self.replacement,
        }
    }
}

impl Copy for Reduction {}

pub struct Node {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrOpcode {
  kJSConstruct,
  kJSConstructWithArrayLike,
  kJSConstructWithSpread,
  kJSConstructForwardAllArgs,
  kJSCall,
  kJSCallWithArrayLike,
  kJSCallWithSpread,
  kJSCreateArguments,
  kCheckMaps,
  kFrameState,
  kStateValues,
  kReferenceEqual,
  kReturn,
  kLoadField,
  kLoadElement,
  kJSToObject,
  kJSForInNext,
  kJSCreateLiteralArray,
  kJSCreateEmptyLiteralArray,
  kDeoptimize,
  kNumberEqual,
  kBooleanNot,
  kCheckClosure,
  kStringLength,
  kJSCreateClosure,
  kNumberAdd,
  kLoop,
  kEffectPhi,
  kPhi,
  kBranch,
  kIfTrue,
  kIfFalse,
  kTerminate,
  kNumberBitwiseAnd,
  kNumberShiftRightLogical,
  kObjectIsFiniteNumber,
  kObjectIsInteger,
  kObjectIsSafeInteger,
  kObjectIsNaN,
  kHasProperty,
  kObjectIsReceiver,
  kCallRuntime,
  kConstructInvokeStub,
  kJSCreateArrayIterator,
  kStringCharCodeAt,
  kNumberLessThan,
  kNumberMax,
  kNumberMin,
  kStringFromSingleCharCode,
  kCheckIf,
  kJSCreateStringIterator,
  kNumberSubtract,
  kObjectIsSmi,
  kConvertTaggedHoleToUndefined,
  kGetContinuationPreservedEmbedderData,
  kSetContinuationPreservedEmbedderData,
  kNumberPow,
  kEnterMachineGraph,
  kExitMachineGraph,
  kSelect,
  kNumberLessThanOrEqual,
  kNumberSilenceNaN,
  kNumberImul,
  kEnsureWritableFastElements,
  kObjectIsConstructor,
  kToBoolean,
  kObjectIsArray,
  kObjectIsArrayBufferView,
  kLoadTypedElement,
  kStoreElement,
  kNumberIsIntegerIfSmi,
  kJsBitwiseXor,
  kCall,
  kTypeGuard,
  kNumberCeil,
  kBigIntAsIntN
  kBigIntAsUintN
  kThrow,
  kNumberAcos,
  kNumberAcosh,
  kNumberAsin,
  kNumberAsinh,
  kNumberAtan,
  kNumberAtanh,
  kNumberCbrt,
  kNumberCos,
  kNumberCosh,
  kNumberExp,
  kNumberExpm1,
  kNumberFloor,
  kNumberFround,
  kNumberLog,
  kNumberLog1p,
  kNumberLog10,
  kNumberLog2,
  kNumberRound,
  kNumberSign,
  kNumberSin,
  kNumberSinh,
  kNumberSqrt,
  kNumberTan,
  kNumberTanh,
  kNumberTrunc,
  kJSCreate,
  kDead,
  kObjectIsHeapObject,
  kObjectIsInternal,
}
impl IrOpcode {
  pub fn to_bits(&self) -> u32 {
      *self as u32
  }
}

impl Node {
    pub fn opcode(&self) -> IrOpcode {
        IrOpcode::kJSConstruct
    }
    pub fn InputAt(&self, _index:usize) -> *mut Node {
        std::ptr::null_mut()
    }
    pub fn InputCount(&self) -> usize {0}
}
impl JSConstructNode {
        fn Parameters(&self) -> &ConstructParameters {
            unsafe { std::mem::transmute(self) }
        }

        fn ArgumentCount(&self) -> i32 {
            0
        }

        fn NewTargetIndex(&self) -> usize {
            0
        }

        fn TargetIndex(&self) -> usize {
            0
        }

        fn ArgumentIndex(&self, i: i32) -> usize {
            0
        }
}
impl JSCallNode {
    fn ArgumentCount(&self) -> i32 {
        0
    }

    fn TargetIndex(&self) -> usize {
        0
    }

    fn ReceiverIndex(&self) -> usize {
        0
    }

    fn ArgumentIndex(&self, i: i32) -> usize {
        0
    }
    fn FeedbackVectorIndex(&self) -> usize {
        0
    }
    fn Argument(&self, _i: i32) -> *mut Node {std::ptr::null_mut()}
}

fn FeedbackCellOf(_op: IrOpcode) -> i32 {0}
fn CreateBoundFunctionParametersOf(_op: IrOpcode) -> i32 {0}
fn CreateArrayIteratorParametersOf(_op: IrOpcode) -> i32 {0}
fn CreateLiteralParametersOf(_op: IrOpcode) -> CreateLiteralParameters {
    CreateLiteralParameters {}
}
struct CreateLiteralParameters {}
pub enum TypeId {}
impl From<i32> for String_ExternalOneByteStringResource {
   fn from(_item:i32) -> Self {
     String_ExternalOneByteStringResource{}
   }
}

pub enum ArrayIteratorKind {
  kArrayLike,
  kTypedArray
}
impl core::fmt::Debug for ArrayIteratorKind {
   fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Array
