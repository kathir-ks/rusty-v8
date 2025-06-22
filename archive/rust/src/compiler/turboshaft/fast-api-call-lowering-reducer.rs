// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod fast_api_call_lowering_reducer {
    use std::any::Any;
    use std::marker::PhantomData;
    use std::mem;
    use std::num::TryFromIntError;
    use std::ops::{BitAnd, BitOr};
    use std::sync::atomic::{AtomicBool, Ordering};

    //use v8::fast_api_calls; // Assuming v8 is available as a crate
    //use crate::compiler::fast_api_calls; // Assuming fast_api_calls is a module
    //use crate::compiler::globals; // Assuming globals is a module
    //use crate::compiler::turboshaft::assembler; // Assuming assembler is a module
    //use crate::compiler::turboshaft::copying_phase; // Assuming copying_phase is a module
    //use crate::compiler::turboshaft::index; // Assuming index is a module
    //use crate::compiler::turboshaft::operations; // Assuming operations is a module
    //use crate::compiler::turboshaft::phase; // Assuming phase is a module
    //use crate::compiler::turboshaft::representations; // Assuming representations is a module

    // Placeholder types and enums.  Need to replace these with actual definitions
    pub struct FastApiCallParameters {
        c_function: FastApiCallFunction,
        c_signature: Box<dyn CFunctionInfo>,
    }

    impl FastApiCallParameters {
        pub fn c_signature(&self) -> &dyn CFunctionInfo {
            self.c_signature.as_ref()
        }
    }

    pub trait CFunctionInfo {
        fn ArgumentCount(&self) -> usize;
        fn ArgumentInfo(&self, i: usize) -> CTypeInfo;
        fn ReturnInfo(&self) -> CTypeInfo;
        fn GetInt64Representation(&self) -> CFunctionInfoInt64Representation;
        fn HasOptions(&self) -> bool;
    }

    pub enum CFunctionInfoInt64Representation {
        kBigInt,
        kNumber,
    }

    pub struct CTypeInfo {
        type_: CTypeInfoType,
        sequence_type: CTypeInfoSequenceType,
        flags: u8,
    }

    impl CTypeInfo {
        pub fn GetType(&self) -> CTypeInfoType {
            self.type_
        }
        pub fn GetSequenceType(&self) -> CTypeInfoSequenceType {
            self.sequence_type
        }
        pub fn GetFlags(&self) -> u8 {
            self.flags
        }
    }

    pub enum CTypeInfoType {
        kVoid,
        kBool,
        kInt32,
        kUint32,
        kInt64,
        kUint64,
        kFloat32,
        kFloat64,
        kPointer,
        kAny,
        kSeqOneByteString,
        kV8Value,
        kApiObject,
        kUint8,
    }

    pub enum CTypeInfoSequenceType {
        kScalar,
        kIsSequence,
    }

    pub struct FastApiCallFunction {
        address: usize, //Representing function pointer as address. Consider using function pointers instead if possible
    }

    // More placeholder types
    pub struct V<T>(PhantomData<T>);
    impl<T> V<T> {
        pub fn Cast<U>(_v: V<T>) -> V<U> {
            V(PhantomData)
        }
    }

    pub struct OpIndex(usize);

    pub struct FrameState;
    pub struct Object;
    pub struct Context;
    pub struct Tuple<T, U>(PhantomData<(T, U)>);
    pub struct Word32;
    pub struct Word64;
    pub struct WordPtr;
    pub struct Float64;
    pub struct Float32;
    pub struct HeapObject;
    pub struct Map;
    pub struct FixedArray;
    pub struct Any;

    //Dummy implementations of functions in Turboshaft
    macro_rules! __ {
        () => {{
            AssemblerHelper
        }};
    }
    struct AssemblerHelper;
    impl AssemblerHelper {
        fn data(&self) -> GraphData {
            GraphData {}
        }
        fn ExternalConstant<T>(&self, _e: T) -> OpIndex {
            OpIndex(0)
        }
        fn StackSlot(&self, _a: usize, _b: usize) -> OpIndex {
            OpIndex(0)
        }
        fn StoreOffHeap<T>(&self, _s: OpIndex, _d: OpIndex, _m: MemoryRepresentation, _offset: usize) {
        }
        fn AdaptLocalArgument(&self, _arg: V<Object>) -> OpIndex {
            OpIndex(0)
        }
        fn graph_zone(&self) -> GraphZone {
            GraphZone {}
        }
        fn Tuple<T,U>(&self, _t: OpIndex, _u: OpIndex) -> OpIndex{
            OpIndex(0)
        }
        fn template_Projection<T>(&self, _t: OpIndex) -> V<T>{
            V(PhantomData)
        }
        fn Word32Equal(&self, _lhs: V<Word32>, _rhs: V<Word32>) -> V<Word32> {
            V(PhantomData)
        }
        fn NoContextConstant(&self) -> Context {
            Context {}
        }
        fn TaggedEqual(&self, _lhs: V<Any>, _rhs: V<Any>) -> V<Word32> {
            V(PhantomData)
        }
        fn ObjectIsSmi(&self, _arg: OpIndex) -> V<Word32> {
            V(PhantomData)
        }
        fn HeapConstant(&self, _factory: &Factory) -> V<HeapObject> {
            V(PhantomData)
        }
        fn LoadMapField(&self, _argument_obj: V<HeapObject>) -> V<Map>{
            V(PhantomData)
        }
        fn LoadInstanceTypeField(&self, _map: V<Map>) -> V<Word32> {
            V(PhantomData)
        }
        fn Word32BitwiseAnd(&self, _x: V<Word32>, _y: Word32) -> V<Word32>{
            V(PhantomData)
        }
        fn template_LoadField<T>(&self, _obj: V<HeapObject>, _access: Access) -> V<T> {
            V(PhantomData)
        }
        fn GetElementStartPointer(&self, _obj: V<HeapObject>, _access: Access) -> V<WordPtr> {
            V(PhantomData)
        }
        fn ReversibleFloat64ToInt32(&self, _r: V<Float64>) -> OpIndex{
            OpIndex(0)
        }
         fn ReversibleFloat64ToUint32(&self, _r: V<Float64>) -> OpIndex{
            OpIndex(0)
        }
         fn ReversibleFloat64ToInt64(&self, _r: V<Float64>) -> OpIndex{
            OpIndex(0)
        }
         fn ReversibleFloat64ToUint64(&self, _r: V<Float64>) -> OpIndex{
            OpIndex(0)
        }
        fn Float64Constant(&self, _c: f64) -> V<Float64> {
            V(PhantomData)
        }
        fn Float64LessThan(&self, _min: V<Float64>, _argument: V<Float64>) -> V<Float64> {
            V(PhantomData)
        }
        fn Float64RoundTiesEven(&self, _clamped: V<Float64>) -> V<Float64>{
            V(PhantomData)
        }
        fn Float64IsNaN(&self, _rounded: V<Float64>) -> V<Float64> {
            V(PhantomData)
        }
        fn CallRuntime<T>(&self, _isolate: &Isolate, _frame_state: V<FrameState>, _context: Context, _lazy_deopt: LazyDeoptOnThrow, _vec: Vec<()>) {

        }
        fn Unreachable(&self) {}
        fn IsolateField(&self, _id: IsolateFieldId) -> OpIndex {
            OpIndex(0)
        }
        fn BitcastHeapObjectToWordPtr(&self, _callee: OpIndex) -> OpIndex {
            OpIndex(0)
        }
        fn IntPtrConstant(&self, _c: i32) -> OpIndex {
            OpIndex(0)
        }
        fn WordPtrConstant(&self, _c: usize) -> OpIndex{
            OpIndex(0)
        }
        fn ChangeInt64ToFloat64(&self, _result: OpIndex) -> V<Any> {
            V(PhantomData)
        }
        fn ChangeUint64ToFloat64(&self, _result: OpIndex) -> V<Any> {
            V(PhantomData)
        }
        fn Call(&self, _callee: OpIndex, _frame_state: V<FrameState>, _arguments: Vec<OpIndex>, _descriptor: &TSCallDescriptor) -> OpIndex{
            OpIndex(0)
        }
        fn Allocate(&self, _k_header_size: usize, _k_young: AllocationType) -> Uninitialized<HeapObject> {
            Uninitialized { value: PhantomData}
        }
        fn InitializeField<T>(&self, _external: Uninitialized<HeapObject>, _access: Access, _field: T) {
        }
        fn FinishInitialization(&self, _external: Uninitialized<HeapObject>) -> V<HeapObject> {
            V(PhantomData)
        }
        fn NewVariable(&self, _rep: RegisterRepresentation) -> Variable {
            Variable{}
        }
        fn SetVariable(&self, _variable: Variable, _value: V<Any>){

        }
        fn GetVariable(&self, _variable: Variable) -> V<Any> {
            V(PhantomData)
        }
        fn TryTruncateFloat64ToInt32(&self, _argument: OpIndex) -> V<Tuple<i32, Word32>> {
            V(PhantomData)
        }
        fn TryTruncateFloat64ToUint32(&self, _argument: OpIndex) -> V<Tuple<u32, Word32>>{
            V(PhantomData)
        }
        fn TryTruncateFloat64ToInt64(&self, _argument: OpIndex) -> V<Tuple<i64, Word32>> {
            V(PhantomData)
        }
         fn TryTruncateFloat64ToUint64(&self, _argument: OpIndex) -> V<Tuple<u64, Word32>> {
            V(PhantomData)
        }
        fn TruncateFloat64ToFloat32(&self, _argument: OpIndex) -> OpIndex {
            OpIndex(0)
        }
    }

    pub struct GraphData {
    }
    impl GraphData{
        fn isolate(&self) -> &Isolate{
            todo!()
        }
        fn set_graph_has_lowered_fast_api_calls(&self){

        }
        fn generating_unreachable_operations(&self) -> bool {
            false
        }
    }
    pub struct GraphZone{}
    pub struct Access{}
    pub struct TSCallDescriptor{}
    impl TSCallDescriptor{
        fn Create(_desc: LinkageSimplifiedCDescriptor, _can_throw: CanThrow, _deopt: LazyDeoptOnThrow, _zone: GraphZone) -> &TSCallDescriptor{
            todo!()
        }
    }
    pub enum CanThrow {
        kNo,
    }
    pub enum LazyDeoptOnThrow {
        kNo,
    }
    pub struct LinkageSimplifiedCDescriptor{}
    impl LinkageSimplifiedCDescriptor {
        fn GetSimplifiedCDescriptor(_zone: GraphZone, _m: MachineSignature, _cd: CallDescriptorNeedsFrameState) -> LinkageSimplifiedCDescriptor{
            todo!()
        }
    }
    pub enum CallDescriptorNeedsFrameState{
        kNeedsFrameState
    }
    pub struct MachineSignature{}
    impl MachineSignature{
        pub fn Builder(_graph_zone: GraphZone, _i: i32, _j: i32) -> MachineSignatureBuilder{
            MachineSignatureBuilder{}
        }
    }
    pub struct MachineSignatureBuilder{}
    impl MachineSignatureBuilder {
        fn AddReturn(&self, _t: MachineType){}
        fn AddParam(&self, _t: MachineType){}
        fn Get(&self) -> MachineSignature{
            MachineSignature{}
        }
    }
    pub struct MachineType{}
    impl MachineType{
        fn TypeForCType(_c: CTypeInfo) -> MachineType{
            MachineType{}
        }
        fn AnyTagged() -> MachineType {
            MachineType{}
        }
    }
    pub struct FastOneByteString{}
    pub struct IsolateFieldId{}
    pub enum AllocationType{
        kYoung
    }
    pub struct Uninitialized<T>{
        value: PhantomData<T>
    }
    pub struct Variable{}
    pub struct RegisterRepresentation{}
    impl RegisterRepresentation{
        fn FromCTypeInfo(_c: CTypeInfo, _i: CFunctionInfoInt64Representation) -> RegisterRepresentation{
            RegisterRepresentation{}
        }
    }
    pub struct ExternalReference{
        reference: usize,
        constant_type: ExternalReferenceConstantType
    }
    impl ExternalReference{
        fn Create(_a: usize, _b: ExternalReferenceConstantType) -> ExternalReference{
            ExternalReference{reference: 0, constant_type: ExternalReferenceConstantType::FAST_C_CALL}
        }
        fn isolate_address() -> ExternalReference{
            ExternalReference{reference: 0, constant_type: ExternalReferenceConstantType::OTHER}
        }
    }
    pub enum ExternalReferenceConstantType{
        FAST_C_CALL,
        OTHER
    }
    pub struct MemoryRepresentation{}
    impl MemoryRepresentation{
        fn UintPtr() -> MemoryRepresentation{
            MemoryRepresentation{}
        }
    }
    pub struct AccessBuilder{}
    impl AccessBuilder{
        fn ForJSExternalObjectValue() -> Access{
            Access{}
        }
        fn ForStringLength() -> Access{
            Access{}
        }
        fn ForSeqOneByteStringCharacter() -> Access{
            Access{}
        }
        fn ForMap() -> Access {
            Access {}
        }
        fn ForJSObjectPropertiesOrHash() -> Access{
            Access{}
        }
        fn ForJSObjectElements() -> Access{
            Access{}
        }
        fn ForJSExternalObjectPointerHandle() -> Access{
            Access{}
        }
    }
    pub struct IsolateAddressId{}
    impl IsolateAddressId{
        fn kExceptionAddress() -> IsolateAddressId{
            IsolateAddressId{}
        }
        fn kContextAddress() -> IsolateAddressId{
            IsolateAddressId{}
        }
    }
    pub struct Linkage{}
    impl Linkage{

    }
    pub struct CallDescriptor{}
    pub struct Isolate{
        factory_: Factory,
    }
    impl Isolate{
        fn factory(&self) -> &Factory{
            &self.factory_
        }
    }

    pub struct Factory{

    }
    impl Factory{
        fn the_hole_value(&self) -> V<HeapObject> {
            V(PhantomData)
        }
        fn null_value(&self) -> V<HeapObject> {
            V(PhantomData)
        }
        fn external_map(&self) -> V<HeapObject> {
            V(PhantomData)
        }
        fn empty_fixed_array(&self) -> V<FixedArray> {
            V(PhantomData)
        }
        fn undefined_value(&self) -> V<HeapObject> {
            V(PhantomData)
        }
    }
    pub struct JSExternalObject{

    }
    impl JSExternalObject{
        const kHeaderSize: usize = 0;
    }

    pub enum FastApiCallOp{
        kSuccessValue,
        kFailureValue
    }

    //Helper functions/structs for the compilation.
    const kStringRepresentationAndEncodingMask: Word32 = Word32 {};
    const kSeqOneByteStringTag: Word32 = Word32 {};
    const JS_ARRAY_TYPE: Word32 = Word32 {};
    const kMinSafeInteger: f64 = -9007199254740991.0;
    const kMaxSafeInteger: f64 = 9007199254740991.0;

    pub enum ContextKInvalidContext{
        kInvalidContext
    }
    pub enum  LazyDeoptOnThrowPropagateException{
        PropagateException
    }

    pub struct NextType{}
    pub struct FastApiCallLoweringReducer<Next: TurboshaftReducer> {
        next: Next,
        isolate_: Isolate,
        factory_: Factory,
        _marker: PhantomData<NextType>
    }

    trait TurboshaftReducer{
        fn set_reducer_data(&mut self, reducer_data: ReducerData);
        fn reducer_data(&self) -> &ReducerData;
    }
    impl<Next: TurboshaftReducer> FastApiCallLoweringReducer<Next>{
        pub fn new(next: Next, isolate: Isolate) -> FastApiCallLoweringReducer<Next> {
           FastApiCallLoweringReducer{
               next,
               isolate_: isolate,
               factory_: isolate.factory().clone(),
               _marker: PhantomData
           }
        }
        fn reduce_fast_api_call(&mut self, frame_state: V<FrameState>, data_argument: V<Object>, context: V<Context>, arguments: &[OpIndex], parameters: &FastApiCallParameters, out_reps: &[RegisterRepresentation]) -> OpIndex{
            __.data().set_graph_has_lowered_fast_api_calls();

            let c_function = parameters.c_function.address;
            let c_signature = parameters.c_signature();
            let c_arg_count = c_signature.ArgumentCount();
            assert_eq!(c_arg_count, arguments.len());

            let mut handle_error = Label::new();
            let mut done = Label::new();
            let result = __.NewVariable(RegisterRepresentation::FromCTypeInfo(c_signature.ReturnInfo(), c_signature.GetInt64Representation()));

            let callee = __.ExternalConstant(ExternalReference::Create(
                c_function, ExternalReferenceConstantType::FAST_C_CALL));

            let mut args: Vec<OpIndex> = Vec::new();
            for i in 0..c_arg_count{
                let type_ = c_signature.ArgumentInfo(i);
                args.push(self.adapt_fast_call_argument(arguments[i], type_, &mut handle_error));
            }

            if !__.data().generating_unreachable_operations(){
                let mut builder = MachineSignature::Builder(
                    __.graph_zone(), 1,
                    c_arg_count as i32 + (if c_signature.HasOptions() { 1 } else { 0 }));

                builder.AddReturn(MachineType::TypeForCType(c_signature.ReturnInfo()));

                for i in 0..c_arg_count{
                    let type_ = c_signature.ArgumentInfo(i);
                    let machine_type = if type_.GetSequenceType() == CTypeInfoSequenceType::kScalar{
                        MachineType::TypeForCType(type_)
                    } else {
                        MachineType::AnyTagged()
                    };
                    builder.AddParam(machine_type);
                }

                let mut stack_slot: Option<OpIndex> = None;
                if c_signature.HasOptions() {
                    const K_ALIGN: usize = mem::align_of::<FastApiCallbackOptions>();
                    const K_SIZE: usize = mem::size_of::<FastApiCallbackOptions>();
                    assert_eq!(K_SIZE, mem::size_of::<usize>() * 2);
                    let slot = __.StackSlot(K_SIZE, K_ALIGN);
                    stack_slot = Some(slot);

                    //Isolate
                    __.StoreOffHeap(
                        slot,
                        __.ExternalConstant(ExternalReference::isolate_address()),
                        MemoryRepresentation::UintPtr(),
                        mem::offset_of!(FastApiCallbackOptions, isolate)
                    );

                    let data_argument_to_pass = __.AdaptLocalArgument(data_argument);
                    __.StoreOffHeap(slot, data_argument_to_pass, MemoryRepresentation::UintPtr(), mem::offset_of!(FastApiCallbackOptions, data));

                    args.push(slot);
                    builder.AddParam(MachineType{});
                }

                let call_descriptor = TSCallDescriptor::Create(
                    Linkage::GetSimplifiedCDescriptor(__.graph_zone(), builder.Get(), CallDescriptorNeedsFrameState::kNeedsFrameState),
                    CanThrow::kNo, LazyDeoptOnThrow::kNo, __.graph_zone());
                let c_call_result = self.wrap_fast_call(&call_descriptor, callee, frame_state, context, args.as_slice());

                let mut trigger_exception = Label::new();
                let exception = __.Load(
                    __.ExternalConstant(ExternalReference::Create(
                        IsolateAddressId::kExceptionAddress(), &self.isolate_)),
                    LoadOpKind::RawAligned,
                    MemoryRepresentation::UintPtr()
                );
                if !__.TaggedEqual(
                    exception,
                    __.HeapConstant(&self.factory_)
                ).is_true(){
                   // trigger_exception.goto();
                }

                let fast_call_result = self.convert_return_value(c_signature, c_call_result);
                __.SetVariable(result, fast_call_result);

                //done.goto(FastApiCallOp::kSuccessValue);
                //trigger_exception.bind();
                __.CallRuntime::<LazyDeoptOnThrowPropagateException>(&self.isolate_, frame_state, __.NoContextConstant(), LazyDeoptOnThrow::kNo, Vec::new());

                __.Unreachable();
            }

           /* if handle_error.bind(){
                __.SetVariable(result, self.default_return_value(c_signature));
                done.goto(FastApiCallOp::kFailureValue);
            }*/

            //done.bind(state);
            __.Tuple(FastApiCallOp::kSuccessValue as i32 as usize, __.GetVariable(result).0 as usize);

            OpIndex(0) //Placeholder
        }
        fn checked<T>(&self, result: V<Tuple<T, Word32>>, otherwise: &mut Label) -> V<T>{
            let result_state = __.template_Projection::<Word32>(result.0 as usize);
            if !__.Word32Equal(result_state, V(PhantomData)).is_true(){
                //otherwise.goto();
            }
            __.template_Projection::<T>(result.0 as usize)
        }
        fn adapt_fast_call_argument(&mut self, argument: OpIndex, arg_type: CTypeInfo, handle_error: &mut Label) -> OpIndex{
            match arg_type.GetSequenceType(){
                CTypeInfoSequenceType::kScalar => {
                    let flags = arg_type.GetFlags() as u8;
                    if flags & CTypeInfoFlags::kEnforceRangeBit as u8 != 0{
                        match arg_type.GetType() {
                            CTypeInfoType::kInt32 => {
                                let result = __.TryTruncateFloat64ToInt32(argument);
                                return self.checked::<i32>(result, handle_error);
                            }
                            CTypeInfoType::kUint32 => {
                                let result = __.TryTruncateFloat64ToUint32(argument);
                                return self.checked::<u32>(result, handle_error);
                            }
                            CTypeInfoType::kInt64 => {
                                let result = __.TryTruncateFloat64ToInt64(argument);
                                return self.checked::<i64>(result, handle_error);
                            }
                            CTypeInfoType::kUint64 => {
                                let result = __.TryTruncateFloat64ToUint64(argument);
                                return self.checked::<u64>(result, handle_error);
                            }
                            _ => {
                                //handle_error.goto();
                                return argument;
                            }
                        }
                    } else if flags & CTypeInfoFlags::kClampBit as u8 != 0{
                        return self.clamp_fast_call_argument(V(PhantomData), arg_type.GetType());
                    } else {
                        match arg_type.GetType() {
                            CTypeInfoType::kV8Value => {
                                return __.AdaptLocalArgument(V::Cast(argument));
                            }
                            CTypeInfoType::kFloat32 => {
                                return __.TruncateFloat64ToFloat32(argument);
                            }
                            CTypeInfoType::kPointer => {
                                if __.ObjectIsSmi(argument).is_true(){
                                    //handle_error.goto();
                                }
                                let mut done = Label::new();

                                if __.TaggedEqual(V(PhantomData), __.HeapConstant(&self.factory_)).is_true(){
                                    //done.goto(0);
                                }
                                if !__.TaggedEqual(__.LoadMapField(V::Cast(argument)), __.HeapConstant(&self.factory_)).is_true(){
                                    //handle_error.goto();
                                }
                                //done.goto(__.template_LoadField::<WordPtr>(V::Cast(argument), AccessBuilder::ForJSExternalObjectValue()));
                                OpIndex(0) //Placeholder
                            }
                            CTypeInfoType::kSeqOneByteString => {
                                if __.ObjectIsSmi(argument).is_true(){
                                    //handle_error.goto();
                                }
                                let argument_obj: V<HeapObject> = V::Cast(argument);
                                let map = __.LoadMapField(argument_obj);
                                let instance_type = __.LoadInstanceTypeField(map);
                                let encoding = __.Word32BitwiseAnd(instance_type, kStringRepresentationAndEncodingMask);
                                if !__.Word32Equal(encoding, kSeqOneByteStringTag).is_true(){
                                    //handle_error.goto();
                                }

                                let length_in_bytes = __.template_LoadField::<WordPtr>(argument_obj, AccessBuilder::ForStringLength());
                                let data_ptr = __.GetElementStartPointer(argument_obj, AccessBuilder::ForSeqOneByteStringCharacter());

                                const K_ALIGN: usize = mem::align_of::<FastOneByteString>();
                                const K_SIZE: usize = mem::size_of::<FastOneByteString>();
                                assert_eq!(K_SIZE, mem::size_of::<usize>() + mem::size_of::<usize>());
                                let stack_slot = __.StackSlot(K_SIZE, K_ALIGN);
                                __.StoreOffHeap(stack_slot, data_ptr, MemoryRepresentation::UintPtr());
                                __.StoreOffHeap(stack_slot, length_in_bytes, MemoryRepresentation::Uint32(), mem::size_of::<usize>());
                                return stack_slot;
                            }
                            _ => {
                                return argument;
                            }
                        }
                    }
                }
                CTypeInfoSequenceType::kIsSequence => {
                    assert_eq!(arg_type.GetType(), CTypeInfoType::kVoid);
                    if __.ObjectIsSmi(argument).is_true(){
                        //handle_error.goto();
                    }

                    let map = __.LoadMapField(V::Cast(argument));
                    let instance_type = __.LoadInstanceTypeField(map);
                    if !__.Word32Equal(instance_type, JS_ARRAY_TYPE).is_true(){
                        //handle_error.goto();
                    }
                    return __.AdaptLocalArgument(V::Cast(argument));
                }
            }
            unreachable!()
        }

        fn clamp_fast_call_argument(&mut self, argument: V<Float64>, scalar_type: CTypeInfoType) -> OpIndex{
            let (min, max) = match scalar_type {
                CTypeInfoType::kInt32 => (std::i32::MIN as f64, std::i32::MAX as f64),
                CTypeInfoType::kUint32 => (0.0, std::u32::MAX as f64),
                CTypeInfoType::kInt64 => (kMinSafeInteger, kMaxSafeInteger),
                CTypeInfoType::kUint64 => (0.0, kMaxSafeInteger),
                _ => unreachable!(),
            };

            let clamped = if __.Float64LessThan(V(PhantomData), argument).is_true(){
                if __.Float64LessThan(argument, V(PhantomData)).is_true(){
                    argument
                } else {
                    __.Float64Constant(max)
                }
            } else {
                __.Float64Constant(min)
            };

            let mut done = Label::new();
            let rounded = __.Float64RoundTiesEven(clamped);
            if __.Float64IsNaN(rounded).is_true(){
                //done.goto(0.0);
            }
            //done.goto(rounded);
            match scalar_type{
                CTypeInfoType::kInt32 => __.ReversibleFloat64ToInt32(V(PhantomData)),
                CTypeInfoType::kUint32 => __.ReversibleFloat64ToUint32(V(PhantomData)),
                CTypeInfoType::kInt64 => __.ReversibleFloat64ToInt64(V(PhantomData)),
                CTypeInfoType::kUint64 => __.ReversibleFloat64ToUint64(V(PhantomData)),
                _ => unreachable!(),
            }
        }
        fn default_return_value(&self, c_signature: &dyn CFunctionInfo) -> V<Any>{
            match c_signature.ReturnInfo().GetType() {
                CTypeInfoType::kVoid => __.HeapConstant(&self.factory_),
                CTypeInfoType::kBool | CTypeInfoType::kInt32 | CTypeInfoType::kUint32 => V(PhantomData),
                CTypeInfoType::kInt64 | CTypeInfoType::kUint64 => {
                    let repr = c_signature.GetInt64Representation();
                    match repr{
                        CFunctionInfoInt64Representation::kBigInt => {
                            V(PhantomData)
                        }
                        CFunctionInfoInt64Representation::kNumber => {
                            __.Float64Constant(0.0)
                        }
                    }
                }
                CTypeInfoType::kFloat32 => __.Float64Constant(0.0),
                CTypeInfoType::kFloat64 => __.Float64Constant(0.0),
                CTypeInfoType::kPointer => __.HeapConstant(&self.factory_),
                _ => unreachable!(),
            }
        }
        fn convert_return_value(&self, c_signature: &dyn CFunctionInfo, result: OpIndex) -> V<Any>{
            match c_signature.ReturnInfo().GetType() {
                CTypeInfoType::kVoid => __.HeapConstant(&self.factory_),
                CTypeInfoType::kBool => __.Word32BitwiseAnd(V(PhantomData), Word32{}),
                CTypeInfoType::kInt32 | CTypeInfoType::kUint32 | CTypeInfoType::kFloat32 | CTypeInfoType::kFloat64 => V(PhantomData),
                CTypeInfoType::kInt64 => {
                    let repr = c_signature.GetInt64Representation();
                    match repr{
                        CFunctionInfoInt64Representation::kBigInt => V(PhantomData),
                        CFunctionInfoInt64Representation::kNumber => __.ChangeInt64ToFloat64(result)
                    }
                }
                CTypeInfoType::kUint64 => {
                    let repr = c_signature.GetInt64Representation();
                    match repr{
                        CFunctionInfoInt64Representation::kBigInt => V(PhantomData),
                        CFunctionInfoInt64Representation::kNumber => __.ChangeUint64ToFloat64(result)
                    }
                }
                CTypeInfoType::kPointer => self.build_allocate_js_external_object(V(PhantomData)),
                _ => unreachable!(),
            }
        }
        fn build_allocate_js_external_object(&self, pointer: V<WordPtr>) -> V<HeapObject>{
            let mut done = Label::new();

            if __.WordPtrEqual(pointer, WordPtr{}).is_true(){
                //done.goto(__.HeapConstant(&self.factory_));
            }

            let external: Uninitialized<HeapObject> = __.Allocate(JSExternalObject::kHeaderSize, AllocationType::kYoung);
            __.InitializeField(external, AccessBuilder::ForMap(), __.HeapConstant(&self.factory_));
            let empty_fixed_array = __.HeapConstant(&self.factory_);
            __.InitializeField(external, AccessBuilder::ForJSObjectPropertiesOrHash(), empty_fixed_array);
            __.InitializeField(external, AccessBuilder::ForJSObjectElements(), empty_fixed_array);

            //ifdef V8_ENABLE_SANDBOX
            let isolate_ptr = __.ExternalConstant(ExternalReference::isolate_address());
            let mut builder = MachineSignature::Builder(__.graph_zone(), 1, 2);
            builder.AddReturn(MachineType{});
            builder.AddParam(MachineType{});
            builder.AddParam(MachineType{});
            let allocate_and_initialize_young_external_pointer_table_entry = __.ExternalConstant(ExternalReference{reference:0, constant_type: ExternalReferenceConstantType::OTHER});
            let call_descriptor = Linkage::GetSimplifiedCDescriptor(__.graph_zone(), builder.Get(), CallDescriptorNeedsFrameState::kNeedsFrameState);
            //let handle = __.Call(allocate_and_initialize_young_external_pointer_table_entry, vec![isolate_ptr, V(PhantomData)], TSCallDescriptor::