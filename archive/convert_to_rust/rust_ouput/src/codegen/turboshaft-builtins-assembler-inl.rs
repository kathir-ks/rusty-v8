// Converted from V8 C++ source files:
// Header: turboshaft-builtins-assembler-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    pub mod compiler {
        pub mod turboshaft {
            use std::cell::RefCell;
            use std::rc::Rc;

            pub struct V<T> {
                _phantom: std::marker::PhantomData<T>,
            }

            impl<T> V<T> {
                pub fn Cast(_: Self) -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                    }
                }
                pub fn Get(&self) -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                    }
                }
            }

            pub struct ConstOrV<T> {
                value: Option<T>,
                v: Option<V<T>>,
            }

            impl<T> ConstOrV<T> {
                pub fn constant_value(&self) -> Option<T>
                where
                    T: Copy,
                {
                    self.value
                }
                pub fn value(&self) -> &V<T> {
                    self.v.as_ref().unwrap()
                }

                pub fn is_constant(&self) -> bool {
                    self.value.is_some()
                }
            }

            pub struct OptionalV<T> {
                value: Option<V<T>>,
            }

            impl<T> OptionalV<T> {
                pub fn has_value(&self) -> bool {
                    self.value.is_some()
                }
                pub fn value(&self) -> V<T> {
                    self.value.as_ref().unwrap().clone()
                }
                 pub fn Nullopt() -> Self {
                    Self { value: None }
                }
            }

            pub struct Label<T> {
                _phantom: std::marker::PhantomData<T>,
            }
            impl<T> Label<T> {
                pub fn valid(&self) -> bool {
                    true
                }
            }

            pub struct LoopLabel<T> {
                _phantom: std::marker::PhantomData<T>,
            }

            pub struct Block {}

            pub struct OpIndex {}

            pub struct Word32 {}

            pub struct Word64 {}

            pub struct WordPtr {}

            pub struct Float32 {}

            pub struct Float64 {}

            pub enum RegisterRepresentation {}

            pub enum MemoryRepresentation {}

            pub struct BuiltinCallDescriptor {}

            pub struct AccessBuilderTS {}

            pub struct TSAssembler<R, B, F, M, Var> {
                data: PipelineData,
                graph: Graph,
                _phantom: std::marker::PhantomData<(R, B, F, M, Var)>,
            }

            impl<R, B, F, M, Var> TSAssembler<R, B, F, M, Var> {
                pub fn data(&self) -> &PipelineData {
                    &self.data
                }
                pub fn graph(&self) -> &Graph {
                    &self.graph
                }
                pub fn resolve<T>(&self, const_or_v: &ConstOrV<T>) -> &V<T> {
                    const_or_v.v.as_ref().unwrap()
                }

                pub fn new_block(&self) -> *mut Block {
                    Box::into_raw(Box::new(Block {}))
                }

                pub fn bind(&self, block: *mut Block) -> bool {
                    true
                }
                 pub fn is_smi(&self, _value: V<i32>) -> V<Word32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn untag_smi(&self, _smi: V<i32>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn load_map_field(&self, _object: V<i32>) -> V<i32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn is_heap_number_map(&self, _map: V<i32>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                 pub fn load_heap_number_value(&self, _number: V<i32>) -> V<Float64> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn js_truncate_float64_to_word32(&self, _float: V<Float64>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn load_instance_type_field(&self, _map: V<i32>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn builtin_call_descriptor(&self) -> *const BuiltinCallDescriptor {
                   std::ptr::null()
                }
                pub fn change_uint32_to_uintptr(&self, input: V<Word32>) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                 pub fn int32_less_than_or_equal(&self, _a: i32, _b: V<Word32>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                 pub fn word32_equal(&self, _a: V<Word32>, _b: V<Word32>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                 pub fn is_feedback_vector_map(&self, _map: V<i32>) -> V<Word32> {
                      V{_phantom: std::marker::PhantomData}
                }
                 pub fn element_offset_from_index(&self, slot: V<WordPtr>, _elements_kind: i32, header_size: i32) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn intptr_less_than_or_equal(&self, _offset: V<WordPtr>, _last_offset: V<WordPtr>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn load_register(&self, _feedback_vector: i32) -> V<i32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn change_int32_to_int64(&self, argc: V<Word32>) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn truncate_word64_to_word32(&self, argc: V<Word64>) -> V<WordPtr> {
                   V{_phantom: std::marker::PhantomData}
                }
                pub fn frame_pointer(&self) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_add(&self, fp: V<WordPtr>, offset: i32) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_sub(&self, argc: V<WordPtr>, kjs_argc_receiver_slots: i32) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn uintptr_less_than(&self, index: V<WordPtr>, length_without_receiver: V<WordPtr>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn load_off_heap(&self, at_index_ptr: V<WordPtr>, any_tagged: MemoryRepresentation) -> V<i32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn uintptr_less_than_or_equal(&self, end_offset: V<WordPtr>, current_iterator: V<WordPtr>) -> OptionalV<Word32> {
                    OptionalV{value: Some(V{_phantom: std::marker::PhantomData})}
                }
                pub fn word_ptr_add2(&self, current_iterator: V<WordPtr>, elements_kind_to_byte_size: i32) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                 pub fn load(&self, feedback_vector: V<i32>, offset: V<WordPtr>, tagged_base: i32, any_tagged: MemoryRepresentation) -> V<i32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn element_offset_from_index2(&self, slot: V<WordPtr>, holey_elements: i32, header_size: i32) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
                 pub fn store(&self, feedback_vector: V<i32>, offset: V<WordPtr>, value: V<i32>, tagged_base: i32, any_tagged: MemoryRepresentation, no_write_barrier: i32) {

                }
                 pub fn undefined_constant(&self) -> V<i32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn smi_constant(&self, from_int: i32) -> V<i32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn smi_bitwise_or(&self, feedback: V<i32>, smi_constant: V<i32>) -> V<i32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn bitcast_word32_to_smi(&self, word32_bitwise_or: V<Word32>) -> V<i32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn word32_bitwise_or(&self, bitcast_smi_to_word32: V<Word32>, bitcast_smi_to_word322: V<Word32>) -> V<Word32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn bitcast_smi_to_word32(&self, a: V<i32>) -> V<Word32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn js_stack_check(&self, context: V<i32>, nullopt: OptionalV<i32>, kbuiltin_entry: i32) {

                }
                pub fn parameter<T>(&self, i: i32, from_machine_type: RegisterRepresentation) -> V<T> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn truncate_word_ptr_to_word32(&self, pop_count: V<WordPtr>) -> V<Word32> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn return_value(&self, truncate_word_ptr_to_word32: V<Word32>, temp: base::Vector<OpIndex>) {

                }
                pub fn perform_stack_check(&self, js_context_parameter: V<i32>) {

                }
                pub fn js_truncate_float64_to_word32_2(&self, value_float64: V<Float64>) -> V<Word32> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_bitwise_and(&self, word_ptr_add: V<WordPtr>, kobject_alignment_mask: i32) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_constant(&self, i: i32) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_shift_left(&self, index: V<WordPtr>, element_size_shift: i32) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
                pub fn word_ptr_add_2(&self, base_size: i32, index: V<WordPtr>) -> V<WordPtr> {
                     V{_phantom: std::marker::PhantomData}
                }
            }

            impl<R, B, F, M, Var> TSAssembler<R, B, F, M, Var> {
                pub fn new(
                    data: PipelineData,
                    graph: Graph,
                    graph2: Graph,
                    phase_zone: *mut Zone,
                ) -> Self {
                    Self {
                        data,
                        graph,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }

            pub struct PipelineData {}

            pub struct Graph {}

            pub struct Zone {}

            pub struct Var<T, A> {
                assembler: *const A,
                value: RefCell<Option<T>>,
            }

            impl<T, A> Var<T, A> {
                pub fn new(assembler: *const A) -> Self {
                    Var {
                        assembler: assembler,
                        value: RefCell::new(None),
                    }
                }

                 pub fn set(&self, value: T) {
                    *self.value.borrow_mut() = Some(value);
                }

                pub fn get(&self) -> T
                where T: Copy {
                    *self.value.borrow().as_ref().unwrap()
                }
            }
        }
    }
}
pub mod internal {
    use crate::turboshaft::compiler::turboshaft::TSAssembler;
    use crate::turboshaft::compiler::turboshaft::V;
    use crate::turboshaft::compiler::turboshaft::Word32;
    use crate::turboshaft::compiler::turboshaft::Block;
    use crate::turboshaft::compiler::turboshaft::BuiltinCallDescriptor;
    use crate::turboshaft::compiler::turboshaft::Label;
    use crate::turboshaft::compiler::turboshaft::MemoryRepresentation;
    use crate::turboshaft::compiler::turboshaft::OpIndex;
    use crate::turboshaft::compiler::turboshaft::RegisterRepresentation;
    use crate::turboshaft::compiler::turboshaft::ConstOrV;
    use crate::turboshaft::compiler::turboshaft::Float64;
    use crate::turboshaft::compiler::turboshaft::OptionalV;
    use crate::turboshaft::compiler::turboshaft::WordPtr;
    use crate::turboshaft::compiler::turboshaft::PipelineData;
    use crate::turboshaft::compiler::turboshaft::Graph;
    use crate::turboshaft::compiler::turboshaft::Zone;
    use crate::turboshaft::compiler::turboshaft::Var;

    pub enum IsKnownTaggedPointer {
        kNo,
        kYes,
    }
    pub mod detail {

        use crate::internal::{BuiltinArgumentsTS,IsAligned};
        use crate::turboshaft::compiler::turboshaft::{V, ConstOrV, WordPtr, Object, TSA_DCHECK, MemoryRepresentation, Word32, UintPtrLessThan, ElementsKindToByteSize, SYSTEM_POINTER_ELEMENTS};
        pub struct BuiltinArgumentsTS {}
        impl BuiltinArgumentsTS {
                pub fn GetLengthWithoutReceiver(&self) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn AtIndex(&self, _index:ConstOrV<WordPtr>) -> V<Object> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn Range(&self, begin:ConstOrV<WordPtr>, end:ConstOrV<WordPtr>) -> Iterator {
                    Iterator {args_: std::ptr::null(), begin_index_: ConstOrV { value: None, v: None }, end_index_: ConstOrV { value: None, v: None }, end_offset_: V{_phantom: std::marker::PhantomData}}
                }
                 pub fn GetLengthWithReceiver(&self) -> V<WordPtr> {
                    V{_phantom: std::marker::PhantomData}
                }
                pub fn Range2(&self, begin:ConstOrV<WordPtr>) -> Iterator {
                    Iterator {args_: std::ptr::null(), begin_index_: ConstOrV { value: None, v: None }, end_index_: ConstOrV { value: None, v: None }, end_offset_: V{_phantom: std::marker::PhantomData}}
                }
                pub fn Range3(&self) -> Iterator {
                    Iterator {args_: std::ptr::null(), begin_index_: ConstOrV { value: None, v: None }, end_index_: ConstOrV { value: None, v: None }, end_offset_: V{_phantom: std::marker::PhantomData}}
                }
        }
        pub struct Iterator {
            args_: *const BuiltinArgumentsTS,
            begin_index_: ConstOrV<WordPtr>,
            end_index_: ConstOrV<WordPtr>,
            end_offset_: V<WordPtr>,
        }

    }

    pub struct FeedbackVector {}
    pub struct Undefined {}
    pub struct Union<T, U> {
        _phantom: std::marker::PhantomData<(T, U)>,
    }
    pub struct MaybeObject {}
    pub struct HeapObject {}
    pub struct Map {}
    pub struct Smi {}
    pub struct Oddball {}
    pub struct BigInt {}
    pub struct JSReceiver {}
    pub struct JSAnyNotNumber {}
    pub struct Number {}
    pub struct Context {}
    pub struct Object {}

    pub enum WriteBarrierMode {
        SKIP_WRITE_BARRIER,
        UNSAFE_SKIP_WRITE_BARRIER,
        UPDATE_WRITE_BARRIER,
        UPDATE_EPHEMERON_KEY_WRITE_BARRIER,
    }

    pub enum UpdateFeedbackMode {
        kNoFeedback,
        kOptionalFeedback,
        kGuaranteedFeedback,
    }
    pub struct InstructionBase {}

    pub struct Builtins {}
    impl Builtins {
        pub fn KindOf(builtin_id: Builtin) -> i32 {
            0
        }
        pub fn TSJ -> i32 {
            0
        }
    }
    pub enum Builtin {}

    pub enum BinaryOperationFeedback {
        kNone,
        kSignedSmall,
        kNumber,
        kBigInt,
        kBigInt64,
        kNumberOrOddball,
        kAny,
    }

    fn ElementsKindToShiftSize(kind: i32) -> i32 {
        0
    }
    fn IsAligned(additional_offset: i32, ktaggedsize: i32) -> bool {
        true
    }
    pub struct AssemblerBase {}
    pub mod base {
        use crate::turboshaft::compiler::turboshaft::OpIndex;
        pub struct Vector<T> {
            _phantom: std::marker::PhantomData<T>,
            }

            impl<T> Vector<T> {
                pub fn new() -> Self {
                    Self{_phantom: std::marker::PhantomData}
                }
            }

            pub fn VectorOf<T>(items: std::initializer_list<T>) -> Vector<T> {
                Vector{_phantom: std::marker::PhantomData}
            }
    }

    macro_rules! GOTO {
        ($label:ident, $($arg:expr),*) => {
            println!("GOTO {}", stringify!($label));
        };
    }
    macro_rules! IF {
        ($condition:expr) => {
            println!("IF {}", stringify!($condition));
        };
    }
     macro_rules! IF_NOT {
        ($condition:expr) => {
            println!("IF NOT {}", stringify!($condition));
        };
    }
    macro_rules! WHILE {
        ($condition:expr) => {
            println!("WHILE {}", stringify!($condition));
        };
    }
    macro_rules! GOTO_IF {
        ($condition:expr, $label:ident) => {
            println!("GOTO_IF {} {}", stringify!($condition), stringify!($label));
        };
    }
    macro_rules! BIND {
        ($label:ident, $($arg:ident),*) => {
            println!("BIND {}", stringify!($label));
        };
        ($label:ident) => {
            println!("BIND {}", stringify!($label));
        };
    }
    macro_rules! TSA_SLOW_DCHECK {
        ($self:ident, $condition:expr) => {
            if false {
                panic!("TSA_SLOW_DCHECK failed: {}", stringify!($condition));
            }
        };
    }
    macro_rules! TSA_DCHECK {
        ($self:ident, $condition:expr) => {
            if false {
                panic!("TSA_DCHECK failed: {}", stringify!($condition));
            }
        };
    }
    macro_rules! DCHECK {
        ($condition:expr) => {
            if false {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        };
    }
    macro_rules! UNREACHABLE {
        () => {
            panic!("UNREACHABLE");
        };
    }
    macro_rules! UNIMPLEMENTED {
        () => {
            panic!("UNIMPLEMENTED");
        };
    }
    macro_rules! CHECK_EQ {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!("CHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
            }
        };
    }
    macro_rules! CodeComment {
        ($comment:expr) => {
            println!("CodeComment: {}", $comment);
        };
    }
    macro_rules! DEFINE_TURBOSHAFT_ALIASES {
        () => {};
    }
    const ODDBALL_TYPE: i32 = 0;
    const HOLEY_ELEMENTS: i32 = 0;
    const BIGINT_TYPE: i32 = 0;
    const SYSTEM_POINTER_ELEMENTS: i32 = 0;
    const KOBJECTALIGNMENTMASK: i32 = 0;
    const KTAGGEDSIZE: i32 = 0;
    const KOBJECTALIGNMENTMASK: i32 = 0;
    const KTAGGEDSIZE: i32 = 0;

    trait FeedbackCollector {
        fn combine_feedback(&mut self, additional_feedback: i32);
        fn overwrite_feedback(&mut self, new_feedback: i32);
        fn feedback_is(&self, checked_feedback: i32) -> V<Word32>;
        fn update_feedback(&mut self);
        fn combine_exception_feedback(&mut self);
    }

    pub struct FeedbackCollectorReducer<Next> {
        next: Next,
        maybe_feedback_vector_: V<Union<FeedbackVector, Undefined>>,
        slot_id_: V<WordPtr>,
        feedback_: Var<Smi, FeedbackCollectorReducer<Next>>,
        feedback_on_exception_: Var<Smi, FeedbackCollectorReducer<Next>>,
        mode_: UpdateFeedbackMode,
    }

    impl<Next> FeedbackCollectorReducer<Next> {
        pub fn new(next: Next) -> Self {
            FeedbackCollectorReducer {
                next,
                maybe_feedback_vector_: V{_phantom: std::marker::PhantomData},
                slot_id_: V{_phantom: std::marker::PhantomData},
                feedback_: Var::new(std::ptr::null()),
                feedback_on_exception_: Var::new(std::ptr::null()),
                mode_: UpdateFeedbackMode::kOptionalFeedback,
            }
        }
        pub fn SmiEqual(&self, a: V<Smi>, b: V<Smi>) -> V<Word32> {
            unsafe {
               (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).word32_equal(
                    unsafe {(*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).bitcast_smi_to_word32(a)},
                    unsafe {(*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).bitcast_smi_to_word32(b)}
                )
            }
        }
    }

    impl<Next> FeedbackCollector for FeedbackCollectorReducer<Next> {
        fn combine_feedback(&mut self, additional_feedback: i32) {
            CodeComment!("CombineFeedback");
            let feedback_ = unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).smi_constant(additional_feedback)
             };
            self.feedback_ = Var::new(self as *const Self);

        }

        fn overwrite_feedback(&mut self, new_feedback: i32) {
            CodeComment!("OverwriteFeedback");
             let feedback_ = unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).smi_constant(new_feedback)
             };
            self.feedback_ = Var::new(self as *const Self);
        }

        fn feedback_is(&self, checked_feedback: i32) -> V<Word32> {
             unsafe {
               (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).word32_equal(self.feedback_.get()._phantom, V{_phantom: std::marker::PhantomData})
            }
        }

        fn update_feedback(&mut self) {
            CodeComment!("UpdateFeedback");
        }
        fn combine_exception_feedback(&mut self) {

        }
    }
    impl<Next> FeedbackCollectorReducer<Next> {
        pub fn LoadFeedbackVector() -> V<Union<FeedbackVector, Undefined>> {
            V{_phantom: std::marker::PhantomData}
        }
        pub fn LoadFeedbackVectorLength(feedback_vector: V<FeedbackVector>) -> V<WordPtr> {
            V{_phantom: std::marker::PhantomData}
        }
        pub fn LoadFeedbackVectorSlot(feedback_vector: V<FeedbackVector>, slot: V<WordPtr>, additional_offset: i32) -> V<MaybeObject> {
            V{_phantom: std::marker::PhantomData}
        }

        pub fn StoreFeedbackVectorSlot(feedback_vector: V<FeedbackVector>, slot: V<WordPtr>, value: V<Object>, barrier_mode: WriteBarrierMode, additional_offset: i32) {

        }
        pub fn SetFeedbackSlot(&mut self, slot_id: V<WordPtr>) {
            self.slot_id_ = slot_id;
        }
        pub fn SetFeedbackVector(&mut self, feedback_vector: V<FeedbackVector>) {
            TSA_DCHECK!(self, V{_phantom: std::marker::PhantomData});
            self.maybe_feedback_vector_ = feedback_vector;
             let feedback_ = unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).smi_constant(0)
             };
            self.feedback_ = Var::new(self as *const Self);
            self.feedback_on_exception_ = Var::new(self as *const Self);
        }

        pub fn LoadFeedbackVectorOrUndefinedIfJitless(&mut self) {
             let undefined = unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).undefined_constant()
             };
            self.maybe_feedback_vector_ = undefined;
            let feedback_ = unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).smi_constant(0)
             };
            self.feedback_ = Var::new(self as *const Self);
            self.feedback_on_exception_ = Var::new(self as *const Self);
        }
        pub fn ChangePositiveInt32ToIntPtr(&self, input: V<Word32>) -> V<WordPtr> {
             unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).change_uint32_to_uintptr(input)
             }
        }
        pub fn IsFeedbackVector(&self, heap_object: V<HeapObject>) -> V<Word32> {
             unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).load_map_field(heap_object)
             }

        }
        pub fn IsOffsetInBounds(&self, offset: V<WordPtr>, length: V<WordPtr>, header_size: i32, kind: i32) -> V<Word32> {
             unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).intptr_less_than_or_equal(offset, V{_phantom: std::marker::PhantomData})
             }
        }
        pub const fn DefaultUpdateFeedbackMode() -> UpdateFeedbackMode {
            UpdateFeedbackMode::kOptionalFeedback
        }
        pub fn SmiBitwiseOr(&self, a: V<Smi>, b: V<Smi>) -> V<Smi> {
           unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).bitcast_word32_to_smi(unsafe {(*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).word32_bitwise_or(unsafe {(*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).bitcast_smi_to_word32(a)},unsafe {(*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).bitcast_smi_to_word32(b)})})
           }
        }
    }

    pub struct NoFeedbackCollectorReducer<Next> {
        next: Next,
    }

    impl<Next> NoFeedbackCollectorReducer<Next> {
        pub fn new(next: Next) -> Self {
            NoFeedbackCollectorReducer { next }
        }
    }

    impl<Next> FeedbackCollector for NoFeedbackCollectorReducer<Next> {
        fn combine_feedback(&mut self, additional_feedback: i32) {}
        fn overwrite_feedback(&mut self, new_feedback: i32) {}
        fn feedback_is(&self, checked_feedback: i32) -> V<Word32> {
            UNREACHABLE!();
        }
        fn update_feedback(&mut self) {}
        fn combine_exception_feedback(&mut self) {}
    }
    pub struct BuiltinsReducer<Next> {
        next: Next,
    }

    impl<Next> BuiltinsReducer<Next> {
        pub fn new(next: Next) -> Self {
            BuiltinsReducer { next }
        }
    }

    impl<Next> BuiltinsReducer<Next> {
        pub fn EmitBuiltinProlog(&self, builtin_id: Builtin) {
            unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).perform_stack_check(self.JSContextParameter());
            }
        }
        pub fn EmitEpilog(&self, _catch_block: *mut Block) {}
        pub fn JSContextParameter(&self) -> V<Context> {
             unsafe {
                 (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).parameter(0, RegisterRepresentation::AnyTagged)
             }
        }
        pub fn PerformStackCheck(&self, context: V<Context>) {
            unsafe {
                (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).js_stack_check(context, OptionalV { value: None }, 0);
            }

        }
        pub fn PopAndReturn(&self, arguments: &BuiltinArgumentsTS, return_value: V<Object>) {
           let length = arguments.GetLengthWithReceiver();
           let truncated = unsafe {
                 (*(self as *const Self).offset_from(0) as *const TSAssembler<i32,i32,i32,i32,i32>).truncate_word_ptr_to_word32(length)
           };
            unsafe {
                 (*(self as *const Self).offset_from(0
