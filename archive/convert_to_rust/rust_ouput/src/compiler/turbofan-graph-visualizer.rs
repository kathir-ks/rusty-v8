// Converted from V8 C++ source files:
// Header: turbofan-graph-visualizer.h
// Implementation: turbofan-graph-visualizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::fmt;
use std::io;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use std::vec::Vec;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Write,
    fs::File,
    io::BufWriter,
    mem,
    string::String,
};

use lazy_static::lazy_static;

pub struct OptimizedCompilationInfo {}
pub struct SharedFunctionInfo {}
pub struct SourcePosition {}
pub struct WasmInliningPosition {}
pub struct WasmModule {}
pub struct WireBytesStorage {}
pub struct TFGraph {}
pub struct LiveRange {}
pub struct TopLevelLiveRange {}
pub struct Instruction {}
pub struct InstructionBlock {}
pub struct InstructionOperand {}
pub struct InstructionSequence {}
pub struct Node {}
pub struct NodeOrigin {}
pub struct NodeOriginTable {}
pub struct RegisterAllocationData {}
pub struct Schedule {}
pub struct SourcePositionTable {}
pub struct Type {}
pub struct Isolate {}
pub struct BytecodeArray {}
pub struct FeedbackVector {}
pub struct Script {}
pub struct Operator {}
pub struct Zone {
    name: String,
}
pub struct BasicBlock {}
pub struct BasicBlockVector {}
pub struct FlagsMode {}
pub struct AddressingMode {}
pub struct PhiInstruction {}
pub struct ParallelMove {}
pub struct MoveOperands {}
pub struct UnallocatedOperand {}
pub struct ConstantOperand {}
pub struct ImmediateOperand {}
pub struct LocationOperand {}
pub struct ValueType {}
pub struct DisallowGarbageCollection {}
pub struct JSHeapBroker {}
pub struct HeapObjectRef {}
pub struct MapRef {}
pub struct FeedbackSource {}
pub struct NativeModule {}
pub struct Module {}
pub struct ArrayList {}
pub struct Uses {}
pub struct Inputs {}
pub struct IrOpcode {}
pub struct BranchHint {}
pub struct Value {}
pub struct InstructionBlockAt {}
pub struct RpoNumber {}
pub struct LifetimePosition {}
pub struct UseInterval {}
pub struct UsePosition {}
pub struct Register {}
pub struct AllocatedOperand {}
pub struct Register::SpecialRegister {}
pub struct DoubleRegister {}
pub struct FloatRegister {}
pub struct Simd256Register {}
pub struct Simd128Register {}

impl Zone {
    fn new(name: String) -> Self {
        Zone { name }
    }
}

#[derive(Debug)]
pub enum TurboJsonError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for TurboJsonError {
    fn from(err: std::io::Error) -> Self {
        TurboJsonError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for TurboJsonError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        TurboJsonError::Utf8(err)
    }
}

lazy_static! {
    static ref TRACE_TURBO_FILENAME_CACHE: Mutex<HashMap<*const OptimizedCompilationInfo, String>> = Mutex::new(HashMap::new());
}

fn get_cached_trace_turbo_filename(info: *const OptimizedCompilationInfo) -> String {
    let mut cache = TRACE_TURBO_FILENAME_CACHE.lock().unwrap();
    if let Some(filename) = cache.get(&info) {
        filename.clone()
    } else {
        let filename = get_visualizer_log_file_name(
            &OptimizedCompilationInfo {},
            None,
            None,
            "json",
        );
        cache.insert(info, filename.clone());
        filename
    }
}

pub struct TurboJsonFile {
    file: BufWriter<File>,
}

impl TurboJsonFile {
    pub fn new(info: *const OptimizedCompilationInfo) -> Result<Self, TurboJsonError> {
        let filename = get_cached_trace_turbo_filename(info);
        let file = File::create(&filename)?;
        Ok(TurboJsonFile {
            file: BufWriter::new(file),
        })
    }
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), TurboJsonError> {
        Ok(self.file.write_all(buf)?)
    }
}

