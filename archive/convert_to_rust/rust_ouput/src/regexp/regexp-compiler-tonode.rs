// Converted from V8 C++ source files:
// Header: N/A
// Implementation: regexp-compiler-tonode.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

//use crate::base::strings;
//use crate::regexp::special_case;
//use icu::locid::Locale;
//use icu::uniset::UnicodeSet;
//use icu::utypes::UErrorCode;

use crate::strings::uri::String;

//use crate::strings::unicode_inl::IsLeadSurrogate;
//use crate::strings::unicode_inl::IsTrailSurrogate;

//use crate::zone::zone_list_inl::ZoneList;

//use crate::ast::ast::Code;
//use crate::ast::ast::RegExpFlags;

//use crate::init::bootstrapper::Isolate;
//use crate::init::bootstrapper::Field;
//use crate::init::bootstrapper::V8;

//use crate::snapshot::deserializer::V8_NODISCARD;
//use crate::snapshot::deserializer::Handles;

//use crate::compiler::backend::ppc::unwinding_info_writer_ppc::flags;

//use crate::compiler::wasm_gc_operator_reducer::NodeId;
//use crate::compiler::wasm_gc_operator_reducer::Node;
//use crate::compiler::wasm_gc_operator_reducer::Operator;
//use crate::compiler::wasm_gc_operator_reducer::Operation;

//use crate::compiler::turboshaft::simplify_tf_loops::Graph;
//use crate::compiler::turboshaft::simplify_tf_loops::Common;

//use crate::logging::code_events::RegExpFlags;

//use crate::torque::ls::message::Range;
//use crate::torque::ls::message::JsonPosition;
//use crate::torque::ls::message::JsonObject;

//use crate::sandbox::external_pointer_table_inl::ExternalPointerTableEntry;

//use crate::execution::isolate::ThreadId;
//use crate::execution::isolate::Maybe;

//use crate::deoptimizer::deoptimizer::Root;

//use crate::objects::string::String;

//use crate::baseline::bytecode_offset_iterator::BytecodeOffsetIterator;

//use crate::asmjs::asm_js::StandardMember;

//use crate::init::isolate_group::Isolate;

//use crate::torque::cpp_builder::Write;

//use crate::ast::ast_value_factory::AstRawString;
//use crate::ast::ast_value_factory::IndirectHandle;

//use crate::ast::ast::Statement;
//use crate::zone::zone_chunk_list::Zone;

//use crate::sandbox::external_entity_table_inl::ExternalEntityTableEntry;

//use crate::sandbox::external_pointer_table::ExternalPointerTable;

//use crate::strings::uri::Key;
//use crate::strings::uri::Value;

//use crate::compiler::backend::mips64::code_generator_mips64::InstructionOperand;

//use crate::sandbox::external_pointer_table_inl::set;

//use crate::strings::string_stream::FmtElm;

//use crate::compiler::backend::mips64::code_generator_mips64::Zone;

//use crate::compiler::backend::mips64::instruction_selector_mips64::OpIndex;

//use crate::compiler::backend::riscv::instruction_selector_riscv::Handle;

//use crate::compiler::turboshaft::wasm_js_lowering_reducer::MachineType;

//use crate::compiler::backend::ppc::instruction_selector_ppc::AtomicMemoryOrder;

//use crate::compiler::backend::mips64::code_generator_mips64::Vec;

//use crate::compiler::backend::riscv::instruction_selector_riscv::InstructionSequence;

//use crate::compiler::backend::arm::code_generator_arm::Zero;

//use crate::sandbox::external_pointer::Address;

//use crate::torque::earley_parser::Item;

//use crate::ast::ast_source_ranges::AstNodeSourceRangesMethods;

//use crate::strings::unicode_decoder::iter;

//use crate::torque::cfg::Block;

//use crate::baseline::ia32::baseline_assembler_ia32_inl::Label;

//use crate::compiler::map_inference::Maybe;

