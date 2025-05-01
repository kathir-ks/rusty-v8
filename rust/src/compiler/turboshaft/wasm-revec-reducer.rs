// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

// TODO: Add cfg attribute for wasm
// #![cfg(target_arch = "wasm32")]

use std::cmp::min;
use std::mem::size_of;
use std::option::Option;
use std::vec::Vec;
//use std::collections::{HashMap, HashSet};
//use crate::compiler::turboshaft::assembler::*;
//use crate::compiler::turboshaft::operations::*;
//use crate::compiler::turboshaft::phase::*;
//use crate::compiler::turboshaft::use_map::*;
//use crate::compiler::wasm_graph_assembler::*;

macro_rules! simd256_loadtransform_op {
    ($v:ident) => {
        $v!(8x8S, 8x16S);
        $v!(8x8U, 8x16U);
        $v!(16x4S, 16x8S);
        $v!(16x4U, 16x8U);
        $v!(32x2S, 32x4S);
        $v!(32x2U, 32x4U);
        $v!(8Splat, 8Splat);
        $v!(16Splat, 16Splat);
        $v!(32Splat, 32Splat);
        $v!(64Splat, 64Splat);
    };
}

macro_rules! simd256_unary_simple_op {
    ($v:ident) => {
        $v!(S128Not, S256Not);
        $v!(I8x16Abs, I8x32Abs);
        $v!(I8x16Neg, I8x32Neg);
        $v!(I16x8ExtAddPairwiseI8x16S, I16x16ExtAddPairwiseI8x32S);
        $v!(I16x8ExtAddPairwiseI8x16U, I16x16ExtAddPairwiseI8x32U);
        $v!(I32x4ExtAddPairwiseI16x8S, I32x8ExtAddPairwiseI16x16S);
        $v!(I32x4ExtAddPairwiseI16x8U, I32x8ExtAddPairwiseI16x16U);
        $v!(I16x8Abs, I16x16Abs);
        $v!(I16x8Neg, I16x16Neg);
        $v!(I32x4Abs, I32x8Abs);
        $v!(I32x4Neg, I32x8Neg);
        $v!(F32x4Abs, F32x8Abs);
        $v!(F32x4Neg, F32x8Neg);
        $v!(F32x4Sqrt, F32x8Sqrt);
        $v!(F64x2Abs, F64x4Abs);
        $v!(F64x2Neg, F64x4Neg);
        $v!(F64x2Sqrt, F64x4Sqrt);
        $v!(I32x4UConvertF32x4, I32x8UConvertF32x8);
        $v!(I32x4SConvertF32x4, I32x8SConvertF32x8);
        $v!(F32x4UConvertI32x4, F32x8UConvertI32x8);
        $v!(F32x4SConvertI32x4, F32x8SConvertI32x8);
        $v!(I32x4RelaxedTruncF32x4S, I32x8RelaxedTruncF32x8S);
        $v!(I32x4RelaxedTruncF32x4U, I32x8RelaxedTruncF32x8U);
    };
}

macro_rules! simd256_unary_sign_extension_op {
    ($v:ident) => {
        $v!(I64x2SConvertI32x4Low, I64x4SConvertI32x4, I64x2SConvertI32x4High);
        $v!(I64x2UConvertI32x4Low, I64x4UConvertI32x4, I64x2UConvertI32x4High);
        $v!(I32x4SConvertI16x8Low, I32x8SConvertI16x8, I32x4SConvertI16x8High);
        $v!(I32x4UConvertI16x8Low, I32x8UConvertI16x8, I32x4UConvertI16x8High);
        $v!(I16x8SConvertI8x16Low, I16x16SConvertI8x16, I16x8SConvertI8x16High);
        $v!(I16x8UConvertI8x16Low, I16x16UConvertI8x16, I16x8UConvertI8x16High);
    };
}

