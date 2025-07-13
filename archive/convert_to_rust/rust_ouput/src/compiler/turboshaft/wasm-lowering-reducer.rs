// Converted from V8 C++ source files:
// Header: wasm-lowering-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::Arc;
use crate::v8::internal::compiler::turboshaft::csa_optimize_phase::*;

pub mod builtin_call_descriptors {
  pub struct BuiltinCallDescriptor {}
  impl BuiltinCallDescriptor {
      pub const WasmRefFunc: Self = Self {};
      pub const WasmInt32ToHeapNumber: Self = Self {};
      pub const WasmStringAsWtf16: Self = Self {};
  }
}
pub mod assembler {
    pub struct Assembler {}
    impl Assembler {
        pub fn new() -> Self {
            Self {}
        }
    }
}
pub mod index {
  pub struct OpIndex {}
  impl OpIndex {
    pub const Invalid: Self = Self {};
  }
}
pub mod operations {
  pub struct Operation {}
}
pub mod phase {
  pub struct Phase {}
}
pub mod wasm_assembler_helpers {
  pub struct WasmAssemblerHelpers {}
}
pub mod wasm {
    pub mod wasm_engine {
        pub struct WasmEngine {}
    }
    pub mod wasm_module {
        pub struct WasmModule {}
    }
    pub mod wasm_objects {
        pub struct WasmGlobal {}
        pub struct WasmStruct {}
        impl WasmStruct {
          pub fn Size(_struct_type: &StructType) -> i32 { 0 }
        }
        pub struct WasmArray {}
        impl WasmArray {
            pub fn MaxLength(_array_type: &ArrayType) -> u32 { 0 }
            pub const kLengthOffset: i32 = 0;
            pub const kHeaderSize: i32 = 0;
        }
        pub struct WasmFuncRef {}
    }
    pub mod wasm_subtyping {
        pub fn GetSubtypingDepth(_module_: &super::wasm_module::WasmModule, _type_index: u32) -> i32 { 0 }
    }
    pub enum ValueTypeKind {
      kI8,
      kI16,
      kI32,
      kI64,
      kF16,
      kF32,
      kF64,
      kS128,
      kRef,
      kRefNull,
      kVoid,
      kTop,
      kBottom
    }
    pub struct ValueType {
        kind_: ValueTypeKind,
        nullable_: bool,
        heap_representation_: HeapTypeRepresentation
    }

    impl ValueType {
      pub fn new(kind_: ValueTypeKind, nullable_: bool, heap_representation_: HeapTypeRepresentation) -> Self {
        Self { kind_, nullable_, heap_representation_ }
      }
      pub fn is_reference_to(&self, _heap_type: HeapType) -> bool { false }
      pub fn kind(&self) -> ValueTypeKind { self.kind_ }
      pub fn use_wasm_null(&self) -> bool { false }
      pub fn value_kind_size_log2(&self) -> i32 { 0 }
      pub fn is_reference(&self) -> bool { false }
      pub fn heap_representation(&self) -> HeapTypeRepresentation { self.heap_representation_ }
      pub fn is_nullable(&self) -> bool { self.nullable_ }
    }
    pub struct ModuleTypeIndex {
        index: usize
    }
    pub struct ArrayType {
        mutability_: bool,
        element_type_: ValueType
    }
    impl ArrayType {
        pub fn mutability(&self) -> bool { self.mutability_ }
        pub fn element_type(&self) -> ValueType { self.element_type_ }
    }
    pub struct StructType {
    }
    impl StructType {
        pub fn mutability(&self, _field_index: i32) -> bool { false }
        pub fn field(&self, _field_index: i32) -> ValueType { ValueType::new(ValueTypeKind::kI32, false, HeapTypeRepresentation::kNone) }
        pub fn field_offset(&self, _field_index: i32) -> i32 { 0 }
    }

    #[derive(PartialEq, Eq)]
    pub enum HeapTypeRepresentation {
        kNone,
        kI31,
        kEq,
        kArray,
        kStruct,
        kString,
        kExternString,
        kNoExtern,
        kNoFunc,
        kNoExn
    }

    pub enum HeapType {
        kAny,
        kExtern
    }

