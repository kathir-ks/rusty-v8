// Converted from V8 C++ source files:
// Header: representation-change.h
// Implementation: representation-change.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod representation_change {
use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::feedback_source::FeedbackSource;
use crate::compiler::js_graph::JSGraph;
use crate::compiler::simplified_operator::SimplifiedOperatorBuilder;
use crate::compiler::use_info::UseInfo;
use crate::execution::isolate::Isolate;
use crate::objects::code::Code;
use crate::objects::maybe_object::MaybeObject;
use crate::deoptimizer::deoptimizer::DeoptimizeReason;
use crate::compiler::turbofan_types::Type;
use crate::compiler::csa_load_elimination::MachineType;
use crate::compiler::machine_operator::MachineOperatorBuilder;
use crate::compiler::node_matchers::HeapObjectMatcher;
use crate::bigint::vector_arithmetic::digit_t;
use crate::heap::factory_inl::Factory;
use crate::compiler::node_matchers::NumberMatcher;
use crate::compiler::simplified_lowering_verifier::SimplifiedLoweringVerifier;
use crate::compiler::js_heap_broker::JSHeapBroker;
use crate::compiler::type_cache::TypeCache;
use crate::compiler::operator::Operator;
use crate::objects::heap_object::HeapObject;
use crate::objects::string::String;
use crate::objects::bigint::BigInt;
use crate::handles::handles::MaybeIndirectHandle;
use crate::handles::handles::Handle;
use crate::handles::handles::Weak;
use crate::zone::zone::Zone;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TruncationKind {
    kNone,
    kBool,
    kWord32,
    kWord64,
    kOddballAndBigIntToNumber,
    kAny,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum IdentifyZeros {
    kIdentifyZeros,
    kDistinguishZeros,
}

#[derive(Debug, Copy, Clone)]
pub struct Truncation {
    kind: TruncationKind,
    identify_zeros: IdentifyZeros,
}

impl Truncation {
    pub fn new(kind: TruncationKind, identify_zeros: IdentifyZeros) -> Self {
        Truncation { kind, identify_zeros }
    }

    pub fn none() -> Self {
        Truncation {
            kind: TruncationKind::kNone,
            identify_zeros: IdentifyZeros::kDistinguishZeros,
        }
    }

    pub fn bool() -> Self {
        Truncation {
            kind: TruncationKind::kBool,
            identify_zeros: IdentifyZeros::kDistinguishZeros,
        }
    }

    pub fn word32() -> Self {
        Truncation {
            kind: TruncationKind::kWord32,
            identify_zeros: IdentifyZeros::kDistinguishZeros,
        }
    }

    pub fn word64() -> Self {
        Truncation {
            kind: TruncationKind::kWord64,
            identify_zeros: IdentifyZeros::kDistinguishZeros,
        }
    }

    pub fn oddball_and_bigint_to_number(identify_zeros: IdentifyZeros) -> Self {
        Truncation {
            kind: TruncationKind::kOddballAndBigIntToNumber,
            identify_zeros,
        }
    }

    pub fn any(identify_zeros: IdentifyZeros) -> Self {
        Truncation {
            kind: TruncationKind::kAny,
            identify_zeros,
        }
    }

    pub fn kind(&self) -> TruncationKind {
        self.kind
    }

    pub fn identify_zeros(&self) -> IdentifyZeros {
        self.identify_zeros
    }

    pub fn description(&self) -> &'static str {
        match self.kind() {
            TruncationKind::kNone => "no-value-use",
            TruncationKind::kBool => "truncate-to-bool",
            TruncationKind::kWord32 => "truncate-to-word32",
            TruncationKind::kWord64 => "truncate-to-word64",
            TruncationKind::kOddballAndBigIntToNumber => match self.identify_zeros() {
                IdentifyZeros::kIdentifyZeros => "truncate-oddball&bigint-to-number (identify zeros)",
                IdentifyZeros::kDistinguishZeros => "truncate-oddball&bigint-to-number (distinguish zeros)",
            },
            TruncationKind::kAny => match self.identify_zeros() {
                IdentifyZeros::kIdentifyZeros => "no-truncation (but identify zeros)",
                IdentifyZeros::kDistinguishZeros => "no-truncation (but distinguish zeros)",
            },
        }
    }

    pub fn generalize(rep1: TruncationKind, rep2: TruncationKind) -> TruncationKind {
        if Truncation::less_general(rep1, rep2) {
            return rep2;
        }
        if Truncation::less_general(rep2, rep1) {
            return rep1;
        }
        if Truncation::less_general(rep1, TruncationKind::kOddballAndBigIntToNumber)
            && Truncation::less_general(rep2, TruncationKind::kOddballAndBigIntToNumber)
        {
            return TruncationKind::kOddballAndBigIntToNumber;
        }
        if Truncation::less_general(rep1, TruncationKind::kAny)
            && Truncation::less_general(rep2, TruncationKind::kAny)
        {
            return TruncationKind::kAny;
        }
        panic!("Tried to combine incompatible truncations");
    }

    pub fn generalize_identify_zeros(i1: IdentifyZeros, i2: IdentifyZeros) -> IdentifyZeros {
        if i1 == i2 {
            return i1;
        } else {
            return IdentifyZeros::kDistinguishZeros;
        }
    }

    pub fn less_general(rep1: TruncationKind, rep2: TruncationKind) -> bool {
        match rep1 {
            TruncationKind::kNone => true,
            TruncationKind::kBool => rep2 == TruncationKind::kBool || rep2 == TruncationKind::kAny,
            TruncationKind::kWord32 => {
                rep2 == TruncationKind::kWord32
                    || rep2 == TruncationKind::kWord64
                    || rep2 == TruncationKind::kOddballAndBigIntToNumber
                    || rep2 == TruncationKind::kAny
            }
            TruncationKind::kWord64 => {
                rep2 == TruncationKind::kWord64
                    || rep2 == TruncationKind::kOddballAndBigIntToNumber
                    || rep2 == TruncationKind::kAny
            }
            TruncationKind::kOddballAndBigIntToNumber => {
                rep2 == TruncationKind::kOddballAndBigIntToNumber || rep2 == TruncationKind::kAny
            }
            TruncationKind::kAny => rep2 == TruncationKind::kAny,
        }
    }

    pub fn less_general_identify_zeros(i1: IdentifyZeros, i2: IdentifyZeros) -> bool {
        i1 == i2 || i1 == IdentifyZeros::kIdentifyZeros
    }

     pub fn identifies_zero_and_minus_zero(&self) -> bool {
        self.identify_zeros == IdentifyZeros::kIdentifyZeros
    }

     pub fn truncates_oddball_and_bigint_to_number(&self) -> bool {
        self.kind == TruncationKind::kOddballAndBigIntToNumber
    }

    pub fn is_used_as_word32(&self) -> bool {
        self.kind == TruncationKind::kWord32
    }

    pub fn any_with_identify_zeros(identify_zeros: IdentifyZeros) -> Self {
        Truncation {
            kind: TruncationKind::kAny,
            identify_zeros,
        }
    }
}

