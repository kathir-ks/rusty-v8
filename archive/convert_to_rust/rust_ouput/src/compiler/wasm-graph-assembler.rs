// Converted from V8 C++ source files:
// Header: wasm-graph-assembler.h
// Implementation: wasm-graph-assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::mem;
use std::ptr;
use crate::Builtin;
use crate::CallDescriptor;
use crate::BranchHint;
use crate::GraphAssembler;
use crate::MachineType;
use crate::GraphAssemblerLabelType;
use crate::WasmTypeCheckConfig;
use crate::TrapId;
use crate::CheckForNull;
use crate::InstanceType;
use crate::SimplifiedOperatorBuilder;
use crate::StubCallMode;
use crate::RootIndex;
use crate::Address;
use crate::JSArrayRef;
use crate::HeapObject;
use crate::SharedFunctionInfoRef;
use crate::JSHeapBroker;
use crate::ValueType;
use crate::Reduction;
use crate::IndirectHandle;
use crate::FeedbackVector;
use crate::Flags;
use crate::BranchSemantics;
use crate::CallInterfaceDescriptor;
use crate::Linkage;

pub struct Zone {}
pub struct NodeId(u32);
pub struct Node {}
pub struct Operator {}
pub struct Tagged_t(usize);
pub struct ObjectAccess {
    machine_type: MachineType,
    write_barrier: WriteBarrierKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriteBarrierKind {
    kNoWriteBarrier,
    kMapWriteBarrier,
    kFullWriteBarrier,
}

impl ObjectAccess {
    pub fn new(machine_type: MachineType, write_barrier: WriteBarrierKind) -> Self {
        ObjectAccess {
            machine_type,
            write_barrier,
        }
    }
    pub fn ToTagged(offset: i32) -> i32 {
        offset
    }
    pub fn ElementOffsetInTaggedFixedArray(index: i32) -> i32 {
        index
    }
    pub fn SharedFunctionInfoOffsetInTaggedJSFunction() -> i32 {
        0 // Replace with the actual offset value
    }
    pub fn ContextOffsetInTaggedJSFunction() -> i32 {
        0 // Replace with actual offset
    }
    pub fn ElementOffsetInProtectedFixedArray(index: i32) -> i32 {
        index
    }

    pub fn ToWasm(offset: i32) -> i32 {
        offset
    }
}

pub struct V8_ENABLE_SANDBOX_BOOL {}

#[derive(Clone, Copy)]
pub enum IrOpcode {
    kFrameState,
}

pub struct Diamond<'a> {
    graph: *mut Graph,
    common: *mut CommonOperatorBuilder,
    condition: *mut Node,
    branch_hint: BranchHint,
    true_label: Option<GraphAssemblerLabel<'a>>,
    false_label: Option<GraphAssemblerLabel<'a>>,
}
impl<'a> Diamond<'a> {
    pub fn Chain(&mut self, _control: *mut Node){
    }
    pub fn Phi(&mut self, _rep: MachineRepresentation, _val1: *mut Node, _val2: *mut Node) -> *mut Node {
        std::ptr::null_mut()
    }
    pub fn new(graph: *mut Graph, common: *mut CommonOperatorBuilder, condition: *mut Node, branch_hint: BranchHint) -> Self {
        Diamond {
            graph,
            common,
            condition,
            branch_hint,
            true_label: None,
            false_label: None,
        }
    }
}

pub struct GraphAssemblerLabel<'a> {
    label_type: GraphAssemblerLabelType,
    nodes: Vec<*mut Node>,
    name: String,
    assembler: &'a WasmGraphAssembler,
    is_bound: bool,
}
impl<'a> GraphAssemblerLabel<'a> {
    pub fn bind(&mut self, _node: *mut Node) {
        self.is_bound = true;
    }
}

