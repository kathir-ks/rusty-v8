// Converted from V8 C++ source files:
// Header: access-builder.h
// Implementation: access-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::mem;
use std::ops::Deref;
use std::ptr;

// use crate::v8::base::compiler_specific::*;
use crate::compiler::js_operator::*;
use crate::compiler::simplified_operator::*;
use crate::compiler::write_barrier_kind::*;
use crate::objects::elements_kind::*;
use crate::objects::js_objects::*;

use crate::objects::tagged_field::ConstFieldInfo;
use crate::v8::internal::HeapObject;
// use crate::handles::handles_inl::*;
use crate::objects::arguments::JSStrictArgumentsObject;
use crate::objects::arguments::JSSloppyArgumentsObject;
use crate::objects::contexts::Context;
use crate::objects::heap_number::HeapNumber;
use crate::objects::js_collection::JSCollection;
use crate::objects::js_generator::JSGeneratorObject;
use crate::objects::objects_inl::JSArray;
use crate::objects::source_text_module::SourceTextModule;
use crate::objects::tagged_field::*;
use crate::compiler::type_cache::TypeCache;
use crate::objects::cell::Cell;
use crate::objects::fixed_array::FixedArray;
use crate::objects::weak_fixed_array::WeakFixedArray;
use crate::objects::property_array::PropertyArray;
use crate::objects::descriptor_array::DescriptorArray;
use crate::objects::map::Map;
use crate::objects::name::Name;
use crate::objects::free_space::FreeSpace;
use crate::objects::string::String;
use crate::objects::string::ConsString;
use crate::objects::string::ThinString;
use crate::objects::string::SlicedString;
use crate::objects::external_string::ExternalString;
use crate::objects::seq_one_byte_string::SeqOneByteString;
use crate::objects::seq_two_byte_string::SeqTwoByteString;
use crate::objects::js_array_iterator::JSArrayIterator;
use crate::objects::js_string_iterator::JSStringIterator;
use crate::objects::js_primitive_wrapper::JSPrimitiveWrapper;
use crate::objects::js_regexp::JSRegExp;
use crate::objects::enum_cache::EnumCache;
use crate::objects::js_array_buffer::JSArrayBuffer;
use crate::objects::js_array_buffer_view::JSArrayBufferView;
use crate::objects::js_typed_array::JSTypedArray;
use crate::objects::js_data_view::JSDataView;
use crate::objects::js_date::JSDate;
use crate::objects::js_iterator_result::JSIteratorResult;
use crate::objects::oddball::Hole;
use crate::objects::bigint::BigInt;
use crate::objects::ordered_hash_table::*;
use crate::objects::name_dictionary::NameDictionary;
use crate::objects::feedback_cell::FeedbackCell;
use crate::objects::feedback_vector::FeedbackVector;
use crate::objects::wasm_array::WasmArray;
use crate::objects::wasm_dispatch_table::WasmDispatchTable;
use crate::objects::context_side_property_cell::ContextSidePropertyCell;
use crate::v8::platform::memory::kExternalStringResourceDataTag;
use crate::v8::platform::memory::kExternalObjectValueTag;
use crate::v8::platform::memory::kCodeIndirectPointerTag;
use crate::v8::platform::memory::kRegExpDataIndirectPointerTag;
use crate::v8::platform::memory::BaseTaggedness;
use crate::v8::platform::memory::kTaggedBase;
use crate::v8::platform::memory::kUntaggedBase;
use crate::compiler::turbofan_typer::Type;
use crate::compiler::optional_map_ref::OptionalMapRef;
use crate::handles::maybe_handle::MaybeHandle;
use crate::handles::handle::Handle;
use crate::compiler::linkage::MachineType;

pub struct AccessBuilder {}

