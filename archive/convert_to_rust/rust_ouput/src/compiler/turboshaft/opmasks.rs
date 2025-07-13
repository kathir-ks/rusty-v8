// Converted from V8 C++ source files:
// Header: opmasks.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod opmasks {
    use crate::compiler::turboshaft::operations::*;
    use crate::compiler::turboshaft::representations::*;
    use std::marker::PhantomData;

    pub struct OpMaskField<T, const OFFSET: usize> {
        _phantom: PhantomData<T>,
    }

    impl<T, const OFFSET: usize> OpMaskField<T, OFFSET> {
        pub const OFFSET: usize = OFFSET;
        pub const SIZE: usize = std::mem::size_of::<T>();

        pub fn new() -> Self {
            assert!(OFFSET + Self::SIZE <= std::mem::size_of::<u64>());
            Self {
                _phantom: PhantomData,
            }
        }
    }

    pub fn encode_for_mask<T>(value: T) -> u64
    where
        T: Into<u64>,
    {
        value.into()
    }

    pub struct UnwrapRepresentation<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> UnwrapRepresentation<T> {
        pub type Type = T;
    }

    impl UnwrapRepresentation<WordRepresentation> {
        pub type Type = WordRepresentationEnum;
    }

    impl UnwrapRepresentation<FloatRepresentation> {
        pub type Type = FloatRepresentationEnum;
    }

    impl UnwrapRepresentation<RegisterRepresentation> {
        pub type Type = RegisterRepresentationEnum;
    }

    pub struct MaskBuilder<Op, Fields> {
        _phantom_op: PhantomData<Op>,
        _phantom_fields: PhantomData<Fields>,
    }

    impl<Op, Fields> MaskBuilder<Op, Fields> {
        const OPERATION_OPCODE_OFFSET: usize = 0;
        const OPERATION_OPCODE_SIZE: usize = std::mem::size_of::<u8>();
        const OPERATION_SIZE: usize = 4;

        pub fn build_base_mask() -> u64 {
            assert_eq!(Self::OPERATION_OPCODE_OFFSET, 0);
            assert_eq!(Self::OPERATION_OPCODE_SIZE, std::mem::size_of::<u8>());
            assert_eq!(Self::OPERATION_SIZE, 4);

            #[cfg(target_endian = "big")]
            {
                (0xFFu64) << ((std::mem::size_of::<u64>() - std::mem::size_of::<u8>()) * 8)
            }
            #[cfg(target_endian = "little")]
            {
                0xFFu64
            }
        }

        pub fn encode_base_value(opcode: Opcode) -> u64 {
            assert_eq!(Self::OPERATION_OPCODE_OFFSET, 0);

            #[cfg(target_endian = "big")]
            {
                (opcode as u64) << ((std::mem::size_of::<u64>() - Self::OPERATION_OPCODE_SIZE) * 8)
            }
            #[cfg(target_endian = "little")]
            {
                opcode as u64
            }
        }
    }

    impl<Op, Fields: FieldList> MaskBuilder<Op, Fields> {
        pub fn build_mask() -> u64 {
            let base_mask = Self::build_base_mask();
            Fields::build_field_mask(base_mask)
        }

        pub fn encode_value(args: Fields::Args) -> u64 {
            let base_value = Self::encode_base_value(operation_to_opcode::<Op>());
            Fields::encode_field_value(base_value, args)
        }

        pub type For<Args> = OpMaskT<Op, { Self::build_mask() }, { Self::encode_value(Args::new()) }>;
    }

    pub trait Field {
        type Type;
        const SIZE: usize;
        const OFFSET: usize;
        fn build_field_mask(base_mask: u64) -> u64;
        fn encode_field_value(base_value: u64, value: Self::Type) -> u64;
    }

    pub trait FieldList {
        type Args: ArgsList;
        fn build_field_mask(base_mask: u64) -> u64;
        fn encode_field_value(base_value: u64, args: Self::Args) -> u64;
    }

    impl<F: Field> FieldList for F {
        type Args = SingleArg<F::Type>;
        fn build_field_mask(base_mask: u64) -> u64 {
            F::build_field_mask(base_mask)
        }
        fn encode_field_value(base_value: u64, args: Self::Args) -> u64 {
            F::encode_field_value(base_value, args.0)
        }
    }

    impl<F1: Field, F2: Field> FieldList for (F1, F2) {
        type Args = DoubleArg<F1::Type, F2::Type>;
        fn build_field_mask(base_mask: u64) -> u64 {
            F1::build_field_mask(base_mask) | F2::build_field_mask(base_mask)
        }
        fn encode_field_value(base_value: u64, args: Self::Args) -> u64 {
            F1::encode_field_value(base_value, args.0) | F2::encode_field_value(base_value, args.1)
        }
    }

    // Add more impls for tuples of different sizes if needed

    pub trait ArgsList {
        fn new() -> Self;
    }

    pub struct SingleArg<T>(pub T);
    impl<T> ArgsList for SingleArg<T> {
        fn new() -> Self {
            Self(unsafe { std::mem::zeroed() }) // Initialize with a default value
        }
    }

    pub struct DoubleArg<T1, T2>(pub T1, pub T2);
    impl<T1, T2> ArgsList for DoubleArg<T1, T2> {
        fn new() -> Self {
            Self(unsafe { std::mem::zeroed() }, unsafe { std::mem::zeroed() })
        }
    }
    impl<T, const OFFSET: usize> Field for OpMaskField<T, OFFSET>
    where
        T: Into<u64> + Copy,
    {
        type Type = T;
        const SIZE: usize = std::mem::size_of::<T>();
        const OFFSET: usize = OFFSET;

        fn build_field_mask(base_mask: u64) -> u64 {
            assert!(Self::SIZE < std::mem::size_of::<u64>());
            assert!(Self::OFFSET + Self::SIZE <= std::mem::size_of::<u64>());

            let ones: u64 = ((-1i64 as u64) >> ((std::mem::size_of::<u64>() - Self::SIZE) * 8));

            #[cfg(target_endian = "big")]
            {
                ones << ((std::mem::size_of::<u64>() - Self::SIZE - Self::OFFSET) * 8)
            }
            #[cfg(target_endian = "little")]
            {
                ones << (Self::OFFSET * 8)
            }
        }

        fn encode_field_value(base_value: u64, value: Self::Type) -> u64 {
            #[cfg(target_endian = "big")]
            {
                encode_for_mask(value) << ((std::mem::size_of::<u64>() - Self::SIZE - Self::OFFSET) * 8)
            }
            #[cfg(target_endian = "little")]
            {
                encode_for_mask(value) << (Self::OFFSET * 8)
            }
        }
    }

    // === Definitions of masks for Turboshaft operations === //

    pub type WordBinopMask =
        MaskBuilder<WordBinopOp, (OpMaskField<WordBinopOpKind, { offset_of!(WordBinopOp, kind) }>, OpMaskField<WordRepresentationEnum, { offset_of!(WordBinopOp, rep) }>)>;
    pub type WordBinopKindMask = MaskBuilder<WordBinopOp, OpMaskField<WordBinopOpKind, { offset_of!(WordBinopOp, kind) }>>;

    pub type KWord32Add =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32Sub =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32Mul =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32SignedMulOverflownBits =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32UnsignedMulOverflownBits =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments

    pub type KWord32BitwiseAnd = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;
    pub type KWord32BitwiseOr = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;
    pub type KWord32BitwiseXor = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;
    pub type KWord64Add =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64Sub =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64Mul =
        WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64BitwiseAnd = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;
    pub type KWord64BitwiseOr = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;
    pub type KWord64BitwiseXor = WordBinopMask::For<DoubleArg<WordBinopOpKind, WordRepresentationEnum>>;

    pub type KBitwiseAnd = WordBinopKindMask::For<SingleArg<WordBinopOpKind>>;
    pub type KBitwiseXor = WordBinopKindMask::For<SingleArg<WordBinopOpKind>>;

    pub type WordUnaryMask =
        MaskBuilder<WordUnaryOp, (OpMaskField<WordUnaryOpKind, { offset_of!(WordUnaryOp, kind) }>, OpMaskField<WordRepresentationEnum, { offset_of!(WordUnaryOp, rep) }>)>;
    pub type KWord32ReverseBytes = WordUnaryMask::For<DoubleArg<WordUnaryOpKind, WordRepresentationEnum>>;
    pub type KWord64ReverseBytes = WordUnaryMask::For<DoubleArg<WordUnaryOpKind, WordRepresentationEnum>>;

    pub type FloatUnaryMask = MaskBuilder<FloatUnaryOp, (OpMaskField<FloatUnaryOpKind, { offset_of!(FloatUnaryOp, kind) }>, OpMaskField<FloatRepresentationEnum, { offset_of!(FloatUnaryOp, rep) }>)>;

    pub type KFloat32Negate = FloatUnaryMask::For<DoubleArg<FloatUnaryOpKind, FloatRepresentationEnum>>;
    pub type KFloat64Abs = FloatUnaryMask::For<DoubleArg<FloatUnaryOpKind, FloatRepresentationEnum>>;
    pub type KFloat64Negate = FloatUnaryMask::For<DoubleArg<FloatUnaryOpKind, FloatRepresentationEnum>>;

    pub type FloatBinopMask = MaskBuilder<FloatBinopOp, (OpMaskField<FloatBinopOpKind, { offset_of!(FloatBinopOp, kind) }>, OpMaskField<FloatRepresentationEnum, { offset_of!(FloatBinopOp, rep) }>)>;

    pub type KFloat32Sub = FloatBinopMask::For<DoubleArg<FloatBinopOpKind, FloatRepresentationEnum>>;
    pub type KFloat32Mul = FloatBinopMask::For<DoubleArg<FloatBinopOpKind, FloatRepresentationEnum>>;
    pub type KFloat64Sub = FloatBinopMask::For<DoubleArg<FloatBinopOpKind, FloatRepresentationEnum>>;
    pub type KFloat64Mul = FloatBinopMask::For<DoubleArg<FloatBinopOpKind, FloatRepresentationEnum>>;

    pub type ShiftMask =
        MaskBuilder<ShiftOp, (OpMaskField<ShiftOpKind, { offset_of!(ShiftOp, kind) }>, OpMaskField<WordRepresentationEnum, { offset_of!(ShiftOp, rep) }>)>;
    pub type ShiftKindMask = MaskBuilder<ShiftOp, OpMaskField<ShiftOpKind, { offset_of!(ShiftOp, kind) }>>;

    pub type KWord32ShiftLeft =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32ShiftRightArithmetic =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32ShiftRightArithmeticShiftOutZeros =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32ShiftRightLogical =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord32RotateRight =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64ShiftLeft =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64ShiftRightArithmetic =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KWord64ShiftRightLogical =
        ShiftMask::For<DoubleArg<ShiftOpKind, WordRepresentationEnum>>; // Replace with actual arguments
    pub type KShiftLeft = ShiftKindMask::For<SingleArg<ShiftOpKind>>;

    pub type PhiMask = MaskBuilder<PhiOp, OpMaskField<RegisterRepresentationEnum, { offset_of!(PhiOp, rep) }>>;
    pub type KTaggedPhi = PhiMask::For<SingleArg<RegisterRepresentationEnum>>;

    pub type ConstantMask = MaskBuilder<ConstantOp, OpMaskField<ConstantOpKind, { offset_of!(ConstantOp, kind) }>>;

    pub type KWord32Constant = ConstantMask::For<SingleArg<ConstantOpKind>>;
    pub type KWord64Constant = ConstantMask::For<SingleArg<ConstantOpKind>>;
    pub type KExternalConstant = ConstantMask::For<SingleArg<ConstantOpKind>>;
    pub type KHeapConstant = ConstantMask::For<SingleArg<ConstantOpKind>>;
    pub type KSmiConstant = ConstantMask::For<SingleArg<ConstantOpKind>>;

    pub type ProjectionMask = MaskBuilder<ProjectionOp, OpMaskField<i32, { offset_of!(ProjectionOp, index) }>>;

    pub type KProjection0 = ProjectionMask::For<SingleArg<i32>>;
    pub type KProjection1 = ProjectionMask::For<SingleArg<i32>>;

    pub type ComparisonMask = MaskBuilder<ComparisonOp, (OpMaskField<ComparisonOpKind, { offset_of!(ComparisonOp, kind) }>, OpMaskField<WordRepresentationEnum, { offset_of!(ComparisonOp, rep) }>)>;

    pub type KWord32Equal = ComparisonMask::For<DoubleArg<ComparisonOpKind, WordRepresentationEnum>>;
    pub type KWord64Equal = ComparisonMask::For<DoubleArg<ComparisonOpKind, WordRepresentationEnum>>;
    pub type ComparisonKindMask = MaskBuilder<ComparisonOp, OpMaskField<ComparisonOpKind, { offset_of!(ComparisonOp, kind) }>>;
    pub type KComparisonEqual = ComparisonKindMask::For<SingleArg<ComparisonOpKind>>;

    pub type ChangeOpMask =
        MaskBuilder<ChangeOp, (OpMaskField<ChangeOpKind, { offset_of!(ChangeOp, kind) }>, OpMaskField<ChangeOpAssumption, { offset_of!(ChangeOp, assumption) }>, OpMaskField<RegisterRepresentationEnum, { offset_of!(ChangeOp, from) }>, OpMaskField<RegisterRepresentationEnum, { offset_of!(ChangeOp, to) }>)>;

    pub type KChangeInt32ToInt64 = ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>;
    pub type KChangeUint32ToUint64 = ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>;
    pub type KFloat64ExtractHighWord32 = ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>;
    pub type KTruncateFloat64ToInt64OverflowToMin =
        ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>; // Replace with actual arguments
    pub type KTruncateFloat32ToInt32OverflowToMin =
        ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>; // Replace with actual arguments
    pub type KTruncateFloat32ToUint32OverflowToMin =
        ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>; // Replace with actual arguments

    pub type KTruncateWord64ToWord32 = ChangeOpMask::For<DoubleArg<ChangeOpKind, ChangeOpAssumption>>;

    pub type OverflowCheckedBinopMask =
        MaskBuilder<OverflowCheckedBinopOp, (OpMaskField<OverflowCheckedBinopOpKind, { offset_of!(OverflowCheckedBinopOp, kind) }>, OpMaskField<WordRepresentationEnum, { offset_of!(OverflowCheckedBinopOp, rep) }>)>;
    pub type KOverflowCheckedWord32Add =
        OverflowCheckedBinopMask::For<DoubleArg<OverflowCheckedBinopOpKind, WordRepresentationEnum>>; // Replace with actual arguments

    pub type TaggedBitcastMask =
        MaskBuilder<TaggedBitcastOp, (OpMaskField<RegisterRepresentationEnum, { offset_of!(TaggedBitcastOp, from) }>, OpMaskField<RegisterRepresentationEnum, { offset_of!(TaggedBitcastOp, to) }>, OpMaskField<TaggedBitcastOpKind, { offset_of!(TaggedBitcastOp, kind) }>)>;
    pub type KBitcastTaggedToWordPtrForTagAndSmiBits =
        TaggedBitcastMask::For<DoubleArg<RegisterRepresentationEnum, RegisterRepresentationEnum>>;
    pub type KBitcastWordPtrToSmi =
        TaggedBitcastMask::For<DoubleArg<RegisterRepresentationEnum, RegisterRepresentationEnum>>;

    pub type TaggedBitcastKindMask =
        MaskBuilder<TaggedBitcastOp, OpMaskField<TaggedBitcastOpKind, { offset_of!(TaggedBitcastOp, kind) }>>;
    pub type KTaggedBitcastSmi =
        TaggedBitcastKindMask::For<SingleArg<TaggedBitcastOpKind>>; // Replace with actual arguments
    pub type KTaggedBitcastHeapObject =
        TaggedBitcastKindMask::For<SingleArg<TaggedBitcastOpKind>>; // Replace with actual arguments

    #[cfg(v8_enable_webassembly)]
    pub mod webassembly {
        use super::*;

        pub type Simd128BinopMask =
            MaskBuilder<Simd128BinopOp, OpMaskField<Simd128BinopOpKind, { offset_of!(Simd128BinopOp, kind) }>>;
        pub type KSimd128I32x4Mul = Simd128BinopMask::For<SingleArg<Simd128BinopOpKind>>;
        pub type KSimd128I16x8Mul = Simd128BinopMask::For<SingleArg<Simd128BinopOpKind>>;

        macro_rules! simd_sign_extension_binop_mask {
            ($kind:ident) => {
                paste::item! {
                    pub type [<KSimd128 $kind>] = Simd128BinopMask::For<SingleArg<Simd128BinopOpKind>>;
                }
            };
        }

        macro_rules! foreach_simd_128_binary_sign_extension_opcode {
            ($macro:ident) => {
                $macro!(I8x16Splat);
                $macro!(I16x8Splat);
            };
        }
        foreach_simd_128_binary_sign_extension_opcode!(simd_sign_extension_binop_mask);

        pub type Simd128UnaryMask =
            MaskBuilder<Simd128UnaryOp, OpMaskField<Simd128UnaryOpKind, { offset_of!(Simd128UnaryOp, kind) }>>;
        macro_rules! simd_unary_mask {
            ($kind:ident) => {
                paste::item! {
                    pub type [<KSimd128 $kind>] = Simd128UnaryMask::For<SingleArg<Simd128UnaryOpKind>>;
                }
            };
        }

        macro_rules! foreach_simd_128_unary_opcode {
            ($macro:ident) => {
                $macro!(Not);
            };
        }
        foreach_simd_128_unary_opcode!(simd_unary_mask);

        pub type Simd128ShiftMask =
            MaskBuilder<Simd128ShiftOp, OpMaskField<Simd128ShiftOpKind, { offset_of!(Simd128ShiftOp, kind) }>>;
        macro_rules! simd_shift_mask {
            ($kind:ident) => {
                paste::item! {
                    pub type [<KSimd128 $kind>] = Simd128ShiftMask::For<SingleArg<Simd128ShiftOpKind>>;
                }
            };
        }

        macro_rules! foreach_simd_128_shift_opcode {
            ($macro:ident) => {
                $macro!(ShiftLeftByScalar);
                $macro!(ShiftRightByScalar);
            };
        }
        foreach_simd_128_shift_opcode!(simd_shift_mask);

        pub type Simd128LoadTransformMask =
            MaskBuilder<
                Simd128LoadTransformOp,
                OpMaskField<Simd128LoadTransformOpTransformKind, { offset_of!(Simd128LoadTransformOp, transform_kind) }>,
            >;
        macro_rules! simd_load_transform_mask {
            ($kind:ident) => {
                paste::item! {
                    pub type [<KSimd128LoadTransform $kind>] = Simd128LoadTransformMask::For<
                        SingleArg<Simd128LoadTransformOpTransformKind>,
                    >;
                }
            };
        }

        macro_rules! foreach_simd_128_load_transform_opcode {
            ($macro:ident) => {
                $macro!(SplatI8x16);
                $macro!(SplatI16x8);
                $macro!(SplatI32x4);
                $macro!(SplatI64x2);
            };
        }
        foreach_simd_128_load_transform_opcode!(simd_load_transform_mask);

        pub type Simd128ReplaceLaneMask =
            MaskBuilder<Simd128ReplaceLaneOp, OpMaskField<Simd128ReplaceLaneOpKind, { offset_of!(Simd128ReplaceLaneOp, kind) }>>;
        pub type KSimd128ReplaceLaneF32x4 =
            Simd128ReplaceLaneMask::For<SingleArg<Simd128ReplaceLaneOpKind>>; // Replace with actual arguments

        #[cfg(v8_enable_wasm_simd256_revec)]
        pub mod simd256 {
            use super::*;

            pub type Simd256UnaryMask =
                MaskBuilder<Simd256UnaryOp, OpMaskField<Simd256UnaryOpKind, { offset_of!(Simd256UnaryOp, kind) }>>;
            macro_rules! simd256_unary_mask {
                ($kind:ident) => {
                    paste::item! {
                        pub type [<KSimd256 $kind>] = Simd256UnaryMask::For<SingleArg<Simd256UnaryOpKind>>;
                    }
                };
            }

            macro_rules! foreach_simd_256_unary_opcode {
                ($macro:ident) => {
                    $macro!(Not);
                };
            }
            foreach_simd_256_unary_opcode!(simd256_unary_mask);
        }
    }
}