macro_rules! simd256_binop_simple_op {
    ($v:ident) => {
        $v!(I8x16Eq, I8x32Eq);
        $v!(I8x16Ne, I8x32Ne);
        $v!(I8x16GtS, I8x32GtS);
        $v!(I8x16GtU, I8x32GtU);
        $v!(I8x16GeS, I8x32GeS);
        $v!(I8x16GeU, I8x32GeU);
        $v!(I16x8Eq, I16x16Eq);
        $v!(I16x8Ne, I16x16Ne);
        $v!(I16x8GtS, I16x16GtS);
        $v!(I16x8GtU, I16x16GtU);
        $v!(I16x8GeS, I16x16GeS);
        $v!(I16x8GeU, I16x16GeU);
        $v!(I32x4Eq, I32x8Eq);
        $v!(I32x4Ne, I32x8Ne);
        $v!(I32x4GtS, I32x8GtS);
        $v!(I32x4GtU, I32x8GtU);
        $v!(I32x4GeS, I32x8GeS);
        $v!(I32x4GeU, I32x8GeU);
        $v!(F32x4Eq, F32x8Eq);
        $v!(F32x4Ne, F32x8Ne);
        $v!(F32x4Lt, F32x8Lt);
        $v!(F32x4Le, F32x8Le);
        $v!(F64x2Eq, F64x4Eq);
        $v!(F64x2Ne, F64x4Ne);
        $v!(F64x2Lt, F64x4Lt);
        $v!(F64x2Le, F64x4Le);
        $v!(S128And, S256And);
        $v!(S128AndNot, S256AndNot);
        $v!(S128Or, S256Or);
        $v!(S128Xor, S256Xor);
        $v!(I8x16SConvertI16x8, I8x32SConvertI16x16);
        $v!(I8x16UConvertI16x8, I8x32UConvertI16x16);
        $v!(I8x16Add, I8x32Add);
        $v!(I8x16AddSatS, I8x32AddSatS);
        $v!(I8x16AddSatU, I8x32AddSatU);
        $v!(I8x16Sub, I8x32Sub);
        $v!(I8x16SubSatS, I8x32SubSatS);
        $v!(I8x16SubSatU, I8x32SubSatU);
        $v!(I8x16MinS, I8x32MinS);
        $v!(I8x16MinU, I8x32MinU);
        $v!(I8x16MaxS, I8x32MaxS);
        $v!(I8x16MaxU, I8x32MaxU);
        $v!(I8x16RoundingAverageU, I8x32RoundingAverageU);
        $v!(I16x8SConvertI32x4, I16x16SConvertI32x8);
        $v!(I16x8UConvertI32x4, I16x16UConvertI32x8);
        $v!(I16x8Add, I16x16Add);
        $v!(I16x8AddSatS, I16x16AddSatS);
        $v!(I16x8AddSatU, I16x16AddSatU);
        $v!(I16x8Sub, I16x16Sub);
        $v!(I16x8SubSatS, I16x16SubSatS);
        $v!(I16x8SubSatU, I16x16SubSatU);
        $v!(I16x8Mul, I16x16Mul);
        $v!(I16x8MinS, I16x16MinS);
        $v!(I16x8MinU, I16x16MinU);
        $v!(I16x8MaxS, I16x16MaxS);
        $v!(I16x8MaxU, I16x16MaxU);
        $v!(I16x8RoundingAverageU, I16x16RoundingAverageU);
        $v!(I32x4Add, I32x8Add);
        $v!(I32x4Sub, I32x8Sub);
        $v!(I32x4Mul, I32x8Mul);
        $v!(I32x4MinS, I32x8MinS);
        $v!(I32x4MinU, I32x8MinU);
        $v!(I32x4MaxS, I32x8MaxS);
        $v!(I32x4MaxU, I32x8MaxU);
        $v!(I32x4DotI16x8S, I32x8DotI16x16S);
        $v!(I64x2Add, I64x4Add);
        $v!(I64x2Sub, I64x4Sub);
        $v!(I64x2Mul, I64x4Mul);
        $v!(I64x2Eq, I64x4Eq);
        $v!(I64x2Ne, I64x4Ne);
        $v!(I64x2GtS, I64x4GtS);
        $v!(I64x2GeS, I64x4GeS);
        $v!(F32x4Add, F32x8Add);
        $v!(F32x4Sub, F32x8Sub);
        $v!(F32x4Mul, F32x8Mul);
        $v!(F32x4Div, F32x8Div);
        $v!(F32x4Min, F32x8Min);
        $v!(F32x4Max, F32x8Max);
        $v!(F32x4Pmin, F32x8Pmin);
        $v!(F32x4Pmax, F32x8Pmax);
        $v!(F64x2Add, F64x4Add);
        $v!(F64x2Sub, F64x4Sub);
        $v!(F64x2Mul, F64x4Mul);
        $v!(F64x2Div, F64x4Div);
        $v!(F64x2Min, F64x4Min);
        $v!(F64x2Max, F64x4Max);
        $v!(F64x2Pmin, F64x4Pmin);
        $v!(F64x2Pmax, F64x4Pmax);
        $v!(F32x4RelaxedMin, F32x8RelaxedMin);
        $v!(F32x4RelaxedMax, F32x8RelaxedMax);
        $v!(F64x2RelaxedMin, F64x4RelaxedMin);
        $v!(F64x2RelaxedMax, F64x4RelaxedMax);
        $v!(I16x8DotI8x16I7x16S, I16x16DotI8x32I7x32S);
    };
}