    impl HeapType {
      pub fn AsNonNull(&self) -> Self { *self }
    }
    pub const kWasmAnyRef: ValueType = ValueType { kind_: ValueTypeKind::kRef, nullable_: false, heap_representation_: HeapTypeRepresentation::kNone };
    pub const kWasmExternRef: ValueType = ValueType { kind_: ValueTypeKind::kRef, nullable_: false, heap_representation_: HeapTypeRepresentation::kNone };
    pub const kWasmI31Ref: ValueType = ValueType { kind_: ValueTypeKind::kRef, nullable_: false, heap_representation_: HeapTypeRepresentation::kI31 };
    pub const kWasmI32: ValueType = ValueType { kind_: ValueTypeKind::kI32, nullable_: false, heap_representation_: HeapTypeRepresentation::kNone };
    pub fn IsSubtypeOf(_a: ValueType, _b: ValueType, _module_: &WasmModule) -> bool { false }
    pub fn kWasmNull() -> RootIndex { RootIndex {} }

    pub mod wasm_shared {
        pub fn module_is_shared(_function_index: u32) -> bool { false }
    }
}

pub mod turboshaft {
  pub mod define_assembler_macros {
  }
  pub mod undef_assembler_macros {
  }
}

pub struct WasmLoweringReducerData {
    wasm_module: Arc<wasm::wasm_module::WasmModule>,
    wasm_shared: bool,
}

impl WasmLoweringReducerData {
    pub fn new(wasm_module: Arc<wasm::wasm_module::WasmModule>, wasm_shared: bool) -> Self {
        WasmLoweringReducerData { wasm_module, wasm_shared }
    }

    pub fn wasm_module(&self) -> &wasm::wasm_module::WasmModule {
        &self.wasm_module
    }

    pub fn wasm_shared(&self) -> bool {
        self.wasm_shared
    }
}

pub struct V<T> {
    value: i32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> V<T> {
    pub fn cast<U>(self) -> V<U> {
        V { value: self.value, _phantom: std::marker::PhantomData }
    }

    pub fn valid(&self) -> bool {
        true
    }
}

pub struct OptionalV<T> {
    value: Option<i32>,
    _phantom: std::marker::PhantomData<T>,
}

pub struct Label<T> {
    _phantom: std::marker::PhantomData<T>,
    assembler: *mut Assembler
}

impl<T> Label<T> {
    pub fn new(_assembler: *mut Assembler) -> Self {
        Label { _phantom: std::marker::PhantomData, assembler: _assembler }
    }
}

pub struct LoopLabel<T, U, V> {
    _phantom: std::marker::PhantomData<(T, U, V)>,
    assembler: *mut Assembler
}

impl<T, U, V> LoopLabel<T, U, V> {
    pub fn new(_assembler: *mut Assembler) -> Self {
        LoopLabel { _phantom: std::marker::PhantomData, assembler: _assembler }
    }
}

pub struct TrapId {}

impl TrapId {
    pub const kTrapNullDereference: Self = Self {};
    pub const kTrapIllegalCast: Self = Self {};
    pub const kTrapArrayTooLarge: Self = Self {};
}

pub struct GlobalMode {}

impl GlobalMode {
    pub const kLoad: Self = Self {};
    pub const kStore: Self = Self {};
}

pub struct RootsTable {}

impl RootsTable {
    pub fn IsReadOnly(_index: RootIndex) -> bool {
        false
    }
}

pub struct IsolateData {}

impl IsolateData {
    pub fn root_slot_offset(_index: RootIndex) -> i32 {
        0
    }
}

pub struct WasmTrustedInstanceData {}
pub struct Any {}
pub struct None {}
pub struct Object {}
pub struct Map {}
pub struct Word32 {}
pub struct FixedArray {}
pub struct FixedAddressArray {}
pub struct String {}
pub struct WordPtr {}
pub struct Float64 {}
pub struct HeapObject {}
pub struct Smi {}

pub struct WasmStructNullable {}
pub struct WasmArrayNullable {}

pub struct Uint32 {}

pub struct WasmFuncRef {}

pub struct Uninitialized<T> {}

pub struct TrapHandler {}

impl TrapHandler {
    pub fn IsTrapHandlerEnabled() -> bool {
        false
    }
}

pub struct AccessBuilder {}

impl AccessBuilder {
    pub fn ForMap(_kind: WriteBarrierKind) -> Self {
        Self {}
    }
    pub fn ForJSObjectPropertiesOrHash() -> Self {
        Self {}
    }
    pub fn ForWasmArrayLength() -> Self {
        Self {}
    }
    pub fn ForSlicedStringOffset() -> Self {
        Self {}
    }
    pub fn ForSlicedStringParent() -> Self {
        Self {}
    }
    pub fn ForThinStringActual() -> Self {
        Self {}
    }
    pub fn ForConsStringFirst() -> Self {
        Self {}
    }
    pub fn ForSeqOneByteStringCharacter() -> Self {
        Self {}
    }
    pub fn ForSeqTwoByteStringCharacter() -> Self {
        Self {}
    }
    pub fn ForExternalStringResourceData() -> Self {
        Self {}
    }
}

pub struct FieldAccess {
    pub offset: i32,
    pub external_pointer_tag: i32
}

pub struct WriteBarrierKind {}

impl WriteBarrierKind {
    pub const kFullWriteBarrier: Self = Self {};
    pub const kNoWriteBarrier: Self = Self {};
}

pub struct BuiltinCallDescriptor {}
impl BuiltinCallDescriptor {
    pub const WasmRefFunc: Self = Self {};
}

pub struct SmiValuesAre31BitsBool {}
impl SmiValuesAre31BitsBool {
    pub fn get() -> bool {
        false
    }
}

pub struct FloatType<T> {
  _phantom: std::marker::PhantomData<T>
}

pub struct Bits {}

pub struct AllocationType {}

impl AllocationType {
    pub const kYoung: Self = Self {};
}

pub struct LoadOp {}

impl LoadOp {
    pub struct Kind {}
    impl Kind {
      pub fn RawAligned() -> Self { Self {} }
      pub fn TaggedBase() -> Self { Self {} }
      pub fn TrapOnNull() -> Self { Self {} }