//use crate::baseline::arm::baseline_assembler_arm_inl::Condition;

//use crate::baseline::arm::baseline_assembler_arm_inl::AbortReason;

//use crate::compiler::scheduler::top;

//use crate::strings::string_builder::empty_string;

//use crate::asmjs::asm_parser::check;

//use crate::ast::modules::position;

//use crate::compiler::js_inlining::target;

//use crate::compiler::loop_unrolling::count;

//use crate::sandbox::cppheap_pointer_table_inl::value;

//use crate::ast::ast::AstNode;

//use crate::ast::source_range_ast_visitor::insert;

//use crate::snapshot::references::Insert;

//use crate::compiler::turbofan_types::min;

//use crate::execution::s390::simulator_s390::Length;

//use crate::compiler::backend::jump_threading::forward;

//use crate::torque::torque_code_generator::out;

//use crate::sandbox::external_pointer_table_inl::get;

//use crate::ast::variables::mode;

//use crate::strings::unicode_inl::convert;

//use crate::init::snapshot::Create;

//use crate::compiler::backend::riscv::instruction_selector_riscv::sequence;

//use crate::compiler::wasm-address-reassociation::optimize;

//use crate::baseline::arm::baseline_assembler_arm_inl::Move;

//use crate::init::snapshot::startup-deserializer::with;

//use crate::compiler::backend::mips64::code_generator_mips64::operand;

//use crate::baseline::arm64::baseline-compiler-arm64-inl::Add;

//use crate::init::deserializer::V8_NODISCARD;

//use crate::baseline::arm64::baseline-compiler-arm64-inl::Register;

//use crate::torque::utils::has;

//use crate::objects::tagged::Tagged;

//use crate::sandbox::sandboxed_pointer_inl::current;

//use crate::compiler::turboshaft::growable-stacks-reducer::Returns;

//use crate::include::v8-promise::Local;
//use crate::include::v8-promise::Value;

//use crate::include::v8-template::then;

//use crate::include::v8-script::Compile;

//use crate::compiler::turboshaft::typer::distance;

//use crate::logging::code_events::RegExpFlags;

//use crate::compiler::backend::bitcast_elider::reduce;

//use crate::torque::declarable::Clear;

//use crate::torque::cpp-builder::into;

//use crate::strings::uri::switch;

//use crate::sandbox::external-pointer-table::from;

//use crate::include::v8-object::Wrap;

//use crate::strings::uri::first;
//use crate::strings::uri::second;

//use crate::torque::earley_parser::children;

//use crate::ast::ast::If;

//use crate::strings::unicode_inl::is;

//use crate::torque::ls::message::range;

//use crate::strings::uri::source;

//use crate::zone::zone_compact_set::singleton;

//use crate::compiler::backend::instruction_selector_adapter::block;

//use crate::regexp::regexp_macro_assembler_arch::Assembler;

//use crate::include::v8-fast-api-calls::make;

//use crate::baseline::arm::baseline-assembler-arm-inl::b;

//use crate::compiler::backend::mips64::code_generator_mips64::uses;

//use crate::compiler::backend::mips64::instruction_selector_mips64::matches;

//use crate::regexp::regexp_bytecodes::code;

//use crate::strings::uri::this;
//use crate::strings::uri::not;

//use crate::flags::flags_impl::comment;

//use crate::sandbox::external-entity-table-inl::index;

//use crate::compiler::turbofan_types::Empty;

//use crate::sandbox::sandbox::Reset;
//use crate::sandbox::sandbox::end;

//use crate::compiler::backend::mips64::code-generator-mips64::instance;

//use crate::torque::utils::has;

//use crate::strings::string_stream::clear;

//use crate::include::v8-profiler::filter;

//use crate::sandbox::sandbox::current;

//use crate::torque::torque-code-generator::out;

//use crate::torque::earley_parser::pos;

//use crate::strings::uri::character;

//use crate::sandbox::external-entity-table-inl::length;