pub struct CommonOperatorBuilder {}
impl CommonOperatorBuilder {
    pub fn NumberConstant(&self, _value: f64) -> Operator {
        Operator {}
    }
    pub fn Branch(&self, _hint: BranchHint) -> Operator {
        Operator {}
    }
    pub fn IfTrue(&self) -> Operator {
        Operator {}
    }
    pub fn IfFalse(&self) -> Operator {
        Operator {}
    }
    pub fn TrapIf(&self, _reason: TrapId, _has_frame_state: bool) -> Operator {
        Operator {}
    }
    pub fn TrapUnless(&self, _reason: TrapId, _has_frame_state: bool) -> Operator {
        Operator {}
    }
}

pub struct MachineGraph {}
impl MachineGraph {
    pub fn RelocatableWasmBuiltinCallTarget(&self, _builtin: Builtin) -> *mut Node {
        std::ptr::null_mut()
    }
}

pub struct Graph {}
impl Graph {
    pub fn NewNode(&mut self, _op: Operator, _args: *mut Node) -> *mut Node {
        std::ptr::null_mut()
    }
    pub fn NewNode(&mut self, _op: Operator) -> *mut Node {
        std::ptr::null_mut()
    }
    pub fn NewNode(&mut self, _op: Operator, _arg1: *mut Node, _arg2: *mut Node) -> *mut Node {
        std::ptr::null_mut()
    }
    pub fn NewNode(&mut self, _op: Operator, _arg1: *mut Node, _arg2: *mut Node, _arg3: *mut Node) -> *mut Node {
        std::ptr::null_mut()
    }
}

pub struct Internals {}
impl Internals {
    pub fn IntegralToSmi(value: i32) -> Address {
        Address {}
    }
}

pub struct IsolateData {}
impl IsolateData {
    pub fn shared_external_pointer_table_offset() -> i32 {
        0
    }
    pub fn external_pointer_table_offset() -> i32 {
        0
    }
    pub fn trusted_pointer_table_offset() -> i32 {
        0
    }
}

pub struct WasmInstanceObject {}
impl WasmInstanceObject {
    pub const kTrustedDataOffset: i32 = 0;
}

pub struct WasmExportedFunctionData {}
impl WasmExportedFunctionData {
    pub const kFunctionIndexOffset: i32 = 0;
    pub const kProtectedInstanceDataOffset: i32 = 0;
}

pub struct FixedArray {}
impl FixedArray {
    pub const kHeaderSize: i32 = 0;
    pub const length_: i32 = 0;
}
pub struct ProtectedFixedArray {}
impl ProtectedFixedArray {
    pub const kHeaderSize: i32 = 0;
    pub const length_: i32 = 0;
}
pub struct WeakFixedArray {}
impl WeakFixedArray {
    pub const kHeaderSize: i32 = 0;
    pub const length_: i32 = 0;
}

pub struct ByteArray {}
impl ByteArray {
    pub const kHeaderSize: i32 = 0;
}

pub struct WasmStruct {}
impl WasmStruct {
    pub const kHeaderSize: i32 = 0;
}

pub struct WasmArray {}
impl WasmArray {
    pub const kHeaderSize: i32 = 0;
}

struct Uint32Matcher(*mut Node);
impl Uint32Matcher {
    fn HasResolvedValue(&self) -> bool {
        false
    }
    fn ResolvedValue(&self) -> u32 {
        0
    }
}

const kTaggedSize: usize = 8;
const kInt32Size: usize = 4;
const kSmiShiftSize: i32 = 1;
const kSmiTagSize: i32 = 1;
const kSmiTagMask: i32 = 1;
const kExternalPointerIndexShift: i32 = 1;
const kExternalPointerTableEntrySizeLog2: i32 = 1;
const kExternalPointerTagMask: i32 = 1;
const kExternalPointerTagShift: i32 = 1;
const kExternalPointerPayloadMask: i32 = 1;
const kTrustedPointerHandleShift: i32 = 1;
const kTrustedPointerTableEntrySizeLog2: i32 = 1;
const kTrustedPointerTableMarkBit: i32 = 1;
const FIRST_WASM_OBJECT_TYPE: i32 = 0;
const LAST_WASM_OBJECT_TYPE: i32 = 0;
const kWasmTrustedInstanceDataIndirectPointerTag: i32 = 0;

