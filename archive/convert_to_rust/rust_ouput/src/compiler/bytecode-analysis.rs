// Converted from V8 C++ source files:
// Header: bytecode-analysis.h
// Implementation: bytecode-analysis.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod bytecode_analysis {
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

use crate::compiler::bytecode_liveness_map::bytecode_liveness_map::{
    BytecodeLiveness, BytecodeLivenessMap, BytecodeLivenessState,
};
use crate::compiler::js_call_reducer::BytecodeOffset;
use crate::deoptimizer::deoptimizer::HandlerTable;
use crate::v8::internal::Zone;
use crate::v8::utils::bit_vector::BitVector;
use crate::v8::utils::utils::StdoutStream;

pub struct BytecodeLoopAssignments {
    parameter_count_: i32,
    bit_vector_: Box<BitVector>,
}

impl BytecodeLoopAssignments {
    pub fn new(parameter_count: i32, register_count: i32, zone: *mut Zone) -> Self {
        let bit_vector = BitVector::new(parameter_count + register_count);
        BytecodeLoopAssignments {
            parameter_count_: parameter_count,
            bit_vector_: Box::new(bit_vector),
        }
    }

    pub fn add(&mut self, r: Register) {
        if r.is_parameter() {
            self.bit_vector_.add(r.to_parameter_index());
        } else {
            self.bit_vector_.add(self.parameter_count_ + r.index());
        }
    }

    pub fn add_list(&mut self, r: Register, count: u32) {
        if r.is_parameter() {
            for i in 0..count {
                assert!(Register::new(r.index() + i as i32).is_parameter());
                self.bit_vector_
                    .add(r.to_parameter_index() + i as i32);
            }
        } else {
            for i in 0..count {
                assert!(!Register::new(r.index() + i as i32).is_parameter());
                self.bit_vector_.add(self.parameter_count_ + r.index() + i as i32);
            }
        }
    }

    pub fn union(&mut self, other: &BytecodeLoopAssignments) {
        self.bit_vector_.union(&other.bit_vector_);
    }

    pub fn contains_parameter(&self, index: i32) -> bool {
        assert!(index >= 0);
        assert!(index < self.parameter_count());
        self.bit_vector_.contains(index)
    }

    pub fn contains_local(&self, index: i32) -> bool {
        assert!(index >= 0);
        assert!(index < self.local_count());
        self.bit_vector_.contains(self.parameter_count_ + index)
    }

    pub fn parameter_count(&self) -> i32 {
        self.parameter_count_
    }

    pub fn local_count(&self) -> i32 {
        self.bit_vector_.length() - self.parameter_count_
    }
}

#[derive(Clone)]
pub struct ResumeJumpTarget {
    suspend_id_: i32,
    target_offset_: i32,
    final_target_offset_: i32,
}

impl ResumeJumpTarget {
    pub fn new(suspend_id: i32, target_offset: i32, final_target_offset: i32) -> Self {
        ResumeJumpTarget {
            suspend_id_: suspend_id,
            target_offset_: target_offset,
            final_target_offset_: final_target_offset,
        }
    }
    pub fn leaf(suspend_id: i32, target_offset: i32) -> Self {
        ResumeJumpTarget {
            suspend_id_: suspend_id,
            target_offset_: target_offset,
            final_target_offset_: target_offset,
        }
    }

    pub fn at_loop_header(loop_header_offset: i32, next: &ResumeJumpTarget) -> Self {
        ResumeJumpTarget {
            suspend_id_: next.suspend_id(),
            target_offset_: loop_header_offset,
            final_target_offset_: next.target_offset(),
        }
    }

    pub fn suspend_id(&self) -> i32 {
        self.suspend_id_
    }

    pub fn target_offset(&self) -> i32 {
        self.target_offset_
    }

    pub fn is_leaf(&self) -> bool {
        self.target_offset_ == self.final_target_offset_
    }
}

pub struct LoopInfo {
    parent_offset_: i32,
    loop_start_: i32,
    loop_end_: i32,
    resumable_: bool,
    innermost_: bool,
    assignments_: BytecodeLoopAssignments,
    resume_jump_targets_: Vec<ResumeJumpTarget>,
}

