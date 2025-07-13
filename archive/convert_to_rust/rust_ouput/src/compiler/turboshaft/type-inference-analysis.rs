// Converted from V8 C++ source files:
// Header: type-inference-analysis.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK {
            ($condition:expr) => {
                if !$condition {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_EQ {
            ($left:expr, $right:expr) => {
                if $left != $right {
                    panic!("DCHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_NE {
            ($left:expr, $right:expr) => {
                if $left == $right {
                    panic!("DCHECK_NE failed: {} == {}", stringify!($left), stringify!($right));
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_IMPLIES {
            ($condition:expr, $implication:expr) => {
                if $condition && !$implication {
                    panic!("DCHECK_IMPLIES failed: {} implies {}", stringify!($condition), stringify!($implication));
                }
            };
        }
    }
    pub mod vector {
        use std::ops::{Deref, DerefMut};

        pub struct Vector<T> {
            data: Vec<T>,
        }

        impl<T> Vector<T> {
            pub fn new() -> Self {
                Vector { data: Vec::new() }
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Vector { data: Vec::with_capacity(capacity) }
            }

            pub fn push(&mut self, value: T) {
                self.data.push(value);
            }

            pub fn pop(&mut self) -> Option<T> {
                self.data.pop()
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }

            pub fn clear(&mut self) {
                self.data.clear();
            }

            pub fn resize(&mut self, new_len: usize, value: T)
            where
                T: Clone,
            {
                self.data.resize(new_len, value);
            }

            pub fn insert(&mut self, index: usize, element: T) {
                self.data.insert(index, element);
            }

            pub fn remove(&mut self, index: usize) -> T {
                self.data.remove(index)
            }
        }

        impl<T> Deref for Vector<T> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl<T> DerefMut for Vector<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }

        impl<'a, T> Vector<&'a T> {
            pub fn as_vector_of_const(&self) -> Vector<*const T> {
                let mut result = Vector::with_capacity(self.len());
                for &item in &self.data {
                    result.push(item as *const T);
                }
                result
            }
        }
        
        #[derive(Debug)]
        pub struct VectorOf<'a, T> {
            data: &'a [T],
        }

        impl<'a, T> VectorOf<'a, T> {
            pub fn new(data: &'a [T]) -> Self {
                VectorOf { data }
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }

            pub fn get(&self, index: usize) -> Option<&'a T> {
                self.data.get(index)
            }
        }
        
        impl<'a, T> Deref for VectorOf<'a, T> {
            type Target = &'a [T];

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
    }
}

pub mod compiler {
    pub mod common_operator {
        pub struct Operator {}
    }

    pub mod turboshaft {
        use std::{
            cell::RefCell,
            fmt,
            fmt::Debug,
            mem,
            ops::Range,
            rc::Rc,
            string::String,
            vec,
        };

        use self::{
            base::logging::{DCHECK, DCHECK_IMPLIES},
            representations::Representation,
            sidetable::GrowingOpIndexSidetable,
            snapshot_table::SnapshotTable,
            typer::WordOperationTyper,
            types::{Float32Type, Float64Type, TupleType, Type, Word32Type, Word64Type},
        };

        pub mod assembler {
            pub struct Assembler {}
        }

        pub mod operations {
            use super::{
                representations::Representation,
                types::{Type, Word32, Word64},
                OpIndex,
            };

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Opcode {
                kBranch,
                kDeoptimize,
                kDeoptimizeIf,
                kFrameState,
                kReturn,
                kStore,
                kRetain,
                kUnreachable,
                kSwitch,
                kTuple,
                kStaticAssert,
                kDebugBreak,
                kDebugPrint,
                kGlobalSet,
                kTrapIf,
                kCheckException,
                kCheckTurboshaftTypeOf,
                kComparison,
                kConstant,
                kFloatBinop,
                kOverflowCheckedBinop,
                kProjection,
                kWordBinop,
                kWord32PairBinop,
                kAtomicWord32Pair,
                kPendingLoopPhi,
                kPhi,
                kGoto,
                kLoadRootRegister,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum ConstantOpKind {
                kInteger,
                kFloat64,
                kExternalReference,
                kString,
                kNumber,
                kBigInt,
                kBoolean,
                kUndefined,
                kNull,
            }

            #[derive(Debug, Clone)]
            pub struct ConstantOpStorage {
                pub int_value: i64,
                pub float_value: f64,
                pub external_reference: usize,
                pub string_value: String,
            }

            impl ConstantOpStorage {
                pub fn new() -> Self {
                    ConstantOpStorage {
                        int_value: 0,
                        float_value: 0.0,
                        external_reference: 0,
                        string_value: String::new(),
                    }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum FloatBinopKind {
                kFloatAdd,
                kFloatSub,
                kFloatMul,
                kFloatDiv,
                kFloatMod,
                kFloatPow,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum OverflowCheckedBinopKind {
                kSignedAdd,
                kSignedSub,
                kSignedMul,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum WordBinopKind {
                kWordAnd,
                kWordOr,
                kWordXor,
                kWordShl,
                kWordShr,
                kWordSar,
                kWordAdd,
                kWordSub,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum ComparisonKind {
                kEqual,
                kNotEqual,
                kLessThan,
                kLessThanOrEqual,
                kGreaterThan,
                kGreaterThanOrEqual,
                kUnorderedEqual,
                kUnorderedNotEqual,
                kUnorderedLessThan,
                kUnorderedLessThanOrEqual,
                kUnorderedGreaterThan,
                kUnorderedGreaterThanOrEqual,
            }

            #[derive(Clone)]
            pub struct Operation {
                pub opcode: Opcode,
                pub inputs: Vec<OpIndex>,
                pub outputs_rep: Vec<Representation>,
                // Specific operation data
                pub constant_kind: Option<ConstantOpKind>,
                pub constant_storage: Option<ConstantOpStorage>,
                pub float_binop_kind: Option<FloatBinopKind>,
                pub overflow_checked_binop_kind: Option<OverflowCheckedBinopKind>,
                pub word_binop_kind: Option<WordBinopKind>,
                pub comparison_kind: Option<ComparisonKind>,
                pub type_check_type: Option<Type>,
                pub projection_index: Option<usize>,
                pub goto_destination: Option<*mut Block>, // raw pointer here because of lifetime issues,
                                                          // careful when using this
                pub successful: bool,
            }

            impl Operation {
                pub fn new(opcode: Opcode) -> Self {
                    Operation {
                        opcode,
                        inputs: Vec::new(),
                        outputs_rep: Vec::new(),
                        constant_kind: None,
                        constant_storage: None,
                        float_binop_kind: None,
                        overflow_checked_binop_kind: None,
                        word_binop_kind: None,
                        comparison_kind: None,
                        type_check_type: None,
                        projection_index: None,
                        goto_destination: None,
                        successful: false,
                    }
                }

                pub fn TryCast<T>(&self) -> Option<&T>
                where
                    T: OperationLike,
                {
                    if T::get_opcode() == self.opcode {
                        // SAFETY: We are checking the opcode before casting
                        Some(unsafe { &*(self as *const Operation as *const T) })
                    } else {
                        None
                    }
                }

                pub fn Cast<T>(&self) -> &T
                where
                    T: OperationLike,
                {
                    match self.TryCast::<T>() {
                        Some(val) => val,
                        None => panic!("Failed to cast Operation to {}", std::any::type_name::<T>()),
                    }
                }

                pub fn ToString(&self) -> String {
                    format!("{:?}", self.opcode)
                }
            }

            impl Debug for Operation {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct("Operation")
                        .field("opcode", &self.opcode)
                        .field("inputs", &self.inputs)
                        .field("outputs_rep", &self.outputs_rep)
                        .finish()
                }
            }

            pub trait OperationLike {
                fn get_opcode() -> Opcode;
            }

            #[derive(Clone)]
            pub struct CheckTurboshaftTypeOfOp {
                pub base: Operation,
                pub input: OpIndex,
                pub type_: Type,
                pub successful: bool,
            }

            impl CheckTurboshaftTypeOfOp {
                pub fn new(input: OpIndex, type_: Type, successful: bool) -> Self {
                    let mut base = Operation::new(Opcode::kCheckTurboshaftTypeOf);
                    base.inputs.push(input);
                    CheckTurboshaftTypeOfOp {
                        base,
                        input,
                        type_,
                        successful,
                    }
                }

                pub fn input(&self) -> OpIndex {
                    self.input
                }
            }

            impl OperationLike for CheckTurboshaftTypeOfOp {
                fn get_opcode() -> Opcode {
                    Opcode::kCheckTurboshaftTypeOf
                }
            }

            #[derive(Clone)]
            pub struct ComparisonOp {
                pub base: Operation,
                pub left: OpIndex,
                pub right: OpIndex,
                pub rep: Representation,
                pub kind: ComparisonKind,
            }

            impl ComparisonOp {
                pub fn new(
                    left: OpIndex,
                    right: OpIndex,
                    rep: Representation,
                    kind: ComparisonKind,
                ) -> Self {
                    let mut base = Operation::new(Opcode::kComparison);
                    base.inputs.push(left);
                    base.inputs.push(right);
                    base.outputs_rep.push(Representation::Tagged); // Comparison always returns a boolean
                    ComparisonOp {
                        base,
                        left,
                        right,
                        rep,
                        kind,
                    }
                }

                pub fn left(&self) -> OpIndex {
                    self.left
                }

                pub fn right(&self) -> OpIndex {
                    self.right
                }
            }

            impl OperationLike for ComparisonOp {
                fn get_opcode() -> Opcode {
                    Opcode::kComparison
                }
            }

            #[derive(Clone)]
            pub struct ConstantOp {
                pub base: Operation,
                pub kind: ConstantOpKind,
                pub storage: ConstantOpStorage,
            }

            impl ConstantOp {
                pub fn new(kind: ConstantOpKind, storage: ConstantOpStorage) -> Self {
                    let mut base = Operation::new(Opcode::kConstant);
                    base.outputs_rep.push(Representation::Tagged); // Constants are always tagged
                    ConstantOp { base, kind, storage }
                }

                pub fn integer(&self) -> i64 {
                    self.storage.int_value
                }

                pub fn float64(&self) -> Float64 {
                    Float64 {
                        value: self.storage.float_value,
                    }
                }

                pub fn kind(&self) -> ConstantOpKind {
                    self.kind
                }
            }

            impl OperationLike for ConstantOp {
                fn get_opcode() -> Opcode {
                    Opcode::kConstant
                }
            }

            #[derive(Clone)]
            pub struct FloatBinopOp {
                pub base: Operation,
                pub left: OpIndex,
                pub right: OpIndex,
                pub kind: FloatBinopKind,
                pub rep: Representation,
            }

            impl FloatBinopOp {
                pub fn new(
                    left: OpIndex,
                    right: OpIndex,
                    kind: FloatBinopKind,
                    rep: Representation,
                ) -> Self {
                    let mut base = Operation::new(Opcode::kFloatBinop);
                    base.inputs.push(left);
                    base.inputs.push(right);
                    base.outputs_rep.push(rep);
                    FloatBinopOp {
                        base,
                        left,
                        right,
                        kind,
                        rep,
                    }
                }

                pub fn left(&self) -> OpIndex {
                    self.left
                }

                pub fn right(&self) -> OpIndex {
                    self.right
                }
            }

            impl OperationLike for FloatBinopOp {
                fn get_opcode() -> Opcode {
                    Opcode::kFloatBinop
                }
            }

            #[derive(Clone)]
            pub struct OverflowCheckedBinopOp {
                pub base: Operation,
                pub left: OpIndex,
                pub right: OpIndex,
                pub kind: OverflowCheckedBinopKind,
                pub rep: Representation,
            }

            impl OverflowCheckedBinopOp {
                pub fn new(
                    left: OpIndex,
                    right: OpIndex,
                    kind: OverflowCheckedBinopKind,
                    rep: Representation,
                ) -> Self {
                    let mut base = Operation::new(Opcode::kOverflowCheckedBinop);
                    base.inputs.push(left);
                    base.inputs.push(right);
                    base.outputs_rep.push(rep);
                    OverflowCheckedBinopOp {
                        base,
                        left,
                        right,
                        kind,
                        rep,
                    }
                }

                pub fn left(&self) -> OpIndex {
                    self.left
                }

                pub fn right(&self) -> OpIndex {
                    self.right
                }
            }

            impl OperationLike for OverflowCheckedBinopOp {
                fn get_opcode() -> Opcode {
                    Opcode::kOverflowCheckedBinop
                }
            }

            #[derive(Clone)]
            pub struct ProjectionOp {
                pub base: Operation,
                pub input: OpIndex,
                pub index: usize,
                pub rep: Representation,
            }

            impl ProjectionOp {
                pub fn new(input: OpIndex, index: usize, rep: Representation) -> Self {
                    let mut base = Operation::new(Opcode::kProjection);
                    base.inputs.push(input);
                    base.outputs_rep.push(rep);
                    ProjectionOp {
                        base,
                        input,
                        index,
                        rep,
                        projection_index: Some(index),
                    }
                }

                pub fn input(&self) -> OpIndex {
                    self.input
                }
            }

            impl OperationLike for ProjectionOp {
                fn get_opcode() -> Opcode {
                    Opcode::kProjection
                }
            }

            #[derive(Clone)]
            pub struct WordBinopOp {
                pub base: Operation,
                pub left: OpIndex,
                pub right: OpIndex,
                pub kind: WordBinopKind,
                pub rep: Representation,
            }

            impl WordBinopOp {
                pub fn new(
                    left: OpIndex,
                    right: OpIndex,
                    kind: WordBinopKind,
                    rep: Representation,
                ) -> Self {
                    let mut base = Operation::new(Opcode::kWordBinop);
                    base.inputs.push(left);
                    base.inputs.push(right);
                    base.outputs_rep.push(rep);
                    WordBinopOp {
                        base,
                        left,
                        right,
                        kind,
                        rep,
                    }
                }

                pub fn left(&self) -> OpIndex {
                    self.left
                }

                pub fn right(&self) -> OpIndex {
                    self.right
                }
            }

            impl OperationLike for WordBinopOp {
                fn get_opcode() -> Opcode {
                    Opcode::kWordBinop
                }
            }

            #[derive(Clone)]
            pub struct PhiOp {
                pub base: Operation,
                pub inputs: Vec<OpIndex>,
                pub rep: Representation,
            }

            impl PhiOp {
                pub fn new(inputs: Vec<OpIndex>, rep: Representation) -> Self {
                    let mut base = Operation::new(Opcode::kPhi);
                    base.inputs = inputs.clone();
                    base.outputs_rep.push(rep);
                    PhiOp { base, inputs, rep }
                }
                pub fn inputs(&self) -> &Vec<OpIndex> {
                    &self.inputs
                }
            }

            impl OperationLike for PhiOp {
                fn get_opcode() -> Opcode {
                    Opcode::kPhi
                }
            }

            #[derive(Clone)]
            pub struct GotoOp {
                pub base: Operation,
                pub destination: *mut Block, // raw pointer here because of lifetime issues,
                                              // careful when using this
            }

            impl GotoOp {
                pub fn new(destination: *mut Block) -> Self {
                    let mut base = Operation::new(Opcode::kGoto);
                    GotoOp {
                        base,
                        destination,
                    }
                }
            }

            impl OperationLike for GotoOp {
                fn get_opcode() -> Opcode {
                    Opcode::kGoto
                }
            }
        }

        pub mod representations {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Representation {
                Tagged,
                Word32,
                Word64,
                Float32,
                Float64,
                Simd128,
                None,
            }
        }

        pub mod sidetable {
            use std::vec;

            #[derive(Debug, Clone)]
            pub struct GrowingOpIndexSidetable<T: Clone> {
                data: Vec<T>,
            }

            impl<T: Clone> GrowingOpIndexSidetable<T> {
                pub fn new(size: usize, default_value: T) -> Self {
                    GrowingOpIndexSidetable {
                        data: vec![default_value; size],
                    }
                }

                pub fn with_default(size: usize, default_value: T) -> Self {
                    GrowingOpIndexSidetable {
                        data: vec![default_value; size],
                    }
                }

                pub fn resize(&mut self, new_size: usize, default_value: T) {
                    self.data.resize(new_size, default_value);
                }

                pub fn len(&self) -> usize {
                    self.data.len()
                }

                pub fn get(&self, index: OpIndex) -> &T {
                    &self.data[index.id()]
                }

                pub fn get_mut(&mut self, index: OpIndex) -> &mut T {
                    &mut self.data[index.id()]
                }

                pub fn set(&mut self, index: OpIndex, value: T) {
                    self.data[index.id()] = value;
                }
            }

            #[derive(Debug, Clone)]
            pub struct GrowingBlockSidetable<T: Clone> {
                data: Vec<T>,
            }

            impl<T: Clone> GrowingBlockSidetable<T> {
                pub fn new(size: usize, default_value: T) -> Self {
                    GrowingBlockSidetable {
                        data: vec![default_value; size],
                    }
                }

                pub fn with_default(size: usize, default_value: T) -> Self {
                    GrowingBlockSidetable {
                        data: vec![default_value; size],
                    }
                }

                pub fn resize(&mut self, new_size: usize, default_value: T) {
                    self.data.resize(new_size, default_value);
                }

                pub fn len(&self) -> usize {
                    self.data.len()
                }

                pub fn get(&self, index: BlockIndex) -> &T {
                    &self.data[index.id()]
                }

                pub fn get_mut(&mut self, index: BlockIndex) -> &mut T {
                    &mut self.data[index.id()]
                }

                pub fn set(&mut self, index: BlockIndex, value: T) {
                    self.data[index.id()] = value;
                }
            }
        }

        pub mod snapshot_table {
            use std::{
                cell::{Ref, RefCell, RefMut},
                collections::HashMap,
                rc::Rc,
            };

            use super::base::vector::{Vector, VectorOf};
            use super::types::Type;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct Key(usize);

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct Snapshot(usize);

            #[derive(Debug, Clone)]
            pub struct SnapshotTable<T: Clone> {
                table: Rc<RefCell<TableData<T>>>,
                phase_zone: *mut Zone,
            }

            impl<T: Clone> SnapshotTable<T> {
                pub fn new(phase_zone: *mut Zone) -> Self {
                    SnapshotTable {
                        table: Rc::new(RefCell::new(TableData::new())),
                        phase_zone,
                    }
                }

                pub fn StartNewSnapshot(
                    &mut self,
                    predecessors: VectorOf<Snapshot>,
                    merge_types: impl Fn(Key, VectorOf<Type>) -> T,
                ) {
                    self.table.borrow_mut().start_new_snapshot(
                        predecessors,
                        &merge_types,
                    );
                }

                pub fn NewKey(&mut self, initial_value: T) -> Key {
                    self.table.borrow_mut().new_key(initial_value)
                }

                pub fn Set(&mut self, key: Key, value: T) {
                    self.table.borrow_mut().set(key, value);
                }

                pub fn Get(&self, key: Key) -> T {
                    self.table.borrow().get(key)
                }

                pub fn Seal(&mut self) -> Option<Snapshot> {
                    self.table.borrow_mut().seal()
                }

                pub fn IsSealed(&self) -> bool {
                    self.table.borrow().is_sealed()
                }
            }

            #[derive(Debug, Clone)]
            struct TableData<T: Clone> {
                keys: Vec<KeyData<T>>,
                snapshots: Vec<SnapshotData<T>>,
                current_snapshot: Option<Snapshot>,
                sealed: bool,
            }

            impl<T: Clone> TableData<T> {
                fn new() -> Self {
                    TableData {
                        keys: Vec::new(),
                        snapshots: Vec::new(),
                        current_snapshot: None,
                        sealed: false,
                    }
                }

                fn start_new_snapshot(
                    &mut self,
                    predecessors: VectorOf<Snapshot>,
                    merge_types: &impl Fn(Key, VectorOf<Type>) -> T,
                ) {
                    if self.sealed {
                        panic!("Cannot start a new snapshot after sealing the table");
                    }

                    let snapshot_index = self.snapshots.len();
                    let mut snapshot_data = SnapshotData {
                        values: HashMap::new(),
                    };

                    for key_data in &self.keys {
                        let key = key_data.key;
                        let mut predecessor_values = Vector::new();

                        for predecessor in predecessors.iter() {
                            let snapshot = &self.snapshots[predecessor.0];
                            match snapshot.values.get(&key) {
                                Some(value) => predecessor_values.push(value.clone()),
                                None => {
                                    // Handle the case where a key is not present in a predecessor
                                    // This could involve using a default value or skipping the predecessor
                                    // For now, skip the predecessor
                                }
                            }
                        }
                        
                        let mut type_vec = Vec::new();
                        for val in &predecessor_values {
                            type_vec.push(val.clone());
                        }
                        let vector_of_types = VectorOf::new(&type_vec);
                        
                        let merged_value = merge_types(key, vector_of_types);
                        snapshot_data.values.insert(key, merged_value);
                    }

                    self.snapshots.push(snapshot_data);
                    self.current_snapshot = Some(Snapshot(snapshot_index));
                }

                fn new_key(&mut self, initial_value: T) -> Key {
                    let key_index = self.keys.len();
                    let key = Key(key_index);
                    self.keys.push(KeyData {
                        key,
                        initial_value: initial_value.clone(),
                    });

                    if let Some(current_snapshot) = &mut self.current_snapshot {
                        let snapshot = &mut self.snapshots[current_snapshot.0];
                        snapshot.values.insert(key, initial_value);
                    }

                    key
                }

                fn set(&mut self, key: Key, value: T) {
                    if let Some(current_snapshot) = &mut self.current_snapshot {
                        let snapshot = &mut self.snapshots[current_snapshot.0];
                        snapshot.values.insert(key, value);
                    } else {
                        panic!("Cannot set a value without a current snapshot");
                    }
                }

                fn get(&self, key: Key) -> T {
                    match &self.current_snapshot {
                        Some(current_snapshot) => {
                            let snapshot = &self.snapshots[current_snapshot.0];
                            match snapshot.values.get(&key) {
                                Some(value) => value.clone(),
                                None => {
                                    // If the key is not found in the snapshot, return the initial value
                                    self.keys[key.0].initial_value.clone()
                                }
                            }
                        }
                        None => {
                            // If there is no current snapshot, return the initial value
                            self.keys[key.0].initial_value.clone()
                        }
                    }
                }

                fn seal(&mut self) -> Option<Snapshot> {
                    self.sealed = true;
                    self.current_snapshot.take()
                }

                fn is_sealed(&self) -> bool {
                    self.sealed
                }
            }

            #[derive(Debug, Clone)]
            struct KeyData<T: Clone> {
                key: Key,
                initial_value: T,
            }

            #[derive(Debug, Clone)]
            struct SnapshotData<T: Clone> {
                values: HashMap<Key, T>,
            }
        }

        pub mod typer {
            use super::{
                operations::{ComparisonKind, FloatBinopKind, OverflowCheckedBinopKind, WordBinopKind},
                representations::Representation,
                types::{Type, Word32Type, Word64Type},
            };

            pub struct Typer {}

            impl Typer {
                pub fn TypeForRepresentation(
                    rep: &Vec<Representation>,
                    zone: *mut Zone,
                ) -> Type {
                    if rep.len() == 0 {
                        return Type::None();
                    }
                    match rep[0] {
                        Representation::Tagged => Type::Any(),
                        Representation::Word32 => Type::Word32(),
                        Representation::Word64 => Type::Word64(),
                        Representation::Float32 => Type::Float32(),
                        Representation::Float64 => Type::Float64(),
                        _ => Type::Any(),
                    }
                }

                pub fn TypeComparison(
                    left_type: Type,
                    right_type: Type,
                    rep: Representation,
                    kind: ComparisonKind,
                    zone: *mut Zone,
                ) -> Type {
                    Type::Boolean()
                }

                pub fn TypeConstant(kind: super::operations::ConstantOpKind, storage: super::operations::ConstantOpStorage) -> Type {
                    match kind {
                        super::operations::ConstantOpKind::kInteger => Type::Number(),
                        super::operations::ConstantOpKind::kFloat64 => Type::Float64(),
                        super::operations::ConstantOpKind::kString => Type::String(),
                        super::operations::ConstantOpKind::kBoolean => Type::Boolean(),
                        _ => Type::Any(),
                    }
                }

                pub fn TypeFloatBinop(
                    left_type: Type,
                    right_type: Type,
                    kind: FloatBinopKind,
                    rep: Representation,
                    zone: *mut Zone,
                ) -> Type {
                    Type::Float64()
                }

                pub fn TypeOverflowCheckedBinop(
                    left_type: Type,
                    right_type: Type,
                    kind: OverflowCheckedBinopKind,
                    rep: Representation,
                    zone: *mut Zone,
                ) -> Type {
                    Type::Number()
                }

                pub fn TypeWordBinop(
                    left_type: Type,
                    right_type: Type,
                    kind: WordBinopKind,
                    rep: Representation,
                    zone: *mut Zone,
                ) -> Type {
                    match rep {
                        Representation::Word32 => Type::Word32(),
                        Representation::Word64 => Type::Word64(),
                        _ => Type::Any(),
                    }
                }

                pub fn TruncateWord32Input(t: Type, b: bool, zone: *mut Zone) -> Type {
                    Type::Word32()
                }
            }
            
            pub struct BranchRefinements<GetTypeFn, RefineTypeFn>
            where
                GetTypeFn: Fn(OpIndex) -> Type,
                RefineTypeFn: Fn(OpIndex, Type),
            {
                get_type: GetTypeFn,
                refine_type: RefineTypeFn,
            }

            impl<GetTypeFn, RefineTypeFn> BranchRefinements<GetTypeFn, RefineTypeFn>
            where
                GetTypeFn: Fn(OpIndex) -> Type,
                RefineTypeFn: Fn(OpIndex, Type),
            {
                pub fn new(get_type: GetTypeFn, refine_type: RefineTypeFn) -> Self {
                    BranchRefinements {
                        get_type,
                        refine_type,
                    }
                }
                
                pub fn RefineTypes(&self, condition: &super::operations::Operation, then_branch: bool, zone: *mut Zone) {
                }
            }
            
            impl<const bits: usize> WordOperationTyper<bits> {
                pub fn WidenMaximal(old_type: Word32Type, new_type: Word32Type, graph_zone: *mut Zone) -> Type {
                    Type::Word32()
                }
                pub fn WidenMaximal64(old_type: Word64Type, new_type: Word64Type, graph_zone: *mut Zone) -> Type {
                    Type::Word64()
                }
            }
        }

        pub mod types {
            use std::fmt;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum Kind {
                Invalid,
                None,
                Any,
                Number,
                Boolean,
                String,
                Symbol,
                BigInt,
                Object,
                Tuple,
                Word32,
                Word64,
                Float32,
                Float64,
            }

            #[derive(Clone)]
            pub struct Type {
                kind: Kind,
                tuple: Option<TupleTypeData>,
                word32: Option<Word32TypeData>,
                word64: Option<Word64TypeData>,
                float32: Option<Float32TypeData>,
                float64: Option<Float64TypeData>,
            }

            impl Type {
                pub fn new(kind: Kind) -> Self {
                    Type {
                        kind,
                        tuple: None,
                        word32: None,
                        word64: None,
                        float32: None,
                        float64: None,
                    }
                }

                pub fn