macro_rules! simd256_binop_sign_extension_op {
    ($v:ident) => {
        $v!(I16x8ExtMulLowI8x16S, I16x16ExtMulI8x16S, I16x8ExtMulHighI8x16S);
        $v!(I16x8ExtMulLowI8x16U, I16x16ExtMulI8x16U, I16x8ExtMulHighI8x16U);
        $v!(I32x4ExtMulLowI16x8S, I32x8ExtMulI16x8S, I32x4ExtMulHighI16x8S);
        $v!(I32x4ExtMulLowI16x8U, I32x8ExtMulI16x8U, I32x4ExtMulHighI16x8U);
        $v!(I64x2ExtMulLowI32x4S, I64x4ExtMulI32x4S, I64x2ExtMulHighI32x4S);
        $v!(I64x2ExtMulLowI32x4U, I64x4ExtMulI32x4U, I64x2ExtMulHighI32x4U);
    };
}

macro_rules! simd256_shift_op {
    ($v:ident) => {
        $v!(I16x8Shl, I16x16Shl);
        $v!(I16x8ShrS, I16x16ShrS);
        $v!(I16x8ShrU, I16x16ShrU);
        $v!(I32x4Shl, I32x8Shl);
        $v!(I32x4ShrS, I32x8ShrS);
        $v!(I32x4ShrU, I32x8ShrU);
        $v!(I64x2Shl, I64x4Shl);
        $v!(I64x2ShrU, I64x4ShrU);
    };
}

macro_rules! simd256_ternary_op {
    ($v:ident) => {
        $v!(S128Select, S256Select);
        $v!(F32x4Qfma, F32x8Qfma);
        $v!(F32x4Qfms, F32x8Qfms);
        $v!(F64x2Qfma, F64x4Qfma);
        $v!(F64x2Qfms, F64x4Qfms);
        $v!(I8x16RelaxedLaneSelect, I8x32RelaxedLaneSelect);
        $v!(I16x8RelaxedLaneSelect, I16x16RelaxedLaneSelect);
        $v!(I32x4RelaxedLaneSelect, I32x8RelaxedLaneSelect);
        $v!(I64x2RelaxedLaneSelect, I64x4RelaxedLaneSelect);
        $v!(I32x4DotI8x16I7x16AddS, I32x8DotI8x32I7x32AddS);
    };
}