      pub fn Immutable(self) -> Self { self }
    }

}

pub struct StoreOp {}

impl StoreOp {
    pub struct Kind {}
    impl Kind {
      pub fn RawAligned() -> Self { Self {} }
      pub fn TaggedBase() -> Self { Self {} }
      pub fn TrapOnNull() -> Self { Self {} }

      pub fn Immutable(self) -> Self { self }
    }
}

pub struct RegisterRepresentation {}

pub struct Type {}

impl Type {
  pub fn is_nullable(&self) -> bool {
    false
  }
  pub fn ref_index(&self) -> u32 {
    0
  }
}

pub struct WasmTypeCheckConfig {
  pub from: Type,
  pub to: Type
}

pub enum CheckForNull {
    kWithNullCheck
}

pub struct FirstWasmObjectType {}
pub struct LastWasmObjectType {}

pub struct WasmTypeInfo {}

impl WasmTypeInfo {
  pub const kSupertypesLengthOffset: i32 = 0;
  pub const kSupertypesOffset: i32 = 0;
}

pub struct ExternalStringResourceData {}

impl AccessBuilder {
  pub fn ForExternalStringResourceData() -> FieldAccess {
    FieldAccess { offset: 0, external_pointer_tag: 0 }
  }
}

pub struct GraphAssembler {}

impl GraphAssembler {
  pub fn Word32Constant(_a: i32) -> Word32 {
    Word32 {}
  }
  pub fn SmiConstant(_a: i32) -> Smi {
    Smi {}
  }
}

pub struct kSeqStringTag {}
pub struct kStringRepresentationMask {}
pub struct kThinStringTag {}
pub struct kConsStringTag {}
pub struct kIsIndirectStringMask {}
pub struct kStringEncodingMask {}
pub struct kExternalStringTag {}
pub struct kUncachedExternalStringMask {}

pub const HEAP_NUMBER_TYPE: i32 = 0;
pub const WASM_ARRAY_TYPE: i32 = 0;
pub const WASM_STRUCT_TYPE: i32 = 0;
pub const FIRST_NONSTRING_TYPE: i32 = 0;
pub const kTaggedSizeLog2: i32 = 0;
pub const kTaggedSize: i32 = 0;
pub const kObjectAlignment: i32 = 0;
pub const kHeapObjectTag: i32 = 0;
pub const kMinusZeroBits: i64 = 0;
pub const kMinusZeroLoBits: i32 = 0;
pub const kMinusZeroHiBits: i32 = 0;
pub const kCharWidthBailoutSentinel: i32 = 0;
pub const kFullWriteBarrier: WriteBarrierKind = WriteBarrierKind {};
pub const kNoWriteBarrier: WriteBarrierKind = WriteBarrierKind {};
pub const V8_STATIC_ROOTS_BOOL: bool = false;

trait Reducer<Next> {
    fn turboshaft_reducer_boilerplate(&self) {}

