// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod raw_machine_assembler {
    use std::{
        borrow::Cow,
        collections::HashMap,
        convert::TryInto,
        fmt,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        marker::PhantomData,
        mem,
        ops::{BitAnd, BitOr, BitXor},
        string::String,
        sync::{Arc, Mutex},
    };

    // Placeholder for Isolate
    pub struct Isolate {}
    impl Isolate {
        pub fn factory(&self) -> Factory {
            Factory {}
        }
    }

    // Placeholder for Factory
    pub struct Factory {}
    impl Factory {
        pub fn InternalizeUtf8String(&self, string: &str) -> Handle<HeapObject> {
            Handle::new(HeapObject {})
        }
    }

    // Placeholder for Handle
    #[derive(Clone)]
    pub struct Handle<T> {
        _phantom: PhantomData<T>,
    }
    impl<T> Handle<T> {
        pub fn new(_obj: T) -> Self {
            Handle {
                _phantom: PhantomData,
            }
        }
    }

    // Placeholder for HeapObject
    #[derive(Clone)]
    pub struct HeapObject {}

    // Placeholder for RelocInfo
    pub mod RelocInfo {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Mode {
            kNoRelocInfo,
        }
    }

    // Placeholder for ExternalReference
    #[derive(Clone)]
    pub struct ExternalReference {}

    // Placeholder for TFGraph
    pub struct TFGraph {
        zone: Zone,
    }
    impl TFGraph {
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
    }

    // Placeholder for Schedule
    pub struct Schedule {}

    // Placeholder for SourcePositionTable
    pub struct SourcePositionTable {}

    // Placeholder for BasicBlock
    pub struct BasicBlock {}

    // Placeholder for CallDescriptor
    pub struct CallDescriptor {
        parameter_count: usize,
    }
    impl CallDescriptor {
        pub fn ParameterCount(&self) -> usize {
            self.parameter_count
        }
    }

    // Placeholder for Zone
    pub struct Zone {}

    // Placeholder for Node
    #[derive(Clone, Debug)]
    pub struct Node {
        opcode: IrOpcode,
        inputs: Vec<NodeRef>,
    }

    impl Node {
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
        pub fn inputs(&self) -> &Vec<NodeRef> {
            &self.inputs
        }
    }

    // Placeholder for NodeRef
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NodeRef(usize);

    // Placeholder for FileAndLine
    pub struct FileAndLine {}

    // Placeholder for RelocInfo::Mode
    // Placeholder for AssemblerDebugInfo
    // Placeholder for BranchHint
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum BranchHint {
        kNone,
    }

    // Placeholder for StackCheckKind
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StackCheckKind {
        kCodeStubAssembler,
    }

    // Placeholder for AllocationType
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum AllocationType {
        kYoung,
    }

    // Placeholder for IndirectPointerTag
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum IndirectPointerTag {
        kFull32Bit,
    }

    // Placeholder for SaveFPRegsMode
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum SaveFPRegsMode {
        kSaveFPRegs,
    }

    // Placeholder for CommonOperatorBuilder
    pub struct CommonOperatorBuilder {}
    impl CommonOperatorBuilder {
        pub fn Int32Constant(&self, value: i32) -> Operator {
            Operator::CommonInt32Constant { value }
        }
        pub fn Int64Constant(&self, value: i64) -> Operator {
            Operator::CommonInt64Constant { value }
        }
        pub fn NumberConstant(&self, value: f64) -> Operator {
            Operator::CommonNumberConstant { value }
        }
        pub fn Float32Constant(&self, value: f32) -> Operator {
            Operator::CommonFloat32Constant { value }
        }
        pub fn Float64Constant(&self, value: f64) -> Operator {
            Operator::CommonFloat64Constant { value }
        }
        pub fn HeapConstant(&self, object: Handle<HeapObject>) -> Operator {
            Operator::CommonHeapConstant { object }
        }
        pub fn ExternalConstant(&self, address: ExternalReference) -> Operator {
            Operator::CommonExternalConstant { address }
        }
        pub fn RelocatableInt32Constant(&self, value: i32, rmode: RelocInfo::Mode) -> Operator {
            Operator::CommonRelocatableInt32Constant { value, rmode }
        }
        pub fn RelocatableInt64Constant(&self, value: i64, rmode: RelocInfo::Mode) -> Operator {
            Operator::CommonRelocatableInt64Constant { value, rmode }
        }
        pub fn Projection(&self, index: i32) -> Operator {
            Operator::CommonProjection { index }
        }
        pub fn Phi(&self, rep: MachineRepresentation, count: usize) -> Operator {
            Operator::CommonPhi { rep, count }
        }
        pub fn Retain(&self) -> Operator {
            Operator::CommonRetain {}
        }
    }

    // Placeholder for SimplifiedOperatorBuilder
    pub struct SimplifiedOperatorBuilder {}
    impl SimplifiedOperatorBuilder {
        pub fn LoadFromObject(&self, access: ObjectAccess) -> Operator {
            Operator::SimplifiedLoadFromObject { access }
        }
        pub fn StoreToObject(&self, access: ObjectAccess) -> Operator {
            Operator::SimplifiedStoreToObject { access }
        }
        pub fn StoreField(&self, access: FieldAccess) -> Operator {
            Operator::SimplifiedStoreField { access }
        }
    }

    // Placeholder for MachineOperatorBuilder
    pub struct MachineOperatorBuilder {
        flags: Flags,
        alignment_requirements: AlignmentRequirements,
    }
    impl MachineOperatorBuilder {
        pub fn new(
            word: MachineRepresentation,
            flags: Flags,
            alignment_requirements: AlignmentRequirements,
        ) -> Self {
            MachineOperatorBuilder {
                flags,
                alignment_requirements,
            }
        }
        pub fn StackSlot(&self, rep: MachineRepresentation, alignment: i32) -> Operator {
            Operator::MachineStackSlot { rep, alignment }
        }
        pub fn Load(&self, type_: MachineType) -> Operator {
            Operator::MachineLoad { type_: type_ }
        }
        pub fn LoadImmutable(&self, type_: MachineType) -> Operator {
            Operator::MachineLoadImmutable { type_: type_ }
        }
        pub fn Store(&self, store_rep: StoreRepresentation) -> Operator {
            Operator::MachineStore {
                store_rep: store_rep,
            }
        }
        pub fn UnalignedLoadSupported(&self, rep: MachineRepresentation) -> bool {
            true
        }
        pub fn UnalignedLoad(&self, type_: MachineType) -> Operator {
            Operator::MachineUnalignedLoad { type_: type_ }
        }
        pub fn UnalignedStoreSupported(&self, rep: MachineRepresentation) -> bool {
            true
        }
        pub fn UnalignedStore(&self, store_rep: UnalignedStoreRepresentation) -> Operator {
            Operator::MachineUnalignedStore {
                store_rep: store_rep,
            }
        }

        pub fn Word32AtomicLoad(&self, rep: AtomicLoadParameters) -> Operator {
            Operator::MachineWord32AtomicLoad { rep }
        }
        pub fn Word64AtomicLoad(&self, rep: AtomicLoadParameters) -> Operator {
            Operator::MachineWord64AtomicLoad { rep }
        }
        pub fn Word32AtomicPairLoad(&self, order: AtomicMemoryOrder) -> Operator {
            Operator::MachineWord32AtomicPairLoad { order }
        }

        pub fn Word32AtomicStore(&self, params: AtomicStoreParameters) -> Operator {
            Operator::MachineWord32AtomicStore { params }
        }
        pub fn Word64AtomicStore(&self, params: AtomicStoreParameters) -> Operator {
            Operator::MachineWord64AtomicStore { params }
        }
        pub fn Word32AtomicPairStore(&self, order: AtomicMemoryOrder) -> Operator {
            Operator::MachineWord32AtomicPairStore { order }
        }

        pub fn Word32AtomicExchange(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicExchange { type_: type_ }
        }
        pub fn Word64AtomicExchange(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicExchange { type_: type_ }
        }
        pub fn Word32AtomicPairExchange(&self) -> Operator {
            Operator::MachineWord32AtomicPairExchange {}
        }

        pub fn Word32AtomicAdd(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicAdd { type_: type_ }
        }
        pub fn Word64AtomicAdd(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicAdd { type_: type_ }
        }
        pub fn Word32AtomicPairAdd(&self) -> Operator {
            Operator::MachineWord32AtomicPairAdd {}
        }

        pub fn Word32AtomicSub(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicSub { type_: type_ }
        }
        pub fn Word64AtomicSub(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicSub { type_: type_ }
        }
        pub fn Word32AtomicPairSub(&self) -> Operator {
            Operator::MachineWord32AtomicPairSub {}
        }

        pub fn Word32AtomicAnd(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicAnd { type_: type_ }
        }
        pub fn Word64AtomicAnd(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicAnd { type_: type_ }
        }
        pub fn Word32AtomicPairAnd(&self) -> Operator {
            Operator::MachineWord32AtomicPairAnd {}
        }

        pub fn Word32AtomicOr(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicOr { type_: type_ }
        }
        pub fn Word64AtomicOr(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicOr { type_: type_ }
        }
        pub fn Word32AtomicPairOr(&self) -> Operator {
            Operator::MachineWord32AtomicPairOr {}
        }

        pub fn Word32AtomicXor(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicXor { type_: type_ }
        }
        pub fn Word64AtomicXor(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicXor { type_: type_ }
        }
        pub fn Word32AtomicPairXor(&self) -> Operator {
            Operator::MachineWord32AtomicPairXor {}
        }

        pub fn Word32AtomicCompareExchange(&self, type_: MachineType) -> Operator {
            Operator::MachineWord32AtomicCompareExchange { type_: type_ }
        }
        pub fn Word64AtomicCompareExchange(&self, type_: MachineType) -> Operator {
            Operator::MachineWord64AtomicCompareExchange { type_: type_ }
        }
        pub fn Word32AtomicPairCompareExchange(&self) -> Operator {
            Operator::MachineWord32AtomicPairCompareExchange {}
        }

        pub fn MemoryBarrier(&self, order: AtomicMemoryOrder) -> Operator {
            Operator::MachineMemoryBarrier { order }
        }

        pub fn WordAnd(&self) -> Operator {
            Operator::MachineWordAnd {}
        }
        pub fn WordOr(&self) -> Operator {
            Operator::MachineWordOr {}
        }
        pub fn WordXor(&self) -> Operator {
            Operator::MachineWordXor {}
        }
        pub fn WordShl(&self) -> Operator {
            Operator::MachineWordShl {}
        }
        pub fn WordShr(&self) -> Operator {
            Operator::MachineWordShr {}
        }
        pub fn WordSar(&self) -> Operator {
            Operator::MachineWordSar {}
        }
        pub fn WordSarShiftOutZeros(&self) -> Operator {
            Operator::MachineWordSarShiftOutZeros {}
        }
        pub fn WordRor(&self) -> Operator {
            Operator::MachineWordRor {}
        }
        pub fn WordEqual(&self) -> Operator {
            Operator::MachineWordEqual {}
        }

        pub fn Word32And(&self) -> Operator {
            Operator::MachineWord32And {}
        }
        pub fn Word32Or(&self) -> Operator {
            Operator::MachineWord32Or {}
        }
        pub fn Word32Xor(&self) -> Operator {
            Operator::MachineWord32Xor {}
        }
        pub fn Word32Shl(&self) -> Operator {
            Operator::MachineWord32Shl {}
        }
        pub fn Word32Shr(&self) -> Operator {
            Operator::MachineWord32Shr {}
        }
        pub fn Word32Sar(&self) -> Operator {
            Operator::MachineWord32Sar {}
        }
        pub fn Word32SarShiftOutZeros(&self) -> Operator {
            Operator::MachineWord32SarShiftOutZeros {}
        }
        pub fn Word32Ror(&self) -> Operator {
            Operator::MachineWord32Ror {}
        }
        pub fn Word32Clz(&self) -> Operator {
            Operator::MachineWord32Clz {}
        }
        pub fn Word32Equal(&self) -> Operator {
            Operator::MachineWord32Equal {}
        }

        pub fn Word64And(&self) -> Operator {
            Operator::MachineWord64And {}
        }
        pub fn Word64Or(&self) -> Operator {
            Operator::MachineWord64Or {}
        }
        pub fn Word64Xor(&self) -> Operator {
            Operator::MachineWord64Xor {}
        }
        pub fn Word64Shl(&self) -> Operator {
            Operator::MachineWord64Shl {}
        }
        pub fn Word64Shr(&self) -> Operator {
            Operator::MachineWord64Shr {}
        }
        pub fn Word64Sar(&self) -> Operator {
            Operator::MachineWord64Sar {}
        }
        pub fn Word64Ror(&self) -> Operator {
            Operator::MachineWord64Ror {}
        }
        pub fn Word64Clz(&self) -> Operator {
            Operator::MachineWord64Clz {}
        }
        pub fn Word64Equal(&self) -> Operator {
            Operator::MachineWord64Equal {}
        }

        pub fn Int32Add(&self) -> Operator {
            Operator::MachineInt32Add {}
        }
        pub fn Int32AddWithOverflow(&self) -> Operator {
            Operator::MachineInt32AddWithOverflow {}
        }
        pub fn Int32Sub(&self) -> Operator {
            Operator::MachineInt32Sub {}
        }
        pub fn Int32SubWithOverflow(&self) -> Operator {
            Operator::MachineInt32SubWithOverflow {}
        }
        pub fn Int32Mul(&self) -> Operator {
            Operator::MachineInt32Mul {}
        }
        pub fn Int32MulHigh(&self) -> Operator {
            Operator::MachineInt32MulHigh {}
        }
        pub fn Int32MulWithOverflow(&self) -> Operator {
            Operator::MachineInt32MulWithOverflow {}
        }
        pub fn Int32Div(&self) -> Operator {
            Operator::MachineInt32Div {}
        }
        pub fn Int32Mod(&self) -> Operator {
            Operator::MachineInt32Mod {}
        }
        pub fn Int32LessThan(&self) -> Operator {
            Operator::MachineInt32LessThan {}
        }
        pub fn Int32LessThanOrEqual(&self) -> Operator {
            Operator::MachineInt32LessThanOrEqual {}
        }
        pub fn Uint32Div(&self) -> Operator {
            Operator::MachineUint32Div {}
        }
        pub fn Uint32LessThan(&self) -> Operator {
            Operator::MachineUint32LessThan {}
        }
        pub fn Uint32LessThanOrEqual(&self) -> Operator {
            Operator::MachineUint32LessThanOrEqual {}
        }
        pub fn Uint32Mod(&self) -> Operator {
            Operator::MachineUint32Mod {}
        }
        pub fn Uint32MulHigh(&self) -> Operator {
            Operator::MachineUint32MulHigh {}
        }

        pub fn Int64Add(&self) -> Operator {
            Operator::MachineInt64Add {}
        }
        pub fn Int64AddWithOverflow(&self) -> Operator {
            Operator::MachineInt64AddWithOverflow {}
        }
        pub fn Int64Sub(&self) -> Operator {
            Operator::MachineInt64Sub {}
        }
        pub fn Int64SubWithOverflow(&self) -> Operator {
            Operator::MachineInt64SubWithOverflow {}
        }
        pub fn Int64Mul(&self) -> Operator {
            Operator::MachineInt64Mul {}
        }
        pub fn Int64MulHigh(&self) -> Operator {
            Operator::MachineInt64MulHigh {}
        }
        pub fn Uint64MulHigh(&self) -> Operator {
            Operator::MachineUint64MulHigh {}
        }
        pub fn Int64MulWithOverflow(&self) -> Operator {
            Operator::MachineInt64MulWithOverflow {}
        }
        pub fn Int64Div(&self) -> Operator {
            Operator::MachineInt64Div {}
        }
        pub fn Int64Mod(&self) -> Operator {
            Operator::MachineInt64Mod {}
        }
        pub fn Int64LessThan(&self) -> Operator {
            Operator::MachineInt64LessThan {}
        }
        pub fn Int64LessThanOrEqual(&self) -> Operator {
            Operator::MachineInt64LessThanOrEqual {}
        }
        pub fn Uint64LessThan(&self) -> Operator {
            Operator::MachineUint64LessThan {}
        }
        pub fn Uint64LessThanOrEqual(&self) -> Operator {
            Operator::MachineUint64LessThanOrEqual {}
        }
        pub fn Uint64Div(&self) -> Operator {
            Operator::MachineUint64Div {}
        }
        pub fn Uint64Mod(&self) -> Operator {
            Operator::MachineUint64Mod {}
        }

        pub fn Int32PairAdd(&self) -> Operator {
            Operator::MachineInt32PairAdd {}
        }
        pub fn Int32PairSub(&self) -> Operator {
            Operator::MachineInt32PairSub {}
        }
        pub fn Int32PairMul(&self) -> Operator {
            Operator::MachineInt32PairMul {}
        }
        pub fn Word32PairShl(&self) -> Operator {
            Operator::MachineWord32PairShl {}
        }
        pub fn Word32PairShr(&self) -> Operator {
            Operator::MachineWord32PairShr {}
        }
        pub fn Word32PairSar(&self) -> Operator {
            Operator::MachineWord32PairSar {}
        }

        pub fn Word32Popcnt(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord32Popcnt {},
            }
        }
        pub fn Word64Popcnt(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord64Popcnt {},
            }
        }
        pub fn Word32Ctz(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord32Ctz {},
            }
        }
        pub fn Word64Ctz(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord64Ctz {},
            }
        }
        pub fn Word32Select(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord32Select {},
            }
        }
        pub fn Word64Select(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineWord64Select {},
            }
        }

        pub fn StackPointerGreaterThan(&self, kind: StackCheckKind) -> Operator {
            Operator::MachineStackPointerGreaterThan { kind }
        }
        pub fn Int32AbsWithOverflow(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineInt32AbsWithOverflow {},
            }
        }
        pub fn Int64AbsWithOverflow(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineInt64AbsWithOverflow {},
            }
        }

        pub fn Float32Add(&self) -> Operator {
            Operator::MachineFloat32Add {}
        }
        pub fn Float32Sub(&self) -> Operator {
            Operator::MachineFloat32Sub {}
        }
        pub fn Float32Mul(&self) -> Operator {
            Operator::MachineFloat32Mul {}
        }
        pub fn Float32Div(&self) -> Operator {
            Operator::MachineFloat32Div {}
        }
        pub fn Float32Abs(&self) -> Operator {
            Operator::MachineFloat32Abs {}
        }
        pub fn Float32Neg(&self) -> Operator {
            Operator::MachineFloat32Neg {}
        }
        pub fn Float32Sqrt(&self) -> Operator {
            Operator::MachineFloat32Sqrt {}
        }
        pub fn Float32Equal(&self) -> Operator {
            Operator::MachineFloat32Equal {}
        }
        pub fn Float32LessThan(&self) -> Operator {
            Operator::MachineFloat32LessThan {}
        }
        pub fn Float32LessThanOrEqual(&self) -> Operator {
            Operator::MachineFloat32LessThanOrEqual {}
        }
        pub fn Float32Max(&self) -> Operator {
            Operator::MachineFloat32Max {}
        }
        pub fn Float32Min(&self) -> Operator {
            Operator::MachineFloat32Min {}
        }

        pub fn Float64Add(&self) -> Operator {
            Operator::MachineFloat64Add {}
        }
        pub fn Float64Sub(&self) -> Operator {
            Operator::MachineFloat64Sub {}
        }
        pub fn Float64Mul(&self) -> Operator {
            Operator::MachineFloat64Mul {}
        }
        pub fn Float64Div(&self) -> Operator {
            Operator::MachineFloat64Div {}
        }
        pub fn Float64Mod(&self) -> Operator {
            Operator::MachineFloat64Mod {}
        }
        pub fn Float64Max(&self) -> Operator {
            Operator::MachineFloat64Max {}
        }
        pub fn Float64Min(&self) -> Operator {
            Operator::MachineFloat64Min {}
        }
        pub fn Float64Abs(&self) -> Operator {
            Operator::MachineFloat64Abs {}
        }
        pub fn Float64Neg(&self) -> Operator {
            Operator::MachineFloat64Neg {}
        }
        pub fn Float64Acos(&self) -> Operator {
            Operator::MachineFloat64Acos {}
        }
        pub fn Float64Acosh(&self) -> Operator {
            Operator::MachineFloat64Acosh {}
        }
        pub fn Float64Asin(&self) -> Operator {
            Operator::MachineFloat64Asin {}
        }
        pub fn Float64Asinh(&self) -> Operator {
            Operator::MachineFloat64Asinh {}
        }
        pub fn Float64Atan(&self) -> Operator {
            Operator::MachineFloat64Atan {}
        }
        pub fn Float64Atanh(&self) -> Operator {
            Operator::MachineFloat64Atanh {}
        }
        pub fn Float64Atan2(&self) -> Operator {
            Operator::MachineFloat64Atan2 {}
        }
        pub fn Float64Cbrt(&self) -> Operator {
            Operator::MachineFloat64Cbrt {}
        }
        pub fn Float64Cos(&self) -> Operator {
            Operator::MachineFloat64Cos {}
        }
        pub fn Float64Cosh(&self) -> Operator {
            Operator::MachineFloat64Cosh {}
        }
        pub fn Float64Exp(&self) -> Operator {
            Operator::MachineFloat64Exp {}
        }
        pub fn Float64Expm1(&self) -> Operator {
            Operator::MachineFloat64Expm1 {}
        }
        pub fn Float64Log(&self) -> Operator {
            Operator::MachineFloat64Log {}
        }
        pub fn Float64Log1p(&self) -> Operator {
            Operator::MachineFloat64Log1p {}
        }
        pub fn Float64Log10(&self) -> Operator {
            Operator::MachineFloat64Log10 {}
        }
        pub fn Float64Log2(&self) -> Operator {
            Operator::MachineFloat64Log2 {}
        }
        pub fn Float64Pow(&self) -> Operator {
            Operator::MachineFloat64Pow {}
        }
        pub fn Float64Sin(&self) -> Operator {
            Operator::MachineFloat64Sin {}
        }
        pub fn Float64Sinh(&self) -> Operator {
            Operator::MachineFloat64Sinh {}
        }
        pub fn Float64Sqrt(&self) -> Operator {
            Operator::MachineFloat64Sqrt {}
        }
        pub fn Float64Tan(&self) -> Operator {
            Operator::MachineFloat64Tan {}
        }
        pub fn Float64Tanh(&self) -> Operator {
            Operator::MachineFloat64Tanh {}
        }
        pub fn Float64Equal(&self) -> Operator {
            Operator::MachineFloat64Equal {}
        }
        pub fn Float64LessThan(&self) -> Operator {
            Operator::MachineFloat64LessThan {}
        }
        pub fn Float64LessThanOrEqual(&self) -> Operator {
            Operator::MachineFloat64LessThanOrEqual {}
        }
        pub fn Float32Select(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat32Select {},
            }
        }
        pub fn Float64Select(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat64Select {},
            }
        }

        pub fn BitcastTaggedToWord(&self) -> Operator {
            Operator::MachineBitcastTaggedToWord {}
        }
        pub fn BitcastTaggedToWordForTagAndSmiBits(&self) -> Operator {
            Operator::MachineBitcastTaggedToWordForTagAndSmiBits {}
        }
        pub fn BitcastMaybeObjectToWord(&self) -> Operator {
            Operator::MachineBitcastMaybeObjectToWord {}
        }
        pub fn BitcastWordToTagged(&self) -> Operator {
            Operator::MachineBitcastWordToTagged {}
        }
        pub fn BitcastWordToTaggedSigned(&self) -> Operator {
            Operator::MachineBitcastWordToTaggedSigned {}
        }
        pub fn TruncateFloat64ToWord32(&self) -> Operator {
            Operator::MachineTruncateFloat64ToWord32 {}
        }
        pub fn ChangeFloat32ToFloat64(&self) -> Operator {
            Operator::MachineChangeFloat32ToFloat64 {}
        }
        pub fn ChangeInt32ToFloat64(&self) -> Operator {
            Operator::MachineChangeInt32ToFloat64 {}
        }
        pub fn ChangeInt64ToFloat64(&self) -> Operator {
            Operator::MachineChangeInt64ToFloat64 {}
        }
        pub fn ChangeUint32ToFloat64(&self) -> Operator {
            Operator::MachineChangeUint32ToFloat64 {}
        }
        pub fn ChangeFloat64ToInt32(&self) -> Operator {
            Operator::MachineChangeFloat64ToInt32 {}
        }
        pub fn ChangeFloat64ToInt64(&self) -> Operator {
            Operator::MachineChangeFloat64ToInt64 {}
        }
        pub fn ChangeFloat64ToUint32(&self) -> Operator {
            Operator::MachineChangeFloat64ToUint32 {}
        }
        pub fn ChangeFloat64ToUint64(&self) -> Operator {
            Operator::MachineChangeFloat64ToUint64 {}
        }
        pub fn TruncateFloat64ToUint32(&self) -> Operator {
            Operator::MachineTruncateFloat64ToUint32 {}
        }
        pub fn TruncateFloat32ToInt32(&self, kind: TruncateKind) -> Operator {
            Operator::MachineTruncateFloat32ToInt32 { kind }
        }
        pub fn TruncateFloat32ToUint32(&self, kind: TruncateKind) -> Operator {
            Operator::MachineTruncateFloat32ToUint32 { kind }
        }
        pub fn TruncateFloat64ToInt64(&self, kind: TruncateKind) -> Operator {
            Operator::MachineTruncateFloat64ToInt64 { kind }
        }
        pub fn TryTruncateFloat32ToInt64(&self) -> Operator {
            Operator::MachineTryTruncateFloat32ToInt64 {}
        }
        pub fn TryTruncateFloat64ToInt64(&self) -> Operator {
            Operator::MachineTryTruncateFloat64ToInt64 {}
        }
        pub fn TryTruncateFloat32ToUint64(&self) -> Operator {
            Operator::MachineTryTruncateFloat32ToUint64 {}
        }
        pub fn TryTruncateFloat64ToUint64(&self) -> Operator {
            Operator::MachineTryTruncateFloat64ToUint64 {}
        }
        pub fn TryTruncateFloat64ToInt32(&self) -> Operator {
            Operator::MachineTryTruncateFloat64ToInt32 {}
        }
        pub fn TryTruncateFloat64ToUint32(&self) -> Operator {
            Operator::MachineTryTruncateFloat64ToUint32 {}
        }
        pub fn ChangeInt32ToInt64(&self) -> Operator {
            Operator::MachineChangeInt32ToInt64 {}
        }
        pub fn ChangeUint32ToUint64(&self) -> Operator {
            Operator::MachineChangeUint32ToUint64 {}
        }
        pub fn TruncateFloat64ToFloat32(&self) -> Operator {
            Operator::MachineTruncateFloat64ToFloat32 {}
        }
        pub fn TruncateFloat64ToFloat16RawBits(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineTruncateFloat64ToFloat16RawBits {},
            }
        }
        pub fn TruncateInt64ToInt32(&self) -> Operator {
            Operator::MachineTruncateInt64ToInt32 {}
        }
        pub fn RoundFloat64ToInt32(&self) -> Operator {
            Operator::MachineRoundFloat64ToInt32 {}
        }
        pub fn RoundInt32ToFloat32(&self) -> Operator {
            Operator::MachineRoundInt32ToFloat32 {}
        }
        pub fn RoundInt64ToFloat32(&self) -> Operator {
            Operator::MachineRoundInt64ToFloat32 {}
        }
        pub fn RoundInt64ToFloat64(&self) -> Operator {
            Operator::MachineRoundInt64ToFloat64 {}
        }
        pub fn RoundUint32ToFloat32(&self) -> Operator {
            Operator::MachineRoundUint32ToFloat32 {}
        }
        pub fn RoundUint64ToFloat32(&self) -> Operator {
            Operator::MachineRoundUint64ToFloat32 {}
        }
        pub fn RoundUint64ToFloat64(&self) -> Operator {
            Operator::MachineRoundUint64ToFloat64 {}
        }
        pub fn BitcastFloat32ToInt32(&self) -> Operator {
            Operator::MachineBitcastFloat32ToInt32 {}
        }
        pub fn BitcastFloat64ToInt64(&self) -> Operator {
            Operator::MachineBitcastFloat64ToInt64 {}
        }
        pub fn BitcastInt32ToFloat32(&self) -> Operator {
            Operator::MachineBitcastInt32ToFloat32 {}
        }
        pub fn BitcastInt64ToFloat64(&self) -> Operator {
            Operator::MachineBitcastInt64ToFloat64 {}
        }
        pub fn Float32RoundDown(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat32RoundDown {},
            }
        }
        pub fn Float64RoundDown(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat64RoundDown {},
            }
        }
        pub fn Float32RoundUp(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat32RoundUp {},
            }
        }
        pub fn Float64RoundUp(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat64RoundUp {},
            }
        }
        pub fn Float32RoundTruncate(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat32RoundTruncate {},
            }
        }
        pub fn Float64RoundTruncate(&self) -> MachineOp {
            MachineOp {
                op: Operator::MachineFloat64RoundTruncate {},
            }
        }
        