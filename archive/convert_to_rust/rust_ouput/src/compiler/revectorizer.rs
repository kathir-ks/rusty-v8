// Converted from V8 C++ source files:
// Header: revectorizer.h
// Implementation: revectorizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct CPU {}
}

pub mod compiler {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    pub struct V8_EXPORT_PRIVATE {}

    pub struct Node {
        id: i32,
        opcode: IrOpcode,
        inputs: Vec<Rc<RefCell<Node>>>,
        uses: Vec<Rc<RefCell<Node>>>,
        op: Operator,
    }

    impl Node {
        pub fn id(&self) -> i32 {
            self.id
        }
        pub fn op(&self) -> &Operator {
            &self.op
        }
        pub fn inputs(&self) -> &Vec<Rc<RefCell<Node>>>> {
            &self.inputs
        }
        pub fn uses(&self) -> &Vec<Rc<RefCell<Node>>>> {
            &self.uses
        }
        pub fn input_at(&self, index: usize) -> &Rc<RefCell<Node>> {
            &self.inputs[index]
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum IrOpcode {
        Invalid,
        Parameter,
        I32Add,
        I64Add,
        Int64Constant,
        ChangeUint32ToUint64,
        LoadFromObject,
        Load,
        ProtectedLoad,
        Store,
        ProtectedStore,
        Phi,
        LoopExitValue,
        S128Zero,
        S128Const,
        ExtractF128,
        I8x16Shuffle,
        F64x2Add,
        F32x4Add,
        I64x2Add,
        I32x4Add,
        I16x8Add,
        I8x16Add,
        F64x2Sub,
        F32x4Sub,
        I64x2Sub,
        I32x4Sub,
        I16x8Sub,
        I8x16Sub,
        F64x2Mul,
        F32x4Mul,
        I64x2Mul,
        I32x4Mul,
        I16x8Mul,
        F64x2Div,
        F32x4Div,
        I16x8AddSatS,
        I16x8SubSatS,
        I16x8AddSatU,
        I16x8SubSatU,
        I8x16AddSatS,
        I8x16SubSatS,
        I8x16AddSatU,
        I8x16SubSatU,
        F64x2Eq,
        F32x4Eq,
        I64x2Eq,
        I32x4Eq,
        I16x8Eq,
        I8x16Eq,
        F64x2Ne,
        F32x4Ne,
        I64x2GtS,
        I32x4GtS,
        I16x8GtS,
        I8x16GtS,
        F64x2Lt,
        F32x4Lt,
        F64x2Le,
        F32x4Le,
        I32x4MinS,
        I16x8MinS,
        I8x16MinS,
        I32x4MinU,
        I16x8MinU,
        I8x16MinU,
        I32x4MaxS,
        I16x8MaxS,
        I8x16MaxS,
        I32x4MaxU,
        I16x8MaxU,
        I8x16MaxU,
        F32x4Abs,
        I32x4Abs,
        I16x8Abs,
        I8x16Abs,
        F32x4Neg,
        I32x4Neg,
        I16x8Neg,
        I8x16Neg,
        F64x2Sqrt,
        F32x4Sqrt,
        F64x2Min,
        F32x4Min,
        F64x2Max,
        F32x4Max,
        I64x2Ne,
        I32x4Ne,
        I16x8Ne,
        I8x16Ne,
        I32x4GtU,
        I16x8GtU,
        I8x16GtU,
        I64x2GeS,
        I32x4GeS,
        I16x8GeS,
        I8x16GeS,
        I32x4GeU,
        I16x8GeU,
        I8x16GeU,
        F32x4Pmin,
        F32x4Pmax,
        F64x2Pmin,
        F64x2Pmax,
        F32x4SConvertI32x4,
        F32x4UConvertI32x4,
        I32x4UConvertF32x4,
        I32x4SConvertF32x4,
        S128And,
        S128Or,
        S128Xor,
        S128Not,
        S128Select,
        S128AndNot,
        I64x2Shl,
        I32x4Shl,
        I16x8Shl,
        I32x4ShrS,
        I16x8ShrS,
        I64x2ShrU,
        I32x4ShrU,
        I16x8ShrU,
        I64x2SConvertI32x4Low,
        I64x2SConvertI32x4High,
        I32x4SConvertI16x8Low,
        I32x4SConvertI16x8High,
        I16x8SConvertI8x16Low,
        I16x8SConvertI8x16High,
        I64x2UConvertI32x4Low,
        I64x2UConvertI32x4High,
        I32x4UConvertI16x8Low,
        I32x4UConvertI16x8High,
        I16x8UConvertI8x16Low,
        I16x8UConvertI8x16High,
        I8x16Splat,
        I16x8Splat,
        I32x4Splat,
        I64x2Splat,
        Dead,
        // Add more opcodes as needed
        F32x8Add,
        F64x4Add,
        F32x8Sub,
        F64x4Sub,
        F32x8Mul,
        F64x4Mul,
        F32x8Div,
        F64x4Div,
        I16x16AddSatS,
        I16x16SubSatS,
        I16x16AddSatU,
        I16x16SubSatU,
        I8x32AddSatS,
        I8x32SubSatS,
        I8x32AddSatU,
        I8x32SubSatU,
        F64x4Eq,
        F32x8Eq,
        I64x4Eq,
        I32x8Eq,
        I16x16Eq,
        I8x32Eq,
        F64x4Ne,
        F32x8Ne,
        I64x4GtS,
        I32x8GtS,
        I16x16GtS,
        I8x32GtS,
        F64x4Lt,
        F32x8Lt,
        F64x4Le,
        F32x8Le,
        I32x8MinS,
        I16x16MinS,
        I8x32MinS,
        I32x8MinU,
        I16x16MinU,
        I8x32MinU,
        I32x8MaxS,
        I16x16MaxS,
        I8x32MaxS,
        I32x8MaxU,
        I16x16MaxU,
        I8x32MaxU,
        F32x8Abs,
        I32x8Abs,
        I16x16Abs,
        I8x32Abs,
        F32x8Neg,
        I32x8Neg,
        I16x16Neg,
        I8x32Neg,
        F64x4Sqrt,
        F32x8Sqrt,
        F64x4Min,
        F32x8Min,
        F64x4Max,
        F32x8Max,
        I64x4Ne,
        I32x8Ne,
        I16x16Ne,
        I8x32Ne,
        I32x8GtU,
        I16x16GtU,
        I8x32GtU,
        I64x4GeS,
        I32x8GeS,
        I16x16GeS,
        I8x32GeS,
        I32x8GeU,
        I16x16GeU,
        I8x32GeU,
        F32x8Pmin,
        F32x8Pmax,
        F64x4Pmin,
        F64x4Pmax,
        F32x8SConvertI32x8,
        F32x8UConvertI32x8,
        I32x8UConvertF32x8,
        I32x8SConvertF32x8,
        S256And,
        S256Or,
        S256Xor,
        S256Not,
        S256Select,
        S256AndNot,
        I64x4Shl,
        I32x8Shl,
        I16x16Shl,
        I32x8ShrS,
        I16x16ShrS,
        I64x4ShrU,
        I32x8ShrU,
        I16x16ShrU,
        I64x4SConvertI32x4,
        I64x4UConvertI32x4,
        I32x8SConvertI16x8,
        I32x8UConvertI16x8,
        I16x16SConvertI8x16,
        I16x16UConvertI8x16,
        I8x32Splat,
        I16x16Splat,
        I32x8Splat,
        I64x4Splat,
        S256Zero,
        S256Const,
        LoadTransform,
    }

    #[derive(Debug, Clone)]
    pub struct Operator {
        opcode: IrOpcode,
        properties: i32,
    }

    impl Operator {
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
        pub fn properties(&self) -> i32 {
            self.properties
        }
    }

    pub struct TFGraph {
        simd_store_nodes: Vec<*mut Node>,
    }

    impl TFGraph {
        pub fn new() -> Self {
            TFGraph {
                simd_store_nodes: Vec::new(),
            }
        }
        pub fn GetSimdStoreNodes(&self) -> &Vec<*mut Node> {
            &self.simd_store_nodes
        }
        pub fn NewNode(&mut self, op: Operator, input0: Rc<RefCell<Node>>, input1: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
            let mut inputs = Vec::new();
            inputs.push(input0);
            inputs.push(input1);
            Rc::new(RefCell::new(Node {
                id: 0,
                opcode: op.opcode(),
                inputs: inputs,
                uses: Vec::new(),
                op: op,
            }))
        }
    }

    pub struct MachineGraph {
        dead: Rc<RefCell<Node>>,
        common: CommonOperatorBuilder,
        machine: MachineOperatorBuilder,
    }

    impl MachineGraph {
        pub fn Dead(&self) -> Rc<RefCell<Node>> {
            self.dead.clone()
        }
        pub fn common(&self) -> &CommonOperatorBuilder {
            &self.common
        }
        pub fn machine(&self) -> &MachineOperatorBuilder {
            &self.machine
        }
        pub fn Int64Constant(&self, value: i64) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                id: 0,
                opcode: IrOpcode::Int64Constant,
                inputs: Vec::new(),
                uses: Vec::new(),
                op: Operator {
                    opcode: IrOpcode::Int64Constant,
                    properties: 0,
                },
            }))
        }
    }

    pub struct CommonOperatorBuilder {}

    impl CommonOperatorBuilder {
        pub fn Phi(&self, rep: MachineRepresentation, count: i32) -> Operator {
            Operator {
                opcode: IrOpcode::Phi,
                properties: 0,
            }
        }
        pub fn LoopExitValue(&self, rep: MachineRepresentation) -> Operator {
            Operator {
                opcode: IrOpcode::LoopExitValue,
                properties: 0,
            }
        }
    }

    pub struct MachineOperatorBuilder {}

    impl MachineOperatorBuilder {
        pub fn I8x16Shuffle(&self, shuffle: &[u8]) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Shuffle,
                properties: 0,
            }
        }
        pub fn S256Zero(&self) -> Operator {
            Operator {
                opcode: IrOpcode::S256Zero,
                properties: 0,
            }
        }
        pub fn S256Const(&self, value: &[u8]) -> Operator {
            Operator {
                opcode: IrOpcode::S256Const,
                properties: 0,
            }
        }
        pub fn ProtectedLoad(&self, mt: MachineType) -> Operator {
            Operator {
                opcode: IrOpcode::ProtectedLoad,
                properties: 0,
            }
        }
        pub fn Load(&self, mt: MachineType) -> Operator {
            Operator {
                opcode: IrOpcode::Load,
                properties: 0,
            }
        }
        pub fn ProtectedStore(&self, mr: MachineRepresentation) -> Operator {
            Operator {
                opcode: IrOpcode::ProtectedStore,
                properties: 0,
            }
        }
        pub fn Store(&self, sr: StoreRepresentation) -> Operator {
            Operator {
                opcode: IrOpcode::Store,
                properties: 0,
            }
        }
        pub fn LoadTransform(&self, kind: MemoryAccessKind, transformation: LoadTransformation) -> Operator {
            Operator {
                opcode: IrOpcode::LoadTransform,
                properties: 0,
            }
        }
        pub fn ExtractF128(&self, lane: i32) -> Operator {
            Operator {
                opcode: IrOpcode::ExtractF128,
                properties: 0,
            }
        }
        pub fn F64x2Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Add,
                properties: 0,
            }
        }
        pub fn F32x4Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Add,
                properties: 0,
            }
        }
        pub fn I64x2Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2Add,
                properties: 0,
            }
        }
        pub fn I32x4Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Add,
                properties: 0,
            }
        }
        pub fn I16x8Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Add,
                properties: 0,
            }
        }
        pub fn I8x16Add(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Add,
                properties: 0,
            }
        }
        pub fn F64x2Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Sub,
                properties: 0,
            }
        }
        pub fn F32x4Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Sub,
                properties: 0,
            }
        }
        pub fn I64x2Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2Sub,
                properties: 0,
            }
        }
        pub fn I32x4Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Sub,
                properties: 0,
            }
        }
        pub fn I16x8Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Sub,
                properties: 0,
            }
        }
        pub fn I8x16Sub(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Sub,
                properties: 0,
            }
        }
        pub fn F64x2Mul(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Mul,
                properties: 0,
            }
        }
        pub fn F32x4Mul(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Mul,
                properties: 0,
            }
        }
        pub fn I64x2Mul(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2Mul,
                properties: 0,
            }
        }
        pub fn I32x4Mul(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Mul,
                properties: 0,
            }
        }
        pub fn I16x8Mul(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Mul,
                properties: 0,
            }
        }
        pub fn F64x2Div(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Div,
                properties: 0,
            }
        }
        pub fn F32x4Div(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Div,
                properties: 0,
            }
        }
        pub fn I16x8AddSatS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8AddSatS,
                properties: 0,
            }
        }
        pub fn I16x8SubSatS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8SubSatS,
                properties: 0,
            }
        }
        pub fn I16x8AddSatU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8AddSatU,
                properties: 0,
            }
        }
        pub fn I16x8SubSatU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8SubSatU,
                properties: 0,
            }
        }
        pub fn I8x16AddSatS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16AddSatS,
                properties: 0,
            }
        }
        pub fn I8x16SubSatS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16SubSatS,
                properties: 0,
            }
        }
        pub fn I8x16AddSatU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16AddSatU,
                properties: 0,
            }
        }
        pub fn I8x16SubSatU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16SubSatU,
                properties: 0,
            }
        }
        pub fn F64x2Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Eq,
                properties: 0,
            }
        }
        pub fn F32x4Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Eq,
                properties: 0,
            }
        }
        pub fn I64x2Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2Eq,
                properties: 0,
            }
        }
        pub fn I32x4Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Eq,
                properties: 0,
            }
        }
        pub fn I16x8Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Eq,
                properties: 0,
            }
        }
        pub fn I8x16Eq(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Eq,
                properties: 0,
            }
        }
        pub fn F64x2Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Ne,
                properties: 0,
            }
        }
        pub fn F32x4Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Ne,
                properties: 0,
            }
        }
        pub fn I64x2GtS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2GtS,
                properties: 0,
            }
        }
        pub fn I32x4GtS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4GtS,
                properties: 0,
            }
        }
        pub fn I16x8GtS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8GtS,
                properties: 0,
            }
        }
        pub fn I8x16GtS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16GtS,
                properties: 0,
            }
        }
        pub fn F64x2Lt(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Lt,
                properties: 0,
            }
        }
        pub fn F32x4Lt(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Lt,
                properties: 0,
            }
        }
        pub fn F64x2Le(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Le,
                properties: 0,
            }
        }
        pub fn F32x4Le(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Le,
                properties: 0,
            }
        }
        pub fn I32x4MinS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4MinS,
                properties: 0,
            }
        }
        pub fn I16x8MinS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8MinS,
                properties: 0,
            }
        }
        pub fn I8x16MinS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16MinS,
                properties: 0,
            }
        }
        pub fn I32x4MinU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4MinU,
                properties: 0,
            }
        }
        pub fn I16x8MinU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8MinU,
                properties: 0,
            }
        }
        pub fn I8x16MinU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16MinU,
                properties: 0,
            }
        }
        pub fn I32x4MaxS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4MaxS,
                properties: 0,
            }
        }
        pub fn I16x8MaxS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8MaxS,
                properties: 0,
            }
        }
        pub fn I8x16MaxS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16MaxS,
                properties: 0,
            }
        }
        pub fn I32x4MaxU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4MaxU,
                properties: 0,
            }
        }
        pub fn I16x8MaxU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8MaxU,
                properties: 0,
            }
        }
        pub fn I8x16MaxU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16MaxU,
                properties: 0,
            }
        }
        pub fn F32x4Abs(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Abs,
                properties: 0,
            }
        }
        pub fn I32x4Abs(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Abs,
                properties: 0,
            }
        }
        pub fn I16x8Abs(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Abs,
                properties: 0,
            }
        }
        pub fn I8x16Abs(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Abs,
                properties: 0,
            }
        }
        pub fn F32x4Neg(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Neg,
                properties: 0,
            }
        }
        pub fn I32x4Neg(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Neg,
                properties: 0,
            }
        }
        pub fn I16x8Neg(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Neg,
                properties: 0,
            }
        }
        pub fn I8x16Neg(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Neg,
                properties: 0,
            }
        }
        pub fn F64x2Sqrt(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Sqrt,
                properties: 0,
            }
        }
        pub fn F32x4Sqrt(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Sqrt,
                properties: 0,
            }
        }
        pub fn F64x2Min(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Min,
                properties: 0,
            }
        }
        pub fn F32x4Min(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Min,
                properties: 0,
            }
        }
        pub fn F64x2Max(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Max,
                properties: 0,
            }
        }
        pub fn F32x4Max(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Max,
                properties: 0,
            }
        }
        pub fn I64x2Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2Ne,
                properties: 0,
            }
        }
        pub fn I32x4Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4Ne,
                properties: 0,
            }
        }
        pub fn I16x8Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8Ne,
                properties: 0,
            }
        }
        pub fn I8x16Ne(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16Ne,
                properties: 0,
            }
        }
        pub fn I32x4GtU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4GtU,
                properties: 0,
            }
        }
        pub fn I16x8GtU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8GtU,
                properties: 0,
            }
        }
        pub fn I8x16GtU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16GtU,
                properties: 0,
            }
        }
        pub fn I64x2GeS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I64x2GeS,
                properties: 0,
            }
        }
        pub fn I32x4GeS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4GeS,
                properties: 0,
            }
        }
        pub fn I16x8GeS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8GeS,
                properties: 0,
            }
        }
        pub fn I8x16GeS(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16GeS,
                properties: 0,
            }
        }
        pub fn I32x4GeU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I32x4GeU,
                properties: 0,
            }
        }
        pub fn I16x8GeU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I16x8GeU,
                properties: 0,
            }
        }
        pub fn I8x16GeU(&self) -> Operator {
            Operator {
                opcode: IrOpcode::I8x16GeU,
                properties: 0,
            }
        }
        pub fn F32x4Pmin(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Pmin,
                properties: 0,
            }
        }
        pub fn F32x4Pmax(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F32x4Pmax,
                properties: 0,
            }
        }
        pub fn F64x2Pmin(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Pmin,
                properties: 0,
            }
        }
        pub fn F64x2Pmax(&self) -> Operator {
            Operator {
                opcode: IrOpcode::F64x2Pmax,
                properties: 0,
            }
        }
        pub fn F32x4SConvertI32x4(&self) -> Operator {
            