#[allow(dead_code)]
mod detail {
    use super::*;

    pub fn is_word(rep: MachineRepresentation) -> bool {
        rep == MachineRepresentation::kWord8
            || rep == MachineRepresentation::kWord16
            || rep == MachineRepresentation::kWord32
    }

    pub fn type_check_is_bigint(type_check: TypeCheckKind) -> bool {
        type_check == TypeCheckKind::kBigInt || type_check == TypeCheckKind::kBigInt64
    }

    pub const fn supports_fp_params_in_c_linkage() -> bool {
        cfg!(target_arch = "x86_64")
    }
}

use detail::*;

pub struct RepresentationChanger {
    cache_: *const TypeCache, // Assuming TypeCache is thread-safe, use a raw pointer
    jsgraph_: *mut JSGraph, // Raw pointer because JSGraph is mutable and heavily used
    broker_: *mut JSHeapBroker, // Raw pointer because JSHeapBroker is mutable and heavily used
    verifier_: *mut SimplifiedLoweringVerifier, // Raw pointer if SimplifiedLoweringVerifier is mutable
    testing_type_errors_: bool,
    type_error_: bool,
    ieee754_fp16_raw_bits_to_fp32_raw_bits_code_: SetOncePointer<Node>,
    ieee754_fp64_to_fp16_raw_bits_code_: SetOncePointer<Node>,
    ieee754_fp16_raw_bits_to_fp32_raw_bits_operator_: SetOncePointer<Operator>,
    ieee754_fp64_to_fp16_raw_bits_operator_: SetOncePointer<Operator>,
}

unsafe impl Send for RepresentationChanger {}
unsafe impl Sync for RepresentationChanger {}

