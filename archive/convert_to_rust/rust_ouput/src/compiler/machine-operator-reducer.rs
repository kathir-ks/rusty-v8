// Converted from V8 C++ source files:
// Header: machine-operator-reducer.h
// Implementation: machine-operator-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod machine_operator_reducer {
use std::cell::RefCell;
use std::rc::Rc;

use crate::base::bits;
use crate::base::division_by_constant;
use crate::base::ieee754;
use crate::base::logging;
use crate::builtins::builtins;
use crate::compiler::branch_elimination::Reduction;
use crate::compiler::common_operator::{DeoptimizeParameters, Operator};
use crate::compiler::diamond::Diamond;
use crate::compiler::js_operator::JSCallReducerAssembler;
use crate::compiler::machine_graph::MachineGraph;
use crate::compiler::machine_operator::{
    Float32RoundUpMode, LoadRepresentation, MachineOperatorBuilder,
    MachineRepresentation, MemoryAccessKind, StoreRepresentation,
};
use crate::compiler::node::{Node, NodeId, NodeProperties};
use crate::compiler::node_matchers::{
    Float32BinopMatcher, Float32Matcher, Float64BinopMatcher, Float64Matcher,
    Int32AddMatcher, Int32BinopMatcher, Int32Matcher, Int64AddMatcher,
    Int64BinopMatcher, Int64Matcher, NodeMatcher, Uint32BinopMatcher,
    Uint32Matcher, Uint64BinopMatcher,
};
use crate::compiler::opcodes;
use crate::compiler::turbofan_graph::TFGraph;
use crate::numbers::conversions_inl::DoubleToInt32;
use crate::numbers::conversions_inl::FastD2IChecked;
use crate::numbers::conversions_inl::FastD2UI;
use crate::numbers::conversions_inl::FastI2D;
use crate::numbers::conversions_inl::FastUI2D;
use crate::numbers::ieee754::Modulo;
use crate::v8::V8_EXPORT_PRIVATE;
use crate::v8::{MemorySpan, V8};

pub struct MachineOperatorReducer {
    editor: *mut Editor,
    mcgraph_: *mut MachineGraph,
    signalling_nan_propagation_: SignallingNanPropagation,
}

#[derive(Clone, Copy)]
pub enum SignallingNanPropagation {
    kSilenceSignallingNan,
    kPropagateSignallingNan,
}

impl MachineOperatorReducer {
    pub fn new(
        editor: *mut Editor,
        mcgraph_: *mut MachineGraph,
        signalling_nan_propagation_: SignallingNanPropagation,
    ) -> Self {
        MachineOperatorReducer {
            editor,
            mcgraph_,
            signalling_nan_propagation_,
        }
    }