//use crate::strings::unicode_decoder::iter;

//use crate::torque::cfg::blocks;

//use crate::torque::utils::has;

//use crate::baseline::ia32::baseline_assembler_ia32_inl::j;

//use crate::strings::unicode_decoder::is;

//use crate::flags::flags_impl::flags;

//use crate::ast::ast::CallType;

//use crate::strings::string-stream::add;

//use crate::ast::ast-source-ranges::find;

//use crate::strings::uri::Use;

//use crate::strings::uri::bool;

//use crate::sandbox::external-entity-table-inl::number;

//use crate::init::bootstrapper::exception;

//use crate::flags::flags_impl::flags;

//use crate::strings::string-stream::elements;

//use crate::compiler::turboshaft::simplify_tf_loops::graph;

//use crate::compiler::backend::mips64::code_generator_mips64::operand;

//use crate::ast::modules::position;

//use crate::ast::ast_source_ranges::range_count;

//use crate::baseline::arm::baseline-assembler-arm-inl::register_count;

//use crate::compiler::turbofan-types::max;
//use crate::compiler::turbofan-types::min;

//use crate::ast::ast_source_ranges::empty;

//use crate::strings::unicode-inl::is;

//use crate::ast::source_range_ast_visitor::body;

//use crate::regexp::regexp-bytecodes::code;

mod regexp_compiler_constants {
    pub const kRangeEndMarker: i32 = -1;
}

use regexp_compiler_constants::*;

const kMaxCodePoint: u32 = 0x10ffff;
const kMaxUtf16CodeUnit: i32 = 0xffff;
const kMaxUtf16CodeUnitU: u32 = 0xffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TextElement {
    element_type: TextElementType,
    atom: *mut RegExpAtom,
}