impl Drop for TurboJsonFile {
    fn drop(&mut self) {
        let _ = self.file.flush();
    }
}

pub struct TurboCfgFile {
    file: BufWriter<File>,
}

impl TurboCfgFile {
    pub fn new(isolate: *mut Isolate) -> Result<Self, TurboJsonError> {
        let filename = Isolate::get_turbo_cfg_file_name(isolate);
        let file = File::options()
            .append(true)
            .create(true)
            .open(&filename)?;
        Ok(TurboCfgFile {
            file: BufWriter::new(file),
        })
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), TurboJsonError> {
        Ok(self.file.write_all(buf)?)
    }
}

impl Drop for TurboCfgFile {
    fn drop(&mut self) {
        let _ = self.file.flush();
    }
}

impl Isolate {
    fn get_turbo_cfg_file_name(_isolate: *mut Isolate) -> String {
        "turbo.cfg".to_string()
    }
}

#[derive(Clone)]
pub struct DirectHandle<T> {
    ptr: *mut T,
}

impl<T> DirectHandle<T> {
    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

pub fn direct_handle<T>(ptr: *mut T, _isolate: *mut Isolate) -> DirectHandle<T> {
    DirectHandle { ptr }
}

impl SourcePosition {
    fn print_json(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        write!(out, "{{}}")
    }
    fn is_known(&self) -> bool {
        true
    }
    fn is_inlined(&self) -> bool {
        false
    }
    fn inlining_id(&self) -> i32 {
        0
    }
    fn script_offset(&self) -> i32 {
        0
    }
}

impl NodeOrigin {
    fn print_json(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        write!(out, "{{}}")
    }
    fn is_known(&self) -> bool {
        true
    }
}

struct SourcePositionAsJSON {
    sp: SourcePosition,
}

fn as_json(sp: &SourcePosition) -> SourcePositionAsJSON {
    SourcePositionAsJSON { sp: sp.clone() }
}

impl fmt::Display for SourcePositionAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = Vec::new();
        self.sp.print_json(&mut buffer).unwrap();
        let s = String::from_utf8(buffer).unwrap();
        write!(f, "{}", s)
    }
}

struct NodeOriginAsJSON {
    no: NodeOrigin,
}

fn as_json_node_origin(no: &NodeOrigin) -> NodeOriginAsJSON {
    NodeOriginAsJSON { no: no.clone() }
}

impl fmt::Display for NodeOriginAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = Vec::new();
        self.no.print_json(&mut buffer).unwrap();
        let s = String::from_utf8(buffer).unwrap();
        write!(f, "{}", s)
    }
}

pub struct SourceIdAssigner {
    printed: Vec<DirectHandle<SharedFunctionInfo>>,
    source_ids: Vec<i32>,
}

impl SourceIdAssigner {
    pub fn new(size: usize) -> Self {
        SourceIdAssigner {
            printed: Vec::with_capacity(size),
            source_ids: Vec::with_capacity(size),
        }
    }

    pub fn get_id_for(&mut self, shared: DirectHandle<SharedFunctionInfo>) -> i32 {
        for (i, printed_shared) in self.printed.iter().enumerate() {
            if printed_shared.ptr as *const _ == shared.ptr as *const _ {
                self.source_ids.push(i as i32);
                return i as i32;
            }
        }

        let source_id = self.printed.len() as i32;
        self.printed.push(shared);
        self.source_ids.push(source_id);
        source_id
    }

    pub fn get_id_at(&self, pos: usize) -> i32 {
        self.source_ids[pos]
    }
}