    fn reduce_global_get(&self, instance: V<WasmTrustedInstanceData>, global: &wasm::wasm_objects::WasmGlobal) -> V<Any> {
        self.lower_global_set_or_get(instance, OpIndex::Invalid, global, GlobalMode::kLoad)
    }

    fn reduce_global_set(&self, instance: V<WasmTrustedInstanceData>, value: V<Any>, global: &wasm::wasm_objects::WasmGlobal) -> OpIndex {
        self.lower_global_set_or_get(instance, value, global, GlobalMode::kStore)
    }

    fn reduce_root_constant(&self, index: RootIndex) -> OpIndex {
        let roots = self.load_root_register();
        #[cfg(target_endian = "big")]
        {
            self.bitcast_word_ptr_to_tagged(self.load(
                roots,
                LoadOp::Kind::RawAligned().Immutable(),
                MemoryRepresentation::UintPtr(),
                IsolateData::root_slot_offset(index),
            ))
        }
        #[cfg(target_endian = "little")]
        {
            self.load(
                roots,
                LoadOp::Kind::RawAligned().Immutable(),
                MemoryRepresentation::TaggedPointer(),
                IsolateData::root_slot_offset(index),
            )
        }
    }

    fn reduce_is_root_constant(&self, object: OpIndex, index: RootIndex) -> V<Word32> {
        #[cfg(feature = "v8_static_roots")]
        {
            if RootsTable::IsReadOnly(index) {
                let root = V::<Object>::cast(self.uint_ptr_constant(0));
                return self.tagged_equal(object, root);
            }
        }
        self.tagged_equal(object, self.reduce_root_constant(index))
    }

    fn reduce_null(&self, type_: wasm::ValueType) -> OpIndex {
      let index = if type_.use_wasm_null() {
            wasm::kWasmNull()
        } else {
            RootIndex {} // Assuming RootIndex can be constructed without arguments
        };
        self.reduce_root_constant(index)
    }

    fn reduce_is_null(&self, object: OpIndex, type_: wasm::ValueType) -> V<Word32> {
        let index = if type_.use_wasm_null() {
            wasm::kWasmNull()
        } else {
            RootIndex {} // Assuming RootIndex can be constructed without arguments
        };
        self.reduce_is_root_constant(object, index)
    }

    fn reduce_assert_not_null(&self, object: V<Object>, type_: wasm::ValueType, trap_id: TrapId) -> V<Object> {
        if (unsafe { v8_flags.experimental_wasm_skip_null_checks }) {
            return object;
        }
        if (trap_id == TrapId::kTrapNullDereference) {
                if (self.null_check_strategy() == NullCheckStrategy::kExplicit ||
                  wasm::IsSubtypeOf(wasm::kWasmI31Ref.AsNonNull(), type_, self.module()) ||
                  !type_.use_wasm_null()) {
                    self.trap_if(self.reduce_is_null(object.cast(), type_), trap_id);
                } else {
                  // Otherwise, load the word after the map word.
                  self.load(object.cast(), LoadOp::Kind::TrapOnNull().Immutable(), MemoryRepresentation::Int32(), kTaggedSize);
                }
        } else {
            self.trap_if(self.reduce_is_null(object.cast(), type_), trap_id);
        }
        return object;
    }

    fn reduce_rtt_canon(&self, rtts: V<FixedArray>, type_index: wasm::ModuleTypeIndex) -> V<Map> {
        let map_offset = 0 + type_index.index * kTaggedSize;
        self.load(rtts.cast(), LoadOp::Kind::TaggedBase().Immutable(), MemoryRepresentation::AnyTagged(), map_offset)
    }

