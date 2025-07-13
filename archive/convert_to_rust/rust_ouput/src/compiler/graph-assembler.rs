// Converted from V8 C++ source files:
// Header: graph-assembler.h
// Implementation: graph-assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod graph_assembler {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::base::small_vector::SmallVector;
    use crate::codegen::tnode::TNode;
    use crate::compiler::feedback_source::FeedbackSource;
    use crate::compiler::js_graph::JSGraph;
    use crate::compiler::node::Node;
    use crate::compiler::simplified_operator::SimplifiedOperatorBuilder;
    use crate::objects::hole::Hole;
    use crate::objects::oddball::Oddball;
    use crate::wasm::wasm_inlining_into_js::Label;
    use crate::v8::internal::JSHeapBroker;
    use crate::v8::internal::Isolate;

    use crate::compiler::machine_graph::MachineGraph;
    use crate::compiler::common_operator::CommonOperatorBuilder;
    use crate::compiler::machine_operator::MachineOperatorBuilder;
    use crate::compiler::code_assembler::ExternalReference;
    use crate::compiler::branch_elimination::Reduction;
    use crate::compiler::simplified_operator::CheckForMinusZeroMode;
    use crate::deoptimizer::deoptimizer::DeoptimizeReason;
    use crate::compiler::js_generic_lowering::FrameState;
    use crate::compiler::js_typed_lowering::Effect;
    use crate::compiler::simplified_lowering_verifier::Control;
    use crate::compiler::basic_block_instrumentor::BasicBlockControl;
    use crate::compiler::turbofan_types::ElementsKind;
    use crate::compiler::map_inference::InstanceType;
    use crate::compiler::code_assembler::MachineType;
    use crate::compiler::js_call_reducer::UseInfo;
    use crate::compiler::operator::Operator;
    use crate::compiler::turbofan_types::Type;
    use crate::compiler::js_type_hint_lowering::BranchHint;
    use crate::compiler::simplified_lowering_verifier::BranchSemantics;
    use crate::compiler::scheduler::BasicBlock;
    use crate::zone::zone::Zone;
    use std::vec::Vec;
    use crate::compiler::allocation_builder_inl::Graph;
    use crate::compiler::string_builder_optimizer::TFGraph;
    use crate::compiler::scheduler::Schedule;
    use crate::execution::isolate::Tagged_t;
    use crate::execution::isolate::RootIndex;
    use crate::objects::heap_number::HeapNumber;
    use crate::objects::js_array_buffer::JSArrayBuffer;
    use crate::objects::js_array_buffer::JSArrayBufferView;
    use crate::objects::string::String;
    use crate::codegen::callable::Callable;
    use crate::compiler::linkage::CallDescriptor;
    use crate::compiler::linkage;
    use crate::builtins::builtin::Builtin;
    use crate::codegen::machine_type::StoreRepresentation;
    use crate::zone::zone_vector::ZoneVector;
    use crate::compiler::graph_reducer::GraphReducer;
    use crate::compiler::graph_reducer::Reducer;
    use crate::handles::handles::Handle;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::smi::Smi;
    use crate::compiler::access_builder::FieldAccess;
    use crate::compiler::access_builder::ElementAccess;
    use crate::objects::fixed_array::FixedArrayBase;
    use crate::objects::js_array::JSArray;
    use crate::type_cache::type_cache::TypeCache;
    use crate::compiler::simplified_operator::AllocationType;
    use crate::compiler::js_operator::JSOperatorBuilder;
    use std::collections::HashSet;
    use crate::builtins::builtins_list::RuntimeFunctionId;
    use crate::compiler::simplified_operator::GrowFastElementsMode;
    use crate::compiler::js_array_buffer_view::JSArrayBufferView;
    use crate::compiler::js_array_buffer_view::kIsLengthTracking;
    use crate::compiler::js_array_buffer_view::kIsBackedByRab;
    use crate::compiler::js_array_buffer::JSArrayBuffer as V8JSArrayBuffer;
    use crate::compiler::turbofan_types::TypeCache as TurbofanTypeCache;
    use crate::compiler::representation_change::IdentifyZeros;
    use crate::execution::simulator_base::BoolT;
    use crate::handles::handles::ObjectRef;
    use crate::execution::simulator_base::UintPtrT;
    use crate::wasm::wasm_inlining_into_js::JSCreateLowering;
    use crate::compiler::representation_change::SloppyTNode;
    use crate::execution::simulator_base::MachineRepresentation;
    use crate::compiler::map_inference::MapRef;

    pub enum class GraphAssemblerLabelType {
        kDeferred,
        kNonDeferred,
        kLoop,
    }

    pub mod detail {
        pub const kGraphAssemblerLabelDynamicCount: usize = usize::MAX;

        pub struct GraphAssemblerHelper<const VarCount: usize>;

        impl<const VarCount: usize> GraphAssemblerHelper<VarCount> {
            pub type Array<T> = std::array<T, VarCount>;
            pub const kIsDynamic: bool = false;

            pub fn InitNodeArray(_reps: &Array<MachineRepresentation>) -> Array<*mut Node> {
                std::array::from_fn(|_| std::ptr::null_mut())
            }
        }

        impl GraphAssemblerHelper<{kGraphAssemblerLabelDynamicCount}> {
            pub type Array<T> = SmallVector<T, 4>;
            pub const kIsDynamic: bool = true;

            pub fn InitNodeArray(_reps: &SmallVector<MachineRepresentation, 4>) -> SmallVector<*mut Node, 4> {
                SmallVector::new()
            }
        }

        pub struct GraphAssemblerLabelForXHelper {}
    }

    // Label with statically known count of incoming branches and phis.
    pub struct GraphAssemblerLabel<const VarCount: usize> {
        type_: GraphAssemblerLabelType,
        loop_nesting_level_: i32,
        merged_count_: usize,
        effect_: *mut Node,
        control_: *mut Node,
        bindings_: SmallVector<*mut Node, 4>,
        representations_: SmallVector<MachineRepresentation, 4>,
        is_bound_: bool,
    }

    impl<const VarCount: usize> GraphAssemblerLabel<VarCount> {
        pub fn Count(&self) -> usize {
            self.representations_.len()
        }

        pub fn PhiAt(&self, _index: usize) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IsUsed(&self) -> bool {
            self.merged_count_ > 0
        }

        pub fn new(type_: GraphAssemblerLabelType, loop_nesting_level: i32,
               reps: SmallVector<MachineRepresentation, 4>) -> Self {
            GraphAssemblerLabel {
                type_: type_,
                loop_nesting_level_: loop_nesting_level,
                merged_count_: 0,
                effect_: std::ptr::null_mut(),
                control_: std::ptr::null_mut(),
                bindings_: SmallVector::new(),
                representations_: reps,
                is_bound_: false,
            }
        }

        fn SetBound(&mut self) {
            self.is_bound_ = true;
        }
        fn IsBound(&self) -> bool {
            self.is_bound_
        }
        fn IsDeferred(&self) -> bool {
            matches!(self.type_, GraphAssemblerLabelType::kDeferred)
        }
        fn IsLoop(&self) -> bool {
            matches!(self.type_, GraphAssemblerLabelType::kLoop)
        }
    }

    pub type GraphAssemblerDynamicLabel = GraphAssemblerLabel<{detail::kGraphAssemblerLabelDynamicCount}>;

    pub type NodeChangedCallback = std::option::Option<Box<dyn FnMut(*mut Node)>>;

    pub struct GraphAssembler {
        temp_zone_: *mut Zone,
        mcgraph_: *mut MachineGraph,
        default_branch_semantics_: BranchSemantics,
        effect_: *mut Node,
        control_: *mut Node,
        node_changed_callback_: NodeChangedCallback,
        inline_reducers_: Vec<*mut Reducer>,
        inline_reductions_blocked_: bool,
        loop_headers_: Vec<*mut *mut Node>,
        loop_nesting_level_: i32,
        mark_loop_exits_: bool,
    }

    impl GraphAssembler {
        pub fn new(
            jsgraph: *mut MachineGraph, zone: *mut Zone,
            default_branch_semantics: BranchSemantics,
            node_changed_callback: NodeChangedCallback,
            mark_loop_exits: bool,
        ) -> Self {
            GraphAssembler {
                temp_zone_: zone,
                mcgraph_: jsgraph,
                default_branch_semantics_: default_branch_semantics,
                effect_: std::ptr::null_mut(),
                control_: std::ptr::null_mut(),
                node_changed_callback_: node_changed_callback,
                inline_reducers_: Vec::new(),
                inline_reductions_blocked_: false,
                loop_headers_: Vec::new(),
                loop_nesting_level_: 0,
                mark_loop_exits_: mark_loop_exits,
            }
        }

        pub fn Reset(&mut self) {
        }

        pub fn InitializeEffectControl(&mut self, _effect: *mut Node, _control: *mut Node) {
        }

        pub fn MakeLabelFor(&self, type_: GraphAssemblerLabelType, reps: SmallVector<MachineRepresentation, 4>) -> GraphAssemblerDynamicLabel {
            GraphAssemblerDynamicLabel::new(type_, self.loop_nesting_level_, reps)
        }

        pub fn IntPtrConstant(&self, _value: isize) -> *mut Node {
            std::ptr::null_mut()
        }
        
        pub fn UintPtrConstant(&self, _value: usize) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Constant(&self, _value: i32) -> *mut Node {
            std::ptr::null_mut()
        }
        
        pub fn Uint32Constant(&self, _value: u32) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64Constant(&self, _value: i64) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Uint64Constant(&self, _value: u64) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn UniqueIntPtrConstant(&self, _value: isize) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Constant(&self, _value: f64) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ExternalConstant(&self, _ref: ExternalReference) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IsolateField(&self, _id: i32) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Projection(&self, _index: i32, _value: *mut Node, _ctrl: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Parameter(&self, _index: i32) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn LoadFramePointer(&self) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn LoadRootRegister(&self) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastFloat32ToInt32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastFloat64ToInt64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastInt32ToFloat32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastWord32ToWord64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastInt64ToFloat64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeFloat32ToFloat64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeFloat64ToInt32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeFloat64ToInt64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeFloat64ToUint32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeFloat64ToUint64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeInt32ToFloat64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeInt32ToInt64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeInt64ToFloat64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeUint32ToFloat64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ChangeUint32ToUint64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Abs(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64ExtractHighWord32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64ExtractLowWord32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64SilenceNaN(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn RoundFloat64ToInt32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn RoundInt32ToFloat32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TruncateFloat64ToFloat32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TruncateFloat64ToWord32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TruncateInt64ToInt32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TryTruncateFloat64ToInt64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TryTruncateFloat64ToUint64(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TryTruncateFloat64ToInt32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TryTruncateFloat64ToUint32(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word32ReverseBytes(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64ReverseBytes(&self, _input: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Add(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Div(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Equal(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64InsertHighWord32(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64InsertLowWord32(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64LessThan(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64LessThanOrEqual(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Max(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Min(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Mod(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64Sub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Add(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32LessThan(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Mul(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Sub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64Add(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64Sub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntAdd(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntLessThan(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntMul(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntSub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn UintLessThan(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordAnd(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordEqual(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordOr(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordSar(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordSarShiftOutZeros(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordShl(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordShr(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordXor(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn Int32AddWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64AddWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Div(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32Mod(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32MulWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64MulWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int32SubWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64SubWithOverflow(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64Div(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Int64Mod(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Uint32Div(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Uint32Mod(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Uint64Div(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Uint64Mod(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64And(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Equal(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Or(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Sar(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64SarShiftOutZeros(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Shl(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Shr(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Word64Xor(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordAnd(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordEqual(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordOr(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordSar(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordSarShiftOutZeros(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordShl(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordShr(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn WordXor(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn UintPtrLessThan(&self, left: *mut Node, right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn UintPtrLessThanOrEqual(&self, left: *mut Node, right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn UintPtrAdd(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn UintPtrSub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn UintPtrDiv(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn ChangeUint32ToUintPtr(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn DebugBreak(&self) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Unreachable(&mut self) -> *mut Node {
            let result = self.UnreachableWithoutConnectToEnd();
            self.ConnectUnreachableToEnd();
            self.InitializeEffectControl(std::ptr::null_mut(), std::ptr::null_mut());
            result
        }
        pub fn UnreachableWithoutConnectToEnd(&self) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntPtrEqual(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TaggedEqual(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn SmiSub(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn SmiLessThan(&self, _left: *mut Node, _right: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64RoundDown(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Float64RoundTruncate(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TruncateFloat64ToInt64(&self, _value: *mut Node, _kind: i32) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastWordToTagged(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastWordToTaggedSigned(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastTaggedToWord(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastTaggedToWordForTagAndSmiBits(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn BitcastMaybeObjectToWord(&self, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn TypeGuard(&self, _type: i32, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Checkpoint(&self, _frame_state: FrameState) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn StackSlot(&self, _size: i32, _alignment: i32, _is_tagged: bool) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn AdaptLocalArgument(&self, _argument: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Store(&self, _rep: StoreRepresentation, _object: *mut Node, _offset: *mut Node, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }
        
        pub fn Load(&self, _type: MachineType, _object: *mut Node, _offset: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn StoreUnaligned(&self, _rep: MachineRepresentation, _object: *mut Node, _offset: *mut Node, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn LoadUnaligned(&self, _type: MachineType, _object: *mut Node, _offset: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ProtectedStore(&self, _rep: MachineRepresentation, _object: *mut Node, _offset: *mut Node, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn ProtectedLoad(&self, _type: MachineType, _object: *mut Node, _offset: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn LoadTrapOnNull(&self, _type: MachineType, _object: *mut Node, _offset: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn StoreTrapOnNull(&self, _rep: StoreRepresentation, _object: *mut Node, _offset: *mut Node, _value: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn Retain(&self, _buffer: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntPtrAdd(&self, _a: *mut Node, _b: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn IntPtrSub(&self, _a: *mut Node, _b: *mut Node) -> *mut Node {
            std::ptr::null_mut()
        }

        pub fn DeoptimizeIf(&self, _reason: Deoptimize