fn json_print_bytecode_source(
    os: &mut dyn std::io::Write,
    source_id: i32,
    function_name: String,
    bytecode_array: DirectHandle<BytecodeArray>,
    feedback_vector: DirectHandle<FeedbackVector>,
) -> Result<(), std::io::Error> {
    write!(os, "\"{}\" : {{", source_id)?;
    write!(os, "\"sourceId\": {}", source_id)?;
    write!(os, ", \"functionName\": \"{}\"", function_name)?;
    write!(os, ", \"bytecodeSource\": ")?;
    bytecode_array.print_json(os)?;
    write!(os, ", \"feedbackVector\": \"")?;
    if !feedback_vector.is_null() {
        let mut stream: Vec<u8> = Vec::new();
        FeedbackVector::print(DirectHandle {ptr:feedback_vector.ptr} ,&mut stream).unwrap();
        let s = String::from_utf8(stream).unwrap();
        let newlines_re = regex::Regex::new(r"\n+").unwrap();
        let replaced = newlines_re.replace_all(&s, "\\n").into_owned();
        write!(os, "{}", replaced)?;
    }
    write!(os, "\"}}")
}

trait PrintJson {
    fn print_json(&self, os: &mut dyn std::io::Write) -> Result<(), std::io::Error>;
}

impl PrintJson for BytecodeArray {
    fn print_json(&self, os: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        write!(os, "{{}}")
    }
}

impl FeedbackVector {
    fn print(vector: DirectHandle<FeedbackVector>, stream: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        write!(stream, "{{}}")
    }
}

fn json_print_function_source(
    os: &mut dyn std::io::Write,
    source_id: i32,
    function_name: String,
    script: DirectHandle<Script>,
    isolate: *mut Isolate,
    shared: DirectHandle<SharedFunctionInfo>,
    with_key: bool,
) -> Result<(), std::io::Error> {
    if with_key {
        write!(os, "\"{}\" : ", source_id)?;
    }

    write!(os, "{{ ")?;
    write!(os, "\"sourceId\": {}", source_id)?;
    write!(os, ", \"functionName\": \"{}\" ", function_name)?;

    let mut start = 0;
    let mut end = 0;

    if !script.is_null() && !is_undefined(script.ptr as *mut Script, isolate) && !shared.is_null() {
        //let source_name = unsafe { (*script.ptr).name() };
        write!(os, ", \"sourceName\": \"\"")?;
        start = 0;
        end = 0;
        write!(os, ", \"sourceText\": \"\"")?;
    } else {
        write!(os, ", \"sourceName\": \"\"")?;
        write!(os, ", \"sourceText\": \"\"")?;
    }
    write!(os, ", \"startPosition\": {}", start)?;
    write!(os, ", \"endPosition\": {}", end)?;
    write!(os, "}}")
}

fn is_undefined(_object: *mut Script, _isolate: *mut Isolate) -> bool {
    false
}

struct InlinedFunctionHolder {
    shared_info: DirectHandle<SharedFunctionInfo>,
    bytecode_array: DirectHandle<BytecodeArray>,
    position: InlinedFunctionPosition,
}

struct InlinedFunctionPosition {
    position: SourcePosition,
}

fn json_print_inlined_function_info(
    os: &mut dyn std::io::Write,
    source_id: i32,
    inlining_id: i32,
    h: &InlinedFunctionHolder,
) -> Result<(), std::io::Error> {
    write!(os, "\"{}\" : ", inlining_id)?;
    write!(os, "{{ \"inliningId\" : {}", inlining_id)?;
    write!(os, ", \"sourceId\" : {}", source_id)?;
    if h.position.position.is_known() {
        write!(os, ", \"inliningPosition\" : {}", as_json(&h.position.position))?;
    }
    write!(os, "}}")
}

fn json_print_all_bytecode_sources(
    os: &mut dyn std::io::Write,
    info: &OptimizedCompilationInfo,
) -> Result<(), std::io::Error> {
    write!(os, "\"bytecodeSources\" : {{")?;

    json_print_bytecode_source(
        os,
        -1,
        "DebugNameCStr".to_string(),
        DirectHandle { ptr: unsafe { std::mem::transmute(1 as usize)}},
        DirectHandle { ptr: unsafe { std::mem::transmute(1 as usize)}},
    )?;

    //const auto& inlined = info.inlined_functions();
    //SourceIdAssigner id_assigner(info.inlined_functions().size());
    write!(os, "}}")
}