    fn reduce_wasm_type_check(&self, object: V<Object>, rtt: OptionalV<Map>, config: WasmTypeCheckConfig) -> V<Word32> {
        if rtt.value.is_some() {
            self.reduce_wasm_type_check_rtt(object, rtt, config)
        } else {
            self.reduce_wasm_type_check_abstract(object, config)
        }
    }

    fn reduce_wasm_type_cast(&self, object: V<Object>, rtt: OptionalV<Map>, config: WasmTypeCheckConfig) -> V<Object> {
        if rtt.value.is_some() {
            self.reduce_wasm_type_cast_rtt(object, rtt, config)
        } else {
            self.reduce_wasm_type_cast_abstract(object, config)
        }
    }

    fn reduce_any_convert_extern(&self, object: V<Object>) -> V<Object> {
        let mut end_label: Label<Object> = Label::new(self.asm());
        let mut null_label: Label<()> = Label::new(self.asm());
        let mut smi_label: Label<()> = Label::new(self.asm());
        let mut int_to_smi_label: Label<()> = Label::new(self.asm());
        let mut heap_number_label: Label<()> = Label::new(self.asm());

        const kInt31MaxValue: i32 = 0x3fffffff;
        const kInt31MinValue: i32 = -kInt31MaxValue - 1;

        self.goto_if(self.reduce_is_null(object.cast(), wasm::kWasmExternRef), &mut null_label);
        self.goto_if(self.is_smi(object), &mut smi_label);
        self.goto_if(self.has_instance_type(object, HEAP_NUMBER_TYPE), &mut heap_number_label);
        self.goto(&mut end_label, object);

        self.bind(&mut null_label);
        self.goto(&mut end_label, self.null(wasm::kWasmAnyRef));

        self.bind(&mut smi_label);
        if SmiValuesAre31BitsBool::get() {
            self.goto(&mut end_label, object);
        } else {
            let mut convert_to_heap_number_label: Label<()> = Label::new(self.asm());
            let int_value: V<Word32> = self.untag_smi(V::<Smi>::cast(object));

            self.goto_if(self.int32_less_than(self.word32_constant(kInt31MaxValue), int_value), &mut convert_to_heap_number_label);
            self.goto_if(self.int32_less_than(int_value, self.word32_constant(kInt31MinValue)), &mut convert_to_heap_number_label);
            self.goto(&mut end_label, object);

            self.bind(&mut convert_to_heap_number_label);
            let heap_number: V<Object> = self.wasm_call_builtin_through_jumptable::<builtin_call_descriptors::BuiltinCallDescriptor>({int_value});
            self.goto(&mut end_label, heap_number);
        }

        self.bind(&mut heap_number_label);
        let float_value: V<Float64> = self.load_heap_number_value(V::<HeapNumber>::cast(object));
        self.goto_if(self.float64_less_than(float_value, self.float64_constant(kInt31MinValue as f64)), &mut end_label, object);
        self.goto_if(self.float64_less_than(self.float64_constant(kInt31MaxValue as f64), float_value), &mut end_label, object);
        let is_minus_zero: V<Word32>;

        if true {
          let minus_zero: V<Word64> = self.word64_constant(kMinusZeroBits);
          let float_bits: V<Word64> = self.bitcast_float64_to_word64(float_value);
          is_minus_zero = self.word64_equal(float_bits, minus_zero);
        } else {
          let mut done: Label<Word32> = Label::new(self.asm());

          let value_lo: V<Word32> = self.float64_extract_low_word32(float_value);
          self.goto_if_not(self.word32_equal(value_lo, self.word32_constant(kMinusZeroLoBits)), &mut done, self.word32_constant(0));
          let value_hi: V<Word32> = self.float64_extract_high_word32(float_value);
          self.goto(&mut done, self.word32_equal(value_hi, self.word32_constant(kMinusZeroHiBits)));
          self.bind(&mut done);
          is_minus_zero = unsafe { std::mem::transmute(0) };
        }
        self.goto_if(is_minus_zero, &mut end_label, object);
        let int_value: V<Word32> = self.truncate_float64_to_int32_overflow_undefined(float_value);
        self.goto_if(self.float64_equal(float_value, self.change_int32_to_float64(int_value)), &mut int_to_smi_label);
        self.goto(&mut end_label, object);

        self.bind(&mut int_to_smi_label);
        self.goto(&mut end_label, self.tag_smi(int_value));

        self.bind(&mut end_label);
        let result: V<Object> = unsafe { std::mem::transmute(0) };
        return result;
    }

