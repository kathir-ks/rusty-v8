// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod opmasks {
    use crate::compiler::turboshaft::operations::*;
    use crate::compiler::turboshaft::representations::*;
    use std::marker::PhantomData;

    macro_rules! offset_of {
        ($struct:path, $field:ident) => {{
            let dummy = core::mem::MaybeUninit::<$struct>::uninit();
            let dummy_ptr = dummy.as_ptr();
            let field_ptr = unsafe { &(*dummy_ptr).$field as *const _ };

            (field_ptr as usize) - (dummy_ptr as usize)
        }};
    }

    macro_rules! field {
        ($struct:path, $field:ident) => {
            FieldData::<$struct, _> {
                offset: offset_of!($struct, $field),
                phantom: PhantomData,
            }
        };
    }

    const K_BITS_PER_BYTE: usize = 8;

    pub struct FieldData<S, T> {
        pub offset: usize,
        pub phantom: PhantomData<(S, T)>,
    }

    pub struct OpMaskField<T, const OFFSET: usize, const SIZE: usize> {
        phantom: PhantomData<T>,
    }

    impl<T, const OFFSET: usize, const SIZE: usize> OpMaskField<T, OFFSET, SIZE> {
        pub const OFFSET: usize = OFFSET;
        pub const SIZE: usize = SIZE;

        const _ASSERT: () = assert!(OFFSET + SIZE <= core::mem::size_of::<u64>());
    }

    pub fn encode_for_mask<T: Into<u64>>(value: T) -> u64 {
        value.into()
    }

    pub trait RepresentationWrapper {
        type Enum;
    }

    impl RepresentationWrapper for WordRepresentation {
        type Enum = WordRepresentationEnum;
    }

    impl RepresentationWrapper for FloatRepresentation {
        type Enum = FloatRepresentationEnum;
    }

    impl RepresentationWrapper for RegisterRepresentation {
        type Enum = RegisterRepresentationEnum;
    }

    pub struct MaskBuilder<Op, Fields> {
        phantom: PhantomData<(Op, Fields)>,
    }

    impl<Op, Fields> MaskBuilder<Op, Fields>
    where
        Op: OperationTrait,
    {
        const OFFSET_OPCODE: usize = offset_of!(Operation, opcode);
        const SIZE_OPCODE: usize = core::mem::size_of::<Opcode>();

        pub const _ASSERT_OPCODE_OFFSET: () = assert!(Self::OFFSET_OPCODE == 0);
        pub const _ASSERT_OPCODE_SIZE: () =
            assert!(Self::SIZE_OPCODE == core::mem::size_of::<u8>());
        pub const _ASSERT_OPERATION_SIZE: () =
            assert!(core::mem::size_of::<Operation>() == 4);

        pub const fn build_base_mask() -> u64 {
            #[cfg(target_endian = "big")]
            {
                (0xFFu64) << ((core::mem::size_of::<u64>() - core::mem::size_of::<u8>()) * K_BITS_PER_BYTE)
            }
            #[cfg(target_endian = "little")]
            {
                0xFFu64
            }
        }

        pub const fn encode_base_value(opcode: Opcode) -> u64 {
            #[cfg(target_endian = "big")]
            {
                (opcode as u64)
                    << ((core::mem::size_of::<u64>() - core::mem::size_of::<Opcode>()) * K_BITS_PER_BYTE)
            }
            #[cfg(target_endian = "little")]
            {
                opcode as u64
            }
        }

        pub const fn build_mask<F>() -> u64
        where
            F: FieldProvider,
        {
            let base_mask = Self::build_base_mask();
            base_mask | Self::build_field_mask::<F>()
        }

        pub const fn encode_value<F>(args: F::Value) -> u64
        where
            F: FieldProvider,
            F::Value: Into<u64>,
        {
            let base_value = Self::encode_base_value(Op::OPCODE);
            base_value | Self::encode_field_value::<F>(args)
        }

        pub const fn build_field_mask<F>() -> u64
        where
            F: FieldProvider,
        {
            let size = F::SIZE;
            let offset = F::OFFSET;

            assert!(size < core::mem::size_of::<u64>());
            assert!(offset + size <= core::mem::size_of::<u64>());

            let ones = u64::MAX >> ((core::mem::size_of::<u64>() - size) * K_BITS_PER_BYTE);

            #[cfg(target_endian = "big")]
            {
                ones << ((core::mem::size_of::<u64>() - size - offset) * K_BITS_PER_BYTE)
            }
            #[cfg(target_endian = "little")]
            {
                ones << (offset * K_BITS_PER_BYTE)
            }
        }

        pub const fn encode_field_value<F>(value: F::Value) -> u64
        where
            F: FieldProvider,
            F::Value: Into<u64>,
        {
            let size = F::SIZE;
            let offset = F::OFFSET;

            #[cfg(target_endian = "big")]
            {
                encode_for_mask(value)
                    << ((core::mem::size_of::<u64>() - size - offset) * K_BITS_PER_BYTE)
            }
            #[cfg(target_endian = "little")]
            {
                encode_for_mask(value) << (offset * K_BITS_PER_BYTE)
            }
        }
    }

    pub trait FieldProvider {
        const OFFSET: usize;
        const SIZE: usize;
        type Value;
    }

    impl<S, T, const OFFSET: usize> FieldProvider for FieldData<S, T>
    where
        T: Copy,
    {
        const OFFSET: usize = OFFSET;
        const SIZE: usize = core::mem::size_of::<T>();
        type Value = T;
    }

    pub struct OpMaskT<Op, const MASK: u64, const VALUE: u64> {
        phantom: PhantomData<Op>,
    }

    impl<Op, const MASK: u64, const VALUE: u64> OpMaskT<Op, const MASK, const VALUE> {
        pub const MASK: u64 = MASK;
        pub const VALUE: u64 = VALUE;
    }

    pub trait OperationTrait {
        const OPCODE: Opcode;
    }

    impl OperationTrait for WordBinopOp {
        const OPCODE: Opcode = Opcode::WordBinop;
    }

    impl OperationTrait for WordUnaryOp {
        const OPCODE: Opcode = Opcode::WordUnary;
    }

    impl OperationTrait for FloatUnaryOp {
        const OPCODE: Opcode = Opcode::FloatUnary;
    }

    impl OperationTrait for FloatBinopOp {
        const OPCODE: Opcode = Opcode::FloatBinop;
    }

    impl OperationTrait for ShiftOp {
        const OPCODE: Opcode = Opcode::Shift;
    }

    impl OperationTrait for PhiOp {
        const OPCODE: Opcode = Opcode::Phi;
    }

    impl OperationTrait for ConstantOp {
        const OPCODE: Opcode = Opcode::Constant;
    }

    impl OperationTrait for ProjectionOp {
        const OPCODE: Opcode = Opcode::Projection;
    }

    impl OperationTrait for ComparisonOp {
        const OPCODE: Opcode = Opcode::Comparison;
    }

    impl OperationTrait for ChangeOp {
        const OPCODE: Opcode = Opcode::Change;
    }

    impl OperationTrait for OverflowCheckedBinopOp {
        const OPCODE: Opcode = Opcode::OverflowCheckedBinop;
    }

    impl OperationTrait for TaggedBitcastOp {
        const OPCODE: Opcode = Opcode::TaggedBitcast;
    }

    #[cfg(v8_enable_webassembly)]
    impl OperationTrait for Simd128BinopOp {
        const OPCODE: Opcode = Opcode::Simd128Binop;
    }

    #[cfg(v8_enable_webassembly)]
    impl OperationTrait for Simd128UnaryOp {
        const OPCODE: Opcode = Opcode::Simd128Unary;
    }

    #[cfg(v8_enable_webassembly)]
    impl OperationTrait for Simd128ShiftOp {
        const OPCODE: Opcode = Opcode::Simd128Shift;
    }

    #[cfg(v8_enable_webassembly)]
    impl OperationTrait for Simd128LoadTransformOp {
        const OPCODE: Opcode = Opcode::Simd128LoadTransform;
    }

    #[cfg(v8_enable_webassembly)]
    impl OperationTrait for Simd128ReplaceLaneOp {
        const OPCODE: Opcode = Opcode::Simd128ReplaceLane;
    }

    #[cfg(all(v8_enable_webassembly, v8_enable_wasm_simd256_revec))]
    impl OperationTrait for Simd256UnaryOp {
        const OPCODE: Opcode = Opcode::Simd256Unary;
    }

    type WordBinopMask =
        MaskBuilder<WordBinopOp, (field!(WordBinopOp, kind), field!(WordBinopOp, rep))>;

    type WordBinopKindMask = MaskBuilder<WordBinopOp, field!(WordBinopOp, kind)>;

    pub type KWord32Add =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kAdd, WordRepresentation::Word32())) }>;
    pub type KWord32Sub =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kSub, WordRepresentation::Word32())) }>;
    pub type KWord32Mul =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kMul, WordRepresentation::Word32())) }>;
    pub type KWord32SignedMulOverflownBits = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kSignedMulOverflownBits, WordRepresentation::Word32())) }>;
    pub type KWord32UnsignedMulOverflownBits = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kUnsignedMulOverflownBits, WordRepresentation::Word32())) }>;

    pub type KWord32BitwiseAnd = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseAnd, WordRepresentation::Word32())) }>;
    pub type KWord32BitwiseOr = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseOr, WordRepresentation::Word32())) }>;
    pub type KWord32BitwiseXor = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseXor, WordRepresentation::Word32())) }>;
    pub type KWord64Add =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kAdd, WordRepresentation::Word64())) }>;
    pub type KWord64Sub =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kSub, WordRepresentation::Word64())) }>;
    pub type KWord64Mul =
        OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kMul, WordRepresentation::Word64())) }>;
    pub type KWord64BitwiseAnd = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseAnd, WordRepresentation::Word64())) }>;
    pub type KWord64BitwiseOr = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseOr, WordRepresentation::Word64())) }>;
    pub type KWord64BitwiseXor = OpMaskT<WordBinopOp, { WordBinopMask::build_mask::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>() }, { WordBinopMask::encode_value::<(field!(WordBinopOp, kind), field!(WordBinopOp, rep))>((WordBinopOp::Kind::kBitwiseXor, WordRepresentation::Word64())) }>;

    pub type KBitwiseAnd = OpMaskT<WordBinopOp, { WordBinopKindMask::build_mask::<field!(WordBinopOp, kind)>() }, { WordBinopKindMask::encode_value::<field!(WordBinopOp, kind)>(WordBinopOp::Kind::kBitwiseAnd) }>;
    pub type KBitwiseXor = OpMaskT<WordBinopOp, { WordBinopKindMask::build_mask::<field!(WordBinopOp, kind)>() }, { WordBinopKindMask::encode_value::<field!(WordBinopOp, kind)>(WordBinopOp::Kind::kBitwiseXor) }>;

    type WordUnaryMask =
        MaskBuilder<WordUnaryOp, (field!(WordUnaryOp, kind), field!(WordUnaryOp, rep))>;
    pub type KWord32ReverseBytes = OpMaskT<WordUnaryOp, { WordUnaryMask::build_mask::<(field!(WordUnaryOp, kind), field!(WordUnaryOp, rep))>() }, { WordUnaryMask::encode_value::<(field!(WordUnaryOp, kind), field!(WordUnaryOp, rep))>((WordUnaryOp::Kind::kReverseBytes, WordRepresentation::Word32())) }>;
    pub type KWord64ReverseBytes = OpMaskT<WordUnaryOp, { WordUnaryMask::build_mask::<(field!(WordUnaryOp, kind), field!(WordUnaryOp, rep))>() }, { WordUnaryMask::encode_value::<(field!(WordUnaryOp, kind), field!(WordUnaryOp, rep))>((WordUnaryOp::Kind::kReverseBytes, WordRepresentation::Word64())) }>;

    type FloatUnaryMask =
        MaskBuilder<FloatUnaryOp, (field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>;

    pub type KFloat32Negate = OpMaskT<FloatUnaryOp, { FloatUnaryMask::build_mask::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>() }, { FloatUnaryMask::encode_value::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>((FloatUnaryOp::Kind::kNegate, FloatRepresentation::Float32())) }>;
    pub type KFloat64Abs = OpMaskT<FloatUnaryOp, { FloatUnaryMask::build_mask::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>() }, { FloatUnaryMask::encode_value::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>((FloatUnaryOp::Kind::kAbs, FloatRepresentation::Float64())) }>;
    pub type KFloat64Negate = OpMaskT<FloatUnaryOp, { FloatUnaryMask::build_mask::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>() }, { FloatUnaryMask::encode_value::<(field!(FloatUnaryOp, kind), field!(FloatUnaryOp, rep))>((FloatUnaryOp::Kind::kNegate, FloatRepresentation::Float64())) }>;

    type FloatBinopMask =
        MaskBuilder<FloatBinopOp, (field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>;

    pub type KFloat32Sub = OpMaskT<FloatBinopOp, { FloatBinopMask::build_mask::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>() }, { FloatBinopMask::encode_value::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>((FloatBinopOp::Kind::kSub, FloatRepresentation::Float32())) }>;
    pub type KFloat32Mul = OpMaskT<FloatBinopOp, { FloatBinopMask::build_mask::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>() }, { FloatBinopMask::encode_value::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>((FloatBinopOp::Kind::kMul, FloatRepresentation::Float32())) }>;
    pub type KFloat64Sub = OpMaskT<FloatBinopOp, { FloatBinopMask::build_mask::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>() }, { FloatBinopMask::encode_value::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>((FloatBinopOp::Kind::kSub, FloatRepresentation::Float64())) }>;
    pub type KFloat64Mul = OpMaskT<FloatBinopOp, { FloatBinopMask::build_mask::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>() }, { FloatBinopMask::encode_value::<(field!(FloatBinopOp, kind), field!(FloatBinopOp, rep))>((FloatBinopOp::Kind::kMul, FloatRepresentation::Float64())) }>;

    type ShiftMask = MaskBuilder<ShiftOp, (field!(ShiftOp, kind), field!(ShiftOp, rep))>;
    type ShiftKindMask = MaskBuilder<ShiftOp, field!(ShiftOp, kind)>;

    pub type KWord32ShiftLeft = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftLeft, WordRepresentation::Word32())) }>;
    pub type KWord32ShiftRightArithmetic = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftRightArithmetic, WordRepresentation::Word32())) }>;
    pub type KWord32ShiftRightArithmeticShiftOutZeros = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftRightArithmeticShiftOutZeros, WordRepresentation::Word32())) }>;
    pub type KWord32ShiftRightLogical = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftRightLogical, WordRepresentation::Word32())) }>;
    pub type KWord32RotateRight = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kRotateRight, WordRepresentation::Word32())) }>;
    pub type KWord64ShiftLeft = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftLeft, WordRepresentation::Word64())) }>;
    pub type KWord64ShiftRightArithmetic = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftRightArithmetic, WordRepresentation::Word64())) }>;
    pub type KWord64ShiftRightLogical = OpMaskT<ShiftOp, { ShiftMask::build_mask::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>() }, { ShiftMask::encode_value::<(field!(ShiftOp, kind), field!(ShiftOp, rep))>((ShiftOp::Kind::kShiftRightLogical, WordRepresentation::Word64())) }>;
    pub type KShiftLeft = OpMaskT<ShiftOp, { ShiftKindMask::build_mask::<field!(ShiftOp, kind)>() }, { ShiftKindMask::encode_value::<field!(ShiftOp, kind)>(ShiftOp::Kind::kShiftLeft) }>;

    type PhiMask = MaskBuilder<PhiOp, field!(PhiOp, rep)>;
    pub type KTaggedPhi =
        OpMaskT<PhiOp, { PhiMask::build_mask::<field!(PhiOp, rep)>() }, { PhiMask::encode_value::<field!(PhiOp, rep)>(RegisterRepresentation::Tagged()) }>;

    type ConstantMask = MaskBuilder<ConstantOp, field!(ConstantOp, kind)>;

    pub type KWord32Constant = OpMaskT<ConstantOp, { ConstantMask::build_mask::<field!(ConstantOp, kind)>() }, { ConstantMask::encode_value::<field!(ConstantOp, kind)>(ConstantOp::Kind::kWord32) }>;
    pub type KWord64Constant = OpMaskT<ConstantOp, { ConstantMask::build_mask::<field!(ConstantOp, kind)>() }, { ConstantMask::encode_value::<field!(ConstantOp, kind)>(ConstantOp::Kind::kWord64) }>;
    pub type KExternalConstant = OpMaskT<ConstantOp, { ConstantMask::build_mask::<field!(ConstantOp, kind)>() }, { ConstantMask::encode_value::<field!(ConstantOp, kind)>(ConstantOp::Kind::kExternal) }>;
    pub type KHeapConstant = OpMaskT<ConstantOp, { ConstantMask::build_mask::<field!(ConstantOp, kind)>() }, { ConstantMask::encode_value::<field!(ConstantOp, kind)>(ConstantOp::Kind::kHeapObject) }>;
    pub type KSmiConstant = OpMaskT<ConstantOp, { ConstantMask::build_mask::<field!(ConstantOp, kind)>() }, { ConstantMask::encode_value::<field!(ConstantOp, kind)>(ConstantOp::Kind::kSmi) }>;

    type ProjectionMask = MaskBuilder<ProjectionOp, field!(ProjectionOp, index)>;

    pub type KProjection0 =
        OpMaskT<ProjectionOp, { ProjectionMask::build_mask::<field!(ProjectionOp, index)>() }, { ProjectionMask::encode_value::<field!(ProjectionOp, index)>(0) }>;
    pub type KProjection1 =
        OpMaskT<ProjectionOp, { ProjectionMask::build_mask::<field!(ProjectionOp, index)>() }, { ProjectionMask::encode_value::<field!(ProjectionOp, index)>(1) }>;

    type ComparisonMask =
        MaskBuilder<ComparisonOp, (field!(ComparisonOp, kind), field!(ComparisonOp, rep))>;

    pub type KWord32Equal = OpMaskT<ComparisonOp, { ComparisonMask::build_mask::<(field!(ComparisonOp, kind), field!(ComparisonOp, rep))>() }, { ComparisonMask::encode_value::<(field!(ComparisonOp, kind), field!(ComparisonOp, rep))>((ComparisonOp::Kind::kEqual, WordRepresentation::Word32())) }>;
    pub type KWord64Equal = OpMaskT<ComparisonOp, { ComparisonMask::build_mask::<(field!(ComparisonOp, kind), field!(ComparisonOp, rep))>() }, { ComparisonMask::encode_value::<(field!(ComparisonOp, kind), field!(ComparisonOp, rep))>((ComparisonOp::Kind::kEqual, WordRepresentation::Word64())) }>;
    type ComparisonKindMask = MaskBuilder<ComparisonOp, field!(ComparisonOp, kind)>;
    pub type KComparisonEqual = OpMaskT<ComparisonOp, { ComparisonKindMask::build_mask::<field!(ComparisonOp, kind)>() }, { ComparisonKindMask::encode_value::<field!(ComparisonOp, kind)>(ComparisonOp::Kind::kEqual) }>;

    type ChangeOpMask = MaskBuilder<
        ChangeOp,
        (
            field!(ChangeOp, kind),
            field!(ChangeOp, assumption),
            field!(ChangeOp, from),
            field!(ChangeOp, to),
        ),
    >;

    pub type KChangeInt32ToInt64 = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kSignExtend, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Word32(), RegisterRepresentation::Word64())) }>;
    pub type KChangeUint32ToUint64 = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kZeroExtend, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Word32(), RegisterRepresentation::Word64())) }>;
    pub type KFloat64ExtractHighWord32 = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kExtractHighHalf, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Float64(), RegisterRepresentation::Word32())) }>;
    pub type KTruncateFloat64ToInt64OverflowToMin = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kSignedFloatTruncateOverflowToMin, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Float64(), RegisterRepresentation::Word64())) }>;
    pub type KTruncateFloat32ToInt32OverflowToMin = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kSignedFloatTruncateOverflowToMin, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Float32(), RegisterRepresentation::Word32())) }>;
    pub type KTruncateFloat32ToUint32OverflowToMin = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kUnsignedFloatTruncateOverflowToMin, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Float32(), RegisterRepresentation::Word32())) }>;

    pub type KTruncateWord64ToWord32 = OpMaskT<ChangeOp, { ChangeOpMask::build_mask::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>() }, { ChangeOpMask::encode_value::<(field!(ChangeOp, kind), field!(ChangeOp, assumption), field!(ChangeOp, from), field!(ChangeOp, to))>((ChangeOp::Kind::kTruncate, ChangeOp::Assumption::kNoAssumption, RegisterRepresentation::Word64(), RegisterRepresentation::Word32())) }>;

    type OverflowCheckedBinopMask = MaskBuilder<
        OverflowCheckedBinopOp,
        (field!(OverflowCheckedBinopOp, kind), field!(OverflowCheckedBinopOp, rep)),
    >;
    pub type KOverflowCheckedWord32Add = OpMaskT<OverflowCheckedBinopOp, { OverflowCheckedBinopMask::build_mask::<(field!(OverflowCheckedBinopOp, kind), field!(OverflowCheckedBinopOp, rep))>() }, { OverflowCheckedBinopMask::encode_value::<(field!(OverflowCheckedBinopOp, kind), field!(OverflowCheckedBinopOp, rep))>((OverflowCheckedBinopOp::Kind::kSignedAdd, WordRepresentation::Word32())) }>;

    type TaggedBitcastMask = MaskBuilder<
        TaggedBitcastOp,
        (
            field!(TaggedBitcastOp, from),
            field!(TaggedBitcastOp, to),
            field!(TaggedBitcastOp, kind),
        ),
    >;
    pub type KBitcastTaggedToWordPtrForTagAndSmiBits = OpMaskT<TaggedBitcastOp, { TaggedBitcastMask::build_mask::<(field!(TaggedBitcastOp, from), field!(TaggedBitcastOp, to), field!(TaggedBitcastOp, kind))>() }, { TaggedBitcastMask::encode_value::<(field!(TaggedBitcastOp, from), field!(TaggedBitcastOp, to), field!(TaggedBitcastOp, kind))>((RegisterRepresentation::Tagged(), RegisterRepresentation::WordPtr(), TaggedBitcastOp::Kind::kTagAndSmiBits)) }>;
    pub type KBitcastWordPtrToSmi = OpMaskT<TaggedBitcastOp, { TaggedBitcastMask::build_mask::<(field!(TaggedBitcastOp, from), field!(TaggedBitcastOp, to), field!(TaggedBitcastOp, kind))>() }, { TaggedBitcastMask::encode_value::<(field!(TaggedBitcastOp, from), field!(TaggedBitcastOp, to), field!(TaggedBitcastOp, kind))>((RegisterRepresentation::WordPtr(), RegisterRepresentation::Tagged(), TaggedBitcastOp::Kind::kSmi)) }>;

    type TaggedBitcastKindMask = MaskBuilder<TaggedBitcastOp, field!(TaggedBitcastOp, kind)>;
    pub type KTaggedBitcastSmi = OpMaskT<TaggedBitcastOp, { TaggedBitcastKindMask::build_mask::<field!(TaggedBitcastOp, kind)>() }, { TaggedBitcastKindMask::encode_value::<field!(TaggedBitcastOp, kind)>(TaggedBitcastOp::Kind::kSmi) }>;
    pub type KTaggedBitcastHeapObject = OpMaskT<TaggedBitcastOp, { TaggedBitcastKindMask::build_mask::<field!(TaggedBitcastOp, kind)>() }, { TaggedBitcastKindMask::encode_value::<field!(TaggedBitcastOp, kind)>(TaggedBitcastOp::Kind::kHeapObject) }>;

    #[cfg(v