fn json_print_all_source_with_positions(
    os: &mut dyn std::io::Write,
    info: &OptimizedCompilationInfo,
    isolate: *mut Isolate,
) -> Result<(), std::io::Error> {
    write!(os, "\"sources\" : {{")?;
    json_print_function_source(
        os,
        -1,
        "DebugNameCStr".to_string(),
        DirectHandle { ptr: unsafe { std::mem::transmute(1 as usize)}},
        isolate,
        DirectHandle { ptr: unsafe { std::mem::transmute(1 as usize)}},
        true,
    )?;
    write!(os, "}}")
}

fn get_visualizer_log_file_name(
    _info: &OptimizedCompilationInfo,
    optional_base_dir: Option<&str>,
    phase: Option<&str>,
    suffix: &str,
) -> String {
    let file_prefix = "turbo";
    let debug_name = "debug";
    let optimization_id = 0;

    let filename = format!("{}-{}-{}", file_prefix, debug_name, optimization_id);
    let source_available = false;
    let source_file = "source";

    let base_dir = optional_base_dir.map_or("".to_string(), |dir| format!("{}/", dir));

    if phase.is_none() && !source_available {
        format!("{}{}.{}", base_dir, filename, suffix)
    } else if phase.is_some() && !source_available {
        format!("{}{}-{}.{}", base_dir, filename, phase.unwrap(), suffix)
    } else if phase.is_none() && source_available {
        format!("{}{}_{}.{}", base_dir, filename, source_file, suffix)
    } else {
        format!(
            "{}{}_{}-{}.{}",
            base_dir,
            filename,
            source_file,
            phase.unwrap(),
            suffix
        )
    }
}

fn safe_id(node: *mut Node) -> i32 {
    if node.is_null() {
        -1
    } else {
        unsafe { (*node).id() }
    }
}

fn safe_mnemonic(node: *mut Node) -> &'static str {
    if node.is_null() {
        "null"
    } else {
        unsafe { (*node).op().mnemonic() }
    }
}

pub struct JSONGraphWriter<'a> {
    os: &'a mut dyn std::io::Write,
    zone: Option<Rc<Zone>>,
    graph: *const TFGraph,
    positions: *const SourcePositionTable,
    origins: *const NodeOriginTable,
    first_node: bool,
    first_edge: bool,
}

impl<'a> JSONGraphWriter<'a> {
    pub fn new(
        os: &'a mut dyn std::io::Write,
        graph: *const TFGraph,
        positions: *const SourcePositionTable,
        origins: *const NodeOriginTable,
    ) -> Self {
        JSONGraphWriter {
            os: os,
            zone: None,
            graph: graph,
            positions: positions,
            origins: origins,
            first_node: true,
            first_edge: true,
        }
    }

    pub fn print_phase(&mut self, phase_name: &str) -> Result<(), std::io::Error> {
        write!(
            self.os,
            "{{\"name\":\"{}\",\"type\":\"graph\",\"data\":",
            phase_name
        )?;
        self.print()?;
        write!(self.os, "}},\n")
    }

    pub fn print(&mut self) -> Result<(), std::io::Error> {
        let allocator = AccountingAllocator {};
        let tmp_zone = Rc::new(Zone::new("ZONE_NAME".to_string()));
        self.zone = Some(tmp_zone.clone());

        //AllNodes all(zone_, graph_, false);
        //AllNodes live(zone_, graph_, true);

        write!(self.os, "{{\n\"nodes\":[")?;
        //for (Node* const node : all.reachable) PrintNode(node, live.IsLive(node));
        write!(self.os, "\n")?;
        write!(self.os, "],\n\"edges\":[")?;
        //for (Node* const node : all.reachable) PrintEdges(node);
        write!(self.os, "\n")?;
        write!(self.os, "]}}")?;
        self.zone = None;
        Ok(())
    }