    fn reduce_extern_convert_any(&self, object: V<Object>) -> V<Object> {
        let mut end: Label<Object> = Label::new(self.asm());
        self.goto_if_not(self.reduce_is_null(object.cast(), wasm::kWasmAnyRef), &mut end, object);
        self.goto(&mut end, self.null(wasm::kWasmExternRef));
        self.bind(&mut end);
        let result: V<Object> = unsafe { std::mem::transmute(0) };
        return result;
    }

    fn reduce_wasm_type_annotation(&self, value: V<Object>, _type: wasm::ValueType) -> V<Object> {
        value
    }

    fn reduce_struct_get(&self, object: V<WasmStructNullable>, _type: &wasm::StructType, _type_index: wasm::ModuleTypeIndex, field_index: i32, is_signed: bool, null_check: CheckForNull) -> V<Any> {
        let (explicit_null_check, implicit_null_check) = self.null_checks_for_struct_op(null_check, field_index);

        if explicit_null_check {
            self.trap_if(self.reduce_is_null(object.cast(), wasm::kWasmAnyRef), TrapId::kTrapNullDereference);
        }

        let load_kind: LoadOp::Kind = if implicit_null_check {
            LoadOp::Kind::TrapOnNull()
        } else {
            LoadOp::Kind::TaggedBase()
        };
        let mut load_kind = load_kind;
        let load_kind = if !_type.mutability(field_index) {
          load_kind.Immutable()
        } else {
          load_kind
        };
        let repr = self.representation_for(_type.field(field_index), is_signed);

        self.load(object.cast(), load_kind, repr, self.field_offset(_type, field_index))
    }

    fn reduce_struct_set(&self, object: V<WasmStructNullable>, value: V<Any>, _type: &wasm::StructType, _type_index: wasm::ModuleTypeIndex, field_index: i32, null_check: CheckForNull) -> V<None> {
        let (explicit_null_check, implicit_null_check) = self.null_checks_for_struct_op(null_check, field_index);

        if explicit_null_check {
            self.trap_if(self.reduce_is_null(object.cast(), wasm::kWasmAnyRef), TrapId::kTrapNullDereference);
        }

        let store_kind: StoreOp::Kind = if implicit_null_check {
            StoreOp::Kind::TrapOnNull()
        } else {
            StoreOp::Kind::TaggedBase()
        };
        let repr = self.representation_for(_type.field(field_index), true);

        self.store(object.cast(), value, store_kind, repr, if _type.field(field_index).is_reference() { WriteBarrierKind::kFullWriteBarrier } else { WriteBarrierKind::kNoWriteBarrier }, self.field_offset(_type, field_index));

        unsafe { std::mem::transmute(0) }
    }

    fn reduce_array_get(&self, array: V<WasmArrayNullable>, index: V<Word32>, array_type: &wasm::ArrayType, is_signed: bool) -> V<Any> {
        let is_mutable = array_type.mutability();
        let load_kind: LoadOp::Kind = if is_mutable {
            LoadOp::Kind::TaggedBase()
        } else {
            LoadOp::Kind::TaggedBase().Immutable()
        };
        self.load(array.cast(), self.change_int32_to_int_ptr(index), load_kind, self.representation_for(array_type.element_type(), is_signed), 0, array_type.element_type().value_kind_size_log2())
    }

    fn reduce_array_set(&self, array: V<WasmArrayNullable>, index: V<Word32>, value: V<Any>, element_type: wasm::ValueType) -> V<None> {
        self.store(array.cast(), self.change_int32_to_int_ptr(index), value, LoadOp::Kind::TaggedBase(), self.representation_for(element_type, true), if element_type.is_reference() { WriteBarrierKind::kFullWriteBarrier } else { WriteBarrierKind::kNoWriteBarrier }, 0, element_type.value_kind_size_log2());
        unsafe { std::mem::transmute(0) }
    }

