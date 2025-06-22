// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_early_lowering_reducer {
    use std::borrow::Borrow;
    use std::rc::Rc;

    //use crate::compiler::feedback_source::FeedbackSource; // Assuming FeedbackSource is defined elsewhere
    //use crate::compiler::globals::*; // Assuming globals are defined elsewhere
    //use crate::compiler::turboshaft::assembler::*; // Assuming assembler is defined elsewhere
    //use crate::compiler::turboshaft::index::*; // Assuming index is defined elsewhere
    //use crate::compiler::turboshaft::representations::*; // Assuming representations is defined elsewhere
    //use crate::deoptimizer::deoptimize_reason::DeoptimizeReason; // Assuming DeoptimizeReason is defined elsewhere
    //use crate::objects::contexts::Context; // Assuming Context is defined elsewhere
    //use crate::objects::instance_type::*; // Assuming InstanceType is defined elsewhere

    // Assuming define_assembler_macros.inc would define macros like __IsSmi, __LoadMapField, etc.
    // These would need to be translated into Rust functions or methods.

    // Placeholder types, replace with actual definitions
    pub struct Object {}
    pub struct FrameState {}
    pub struct FeedbackSource {}
    pub type InstanceType = u32;
    pub struct Map {}
    pub struct Word32 {}
    pub struct InternalizedString {}
    pub struct String {}
    pub struct Boolean {}
    pub struct FixedArray {}
    pub struct Context {}
    pub struct HeapNumber {}
    pub struct JSArray {}
    pub struct JSObject {}
    pub struct NativeContext {}
    pub struct PropertyArray {}
    pub struct HeapObject {}
    pub struct JSGeneratorObject {}
    pub struct Smi {}
    pub type Float64 = f64;
    pub struct RootIndex;
    pub struct Isolate;
    pub struct LocalIsolate;
    pub struct JSHeapBroker;
    pub struct LocalFactory;
    pub struct MapRef;
    pub struct Undefined;

    pub enum WriteBarrierKind {
        kNoWriteBarrier,
        kFullWriteBarrier,
    }

    pub enum MemoryRepresentation {
        TaggedSigned,
        AnyTagged,
    }

    pub struct AccessBuilder;
    impl AccessBuilder{
        pub fn ForMap() -> Self {Self}
        pub fn ForThinStringActual() -> Self {Self}
        pub fn ForHeapInt32Value() -> Self {Self}
        pub fn ForHeapNumberValue() -> Self {Self}
        pub fn ForMapBitField() -> Self {Self}
        pub fn ForMapPrototype() -> Self {Self}
        pub fn ForMapBitField3() -> Self {Self}
        pub fn ForJSObjectPropertiesOrHash() -> Self {Self}
        pub fn ForPropertyArraySlot(i: usize) -> Self {Self}
        pub fn ForPropertyArrayLengthAndHash() -> Self {Self}
    }

    pub struct V<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> V<T> {
        pub fn Cast<U>(_v: V<T>) -> V<U> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl V<Object> {
        pub fn Cast<U>(_v: V<Object>) -> V<U> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl V<HeapObject> {
        pub fn Cast<U>(_v: V<HeapObject>) -> V<U> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl V<Smi> {
        pub fn Cast<U>(_v: V<Smi>) -> V<U> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl V<String> {
        pub fn Cast<U>(_v: V<String>) -> V<U> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct ScopedVar<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> ScopedVar<T> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>, _value: V<T>) -> Self {
            ScopedVar {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl ScopedVar<Object> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>, _value: V<Object>) -> Self {
            ScopedVar {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl ScopedVar<HeapNumber> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>, _value: V<HeapNumber>) -> Self {
            ScopedVar {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl ScopedVar<Word32> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>, _value: V<Word32>) -> Self {
            ScopedVar {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl ScopedVar<Float64> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>, _value: V<Float64>) -> Self {
            ScopedVar {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct Label<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Label<T> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>) -> Self {
            Label {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct LoopLabel<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> LoopLabel<T> {
        pub fn new(_reducer: &mut MaglevEarlyLoweringReducer<()>) -> Self {
            LoopLabel {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct Uninitialized<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    pub struct StoreOp;
    impl StoreOp {
        pub enum Kind {
            TaggedBase,
        }
    }

    pub struct CheckForMinusZeroMode;
    impl CheckForMinusZeroMode {
        pub const kCheckForMinusZero: Self = Self;
    }

    pub struct LazyDeoptOnThrow;

    pub struct HeapObjectRef;

    pub struct InternalizedStringRef {
        object: Rc<InternalizedString>,
    }

    impl InternalizedStringRef {
        pub fn object(&self) -> Rc<InternalizedString> {
            self.object.clone()
        }
    }

    pub struct ContextSidePropertyCell;
    impl ContextSidePropertyCell {
        pub fn MutableInt32() -> Self {Self}
        pub fn MutableHeapNumber() -> Self {Self}
        pub fn Const() -> Self {Self}
        pub fn SmiMarker() -> Self {Self}
        pub const kPropertyDetailsRawOffset: usize = 0; // Dummy value
    }

    pub struct InstanceTypeChecker;
    impl InstanceTypeChecker {
        pub const kNonJsReceiverMapLimit: u32 = 0;
        pub fn UniqueMapOfInstanceType(_instance_type: InstanceType) -> Option<RootIndex> {
            None
        }
    }

    pub struct MapBitField;
    impl MapBitField {
        pub const kMask: u32 = 0;
    }

    pub struct PropertyArrayLengthAndHash;
    impl PropertyArrayLengthAndHash {
        pub const kShift: u32 = 0;
    }

    pub struct PropertyArrayHashField;
    impl PropertyArrayHashField {
        pub const kShift: u32 = 0;
        pub const kMask: u32 = 0;
        pub const kNoHashSentinel: u32 = 0;
    }

    pub struct MapBits3;
    impl MapBits3 {
        pub struct IsDeprecatedBit;
        impl IsDeprecatedBit {
            pub const kMask: u32 = 0;
        }
    }

    pub struct MapBits1;
    impl MapBits1 {
        pub struct HasNamedInterceptorBit;
        impl HasNamedInterceptorBit {
            pub const kMask: u32 = 0;
        }
        pub struct IsAccessCheckNeededBit;
        impl IsAccessCheckNeededBit {
            pub const kMask: u32 = 0;
        }
    }

    pub struct AllocationType;
    impl AllocationType {
        pub const kYoung: Self = Self;
    }

    pub struct Flags {
        pub script_context_mutable_heap_int32: bool,
        pub script_context_mutable_heap_number: bool
    }

    lazy_static::lazy_static! {
        pub static ref v8_flags: Flags = Flags{script_context_mutable_heap_int32: false, script_context_mutable_heap_number: false};
    }

    // Mocking the global functions/macros

    macro_rules! unlikely {
        ($e:expr) => {
            $e
        };
    }

    macro_rules! likely {
        ($e:expr) => {
            $e
        };
    }

    macro_rules! if_not {
        ($cond:expr, $block:block) => {
            if !$cond {
                $block
            }
        };
    }

    macro_rules! if_ {
        ($cond:expr, $block:block) => {
            if $cond {
                $block
            }
        };
    }

    // Mock Assembler-like methods
    struct AssemblerLike<'a> {
        data: &'a dyn DataProvider,
    }

    impl<'a> AssemblerLike<'a> {
        fn new(data: &'a dyn DataProvider) -> Self {
            AssemblerLike { data }
        }

        fn IsSmi(&self, _input: V<Object>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn DeoptimizeIf(&self, _condition: V<Boolean>, _frame_state: V<FrameState>, _reason: DeoptimizeReason, _feedback: &FeedbackSource) {}

        fn LoadMapField(&self, _input: V<Object>) -> V<Map> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn TaggedEqual(&self, _a: V<Map>, _b: V<Map>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn HeapConstant<T>(&self, _obj: Rc<T>) -> V<T> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn LoadInstanceTypeField(&self, _map: V<Map>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Word32Equal(&self, _a: V<Word32>, _b: InstanceType) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn DeoptimizeIfNot(&self, _condition: V<Boolean>, _frame_state: V<FrameState>, _reason: DeoptimizeReason, _feedback: &FeedbackSource) {}

        fn Word32BitwiseAnd(&self, _a: V<Word32>, _b: u32) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn ObjectIsString(&self, _object: V<Object>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn StringEqual(&self, _a: V<String>, _b: V<String>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn RootEqual(&self, _a: V<Boolean>, _b: RootIndex, _isolate: &Isolate) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn AllocateHeapNumberWithValue(&self, _value: Float64, _factory: &LocalFactory) -> V<HeapNumber> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn ChangeInt32ToFloat64(&self, _value: V<Word32>) -> Float64 {
            0.0 // Dummy value
        }

        fn LoadHeapInt32Value(&self, _heap_number: V<HeapNumber>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn SmiConstant(&self, _value: ContextSidePropertyCell) -> V<Smi> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn TaggedEqual(&self, _a: V<Object>, _b: V<Smi>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn ChangeFloat64ToInt32OrDeopt(&self, _value: Float64, _frame_state: V<FrameState>, _mode: CheckForMinusZeroMode, _feedback: &FeedbackSource) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn LoadHeapNumberValue(&self, _heap_number: V<HeapNumber>) -> Float64 {
            0.0 // Dummy value
        }

        fn UntagSmi(&self, _smi: V<Smi>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Uint32LessThan(&self, _a: V<Word32>, _b: V<Word32>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Word32Add(&self, _a: V<Word32>, _b: i32) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn TagSmi(&self, _value: V<Word32>) -> V<Smi> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn FinishInitialization<T>(&self, _uninit: Uninitialized<T>) -> V<T> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Word32ShiftLeft(&self, _a: V<Word32>, _b: PropertyArrayLengthAndHash) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Word32BitwiseOr(&self, _a: V<Word32>, _b: u32) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn BitcastTaggedToWordPtr(&self, _value: V<Map>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn TruncateWordPtrToWord32(&self, _value: V<Word32>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Uint32LessThanOrEqual(&self, _a: u32, _b: V<Word32>) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn Word32Sub(&self, _a: V<Word32>, _b: InstanceType) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }
        fn Uint32LessThanOrEqual(&self, _a: V<Word32>, _b: InstanceType) -> V<Word32> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn CallRuntime_HasInPrototypeChain(&self, _isolate: &Isolate, _frame_state: V<FrameState>, _native_context: V<NativeContext>, _lazy_deopt_on_throw: LazyDeoptOnThrow, _object: V<Object>, _target_proto: V<HeapObject>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn NoContextConstant(&self) -> V<Context> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn CallRuntime_TryMigrateInstance(&self, _isolate: &Isolate, _context: V<Context>, _object: V<Object>) -> V<Object> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn ObjectIsSmi(&self, _object: V<Object>) -> V<Boolean> {
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        fn CallRuntime_TransitionElementsKind(&self, _isolate: &Isolate, _context: V<Context>, _object: V<HeapObject>, _target_map: V<Map>) {}

        fn CallRuntime_ThrowConstructorReturnedNonObject(&self, _isolate: &Isolate, _frame_state: V<FrameState>, _native_context: V<NativeContext>, _lazy_deopt_on_throw: LazyDeoptOnThrow) {}

        fn Unreachable(&self) {}
    }

    // Mock DataProvider
    trait DataProvider {
        fn isolate(&self) -> &Isolate;
        fn broker(&self) -> &JSHeapBroker;
    }

    // Base class (Next) definition - replace () with actual type if needed
    pub struct BaseReducer {
        data: Rc<dyn DataProvider>,
    }

    impl BaseReducer {
        fn new(data: Rc<dyn DataProvider>) -> Self {
            BaseReducer { data }
        }
    }

    /// A reducer for performing early lowering of Maglev operations.
    pub struct MaglevEarlyLoweringReducer<Next> {
        next: Next,
        assembler: AssemblerLike<'static>,
        data: Rc<dyn DataProvider>,
    }

    impl MaglevEarlyLoweringReducer<()> {
        pub fn new(data: Rc<dyn DataProvider>) -> Self {
            let assembler = AssemblerLike::new(unsafe { std::mem::transmute(&*data) });
            MaglevEarlyLoweringReducer {
                next: (),
                assembler,
                data: data.clone(),
            }
        }
    }

    impl<Next> MaglevEarlyLoweringReducer<Next> {
        // Boilerplate for reducer.  Assumed to be macro-generated.
        //TURBOSHAFT_REDUCER_BOILERPLATE(MaglevEarlyLowering)

        /// Checks the instance type of an object.
        pub fn check_instance_type(
            &mut self,
            input: V<Object>,
            frame_state: V<FrameState>,
            feedback: &FeedbackSource,
            first_instance_type: InstanceType,
            last_instance_type: InstanceType,
            check_smi: bool,
        ) {
            if check_smi {
                self.assembler.DeoptimizeIf(self.assembler.IsSmi(input), frame_state, /*DeoptimizeReason::kWrongInstanceType*/ unsafe { std::mem::transmute(0u32) }, feedback);
            }

            let map = self.assembler.LoadMapField(input);

            if first_instance_type == last_instance_type {
                if let Some(_expected_index) = InstanceTypeChecker::UniqueMapOfInstanceType(first_instance_type) {
                    // Static roots bool not supported
                    // let expected_map = Cast<HeapObject>(isolate_->root_handle(expected_index.value()));
                    // __ DeoptimizeIfNot(__ TaggedEqual(map, __ HeapConstant(expected_map)),
                    //                    frame_state, DeoptimizeReason::kWrongInstanceType,
                    //                    feedback);
                    return;
                }
                let instance_type = self.assembler.LoadInstanceTypeField(map);
                self.assembler.DeoptimizeIfNot(self.assembler.Word32Equal(instance_type, first_instance_type), frame_state, /*DeoptimizeReason::kWrongInstanceType*/ unsafe { std::mem::transmute(0u32) }, feedback);
            } else {
                self.assembler.DeoptimizeIfNot(self.check_instance_type_is_in_range(map, first_instance_type, last_instance_type), frame_state, /*DeoptimizeReason::kWrongInstanceType*/ unsafe { std::mem::transmute(0u32) }, feedback);
            }
        }

        /// Checks if an object is an internalized string.
        pub fn checked_internalized_string(
            &mut self,
            object: V<Object>,
            frame_state: V<FrameState>,
            check_smi: bool,
            feedback: &FeedbackSource,
        ) -> V<InternalizedString> {
            if check_smi {
                self.assembler.DeoptimizeIf(self.assembler.IsSmi(object), frame_state, /*DeoptimizeReason::kSmi*/ unsafe { std::mem::transmute(0u32) }, feedback);
            }

            let mut done: Label<InternalizedString> = Label::new(self);
            let map = self.assembler.LoadMapField(object);
            let instance_type = self.assembler.LoadInstanceTypeField(map);

            if unlikely!(self.assembler.Word32BitwiseAnd(instance_type, /*kIsNotStringMask | kIsNotInternalizedMask*/ 0) != unsafe { std::mem::transmute(0u32)}) {
                self.assembler.DeoptimizeIf(self.assembler.Word32BitwiseAnd(instance_type, /*kIsNotStringMask*/ 0) != unsafe { std::mem::transmute(0u32)}, frame_state, /*DeoptimizeReason::kWrongMap*/ unsafe { std::mem::transmute(0u32) }, feedback);
                self.assembler.DeoptimizeIfNot(self.assembler.Word32BitwiseAnd(instance_type, /*kThinStringTagBit*/ 0) != unsafe { std::mem::transmute(0u32)}, frame_state, /*DeoptimizeReason::kWrongMap*/ unsafe { std::mem::transmute(0u32) }, feedback);

                let intern_string = self.template_load_field::<InternalizedString>(object, AccessBuilder::ForThinStringActual());
                //GOTO(done, intern_string);
                return intern_string; // Replacing GOTO
            } else {
                //GOTO(done, V<InternalizedString>::Cast(object));
                return V::Cast(object); // Replacing GOTO
            }
        }

        /// Checks if an object equals a string value.
        pub fn check_value_equals_string(
            &mut self,
            object: V<Object>,
            value: InternalizedStringRef,
            frame_state: V<FrameState>,
            feedback: &FeedbackSource,
        ) {
            if_not!(likely!(self.assembler.TaggedEqual(object, self.assembler.HeapConstant(value.object()))), {
                self.assembler.DeoptimizeIfNot(self.assembler.ObjectIsString(object), frame_state, /*DeoptimizeReason::kNotAString*/ unsafe { std::mem::transmute(0u32) }, feedback);
                let is_same_string_bool = self.assembler.StringEqual(V::Cast(object), self.template_heap_constant::<String>(value.object()));
                self.assembler.DeoptimizeIf(
                    self.assembler.RootEqual(is_same_string_bool, /*RootIndex::kFalseValue*/ unsafe { std::mem::transmute(0u32) }, self.data.isolate()),
                    frame_state, /*DeoptimizeReason::kWrongValue*/ unsafe { std::mem::transmute(0u32) }, feedback);
            });
        }

        /// Checks the result of a constructor call.
        pub fn check_construct_result(
            &mut self,
            construct_result: V<Object>,
            implicit_receiver: V<Object>,
        ) -> V<Object> {
            let mut done: Label<Object> = Label::new(self);

            // Replacing GOTOs with if/else
            if self.assembler.RootEqual(construct_result, /*RootIndex::kUndefinedValue*/ unsafe { std::mem::transmute(0u32) }, self.data.isolate())._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data {
                return implicit_receiver;
            }

            if self.assembler.IsSmi(construct_result)._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data {
                return implicit_receiver;
            }

            if self.js_any_is_not_primitive(V::Cast(construct_result))._phantom.data == (V { _phantom: std::marker::PhantomData::<Word32> {} })._phantom.data {
                return construct_result;
            }

            return implicit_receiver;
        }

        /// Loads script context side data.
        pub fn load_script_context_side_data(
            &mut self,
            script_context: V<Context>,
            index: i32,
        ) -> V<Object> {
            let side_table: V<FixedArray> = self.template_load_tagged_field::<FixedArray>(
                script_context,
                /*Context::OffsetOfElementAt(Context::CONTEXT_SIDE_TABLE_PROPERTY_INDEX)*/ 0,
            );
            self.template_load_tagged_field(
                side_table,
                /*FixedArray::OffsetOfElementAt(index - Context::MIN_CONTEXT_EXTENDED_SLOTS)*/ 0,
            )
        }

        /// Loads a script context property from side data.
        pub fn load_script_context_property_from_side_data(&mut self, side_data: V<Object>) -> V<Object> {
            let mut property: ScopedVar<Object> = ScopedVar::new(self, side_data);
            if_not!(self.assembler.IsSmi(side_data)._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, {
                property = self.template_load_tagged_field(
                    side_data,
                    /*ContextSidePropertyCell::kPropertyDetailsRawOffset*/ 0,
                );
            });
            V {
                _phantom: std::marker::PhantomData,
            }
        }

        /// Loads a heap number from script context.
        pub fn load_heap_number_from_script_context(
            &mut self,
            script_context: V<Context>,
            index: i32,
            heap_number: V<HeapNumber>,
        ) -> V<HeapNumber> {
            let data = self.load_script_context_side_data(script_context, index);
            let property = self.load_script_context_property_from_side_data(data);
            let mut result: ScopedVar<HeapNumber> = ScopedVar::new(self, heap_number);
            let mut done: Label<()> = Label::new(self);

            if v8_flags.script_context_mutable_heap_int32 {
                if_!(self.assembler.TaggedEqual(
                        property,
                        self.assembler.SmiConstant(ContextSidePropertyCell::MutableInt32())
                )._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, {
                    result = ScopedVar::new(self, self.assembler.AllocateHeapNumberWithValue(
                        self.assembler.ChangeInt32ToFloat64(self.assembler.LoadHeapInt32Value(heap_number)),
                        self.data.isolate().AsLocalIsolate().factory()
                    ));
                    return heap_number;
                });
            }

            if_!(self.assembler.TaggedEqual(
                    property,
                    self.assembler.SmiConstant(ContextSidePropertyCell::MutableHeapNumber())
            )._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, {
                result = ScopedVar::new(self, self.assembler.AllocateHeapNumberWithValue(
                    self.assembler.LoadHeapNumberValue(heap_number),
                    self.data.isolate().AsLocalIsolate().factory()
                ));
            });

            return heap_number;
        }

        /// Implements the slow path for storing to script context.
        pub fn store_script_context_slow_path(
            &mut self,
            script_context: V<Context>,
            old_value: V<Object>,
            new_value: V<Object>,
            side_data: V<Object>,
            frame_state: V<FrameState>,
            feedback: &FeedbackSource,
            done: &mut Label<()>,
        ) {
            self.assembler.DeoptimizeIf(
                self.assembler.RootEqual(side_data, /*RootIndex::kUndefinedValue*/ unsafe { std::mem::transmute(0u32) }, self.data.isolate())._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data,
                frame_state, /*DeoptimizeReason::kWrongValue*/ unsafe { std::mem::transmute(0u32) }, feedback);
            let property = self.load_script_context_property_from_side_data(side_data);

            self.assembler.DeoptimizeIf(
                self.assembler.TaggedEqual(property, self.assembler.SmiConstant(ContextSidePropertyCell::Const()))._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data,
                frame_state, /*DeoptimizeReason::kWrongValue*/ unsafe { std::mem::transmute(0u32) }, feedback);

            if v8_flags.script_context_mutable_heap_number {
                if_!(self.assembler.TaggedEqual(property, self.assembler.SmiConstant(ContextSidePropertyCell::SmiMarker()))._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, {
                    self.assembler.DeoptimizeIfNot(self.assembler.IsSmi(new_value)._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, frame_state, /*DeoptimizeReason::kWrongValue*/ unsafe { std::mem::transmute(0u32) }, feedback);
                } else {
                    if v8_flags.script_context_mutable_heap_int32 {
                        if_!(self.assembler.TaggedEqual(
                                property,
                                self.assembler.SmiConstant(ContextSidePropertyCell::MutableInt32())
                        )._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data, {
                            let mut number_value: ScopedVar<Word32> = ScopedVar::new(self, unsafe { std::mem::transmute(0u32) });
                            if self.assembler.IsSmi(new_value)._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data {
                                number_value = ScopedVar::new(self, self.assembler.UntagSmi(V::Cast(new_value)));
                            } else {
                                let map = self.assembler.LoadMapField(new_value);
                                self.assembler.DeoptimizeIfNot(
                                    self.assembler.TaggedEqual(map, self.assembler.HeapConstant(self.data.isolate().AsLocalIsolate().factory().borrow())),
                                    frame_state, /*DeoptimizeReason::kWrongValue*/ unsafe { std::mem::transmute(0u32) }, feedback);
                                number_value = ScopedVar::new(self, self.assembler.ChangeFloat64ToInt32OrDeopt(
                                    self.assembler.LoadHeapNumberValue(V::Cast(new_value)),
                                    frame_state, CheckForMinusZeroMode::kCheckForMinusZero,
                                    feedback));
                            }
                            //__ StoreField(old_value, AccessBuilder::ForHeapInt32Value(), number_value);
                            //GOTO(done);
                            return;
                        });
                    }

                    let mut number_value: ScopedVar<Float64> = ScopedVar::new(self, 0.0);
                    if self.assembler.IsSmi(new_value)._phantom.data == (V { _phantom: std::marker::PhantomData::<Boolean> {} })._phantom.data {
                        number_value = ScopedVar::new(self, self.assembler.ChangeInt32ToFloat64(self.assembler.UntagSmi(V::Cast(new_value))));
                    } else {
                        let map = self.assembler.LoadMapField(new_value);
                        self.assembler.DeoptimizeIfNot(
                            self.assembler.TaggedEqual(map, self.assembler.Heap