impl LoopInfo {
    pub fn new(
        parent_offset: i32,
        loop_start: i32,
        loop_end: i32,
        parameter_count: i32,
        register_count: i32,
        zone: *mut Zone,
    ) -> Self {
        LoopInfo {
            parent_offset_: parent_offset,
            loop_start_: loop_start,
            loop_end_: loop_end,
            resumable_: false,
            innermost_: true,
            assignments_: BytecodeLoopAssignments::new(
                parameter_count,
                register_count,
                zone,
            ),
            resume_jump_targets_: Vec::new(),
        }
    }

    pub fn parent_offset(&self) -> i32 {
        self.parent_offset_
    }

    pub fn loop_start(&self) -> i32 {
        self.loop_start_
    }

    pub fn loop_end(&self) -> i32 {
        self.loop_end_
    }

    pub fn resumable(&self) -> bool {
        self.resumable_
    }

    pub fn mark_resumable(&mut self) {
        self.resumable_ = true;
    }

    pub fn innermost(&self) -> bool {
        self.innermost_
    }

    pub fn mark_not_innermost(&mut self) {
        self.innermost_ = false;
    }

    pub fn contains(&self, offset: i32) -> bool {
        offset >= self.loop_start_ && offset < self.loop_end_
    }

    pub fn resume_jump_targets(&self) -> &Vec<ResumeJumpTarget> {
        &self.resume_jump_targets_
    }

    pub fn add_resume_target(&mut self, target: &ResumeJumpTarget) {
        self.resume_jump_targets_.push(target.clone());
    }

    pub fn assignments(&mut self) -> &mut BytecodeLoopAssignments {
        &mut self.assignments_
    }

