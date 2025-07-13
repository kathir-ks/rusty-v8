// Converted from V8 C++ source files:
// Header: common-operator.h
// Implementation: common-operator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod hashing {
    pub fn hash_combine<T: std::hash::Hash>(seed: usize, value: T) -> usize {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut s);
        value.hash(&mut s);
        s.finish() as usize
    }

    pub fn hash_range<T: std::hash::Hash>(start: *const T, end: *const T) -> usize {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        unsafe {
            let mut current = start;
            while current < end {
                (*current).hash(&mut s);
                current = current.offset(1);
            }
        }
        s.finish() as usize
    }
}
pub mod lazy_instance {
    use std::sync::{Once, Mutex, MutexGuard};

    pub struct Lazy<T> {
        instance: Mutex<Option<T>>,
        once: Once,
    }

    impl<T> Lazy<T> {
        pub const fn new() -> Self {
            Lazy {
                instance: Mutex::new(None),
                once: Once::new(),
            }
        }

        pub fn get<F>(&self, init: F) -> MutexGuard<'_, Option<T>>
        where
            F: FnOnce() -> T,
        {
            self.once.call_once(|| {
                let value = init();
                let mut instance = self.instance.lock().unwrap();
                *instance = Some(value);
            });
            self.instance.lock().unwrap()
        }
    }
}
}

pub mod codegen {
pub enum MachineType {
        Int8,
        Uint8,
        Int16,
        Uint16,
        Int32,
        Uint32,
        Int64,
        Uint64,
        Float32,
        Float64,
        TaggedSigned,
        TaggedPointer,
        Any,
    }
pub enum RelocInfo {
    NoInfo,
}

impl RelocInfo {
    pub fn mode(&self) -> Mode {
        Mode::NoInfo
    }
}

pub enum Mode {
    NoInfo,
}
}

pub mod common {
pub mod globals {}
}

pub mod compiler {

use std::fmt;
use std::hash::{Hash, Hasher};
use std::{any::Any, f32, f64, i32, i64, mem, ops::Deref, os::raw::c_char, ptr, rc::Rc, str};

use crate::{
    codegen::MachineType, codegen::RelocInfo, deoptimizer::DeoptimizeReason,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchHint {
    kNone,
    kTrue,
    kFalse,
}

impl fmt::Display for BranchHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchHint::kNone => write!(f, "None"),
            BranchHint::kTrue => write!(f, "True"),
            BranchHint::kFalse => write!(f, "False"),
        }
    }
}

impl Default for BranchHint {
    fn default() -> Self {
        BranchHint::kNone
    }
}

impl std::ops::Not for BranchHint {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BranchHint::kNone => BranchHint::kNone,
            BranchHint::kTrue => BranchHint::kFalse,
            BranchHint::kFalse => BranchHint::kTrue,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchSemantics {
    kJS,
    kMachine,
    kUnspecified,
}

impl fmt::Display for BranchSemantics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchSemantics::kJS => write!(f, "JS"),
            BranchSemantics::kMachine => write!(f, "Machine"),
            BranchSemantics::kUnspecified => write!(f, "Unspecified"),
        }
    }
}

pub fn negate_branch_hint(hint: BranchHint) -> BranchHint {
    match hint {
        BranchHint::kNone => hint,
        BranchHint::kTrue => BranchHint::kFalse,
        BranchHint::kFalse => BranchHint::kTrue,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrapId {
    kTrapUnreachable,
    kTrapMemOutOfBounds,
    kTrapDivByZero,
    kTrapDivUnrepresentable,
    kTrapRemByZero,
    kTrapFloatUnrepresentable,
    kTrapTableOutOfBounds,
    kTrapFuncSigMismatch,
}

impl fmt::Display for TrapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrapId::kTrapUnreachable => write!(f, "TrapUnreachable"),
            TrapId::kTrapMemOutOfBounds => write!(f, "TrapMemOutOfBounds"),
            TrapId::kTrapDivByZero => write!(f, "TrapDivByZero"),
            TrapId::kTrapDivUnrepresentable => write!(f, "TrapDivUnrepresentable"),
            TrapId::kTrapRemByZero => write!(f, "TrapRemByZero"),
            TrapId::kTrapFloatUnrepresentable => write!(f, "TrapFloatUnrepresentable"),
            TrapId::kTrapTableOutOfBounds => write!(f, "TrapTableOutOfBounds"),
            TrapId::kTrapFuncSigMismatch => write!(f, "TrapFuncSigMismatch"),
        }
    }
}