    fn reduce_array_length(&self, array: V<WasmArrayNullable>, null_check: CheckForNull) -> V<Word32> {
        let explicit_null_check = null_check == CheckForNull::kWithNullCheck && self.null_check_strategy() == NullCheckStrategy::kExplicit;
        let implicit_null_check = null_check == CheckForNull::kWithNullCheck && self.null_check_strategy() == NullCheckStrategy::kTrapHandler;

        if explicit_null_check {
            self.trap_if(self.reduce_is_null(array.cast(), wasm::kWasmAnyRef), TrapId::kTrapNullDereference);
        }

        let load_kind: LoadOp::Kind = if implicit_null_check {
            LoadOp::Kind::TrapOnNull().Immutable()
        } else {
            LoadOp::Kind::TaggedBase().Immutable()
        };

        self.load(array.cast(), load_kind, self.representation_for(wasm::kWasmI32, true), wasm::wasm_objects::WasmArray::kLengthOffset)
    }

    fn reduce_wasm_allocate_array(&self, rtt: V<Map>, length: V<Word32>, array_type: &wasm::ArrayType) -> V<wasm::wasm_objects::WasmArray> {
      self.trap_if_not(
          self.uint32_less_than_or_equal(
              length, self.word32_constant(wasm::wasm_objects::WasmArray::MaxLength(array_type) as i32)),
          TrapId::kTrapArrayTooLarge);
      let element_type = array_type.element_type();

      let padded_length = self.word32_bitwise_and(
          self.word32_add(self.word32_mul(length, self.word32_constant(
                                            element_type.value_kind_size() as i32)),
                       self.word32_constant(kObjectAlignment - 1)),
          self.word32_constant(-kObjectAlignment));
      let a: Uninitialized<wasm::wasm_objects::WasmArray> = self.allocate(
          self.change_uint32_to_uint_ptr(self.word32_add(
              padded_length, self.word32_constant(wasm::wasm_objects::WasmArray::kHeaderSize as i32))),
          AllocationType::kYoung);

      self.initialize_field(a.cast(), AccessBuilder::ForMap(WriteBarrierKind::kNoWriteBarrier),
                         rtt);
      self.initialize_field(a.cast(), AccessBuilder::ForJSObjectPropertiesOrHash(),
                         self.load_root(RootIndex {}));
      self.initialize_field(a.cast(), AccessBuilder::ForWasmArrayLength(), length);

      let array: V<wasm::wasm_objects::WasmArray> = self.finish_initialization(a);
      return array;
    }

    fn reduce_wasm_allocate_struct(&self, rtt: V<Map>, struct_type: &wasm::StructType) -> V<wasm::wasm_objects::WasmStruct> {
      let size = wasm::wasm_objects::WasmStruct::Size(struct_type);
      let s: Uninitialized<wasm::wasm_objects::WasmStruct> =
          self.allocate(size, AllocationType::kYoung);
      self.initialize_field(s.cast(), AccessBuilder::ForMap(WriteBarrierKind::kNoWriteBarrier),
                         rtt);
      self.initialize_field(s.cast(), AccessBuilder::ForJSObjectPropertiesOrHash(),
                         self.load_root(RootIndex {}));
      let struct_value: V<wasm::wasm_objects::WasmStruct> = self.finish_initialization(s);
      return struct_value;
    }

    fn reduce_wasm_ref_func(&self, wasm_instance: V<WasmTrustedInstanceData>, function_index: u32) -> V<WasmFuncRef> {
      let func_refs: V<FixedArray> = self.load_immutable_instance_field(
          wasm_instance, FuncRefs, MemoryRepresentation::TaggedPointer());
      let maybe_func_ref: V<Object> =
          self.load_fixed_array_element(func_refs, function_index);

      let mut done: Label<WasmFuncRef> = Label::new(self.asm());
      if self.is_smi(maybe_func_ref) {
        let extract_shared_data =
            !self.shared() && self.module().function_is_shared(function_index);

        let from_builtin: V<WasmFuncRef> = self.wasm_call_builtin_through_jumptable::<builtin_call_descriptors::BuiltinCallDescriptor>(
            {self.word32_constant(function_index as i32),
             self.word32_constant(if extract_shared_data { 1 } else { 0 })});

        self.goto(&mut done, from_builtin);
      } else {
        self.goto(&mut done, V::<WasmFuncRef>::cast(maybe_