    pub fn reducer_name(&self) -> &'static str {
        "MachineOperatorReducer"
    }

    pub fn reduce(&mut self, node: *mut Node) -> Reduction {
        unsafe {
            match (*node).opcode() {
                IrOpcode::kProjection => {
                    self.reduce_projection(
                        (*node).op().projection_index(),
                        (*node).input_at(0) as *mut Node,
                    )
                }
                IrOpcode::kWord32And => self.reduce_word32and(node),
                IrOpcode::kWord64And => self.reduce_word64and(node),
                IrOpcode::kWord32Or => self.reduce_word32or(node),
                IrOpcode::kWord64Or => self.reduce_word64or(node),
                IrOpcode::kWord32Xor => self.reduce_word32xor(node),
                IrOpcode::kWord64Xor => self.reduce_word64xor(node),
                IrOpcode::kWord32Shl => self.reduce_word32shl(node),
                IrOpcode::kWord64Shl => self.reduce_word64shl(node),
                IrOpcode::kWord32Shr => self.reduce_word32shr(node),
                IrOpcode::kWord64Shr => self.reduce_word64shr(node),
                IrOpcode::kWord32Sar => self.reduce_word32sar(node),
                IrOpcode::kWord64Sar => self.reduce_word64sar(node),
                IrOpcode::kWord32Ror => {
                    let m = Int32BinopMatcher::new(node as *mut Node);
                    if m.right().is(0) {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.is_foldable() {
                        let left_value = m.left().resolved_value();
                        let right_value = m.right().resolved_value();
                        return Reduction::ReplaceInt32(bits::rotate_right32(
                            left_value,
                            (right_value & 31) as u32,
                        ));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kWord32Equal => self.reduce_word32equal(node),
                IrOpcode::kWord64Equal => self.reduce_word64equal(node),
                IrOpcode::kInt32Add => self.reduce_int32add(node),
                IrOpcode::kInt64Add => self.reduce_int64add(node),
                IrOpcode::kInt32Sub => self.reduce_int32sub(node),
                IrOpcode::kInt64Sub => self.reduce_int64sub(node),
                IrOpcode::kInt32Mul => {
                    let m = Int32BinopMatcher::new(node as *mut Node);
                    if m.right().is(0) {
                        return Reduction::Replace(m.right().node() as *mut Node);
                    }
                    if m.right().is(1) {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.is_foldable() {
                        let left_value = m.left().resolved_value();
                        let right_value = m.right().resolved_value();
                        return Reduction::ReplaceInt32(
                            (left_value as i64 * right_value as i64) as i32,
                        );
                    }
                    if m.right().is(-1) {
                        (*node).replace_input(0, self.int32constant(0) as *mut Node);
                        (*node).replace_input(1, m.left().node() as *mut Node);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().int32add(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    if m.right().is_power_of_2() {
                        let value = m.right().resolved_value();
                        let power = bits::which_power_of_two(value);
                        (*node).replace_input(
                            1,
                            self.int32constant(power) as *mut Node,
                        );
                        (*NodeProperties::cast(node as *mut Node))
                            .change_op(self.machine().word32shl());
                        return Reduction::Changed(node as *mut Node)
                            .followed_by(self.reduce_word32shl(node));
                    }
                    if m.right().has_resolved_value() && (*(m.left().node())).opcode() == IrOpcode::kInt32Mul {
                      let n = Int32BinopMatcher::new(m.left().node() as *mut Node);
                      if n.right().has_resolved_value() && (*NodeProperties::cast(node as *mut Node)).owns_input(m.left().node() as *mut Node) {
                          (*node).replace_input(1, self.int32constant((m.right().resolved_value() as i64 * n.right().resolved_value() as i64) as i32) as *mut Node);
                          (*node).replace_input(0, n.left().node() as *mut Node);
                          return Reduction::Changed(node as *mut Node);
                      }
                    }
                    Reduction::NoChange
                }
                IrOpcode::kInt32MulWithOverflow => {
                  let m = Int32BinopMatcher::new(node as *mut Node);
                  if m.right().is(2) {
                      (*node).replace_input(1, m.left().node() as *mut Node);
                      (*NodeProperties::cast(node as *mut Node)).change_op(
                          self.machine().int32add_with_overflow(),
                      );
                      return Reduction::Changed(node as *mut Node);
                  }
                  if m.right().is(-1) {
                      (*node).replace_input(
                          0,
                          self.int32constant(0) as *mut Node,
                      );
                      (*node).replace_input(1, m.left().node() as *mut Node);
                      (*NodeProperties::cast(node as *mut Node)).change_op(
                          self.machine().int32sub_with_overflow(),
                      );
                      return Reduction::Changed(node as *mut Node);
                  }
                  Reduction::NoChange
              }
                IrOpcode::kInt64Mul => self.reduce_int64mul(node),
                IrOpcode::kInt32Div => self.reduce_int32div(node),
                IrOpcode::kInt64Div => self.reduce_int64div(node),
                IrOpcode::kUint32Div => self.reduce_uint32div(node),
                IrOpcode::kUint64Div => self.reduce_uint64div(node),
                IrOpcode::kInt32Mod => self.reduce_int32mod(node),
                IrOpcode::kInt64Mod => self.reduce_int64mod(node),
                IrOpcode::kUint32Mod => self.reduce_uint32mod(node),
                IrOpcode::kUint64Mod => self.reduce_uint64mod(node),
                IrOpcode::kInt32LessThan => {
                    let m = Int32BinopMatcher::new(node as *mut Node);
                    if m.is_foldable() {
                        return Reduction::ReplaceBool(
                            m.left().resolved_value() < m.right().resolved_value(),
                        );
                    }
                    if m.left_equals_right() {
                        return Reduction::ReplaceBool(false);
                    }
                    if (*(m.left().node())).opcode() == IrOpcode::kWord32Or && m.right().is(0) {
                      let mleftmatcher = Int32BinopMatcher::new(m.left().node() as *mut Node);
                      if mleftmatcher.left().is_negative() || mleftmatcher.right().is_negative() {
                        return Reduction::ReplaceBool(true);
                      }
                    }
                    self.reduce_word32comparisons(node)
                }
                IrOpcode::kInt32LessThanOrEqual => {
                    let m = Int32BinopMatcher::new(node as *mut Node);
                    if (m.is_foldable()) {
                        return Reduction::ReplaceBool(
                            m.left().resolved_value() <= m.right().resolved_value(),
                        );
                    }
                    if (m.left_equals_right() {
                        return Reduction::ReplaceBool(true);
                    }
                    self.reduce_word32comparisons(node)
                }
                IrOpcode::kUint32LessThan => {
                    let m = Uint32BinopMatcher::new(node as *mut Node);
                    if m.left().is(kMaxUInt32 as i32) {
                        return Reduction::ReplaceBool(false);
                    }
                    if m.right().is(0) {
                        return Reduction::ReplaceBool(false);
                    }
                    if (m.is_foldable() {
                        return Reduction::ReplaceBool(
                            m.left().resolved_value() < m.right().resolved_value(),
                        );
                    }
                    if (m.left_equals_right() {
                        return Reduction::ReplaceBool(false);
                    }
                    if (*(m.left().node())).opcode() == IrOpcode::kWord32Sar && m.right().has_resolved_value() {
                      let mleft = Int32BinopMatcher::new(m.left().node() as *mut Node);
                      if mleft.right().has_resolved_value() {
                          let c = m.right().resolved_value() as u32;
                          let k = mleft.right().resolved_value() as u32 & 0x1F;
                          if c < (kMaxInt >> k) as u32 {
                              (*node).replace_input(0, mleft.left().node() as *mut Node);
                              (*node).replace_input(1, self.uint32constant(c << k) as *mut Node);
                              return Reduction::Changed(node as *mut Node);
                          }
                      }
                    }
                    self.reduce_word32comparisons(node)
                }
                IrOpcode::kUint32LessThanOrEqual => {
                    self.reduce_uintn_less_than_or_equal::<Word32Adapter>(node)
                }
                IrOpcode::kFloat32Sub => {
                    let m = Float32BinopMatcher::new(node as *mut Node);
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.right().is(0)
                        && f64::copysign(1.0, m.right().resolved_value() as f64) > 0.0
                    {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat32(silence_nan(m.right().resolved_value() as f32));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat32(silence_nan(m.left().resolved_value() as f32));
                    }
                    if m.is_foldable() {
                        return Reduction::ReplaceFloat32(
                            m.left().resolved_value() - m.right().resolved_value(),
                        );
                    }
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.left().is_minus_zero()
                    {
                        if self.machine().float32round_up().is_supported()
                            && m.right().is_float32round_down()
                        {
                            if (*(m.right().input_at(0))).opcode() == IrOpcode::kFloat32Sub {
                                let mright0 = Float32BinopMatcher::new(m.right().input_at(0) as *mut Node);
                                if mright0.left().is_minus_zero() {
                                    return Reduction::Replace(
                                        (*self.graph()).new_node(
                                            self.machine().float32round_up().op(),
                                            mright0.right().node() as *mut Node,
                                        ) as *mut Node,
                                    );
                                }
                            }
                        }
                        (*node).remove_input(0);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float32neg(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Add => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.left().resolved_value()));
                    }
                    if m.is_foldable() {
                        return Reduction::ReplaceFloat64(
                            m.left().resolved_value() + m.right().resolved_value(),
                        );
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Sub => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.right().is(0)
                        && (*(base::Double::new(m.right().resolved_value()))).sign() > 0
                    {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.left().resolved_value()));
                    }
                    if m.is_foldable() {
                        return Reduction::ReplaceFloat64(
                            m.left().resolved_value() - m.right().resolved_value(),
                        );
                    }
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.left().is_minus_zero()
                    {
                        if self.machine().float64round_up().is_supported()
                            && m.right().is_float64round_down()
                        {
                            if (*(m.right().input_at(0))).opcode() == IrOpcode::kFloat64Sub {
                                let mright0 = Float64BinopMatcher::new(m.right().input_at(0) as *mut Node);
                                if mright0.left().is_minus_zero() {
                                    return Reduction::Replace(
                                        (*self.graph()).new_node(
                                            self.machine().float64round_up().op(),
                                            mright0.right().node() as *mut Node,
                                        ) as *mut Node,
                                    );
                                }
                            }
                        }
                        (*node).remove_input(0);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float64neg(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Mul => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.right().is(1)
                    {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.right().is(-1) {
                        (*node).replace_input(
                            0,
                            self.float64constant(-0.0) as *mut Node,
                        );
                        (*node).replace_input(1, m.left().node() as *mut Node);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float64sub(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.is_foldable() {
                        return Reduction::ReplaceFloat64(
                            m.left().resolved_value() * m.right().resolved_value(),
                        );
                    }
                    if m.right().is(2) {
                        (*node).replace_input(1, m.left().node() as *mut Node);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float64add(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Div => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.right().is(1)
                    {
                        return Reduction::Replace(m.left().node() as *mut Node);
                    }
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.left().resolved_value()));
                    }
                    if (m.is_foldable()) {
                      return Reduction::ReplaceFloat64(
                          base::Divide(m.left().resolved_value(), m.right().resolved_value()),
                      );
                    }
                    if self.signalling_nan_propagation_ == SignallingNanPropagation::kPropagateSignallingNan
                        && m.right().is(-1)
                    {
                        (*node).remove_input(1);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float64neg(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    }
                    if m.right().is_normal() && m.right().is_positive_or_negative_power_of_2() {
                      (*node).replace_input(
                          1,
                          self.float64constant(1.0 / m.right().resolved_value()) as *mut Node,
                      );
                      (*NodeProperties::cast(node as *mut Node)).change_op(
                          self.machine().float64mul(),
                      );
                      return Reduction::Changed(node as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Mod => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if m.right().is(0) {
                        return Reduction::ReplaceFloat64(f64::NAN);
                    }
                    if m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.left().resolved_value()));
                    }
                    if m.is_foldable() {
                      return Reduction::ReplaceFloat64(Modulo(m.left().resolved_value(), m.right().resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Acos => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::acos(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Acosh => {
                  let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                  if (m.has_resolved_value()) {
                    return Reduction::ReplaceFloat64(base::ieee754::acosh(m.resolved_value()));
                  }
                  Reduction::NoChange
                }
                IrOpcode::kFloat64Asin => {
                  let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                  if (m.has_resolved_value()) {
                    return Reduction::ReplaceFloat64(base::ieee754::asin(m.resolved_value()));
                  }
                  Reduction::NoChange
                }
                IrOpcode::kFloat64Asinh => {
                  let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                  if (m.has_resolved_value()) {
                    return Reduction::ReplaceFloat64(base::ieee754::asinh(m.resolved_value()));
                  }
                  Reduction::NoChange
                }
                IrOpcode::kFloat64Atan => {
                  let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                  if (m.has_resolved_value()) {
                    return Reduction::ReplaceFloat64(base::ieee754::atan(m.resolved_value()));
                  }
                  Reduction::NoChange
                }
                IrOpcode::kFloat64Atanh => {
                  let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                  if (m.has_resolved_value()) {
                    return Reduction::ReplaceFloat64(base::ieee754::atanh(m.resolved_value()));
                  }
                  Reduction::NoChange
                }
                IrOpcode::kFloat64Atan2 => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if (m.right().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.right().resolved_value()));
                    }
                    if m.left().is_nan() {
                        return Reduction::ReplaceFloat64(silence_nan(m.left().resolved_value()));
                    }
                    if m.is_foldable() {
                      return Reduction::ReplaceFloat64(base::ieee754::atan2(m.left().resolved_value(), m.right().resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Cbrt => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::cbrt(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Cos => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::cos(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Cosh => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::cosh(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Exp => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::exp(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Expm1 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::expm1(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Log => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::log(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Log1p => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::log1p(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Log10 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::log10(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Log2 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::log2(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Pow => {
                    let m = Float64BinopMatcher::new(node as *mut Node);
                    if (m.is_foldable() {
                        return Reduction::ReplaceFloat64(
                            f64::powf(m.left().resolved_value(), m.right().resolved_value()),
                        );
                    } else if m.right().is(0.0) {
                        return Reduction::ReplaceFloat64(1.0);
                    } else if m.right().is(2.0) {
                        (*node).replace_input(1, m.left().node() as *mut Node);
                        (*NodeProperties::cast(node as *mut Node)).change_op(
                            self.machine().float64mul(),
                        );
                        return Reduction::Changed(node as *mut Node);
                    } else if m.right().is(0.5) {
                      return Reduction::Replace(self.float64_pow_half(m.left().node() as *mut Node) as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Sin => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::sin(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Sinh => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::sinh(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Tan => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::tan(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kFloat64Tanh => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if (m.has_resolved_value()) {
                      return Reduction::ReplaceFloat64(base::ieee754::tanh(m.resolved_value()));
                    }
                    Reduction::NoChange
                }
                IrOpcode::kChangeFloat32ToFloat64 => {
                    let m = Float32Matcher::new((*node).input_at(0) as *mut Node);
                    if m.has_resolved_value() {
                        if self.signalling_nan_propagation_ == SignallingNanPropagation::kSilenceSignallingNan && m.is_nan()
                        {
                            return Reduction::ReplaceFloat64(silence_nan(m.resolved_value() as f64));
                        }
                        return Reduction::ReplaceFloat64(m.resolved_value() as f64);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kChangeFloat64ToInt32 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if m.has_resolved_value() {
                        return Reduction::ReplaceInt32(conversions_inl::double_to_int32(m.resolved_value()));
                    }
                    if m.is_change_int32to_float64() {
                        return Reduction::Replace(m.node() as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kChangeFloat64ToInt64 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if m.has_resolved_value() {
                        return Reduction::ReplaceInt64(m.resolved_value() as i64);
                    }
                    if m.is_change_int64to_float64() {
                        return Reduction::Replace(m.node() as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kChangeFloat64ToUint32 => {
                    let m = Float64Matcher::new((*node).input_at(0) as *mut Node);
                    if m.has_resolved_value() {
                        return Reduction::ReplaceInt32(conversions_inl::fast_d2ui(m.resolved_value()));
                    }
                    if m.is_change_uint32to_float64() {
                        return Reduction::Replace(m.node() as *mut Node);
                    }
                    Reduction::NoChange
                }
                IrOpcode::kChangeInt32ToFloat64 => {
                    let m = Int32Matcher::new((*node).input_at(0) as *mut Node);
                    if m.has_resolved_value() {
                        return Reduction::ReplaceFloat64(conversions_inl::fast_i2d(m.resolved_
