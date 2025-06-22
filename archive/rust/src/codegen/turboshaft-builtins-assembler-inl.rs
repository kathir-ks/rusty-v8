// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod turboshaft_builtins_assembler {
    use std::{
        marker::PhantomData,
        mem,
        ops::{BitAnd, BitOr, Not},
        ptr::NonNull,
    };

    // use crate::common::globals::*; // Assuming globals.h translates to this
    // use crate::compiler::turboshaft::access_builder::*;
    // use crate::compiler::turboshaft::assembler::*;
    // use crate::compiler::turboshaft::machine_lowering_reducer::*;
    // use crate::compiler::turboshaft::operation_matcher::*;
    // use crate::compiler::turboshaft::runtime_call_descriptors::*;
    // use crate::compiler::turboshaft::sidetable::*;
    // use crate::interpreter::bytecode_register::*;
    // use crate::objects::elements_kind::*;

    // Dummy definitions, replace with actual Rust equivalents
    pub type Word32 = u32;
    pub type Word64 = u64;
    pub type WordPtr = usize;
    pub type Float32 = f32;
    pub type Float64 = f64;
    pub type Object = usize;
    pub type Context = usize;
    pub type MaybeObject = usize;
    pub type HeapObject = usize;
    pub type FeedbackVector = usize;
    pub type BigInt = usize;
    pub type JSAnyNotNumber = usize;
    pub type Map = usize;
    pub type HeapNumber = usize;
    pub type Oddball = usize;
    pub type Smi = usize;
    pub type Undefined = usize;

    pub type ElementsKind = u32;
    pub const HOLEY_ELEMENTS: ElementsKind = 0;
    pub const SYSTEM_POINTER_ELEMENTS: ElementsKind = 0;

    pub type RegisterRepresentation = u32;
    pub type MemoryRepresentation = u32;

    pub struct BuiltinCallDescriptor {}
    impl BuiltinCallDescriptor {
        pub const ReThrow: Self = BuiltinCallDescriptor {};
        pub const NonNumberToNumeric: Self = BuiltinCallDescriptor {};
        pub const NonNumberToNumber: Self = BuiltinCallDescriptor {};
    }

    pub struct AccessBuilderTS {}
    impl AccessBuilderTS {
        pub fn ForFeedbackVectorLength() -> Self {
            AccessBuilderTS {}
        }
        pub fn ForHeapNumberOrOddballOrHoleValue() -> Self {
            AccessBuilderTS {}
        }
    }

    pub struct PipelineData {}
    pub struct Graph {}

    pub struct Zone {}

    pub struct CallDescriptor {}
    impl CallDescriptor {
        pub fn GetJSCallContextParamIndex(_arg: i32) -> i32 {
            0
        }
    }

    pub mod Linkage {
        pub fn GetJSCallContextParamIndex(_arg: i32) -> i32 {
            0
        }
    }

    pub struct FrameState {}

    pub struct JSStackCheckOp {}
    impl JSStackCheckOp {
        pub const Kind: JSStackCheckOpKind = JSStackCheckOpKind {};
    }

    pub struct JSStackCheckOpKind {}
    impl JSStackCheckOpKind {
        pub const kBuiltinEntry: u32 = 0;
    }

    pub type Builtin = u32;
    pub mod Builtins {
        pub const TSJ: u32 = 0;
        pub fn KindOf(_arg: Builtin) -> Builtin {
            0
        }
    }

    pub const BIGINT_TYPE: Word32 = 0;
    pub const ODDBALL_TYPE: Word32 = 0;

    pub type BinaryOperationFeedback = u32;
    pub mod BinaryOperationFeedback {
        pub const kSignedSmall: u32 = 0;
        pub const kNumber: u32 = 0;
        pub const kBigInt64: u32 = 0;
        pub const kBigInt: u32 = 0;
        pub const kNumberOrOddball: u32 = 0;
        pub const kAny: u32 = 0;
        pub const kNone: u32 = 0;
    }

    pub type Isolate = usize;
    pub type WriteBarrierMode = u32;
    pub const SKIP_WRITE_BARRIER: WriteBarrierMode = 0;
    pub const UPDATE_WRITE_BARRIER: WriteBarrierMode = 1;
    pub const UNSAFE_SKIP_WRITE_BARRIER: WriteBarrierMode = 2;
    pub const UPDATE_EPHEMERON_KEY_WRITE_BARRIER: WriteBarrierMode = 3;

    pub enum UpdateFeedbackMode {
        kNoFeedback,
        kOptionalFeedback,
        kGuaranteedFeedback,
    }

    pub mod interpreter {
        pub mod Register {
            pub fn feedback_vector() -> usize {
                0
            }
        }
    }

    pub mod base {
        pub struct VectorOf<T>(Vec<T>);

        impl<T> VectorOf<T> {
            pub fn new(data: Vec<T>) -> Self {
                VectorOf(data)
            }
        }

        impl<T> From<Vec<T>> for VectorOf<T> {
            fn from(data: Vec<T>) -> Self {
                VectorOf(data)
            }
        }

        impl<T> AsRef<[T]> for VectorOf<T> {
            fn as_ref(&self) -> &[T] {
                &self.0
            }
        }
    }

    pub fn ElementsKindToShiftSize(_kind: ElementsKind) -> usize {
        0
    }

    pub const kObjectAlignmentMask: WordPtr = 0;
    pub const kTaggedSize: usize = 8;
    pub const kSystemPointerSize: usize = 8;

    pub struct StandardFrameConstants {}
    impl StandardFrameConstants {
        pub const kFixedSlotCountAboveFp: i32 = 0;
    }

    #[macro_export]
    macro_rules! DEFINE_TURBOSHAFT_ALIASES {
        () => {};
    }

    #[macro_export]
    macro_rules! BUILTIN_REDUCER {
        ($name:ident) => {
            $crate::turboshaft_builtins_assembler::TURBOSHAFT_REDUCER_BOILERPLATE!($name);
            $crate::turboshaft_builtins_assembler::DEFINE_TURBOSHAFT_ALIASES!();
        };
    }

    #[macro_export]
    macro_rules! TURBOSHAFT_REDUCER_BOILERPLATE {
        ($name:ident) => {};
    }

    pub enum IsKnownTaggedPointer {
        kNo,
        kYes,
    }

    pub mod detail {
        use super::*;

        pub struct BuiltinArgumentsTS<'a, Assembler> {
            assembler_: &'a Assembler,
            argc_: WordPtr,
            fp_: WordPtr,
            base_: WordPtr,
            _phantom: PhantomData<Assembler>,
        }

        impl<'a, Assembler> BuiltinArgumentsTS<'a, Assembler> {
            pub fn new<T>(assembler: &'a Assembler, argc: T, fp: Option<WordPtr>) -> Self
            where
                T: Into<WordPtr> + Copy,
            {
                //This is where the logic for different types of argc and WordPtr will be
                // implemented. Since Rust does not have implicit type conversion, it is necessary
                // to use `into()` and `copy()`
                let argc = argc.into();

                let fp_val = match fp {
                    Some(fp_val) => fp_val,
                    None => 0, // TODO: Replace with actual frame pointer logic `__ FramePointer()`
                };

                let offset =
                    (StandardFrameConstants::kFixedSlotCountAboveFp + 1) as usize * kSystemPointerSize;

                BuiltinArgumentsTS {
                    assembler_: assembler,
                    argc_: argc,
                    fp_: fp_val,
                    // base_ points to the first argument, not the receiver
                    // whether present or not.
                    base_: fp_val + offset, //TODO: Replace WordPtrAdd
                    _phantom: PhantomData,
                }
            }

            pub fn Asm(&self) -> &Assembler {
                self.assembler_
            }

            pub fn GetLengthWithReceiver(&self) -> WordPtr {
                self.argc_
            }

            pub fn GetLengthWithoutReceiver(&self) -> WordPtr {
                self.argc_ - 0 //TODO: Replace `kJSArgcReceiverSlots`
            }

            pub fn AtIndex(&self, index: WordPtr) -> Object {
                // TSA_DCHECK(this, __ UintPtrLessThan(index, GetLengthWithoutReceiver()));
                self.LoadOffHeapAtIndexPtr(index, MemoryRepresentation::AnyTagged()) //TODO: Replace with actual `LoadOffHeap` call
            }

            //Placeholder, replace with actual logic
            fn LoadOffHeapAtIndexPtr(&self, _index: WordPtr, _memory_representation: MemoryRepresentation) -> Object {
                0
            }

            fn AtIndexPtr(&self, index: WordPtr) -> WordPtr {
                let offset = self.ElementOffsetFromIndex(index, SYSTEM_POINTER_ELEMENTS, 0);
                self.WordPtrAdd(self.base_, offset)
            }

            fn ElementOffsetFromIndex(&self, index: WordPtr, kind: ElementsKind, base_size: i32) -> WordPtr {
                let element_size_shift = ElementsKindToShiftSize(kind);

                (base_size as usize) + (1 << element_size_shift) * index
            }

            fn WordPtrAdd(&self, a: WordPtr, b: WordPtr) -> WordPtr {
                a + b
            }

            pub struct Iterator<'b, 'a> {
                args_: &'b BuiltinArgumentsTS<'a, Assembler>,
                begin_index_: WordPtr,
                end_index_: WordPtr,
                end_offset_: WordPtr,
                _phantom: PhantomData<&'a Assembler>,
            }

            impl<'b, 'a, Assembler> Iterator<'b, 'a> {
                // {end} is the iterator-typical exclusive one past the last element.
                pub fn new(args: &'b BuiltinArgumentsTS<'a, Assembler>, begin_index: WordPtr, end_index: WordPtr) -> Self {
                    Iterator {
                        args_: args,
                        begin_index_: begin_index,
                        end_index_: end_index,
                        end_offset_: 0,
                        _phantom: PhantomData,
                    }
                }

                pub fn Range(args: &'b BuiltinArgumentsTS<'a, Assembler>, begin: WordPtr, end: WordPtr) -> Self {
                    Iterator::new(args, begin, end)
                }

                pub fn Begin(&mut self) -> WordPtr {
                    self.end_offset_ = self.args_.AtIndexPtr(self.end_index_);
                    self.args_.AtIndexPtr(self.begin_index_)
                }

                //TODO: replace placeholder logic
                pub fn IsEnd(&self, current_iterator: WordPtr) -> Option<Word32> {
                    if self.UintPtrLessThanOrEqual(self.end_offset_, current_iterator) {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }

                //TODO: replace placeholder logic
                fn UintPtrLessThanOrEqual(&self, a: WordPtr, b: WordPtr) -> bool {
                    a <= b
                }

                //TODO: replace placeholder logic
                pub fn Advance(&self, current_iterator: WordPtr) -> WordPtr {
                    self.args_.WordPtrAdd(
                        current_iterator,
                        ElementsKindToByteSize(SYSTEM_POINTER_ELEMENTS) as WordPtr,
                    )
                }

                //TODO: replace placeholder logic
                pub fn Dereference(&self, current_iterator: WordPtr) -> Object {
                    self.args_.LoadOffHeapAtIterator(
                        current_iterator,
                        MemoryRepresentation::AnyTagged(),
                    )
                }

                //Placeholder, replace with actual logic
                fn LoadOffHeapAtIterator(&self, _index: WordPtr, _memory_representation: MemoryRepresentation) -> Object {
                    0
                }
            }

            pub fn Range(&self, begin: WordPtr, end: WordPtr) -> Iterator<'_, 'a> {
                Iterator::Range(self, begin, end)
            }

            pub fn RangeFrom(&self, begin: WordPtr) -> Iterator<'_, 'a> {
                Iterator::new(self, begin, self.GetLengthWithoutReceiver())
            }

            pub fn RangeAll(&self) -> Iterator<'_, 'a> {
                Iterator::new(self, 0, self.GetLengthWithoutReceiver())
            }
        }

        fn ElementsKindToByteSize(_kind: ElementsKind) -> usize {
            8
        }
    }

    trait ReducerTrait<Assembler> {
        fn Asm(&self) -> &Assembler;
    }

    pub struct FeedbackCollectorReducer<Next, Assembler> {
        next: Next,
        feedback_: usize,
        feedback_on_exception_: usize,
        slot_id_: WordPtr,
        maybe_feedback_vector_: MaybeObject,
        mode_: UpdateFeedbackMode,
        _phantom: PhantomData<Assembler>,
    }

    impl<Next, Assembler> FeedbackCollectorReducer<Next, Assembler> {
        pub fn new(next: Next) -> Self {
            FeedbackCollectorReducer {
                next,
                feedback_: 0,
                feedback_on_exception_: 0,
                slot_id_: 0,
                maybe_feedback_vector_: 0,
                mode_: UpdateFeedbackMode::kOptionalFeedback,
                _phantom: PhantomData,
            }
        }

        const fn HasFeedbackCollector() -> bool {
            true
        }

        fn CombineFeedback(&mut self, additional_feedback: i32) {
            // __ CodeComment("CombineFeedback");
            self.feedback_ = self.SmiBitwiseOr(
                self.feedback_,
                self.SmiConstant(additional_feedback),
            );
        }

        fn OverwriteFeedback(&mut self, new_feedback: i32) {
            // __ CodeComment("OverwriteFeedback");
            self.feedback_ = self.SmiConstant(new_feedback);
        }

        fn CombineFeedbackOnException(&mut self, additional_feedback: i32) {
            self.feedback_on_exception_ = self.SmiConstant(additional_feedback);
        }

        fn CombineExceptionFeedback(&mut self) {
            self.feedback_ = self.SmiBitwiseOr(self.feedback_, self.feedback_on_exception_);
        }

        fn FeedbackIs(&self, checked_feedback: i32) -> Word32 {
            self.SmiEqual(self.feedback_, self.SmiConstant(checked_feedback))
        }

        fn LoadFeedbackVector(&self) -> MaybeObject {
            // Assuming LoadRegister and interpreter::Register::feedback_vector() are defined elsewhere
            0 // Replace with actual implementation
        }

        fn LoadFeedbackVectorLength(&self, feedback_vector: FeedbackVector) -> WordPtr {
            let length: Word32 = self.LoadField(
                feedback_vector,
                AccessBuilderTS::ForFeedbackVectorLength(),
            );
            self.ChangePositiveInt32ToIntPtr(length)
        }

        fn LoadFeedbackVectorSlot(
            &self,
            feedback_vector: FeedbackVector,
            slot: WordPtr,
            additional_offset: i32,
        ) -> MaybeObject {
            // __ CodeComment("LoadFeedbackVectorSlot");
            let header_size = 0 + additional_offset; // FeedbackVector::kRawFeedbackSlotsOffset;

            let offset = self.ElementOffsetFromIndex(slot, HOLEY_ELEMENTS, header_size as isize);

            // TSA_SLOW_DCHECK(this, IsOffsetInBounds(
            //     offset, LoadFeedbackVectorLength(feedback_vector),
            //     FeedbackVector::kHeaderSize));

            self.Load(feedback_vector, offset, MemoryRepresentation::AnyTagged())
        }

        fn StoreFeedbackVectorSlot(
            &mut self,
            feedback_vector: FeedbackVector,
            slot: WordPtr,
            value: Object,
            barrier_mode: WriteBarrierMode,
            additional_offset: i32,
        ) {
            // __ CodeComment("StoreFeedbackVectorSlot");
            //DCHECK(IsAligned(additional_offset, kTaggedSize));
            let header_size = 0 + additional_offset; //FeedbackVector::kRawFeedbackSlotsOffset;
            let offset = self.ElementOffsetFromIndex(slot, HOLEY_ELEMENTS, header_size as isize);
            //TSA_DCHECK(this, IsOffsetInBounds(offset,
            //                                  LoadFeedbackVectorLength(feedback_vector),
            //                                  FeedbackVector::kHeaderSize));

            match barrier_mode {
                SKIP_WRITE_BARRIER => {
                    self.Store(
                        feedback_vector,
                        offset,
                        value,
                        MemoryRepresentation::AnyTagged(),
                    );
                    return;
                }
                UNSAFE_SKIP_WRITE_BARRIER => {
                    panic!("UNIMPLEMENTED");
                }
                UPDATE_WRITE_BARRIER => {
                    panic!("UNIMPLEMENTED");
                }
                UPDATE_EPHEMERON_KEY_WRITE_BARRIER => {
                    panic!("UNREACHABLE");
                }
                _ => {
                    panic!("Unknown WriteBarrierMode");
                }
            }
        }

        fn SetFeedbackSlot(&mut self, slot_id: WordPtr) {
            self.slot_id_ = slot_id;
        }

        fn SetFeedbackVector(&mut self, feedback_vector: FeedbackVector) {
            //TSA_DCHECK(this, IsFeedbackVector(feedback_vector));
            self.maybe_feedback_vector_ = feedback_vector;
            self.feedback_ = self.SmiConstant(0);
            self.feedback_on_exception_ = self.feedback_;
        }

        fn LoadFeedbackVectorOrUndefinedIfJitless(&mut self) {
            self.maybe_feedback_vector_ = self.LoadFeedbackVector(); //Replace with actual logic
            self.feedback_ = self.SmiConstant(0);
            self.feedback_on_exception_ = self.feedback_;
        }

        const fn DefaultUpdateFeedbackMode() -> UpdateFeedbackMode {
            UpdateFeedbackMode::kOptionalFeedback
        }

        fn UpdateFeedback(&mut self) {
            if let UpdateFeedbackMode::kNoFeedback = self.mode_ {
                // TSA_DCHECK(this, __ IsUndefined(maybe_feedback_vector_));
                return;
            }

            if let UpdateFeedbackMode::kOptionalFeedback = self.mode_ {
                if self.maybe_feedback_vector_ == 0 {
                    //GOTO_IF(__ IsUndefined(maybe_feedback_vector_), done);
                    return;
                }
            } else {
                assert!(matches!(self.mode_, UpdateFeedbackMode::kGuaranteedFeedback));
            }

            let feedback_vector = self.maybe_feedback_vector_; // V<FeedbackVector>::Cast(maybe_feedback_vector_);

            let feedback_element =
                self.LoadFeedbackVectorSlot(feedback_vector, self.slot_id_, 0);
            let previous_feedback = feedback_element as usize; //V<Smi>::Cast(feedback_element);
            let combined_feedback = self.SmiBitwiseOr(previous_feedback, self.feedback_);
            if previous_feedback != combined_feedback {
                self.StoreFeedbackVectorSlot(
                    feedback_vector,
                    self.slot_id_,
                    combined_feedback,
                    SKIP_WRITE_BARRIER,
                    0,
                );
                // TODO(nicohartmann):
                // ReportFeedbackUpdate(maybe_feedback_vector_, slot_id_,
                // "UpdateFeedback");
            }
        }

        fn SmiBitwiseOr(&self, a: usize, b: usize) -> usize {
            a | b
        }

        fn SmiEqual(&self, a: usize, b: usize) -> Word32 {
            if a == b {
                1
            } else {
                0
            }
        }

        fn ChangePositiveInt32ToIntPtr(&self, input: Word32) -> WordPtr {
            //TSA_DCHECK(this, __ Int32LessThanOrEqual(0, input));
            input as WordPtr
        }

        fn IsFeedbackVector(&self, _heap_object: HeapObject) -> Word32 {
            1 //Replace with actual logic
        }

        fn IsOffsetInBounds(
            &self,
            offset: WordPtr,
            length: WordPtr,
            header_size: i32,
            kind: ElementsKind,
        ) -> Word32 {
            // Make sure we point to the last field.
            let element_size = 1 << ElementsKindToShiftSize(kind);
            let correction = header_size as usize - element_size;
            let last_offset = self.ElementOffsetFromIndex(length, kind, correction as isize);
            if self.IntPtrLessThanOrEqual(offset, last_offset) {
                1
            } else {
                0
            }
        }

        fn ElementOffsetFromIndex(&self, index: WordPtr, kind: ElementsKind, base_size: isize) -> WordPtr {
            let element_size_shift = ElementsKindToShiftSize(kind);
            (base_size as usize) + (1 << element_size_shift) * index
        }

        fn IntPtrLessThanOrEqual(&self, a: WordPtr, b: WordPtr) -> bool {
            a <= b
        }

        fn SmiConstant(&self, value: i32) -> usize {
            value as usize
        }

        fn LoadField<T>(&self, _base: usize, _access: AccessBuilderTS) -> T {
            0 as T
        }

        fn Load(&self, _base: usize, _offset: WordPtr, _representation: MemoryRepresentation) -> MaybeObject {
            0
        }

        fn Store(&mut self, _base: usize, _offset: WordPtr, _value: Object, _representation: MemoryRepresentation) {}
    }

    impl<Next, Assembler> ReducerTrait<Assembler> for FeedbackCollectorReducer<Next, Assembler> {
        fn Asm(&self) -> &Assembler {
            todo!()
        }
    }

    pub struct NoFeedbackCollectorReducer<Next, Assembler> {
        next: Next,
        _phantom: PhantomData<Assembler>,
    }

    impl<Next, Assembler> NoFeedbackCollectorReducer<Next, Assembler> {
        pub fn new(next: Next) -> Self {
            NoFeedbackCollectorReducer {
                next,
                _phantom: PhantomData,
            }
        }

        const fn HasFeedbackCollector() -> bool {
            false
        }

        fn CombineFeedback(&mut self, _additional_feedback: i32) {}

        fn OverwriteFeedback(&mut self, _new_feedback: i32) {}

        fn FeedbackIs(&self, _checked_feedback: i32) -> Word32 {
            panic!("UNREACHABLE");
        }

        fn UpdateFeedback(&mut self) {}
        fn CombineExceptionFeedback(&mut self) {}
    }

    impl<Next, Assembler> ReducerTrait<Assembler> for NoFeedbackCollectorReducer<Next, Assembler> {
        fn Asm(&self) -> &Assembler {
            todo!()
        }
    }

    pub struct BuiltinsReducer<Next, Assembler> {
        next: Next,
        data_: Box<BuiltinsReducerData>,
        matcher_: OperationMatcher,
        _phantom: PhantomData<Assembler>,
    }

    pub struct BuiltinsReducerData {
        builtin_call_descriptor_: BuiltinCallDescriptor,
        isolate_: Isolate,
        graph_: Graph,
    }

    impl<Next, Assembler> BuiltinsReducer<Next, Assembler> {
        pub fn new(next: Next, data_: BuiltinsReducerData, graph_: Graph) -> Self {
            BuiltinsReducer {
                next,
                data_: Box::new(data_),
                matcher_: OperationMatcher { graph_: &graph_ },
                _phantom: PhantomData,
            }
        }

        pub fn EmitBuiltinProlog(&self, _builtin_id: Builtin) {
            // Bind the entry block.
            // __ Bind(__ NewBlock());
            // Eagerly emit all parameters such that they are guaranteed to be in the
            // entry block (assembler will cache them).
            // const compiler::CallDescriptor* desc =
            //     __ data() -> builtin_call_descriptor();
            // for (int i = 0; i < static_cast<int>(desc->ParameterCount()); ++i) {
            //   __ Parameter(i, RegisterRepresentation::FromMachineType(
            //                       desc->GetParameterType(i)));
            // }
            // TODO(nicohartmann): CSA tracks some debug information here.
            // Emit stack check.
            // if (Builtins::KindOf(builtin_id) == Builtins::TSJ) {
            //   __ PerformStackCheck(__ JSContextParameter());
            // }
        }

        pub fn EmitEpilog(&self, _catch_block: Option<usize>) {
            //DCHECK_EQ(__ HasFeedbackCollector(), catch_block != nullptr);
            // if (catch_block) {
            //   // If the handler can potentially throw, we catch the exception here and
            //   // update the feedback vector before we rethrow the exception.
            //   if (__ Bind(catch_block)) {
            //     V<Object> exception = __ CatchBlockBegin();
            //     __ CombineExceptionFeedback();
            //     __ UpdateFeedback();
            //     __ template CallRuntime<
            //         compiler::turboshaft::RuntimeCallDescriptor::ReThrow>(
            //         __ data()->isolate(), __ NoContextConstant(), {exception});
            //     __ Unreachable();
            //   }
            // }
        }

        pub fn JSContextParameter(&self) -> Context {
            0 // Replace with actual implementation
        }

        pub fn PerformStackCheck(&self, _context: Context) {
            // __ JSStackCheck(context,
            //                 OptionalV<compiler::turboshaft::FrameState>::Nullopt(),
            //                 compiler::turboshaft::JSStackCheckOp::Kind::kBuiltinEntry);
        }

        pub fn PopAndReturn(&self, _arguments: &detail::BuiltinArgumentsTS<'_, Assembler>, _return_value: Object) {
            // PopAndReturn is supposed to be using ONLY in CSA/Torque builtins for
            // dropping ALL JS arguments that are currently located on the stack.
            // The check below ensures that there are no directly accessible stack
            // parameters from current builtin, which implies that the builtin with
            // JS calling convention (TFJ) was created with kDontAdaptArgumentsSentinel.
            // This simplifies semantics of this instruction because in case of presence
            // of directly accessible stack parameters it's impossible to distinguish
            // the following cases:
            // 1) stack parameter is included in JS arguments (and therefore it will be
            //    dropped as a part of 'pop' number of arguments),
            // 2) stack parameter is NOT included in JS arguments (and therefore it
            // should
            //    be dropped in ADDITION to the 'pop' number of arguments).
            // Additionally, in order to simplify assembly code, PopAndReturn is also
            // not allowed in builtins with stub linkage and parameters on stack.
            // CHECK_EQ(__ data()->builtin_call_descriptor()->ParameterSlotCount(), 0);
            // let pop_count = arguments.GetLengthWithReceiver();
            //std::initializer_list<const OpIndex> temp{return_value};
            //__ Return(__ TruncateWordPtrToWord32(pop_count), base::VectorOf(temp));
        }

        fn data(&self) -> &BuiltinsReducerData {
            &self.data_
        }

        pub fn TruncateTaggedToWord32(&self, _context: Context, _value: Object) -> Word32 {
            0
        }

        pub fn IsBigIntInstanceType(&self, instance_type: Word32) -> Word32 {
            self.InstanceTypeEqual(instance_type, BIGINT_TYPE)
        }

        pub fn IsSmallBigInt(&self, _value: BigInt) -> Word32 {
            panic!("UNIMPLEMENTED")
        }
        pub fn InstanceTypeEqual(&self, instance_type: Word32, other_instance_type: Word32) -> Word32 {
            if instance_type == other_instance_type {
                1
            } else {
                0
            }
        }

        pub fn AlignTagged(&self, size: WordPtr) -> WordPtr {
            (size + kObjectAlignmentMask) & (!kObjectAlignmentMask)
        }

        pub fn ElementOffsetFromIndex(&self, index: WordPtr, kind: ElementsKind, base_size: i32) -> WordPtr {
            let element_size_shift = ElementsKindToShiftSize(kind);

            (base_size as usize) + (1 << element_size_shift) * index
        }

        //TODO: implement matcher
        pub fn TryToIntPtrConstant(&self, _index: WordPtr) -> Option<WordPtr> {
            None
        }

        //TODO: Implement actual function body
        pub fn TaggedToWord32OrBigIntImpl(&self, _context: Context, _value: Object, _is_known_tagged_pointer: IsKnownTaggedPointer) {}
    }

    impl<Next, Assembler> ReducerTrait<Assembler> for BuiltinsReducer<Next, Assembler> {
        fn Asm(&self) -> &Assembler {
            todo!()
        }
    }

    struct OperationMatcher {
        graph_: &Graph,
    }

    impl OperationMatcher {
        fn MatchIntegralWordPtrConstant(&self, _op_index: usize, _value: &mut i64) -> bool {
            false
        }
    }

    fn ElementsKindToByteSize(_kind: ElementsKind) -> usize {
        8
    }

    pub struct TSAssembler<Reducer, BuiltinsReducer, FeedbackReducer, MachineLoweringReducer, VariableReducer> {
        _phantom: PhantomData<(
            Reducer,
            BuiltinsReducer,
            FeedbackReducer,
            MachineLoweringReducer,
            VariableReducer,
        )>,
    }

    impl<Reducer, BuiltinsReducer, FeedbackReducer, MachineLoweringReducer, VariableReducer>
        TSAssembler<Reducer, BuiltinsReducer, FeedbackReducer, MachineLoweringReducer, VariableReducer>
    {
        pub fn new(
            _data: &PipelineData,
            _graph: &Graph,
            _graph2: &Graph,
            _phase_zone: &Zone,
        ) -> Self {
            TSAssembler {
                _phantom: PhantomData,
            }
        }
    }
}