impl RepresentationChanger {
    pub fn new(
        jsgraph: *mut JSGraph,
        broker: *mut JSHeapBroker,
        verifier: *mut SimplifiedLoweringVerifier,
    ) -> Self {
          let type_cache = TypeCache::get(); // Call the static method to get the instance
        RepresentationChanger {
            cache_: type_cache, // Store the pointer
            jsgraph_: jsgraph,
            broker_: broker,
            verifier_: verifier,
            testing_type_errors_: false,
            type_error_: false,
            ieee754_fp16_raw_bits_to_fp32_raw_bits_code_: SetOncePointer::new(),
            ieee754_fp64_to_fp16_raw_bits_code_: SetOncePointer::new(),
            ieee754_fp16_raw_bits_to_fp32_raw_bits_operator_: SetOncePointer::new(),
            ieee754_fp64_to_fp16_raw_bits_operator_: SetOncePointer::new(),
        }
    }

    pub fn get_representation_for(
        &mut self,
        node: *mut Node,
        output_rep: MachineRepresentation,
        output_type: Type,
        use_node: *mut Node,
        use_info: UseInfo,
    ) -> *mut Node {
        unsafe {
            if !output_type.is_none() && !output_type.is(Type::machine()) {
               
            }

            if output_rep == MachineRepresentation::kNone && !output_type.is_none() {
                return self.type_error(node, output_rep, output_type, use_info.representation());
            }

             if output_type.is(Type::bigint())
                && output_rep == MachineRepresentation::kWord64
                && !type_check_is_bigint(use_info.type_check())
            {
                 if output_type.is(Type::unsigned_bigint64()) {
                     let change_uint64_to_bigint = (*self.jsgraph_).simplified().change_uint64_to_bigint();
                     return self.insert_conversion(node, change_uint64_to_bigint, use_node);
                 } else {
                    let change_int64_to_bigint = (*self.jsgraph_).simplified().change_int64_to_bigint();
                     return self.insert_conversion(node, change_int64_to_bigint, use_node);
                 }
                
            } else if output_rep == MachineRepresentation::kFloat16RawBits {
                if (*(*self.jsgraph_).machine()).change_float16_raw_bits_to_float64().is_supported() {
                     let change_float16_raw_bits_to_float64_op = (*(*self.jsgraph_).machine()).change_float16_raw_bits_to_float64().op();
                   (*self.jsgraph_).graph().new_node(change_float16_raw_bits_to_float64_op, node);
                } else {
                    return self.insert_change_float16_raw_bits_to_float64_fallback(node);
                }
            }

             if use_info.type_check() == TypeCheckKind::kNone
                || (output_rep != MachineRepresentation::kWord32
                    && !type_check_is_bigint(use_info.type_check()))
            {
                if use_info.representation() == output_rep {
                    return node;
                }
                if is_word(use_info.representation()) && is_word(output_rep) {
                    return node;
                }
            }

            match use_info.representation() {
                MachineRepresentation::kTaggedSigned => {
                    if use_info.type_check() != TypeCheckKind::kNone
                        && use_info.type_check() != TypeCheckKind::kSignedSmall
                    {}
                    self.get_tagged_signed_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_node,
                        use_info,
                    )
                }
                MachineRepresentation::kTaggedPointer => {
                    if use_info.type_check() != TypeCheckKind::kNone
                        && use_info.type_check() != TypeCheckKind::kHeapObject
                        && use_info.type_check() != TypeCheckKind::kBigInt
                    {}
                    self.get_tagged_pointer_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_node,
                        use_info,
                    )
                }
                MachineRepresentation::kTagged => {
                    if TypeCheckKind::kNone != use_info.type_check() {}
                    self.get_tagged_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_info.truncation(),
                    )
                }
                MachineRepresentation::kFloat16RawBits => {
                    if TypeCheckKind::kNone != use_info.type_check() {}
                    self.get_float16_raw_bits_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_node,
                        use_info,
                    )
                }
                MachineRepresentation::kFloat32 => {
                    if TypeCheckKind::kNone != use_info.type_check() {}
                    self.get_float32_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_info.truncation(),
                    )
                }
                MachineRepresentation::kFloat64 => {
                    if use_info.type_check() != TypeCheckKind::kNone
                        && use_info.type_check() != TypeCheckKind::kNumber
                        && use_info.type_check() != TypeCheckKind::kNumberOrBoolean
                        && use_info.type_check() != TypeCheckKind::kNumberOrOddball
                    {}
                    self.get_float64_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_node,
                        use_info,
                    )
                }
                MachineRepresentation::kBit => {
                    if TypeCheckKind::kNone != use_info.type_check() {}
                    self.get_bit_representation_for(node, output_rep, output_type)
                }
                MachineRepresentation::kWord8
                | MachineRepresentation::kWord16
                | MachineRepresentation::kWord32 => self.get_word32_representation_for(
                    node,
                    output_rep,
                    output_type,
                    use_node,
                    use_info,
                ),
                MachineRepresentation::kWord64 => {
                    if use_info.type_check() != TypeCheckKind::kNone
                        && use_info.type_check() != TypeCheckKind::kSigned64
                        && use_info.type_check() != TypeCheckKind::kAdditiveSafeInteger
                        && !type_check_is_bigint(use_info.type_check())
                        && use_info.type_check() != TypeCheckKind::kArrayIndex
                    {}
                    self.get_word64_representation_for(
                        node,
                        output_rep,
                        output_type,
                        use_node,
                        use_info,
                    )
                }
                MachineRepresentation::kSimd128
                | MachineRepresentation::kSimd256
                | MachineRepresentation::kNone => node,
                MachineRepresentation::kFloat16
                | MachineRepresentation::kCompressed
                | MachineRepresentation::kCompressedPointer
                | MachineRepresentation::kSandboxedPointer
                | MachineRepresentation::kProtectedPointer
                | MachineRepresentation::kIndirectPointer
                | MachineRepresentation::kMapWord => {
                   panic!();
                }
            }
        }
    }

     fn get_tagged_signed_representation_for(
        &mut self,
        node: *mut Node,
        output_rep: MachineRepresentation,
        output_type: Type,
        use_node: *mut Node,
        use_info: UseInfo,
    ) -> *mut Node {
        unsafe {
           match (*node).opcode() {
                IrOpcode::kNumberConstant => {
                    if output_type.is(Type::signed_small()) {
                        return node;
                    }
                }
                _ => {}
            }
            let op;
            if output_type.is(Type::none()) {
               let dead_value = (*(*self.jsgraph_).common()).dead_value(MachineRepresentation::kTaggedSigned);
                return (*self.jsgraph_).graph().new_node(
                    dead_value,
                    node,
                );
            } else if is_word(output_rep) {
                if output_type.is(Type::signed31()) {
                    op = (*self.jsgraph_).simplified().change_int31_to_tagged_signed();
                } else if output_type.is(Type::signed32()) {
                    if smi_values_are32_bits() {
                         op = (*self.jsgraph_).simplified().change_int32_to_tagged();
                    } else if use_info.type_check() == TypeCheckKind::kSignedSmall {
                         op = (*self.jsgraph_).simplified().checked_int32_to_tagged_signed(use_info.feedback());
                    } else {
                        return self.type_error(
                            node,
                            output_rep,
                            output_type,
                            MachineRepresentation::kTaggedSigned,
                        );
                    }
                } else if output_type.is(Type::unsigned32())
                    && use_info.type_check() == TypeCheckKind::kSignedSmall
                {
                     op = (*self.jsgraph_).simplified().checked_uint32_to_tagged_signed(use_info.feedback());
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else if output_rep == MachineRepresentation::kWord64 {
                if output_type.is(Type::signed31()) {
                    self.insert_truncate_int64_to_int32(node);
                    op = (*self.jsgraph_).simplified().change_int31_to_tagged_signed();
                } else if output_type.is(Type::signed32()) && smi_values_are32_bits() {
                    self.insert_truncate_int64_to_int32(node);
                    op = (*self.jsgraph_).simplified().change_int32_to_tagged();
                } else if use_info.type_check() == TypeCheckKind::kSignedSmall {
                    if output_type.is( (*self.cache_).k_positive_safe_integer)) {
                         op = (*self.jsgraph_).simplified().checked_uint64_to_tagged_signed(use_info.feedback());
                    } else if output_type.is( (*self.cache_).k_safe_integer)) {
                         op = (*self.jsgraph_).simplified().checked_int64_to_tagged_signed(use_info.feedback());
                    } else {
                        return self.type_error(
                            node,
                            output_rep,
                            output_type,
                            MachineRepresentation::kTaggedSigned,
                        );
                    }
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else if output_rep == MachineRepresentation::kFloat64 {
                if output_type.is(Type::signed31()) {
                    self.insert_change_float64_to_int32(node);
                     op = (*self.jsgraph_).simplified().change_int31_to_tagged_signed();
                } else if output_type.is(Type::signed32()) {
                    self.insert_change_float64_to_int32(node);
                    if smi_values_are32_bits() {
                         op = (*self.jsgraph_).simplified().change_int32_to_tagged();
                    } else if use_info.type_check() == TypeCheckKind::kSignedSmall {
                         op = (*self.jsgraph_).simplified().checked_int32_to_tagged_signed(use_info.feedback());
                    } else {
                        return self.type_error(
                            node,
                            output_rep,
                            output_type,
                            MachineRepresentation::kTaggedSigned,
                        );
                    }
                } else if output_type.is(Type::unsigned32())
                    && use_info.type_check() == TypeCheckKind::kSignedSmall
                {
                    self.insert_change_float64_to_uint32(node);
                     op = (*self.jsgraph_).simplified().checked_uint32_to_tagged_signed(use_info.feedback());
                } else if use_info.type_check() == TypeCheckKind::kSignedSmall {
                   let check_mode = if output_type.maybe(Type::minus_zero()) {
                        CheckForMinusZeroMode::kCheckForMinusZero
                    } else {
                        CheckForMinusZeroMode::kDontCheckForMinusZero
                    };
                    self.insert_checked_float64_to_int32(node, check_mode, use_info.feedback(), use_node);
                    if smi_values_are32_bits() {
                         op = (*self.jsgraph_).simplified().change_int32_to_tagged();
                    } else {
                         op = (*self.jsgraph_).simplified().checked_int32_to_tagged_signed(use_info.feedback());
                    }
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else if output_rep == MachineRepresentation::kFloat32 {
                if use_info.type_check() == TypeCheckKind::kSignedSmall {
                    self.insert_change_float32_to_float64(node);
                    let check_mode = if output_type.maybe(Type::minus_zero()) {
                        CheckForMinusZeroMode::kCheckForMinusZero
                    } else {
                        CheckForMinusZeroMode::kDontCheckForMinusZero
                    };
                    self.insert_checked_float64_to_int32(node, check_mode, use_info.feedback(), use_node);
                    if smi_values_are32_bits() {
                         op = (*self.jsgraph_).simplified().change_int32_to_tagged();
                    } else {
                         op = (*self.jsgraph_).simplified().checked_int32_to_tagged_signed(use_info.feedback());
                    }
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else if can_be_tagged_pointer(output_rep) {
                if use_info.type_check() == TypeCheckKind::kSignedSmall {
                    op = (*self.jsgraph_).simplified().checked_tagged_to_tagged_signed(use_info.feedback());
                } else if output_type.is(Type::signed_small()) {
                     op = (*self.jsgraph_).simplified().change_tagged_to_tagged_signed();
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else if output_rep == MachineRepresentation::kBit {
                if use_info.type_check() == TypeCheckKind::kSignedSmall {
                    self.insert_change_bit_to_tagged(node);
                     op = (*self.jsgraph_).simplified().checked_tagged_to_tagged_signed(use_info.feedback());
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedSigned,
                    );
                }
            } else {
                return self.type_error(
                    node,
                    output_rep,
                    output_type,
                    MachineRepresentation::kTaggedSigned,
                );
            }
            self.insert_conversion(node, op, use_node)
        }
    }

    fn get_tagged_pointer_representation_for(
        &mut self,
        node: *mut Node,
        output_rep: MachineRepresentation,
        output_type: Type,
        use_node: *mut Node,
        use_info: UseInfo,
    ) -> *mut Node {
        unsafe {
            match (*node).opcode() {
                IrOpcode::kHeapConstant => {
                    if type_check_is_bigint(use_info.type_check()) {

                    }
                    return node;
                }
                IrOpcode::kInt32Constant
                | IrOpcode::kFloat64Constant
                | IrOpcode::kFloat32Constant => panic!(),
                _ => {}
            }
            let op;
            if output_type.is(Type::none()) {
                let dead_value = (*(*self.jsgraph_).common()).dead_value(MachineRepresentation::kTaggedPointer);
                return (*self.jsgraph_).graph().new_node(
                    dead_value,
                    node,
                );
            }
            if type_check_is_bigint(use_info.type_check()) && !output_type.is(Type::bigint()) {
                if !can_be_tagged_pointer(output_rep) {
                    let unreachable = self.insert_unconditional_deopt(
                        use_node,
                        DeoptimizeReason::kNotABigInt,
                    );
                    let dead_value = (*(*self.jsgraph_).common()).dead_value(MachineRepresentation::kTaggedPointer);
                    return (*self.jsgraph_).graph().new_node(
                        dead_value,
                        unreachable,
                    );
                }
            }
             if output_rep == MachineRepresentation::kBit {
                if output_type.is(Type::boolean()) {
                     op = (*self.jsgraph_).simplified().change_bit_to_tagged();
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTagged,
                    );
                }
            } else if is_word(output_rep) {
                 if output_type.is(Type::unsigned32()) {
                    self.insert_change_uint32_to_float64(node);
                } else if output_type.is(Type::signed32()) {
                    self.insert_change_int32_to_float64(node);
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedPointer,
                    );
                }
                 op = (*self.jsgraph_).simplified().change_float64_to_tagged_pointer();
            } else if output_rep == MachineRepresentation::kWord64 {
                 if output_type.is( (*self.cache_).k_safe_integer)) {
                     let change_int64_to_float64 = (*(*self.jsgraph_).machine()).change_int64_to_float64();
                    (*self.jsgraph_).graph().new_node(change_int64_to_float64, node);
                     op = (*self.jsgraph_).simplified().change_float64_to_tagged_pointer();
                } else if output_type.is(Type::signed_bigint64())
                    && use_info.type_check() == TypeCheckKind::kBigInt
                {
                     op = (*self.jsgraph_).simplified().change_int64_to_bigint();
                } else if output_type.is(Type::unsigned_bigint64())
                    && use_info.type_check() == TypeCheckKind::kBigInt
                {
                     op = (*self.jsgraph_).simplified().change_uint64_to_bigint();
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedPointer,
                    );
                }
            } else if output_rep == MachineRepresentation::kFloat32 {
                 if output_type.is(Type::number()) {
                    self.insert_change_float32_to_float64(node);
                     op = (*self.jsgraph_).simplified().change_float64_to_tagged_pointer();
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedPointer,
                    );
                }
            } else if output_rep == MachineRepresentation::kFloat64 {
                 if output_type.is(Type::number()) {
                     op = (*self.jsgraph_).simplified().change_float64_to_tagged_pointer();
                } else {
                    return self.type_error(
                        node,
                        output_rep,
                        output_type,
                        MachineRepresentation::kTaggedPointer,
                    );
                }
            } else if is_any_tagged(output_rep) {
                 if use_info.type_check() == TypeCheckKind::kBigInt {
                    if output_type.is(Type::bigint()) {
                        if output_rep != MachineRepresentation::kTaggedSigned {}
                        return node;
                    }
                     op = (*self.jsgraph_).simplified().check_bigint(use_info.feedback());
                } else if use_info.type_check() == TypeCheckKind::kBigInt64 {
                    if output_type.is(Type::signed_bigint64()) {
                        if output_rep != MachineRepresentation::kTaggedSigned {}
                        return node;
                    }
                     if !output_type.is(Type::bigint()) {
                       let check_bigint = (*self.jsgraph_).simplified().check_bigint(use_info.feedback());
                         self.insert_conversion(node, check_bigint, use_node);
                    }
                     op = (*self.jsgraph_).simplified().checked_bigint_to_bigint64(use_info.feedback());
                } else if output_rep == MachineRepresentation::kTaggedPointer
                    || !output_type.maybe(Type::signed_small())
                {
                    if output_rep != MachineRepresentation::kTaggedSigned {}
                    return node;
                } else {
                    op = (*self.jsgraph_).simplified().checked_tagged_to_tagged_pointer(use_info.feedback());
                }
            } else {
                return self.type_error(
                    node,
                    output_rep,
                    output_type,
                    MachineRepresentation::kTaggedPointer,
                );
            }
            self.insert_conversion(node, op, use_node)
        }
    }

    fn get_tagged_representation_for(
        &mut self,
        node: *mut Node,
        output_rep: MachineRepresentation,
        output_type: Type,
        truncation: Truncation,
    ) -> *mut Node {
        unsafe {
            match (*node).opcode() {
                IrOpcode::kNumberConstant | IrOpcode::kHeapConstant => return node,
                IrOpcode::kInt32Constant
                | IrOpcode::kFloat64Constant
                | IrOpcode::kFloat32Constant => panic!(),
                _ => {}
            }

             if output_rep == MachineRepresentation::kTaggedSigned
                || output_rep == MachineRepresentation::kTaggedPointer
                || output_rep == MachineRepresentation::kMapWord
            {
                return node;
            }
             let op;
            if output_type.is(Type::none()) {
                let dead_value = (*(*self.jsgraph_).common()).dead_value(MachineRepresentation::kTagged);
                return (*self.jsgraph_).graph().new_node(

