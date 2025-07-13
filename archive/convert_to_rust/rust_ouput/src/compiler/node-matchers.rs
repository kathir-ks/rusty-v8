// Converted from V8 C++ source files:
// Header: node-matchers.h
// Implementation: node-matchers.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
    use crate::base::{bounds::IsInRange, compiler_specific::NO_RETURN, Double};
    use crate::codegen::external_reference::ExternalReference;
    use crate::common::globals::FLAG_debug_code;
    use crate::compiler::{
        common_operator::Operator,
        machine_operator::{AtomicMemoryOrder, MachineRepresentation},
        node::Node,
        opcodes::IrOpcode,
    };
    use std::{
        cmp::Ordering,
        f64::NAN,
        fmt::{Debug, Display},
        hash::Hash,
        marker::PhantomData,
        mem::MaybeUninit,
        ops::{Add, Div, Mul, Neg, Rem, Shl, Shr, Sub},
        sync::atomic::AtomicPtr,
    };

    pub struct JSHeapBroker;

    pub struct NodeMatcher<'a> {
        node_: *mut Node,
        _phantom: PhantomData<&'a Node>,
    }

    impl<'a> NodeMatcher<'a> {
        pub fn new(node: *mut Node) -> Self {
            NodeMatcher {
                node_: node,
                _phantom: PhantomData,
            }
        }

        pub fn node(&self) -> &Node {
            unsafe { &*self.node_ }
        }

        pub fn op(&self) -> &Operator {
            self.node().op()
        }

        pub fn opcode(&self) -> IrOpcode {
            self.node().opcode()
        }

        pub fn has_property(&self, property: Operator::Property) -> bool {
            self.op().has_property(property)
        }

        pub fn input_at(&self, index: usize) -> &Node {
            self.node().input_at(index)
        }

        pub fn equals(&self, node: *mut Node) -> bool {
            self.node_ == node
        }

        pub fn is_comparison(&self) -> bool {
            IrOpcode::is_comparison_opcode(self.opcode())
        }

        pub fn is_add(&self) -> bool {
            self.opcode() == IrOpcode::kAdd
        }

        pub fn is_allocate(&self) -> bool {
            self.opcode() == IrOpcode::kAllocate
        }

        pub fn is_arguments_elements(&self) -> bool {
            self.opcode() == IrOpcode::kArgumentsElements
        }

        pub fn is_arguments_length(&self) -> bool {
            self.opcode() == IrOpcode::kArgumentsLength
        }

        pub fn is_bitcast_tagged_to_word(&self) -> bool {
            self.opcode() == IrOpcode::kBitcastTaggedToWord
        }

        pub fn is_bitcast_word32_to_word64(&self) -> bool {
            self.opcode() == IrOpcode::kBitcastWord32ToWord64
        }

        pub fn is_bitcast_word64_to_word32(&self) -> bool {
            self.opcode() == IrOpcode::kBitcastWord64ToWord32
        }

        pub fn is_branch(&self) -> bool {
            self.opcode() == IrOpcode::kBranch
        }

        pub fn is_call(&self) -> bool {
            self.opcode() == IrOpcode::kCall
        }

        pub fn is_check_heap_flag(&self) -> bool {
            self.opcode() == IrOpcode::kCheckHeapFlag
        }

        pub fn is_check_if(&self) -> bool {
            self.opcode() == IrOpcode::kCheckIf
        }

        pub fn is_check_map(&self) -> bool {
            self.opcode() == IrOpcode::kCheckMap
        }

        pub fn is_check_number(&self) -> bool {
            self.opcode() == IrOpcode::kCheckNumber
        }

        pub fn is_check_receiver(&self) -> bool {
            self.opcode() == IrOpcode::kCheckReceiver
        }

        pub fn is_check_tagged_point(&self) -> bool {
            self.opcode() == IrOpcode::kCheckTaggedPoint
        }

        pub fn is_check_value(&self) -> bool {
            self.opcode() == IrOpcode::kCheckValue
        }

        pub fn is_class_fields_size(&self) -> bool {
            self.opcode() == IrOpcode::kClassFieldsSize
        }

        pub fn is_class_of(&self) -> bool {
            self.opcode() == IrOpcode::kClassOf
        }

        pub fn is_compressed_heap_constant(&self) -> bool {
            self.opcode() == IrOpcode::kCompressedHeapConstant
        }

        pub fn is_constant(&self) -> bool {
            self.opcode() == IrOpcode::kConstant
        }

        pub fn is_control(&self) -> bool {
            self.opcode() == IrOpcode::kControl
        }

        pub fn is_deoptimize(&self) -> bool {
            self.opcode() == IrOpcode::kDeoptimize
        }

        pub fn is_end(&self) -> bool {
            self.opcode() == IrOpcode::kEnd
        }

        pub fn is_external_constant(&self) -> bool {
            self.opcode() == IrOpcode::kExternalConstant
        }

        pub fn is_float32_abs(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Abs
        }

        pub fn is_float32_add(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Add
        }

        pub fn is_float32_ceil(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Ceil
        }

        pub fn is_float32_convert_i32(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32ConvertI32
        }

        pub fn is_float32_convert_u32(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32ConvertU32
        }

        pub fn is_float32_div(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Div
        }

        pub fn is_float32_equal(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Equal
        }

        pub fn is_float32_floor(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Floor
        }

        pub fn is_float32_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32LessThan
        }

        pub fn is_float32_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32LessThanOrEqual
        }

        pub fn is_float32_max(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Max
        }

        pub fn is_float32_min(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Min
        }

        pub fn is_float32_mul(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Mul
        }

        pub fn is_float32_neg(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Neg
        }

        pub fn is_float32_round_down(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32RoundDown
        }

        pub fn is_float32_round_truncate(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32RoundTruncate
        }

        pub fn is_float32_sqrt(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Sqrt
        }

        pub fn is_float32_sub(&self) -> bool {
            self.opcode() == IrOpcode::kFloat32Sub
        }

        pub fn is_float64_abs(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Abs
        }

        pub fn is_float64_acos(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Acos
        }

        pub fn is_float64_add(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Add
        }

        pub fn is_float64_asin(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Asin
        }

        pub fn is_float64_atan(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Atan
        }

        pub fn is_float64_atan2(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Atan2
        }

        pub fn is_float64_ceil(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Ceil
        }

        pub fn is_float64_combine(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Combine
        }

        pub fn is_float64_convert_i32(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ConvertI32
        }

        pub fn is_float64_convert_i64(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ConvertI64
        }

        pub fn is_float64_convert_u32(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ConvertU32
        }

        pub fn is_float64_convert_u64(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ConvertU64
        }

        pub fn is_float64_cos(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Cos
        }

        pub fn is_float64_div(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Div
        }

        pub fn is_float64_equal(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Equal
        }

        pub fn is_float64_exp(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Exp
        }

        pub fn is_float64_floor(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Floor
        }

        pub fn is_float64_fmod(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Fmod
        }

        pub fn is_float64_from_bits(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64FromBits
        }

        pub fn is_float64_log(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Log
        }

        pub fn is_float64_log10(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Log10
        }

        pub fn is_float64_log2(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Log2
        }

        pub fn is_float64_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64LessThan
        }

        pub fn is_float64_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64LessThanOrEqual
        }

        pub fn is_float64_max(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Max
        }

        pub fn is_float64_min(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Min
        }

        pub fn is_float64_mul(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Mul
        }

        pub fn is_float64_neg(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Neg
        }

        pub fn is_float64_pow(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Pow
        }

        pub fn is_float64_round_down(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64RoundDown
        }

        pub fn is_float64_round_truncate(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64RoundTruncate
        }

        pub fn is_float64_scalar_to_vector(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ScalarToVector
        }

        pub fn is_float64_sin(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Sin
        }

        pub fn is_float64_sqrt(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Sqrt
        }

        pub fn is_float64_sub(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Sub
        }

        pub fn is_float64_tan(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64Tan
        }

        pub fn is_float64_to_bits(&self) -> bool {
            self.opcode() == IrOpcode::kFloat64ToBits
        }

        pub fn is_frame_state(&self) -> bool {
            self.opcode() == IrOpcode::kFrameState
        }

        pub fn is_heap_constant(&self) -> bool {
            self.opcode() == IrOpcode::kHeapConstant
        }

        pub fn is_if_false(&self) -> bool {
            self.opcode() == IrOpcode::kIfFalse
        }

        pub fn is_if_true(&self) -> bool {
            self.opcode() == IrOpcode::kIfTrue
        }

        pub fn is_int32_add(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Add
        }

        pub fn is_int32_and(&self) -> bool {
            self.opcode() == IrOpcode::kInt32And
        }

        pub fn is_int32_arithmetic_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ArithmeticShiftRight
        }

        pub fn is_int32_bitfield_insert(&self) -> bool {
            self.opcode() == IrOpcode::kInt32BitfieldInsert
        }

        pub fn is_int32_bitfield_extract_signed(&self) -> bool {
            self.opcode() == IrOpcode::kInt32BitfieldExtractSigned
        }

        pub fn is_int32_bitfield_extract_unsigned(&self) -> bool {
            self.opcode() == IrOpcode::kInt32BitfieldExtractUnsigned
        }

        pub fn is_int32_clz(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Clz
        }

        pub fn is_int32_compare(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Compare
        }

        pub fn is_int32_constant(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Constant
        }

        pub fn is_int32_div(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Div
        }

        pub fn is_int32_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Equal
        }

        pub fn is_int32_extract_low_word32(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ExtractLowWord32
        }

        pub fn is_int32_insert_low_word32(&self) -> bool {
            self.opcode() == IrOpcode::kInt32InsertLowWord32
        }

        pub fn is_int32_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kInt32LessThan
        }

        pub fn is_int32_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt32LessThanOrEqual
        }

        pub fn is_int32_logical_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt32LogicalShiftRight
        }

        pub fn is_int32_mul(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Mul
        }

        pub fn is_int32_neg(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Neg
        }

        pub fn is_int32_or(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Or
        }

        pub fn is_int32_pair_add(&self) -> bool {
            self.opcode() == IrOpcode::kInt32PairAdd
        }

        pub fn is_int32_ror(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Ror
        }

        pub fn is_int32_sub(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Sub
        }

        pub fn is_int32_to_float32(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ToFloat32
        }

        pub fn is_int32_to_float64(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ToFloat64
        }

        pub fn is_int32_to_int52(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ToInt52
        }

        pub fn is_int32_to_tagged(&self) -> bool {
            self.opcode() == IrOpcode::kInt32ToTagged
        }

        pub fn is_int32_trunc_float32(&self) -> bool {
            self.opcode() == IrOpcode::kInt32TruncFloat32
        }

        pub fn is_int32_trunc_float64(&self) -> bool {
            self.opcode() == IrOpcode::kInt32TruncFloat64
        }

        pub fn is_int32_trunc_float64_to_tagged(&self) -> bool {
            self.opcode() == IrOpcode::kInt32TruncFloat64ToTagged
        }

        pub fn is_int32_unsigned_arithmetic_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt32UnsignedArithmeticShiftRight
        }

        pub fn is_int32_unsigned_div(&self) -> bool {
            self.opcode() == IrOpcode::kInt32UnsignedDiv
        }

        pub fn is_int32_unsigned_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kInt32UnsignedLessThan
        }

        pub fn is_int32_unsigned_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt32UnsignedLessThanOrEqual
        }

        pub fn is_int32_unsigned_mod(&self) -> bool {
            self.opcode() == IrOpcode::kInt32UnsignedMod
        }

        pub fn is_int32_xor(&self) -> bool {
            self.opcode() == IrOpcode::kInt32Xor
        }

        pub fn is_int64_add(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Add
        }

        pub fn is_int64_and(&self) -> bool {
            self.opcode() == IrOpcode::kInt64And
        }

        pub fn is_int64_arithmetic_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt64ArithmeticShiftRight
        }

        pub fn is_int64_compare(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Compare
        }

        pub fn is_int64_constant(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Constant
        }

        pub fn is_int64_div(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Div
        }

        pub fn is_int64_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Equal
        }

        pub fn is_int64_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kInt64LessThan
        }

        pub fn is_int64_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt64LessThanOrEqual
        }

        pub fn is_int64_logical_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt64LogicalShiftRight
        }

        pub fn is_int64_mod(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Mod
        }

        pub fn is_int64_mul(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Mul
        }

        pub fn is_int64_neg(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Neg
        }

        pub fn is_int64_or(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Or
        }

        pub fn is_int64_pair_add(&self) -> bool {
            self.opcode() == IrOpcode::kInt64PairAdd
        }

        pub fn is_int64_pair_mul(&self) -> bool {
            self.opcode() == IrOpcode::kInt64PairMul
        }

        pub fn is_int64_ror(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Ror
        }

        pub fn is_int64_sub(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Sub
        }

        pub fn is_int64_to_float32(&self) -> bool {
            self.opcode() == IrOpcode::kInt64ToFloat32
        }

        pub fn is_int64_to_float64(&self) -> bool {
            self.opcode() == IrOpcode::kInt64ToFloat64
        }

        pub fn is_int64_to_int32(&self) -> bool {
            self.opcode() == IrOpcode::kInt64ToInt32
        }

        pub fn is_int64_to_tagged(&self) -> bool {
            self.opcode() == IrOpcode::kInt64ToTagged
        }

        pub fn is_int64_trunc_float64(&self) -> bool {
            self.opcode() == IrOpcode::kInt64TruncFloat64
        }

        pub fn is_int64_unsigned_arithmetic_shift_right(&self) -> bool {
            self.opcode() == IrOpcode::kInt64UnsignedArithmeticShiftRight
        }

        pub fn is_int64_unsigned_div(&self) -> bool {
            self.opcode() == IrOpcode::kInt64UnsignedDiv
        }

        pub fn is_int64_unsigned_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kInt64UnsignedLessThan
        }

        pub fn is_int64_unsigned_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kInt64UnsignedLessThanOrEqual
        }

        pub fn is_int64_unsigned_mod(&self) -> bool {
            self.opcode() == IrOpcode::kInt64UnsignedMod
        }

        pub fn is_int64_xor(&self) -> bool {
            self.opcode() == IrOpcode::kInt64Xor
        }

        pub fn is_js_create(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreate
        }

        pub fn is_js_create_arguments(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateArguments
        }

        pub fn is_js_create_array(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateArray
        }

        pub fn is_js_create_array_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateArrayIterator
        }

        pub fn is_js_create_collection_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateCollectionIterator
        }

        pub fn is_js_create_date(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateDate
        }

        pub fn is_js_create_generator_object(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateGeneratorObject
        }

        pub fn is_js_create_iterable_result(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateIterableResult
        }

        pub fn is_js_create_literal_object(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateLiteralObject
        }

        pub fn is_js_create_map_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateMapIterator
        }

        pub fn is_js_create_object(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateObject
        }

        pub fn is_js_create_promise(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreatePromise
        }

        pub fn is_js_create_regexp(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateRegExp
        }

        pub fn is_js_create_set_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateSetIterator
        }

        pub fn is_js_create_string_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateStringIterator
        }

        pub fn is_js_create_typed_array(&self) -> bool {
            self.opcode() == IrOpcode::kJSCreateTypedArray
        }

        pub fn is_js_enumerable_own_names(&self) -> bool {
            self.opcode() == IrOpcode::kJSEnumerableOwnNames
        }

        pub fn is_js_get_iterator(&self) -> bool {
            self.opcode() == IrOpcode::kJSGetIterator
        }

        pub fn is_js_has_property(&self) -> bool {
            self.opcode() == IrOpcode::kJSHasProperty
        }

        pub fn is_js_load_context(&self) -> bool {
            self.opcode() == IrOpcode::kJSLoadContext
        }

        pub fn is_js_store_context(&self) -> bool {
            self.opcode() == IrOpcode::kJSStoreContext
        }

        pub fn is_load_element(&self) -> bool {
            self.opcode() == IrOpcode::kLoadElement
        }

        pub fn is_load_field(&self) -> bool {
            self.opcode() == IrOpcode::kLoadField
        }

        pub fn is_load_parent_field(&self) -> bool {
            self.opcode() == IrOpcode::kLoadParentField
        }

        pub fn is_load_typed_element(&self) -> bool {
            self.opcode() == IrOpcode::kLoadTypedElement
        }

        pub fn is_load(&self) -> bool {
            self.opcode() == IrOpcode::kLoad
        }

        pub fn is_machine_type(&self) -> bool {
            self.opcode() == IrOpcode::kMachineType
        }

        pub fn is_map_value(&self) -> bool {
            self.opcode() == IrOpcode::kMapValue
        }

        pub fn is_merge(&self) -> bool {
            self.opcode() == IrOpcode::kMerge
        }

        pub fn is_number_add(&self) -> bool {
            self.opcode() == IrOpcode::kNumberAdd
        }

        pub fn is_number_bitwise_and(&self) -> bool {
            self.opcode() == IrOpcode::kNumberBitwiseAnd
        }

        pub fn is_number_bitwise_not(&self) -> bool {
            self.opcode() == IrOpcode::kNumberBitwiseNot
        }

        pub fn is_number_bitwise_or(&self) -> bool {
            self.opcode() == IrOpcode::kNumberBitwiseOr
        }

        pub fn is_number_bitwise_xor(&self) -> bool {
            self.opcode() == IrOpcode::kNumberBitwiseXor
        }

        pub fn is_number_ceil(&self) -> bool {
            self.opcode() == IrOpcode::kNumberCeil
        }

        pub fn is_number_constant(&self) -> bool {
            self.opcode() == IrOpcode::kNumberConstant
        }

        pub fn is_number_divide(&self) -> bool {
            self.opcode() == IrOpcode::kNumberDivide
        }

        pub fn is_number_equal(&self) -> bool {
            self.opcode() == IrOpcode::kNumberEqual
        }

        pub fn is_number_expm1(&self) -> bool {
            self.opcode() == IrOpcode::kNumberExpm1
        }

        pub fn is_number_floor(&self) -> bool {
            self.opcode() == IrOpcode::kNumberFloor
        }

        pub fn is_number_fround(&self) -> bool {
            self.opcode() == IrOpcode::kNumberFround
        }

        pub fn is_number_imul(&self) -> bool {
            self.opcode() == IrOpcode::kNumberImul
        }

        pub fn is_number_less_than(&self) -> bool {
            self.opcode() == IrOpcode::kNumberLessThan
        }

        pub fn is_number_less_than_or_equal(&self) -> bool {
            self.opcode() == IrOpcode::kNumberLessThanOrEqual
        }

        pub fn is_number_log(&self) -> bool {
            self.opcode() == IrOpcode::kNumberLog
        }

        pub fn is_number_log1p(&self) -> bool {
            self.opcode() == IrOpcode::kNumberLog1p
        }

        pub fn is_number_max(&self) -> bool {
            self.opcode() == IrOpcode::kNumberMax
        }

        pub fn is_number_min(&self) -> bool {
            self.opcode() == IrOpcode::kNumberMin
        }

        pub fn is_number_multiply(&self) -> bool {
            self.opcode() == IrOpcode::kNumberMultiply
        }

        pub fn is_number_negate(&self) -> bool {
            self.opcode() == IrOpcode::kNumberNegate
        }

        pub fn is_number_pow(&self) -> bool {
            self.opcode() == IrOpcode::kNumberPow
        }

        pub fn is_number_round(&self) -> bool {
            self.opcode() == IrOpcode::kNumberRound
        }

        pub fn is_number_sign(&self) -> bool {
            self.opcode() == IrOpcode::kNumberSign
        }

        pub fn is_number_sin(&self) -> bool {
            self.opcode() == IrOpcode::kNumberSin
        }

        pub fn is_number_sqrt(&self) -> bool {
            self.opcode() == IrOpcode::kNumberSqrt
        }

        pub fn is_number_subtract(&self) -> bool {
            self.opcode() == IrOpcode::kNumberSubtract
        }

        pub fn is_number_trunc(&self) -> bool {
            self.opcode() == IrOpcode::kNumberTrunc
        }

        pub fn is_object_is_smi(&self) -> bool {
            self.opcode() == IrOpcode::kObjectIsSmi
        }

        pub fn is_object_to_boolean(&self) -> bool {
            self.opcode() == IrOpcode::kObjectToBoolean
        }

        pub fn is_parameter(&self) -> bool {
            self.opcode() == IrOpcode::kParameter
        }

        pub fn is_phi(&self) -> bool {
            self.opcode() == IrOpcode::kPhi
        }

        pub fn is_pointer_constant(&self) -> bool {
            self.opcode() == IrOpcode::kPointerConstant
        }

        pub fn is_projection(&self) -> bool {
            self.opcode() == IrOp