macro_rules! simd256_splat_op {
    ($v:ident) => {
        $v!(I8x16, I8x32);
        $v!(I16x8, I16x16);
        $v!(I32x4, I32x8);
        $v!(I64x2, I64x4);
        $v!(F32x4, F32x8);
        $v!(F64x2, F64x4);
    };
}

macro_rules! reduce_seed_kind {
    ($v:ident) => {
        $v!(I64x2Add);
        $v!(I32x4Add);
        $v!(I8x16Add);
        $v!(I16x8AddSatS);
        $v!(I16x8AddSatU);
        $v!(I8x16AddSatS);
        $v!(I8x16AddSatU);
        $v!(I16x8SConvertI32x4);
        $v!(I16x8UConvertI32x4);
        $v!(I8x16SConvertI16x8);
        $v!(I8x16UConvertI16x8);
    };
}

const kSimd128Size: usize = 16;
const kSimd256Size: usize = 32;

mod turboshaft {
    use super::*;
    //use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OpIndex(usize);

    impl OpIndex {
        pub fn new(index: usize) -> Self {
            OpIndex(index)
        }

        pub fn invalid() -> Self {
            OpIndex(usize::MAX)
        }

        pub fn is_valid(&self) -> bool {
            self.0 != usize::MAX
        }

        pub fn to_usize(&self) -> usize {
            self.0
        }