impl AccessBuilder {
  pub fn ForExternalIntPtr() -> FieldAccess {
    FieldAccess {
      base_taggedness: kUntaggedBase,
      offset: 0,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::any(),
      machine_type: MachineType::IntPtr(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "ExternalIntPtr",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForMap(write_barrier: WriteBarrierKind) -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: HeapObject::kMapOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::MapInHeader(),
      write_barrier_kind: write_barrier,
      name_str: "Map",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForHeapNumberValue() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: mem::size_of::<usize>()*2,  // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kFloat64,
      machine_type: MachineType::Float64(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "HeapNumberValue",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForHeapInt32Value() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: mem::size_of::<usize>()*2 + 4, // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kInt32,
      machine_type: MachineType::Int32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "HeapInt32Value",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForHeapInt32UpperValue() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: mem::size_of::<usize>()*2 + 0, // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kInt32,
      machine_type: MachineType::Int32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "HeapInt32ValueUpperValue",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForHeapNumberOrOddballOrHoleValue() -> FieldAccess {
      FieldAccess {
          base_taggedness: kTaggedBase,
          offset: mem::size_of::<usize>()*2,  // This offset is wrong kathir
          name: MaybeHandle::empty(),
          map: OptionalMapRef::empty(),
          ty: TypeCache::get().kFloat64,
          machine_type: MachineType::Float64(),
          write_barrier_kind: kNoWriteBarrier,
          name_str: "HeapNumberValue",
          const_field_info: ConstFieldInfo::None(),
          is_immutable: false,
          indirect_pointer_tag: 0,
          is_bounded_size_access: false,
      }
  }

  pub fn ForBigIntBitfield() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: BigInt::kBitfieldOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kInt32,
      machine_type: MachineType::Uint32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "BigIntBitfield",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  #[cfg(BIGINT_NEEDS_PADDING)]
  pub fn ForBigIntOptionalPadding() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: BigInt::kPaddingOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kInt32,
      machine_type: MachineType::Uint32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "BigIntOptionalPadding",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForBigIntLeastSignificantDigit64() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: mem::size_of::<usize>() * 2, // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kBigUint64,
      machine_type: MachineType::Uint64(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "BigIntLeastSignificantDigit64",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSObjectPropertiesOrHash() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSObject::kPropertiesOrHashOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::any(),
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSObjectPropertiesOrHash",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSObjectPropertiesOrHashKnownPointer() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSObject::kPropertiesOrHashOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::any(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSObjectPropertiesOrHashKnownPointer",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSObjectElements() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSObject::kElementsOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSObjectElements",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSObjectInObjectProperty(
    map: MapRef,
    index: i32,
    machine_type: MachineType,
  ) -> FieldAccess {
    let offset = map.GetInObjectPropertyOffset(index);
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: offset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::NonInternal(),
      machine_type: machine_type,
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSObjectInObjectProperty",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSObjectOffset(offset: i32, write_barrier_kind: WriteBarrierKind) -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: offset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::NonInternal(),
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: write_barrier_kind,
      name_str: "JSObjectOffset",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSCollectionTable() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSCollection::kTableOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSCollectionTable",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSCollectionIteratorTable() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: 8, // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSCollectionIteratorTable",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSCollectionIteratorIndex() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: 0, // This offset is wrong kathir
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kFixedArrayLengthType,
      machine_type: MachineType::TaggedSigned(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSCollectionIteratorIndex",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSExternalObjectValue() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSExternalObject::kValueOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::ExternalPointer(),
      machine_type: MachineType::Pointer(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSExternalObjectValue",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: kExternalObjectValueTag,
      is_bounded_size_access: false,
    }
  }

  #[cfg(V8_ENABLE_SANDBOX)]
  pub fn ForJSExternalObjectPointerHandle() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSExternalObject::kValueOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kUint32,
      machine_type: MachineType::Uint32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSExternalObjectPointerHandle",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSFunctionPrototypeOrInitialMap() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kPrototypeOrInitialMapOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::any(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSFunctionPrototypeOrInitialMap",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSFunctionContext() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kContextOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSFunctionContext",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSFunctionSharedFunctionInfo() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kSharedFunctionInfoOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSFunctionSharedFunctionInfo",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSFunctionFeedbackCell() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kFeedbackCellOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSFunctionFeedbackCell",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  #[cfg(V8_ENABLE_LEAPTIERING)]
  pub fn ForJSFunctionDispatchHandleNoWriteBarrier() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kDispatchHandleOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kInt32,
      machine_type: MachineType::Int32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSFunctionDispatchHandle",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  #[cfg(not(V8_ENABLE_LEAPTIERING))]
  pub fn ForJSFunctionCode() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSFunction::kCodeOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSFunctionCode",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSBoundFunctionBoundTargetFunction() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSBoundFunction::kBoundTargetFunctionOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Callable(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSBoundFunctionBoundTargetFunction",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSBoundFunctionBoundThis() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSBoundFunction::kBoundThisOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::NonInternal(),
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSBoundFunctionBoundThis",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSBoundFunctionBoundArguments() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSBoundFunction::kBoundArgumentsOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSBoundFunctionBoundArguments",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectContext() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kContextOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSGeneratorObjectContext",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectFunction() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kFunctionOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::CallableFunction(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSGeneratorObjectFunction",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectReceiver() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kReceiverOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSGeneratorObjectReceiver",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectContinuation() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kContinuationOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::SignedSmall(),
      machine_type: MachineType::TaggedSigned(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSGeneratorObjectContinuation",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectInputOrDebugPos() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kInputOrDebugPosOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::NonInternal(),
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSGeneratorObjectInputOrDebugPos",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectParametersAndRegisters() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kParametersAndRegistersOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::Internal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSGeneratorObjectParametersAndRegisters",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSGeneratorObjectResumeMode() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSGeneratorObject::kResumeModeOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::SignedSmall(),
      machine_type: MachineType::TaggedSigned(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSGeneratorObjectResumeMode",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSAsyncFunctionObjectPromise() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: 0, // This offset is wrong kathir
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherObject(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSAsyncFunctionObjectPromise",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSAsyncGeneratorObjectQueue() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: 0,  // This offset is wrong kathir
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::NonInternal(),
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSAsyncGeneratorObjectQueue",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSAsyncGeneratorObjectIsAwaiting() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: 0,  // This offset is wrong kathir
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::SignedSmall(),
      machine_type: MachineType::TaggedSigned(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSAsyncGeneratorObjectIsAwaiting",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSArrayLength(elements_kind: ElementsKind) -> FieldAccess {
    let type_cache = TypeCache::get();
    let mut access = FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSArray::kLengthOffset,
      name: Handle::empty(),
      map: OptionalMapRef::empty(),
      ty: type_cache.kJSArrayLengthType,
      machine_type: MachineType::AnyTagged(),
      write_barrier_kind: kFullWriteBarrier,
      name_str: "JSArrayLength",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    };
    if IsDoubleElementsKind(elements_kind) {
      access.ty = type_cache.kFixedDoubleArrayLengthType;
      access.machine_type = MachineType::TaggedSigned();
      access.write_barrier_kind = kNoWriteBarrier;
    } else if IsFastElementsKind(elements_kind) {
      access.ty = type_cache.kFixedArrayLengthType;
      access.machine_type = MachineType::TaggedSigned();
      access.write_barrier_kind = kNoWriteBarrier;
    }
    access
  }

  pub fn ForJSArrayBufferBitField() -> FieldAccess {
    FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSArrayBuffer::kBitFieldOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kUint8,
      machine_type: MachineType::Uint32(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSArrayBufferBitField",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    }
  }

  pub fn ForJSArrayBufferByteLength() -> FieldAccess {
    let mut access = FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSArrayBuffer::kRawByteLengthOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kJSArrayBufferByteLengthType,
      machine_type: MachineType::UintPtr(),
      write_barrier_kind: kNoWriteBarrier,
      name_str: "JSArrayBufferByteLength",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    };
    #[cfg(V8_ENABLE_SANDBOX)]
    {
      access.is_bounded_size_access = true;
    }
    access
  }

  pub fn ForJSArrayBufferViewBuffer() -> FieldAccess {
    let mut access = FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSArrayBufferView::kBufferOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: Type::OtherInternal(),
      machine_type: MachineType::TaggedPointer(),
      write_barrier_kind: kPointerWriteBarrier,
      name_str: "JSArrayBufferViewBuffer",
      const_field_info: ConstFieldInfo::None(),
      is_immutable: false,
      indirect_pointer_tag: 0,
      is_bounded_size_access: false,
    };
    access.is_immutable = true;
    access
  }

  pub fn ForJSArrayBufferViewByteLength() -> FieldAccess {
    let mut access = FieldAccess {
      base_taggedness: kTaggedBase,
      offset: JSArrayBufferView::kRawByteLengthOffset,
      name: MaybeHandle::empty(),
      map: OptionalMapRef::empty(),
      ty: TypeCache::get().kJSArrayBufferViewByteLengthType,
      machine_type
