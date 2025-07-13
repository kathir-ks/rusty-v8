// Converted from V8 C++ source files:
// Header: regexp-dotprinter.h
// Implementation: regexp-dotprinter.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod regexp_dotprinter {
    use crate::ast::ast::CallType;
    use crate::compiler::node::StdoutStream;
    use crate::regexp::regexp_compiler_tonode::NegativeLookaroundChoiceNode;
    use crate::strings::uri::{V8, void};

    pub struct AllStatic {}

    pub struct RegExpNodeInfo {
        pub visited: bool,
        pub follows_newline_interest: bool,
        pub follows_word_interest: bool,
        pub follows_start_interest: bool,
    }

    impl RegExpNodeInfo {
        pub fn new() -> Self {
            RegExpNodeInfo {
                visited: false,
                follows_newline_interest: false,
                follows_word_interest: false,
                follows_start_interest: false,
            }
        }
    }

    pub struct Label {
        pos: i32,
        is_bound: bool,
    }

    impl Label {
        pub fn new() -> Self {
            Label {
                pos: -1,
                is_bound: false,
            }
        }

        pub fn bind(&mut self, pos: i32) {
            self.pos = pos;
            self.is_bound = true;
        }

        pub fn pos(&self) -> i32 {
            self.pos
        }

        pub fn is_bound(&self) -> bool {
            self.is_bound
        }
    }

    pub struct RegExpNode {
        info: RegExpNodeInfo,
        label: Label,
    }

    impl RegExpNode {
        pub fn new() -> Self {
            RegExpNode {
                info: RegExpNodeInfo::new(),
                label: Label::new(),
            }
        }

        pub fn info(&mut self) -> &mut RegExpNodeInfo {
            &mut self.info
        }

        pub fn label(&mut self) -> &mut Label {
            &mut self.label
        }

        pub fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
            visitor.visit(self);
        }
    }

    pub trait NodeVisitor {
        fn visit(&mut self, node: &mut RegExpNode);
        fn visit_choice(&mut self, that: &mut ChoiceNode);
        fn visit_loop_choice(&mut self, that: &mut LoopChoiceNode);
        fn visit_negative_lookaround_choice(&mut self, that: &mut NegativeLookaroundChoiceNode);
        fn visit_text(&mut self, that: &mut TextNode);
        fn visit_back_reference(&mut self, that: &mut BackReferenceNode);
        fn visit_end(&mut self, that: &mut EndNode);
        fn visit_assertion(&mut self, that: &mut AssertionNode);
        fn visit_action(&mut self, that: &mut ActionNode);
    }

    pub struct ChoiceNode {
        alternatives: Vec<GuardedAlternative>,
    }

    impl ChoiceNode {
        pub fn new(alternatives: Vec<GuardedAlternative>) -> Self {
            ChoiceNode { alternatives }
        }

        pub fn alternatives(&mut self) -> &mut Vec<GuardedAlternative> {
            &mut self.alternatives
        }
    }

    pub struct LoopChoiceNode {
        choice_node: ChoiceNode,
    }

    impl LoopChoiceNode {
        pub fn new(alternatives: Vec<GuardedAlternative>) -> Self {
            LoopChoiceNode {
                choice_node: ChoiceNode::new(alternatives),
            }
        }
    }

    pub struct GuardedAlternative {
        node: Box<RegExpNode>,
    }

    impl GuardedAlternative {
        pub fn new(node: Box<RegExpNode>) -> Self {
            GuardedAlternative { node }
        }

        pub fn node(&mut self) -> &mut RegExpNode {
            self.node.as_mut()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TextElementType {
        ATOM,
        CLASS_RANGES,
    }

    pub struct TextElement {
        text_type: TextElementType,
        atom: Option<String>,
        class_ranges: Option<RegExpClassRanges>,
    }

    impl TextElement {
        pub fn new_atom(atom: String) -> Self {
            TextElement {
                text_type: TextElementType::ATOM,
                atom: Some(atom),
                class_ranges: None,
            }
        }

        pub fn new_class_ranges(class_ranges: RegExpClassRanges) -> Self {
            TextElement {
                text_type: TextElementType::CLASS_RANGES,
                atom: None,
                class_ranges: Some(class_ranges),
            }
        }

        pub fn text_type(&self) -> TextElementType {
            self.text_type
        }

        pub fn atom(&self) -> Option<&String> {
            self.atom.as_ref()
        }

        pub fn class_ranges(&self) -> Option<&RegExpClassRanges> {
            self.class_ranges.as_ref()
        }
    }

    pub struct RegExpClassRanges {
        is_negated: bool,
        ranges: Vec<CharacterRange>,
    }

    impl RegExpClassRanges {
        pub fn new(is_negated: bool, ranges: Vec<CharacterRange>) -> Self {
            RegExpClassRanges { is_negated, ranges }
        }

        pub fn is_negated(&self) -> bool {
            self.is_negated
        }

        pub fn ranges(&self) -> &Vec<CharacterRange> {
            &self.ranges
        }
    }

    #[derive(Clone, Copy)]
    pub struct CharacterRange {
        from: u32,
        to: u32,
    }

    impl CharacterRange {
        pub fn new(from: u32, to: u32) -> Self {
            CharacterRange { from, to }
        }

        pub fn from(&self) -> u32 {
            self.from
        }

        pub fn to(&self) -> u32 {
            self.to
        }
    }

    pub struct TextNode {
        elements: Vec<TextElement>,
        on_success: Box<RegExpNode>,
    }

    impl TextNode {
        pub fn new(elements: Vec<TextElement>, on_success: Box<RegExpNode>) -> Self {
            TextNode { elements, on_success }
        }

        pub fn elements(&mut self) -> &mut Vec<TextElement> {
            &mut self.elements
        }

        pub fn on_success(&mut self) -> &mut RegExpNode {
            self.on_success.as_mut()
        }
    }

    pub struct BackReferenceNode {
        start_register: i32,
        end_register: i32,
        on_success: Box<RegExpNode>,
    }

    impl BackReferenceNode {
        pub fn new(start_register: i32, end_register: i32, on_success: Box<RegExpNode>) -> Self {
            BackReferenceNode {
                start_register,
                end_register,
                on_success,
            }
        }

        pub fn start_register(&self) -> i32 {
            self.start_register
        }

        pub fn end_register(&self) -> i32 {
            self.end_register
        }

        pub fn on_success(&mut self) -> &mut RegExpNode {
            self.on_success.as_mut()
        }
    }

    pub struct EndNode {}

    impl EndNode {
        pub fn new() -> Self {
            EndNode {}
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum AssertionType {
        AT_END,
        AT_START,
        AT_BOUNDARY,
        AT_NON_BOUNDARY,
        AFTER_NEWLINE,
    }

    pub struct AssertionNode {
        assertion_type: AssertionType,
        on_success: Box<RegExpNode>,
    }

    impl AssertionNode {
        pub fn new(assertion_type: AssertionType, on_success: Box<RegExpNode>) -> Self {
            AssertionNode {
                assertion_type,
                on_success,
            }
        }

        pub fn assertion_type(&self) -> AssertionType {
            self.assertion_type
        }

        pub fn on_success(&mut self) -> &mut RegExpNode {
            self.on_success.as_mut()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ActionType {
        SET_REGISTER_FOR_LOOP,
        INCREMENT_REGISTER,
        STORE_POSITION,
        BEGIN_POSITIVE_SUBMATCH,
        BEGIN_NEGATIVE_SUBMATCH,
        POSITIVE_SUBMATCH_SUCCESS,
        EMPTY_MATCH_CHECK,
        CLEAR_CAPTURES,
        MODIFY_FLAGS,
    }

    pub struct ActionNode {
        action_type_: ActionType,
        data_: ActionData,
        on_success: Box<RegExpNode>,
        flags_: i32,
    }

    impl ActionNode {
        pub fn new(action_type_: ActionType, data_: ActionData, on_success: Box<RegExpNode>, flags_: i32) -> Self {
            ActionNode {
                action_type_,
                data_,
                on_success,
                flags_: flags_,
            }
        }

        pub fn action_type(&self) -> ActionType {
            self.action_type_
        }

        pub fn data(&self) -> &ActionData {
            &self.data_
        }

        pub fn on_success(&mut self) -> &mut RegExpNode {
            self.on_success.as_mut()
        }
        pub fn flags(&self) -> i32 {
            self.flags_
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub union ActionData {
        u_store_register: StoreRegister,
        u_increment_register: IncrementRegister,
        u_position_register: PositionRegister,
        u_submatch: Submatch,
        u_empty_match_check: EmptyMatchCheck,
        u_clear_captures: ClearCaptures,
    }

    impl ActionData {
        pub fn new() -> Self {
            ActionData {
                u_store_register: StoreRegister { reg: 0, value: 0 },
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct StoreRegister {
        pub reg: i32,
        pub value: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct IncrementRegister {
        pub reg: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct PositionRegister {
        pub reg: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Submatch {
        pub current_position_register: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct EmptyMatchCheck {
        pub start_register: i32,
        pub repetition_register: i32,
        pub repetition_limit: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ClearCaptures {
        pub range_from: i32,
        pub range_to: i32,
    }

    pub struct DotPrinterImpl {
        os_: StdoutStream,
    }

    impl DotPrinterImpl {
        pub fn new(os: StdoutStream) -> Self {
            DotPrinterImpl { os_: os }
        }

        pub fn print_node(&mut self, label: &str, node: &mut RegExpNode) {
            self.os_.write_str("digraph G {\n").unwrap();
            self.os_.write_str("  graph [label=\"").unwrap();
            for c in label.chars() {
                match c {
                    '\\' => self.os_.write_str("\\\\").unwrap(),
                    '"' => self.os_.write_str("\\\"").unwrap(),
                    _ => self.os_.write_char(c).unwrap(),
                }
            }
            self.os_.write_str("\"];\n").unwrap();
            self.visit(node);
            self.os_.write_str("}\n").unwrap();
        }

        fn print_on_failure(&mut self, from: *const RegExpNode, on_failure: *const RegExpNode) {
            self.os_
                .write_str(&format!("  n{:p} -> n{:p} [style=dotted];\n", from, on_failure))
                .unwrap();

             unsafe {
                let on_failure_node = &mut *(on_failure as *mut RegExpNode);
                 self.visit(on_failure_node);
             }
        }


        fn print_attributes(&mut self, that: *const RegExpNode) {
            unsafe {
                let that_node = &mut *(that as *mut RegExpNode);
                self.os_
                    .write_str(&format!(
                        "  a{:p} [shape=Mrecord, color=grey, fontcolor=grey, margin=0.1, fontsize=10, label=\"{{",
                        that
                    ))
                    .unwrap();
                let mut printer = AttributePrinter::new(&mut self.os_);
                let info = that_node.info();
                printer.print_bit("NI", info.follows_newline_interest);
                printer.print_bit("WI", info.follows_word_interest);
                printer.print_bit("SI", info.follows_start_interest);
                let label = that_node.label();
                if label.is_bound() {
                    printer.print_positive("@", label.pos());
                }
                self.os_
                    .write_str(&format!(
                        "}}\"];\n  a{:p} -> n{:p} [style=dashed, color=grey, arrowhead=none];\n",
                        that, that
                    ))
                    .unwrap();
            }
        }

    }

    impl NodeVisitor for DotPrinterImpl {
        fn visit(&mut self, node: &mut RegExpNode) {
            if node.info().visited {
                return;
            }
            node.info().visited = true;
            node.accept(self);
        }

        fn visit_choice(&mut self, that: &mut ChoiceNode) {
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" [shape=Mrecord, label=\"?\"];\n").unwrap();
            for alt in that.alternatives().iter_mut() {
                self.os_.write_str("  n").unwrap();
                // self.os_.write_str(&format!("{:p}", that)).unwrap();
                self.os_.write_str(" -> n").unwrap();
                // self.os_.write_str(&format!("{:p}", alt.node())).unwrap();
                self.os_.write_str(";\n").unwrap();
            }
            for alt in that.alternatives().iter_mut() {
                self.visit(alt.node());
            }
            // self.print_attributes(that);
        }

        fn visit_loop_choice(&mut self, that: &mut LoopChoiceNode) {
            self.visit_choice(&mut that.choice_node);
        }

        fn visit_negative_lookaround_choice(&mut self, that: &mut NegativeLookaroundChoiceNode) {
            // self.visit_choice(that);
        }

        fn visit_text(&mut self, that: &mut TextNode) {
             self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" [label=\"").unwrap();
            for (i, elm) in that.elements().iter_mut().enumerate() {
                if i > 0 {
                    self.os_.write_str(" ").unwrap();
                }
                match elm.text_type() {
                    TextElementType::ATOM => {
                        if let Some(atom) = elm.atom() {
                            for c in atom.chars() {
                                self.os_.write_char(c).unwrap();
                            }
                        }
                    }
                    TextElementType::CLASS_RANGES => {
                        if let Some(node) = elm.class_ranges() {
                            self.os_.write_str("[").unwrap();
                            if node.is_negated() {
                                self.os_.write_str("^").unwrap();
                            }
                            for range in node.ranges().iter() {
                                self.os_.write_str(&format!("{}-{}", range.from(), range.to())).unwrap();
                            }
                            self.os_.write_str("]").unwrap();
                        }
                    }
                }
            }
            self.os_.write_str("\", shape=box, peripheries=2];\n").unwrap();
            // self.print_attributes(that);
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" -> n").unwrap();
            // self.os_.write_str(&format!("{:p}", that.on_success())).unwrap();
            self.os_.write_str(";\n").unwrap();
            self.visit(that.on_success());
        }

        fn visit_back_reference(&mut self, that: &mut BackReferenceNode) {
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(&format!(" [label=\"${}..${}\", shape=doubleoctagon];\n", that.start_register(), that.end_register())).unwrap();
            // self.print_attributes(that);
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" -> n").unwrap();
            // self.os_.write_str(&format!("{:p}", that.on_success())).unwrap();
            self.os_.write_str(";\n").unwrap();
            self.visit(that.on_success());
        }

        fn visit_end(&mut self, that: &mut EndNode) {
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" [style=bold, shape=point];\n").unwrap();
            // self.print_attributes(that);
        }

        fn visit_assertion(&mut self, that: &mut AssertionNode) {
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" [").unwrap();
            match that.assertion_type() {
                AssertionType::AT_END => {
                    self.os_.write_str("label=\"$\", shape=septagon").unwrap();
                }
                AssertionType::AT_START => {
                    self.os_.write_str("label=\"^\", shape=septagon").unwrap();
                }
                AssertionType::AT_BOUNDARY => {
                    self.os_.write_str("label=\"\\b\", shape=septagon").unwrap();
                }
                AssertionType::AT_NON_BOUNDARY => {
                    self.os_.write_str("label=\"\\B\", shape=septagon").unwrap();
                }
                AssertionType::AFTER_NEWLINE => {
                    self.os_.write_str("label=\"(?<=\\n>)\", shape=septagon").unwrap();
                }
            }
            self.os_.write_str("];\n").unwrap();
            // self.print_attributes(that);
            let successor = that.on_success();
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" -> n").unwrap();
            // self.os_.write_str(&format!("{:p}", successor)).unwrap();
            self.os_.write_str(";\n").unwrap();
            self.visit(successor);
        }

        fn visit_action(&mut self, that: &mut ActionNode) {
             self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" [").unwrap();
            match that.action_type() {
                ActionType::SET_REGISTER_FOR_LOOP => {
                     self.os_.write_str(&format!("label=\"${}:={}\", shape=octagon", that.data().u_store_register.reg, that.data().u_store_register.value)).unwrap();
                }
                ActionType::INCREMENT_REGISTER => {
                     self.os_.write_str(&format!("label=\"${}++\", shape=octagon", that.data().u_increment_register.reg)).unwrap();
                }
                ActionType::STORE_POSITION => {
                     self.os_.write_str(&format!("label=\"${}:=$pos\", shape=octagon", that.data().u_position_register.reg)).unwrap();
                }
                ActionType::BEGIN_POSITIVE_SUBMATCH => {
                     self.os_.write_str(&format!("label=\"${}:=$pos,begin-positive\", shape=septagon", that.data().u_submatch.current_position_register)).unwrap();
                }
                ActionType::BEGIN_NEGATIVE_SUBMATCH => {
                     self.os_.write_str(&format!("label=\"${}:=$pos,begin-negative\", shape=septagon", that.data().u_submatch.current_position_register)).unwrap();
                }
                ActionType::POSITIVE_SUBMATCH_SUCCESS => {
                    self.os_.write_str("label=\"escape\", shape=septagon").unwrap();
                }
                ActionType::EMPTY_MATCH_CHECK => {
                     self.os_.write_str(&format!("label=\"${}=$pos?,${}<{}?\", shape=septagon", that.data().u_empty_match_check.start_register, that.data().u_empty_match_check.repetition_register, that.data().u_empty_match_check.repetition_limit)).unwrap();
                }
                ActionType::CLEAR_CAPTURES => {
                     self.os_.write_str(&format!("label=\"clear ${} to ${}\", shape=septagon", that.data().u_clear_captures.range_from, that.data().u_clear_captures.range_to)).unwrap();
                }
                ActionType::MODIFY_FLAGS => {
                    self.os_.write_str(&format!("label=\"flags ${}\", shape=septagon", that.flags())).unwrap();
                }
            }
            self.os_.write_str("];\n").unwrap();
            // self.print_attributes(that);
            let successor = that.on_success();
            self.os_.write_str("  n").unwrap();
            // self.os_.write_str(&format!("{:p}", that)).unwrap();
            self.os_.write_str(" -> n").unwrap();
            // self.os_.write_str(&format!("{:p}", successor)).unwrap();
            self.os_.write_str(";\n").unwrap();
            self.visit(successor);
        }
    }

    struct AttributePrinter<'a> {
        os_: &'a mut StdoutStream,
        first_: bool,
    }

    impl<'a> AttributePrinter<'a> {
        fn new(os: &'a mut StdoutStream) -> Self {
            AttributePrinter { os_: os, first_: true }
        }

        fn print_separator(&mut self) {
            if self.first_ {
                self.first_ = false;
            } else {
                self.os_.write_str("|").unwrap();
            }
        }

        fn print_bit(&mut self, name: &str, value: bool) {
            if !value {
                return;
            }
            self.print_separator();
            self.os_.write_str(&format!("{{{}}}", name)).unwrap();
        }

        fn print_positive(&mut self, name: &str, value: i32) {
            if value < 0 {
                return;
            }
            self.print_separator();
            self.os_.write_str(&format!("{{{}: {}}}", name, value)).unwrap();
        }
    }

    pub struct DotPrinter {}

    impl DotPrinter {
        pub fn dot_print(label: &str, node: &mut RegExpNode) {
            let os = StdoutStream {};
            let mut printer = DotPrinterImpl::new(os);
            printer.print_node(label, node);
        }
    }
}