    fn print_node(&mut self, _node: *mut Node, _is_live: bool) -> Result<(), std::io::Error> {
        if self.first_node {
            self.first_node = false;
        } else {
            write!(self.os, ",\n")?;
        }
        /*let label = String::new();
        let title = String::new();
        let properties = String::new();
        //node->op()->PrintTo(label, Operator::PrintVerbosity::kSilent);
        //node->op()->PrintTo(title, Operator::PrintVerbosity::kVerbose);
        //node->op()->PrintPropsTo(properties);
        write!(self.os, "{{\"id\":{},\"label\":\"{}\"", safe_id(node), label)?;
        write!(self.os, ",\"title\":\"{}\"", title)?;
        write!(self.os, ",\"live\": {}", if is_live { "true" } else { "false" })?;
        write!(self.os, ",\"properties\":\"{}\"", properties)?;
        //IrOpcode::Value opcode = node->opcode();
        //if (IrOpcode::IsPhiOpcode(opcode)) {
        //    os << ",\"rankInputs\":[0," << NodeProperties::FirstControlIndex(node)
        //       << "]";
        //    os << ",\"rankWithInput\":[" << NodeProperties::FirstControlIndex(node)
        //       << "]";
        //} else if (opcode == IrOpcode::kIfTrue || opcode == IrOpcode::kIfFalse ||
        //           opcode == IrOpcode::kLoop) {
        //    os << ",\"rankInputs\":[" << NodeProperties::FirstControlIndex(node)
        //       << "]";
        //}
        //if (opcode == IrOpcode::kBranch) {
        //    os << ",\"rankInputs\":[0]";
        //}
        //if (positions_ != nullptr) {
        //    SourcePosition position = positions_->GetSourcePosition(node);
        //    if (position.IsKnown()) {
        //        os << ", \"sourcePosition\" : " << AsJSON(position);
        //    }
        //}
        //if (origins_) {
        //    NodeOrigin origin = origins_->GetNodeOrigin(node);
        //    if (origin.IsKnown()) {
        //        os << ", \"origin\" : " << AsJSON(origin);
        //    }
        //}
        //os << ",\"opcode\":\"" << IrOpcode::Mnemonic(node->opcode()) << "\"";
        //os << ",\"control\":"
        //   << (NodeProperties::IsControl(node) ? "true" : "false");
        //os << ",\"opinfo\":\"" << node->op()->ValueInputCount() << " v "
        //   << node->op()->EffectInputCount() << " eff "
        //   << node->op()->ControlInputCount() << " ctrl in, "
        //   << node->op()->ValueOutputCount() << " v "
        //   << node->op()->EffectOutputCount() << " eff "
        //   << node->op()->ControlOutputCount() << " ctrl out\"";
        //if (auto type_opt = GetType(node)) {
        //    std::ostringstream type_out;
        //    type_opt->PrintTo(type_out);
        //    os << ",\"type\":\"" << JSONEscaped(type_out) << "\"";
        //}
        //os << "}";*/
        Ok(())
    }

    fn print_edges(&mut self, _node: *mut Node) -> Result<(), std::io::Error> {
        /*for (int i = 0; i < node->InputCount(); i++) {
            Node* input = node->InputAt(i);
            if (input == nullptr) continue;
            PrintEdge(node, i, input);
        }*/
        Ok(())
    }

    fn print_edge(&mut self, _from: *mut Node, _index: i32, _to: *mut Node) -> Result<(), std::io::Error> {
        if self.first_edge {
            self.first_edge = false;
        } else {
            write!(self.os, ",\n")?;
        }
        /*const char* edge_type = nullptr;
        if (index < NodeProperties::FirstValueIndex(from)) {
            edge_type = "unknown";
        } else if (index < NodeProperties::FirstContextIndex(from)) {
            edge_type = "value";
        } else if (index < NodeProperties::FirstFrameStateIndex(from)) {
            edge_type = "context";
        } else if (index < NodeProperties::FirstEffectIndex(from)) {
            edge_type = "frame-state";
        } else if (index < NodeProperties::FirstControlIndex(from)) {
            edge_type = "effect";
        } else {
            edge_type = "control";
        }
        write!(self.os, "{{\"source\":{},\"target\":{},\"index\":{},\"type\":\"{}\"}}", safe_id(to), safe_id(from), index, edge_type)*/
        Ok(())
    }