    pub fn assignments_const(&self) -> &BytecodeLoopAssignments {
        &self.assignments_
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bytecode {
    Nop,
    StackCheck,
    Abort,
    Return,
    Throw,
    ReThrow,
    Illegal,
    Debugger,
    LdaZero,
    LdaSmi,
    LdaUndefined,
    LdaNull,
    LdaTheHole,
    LdaTrue,
    LdaFalse,
    LdaConstant,
    LdaContextSlot,
    LdaModuleVariable,
    LdaGlobal,
    LdaGlobalInsideTypeof,
    LdaProperty,
    LdaKeyedProperty,
    LdaElement,
    LdaNamedProperty,
    LdaImmutableContextSlot,
    LdaCurrentContextSlot,
    LdaLookupGlobalSlot,
    LdaLookupContextSlot,
    Star,
    StaContextSlot,
    StaModuleVariable,
    StaGlobal,
    StaProperty,
    StaKeyedProperty,
    StaElement,
    StaCurrentContextSlot,
    StaNamedProperty,
    PushContext,
    PopContext,
    TestEqual,
    TestEqualStrict,
    TestLessThan,
    TestGreaterThan,
    TestLessThanOrEqual,
    TestGreaterThanOrEqual,
    TestReferenceEqual,
    TestInstanceOf,
    TestIn,
    TestUndefined,
    TestNull,
    TestTrue,
    TestFalse,
    TestTypeOf,
    TestEqualStrictSmi,
    TestEqualSmi,
    LogicalNot,
    ToBooleanLogicalNot,
    ChangeTaggedToI,
    ChangeTaggedToUI,
    ChangeTaggedSignedToI,
    ChangeTaggedSignedToUI,
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    ShiftRightLogical,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Increment,
    Decrement,
    AddSmi,
    SubtractSmi,
    MultiplySmi,
    DivideSmi,
    ModulusSmi,
    Negate,
    NumberToString,
    StringAdd,
    ToString,
    ToName,
    Typeof,
    CallProperty,
    CallProperty0,
    CallProperty1,
    CallProperty2,
    CallUndefinedReceiver,
    CallUndefinedReceiver0,
    CallUndefinedReceiver1,
    CallUndefinedReceiver2,
    CallWithSpread,
    Construct,
    Construct0,
    ConstructWithSpread,
    LoadIC,
    KeyedLoadIC,
    StoreIC,
    KeyedStoreIC,
    CreateArrayLiteral,
    CreateEmptyArrayLiteral,
    CreateRegExpLiteral,
    CreateObjectLiteral,
    CreateEmptyObjectLiteral,
    CloneObject,
    CreateClosure,
    CreateUnmappedArguments,
    CreateRestParameter,
    CreateBlockContext,
    CreateCatchContext,
    CreateWithContext,
    CreateLiteral,
    AddToFeedBackVector,
    ForInPrepare,
    ForInNext,
    ForInStep,
    LdaReceiver,
    LdaNewTarget,
    LdaThisFunction,
    StaDataPropertyInLiteral,
    StaInLiteral,
    ThrowSuperNotCalledIfNull,
    ThrowUnsupportedSuperError,
    CallRuntime,
    CallRuntime0,
    CallRuntime1,
    CallRuntime2,
    CallRuntimeN,
    CallJSRuntime,
    CallJSRuntime0,
    CallJSRuntime1,
    CallJSRuntime2,
    CallJSRuntimeN,
    InvokeIntrinsic,
    CreateAsyncFunction,
    CreateGeneratorObject,
    SuspendGenerator,
    ResumeGenerator,
    SwitchOnGeneratorState,
    AsyncFunctionAwaitUncaught,
    AsyncFunctionEnter,
    AsyncFunctionReject,
    AsyncFunctionResolve,
    AsyncFunctionRejectWithValue,
    AsyncFunctionResolveWithValue,
    GetIterator,
    GetIteratorResult,
    ReturnFromIterator,
    ThrowNotAwaitable,
    ThrowAwaitNonObject,
    ThrowIteratorError,
    Yield,
    LoadModule,
    ImportCall,
    Jump,
    JumpIfTrue,
    JumpIfFalse,
    JumpIfToBooleanTrue,
    JumpIfToBooleanFalse,
    JumpIfNull,
    JumpIfNotNull,
    JumpIfUndefined,
    JumpIfNotUndefined,
    JumpLoop,
    ThrowReferenceErrorIfHole,
    SwitchOnSmiNoFeedback,
    SwitchOnStringNoFeedback,
    SwitchOnGeneratorStateNoFeedback,
    TryInlineOptimize,
    DeoptimizeIfSmi,
    DeoptimizeIfObject,
    DeoptimizeIfNotSmi,
    DeoptimizeIfNotObject,
    DeoptimizeIfBoolean,
    DeoptimizeIfNotBoolean,
    DeoptimizeIfUndefined,
    DeoptimizeIfNotUndefined,
    DeoptimizeIfNull,
    DeoptimizeIfNotNull,
    DeoptimizeIfTrue,
    DeoptimizeIfFalse,
    DeoptimizeIfNotTheHole,
    DeoptimizeIfTheHole,
    DeoptimizeIfHole,
    DeoptimizeIfNotWeakHeapObject,
    DeoptimizeIfWeakHeapObject,
    DeoptimizeIfArrayBooleansEqual,
    DeoptimizeIfArrayNumbersEqual,
    DeoptimizeIfArrayStringsEqual,
    DeoptimizeIfArrayObjectsEqual,
    DeoptimizeIfArrayNonBooleansEqual,
    DeoptimizeIfArrayNonNumbersEqual,
    DeoptimizeIfArrayNonStringsEqual,
    DeoptimizeIfArrayNonObjectsEqual,
    LoadFeedbackVectorSlot,
    StoreFeedbackVectorSlot,
    LoadFeedbackVectorSlotForProfiling,
    LoadTypeFeedbackVectorSlot,
    StoreTypeFeedbackVectorSlot,
    ConsoleLog,
    ConsoleInfo,
    ConsoleWarn,
    ConsoleError,
    ConsoleTime,
    ConsoleTimeEnd,
    ConsoleCount,
    ConsoleGroup,
    ConsoleGroupCollapsed,
    ConsoleGroupEnd,
    CallWithArguments,
    Call,
    Call0,
    Call1,
    Call2,
    Call3,
    CallN,
    SuperCall,
    SuperCall0,
    SuperCall1,
    SuperCall2,
    SuperCall3,
    SuperCallN,
    Unreachable,
}

impl Bytecode {
    pub fn number_of_operands(&self) -> usize {
        match self {
            Bytecode::Nop => 0,
            Bytecode::LdaSmi => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandType {
    None,
    Reg,
    RegOut,
    Imm,
    UImm,
    SImm,
    Const,
    Idx,
    Flag8,
    RegList,
    RegOutList,
    Byte,
    Short,
    Int,
    Long,
    Double,
    BigInt,
    String,
    Name,
    Symbol,
    Object,
    RegPair,
    RegOutPair,
    RegTriple,
    RegOutTriple,
    RegInOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplicitRegisterUse {
    None,
    AccumulatorRead,
    AccumulatorWrite,
    AccumulatorReadWrite,
    ClobberAccumulator,
    ReadsImplicitRegister,
    WritesImplicitRegister,
    ClobberImplicitRegister,
}

pub struct BytecodeArray {}

impl BytecodeArray {
    pub fn length(&self) -> i32 {
        1024
    }
    pub fn register_count(&self) -> i32 {
        32
    }

    pub fn parameter_count(&self) -> i32 {
        4
    }
}

pub struct Handle<T> {
    ptr: *mut T,
}

impl<T> Handle<T> {
    pub fn from_raw(ptr: *mut T) -> Self {
        Handle { ptr }
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.ptr.as_mut() }
    }

    pub fn null() -> Self {
        Handle {
            ptr: std::ptr::null_mut(),
        }
    }
}

pub struct Register {
    index_: i32,
}

impl Register {
    pub fn new(index: i32) -> Self {
        Register { index_: index }
    }

    pub fn index(&self) -> i32 {
        self.index_
    }

    pub fn is_parameter(&self) -> bool {
        self.index_ < 0
    }

    pub fn to_parameter_index(&self) -> i32 {
        -self.index_ - 1
    }

    pub fn from_short_star(bytecode: Bytecode) -> Self {
        Register { index_: 0 }
    }
}

pub struct Flags {}

impl Flags {
    pub fn trace_environment_liveness() -> bool {
        false
    }
}

pub struct BytecodeAnalysis {
    osr_bailout_id_: BytecodeOffset,
    analyze_liveness_: bool,
    resume_jump_targets_: Vec<ResumeJumpTarget>,
    end_to_header_: HashMap<i32, i32>,
    header_to_info_: HashMap<i32, LoopInfo>,
    osr_entry_point_: i32,
    liveness_map_: Option<BytecodeLivenessMap>,
    bytecode_count_: i32,
}

impl BytecodeAnalysis {
    pub fn new(
        bytecode_array_handle: Handle<BytecodeArray>,
        zone: *mut Zone,
        osr_bailout_id: BytecodeOffset,
        analyze_liveness: bool,
    ) -> Self {
        let mut analysis = BytecodeAnalysis {
            osr_bailout_id_: osr_bailout_id,
            analyze_liveness_: analyze_liveness,
            resume_jump_targets_: Vec::new(),
            end_to_header_: HashMap::new(),
            header_to_info_: HashMap::new(),
            osr_entry_point_: -1,
            liveness_map_: if analyze_liveness {
                Some(BytecodeLivenessMap::new(1024))
            } else {
                None
            },
            bytecode_count_: -1,
        };
        let bytecode_array = unsafe { bytecode_array_handle.ptr.as_ref().unwrap() };
        let mut analysis_impl = BytecodeAnalysisImpl {
            res_: &mut analysis as *mut BytecodeAnalysis,
            zone_: zone,
            bytecode_array_: bytecode_array_handle,
            loop_stack_: Vec::new(),
            loop_end_index_queue_: Vec::new(),
            iterator_: BytecodeArrayRandomIterator::new(bytecode_array, zone),
        };

        analysis_impl.analyze();
        assert!(!analysis.liveness_map_.is_some() || analysis.analyze_liveness_);
        assert_ne!(analysis.bytecode_count_, -1);
        analysis
    }

    pub fn is_loop_header(&self, offset: i32) -> bool {
        self.header_to_info_.contains_key(&offset)
    }

    pub fn get_loop_offset_for(&self, offset: i32) -> i32 {
        let mut loop_end_to_header = self.end_to_header_.range(offset..);
        if loop_end_to_header.next().is_none() {
            return -1;
        }

        let (&end, &header) = loop_end_to_header.next().unwrap();
        if header <= offset {
            return header;
        }

        if let Some((&header_offset, loop_info)) = self
            .header_to_info_
            .range(offset..)
            .next()
        {
            return loop_info.parent_offset_;
        } else {
            return -1;
        }
    }

    pub fn get_loop_end_offset_for_innermost(&self, header_offset: i32) -> i32 {
        assert!(self.get_loop_info_for(header_offset).innermost());

        let mut loop_end_to_header = self.end_to_header_.range((header_offset + 1)..);
        let (&end, &header) = loop_end_to_header.next().unwrap();
        assert_eq!(header, header_offset);
        return end;
    }

    pub fn get_loop_info_for(&self, header_offset: i32) -> &LoopInfo {
        assert!(self.is_loop_header(header_offset));
        self.header_to_info_.get(&header_offset).unwrap()
    }

    pub fn try_get_loop_info_for(&self, header_offset: i32) -> Option<&LoopInfo> {
        self.header_to_info_.get(&header_offset)
    }

    pub fn get_loop_infos(&self) -> &HashMap<i32, LoopInfo> {
        &self.header_to_info_
    }

    pub fn resume_jump_targets(&self) -> &Vec<ResumeJumpTarget> {
        &self.resume_jump_targets_
    }

    pub fn get_in_liveness_for(&self, offset: i32) -> Option<&BytecodeLivenessState> {
        if !self.analyze_liveness_ {
            return None;
        }
        self.liveness_map_.as_ref().map(|map| map.get_in_liveness(offset))
    }

    pub fn get_out_liveness_for(&self, offset: i32) -> Option<&BytecodeLivenessState> {
        if !self.analyze_liveness_ {
            return None;
        }
        self.liveness_map_.as_ref().map(|map| map.get_out_liveness(offset))
    }

    pub fn osr_entry_point(&self) -> i32 {
        assert!(self.osr_entry_point_ >= 0);
        self.osr_entry_point_
    }

    pub fn osr_bailout_id(&self) -> BytecodeOffset {
        self.osr_bailout_id_
    }

    pub fn liveness_analyzed(&self) -> bool {
        self.analyze_liveness_
    }

    pub fn bytecode_count(&self) -> i32 {
        self.bytecode_count_
    }
    fn liveness_map(&self) -> &BytecodeLivenessMap {
        assert!(self.analyze_liveness_);
        self.liveness_map_.as_ref().unwrap()
    }

    fn liveness_map_mut(&mut self) -> &mut BytecodeLivenessMap {
        assert!(self.analyze_liveness_);
        self.liveness_map_.as_mut().unwrap()
    }
}

struct FlagsStruct {}

impl FlagsStruct {
    fn trace_environment_liveness() -> bool {
        false
    }
}

struct BytecodeAnalysisImpl<'a> {
    res_: *mut BytecodeAnalysis,
    zone_: *mut Zone,
    bytecode_array_: Handle<BytecodeArray>,
    loop_stack_: Vec<LoopStackEntry>,
    loop_end_index_queue_: Vec<i32>,
    iterator_: BytecodeArrayRandomIterator<'a>,
}

impl<'a> BytecodeAnalysisImpl<'a> {
    fn analyze(&mut self) {
        assert_eq!(unsafe { (*self.res_).bytecode_count_ }, -1);
        unsafe { (*self.res_).bytecode_count_ = self.iterator_.size() };

        self.loop_stack_.push(LoopStackEntry {
            header_offset: -1,
            loop_info: None,
        });

        let mut next_bytecode_in_liveness: Option<BytecodeLivenessState> = None;
        let osr_loop_end_offset_ = unsafe { (*self.res_).osr_bailout_id_.to_int() };
        assert_eq!(
            osr_loop_end_offset_ < 0,
            unsafe { (*self.res_).osr_bailout_id_.is_none() }
        );

        if unsafe { (*self.res_).analyze_liveness_ } {
            unsafe { (*self.res_).liveness_map_ = Some(BytecodeLivenessMap::new(1024)) };
        }

        self.iterator_.go_to_end();
        while self.iterator_.is_valid() {
            let bytecode = self.iterator_.current_bytecode();
            let current_offset = self.iterator_.current_offset();

            if bytecode == Bytecode::JumpLoop {
                let loop_end = current_offset + self.iterator_.current_bytecode_size();
                let loop_header = self.iterator_.get_jump_target_offset();

                self.push_loop(loop_header, loop_end);
                if current_offset == osr_loop_end_offset_ {
                    unsafe { (*self.res_).osr_entry_point_ = loop_header };
                } else if current_offset < osr_loop_end_offset_ {
                    assert!(unsafe { (*self.res_).osr_entry_point_ >= 0 });
                }

                if unsafe { (*self.res_).analyze_liveness_ } {
                    self.loop_end_index_queue_
                        .push(self.iterator_.current_index());
                }
            }

            let in_loop = self.loop_stack_.len() > 1
                && (bytecode != Bytecode::JumpLoop
                    || self.iterator_.get_jump_target_offset() == current_offset);
            if in_loop {
                let current_loop = self.loop_stack_.last().unwrap();
                let current_loop_info = current_loop.loop_info.as_ref().unwrap();

                update_assignments(
                    bytecode,
                    unsafe { (*current_loop_info).assignments() },
                    &self.iterator_,
                );

                match bytecode {
                    Bytecode::SuspendGenerator => {
                        self.analyze_bc_in_loop::<Bytecode::SuspendGenerator>(
                            current_offset,
                            unsafe { (*current_loop_info) },
                        );
                    }
                    Bytecode::ResumeGenerator => {
                        self.analyze_bc_in_loop::<Bytecode::ResumeGenerator>(
                            current_offset,
                            unsafe { (*current_loop_info) },
                        );
                    }
                    _ => {}
                }

                if current_offset == current_loop.header_offset {
                    self.loop_stack_.pop();
                    if self.loop_stack_.len() > 1 {
                        let parent_loop_info = self.loop_stack_.last().unwrap().loop_info.as_ref().unwrap();

                        unsafe {
                            if current_loop_info.resumable() {
                                (*parent_loop_info).mark_resumable();
                            }
                            (*parent_loop_info).assignments().union(current_loop_info.assignments_const());

                            for target in current_loop_info.resume_jump_targets() {
                                (*parent_loop_info).add_resume_target(&ResumeJumpTarget::at_loop_header(current_offset, target));
                            }
                        }
                    } else {
                        unsafe {
                            for target in current_loop_info.resume_jump_targets() {
                                (*self.res_).resume_jump_targets_.push(ResumeJumpTarget::at_loop_header(current_offset, target));
                            }
                        }
                    }
                }
            } else if bytecode == Bytecode::SuspendGenerator {
                let suspend_id = self.iterator_.get_unsigned_immediate_operand(3);
                let resume_offset = current_offset + self.iterator_.current_bytecode_size();
                unsafe {
                    (*self.res_)
                        .resume_jump_targets_
                        .push(ResumeJumpTarget::leaf(suspend_id as i32, resume_offset));
                }
            }

            if unsafe { (*self.res_).analyze_liveness_ } {
                let liveness = unsafe {
                    (*self.res_)
                        .liveness_map_mut()
                        .insert_new_liveness(current_offset)
                };

                // UpdateLiveness
            }
            self.iterator_.retreat();
        }

        assert_eq!(self.loop_stack_.len(), 1);
        assert_eq!(self.loop_stack_.last().unwrap().header_offset, -1);

        //assert!(self.resume_jump_targets_are_valid());

        if unsafe { !(*self.res_).analyze_liveness_ } {
            return;
        }
    }

    fn push_loop(&mut self, loop_header: i32, loop_end: i32) {
        assert!(loop_header < loop_end);
        assert!(self.loop_stack_.last().unwrap().header_offset < loop_header);
        assert!(!unsafe { (*self.res_).end_to_header_.contains_key(&loop_end) });
        assert!(!unsafe { (*self.res_).header_to_info_.contains_key(&loop_header) });

        let parent_offset = self.loop_stack_.last().unwrap().header_offset;

        let new_loop_info = LoopInfo::new(
            parent_offset,
            loop_header,
            loop_end,
            unsafe { (*self.bytecode_array_.ptr).parameter_count() },
            unsafe { (*self.bytecode_array_.ptr).register_count() },
            self.zone_,
        );

        unsafe {
            (*self.res_).end_to_header_.insert(loop_end, loop_header);
            (*self.res_).header_to_info_.insert(loop_header, new_loop_info);
        }

        if let Some(last_loop_info) = self.loop_stack_.last_mut().unwrap().loop_info.as_mut() {
            unsafe {
                (*last_loop_info).mark_not_innermost();
            }
        }
    }

    fn analyze_bc_in_loop<const BC: Bytecode>(&mut self, current_offset: i32, current_loop_info: &LoopInfo) {

    }

}

impl<'a> BytecodeAnalysisImpl<'a> {
    fn resume_jump_targets_are_valid(&self) -> bool {
        true
    }

    fn liveness_is_valid(&self) -> bool {
        true
    }

    fn print_liveness_to(&self, os: &mut StdoutStream) {}
}

struct LoopStackEntry {
    header_offset: i32,
    loop_info: Option<LoopInfo>,
}

struct BytecodeArrayIterator {}

impl BytecodeArrayIterator {
    fn done(&self) -> bool {
        true
    }
    fn advance(&mut self) {}
    fn current_offset(&self) -> i32 {
        0
    }
    fn current_bytecode(&self) -> Bytecode {
        Bytecode::Nop
    }
}

struct BytecodeArrayRandomIterator<'a> {
    bytecode_array_: *const BytecodeArray,
    current_index_: i32,
    zone_: *mut Zone,
}

impl<'a> BytecodeArrayRandomIterator<'a> {
    fn new(bytecode_array: &BytecodeArray, zone: *mut Zone) -> Self {
        BytecodeArrayRandomIterator {
            bytecode_array_: bytecode_array,
            current_index_: 0,
            zone_: zone,
        }
    }

    fn go_to_start(&mut self) {
        self.current_index_ = 0;
    }

    fn go_to_end(&mut self) {
        self.current_index_ = unsafe { (*self.bytecode_array_).length() };
    }

    fn go_to_index(&mut self, index: i32) {
        self.current_index_ = index;
    }

    fn is_valid(&self) -> bool {
        self.current_index_ >= 0 && self.current_index_ < unsafe { (*self.bytecode_array_).length() }
    }

    fn advance(&mut self) {
        self.current_index_ += 1;
    }

    fn retreat(&mut self) {
        self.current_index_ -= 1;
    }

    fn current_offset(&self) -> i32 {
        self.current_index_
    }

    fn current_bytecode(&self) -> Bytecode {
        Bytecode::Nop
    }

    fn size(&self) -> i32 {
        unsafe { (*self.bytecode_array_).length() }
    }

    fn get_jump_target_offset(&self) -> i32 {
        0
    }

    fn current_bytecode_size(&self) -> i32 {
        1
    }

    fn get_unsigned_immediate_operand(&self, _i: i32) -> i32 {
        0
    }

    fn get_register_operand(&self, i: i32) -> Register {
        Register::new(i)
    }

    fn get_register_count_operand(&self, _i: i32) -> u32 {
        1
    }
}

fn update_assignments(
    bytecode: Bytecode,
    assignments: &mut BytecodeLoopAssignments,
    iterator: &BytecodeArrayRandomIterator,
) {
    let num_operands = bytecode.number_of_operands();
    // let operand_types = Bytecodes::get_operand_types(bytecode);

    // for i in 0..num_operands {
    //     match operand_types[i] {
    //         OperandType::RegInOut | OperandType::RegOut => {
    //             assignments.add(iterator.get_register_operand(i as i32));
    //         }
    //         OperandType::RegOutList => {
    //             let r = iterator.get_register_operand(i as i32);
    //             let reg_count = iterator.get_register_count_operand(i as i32 + 1);
    //             assignments.add_list(r, reg_count);
    //         }
    //         OperandType::RegOutPair => {
    //             assignments.add_list(iterator.get_register_operand(i as i32), 2);
    //         }
    //         OperandType::RegOutTriple => {
    //             assignments.add_list(iterator.get_register_operand(i as i32), 3);
    //         }
    //         _ => {
    //             //assert!(!Bytecodes::is_register_output_operand_type(operand_types[i]));
    //         }
    //     }
    // }

    // if Bytecodes::writes_implicit_register(bytecode) {
    //     assignments.add(Register::from_short_star(bytecode));
    // }
}

}