        pub fn from_usize(value: usize) -> Self{
            OpIndex(value)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum RegisterRepresentation {
        Simd128(),
        Simd256(),
        Word32(),
        WordPtr(),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Opcode {
        Simd128Constant,
        Simd128LoadTransform,
        Load,
        Store,
        Phi,
        Simd128Unary,
        Simd128Binop,
        Simd128Shift,
        Simd128Ternary,
        Simd128Splat,
        Simd128Shuffle,
        Simd128ReplaceLane,
        Simd256Extract128Lane,
        SimdPack128To256,
        Simd256Constant,
        Simd256LoadTransform,
        Simd256Unary,
        Simd256Binop,
        Simd256Shift,
        Simd256Ternary,
        Simd256Splat,
        Simd256Shufd,
        Simd256Shufps,
        Simd256Unpack,
        PendingLoopPhi,
        Word32Constant,
        IntPtrConstant,
        WordPtrAdd,
        Block,
    }

    pub trait OperationTrait {
        fn opcode(&self) -> Opcode;
        fn inputs(&self) -> Vec<OpIndex>;
    }

    pub struct Graph {
        operations: RefCell<Vec<Operation>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                operations: RefCell::new(Vec::new()),
            }
        }

        pub fn create<T>(&self, operation: T) -> OpIndex
            where
                T: Into<Operation>
        {
            let mut ops = self.operations.borrow_mut();
            ops.push(operation.into());
            OpIndex((ops.len() - 1) as usize)
        }

        pub fn get(&self, index: OpIndex) -> Operation {
            self.operations.borrow()[index.0].clone()
        }

        pub fn index(&self, operation: Operation) -> OpIndex{
            let ops = self.operations.borrow();
            for (i, op) in ops.iter().enumerate() {
                if *op == operation {
                    return OpIndex(i);
                }
            }
            OpIndex::invalid()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Operation {
        Simd128Constant(Simd128ConstantOp),
        Simd128LoadTransform(Simd128LoadTransformOp),
        Load(LoadOp),
        Store(StoreOp),
        Phi(PhiOp),
        Simd128Unary(Simd128UnaryOp),
        Simd128Binop(Simd128BinopOp),
        Simd128Shift(Simd128ShiftOp),
        Simd128Ternary(Simd128TernaryOp),
        Simd128Splat(Simd128SplatOp),
        Simd128Shuffle(Simd128ShuffleOp),
        Simd128ReplaceLane(Simd128ReplaceLaneOp),
        Simd256Extract128Lane(Simd256Extract128LaneOp),
        SimdPack128To256(SimdPack128To256Op),
        Simd256Constant(Simd256ConstantOp),
        Simd256LoadTransform(Simd256LoadTransformOp),
        Simd256Unary(Simd256UnaryOp),
        Simd256Binop(Simd256BinopOp),
        Simd256Shift(Simd256ShiftOp),
        Simd256Ternary(Simd256TernaryOp),
        Simd256Splat(Simd256SplatOp),
        Simd256Shufd(Simd256ShufdOp),
        Simd256Shufps(Simd256ShufpsOp),
        Simd256Unpack(Simd256UnpackOp),
        PendingLoopPhi(PendingLoopPhiOp),
        Word32Constant(Word32ConstantOp),
        IntPtrConstant(IntPtrConstantOp),
        WordPtrAdd(WordPtrAddOp),
        Block(BlockOp),
    }

    impl OperationTrait for Operation {
        fn opcode(&self) -> Opcode {
            match self {
                Operation::Simd128Constant(_) => Opcode::Simd128Constant,
                Operation::Simd128LoadTransform(_) => Opcode::Simd128LoadTransform,
                Operation::Load(_) => Opcode::Load,
                Operation::Store(_) => Opcode::Store,
                Operation::Phi(_) => Opcode::Phi,
                Operation::Simd128Unary(_) => Opcode::Simd128Unary,
                Operation::Simd128Binop(_) => Opcode::Simd128Binop,
                Operation::Simd128Shift(_) => Opcode::Simd128Shift,
                Operation::Simd128Ternary(_) => Opcode::Simd128Ternary,
                Operation::Simd128Splat(_) => Opcode::Simd128Splat,
                Operation::Simd128Shuffle(_) => Opcode::Simd128Shuffle,
                Operation::Simd128ReplaceLane(_) => Opcode::Simd128ReplaceLane,
                Operation::Simd256Extract128Lane(_) => Opcode::Simd256Extract128Lane,
                Operation::SimdPack128To256(_) => Opcode::SimdPack128To256,
                Operation::Simd256Constant(_) => Opcode::Simd256Constant,
                Operation::Simd256LoadTransform(_) => Opcode::Simd256LoadTransform,
                Operation::Simd256Unary(_) => Opcode::Simd256Unary,
                Operation::Simd256Binop(_) => Opcode::Simd256Binop,
                Operation::Simd256Shift(_) => Opcode::Simd256Shift,
                Operation::Simd256Ternary(_) => Opcode::Simd256Ternary,
                Operation::Simd256Splat(_) => Opcode::Simd256Splat,
                Operation::Simd256Shufd(_) => Opcode::Simd256Shufd,
                Operation::Simd256Shufps(_) => Opcode::Simd256Shufps,
                Operation::Simd256Unpack(_) => Opcode::Simd256Unpack,
                Operation::PendingLoopPhi(_) => Opcode::PendingLoopPhi,
                Operation::Word32Constant(_) => Opcode::Word32Constant,
                Operation::IntPtrConstant(_) => Opcode::IntPtrConstant,
                Operation::WordPtrAdd(_) => Opcode::WordPtrAdd,
                Operation::Block(_) => Opcode::Block,
            }
        }

        fn inputs(&self) -> Vec<OpIndex> {
            match self {
                Operation::Simd128Constant(_) => Vec::new(),
                Operation::Simd128LoadTransform(op) => vec![op.base, op.index],
                Operation::Load(op) => vec![op.base, op.index],
                Operation::Store(op) => vec![op.base, op.index, op.value],
                Operation::Phi(op) => op.inputs.clone(),
                Operation::Simd128Unary(op) => vec![op.input],
                Operation::Simd128Binop(op) => vec![op.left, op.right],
                Operation::Simd128Shift(op) => vec![op.input, op.shift],
                Operation::Simd128Ternary(op) => vec![op.first, op.second, op.third],
                Operation::Simd128Splat(op) => vec![op.input],
                Operation::Simd128Shuffle(op) => vec![op.left, op.right],
                Operation::Simd128ReplaceLane(op) => vec![op.base, op.value],
                Operation::Simd256Extract128Lane(op) => vec![op.input],
                Operation::SimdPack128To256(op) => vec![op.left, op.right],
                Operation::Simd256Constant(_) => Vec::new(),
                Operation::Simd256LoadTransform(op) => vec![op.base, op.index],
                Operation::Simd256Unary(op) => vec![op.input],
                Operation::Simd256Binop(op) => vec![op.left, op.right],
                Operation::Simd256Shift(op) => vec![op.input, op.shift],
                Operation::Simd256Ternary(op) => vec![op.first, op.second, op.third],
                Operation::Simd256Splat(op) => vec![op.input],
                Operation::Simd256Shufd(op) => vec![op.input],
                Operation::Simd256Shufps(op) => vec![op.left, op.right],
                Operation::Simd256Unpack(op) => vec![op.left, op.right],
                Operation::PendingLoopPhi(_) => Vec::new(),
                Operation::Word32Constant(_) => Vec::new(),
                Operation::IntPtrConstant(_) => Vec::new(),
                Operation::WordPtrAdd(op) => vec![op.left, op.right],
                Operation::Block(_) => Vec::new(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Simd128ConstantOp {
        pub value: [u8; kSimd128Size],
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Simd128LoadTransformOp {
        pub base: OpIndex,
        pub index: OpIndex,
        pub load_kind: LoadKind,
        pub transform_kind: Simd128LoadTransformOp::TransformKind,
        pub offset: i32,
    }

    impl Simd128LoadTransformOp {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum TransformKind {
            k8x8S,
            k8x16S,
            k8x8U,
            k8x16U,
            k16x4S,
            k16x8S,
            k16x4U,
            k16x8U,
            k32x2S,
            k32x4S,
            k32x2U,
            k32x4U,
            k8Splat,
            k16Splat,
            k32Splat,
            k64Splat,
            k64Zero,
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct LoadOp {
        pub base: OpIndex,
        pub index: OpIndex,
        pub kind: LoadKind,
        pub offset: i32,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct StoreOp {
        pub base: OpIndex,
        pub index: OpIndex,
        pub value: OpIndex,
        pub kind: StoreKind,
        pub write_barrier: bool,
        pub offset: i32,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct PhiOp {
        pub inputs: Vec<OpIndex>,
        pub rep: RegisterRepresentation,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Simd128UnaryOp {
        pub input: OpIndex,
        pub kind: Simd128UnaryOp::Kind,
    }

    impl Simd128UnaryOp {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Kind {
            S128Not,
            I8x16Abs,
            I8x16Neg,
            I16x8ExtAddPairwiseI8x16S,
            I16x8ExtAddPairwiseI8x16U,
            I32x4ExtAddPairwiseI16x8S,
            I32x4ExtAddPairwiseI16x8U,
            I16x8Abs,
            I16x8Neg,
            I32x4Abs,
            I32x4Neg,
            F32x4Abs,
            F32x4Neg,
            F32x4Sqrt,
            F64x2Abs,
            F64x2Neg,
            F64x2Sqrt,
            I32x4UConvertF32x4,
            I32x4SConvertF32x4,
            F32x4UConvertI32x4,
            F32x4SConvertI32x4,
            I32x4RelaxedTruncF32x4S,
            I32x4RelaxedTruncF32x4U,
            I64x2SConvertI32x4Low,
            I64x2UConvertI32x4Low,
            I32x4SConvertI16x8Low,
            I32x4UConvertI16x8Low,
            I16x8SConvertI8x16Low,
            I16x8UConvertI8x16Low,
            I64x2SConvertI32x4High,
            I64x2UConvertI32x4High,
            I32x4SConvertI16x8High,
            I32x4UConvertI16x8High,
            I16x8SConvertI8x16High,
            I16x8UConvertI8x16High,
            I16x8SConvertI8