    fn get_type(&self, _node: *mut Node) -> Option<Type> {
        /*if (!NodeProperties::IsTyped(node)) return std::nullopt;
        return NodeProperties::GetType(node);*/
        None
    }
}

struct GraphAsJSON {
    graph: TFGraph,
    positions: *mut SourcePositionTable,
    origins: *mut NodeOriginTable,
}

fn as_json_graph(
    graph: &TFGraph,
    positions: *mut SourcePositionTable,
    origins: *mut NodeOriginTable,
) -> GraphAsJSON {
    GraphAsJSON {
        graph: graph.clone(),
        positions: positions,
        origins: origins,
    }
}

impl fmt::Display for GraphAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = Vec::new();
        let mut writer = JSONGraphWriter::new(&mut buffer, &self.graph, self.positions, self.origins);
        writer.print().unwrap();
        let s = String::from_utf8(buffer).unwrap();
        write!(f, "{}", s)
    }
}

struct AsRPO {
    graph: TFGraph,
}

impl fmt::Display for AsRPO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let allocator = AccountingAllocator {};
        let local_zone = Zone::new("ZONE_NAME".to_string());

        let mut state: Vec<u8> = vec![0; 0]; //self.graph.NodeCount()];
        let mut stack: Vec<*mut Node> = Vec::new();

        //stack.push(self.graph.end());
        //state[self.graph.end()->id()] = kOnStack;
        while !stack.is_empty() {
            let _n = stack.last().unwrap();
            let _pop = true;
            /*for (Node* const i : n->inputs()) {
              if (state[i->id()] == kUnvisited) {
                state[i->id()] = kOnStack;
                stack.push(i);
                pop = false;
                break;
              }
            }*/
            /*if (pop) {
              state[n->id()] = kVisited;
              stack.pop();
              os << "#" << n->id() << ":" << *n->op() << "(";
              // Print the inputs.
              int j = 0;
              for (Node* const i : n->inputs()) {
                if (j++ > 0) os << ", ";
                os << "#" << SafeId(i) << ":" << SafeMnemonic(i);
              }
              os << ")";
              // Print the node type, if any.
              if (NodeProperties::IsTyped(n)) {
                os << "  [Type: " << NodeProperties::GetType(n) << "]";
              }
              os << std::endl;
            }*/
        }
        write!(f, "{}", "")
    }
}

struct AsC1VCompilation {
    info: *const OptimizedCompilationInfo,
}

struct AsScheduledGraph {
    schedule: *const Schedule,
}

struct AsC1V {
    phase: &'static str,
    schedule: *const Schedule,
    instructions: *const InstructionSequence,
    positions: *const SourcePositionTable,
}

struct AsC1VRegisterAllocationData {
    phase: &'static str,
    data: *const RegisterAllocationData,
}

impl fmt::Display for AsC1VCompilation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let allocator = AccountingAllocator {};
        let tmp_zone = Zone::new("ZONE_NAME".to_string());
        //GraphC1Visualizer(os, &tmp_zone).PrintCompilation(ac.info_);
        write!(f, "{}", "")
    }
}

impl fmt::Display for AsC1V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let allocator = AccountingAllocator {};
        let tmp_zone = Zone::new("ZONE_NAME".to_string());
        //GraphC1Visualizer(os, &tmp_zone)
        //  .PrintSchedule(ac.phase_, ac.schedule_, ac.positions_, ac.instructions_);
        write!(f, "{}", "")
    }
}

impl fmt::Display for AsC1VRegisterAllocationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let allocator = AccountingAllocator {};
        let tmp_zone = Zone::new("ZONE_NAME".to_string());
        //GraphC1Visualizer(os, &tmp_zone).PrintLiveRanges(ac.phase_, ac.data_);
        write!(f, "{}", "")
    }
}

struct LiveRangeAsJSON {
    range: *const LiveRange,
    code: *const InstructionSequence,
}

struct TopLevelLiveRangeAsJSON {
    range: *const TopLevelLiveRange,
    code: *const InstructionSequence,
}