fn IsSubtype(_rep: MachineRepresentation, _rep2: MachineRepresentation) -> bool {
    true
}
fn ElementSizeInBytes(_rep: MachineRepresentation) -> i32 {
    0
}
fn IsSharedExternalPointerType(_range: ExternalPointerTagRange) -> bool {
    false
}
fn ExternalPointerCanBeEmpty(_range: ExternalPointerTagRange) -> bool {
    false
}

pub enum MachineRepresentation {
    kNone,
    kBit,
    kWord8,
    kWord16,
    kWord32,
    kWord64,
    kFloat32,
    kFloat64,
    kSimd128,
    kTaggedSigned,
    kTaggedPointer,
    kTagged,
    kExternalPointer,
    kProtectedPointer,
    kSandboxedPointer,
}
impl MachineType {
    pub fn TypeForRepresentation(representation: MachineRepresentation, is_packed: bool) -> MachineType {
        MachineType {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AllocationType {
    kOld,
    kYoung,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExternalPointerTagRange {}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndirectPointerTag {}

pub struct WasmGraphAssembler {
    base: GraphAssembler,
    mcgraph_: *mut MachineGraph,
    graph_: *mut Graph,
    common_: *mut CommonOperatorBuilder,
    simplified_: SimplifiedOperatorBuilder,
    zone_: *mut Zone,
}

impl WasmGraphAssembler {
    pub fn new(mcgraph: *mut MachineGraph, zone: *mut Zone) -> Self {
        let base = GraphAssembler::new(mcgraph, zone, BranchSemantics::kMachine);
        let mcgraph_ = mcgraph;
        let simplified_ = SimplifiedOperatorBuilder {};
        let graph_ = unsafe { (*mcgraph).graph() };
        let common_ = unsafe { (*mcgraph).common() };
        WasmGraphAssembler {
            base,
            mcgraph_,
            simplified_,
            zone_: zone,
            graph_: graph_,
            common_: common_,
        }
    }

    fn mcgraph(&self) -> *mut MachineGraph {
        self.mcgraph_
    }
    fn graph(&self) -> *mut Graph {
        self.graph_
    }
    fn common(&self) -> *mut CommonOperatorBuilder {
        self.common_
    }

    pub fn temp_zone(&self) -> *mut Zone {
        self.zone_
    }

    template! {
        typename... Args
    }
    pub fn CallBuiltinThroughJumptable<Args>(&mut self, builtin: Builtin, properties: Operator::Properties, args: Args) -> *mut Node {
        let call_descriptor = GetBuiltinCallDescriptor(
            builtin,
            self.temp_zone(),
            StubCallMode::kCallWasmRuntimeStub,
            false,
            properties,
        );
        let call_target = unsafe { (*self.mcgraph()).RelocatableWasmBuiltinCallTarget(builtin) };
        self.Call(call_descriptor, call_target, args)
    }

    pub fn GetBuiltinPointerTarget(&self, builtin: Builtin) -> *mut Node {
        //static_assert!(std::is_same<Smi, BuiltinPtr>(), "BuiltinPtr must be Smi");
        self.NumberConstant(builtin as i32 as f64)
    }

    template! {
        typename... Args
    }
    pub fn CallBuiltin<Args>(&mut self, name: Builtin, properties: Operator::Properties, args: Args) -> *mut Node {
        self.CallBuiltinImpl(name, false, properties, args)
    }

    template! {
        typename... Args
    }
    pub fn CallBuiltinWithFrameState<Args>(&mut self, name: Builtin, properties: Operator::Properties, frame_state: *mut Node, args: Args) -> *mut Node {
        //DCHECK_EQ(frame_state->opcode(), IrOpcode::kFrameState);
        unsafe {
            if (*frame_state).opcode() != IrOpcode::kFrameState {
                panic!("Frame state opcode is not kFrameState");
            }
        }
        self.CallBuiltinImpl(name, true, properties, frame_state, args)
    }

    pub fn Branch(
        &mut self,
        cond: *mut Node,
        true_node: &mut *mut Node,
        false_node: &mut *mut Node,
        hint: BranchHint,
    ) -> *mut Node {
        if cond.is_null() {
            panic!("Condition cannot be null");
        }
        let branch = unsafe {
            (*self.graph()).NewNode((*self.common()).Branch(hint), cond)
        };
        *true_node = unsafe { (*self.graph()).NewNode((*self.common()).IfTrue(), branch) };
        *false_node = unsafe { (*self.graph()).NewNode((*self.common()).IfFalse(), branch) };
        branch
    }

    pub fn NumberConstant(&self, value: f64) -> *mut Node {
        unsafe {
            (*self.graph()).NewNode((*self.common()).NumberConstant(value))
        }
    }

    pub fn SmiConstant(&self, value: Tagged_t) -> *mut Node {
        let tagged_value: Address = Internals::IntegralToSmi(value.0 as i32);
        if kTaggedSize == kInt32Size {
            self.Int32Constant(tagged_value.0 as i32)
        } else {
            self.Int64Constant(tagged_value.0 as i64)
        }
    }

    pub fn MergeControlToEnd(&mut self, control: *mut Node) {
        unsafe {
            NodeProperties::MergeControlToEnd((*self.graph()), (*self.common()), control);
        }
    }

    pub fn BuildTruncateIntPtrToInt32(&self, value: *mut Node) -> *mut Node {
        if unsafe { (*self.mcgraph()).machine().Is64() } {
            self.TruncateInt64ToInt32(value)
        } else {
            value
        }
    }

    pub fn BuildChangeInt32ToIntPtr(&self, value: *mut Node) -> *mut Node {
        if unsafe { (*self.mcgraph()).machine().Is64() } {
            self.ChangeInt32ToInt64(value)
        } else {
            value
        }
    }

    pub fn BuildChangeIntPtrToInt64(&self, value: *mut Node) -> *mut Node {
        if unsafe { (*self.mcgraph()).machine().Is32() } {
            self.ChangeInt32ToInt64(value)
        } else {
            value
        }
    }

    pub fn BuildChangeUint32ToUintPtr(&self, node: *mut Node) -> *mut Node {
        if unsafe { (*self.mcgraph()).machine().Is32() } {
            return node;
        }
        let matcher = Uint32Matcher(node);
        if matcher.HasResolvedValue() {
            let value: u32 = matcher.ResolvedValue();
            return self.IntPtrConstant(unsafe { mem::transmute::<u32, i64>(value) });
        }
        self.ChangeUint32ToUint64(node)
    }

    pub fn BuildSmiShiftBitsConstant(&self) -> *mut Node {
        self.IntPtrConstant((kSmiShiftSize + kSmiTagSize) as i64)
    }

    pub fn BuildSmiShiftBitsConstant32(&self) -> *mut Node {
        self.Int32Constant((kSmiShiftSize + kSmiTagSize) as i32)
    }

    pub fn BuildChangeInt32ToSmi(&self, value: *mut Node) -> *mut Node {
        if COMPRESS_POINTERS_BOOL {
            self.BitcastWord32ToWord64(self.Word32Shl(
                value,
                self.BuildSmiShiftBitsConstant32(),
            ))
        } else {
            self.WordShl(
                self.BuildChangeInt32ToIntPtr(value),
                self.BuildSmiShiftBitsConstant(),
            )
        }
    }

    pub fn BuildChangeUint31ToSmi(&self, value: *mut Node) -> *mut Node {
        if COMPRESS_POINTERS_BOOL {
            self.Word32Shl(value, self.BuildSmiShiftBitsConstant32())
        } else {
            self.WordShl(
                self.BuildChangeUint32ToUintPtr(value),
                self.BuildSmiShiftBitsConstant(),
            )
        }
    }

    pub fn BuildChangeSmiToInt32(&self, value: *mut Node) -> *mut Node {
        if COMPRESS_POINTERS_BOOL {
            self.Word32Sar(value, self.BuildSmiShiftBitsConstant32())
        } else {
            self.BuildTruncateIntPtrToInt32(self.WordSar(
                value,
                self.BuildSmiShiftBitsConstant(),
            ))
        }
    }

    pub fn BuildConvertUint32ToSmiWithSaturation(
        &mut self,
        value: *mut Node,
        maxval: u32,
    ) -> *mut Node {
        if !Smi::IsValid(maxval) {
            panic!("Maxval is not a valid Smi");
        }
        let max = unsafe { (*self.mcgraph()).Uint32Constant(maxval) };
        let check = self.Uint32LessThanOrEqual(value, max);
        let valsmi = self.BuildChangeUint31ToSmi(value);
        let maxsmi = self.NumberConstant(maxval as f64);
        let mut d = Diamond::new(unsafe {(*self.graph()).clone()}, unsafe {(*self.common()).clone()}, check, BranchHint::kTrue);
        d.Chain(unsafe { self.control() });
        d.Phi(MachineRepresentation::kTagged, valsmi, maxsmi)
    }

    pub fn BuildChangeSmiToIntPtr(&self, value: *mut Node) -> *mut Node {
        if COMPRESS_POINTERS_BOOL {
            self.BuildChangeInt32ToIntPtr(self.Word32Sar(
                value,
                self.BuildSmiShiftBitsConstant32(),
            ))
        } else {
            self.WordSar(value, self.BuildSmiShiftBitsConstant())
        }
    }

    pub fn Allocate(&mut self, size: i32) -> *mut Node {
        self.AllocateNode(self.Int32Constant(size))
    }

    pub fn AllocateNode(&mut self, size: *mut Node) -> *mut Node {
        unsafe {
            let allocation_type = AllocationType::kYoung;
            let op = self.simplified().AllocateRaw(Type::Any(), allocation_type);
            let effect = self.effect();
            let control = self.control();
            let new_node = (*self.graph()).NewNode(op, size, effect, control);
            self.AddNode(new_node)
        }
    }

    pub fn LoadFromObject(&mut self, type_: MachineType, base: *mut Node, offset: *mut Node) -> *mut Node {
        unsafe {
            let object_access = ObjectAccess::new(type_, WriteBarrierKind::kNoWriteBarrier);
            let op = self.simplified().LoadFromObject(object_access);
            let effect = self.effect();
            let control = self.control();
            let new_node = (*self.graph()).NewNode(op, base, offset, effect, control);
            self.AddNode(new_node)
        }
    }

    pub fn LoadFromObject_int(&mut self, type_: MachineType, base: *mut Node, offset: i32) -> *mut Node {
        self.LoadFromObject(type_, base, self.IntPtrConstant(offset as i64))
    }

    pub fn LoadProtectedPointerFromObject(&mut self, object: *mut Node, offset: *mut Node) -> *mut Node {
        let machine_type = if true { //V8_ENABLE_SANDBOX_BOOL
            MachineType::ProtectedPointer()
        } else {
            MachineType::AnyTagged()
        };
        self.LoadFromObject(machine_type, object, offset)
    }

    pub fn LoadProtectedPointerFromObject_int(&mut self, object: *mut Node, offset: i32) -> *mut Node {
        self.LoadProtectedPointerFromObject(object, self.IntPtrConstant(offset as i64))
    }

    pub fn LoadImmutableProtectedPointerFromObject(&mut self, object: *mut Node, offset: *mut Node) -> *mut Node {
        let machine_type = if true { //V8_ENABLE_SANDBOX_BOOL
            MachineType::ProtectedPointer()
        } else {
            MachineType::AnyTagged()
        };
        self.LoadImmutableFromObject(machine_type, object, offset)
    }

    pub fn LoadImmutableProtectedPointerFromObject_int(&mut self, object: *mut Node, offset: i32) -> *mut Node {
        self.LoadImmutableProtectedPointerFromObject(object, self.IntPtrConstant(offset as i64))
    }

    pub fn LoadImmutableFromObject(&mut self, type_: MachineType, base: *mut Node, offset: *mut Node) -> *mut Node {
        unsafe {
            let object_access = ObjectAccess::new(type_, WriteBarrierKind::kNoWriteBarrier);
            let op = self.simplified().LoadImmutableFromObject(object_access);
            let effect = self.effect();
            let control = self.control();
            let new_node = (*self.graph()).NewNode(op, base, offset, effect, control);
            self.AddNode(new_node)
        }
    }

    pub fn LoadImmutableFromObject_int(&mut self, type_: MachineType, base: *mut Node, offset: i32) -> *mut Node {
        self.LoadImmutableFromObject(type_, base, self.IntPtrConstant(offset as i64))
    }

    pub fn LoadImmutable(&mut self, rep: LoadRepresentation, base: *mut Node, offset: *mut Node) -> *mut Node {
        unsafe {
            let op = (*self.mcgraph()).machine().LoadImmutable(rep);
            let new_node = (*self.graph()).NewNode(op, base, offset);
            self.AddNode(new_node)
        }
    }

    pub fn LoadImmutable_int(&mut self, rep: LoadRepresentation, base: *mut Node, offset: i32) -> *mut Node {
        self.LoadImmutable(rep, base, self.IntPtrConstant(offset as i64))
    }

    pub fn LoadWasmCodePointer(&mut self, code_pointer: *mut Node) -> *mut Node {
        let table_entry = self.IntAdd(
            self.ExternalConstant(ExternalReference::wasm_code_pointer_table()),
            self.IntMul(
                self.BuildChangeUint32ToUintPtr(code_pointer),
                self.UintPtrConstant(mem::size_of::<WasmCodePointerTableEntry>() as i64),
            ),
        );
        unsafe {
            let op = (*self.mcgraph()).machine().Load(LoadRepresentation::UintPtr());
            let new_node = (*self.graph()).NewNode(op, table_entry);
            self.AddNode(new_node)
        }
    }

    pub fn StoreToObject(
        &mut self,
        access: ObjectAccess,
        base: *mut Node,
        offset: *mut Node,
        value: *mut Node,
    ) -> *mut Node {
        unsafe {
            let op = self.simplified().StoreToObject(access);
            let effect = self.effect();
            let control = self.control();
            let new_node = (*self.graph()).NewNode(op, base, offset, value, effect, control);
            self.AddNode(new_node)
        }
    }

    pub fn StoreToObject_int(
        &mut self,
        access: ObjectAccess,
        base: *mut Node,
        offset: i32,
        value: *mut Node,
    ) -> *mut Node {
        self.StoreToObject(access, base, self.IntPtrConstant(offset as i64), value)
    }

    pub fn InitializeImmutableInObject(
        &mut self,
        access: ObjectAccess,
        base: *mut Node,
        offset: *mut Node,
        value: *mut Node,
    ) -> *mut Node {
        unsafe {
            let op = self.simplified().InitializeImmutableInObject(access);
            let effect = self.effect();
            let control = self.control();
            let new_node = (*self.graph()).NewNode(op, base, offset, value, effect, control);
            self.AddNode(new_node)
        }
    }

    pub fn InitializeImmutableInObject_int(
        &mut self,
        access: ObjectAccess,
        base: *mut Node,
        offset: i32,
        value: *mut Node,
    ) -> *mut Node {
        self.InitializeImmutableInObject(access, base, self.IntPtrConstant(offset as i64), value)
    }

    pub fn BuildDecodeSandboxedExternalPointer(
        &mut self,
        handle: *mut Node,
        tag_range: ExternalPointerTagRange,
        isolate_root: *mut Node,
    ) -> *mut Node {
        if true { //V8_ENABLE_SANDBOX
            let index = self.Word32Shr(handle, self.Int32Constant(kExternalPointerIndexShift));
            let offset = self.ChangeUint32ToUint64(self.Word32Shl(
                index,
                self.Int32Constant(kExternalPointerTableEntrySizeLog2),
            ));
            let table: *mut Node;
            if IsSharedExternalPointerType(tag_range) {
                let table_address = self.Load(
                    MachineType::Pointer(),
                    isolate_root,
                    IsolateData::shared_external_pointer_table_offset(),
                );
                table = self.Load(
                    MachineType::Pointer(),
                    table_address,
                    Internals::kExternalPointerTableBasePointerOffset,
                );
            } else {
                table = self.Load(
                    MachineType::Pointer(),
                    isolate_root,
                    IsolateData::external_pointer_table_offset()
                        + Internals::kExternalPointerTableBasePointerOffset,
                );
            }

            if ExternalPointerCanBeEmpty(tag_range) {
                panic!("Unsupported");
            }

            let entry = self.Load(MachineType::Pointer(), table, offset);
            if true { //tag_range.Size() == 1 {
                let actual_tag = self.WordAnd(
                    entry,
                    self.UintPtrConstant(kExternalPointerTagMask as i64),
                );
                let actual_tag = self.TruncateInt64ToInt32(self.WordShr(
                    actual_tag,
                    self.IntPtrConstant(kExternalPointerTagShift as i64),
                ));
                let expected_tag = self.Int32Constant(0); //tag_range.first);
                let pointer = self.WordAnd(
                    entry,
                    self.IntPtrConstant(kExternalPointerPayloadMask as i64),
                );
                self
                    .GotoIf(self.Word32Equal(actual_tag, expected_tag), &mut GraphAssemblerLabel{
                        label_type: GraphAssemblerLabelType::Normal,
                        nodes: Vec::new(),
                        name: String::from(""),
                        assembler: self,
                        is_bound: false,
                    }, BranchHint::kTrue);
                self.RuntimeAbort(AbortReason::kExternalPointerTagMismatch);
                pointer
            } else {
                panic!("Unsupported");
            }
        } else {
            panic!("Unsupported");
        }
    }

    pub fn BuildDecodeTrustedPointer(&mut self, handle: *mut Node, tag: IndirectPointerTag) -> *mut Node {
        if true { // V8_ENABLE_SANDBOX
            let index = self.Word32Shr(handle, self.Int32Constant(kTrustedPointerHandleShift));
            let offset = self.ChangeUint32ToUint64(self.Word32Shl(index, self.Int32Constant(kTrustedPointerTableEntrySizeLog2)));
            let table = self.Load(
                MachineType::Pointer(),
                self.LoadRootRegister(),
                IsolateData::trusted_pointer_table_offset() + Internals::kTrustedPointerTableBasePointerOffset,
            );
            let decoded_ptr = self.Load(MachineType::Pointer(), table, offset);
            let decoded_ptr = self.WordAnd(decoded_ptr, self.IntPtrConstant(!(tag as i64 | kTrustedPointerTableMarkBit as i64)));
            let decoded_ptr = self.BitcastWordToTagged(decoded_ptr);
            decoded_ptr
        } else {
            panic!("Unsupported");
        }
    }

    pub fn BuildLoadExternalPointerFromObject(
        &mut self,
        object: *mut Node,
        field_offset: i32,
        tag_range: ExternalPointerTagRange,
        isolate_root: *mut Node,
    ) -> *mut Node {
        if !true { //tag_range.IsEmpty() {
             panic!("tag range is empty");
        }
        let handle = self.LoadFromObject_int(MachineType::Uint32(), object, ObjectAccess::ToTagged(field_offset));
        self.BuildDecodeSandboxedExternalPointer(handle, tag_range, isolate_root)
    }

    pub fn IsSmi(&self, object: *mut Node) -> *mut Node {
        if COMPRESS_POINTERS_BOOL {
            self.Word32Equal(
                self.Word32And(object, self.Int32Constant(kSmiTagMask)),
                self.Int32Constant(kSmiTag),
            )
        } else {
            self.WordEqual(
                self.WordAnd(object, self.IntPtrConstant(kSmiTagMask as i64)),
                self.IntPtrConstant(kSmiTag as i64),
            )
        }
    }

    pub fn LoadMap(&mut self, object: *mut Node) -> *mut Node {
        let map_word = self.LoadImmutableFromObject_int(
            MachineType::TaggedPointer(),
            object,
            HeapObject::kMapOffset - kHeapObjectTag,
        );
        self.UnpackMapWord(map_word)
    }

    pub fn StoreMap(&mut self, heap_object: *mut Node, map: *mut Node) {
        let access = ObjectAccess::new(
            MachineType::TaggedPointer(),
            WriteBarrierKind::kMapWriteBarrier,
        );
        let map = unsafe { map };// PackMapWord(TNode<Map>::UncheckedCast(map));
        self.InitializeImmutableInObject(
            access,
            heap_object,
            HeapObject::kMapOffset - kHeapObjectTag,
            map,
        );
    }

    pub fn LoadInstanceType(&mut self, map: *mut Node) -> *mut Node {
        self.LoadImmutableFromObject_int(
            MachineType::Uint16(),
            map,
            ObjectAccess::ToTagged(Map::kInstanceTypeOffset),
        )
    }

    pub fn LoadWasmTypeInfo(&mut self, map: *mut Node) -> *mut Node {
        let offset = Map::kConstructorOrBackPointerOrNativeContextOffset;
        self.LoadImmutableFromObject_int(
            MachineType::TaggedPointer(),
            map,
            ObjectAccess::ToTagged(offset),
        )
    }

    pub fn LoadFixedArrayLengthAsSmi(&mut self, fixed_array: *mut Node) -> *mut Node {
        self.LoadImmutableFromObject_int(
            MachineType::TaggedSigned(),
            fixed_array,
            ObjectAccess::ToTagged(FixedArray::length_),
        )
    }

    pub fn LoadFixedArrayElement(
        &mut self,
        fixed_array: *mut Node,
        index_intptr: *mut Node,
        type_: MachineType,
    ) -> *mut Node {
        let offset = self.IntAdd(
            self.IntMul(
                index_intptr,
                self.IntPtrConstant(kTaggedSize as i64),
            ),
            self.IntPtrConstant(ObjectAccess::ToTagged(OFFSET_OF_DATA_START(FixedArray)) as i64),
        );
        self.LoadFromObject(type_, fixed_array, offset)
    }

    pub fn LoadWeakFixedArrayElement(&mut self, fixed_array: *mut Node, index_intptr: *mut Node) -> *mut Node {
        let offset = self.IntAdd(
            self.IntMul(
                index_intptr,
                self.IntPtrConstant(kTaggedSize as i64),
            ),
            self.IntPtrConstant(ObjectAccess::ToTagged(OFFSET_OF_DATA_START(WeakFixedArray)) as i64),
        );
        self.LoadFromObject(MachineType::AnyTagged(), fixed_array, offset)
    }

    pub fn LoadImmutableFixedArrayElement(
        &mut self,
        fixed_array: *mut Node,
        index_intptr: *mut Node,
        type_: MachineType,
    ) ->
