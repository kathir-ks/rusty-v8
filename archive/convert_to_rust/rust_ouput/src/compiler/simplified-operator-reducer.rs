// Converted from V8 C++ source files:
// Header: simplified-operator-reducer.h
// Implementation: simplified-operator-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
    use std::convert::TryInto;
    use std::f64;
    use std::mem::transmute;
    use std::ops::Deref;
    use crate::v8::internal::{Factory, Isolate};

    pub struct V8_EXPORT_PRIVATE {}

    pub enum BranchSemantics {
        kJS,
        kBool,
    }

    pub struct Reduction {}

    pub struct JSGraph {
        isolate: Box<Isolate>,
        graph: Box<TFGraph>,
        machine: Box<MachineOperatorBuilder>,
        simplified: Box<SimplifiedOperatorBuilder>,
    }

    impl JSGraph {
        pub fn isolate(&self) -> &Isolate {
            &self.isolate
        }
        pub fn graph(&self) -> &TFGraph {
            &self.graph
        }
        pub fn machine(&self) -> &MachineOperatorBuilder {
            &self.machine
        }
        pub fn simplified(&self) -> &SimplifiedOperatorBuilder {
            &self.simplified
        }
        pub fn BooleanConstant(&self, value: bool) -> Node {
            Node {}
        }
        pub fn Float64Constant(&self, value: f64) -> Node {
            Node {}
        }
        pub fn Int32Constant(&self, value: i32) -> Node {
            Node {}
        }
        pub fn ConstantNoHole<T>(&self, value: T) -> Node {
            Node {}
        }
    }

    pub struct JSHeapBroker {}

    pub struct TFGraph {}

    pub struct MachineOperatorBuilder {}

    impl MachineOperatorBuilder {
        pub fn ChangeInt32ToFloat64(&self) -> &'static Operator {
            unsafe { &CHANGE_INT32_TO_FLOAT64_OPERATOR }
        }
        pub fn ChangeUint32ToFloat64(&self) -> &'static Operator {
            unsafe { &CHANGE_UINT32_TO_FLOAT64_OPERATOR }
        }
        pub fn ChangeFloat64ToInt32(&self) -> &'static Operator {
            unsafe { &CHANGE_FLOAT64_TO_INT32_OPERATOR }
        }
        pub fn ChangeFloat64ToUint32(&self) -> &'static Operator {
            unsafe { &CHANGE_FLOAT64_TO_UINT32_OPERATOR }
        }
        pub fn TruncateFloat64ToWord32(&self) -> &'static Operator {
            unsafe { &TRUNCATE_FLOAT64_TO_WORD32_OPERATOR }
        }
    }

    pub struct SimplifiedOperatorBuilder {}

    pub struct Node {}

    impl Node {
        pub fn opcode(&self) -> IrOpcode {
            IrOpcode::kOther
        }
        pub fn InputAt(&self, index: usize) -> &Node {
            &Node {}
        }
        pub fn ReplaceInput(&mut self, index: usize, new_input: &Node) {}
        pub fn use_edges(&self) -> Vec<Edge> {
          Vec::new()
        }

        pub fn Kill(&mut self) {}
        pub fn IsDead(&self) -> bool {
          false
        }
    }

    pub struct Edge {
      from_: *mut Node,
    }
    impl Edge {
      pub fn from(&self) -> &Node {
        unsafe { &*self.from_ }
      }
    }

    pub struct Operator {}

    #[repr(C)]
    pub enum IrOpcode {
        kBooleanNot,
        kChangeBitToTagged,
        kChangeTaggedToBit,
        kChangeFloat64ToTagged,
        kChangeInt31ToTaggedSigned,
        kChangeInt32ToTagged,
        kChangeTaggedToFloat64,
        kTruncateTaggedToFloat64,
        kChangeTaggedSignedToInt32,
        kChangeTaggedToInt32,
        kChangeTaggedToUint32,
        kChangeUint32ToTagged,
        kTruncateTaggedToWord32,
        kCheckedFloat64ToInt32,
        kCheckedTaggedToArrayIndex,
        kCheckedTaggedToInt32,
        kCheckedTaggedSignedToInt32,
        kCheckIf,
        kCheckNumberFitsInt32,
        kCheckNumber,
        kCheckHeapObject,
        kCheckSmi,
        kObjectIsSmi,
        kNumberAbs,
        kReferenceEqual,
        kCheckedInt32Add,
        kConvertTaggedHoleToUndefined,
        kOther,
    }

    pub struct NodeProperties {}

    impl NodeProperties {
        pub fn GetEffectInput(node: &Node) -> &Node {
            &Node {}
        }
        pub fn ChangeOp(node: &mut Node, op: &Operator) {}
        pub fn GetTotalInputCount(op: &Operator) -> usize {
            0
        }
    }

    pub struct AdvancedReducer {
        editor: Box<Editor>,
    }

    impl AdvancedReducer {
        pub fn Changed(&mut self, _node: &Node) -> Reduction {
            Reduction {}
        }
        pub fn NoChange(&self) -> Reduction {
            Reduction {}
        }
        pub fn Replace(&mut self, _node: &Node) -> Reduction {
            Reduction {}
        }
        pub fn ReplaceWithValue(&mut self, _node: &Node, _value: &Node) {}
        pub fn RelaxEffectsAndControls(&mut self, _node: &Node) {}

    }

    pub struct Editor {}

    pub struct SimplifiedOperatorReducer {
        base: AdvancedReducer,
        jsgraph_: *const JSGraph,
        broker_: *const JSHeapBroker,
        branch_semantics_: BranchSemantics,
    }

    impl SimplifiedOperatorReducer {
        pub fn new(
            editor: Editor,
            jsgraph: *const JSGraph,
            broker: *const JSHeapBroker,
            branch_semantics: BranchSemantics,
        ) -> Self {
            SimplifiedOperatorReducer {
                base: AdvancedReducer { editor: Box::new(editor) },
                jsgraph_: jsgraph,
                broker_: broker,
                branch_semantics_: branch_semantics,
            }
        }

        pub fn reducer_name(&self) -> &'static str {
            "SimplifiedOperatorReducer"
        }

        pub fn Reduce(&mut self, node: &mut Node) -> Reduction {
            unsafe {
                match node.opcode() {
                    IrOpcode::kBooleanNot => {
                        let m = HeapObjectMatcher::new(node.InputAt(0));
                        if m.Is(self.factory().true_value()) {
                            return self.ReplaceBoolean(false);
                        }
                        if m.Is(self.factory().false_value()) {
                            return self.ReplaceBoolean(true);
                        }
                        if m.IsBooleanNot() {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeBitToTagged => {
                        let m = Int32Matcher::new(node.InputAt(0));
                        if m.Is(0) {
                            return self.Replace(&(*self.jsgraph_).FalseConstant());
                        }
                        if m.Is(1) {
                            return self.Replace(&(*self.jsgraph_).TrueConstant());
                        }
                        if m.IsChangeTaggedToBit() {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeTaggedToBit => {
                        let m = HeapObjectMatcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            let maybe_result = m.Ref(&(*self.broker_)).TryGetBooleanValue(&(*self.broker_));
                            if let Some(result) = maybe_result {
                                return self.ReplaceInt32(result as i32);
                            }
                        }
                        if m.IsChangeBitToTagged() {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeFloat64ToTagged => {
                        let m = Float64Matcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            return self.ReplaceNumber(m.ResolvedValue());
                        }
                        if m.IsChangeTaggedToFloat64() {
                            return self.Replace(m.node().InputAt(0));
                        }
                    }
                    IrOpcode::kChangeInt31ToTaggedSigned => {}
                    IrOpcode::kChangeInt32ToTagged => {
                        let m = Int32Matcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            return self.ReplaceNumber(m.ResolvedValue() as f64);
                        }
                        if m.IsChangeTaggedSignedToInt32() {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeTaggedToFloat64 => {}
                    IrOpcode::kTruncateTaggedToFloat64 => {
                        let m = NumberMatcher::new(node.InputAt(0));
                        if (m.HasResolvedValue()) {
                            return self.ReplaceFloat64(m.ResolvedValue());
                        }
                        if (m.IsChangeFloat64ToTagged() || m.IsChangeFloat64ToTaggedPointer()) {
                            return self.Replace(m.node().InputAt(0));
                        }
                        if (m.IsChangeInt31ToTaggedSigned() || m.IsChangeInt32ToTagged()) {
                            return self.Change(node, self.machine().ChangeInt32ToFloat64(), m.InputAt(0));
                        }
                        if (m.IsChangeUint32ToTagged()) {
                            return self.Change(node, self.machine().ChangeUint32ToFloat64(), m.InputAt(0));
                        }
                        break;
                    }
                    IrOpcode::kChangeTaggedSignedToInt32 => {}
                    IrOpcode::kChangeTaggedToInt32 => {
                        let m = NumberMatcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            return self.ReplaceInt32(double_to_i32(m.ResolvedValue()));
                        }
                        if m.IsChangeFloat64ToTagged() || m.IsChangeFloat64ToTaggedPointer() {
                            return self.Change(node, self.machine().ChangeFloat64ToInt32(), m.InputAt(0));
                        }
                        if m.IsChangeInt31ToTaggedSigned() || m.IsChangeInt32ToTagged() {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeTaggedToUint32 => {
                        let m = NumberMatcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            return self.ReplaceUint32(double_to_u32(m.ResolvedValue()));
                        }
                        if m.IsChangeFloat64ToTagged() || m.IsChangeFloat64ToTaggedPointer() {
                            return self.Change(node, self.machine().ChangeFloat64ToUint32(), m.InputAt(0));
                        }
                        if m.IsChangeUint32ToTagged()) {
                            return self.Replace(m.InputAt(0));
                        }
                    }
                    IrOpcode::kChangeUint32ToTagged => {
                        let m = Uint32Matcher::new(node.InputAt(0));
                        if m.HasResolvedValue()) {
                            return self.ReplaceNumber(fast_ui2d(m.ResolvedValue()));
                        }
                        break;
                    }
                    IrOpcode::kTruncateTaggedToWord32 => {
                        let m = NumberMatcher::new(node.InputAt(0));
                        if m.HasResolvedValue()) {
                            return self.ReplaceInt32(double_to_i32(m.ResolvedValue()));
                        }
                        if m.IsChangeInt31ToTaggedSigned() || m.IsChangeInt32ToTagged() ||
                           m.IsChangeUint32ToTagged() {
                            return self.Replace(m.InputAt(0));
                        }
                        if m.IsChangeFloat64ToTagged() || m.IsChangeFloat64ToTaggedPointer() {
                            return self.Change(node, self.machine().TruncateFloat64ToWord32(), m.InputAt(0));
                        }
                        break;
                    }
                    IrOpcode::kCheckedFloat64ToInt32 => {
                        let m = Float64Matcher::new(node.InputAt(0));
                        if m.HasResolvedValue() && is_i32_double(m.ResolvedValue())) {
                            let value = (&*self.jsgraph_).Int32Constant(m.ResolvedValue() as i32);
                            self.base.ReplaceWithValue(node, &value);
                            return self.Replace(&value);
                        }
                        break;
                    }
                    IrOpcode::kCheckedTaggedToArrayIndex => {}
                    IrOpcode::kCheckedTaggedToInt32 => {}
                    IrOpcode::kCheckedTaggedSignedToInt32 => {
                        let m = NodeMatcher::new(node.InputAt(0));
                        if m.IsConvertTaggedHoleToUndefined() {
                            node.ReplaceInput(0, m.InputAt(0));
                            return self.base.Changed(node);
                        }
                        break;
                    }
                    IrOpcode::kCheckIf => {
                        let m = HeapObjectMatcher::new(node.InputAt(0));
                        if m.Is(self.factory().true_value()) {
                            let effect = NodeProperties::GetEffectInput(node);
                            return self.Replace(effect);
                        }
                        break;
                    }
                    IrOpcode::kCheckNumberFitsInt32 => {}
                    IrOpcode::kCheckNumber => {
                        let m = NodeMatcher::new(node.InputAt(0));
                        if m.IsConvertTaggedHoleToUndefined() {
                            node.ReplaceInput(0, m.InputAt(0));
                            return self.base.Changed(node);
                        }
                        break;
                    }
                    IrOpcode::kCheckHeapObject => {
                        let input = node.InputAt(0);
                        if DecideObjectIsSmi(input) == Decision::kFalse {
                            self.base.ReplaceWithValue(node, input);
                            return self.Replace(input);
                        }
                        let m = NodeMatcher::new(input);
                        if m.IsCheckHeapObject() {
                            self.base.ReplaceWithValue(node, input);
                            return self.Replace(input);
                        }
                        break;
                    }
                    IrOpcode::kCheckSmi => {
                        let input = node.InputAt(0);
                        if DecideObjectIsSmi(input) == Decision::kTrue {
                            self.base.ReplaceWithValue(node, input);
                            return self.Replace(input);
                        }
                        let m = NodeMatcher::new(input);
                        if m.IsCheckSmi() {
                            self.base.ReplaceWithValue(node, input);
                            return self.Replace(input);
                        } else if m.IsConvertTaggedHoleToUndefined() {
                            node.ReplaceInput(0, m.InputAt(0));
                            return self.base.Changed(node);
                        }
                        break;
                    }
                    IrOpcode::kObjectIsSmi => {
                        let input = node.InputAt(0);
                        match DecideObjectIsSmi(input) {
                            Decision::kTrue => return self.ReplaceBoolean(true),
                            Decision::kFalse => return self.ReplaceBoolean(false),
                            Decision::kUnknown => break,
                        }
                        break;
                    }
                    IrOpcode::kNumberAbs => {
                        let m = NumberMatcher::new(node.InputAt(0));
                        if m.HasResolvedValue() {
                            return self.ReplaceNumber(m.ResolvedValue().abs());
                        }
                        break;
                    }
                    IrOpcode::kReferenceEqual => {
                        let m = HeapObjectBinopMatcher::new(node);
                        if m.left().node() as *const _ == m.right().node() as *const _ {
                            return self.ReplaceBoolean(true);
                        }
                        break;
                    }
                    IrOpcode::kCheckedInt32Add => {
                        let m = Int32BinopMatcher::new(node);
                        if m.right().HasResolvedValue() {
                            let checked_int32_add = m.left().node();
                            if checked_int32_add.opcode() == IrOpcode::kCheckedInt32Add {
                                let n = Int32BinopMatcher::new(checked_int32_add);
                                if n.right().HasResolvedValue() &&
                                   (n.right().ResolvedValue() >= 0) == (m.right().ResolvedValue() >= 0) {
                                    let val: i32;
                                    let overflow = (n.right().ResolvedValue() as i64).checked_add(m.right().ResolvedValue() as i64).is_none();

                                    if !overflow {
                                        let val_unwrapped = (n.right().ResolvedValue() as i64) + (m.right().ResolvedValue() as i64);
                                        val = val_unwrapped as i32;
                                        let mut has_no_other_uses = true;
                                        for edge in checked_int32_add.use_edges() {
                                          if !edge.from().IsDead() && (edge.from() as *const _) != (node as *const _) {
                                            has_no_other_uses = false;
                                            break;
                                          }
                                        }
                                        if has_no_other_uses {
                                            node.ReplaceInput(0, n.left().node());
                                            node.ReplaceInput(1, (&*self.jsgraph_).Int32Constant(val));
                                            self.base.RelaxEffectsAndControls(checked_int32_add);
                                            checked_int32_add.Kill();
                                            return self.base.Changed(node);
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                    _ => {}
                }
            }
            self.base.NoChange()
        }

        fn Change(&mut self, node: &mut Node, op: &Operator, a: &Node) -> Reduction {
            unsafe {
                assert_eq!(
                    0,
                    NodeProperties::GetTotalInputCount(op)
                );
                assert!(1 <= 0);
                node.ReplaceInput(0, a);
                NodeProperties::ChangeOp(node, op);
            }
            self.base.Changed(node)
        }

        fn ReplaceBoolean(&mut self, value: bool) -> Reduction {
            if let BranchSemantics::kJS = self.branch_semantics_ {
                unsafe {
                    return self.Replace(&(*self.jsgraph_).BooleanConstant(value));
                }
            } else {
                return self.ReplaceInt32(if value { 1 } else { 0 });
            }
        }

        fn ReplaceFloat64(&mut self, value: f64) -> Reduction {
            unsafe {
                return self.Replace(&(*self.jsgraph_).Float64Constant(value));
            }
        }

        fn ReplaceInt32(&mut self, value: i32) -> Reduction {
            unsafe {
                return self.Replace(&(*self.jsgraph_).Int32Constant(value));
            }
        }

        fn ReplaceUint32(&mut self, value: u32) -> Reduction {
            self.ReplaceInt32(value as i32)
        }

        fn ReplaceNumber(&mut self, value: f64) -> Reduction {
            unsafe {
                return self.Replace(&(*self.jsgraph_).ConstantNoHole(value));
            }
        }

        fn ReplaceNumber_i32(&mut self, value: i32) -> Reduction {
            unsafe {
                return self.Replace(&(*self.jsgraph_).ConstantNoHole(value));
            }
        }

        fn factory(&self) -> &Factory {
            unsafe { (&*self.jsgraph_).isolate().factory() }
        }

        fn graph(&self) -> &TFGraph {
            unsafe { (&*self.jsgraph_).graph() }
        }

        fn machine(&self) -> &MachineOperatorBuilder {
            unsafe { (&*self.jsgraph_).machine() }
        }

        fn simplified(&self) -> &SimplifiedOperatorBuilder {
            unsafe { (&*self.jsgraph_).simplified() }
        }

        fn jsgraph(&self) -> &JSGraph {
            unsafe { &*self.jsgraph_ }
        }
        fn broker(&self) -> &JSHeapBroker {
            unsafe { &*self.broker_ }
        }
    }

    // matchers
    pub struct MatcherBase<'a> {
        node: &'a Node,
        resolved_value: Option<f64>,
    }

    impl<'a> MatcherBase<'a> {
        pub fn new(node: &'a Node) -> Self {
            MatcherBase {
                node,
                resolved_value: None,
            }
        }

        pub fn HasResolvedValue(&self) -> bool {
            self.resolved_value.is_some()
        }

        pub fn ResolvedValue(&self) -> f64 {
            self.resolved_value.unwrap_or(0.0)
        }

        pub fn node(&self) -> &'a Node {
            self.node
        }
    }

    pub struct NumberMatcher<'a>(MatcherBase<'a>);

    impl<'a> NumberMatcher<'a> {
        pub fn new(node: &'a Node) -> Self {
            NumberMatcher(MatcherBase::new(node))
        }

        pub fn HasResolvedValue(&self) -> bool {
            self.0.HasResolvedValue()
        }

        pub fn ResolvedValue(&self) -> f64 {
            self.0.ResolvedValue()
        }

        pub fn IsChangeFloat64ToTagged(&self) -> bool {
            false
        }
        pub fn IsChangeFloat64ToTaggedPointer(&self) -> bool {
            false
        }
        pub fn IsChangeInt31ToTaggedSigned(&self) -> bool {
            false
        }
        pub fn IsChangeInt32ToTagged(&self) -> bool {
            false
        }
        pub fn IsChangeUint32ToTagged(&self) -> bool {
            false
        }
        pub fn InputAt(&self, _index: usize) -> &Node {
            &Node {}
        }

        pub fn node(&self) -> &Node {
            self.0.node()
        }
    }

    pub struct Int32Matcher<'a>(MatcherBase<'a>);

    impl<'a> Int32Matcher<'a> {
        pub fn new(node: &'a Node) -> Self {
            Int32Matcher(MatcherBase::new(node))
        }

        pub fn Is( &self, value: i32) -> bool {
            false
        }
        pub fn HasResolvedValue(&self) -> bool {
            self.0.HasResolvedValue()
        }

        pub fn ResolvedValue(&self) -> i32 {
            self.0.ResolvedValue() as i32
        }
        pub fn IsChangeTaggedToBit(&self) -> bool {
            false
        }
        pub fn InputAt(&self, _index: usize) -> &Node {
            &Node {}
        }
    }

    pub struct Uint32Matcher<'a>(MatcherBase<'a>);

    impl<'a> Uint32Matcher<'a> {
        pub fn new(node: &'a Node) -> Self {
            Uint32Matcher(MatcherBase::new(node))
        }
        pub fn HasResolvedValue(&self) -> bool {
            self.0.HasResolvedValue()
        }

        pub fn ResolvedValue(&self) -> u32 {
            self.0.ResolvedValue() as u32
        }
    }

    pub struct Float64Matcher<'a>(MatcherBase<'a>);

    impl<'a> Float64Matcher<'a> {
        pub fn new(node: &'a Node) -> Self {
            Float64Matcher(MatcherBase::new(node))
        }
        pub fn HasResolvedValue(&self) -> bool {
            self.0.HasResolvedValue()
        }

        pub fn ResolvedValue(&self) -> f64 {
            self.0.ResolvedValue()
        }
        pub fn IsChangeTaggedToFloat64(&self) -> bool {
            false
        }
    }

    pub struct HeapObjectMatcher<'a>(MatcherBase<'a>);

    impl<'a> HeapObjectMatcher<'a> {
        pub fn new(node: &'a Node) -> Self {
            HeapObjectMatcher(MatcherBase::new(node))
        }
        pub fn Is(&self, _value: *const ()) -> bool {
            false
        }
        pub fn HasResolvedValue(&self) -> bool {
            self.0.HasResolvedValue()
        }
        pub fn Ref(&self, _broker: &JSHeapBroker) -> HeapObjectRef {
            HeapObjectRef {}
        }
        pub fn IsBooleanNot(&self) -> bool {
            false
        }
        pub fn IsChangeBitToTagged(&self) -> bool {
            false
        }
    }

    pub struct NodeMatcher<'a>(MatcherBase<'a>);
    impl<'a> NodeMatcher<'a> {
      pub fn new(node: &'a Node) -> Self {
        NodeMatcher(MatcherBase::new(node))
      }
        pub fn IsConvertTaggedHoleToUndefined(&self) -> bool {
            false
        }
        pub fn IsCheckHeapObject(&self) -> bool {
            false
        }
        pub fn IsCheckSmi(&self) -> bool {
            false
        }
        pub fn InputAt(&self, _index: usize) -> &Node {
            &Node {}
        }
    }

    pub struct HeapObjectRef {}
    impl HeapObjectRef {
        pub fn TryGetBooleanValue(&self, _broker: &JSHeapBroker) -> Option<bool> {
            None
        }
    }

    pub struct HeapObjectBinopMatcher<'a> {
        left_: NumberMatcher<'a>,
        right_: NumberMatcher<'a>,
    }
    impl<'a> HeapObjectBinopMatcher<'a> {
      pub fn new(node: &'a Node) -> Self {
        HeapObjectBinopMatcher{ left_: NumberMatcher::new(node), right_: NumberMatcher::new(node)}
      }
      pub fn left(&self) -> &NumberMatcher<'a> {
          &self.left_
      }

      pub fn right(&self) -> &NumberMatcher<'a> {
          &self.right_
      }
    }

    pub struct Int32BinopMatcher<'a> {
        left_: NumberMatcher<'a>,
        right_: Int32Matcher<'a>,
    }
    impl<'a> Int32BinopMatcher<'a> {
      pub fn new(node: &'a Node) -> Self {
        Int32BinopMatcher{ left_: NumberMatcher::new(node), right_: Int32Matcher::new(node)}
      }
      pub fn left(&self) -> &NumberMatcher<'a> {
          &self.left_
      }

      pub fn right(&self) -> &Int32Matcher<'a> {
          &self.right_
      }
    
    }

    fn double_to_i32(value: f64) -> i32 {
        if value.is_nan() {
            0
        } else if value >= i32::MAX as f64 {
            i32::MAX
        } else if value <= i32::MIN as f64 {
            i32::MIN
        } else {
            value as i32
        }
    }

    fn double_to_u32(value: f64) -> u32 {
        if value.is_nan() {
            0
        } else if value >= u32::MAX as f64 {
            u32::MAX
        } else if value <= u32::MIN as f64 {
            u32::MIN
        } else {
            value as u32
        }
    }

    fn fast_ui2d(value: u32) -> f64 {
      value as f64
    }

    fn is_i32_double(value: f64) -> bool {
      if value.is_nan() || value.is_infinite() {
        return false;
      }
      let truncated = value as i64;
      truncated as f64 == value && truncated >= i32::MIN as i64 && truncated <= i32::MAX as i64
    }

    #[derive(PartialEq, Eq)]
    enum Decision {
        kTrue,
        kFalse,
        kUnknown,
    }

    fn DecideObjectIsSmi(_input: &Node) -> Decision {
        Decision::kUnknown
    }
    fn IsSmiDouble(_val : f64) -> bool {
      false
    }

    static mut CHANGE_INT32_TO_FLOAT64_OPERATOR: Operator = Operator {};
    static mut CHANGE_UINT32_TO_FLOAT64_OPERATOR: Operator = Operator {};
    static mut CHANGE_FLOAT64_TO_INT32_OPERATOR: Operator = Operator {};
    static mut CHANGE_FLOAT64_TO_UINT32_OPERATOR: Operator = Operator {};
    static mut TRUNCATE_FLOAT64_TO_WORD32_OPERATOR: Operator = Operator {};
}