struct RegisterAllocationDataAsJSON {
    data: *const RegisterAllocationData,
    code: *const InstructionSequence,
}

struct InstructionOperandAsJSON {
    op: *const InstructionOperand,
    code: *const InstructionSequence,
}

struct InstructionAsJSON {
    index: i32,
    instr: *const Instruction,
    code: *const InstructionSequence,
}

struct InstructionBlockAsJSON {
    block: *const InstructionBlock,
    code: *const InstructionSequence,
}

struct InstructionSequenceAsJSON {
    sequence: *const InstructionSequence,
}

impl fmt::Display for AsScheduledGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //PrintScheduledGraph(os, scheduled.schedule);
        write!(f, "{}", "")
    }
}

const K_UNVISITED: u8 = 0;
const K_ON_STACK: u8 = 1;
const K_VISITED: u8 = 2;

fn print_indent(os: &mut dyn std::io::Write, indent: i32) -> Result<(), std::io::Error> {
    write!(os, "     ")?;
    for _i in 0..indent {
        write!(os, ". ")?;
    }
    Ok(())
}

fn print_scheduled_node(
    os: &mut dyn std::io::Write,
    indent: i32,
    node: *mut Node,
) -> Result<(), std::io::Error> {
    print_indent(os, indent)?;
    write!(os, "#{}:", unsafe { (*node).id() })?;
    //<< *n->op() << "(";
    // Print the inputs.
    let mut _j = 0;
    /*for (Node* const i : n->inputs()) {
      if (j++ > 0) os << ", ";
      os << "#" << SafeId(i) << ":" << SafeMnemonic(i);
    }*/
    write!(os, ")")?;
    // Print the node type, if any.
    //if (NodeProperties::IsTyped(n)) {
    //  os << "  [Type: " << NodeProperties::GetType(n) << "]";
    //}
    Ok(())
}

fn print_scheduled_graph(
    os: &mut dyn std::io::Write,
    schedule: *const Schedule,
) -> Result<(), std::io::Error> {
    //const BasicBlockVector* rpo = schedule->rpo_order();
    //for (size_t i = 0; i < rpo->size(); i++) {
    //  BasicBlock* current = (*rpo)[i];
    //  int indent = current->loop_depth();

    //  os << "  + Block B" << current->rpo_number() << " (pred:";
    //  for (BasicBlock* predecessor : current->predecessors()) {
    //    os << " B" << predecessor->rpo_number();
    //  }
    //  if (current->IsLoopHeader()) {
    //    os << ", loop until B" << current->loop_end()->rpo_number();
    //  } else if (current->loop_header()) {
    //    os << ", in loop B" << current->loop_header()->rpo_number();
    //  }
    //  os << ")" << std::endl;

    //  for (BasicBlock::const_iterator it = current->begin(); it != current->end();
    //       ++it) {
    //    Node* node = *it;
    //    PrintScheduledNode(os, indent, node);
    //    os << std::endl;
    //  }

    //  if (current->SuccessorCount() > 0) {
    //    if (current->control_input() != nullptr) {
    //      PrintScheduledNode(os, indent, current->control_input());
    //    } else {
    //      PrintIndent(os, indent);
    //      os << "Goto";
    //    }
    //    os << " ->";

    //    bool isFirst = true;
    //    for (BasicBlock* successor : current->successors()) {
    //      if (isFirst) {
    //        isFirst = false;
    //      } else {
    //        os << ",";
    //      }
    //      os << " B" << successor->rpo_number();
    //    }
    //    os << std::endl;
    //  } else {
    //    DCHECK_NULL(current->control_input());
    //  }
    //}
    Ok(())
}

impl fmt::Display for LiveRangeAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl fmt::Display for TopLevelLiveRangeAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{}}")
    }
}

fn print_top_level_live_ranges(
    os: &mut dyn std::io::Write,
    _ranges: Vec<*mut TopLevelLiveRange>,
    _code: *const InstructionSequence,
) -> Result<(), std::io::Error> {
    write!(os, "{{}}")
}

impl fmt::Display for RegisterAllocationDataAsJSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}