pub fn trap_id_of(op: &Operator) -> TrapId {
    match op.opcode {
        IrOpcode::kTrapIf | IrOpcode::kTrapUnless => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(trap_id) = any.downcast_ref::<TrapId>() {
                    return *trap_id;
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchParameters {
    semantics: BranchSemantics,
    hint: BranchHint,
}

impl BranchParameters {
    pub fn new(semantics: BranchSemantics, hint: BranchHint) -> Self {
        BranchParameters { semantics, hint }
    }

    pub fn semantics(&self) -> BranchSemantics {
        self.semantics
    }
    pub fn hint(&self) -> BranchHint {
        self.hint
    }
}

impl Hash for BranchParameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.semantics.hash(state);
        self.hint.hash(state);
    }
}

impl fmt::Display for BranchParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.semantics, self.hint)
    }
}

pub fn branch_parameters_of(op: &Operator) -> &BranchParameters {
    if op.opcode == IrOpcode::kBranch {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(branch_params) = any.downcast_ref::<BranchParameters>() {
                return branch_params;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn branch_hint_of(op: &Operator) -> BranchHint {
    match op.opcode {
        IrOpcode::kIfValue => if_value_parameters_of(op).hint(),
        IrOpcode::kIfDefault => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(branch_hint) = any.downcast_ref::<BranchHint>() {
                    return *branch_hint;
                }
            }
            panic!("Invalid operator parameter type");
        }
        IrOpcode::kBranch => branch_parameters_of(op).hint(),
        _ => panic!("Invalid operator opcode"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssertParameters {
    semantics: BranchSemantics,
    condition_string: String,
    file: String,
    line: i32,
}

impl AssertParameters {
    pub fn new(
        semantics: BranchSemantics,
        condition_string: String,
        file: String,
        line: i32,
    ) -> Self {
        AssertParameters {
            semantics,
            condition_string,
            file,
            line,
        }
    }

    pub fn semantics(&self) -> BranchSemantics {
        self.semantics
    }
    pub fn condition_string(&self) -> &str {
        &self.condition_string
    }
    pub fn file(&self) -> &str {
        &self.file
    }
    pub fn line(&self) -> i32 {
        self.line
    }
}

impl fmt::Display for AssertParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.semantics, self.condition_string, self.file, self.line
        )
    }
}

pub fn assert_parameters_of(op: &Operator) -> &AssertParameters {
    if op.opcode == IrOpcode::kAssert {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(assert_params) = any.downcast_ref::<AssertParameters>() {
                return assert_params;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn value_input_count_of_return(op: &Operator) -> i32 {
    if op.opcode == IrOpcode::kReturn {
        op.value_input_count - 1
    } else {
        panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeoptimizeParameters {
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
}

impl DeoptimizeParameters {
    pub fn new(reason: DeoptimizeReason, feedback: FeedbackSource) -> Self {
        DeoptimizeParameters { reason, feedback }
    }

    pub fn reason(&self) -> DeoptimizeReason {
        self.reason
    }
    pub fn feedback(&self) -> &FeedbackSource {
        &self.feedback
    }
}

impl fmt::Display for DeoptimizeParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.reason, self.feedback)
    }
}

pub fn deoptimize_parameters_of(op: &Operator) -> &DeoptimizeParameters {
    match op.opcode {
        IrOpcode::kDeoptimize | IrOpcode::kDeoptimizeIf | IrOpcode::kDeoptimizeUnless => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(deopt_params) = any.downcast_ref::<DeoptimizeParameters>() {
                    return deopt_params;
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SelectParameters {
    representation: MachineType,
    hint: BranchHint,
    semantics: BranchSemantics,
}

impl SelectParameters {
    pub fn new(
        representation: MachineType,
        hint: BranchHint,
        semantics: BranchSemantics,
    ) -> Self {
        SelectParameters {
            representation,
            hint,
            semantics,
        }
    }

    pub fn representation(&self) -> MachineType {
        self.representation
    }
    pub fn hint(&self) -> BranchHint {
        self.hint
    }
    pub fn semantics(&self) -> BranchSemantics {
        self.semantics
    }
}

impl fmt::Display for SelectParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.representation, self.hint, self.semantics
        )
    }
}

pub fn select_parameters_of(op: &Operator) -> &SelectParameters {
    if op.opcode == IrOpcode::kSelect {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(select_params) = any.downcast_ref::<SelectParameters>() {
                return select_params;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn call_descriptor_of(op: &Operator) -> &CallDescriptor {
    match op.opcode {
        IrOpcode::kCall | IrOpcode::kTailCall => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(call_desc) = any.downcast_ref::<CallDescriptor>() {
                    return call_desc;
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode"),
    }
}

pub fn projection_index_of(op: &Operator) -> usize {
    if op.opcode == IrOpcode::kProjection {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(index) = any.downcast_ref::<usize>() {
                return *index;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn loop_exit_value_representation_of(op: &Operator) -> MachineType {
    if op.opcode == IrOpcode::kLoopExitValue {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(rep) = any.downcast_ref::<MachineType>() {
                return *rep;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn phi_representation_of(op: &Operator) -> MachineType {
    if op.opcode == IrOpcode::kPhi {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(rep) = any.downcast_ref::<MachineType>() {
                return *rep;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterInfo {
    index: i32,
    debug_name: Option<String>,
}

impl ParameterInfo {
    pub const kMinIndex: i32 = Linkage::kJSCallClosureParamIndex;

    pub fn new(index: i32, debug_name: Option<String>) -> Self {
        ParameterInfo { index, debug_name }
    }

    pub fn index(&self) -> i32 {
        self.index
    }
    pub fn debug_name(&self) -> Option<&String> {
        self.debug_name.as_ref()
    }
}

impl fmt::Display for ParameterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index)?;
        if let Some(name) = &self.debug_name {
            write!(f, ", debug name: {}", name)?;
        }
        Ok(())
    }
}

pub fn parameter_index_of(op: &Operator) -> i32 {
    if op.opcode == IrOpcode::kParameter {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(param_info) = any.downcast_ref::<ParameterInfo>() {
                return param_info.index();
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn parameter_info_of(op: &Operator) -> &ParameterInfo {
    if op.opcode == IrOpcode::kParameter {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(param_info) = any.downcast_ref::<ParameterInfo>() {
                return param_info;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectStateInfo {
    object_id: u32,
    size: i32,
}

impl ObjectStateInfo {
    pub fn new(object_id: u32, size: i32) -> Self {
        ObjectStateInfo { object_id, size }
    }

    pub fn object_id(&self) -> u32 {
        self.object_id
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}

impl fmt::Display for ObjectStateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, size:{}", self.object_id, self.size)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedObjectStateInfo {
    object_id: u32,
    machine_types: Vec<MachineType>,
}

impl TypedObjectStateInfo {
    pub fn new(object_id: u32, machine_types: Vec<MachineType>) -> Self {
        TypedObjectStateInfo {
            object_id,
            machine_types,
        }
    }

    pub fn object_id(&self) -> u32 {
        self.object_id
    }
    pub fn machine_types(&self) -> &Vec<MachineType> {
        &self.machine_types
    }
}

impl fmt::Display for TypedObjectStateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, {:?}", self.object_id, self.machine_types)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelocatablePtrConstantType {
    kInt32,
    kInt64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelocatablePtrConstantInfo {
    value: i64,
    rmode: RelocInfo::Mode,
    constant_type: RelocatablePtrConstantType,
}

impl RelocatablePtrConstantInfo {
    pub fn new(value: i64, rmode: RelocInfo::Mode, constant_type: RelocatablePtrConstantType) -> Self {
        RelocatablePtrConstantInfo {
            value,
            rmode,
            constant_type,
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn rmode(&self) -> RelocInfo::Mode {
        self.rmode
    }
    pub fn constant_type(&self) -> RelocatablePtrConstantType {
        self.constant_type
    }
}

impl fmt::Display for RelocatablePtrConstantInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {:?}, {:?}",
            self.value, self.rmode, self.constant_type
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SparseInputMask {
    bit_mask: u32,
}

impl SparseInputMask {
    pub const kDenseBitMask: u32 = 0x0;
    pub const kEndMarker: u32 = 0x1;
    pub const kEntryMask: u32 = 0x1;
    pub const kMaxSparseInputs: i32 = (std::mem::size_of::<u32>() as i32 * 8) - 1;

    pub fn new(bit_mask: u32) -> Self {
        SparseInputMask { bit_mask }
    }

    pub fn dense() -> Self {
        SparseInputMask {
            bit_mask: SparseInputMask::kDenseBitMask,
        }
    }

    pub fn mask(&self) -> u32 {
        self.bit_mask
    }

    pub fn is_dense(&self) -> bool {
        self.bit_mask == SparseInputMask::kDenseBitMask
    }

    pub fn count_real(&self) -> i32 {
        if !self.is_dense() {
             (self.bit_mask.count_ones() - SparseInputMask::kEndMarker.count_ones()) as i32
        } else {
            0
        }
    }

    //TODO: Implement InputIterator
    //pub fn iterate_over_inputs(&self, node: &Node) -> InputIterator {
    //    InputIterator::new(self.bit_mask, node)
    //}
}

impl fmt::Display for SparseInputMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dense() {
            write!(f, "dense")
        } else {
            write!(f, "sparse:")?;
            let mut mask = self.bit_mask;
            while mask != SparseInputMask::kEndMarker {
                if mask & SparseInputMask::kEntryMask {
                    write!(f, "^")?;
                } else {
                    write!(f, ".")?;
                }
                mask >>= 1;
            }
            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedStateValueInfo {
    machine_types: Vec<MachineType>,
    sparse_input_mask: SparseInputMask,
}

impl TypedStateValueInfo {
    pub fn new(machine_types: Vec<MachineType>, sparse_input_mask: SparseInputMask) -> Self {
        TypedStateValueInfo {
            machine_types,
            sparse_input_mask,
        }
    }

    pub fn machine_types(&self) -> &Vec<MachineType> {
        &self.machine_types
    }
    pub fn sparse_input_mask(&self) -> SparseInputMask {
        self.sparse_input_mask
    }
}

impl fmt::Display for TypedStateValueInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {}", self.machine_types, self.sparse_input_mask)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionObservability {
    kObservable,
    kNotObservable,
}

impl fmt::Display for RegionObservability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionObservability::kObservable => write!(f, "observable"),
            RegionObservability::kNotObservable => write!(f, "not-observable"),
        }
    }
}

pub fn region_observability_of(op: &Operator) -> RegionObservability {
    if op.opcode == IrOpcode::kBeginRegion {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(region_observability) = any.downcast_ref::<RegionObservability>() {
                return *region_observability;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn type_guard_type_of(op: &Operator) -> Type {
    if op.opcode == IrOpcode::kTypeGuard {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(type_val) = any.downcast_ref::<Type>() {
                return type_val.clone();
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn osr_value_index_of(op: &Operator) -> i32 {
    if op.opcode == IrOpcode::kOsrValue {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(index) = any.downcast_ref::<i32>() {
                return *index;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn sparse_input_mask_of(op: &Operator) -> SparseInputMask {
    match op.opcode {
        IrOpcode::kStateValues => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(sparse_mask) = any.downcast_ref::<SparseInputMask>() {
                    return *sparse_mask;
                }
            }
            panic!("Invalid operator parameter type");
        }
        IrOpcode::kTypedStateValues => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(typed_state) = any.downcast_ref::<TypedStateValueInfo>() {
                    return typed_state.sparse_input_mask();
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode"),
    }
}

pub fn machine_types_of(op: &Operator) -> &Vec<MachineType> {
    match op.opcode {
        IrOpcode::kTypedObjectState => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(typed_object) = any.downcast_ref::<TypedObjectStateInfo>() {
                    return typed_object.machine_types();
                }
            }
            panic!("Invalid operator parameter type");
        }
        IrOpcode::kTypedStateValues => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(typed_state) = any.downcast_ref::<TypedStateValueInfo>() {
                    return typed_state.machine_types();
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreateArgumentsType {
    kUnmappedArguments,
    kMappedArguments,
    kRestParameter,
}

pub fn arguments_state_type_of(op: &Operator) -> CreateArgumentsType {
    if op.opcode == IrOpcode::kArgumentsElementsState {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(arg_type) = any.downcast_ref::<CreateArgumentsType>() {
                return *arg_type;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

pub fn object_id_of(op: &Operator) -> u32 {
    match op.opcode {
        IrOpcode::kObjectState => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(object_state) = any.downcast_ref::<ObjectStateInfo>() {
                    return object_state.object_id();
                }
            }
            panic!("Invalid operator parameter type");
        }
        IrOpcode::kTypedObjectState => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(typed_object) = any.downcast_ref::<TypedObjectStateInfo>() {
                    return typed_object.object_id();
                }
            }
            panic!("Invalid operator parameter type");
        }
        IrOpcode::kObjectId => {
            if let Some(any) = op.parameter.as_ref() {
                if let Some(object_id) = any.downcast_ref::<u32>() {
                    return *object_id;
                }
            }
            panic!("Invalid operator parameter type");
        }
        _ => panic!("Invalid operator opcode"),
    }
}

pub fn dead_value_representation_of(op: &Operator) -> MachineType {
    if op.opcode == IrOpcode::kDeadValue {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(machine_type) = any.downcast_ref::<MachineType>() {
                return *machine_type;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IfValueParameters {
    value: i32,
    comparison_order: i32,
    hint: BranchHint,
}

impl IfValueParameters {
    pub fn new(value: i32, comparison_order: i32, hint: BranchHint) -> Self {
        IfValueParameters {
            value,
            comparison_order,
            hint,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
    pub fn comparison_order(&self) -> i32 {
        self.comparison_order
    }
    pub fn hint(&self) -> BranchHint {
        self.hint
    }
}

impl fmt::Display for IfValueParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (order {}, hint {})",
            self.value, self.comparison_order, self.hint
        )
    }
}

pub fn if_value_parameters_of(op: &Operator) -> &IfValueParameters {
    if op.opcode == IrOpcode::kIfValue {
        if let Some(any) = op.parameter.as_ref() {
            if let Some(if_value_params) = any.downcast_ref::<IfValueParameters>() {
                return if_value_params;
            }
        }
        panic!("Invalid operator parameter type");
    } else {
        panic!("Invalid operator opcode");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameStateInfo {
   
}

pub fn frame_state_info_of(op: &Operator) -> &FrameStateInfo {
    if op.opcode == IrOpcode::kFrameState {
        return op.parameter.as_ref().unwrap().downcast_ref::<FrameStateInfo>().unwrap();
    }
    panic!("Unexpected operator opcode: {:?}", op.opcode);
}

pub fn heap_constant_of(op: &Operator) -> HeapObject {
    if op.opcode == IrOpcode::kHeapConstant {
        return *op.parameter.as_ref().unwrap().downcast_ref::<HeapObject>().unwrap();
    }
    panic!("Unexpected operator opcode: {:?}", op.opcode);
}

pub struct HeapObject {}
pub struct IndirectHandle<T> {
    object: T,
}

impl<T> Deref for IndirectHandle<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

pub struct StaticAssertSource {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SLVerifierHintParameters {
    semantics: Operator,
    override_output_type: Option<Type>,
}

impl SLVerifierHintParameters {
   
}

impl fmt::Display for SLVerifierHintParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f,"")
    }
}

pub fn sl_verifier_hint_parameters_of(op: &Operator) -> &SLVerifierHintParameters {
    if op.opcode == IrOpcode::kSLVerifierHint {
        return op.parameter.as_ref().unwrap().downcast_ref::<SLVerifierHintParameters>().unwrap();
    }
    panic!("Unexpected operator opcode: {:?}", op.opcode);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExitMachineGraphParameters {
    output_representation: MachineType,
    output_type: Type,
}

impl ExitMachineGraphParameters {
   
}

impl fmt::Display for ExitMachineGraphParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f,"")
    }
}

pub fn exit_machine_graph_parameters_of(op: &Operator) -> &ExitMachineGraphParameters {
    if op.opcode == IrOpcode::kExitMachineGraph {
        return op.parameter.as_ref().unwrap().downcast_ref::<ExitMachineGraphParameters>().unwrap();
    }
    panic!("Unexpected operator opcode: {:?}", op.opcode);
}

#[derive(Clone)]
pub struct CommonOperatorBuilder {
    cache_: Rc<CommonOperatorGlobalCache>,
    zone_: Rc<Zone>,
}

impl CommonOperatorBuilder {
    pub fn new(zone: Rc<Zone>) -> Self {
        CommonOperatorBuilder {
            cache_: Rc::new(CommonOperatorGlobalCache::new()),
            zone_: zone,
        }
    }

    pub fn plug(&self) -> &Operator {
        &self.cache_.kPlugOperator
    }

    pub fn chained(&self, op: &Operator) -> &Operator {
        let mnemonic = match op.opcode {
            IrOpcode::kChangeInt64ToBigInt => "Chained[ChangeInt64ToBigInt]",
            IrOpcode::kChangeUint64ToBigInt => "Chained[Change