impl TextElement {
    fn Atom(atom: *mut RegExpAtom) -> Self {
        TextElement {
            element_type: TextElementType::kAtom,
            atom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TextElementType {
    kAtom,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CharacterRange {
    from_: u32,
    to_: u32,
}

impl CharacterRange {
    fn Range(from_: u32, to_: u32) -> Self {
        CharacterRange { from_: from_, to_: to_ }
    }

    fn Singleton(c: u16) -> Self {
        CharacterRange {
            from_: c as u32,
            to_: c as u32,
        }
    }

    fn Everything() -> Self {
        CharacterRange {
            from_: 0,
            to_: kMaxCodePoint,
        }
    }

    fn from(&self) -> u32 {
        self.from_
    }

    fn to(&self) -> u32 {
        self.to_
    }

    fn IsEverything(&self, max_code_point: u32) -> bool {
        self.from_ == 0 && self.to_ == max_code_point
    }

    fn Canonicalize(_ranges: &mut Vec<CharacterRange>) {}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ZoneList<T> {
    elements: Vec<T>,
}

impl<T> ZoneList<T> {
    fn new(capacity: usize) -> Self {
        ZoneList {
            elements: Vec::with_capacity(capacity),
        }
    }

    fn Add(&mut self, element: T) {
        self.elements.push(element);
    }

    fn AddAll(&mut self, other: &ZoneList<T>) {
        self.elements.extend(other.elements.clone()); // Consider using extend_from_slice if T is Copy
    }

    fn at(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn length(&self) -> usize {
        self.elements.len()
    }

    fn first(&self) -> &T {
        &self.elements[0]
    }

    fn Clear(&mut self) {
        self.elements.clear();
    }

    fn Rewind(&mut self, new_length: usize) {
        self.elements.truncate(new_length);
    }
}

#[derive(Debug, Clone)]
struct RegExpNode {}

#[derive(Debug, Clone)]
struct RegExpAtom {
    data: String,
}

impl RegExpAtom {
    fn new(data: String) -> Self {
        RegExpAtom { data }
    }

    fn length(&self) -> usize {
        self.data.len()
    }

    fn AsAtom(&self) -> &RegExpAtom {
        self
    }

    fn data(&self) -> &String {
        &self.data
    }
}

#[derive(Debug, Clone)]
struct RegExpText {
    elements: Vec<TextElement>,
    read_backward: bool,
}

impl RegExpText {
    fn new(elements: Vec<TextElement>, read_backward: bool) -> Self {
        RegExpText {
            elements,
            read_backward,
        }
    }

    fn elements(&self) -> &Vec<TextElement> {
        &self.elements
    }
}

#[derive(Debug, Clone)]
struct TextNode {
    text: *mut RegExpClassRanges, // Assuming this is what's intended here
    read_backward: bool,
    on_success: *mut RegExpNode,
}

impl TextNode {
  
    fn CreateForCharacterRanges(_zone: &mut Zone, _bmp: &mut Vec<CharacterRange>, _read_backward: bool, _on_success: *mut RegExpNode) -> *mut TextNode {
         todo!()
    }
    
    fn CreateForSurrogatePair(_zone: &mut Zone, _leading_range: CharacterRange, _trailing_ranges: &mut Vec<CharacterRange>, _read_backward: bool, _on_success: *mut RegExpNode) -> *mut TextNode {
         todo!()
    }
    
}

#[derive(Debug, Clone)]
struct RegExpCompiler {
    flags: RegExpFlags,
    read_backward: bool,
    current_expansion_factor: i32,
}

impl RegExpCompiler {
    fn zone(&mut self) -> &mut Zone {
        todo!()
    }
    fn read_backward(&self) -> bool {
        self.read_backward
    }

    fn set_read_backward(&mut self, read_backward: bool) {
        self.read_backward = read_backward;
    }

    fn AllocateRegister(&mut self) -> i32 {
        0 // Placeholder
    }

    fn optimize(&self) -> bool {
        true // Placeholder
    }

    fn flags(&self) -> RegExpFlags {
        self.flags
    }

    fn set_flags(&mut self, flags: RegExpFlags) {
        self.flags = flags;
    }

    fn current_expansion_factor(&self) -> i32 {
        self.current_expansion_factor
    }

    fn set_current_expansion_factor(&mut self, factor: i32) {
        self.current_expansion_factor = factor;
    }

    fn ToNodeMaybeCheckForStackOverflow(&mut self) {}

    fn UnicodeLookaroundStackRegister(&mut self) -> i32 {0}
    fn UnicodeLookaroundPositionRegister(&mut self) -> i32 {0}
}

#[derive(Debug, Clone)]
struct ChoiceNode {
    alternatives: Vec<GuardedAlternative>,
}

impl ChoiceNode {
    fn AddAlternative(&mut self, alternative: GuardedAlternative) {
        self.alternatives.push(alternative);
    }
    fn SetDoNotInline(&mut self) {}
}

#[derive(Debug, Clone)]
struct GuardedAlternative {
    node: *mut RegExpNode,
}

impl GuardedAlternative {
    fn new(node: *mut RegExpNode) -> Self {
        GuardedAlternative { node }
    }
    fn AddGuard(&mut self, _body_guard: *mut Guard, _zone: &mut Zone) {}
}

#[derive(Debug, Clone)]
struct CharacterRangeVector {
    ranges: Vec<CharacterRange>,
}

impl CharacterRangeVector {
    fn new() -> Self {
        CharacterRangeVector { ranges: Vec::new() }
    }
    fn emplace_back(&mut self, range: CharacterRange) {
        self.ranges.push(range);
    }
    fn empty(&self) -> bool {
        self.ranges.is_empty()
    }
    fn at(&self, index: usize) -> &CharacterRange {
        &self.ranges[index]
    }
    fn push(&mut self, _bmp: CharacterRange) {}
}

struct UnicodeRangeSplitter {
    bmp_: CharacterRangeVector,
    lead_surrogates_: CharacterRangeVector,
    trail_surrogates_: CharacterRangeVector,
    non_bmp_: CharacterRangeVector,
}

impl UnicodeRangeSplitter {
    fn new(_base: &mut Vec<CharacterRange>) -> Self {
        UnicodeRangeSplitter {
            bmp_: CharacterRangeVector::new(),
            lead_surrogates_: CharacterRangeVector::new(),
            trail_surrogates_: CharacterRangeVector::new(),
            non_bmp_: CharacterRangeVector::new(),
        }
    }

    fn AddRange(&mut self, range: CharacterRange) {
        static constexpr_uc32 kBmp1Start: u32 = 0;
        static constexpr_uc32 kBmp1End: u32 = kLeadSurrogateStart - 1;
        static constexpr_uc32 kBmp2Start: u32 = kTrailSurrogateEnd + 1;
        static constexpr_uc32 kBmp2End: u32 = kNonBmpStart - 1;

        static constexpr_uc32 kStarts: [u32; 5] = [
            kBmp1Start,
            kLeadSurrogateStart,
            kTrailSurrogateStart,
            kBmp2Start,
            kNonBmpStart,
        ];

        static constexpr_uc32 kEnds: [u32; 5] = [
            kBmp1End,
            kLeadSurrogateEnd,
            kTrailSurrogateEnd,
            kBmp2End,
            kNonBmpEnd,
        ];

        let kTargets: [*mut CharacterRangeVector; 5] = [
            &mut self.bmp_,
            &mut self.lead_surrogates_,
            &mut self.trail_surrogates_,
            &mut self.bmp_,
            &mut self.non_bmp_,
        ];

        static constexpr_int kCount: usize = 5;
      
        for i in 0..kCount {
            if kStarts[i] > range.to() {
                break;
            }
            let from = std::cmp::max(kStarts[i], range.from());
            let to = std::cmp::min(kEnds[i], range.to());
            if from > to {
                continue;
            }
            unsafe {
              (&mut *kTargets[i]).emplace_back(CharacterRange::Range(from, to));
            }
        }
    }

    fn bmp(&mut self) -> &mut CharacterRangeVector {
        &mut self.bmp_
    }

    fn lead_surrogates(&mut self) -> &mut CharacterRangeVector {
        &mut self.lead_surrogates_
    }

    fn trail_surrogates(&mut self) -> &mut CharacterRangeVector {
        &mut self.trail_surrogates_
    }

    fn non_bmp(&mut self) -> &mut CharacterRangeVector {
        &mut self.non_bmp_
    }
}

struct RegExpClassRanges {
  set_: CharacterSet,
  is_negated_: bool,
  standard_type_: StandardCharacterSet,
}

impl RegExpClassRanges {
    const IS_CASE_FOLDED: RegExpClassRangesFlags = 1;
  
    fn ToNode(&mut self, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
}

type RegExpClassRangesFlags = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StandardCharacterSet {
    kWhitespace,
    kNotWhitespace,
    kWord,
    kNotWord,
    kDigit,
    kNotDigit,
    kLineTerminator,
    kNotLineTerminator,
    kEverything,
}

impl From<u32> for StandardCharacterSet {
    fn from(value: u32) -> Self {
        match value {
            0 => StandardCharacterSet::kWhitespace,
            1 => StandardCharacterSet::kNotWhitespace,
            2 => StandardCharacterSet::kWord,
            3 => StandardCharacterSet::kNotWord,
            4 => StandardCharacterSet::kDigit,
            5 => StandardCharacterSet::kNotDigit,
            6 => StandardCharacterSet::kLineTerminator,
            7 => StandardCharacterSet::kNotLineTerminator,
            8 => StandardCharacterSet::kEverything,
            _ => panic!("Invalid StandardCharacterSet value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CharacterSet {
    standard_set_type_: Option<StandardCharacterSet>,
    ranges_: *mut ZoneList<CharacterRange>,
}

impl CharacterSet {
    fn is_standard(&self) -> bool {
        self.standard_set_type_.is_some()
    }

    fn set_standard_set_type(&mut self, standard_set_type: StandardCharacterSet) {
        self.standard_set_type_ = Some(standard_set_type);
    }

    fn ranges(&mut self, _zone: &mut Zone) -> *mut ZoneList<CharacterRange> {
        todo!()
    }

    fn Canonicalize(&mut self) {}
}

struct RegExpClassSetOperand {
    strings_: *mut CharacterClassStrings,
    ranges_: *mut ZoneList<CharacterRange>,
}

impl RegExpClassSetOperand {
    fn Union(&mut self, _other: &mut RegExpClassSetOperand, _zone: &mut Zone) {}
    fn Intersect(&mut self, _other: &mut RegExpClassSetOperand, _temp_ranges: *mut ZoneList<CharacterRange>, _zone: &mut Zone) {}
    fn Subtract(&mut self, _other: &mut RegExpClassSetOperand, _temp_ranges: *mut ZoneList<CharacterRange>, _zone: &mut Zone) {}
    fn ComputeExpression(_this: &mut RegExpClassSetExpression, _temp_ranges: *mut ZoneList<CharacterRange>, _zone: &mut Zone) -> *mut RegExpClassSetOperand {
        todo!()
    }
    fn ToNode(&self, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn is_negated(&self) -> bool {
        false
    }
    fn has_strings(&self) -> bool {
        todo!()
    }
    fn strings(&mut self) -> *mut CharacterClassStrings {
        todo!()
    }
    fn ranges(&mut self) -> *mut ZoneList<CharacterRange> {
        todo!()
    }
    fn AsClassSetOperand(&self) -> &RegExpClassSetOperand {
        self
    }
}

struct RegExpClassSetExpression {
    operands_: *mut ZoneList<RegExpTree>,
    operation_: OperationType,
    is_negated_: bool,
}

impl RegExpClassSetExpression {
    fn ToNode(&mut self, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn operands(&mut self) -> *mut ZoneList<RegExpTree> {
        todo!()
    }
    fn operation(&self) -> &OperationType {
        todo!()
    }
    fn IsClassSetOperand(&self) -> bool {
        false
    }
    fn IsClassSetExpression(&self) -> bool {
        true
    }
    fn AsClassSetExpression(&self) -> &RegExpClassSetExpression {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OperationType {
    kUnion,
    kIntersection,
    kSubtraction,
}

struct CharacterClassStrings {}

struct RegExpDisjunction {}

impl RegExpDisjunction {
    fn new(_alternatives: *mut ZoneList<RegExpTree>) -> Self {
         RegExpDisjunction {}
    }
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn alternatives(&mut self) -> *mut ZoneList<RegExpTree> {
        todo!()
    }
}

struct RegExpTree {}
impl RegExpTree {
    fn IsAtom(&self) -> bool {
        false
    }
    fn IsClassSetOperand(&self) -> bool {
        false
    }
    fn IsClassSetExpression(&self) -> bool {
        false
    }
    fn ToNode(&self, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn AsAtom(&self) -> &RegExpAtom {
        todo!()
    }
    fn CaptureRegisters(&self) -> Interval {
         Interval{}
    }
    fn min_match(&self) -> i32 {
         0
    }
    fn AsAssertion(&self) -> *mut RegExpAssertion {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Interval {}
impl Interval {
    fn is_empty(&self) -> bool {
         false
    }
}

struct RegExpQuantifier {}
impl RegExpQuantifier {
    fn ToNode(_min: i32, _max: i32, _is_greedy: bool, _body: *mut RegExpTree, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode, _not_at_start: bool) -> *mut RegExpNode {
        todo!()
    }
    fn min(&self) -> i32 {
         0
    }
    fn max(&self) -> i32 {
         0
    }
    fn is_greedy(&self) -> bool {
         false
    }
    fn body(&self) -> *mut RegExpTree {
         todo!()
    }
}

struct RegExpAlternative {
    nodes_: *mut ZoneList<RegExpTree>,
}
impl RegExpAlternative {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn nodes(&mut self) -> *mut ZoneList<RegExpTree> {
         todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    START_OF_LINE,
    START_OF_INPUT,
    BOUNDARY,
    NON_BOUNDARY,
    END_OF_INPUT,
    END_OF_LINE,
}

struct RegExpAssertion {
    assertion_type_: Type,
}

impl RegExpAssertion {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn assertion_type(&self) -> Type {
         Type::START_OF_LINE
    }
}

struct BackReferenceNode {}
struct ActionNode {}
impl ActionNode {
    fn PositiveSubmatchSuccess(_stack_pointer_register: i32, _position_register: i32, _capture_register_count: i32, _capture_register_start: i32, _on_success: *mut RegExpNode) -> *mut ActionNode {
         todo!()
    }
    fn BeginPositiveSubmatch(_stack_pointer_register: i32, _position_register: i32, _match: *mut RegExpNode, _on_match_success: *mut ActionNode) -> *mut RegExpNode {
         todo!()
    }
    fn ModifyFlags(_flags: RegExpFlags, _on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn StorePosition(_end_reg: i32, _b: bool, _on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn IncrementRegister(_reg_ctr: i32, _center: *mut LoopChoiceNode) -> *mut ActionNode {
        todo!()
    }
    fn EmptyMatchCheck(_body_start_reg: i32, _reg_ctr: i32, _min: i32, _loop_return: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn ClearCaptures(_capture_registers: Interval, _body_node: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn SetRegisterForLoop(_reg_ctr: i32, _i: i32, _center: *mut LoopChoiceNode) -> *mut RegExpNode {
        todo!()
    }
    fn AsActionNode(&self) -> &ActionNode {
        todo!()
    }
}

struct RegExpCapture {}
impl RegExpCapture {
    fn ToNode(_body: *mut RegExpTree, _index: i32, _compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn StartRegister(_index: i32) -> i32 {
         0
    }
    fn EndRegister(_index: i32) -> i32 {
         0
    }
}

struct RegExpBackReference {
    captures_: *mut Vec<*mut RegExpCapture>,
}
impl RegExpBackReference {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn captures(&mut self) -> *mut Vec<*mut RegExpCapture> {
         todo!()
    }
}

struct RegExpEmpty {}
impl RegExpEmpty {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        _on_success
    }
}

struct RegExpGroup {
    flags_: RegExpFlags,
    body_: *mut RegExpTree,
}
impl RegExpGroup {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn flags(&self) -> RegExpFlags {
        self.flags_
    }
    fn body(&self) -> *mut RegExpTree {
         self.body_
    }
}

struct RegExpLookaround {}
impl RegExpLookaround {
    fn ToNode(_compiler: &mut RegExpCompiler, _on_success: *mut RegExpNode) -> *mut RegExpNode {
        todo!()
    }
    fn type(&self) -> i32 {
         0
    }
    fn is_positive(&self) -> bool {
         false
    }
    fn body(&self) -> *mut RegExpTree {
         todo!()
    }
}

struct LoopChoiceNode {}
impl LoopChoiceNode {
    fn new(_b: bool, _read_backward: bool, _min: i32, _zone: &mut Zone) -> *mut LoopChoiceNode {
         todo!()
    }
    fn AddContinueAlternative(&mut self, _rest_alt: GuardedAlternative) {}
    fn AddLoopAlternative(&mut self, _body_alt: GuardedAlternative) {}
    fn set_not_at_start(&mut self) {}
}

struct AssertionNode {}
impl AssertionNode {
    fn AfterNewline(_on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn AtStart(_on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn AtBoundary(_on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn AtNonBoundary(_on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
    fn AtEnd(_on_success: *mut RegExpNode) -> *mut RegExpNode {
         todo!()
    }
}

struct Guard {}

struct NegativeSubmatchSuccess {}
struct NegativeLookaroundChoiceNode {}
